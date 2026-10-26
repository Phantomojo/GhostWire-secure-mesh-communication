//! Sealed sender envelope for GhostWire.
//!
//! Wraps an inner (already encrypted) message payload so that the sender
//! identity is only visible to the recipient. Relays see only the recipient
//! hint and an opaque ciphertext blob.

use ring::rand::SecureRandom;
use serde::{Deserialize, Serialize};
use x25519_dalek::{PublicKey as XPublicKey, StaticSecret};

/// Version byte for sealed sender v1.
pub const SEALED_SENDER_VERSION: u8 = 0x01;

/// Sealed envelope wrapping an inner message payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SealedEnvelope {
    pub version: u8,
    pub recipient_hint: [u8; 32],
    pub ephemeral_pubkey: [u8; 32],
    pub pq_key: Option<Vec<u8>>,
    pub nonce: [u8; 24],
    pub ciphertext: Vec<u8>,
    pub mac: [u8; 32],
    pub delivery_token: Option<[u8; 12]>,
}

#[derive(Debug, thiserror::Error)]
pub enum SealedError {
    #[error("invalid key length")]
    InvalidKeyLength,
    #[error("encryption failed")]
    EncryptionFailed,
    #[error("decryption failed")]
    DecryptionFailed,
    #[error("invalid envelope")]
    InvalidEnvelope,
}

impl SealedEnvelope {
    /// Seal a plaintext payload to a recipient identified by their X25519
    /// public key. The recipient hint is BLAKE3(recipient pubkey).
    pub fn seal(
        plaintext: &[u8],
        recipient_x25519_pub: &[u8; 32],
    ) -> Result<Self, SealedError> {
        use chacha20poly1305::{
            aead::{Aead, KeyInit},
            XChaCha20Poly1305, XNonce,
        };
        use rand_core::OsRng;
        use x25519_dalek::PublicKey as XPk;

        // Ephemeral sender keypair.
        let ephemeral_secret = StaticSecret::random_from_rng(OsRng);
        let ephemeral_public = XPk::from(&ephemeral_secret);

        // X25519 shared secret.
        let recipient_pub = XPk::from(*recipient_x25519_pub);
        let shared = ephemeral_secret.diffie_hellman(&recipient_pub);

        // HKDF-SHA256 key derivation.
        let salt = ring::hkdf::Salt::new(ring::hkdf::HKDF_SHA256, b"ghostwire/sealed-sender/v1");
        let prk = salt.extract(shared.as_bytes());
        let okm = prk
            .expand(&[b"ghostwire/sealed-sender/v1"], ring::hkdf::HKDF_SHA256)
            .map_err(|_| SealedError::EncryptionFailed)?;
        let mut key = [0u8; 32];
        okm.fill(&mut key).map_err(|_| SealedError::EncryptionFailed)?;

        // Nonce.
        let mut nonce_bytes = [0u8; 24];
        ring::rand::SystemRandom::new()
            .fill(&mut nonce_bytes[..])
            .map_err(|_| SealedError::EncryptionFailed)?;

        // Encrypt.
        let cipher = XChaCha20Poly1305::new(&key.into());
        let nonce = XNonce::from_slice(&nonce_bytes);
        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|_| SealedError::EncryptionFailed)?;

        // Recipient hint = BLAKE3(recipient pubkey).
        let recipient_hint = *blake3::hash(recipient_x25519_pub).as_bytes();

        // MAC = BLAKE3-keyed over ciphertext.
        let mac = *blake3::keyed_hash(&key, &ciphertext).as_bytes();

        Ok(Self {
            version: SEALED_SENDER_VERSION,
            recipient_hint,
            ephemeral_pubkey: *ephemeral_public.as_bytes(),
            pq_key: None,
            nonce: nonce_bytes,
            ciphertext,
            mac,
            delivery_token: None,
        })
    }

    /// Unseal a payload using the recipient's X25519 static secret.
    pub fn unseal(
        &self,
        recipient_secret: &StaticSecret,
    ) -> Result<Vec<u8>, SealedError> {
        use chacha20poly1305::{
            aead::{Aead, KeyInit},
            XChaCha20Poly1305, XNonce,
        };

        if self.version != SEALED_SENDER_VERSION {
            return Err(SealedError::InvalidEnvelope);
        }

        let ephemeral_pub = XPublicKey::from(self.ephemeral_pubkey);
        let shared = recipient_secret.diffie_hellman(&ephemeral_pub);

        let salt = ring::hkdf::Salt::new(ring::hkdf::HKDF_SHA256, b"ghostwire/sealed-sender/v1");
        let prk = salt.extract(shared.as_bytes());
        let okm = prk
            .expand(&[b"ghostwire/sealed-sender/v1"], ring::hkdf::HKDF_SHA256)
            .map_err(|_| SealedError::DecryptionFailed)?;
        let mut key = [0u8; 32];
        okm.fill(&mut key).map_err(|_| SealedError::DecryptionFailed)?;

        // Verify MAC.
        let expected_mac = *blake3::keyed_hash(&key, &self.ciphertext).as_bytes();
        if expected_mac != self.mac {
            return Err(SealedError::DecryptionFailed);
        }

        let cipher = XChaCha20Poly1305::new(&key.into());
        let nonce = XNonce::from_slice(&self.nonce);
        cipher
            .decrypt(nonce, self.ciphertext.as_slice())
            .map_err(|_| SealedError::DecryptionFailed)
    }
}

/// Helper: seal a UTF-8 message string.
pub fn seal_message(
    message: &str,
    recipient_x25519_pub: &[u8; 32],
) -> Result<SealedEnvelope, SealedError> {
    SealedEnvelope::seal(message.as_bytes(), recipient_x25519_pub)
}

/// Helper: unseal into a UTF-8 string.
pub fn unseal_message(
    env: &SealedEnvelope,
    recipient_secret: &StaticSecret,
) -> Result<String, SealedError> {
    let bytes = env.unseal(recipient_secret)?;
    String::from_utf8(bytes).map_err(|_| SealedError::DecryptionFailed)
}
