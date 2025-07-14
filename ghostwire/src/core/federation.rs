//! Federation/bridging: trust, signed relays, whitelists.
//! Next: integrate with adapter registry and mesh logic.

/// Trait for bridge trust policies.
pub trait BridgeTrust {
    /// Check if a bridge is trusted.
    fn is_trusted(&self, bridge_id: &str) -> bool;
    /// Sign a relayed message.
    fn sign_relay(&self, msg: &Message) -> SignedRelay;
}

/// Default stub: trust all, dummy signature.
pub struct TrustAllBridgeTrust;
impl BridgeTrust for TrustAllBridgeTrust {
    fn is_trusted(&self, _bridge_id: &str) -> bool { true }
    fn sign_relay(&self, _msg: &Message) -> SignedRelay {
        SignedRelay { relay_id: "dummy".into(), signature: vec![] }
    }
}

/// Struct for signed relayed messages.
#[derive(Default, Clone)]
pub struct SignedRelay {
    pub relay_id: String,
    pub signature: Vec<u8>,
}

/// Placeholder type for demo
pub struct Message;
pub struct FederationInfo;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>; 