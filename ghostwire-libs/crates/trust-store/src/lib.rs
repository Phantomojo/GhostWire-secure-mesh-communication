//! # trust-store
//!
//! Web of Trust with TOFU (Trust-On-First-Use) key verification and social graph
//! attestation for P2P networks.
//!
//! ## Features
//!
//! - **TOFU key pinning** — pin a peer's public key on first contact
//! - **Key change detection** — flag potential MITM attacks
//! - **Social graph attestation** — peers vouch for each other's keys
//! - **Computed trust scores** — bubble trust through the graph with decay
//! - **Ed25519 signature verification** — cryptographically verify attestations
//!
//! ## Quick Start
//!
//! ```rust
//! use trust_store::TrustStore;
//!
//! # #[tokio::main]
//! # async fn main() {
//! let store = TrustStore::new();
//! let key = vec![0x01u8; 32];
//!
//! // First contact — TOFU pin
//! let is_new = store.verify_or_pin("peer1", &key).await.unwrap();
//! assert!(is_new);
//!
//! // Second contact with same key — trusted
//! let is_new = store.verify_or_pin("peer1", &key).await.unwrap();
//! assert!(!is_new);
//! # }
//! ```
//!
//! ## No PKI, No CA, No Blockchain
//!
//! Pure peer attestation — like SSH known_hosts + web of trust combined.

#![warn(missing_docs)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

/// Errors that can occur during trust store operations.
#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum TrustError {
    /// Key has changed since first contact — possible MITM
    #[error("Key change detected for peer {peer_id}")]
    KeyChanged { peer_id: String },
    /// Unknown peer
    #[error("Unknown peer: {peer_id}")]
    UnknownPeer { peer_id: String },
    /// Attestation verification failed
    #[error("Attestation verification failed: {0}")]
    AttestationFailed(String),
    /// Key mismatch during verification
    #[error("Key mismatch during verification")]
    KeyMismatch,
}

/// Result type for trust store operations
pub type TrustResult<T> = Result<T, TrustError>;

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
    /// Numeric score for this trust level
    pub fn score(&self) -> f32 {
        match self {
            Self::Unknown => 0.0,
            Self::FirstContact => 0.3,
            Self::Trusted => 0.6,
            Self::Vouched => 0.7,
            Self::Verified => 1.0,
            Self::KeyChanged => -1.0,
            Self::Blocked => -2.0,
        }
    }

    /// Whether this trust level is usable for communication
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
    /// Ed25519 signature from attester
    pub signature: Vec<u8>,
    /// Unix timestamp
    pub timestamp: u64,
    /// Optional human-readable note
    pub note: Option<String>,
}

/// Our record of a known peer
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct PeerRecord {
    pub peer_id: String,
    /// The first public key we saw from this peer (pinned)
    pub pinned_key: Vec<u8>,
    /// The most recently seen key
    pub current_key: Vec<u8>,
    pub trust_level: TrustLevel,
    pub first_seen: u64,
    pub last_seen: u64,
    pub attestations: Vec<Attestation>,
    pub interaction_count: u32,
    pub display_name: Option<String>,
}

impl PeerRecord {
    /// Whether the key has changed since first contact
    pub fn has_key_changed(&self) -> bool {
        self.pinned_key != self.current_key
    }

    /// Compute trust score incorporating attestations and interaction history
    pub fn computed_trust_score(&self, our_attesters: &HashMap<String, f32>) -> f32 {
        let base = self.trust_level.score();
        if base < 0.0 {
            return base;
        }

        let attestation_bonus: f32 = self
            .attestations
            .iter()
            .filter_map(|a| our_attesters.get(&a.attester_id))
            .map(|&attester_score| attester_score * 0.3)
            .sum::<f32>()
            .min(0.3);

        let interaction_bonus = (self.interaction_count as f32 / 100.0).min(0.1);

        (base + attestation_bonus + interaction_bonus).min(1.0)
    }
}

/// The trust store — manages all known peer records
pub struct TrustStore {
    peers: Arc<RwLock<HashMap<String, PeerRecord>>>,
    our_trust_scores: Arc<RwLock<HashMap<String, f32>>>,
}

impl TrustStore {
    /// Create a new empty trust store
    pub fn new() -> Self {
        Self {
            peers: Arc::new(RwLock::new(HashMap::new())),
            our_trust_scores: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Called when we first encounter a peer's public key (TOFU).
    ///
    /// Returns `Ok(true)` if this is a new peer, `Ok(false)` if key matches.
    /// Returns `Err` if key has changed for a known peer.
    pub async fn verify_or_pin(&self, peer_id: &str, public_key: &[u8]) -> TrustResult<bool> {
        let mut peers = self.peers.write().await;

        if let Some(record) = peers.get_mut(peer_id) {
            record.current_key = public_key.to_vec();
            record.last_seen = now_secs();
            record.interaction_count += 1;

            if record.pinned_key != public_key {
                record.trust_level = TrustLevel::KeyChanged;
                return Err(TrustError::KeyChanged {
                    peer_id: peer_id.to_string(),
                });
            }

            if record.trust_level == TrustLevel::FirstContact {
                record.trust_level = TrustLevel::Trusted;
            }

            Ok(false)
        } else {
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
    pub async fn add_attestation(&self, attestation: Attestation) -> TrustResult<()> {
        self.verify_attestation_signature(&attestation)
            .await
            .map_err(TrustError::AttestationFailed)?;

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

        if !record
            .attestations
            .iter()
            .any(|a| a.attester_id == attestation.attester_id)
        {
            record.attestations.push(attestation);

            if record.trust_level == TrustLevel::Unknown
                || record.trust_level == TrustLevel::FirstContact
            {
                record.trust_level = TrustLevel::Vouched;
            }
        }

        Ok(())
    }

    /// Upgrade a peer to Verified status (after out-of-band verification)
    pub async fn mark_verified(&self, peer_id: &str, verified_key: &[u8]) -> TrustResult<()> {
        let mut peers = self.peers.write().await;
        let record = peers
            .get_mut(peer_id)
            .ok_or_else(|| TrustError::UnknownPeer {
                peer_id: peer_id.to_string(),
            })?;

        if record.pinned_key != verified_key {
            return Err(TrustError::KeyMismatch);
        }

        record.trust_level = TrustLevel::Verified;
        Ok(())
    }

    /// Block a peer
    pub async fn block(&self, peer_id: &str) {
        let mut peers = self.peers.write().await;
        if let Some(record) = peers.get_mut(peer_id) {
            record.trust_level = TrustLevel::Blocked;
        }
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

    /// Set our trust score for a peer
    pub async fn set_attester_trust(&self, peer_id: String, score: f32) {
        self.our_trust_scores
            .write()
            .await
            .insert(peer_id, score.clamp(0.0, 1.0));
    }

    /// Count of blocked peers
    pub async fn blocked_count(&self) -> usize {
        self.peers
            .read()
            .await
            .values()
            .filter(|r| r.trust_level == TrustLevel::Blocked)
            .count()
    }

    /// Average trust score across all known peers
    pub async fn average_trust_score(&self) -> f32 {
        let peers = self.peers.read().await;
        if peers.is_empty() {
            return 0.0;
        }
        let sum: f32 = peers.values().map(|r| r.trust_level.score()).sum();
        sum / peers.len() as f32
    }

    async fn verify_attestation_signature(&self, att: &Attestation) -> Result<(), String> {
        use ring::signature::{UnparsedPublicKey, ED25519};

        if att.signature.is_empty() {
            return Err(format!(
                "Attestation from {} has empty signature",
                att.attester_id
            ));
        }
        if att.subject_pub_key.is_empty() {
            return Err(format!(
                "Attestation from {} has empty subject public key",
                att.attester_id
            ));
        }

        let attester_pub_key = {
            let peers = self.peers.read().await;
            peers
                .get(&att.attester_id)
                .map(|r| r.current_key.clone())
                .ok_or_else(|| {
                    format!(
                        "Cannot verify attestation from unknown peer: {}",
                        att.attester_id
                    )
                })?
        };

        let payload = [
            att.subject_id.as_bytes(),
            &att.subject_pub_key,
            &att.timestamp.to_le_bytes(),
        ]
        .concat();

        let pub_key = UnparsedPublicKey::new(&ED25519, &attester_pub_key);
        pub_key.verify(&payload, &att.signature).map_err(|e| {
            format!(
                "Attestation signature verification failed for {} → {}: {}",
                att.attester_id, att.subject_id, e
            )
        })
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
}
