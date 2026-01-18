use crate::core::encryption::EncryptionManager;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sled::{Db, Tree};
use tracing;

/// Persistent database manager for GhostWire.
/// Manages encrypted storage for messages, peer keys, and profile metadata.
pub struct PersistenceManager {
    db: Db,
    _profile_name: Option<String>, // Renamed to _profile_name to mark as intentionally unused
}

impl PersistenceManager {
    pub fn new(profile_name: Option<String>) -> Result<Self> {
        let home =
            dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        let base_dir = home.join(".ghostwire").join("vault");

        if !base_dir.exists() {
            std::fs::create_dir_all(&base_dir)?;
        }

        let db_path = if let Some(p) = &profile_name {
            base_dir.join(format!("ghost_{}", p))
        } else {
            base_dir.join("ghost_default")
        };

        let db = sled::open(db_path)?;

        Ok(Self {
            db,
            _profile_name: profile_name,
        })
    }

    /// Access a specific 'tree' (bucket) in the database
    fn get_tree(&self, name: &str) -> Result<Tree> {
        self.db.open_tree(name).map_err(Into::into)
    }

    /// Store an encrypted object.
    /// Object is serialized to JSON, then encrypted using EncryptionManager primitives, then stored.
    pub fn store_secure<T: Serialize>(&self, tree_name: &str, key: &str, data: &T) -> Result<()> {
        let tree = self.get_tree(tree_name)?;

        // 1. Serialize
        let json = serde_json::to_vec(data)?;

        // 2. Encrypt
        // We use the static Master Key encryption primitive from EncryptionManager
        // For per-entry encryption, we use a deterministic nonce derivation or random nonce.
        // PersistenceManager leverages EncryptionManager::secure_save style logic.

        // Temporary: Using standard hex encoding until we unify the encryption buffers
        // Actually, we'll store binary blobs.

        // To keep it robust, we'll use a specialized secure wrapper for sled
        let encrypted = self.encrypt_payload(&json)?;

        tree.insert(key, encrypted)?;
        Ok(())
    }

    /// Retrieve and decrypt an object
    pub fn load_secure<T: for<'de> Deserialize<'de>>(
        &self,
        tree_name: &str,
        key: &str,
    ) -> Result<Option<T>> {
        let tree = self.get_tree(tree_name)?;

        if let Some(val) = tree.get(key)? {
            let decrypted = self.decrypt_payload(&val)?;
            let data = serde_json::from_slice(&decrypted)?;
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }

    /// List all keys in a tree
    pub fn list_keys(&self, tree_name: &str) -> Result<Vec<String>> {
        let tree = self.get_tree(tree_name)?;
        let mut keys = Vec::new();
        for item in tree.iter() {
            let (k, _) = item?;
            keys.push(String::from_utf8(k.to_vec())?);
        }
        Ok(keys)
    }

    /// Batch load items from a tree
    pub fn load_all_secure<T: for<'de> Deserialize<'de>>(&self, tree_name: &str) -> Result<Vec<T>> {
        let tree = self.get_tree(tree_name)?;
        let mut results = Vec::new();
        for item in tree.iter() {
            let (_, v) = item?;
            match self.decrypt_payload(&v) {
                Ok(decrypted) => {
                    if let Ok(data) = serde_json::from_slice(&decrypted) {
                        results.push(data);
                    }
                }
                Err(e) => tracing::error!("Failed to decrypt entry in {}: {}", tree_name, e),
            }
        }
        Ok(results)
    }

    /// Helper for encryption (wraps EncryptionManager logic)
    fn encrypt_payload(&self, data: &[u8]) -> Result<Vec<u8>> {
        let result = EncryptionManager::encrypt_bytes(data)
            .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

        // We store as [Nonce (12)] [Ciphertext] to match EncryptionManager's file format
        let mut combined = result.nonce;
        combined.extend_from_slice(&result.ciphertext);
        Ok(combined)
    }

    fn decrypt_payload(&self, data: &[u8]) -> Result<Vec<u8>> {
        if data.len() < 12 {
            return Err(anyhow::anyhow!("Data too short for decryption"));
        }

        let nonce = &data[0..12];
        let ciphertext = &data[12..];

        let decrypted = EncryptionManager::decrypt_bytes(ciphertext, nonce)
            .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

        Ok(decrypted)
    }

    pub fn flush(&self) -> Result<()> {
        self.db.flush()?;
        Ok(())
    }
}
