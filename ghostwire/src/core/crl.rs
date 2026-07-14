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
//! - Merge is idempotent — duplicate entries harmlessly ignored
//! - TTL on entries prevents unbounded growth (default: 30 days)

use dashmap::DashMap;

use serde::{Deserialize, Serialize};
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
    /// Signature over (key_id + revoked_by + timestamp) by revoked_by's key
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
}

// ── CrlStore ──────────────────────────────────────────────────────────────────

/// Thread-safe CRL store. O(1) lookup, gossip-mergeable.
pub struct CrlStore {
    /// key_id → RevocationEntry
    entries: DashMap<String, RevocationEntry>,
    /// Per-node version tracking (for gossip dedup)
    _peer_versions: DashMap<String, u64>,
    /// TTL for entries
    entry_ttl: u64,
}

impl CrlStore {
    pub fn new() -> Self {
        Self {
            entries: DashMap::new(),
            _peer_versions: DashMap::new(),
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

    /// Revoke a key locally (self-revocation or admin revocation).
    /// Signs the entry with the provided signer.
    pub fn revoke(
        &self,
        key_id: &str,
        signer_id: &str,
        sign_fn: &dyn Fn(&[u8]) -> Vec<u8>,
    ) -> RevocationEntry {
        let timestamp = now_secs();

        // Sign: key_id + revoked_by + timestamp
        let mut payload = Vec::new();
        payload.extend_from_slice(key_id.as_bytes());
        payload.extend_from_slice(signer_id.as_bytes());
        payload.extend_from_slice(&timestamp.to_le_bytes());
        let signature = sign_fn(&payload);

        let entry = RevocationEntry {
            key_id: key_id.to_string(),
            revoked_by: signer_id.to_string(),
            timestamp,
            signature,
        };

        info!(key_id = %key_id, revoked_by = %signer_id, "Key revoked");
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

        // Verify signature
        let mut payload = Vec::new();
        payload.extend_from_slice(entry.key_id.as_bytes());
        payload.extend_from_slice(entry.revoked_by.as_bytes());
        payload.extend_from_slice(&entry.timestamp.to_le_bytes());

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
            "Merged remote revocation"
        );
        self.entries.insert(entry.key_id.clone(), entry.clone());
        MergeResult::Accepted
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
}
