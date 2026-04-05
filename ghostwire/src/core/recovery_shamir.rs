//! recovery.rs — Shamir Secret Sharing for identity key recovery
//!
//! Problem: If a user loses their device or forgets their password, they lose
//! their identity forever. Most privacy apps just say "you're on your own."
//!
//! Solution: Split the identity master key into N shares using Shamir's Secret
//! Sharing. Distribute to trusted contacts. Recovery requires M-of-N to
//! cooperate. No single contact can reconstruct the key alone.
//!
//! This is a genuinely novel UX feature in the privacy app space.
//! The `shamir_secret_sharing` crate is already in Ghostwire's Cargo.toml.
//!
//! Flow:
//!   1. User chooses M (threshold) and N (total shares), picks N trusted peers
//!   2. Each share is encrypted to that peer's public key before transmission
//!   3. Shares are stored in each peer's local encrypted DB
//!   4. To recover: contact M peers, collect their shares, reconstruct key
//!   5. Re-encrypt reconstructed key with new password, save as new identity

use crate::genesis_constants::{RECOVERY_KDF_DOMAIN, derive_with_domain};
use shamir_secret_sharing::num_bigint::BigInt;
use num_traits::ToPrimitive;
use base64::Engine;

use anyhow::{Context, Result};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Key, Nonce,
};
use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// A single share, encrypted for a specific guardian peer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedShare {
    /// Index of this share (1-based, as Shamir requires)
    pub share_index: u8,
    /// Guardian's peer ID (for routing the share to them)
    pub guardian_id: String,
    /// Guardian's display name (for UX)
    pub guardian_name: String,
    /// The share bytes, encrypted to guardian's X25519 public key
    pub ciphertext: Vec<u8>,
    /// Ephemeral X25519 pub used for this encryption (base64)
    pub ephemeral_pub: String,
    /// ChaCha20 nonce (base64)
    pub nonce: String,
    /// BLAKE3 hash of the unencrypted share (for integrity check on recovery)
    pub share_hash: String,
    /// Unix timestamp of creation
    pub created_at: u64,
}

/// Metadata stored locally about the recovery scheme (no secret material here)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoverySchemeMetadata {
    pub threshold: u8,
    pub total_shares: u8,
    pub guardian_ids: Vec<String>,
    pub created_at: u64,
    /// BLAKE3 hash of the master secret (used to verify reconstruction)
    pub master_hash: String,
}

/// Sensitive: the reconstructed master key. Zeroized on drop.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct ReconstructedKey(pub Vec<u8>);

/// Split a `master_key` into `total` shares, requiring `threshold` to recover.
///
/// Returns encrypted shares (one per guardian) + scheme metadata.
/// The caller is responsible for routing each share to its guardian.
pub fn create_recovery_shares(
    master_key: &[u8],
    threshold: u8,
    total: u8,
    guardians: &[(String, String, [u8; 32])], // (peer_id, name, x25519_pub)
) -> Result<(Vec<EncryptedShare>, RecoverySchemeMetadata)> {
    if threshold < 2 {
        anyhow::bail!("Threshold must be at least 2");
    }
    if total < threshold {
        anyhow::bail!("Total shares ({}) must be >= threshold ({})", total, threshold);
    }
    if guardians.len() != total as usize {
        anyhow::bail!("Guardian count ({}) must equal total shares ({})", guardians.len(), total);
    }
    if master_key.len() < 16 {
        anyhow::bail!("Master key too short: {} bytes", master_key.len());
    }

    // Generate raw shares
    let shares = shamir_split(master_key, threshold, total)
        .context("Shamir split failed")?;

    let rng = SystemRandom::new();
    let master_hash = blake3::hash(master_key).to_hex().to_string();
    let mut encrypted_shares = Vec::with_capacity(total as usize);

    for (i, ((peer_id, name, pub_key), raw_share)) in guardians.iter().zip(shares.iter()).enumerate() {
        let share_hash = blake3::hash(raw_share).to_hex().to_string();
        let encrypted = encrypt_share_for_guardian(raw_share, pub_key, &rng)
            .context(format!("Failed to encrypt share {} for {}", i + 1, name))?;

        encrypted_shares.push(EncryptedShare {
            share_index: (i + 1) as u8,
            guardian_id: peer_id.clone(),
            guardian_name: name.clone(),
            ciphertext: encrypted.0,
            ephemeral_pub: encrypted.1,
            nonce: encrypted.2,
            share_hash,
            created_at: now_secs(),
        });
    }

    let metadata = RecoverySchemeMetadata {
        threshold,
        total_shares: total,
        guardian_ids: guardians.iter().map(|(id, _, _)| id.clone()).collect(),
        created_at: now_secs(),
        master_hash,
    };

    Ok((encrypted_shares, metadata))
}

/// Reconstruct the master key from M decrypted shares.
///
/// `raw_shares`: Vec of (share_index, share_bytes) — must have >= threshold entries.
/// Returns the reconstructed key, verified against `expected_hash`.
pub fn reconstruct_key(
    raw_shares: &[(u8, Vec<u8>)],
    threshold: u8,
    expected_hash: &str,
) -> Result<ReconstructedKey> {
    if raw_shares.len() < threshold as usize {
        anyhow::bail!(
            "Not enough shares: have {}, need {}",
            raw_shares.len(),
            threshold
        );
    }

    let reconstructed = shamir_combine(raw_shares)
        .context("Shamir combine failed — shares may be corrupt or mismatched")?;

    // Verify integrity
    let actual_hash = blake3::hash(&reconstructed).to_hex().to_string();
    if actual_hash != expected_hash {
        anyhow::bail!(
            "Reconstructed key hash mismatch — recovery failed (wrong shares or tampering)"
        );
    }

    Ok(ReconstructedKey(reconstructed))
}

/// Decrypt a share that was encrypted for us (we are a guardian).
pub fn decrypt_share(
    encrypted: &EncryptedShare,
    our_x25519_secret: &x25519_dalek::StaticSecret,
) -> Result<Vec<u8>> {
    use base64::Engine;

    let ephemeral_pub_bytes: Vec<u8> =
        base64::engine::general_purpose::STANDARD.decode(&encrypted.ephemeral_pub)?;
    let ephemeral_arr: [u8; 32] = ephemeral_pub_bytes
        .try_into()
        .map_err(|_| anyhow::anyhow!("Bad ephemeral pub length"))?;

    let ephemeral_pub = x25519_dalek::PublicKey::from(ephemeral_arr);
    let shared = our_x25519_secret.diffie_hellman(&ephemeral_pub);
    let key_bytes = derive_with_domain(RECOVERY_KDF_DOMAIN, shared.as_bytes());
    let key = Key::from_slice(&key_bytes);
    let cipher = ChaCha20Poly1305::new(key);

    let nonce_bytes: Vec<u8> =
        base64::engine::general_purpose::STANDARD.decode(&encrypted.nonce)?;
    let nonce = Nonce::from_slice(&nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, encrypted.ciphertext.as_slice())
        .map_err(|_| anyhow::anyhow!("Share decryption failed"))?;

    // Verify share integrity
    let actual_hash = blake3::hash(&plaintext).to_hex().to_string();
    if actual_hash != encrypted.share_hash {
        anyhow::bail!("Share integrity check failed — share may be corrupt");
    }

    Ok(plaintext)
}

// ── Internal helpers ─────────────────────────────────────────────────────────

/// Encrypt raw share bytes for a guardian's X25519 public key.
/// Returns (ciphertext, ephemeral_pub_b64, nonce_b64)
fn encrypt_share_for_guardian(
    share: &[u8],
    guardian_pub: &[u8; 32],
    rng: &SystemRandom,
) -> Result<(Vec<u8>, String, String)> {
    use base64::Engine;

    let ephemeral_secret = x25519_dalek::EphemeralSecret::random_from_rng(rand::thread_rng());
    let ephemeral_pub = x25519_dalek::PublicKey::from(&ephemeral_secret);

    let guardian_x25519 = x25519_dalek::PublicKey::from(*guardian_pub);
    let shared = ephemeral_secret.diffie_hellman(&guardian_x25519);
    let key_bytes = derive_with_domain(RECOVERY_KDF_DOMAIN, shared.as_bytes());
    let key = Key::from_slice(&key_bytes);
    let cipher = ChaCha20Poly1305::new(key);

    let mut nonce_bytes = [0u8; 12];
    rng.fill(&mut nonce_bytes)
        .map_err(|_| anyhow::anyhow!("RNG failure"))?;
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, share)
        .map_err(|e| anyhow::anyhow!("Encryption failed: {:?}", e))?;

    Ok((
        ciphertext,
        base64::engine::general_purpose::STANDARD.encode(ephemeral_pub.as_bytes()),
        base64::engine::general_purpose::STANDARD.encode(nonce_bytes),
    ))
}

/// Thin wrapper around shamir_secret_sharing crate
fn shamir_split(secret: &[u8], threshold: u8, total: u8) -> Result<Vec<Vec<u8>>> {
    use shamir_secret_sharing::num_bigint::BigInt;
    use shamir_secret_sharing::ShamirSecretSharing;

    // Use the secp256k1 prime (standard in crypto)
    let prime = BigInt::parse_bytes(b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f", 16)
        .unwrap();

    let sss = ShamirSecretSharing {
        threshold: threshold as usize,
        share_amount: total as usize,
        prime,
    };

    // Convert secret bytes to BigInt
    let secret_bigint = BigInt::from_signed_bytes_be(secret);

    // Split into shares
    let shares = sss.split(secret_bigint);

    // Encode shares as bytes
    let mut encoded_shares: Vec<Vec<u8>> = Vec::new();
    for (idx, share_val) in shares {
        let mut encoded = Vec::new();
        encoded.push(idx as u8);
        encoded.extend_from_slice(&share_val.to_bytes_be().1);
        encoded_shares.push(encoded);
    }

    Ok(encoded_shares)
}

fn shamir_combine(shares: &[(u8, Vec<u8>)]) -> Result<Vec<u8>> {
    use shamir_secret_sharing::num_bigint::BigInt;
    use shamir_secret_sharing::ShamirSecretSharing;

    // Use the same prime as shamir_split
    let prime = BigInt::parse_bytes(b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f", 16)
        .unwrap();

    // Threshold is the number of shares we have
    let threshold = shares.len();

    let sss = ShamirSecretSharing {
        threshold,
        share_amount: threshold, // doesn't matter for combine
        prime,
    };

    // Decode shares from bytes to BigInt
    let decoded_shares: Vec<(usize, BigInt)> = shares
        .iter()
        .map(|(idx, data)| {
            let share_val = BigInt::from_signed_bytes_be(&data[..]);
            (*idx as usize, share_val)
        })
        .collect();

    // Recover the secret
    let secret_bigint = sss.recover(&decoded_shares);

    // Convert back to bytes
    let secret_bytes = secret_bigint.to_signed_bytes_be();
    Ok(secret_bytes)
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

// ── Additional Cargo.toml deps needed ────────────────────────────────────────
// blake3 = "1.5"
// num-bigint = "0.4"
// shamir_secret_sharing already present
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use x25519_dalek::{PublicKey, StaticSecret};

    fn make_guardian() -> (String, String, [u8; 32], StaticSecret) {
        let secret = StaticSecret::random_from_rng(rand::thread_rng());
        let pub_key: PublicKey = (&secret).into();
        let pub_bytes = *pub_key.as_bytes();
        (
            uuid::Uuid::new_v4().to_string(),
            "Guardian Name".to_string(),
            pub_bytes,
            secret,
        )
    }

    #[test]
    fn encrypt_decrypt_share() {
        let (id, name, pub_bytes, secret) = make_guardian();
        let raw_share = b"this is my secret share bytes 12";
        let rng = SystemRandom::new();

        let (ct, epk, nonce) = encrypt_share_for_guardian(raw_share, &pub_bytes, &rng).unwrap();
        let share_hash = blake3::hash(raw_share).to_hex().to_string();

        let enc = EncryptedShare {
            share_index: 1,
            guardian_id: id,
            guardian_name: name,
            ciphertext: ct,
            ephemeral_pub: epk,
            nonce,
            share_hash,
            created_at: 0,
        };

        let recovered = decrypt_share(&enc, &secret).unwrap();
        assert_eq!(recovered, raw_share);
    }
}
