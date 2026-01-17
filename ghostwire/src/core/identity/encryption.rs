use anyhow::{anyhow, Result};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use pbkdf2::pbkdf2_hmac;
use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

const PBKDF2_ITERATIONS: u32 = 100_000;
const SALT_LEN: usize = 32;

/// Encrypted identity storage format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedIdentity {
    pub version: u32,
    pub encrypted_data: Vec<u8>,
    pub salt: Vec<u8>,
    pub nonce: Vec<u8>,
}

/// Derive key from password and salt using PBKDF2
pub fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, PBKDF2_ITERATIONS, &mut key[..]);
    key
}

/// Encrypt identity data with a password
pub fn encrypt_identity(data: &[u8], password: &str) -> Result<EncryptedIdentity> {
    // Generate random salt
    let mut salt = vec![0u8; SALT_LEN];
    getrandom::getrandom(&mut salt)?;

    // Derive key from password
    let key = derive_key(password, &salt);

    // Create cipher
    let cipher = ChaCha20Poly1305::new(&key.into());

    // Generate random nonce
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes[..]);
    let nonce = Nonce::from(nonce_bytes);

    // Encrypt
    let encrypted_data = cipher
        .encrypt(&nonce, data)
        .map_err(|e| anyhow!("Encryption failed: {}", e))?;

    Ok(EncryptedIdentity {
        version: 2,
        encrypted_data,
        salt,
        nonce: nonce.to_vec(),
    })
}

/// Decrypt identity data with a password
pub fn decrypt_identity(encrypted: &EncryptedIdentity, password: &str) -> Result<Vec<u8>> {
    // Derive key using same parameters
    let key = derive_key(password, &encrypted.salt);

    // Create cipher
    let cipher = ChaCha20Poly1305::new(&key.into());

    // Decrypt
    let nonce = Nonce::from_slice(&encrypted.nonce);
    let decrypted_data = cipher
        .decrypt(nonce, encrypted.encrypted_data.as_ref())
        .map_err(|_| anyhow!("Decryption failed: incorrect password or corrupted data"))?;

    Ok(decrypted_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let data = b"test identity data";
        let password = "correct_password";

        let encrypted = encrypt_identity(&data[..], password).unwrap();
        let decrypted = decrypt_identity(&encrypted, password).unwrap();

        assert_eq!(&data[..], decrypted.as_slice());
    }

    #[test]
    fn test_wrong_password() {
        let data = b"test identity data";
        let password = "correct_password";
        let wrong_password = "wrong_password";

        let encrypted = encrypt_identity(&data[..], password).unwrap();
        let result = decrypt_identity(&encrypted, wrong_password);

        assert!(result.is_err());
    }

    #[test]
    fn test_different_salts() {
        let data = b"test identity data";
        let password = "same_password";

        let encrypted1 = encrypt_identity(&data[..], password).unwrap();
        let encrypted2 = encrypt_identity(&data[..], password).unwrap();

        // Same password but different salts/nonces should produce different ciphertext
        assert_ne!(encrypted1.encrypted_data, encrypted2.encrypted_data);
        assert_ne!(encrypted1.salt, encrypted2.salt);
        assert_ne!(encrypted1.nonce, encrypted2.nonce);

        // But both should decrypt correctly
        let decrypted1 = decrypt_identity(&encrypted1, password).unwrap();
        let decrypted2 = decrypt_identity(&encrypted2, password).unwrap();

        assert_eq!(data, decrypted1.as_slice());
        assert_eq!(data, decrypted2.as_slice());
    }
}
