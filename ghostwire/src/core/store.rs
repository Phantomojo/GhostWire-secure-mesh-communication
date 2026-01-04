use crate::core::encryption::EncryptionManager;
use crate::core::message::Message;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Encrypted container for persisted messages
#[derive(Serialize, Deserialize, Default)]
struct EncryptedLedger {
    messages: Vec<Message>,
}

const MAX_MESSAGES: usize = 1000;

/// Sovereign storage for encrypted message history.
/// Uses the system's Master Key for zero-knowledge persistence.
pub struct SecureMessageStore {
    path: PathBuf,
    cache: Arc<Mutex<Vec<Message>>>,
}

impl SecureMessageStore {
    /// Initialize the store at ~/.ghostwire/messages.json
    pub fn new() -> Result<Self> {
        let home =
            dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        let ghost_dir = home.join(".ghostwire");
        if !ghost_dir.exists() {
            std::fs::create_dir_all(&ghost_dir)?;
        }
        let path = ghost_dir.join("messages.json");

        let store = SecureMessageStore {
            path: path.clone(),
            cache: Arc::new(Mutex::new(Vec::new())),
        };

        // Attempt to load existing history
        if path.exists() {
            if let Err(e) = store.load() {
                // Log but don't fail initialization
                eprintln!("⚠️ Warning: Failed to load message history: {}", e);
            }
        }

        Ok(store)
    }

    /// Persist a message to the encrypted ledger
    pub fn save_message(&self, message: &Message) -> Result<()> {
        let mut cache = self
            .cache
            .lock()
            .map_err(|_| anyhow::anyhow!("Storage mutex poisoned"))?;
        cache.push(message.clone());

        // Prune if exceeds limit
        if cache.len() > MAX_MESSAGES {
            let start = cache.len() - MAX_MESSAGES;
            *cache = cache[start..].to_vec();
        }

        let ledger = EncryptedLedger {
            messages: cache.clone(),
        };

        EncryptionManager::secure_save(&self.path, &ledger)
            .map_err(|e| anyhow::anyhow!("Encryption storage failed: {}", e))?;

        Ok(())
    }

    /// Load the ledger from disk and decrypt into cache
    pub fn load(&self) -> Result<Vec<Message>> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }

        let ledger: EncryptedLedger = EncryptionManager::secure_load(&self.path)
            .map_err(|e| anyhow::anyhow!("Decryption of history failed: {}", e))?;

        let mut cache = self
            .cache
            .lock()
            .map_err(|_| anyhow::anyhow!("Storage mutex poisoned"))?;
        *cache = ledger.messages.clone();

        Ok(ledger.messages)
    }

    /// Retrieve historical messages for a specific peer or broadcast
    pub fn get_history(&self, peer_id: &str, limit: usize) -> Vec<Message> {
        let cache = self.cache.lock().unwrap();
        cache
            .iter()
            .filter(|m| m.recipient == peer_id || m.sender == peer_id || peer_id == "all")
            .cloned()
            .rev() // Newest first
            .take(limit)
            .collect()
    }

    /// Delete all messages (Wipe command)
    pub fn clear(&self) -> Result<()> {
        let mut cache = self
            .cache
            .lock()
            .map_err(|_| anyhow::anyhow!("Storage mutex poisoned"))?;
        cache.clear();

        if self.path.exists() {
            std::fs::remove_file(&self.path)?;
        }
        Ok(())
    }
}
