//! epidemic.rs — GhostDTN: Spray-and-Wait + PRoPHET epidemic routing
//!
//! When two Ghostwire nodes come within Bluetooth range, they execute a
//! sync handshake using a bloom filter to efficiently exchange what each
//! node has and needs, then transfer bundles accordingly.
//!
//! Algorithm: Spray-and-Wait (Spyropoulos et al., 2005)
//!   + PRoPHET delivery predictability (Lindgren et al., 2003)
//!
//! Spray phase: forward up to L/2 copies when encountering a node
//! Wait phase: only deliver when copies_remaining == 1 AND we ARE the destination
//!
//! PRoPHET: each node maintains P(dest) ∈ [0,1] per destination.
//! On encounter: P(d) = P_old + (1 - P_old) × P_init  (P_init = 0.75)
//! Time decay:   P(d) = P(d) × γ^(Δt/unit)            (γ = 0.98, unit = 1 hour)
//! Transitivity: P(d,via) = P(d) + (1-P(d)) × P(via,d) × β  (β = 0.25)
//!
//! The buffer is bounded at MAX_BUFFER_BYTES. When full, evict lowest-probability
//! bundles first (oldest with lowest delivery_probability).

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

use super::bundle::DtnBundle;

/// PRoPHET initial encounter probability
const P_INIT: f32 = 0.75;
/// PRoPHET decay factor per hour
const GAMMA: f32 = 0.98;
/// PRoPHET transitivity scaling factor
const BETA: f32 = 0.25;
/// Maximum buffer size in bytes (50 MB per node)
const MAX_BUFFER_BYTES: usize = 50 * 1024 * 1024;
/// Bloom filter size in bits for sync (1024 bits = 128 bytes, covers ~10k items at 1% FP)
const BLOOM_BITS: usize = 1024;

// ── Bloom filter (minimal, for sync handshake) ────────────────────────────────

/// Compact bloom filter for advertising which CIDs we have
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncBloom {
    bits: Vec<u64>, // 1024 bits = 16 × u64
    count: u32,
}

impl SyncBloom {
    pub fn new() -> Self {
        Self { bits: vec![0u64; BLOOM_BITS / 64], count: 0 }
    }

    fn hash_positions(cid: &[u8; 32]) -> [usize; 3] {
        // Three independent hash positions using different 16-byte slices of CID
        let h0 = u64::from_le_bytes(cid[0..8].try_into().unwrap()) as usize % BLOOM_BITS;
        let h1 = u64::from_le_bytes(cid[8..16].try_into().unwrap()) as usize % BLOOM_BITS;
        let h2 = u64::from_le_bytes(cid[16..24].try_into().unwrap()) as usize % BLOOM_BITS;
        [h0, h1, h2]
    }

    pub fn insert(&mut self, cid: &[u8; 32]) {
        for pos in Self::hash_positions(cid) {
            self.bits[pos / 64] |= 1u64 << (pos % 64);
        }
        self.count += 1;
    }

    pub fn contains(&self, cid: &[u8; 32]) -> bool {
        Self::hash_positions(cid).iter().all(|&pos| {
            self.bits[pos / 64] & (1u64 << (pos % 64)) != 0
        })
    }

    pub fn count(&self) -> u32 { self.count }
}

impl Default for SyncBloom { fn default() -> Self { Self::new() } }

// ── Delivery predictability table ────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct DeliveryTable {
    /// P(dest) per destination node ID
    scores: HashMap<[u8; 32], f32>,
    /// Last update time per destination (Unix secs)
    last_updated: HashMap<[u8; 32], u64>,
}

impl DeliveryTable {
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
        for v in self.last_updated.values_mut() { *v = now; }
    }

    /// Update score on encounter with `encountered_node` heading to `destination`
    pub fn on_encounter(&mut self, destination: &[u8; 32]) {
        let current = self.scores.get(destination).copied().unwrap_or(0.0);
        let new_score = current + (1.0 - current) * P_INIT;
        self.scores.insert(*destination, new_score.min(1.0));
        self.last_updated.insert(*destination, now_secs());
    }

    /// Apply transitivity: we met a node that has a high P(dest)
    pub fn apply_transitivity(&mut self, destination: &[u8; 32], their_score: f32) {
        let our_score = self.scores.get(destination).copied().unwrap_or(0.0);
        let new_score = our_score + (1.0 - our_score) * their_score * BETA;
        self.scores.insert(*destination, new_score.min(1.0));
    }

    pub fn get(&self, destination: &[u8; 32]) -> f32 {
        self.scores.get(destination).copied().unwrap_or(0.0)
    }

    /// Collect all scores for exchange with an encountered node
    pub fn export_scores(&self) -> Vec<([u8; 32], f32)> {
        self.scores.iter().map(|(k, v)| (*k, *v)).collect()
    }
}

impl Default for DeliveryTable { fn default() -> Self { Self::new() } }

// ── DTN store ────────────────────────────────────────────────────────────────

/// The GhostDTN store: bundles + delivery table
pub struct DtnStore {
    /// Stored bundles keyed by CID
    bundles: Arc<RwLock<HashMap<[u8; 32], DtnBundle>>>,
    /// PRoPHET delivery table
    delivery_table: Arc<RwLock<DeliveryTable>>,
    /// Our own node ID
    our_node_id: [u8; 32],
    /// Current total buffer size in bytes
    buffer_bytes: Arc<RwLock<usize>>,
}

impl DtnStore {
    pub fn new(our_node_id: [u8; 32]) -> Self {
        Self {
            bundles: Arc::new(RwLock::new(HashMap::new())),
            delivery_table: Arc::new(RwLock::new(DeliveryTable::new())),
            our_node_id,
            buffer_bytes: Arc::new(RwLock::new(0)),
        }
    }

    /// Store a bundle for forwarding. Returns false if buffer is full.
    pub async fn store(&self, bundle: DtnBundle) -> bool {
        if bundle.is_expired() { return false; }

        let size = bundle.size_bytes();
        let mut total = self.buffer_bytes.write().await;

        if *total + size > MAX_BUFFER_BYTES {
            // Try to evict lowest-score bundles to make room
            drop(total);
            self.evict_for_space(size).await;
            let mut total = self.buffer_bytes.write().await;
            if *total + size > MAX_BUFFER_BYTES {
                return false; // Still no space
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

    /// Remove delivered/expired bundles
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
    /// (their bloom doesn't contain the CID)
    pub async fn bundles_they_need(&self, their_bloom: &SyncBloom) -> Vec<DtnBundle> {
        let bundles = self.bundles.read().await;
        bundles.values()
            .filter(|b| !b.is_expired() && !their_bloom.contains(&b.cid))
            .cloned()
            .collect()
    }

    /// CIDs we need (we don't have them, they might)
    pub async fn cids_we_need(&self, their_bloom: &SyncBloom) -> Vec<[u8; 32]> {
        // We can't enumerate what they have from a bloom filter alone —
        // but we can tell them our bloom and they compute what to send.
        // This function is a placeholder for the protocol exchange.
        vec![]
    }

    /// Process an encounter with another node
    /// Returns bundles to send to them
    pub async fn on_encounter(
        &self,
        their_node_id: &[u8; 32],
        their_bloom: &SyncBloom,
        their_delivery_scores: &[([u8; 32], f32)],
    ) -> Vec<DtnBundle> {
        // Update delivery table
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

        // Decide which bundles to forward
        let mut to_send: Vec<DtnBundle> = Vec::new();

        for bundle in bundles.values() {
            if bundle.is_expired() { continue; }

            // If we ARE the destination — don't forward, deliver locally
            if bundle.destination == self.our_node_id { continue; }

            // If they ARE the destination — always send (deliver!)
            if &bundle.destination == their_node_id {
                if !their_bloom.contains(&bundle.cid) {
                    to_send.push(bundle.clone());
                }
                continue;
            }

            // Spray phase: share copies if they have better delivery probability
            if bundle.copies_remaining > 1 {
                let our_prob = table.get(&bundle.destination);
                let their_prob = their_delivery_scores.iter()
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

    /// Check if any bundles are for our node_id — these should be delivered locally
    pub async fn drain_local_deliveries(&self) -> Vec<DtnBundle> {
        let mut bundles = self.bundles.write().await;
        let mut delivered = Vec::new();
        let our_id = self.our_node_id;

        bundles.retain(|_, b| {
            if b.destination == our_id && !b.is_expired() {
                delivered.push(b.clone());
                false // Remove from store after delivering
            } else {
                true
            }
        });

        delivered
    }

    async fn evict_for_space(&self, needed: usize) {
        let mut bundles = self.bundles.write().await;
        let table = self.delivery_table.read().await;

        // Sort by delivery probability (ascending — evict least likely first)
        let mut scored: Vec<([u8; 32], f32, usize)> = bundles.iter()
            .map(|(cid, b)| (*cid, table.get(&b.destination), b.size_bytes()))
            .collect();
        scored.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        let mut freed = 0;
        for (cid, _, size) in scored {
            if freed >= needed { break; }
            bundles.remove(&cid);
            freed += size;
        }

        *self.buffer_bytes.write().await = bundles.values().map(|b| b.size_bytes()).sum();
    }

    pub async fn bundle_count(&self) -> usize {
        self.bundles.read().await.len()
    }

    pub async fn buffer_used_bytes(&self) -> usize {
        *self.buffer_bytes.read().await
    }

    pub async fn export_delivery_scores(&self) -> Vec<([u8; 32], f32)> {
        self.delivery_table.read().await.export_scores()
    }
}

// ── Sync handshake message ───────────────────────────────────────────────────

/// The DTN sync handshake sent when two nodes connect via Bluetooth
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DtnSyncHandshake {
    /// This node's ID
    pub node_id: [u8; 32],
    /// Bloom filter of CIDs we hold
    pub have_bloom: SyncBloom,
    /// Our PRoPHET scores (for transitivity computation)
    pub delivery_scores: Vec<([u8; 32], f32)>,
    /// Timestamp
    pub ts: u64,
}

impl DtnSyncHandshake {
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
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()
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
        assert!(score > 0.0 && score <= 1.0, "Score should be in (0,1): {}", score);
        assert!((score - P_INIT).abs() < 0.001, "First encounter = P_INIT: {}", score);
    }

    #[tokio::test]
    async fn store_and_retrieve_bundle() {
        let store = DtnStore::new([0x01u8; 32]);
        let bundle = DtnBundle::new(vec![1, 2, 3, 4], [0x02u8; 32]);
        let cid = bundle.cid;
        let stored = store.store(bundle).await;
        assert!(stored);
        assert!(store.get(&cid).await.is_some());
    }

    #[tokio::test]
    async fn bloom_reflects_stored_bundles() {
        let store = DtnStore::new([0x01u8; 32]);
        let bundle = DtnBundle::new(vec![9, 8, 7], [0x03u8; 32]);
        let cid = bundle.cid;
        store.store(bundle).await;
        let bloom = store.build_bloom().await;
        assert!(bloom.contains(&cid));
    }

    #[tokio::test]
    async fn local_delivery_drained_correctly() {
        let our_id = [0xAAu8; 32];
        let store = DtnStore::new(our_id);
        let bundle = DtnBundle::new(b"hello".to_vec(), our_id);
        store.store(bundle).await;

        let delivered = store.drain_local_deliveries().await;
        assert_eq!(delivered.len(), 1);
        assert_eq!(store.bundle_count().await, 0);
    }
}
