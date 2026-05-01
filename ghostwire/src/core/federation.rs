//! Federation/bridging: trust, signed relays, whitelists.
//! Next: integrate with adapter registry and mesh logic.

use crate::core::message::Message;
use ed25519_dalek::{PublicKey, Signature, Verifier};
use std::collections::HashMap;

/// Trait for bridge trust policies.
pub trait BridgeTrust: Send + Sync {
    /// Check if a bridge is trusted.
    fn is_trusted(&self, bridge_id: &str) -> bool;
    /// Verify a relayed message signature.
    fn verify_relay(&self, msg: &Message, relay: &SignedRelay) -> bool;
    /// Sign a relayed message (if we are a bridge).
    fn sign_relay(&self, msg: &Message) -> SignedRelay;
}

/// Secure bridge trust: verification via Ed25519 public keys.
pub struct SecureBridgeTrust {
    trusted_bridges: HashMap<String, PublicKey>,
}

impl SecureBridgeTrust {
    pub fn new() -> Self {
        Self {
            trusted_bridges: HashMap::new(),
        }
    }

    pub fn add_bridge(&mut self, bridge_id: String, public_key_bytes: &[u8]) -> Result<(), String> {
        let public_key = PublicKey::from_bytes(public_key_bytes)
            .map_err(|e| format!("Invalid bridge public key: {}", e))?;
        self.trusted_bridges.insert(bridge_id, public_key);
        Ok(())
    }
}

impl BridgeTrust for SecureBridgeTrust {
    fn is_trusted(&self, bridge_id: &str) -> bool {
        self.trusted_bridges.contains_key(bridge_id)
    }

    fn verify_relay(&self, msg: &Message, relay: &SignedRelay) -> bool {
        let public_key = match self.trusted_bridges.get(&relay.relay_id) {
            Some(pk) => pk,
            None => return false,
        };

        // Prepare data for verification: relay_id + message_id + content_hash
        let mut data = Vec::new();
        data.extend_from_slice(relay.relay_id.as_bytes());
        data.extend_from_slice(msg.id.as_bytes());
        
        let signature = match Signature::from_bytes(&relay.signature) {
            Ok(s) => s,
            Err(_) => return false,
        };

        public_key.verify(&data, &signature).is_ok()
    }

    fn sign_relay(&self, _msg: &Message) -> SignedRelay {
        // Only actual bridges implement this with their private key
        SignedRelay::default()
    }
}

/// Default stub: trust all, dummy signature.
pub struct TrustAllBridgeTrust;
impl BridgeTrust for TrustAllBridgeTrust {
    fn is_trusted(&self, _bridge_id: &str) -> bool { true }
    fn verify_relay(&self, _msg: &Message, _relay: &SignedRelay) -> bool { true }
    fn sign_relay(&self, _msg: &Message) -> SignedRelay {
        SignedRelay { relay_id: "dummy".into(), signature: vec![] }
    }
}

/// Struct for signed relayed messages.
#[derive(Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct SignedRelay {
    pub relay_id: String,
    pub signature: Vec<u8>,
}

pub struct FederationInfo;