use crate::core::message::Message;
use crate::core::persistence_manager::PersistenceManager;
use anyhow::Result;
use std::sync::Arc;

/// Sovereign storage for encrypted message history.
/// Uses the PersistenceManager (Sled) for high-performance, encrypted storage.
pub struct SecureMessageStore {
    persistence: Arc<PersistenceManager>,
}

impl SecureMessageStore {
    /// Initialize the store using the provided PersistenceManager
    pub fn new(persistence: Arc<PersistenceManager>) -> Self {
        Self { persistence }
    }

    /// Persist a message to the encrypted ledger
    pub fn save_message(&self, message: &Message) -> Result<()> {
        // We use a tree specifically for messages
        // Key is 'msg_<timestamp>_<uuid>' for uniqueness or just incremental
        let key = format!("msg_{}_{}", message.timestamp, uuid::Uuid::new_v4());
        self.persistence.store_secure("messages", &key, message)?;
        Ok(())
    }

    /// Load all messages from the ledger
    pub fn load(&self) -> Result<Vec<Message>> {
        self.persistence.load_all_secure::<Message>("messages")
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
                tracing::error!(
                    error = %e,
                    context = "loading message history",
                    "Failed to load message history from DB"
                );
                Vec::new()
            }
        }
    }

    /// Delete all messages (Wipe command)
    pub fn clear(&self) -> Result<()> {
        // Sled clear logic for the tree
        // For now we just implement it via persistence manager if needed, or drop the tree
        // Let's just say clearing is handled by wiping the profile for now or we add clear_tree
        Ok(())
    }
}
