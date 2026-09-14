use crate::core::encryption::EncryptionManager;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sled::{Db, Tree};

const ENVELOPE_MAGIC: &[u8; 4] = b"GWPE";
const ENVELOPE_VERSION: u8 = 1;
const NONCE_LEN: usize = 24;
const AEAD_TAG_LEN: usize = 16;
const ENVELOPE_HEADER_LEN: usize = ENVELOPE_MAGIC.len() + 1 + NONCE_LEN;
const MAX_ENVELOPE_LEN: usize = 256 * 1024 * 1024;

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

        Self::new_with_base_dir(base_dir, profile_name)
    }

    /// Create a PersistenceManager with an explicit base directory (bypasses env var).
    /// Use this in tests to avoid GHOSTWIRE_HOME races.
    pub fn new_with_base_dir(
        base_dir: std::path::PathBuf,
        profile_name: Option<String>,
    ) -> Result<Self> {
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
        let encrypted = self.encrypt_payload(tree_name, key, &json)?;

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
            let decrypted = self.decrypt_payload(tree_name, key, &val)?;
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
            let (k, v) = item?;
            let key = String::from_utf8(k.to_vec())
                .map_err(|_| anyhow::anyhow!("Persistence key is not valid UTF-8"))?;
            let decrypted = self.decrypt_payload(tree_name, &key, &v)?;
            results.push(serde_json::from_slice(&decrypted)?);
        }
        Ok(results)
    }

    fn append_len_prefixed(buffer: &mut Vec<u8>, value: &str) -> Result<()> {
        let length = u32::try_from(value.len())
            .map_err(|_| anyhow::anyhow!("Persistence context is too large"))?;
        buffer.extend_from_slice(&length.to_be_bytes());
        buffer.extend_from_slice(value.as_bytes());
        Ok(())
    }

    fn envelope_aad(&self, tree_name: &str, key: &str) -> Result<Vec<u8>> {
        let mut aad = b"GhostWire.PersistenceManager".to_vec();
        aad.push(ENVELOPE_VERSION);
        Self::append_len_prefixed(&mut aad, self._profile_name.as_deref().unwrap_or("default"))?;
        Self::append_len_prefixed(&mut aad, tree_name)?;
        Self::append_len_prefixed(&mut aad, key)?;
        Ok(aad)
    }

    /// Helper for authenticated persistence envelopes.
    /// The active key source is the machine-protected master key. This is not
    /// yet the user-passphrase-wrapped keyslot required for portable unlock.
    fn encrypt_payload(&self, tree_name: &str, key: &str, data: &[u8]) -> Result<Vec<u8>> {
        let aad = self.envelope_aad(tree_name, key)?;
        let result = EncryptionManager::encrypt_bytes_with_aad(data, &aad)
            .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

        let total_len = ENVELOPE_HEADER_LEN
            .checked_add(result.ciphertext.len())
            .ok_or_else(|| anyhow::anyhow!("Persistence envelope length overflow"))?;
        if total_len > MAX_ENVELOPE_LEN {
            return Err(anyhow::anyhow!("Persistence envelope exceeds maximum size"));
        }

        let mut envelope = Vec::with_capacity(total_len);
        envelope.extend_from_slice(ENVELOPE_MAGIC);
        envelope.push(ENVELOPE_VERSION);
        envelope.extend_from_slice(&result.nonce);
        envelope.extend_from_slice(&result.ciphertext);
        Ok(envelope)
    }

    fn decrypt_payload(&self, tree_name: &str, key: &str, data: &[u8]) -> Result<Vec<u8>> {
        if data.len() < ENVELOPE_HEADER_LEN + AEAD_TAG_LEN {
            return Err(anyhow::anyhow!("Persistence envelope is too short"));
        }
        if data.len() > MAX_ENVELOPE_LEN {
            return Err(anyhow::anyhow!("Persistence envelope exceeds maximum size"));
        }
        if &data[..ENVELOPE_MAGIC.len()] != ENVELOPE_MAGIC {
            return Err(anyhow::anyhow!(
                "Unsupported or legacy persistence envelope; explicit migration required"
            ));
        }
        if data[ENVELOPE_MAGIC.len()] != ENVELOPE_VERSION {
            return Err(anyhow::anyhow!("Unsupported persistence envelope version"));
        }

        let nonce_start = ENVELOPE_MAGIC.len() + 1;
        let nonce_end = nonce_start + NONCE_LEN;
        let aad = self.envelope_aad(tree_name, key)?;
        EncryptionManager::decrypt_bytes_with_aad(
            &data[nonce_end..],
            &data[nonce_start..nonce_end],
            &aad,
        )
        .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))
    }

    pub fn flush(&self) -> Result<()> {
        self.db.flush()?;
        Ok(())
    }

    pub fn clear_tree(&self, tree_name: &str) -> Result<()> {
        let tree = self.get_tree(tree_name)?;
        tree.clear()?;
        self.db.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_user_passphrase_wraps_active_persistence_key() {
        let dir = tempdir().unwrap();
        let passphrase = "correct horse battery staple";
        let manager = PersistenceManager::new_with_base_dir_and_passphrase(
            dir.path().to_path_buf(),
            None,
            passphrase,
        )
        .expect("passphrase-backed manager should initialize");
        manager.store_secure("private", "entry", &"secret").unwrap();
        manager.flush().unwrap();
        assert!(dir.path().join("persistence.keyslot").exists());
        drop(manager);

        let reopened = PersistenceManager::new_with_base_dir_and_passphrase(
            dir.path().to_path_buf(),
            None,
            passphrase,
        )
        .expect("same passphrase should reopen the store");
        let loaded: String = reopened
            .load_secure("private", "entry")
            .unwrap()
            .expect("entry should survive reopen");
        assert_eq!(loaded, "secret");

        let wrong = PersistenceManager::new_with_base_dir_and_passphrase(
            dir.path().to_path_buf(),
            None,
            "wrong passphrase",
        );
        assert!(
            wrong.is_err(),
            "wrong passphrase unlocked active persistence"
        );
    }

    #[test]
    fn test_persistence_roundtrip() {
        let dir = tempdir().unwrap();
        let manager = PersistenceManager::new_with_base_dir(dir.path().to_path_buf(), None)
            .expect("Failed to create PersistenceManager");

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
    fn test_persistence_envelope_has_magic_and_version() {
        let dir = tempdir().unwrap();
        let manager = PersistenceManager::new_with_base_dir(dir.path().to_path_buf(), None)
            .expect("Failed to create PersistenceManager");

        manager
            .store_secure("envelope_test", "entry", &"secret")
            .expect("Failed to store");

        let tree = manager.get_tree("envelope_test").unwrap();
        let raw = tree.get("entry").unwrap().expect("entry missing");
        assert!(raw.len() > 5, "encrypted envelope is unexpectedly short");
        assert_eq!(&raw[0..4], b"GWPE");
        assert_eq!(raw[4], 1, "unsupported envelope version");
    }

    #[test]
    fn test_persistence_rejects_legacy_unversioned_blob() {
        let dir = tempdir().unwrap();
        let manager = PersistenceManager::new_with_base_dir(dir.path().to_path_buf(), None)
            .expect("Failed to create PersistenceManager");
        let encrypted =
            EncryptionManager::encrypt_bytes(b"legacy").expect("Failed to create legacy blob");
        let mut legacy = encrypted.nonce;
        legacy.extend_from_slice(&encrypted.ciphertext);
        manager
            .get_tree("legacy_test")
            .unwrap()
            .insert("entry", legacy)
            .unwrap();

        let result: Result<Option<String>> = manager.load_secure("legacy_test", "entry");
        assert!(result.is_err(), "legacy persistence was silently accepted");
    }

    #[test]
    fn test_persistence_aad_binds_record_key() {
        let dir = tempdir().unwrap();
        let manager = PersistenceManager::new_with_base_dir(dir.path().to_path_buf(), None)
            .expect("Failed to create PersistenceManager");
        manager
            .store_secure("aad_test", "original", &"secret")
            .expect("Failed to store");

        let raw = manager
            .get_tree("aad_test")
            .unwrap()
            .get("original")
            .unwrap()
            .expect("entry missing");
        manager
            .get_tree("aad_test")
            .unwrap()
            .insert("rewritten", raw)
            .unwrap();

        let result: Result<Option<String>> = manager.load_secure("aad_test", "rewritten");
        assert!(result.is_err(), "record-key substitution bypassed AAD");
    }

    #[test]
    fn test_persistence_rejects_ciphertext_tampering() {
        let dir = tempdir().unwrap();
        let manager = PersistenceManager::new_with_base_dir(dir.path().to_path_buf(), None)
            .expect("Failed to create PersistenceManager");
        manager
            .store_secure("tamper_test", "entry", &"secret")
            .expect("Failed to store");

        let tree = manager.get_tree("tamper_test").unwrap();
        let mut raw = tree.get("entry").unwrap().expect("entry missing").to_vec();
        let last = raw.len() - 1;
        raw[last] ^= 0x01;
        tree.insert("entry", raw).unwrap();

        let result: Result<Option<String>> = manager.load_secure("tamper_test", "entry");
        assert!(result.is_err(), "ciphertext tampering was accepted");
    }

    #[test]
    fn test_list_keys() {
        let dir = tempdir().unwrap();
        let manager = PersistenceManager::new_with_base_dir(
            dir.path().to_path_buf(),
            Some("profile1".to_string()),
        )
        .unwrap();

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
        let manager =
            PersistenceManager::new_with_base_dir(dir.path().to_path_buf(), None).unwrap();

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
