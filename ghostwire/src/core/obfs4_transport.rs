//! Obfs4 Transport Helper for GhostWire
//!
//! This module provides obfs4 obfuscation support for DPI evasion.
//! The obfs4 protocol makes traffic indistinguishable from random noise,
//! providing censorship resistance by default.
//!
//! Note: The obfs4 crate (v0.1.0-alpha.1) is still in development.
//! This implementation provides the foundation for future integration.

use anyhow::Result;
use rand_core::OsRng;
use rand_core::RngCore;

/// Obfs4 configuration
#[derive(Debug, Clone)]
pub struct Obfs4Config {
    /// Enable obfs4 obfuscation (default: true)
    pub enabled: bool,
    /// Obfs4 secret key (if None, generated randomly)
    pub secret_key: Option<[u8; 32]>,
    /// Public key for server mode (clients need this to connect)
    pub public_key: Option<Vec<u8>>,
}

impl Default for Obfs4Config {
    fn default() -> Self {
        Self {
            enabled: true,
            secret_key: None,
            public_key: None,
        }
    }
}

/// Obfs4 state holder for the transport
pub struct Obfs4State {
    config: Obfs4Config,
    public_key: Vec<u8>,
    secret_key: [u8; 32],
}

impl Obfs4State {
    /// Create a new obfs4 state with generated keys
    pub fn new(config: Obfs4Config) -> Result<Self> {
        let secret = if let Some(key) = config.secret_key {
            key
        } else {
            let mut secret = [0u8; 32];
            OsRng.fill_bytes(&mut secret);
            secret
        };

        // The public key MUST NOT be the secret.
        //
        // This read `let public_key = secret.to_vec();` with a "Placeholder -
        // real impl uses elligator2" comment. So anything that published or
        // transmitted this "public key" was handing out the secret: a peer
        // configured with it could derive the node's obfuscation key, and a
        // `serialize_public_key` of it would have leaked the secret to every
        // log, config file and bootstrap record it touched.
        //
        // It was never exploited only because nothing constructs an Obfs4State
        // outside its own tests. The placeholder is removed rather than fixed
        // with another placeholder: there is no meaningful "public key" without
        // a real handshake, so this no longer invents one.
        //
        // `public_key` stays empty until a real obfs4 handshake is implemented;
        // `is_usable()` reports false so no caller can treat this as ready.

        Ok(Self {
            config,
            public_key: Vec::new(),
            secret_key: secret,
        })
    }

    /// Get the public key for sharing with peers
    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    /// Whether the operator asked for obfs4.
    ///
    /// This reflects the CONFIG flag only. It does NOT mean obfs4 is working:
    /// there is no obfs4 handshake, framing or socket wrapper in this module,
    /// and none is applied to the swarm transport. Use `is_usable()` for
    /// "can this actually carry obfuscated traffic".
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    /// Whether obfs4 can actually be used. Always false today.
    ///
    /// A real obfs4 needs an Elligator2 key exchange, a SipHash-authenticated
    /// handshake, the obfs4 length-prefixed framing with padding and salt, and a
    /// TCP stream wrapper wired into the libp2p transport chain. None of that
    /// exists here. This exists so that no future caller can read `enabled:
    /// true` and conclude that obfuscation is active.
    pub fn is_usable(&self) -> bool {
        false
    }

    /// Get the secret key (for advanced usage)
    pub fn secret_key(&self) -> &[u8; 32] {
        &self.secret_key
    }
}

/// Generate a new obfs4 keypair for sharing with peers.
///
/// Returns `(secret, public)`. The public half is EMPTY until a real obfs4
/// handshake exists -- see the note in `Obfs4State::new`. It previously
/// returned `secret.to_vec()`, which meant the "public" key was the secret.
pub fn generate_obfs4_keypair() -> ([u8; 32], Vec<u8>) {
    let mut secret = [0u8; 32];
    OsRng.fill_bytes(&mut secret);
    (secret, Vec::new())
}

/// Serialize obfs4 public key for transmission
pub fn serialize_public_key(public_key: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(public_key)
}

/// Deserialize obfs4 public key from transmission
pub fn deserialize_public_key(encoded: &str) -> Result<Vec<u8>> {
    use base64::Engine;
    Ok(base64::engine::general_purpose::STANDARD.decode(encoded)?)
}

/// Get obfs4 version info
pub fn obfs4_version() -> &'static str {
    "0.1.0-alpha.1 (ptrs)"
}

/// Check if obfs4 is ready for use
/// Note: The current obfs4 crate is alpha quality
pub fn is_obfs4_production_ready() -> bool {
    // Not merely "alpha". There is no obfs4 implementation in this crate at
    // all -- no handshake, no framing, no socket wrapper, and the previous
    // "public key" was the secret key. It is not wired into the swarm. Treat
    // GhostWire as having NO transport obfuscation until that changes.
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keypair_never_returns_the_secret_as_public() {
        let (secret, public) = generate_obfs4_keypair();
        assert_eq!(secret.len(), 32);
        // The public half is empty until a real handshake exists. It used to be
        // `secret.to_vec()`, so the "public key" was the secret key and
        // `serialize_public_key` would have leaked it into any log, config or
        // bootstrap record.
        assert!(
            public.is_empty(),
            "a placeholder public key is worse than none: it must not be the secret"
        );
        assert!(
            public != secret.to_vec(),
            "the public key must never equal the secret key"
        );
    }

    #[test]
    fn obfs4_state_never_exposes_the_secret_as_a_public_key() {
        let state = Obfs4State::new(Obfs4Config::default()).unwrap();
        assert!(
            state.public_key() != state.secret_key(),
            "public_key() must never return the secret key"
        );
        assert!(
            state.public_key().is_empty(),
            "no meaningful public key exists without a real handshake"
        );
    }

    #[test]
    fn enabled_does_not_imply_usable() {
        // The config flag defaults to true, so anything that treated
        // `is_enabled()` as "obfuscation is working" would report a
        // protection the node does not have.
        let state = Obfs4State::new(Obfs4Config::default()).unwrap();
        assert!(state.is_enabled(), "the config flag is still honoured");
        assert!(
            !state.is_usable(),
            "is_usable() must be false: there is no obfs4 implementation"
        );
        assert!(!is_obfs4_production_ready());
    }

    #[test]
    fn test_config_default() {
        let config = Obfs4Config::default();
        assert!(config.enabled);
        assert!(config.secret_key.is_none());
        assert!(config.public_key.is_none());
    }

    #[test]
    fn test_serialize_deserialize_key() {
        let (_, public) = generate_obfs4_keypair();
        let encoded = serialize_public_key(&public);
        let decoded = deserialize_public_key(&encoded).unwrap();
        assert_eq!(public, decoded);
    }

    #[test]
    fn test_obfs4_state_creation() {
        let config = Obfs4Config::default();
        let state = Obfs4State::new(config).unwrap();
        assert!(state.is_enabled());
        assert!(state.public_key().is_empty());
    }

    #[test]
    fn test_version() {
        assert!(!obfs4_version().is_empty());
    }

    #[test]
    fn test_production_ready() {
        // Should return false for alpha version
        assert!(!is_obfs4_production_ready());
    }
}
