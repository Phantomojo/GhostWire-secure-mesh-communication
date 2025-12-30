use crate::core::encryption::EncryptionManager;
use anyhow::Result;
use libp2p::identity::{Keypair, PublicKey};
use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};
use std::sync::RwLock;

pub struct EphemeralIdentity {
    pub identity_id: String,
    secret_key: Vec<u8>,
}

impl EphemeralIdentity {
    pub fn new() -> Result<Self> {
        let mut secret_key = vec![0u8; 32];
        let rng = SystemRandom::new();
        rng.fill(&mut secret_key)
            .map_err(|e| anyhow::anyhow!("RNG error: {:?}", e))?;
        let mut id_bytes = [0u8; 8];
        rng.fill(&mut id_bytes[..])
            .map_err(|e| anyhow::anyhow!("RNG error: {:?}", e))?;
        let identity_id = format!("id_{:x}", u64::from_le_bytes(id_bytes));
        Ok(EphemeralIdentity {
            identity_id,
            secret_key,
        })
    }

    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        // Simple hash-based signature for demo
        let mut signature = Vec::new();
        for (i, &byte) in message.iter().enumerate() {
            let key_byte = self.secret_key[i % self.secret_key.len()];
            signature.push(byte.wrapping_add(key_byte));
        }
        signature
    }

    pub fn verify(&self, signed_message: &[u8], _public_key: &[u8]) -> Result<Vec<u8>> {
        // For demo purposes, just return the message as-is
        Ok(signed_message.to_vec())
    }

    pub fn public_key_bytes(&self) -> Vec<u8> {
        // Return a simple public key for demo
        self.secret_key.clone()
    }
}

impl Clone for EphemeralIdentity {
    fn clone(&self) -> Self {
        EphemeralIdentity {
            identity_id: self.identity_id.clone(),
            secret_key: self.secret_key.clone(),
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
}

#[derive(Debug, Serialize)]
pub struct Identity {
    pub id: String,
    pub did: String, // did:ghost:<pk_hash>
    metadata: RwLock<IdentityMetadata>,
    #[serde(skip_serializing, skip_deserializing)]
    keypair: Keypair,
}

impl Identity {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Self::load_or_create()
    }

    pub fn load_or_create() -> Result<Self, Box<dyn std::error::Error>> {
        let storage_path = Self::get_storage_path()?;

        if storage_path.exists() {
            match Self::load_from_disk(&storage_path) {
                Ok(identity) => {
                    tracing::info!("🔑 Loaded persistent identity: {}", identity.id);
                    return Ok(identity);
                }
                Err(e) => {
                    tracing::warn!(
                        "⚠️ Failed to load identity (corrupt?): {}. Creating new one.",
                        e
                    );
                    // Backup corrupt file? For now just overwrite.
                }
            }
        }

        tracing::info!("🆕 Creating new persistent identity...");
        let identity = Self::create_new()?;
        identity.save_to_disk()?;
        Ok(identity)
    }

    fn create_new() -> Result<Self, Box<dyn std::error::Error>> {
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
        };

        Ok(Identity {
            id,
            did,
            metadata: RwLock::new(metadata),
            keypair,
        })
    }

    fn get_storage_path() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
        let home = dirs::home_dir().ok_or("Could not find home directory")?;
        let ghost_dir = home.join(".ghostwire");
        if !ghost_dir.exists() {
            std::fs::create_dir_all(&ghost_dir)?;
        }
        Ok(ghost_dir.join("identity.json"))
    }

    fn load_from_disk(path: &std::path::PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let persisted: PersistedIdentity = EncryptionManager::secure_load(path)?;

        // Decode keypair from protobuf bytes
        let keypair = Keypair::from_protobuf_encoding(&persisted.keypair_bytes)
            .map_err(|e| format!("Invalid keypair data: {:?}", e))?;

        Ok(Identity {
            id: persisted.id,
            did: persisted.did,
            metadata: RwLock::new(persisted.metadata),
            keypair,
        })
    }

    pub fn save_to_disk(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::get_storage_path()?;

        let keypair_bytes = self
            .keypair
            .to_protobuf_encoding()
            .map_err(|e| format!("Failed to encode keypair: {:?}", e))?;

        let persisted = PersistedIdentity {
            id: self.id.clone(),
            did: self.did.clone(),
            metadata: self.metadata.read().unwrap().clone(),
            keypair_bytes,
        };

        EncryptionManager::secure_save(&path, &persisted)?;
        Ok(())
    }

    pub fn get_metadata(&self) -> IdentityMetadata {
        self.metadata.read().unwrap().clone()
    }

    pub fn update_metadata(
        &self,
        nickname: Option<String>,
        bio: Option<String>,
        avatar_url: Option<String>,
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
        meta.last_updated = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Auto-save on update
        if let Err(e) = self.save_to_disk() {
            tracing::error!("Failed to save identity after metadata update: {}", e);
        }
    }

    pub fn keypair(&self) -> &Keypair {
        &self.keypair
    }

    pub fn public_key(&self) -> PublicKey {
        self.keypair.public()
    }
}

#[derive(Serialize, Deserialize)]
struct PersistedIdentity {
    id: String,
    did: String,
    metadata: IdentityMetadata,
    keypair_bytes: Vec<u8>,
}
