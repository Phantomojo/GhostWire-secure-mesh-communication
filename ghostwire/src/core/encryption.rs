//! Encryption: key revocation, rotation announcements, CRLs.
//! Next: integrate with key manager and web API.

use crate::core::signal::SignalManager;
use ring::aead::{self, UnboundKey, AES_256_GCM};
use ring::agreement::{self, EphemeralPrivateKey};
use ring::rand::{SecureRandom, SystemRandom};
use ring::signature::{Ed25519KeyPair, KeyPair};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, error, info, warn};

/// Enhanced encrypted message with additional security features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedMessage {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
    pub signature: Vec<u8>,
    pub sender_id: String,
    pub timestamp: u64,
    pub key_id: String,
    pub message_type: MessageType,
    pub version: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Text,
    Binary,
    Control,
    Heartbeat,
}

/// Enhanced key pair with additional metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GhostKeyPair {
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
    pub key_id: String,
    pub created_at: u64,
    pub expires_at: Option<u64>,
    pub usage_count: usize,
    pub key_type: KeyType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyType {
    Ed25519,
    X25519,
    Hybrid, // Both Ed25519 and X25519
}

/// Enhanced peer key information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerKey {
    pub public_key: Vec<u8>,
    pub key_id: String,
    pub last_updated: u64,
    pub key_type: KeyType,
    pub is_verified: bool,
    pub trust_score: f32,
}

/// Struct for tracking revoked keys.
#[derive(Default, Clone)]
pub struct RevocationList {
    pub revoked_keys: Vec<String>,
}

/// Trait for announcing key rotations.
pub trait KeyRotationAnnouncer {
    /// Announce a key rotation event.
    fn announce_rotation(&self, key_id: &str);
}

/// Default stub: no-op.
pub struct NoOpKeyRotationAnnouncer;
impl KeyRotationAnnouncer for NoOpKeyRotationAnnouncer {
    fn announce_rotation(&self, _key_id: &str) {}
}

/// Enhanced encryption manager with advanced security features
pub struct EncryptionManager {
    key_pair: GhostKeyPair,
    peer_keys: Arc<Mutex<HashMap<String, PeerKey>>>,
    rng: SystemRandom,
    session_keys: Arc<Mutex<HashMap<String, SessionKey>>>,
    key_rotation_interval: u64,
    last_key_rotation: u64,
    signal_manager: SignalManager,
}

#[derive(Debug, Clone)]
pub struct SessionKey {
    pub key: Vec<u8>,
    pub created_at: u64,
    pub expires_at: u64,
    pub usage_count: usize,
}

/// Trait for managing encryption key lifecycle (rotation, revocation, backup).
pub trait KeyManager {
    /// Rotate keys (periodic or on-demand).
    fn rotate(&mut self);
    /// Revoke a key by ID.
    fn revoke(&mut self, key_id: &str);
    /// Backup keys (encrypted export).
    fn backup(&self, passphrase: &str) -> Vec<u8>;
    /// Restore keys from backup.
    fn restore(&mut self, data: &[u8], passphrase: &str);
}

/// Stub implementation: does nothing.
pub struct StubKeyManager;

impl KeyManager for StubKeyManager {
    fn rotate(&mut self) {}
    fn revoke(&mut self, _key_id: &str) {}
    fn backup(&self, _passphrase: &str) -> Vec<u8> {
        vec![]
    }
    fn restore(&mut self, _data: &[u8], _passphrase: &str) {}
}

impl EncryptionManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let rng = SystemRandom::new();

        // Generate hybrid key pair (both Ed25519 and X25519)
        let key_pair = Self::generate_hybrid_key_pair(&rng)?;
        let identity_key = key_pair.public_key.clone();

        Ok(EncryptionManager {
            key_pair,
            peer_keys: Arc::new(Mutex::new(HashMap::new())),
            rng,
            session_keys: Arc::new(Mutex::new(HashMap::new())),
            key_rotation_interval: 86400 * 7, // 7 days
            last_key_rotation: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            signal_manager: SignalManager::new(identity_key),
        })
    }

    fn generate_hybrid_key_pair(
        rng: &SystemRandom,
    ) -> Result<GhostKeyPair, Box<dyn std::error::Error>> {
        // Generate Ed25519 key pair for signing
        let ed25519_key_pair_bytes = Ed25519KeyPair::generate_pkcs8(rng)
            .map_err(|e| format!("Failed to generate Ed25519 key pair: {:?}", e))?;
        let _ed25519_key_pair = Ed25519KeyPair::from_pkcs8(ed25519_key_pair_bytes.as_ref())
            .map_err(|e| format!("Failed to create Ed25519 key pair: {:?}", e))?;

        // Generate X25519 key pair for encryption
        let x25519_private_key = EphemeralPrivateKey::generate(&agreement::X25519, rng)
            .map_err(|e| format!("Failed to generate X25519 private key: {:?}", e))?;
        let x25519_public_key = x25519_private_key
            .compute_public_key()
            .map_err(|e| format!("Failed to compute X25519 public key: {:?}", e))?;

        // Combine both keys
        let combined_private_key = ed25519_key_pair_bytes.as_ref().to_vec();
        // Note: In a real implementation, you'd properly extract and combine the private keys
        // For now, we'll use the Ed25519 key as the primary key

        let key_pair = GhostKeyPair {
            public_key: _ed25519_key_pair.public_key().as_ref().to_vec(),
            private_key: combined_private_key,
            key_id: Self::generate_key_id(),
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            expires_at: Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    + 86400 * 30,
            ), // 30 days
            usage_count: 0,
            key_type: KeyType::Hybrid,
        };

        Ok(key_pair)
    }

    fn generate_key_id() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        format!("key_{:x}", timestamp)
    }

    pub fn get_public_key(&self) -> &[u8] {
        &self.key_pair.public_key
    }

    pub fn get_key_id(&self) -> String {
        self.key_pair.key_id.clone()
    }

    pub fn add_peer_key(&self, peer_id: String, public_key: Vec<u8>, key_type: KeyType) {
        let mut peer_keys = self.peer_keys.lock().unwrap();
        peer_keys.insert(
            peer_id,
            PeerKey {
                public_key,
                key_id: Self::generate_key_id(),
                last_updated: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                key_type,
                is_verified: false,
                trust_score: 0.5, // Default trust score
            },
        );
    }

    pub fn encrypt_message(
        &self,
        message: &str,
        recipient_id: &str,
        message_type: MessageType,
    ) -> Result<EncryptedMessage, Box<dyn std::error::Error>> {
        // Try Signal encryption first if session exists
        if let Ok(signal_msg) = self
            .signal_manager
            .encrypt_with_signal(recipient_id, message.as_bytes())
        {
            return Ok(signal_msg);
        }

        let peer_keys = self.peer_keys.lock().unwrap();
        let peer_key = peer_keys.get(recipient_id).ok_or("Recipient not found")?;

        // Derive session key
        let session_key = self.derive_session_key(recipient_id, &peer_key.public_key)?;

        // Encrypt the message using AES-256-GCM
        let encrypted_data = self.encrypt_with_aes_gcm(message.as_bytes(), &session_key)?;

        // Sign the encrypted message
        let signature = self.sign_message(&encrypted_data.ciphertext)?;

        // Update usage count
        self.increment_key_usage();

        Ok(EncryptedMessage {
            ciphertext: encrypted_data.ciphertext,
            nonce: encrypted_data.nonce,
            signature,
            sender_id: self.key_pair.key_id.clone(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            key_id: self.key_pair.key_id.clone(),
            message_type,
            version: 0x02, // Current version
        })
    }

    pub fn decrypt_message(
        &self,
        encrypted_msg: &EncryptedMessage,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Get peer public key for verification
        let peer_keys = self.peer_keys.lock().unwrap();
        let peer_key = peer_keys
            .get(&encrypted_msg.sender_id)
            .ok_or("Sender not found in peer keys")?;
        let peer_public_key = peer_key.public_key.clone();
        drop(peer_keys); // Release lock

        // Verify signature first
        self.verify_signature(
            &encrypted_msg.ciphertext,
            &encrypted_msg.signature,
            &peer_public_key,
        )?;

        // Check message version
        if encrypted_msg.version == 0x03 {
            // Signal protocol decryption
            let decrypted = self
                .signal_manager
                .decrypt_with_signal(&encrypted_msg.sender_id, encrypted_msg)
                .map_err(|e| format!("Signal decryption failed: {}", e))?;
            return String::from_utf8(decrypted).map_err(|e| e.into());
        }

        if encrypted_msg.version != 0x02 {
            return Err("Unsupported message version".into());
        }

        // Derive session key for decryption
        let session_key = self.derive_session_key(&encrypted_msg.sender_id, &peer_public_key)?;

        // Decrypt using AES-256-GCM
        let decrypted_data = self.decrypt_with_aes_gcm(
            &encrypted_msg.ciphertext,
            &encrypted_msg.nonce,
            &session_key,
        )?;

        String::from_utf8(decrypted_data).map_err(|e| e.into())
    }

    fn encrypt_with_aes_gcm(
        &self,
        data: &[u8],
        key: &[u8],
    ) -> Result<AesGcmResult, Box<dyn std::error::Error>> {
        // Ensure key is exactly 32 bytes for AES-256
        if key.len() != 32 {
            return Err("Key must be exactly 32 bytes for AES-256-GCM".into());
        }

        // Create unbound key
        let unbound_key = UnboundKey::new(&AES_256_GCM, key)
            .map_err(|e| format!("Failed to create AES-256-GCM key: {:?}", e))?;

        // Generate random nonce (96 bits = 12 bytes for GCM)
        let nonce_bytes = self.generate_nonce();
        let nonce = aead::Nonce::assume_unique_for_key(nonce_bytes);

        // Create LessSafeKey for encryption
        let key = aead::LessSafeKey::new(unbound_key);

        // Prepare data for in-place encryption
        let mut in_out = data.to_vec();

        // Encrypt in place (appends authentication tag)
        key.seal_in_place_append_tag(nonce, aead::Aad::empty(), &mut in_out)
            .map_err(|e| format!("AES-GCM encryption failed: {:?}", e))?;

        Ok(AesGcmResult {
            ciphertext: in_out,
            nonce: nonce_bytes.to_vec(),
        })
    }

    fn decrypt_with_aes_gcm(
        &self,
        ciphertext: &[u8],
        nonce: &[u8],
        key: &[u8],
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Ensure key is exactly 32 bytes for AES-256
        if key.len() != 32 {
            return Err("Key must be exactly 32 bytes for AES-256-GCM".into());
        }

        // Ensure nonce is exactly 12 bytes
        if nonce.len() != 12 {
            return Err("Nonce must be exactly 12 bytes for GCM".into());
        }

        // Create unbound key
        let unbound_key = UnboundKey::new(&AES_256_GCM, key)
            .map_err(|e| format!("Failed to create AES-256-GCM key: {:?}", e))?;

        // Convert nonce bytes to Nonce
        let mut nonce_array = [0u8; 12];
        nonce_array.copy_from_slice(nonce);
        let nonce = aead::Nonce::assume_unique_for_key(nonce_array);

        // Create Opening key
        let mut opening_key = aead::LessSafeKey::new(unbound_key);

        // Decrypt in place (verifies and removes authentication tag)
        let mut in_out = ciphertext.to_vec();
        let plaintext = opening_key
            .open_in_place(nonce, aead::Aad::empty(), &mut in_out)
            .map_err(|e| {
                format!(
                    "AES-GCM decryption failed (possibly tampered data): {:?}",
                    e
                )
            })?;

        Ok(plaintext.to_vec())
    }

    fn derive_session_key(
        &self,
        peer_id: &str,
        peer_public_key: &[u8],
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Check if we have a cached session key
        let mut session_keys = self.session_keys.lock().unwrap();
        if let Some(session_key) = session_keys.get(peer_id) {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            if now < session_key.expires_at {
                return Ok(session_key.key.clone());
            }
        }

        // Derive new session key using HKDF
        let mut key = [0u8; 32];
        let mut hasher = ring::digest::Context::new(&ring::digest::SHA256);
        hasher.update(b"GhostWire-Session-Key-v1");

        // WARNING: INSECURE STUB IMPLEMENTATION
        // We are strictly for testing/debugging purposes using a symmetric derivation
        // derived ONLY from public keys because we lack X25519 Private Key storage
        // in current GhostKeyPair. Real implementation MUST use ECDH (X25519).
        // This allows Alice and Bob to derive the same key.

        let my_pub = &self.key_pair.public_key;
        let peer_pub = peer_public_key;

        // Deterministic ordering to ensure symmetry
        if my_pub < &peer_pub.to_vec() {
            hasher.update(my_pub);
            hasher.update(peer_pub);
        } else {
            hasher.update(peer_pub);
            hasher.update(my_pub);
        }

        // Remove random nonce usage!
        key.copy_from_slice(hasher.finish().as_ref());

        // Cache the session key
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        session_keys.insert(
            peer_id.to_string(),
            SessionKey {
                key: key.to_vec(),
                created_at: now,
                expires_at: now + 3600, // 1 hour
                usage_count: 0,
            },
        );

        Ok(key.to_vec())
    }

    // Enhanced encryption method for CLI compatibility
    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        // Derive a strong 32-byte key from our key pair
        let key = self.derive_cli_key();

        match self.encrypt_with_aes_gcm(data, &key) {
            Ok(result) => {
                // Combine nonce and ciphertext
                let mut combined = result.nonce;
                combined.extend_from_slice(&result.ciphertext);
                combined
            }
            Err(e) => {
                error!("CLI encryption failed: {}", e);
                panic!(
                    "Encryption failed - this should never happen in production: {}",
                    e
                );
            }
        }
    }

    // Derive a deterministic but unique 32-byte key for CLI operations
    fn derive_cli_key(&self) -> Vec<u8> {
        use ring::digest;

        // Use SHA-256 to derive a 32-byte key from our keypair
        let mut ctx = digest::Context::new(&digest::SHA256);
        ctx.update(b"GhostWire-CLI-v1"); // Domain separation
        ctx.update(&self.key_pair.private_key);
        ctx.update(&self.key_pair.public_key);

        ctx.finish().as_ref().to_vec()
    }

    // Enhanced decryption method for CLI compatibility
    pub fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        if data.len() < 12 {
            error!("Encrypted data too short (need at least 12 bytes for nonce)");
            panic!("Corrupted or invalid encrypted data");
        }

        // Derive the same key we used for encryption
        let key = self.derive_cli_key();

        // Extract nonce and ciphertext
        let nonce = &data[..12];
        let ciphertext = &data[12..];

        match self.decrypt_with_aes_gcm(ciphertext, nonce, &key) {
            Ok(decrypted) => decrypted,
            Err(e) => {
                error!("CLI decryption failed: {}", e);
                panic!(
                    "Decryption failed - data may be corrupted or tampered: {}",
                    e
                );
            }
        }
    }

    // Enhanced signature methods
    pub fn sign_message(&self, message: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Use Ed25519 for signing
        let key_pair_bytes = &self.key_pair.private_key;
        let key_pair = Ed25519KeyPair::from_pkcs8(key_pair_bytes)
            .map_err(|e| format!("Failed to create key pair for signing: {:?}", e))?;

        let signature = key_pair.sign(message);
        Ok(signature.as_ref().to_vec())
    }

    pub fn verify_signature(
        &self,
        message: &[u8],
        signature: &[u8],
        public_key: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Verify Ed25519 signature
        let public_key =
            ring::signature::UnparsedPublicKey::new(&ring::signature::ED25519, public_key);

        public_key
            .verify(message, signature)
            .map_err(|e| format!("Signature verification failed: {:?}", e).into())
    }

    fn generate_nonce(&self) -> [u8; 12] {
        let mut nonce = [0u8; 12];
        self.rng.fill(&mut nonce).unwrap();
        nonce
    }

    fn increment_key_usage(&self) {
        // In a real implementation, this would be thread-safe
        // For now, we'll just log the usage
        debug!("Key usage incremented for key: {}", self.key_pair.key_id);
    }

    pub fn export_public_key(&self) -> Vec<u8> {
        self.key_pair.public_key.clone()
    }

    pub fn get_peer_count(&self) -> usize {
        self.peer_keys.lock().unwrap().len()
    }

    /// Check if key rotation is needed
    pub fn should_rotate_keys(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        now - self.last_key_rotation > self.key_rotation_interval
    }

    /// Rotate keys if needed
    pub fn rotate_keys_if_needed(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.should_rotate_keys() {
            info!("Rotating encryption keys");
            let new_key_pair = Self::generate_hybrid_key_pair(&self.rng)?;
            self.key_pair = new_key_pair;
            self.last_key_rotation = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            // Clear session keys
            self.session_keys.lock().unwrap().clear();
        }
        Ok(())
    }
}

#[derive(Debug)]
struct AesGcmResult {
    ciphertext: Vec<u8>,
    nonce: Vec<u8>,
}

impl Default for EncryptionManager {
    fn default() -> Self {
        Self::new().expect("Failed to create default EncryptionManager")
    }
}

pub type Encryption = EncryptionManager;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_roundtrip() {
        let alice = EncryptionManager::new().unwrap();
        let bob = EncryptionManager::new().unwrap();

        let message = "Hello, Bob!";

        // Bob needs to exist in Alice's peer keys
        alice.add_peer_key(
            bob.get_key_id(),
            bob.get_public_key().to_vec(),
            KeyType::Hybrid,
        );

        // Alice needs to exist in Bob's peer keys (for Bob to verify/derive key)
        bob.add_peer_key(
            alice.get_key_id(),
            alice.get_public_key().to_vec(),
            KeyType::Hybrid,
        );

        // Alice encrypts for Bob
        let encrypted = alice
            .encrypt_message(message, &bob.get_key_id(), MessageType::Text)
            .unwrap();

        // Bob decrypts
        let decrypted = bob.decrypt_message(&encrypted).unwrap();

        assert_eq!(message, decrypted);
    }

    #[test]
    fn test_key_generation() {
        let encryption = EncryptionManager::new().unwrap();
        assert!(!encryption.get_public_key().is_empty());
        assert!(!encryption.get_key_id().is_empty());
    }
}
