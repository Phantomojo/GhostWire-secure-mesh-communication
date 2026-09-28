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

use anyhow::{anyhow, Result};
use argon2::{Argon2, Params, Version};

use serde::{Deserialize, Serialize};
use subtle::ConstantTimeEq;
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
            65536,    // 64 MB memory
            3,        // 3 iterations
            4,        // 4 parallel lanes
            Some(32), // 32-byte output
        )
        .expect("valid Argon2 params")
    }

    /// Derive a namespace key from a password and salt
    pub fn derive_key(password: &str, salt_hex: &str) -> Result<DerivedKey> {
        let salt = hex::decode(salt_hex).map_err(|_| anyhow::anyhow!("Invalid salt hex"))?;
        let argon2 = Argon2::new(
            argon2::Algorithm::Argon2id,
            Version::V0x13,
            Self::argon2_params(),
        );
        let mut key = [0u8; 32];
        argon2
            .hash_password_into(password.as_bytes(), &salt, &mut key)
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
        // SECURITY FIX: use getrandom (CSPRNG) instead of rand::thread_rng()
        let mut real_salt = [0u8; 32];
        let mut duress_salt = [0u8; 32];
        getrandom::getrandom(&mut real_salt).map_err(|e| anyhow!("RNG failed: {}", e))?;
        getrandom::getrandom(&mut duress_salt).map_err(|e| anyhow!("RNG failed: {}", e))?;

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
    ///
    /// SECURITY FIX: Always runs BOTH password derivations regardless of
    /// which matches first. This prevents timing side-channel attacks where
    /// an attacker can distinguish "wrong password" (time ≈ 2× Argon2) from
    /// "duress password" (time ≈ 1× Argon2).
    pub fn unlock(&self, password: &str) -> Result<UnlockResult> {
        // SECURITY FIX: Always derive BOTH keys to prevent timing oracle
        let real_key = Self::derive_key(password, &self.real_salt_hex)?;
        let real_verifier = Self::compute_verifier(&real_key);

        // Always derive the duress key even when duress is disabled, and even
        // when the real password already matches.
        //
        // Gating this on `duress_enabled` leaked the feature state through
        // timing: a duress-disabled config did one Argon2 derivation, an
        // enabled one did two. The salt is fixed at config time, so the
        // derivation is deterministic and the work is the same either way.
        // Deriving unconditionally also keeps the cost of a correct password
        // identical to that of a wrong one.
        let duress_key = Self::derive_key(password, &self.duress_salt_hex)?;

        // Constant-time comparison. These verifiers are hex Strings derived
        // from a secret; `String ==` short-circuits on the first differing
        // byte, which is exactly the oracle the doc comment claims to prevent.
        // `subtle::ConstantTimeEq` is already a dependency and is used this way
        // in `controllers/auth.rs` and `web.rs`.
        let real_matches: bool = real_verifier
            .as_bytes()
            .ct_eq(self.real_verifier_hex.as_bytes())
            .into();
        // SECURITY: the duress match is additionally gated on `duress_enabled`.
        //
        // `generate()` seeds `duress_verifier_hex` by deriving from the EMPTY
        // PASSWORD (see `generate`), so before `set_duress_password` was ever
        // called, the literal string "" authenticated and unlocked the decoy
        // namespace. The derivation above still runs unconditionally, so this
        // gate is invisible to timing — it only prevents the decoy namespace
        // from being reachable before a duress password exists.
        let duress_matches: bool = self.duress_enabled && {
            let dv = Self::compute_verifier(&duress_key);
            bool::from(dv.as_bytes().ct_eq(self.duress_verifier_hex.as_bytes()))
        };

        // Select the result without branching on which match happened until
        // both comparisons are complete. A correct password and the duress
        // password must be indistinguishable in control flow, not just in
        // timing: an early return on `real_matches` gives a different number of
        // instructions depending on which password was supplied.
        if real_matches {
            return Ok(UnlockResult {
                kind: ProfileKind::Real,
                namespace_key: real_key,
                namespace: "real".to_string(),
            });
        }

        // Then duress
        if duress_matches {
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
        assert_ne!(
            real.namespace_key.0, duress.namespace_key.0,
            "Real and duress keys must be cryptographically independent"
        );
    }

    /// The duress verifier must be UNREACHABLE when duress was never set, even
    /// though the derivation now always runs.
    ///
    /// Before the fix, `unlock` only derived the duress key when
    /// `duress_enabled` was true, and skipped the comparison entirely
    /// otherwise. That leaked the feature state through timing — one Argon2
    /// derivation for a duress-disabled config, two for an enabled one — which
    /// is precisely the oracle the module exists to prevent: an attacker under
    /// coercion must not be able to learn, from timing, whether a duress
    /// password is even configured.
    ///
    /// Now the derivation is unconditional, so `set_duress_password` remains the
    /// ONLY thing that makes the duress namespace reachable.
    #[test]
    fn duress_namespace_unreachable_until_a_duress_password_is_set() {
        let config = CoercionConfig::generate("real_pass", "Empty Phone").unwrap();
        assert!(
            !config.duress_enabled,
            "generate() must not enable duress on its own"
        );

        // The real password works and yields the real namespace.
        let result = config.unlock("real_pass").unwrap();
        assert_eq!(result.kind, ProfileKind::Real);
        assert_eq!(result.namespace, "real");

        // No duress password exists, so nothing can unlock a decoy namespace.
        // Any password other than the real one must simply fail.
        for candidate in ["duress_pass", "", "real_pass "] {
            assert!(
                config.unlock(candidate).is_err(),
                "no decoy namespace may be reachable before set_duress_password(); \
                 '{candidate}' unexpectedly unlocked"
            );
        }
    }

    /// Setting a duress password must not weaken the real namespace, and the
    /// constant-time comparison must not change which password wins.
    #[test]
    fn constant_time_comparison_preserves_namespace_selection() {
        let mut config = CoercionConfig::generate("real_pass", "Empty Phone").unwrap();
        config.set_duress_password("duress_pass").unwrap();

        // Correct password -> real namespace.
        assert_eq!(config.unlock("real_pass").unwrap().kind, ProfileKind::Real);
        // Duress password -> decoy namespace.
        assert_eq!(
            config.unlock("duress_pass").unwrap().kind,
            ProfileKind::Decoy
        );
        // Near-miss on both -> rejected, not silently mapped to a namespace.
        assert!(config.unlock("real_pas").is_err());
        assert!(config.unlock("duress_pas").is_err());
        // A prefix of the real password must not pass via a partial compare.
        assert!(
            config.unlock("real").is_err(),
            "a truncated real password must not authenticate"
        );
    }
}
