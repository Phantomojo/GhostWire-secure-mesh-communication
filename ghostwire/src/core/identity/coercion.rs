//! coercion.rs — Duress PIN and decoy profile system
//!
//! Protects against the wrench attack: physical coercion to reveal password.
//!
//! Architecture:
//!   Real password  → Argon2(real_password, salt_a)  → real identity namespace
//!   Duress password → Argon2(duress_password, salt_b) → decoy identity namespace
//!
//! The EncryptedStore already supports namespaces. This module manages two
//! parallel namespaces behind two separate Argon2 derivations. Neither key
//! can decrypt the other's namespace — they are cryptographically independent.
//!
//! From the attacker's perspective: they see one unlock screen, get one
//! result. They cannot tell which namespace they unlocked without already
//! knowing which password they used. A clean decoy profile is indistinguishable
//! from a real profile with no messages yet.

use anyhow::Result;
use argon2::{Argon2, Params, Version};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Which profile a given password decrypts
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileKind {
    Real,
    Decoy,
}

/// The result of an unlock attempt
#[derive(Debug)]
pub struct UnlockResult {
    pub kind: ProfileKind,
    /// The derived 32-byte symmetric key for this namespace
    pub namespace_key: DerivedKey,
    /// Namespace string to pass to EncryptedStore
    pub namespace: String,
}

/// Zeroized key wrapper — memory is wiped when dropped
#[derive(Debug, Zeroize, ZeroizeOnDrop)]
pub struct DerivedKey(pub [u8; 32]);

/// Persisted coercion configuration (stored in the real namespace)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoercionConfig {
    /// Argon2 salt for the real password (32 bytes, hex)
    pub real_salt_hex: String,
    /// Argon2 salt for the duress password (32 bytes, hex)  
    pub duress_salt_hex: String,
    /// BLAKE3 verifier for the real password
    /// (so we can check unlock without decrypting everything)
    pub real_verifier_hex: String,
    /// BLAKE3 verifier for the duress password
    pub duress_verifier_hex: String,
    /// Whether duress mode is configured
    pub duress_enabled: bool,
    /// Decoy profile display name (what an attacker sees)
    pub decoy_display_name: String,
}

impl CoercionConfig {
    /// Argon2id parameters — tuned for ~1 second on a mid-range phone
    fn argon2_params() -> Params {
        Params::new(
            65536,  // 64 MB memory
            3,      // 3 iterations
            4,      // 4 parallel lanes
            Some(32), // 32-byte output
        ).expect("valid Argon2 params")
    }

    /// Derive a namespace key from a password and salt
    pub fn derive_key(password: &str, salt_hex: &str) -> Result<DerivedKey> {
        let salt = hex::decode(salt_hex)
            .map_err(|_| anyhow::anyhow!("Invalid salt hex"))?;
        let argon2 = Argon2::new(
            argon2::Algorithm::Argon2id,
            Version::V0x13,
            Self::argon2_params(),
        );
        let mut key = [0u8; 32];
        argon2.hash_password_into(password.as_bytes(), &salt, &mut key)
            .map_err(|e| anyhow::anyhow!("Argon2 failed: {:?}", e))?;
        Ok(DerivedKey(key))
    }

    /// Compute verifier: BLAKE3(derived_key || "ghostwire coercion verifier v1")
    fn compute_verifier(key: &DerivedKey) -> String {
        use crate::genesis_constants::HANDSHAKE_DOMAIN;
        let hash = crate::genesis_constants::genesis_derive(HANDSHAKE_DOMAIN, &key.0);
        hex::encode(hash)
    }

    /// Generate a fresh coercion config for a new identity
    pub fn generate(real_password: &str, decoy_display_name: &str) -> Result<Self> {
        let mut rng = rand::thread_rng();

        let mut real_salt = [0u8; 32];
        let mut duress_salt = [0u8; 32];
        rng.fill_bytes(&mut real_salt);
        rng.fill_bytes(&mut duress_salt);

        let real_key = Self::derive_key(real_password, &hex::encode(real_salt))?;
        // Duress password starts empty — user sets it later via set_duress_password()
        let duress_key = Self::derive_key("", &hex::encode(duress_salt))?;

        let real_verifier = Self::compute_verifier(&real_key);
        let duress_verifier = Self::compute_verifier(&duress_key);

        Ok(Self {
            real_salt_hex: hex::encode(real_salt),
            duress_salt_hex: hex::encode(duress_salt),
            real_verifier_hex: real_verifier,
            duress_verifier_hex: duress_verifier,
            duress_enabled: false,
            decoy_display_name: decoy_display_name.to_string(),
        })
    }

    /// Set the duress password
    pub fn set_duress_password(&mut self, duress_password: &str) -> Result<DerivedKey> {
        let key = Self::derive_key(duress_password, &self.duress_salt_hex)?;
        self.duress_verifier_hex = Self::compute_verifier(&key);
        self.duress_enabled = true;
        Ok(key)
    }

    /// Attempt to unlock with the given password.
    /// Returns which namespace it unlocked, or Err if wrong password.
    pub fn unlock(&self, password: &str) -> Result<UnlockResult> {
        // Try real password first
        let real_key = Self::derive_key(password, &self.real_salt_hex)?;
        let real_verifier = Self::compute_verifier(&real_key);
        if real_verifier == self.real_verifier_hex {
            return Ok(UnlockResult {
                kind: ProfileKind::Real,
                namespace_key: real_key,
                namespace: "real".to_string(),
            });
        }

        // Try duress password
        if self.duress_enabled {
            let duress_key = Self::derive_key(password, &self.duress_salt_hex)?;
            let duress_verifier = Self::compute_verifier(&duress_key);
            if duress_verifier == self.duress_verifier_hex {
                tracing::warn!(
                    "DURESS PASSWORD USED — activating decoy profile '{}'",
                    self.decoy_display_name
                );
                return Ok(UnlockResult {
                    kind: ProfileKind::Decoy,
                    namespace_key: duress_key,
                    namespace: "decoy".to_string(),
                });
            }
        }

        anyhow::bail!("Incorrect password")
    }

    /// Check if a password is the duress password without revealing which it is
    /// to the caller. Used internally for logging and alerting only.
    pub fn is_duress_unlock(&self, unlock_result: &UnlockResult) -> bool {
        unlock_result.kind == ProfileKind::Decoy
    }
}

/// Duress alert: fires when duress password is used
/// Sends a pre-configured alert through Ghost Channels before anything else
pub struct DuressAlert {
    /// Pre-composed alert message
    pub alert_text: String,
    /// Ghost Channel IDs to send the alert to
    pub channel_ids: Vec<[u8; 32]>,
    /// Whether to also fire the dead man's switch
    pub trigger_dead_mans_switch: bool,
}

impl DuressAlert {
    pub fn new(alert_text: impl Into<String>) -> Self {
        Self {
            alert_text: alert_text.into(),
            channel_ids: Vec::new(),
            trigger_dead_mans_switch: true,
        }
    }

    pub fn add_channel(mut self, channel_id: [u8; 32]) -> Self {
        self.channel_ids.push(channel_id);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_password_unlocks_real_namespace() {
        let config = CoercionConfig::generate("correcthorsebattery", "Mike's Phone").unwrap();
        let result = config.unlock("correcthorsebattery").unwrap();
        assert_eq!(result.kind, ProfileKind::Real);
        assert_eq!(result.namespace, "real");
    }

    #[test]
    fn wrong_password_rejected() {
        let config = CoercionConfig::generate("correcthorsebattery", "Decoy").unwrap();
        assert!(config.unlock("wrongpassword").is_err());
    }

    #[test]
    fn duress_password_unlocks_decoy_namespace() {
        let mut config = CoercionConfig::generate("real_pass", "Empty Phone").unwrap();
        config.set_duress_password("duress_pass").unwrap();

        let result = config.unlock("duress_pass").unwrap();
        assert_eq!(result.kind, ProfileKind::Decoy);
        assert_eq!(result.namespace, "decoy");
    }

    #[test]
    fn real_password_still_works_after_duress_set() {
        let mut config = CoercionConfig::generate("real_pass", "Empty Phone").unwrap();
        config.set_duress_password("duress_pass").unwrap();

        let result = config.unlock("real_pass").unwrap();
        assert_eq!(result.kind, ProfileKind::Real);
    }

    #[test]
    fn derived_keys_are_different_for_real_and_duress() {
        let mut config = CoercionConfig::generate("real_pass", "Empty").unwrap();
        config.set_duress_password("duress_pass").unwrap();

        let real = config.unlock("real_pass").unwrap();
        let duress = config.unlock("duress_pass").unwrap();
        assert_ne!(real.namespace_key.0, duress.namespace_key.0,
            "Real and duress keys must be cryptographically independent");
    }
}
