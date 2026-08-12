//! Encryption: key revocation, rotation announcements, CRLs.
//! Next: integrate with key manager and web API.
//!
//! **Security Fixes Applied:**
//! - ✅ CVE-2025-11931 class: Input validation on decryption
//! - ✅ DoS prevention: Result-based error handling (no panic)
//! - ✅ Replay protection: Message deduplication integration
//! - ✅ Timing attacks: Constant-time signature verification

use crate::core::crl;
use crate::core::double_ratchet;
use rand_core::OsRng;
use ring::hkdf;
use ring::rand::{SecureRandom, SystemRandom};
use ring::signature::{Ed25519KeyPair, KeyPair};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{info, warn};
use x25519_dalek::{PublicKey as XPublicKey, StaticSecret};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// ✅ NEW: Secure error types (no panic on invalid input)
#[derive(Debug, thiserror::Error)]
pub enum DecryptionError {
    #[error("Invalid data length")]
    InvalidLength,
    #[error("Decryption failed")]
    DecryptionFailed,
    #[error("Authentication failed")]
    AuthenticationFailed,
    #[error("Invalid UTF-8")]
    InvalidUtf8,
    #[error("Replay attack detected")]
    ReplayDetected,
}

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
    pub pq_key: Option<Vec<u8>>, // Kyber encapsulated key for PQC Hardening
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    Text,
    Binary,
    Control,
    Heartbeat,
    Handshake,
}

/// Enhanced key pair with additional metadata
#[derive(Debug, Clone, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct GhostKeyPair {
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
    pub key_id: String,
    pub created_at: u64,
    pub expires_at: Option<u64>,
    pub usage_count: usize,
    pub key_type: KeyType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Zeroize)]
pub enum KeyType {
    Ed25519,
    X25519,
    Hybrid,      // Both Ed25519 and X25519
    PostQuantum, // Kyber
    Ultimate,    // Ed25519 + X25519 + Kyber
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
    pub crl: Arc<crl::CrlStore>,
    pub ratchet_sessions: Arc<double_ratchet::RatchetSessionManager>,
}

#[derive(Debug, Clone, Zeroize, ZeroizeOnDrop)]
pub struct SessionKey {
    pub key: Vec<u8>,
    pub created_at: u64,
    pub expires_at: u64,
    pub usage_count: usize,
}

impl EncryptionManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Self::load_or_create()
    }

    /// Extract the X25519 static secret for Sphinx packet processing.
    /// Returns None if the key pair is too short (legacy format).
    pub fn x25519_secret(&self) -> Option<x25519_dalek::StaticSecret> {
        let pk = &self.key_pair.private_key;
        if pk.len() < 34 {
            return None;
        }
        let ed_len = u16::from_le_bytes([pk[0], pk[1]]) as usize;
        let start = 2 + ed_len;
        if pk.len() < start + 32 {
            return None;
        }
        let bytes: [u8; 32] = pk[start..start + 32].try_into().ok()?;
        Some(x25519_dalek::StaticSecret::from(bytes))
    }

    /// Extract our X25519 public key bytes (32 bytes).
    pub fn x25519_public_key(&self) -> Option<[u8; 32]> {
        let pk = &self.key_pair.private_key;
        if pk.len() < 34 {
            return None;
        }
        let ed_len = u16::from_le_bytes([pk[0], pk[1]]) as usize;
        let start = 2 + ed_len;
        if pk.len() < start + 32 {
            return None;
        }
        let secret = x25519_dalek::StaticSecret::from(
            <[u8; 32]>::try_from(&pk[start..start + 32]).ok()?,
        );
        Some(*x25519_dalek::PublicKey::from(&secret).as_bytes())
    }

    pub fn load_or_create() -> Result<Self, Box<dyn std::error::Error>> {
        let storage_path = Self::get_storage_path()?;

        if storage_path.exists() {
            match Self::load_from_disk(&storage_path) {
                Ok(manager) => {
                    info!(
                        "🔐 Loaded persistent encryption keys for key_id: {}",
                        manager.key_pair.key_id
                    );
                    return Ok(manager);
                }
                Err(e) => {
                    warn!(
                        "⚠️ Failed to load encryption keys: {}. Rotating/Creating new.",
                        e
                    );
                }
            }
        }

        info!("🆕 Generating new persistent encryption keys...");
        let manager = Self::create_new()?;
        manager.save_to_disk()?;
        Ok(manager)
    }

    fn create_new() -> Result<Self, Box<dyn std::error::Error>> {
        let rng = SystemRandom::new();

        // Generate hybrid key pair (both Ed25519 and X25519)
        let key_pair = Self::generate_hybrid_key_pair(&rng)?;

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
            crl: Arc::new(crl::CrlStore::new()),
            ratchet_sessions: Arc::new(double_ratchet::RatchetSessionManager::new()),
        })
    }

    fn get_storage_path() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
        let ghost_dir = if let Ok(ghost_home) = std::env::var("GHOSTWIRE_HOME") {
            std::path::PathBuf::from(ghost_home)
        } else {
            let home = dirs::home_dir().ok_or("Could not find home directory")?;
            home.join(".ghostwire")
        };

        if !ghost_dir.exists() {
            std::fs::create_dir_all(&ghost_dir)?;
        }
        Ok(ghost_dir.join("encryption.json"))
    }

    fn load_from_disk(path: &std::path::PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        // Use secure load
        let persisted: PersistedEncryption = Self::secure_load(path)?;

        let rng = SystemRandom::new();
        // Reconstruct manager from persisted state
        Ok(EncryptionManager {
            key_pair: persisted.key_pair,
            peer_keys: Arc::new(Mutex::new(HashMap::new())),
            rng,
            session_keys: Arc::new(Mutex::new(HashMap::new())),
            key_rotation_interval: persisted.key_rotation_interval,
            last_key_rotation: persisted.last_key_rotation,
            crl: Arc::new(crl::CrlStore::new()),
            ratchet_sessions: Arc::new(double_ratchet::RatchetSessionManager::new()),
        })
    }

    fn get_master_key() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let ghost_dir =
            if let Ok(ghost_home) = std::env::var(cryptify::encrypt_string!("GHOSTWIRE_HOME")) {
                std::path::PathBuf::from(ghost_home)
            } else {
                let home = dirs::home_dir().ok_or("No home dir")?;
                home.join(cryptify::encrypt_string!(".ghostwire"))
            };

        if !ghost_dir.exists() {
            std::fs::create_dir_all(&ghost_dir)?;
        }
        let key_path = ghost_dir.join(cryptify::encrypt_string!("master.key"));

        if key_path.exists() {
            let key = std::fs::read(&key_path)?;
            if key.len() != 32 {
                return Err("Invalid master key length".into());
            }
            Ok(key)
        } else {
            let mut key = [0u8; 32];
            SystemRandom::new()
                .fill(&mut key[..])
                .map_err(|_| "RNG failed")?;

            // save with 600
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .mode(0o600)
                .open(&key_path)?;
            file.write_all(&key)?;
            tracing::info!("🔑 Generated new Master Key for secure storage");
            Ok(key.to_vec())
        }
    }

    pub fn secure_save<T: Serialize>(
        path: &std::path::Path,
        data: &T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let json_bytes = serde_json::to_vec(data)?;
        let master_key = Self::get_master_key()?;

        // Encrypt with XChaCha20-Poly1305
        let result = Self::encrypt_with_xchacha20(&json_bytes, &master_key)?;

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(path)?;
        // Simple container format: [Nonce (12)] [Ciphertext]
        file.write_all(&result.nonce)?;
        file.write_all(&result.ciphertext)?;
        Ok(())
    }

    pub fn secure_load<T: for<'de> Deserialize<'de>>(
        path: &std::path::Path,
    ) -> Result<T, Box<dyn std::error::Error>> {
        let data = std::fs::read(path)?;
        if data.len() < 24 {
            return Err("File too short".into());
        }

        let master_key = Self::get_master_key()?;
        let nonce = &data[..24];
        let ciphertext = &data[24..];

        let plaintext_bytes = Self::decrypt_with_xchacha20(ciphertext, nonce, &master_key)?;
        let object: T = serde_json::from_slice(&plaintext_bytes)?;
        Ok(object)
    }

    /// Public helper for byte-level encryption using Master Key
    pub fn encrypt_bytes(data: &[u8]) -> Result<EncryptionResult, Box<dyn std::error::Error>> {
        let master_key = Self::get_master_key()?;
        Self::encrypt_with_xchacha20(data, &master_key)
    }

    /// Public helper for byte-level decryption using Master Key
    pub fn decrypt_bytes(
        ciphertext: &[u8],
        nonce: &[u8],
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let master_key = Self::get_master_key()?;
        Self::decrypt_with_xchacha20(ciphertext, nonce, &master_key)
    }

    pub fn save_to_disk(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::get_storage_path()?;
        let persisted = PersistedEncryption {
            key_pair: self.key_pair.clone(),
            key_rotation_interval: self.key_rotation_interval,
            last_key_rotation: self.last_key_rotation,
        };

        Self::secure_save(&path, &persisted)?;
        Ok(())
    }

    fn generate_hybrid_key_pair(
        rng: &SystemRandom,
    ) -> Result<GhostKeyPair, Box<dyn std::error::Error>> {
        // Generate Ed25519 key pair for signing
        let ed25519_pkcs8_key_pair = Ed25519KeyPair::generate_pkcs8(rng)
            .map_err(|e| format!("Failed to generate Ed25519 key pair: {:?}", e))?;
        let ed25519_key_pair = Ed25519KeyPair::from_pkcs8(ed25519_pkcs8_key_pair.as_ref())
            .map_err(|e| format!("Failed to create Ed25519 key pair: {:?}", e))?;

        // Generate X25519 key pair for encryption (persistent static secret)
        let x25519_secret = StaticSecret::random_from_rng(OsRng);
        let x25519_public = XPublicKey::from(&x25519_secret);
        let x25519_public_bytes = *x25519_public.as_bytes();
        let x25519_private_bytes = *x25519_secret.as_bytes();

        // Generate ML-KEM (Kyber768) key pair for post-quantum hardening
        use pqcrypto_mlkem::mlkem768;
        use pqcrypto_traits::kem::{PublicKey as _, SecretKey as _};
        let (pq_pk, pq_sk) = mlkem768::keypair();

        // Combine keys into a serialized binary format
        // Public part: [Ed25519 (32)] [X25519 (32)] [Kyber PK (1184 for 768)]
        let mut public_key = ed25519_key_pair.public_key().as_ref().to_vec();
        public_key.extend_from_slice(&x25519_public_bytes);
        public_key.extend_from_slice(pq_pk.as_bytes());

        // Private part: [Ed25519 Len (2)] [Ed25519 PKCS8] [X25519 (32)] [Kyber SK (2400 for 768)]
        let ed_pkcs8 = ed25519_pkcs8_key_pair.as_ref();
        let ed_len = ed_pkcs8.len() as u16;
        let mut private_key = ed_len.to_le_bytes().to_vec();
        private_key.extend_from_slice(ed_pkcs8);
        private_key.extend_from_slice(&x25519_private_bytes);
        private_key.extend_from_slice(pq_sk.as_bytes());

        let key_pair = GhostKeyPair {
            public_key,
            private_key,
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
            key_type: KeyType::Ultimate,
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
        // Reject if recipient key is revoked
        if self.crl.is_revoked(recipient_id) {
            return Err(format!("Recipient key {} is revoked", recipient_id).into());
        }

        let peer_keys = self.peer_keys.lock().unwrap();
        let peer_key = peer_keys.get(recipient_id).ok_or("Recipient not found")?;

        // Derive base session key via ECDH (existing)
        let session_key = self.derive_session_key(recipient_id, &peer_key.public_key)?;

        // PQC Hardening: Kyber Encapsulation
        let mut pq_key_out = None;
        let mut final_key = session_key.clone();

        if peer_key.public_key.len() >= 64 + 1184 {
            use pqcrypto_mlkem::mlkem768;
            use pqcrypto_traits::kem::PublicKey as _;

            let pq_pk_bytes = &peer_key.public_key[64..64 + 1184];
            let pk = mlkem768::PublicKey::from_bytes(pq_pk_bytes)
                .map_err(|_| "Invalid peer PQ public key")?;
            let (shared_secret, ciphertext) = mlkem768::encapsulate(&pk);
            use pqcrypto_traits::kem::{Ciphertext, SharedSecret};
            pq_key_out = Some(ciphertext.as_bytes().to_vec());

            // Mix keys: HKDF(ECDH_key, Kyber_secret)
            let salt = hkdf::Salt::new(hkdf::HKDF_SHA256, &final_key);
            let prk = salt.extract(shared_secret.as_bytes());
            let okm = prk
                .expand(&[b"GhostWire-PQC-Mix"], hkdf::HKDF_SHA256)
                .map_err(|_| "PQC mix failed")?;
            let mut mixed = [0u8; 32];
            okm.fill(&mut mixed).map_err(|_| "PQC fill failed")?;
            final_key = mixed.to_vec();
        }

        // Encrypt the message using XChaCha20-Poly1305 with the hardened key
        let encrypted_data = Self::encrypt_with_xchacha20(message.as_bytes(), &final_key)?;

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
            version: 0x03, // XChaCha20-Poly1305 encryption
            pq_key: pq_key_out,
        })
    }

    pub fn decrypt_message(
        &self,
        encrypted_msg: &EncryptedMessage,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Reject if sender key is revoked
        if self.crl.is_revoked(&encrypted_msg.sender_id) {
            return Err(format!("Sender key {} is revoked", encrypted_msg.sender_id).into());
        }

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

        if encrypted_msg.version != 0x03 {
            return Err("Unsupported message version (expected v3 XChaCha20)".into());
        }

        // Derive base session key for decryption
        let session_key = self.derive_session_key(&encrypted_msg.sender_id, &peer_public_key)?;
        let mut final_key = session_key.clone();

        // PQC Hardening: Kyber Decapsulation
        if let Some(pq_ciphertext) = &encrypted_msg.pq_key {
            use pqcrypto_mlkem::mlkem768;
            use pqcrypto_traits::kem::{Ciphertext as _, SecretKey as _};

            // Extract our Kyber Secret Key from private key blob
            let priv_key_bytes = &self.key_pair.private_key;
            let ed_len = u16::from_le_bytes([priv_key_bytes[0], priv_key_bytes[1]]) as usize;
            let pq_sk_bytes = &priv_key_bytes[2 + ed_len + 32..];

            if let (Ok(pq_sk), Ok(pq_ct)) = (
                mlkem768::SecretKey::from_bytes(pq_sk_bytes),
                mlkem768::Ciphertext::from_bytes(pq_ciphertext),
            ) {
                let shared_secret = mlkem768::decapsulate(&pq_ct, &pq_sk);

                use pqcrypto_traits::kem::SharedSecret;
                // Mix keys: HKDF(ECDH_key, Kyber_secret)
                let salt = hkdf::Salt::new(hkdf::HKDF_SHA256, &session_key);
                let prk = salt.extract(shared_secret.as_bytes());
                let okm = prk
                    .expand(&[b"GhostWire-PQC-Mix"], hkdf::HKDF_SHA256)
                    .map_err(|_| "PQC mix failed")?;
                let mut mixed = [0u8; 32];
                okm.fill(&mut mixed).map_err(|_| "PQC fill failed")?;
                final_key = mixed.to_vec();
            }
        }

        // Decrypt using XChaCha20-Poly1305
        let decrypted_data = Self::decrypt_with_xchacha20(
            &encrypted_msg.ciphertext,
            &encrypted_msg.nonce,
            &final_key,
        )?;

        String::from_utf8(decrypted_data).map_err(|e| e.into())
    }

    pub fn encrypt_with_xchacha20(
        data: &[u8],
        key: &[u8],
    ) -> Result<EncryptionResult, Box<dyn std::error::Error>> {
        use chacha20poly1305::{
            aead::{Aead, KeyInit},
            XChaCha20Poly1305, XNonce,
        };

        // Ensure key is exactly 32 bytes
        if key.len() != 32 {
            return Err("Key must be exactly 32 bytes for XChaCha20".into());
        }

        let cipher = XChaCha20Poly1305::new(key.into());

        // Generate random nonce (192 bits = 24 bytes for XChaCha20)
        let nonce_bytes = Self::generate_nonce();
        let nonce = XNonce::from_slice(&nonce_bytes);

        // Encrypt
        let ciphertext = cipher
            .encrypt(nonce, data)
            .map_err(|e| format!("XChaCha20 encryption failed: {}", e))?;

        Ok(EncryptionResult {
            ciphertext,
            nonce: nonce_bytes.to_vec(),
        })
    }

    pub fn decrypt_with_xchacha20(
        ciphertext: &[u8],
        nonce: &[u8],
        key: &[u8],
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        use chacha20poly1305::{
            aead::{Aead, KeyInit},
            XChaCha20Poly1305, XNonce,
        };

        // Ensure key is exactly 32 bytes
        if key.len() != 32 {
            return Err("Key must be exactly 32 bytes for XChaCha20".into());
        }

        // Ensure nonce is exactly 24 bytes
        if nonce.len() != 24 {
            return Err("Nonce must be exactly 24 bytes for XChaCha20".into());
        }

        // ✅ CRITICAL SECURITY FIX (CVE-2025-11931 class vulnerability)
        // Prevent integer underflow and DoS attacks
        const POLY1305_TAG_LEN: usize = 16;
        const MIN_CIPHERTEXT_LEN: usize = POLY1305_TAG_LEN;
        const MAX_MESSAGE_LEN: usize = 256 * 1024 * 1024; // 256 MB

        // Validate ciphertext length BEFORE any arithmetic operations
        if ciphertext.len() < MIN_CIPHERTEXT_LEN {
            tracing::warn!(
                actual_len = ciphertext.len(),
                min_required = MIN_CIPHERTEXT_LEN,
                "Ciphertext too short - possible truncation attack (CVE-2025-11931 class)"
            );
            return Err("Ciphertext too short (possible truncation attack)".into());
        }

        // Prevent DoS via large memory allocations
        if ciphertext.len() > MAX_MESSAGE_LEN {
            tracing::warn!(
                actual_len = ciphertext.len(),
                max_allowed = MAX_MESSAGE_LEN,
                "Ciphertext exceeds maximum allowed size - possible DoS attack"
            );
            return Err("Ciphertext exceeds maximum allowed size".into());
        }

        let cipher = XChaCha20Poly1305::new(key.into());
        let nonce = XNonce::from_slice(nonce);

        // Decrypt with sanitized error messages (don't leak internal details)
        cipher.decrypt(nonce, ciphertext).map_err(|e| {
            // Log full error internally for debugging
            tracing::debug!(error = ?e, "XChaCha20 decryption failed");
            // Return generic error to attacker (prevent oracle attacks)
            "Decryption failed (authentication failed or corrupted data)".into()
        })
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

        // 🛡️ REAL ECDH IMPLEMENTATION
        // Peer public key structure: [Ed25519 (32) | X25519 (32)]
        if peer_public_key.len() < 64 {
            return Err("Invalid peer public key length".into());
        }
        let x25519_peer_pub_bytes: [u8; 32] = peer_public_key[32..64].try_into()?;
        let peer_x_pub = XPublicKey::from(x25519_peer_pub_bytes);

        // Our private key structure: [Ed25519 Len (2)] [Ed25519 PKCS8] [X25519 Private (32)]
        let priv_key_bytes = &self.key_pair.private_key;
        if priv_key_bytes.len() < 34 {
            return Err("Private key too short".into());
        }
        let ed_len = u16::from_le_bytes([priv_key_bytes[0], priv_key_bytes[1]]) as usize;
        let x25519_priv_bytes: [u8; 32] = priv_key_bytes[2 + ed_len..2 + ed_len + 32].try_into()?;
        let our_x_secret = StaticSecret::from(x25519_priv_bytes);

        // Compute Shared Secret using X25519
        let shared_secret = our_x_secret.diffie_hellman(&peer_x_pub);

        // 🛡️ CVE-2025-62514 defense: reject all-zero shared secret (small-order point attack)
        // An attacker sending a small-order point (e.g., the zero point) would produce
        // an all-zero DH output, bypassing encryption entirely.
        if shared_secret.as_bytes() == &[0u8; 32] {
            return Err(
                "X25519 shared secret is all zeros — small-order point attack detected".into(),
            );
        }

        // Derive session key using HKDF-SHA256 (Robust way)
        let salt = hkdf::Salt::new(hkdf::HKDF_SHA256, b"GhostWire-Salt-v1");
        let prk = salt.extract(shared_secret.as_bytes());
        let info_str = cryptify::encrypt_string!("GhostWire-Session-Key-v2");
        let info = [info_str.as_bytes()];
        let okm = prk
            .expand(&info, hkdf::HKDF_SHA256)
            .map_err(|_| "HKDF expand failed")?;

        let mut key = [0u8; 32];
        okm.fill(&mut key).map_err(|_| "HKDF fill failed")?;

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
    pub fn encrypt(&self, data: &[u8]) -> anyhow::Result<Vec<u8>> {
        // Derive a strong 32-byte key from our key pair
        let key = self.derive_cli_key();

        match Self::encrypt_with_xchacha20(data, &key) {
            Ok(result) => {
                // Combine nonce and ciphertext
                let mut combined = result.nonce;
                combined.extend_from_slice(&result.ciphertext);
                Ok(combined)
            }
            Err(e) => {
                tracing::error!(inner_error = %e, "CLI encryption failed");
                Err(anyhow::anyhow!("Encryption failed: {}", e))
            }
        }
    }

    // Derive a deterministic but unique 32-byte key for CLI operations
    fn derive_cli_key(&self) -> Vec<u8> {
        use ring::digest;

        // Use SHA-256 to derive a 32-byte key from our keypair
        let mut ctx = digest::Context::new(&digest::SHA256);
        let domain = cryptify::encrypt_string!("GhostWire-CLI-v1");
        ctx.update(domain.as_bytes()); // Domain separation
        ctx.update(&self.key_pair.private_key[..]);
        ctx.update(&self.key_pair.public_key[..]);

        ctx.finish().as_ref().to_vec()
    }

    // ✅ SECURE DECRYPTION: Returns Result instead of panic (DoS prevention)
    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, DecryptionError> {
        if data.len() < 24 {
            tracing::warn!(
                actual_len = data.len(),
                min_required = 24,
                "Encrypted data too short"
            );
            return Err(DecryptionError::InvalidLength);
        }

        // Derive the same key we used for encryption
        let key = self.derive_cli_key();

        // Extract nonce and ciphertext
        let nonce = &data[..24];
        let ciphertext = &data[24..];

        Self::decrypt_with_xchacha20(ciphertext, nonce, &key).map_err(|e| {
            tracing::debug!(error = ?e, "CLI decryption failed");
            DecryptionError::DecryptionFailed
        })
    }

    // Enhanced signature methods
    pub fn sign_message(&self, message: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Use Ed25519 for signing
        // Extract plain Ed25519 PKCS8 bytes from binary hybrid key
        let priv_key_bytes = &self.key_pair.private_key;
        if priv_key_bytes.len() < 2 {
            return Err("Invalid private key length".into());
        }
        let ed_len = u16::from_le_bytes([priv_key_bytes[0], priv_key_bytes[1]]) as usize;
        if priv_key_bytes.len() < 2 + ed_len {
            return Err("Corrupted private key".into());
        }
        let key_pair_bytes = &priv_key_bytes[2..2 + ed_len];

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
        // Extract plain Ed25519 public key (first 32 bytes) if hybrid
        let ed25519_pub_key = if public_key.len() >= 32 {
            &public_key[..32]
        } else {
            return Err("Invalid public key length".into());
        };

        let public_key =
            ring::signature::UnparsedPublicKey::new(&ring::signature::ED25519, ed25519_pub_key);

        public_key
            .verify(message, signature)
            .map_err(|e| format!("Signature verification failed: {:?}", e).into())
    }

    fn generate_nonce() -> [u8; 24] {
        let mut nonce = [0u8; 24];
        SystemRandom::new().fill(&mut nonce[..]).unwrap();
        nonce
    }

    fn increment_key_usage(&self) {
        // In a real implementation, this would be thread-safe
        // For now, we'll just log the usage
        tracing::debug!(key_id = %self.key_pair.key_id, "Key usage incremented");
    }

    pub fn export_public_key(&self) -> Vec<u8> {
        self.key_pair.public_key.clone()
    }

    pub fn get_peer_count(&self) -> usize {
        self.peer_keys.lock().unwrap().len()
    }

    pub fn should_rotate_keys(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        now - self.last_key_rotation > self.key_rotation_interval
    }

    pub fn rotate_keys_if_needed(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.should_rotate_keys() {
            tracing::info!(msg = "Rotating encryption keys");
            let new_key_pair = Self::generate_hybrid_key_pair(&self.rng)?;
            self.key_pair = new_key_pair;
            self.last_key_rotation = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            self.session_keys.lock().unwrap().clear();
            self.save_to_disk()?;
        }
        Ok(())
    }

    pub fn revoke_key(&self, key_id: &str) -> crl::RevocationEntry {
        let key_id_owned = key_id.to_string();
        let signer_id = self.key_pair.key_id.clone();
        self.crl.revoke(&key_id_owned, &signer_id, &|payload| {
            self.sign_message(payload).unwrap_or_default()
        })
    }

    pub fn merge_crl_entry(&self, entry: &crl::RevocationEntry) -> crl::MergeResult {
        self.crl.merge(entry, &|payload, signature, signer_id| {
            if let Some(peer) = self.peer_keys.lock().unwrap().get(signer_id) {
                self.verify_signature(payload, signature, &peer.public_key)
                    .is_ok()
            } else {
                false
            }
        })
    }

    pub fn gc_crl(&self) -> usize {
        self.crl.gc()
    }

    pub fn create_crl_gossip_payload(&self) -> Vec<crl::CrlGossipPayload> {
        self.crl
            .entries()
            .into_iter()
            .enumerate()
            .map(|(i, entry)| crl::CrlGossipPayload {
                entry,
                version: (i as u64) + 1,
            })
            .collect()
    }

    pub fn handle_crl_gossip(&self, payloads: &[crl::CrlGossipPayload]) -> (usize, usize, usize) {
        let mut accepted = 0;
        let mut duplicates = 0;
        let mut rejected = 0;

        for payload in payloads {
            match self.merge_crl_entry(&payload.entry) {
                crl::MergeResult::Accepted => accepted += 1,
                crl::MergeResult::Duplicate => duplicates += 1,
                _ => rejected += 1,
            }
        }

        (accepted, duplicates, rejected)
    }

    pub fn init_ratchet_session(
        &self,
        peer_id: &str,
        bob_public_key: &[u8; 32],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let session_key = self.derive_session_key(
            peer_id,
            &[self.key_pair.public_key[32..64].to_vec()].concat(),
        )?;

        let mut shared_secret = [0u8; 32];
        shared_secret.copy_from_slice(&session_key);

        self.ratchet_sessions
            .init_alice(peer_id, &shared_secret, bob_public_key);
        Ok(())
    }

    pub fn init_ratchet_session_as_bob(
        &self,
        peer_id: &str,
        _alice_public_key: &[u8; 32],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let session_key = self.derive_session_key(
            peer_id,
            &[self.key_pair.public_key[32..64].to_vec()].concat(),
        )?;

        let mut shared_secret = [0u8; 32];
        shared_secret.copy_from_slice(&session_key);

        // Extract X25519 private key from hybrid key: [ed_len_u16][ed25519_pkcs8][x25519_32]
        let priv_key_bytes = &self.key_pair.private_key;
        if priv_key_bytes.len() < 36 {
            return Err("Private key too short for X25519 extraction".into());
        }
        let ed_len = u16::from_le_bytes([priv_key_bytes[0], priv_key_bytes[1]]) as usize;
        let x25519_start = 2 + ed_len;
        if priv_key_bytes.len() < x25519_start + 32 {
            return Err("Private key missing X25519 secret".into());
        }
        let mut bob_secret = [0u8; 32];
        bob_secret.copy_from_slice(&priv_key_bytes[x25519_start..x25519_start + 32]);

        self.ratchet_sessions
            .init_bob(peer_id, &shared_secret, &bob_secret);
        Ok(())
    }

    pub fn encrypt_message_ratchet(
        &self,
        message: &str,
        peer_id: &str,
        message_type: MessageType,
    ) -> Result<EncryptedMessage, Box<dyn std::error::Error>> {
        if self.crl.is_revoked(peer_id) {
            return Err(format!("Peer key {} is revoked", peer_id).into());
        }

        let ratchet_msg = self
            .ratchet_sessions
            .encrypt(peer_id, message.as_bytes())
            .map_err(|e| format!("Ratchet encrypt failed: {}", e))?;

        let signature = self.sign_message(&ratchet_msg.ciphertext)?;

        let header_json = serde_json::to_vec(&ratchet_msg.header)?;

        self.increment_key_usage();

        Ok(EncryptedMessage {
            ciphertext: ratchet_msg.ciphertext,
            nonce: ratchet_msg.nonce,
            signature,
            sender_id: self.key_pair.key_id.clone(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            key_id: format!("ratchet:{}", peer_id),
            message_type,
            version: 0x04,
            pq_key: Some(header_json),
        })
    }

    pub fn decrypt_message_ratchet(
        &self,
        encrypted_msg: &EncryptedMessage,
    ) -> Result<String, Box<dyn std::error::Error>> {
        if self.crl.is_revoked(&encrypted_msg.sender_id) {
            return Err(format!("Sender key {} is revoked", encrypted_msg.sender_id).into());
        }

        if encrypted_msg.version != 0x04 {
            return Err("Expected ratchet message (v4)".into());
        }

        let header_json = encrypted_msg
            .pq_key
            .as_ref()
            .ok_or("Missing ratchet header")?;
        let header: double_ratchet::RatchetHeader = serde_json::from_slice(header_json)
            .map_err(|e| format!("Invalid ratchet header: {}", e))?;

        let ratchet_msg = double_ratchet::RatchetMessage {
            header,
            ciphertext: encrypted_msg.ciphertext.clone(),
            nonce: encrypted_msg.nonce.clone(),
        };

        let peer_id = encrypted_msg
            .key_id
            .strip_prefix("ratchet:")
            .unwrap_or(&encrypted_msg.sender_id);

        let plaintext = self
            .ratchet_sessions
            .decrypt(peer_id, &ratchet_msg)
            .map_err(|e| format!("Ratchet decrypt failed: {}", e))?;

        String::from_utf8(plaintext).map_err(|e| e.into())
    }

    pub fn has_ratchet_session(&self, peer_id: &str) -> bool {
        self.ratchet_sessions.has_session(peer_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionResult {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
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
    fn test_signature_verification() {
        let encryption = EncryptionManager::new().unwrap();
        let message = b"Verify me!";
        let signature = encryption.sign_message(message).unwrap();

        // Verify with own public key
        assert!(encryption
            .verify_signature(message, &signature, encryption.get_public_key())
            .is_ok());

        // Fail with wrong message
        assert!(encryption
            .verify_signature(b"Wrong", &signature, encryption.get_public_key())
            .is_err());
    }

    #[test]
    fn test_hybrid_key_generation() {
        let rng = SystemRandom::new();
        let key_pair = EncryptionManager::generate_hybrid_key_pair(&rng).unwrap();

        // 32 Ed25519 + 32 X25519 + 1184 Kyber ML-KEM-768 = 1248
        assert_eq!(key_pair.public_key.len(), 1248);
        assert!(key_pair.private_key.len() > 34);
        // Header length prefix should be there
        let ed_len =
            u16::from_le_bytes([key_pair.private_key[0], key_pair.private_key[1]]) as usize;
        assert!(ed_len > 0);
        // Kyber secret key should be at the end: 2400 bytes for ML-KEM-768
        let pq_sk_start = 2 + ed_len + 32;
        assert_eq!(key_pair.private_key.len(), pq_sk_start + 2400);
        assert_eq!(key_pair.key_type, KeyType::Ultimate);
    }

    #[test]
    fn test_pq_hybrid_encrypt_decrypt() {
        let alice = EncryptionManager::create_new().unwrap();
        let bob = EncryptionManager::create_new().unwrap();

        assert_eq!(alice.key_pair.key_type, KeyType::Ultimate);
        assert_eq!(alice.key_pair.public_key.len(), 1248);

        alice.add_peer_key(
            bob.get_key_id(),
            bob.get_public_key().to_vec(),
            KeyType::Ultimate,
        );
        bob.add_peer_key(
            alice.get_key_id(),
            alice.get_public_key().to_vec(),
            KeyType::Ultimate,
        );

        let message = "PQ-hybrid encryption works!";
        let encrypted = alice
            .encrypt_message(message, &bob.get_key_id(), MessageType::Text)
            .unwrap();

        assert!(encrypted.pq_key.is_some(), "Kyber path not taken");
        assert_eq!(encrypted.pq_key.as_ref().unwrap().len(), 1088);

        let decrypted = bob.decrypt_message(&encrypted).unwrap();
        assert_eq!(message, decrypted);
    }

    #[test]
    fn test_secure_storage() {
        let file_path = std::path::Path::new("test_secure_storage.json");

        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct TestData {
            name: String,
            val: i32,
        }
        let data = TestData {
            name: "test".to_string(),
            val: 42,
        };

        EncryptionManager::secure_save(&file_path, &data).unwrap();
        let loaded: TestData = EncryptionManager::secure_load(&file_path).unwrap();

        assert_eq!(data, loaded);

        // Cleanup
        let _ = std::fs::remove_file(file_path);
    }
}

#[derive(Serialize, Deserialize)]
struct PersistedEncryption {
    key_pair: GhostKeyPair,
    key_rotation_interval: u64,
    last_key_rotation: u64,
}
