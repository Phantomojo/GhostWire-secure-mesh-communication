use crate::core::encryption::EncryptionManager;
use crate::core::passphrase_keyslot::{self, PersistenceKey};
use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use sled::{Db, Tree};
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::sync::{RwLock, RwLockReadGuard};
use zeroize::Zeroizing;

const ENVELOPE_MAGIC: &[u8; 4] = b"GWPE";
const ENVELOPE_VERSION: u8 = 1;
const NONCE_LEN: usize = 24;
const AEAD_TAG_LEN: usize = 16;
const ENVELOPE_HEADER_LEN: usize = ENVELOPE_MAGIC.len() + 1 + NONCE_LEN;
const MAX_ENVELOPE_LEN: usize = 256 * 1024 * 1024;
const META_TREE: &str = "__ghostwire_meta";
const META_MODE_KEY: &str = "mode";
const META_USER_MODE: &[u8] = b"GWPM1-USER";

#[derive(Debug)]
enum KeySource {
    Machine,
    Locked,
    User(PersistenceKey),
}

/// Persistent database manager for GhostWire.
///
/// A newly constructed manager opens in machine-key compatibility mode only
/// when the database is legacy. User-mode databases open locked and require an
/// explicit passphrase unlock before private records can be read or written.
pub struct PersistenceManager {
    db: RwLock<Db>,
    base_dir: PathBuf,
    profile_name: Option<String>,
    key_source: RwLock<KeySource>,
    legacy_machine_key: RwLock<Option<Zeroizing<Vec<u8>>>>,
}

impl PersistenceManager {
    pub fn new(profile_name: Option<String>) -> Result<Self> {
        let base_dir = if let Ok(ghost_home) = std::env::var("GHOSTWIRE_HOME") {
            std::path::PathBuf::from(ghost_home).join("vault")
        } else {
            let home = dirs::home_dir().ok_or_else(|| anyhow!("Could not find home directory"))?;
            home.join(".ghostwire").join("vault")
        };
        Self::new_with_base_dir(base_dir, profile_name)
    }

    /// Create a manager with an explicit base directory.
    /// User-mode stores remain locked; legacy stores temporarily use the
    /// machine key until `unlock_with_passphrase` performs explicit migration.
    pub fn new_with_base_dir(base_dir: PathBuf, profile_name: Option<String>) -> Result<Self> {
        Self::open(base_dir, profile_name, None)
    }

    pub fn new_locked(profile_name: Option<String>) -> Result<Self> {
        let base_dir = if let Ok(ghost_home) = std::env::var("GHOSTWIRE_HOME") {
            std::path::PathBuf::from(ghost_home).join("vault")
        } else {
            let home = dirs::home_dir().ok_or_else(|| anyhow!("Could not find home directory"))?;
            home.join(".ghostwire").join("vault")
        };
        Self::new_locked_with_base_dir(base_dir, profile_name)
    }

    pub fn new_locked_with_base_dir(
        base_dir: PathBuf,
        profile_name: Option<String>,
    ) -> Result<Self> {
        Self::open(base_dir, profile_name, Some(KeySource::Locked))
    }

    pub fn new_with_base_dir_and_passphrase(
        base_dir: PathBuf,
        profile_name: Option<String>,
        passphrase: &str,
    ) -> Result<Self> {
        std::fs::create_dir_all(&base_dir)?;
        let db_path = db_path(&base_dir, profile_name.as_deref());
        let existed = db_path.exists();
        let db = sled::open(&db_path)?;
        let source = if has_user_mode(&db)? {
            KeySource::User(passphrase_keyslot::load(
                &base_dir,
                profile_name.as_deref(),
                passphrase,
            )?)
        } else if existed {
            bail!("legacy persistence requires explicit migration through unlock");
        } else {
            let key = PersistenceKey::random()?;
            passphrase_keyslot::create_for_key(
                &base_dir,
                profile_name.as_deref(),
                passphrase,
                &key,
            )?;
            set_user_mode(&db)?;
            KeySource::User(key)
        };
        Ok(Self {
            db: RwLock::new(db),
            base_dir,
            profile_name,
            key_source: RwLock::new(source),
            legacy_machine_key: RwLock::new(None),
        })
    }

    fn open(
        base_dir: PathBuf,
        profile_name: Option<String>,
        forced_source: Option<KeySource>,
    ) -> Result<Self> {
        std::fs::create_dir_all(&base_dir)?;
        let db = sled::open(db_path(&base_dir, profile_name.as_deref()))?;
        let user_mode = has_user_mode(&db)?;
        let source = forced_source.unwrap_or_else(|| {
            if user_mode {
                KeySource::Locked
            } else {
                KeySource::Machine
            }
        });
        let legacy_machine_key = if !user_mode {
            Some(Zeroizing::new(
                EncryptionManager::machine_master_key()
                    .map_err(|e| anyhow!("machine key initialization failed: {e}"))?,
            ))
        } else {
            None
        };
        Ok(Self {
            db: RwLock::new(db),
            base_dir,
            profile_name,
            key_source: RwLock::new(source),
            legacy_machine_key: RwLock::new(legacy_machine_key),
        })
    }

    fn get_tree(&self, name: &str) -> Result<Tree> {
        self.db
            .read()
            .map_err(|_| anyhow!("persistence database lock poisoned"))?
            .open_tree(name)
            .map_err(Into::into)
    }

    fn key_source(&self) -> Result<RwLockReadGuard<'_, KeySource>> {
        self.key_source
            .read()
            .map_err(|_| anyhow!("persistence key lock poisoned"))
    }

    fn require_user_key(&self) -> Result<PersistenceKey> {
        match &*self.key_source()? {
            KeySource::User(key) => Ok(key.clone()),
            KeySource::Locked => bail!("persistence is locked; unlock the profile first"),
            KeySource::Machine => {
                bail!("persistence is legacy machine-key mode; migrate through unlock")
            }
        }
    }

    fn ensure_accessible(&self) -> Result<()> {
        if matches!(*self.key_source()?, KeySource::Locked) {
            bail!("persistence is locked; unlock the profile first");
        }
        Ok(())
    }

    fn legacy_machine_key(&self) -> Result<Zeroizing<Vec<u8>>> {
        self.legacy_machine_key
            .read()
            .map_err(|_| anyhow!("legacy machine-key lock poisoned"))?
            .as_ref()
            .cloned()
            .ok_or_else(|| anyhow!("legacy machine key is unavailable"))
    }

    pub fn is_unlocked(&self) -> bool {
        matches!(
            *self.key_source.read().unwrap_or_else(|e| e.into_inner()),
            KeySource::User(_)
        )
    }

    pub fn is_user_mode(&self) -> Result<bool> {
        let db = self
            .db
            .read()
            .map_err(|_| anyhow!("persistence database lock poisoned"))?;
        has_user_mode(&db)
    }

    /// Initialize a blank first-run store under the user passphrase.
    /// A non-empty legacy store returns `Ok(false)` so the caller must use
    /// identity-authenticated migration instead of silently re-keying data.
    pub fn initialize_user_mode_if_empty(&self, passphrase: &str) -> Result<bool> {
        if self.is_user_mode()? {
            return Ok(false);
        }
        {
            let db = self
                .db
                .read()
                .map_err(|_| anyhow!("persistence database lock poisoned"))?;
            for tree_name in db.tree_names() {
                let tree_name = String::from_utf8(tree_name.to_vec())
                    .map_err(|_| anyhow!("persistence tree name is not valid UTF-8"))?;
                if tree_name == META_TREE {
                    continue;
                }
                let tree = db.open_tree(&tree_name)?;
                if tree.iter().next().transpose()?.is_some() {
                    return Ok(false);
                }
            }
        }

        let keyslot =
            passphrase_keyslot::keyslot_path(&self.base_dir, self.profile_name.as_deref());
        let key = if keyslot.exists() {
            passphrase_keyslot::load(&self.base_dir, self.profile_name.as_deref(), passphrase)?
        } else {
            let key = PersistenceKey::random()?;
            passphrase_keyslot::create_for_key(
                &self.base_dir,
                self.profile_name.as_deref(),
                passphrase,
                &key,
            )?;
            key
        };
        {
            let db = self
                .db
                .read()
                .map_err(|_| anyhow!("persistence database lock poisoned"))?;
            set_user_mode(&db)?;
        }
        *self
            .legacy_machine_key
            .write()
            .map_err(|_| anyhow!("legacy machine-key lock poisoned"))? = None;
        *self
            .key_source
            .write()
            .map_err(|_| anyhow!("persistence key lock poisoned"))? = KeySource::User(key);
        Ok(true)
    }

    /// Require that a caller has an unlocked user-keyslot, not merely a
    /// legacy machine-key compatibility key.
    pub fn ensure_user_unlocked(&self) -> Result<()> {
        self.require_user_key().map(|_| ())
    }

    /// Encrypt a record owned by a subsystem outside the normal sled trees.
    /// The profile, namespace, record key, and user persistence key are bound
    /// by the same authenticated envelope used for normal persistence.
    pub fn encrypt_user_record(&self, namespace: &str, key: &str, data: &[u8]) -> Result<Vec<u8>> {
        self.ensure_user_unlocked()?;
        self.encrypt_payload(namespace, key, data)
    }

    /// Decrypt a record owned by a subsystem outside the normal sled trees.
    pub fn decrypt_user_record(&self, namespace: &str, key: &str, data: &[u8]) -> Result<Vec<u8>> {
        self.ensure_user_unlocked()?;
        self.decrypt_payload(namespace, key, data)
    }

    /// Unlock an existing user-mode store with the user passphrase.
    /// Legacy machine-key records require identity-authenticated migration.
    pub fn unlock_with_passphrase(&self, passphrase: &str) -> Result<()> {
        if !self.is_user_mode()? {
            bail!("legacy persistence requires identity-authenticated migration");
        }
        let key =
            passphrase_keyslot::load(&self.base_dir, self.profile_name.as_deref(), passphrase)?;
        *self
            .key_source
            .write()
            .map_err(|_| anyhow!("persistence key lock poisoned"))? = KeySource::User(key);
        Ok(())
    }

    /// Migrate the legacy machine-key store after the caller has verified the
    /// passphrase against the user identity. This method deliberately does not
    /// attempt to verify the identity password itself.
    pub fn migrate_legacy_with_verified_passphrase(&self, passphrase: &str) -> Result<()> {
        if self.is_user_mode()? {
            return self.unlock_with_passphrase(passphrase);
        }
        self.migrate_machine_store(passphrase)
    }

    /// Replace the passphrase without changing the data-encryption key.
    pub fn change_passphrase(&self, new_passphrase: &str) -> Result<()> {
        let key = self.require_user_key()?;
        passphrase_keyslot::replace_for_key(
            &self.base_dir,
            self.profile_name.as_deref(),
            new_passphrase,
            &key,
        )
    }

    pub fn lock(&self) -> Result<()> {
        let mut source = self
            .key_source
            .write()
            .map_err(|_| anyhow!("persistence key lock poisoned"))?;
        if matches!(*source, KeySource::User(_)) {
            *source = KeySource::Locked;
        }
        Ok(())
    }

    pub fn store_secure<T: Serialize>(&self, tree_name: &str, key: &str, data: &T) -> Result<()> {
        self.ensure_accessible()?;
        let tree = self.get_tree(tree_name)?;
        let json = serde_json::to_vec(data)?;
        let encrypted = self.encrypt_payload(tree_name, key, &json)?;
        tree.insert(key, encrypted)?;
        Ok(())
    }

    pub fn load_secure<T: for<'de> Deserialize<'de>>(
        &self,
        tree_name: &str,
        key: &str,
    ) -> Result<Option<T>> {
        self.ensure_accessible()?;
        let tree = self.get_tree(tree_name)?;
        if let Some(val) = tree.get(key)? {
            let decrypted = self.decrypt_payload(tree_name, key, &val)?;
            let data = serde_json::from_slice(&decrypted)?;
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }

    pub fn list_keys(&self, tree_name: &str) -> Result<Vec<String>> {
        self.ensure_accessible()?;
        let tree = self.get_tree(tree_name)?;
        tree.iter()
            .map(|item| {
                let (key, _) = item?;
                String::from_utf8(key.to_vec()).map_err(Into::into)
            })
            .collect()
    }

    pub fn load_all_secure<T: for<'de> Deserialize<'de>>(&self, tree_name: &str) -> Result<Vec<T>> {
        self.ensure_accessible()?;
        let tree = self.get_tree(tree_name)?;
        let mut results = Vec::new();
        for item in tree.iter() {
            let (key, value) = item?;
            let key = String::from_utf8(key.to_vec())
                .map_err(|_| anyhow!("Persistence key is not valid UTF-8"))?;
            let decrypted = self.decrypt_payload(tree_name, &key, &value)?;
            results.push(serde_json::from_slice(&decrypted)?);
        }
        Ok(results)
    }

    fn append_len_prefixed(buffer: &mut Vec<u8>, value: &str) -> Result<()> {
        let length =
            u32::try_from(value.len()).map_err(|_| anyhow!("Persistence context is too large"))?;
        buffer.extend_from_slice(&length.to_be_bytes());
        buffer.extend_from_slice(value.as_bytes());
        Ok(())
    }

    fn envelope_aad(&self, tree_name: &str, key: &str) -> Result<Vec<u8>> {
        let mut aad = b"GhostWire.PersistenceManager".to_vec();
        aad.push(ENVELOPE_VERSION);
        Self::append_len_prefixed(&mut aad, self.profile_name.as_deref().unwrap_or("default"))?;
        Self::append_len_prefixed(&mut aad, tree_name)?;
        Self::append_len_prefixed(&mut aad, key)?;
        Ok(aad)
    }

    fn encrypt_payload(&self, tree_name: &str, key: &str, data: &[u8]) -> Result<Vec<u8>> {
        let aad = self.envelope_aad(tree_name, key)?;
        let result = match &*self.key_source()? {
            KeySource::Machine => {
                let machine_key = self.legacy_machine_key()?;
                EncryptionManager::encrypt_bytes_with_key_aad(data, machine_key.as_slice(), &aad)
            }
            KeySource::User(persistence_key) => EncryptionManager::encrypt_bytes_with_key_aad(
                data,
                persistence_key.as_bytes(),
                &aad,
            ),
            KeySource::Locked => return Err(anyhow!("persistence is locked")),
        }
        .map_err(|e| anyhow!("Encryption failed: {e}"))?;
        let total_len = ENVELOPE_HEADER_LEN
            .checked_add(result.ciphertext.len())
            .ok_or_else(|| anyhow!("Persistence envelope length overflow"))?;
        if total_len > MAX_ENVELOPE_LEN {
            bail!("Persistence envelope exceeds maximum size");
        }
        let mut envelope = Vec::with_capacity(total_len);
        envelope.extend_from_slice(ENVELOPE_MAGIC);
        envelope.push(ENVELOPE_VERSION);
        envelope.extend_from_slice(&result.nonce);
        envelope.extend_from_slice(&result.ciphertext);
        Ok(envelope)
    }

    fn encrypt_payload_with_key(
        &self,
        tree_name: &str,
        key: &str,
        data: &[u8],
        persistence_key: &PersistenceKey,
    ) -> Result<Vec<u8>> {
        let aad = self.envelope_aad(tree_name, key)?;
        let result =
            EncryptionManager::encrypt_bytes_with_key_aad(data, persistence_key.as_bytes(), &aad)
                .map_err(|e| anyhow!("Encryption failed: {e}"))?;
        let mut envelope = Vec::with_capacity(ENVELOPE_HEADER_LEN + result.ciphertext.len());
        envelope.extend_from_slice(ENVELOPE_MAGIC);
        envelope.push(ENVELOPE_VERSION);
        envelope.extend_from_slice(&result.nonce);
        envelope.extend_from_slice(&result.ciphertext);
        Ok(envelope)
    }

    fn decrypt_machine_payload(&self, tree_name: &str, key: &str, data: &[u8]) -> Result<Vec<u8>> {
        if data.len() < ENVELOPE_HEADER_LEN + AEAD_TAG_LEN {
            bail!("Persistence envelope is too short");
        }
        if data.len() > MAX_ENVELOPE_LEN {
            bail!("Persistence envelope exceeds maximum size");
        }
        if &data[..ENVELOPE_MAGIC.len()] != ENVELOPE_MAGIC {
            bail!("Unsupported or legacy persistence envelope; explicit migration required");
        }
        if data[ENVELOPE_MAGIC.len()] != ENVELOPE_VERSION {
            bail!("Unsupported persistence envelope version");
        }
        let nonce_start = ENVELOPE_MAGIC.len() + 1;
        let nonce_end = nonce_start + NONCE_LEN;
        let aad = self.envelope_aad(tree_name, key)?;
        let machine_key = self.legacy_machine_key()?;
        EncryptionManager::decrypt_bytes_with_key_aad(
            &data[nonce_end..],
            &data[nonce_start..nonce_end],
            machine_key.as_slice(),
            &aad,
        )
        .map_err(|e| anyhow!("Legacy decryption failed: {e}"))
    }

    fn decrypt_payload(&self, tree_name: &str, key: &str, data: &[u8]) -> Result<Vec<u8>> {
        if data.len() < ENVELOPE_HEADER_LEN + AEAD_TAG_LEN {
            bail!("Persistence envelope is too short");
        }
        if data.len() > MAX_ENVELOPE_LEN {
            bail!("Persistence envelope exceeds maximum size");
        }
        if &data[..ENVELOPE_MAGIC.len()] != ENVELOPE_MAGIC {
            bail!("Unsupported or legacy persistence envelope; explicit migration required");
        }
        if data[ENVELOPE_MAGIC.len()] != ENVELOPE_VERSION {
            bail!("Unsupported persistence envelope version");
        }
        let nonce_start = ENVELOPE_MAGIC.len() + 1;
        let nonce_end = nonce_start + NONCE_LEN;
        let aad = self.envelope_aad(tree_name, key)?;
        match &*self.key_source()? {
            KeySource::Machine => {
                let machine_key = self.legacy_machine_key()?;
                EncryptionManager::decrypt_bytes_with_key_aad(
                    &data[nonce_end..],
                    &data[nonce_start..nonce_end],
                    machine_key.as_slice(),
                    &aad,
                )
            }
            KeySource::User(persistence_key) => EncryptionManager::decrypt_bytes_with_key_aad(
                &data[nonce_end..],
                &data[nonce_start..nonce_end],
                persistence_key.as_bytes(),
                &aad,
            ),
            KeySource::Locked => return Err(anyhow!("persistence is locked")),
        }
        .map_err(|e| anyhow!("Decryption failed: {e}"))
    }

    fn migrate_machine_store(&self, passphrase: &str) -> Result<()> {
        let keyslot_path =
            passphrase_keyslot::keyslot_path(&self.base_dir, self.profile_name.as_deref());
        let had_keyslot = keyslot_path.exists();
        let new_key = if had_keyslot {
            passphrase_keyslot::load(&self.base_dir, self.profile_name.as_deref(), passphrase)?
        } else {
            let key = PersistenceKey::random()?;
            passphrase_keyslot::create_for_key(
                &self.base_dir,
                self.profile_name.as_deref(),
                passphrase,
                &key,
            )?;
            key
        };

        let target = db_path(&self.base_dir, self.profile_name.as_deref());
        let staging = target.with_extension("passphrase-migration");
        let _ = std::fs::remove_dir_all(&staging);
        let staged = sled::open(&staging)?;
        let source = self
            .db
            .read()
            .map_err(|_| anyhow!("persistence database lock poisoned"))?;
        for tree_name in source.tree_names() {
            let tree_name = String::from_utf8(tree_name.to_vec())
                .map_err(|_| anyhow!("persistence tree name is not valid UTF-8"))?;
            if tree_name == META_TREE {
                continue;
            }
            let source_tree = source.open_tree(&tree_name)?;
            let target_tree = staged.open_tree(&tree_name)?;
            for item in source_tree.iter() {
                let (raw_key, raw_value) = item?;
                let key = String::from_utf8(raw_key.to_vec())
                    .map_err(|_| anyhow!("persistence key is not valid UTF-8"))?;
                let plaintext = self.decrypt_machine_payload(&tree_name, &key, &raw_value)?;
                let encrypted =
                    self.encrypt_payload_with_key(&tree_name, &key, &plaintext, &new_key)?;
                target_tree.insert(key, encrypted)?;
            }
        }
        set_user_mode(&staged)?;
        staged.flush()?;
        drop(staged);
        drop(source);

        let mut db = self
            .db
            .write()
            .map_err(|_| anyhow!("persistence database lock poisoned"))?;
        let placeholder = sled::Config::new().temporary(true).open()?;
        let old_db = std::mem::replace(&mut *db, placeholder);
        drop(old_db);
        let legacy_backup = target.with_extension("machine-key.legacy");
        let _ = std::fs::remove_dir_all(&legacy_backup);
        std::fs::rename(&target, &legacy_backup)
            .with_context(|| format!("stage legacy persistence at {}", legacy_backup.display()))?;
        if let Err(error) = std::fs::rename(&staging, &target) {
            let _ = std::fs::rename(&legacy_backup, &target);
            let restored = sled::open(&target)?;
            let placeholder = std::mem::replace(&mut *db, restored);
            drop(placeholder);
            return Err(error.into());
        }
        let new_db = sled::open(&target)?;
        debug_assert!(had_keyslot || keyslot_path.exists());
        let placeholder = std::mem::replace(&mut *db, new_db);
        drop(placeholder);
        drop(db);
        let _ = std::fs::remove_dir_all(&legacy_backup);
        *self
            .legacy_machine_key
            .write()
            .map_err(|_| anyhow!("legacy machine-key lock poisoned"))? = None;
        *self
            .key_source
            .write()
            .map_err(|_| anyhow!("persistence key lock poisoned"))? = KeySource::User(new_key);
        Ok(())
    }

    pub fn flush(&self) -> Result<()> {
        self.db
            .read()
            .map_err(|_| anyhow!("persistence database lock poisoned"))?
            .flush()?;
        Ok(())
    }

    pub fn clear_tree(&self, tree_name: &str) -> Result<()> {
        self.ensure_accessible()?;
        let tree = self.get_tree(tree_name)?;
        tree.clear()?;
        self.flush()
    }
}

fn db_path(base_dir: &Path, profile: Option<&str>) -> PathBuf {
    base_dir.join(format!("ghost_{}", profile.unwrap_or("default")))
}

fn has_user_mode(db: &Db) -> Result<bool> {
    Ok(db
        .open_tree(META_TREE)?
        .get(META_MODE_KEY)?
        .map(|value| value.as_ref() == META_USER_MODE)
        .unwrap_or(false))
}

fn set_user_mode(db: &Db) -> Result<()> {
    db.open_tree(META_TREE)?
        .insert(META_MODE_KEY, META_USER_MODE)?;
    db.flush()?;
    Ok(())
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
        assert!(dir
            .path()
            .join("ghost_default.persistence.keyslot")
            .exists());
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
        assert!(PersistenceManager::new_with_base_dir_and_passphrase(
            dir.path().to_path_buf(),
            None,
            "wrong passphrase",
        )
        .is_err());
    }

    #[test]
    fn test_locked_manager_requires_unlock() {
        let dir = tempdir().unwrap();
        let manager = PersistenceManager::new_with_base_dir_and_passphrase(
            dir.path().to_path_buf(),
            None,
            "correct horse battery staple",
        )
        .unwrap();
        manager.store_secure("private", "entry", &"secret").unwrap();
        drop(manager);
        let locked =
            PersistenceManager::new_locked_with_base_dir(dir.path().to_path_buf(), None).unwrap();
        assert!(locked.load_secure::<String>("private", "entry").is_err());
        locked
            .unlock_with_passphrase("correct horse battery staple")
            .unwrap();
        assert_eq!(
            locked.load_secure::<String>("private", "entry").unwrap(),
            Some("secret".into())
        );
        locked.lock().unwrap();
        assert!(locked.load_secure::<String>("private", "entry").is_err());
    }

    #[test]
    fn test_machine_store_migration_requires_correct_passphrase() {
        let dir = tempdir().unwrap();
        let legacy = PersistenceManager::new_with_base_dir(dir.path().to_path_buf(), None).unwrap();
        let raw = legacy
            .encrypt_payload("private", "entry", &serde_json::to_vec(&"secret").unwrap())
            .unwrap();
        legacy
            .get_tree("private")
            .unwrap()
            .insert("entry", raw)
            .unwrap();
        legacy.flush().unwrap();
        legacy
            .migrate_legacy_with_verified_passphrase("correct horse battery staple")
            .unwrap();
        assert_eq!(
            legacy.load_secure::<String>("private", "entry").unwrap(),
            Some("secret".into())
        );
        drop(legacy);
        assert!(PersistenceManager::new_with_base_dir_and_passphrase(
            dir.path().to_path_buf(),
            None,
            "wrong passphrase",
        )
        .is_err());
    }

    #[test]
    fn test_persistence_roundtrip() {
        let dir = tempdir().unwrap();
        let manager = PersistenceManager::new_with_base_dir_and_passphrase(
            dir.path().to_path_buf(),
            None,
            "test passphrase",
        )
        .unwrap();
        let original = ("test_item".to_string(), 42u32);
        manager
            .store_secure("test_tree", "test_key", &original)
            .unwrap();
        let loaded: (String, u32) = manager
            .load_secure("test_tree", "test_key")
            .unwrap()
            .unwrap();
        assert_eq!(original, loaded);
    }

    #[test]
    fn test_persistence_envelope_has_magic_and_version() {
        let dir = tempdir().unwrap();
        let manager = PersistenceManager::new_with_base_dir_and_passphrase(
            dir.path().to_path_buf(),
            None,
            "test passphrase",
        )
        .unwrap();
        manager
            .store_secure("envelope_test", "entry", &"secret")
            .unwrap();
        let raw = manager
            .get_tree("envelope_test")
            .unwrap()
            .get("entry")
            .unwrap()
            .unwrap();
        assert!(raw.len() > 5);
        assert_eq!(&raw[0..4], b"GWPE");
        assert_eq!(raw[4], 1);
    }

    #[test]
    fn test_persistence_rejects_legacy_unversioned_blob() {
        let dir = tempdir().unwrap();
        let manager = PersistenceManager::new_with_base_dir_and_passphrase(
            dir.path().to_path_buf(),
            None,
            "test passphrase",
        )
        .unwrap();
        let encrypted = EncryptionManager::encrypt_bytes(b"legacy").unwrap();
        let mut legacy = encrypted.nonce;
        legacy.extend_from_slice(&encrypted.ciphertext);
        manager
            .get_tree("legacy_test")
            .unwrap()
            .insert("entry", legacy)
            .unwrap();
        let result: Result<Option<String>> = manager.load_secure("legacy_test", "entry");
        assert!(result.is_err());
    }

    #[test]
    fn test_persistence_aad_binds_record_key() {
        let dir = tempdir().unwrap();
        let manager = PersistenceManager::new_with_base_dir_and_passphrase(
            dir.path().to_path_buf(),
            None,
            "test passphrase",
        )
        .unwrap();
        manager
            .store_secure("aad_test", "original", &"secret")
            .unwrap();
        let raw = manager
            .get_tree("aad_test")
            .unwrap()
            .get("original")
            .unwrap()
            .unwrap();
        manager
            .get_tree("aad_test")
            .unwrap()
            .insert("rewritten", raw)
            .unwrap();
        let result: Result<Option<String>> = manager.load_secure("aad_test", "rewritten");
        assert!(result.is_err());
    }

    #[test]
    fn test_persistence_rejects_ciphertext_tampering() {
        let dir = tempdir().unwrap();
        let manager = PersistenceManager::new_with_base_dir_and_passphrase(
            dir.path().to_path_buf(),
            None,
            "test passphrase",
        )
        .unwrap();
        manager
            .store_secure("tamper_test", "entry", &"secret")
            .unwrap();
        let tree = manager.get_tree("tamper_test").unwrap();
        let mut raw = tree.get("entry").unwrap().unwrap().to_vec();
        let last = raw.len() - 1;
        raw[last] ^= 1;
        tree.insert("entry", raw).unwrap();
        let result: Result<Option<String>> = manager.load_secure("tamper_test", "entry");
        assert!(result.is_err());
    }

    #[test]
    fn test_list_keys_and_load_all() {
        let dir = tempdir().unwrap();
        let manager = PersistenceManager::new_with_base_dir_and_passphrase(
            dir.path().to_path_buf(),
            None,
            "test passphrase",
        )
        .unwrap();
        manager.store_secure("items", "1", &10).unwrap();
        manager.store_secure("items", "2", &20).unwrap();
        manager.store_secure("items", "3", &30).unwrap();
        assert_eq!(manager.list_keys("items").unwrap().len(), 3);
        let all: Vec<i32> = manager.load_all_secure("items").unwrap();
        assert_eq!(all.len(), 3);
    }
}
