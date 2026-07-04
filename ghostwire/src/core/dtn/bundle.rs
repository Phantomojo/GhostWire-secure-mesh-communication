//! bundle.rs — DTN bundle format for GhostDTN
//!
//! Wraps existing WireMessage with DTN routing metadata:
//!   - copies_remaining: Spray-and-Wait copy count (u8, starts at L=6)
//!   - ttl_secs: time-to-live (default 72 hours)
//!   - delivery_score: PRoPHET probability estimate for next hop
//!   - cid: content hash (BLAKE3) for deduplication and bloom filter

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Default spray copies (L=6 from literature — optimal for urban mesh scenarios)
pub const SPRAY_L: u8 = 6;
/// Default TTL: 72 hours in seconds
pub const BUNDLE_TTL_SECS: u64 = 72 * 3600;

/// A DTN bundle wrapping a serialised WireMessage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DtnBundle {
    /// Content ID: BLAKE3 hash of the encrypted payload — for deduplication
    pub cid: [u8; 32],
    /// The actual encrypted message bytes (a serialised WireMessage)
    pub payload: Vec<u8>,
    /// Unix timestamp when this bundle was created
    pub created_at: u64,
    /// Unix timestamp when this bundle expires
    pub expires_at: u64,
    /// Destination node ID (32 bytes)
    pub destination: [u8; 32],
    /// Remaining spray copies.
    /// When > 1: split copies when relaying (Spray phase)
    /// When == 1: only deliver directly to destination (Wait phase)
    pub copies_remaining: u8,
    /// PRoPHET delivery probability to destination (0.0–1.0)
    /// Updated as we encounter nodes with better delivery paths
    pub delivery_probability: f32,
    /// How many times this bundle has been forwarded
    pub hop_count: u8,
}

impl DtnBundle {
    /// Create a new bundle from encrypted payload bytes
    pub fn new(payload: Vec<u8>, destination: [u8; 32]) -> Self {
        let cid = *blake3::hash(&payload).as_bytes();
        let now = now_secs();
        Self {
            cid,
            payload,
            created_at: now,
            expires_at: now + BUNDLE_TTL_SECS,
            destination,
            copies_remaining: SPRAY_L,
            delivery_probability: 0.0,
            hop_count: 0,
        }
    }

    /// Whether this bundle has expired
    pub fn is_expired(&self) -> bool {
        now_secs() > self.expires_at
    }

    /// Split copies for relay according to Spray-and-Wait:
    /// Give relay floor(copies/2), keep ceil(copies/2)
    /// Returns copies to give to the relay node
    pub fn split_copies(&mut self) -> u8 {
        if self.copies_remaining <= 1 {
            return 0; // Wait phase — no more spraying
        }
        let to_give = self.copies_remaining / 2;
        self.copies_remaining -= to_give;
        to_give
    }

    /// Create a relay copy with the given copy count
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

    /// Size in bytes (approximate)
    pub fn size_bytes(&self) -> usize {
        self.payload.len() + 80 // header overhead
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
    fn spray_and_wait_copy_split() {
        let mut bundle = DtnBundle::new(vec![1, 2, 3], [0u8; 32]);
        assert_eq!(bundle.copies_remaining, SPRAY_L); // 6

        let given = bundle.split_copies(); // give 3, keep 3
        assert_eq!(given, 3);
        assert_eq!(bundle.copies_remaining, 3);

        let given2 = bundle.split_copies(); // give 1, keep 2
        assert_eq!(given2, 1);
        assert_eq!(bundle.copies_remaining, 2);
    }

    #[test]
    fn wait_phase_gives_zero_copies() {
        let mut bundle = DtnBundle::new(vec![1, 2, 3], [0u8; 32]);
        bundle.copies_remaining = 1;
        assert_eq!(bundle.split_copies(), 0);
        assert_eq!(bundle.copies_remaining, 1); // unchanged
    }

    #[test]
    fn cid_is_deterministic() {
        let payload = b"test payload".to_vec();
        let dest = [0xABu8; 32];
        let b1 = DtnBundle::new(payload.clone(), dest);
        let b2 = DtnBundle::new(payload, dest);
        assert_eq!(b1.cid, b2.cid);
    }
}
