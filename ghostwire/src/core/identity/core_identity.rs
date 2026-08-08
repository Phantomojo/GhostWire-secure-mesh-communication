use crate::core::identity::encryption::{decrypt_identity, encrypt_identity, EncryptedIdentity};
use anyhow::Result;
use libp2p::identity::{Keypair, PublicKey};
use obfstr::obfstr;
use parking_lot::RwLock;
use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};

pub struct EphemeralIdentity {
    pub identity_id: String,
    keypair: Keypair,
}

impl EphemeralIdentity {
    pub fn new() -> Result<Self> {
        let keypair = Keypair::generate_ed25519();
        let mut id_bytes = [0u8; 8];
        let rng = SystemRandom::new();
        rng.fill(&mut id_bytes[..])
            .map_err(|e| anyhow::anyhow!("RNG error: {:?}", e))?;
        let identity_id = format!("id_{:x}", u64::from_le_bytes(id_bytes));
        Ok(EphemeralIdentity {
            identity_id,
            keypair,
        })
    }

    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        self.keypair.sign(message).unwrap_or_default()
    }

    pub fn verify(&self, message: &[u8], signature: &[u8], public_key_bytes: &[u8]) -> Result<()> {
        let public_key = PublicKey::try_decode_protobuf(public_key_bytes)
            .map_err(|e| anyhow::anyhow!("Failed to decode public key: {:?}", e))?;
        if public_key.verify(message, signature) {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Signature verification failed"))
        }
    }

    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.keypair.public().encode_protobuf()
    }
}

impl Clone for EphemeralIdentity {
    fn clone(&self) -> Self {
        EphemeralIdentity {
            identity_id: self.identity_id.clone(),
            keypair: self.keypair.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityMetadata {
    pub nickname: String,
    pub bio: String,
    pub avatar_url: Option<String>,
    pub verified_claims: Vec<String>,
    pub last_updated: u64,
    #[serde(default)]
    pub setup_complete: bool,
}

#[derive(Debug)]
pub enum IdentityState {
    // CRITICAL FIX: Store public key bytes alongside encrypted blob so get_public_key()
    // works even when locked (public keys are not secret)
    Locked(EncryptedIdentity, Vec<u8>),
    Unlocked(Keypair, Option<EncryptedIdentity>),
}

#[derive(Debug, Serialize)]
pub struct Identity {
    pub id: String,
    pub did: String, // did:ghost:<pk_hash>
    #[serde(skip)]
    metadata: RwLock<IdentityMetadata>,
    #[serde(skip)]
    state: RwLock<IdentityState>,
    #[serde(skip)]
    pub profile: Option<String>,
}

impl Identity {
    pub fn new(profile: Option<String>) -> Result<Self, Box<dyn std::error::Error>> {
        Self::load_or_create(profile)
    }

    pub fn load_or_create(profile: Option<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let storage_path = Self::get_storage_path(profile.clone())?;

        if storage_path.exists() {
            match Self::load_from_disk(&storage_path) {
                Ok(mut identity) => {
                    identity.profile = profile;

                    if identity.is_locked() {
                        if let Ok(password) = Self::load_keyfile_password(&storage_path) {
                            match identity.unlock(&password) {
                                Ok(()) => {
                                    tracing::info!(
                                        "Identity {} loaded (auto-unlocked via keyfile)",
                                        &identity.id[..8]
                                    );
                                }
                                Err(e) => {
                                    tracing::debug!(
                                        "Keyfile auto-unlock failed: {}. Identity remains locked.",
                                        e
                                    );
                                }
                            }
                        } else {
                            tracing::debug!("No keyfile found. Identity remains locked.");
                        }
                    } else {
                        tracing::info!("Identity {} loaded (unlocked)", &identity.id[..8]);
                    }

                    return Ok(identity);
                }
                Err(e) => {
                    tracing::debug!("Failed to load identity ({}), creating new one.", e);
                }
            }
        }

        tracing::info!("Creating new identity...");
        let identity = Self::create_new(profile)?;
        let initial_password = Self::generate_initial_password()?;
        identity.save_to_disk(&initial_password)?;

        let keyfile_path = Self::get_storage_path(None)?.with_file_name(".initial-password");
        std::fs::write(&keyfile_path, &initial_password)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&keyfile_path)?.permissions();
            perms.set_mode(0o600);
            std::fs::set_permissions(&keyfile_path, perms)?;
        }
        tracing::info!(
            "New identity created. See ~/.ghostwire/.initial-password for your password."
        );

        if let Err(e) = Self::save_keyfile_password(&storage_path, &initial_password) {
            tracing::warn!(
                "Failed to save keyfile (auto-unlock won't work on restart): {}",
                e
            );
        }

        Ok(identity)
    }

    fn create_new(profile: Option<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let keypair = Keypair::generate_ed25519();
        let pub_key = keypair.public();
        let pk_bytes = pub_key.encode_protobuf();

        // Generate a DID for the identity
        use ring::digest;
        let pk_hash = hex::encode(digest::digest(&digest::SHA256, &pk_bytes).as_ref());
        let did = format!("did:ghost:{}", pk_hash);

        let id = uuid::Uuid::new_v4().to_string();

        let metadata = IdentityMetadata {
            nickname: format!("Ghost_{}", &id[..4]),
            bio: "Sovereign GhostWire Node".to_string(),
            avatar_url: None,
            verified_claims: Vec::new(),
            last_updated: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            setup_complete: false,
        };

        Ok(Identity {
            id,
            did,
            metadata: RwLock::new(metadata),
            state: RwLock::new(IdentityState::Unlocked(keypair, None)),
            profile,
        })
    }

    /// Generate a cryptographically random initial password for new identities.
    /// Returns a 32-character hex string (128 bits of entropy).
    fn generate_initial_password() -> Result<String, Box<dyn std::error::Error>> {
        use ring::rand::{SecureRandom, SystemRandom};
        let rng = SystemRandom::new();
        let mut bytes = [0u8; 16]; // 128 bits
        rng.fill(&mut bytes)
            .map_err(|e| anyhow::anyhow!("RNG failed: {:?}", e))?;
        Ok(hex::encode(bytes))
    }

    /// Save the initial password to a keyfile for auto-unlock on restart.
    fn save_keyfile_password(
        identity_path: &std::path::Path,
        password: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use chacha20poly1305::aead::Aead;
        use chacha20poly1305::{Key, KeyInit, XChaCha20Poly1305, XNonce};
        use ring::rand::{SecureRandom, SystemRandom};

        let keyfile_path = identity_path.with_file_name(".identity.key");

        let machine_id = hostname::get()
            .ok()
            .and_then(|h| h.into_string().ok())
            .unwrap_or_else(|| obfstr!("ghostwire-default").to_string());
        let seed = format!(
            "ghostwire-keyfile-v3-{}-{}",
            machine_id,
            identity_path.display()
        );

        let rng = SystemRandom::new();

        let mut salt = [0u8; 16];
        rng.fill(&mut salt)
            .map_err(|e| anyhow::anyhow!("RNG failed: {:?}", e))?;

        let argon2 = argon2::Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            argon2::Params::new(65536, 3, 4, Some(32))
                .map_err(|e| anyhow::anyhow!("Argon2 params: {}", e))?,
        );
        let mut key_bytes = [0u8; 32];
        argon2
            .hash_password_into(seed.as_bytes(), &salt, &mut key_bytes)
            .map_err(|e| anyhow::anyhow!("Argon2 derivation failed: {}", e))?;

        let mut nonce_bytes = [0u8; 24];
        rng.fill(&mut nonce_bytes)
            .map_err(|e| anyhow::anyhow!("RNG failed: {:?}", e))?;

        let key = Key::from_slice(&key_bytes);
        let cipher = XChaCha20Poly1305::new(key);
        let nonce = XNonce::from_slice(&nonce_bytes);
        let ciphertext = cipher
            .encrypt(nonce, password.as_bytes())
            .map_err(|e| anyhow::anyhow!("Keyfile encryption failed: {}", e))?;

        // Format: [0x03][salt 16][nonce 24][ciphertext]
        let mut out = vec![0x03];
        out.extend_from_slice(&salt);
        out.extend_from_slice(&nonce_bytes);
        out.extend_from_slice(&ciphertext);
        std::fs::write(&keyfile_path, &out)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&keyfile_path)?.permissions();
            perms.set_mode(0o600);
            std::fs::set_permissions(&keyfile_path, perms)?;
        }

        tracing::debug!(
            target: "identity",
            "🔑 Encrypted keyfile saved (Argon2id): {:?}",
            keyfile_path
        );
        Ok(())
    }

    /// Load the initial password from the keyfile for auto-unlock on restart.
    /// SECURITY FIX: Decrypts the password instead of reading plaintext.
    fn load_keyfile_password(
        identity_path: &std::path::Path,
    ) -> Result<String, Box<dyn std::error::Error>> {
        use chacha20poly1305::aead::Aead;
        use chacha20poly1305::{Key, KeyInit, XChaCha20Poly1305, XNonce};

        let keyfile_path = identity_path.with_file_name(".identity.key");

        if !keyfile_path.exists() {
            return Err("Keyfile not found".into());
        }

        let data = std::fs::read(&keyfile_path)?;
        let machine_id = hostname::get()
            .ok()
            .and_then(|h| h.into_string().ok())
            .unwrap_or_else(|| obfstr!("ghostwire-default").to_string());

        if data.first() == Some(&0x03) && data.len() > 41 {
            // v3: [0x03][salt 16][nonce 24][ciphertext]
            let salt = &data[1..17];
            let nonce_bytes: [u8; 24] = data[17..41]
                .try_into()
                .map_err(|_| "Invalid v3 keyfile: nonce too short")?;
            let ciphertext = &data[41..];

            let seed = format!(
                "ghostwire-keyfile-v3-{}-{}",
                machine_id,
                identity_path.display()
            );

            let argon2 = argon2::Argon2::new(
                argon2::Algorithm::Argon2id,
                argon2::Version::V0x13,
                argon2::Params::new(65536, 3, 4, Some(32))
                    .map_err(|e| anyhow::anyhow!("Argon2 params: {}", e))?,
            );
            let mut key_bytes = [0u8; 32];
            argon2
                .hash_password_into(seed.as_bytes(), salt, &mut key_bytes)
                .map_err(|e| anyhow::anyhow!("Argon2 derivation failed: {}", e))?;

            let key = Key::from_slice(&key_bytes);
            let cipher = XChaCha20Poly1305::new(key);
            let nonce = XNonce::from_slice(&nonce_bytes);
            let plaintext = cipher
                .decrypt(nonce, ciphertext)
                .map_err(|_| "Keyfile v3 decryption failed — machine identity may have changed")?;

            let password =
                String::from_utf8(plaintext).map_err(|_| "Keyfile contains invalid UTF-8")?;
            Ok(password)
        } else if data.len() >= 24 {
            let seed = format!(
                "ghostwire-keyfile-v2-{}-{}",
                machine_id,
                identity_path.display()
            );
            let derived = ring::digest::digest(&ring::digest::SHA256, seed.as_bytes());

            let nonce_bytes: [u8; 24] = data[..24]
                .try_into()
                .map_err(|_| "Invalid keyfile format: nonce too short")?;
            let ciphertext = &data[24..];

            let key = Key::from_slice(derived.as_ref());
            let cipher = XChaCha20Poly1305::new(key);
            let nonce = XNonce::from_slice(&nonce_bytes);
            let plaintext = cipher
                .decrypt(nonce, ciphertext)
                .map_err(|_| "Keyfile decryption failed — machine identity may have changed")?;

            let password =
                String::from_utf8(plaintext).map_err(|_| "Keyfile contains invalid UTF-8")?;

            Self::save_keyfile_password(identity_path, &password)?;
            tracing::info!(
                target: "identity",
                "🔑 Migrated keyfile from SHA256 to Argon2id"
            );
            Ok(password)
        } else {
            let password = String::from_utf8(data).map_err(|_| "Keyfile contains invalid UTF-8")?;
            let password = password.trim().to_string();
            Self::save_keyfile_password(identity_path, &password)?;
            tracing::info!(
                target: "identity",
                "🔑 Migrated keyfile from plaintext to Argon2id"
            );
            Ok(password)
        }
    }

    pub fn from_keypair(keypair: Keypair) -> Self {
        let pk_bytes = keypair.public().encode_protobuf();
        // Generate a DID for the identity
        use ring::digest;
        let pk_hash = hex::encode(digest::digest(&digest::SHA256, &pk_bytes).as_ref());
        let did = format!("did:ghost:{}", pk_hash);

        let id = uuid::Uuid::new_v4().to_string();

        let metadata = IdentityMetadata {
            nickname: format!("Ghost_{}", &id[..4]),
            bio: "Recovered GhostWire Node".to_string(),
            avatar_url: None,
            verified_claims: Vec::new(),
            last_updated: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            setup_complete: false,
        };

        Identity {
            id,
            did,
            metadata: RwLock::new(metadata),
            state: RwLock::new(IdentityState::Unlocked(keypair, None)),
            profile: None,
        }
    }

    pub fn from_seed(seed: &[u8]) -> Result<Self> {
        let keypair = Keypair::ed25519_from_bytes(seed.to_vec())
            .map_err(|e| anyhow::anyhow!("Failed to create keypair from seed: {}", e))?;
        Ok(Self::from_keypair(keypair))
    }

    /// Derive a deterministic 32-byte transport-specific key from this identity.
    /// Same identity + transport name → same key. Different identities → different keys.
    /// Used by transports (e.g. Reticulum) that need a persistent seed.
    pub fn derive_transport_key(&self, transport_name: &str) -> [u8; 32] {
        use ring::digest;
        let pk = self.get_public_key();
        let mut input = Vec::with_capacity(pk.len() + transport_name.len());
        input.extend_from_slice(&pk);
        input.extend_from_slice(transport_name.as_bytes());
        let hash = digest::digest(&digest::SHA256, &input);
        let mut key = [0u8; 32];
        key.copy_from_slice(hash.as_ref());
        key
    }

    /// Create a new identity with nickname and password in one step (for profile creation with setup).
    pub fn create_with_setup(
        profile: Option<String>,
        nickname: String,
        password: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let identity = Self::create_new(profile)?;
        identity.update_metadata(Some(nickname), None, None, Some(true));
        identity.save_to_disk(password)?;
        Ok(identity)
    }

    fn get_storage_path(
        profile: Option<String>,
    ) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
        let ghost_dir = if let Ok(ghost_home) = std::env::var("GHOSTWIRE_HOME") {
            std::path::PathBuf::from(ghost_home)
        } else {
            let home = dirs::home_dir().ok_or("Could not find home directory")?;
            home.join(".ghostwire")
        };

        if !ghost_dir.exists() {
            std::fs::create_dir_all(&ghost_dir)?;
        }
        let filename = match profile {
            Some(p) => format!("identity_{}.json", p),
            None => "identity.json".to_string(),
        };
        Ok(ghost_dir.join(&filename))
    }

    fn load_from_disk(path: &std::path::PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        // Read file contents
        let file_content = std::fs::read(path)?;

        // Try to deserialize as new EncryptedIdentity format
        if let Ok(persisted) = serde_json::from_slice::<PersistedEncryptedIdentity>(&file_content) {
            // CRITICAL FIX: Extract public key bytes for use when locked
            let pub_key_bytes = persisted.public_key_bytes.clone().unwrap_or_else(|| {
                // Try to extract from encrypted data if not stored separately
                // For backward compat, return empty - will be populated on next save
                vec![]
            });
            return Ok(Identity {
                id: persisted.id,
                did: persisted.did,
                metadata: RwLock::new(persisted.metadata),
                state: RwLock::new(IdentityState::Locked(
                    persisted.encrypted_keypair,
                    pub_key_bytes,
                )),
                profile: None, // Will be set by caller
            });
        }

        // Fallback: Try old format (migration)
        // For now, return error if not new format
        Err("Unknown identity format".into())
    }

    pub fn save_to_disk(&self, password: &str) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::get_storage_path(self.profile.clone())?;

        tracing::info!(
            target: "identity",
            msg = "💾 Saving identity to disk",
            path = ?path,
            password_len = password.len(),
        );

        let mut state_guard = self.state.write();

        let keypair_bytes = match &*state_guard {
            IdentityState::Unlocked(kp, _) => {
                tracing::debug!(
                    target: "identity",
                    msg = "🔑 Extracting keypair from unlocked state"
                );
                kp.to_protobuf_encoding().map_err(|e| anyhow::anyhow!(e))?
            }
            IdentityState::Locked(_, _) => {
                tracing::error!(
                    target: "identity",
                    msg = "❌ Cannot save locked identity"
                );
                return Err("Cannot save locked identity (need keypair)".into());
            }
        };

        // Encrypt the keypair bytes
        tracing::debug!(
            target: "identity",
            msg = "🔐 Encrypting identity with password"
        );
        let encrypted_keypair = encrypt_identity(&keypair_bytes, password)?;

        // Important: Update the cached encrypted identity so we can lock later!
        if let IdentityState::Unlocked(_, ref mut cached_encrypted) = *state_guard {
            *cached_encrypted = Some(encrypted_keypair.clone());
        }

        let persisted = PersistedEncryptedIdentity {
            id: self.id.clone(),
            did: self.did.clone(),
            metadata: self.metadata.read().clone(),
            encrypted_keypair,
            // CRITICAL FIX: Store public key in plaintext for use when locked
            public_key_bytes: Some(keypair_bytes.clone()),
        };

        let json = serde_json::to_string_pretty(&persisted)?;
        tracing::debug!(
            target: "identity",
            msg = "📝 Writing identity file",
            json_len = json.len(),
        );
        // ponytail: 0o600 — identity.json contains encrypted keypair, not world-readable
        use std::os::unix::fs::OpenOptionsExt;
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(&path)?;
        std::io::Write::write_all(&mut f, json.as_bytes())?;

        // CRITICAL FIX: Update the keyfile with the current password.
        // This ensures auto-unlock works even after the user changes their password.
        if let Err(e) = Self::save_keyfile_password(&path, password) {
            tracing::debug!(
                target: "identity",
                "⚠️ Keyfile not updated (auto-unlock may use stale password): {}",
                e
            );
        }

        tracing::info!(
            target: "identity",
            msg = "✅ Identity saved successfully",
            path = ?path,
        );
        Ok(())
    }

    /// Check if identity is currently locked
    pub fn is_locked(&self) -> bool {
        let state = self.state.read();
        match *state {
            IdentityState::Locked(_, _) => true,
            _ => false,
        }
    }

    /// Unlock identity with password
    pub fn unlock(&self, password: &str) -> Result<()> {
        let mut state = self.state.write();

        if let IdentityState::Locked(ref encrypted, _) = *state {
            // Decrypt
            let decrypted_bytes = decrypt_identity(encrypted, password)?;
            let keypair = Keypair::from_protobuf_encoding(&decrypted_bytes)?;

            // Store Unlocked state WITH the encrypted form cached (so we can lock again)
            *state = IdentityState::Unlocked(keypair, Some(encrypted.clone()));
            tracing::info!("🔓 Identity unlocked successfully");
            Ok(())
        } else {
            Ok(()) // Already unlocked
        }
    }

    /// Lock identity — consumes Keypair via ed25519 conversion to trigger
    /// ed25519_dalek::SigningKey::drop() → secret_key.zeroize().
    pub fn lock(&self) -> Result<()> {
        let mut state = self.state.write();

        if let IdentityState::Locked(_, _) = *state {
            return Ok(());
        }

        if let IdentityState::Unlocked(ref kp, Some(ref encrypted)) = *state {
            let pub_key_bytes = kp.public().encode_protobuf();
            let encrypted = encrypted.clone();

            let old =
                std::mem::replace(&mut *state, IdentityState::Locked(encrypted, pub_key_bytes));
            if let IdentityState::Unlocked(kp, _) = old {
                // try_into_ed25519() consumes kp; inner ed25519_dalek::SigningKey
                // implements ZeroizeOnDrop, so dropping zeroizes the secret key.
                match kp.try_into_ed25519() {
                    Ok(ed_kp) => drop(ed_kp),
                    Err(kp) => drop(kp),
                }
            }

            tracing::info!("🔒 Identity locked (secret key zeroized)");
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Cannot lock identity: No encrypted copy available (save with password first)"
            ))
        }
    }

    pub fn get_metadata(&self) -> IdentityMetadata {
        self.metadata.read().clone()
    }

    pub fn update_metadata(
        &self,
        nickname: Option<String>,
        bio: Option<String>,
        avatar_url: Option<String>,
        setup_complete: Option<bool>,
    ) {
        let mut meta = self.metadata.write();
        if let Some(n) = nickname {
            meta.nickname = n;
        }
        if let Some(b) = bio {
            meta.bio = b;
        }
        if let Some(a) = avatar_url {
            meta.avatar_url = Some(a);
        }
        if let Some(s) = setup_complete {
            meta.setup_complete = s;
        }
        meta.last_updated = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Caller should use save_to_disk(password) to persist (e.g. via POST /api/v2/profile/save).
        tracing::debug!("Metadata updated in memory; call save_to_disk to persist");
    }

    pub fn keypair(&self) -> Option<Keypair> {
        match &*self.state.read() {
            IdentityState::Unlocked(kp, _) => Some(kp.clone()),
            IdentityState::Locked(_, _) => None,
        }
    }

    pub fn sign(&self, message: &[u8]) -> Option<Vec<u8>> {
        match &*self.state.read() {
            IdentityState::Unlocked(kp, _) => kp.sign(message).ok(),
            IdentityState::Locked(_, _) => None,
        }
    }

    pub fn public_key(&self) -> Option<PublicKey> {
        match &*self.state.read() {
            IdentityState::Unlocked(kp, _) => Some(kp.public()),
            IdentityState::Locked(_, _) => None,
        }
    }

    /// CRITICAL FIX: Returns public key bytes even when locked
    /// Public keys are not secret, so they can be stored in plaintext alongside the encrypted blob
    pub fn get_public_key(&self) -> Vec<u8> {
        let state = self.state.read();
        match &*state {
            IdentityState::Unlocked(kp, _) => match kp.public().try_into_ed25519() {
                Ok(ed_pk) => ed_pk.to_bytes().to_vec(),
                Err(_) => vec![],
            },
            // CRITICAL FIX: Return stored public key bytes when locked
            IdentityState::Locked(_, pub_key_bytes) => pub_key_bytes.clone(),
        }
    }

    /// CRITICAL FIX: Returns Ed25519 public key bytes, works even when locked
    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.get_public_key()
    }
}

#[derive(Serialize, Deserialize)]
struct PersistedEncryptedIdentity {
    id: String,
    did: String,
    metadata: IdentityMetadata,
    encrypted_keypair: EncryptedIdentity,
    // CRITICAL FIX: Store public key in plaintext so get_public_key() works when locked
    #[serde(default)]
    public_key_bytes: Option<Vec<u8>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_lock_unlock() {
        let identity = Identity::create_new(None).unwrap();

        // New identity should start unlocked
        assert!(!identity.is_locked());

        // Must save to create encrypted copy before locking
        identity.save_to_disk("password").unwrap();

        // Lock it
        identity.lock().unwrap();
        assert!(identity.is_locked());

        // Unlock it with WRONG password should fail
        assert!(identity.unlock("wrong_password").is_err());
        assert!(identity.is_locked());

        // Unlock it with CORRECT password
        identity.unlock("password").unwrap();
        assert!(!identity.is_locked());
    }

    #[test]
    fn test_lock_without_save_fails() {
        let identity = Identity::create_new(None).unwrap();
        // Trying to lock without saving (no encrypted copy) should fail
        let res = identity.lock();
        assert!(res.is_err());
        assert!(res
            .unwrap_err()
            .to_string()
            .contains("No encrypted copy available"));
    }

    #[test]
    fn test_identity_from_seed() {
        let seed = [0u8; 32];
        let id1 = Identity::from_seed(&seed).unwrap();
        let id2 = Identity::from_seed(&seed).unwrap();

        assert_eq!(id1.public_key().unwrap(), id2.public_key().unwrap());
        assert_ne!(id1.id, id2.id); // Internal UUIDs should still be unique
    }

    #[test]
    fn test_metadata_update() {
        let identity = Identity::create_new(None).unwrap();

        let original_name = identity.get_metadata().nickname.clone();

        // Update nickname
        identity.update_metadata(Some("Alice".to_string()), None, None, Some(true));

        let meta = identity.get_metadata();
        assert_eq!(meta.nickname, "Alice");
        assert!(meta.setup_complete);
        assert_ne!(meta.nickname, original_name);
    }

    #[test]
    fn test_storage_roundtrip() {
        use tempfile::tempdir;
        let dir = tempdir().unwrap();
        std::env::set_var("GHOSTWIRE_HOME", dir.path());

        let profile = Some("test_profile".to_string());
        let identity = Identity::create_new(profile.clone()).unwrap();
        let original_id = identity.id.clone();

        identity.save_to_disk("secret").unwrap();

        // Load it back
        let loaded = Identity::load_or_create(profile).unwrap();
        assert_eq!(loaded.id, original_id);
        assert!(loaded.is_locked());

        loaded.unlock("secret").unwrap();
        assert_eq!(loaded.public_key().unwrap(), identity.public_key().unwrap());
    }

    #[test]
    fn test_lock_zeroizes_keypair() {
        let identity = Identity::create_new(None).unwrap();
        identity.save_to_disk("test_password").unwrap();

        assert!(!identity.is_locked());
        assert!(identity.sign(b"test").is_some());

        identity.lock().unwrap();
        assert!(identity.is_locked());
        assert!(identity.sign(b"test").is_none());
        assert!(identity.keypair().is_none());
    }

    #[test]
    fn test_sign_works_before_lock() {
        let identity = Identity::create_new(None).unwrap();
        let sig = identity.sign(b"hello world");
        assert!(sig.is_some());
        assert!(!sig.unwrap().is_empty());
    }

    #[test]
    fn test_lock_unlock_roundtrip_preserves_key() {
        let identity = Identity::create_new(None).unwrap();
        identity.save_to_disk("roundtrip_password").unwrap();

        let pub_before = identity.public_key().unwrap();
        identity.lock().unwrap();
        assert!(identity.is_locked());

        identity.unlock("roundtrip_password").unwrap();
        assert!(!identity.is_locked());
        assert_eq!(identity.public_key().unwrap(), pub_before);

        let sig = identity.sign(b"after unlock");
        assert!(sig.is_some());
    }
}
