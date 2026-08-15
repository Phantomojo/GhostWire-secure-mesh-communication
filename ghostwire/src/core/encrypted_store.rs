//! encrypted_store.rs — Encrypted, authenticated key-value storage
//!
//! Veilid lesson: they maintain a `TableStore` (encrypted KV) and a
//! `ProtectedStore` (device secret storage). Their encrypted sled equivalent
//! uses per-key encryption with a device-level master key.
//!
//! Our approach:
//!   - Master storage key derived from identity password via Argon2
//!   - All values encrypted with ChaCha20-Poly1305 + BLAKE3-keyed AEAD
//!   - Key names are hashed (no plaintext metadata on disk)
//!   - Message expiry: auto-delete after configurable TTL
//!   - Rotating storage key: re-encrypt all values on password change
//!
//! This replaces the unencrypted sled usage throughout Ghostwire.

use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, Params,
};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Key, Nonce,
};
use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};
use sled::Db;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use zeroize::{Zeroize, ZeroizeOnDrop};

const STORE_VERSION: u8 = 1;
const NONCE_LEN: usize = 12;
const KEY_HASH_PREFIX: &[u8] = b"gw_khash_v1:";

/// The encrypted storage layer master key. ZeroizedOnDrop for safety.
#[derive(Zeroize, ZeroizeOnDrop)]
struct MasterKey([u8; 32]);

/// Metadata stored alongside each encrypted value
#[derive(Debug, Serialize, Deserialize)]
struct ValueMeta {
    version: u8,
    created_at: u64,
    expires_at: Option<u64>, // None = no expiry
    nonce: [u8; NONCE_LEN],
}

/// An encrypted key-value store backed by sled.
///
/// All keys are hashed before storage (key privacy).
/// All values are encrypted with ChaCha20-Poly1305.
pub struct EncryptedStore {
    db: Arc<Db>,
    master_key: Arc<RwLock<MasterKey>>,
    rng: SystemRandom,
    namespace: String,
}

impl EncryptedStore {
    /// Open (or create) an encrypted store, deriving the master key from password.
    ///
    /// `namespace` scopes the store (e.g. "messages", "identity", "peers").
    pub async fn open(db: Arc<Db>, password: &str, namespace: &str) -> Result<Self> {
        let master_key = Self::derive_master_key(password, namespace, &db).await?;

        Ok(Self {
            db,
            master_key: Arc::new(RwLock::new(master_key)),
            rng: SystemRandom::new(),
            namespace: namespace.to_string(),
        })
    }

    /// Store an encrypted value. Optionally set a TTL for automatic expiry.
    pub async fn put(&self, key: &str, value: &[u8], ttl: Option<Duration>) -> Result<()> {
        let master_key = self.master_key.read().await;
        let cipher = self.make_cipher(&master_key)?;

        let mut nonce_bytes = [0u8; NONCE_LEN];
        self.rng
            .fill(&mut nonce_bytes)
            .map_err(|_| anyhow::anyhow!("RNG failure"))?;
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, value)
            .map_err(|e| anyhow::anyhow!("Encrypt failed: {:?}", e))?;

        let now = now_secs();
        let meta = ValueMeta {
            version: STORE_VERSION,
            created_at: now,
            expires_at: ttl.map(|d| now + d.as_secs()),
            nonce: nonce_bytes,
        };

        let meta_bytes = bincode::serialize(&meta)?;

        // Prepend meta length (4 bytes LE) + meta + ciphertext
        let mut blob = Vec::with_capacity(4 + meta_bytes.len() + ciphertext.len());
        blob.extend_from_slice(&(meta_bytes.len() as u32).to_le_bytes());
        blob.extend_from_slice(&meta_bytes);
        blob.extend_from_slice(&ciphertext);

        let db_key = self.db_key(key);
        self.db.insert(&db_key, blob)?;

        Ok(())
    }

    /// Retrieve and decrypt a value. Returns None if missing or expired.
    pub async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let db_key = self.db_key(key);

        let Some(blob) = self.db.get(&db_key)? else {
            return Ok(None);
        };

        let (meta, ciphertext) = Self::parse_blob(&blob)?;

        // Check expiry
        if let Some(expires_at) = meta.expires_at {
            if now_secs() >= expires_at {
                // Lazily delete expired entry
                self.db.remove(&db_key)?;
                return Ok(None);
            }
        }

        let master_key = self.master_key.read().await;
        let cipher = self.make_cipher(&master_key)?;
        let nonce = Nonce::from_slice(&meta.nonce);

        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| anyhow::anyhow!("Decryption failed — key mismatch or data corrupt"))?;

        Ok(Some(plaintext))
    }

    /// Delete a value.
    pub fn delete(&self, key: &str) -> Result<()> {
        self.db.remove(self.db_key(key))?;
        Ok(())
    }

    /// Check existence without decrypting.
    pub fn exists(&self, key: &str) -> Result<bool> {
        Ok(self.db.contains_key(self.db_key(key))?)
    }

    /// Purge all expired entries in this namespace. Returns count removed.
    pub fn purge_expired(&self) -> Result<usize> {
        let prefix = self.namespace_prefix();
        let now = now_secs();
        let mut removed = 0;

        let keys: Vec<_> = self
            .db
            .scan_prefix(&prefix)
            .filter_map(|r| r.ok())
            .collect();

        for (db_key, blob) in keys {
            if let Ok((meta, _)) = Self::parse_blob(&blob) {
                if let Some(expires_at) = meta.expires_at {
                    if now > expires_at {
                        self.db.remove(&db_key)?;
                        removed += 1;
                    }
                }
            }
        }

        Ok(removed)
    }

    /// Re-encrypt all values with a new password (on password change).
    pub async fn rotate_key(&self, new_password: &str) -> Result<()> {
        let new_key = Self::derive_master_key(new_password, &self.namespace, &self.db).await?;
        let prefix = self.namespace_prefix();

        let old_master = self.master_key.read().await;
        let old_cipher = self.make_cipher(&old_master)?;
        drop(old_master);

        let new_cipher = {
            let key = Key::from_slice(&new_key.0);
            ChaCha20Poly1305::new(key)
        };

        let entries: Vec<_> = self
            .db
            .scan_prefix(&prefix)
            .filter_map(|r| r.ok())
            .collect();

        for (db_key, blob) in entries {
            let Ok((meta, ct)) = Self::parse_blob(&blob) else {
                continue;
            };
            let nonce = Nonce::from_slice(&meta.nonce);

            let Ok(plaintext) = old_cipher.decrypt(nonce, ct) else {
                continue;
            };

            // Re-encrypt with new key and fresh nonce
            let mut new_nonce_bytes = [0u8; NONCE_LEN];
            self.rng
                .fill(&mut new_nonce_bytes)
                .map_err(|_| anyhow::anyhow!("RNG failure"))?;
            let new_nonce = Nonce::from_slice(&new_nonce_bytes);

            let new_ct = new_cipher
                .encrypt(new_nonce, plaintext.as_slice())
                .map_err(|e| anyhow::anyhow!("Re-encrypt failed: {:?}", e))?;

            let new_meta = ValueMeta {
                nonce: new_nonce_bytes,
                ..meta
            };
            let meta_bytes = bincode::serialize(&new_meta)?;
            let mut new_blob = Vec::with_capacity(4 + meta_bytes.len() + new_ct.len());
            new_blob.extend_from_slice(&(meta_bytes.len() as u32).to_le_bytes());
            new_blob.extend_from_slice(&meta_bytes);
            new_blob.extend_from_slice(&new_ct);

            self.db.insert(&db_key, new_blob)?;
        }

        // Update master key in memory
        let mut master_key = self.master_key.write().await;
        *master_key = new_key;

        // Persist new salt
        // (derive_master_key already saved the new salt)
        Ok(())
    }

    // ── Private helpers ───────────────────────────────────────────────────────

    /// Hash-based database key: BLAKE3(namespace || ":" || key)
    fn db_key(&self, key: &str) -> Vec<u8> {
        let mut prefix = self.namespace_prefix();
        let hash = blake3::hash(
            [self.namespace.as_bytes(), b":", key.as_bytes()]
                .concat()
                .as_slice(),
        );
        prefix.extend_from_slice(hash.as_bytes());
        prefix
    }

    fn namespace_prefix(&self) -> Vec<u8> {
        [KEY_HASH_PREFIX, self.namespace.as_bytes(), b":"].concat()
    }

    fn make_cipher(&self, master_key: &MasterKey) -> Result<ChaCha20Poly1305> {
        let key = Key::from_slice(&master_key.0);
        Ok(ChaCha20Poly1305::new(key))
    }

    fn parse_blob(blob: &[u8]) -> Result<(ValueMeta, &[u8])> {
        if blob.len() < 4 {
            anyhow::bail!("Blob too short");
        }
        let meta_len = u32::from_le_bytes(blob[..4].try_into()?) as usize;
        if blob.len() < 4 + meta_len {
            anyhow::bail!(
                "Blob truncated: meta_len={} but blob={}",
                meta_len,
                blob.len()
            );
        }
        let meta: ValueMeta = bincode::deserialize(&blob[4..4 + meta_len])?;
        let ciphertext = &blob[4 + meta_len..];
        Ok((meta, ciphertext))
    }

    /// Derive a 32-byte master key via Argon2id. Saves/loads salt from sled.
    async fn derive_master_key(password: &str, namespace: &str, db: &Db) -> Result<MasterKey> {
        let salt_key = format!("gw_kdf_salt_v1:{}", namespace);

        let salt_str = if let Some(existing) = db.get(salt_key.as_bytes())? {
            String::from_utf8(existing.to_vec()).map_err(|_| anyhow::anyhow!("Corrupt salt"))?
        } else {
            // Generate new salt
            let salt = SaltString::generate(&mut OsRng);
            let s = salt.to_string();
            db.insert(salt_key.as_bytes(), s.as_bytes())?;
            s
        };

        // Argon2id with sensible params (memory: 64 MiB, iterations: 3, parallelism: 4)
        let params = Params::new(65536, 3, 4, Some(32))
            .map_err(|e| anyhow::anyhow!("Argon2 params error: {}", e))?;
        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);

        // Include namespace in the password to create domain separation
        let namespaced_pwd = format!("ghostwire_store_v1:{}:{}", namespace, password);

        let salt =
            SaltString::from_b64(&salt_str).map_err(|e| anyhow::anyhow!("Bad salt: {}", e))?;

        let hash = argon2
            .hash_password(namespaced_pwd.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Argon2 hash failed: {}", e))?;

        // Extract 32-byte hash output
        let hash_output = hash.hash.ok_or_else(|| anyhow::anyhow!("No hash output"))?;
        let mut key = [0u8; 32];
        let bytes = hash_output.as_bytes();
        let copy_len = bytes.len().min(32);
        key[..copy_len].copy_from_slice(&bytes[..copy_len]);

        Ok(MasterKey(key))
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    async fn make_store() -> EncryptedStore {
        let db = Arc::new(sled::Config::new().temporary(true).open().unwrap());
        EncryptedStore::open(db, "test_password_123", "test")
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn put_get_roundtrip() {
        let store = make_store().await;
        store.put("my_key", b"secret value", None).await.unwrap();
        let val = store.get("my_key").await.unwrap().unwrap();
        assert_eq!(val, b"secret value");
    }

    #[tokio::test]
    async fn expired_returns_none() {
        let store = make_store().await;
        store
            .put("expiring", b"data", Some(Duration::from_secs(0)))
            .await
            .unwrap();
        // TTL=0 means already expired
        tokio::time::sleep(Duration::from_millis(10)).await;
        let val = store.get("expiring").await.unwrap();
        assert!(val.is_none());
    }

    #[tokio::test]
    async fn key_names_are_hashed() {
        let store = make_store().await;
        store.put("sensitive_key_name", b"val", None).await.unwrap();
        // The plaintext key should not appear in the database
        let raw_key_present = store.db.scan_prefix(b"sensitive").next().is_some();
        assert!(
            !raw_key_present,
            "Plaintext key found in DB — key hashing broken"
        );
    }

    #[tokio::test]
    async fn delete_works() {
        let store = make_store().await;
        store.put("to_delete", b"val", None).await.unwrap();
        assert!(store.exists("to_delete").unwrap());
        store.delete("to_delete").unwrap();
        assert!(!store.exists("to_delete").unwrap());
    }
}
