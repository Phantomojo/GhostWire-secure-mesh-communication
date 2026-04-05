use crate::core::encryption::EncryptionManager;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sled::{Db, Tree};

/// Persistent database manager for GhostWire.
/// Manages encrypted storage for messages, peer keys, and profile metadata.
pub struct PersistenceManager {
    db: Db,
    _profile_name: Option<String>, // Renamed to _profile_name to mark as intentionally unused
}

impl PersistenceManager {
    pub fn new(profile_name: Option<String>) -> Result<Self> {
        let base_dir = if let Ok(ghost_home) = std::env::var("GHOSTWIRE_HOME") {
            std::path::PathBuf::from(ghost_home).join("vault")
        } else {
            let home =
                dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
            home.join(".ghostwire").join("vault")
        };

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
                Err(e) => {
                    let msg = format!("Failed to decrypt entry in {}: {}", tree_name, e);
                    tracing::error!("{}", msg);
                }
            }
        }
        Ok(results)
    }

    /// Helper for encryption (wraps EncryptionManager logic)
    fn encrypt_payload(&self, data: &[u8]) -> Result<Vec<u8>> {
        let result = EncryptionManager::encrypt_bytes(data)
            .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

        // Store as [Nonce (24 bytes for XChaCha20)] [Ciphertext]
        let mut combined = result.nonce;
        combined.extend_from_slice(&result.ciphertext);
        Ok(combined)
    }

    fn decrypt_payload(&self, data: &[u8]) -> Result<Vec<u8>> {
        if data.len() < 24 {
            return Err(anyhow::anyhow!("Data too short for decryption"));
        }

        let nonce = &data[0..24];
        let ciphertext = &data[24..];

        let decrypted = EncryptionManager::decrypt_bytes(ciphertext, nonce)
            .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

        Ok(decrypted)
    }

    pub fn flush(&self) -> Result<()> {
        self.db.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_persistence_roundtrip() {
        let dir = tempdir().unwrap();
        std::env::set_var("GHOSTWIRE_HOME", dir.path());

        let manager = PersistenceManager::new(None).expect("Failed to create PersistenceManager");

        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct TestData {
            name: String,
            value: u32,
        }

        let original = TestData {
            name: "test_item".to_string(),
            value: 42,
        };

        // Store
        manager
            .store_secure("test_tree", "test_key", &original)
            .expect("Failed to store");
        manager.flush().unwrap();

        // Load
        let loaded: TestData = manager
            .load_secure("test_tree", "test_key")
            .expect("Failed to load")
            .expect("Item not found");

        assert_eq!(original, loaded);
    }

    #[test]
    fn test_list_keys() {
        let dir = tempdir().unwrap();
        std::env::set_var("GHOSTWIRE_HOME", dir.path());

        let manager = PersistenceManager::new(Some("profile1".to_string())).unwrap();

        manager.store_secure("tree_a", "key1", &"val1").unwrap();
        manager.store_secure("tree_a", "key2", &"val2").unwrap();
        manager.store_secure("tree_b", "key3", &"val3").unwrap();

        let keys_a = manager.list_keys("tree_a").unwrap();
        assert_eq!(keys_a.len(), 2);
        assert!(keys_a.contains(&"key1".to_string()));
        assert!(keys_a.contains(&"key2".to_string()));

        let keys_b = manager.list_keys("tree_b").unwrap();
        assert_eq!(keys_b.len(), 1);
        assert!(keys_b.contains(&"key3".to_string()));
    }

    #[test]
    fn test_load_all() {
        let dir = tempdir().unwrap();
        std::env::set_var("GHOSTWIRE_HOME", dir.path());
        let manager = PersistenceManager::new(None).unwrap();

        manager.store_secure("items", "1", &10).unwrap();
        manager.store_secure("items", "2", &20).unwrap();
        manager.store_secure("items", "3", &30).unwrap();

        let all: Vec<i32> = manager.load_all_secure("items").unwrap();
        assert_eq!(all.len(), 3);
        assert!(all.contains(&10));
        assert!(all.contains(&20));
        assert!(all.contains(&30));
    }
}
