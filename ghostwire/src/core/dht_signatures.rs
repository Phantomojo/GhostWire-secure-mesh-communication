//! dht_signatures.rs — Ed25519 signatures on DHT records
//!
//! Without this, any Sybil node that passes the PoW check can store
//! arbitrary values in the DHT. Someone could poison routing entries,
//! inject fake rendezvous addresses, or corrupt the knowledge ark index.
//!
//! This module adds:
//!   1. Signed DHT records: every value stored in the DHT is signed
//!      by the storing node's Ed25519 identity key
//!   2. Verification on retrieval: nodes reject unsigned or
//!      incorrectly-signed values before using them
//!   3. Record freshness: a timestamp in the signed content prevents
//!      replay of old records

use anyhow::Result;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// A signed DHT record — the unit stored and retrieved from the DHT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedDhtRecord {
    /// The key under which this record is stored (32 bytes)
    pub dht_key: [u8; 32],
    /// The actual value bytes
    pub value: Vec<u8>,
    /// The storing node's Ed25519 public key (32 bytes)
    pub publisher_pubkey: [u8; 32],
    /// Unix timestamp when this record was created
    pub timestamp: u64,
    /// Sequence number — higher wins when two records conflict
    pub sequence: u64,
    /// TTL in seconds (default: 24 hours)
    pub ttl_secs: u64,
    /// Ed25519 signature over (dht_key || value || publisher_pubkey || timestamp || sequence)
    pub signature: Vec<u8>,
}

impl SignedDhtRecord {
    /// Create and sign a new DHT record
    pub fn new(
        dht_key: [u8; 32],
        value: Vec<u8>,
        signing_key: &SigningKey,
        sequence: u64,
        ttl_secs: u64,
    ) -> Result<Self> {
        let publisher_pubkey = signing_key.verifying_key().to_bytes();
        let timestamp = now_secs();

        let to_sign =
            Self::signable_bytes(&dht_key, &value, &publisher_pubkey, timestamp, sequence);
        let signature = signing_key.sign(&to_sign).to_bytes().to_vec();

        Ok(Self {
            dht_key,
            value,
            publisher_pubkey,
            timestamp,
            sequence,
            ttl_secs,
            signature,
        })
    }

    /// Verify this record's signature and freshness
    pub fn verify(&self) -> Result<()> {
        // Check expiry
        let age_secs = now_secs().saturating_sub(self.timestamp);
        if age_secs > self.ttl_secs {
            anyhow::bail!(
                "DHT record expired: age={}s ttl={}s",
                age_secs,
                self.ttl_secs
            );
        }

        // Reject records from the future (clock skew tolerance: 5 minutes)
        if self.timestamp > now_secs() + 300 {
            anyhow::bail!("DHT record timestamp is in the future — possible replay attack");
        }

        // Verify signature
        let pubkey = VerifyingKey::from_bytes(&self.publisher_pubkey)
            .map_err(|e| anyhow::anyhow!("Invalid publisher key: {}", e))?;

        let to_verify = Self::signable_bytes(
            &self.dht_key,
            &self.value,
            &self.publisher_pubkey,
            self.timestamp,
            self.sequence,
        );

        let sig = Signature::from_slice(&self.signature)
            .map_err(|e| anyhow::anyhow!("Invalid signature bytes: {}", e))?;

        pubkey.verify(&to_verify, &sig).map_err(|_| {
            anyhow::anyhow!("DHT record signature verification failed — poisoned record?")
        })
    }

    /// Whether this record is fresher than another (for conflict resolution)
    pub fn is_fresher_than(&self, other: &Self) -> bool {
        // Higher sequence wins; if equal, newer timestamp wins
        self.sequence > other.sequence
            || (self.sequence == other.sequence && self.timestamp > other.timestamp)
    }

    fn signable_bytes(
        dht_key: &[u8; 32],
        value: &[u8],
        publisher_pubkey: &[u8; 32],
        timestamp: u64,
        sequence: u64,
    ) -> Vec<u8> {
        let mut data = Vec::with_capacity(32 + value.len() + 32 + 8 + 8 + 32);
        data.extend_from_slice(crate::genesis_constants::DHT_GENESIS_HASH);
        data.extend_from_slice(dht_key);
        data.extend_from_slice(value);
        data.extend_from_slice(publisher_pubkey);
        data.extend_from_slice(&timestamp.to_le_bytes());
        data.extend_from_slice(&sequence.to_le_bytes());
        data
    }

    pub fn is_expired(&self) -> bool {
        now_secs().saturating_sub(self.timestamp) > self.ttl_secs
    }
}

/// DHT record verifier — use this in the Kademlia put/get handlers
pub struct DhtRecordVerifier;

impl DhtRecordVerifier {
    /// Verify a record retrieved from the DHT before using it.
    /// Returns Ok(record) if valid, Err if poisoned/expired/tampered.
    pub fn verify_retrieved(record: &SignedDhtRecord) -> Result<()> {
        record.verify()
    }

    /// Check if a new record should replace an existing one
    pub fn should_replace(existing: &SignedDhtRecord, candidate: &SignedDhtRecord) -> bool {
        // First: both must be valid
        if candidate.verify().is_err() {
            return false; // Never replace with an invalid record
        }
        if existing.is_expired() {
            return true; // Always replace expired records
        }
        // Same publisher: accept if fresher
        if existing.publisher_pubkey == candidate.publisher_pubkey {
            return candidate.is_fresher_than(existing);
        }
        // Different publishers: keep the one with higher sequence
        // (handles key rotation — new key can take over with sequence bump)
        candidate.sequence > existing.sequence
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
    use rand_core::OsRng;

    fn make_signing_key() -> SigningKey {
        SigningKey::generate(&mut OsRng)
    }

    #[test]
    fn sign_and_verify_roundtrip() {
        let key = make_signing_key();
        let record =
            SignedDhtRecord::new([0x01u8; 32], b"test value".to_vec(), &key, 1, 3600).unwrap();
        assert!(record.verify().is_ok());
    }

    #[test]
    fn tampered_value_fails_verification() {
        let key = make_signing_key();
        let mut record =
            SignedDhtRecord::new([0x02u8; 32], b"original".to_vec(), &key, 1, 3600).unwrap();
        record.value = b"tampered".to_vec();
        assert!(record.verify().is_err());
    }

    #[test]
    fn freshness_comparison() {
        let key = make_signing_key();
        let r1 = SignedDhtRecord::new([0x03u8; 32], b"v1".to_vec(), &key, 1, 3600).unwrap();
        let r2 = SignedDhtRecord::new([0x03u8; 32], b"v2".to_vec(), &key, 2, 3600).unwrap();
        assert!(r2.is_fresher_than(&r1));
        assert!(!r1.is_fresher_than(&r2));
    }

    #[test]
    fn should_replace_logic() {
        let key = make_signing_key();
        let old = SignedDhtRecord::new([0x04u8; 32], b"old".to_vec(), &key, 1, 3600).unwrap();
        let new = SignedDhtRecord::new([0x04u8; 32], b"new".to_vec(), &key, 2, 3600).unwrap();
        assert!(DhtRecordVerifier::should_replace(&old, &new));
        assert!(!DhtRecordVerifier::should_replace(&new, &old));
    }
}
