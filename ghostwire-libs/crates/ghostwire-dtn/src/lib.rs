//! # ghostwire-dtn
//!
//! Delay-Tolerant Networking (DTN) with Spray-and-Wait + PRoPHET epidemic routing.
//!
//! ## Features
//!
//! - **Spray-and-Wait** (Spyropoulos et al., 2005) — efficient bundle distribution
//! - **PRoPHET** (Lindgren et al., 2003) — delivery predictability with time decay and transitivity
//! - **Bloom filter sync** — efficient set reconciliation during Bluetooth encounters
//! - **Bundle encryption** — X25519 ECDH + HKDF + XChaCha20-Poly1305
//! - **Bounded buffer** — 50MB max with LRU eviction by delivery probability
//! - **GNN-enhanced routing** — optional ML-based forwarding decisions
//!
//! ## Quick Start
//!
//! ```rust
//! use ghostwire_dtn::{DtnStore, DtnBundle, DtnSyncHandshake, SyncBloom};
//!
//! # #[tokio::main]
//! # async fn main() {
//! let store = DtnStore::new([0x01u8; 32]);
//!
//! // Create a bundle
//! let bundle = DtnBundle::new(vec![1, 2, 3, 4], [0x02u8; 32], None);
//! let cid = bundle.cid;
//!
//! // Store it
//! let stored = store.store(bundle).await;
//! assert!(stored);
//!
//! // Retrieve it
//! let retrieved = store.get(&cid).await;
//! assert!(retrieved.is_some());
//! # }
//! ```
//!
//! ## Protocol
//!
//! When two nodes come within range:
//! 1. Exchange `DtnSyncHandshake` messages (bloom filter + delivery scores)
//! 2. Each node computes which bundles to forward (`on_encounter`)
//! 3. Bundles are transferred
//! 4. Delivery tables are updated with transitivity
//!
//! ## Reference
//!
//! - Spyropoulos, T., et al. "Spray and Wait: An Efficient Routing Scheme for Intermittently Connected Mobile Networks."
//! - Lindgren, A., et al. "Probabilistic Routing in Intermittently Connected Networks."

#![warn(missing_docs)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

// ── Constants ────────────────────────────────────────────────────────────────

/// Default spray copies (L=6 from literature)
pub const SPRAY_L: u8 = 6;
/// Default TTL: 72 hours in seconds
pub const BUNDLE_TTL_SECS: u64 = 72 * 3600;
/// PRoPHET initial encounter probability
const P_INIT: f32 = 0.75;
/// PRoPHET decay factor per hour
const GAMMA: f32 = 0.98;
/// PRoPHET transitivity scaling factor
const BETA: f32 = 0.25;
/// Maximum buffer size in bytes (50 MB)
const MAX_BUFFER_BYTES: usize = 50 * 1024 * 1024;
/// Bloom filter size in bits
const BLOOM_BITS: usize = 1024;

// ── Bloom Filter ─────────────────────────────────────────────────────────────

/// Compact bloom filter for advertising which CIDs we hold
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncBloom {
    bits: Vec<u64>,
    count: u32,
}

impl SyncBloom {
    /// Create a new empty bloom filter
    pub fn new() -> Self {
        Self {
            bits: vec![0u64; BLOOM_BITS / 64],
            count: 0,
        }
    }

    fn hash_positions(cid: &[u8; 32]) -> [usize; 3] {
        let h0 = u64::from_le_bytes(cid[0..8].try_into().unwrap()) as usize % BLOOM_BITS;
        let h1 = u64::from_le_bytes(cid[8..16].try_into().unwrap()) as usize % BLOOM_BITS;
        let h2 = u64::from_le_bytes(cid[16..24].try_into().unwrap()) as usize % BLOOM_BITS;
        [h0, h1, h2]
    }

    /// Insert a CID into the bloom filter
    pub fn insert(&mut self, cid: &[u8; 32]) {
        for pos in Self::hash_positions(cid) {
            self.bits[pos / 64] |= 1u64 << (pos % 64);
        }
        self.count += 1;
    }

    /// Check if a CID might be in the bloom filter
    pub fn contains(&self, cid: &[u8; 32]) -> bool {
        Self::hash_positions(cid)
            .iter()
            .all(|&pos| self.bits[pos / 64] & (1u64 << (pos % 64)) != 0)
    }

    /// Number of items inserted
    pub fn count(&self) -> u32 {
        self.count
    }
}

impl Default for SyncBloom {
    fn default() -> Self {
        Self::new()
    }
}

// ── DTN Bundle ───────────────────────────────────────────────────────────────

/// Encrypted DTN transport payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedBundlePayload {
    /// Ephemeral X25519 public key
    pub ephemeral_pub: [u8; 32],
    /// XChaCha20-Poly1305 nonce
    pub nonce: Vec<u8>,
    /// Encrypted ciphertext
    pub ciphertext: Vec<u8>,
}

impl EncryptedBundlePayload {
    /// Encrypt payload to the destination's X25519 public key
    pub fn encrypt(
        plaintext: &[u8],
        dest_x25519_pub: &[u8; 32],
    ) -> Result<Self, Box<dyn std::error::Error>> {
        use chacha20poly1305::{aead::Aead, KeyInit, XChaCha20Poly1305, XNonce};
        use rand::RngCore;
        use ring::hkdf;
        use x25519_dalek::{PublicKey, StaticSecret};

        let mut ephemeral_bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut ephemeral_bytes);
        ephemeral_bytes[0] &= 248;
        ephemeral_bytes[31] &= 127;
        ephemeral_bytes[31] |= 64;

        let ephemeral_secret = StaticSecret::from(ephemeral_bytes);
        let ephemeral_pub = PublicKey::from(&ephemeral_secret);
        let dest_pubkey = PublicKey::from(*dest_x25519_pub);
        let shared = ephemeral_secret.diffie_hellman(&dest_pubkey);

        let salt = hkdf::Salt::new(hkdf::HKDF_SHA256, b"ghostwire-dtn-bundle");
        let prk = salt.extract(shared.as_bytes());
        let okm = prk
            .expand(&[b"dtn-transport-key"], hkdf::HKDF_SHA256)
            .map_err(|_| "HKDF expand failed")?;
        let mut key = [0u8; 32];
        okm.fill(&mut key).map_err(|_| "HKDF fill failed")?;

        let cipher = XChaCha20Poly1305::new(&key.into());
        let mut nonce_bytes = [0u8; 24];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = XNonce::from_slice(&nonce_bytes);
        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| format!("DTN bundle encryption failed: {:?}", e))?;

        Ok(Self {
            ephemeral_pub: *ephemeral_pub.as_bytes(),
            nonce: nonce_bytes.to_vec(),
            ciphertext,
        })
    }

    /// Decrypt payload using the destination's X25519 secret key
    pub fn decrypt(
        &self,
        dest_x25519_secret: &[u8; 32],
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        use chacha20poly1305::{aead::Aead, KeyInit, XChaCha20Poly1305, XNonce};
        use ring::hkdf;
        use x25519_dalek::{PublicKey, StaticSecret};

        let dest_secret = StaticSecret::from(*dest_x25519_secret);
        let sender_pub = PublicKey::from(self.ephemeral_pub);
        let shared = dest_secret.diffie_hellman(&sender_pub);

        let salt = hkdf::Salt::new(hkdf::HKDF_SHA256, b"ghostwire-dtn-bundle");
        let prk = salt.extract(shared.as_bytes());
        let okm = prk
            .expand(&[b"dtn-transport-key"], hkdf::HKDF_SHA256)
            .map_err(|_| "HKDF expand failed")?;
        let mut key = [0u8; 32];
        okm.fill(&mut key).map_err(|_| "HKDF fill failed")?;

        let cipher = XChaCha20Poly1305::new(&key.into());
        let nonce = XNonce::from_slice(&self.nonce);
        let plaintext = cipher
            .decrypt(nonce, self.ciphertext.as_slice())
            .map_err(|_| "DTN bundle decryption failed")?;

        Ok(plaintext)
    }
}

/// A DTN bundle wrapping encrypted payload bytes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DtnBundle {
    /// Content ID: BLAKE3 hash of the payload
    pub cid: [u8; 32],
    /// The encrypted message bytes
    pub payload: Vec<u8>,
    /// Unix timestamp when created
    pub created_at: u64,
    /// Unix timestamp when expires
    pub expires_at: u64,
    /// Destination node ID
    pub destination: [u8; 32],
    /// Remaining spray copies
    pub copies_remaining: u8,
    /// PRoPHET delivery probability
    pub delivery_probability: f32,
    /// Hop count
    pub hop_count: u8,
    /// Whether DTN encrypted
    #[serde(default)]
    pub is_dtn_encrypted: bool,
}

impl DtnBundle {
    /// Create a new bundle from payload bytes
    pub fn new(
        payload: Vec<u8>,
        destination: [u8; 32],
        dest_x25519_pub: Option<&[u8; 32]>,
    ) -> Self {
        let final_payload = if let Some(dest_pub) = dest_x25519_pub {
            match EncryptedBundlePayload::encrypt(&payload, dest_pub) {
                Ok(encrypted) => {
                    let bytes = bincode::serialize(&encrypted).unwrap_or(payload);
                    let cid = *blake3::hash(&bytes).as_bytes();
                    let now = now_secs();
                    return Self {
                        cid,
                        payload: bytes,
                        created_at: now,
                        expires_at: now + BUNDLE_TTL_SECS,
                        destination,
                        copies_remaining: SPRAY_L,
                        delivery_probability: 0.0,
                        hop_count: 0,
                        is_dtn_encrypted: true,
                    };
                }
                Err(e) => {
                    tracing::warn!("DTN transport encryption failed: {}", e);
                    payload
                }
            }
        } else {
            payload
        };

        let cid = *blake3::hash(&final_payload).as_bytes();
        let now = now_secs();
        Self {
            cid,
            payload: final_payload,
            created_at: now,
            expires_at: now + BUNDLE_TTL_SECS,
            destination,
            copies_remaining: SPRAY_L,
            delivery_probability: 0.0,
            hop_count: 0,
            is_dtn_encrypted: dest_x25519_pub.is_some(),
        }
    }

    /// Decrypt the DTN transport layer
    pub fn decrypt_transport_layer(
        &self,
        dest_x25519_secret: &[u8; 32],
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if !self.is_dtn_encrypted {
            return Ok(self.payload.clone());
        }
        let encrypted: EncryptedBundlePayload = bincode::deserialize(&self.payload)
            .map_err(|e| format!("Failed to deserialize: {}", e))?;
        encrypted.decrypt(dest_x25519_secret)
    }

    /// Whether this bundle has expired
    pub fn is_expired(&self) -> bool {
        now_secs() > self.expires_at
    }

    /// Split copies for relay (Spray-and-Wait)
    pub fn split_copies(&mut self) -> u8 {
        if self.copies_remaining <= 1 {
            return 0;
        }
        let to_give = self.copies_remaining / 2;
        self.copies_remaining -= to_give;
        to_give
    }

    /// Create a relay copy
    pub fn make_relay_copy(&self, copies: u8) -> Self {
        let mut copy = self.clone();
        copy.copies_remaining = copies;
        copy.hop_count = copy.hop_count.saturating_add(1);
        copy
    }

    /// Seconds remaining until expiry
    pub fn ttl_remaining(&self) -> u64 {
        self.expires_at.saturating_sub(now_secs())
    }

    /// Approximate size in bytes
    pub fn size_bytes(&self) -> usize {
        self.payload.len() + 80
    }
}

// ── Delivery Table ───────────────────────────────────────────────────────────

/// PRoPHET delivery predictability table
#[derive(Debug, Clone)]
pub struct DeliveryTable {
    scores: HashMap<[u8; 32], f32>,
    last_updated: HashMap<[u8; 32], u64>,
}

impl DeliveryTable {
    /// Create a new empty delivery table
    pub fn new() -> Self {
        Self {
            scores: HashMap::new(),
            last_updated: HashMap::new(),
        }
    }

    /// Apply time decay to all scores
    pub fn decay_all(&mut self) {
        let now = now_secs();
        for (dest, score) in self.scores.iter_mut() {
            let last = self.last_updated.get(dest).copied().unwrap_or(now);
            let hours_elapsed = (now.saturating_sub(last)) as f32 / 3600.0;
            *score *= GAMMA.powf(hours_elapsed);
        }
        let now = now_secs();
        for v in self.last_updated.values_mut() {
            *v = now;
        }
    }

    /// Update score on encounter
    pub fn on_encounter(&mut self, destination: &[u8; 32]) {
        let current = self.scores.get(destination).copied().unwrap_or(0.0);
        let new_score = current + (1.0 - current) * P_INIT;
        self.scores.insert(*destination, new_score.min(1.0));
        self.last_updated.insert(*destination, now_secs());
    }

    /// Apply transitivity
    pub fn apply_transitivity(&mut self, destination: &[u8; 32], their_score: f32) {
        let our_score = self.scores.get(destination).copied().unwrap_or(0.0);
        let new_score = our_score + (1.0 - our_score) * their_score * BETA;
        self.scores.insert(*destination, new_score.min(1.0));
    }

    /// Get score for a destination
    pub fn get(&self, destination: &[u8; 32]) -> f32 {
        self.scores.get(destination).copied().unwrap_or(0.0)
    }

    /// Export all scores
    pub fn export_scores(&self) -> Vec<([u8; 32], f32)> {
        self.scores.iter().map(|(k, v)| (*k, *v)).collect()
    }
}

impl Default for DeliveryTable {
    fn default() -> Self {
        Self::new()
    }
}

// ── DTN Store ────────────────────────────────────────────────────────────────

/// The GhostDTN store: bundles + delivery table
pub struct DtnStore {
    bundles: Arc<RwLock<HashMap<[u8; 32], DtnBundle>>>,
    delivery_table: Arc<RwLock<DeliveryTable>>,
    our_node_id: [u8; 32],
    buffer_bytes: Arc<RwLock<usize>>,
}

impl DtnStore {
    /// Create a new DTN store
    pub fn new(our_node_id: [u8; 32]) -> Self {
        Self {
            bundles: Arc::new(RwLock::new(HashMap::new())),
            delivery_table: Arc::new(RwLock::new(DeliveryTable::new())),
            our_node_id,
            buffer_bytes: Arc::new(RwLock::new(0)),
        }
    }

    /// Store a bundle for forwarding
    pub async fn store(&self, bundle: DtnBundle) -> bool {
        if bundle.is_expired() {
            return false;
        }
        let size = bundle.size_bytes();
        let mut total = self.buffer_bytes.write().await;
        if *total + size > MAX_BUFFER_BYTES {
            drop(total);
            self.evict_for_space(size).await;
            let mut total = self.buffer_bytes.write().await;
            if *total + size > MAX_BUFFER_BYTES {
                return false;
            }
            *total += size;
        } else {
            *total += size;
        }
        self.bundles.write().await.insert(bundle.cid, bundle);
        true
    }

    /// Retrieve a bundle by CID
    pub async fn get(&self, cid: &[u8; 32]) -> Option<DtnBundle> {
        self.bundles.read().await.get(cid).cloned()
    }

    /// Remove expired bundles
    pub async fn purge_expired(&self) {
        let mut bundles = self.bundles.write().await;
        let before = bundles.len();
        bundles.retain(|_, b| !b.is_expired());
        let removed = before - bundles.len();
        if removed > 0 {
            tracing::debug!("DTN: purged {} expired bundles", removed);
        }
    }

    /// Build a bloom filter of all CIDs we hold
    pub async fn build_bloom(&self) -> SyncBloom {
        let bundles = self.bundles.read().await;
        let mut bloom = SyncBloom::new();
        for cid in bundles.keys() {
            bloom.insert(cid);
        }
        bloom
    }

    /// Find bundles we have that the other node needs
    pub async fn bundles_they_need(&self, their_bloom: &SyncBloom) -> Vec<DtnBundle> {
        let bundles = self.bundles.read().await;
        bundles
            .values()
            .filter(|b| !b.is_expired() && !their_bloom.contains(&b.cid))
            .cloned()
            .collect()
    }

    /// Process an encounter with another node
    pub async fn on_encounter(
        &self,
        their_node_id: &[u8; 32],
        their_bloom: &SyncBloom,
        their_delivery_scores: &[([u8; 32], f32)],
    ) -> Vec<DtnBundle> {
        {
            let mut table = self.delivery_table.write().await;
            table.on_encounter(their_node_id);
            table.decay_all();
            for (dest, score) in their_delivery_scores {
                table.apply_transitivity(dest, *score);
            }
        }

        let table = self.delivery_table.read().await;
        let bundles = self.bundles.read().await;
        let mut to_send: Vec<DtnBundle> = Vec::new();

        for bundle in bundles.values() {
            if bundle.is_expired() {
                continue;
            }
            if bundle.destination == self.our_node_id {
                continue;
            }
            if &bundle.destination == their_node_id {
                if !their_bloom.contains(&bundle.cid) {
                    to_send.push(bundle.clone());
                }
                continue;
            }
            if bundle.copies_remaining > 1 {
                let our_prob = table.get(&bundle.destination);
                let their_prob = their_delivery_scores
                    .iter()
                    .find(|(d, _)| d == &bundle.destination)
                    .map(|(_, p)| *p)
                    .unwrap_or(0.0);
                if their_prob > our_prob && !their_bloom.contains(&bundle.cid) {
                    to_send.push(bundle.clone());
                }
            }
        }
        to_send
    }

    /// Process an encounter with GNN-enhanced routing
    #[cfg(feature = "gnn-routing")]
    pub async fn on_encounter_with_gnn(
        &self,
        their_node_id: &[u8; 32],
        their_bloom: &SyncBloom,
        their_delivery_scores: &[([u8; 32], f32)],
        gnn_decisions: &HashMap<[u8; 32], (bool, f32)>,
    ) -> Vec<DtnBundle> {
        {
            let mut table = self.delivery_table.write().await;
            table.on_encounter(their_node_id);
            table.decay_all();
            for (dest, score) in their_delivery_scores {
                table.apply_transitivity(dest, *score);
            }
        }

        let table = self.delivery_table.read().await;
        let bundles = self.bundles.read().await;
        let mut to_send: Vec<DtnBundle> = Vec::new();

        for bundle in bundles.values() {
            if bundle.is_expired() {
                continue;
            }
            if bundle.destination == self.our_node_id {
                continue;
            }
            if &bundle.destination == their_node_id {
                if !their_bloom.contains(&bundle.cid) {
                    to_send.push(bundle.clone());
                }
                continue;
            }
            if bundle.copies_remaining > 1 {
                if let Some(&(should_forward, confidence)) = gnn_decisions.get(&bundle.destination)
                {
                    if should_forward && confidence >= 0.3 && !their_bloom.contains(&bundle.cid) {
                        to_send.push(bundle.clone());
                        continue;
                    }
                }
                let our_prob = table.get(&bundle.destination);
                let their_prob = their_delivery_scores
                    .iter()
                    .find(|(d, _)| d == &bundle.destination)
                    .map(|(_, p)| *p)
                    .unwrap_or(0.0);
                if their_prob > our_prob && !their_bloom.contains(&bundle.cid) {
                    to_send.push(bundle.clone());
                }
            }
        }
        to_send
    }

    /// Check for bundles destined for our node
    pub async fn drain_local_deliveries(&self) -> Vec<DtnBundle> {
        let mut bundles = self.bundles.write().await;
        let mut delivered = Vec::new();
        let our_id = self.our_node_id;
        bundles.retain(|_, b| {
            if b.destination == our_id && !b.is_expired() {
                delivered.push(b.clone());
                false
            } else {
                true
            }
        });
        delivered
    }

    async fn evict_for_space(&self, needed: usize) {
        let mut bundles = self.bundles.write().await;
        let table = self.delivery_table.read().await;
        let mut scored: Vec<([u8; 32], f32, usize)> = bundles
            .iter()
            .map(|(cid, b)| (*cid, table.get(&b.destination), b.size_bytes()))
            .collect();
        scored.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        let mut freed = 0;
        for (cid, _, size) in scored {
            if freed >= needed {
                break;
            }
            bundles.remove(&cid);
            freed += size;
        }
        *self.buffer_bytes.write().await = bundles.values().map(|b| b.size_bytes()).sum();
    }

    /// Number of stored bundles
    pub async fn bundle_count(&self) -> usize {
        self.bundles.read().await.len()
    }

    /// Buffer usage in bytes
    pub async fn buffer_used_bytes(&self) -> usize {
        *self.buffer_bytes.read().await
    }

    /// Export delivery scores
    pub async fn export_delivery_scores(&self) -> Vec<([u8; 32], f32)> {
        self.delivery_table.read().await.export_scores()
    }
}

// ── Sync Handshake ───────────────────────────────────────────────────────────

/// DTN sync handshake message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct DtnSyncHandshake {
    pub node_id: [u8; 32],
    pub have_bloom: SyncBloom,
    pub delivery_scores: Vec<([u8; 32], f32)>,
    pub ts: u64,
}

impl DtnSyncHandshake {
    /// Build a handshake message from a store
    pub async fn build(store: &DtnStore) -> Self {
        Self {
            node_id: store.our_node_id,
            have_bloom: store.build_bloom().await,
            delivery_scores: store.export_delivery_scores().await,
            ts: now_secs(),
        }
    }
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bloom_insert_and_contains() {
        let mut bloom = SyncBloom::new();
        let cid = [0xABu8; 32];
        bloom.insert(&cid);
        assert!(bloom.contains(&cid));
        assert!(!bloom.contains(&[0xCDu8; 32]));
    }

    #[test]
    fn prophet_on_encounter_increases_score() {
        let mut table = DeliveryTable::new();
        let dest = [0x01u8; 32];
        assert_eq!(table.get(&dest), 0.0);
        table.on_encounter(&dest);
        let score = table.get(&dest);
        assert!(score > 0.0 && score <= 1.0, "Score: {}", score);
        assert!(
            (score - P_INIT).abs() < 0.001,
            "First encounter = P_INIT: {}",
            score
        );
    }

    #[tokio::test]
    async fn store_and_retrieve_bundle() {
        let store = DtnStore::new([0x01u8; 32]);
        let bundle = DtnBundle::new(vec![1, 2, 3, 4], [0x02u8; 32], None);
        let cid = bundle.cid;
        let stored = store.store(bundle).await;
        assert!(stored);
        assert!(store.get(&cid).await.is_some());
    }

    #[tokio::test]
    async fn bloom_reflects_stored_bundles() {
        let store = DtnStore::new([0x01u8; 32]);
        let bundle = DtnBundle::new(vec![9, 8, 7], [0x03u8; 32], None);
        let cid = bundle.cid;
        store.store(bundle).await;
        let bloom = store.build_bloom().await;
        assert!(bloom.contains(&cid));
    }

    #[tokio::test]
    async fn local_delivery_drained_correctly() {
        let our_id = [0xAAu8; 32];
        let store = DtnStore::new(our_id);
        let bundle = DtnBundle::new(b"hello".to_vec(), our_id, None);
        store.store(bundle).await;
        let delivered = store.drain_local_deliveries().await;
        assert_eq!(delivered.len(), 1);
        assert_eq!(store.bundle_count().await, 0);
    }

    #[test]
    fn spray_and_wait_copy_split() {
        let mut bundle = DtnBundle::new(vec![1, 2, 3], [0u8; 32], None);
        assert_eq!(bundle.copies_remaining, SPRAY_L);
        let given = bundle.split_copies();
        assert_eq!(given, 3);
        assert_eq!(bundle.copies_remaining, 3);
        let given2 = bundle.split_copies();
        assert_eq!(given2, 1);
        assert_eq!(bundle.copies_remaining, 2);
    }

    #[test]
    fn wait_phase_gives_zero_copies() {
        let mut bundle = DtnBundle::new(vec![1, 2, 3], [0u8; 32], None);
        bundle.copies_remaining = 1;
        assert_eq!(bundle.split_copies(), 0);
        assert_eq!(bundle.copies_remaining, 1);
    }

    #[test]
    fn cid_is_deterministic() {
        let payload = b"test payload".to_vec();
        let dest = [0xABu8; 32];
        let b1 = DtnBundle::new(payload.clone(), dest, None);
        let b2 = DtnBundle::new(payload, dest, None);
        assert_eq!(b1.cid, b2.cid);
    }

    #[test]
    fn dtn_encryption_roundtrip() {
        use rand::RngCore;
        use x25519_dalek::{PublicKey, StaticSecret};

        let mut secret_bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut secret_bytes);
        secret_bytes[0] &= 248;
        secret_bytes[31] &= 127;
        secret_bytes[31] |= 64;

        let secret = StaticSecret::from(secret_bytes);
        let pubkey = PublicKey::from(&secret);
        let pub_bytes = *pubkey.as_bytes();

        let plaintext = b"secret DTN message".to_vec();
        let bundle = DtnBundle::new(plaintext.clone(), [0xABu8; 32], Some(&pub_bytes));
        assert!(bundle.is_dtn_encrypted);
        assert_ne!(bundle.payload, plaintext);

        let decrypted = bundle.decrypt_transport_layer(&secret_bytes).unwrap();
        assert_eq!(decrypted, plaintext);
    }
}
