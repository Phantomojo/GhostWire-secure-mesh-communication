use crate::core::identity::core_identity::Identity;
use anyhow::{anyhow, Result};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    XChaCha20Poly1305, XNonce,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use shamir_secret_sharing::num_bigint::BigInt;
use shamir_secret_sharing::ShamirSecretSharing;

// Re-using/Adapting from encryption.rs for share encryption
// Each share will be encrypted with the recovery password before storing in DHT.

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EncryptedRecoveryShare {
    pub index: usize,
    pub index_hash: String, // Hash of (username + "recovery" + index) for DHT key
    pub encrypted_data: Vec<u8>,
    pub nonce: Vec<u8>,
    pub salt: Vec<u8>,
}

pub struct RecoveryManager {
    // In a real implementation, this would hold a reference to the DHT/MeshManager
    // For now, we will just implement the logic to create/recover the shares.
}

impl RecoveryManager {
    /// Enables recovery for an identity by splitting its private key seed into shares.
    /// Returns a list of EncryptedRecoveryShare ready for DHT storage.
    ///
    /// - `identity`: The identity to back up.
    /// - `recovery_password`: The password used to encrypt the shares.
    /// - `threshold`: Minimum shares needed to recover (e.g., 3).
    /// - `total_shares`: Total shares to generate (e.g., 5).
    pub fn create_recovery_shares(
        identity: &Identity,
        recovery_password: &str,
        threshold: usize,
        total_shares: usize,
    ) -> Result<Vec<EncryptedRecoveryShare>> {
        let keypair_opt = identity.keypair();
        let keypair =
            keypair_opt.ok_or_else(|| anyhow!("Identity is locked. Cannot enable recovery."))?;

        // Back up the whole keypair encoding for full restoration
        let secret_bytes = keypair
            .to_protobuf_encoding()
            .map_err(|e| anyhow!("Encoding failed: {}", e))?;

        // 1. Split the secret using Shamir Secret Sharing
        let prime_bytes = &b"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"[..];
        let prime = BigInt::parse_bytes(prime_bytes, 16).unwrap() + BigInt::from(1);
        let sss = ShamirSecretSharing {
            threshold,
            share_amount: total_shares,
            prime: prime.clone(),
        };

        let secret_hex = hex::encode(&secret_bytes);
        let secret_bigint = BigInt::parse_bytes(secret_hex.as_bytes(), 16).unwrap();
        let shares = sss.split(secret_bigint);

        // 2. Encrypt each share
        let mut encrypted_shares = Vec::new();
        let username = identity.get_metadata().nickname;

        for (id, share_bigint) in shares {
            let share_str = share_bigint.to_str_radix(16);
            // Salt for this specific share
            let mut salt = [0u8; 16];
            rand::thread_rng().fill(&mut salt);

            // Derive key from recovery_password + salt
            let key_bytes =
                crate::core::identity::encryption::derive_key(recovery_password, &salt[..])?;
            let cipher = XChaCha20Poly1305::new(&key_bytes.into());

            // Generate random nonce (24 bytes)
            let mut nonce_bytes = [0u8; 24];
            rand::thread_rng().fill(&mut nonce_bytes);
            let nonce = XNonce::from_slice(&nonce_bytes);

            // Encrypt the share string
            let payload = share_str.as_bytes();
            let encrypted_data = cipher
                .encrypt(nonce, payload)
                .map_err(|e| anyhow!("Encryption failed: {}", e))?;

            // Generate DHT key: Hash(username + "recovery" + index)
            // This is deterministic so we can find it later.
            let dht_key_preimage = format!("{}:recovery:{}", username, id);
            let dht_key = hex::encode(Sha256::digest(dht_key_preimage.as_bytes()));

            encrypted_shares.push(EncryptedRecoveryShare {
                index: id,
                index_hash: dht_key,
                encrypted_data,
                nonce: nonce.to_vec(),
                salt: salt.to_vec(),
            });
        }

        Ok(encrypted_shares)
    }

    /// Recovers the identity secret from a list of encrypted shares.
    pub fn recover_identity_secret(
        encrypted_shares: Vec<EncryptedRecoveryShare>,
        recovery_password: &str,
    ) -> Result<Vec<u8>> {
        if encrypted_shares.is_empty() {
            return Err(anyhow!("No shares provided"));
        }

        let mut share_bigints = Vec::new();
        // We assume all shares have the same threshold/config implicitly compatible
        // Decrypt each share
        for share in encrypted_shares {
            let key_bytes =
                crate::core::identity::encryption::derive_key(recovery_password, &share.salt)?;
            let cipher = XChaCha20Poly1305::new(&key_bytes.into());
            let nonce = XNonce::from_slice(&share.nonce);

            let decrypted_bytes = cipher
                .decrypt(nonce, share.encrypted_data.as_ref())
                .map_err(|_| anyhow!("Failed to decrypt share {}. Wrong password?", share.index))?;

            let share_str = String::from_utf8(decrypted_bytes)?;
            let bigint = BigInt::parse_bytes(share_str.as_bytes(), 16)
                .ok_or_else(|| anyhow!("Failed to parse share hex"))?;
            share_bigints.push((share.index, bigint));
        }

        // Combine shares
        let prime_bytes = &b"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"[..];
        let prime = BigInt::parse_bytes(prime_bytes, 16).unwrap() + BigInt::from(1);
        let sss = ShamirSecretSharing {
            threshold: share_bigints.len(), // Use what we have
            share_amount: share_bigints.len(),
            prime,
        };

        let recovered_bigint = sss.recover(share_bigints.as_slice());

        // Decode BigInt back to bytes
        let (_, secret_bytes) = recovered_bigint.to_bytes_be();
        Ok(secret_bytes)
    }
}
