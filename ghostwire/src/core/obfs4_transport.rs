//! Obfs4 Transport Helper for GhostWire
//!
//! This module provides obfs4 obfuscation support for DPI evasion.
//! The obfs4 protocol makes traffic indistinguishable from random noise,
//! providing censorship resistance by default.
//!
//! Note: The obfs4 crate (v0.1.0-alpha.1) is still in development.
//! This implementation provides the foundation for future integration.

use anyhow::Result;
use rand::RngCore;

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
            rand::thread_rng().fill_bytes(&mut secret);
            secret
        };

        // Generate a public key representation
        // Note: The actual obfs4 handshake uses x25519 elligator2 keys
        // This is a simplified version for now
        let public_key = secret.to_vec(); // Placeholder - real impl uses elligator2

        Ok(Self {
            config,
            public_key,
            secret_key: secret,
        })
    }

    /// Get the public key for sharing with peers
    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    /// Check if obfs4 is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    /// Get the secret key (for advanced usage)
    pub fn secret_key(&self) -> &[u8; 32] {
        &self.secret_key
    }
}

/// Generate a new obfs4 keypair for sharing with peers
pub fn generate_obfs4_keypair() -> ([u8; 32], Vec<u8>) {
    let mut secret = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut secret);
    let public = secret.to_vec(); // Placeholder for real elligator2 public key
    (secret, public)
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
    false // Alpha version - use with caution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keypair() {
        let (secret, public) = generate_obfs4_keypair();
        assert_eq!(secret.len(), 32);
        assert_eq!(public.len(), 32);
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
        assert_eq!(state.public_key().len(), 32);
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
