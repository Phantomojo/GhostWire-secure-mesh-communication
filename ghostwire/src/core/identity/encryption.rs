use anyhow::{anyhow, Result};
use argon2::{password_hash::rand_core::OsRng, Argon2};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    XChaCha20Poly1305, XNonce,
};
use rand_core::RngCore;
use serde::{Deserialize, Serialize};

const SALT_LEN: usize = 32;

/// Encrypted identity storage format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedIdentity {
    pub version: u32,
    pub encrypted_data: Vec<u8>,
    pub salt: Vec<u8>,
    pub nonce: Vec<u8>,
}

/// Derive key from password and salt using Argon2id
pub fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; 32]> {
    // ponytail: upgraded from Argon2::default() (19 MiB, 2 iter, 1 lane)
    // to match EncryptedStore's strong params (64 MiB, 3 iter, 4 lanes)
    let params = argon2::Params::new(65536, 3, 4, Some(32))
        .map_err(|e| anyhow!("Argon2 params failed: {}", e))?;
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
    let mut key = [0u8; 32];

    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| anyhow!("Argon2 key derivation failed: {}", e))?;

    Ok(key)
}

/// Encrypt identity data with a password
pub fn encrypt_identity(data: &[u8], password: &str) -> Result<EncryptedIdentity> {
    // Generate random salt
    let mut salt = [0u8; SALT_LEN];
    OsRng.fill_bytes(&mut salt);

    // Derive key from password using Argon2id
    let key = derive_key(password, &salt)?;

    // Create cipher
    let cipher = XChaCha20Poly1305::new(&key.into());

    // Generate random nonce
    let mut nonce_bytes = [0u8; 24];
    OsRng.fill_bytes(&mut nonce_bytes[..]);
    let nonce = XNonce::from_slice(&nonce_bytes);

    // Encrypt
    let encrypted_data = cipher
        .encrypt(nonce, data)
        .map_err(|e| anyhow!("Encryption failed: {}", e))?;

    Ok(EncryptedIdentity {
        version: 4, // Version 4 uses Argon2id + XChaCha20
        encrypted_data,
        salt: salt.to_vec(),
        nonce: nonce.to_vec(),
    })
}

/// Decrypt identity data with a password
pub fn decrypt_identity(encrypted: &EncryptedIdentity, password: &str) -> Result<Vec<u8>> {
    // Check version
    if encrypted.version < 4 {
        // Fallback for legacy PBKDF2 (if we wanted to keep backward compat, but for now we upgrade)
        // For this task, we assume a hard upgrade or handle legacy later.
        // Let's implement full replacement first.
        return Err(anyhow!(
            "Legacy identity versions (v1/v2/v3) not supported in this build yet."
        ));
    }

    // Derive key using same parameters
    let key = derive_key(password, &encrypted.salt)?;

    // Create cipher
    let cipher = XChaCha20Poly1305::new(&key.into());

    // Decrypt
    let nonce = XNonce::from_slice(&encrypted.nonce);
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
