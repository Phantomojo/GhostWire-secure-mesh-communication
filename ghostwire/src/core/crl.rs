// Copyright (C) 2025-2026 Michael Irungu
// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Certificate Revocation List (CRL) — Gossip-Based
//!
//! Decentralized key revocation via gossip propagation. No central authority.
//!
//! ## How it works
//!
//! 1. A node revokes a compromised key → signs a `RevocationEntry`
//! 2. Entry is broadcast via existing gossipsub mesh
//! 3. Receiving nodes verify the signature and merge into their local CRL
//! 4. Revoked keys are rejected at encrypt/decrypt boundaries
//!
//! ## Security properties
//!
//! - Entries are signed by the revoking node (non-repudiable)
//! - Timestamps prevent replay of stale revocations
//! - Monotonic `epoch` fail-closes gossip merge against rollback
//!   (rejects entries whose epoch is below the per-issuer high-water mark)
//! - Merge is idempotent — duplicate entries harmlessly ignored
//! - TTL on entries prevents unbounded growth (default: 30 days)

use dashmap::DashMap;

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::{debug, info, warn};

// ── Constants ─────────────────────────────────────────────────────────────────

/// Default TTL for revocation entries (30 days)
pub const DEFAULT_CRL_TTL: u64 = 30 * 24 * 3600;

/// How often to garbage-collect expired entries (1 hour)
pub const CRL_GC_INTERVAL: Duration = Duration::from_secs(3600);

// ── Types ─────────────────────────────────────────────────────────────────────

/// A single revocation entry — signed by the revoking node.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RevocationEntry {
    /// Key ID being revoked
    pub key_id: String,
    /// Node ID of the revoking party
    pub revoked_by: String,
    /// Unix timestamp when revocation was issued
    pub timestamp: u64,
    /// Monotonic issuer epoch (bumped on revoke and on key rotation).
    /// Reject on merge when `epoch` < high-water for `revoked_by` (anti-rollback).
    #[serde(default)]
    pub epoch: u64,
    /// Signature over (key_id + revoked_by + timestamp + epoch) by revoked_by's key
    pub signature: Vec<u8>,
}

/// CRL event for gossip broadcast.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrlGossipPayload {
    /// The revocation entry
    pub entry: RevocationEntry,
    /// Version counter (monotonically increasing per node)
    pub version: u64,
}

/// Result of a merge operation.
#[derive(Debug, Clone, PartialEq)]
pub enum MergeResult {
    /// Entry was new and accepted
    Accepted,
    /// Entry was already known (duplicate)
    Duplicate,
    /// Entry signature verification failed
    InvalidSignature,
    /// Entry timestamp is too old (expired)
    Expired,
    /// Entry epoch is below the per-issuer high-water mark (rollback/stale)
    StaleEpoch,
}

// ── CrlStore ──────────────────────────────────────────────────────────────────

/// Thread-safe CRL store. O(1) lookup, gossip-mergeable.
pub struct CrlStore {
    /// key_id → RevocationEntry
    entries: DashMap<String, RevocationEntry>,
    /// Per-node version tracking (for gossip dedup)
    _peer_versions: DashMap<String, u64>,
    /// Per-issuer anti-rollback high-water: revoked_by → highest accepted epoch
    peer_epochs: DashMap<String, u64>,
    /// Local monotonic epoch (bumped on revoke and key rotation)
    epoch: AtomicU64,
    /// TTL for entries
    entry_ttl: u64,
}

impl CrlStore {
    pub fn new() -> Self {
        Self {
            entries: DashMap::new(),
            _peer_versions: DashMap::new(),
            peer_epochs: DashMap::new(),
            epoch: AtomicU64::new(0),
            entry_ttl: DEFAULT_CRL_TTL,
        }
    }

    pub fn with_ttl(ttl: u64) -> Self {
        Self {
            entry_ttl: ttl,
            ..Self::new()
        }
    }

    /// Check if a key is revoked. O(1).
    pub fn is_revoked(&self, key_id: &str) -> bool {
        self.entries.contains_key(key_id)
    }

    /// Get the revocation entry for a key, if any.
    pub fn get(&self, key_id: &str) -> Option<RevocationEntry> {
        self.entries.get(key_id).map(|e| e.clone())
    }

    /// Local monotonic epoch (current value; revoke/rotation call `bump_epoch`).
    pub fn epoch(&self) -> u64 {
        self.epoch.load(Ordering::Acquire)
    }

    /// Bump the local CRL epoch (called on key rotation so rotated keys
    /// cannot be rolled back to pre-rotation revocation state via gossip).
    pub fn bump_epoch(&self) -> u64 {
        self.epoch.fetch_add(1, Ordering::AcqRel) + 1
    }

    /// Revoke a key locally (self-revocation or admin revocation).
    /// Signs the entry with the provided signer. Bumps the local epoch.
    pub fn revoke(
        &self,
        key_id: &str,
        signer_id: &str,
        sign_fn: &dyn Fn(&[u8]) -> Vec<u8>,
    ) -> RevocationEntry {
        let timestamp = now_secs();
        let epoch = self.bump_epoch();

        // Sign: key_id + revoked_by + timestamp + epoch
        let mut payload = Vec::new();
        payload.extend_from_slice(key_id.as_bytes());
        payload.extend_from_slice(signer_id.as_bytes());
        payload.extend_from_slice(&timestamp.to_le_bytes());
        payload.extend_from_slice(&epoch.to_le_bytes());
        let signature = sign_fn(&payload);

        let entry = RevocationEntry {
            key_id: key_id.to_string(),
            revoked_by: signer_id.to_string(),
            timestamp,
            epoch,
            signature,
        };

        info!(key_id = %key_id, revoked_by = %signer_id, epoch, "Key revoked");
        self.raise_peer_epoch(signer_id, epoch);
        self.entries.insert(key_id.to_string(), entry.clone());
        entry
    }

    /// Merge a remote revocation entry (from gossip).
    /// Verifies signature before accepting.
    pub fn merge(
        &self,
        entry: &RevocationEntry,
        verify_fn: &dyn Fn(&[u8], &[u8], &str) -> bool,
    ) -> MergeResult {
        // Check expiry
        let age = now_secs().saturating_sub(entry.timestamp);
        if age > self.entry_ttl {
            debug!(key_id = %entry.key_id, age = age, "Revocation expired, skipping");
            return MergeResult::Expired;
        }

        // Check duplicate
        if self.entries.contains_key(&entry.key_id) {
            return MergeResult::Duplicate;
        }

        // Fail closed on epoch rollback (per-issuer high-water)
        if let Some(high) = self.peer_epochs.get(&entry.revoked_by) {
            if entry.epoch < *high {
                warn!(
                    key_id = %entry.key_id,
                    revoked_by = %entry.revoked_by,
                    entry_epoch = entry.epoch,
                    high_water = *high,
                    "Stale revocation epoch rejected"
                );
                return MergeResult::StaleEpoch;
            }
        }

        // Verify signature (covers epoch so peers cannot inflate high-water)
        let mut payload = Vec::new();
        payload.extend_from_slice(entry.key_id.as_bytes());
        payload.extend_from_slice(entry.revoked_by.as_bytes());
        payload.extend_from_slice(&entry.timestamp.to_le_bytes());
        payload.extend_from_slice(&entry.epoch.to_le_bytes());

        if !verify_fn(&payload, &entry.signature, &entry.revoked_by) {
            warn!(
                key_id = %entry.key_id,
                revoked_by = %entry.revoked_by,
                "Revocation signature invalid"
            );
            return MergeResult::InvalidSignature;
        }

        info!(
            key_id = %entry.key_id,
            revoked_by = %entry.revoked_by,
            epoch = entry.epoch,
            "Merged remote revocation"
        );
        self.raise_peer_epoch(&entry.revoked_by, entry.epoch);
        self.entries.insert(entry.key_id.clone(), entry.clone());
        MergeResult::Accepted
    }

    /// Raise the per-issuer high-water mark (no-op if already higher).
    fn raise_peer_epoch(&self, issuer: &str, epoch: u64) {
        self.peer_epochs
            .entry(issuer.to_string())
            .and_modify(|h| {
                if epoch > *h {
                    *h = epoch;
                }
            })
            .or_insert(epoch);
    }

    /// Garbage-collect expired entries. Call periodically.
    pub fn gc(&self) -> usize {
        let cutoff = now_secs().saturating_sub(self.entry_ttl);
        let mut removed = 0;
        self.entries.retain(|_k, entry| {
            if entry.timestamp < cutoff {
                removed += 1;
                false
            } else {
                true
            }
        });
        if removed > 0 {
            debug!(removed = removed, "CRL GC completed");
        }
        removed
    }

    /// Number of active revocations.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Iterate all entries (for serialization / broadcast).
    pub fn entries(&self) -> Vec<RevocationEntry> {
        self.entries.iter().map(|e| e.value().clone()).collect()
    }
}

impl Default for CrlStore {
    fn default() -> Self {
        Self::new()
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    /// Dummy sign function for testing (returns fixed signature).
    fn dummy_sign(_data: &[u8]) -> Vec<u8> {
        vec![0xAB, 0xCD, 0xEF]
    }

    /// Dummy verify function — always returns true (for testing merge logic).
    fn dummy_verify(_payload: &[u8], _sig: &[u8], _signer: &str) -> bool {
        true
    }

    /// Verify function that rejects specific signer.
    fn reject_verify(_payload: &[u8], _sig: &[u8], signer: &str) -> bool {
        signer != "bad_node"
    }

    #[test]
    fn test_revoke_and_check() {
        let store = CrlStore::new();
        assert!(!store.is_revoked("key_abc"));

        store.revoke("key_abc", "node_1", &dummy_sign);
        assert!(store.is_revoked("key_abc"));
        assert!(!store.is_revoked("key_xyz"));
    }

    #[test]
    fn test_get_entry() {
        let store = CrlStore::new();
        assert!(store.get("key_abc").is_none());

        let entry = store.revoke("key_abc", "node_1", &dummy_sign);
        let retrieved = store.get("key_abc").unwrap();
        assert_eq!(retrieved.key_id, entry.key_id);
        assert_eq!(retrieved.revoked_by, "node_1");
    }

    #[test]
    fn test_merge_new_entry() {
        let store = CrlStore::new();
        let entry = RevocationEntry {
            key_id: "key_abc".to_string(),
            revoked_by: "node_1".to_string(),
            timestamp: now_secs(),
            epoch: 0,
            signature: vec![0xAB],
        };

        assert_eq!(store.merge(&entry, &dummy_verify), MergeResult::Accepted);
        assert!(store.is_revoked("key_abc"));
    }

    #[test]
    fn test_merge_duplicate() {
        let store = CrlStore::new();
        let entry = RevocationEntry {
            key_id: "key_abc".to_string(),
            revoked_by: "node_1".to_string(),
            timestamp: now_secs(),
            epoch: 0,
            signature: vec![0xAB],
        };

        store.merge(&entry, &dummy_verify);
        assert_eq!(store.merge(&entry, &dummy_verify), MergeResult::Duplicate);
    }

    #[test]
    fn test_merge_expired() {
        let store = CrlStore::with_ttl(3600); // 1 hour TTL
        let entry = RevocationEntry {
            key_id: "key_abc".to_string(),
            revoked_by: "node_1".to_string(),
            timestamp: now_secs() - 7200, // 2 hours ago
            epoch: 0,
            signature: vec![0xAB],
        };

        assert_eq!(store.merge(&entry, &dummy_verify), MergeResult::Expired);
        assert!(!store.is_revoked("key_abc"));
    }

    #[test]
    fn test_merge_invalid_signature() {
        let store = CrlStore::new();
        let entry = RevocationEntry {
            key_id: "key_abc".to_string(),
            revoked_by: "bad_node".to_string(),
            timestamp: now_secs(),
            epoch: 0,
            signature: vec![0xAB],
        };

        assert_eq!(
            store.merge(&entry, &reject_verify),
            MergeResult::InvalidSignature
        );
        assert!(!store.is_revoked("key_abc"));
    }

    #[test]
    fn test_gc_removes_expired() {
        let store = CrlStore::with_ttl(3600);

        // Add fresh entry
        store.revoke("key_fresh", "node_1", &dummy_sign);

        // Manually insert expired entry
        let expired = RevocationEntry {
            key_id: "key_expired".to_string(),
            revoked_by: "node_1".to_string(),
            timestamp: now_secs() - 7200,
            epoch: 0,
            signature: vec![0xAB],
        };
        store.entries.insert("key_expired".to_string(), expired);

        assert_eq!(store.len(), 2);
        let removed = store.gc();
        assert_eq!(removed, 1);
        assert_eq!(store.len(), 1);
        assert!(store.is_revoked("key_fresh"));
        assert!(!store.is_revoked("key_expired"));
    }

    #[test]
    fn test_empty_store() {
        let store = CrlStore::new();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
        assert_eq!(store.gc(), 0);
    }

    #[test]
    fn test_revoke_bumps_epoch() {
        let store = CrlStore::new();
        assert_eq!(store.epoch(), 0);
        let e1 = store.revoke("key_a", "node_1", &dummy_sign);
        assert_eq!(e1.epoch, 1);
        assert_eq!(store.epoch(), 1);
        let e2 = store.revoke("key_b", "node_1", &dummy_sign);
        assert_eq!(e2.epoch, 2);
        assert_eq!(store.epoch(), 2);
    }

    #[test]
    fn test_bump_epoch() {
        let store = CrlStore::new();
        assert_eq!(store.bump_epoch(), 1);
        assert_eq!(store.bump_epoch(), 2);
        assert_eq!(store.epoch(), 2);
    }

    #[test]
    fn test_merge_rejects_stale_epoch() {
        let store = CrlStore::new();
        let fresh = RevocationEntry {
            key_id: "key_new".to_string(),
            revoked_by: "node_1".to_string(),
            timestamp: now_secs(),
            epoch: 5,
            signature: vec![0xAB],
        };
        assert_eq!(store.merge(&fresh, &dummy_verify), MergeResult::Accepted);

        let stale = RevocationEntry {
            key_id: "key_old".to_string(),
            revoked_by: "node_1".to_string(),
            timestamp: now_secs(),
            epoch: 3,
            signature: vec![0xAB],
        };
        assert_eq!(store.merge(&stale, &dummy_verify), MergeResult::StaleEpoch);
        assert!(!store.is_revoked("key_old"));
    }

    #[test]
    fn test_merge_accepts_forward_epoch() {
        let store = CrlStore::new();
        let e1 = RevocationEntry {
            key_id: "key_a".to_string(),
            revoked_by: "node_1".to_string(),
            timestamp: now_secs(),
            epoch: 2,
            signature: vec![0xAB],
        };
        assert_eq!(store.merge(&e1, &dummy_verify), MergeResult::Accepted);

        let e2 = RevocationEntry {
            key_id: "key_b".to_string(),
            revoked_by: "node_1".to_string(),
            timestamp: now_secs(),
            epoch: 7,
            signature: vec![0xAB],
        };
        assert_eq!(store.merge(&e2, &dummy_verify), MergeResult::Accepted);
        assert!(store.is_revoked("key_b"));
    }

    #[test]
    fn test_merge_equal_epoch_ok() {
        let store = CrlStore::new();
        let e1 = RevocationEntry {
            key_id: "key_a".to_string(),
            revoked_by: "node_1".to_string(),
            timestamp: now_secs(),
            epoch: 4,
            signature: vec![0xAB],
        };
        assert_eq!(store.merge(&e1, &dummy_verify), MergeResult::Accepted);

        let e2 = RevocationEntry {
            key_id: "key_b".to_string(),
            revoked_by: "node_1".to_string(),
            timestamp: now_secs(),
            epoch: 4,
            signature: vec![0xAB],
        };
        assert_eq!(store.merge(&e2, &dummy_verify), MergeResult::Accepted);
    }

    #[test]
    fn test_peer_epoch_isolated_per_issuer() {
        let store = CrlStore::new();
        let from_a = RevocationEntry {
            key_id: "key_a".to_string(),
            revoked_by: "node_a".to_string(),
            timestamp: now_secs(),
            epoch: 10,
            signature: vec![0xAB],
        };
        assert_eq!(store.merge(&from_a, &dummy_verify), MergeResult::Accepted);

        let from_b = RevocationEntry {
            key_id: "key_b".to_string(),
            revoked_by: "node_b".to_string(),
            timestamp: now_secs(),
            epoch: 1,
            signature: vec![0xAB],
        };
        assert_eq!(store.merge(&from_b, &dummy_verify), MergeResult::Accepted);
    }
}
