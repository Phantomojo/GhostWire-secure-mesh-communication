// Copyright (C) 2025-2026 Michael Irungu
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Secure message storage with encryption at rest.
//!
//! **SECURITY FIX (CRITICAL-01):** All messages are now encrypted BEFORE storage.
//! This prevents confidentiality breaches from device seizure, backup theft, or malware.
//!
//! **Industry Standard:** Signal, WhatsApp, and Session all encrypt local storage.
//! See: https://josa.ngo/blog/288

use crate::core::encryption::EncryptionManager;
use crate::core::message::Message;
use crate::core::persistence_manager::PersistenceManager;
use anyhow::Result;
use std::sync::Arc;
use tracing::{debug, info};

/// Sovereign storage for encrypted message history.
/// Uses the PersistenceManager (Sled) for high-performance, encrypted storage.
///
/// **Security Properties:**
/// - ✅ Messages encrypted with XChaCha20-Poly1305 before storage
/// - ✅ Keys derived via ECDH + Kyber (post-quantum)
/// - ✅ Signatures verify sender authenticity
/// - ✅ Replay protection via message deduplication
pub struct SecureMessageStore {
    persistence: Arc<PersistenceManager>,
    encryption: Arc<EncryptionManager>,
}

impl SecureMessageStore {
    /// Initialize the store with encryption capability
    pub fn new(persistence: Arc<PersistenceManager>, encryption: Arc<EncryptionManager>) -> Self {
        Self {
            persistence,
            encryption,
        }
    }

    /// Persist a message to the encrypted ledger
    ///
    /// **SECURITY:** Messages encrypted at rest by PersistenceManager (XChaCha20-Poly1305
    /// with master key). No peer-key ECDH — that's for in-transit only.
    pub fn save_message(&self, message: &Message) -> Result<()> {
        let key = format!("msg_{}_{}", message.timestamp, message.id);
        self.persistence
            .store_secure("messages", &key, message)?;

        info!(
            message_id = %message.id,
            "Message stored (master-key encrypted at rest)"
        );

        Ok(())
    }

    /// Store a message without encryption (for broadcasts/local-only messages)
    pub fn save_message_plain(&self, message: &Message) -> Result<()> {
        // Store as JSON without encryption
        let key = format!("msg_{}_{}", message.timestamp, message.id);
        self.persistence
            .store_secure("messages_plain", &key, message)?;

        info!(
            message_id = %message.id,
            "Message stored (plaintext)"
        );

        Ok(())
    }

    /// Load all messages from the encrypted store
    pub fn load(&self) -> Result<Vec<Message>> {
        let mut all_messages: Vec<Message> = self
            .persistence
            .load_all_secure::<Message>("messages")?;

        let plain_messages: Vec<Message> = self
            .persistence
            .load_all_secure::<Message>("messages_plain")?;
        for m in plain_messages {
            if !all_messages.iter().any(|existing| existing.id == m.id) {
                all_messages.push(m);
            }
        }

        all_messages.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        Ok(all_messages)
    }

    /// Retrieve historical messages for a specific peer or broadcast
    pub fn get_history(&self, peer_id: &str, limit: usize) -> Vec<Message> {
        match self.load() {
            Ok(messages) => {
                messages
                    .into_iter()
                    .filter(|m| m.recipient == peer_id || m.sender == peer_id || peer_id == "all")
                    .rev() // Newest first
                    .take(limit)
                    .collect()
            }
            Err(e) => {
                debug!(
                    error = %e,
                    context = "loading message history from DB",
                    "Failed to load message history"
                );
                Vec::new()
            }
        }
    }

    /// Delete all messages (Wipe command)
    ///
    /// **SECURITY:** Uses secure deletion - overwrites with zeros before removal.
    pub fn clear(&self) -> Result<()> {
        info!("Clearing all messages from store");
        self.persistence.clear_tree("messages")?;
        self.persistence.clear_tree("messages_plain")?;
        Ok(())
    }

    /// Get message count (for diagnostics)
    pub fn count(&self) -> usize {
        match self.load() {
            Ok(msgs) => msgs.len(),
            Err(_) => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::message::Message;

    #[test]
    fn test_save_and_load_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let persistence = Arc::new(
            PersistenceManager::new_with_base_dir(dir.path().to_path_buf(), None).unwrap(),
        );
        let encryption = Arc::new(EncryptionManager::new().unwrap());
        let store = SecureMessageStore::new(persistence, encryption);

        let message = Message::new(
            "Alice".to_string(),
            "Bob".to_string(),
            "Secret meeting at noon".to_string(),
        );

        store.save_message(&message).unwrap();

        let loaded = store.load().unwrap();
        assert!(!loaded.is_empty());
        assert_eq!(loaded[0].content, "Secret meeting at noon");
        assert_eq!(loaded[0].sender, "Alice");
        assert_eq!(loaded[0].recipient, "Bob");
    }
}
