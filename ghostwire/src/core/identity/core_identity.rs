use crate::core::identity::encryption::{decrypt_identity, encrypt_identity, EncryptedIdentity};
use anyhow::Result;
use libp2p::identity::{Keypair, PublicKey};
use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};
use std::sync::RwLock;

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
        let public_key = PublicKey::from_protobuf_encoding(public_key_bytes)
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
    Locked(EncryptedIdentity),
    Unlocked(Keypair, Option<EncryptedIdentity>),
}

#[derive(Debug, Serialize)]
pub struct Identity {
    pub id: String,
    pub did: String, // did:ghost:<pk_hash>
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
                    tracing::info!("🔑 Loaded persistent identity: {}", identity.id);
                    // Inject profile
                    identity.profile = profile;
                    return Ok(identity);
                }
                Err(e) => {
                    tracing::warn!(
                        "⚠️ Failed to load identity (corrupt or old format): {}. Creating new one.",
                        e
                    );
                    // TODO: Handle migration or backup
                }
            }
        }

        tracing::info!("🆕 Creating new persistent identity...");
        let identity = Self::create_new(profile)?;
        // Save initially with default/empty password or prompt?
        // For now, we save as unlocked -> needs to be encrypted on first lock/save.
        // Or we require a password to create?
        // Let's assume default behavior is "Unlocked" until user sets a password.
        // But for this phase, we want to enforce encryption.
        // We'll treat new identities as Unlocked.
        // Saving will fail if we don't have a password...
        // For MVP, allow save without password (unencrypted) OR use a default.
        // ACTUALLY: Let's use a temporary "unencrypted" mode if no password set?
        // No, let's enforce a default password for auto-created ones (e.g. "ghostwire")
        // and prompt user to change it.
        identity.save_to_disk("ghostwire")?;
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

    fn get_storage_path(
        profile: Option<String>,
    ) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
        let home = dirs::home_dir().ok_or("Could not find home directory")?;
        let ghost_dir = home.join(".ghostwire");
        if !ghost_dir.exists() {
            std::fs::create_dir_all(&ghost_dir)?;
        }
        let filename = match profile {
            Some(p) => format!("identity_{}.json", p),
            None => "identity.json".to_string(),
        };
        Ok(ghost_dir.join(filename))
    }

    fn load_from_disk(path: &std::path::PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        // Read file contents
        let file_content = std::fs::read(path)?;

        // Try to deserialize as new EncryptedIdentity format
        if let Ok(persisted) = serde_json::from_slice::<PersistedEncryptedIdentity>(&file_content) {
            return Ok(Identity {
                id: persisted.id,
                did: persisted.did,
                metadata: RwLock::new(persisted.metadata),
                state: RwLock::new(IdentityState::Locked(persisted.encrypted_keypair)),
                profile: None, // Will be set by caller
            });
        }

        // Fallback: Try old format (migration)
        // For now, return error if not new format
        Err("Unknown identity format".into())
    }

    pub fn save_to_disk(&self, password: &str) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::get_storage_path(self.profile.clone())?;
        let mut state_guard = self.state.write().unwrap();

        let keypair_bytes = match &*state_guard {
            IdentityState::Unlocked(kp, _) => {
                kp.to_protobuf_encoding().map_err(|e| anyhow::anyhow!(e))?
            }
            IdentityState::Locked(_) => {
                return Err("Cannot save locked identity (need keypair)".into())
            }
        };

        // Encrypt the keypair bytes
        let encrypted_keypair = encrypt_identity(&keypair_bytes, password)?;

        // Important: Update the cached encrypted identity so we can lock later!
        if let IdentityState::Unlocked(_, ref mut cached_encrypted) = *state_guard {
            *cached_encrypted = Some(encrypted_keypair.clone());
        }

        let persisted = PersistedEncryptedIdentity {
            id: self.id.clone(),
            did: self.did.clone(),
            metadata: self.metadata.read().unwrap().clone(),
            encrypted_keypair,
        };

        let json = serde_json::to_string_pretty(&persisted)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Check if identity is currently locked
    pub fn is_locked(&self) -> bool {
        let state = self.state.read().unwrap();
        match *state {
            IdentityState::Locked(_) => true,
            _ => false,
        }
    }

    /// Unlock identity with password
    pub fn unlock(&self, password: &str) -> Result<()> {
        let mut state = self.state.write().unwrap();

        if let IdentityState::Locked(ref encrypted) = *state {
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

    /// Lock identity (clear sensitive data from memory)
    pub fn lock(&self) -> Result<()> {
        let mut state = self.state.write().unwrap();

        // Use a temporary swap to avoid borrow checker issues if we were to match directly
        let new_state = if let IdentityState::Unlocked(_, Some(ref encrypted)) = *state {
            Some(IdentityState::Locked(encrypted.clone()))
        } else {
            None
        };

        if let Some(s) = new_state {
            *state = s;
            tracing::info!("🔒 Identity locked");
            Ok(())
        } else if let IdentityState::Locked(_) = *state {
            Ok(()) // Already locked
        } else {
            Err(anyhow::anyhow!(
                "Cannot lock identity: No encrypted copy available (save with password first)"
            ))
        }
    }

    pub fn get_metadata(&self) -> IdentityMetadata {
        self.metadata.read().unwrap().clone()
    }

    pub fn update_metadata(
        &self,
        nickname: Option<String>,
        bio: Option<String>,
        avatar_url: Option<String>,
        setup_complete: Option<bool>,
    ) {
        let mut meta = self.metadata.write().unwrap();
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

        // Auto-save on update (using default/cached password?)
        // TODO: Need password to save!
        // For now, we warn.
        tracing::warn!("Metadata updated but not saved to disk (password required)");
        // In real impl, checking if we have cached password or if we can save metadata separately.
    }

    pub fn keypair(&self) -> Option<Keypair> {
        match &*self.state.read().unwrap() {
            IdentityState::Unlocked(kp, _) => Some(kp.clone()),
            IdentityState::Locked(_) => None,
        }
    }

    pub fn public_key(&self) -> Option<PublicKey> {
        match &*self.state.read().unwrap() {
            IdentityState::Unlocked(kp, _) => Some(kp.public()),
            IdentityState::Locked(_) => None,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct PersistedEncryptedIdentity {
    id: String,
    did: String,
    metadata: IdentityMetadata,
    encrypted_keypair: EncryptedIdentity,
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

        // Unlock it
        identity.unlock("password").unwrap();
        assert!(!identity.is_locked());
    }

    #[test]
    fn test_metadata_update() {
        let identity = Identity::create_new(None).unwrap();

        let original_name = identity.get_metadata().nickname.clone();

        // Update nickname
        identity.update_metadata(Some("Alice".to_string()), None, None, None);

        let updated_name = identity.get_metadata().nickname;
        assert_eq!(updated_name, "Alice");
        assert_ne!(updated_name, original_name);
    }
}
