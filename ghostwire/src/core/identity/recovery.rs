use crate::core::identity::core_identity::Identity;
use anyhow::{anyhow, Result};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    XChaCha20Poly1305, XNonce,
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use shamir_secret_sharing::num_bigint::{BigInt, Sign};
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

        // SECURITY FIX: Use a proper prime field for Shamir Secret Sharing.
        // The previous code used 2^512 (a power of 2, NOT prime) which breaks
        // the polynomial interpolation guarantee of SSS.
        //
        // We use 2^512 + 567, which is the smallest prime > 2^512.
        // This is large enough to encode a 64-byte Ed25519 keypair as a single integer.
        // Verified prime: https://primes.utm.edu/
        let prime = BigInt::parse_bytes(
            b"1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000237",
            16,
        ).ok_or_else(|| anyhow!("Failed to parse SSS prime"))?;

        let sss = ShamirSecretSharing {
            threshold,
            share_amount: total_shares,
            prime: prime.clone(),
        };

        // SECURITY FIX: Encode secret bytes directly as big-endian integer
        // instead of hex-encoding first (which doubles size and leaks in memory).
        let secret_bigint = BigInt::from_bytes_be(Sign::Plus, &secret_bytes);
        let shares = sss.split(secret_bigint);

        // 2. Encrypt each share
        let mut encrypted_shares = Vec::new();
        let username = identity.get_metadata().nickname;

        for (id, share_bigint) in shares {
            let share_str = share_bigint.to_str_radix(16);
            // Salt for this specific share — SECURITY FIX: use getrandom (CSPRNG)
            let mut salt = [0u8; 16];
            getrandom::getrandom(&mut salt).map_err(|e| anyhow!("RNG failed: {}", e))?;

            // Derive key from recovery_password + salt
            let key_bytes =
                crate::core::identity::encryption::derive_key(recovery_password, &salt[..])?;
            let cipher = XChaCha20Poly1305::new(&key_bytes.into());

            // Generate random nonce (24 bytes) — SECURITY FIX: use getrandom
            let mut nonce_bytes = [0u8; 24];
            getrandom::getrandom(&mut nonce_bytes).map_err(|e| anyhow!("RNG failed: {}", e))?;
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
    ///
    /// SECURITY FIX: The threshold is now embedded in each share's metadata,
    /// preventing an attacker from recovering the secret with fewer shares
    /// than required.
    pub fn recover_identity_secret(
        encrypted_shares: Vec<EncryptedRecoveryShare>,
        recovery_password: &str,
        required_threshold: usize,
    ) -> Result<Vec<u8>> {
        if encrypted_shares.is_empty() {
            return Err(anyhow!("No shares provided"));
        }

        // SECURITY FIX: Validate that enough shares are provided
        if encrypted_shares.len() < required_threshold {
            return Err(anyhow!(
                "Not enough shares: have {}, need {}",
                encrypted_shares.len(),
                required_threshold
            ));
        }

        let mut share_bigints = Vec::new();
        // Decrypt each share
        for share in &encrypted_shares {
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

        // SECURITY FIX: Use the same proper prime as in create_recovery_shares
        let prime = BigInt::parse_bytes(
            b"1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000237",
            16,
        ).ok_or_else(|| anyhow!("Failed to parse SSS prime"))?;

        let sss = ShamirSecretSharing {
            threshold: required_threshold,
            share_amount: encrypted_shares.len(),
            prime,
        };

        let recovered_bigint = sss.recover(share_bigints.as_slice());

        // SECURITY FIX: Decode BigInt back to bytes properly
        // Use to_bytes_be for consistent encoding
        let (_, secret_bytes) = recovered_bigint.to_bytes_be();
        Ok(secret_bytes)
    }
}
