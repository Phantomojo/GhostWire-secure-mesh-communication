//! reputation.rs — Web of trust and TOFU key verification
//!
//! Problem: Ghostwire is fully self-sovereign, which means Sybil attacks
//! are trivial — anyone can create millions of fake identities.
//!
//! Solution: Trust On First Use (TOFU) + social graph attestation.
//!   - Once you've seen a peer's key, pin it. Key changes are flagged loudly.
//!   - Peers can vouch for each other's keys (like SSH known_hosts + web of trust)
//!   - Trust scores bubble up through the graph but decay with distance
//!
//! No PKI, no CA, no blockchain. Pure peer attestation.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Trust level for a known peer
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrustLevel {
    /// Never seen before
    Unknown,
    /// First contact — key pinned but not verified
    FirstContact,
    /// Key matches what we've seen before
    Trusted,
    /// Someone we trust has vouched for this peer
    Vouched,
    /// We have met this peer out-of-band (QR scan, in person)
    Verified,
    /// Key has changed since we first saw it — possible MITM
    KeyChanged,
    /// Explicitly blocked by the user
    Blocked,
}

impl TrustLevel {
    pub fn score(&self) -> f32 {
        match self {
            Self::Unknown => 0.0,
            Self::FirstContact => 0.3,
            Self::Trusted => 0.6,
            Self::Vouched => 0.7,
            Self::Verified => 1.0,
            Self::KeyChanged => -1.0, // Negative = warning
            Self::Blocked => -2.0,
        }
    }

    pub fn is_usable(&self) -> bool {
        !matches!(self, Self::Blocked | Self::KeyChanged)
    }
}

/// A vouching attestation from one peer about another
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attestation {
    /// The peer making the claim
    pub attester_id: String,
    /// The peer being attested
    pub subject_id: String,
    /// The subject's public key being attested
    pub subject_pub_key: Vec<u8>,
    /// Ed25519 signature from attester over (subject_id || subject_pub_key || timestamp)
    pub signature: Vec<u8>,
    /// Unix timestamp
    pub timestamp: u64,
    /// Optional human-readable note
    pub note: Option<String>,
}

/// Our record of a known peer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerRecord {
    pub peer_id: String,
    /// The first public key we saw from this peer (pinned)
    pub pinned_key: Vec<u8>,
    /// The most recently seen key (may differ from pinned if changed)
    pub current_key: Vec<u8>,
    pub trust_level: TrustLevel,
    pub first_seen: u64,
    pub last_seen: u64,
    /// Attestations we've received about this peer from others we trust
    pub attestations: Vec<Attestation>,
    /// How many successful interactions we've had
    pub interaction_count: u32,
    /// User-provided display name
    pub display_name: Option<String>,
}

impl PeerRecord {
    pub fn has_key_changed(&self) -> bool {
        self.pinned_key != self.current_key
    }

    /// Compute trust score incorporating attestations and interaction history
    pub fn computed_trust_score(&self, our_attesters: &HashMap<String, f32>) -> f32 {
        let base = self.trust_level.score();
        if base < 0.0 {
            return base; // Blocked or key-changed always returns raw score
        }

        // Attestation bonus: each attester we trust adds weight
        let attestation_bonus: f32 = self
            .attestations
            .iter()
            .filter_map(|a| our_attesters.get(&a.attester_id))
            .map(|&attester_score| attester_score * 0.3) // Max 0.3 per attester
            .sum::<f32>()
            .min(0.3); // Cap total attestation bonus

        // Interaction bonus: more positive interactions = more trust
        let interaction_bonus = (self.interaction_count as f32 / 100.0).min(0.1);

        (base + attestation_bonus + interaction_bonus).min(1.0)
    }
}

/// The trust store — manages all known peer records
pub struct TrustStore {
    peers: Arc<RwLock<HashMap<String, PeerRecord>>>,
    /// Our own trust scores for people who might attest others
    our_trust_scores: Arc<RwLock<HashMap<String, f32>>>,
}

impl TrustStore {
    pub fn new() -> Self {
        Self {
            peers: Arc::new(RwLock::new(HashMap::new())),
            our_trust_scores: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Called when we first encounter a peer's public key (TOFU)
    ///
    /// Returns:
    ///   - Ok(true) if this is a new peer (key pinned)
    ///   - Ok(false) if key matches existing record (trusted)
    ///   - Err with warning if key has CHANGED for a known peer
    pub async fn verify_or_pin(&self, peer_id: &str, public_key: &[u8]) -> Result<bool> {
        let mut peers = self.peers.write().await;

        if let Some(record) = peers.get_mut(peer_id) {
            record.current_key = public_key.to_vec();
            record.last_seen = now_secs();
            record.interaction_count += 1;

            if record.pinned_key != public_key {
                // KEY CHANGED — this is a serious warning
                record.trust_level = TrustLevel::KeyChanged;
                anyhow::bail!(
                    "⚠️  KEY CHANGE DETECTED for peer {}! \
                     Previous key: {} \
                     New key: {} \
                     This could indicate a man-in-the-middle attack.",
                    peer_id,
                    hex::encode(&record.pinned_key),
                    hex::encode(public_key)
                );
            }

            // Key matches — update trust if it was FirstContact
            if record.trust_level == TrustLevel::FirstContact {
                record.trust_level = TrustLevel::Trusted;
            }

            Ok(false)
        } else {
            // New peer — TOFU pin
            peers.insert(
                peer_id.to_string(),
                PeerRecord {
                    peer_id: peer_id.to_string(),
                    pinned_key: public_key.to_vec(),
                    current_key: public_key.to_vec(),
                    trust_level: TrustLevel::FirstContact,
                    first_seen: now_secs(),
                    last_seen: now_secs(),
                    attestations: vec![],
                    interaction_count: 1,
                    display_name: None,
                },
            );
            Ok(true)
        }
    }

    /// Record an attestation from a trusted peer about another peer
    pub async fn add_attestation(&self, attestation: Attestation) -> Result<()> {
        // Verify the attestation signature before accepting it
        self.verify_attestation_signature(&attestation)?;

        let mut peers = self.peers.write().await;
        let record = peers
            .entry(attestation.subject_id.clone())
            .or_insert_with(|| PeerRecord {
                peer_id: attestation.subject_id.clone(),
                pinned_key: attestation.subject_pub_key.clone(),
                current_key: attestation.subject_pub_key.clone(),
                trust_level: TrustLevel::Vouched,
                first_seen: now_secs(),
                last_seen: now_secs(),
                attestations: vec![],
                interaction_count: 0,
                display_name: None,
            });

        // Don't add duplicate attestations from same attester
        if !record
            .attestations
            .iter()
            .any(|a| a.attester_id == attestation.attester_id)
        {
            record.attestations.push(attestation);

            // Upgrade trust if not already higher
            if record.trust_level == TrustLevel::Unknown
                || record.trust_level == TrustLevel::FirstContact
            {
                record.trust_level = TrustLevel::Vouched;
            }
        }

        Ok(())
    }

    /// Upgrade a peer to Verified status (after out-of-band verification e.g. QR scan)
    pub async fn mark_verified(&self, peer_id: &str, verified_key: &[u8]) -> Result<()> {
        let mut peers = self.peers.write().await;
        let record = peers
            .get_mut(peer_id)
            .ok_or_else(|| anyhow::anyhow!("Unknown peer: {}", peer_id))?;

        if record.pinned_key != verified_key {
            anyhow::bail!("Key mismatch during verification — QR key doesn't match pinned key");
        }

        record.trust_level = TrustLevel::Verified;
        Ok(())
    }

    /// Block a peer.
    ///
    /// Previously this silently did nothing when the peer was not already in
    /// the store: `if let Some(record) = peers.get_mut(peer_id)` simply fell
    /// through, and the caller had no way to tell. A user who blocked a peer
    /// they had never recorded would be told it worked, and the peer would
    /// remain fully trusted. Blocking is a security decision the user made
    /// explicitly, so it must be recorded even for an unknown peer — an
    /// explicit deny has to win over a missing record.
    ///
    /// Returns `true` if the peer was newly inserted (i.e. was not previously
    /// known), so callers can report what actually happened.
    pub async fn block(&self, peer_id: &str) -> bool {
        let mut peers = self.peers.write().await;
        if let Some(record) = peers.get_mut(peer_id) {
            record.trust_level = TrustLevel::Blocked;
            false
        } else {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            peers.insert(
                peer_id.to_string(),
                PeerRecord {
                    peer_id: peer_id.to_string(),
                    pinned_key: Vec::new(),
                    current_key: Vec::new(),
                    trust_level: TrustLevel::Blocked,
                    first_seen: now,
                    last_seen: now,
                    attestations: Vec::new(),
                    interaction_count: 0,
                    display_name: None,
                },
            );
            true
        }
    }

    /// Is this peer explicitly blocked?
    ///
    /// A missing record is NOT blocked. Callers that want fail-closed behaviour
    /// must consult this explicitly rather than infer it from absence.
    pub async fn is_blocked(&self, peer_id: &str) -> bool {
        let peers = self.peers.read().await;
        peers
            .get(peer_id)
            .map(|r| r.trust_level == TrustLevel::Blocked)
            .unwrap_or(false)
    }

    /// Get trust info for a peer
    pub async fn get(&self, peer_id: &str) -> Option<PeerRecord> {
        self.peers.read().await.get(peer_id).cloned()
    }

    /// Get trust score (0.0–1.0, or negative for warnings)
    pub async fn trust_score(&self, peer_id: &str) -> f32 {
        let peers = self.peers.read().await;
        let scores = self.our_trust_scores.read().await;

        peers
            .get(peer_id)
            .map(|r| r.computed_trust_score(&scores))
            .unwrap_or(0.0)
    }

    /// List all known peers, sorted by trust score descending
    pub async fn all_peers(&self) -> Vec<PeerRecord> {
        let peers = self.peers.read().await;
        let scores = self.our_trust_scores.read().await;
        let mut list: Vec<_> = peers.values().cloned().collect();
        list.sort_by(|a, b| {
            b.computed_trust_score(&scores)
                .partial_cmp(&a.computed_trust_score(&scores))
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        list
    }

    /// Set our trust score for a peer (used when they might attest others)
    pub async fn set_attester_trust(&self, peer_id: String, score: f32) {
        self.our_trust_scores
            .write()
            .await
            .insert(peer_id, score.clamp(0.0, 1.0));
    }

    fn verify_attestation_signature(&self, att: &Attestation) -> Result<()> {
        // In production: verify att.signature using att.attester_id's public key
        // against payload: (subject_id || subject_pub_key || timestamp as 8-byte LE)
        //
        // Using ring::signature::Ed25519 or ed25519-dalek:
        //
        // let attester_pub_key = self.get_attester_pub_key(&att.attester_id)?;
        // let payload = [att.subject_id.as_bytes(), &att.subject_pub_key,
        //                &att.timestamp.to_le_bytes()].concat();
        // ring::signature::UnparsedPublicKey::new(
        //     &ring::signature::ED25519, &attester_pub_key
        // ).verify(&payload, &att.signature)?;
        //
        // Stub for now — TODO: wire with identity keypair lookup
        let _ = att;
        Ok(())
    }
}

impl Default for TrustStore {
    fn default() -> Self {
        Self::new()
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tofu_pin_on_first_contact() {
        let store = TrustStore::new();
        let key = vec![0x01u8; 32];
        let is_new = store.verify_or_pin("peer1", &key).await.unwrap();
        assert!(is_new);

        let record = store.get("peer1").await.unwrap();
        assert_eq!(record.trust_level, TrustLevel::FirstContact);
    }

    #[tokio::test]
    async fn second_contact_with_same_key_is_trusted() {
        let store = TrustStore::new();
        let key = vec![0x02u8; 32];
        store.verify_or_pin("peer1", &key).await.unwrap();
        let is_new = store.verify_or_pin("peer1", &key).await.unwrap();
        assert!(!is_new);

        let record = store.get("peer1").await.unwrap();
        assert_eq!(record.trust_level, TrustLevel::Trusted);
    }

    #[tokio::test]
    async fn key_change_returns_error() {
        let store = TrustStore::new();
        let key1 = vec![0x03u8; 32];
        let key2 = vec![0x04u8; 32];
        store.verify_or_pin("peer1", &key1).await.unwrap();
        let result = store.verify_or_pin("peer1", &key2).await;
        assert!(result.is_err());

        let record = store.get("peer1").await.unwrap();
        assert_eq!(record.trust_level, TrustLevel::KeyChanged);
    }

    #[tokio::test]
    async fn block_sets_level() {
        let store = TrustStore::new();
        let key = vec![0x05u8; 32];
        store.verify_or_pin("peer1", &key).await.unwrap();
        store.block("peer1").await;

        let record = store.get("peer1").await.unwrap();
        assert_eq!(record.trust_level, TrustLevel::Blocked);
        assert!(!record.trust_level.is_usable());
    }

    #[test]
    fn sybil_detector_allows_normal() {
        let mut detector = SybilDetector::new();
        let now = 1000000;
        assert!(!detector.record_first_contact(now));
    }

    #[test]
    fn sybil_detector_blocks_rapid() {
        let mut detector = SybilDetector::new();
        let now = 1000000;
        for _ in 0..11 {
            detector.record_first_contact(now);
        }
        assert!(detector.is_sybil());
    }

    #[test]
    fn decay_recency() {
        let score_new = compute_score_with_decay(0.6, 100, 100, 10, 200);
        let score_old = compute_score_with_decay(0.6, 100, 100, 10, 100 + 86400 * 30);
        assert!(score_new > score_old);
    }

    #[test]
    fn decay_negative_passthrough() {
        let score = compute_score_with_decay(-1.0, 100, 100, 10, 200);
        assert_eq!(score, -1.0);
    }
}

impl TrustStore {
    pub async fn blocked_count(&self) -> usize {
        self.peers
            .read()
            .await
            .values()
            .filter(|r| r.trust_level == TrustLevel::Blocked)
            .count()
    }

    pub async fn average_trust_score(&self) -> f32 {
        let peers = self.peers.read().await;
        if peers.is_empty() {
            return 0.0;
        }
        let sum: f32 = peers.values().map(|r| r.trust_level.score()).sum();
        sum / peers.len() as f32
    }

    pub async fn ranked_peers(&self) -> Vec<(String, f32)> {
        let peers = self.peers.read().await;
        let attester_scores = self.our_trust_scores.read().await;
        let mut scores: Vec<(String, f32)> = peers
            .iter()
            .filter(|(_, r)| r.trust_level.is_usable())
            .map(|(id, r)| (id.clone(), r.computed_trust_score(&attester_scores)))
            .collect();
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scores
    }
}

const DECAY_HALF_LIFE_SECS: f64 = 86400.0 * 7.0;
const SYBIL_WINDOW_SECS: u64 = 60;
const SYBIL_THRESHOLD: usize = 10;

pub struct SybilDetector {
    recent_first_contacts: std::collections::VecDeque<u64>,
}

impl SybilDetector {
    pub fn new() -> Self {
        Self {
            recent_first_contacts: std::collections::VecDeque::new(),
        }
    }

    pub fn record_first_contact(&mut self, now: u64) -> bool {
        self.recent_first_contacts.push_back(now);
        while let Some(&front) = self.recent_first_contacts.front() {
            if now.saturating_sub(front) > SYBIL_WINDOW_SECS {
                self.recent_first_contacts.pop_front();
            } else {
                break;
            }
        }
        self.recent_first_contacts.len() > SYBIL_THRESHOLD
    }

    pub fn is_sybil(&self) -> bool {
        self.recent_first_contacts.len() > SYBIL_THRESHOLD
    }
}

pub fn compute_score_with_decay(
    base_trust: f32,
    first_seen: u64,
    last_seen: u64,
    interaction_count: u32,
    now: u64,
) -> f32 {
    if base_trust < 0.0 {
        return base_trust;
    }

    let age_secs = now.saturating_sub(first_seen) as f64;
    let recency_secs = now.saturating_sub(last_seen) as f64;

    let age_decay = (-0.693 * age_secs / DECAY_HALF_LIFE_SECS).exp();
    let recency_boost = if recency_secs < 86400.0 {
        1.0
    } else if recency_secs < 86400.0 * 7.0 {
        0.8
    } else {
        0.5
    };

    let interaction_bonus = (interaction_count as f32 / 100.0).min(0.1);

    let score = (base_trust * age_decay as f32 + interaction_bonus) * recency_boost;
    score.clamp(0.0, 1.0)
}

#[cfg(test)]
mod block_tests {
    use super::{TrustLevel, TrustStore};

    /// REGRESSION: blocking a peer that is not yet in the store must still
    /// record the block. The old implementation used `if let Some(..)`, so an
    /// unknown peer was silently NOT blocked while the caller reported success.
    #[tokio::test]
    async fn blocking_unknown_peer_is_recorded() {
        let store = TrustStore::new();
        let peer = "12D3KooW-unknown-peer-never-seen-before";

        assert!(
            !store.is_blocked(peer).await,
            "unknown peer starts unblocked"
        );

        let newly_inserted = store.block(peer).await;
        assert!(
            newly_inserted,
            "blocking an unknown peer must report that it was newly inserted"
        );
        assert!(
            store.is_blocked(peer).await,
            "SECURITY REGRESSION: an explicit block on an unknown peer was silently \
             dropped, so the peer stayed fully trusted"
        );
    }

    /// Blocking a peer that IS already known must work too, and must be idempotent.
    #[tokio::test]
    async fn blocking_known_peer_is_idempotent() {
        let store = TrustStore::new();
        let peer = "12D3KooW-known-peer";
        store.block(peer).await;

        let second = store.block(peer).await;
        assert!(!second, "re-blocking a known peer must not re-insert it");
        assert!(store.is_blocked(peer).await);
    }

    /// Absence must not read as "blocked" — callers wanting fail-closed
    /// behaviour have to ask explicitly via `is_blocked`.
    #[tokio::test]
    async fn absent_peer_is_not_blocked() {
        let store = TrustStore::new();
        assert!(!store.is_blocked("never-seen").await);
    }

    /// A block must be visible in the peer listing so the UI can show it.
    #[tokio::test]
    async fn blocked_peer_appears_in_listing() {
        let store = TrustStore::new();
        store.block("12D3KooW-listed-blocked").await;
        let all = store.all_peers().await;
        assert!(all.iter().any(|p| p.trust_level == TrustLevel::Blocked));
    }
}
