//! GhostWire world-scale discovery primitives.
//!
//! Discovery is evidence, not trust. The wire advertisement is independently
//! signed with the node's GhostWire/libp2p identity and is valid only for one
//! network namespace and a bounded lifetime.

use anyhow::{anyhow, bail, Result};
use libp2p::{identity::PublicKey, Multiaddr, PeerId};
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

pub const PROTOCOL_VERSION: u16 = 1;
pub const DOMAIN_SEPARATOR: &[u8] = b"ghostwire-world-discovery-v1\0";
pub const MAX_ADVERTISEMENT_BYTES: usize = 8 * 1024;
pub const MAX_ADDRESSES: usize = 8;
pub const MAX_CAPABILITIES: usize = 16;
pub const MAX_TTL_SECS: u64 = 15 * 60;
pub const MIN_TTL_SECS: u64 = 15;
pub const CLOCK_SKEW_SECS: u64 = 60;
pub const MAX_NETWORK_ID_LEN: usize = 64;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldDiscoveryAdvertisement {
    pub version: u16,
    pub network_id: String,
    pub peer_id: String,
    pub did: String,
    /// Libp2p public-key protobuf bytes. This is public material.
    pub public_key: Vec<u8>,
    /// Listener addresses without a `/p2p` suffix; the receiver adds the
    /// authenticated issuer PeerID before dialing.
    pub addresses: Vec<String>,
    pub issued_at: u64,
    pub expires_at: u64,
    pub sequence: u64,
    pub capabilities: Vec<String>,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct WorldDiscoveryHealth {
    pub enabled: bool,
    pub network_id: String,
    pub topic: String,
    pub state: String,
    pub published_total: u64,
    pub accepted_total: u64,
    pub rejected_total: u64,
    pub expired_total: u64,
    pub candidate_count: usize,
    pub last_published_unix: u64,
    pub last_received_unix: u64,
    pub provider_active: bool,
    pub last_error: Option<String>,
}

impl WorldDiscoveryHealth {
    pub fn new(enabled: bool, network_id: impl Into<String>) -> Self {
        let network_id = network_id.into();
        Self {
            enabled,
            topic: discovery_topic(&network_id),
            network_id,
            state: if enabled { "starting" } else { "disabled" }.into(),
            published_total: 0,
            accepted_total: 0,
            rejected_total: 0,
            expired_total: 0,
            candidate_count: 0,
            last_published_unix: 0,
            last_received_unix: 0,
            provider_active: false,
            last_error: None,
        }
    }
}

pub type SharedWorldDiscoveryHealth = Arc<RwLock<WorldDiscoveryHealth>>;

pub fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub fn discovery_topic(network_id: &str) -> String {
    format!("/ghostwire/discovery/v1/{network_id}")
}

pub fn provider_record_key(network_id: &str) -> Vec<u8> {
    format!("/ghostwire/discovery/providers/v1/{network_id}").into_bytes()
}

pub fn advertisement_record_key(network_id: &str, peer_id: &str) -> Vec<u8> {
    format!("/ghostwire/discovery/record/v1/{network_id}/{peer_id}").into_bytes()
}

pub fn private_endpoints_allowed() -> bool {
    std::env::var_os("GHOSTWIRE_DISCOVERY_INCLUDE_PRIVATE").is_some()
}

pub fn configured_network_id() -> String {
    std::env::var("GHOSTWIRE_NETWORK_ID").unwrap_or_else(|_| "ghostwire-one".to_string())
}

pub fn validate_network_id(network_id: &str) -> Result<()> {
    if network_id.is_empty() || network_id.len() > MAX_NETWORK_ID_LEN {
        bail!("invalid GhostWire discovery network id length");
    }
    if !network_id
        .bytes()
        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'.' | b'_' | b'-'))
    {
        bail!("invalid GhostWire discovery network id characters");
    }
    Ok(())
}

fn is_allowed_ip(ip: IpAddr, allow_private: bool) -> bool {
    match ip {
        IpAddr::V4(v4) => {
            if v4.is_unspecified()
                || v4.is_loopback()
                || v4.is_multicast()
                || v4.is_broadcast()
                || v4.is_link_local()
            {
                return false;
            }
            allow_private || !v4.is_private()
        }
        IpAddr::V6(v6) => {
            if v6.is_unspecified() || v6.is_loopback() || v6.is_multicast() {
                return false;
            }
            allow_private || !is_unique_local_v6(v6)
        }
    }
}

fn is_unique_local_v6(ip: Ipv6Addr) -> bool {
    (ip.segments()[0] & 0xfe00) == 0xfc00
}

pub fn validate_address(address: &str, allow_private: bool) -> Result<Multiaddr> {
    let parsed = address
        .parse::<Multiaddr>()
        .map_err(|e| anyhow!("invalid discovery multiaddress: {e}"))?;
    let mut saw_ip = false;
    for protocol in parsed.iter() {
        match protocol {
            libp2p::multiaddr::Protocol::Ip4(ip) => {
                saw_ip = true;
                if !is_allowed_ip(IpAddr::V4(ip), allow_private) {
                    bail!("discovery address uses a disallowed IPv4 endpoint");
                }
            }
            libp2p::multiaddr::Protocol::Ip6(ip) => {
                saw_ip = true;
                if !is_allowed_ip(IpAddr::V6(ip), allow_private) {
                    bail!("discovery address uses a disallowed IPv6 endpoint");
                }
            }
            libp2p::multiaddr::Protocol::Dns(_)
            | libp2p::multiaddr::Protocol::Dns4(_)
            | libp2p::multiaddr::Protocol::Dns6(_)
            | libp2p::multiaddr::Protocol::Dnsaddr(_) => {}
            libp2p::multiaddr::Protocol::P2p(_) => {
                bail!("discovery advertisement address must not embed a PeerID");
            }
            _ => {}
        }
    }
    if !saw_ip {
        // DNS addresses are allowed only when explicitly present and bounded;
        // the transport still performs normal DNS resolution and policy checks.
        let has_dns = parsed.iter().any(|protocol| {
            matches!(
                protocol,
                libp2p::multiaddr::Protocol::Dns(_)
                    | libp2p::multiaddr::Protocol::Dns4(_)
                    | libp2p::multiaddr::Protocol::Dns6(_)
                    | libp2p::multiaddr::Protocol::Dnsaddr(_)
            )
        });
        if !has_dns {
            bail!("discovery address has no routable IP or DNS component");
        }
    }
    Ok(parsed)
}

impl WorldDiscoveryAdvertisement {
    pub fn unsigned(
        network_id: impl Into<String>,
        peer_id: PeerId,
        did: impl Into<String>,
        public_key: Vec<u8>,
        addresses: Vec<String>,
        sequence: u64,
        issued_at: u64,
        expires_at: u64,
        capabilities: Vec<String>,
    ) -> Self {
        Self {
            version: PROTOCOL_VERSION,
            network_id: network_id.into(),
            peer_id: peer_id.to_string(),
            did: did.into(),
            public_key,
            addresses,
            issued_at,
            expires_at,
            sequence,
            capabilities,
            signature: Vec::new(),
        }
    }

    pub fn signing_bytes(&self) -> Result<Vec<u8>> {
        let mut unsigned = self.clone();
        unsigned.signature.clear();
        let encoded = serde_json::to_vec(&unsigned)?;
        let mut bytes = Vec::with_capacity(DOMAIN_SEPARATOR.len() + encoded.len());
        bytes.extend_from_slice(DOMAIN_SEPARATOR);
        bytes.extend_from_slice(&encoded);
        Ok(bytes)
    }

    pub fn sign_with<F>(mut self, signer: F) -> Result<Self>
    where
        F: FnOnce(&[u8]) -> Option<Vec<u8>>,
    {
        self.signature =
            signer(&self.signing_bytes()?).ok_or_else(|| anyhow!("identity is locked"))?;
        Ok(self)
    }

    pub fn verify(
        &self,
        expected_network_id: &str,
        now: u64,
        allow_private: bool,
    ) -> Result<PeerId> {
        if self.version != PROTOCOL_VERSION {
            bail!("unsupported discovery advertisement version");
        }
        validate_network_id(expected_network_id)?;
        if self.network_id != expected_network_id {
            bail!("discovery advertisement network scope mismatch");
        }
        if self.did.len() > 256 || self.signature.is_empty() || self.signature.len() > 128 {
            bail!("invalid discovery advertisement identity/signature fields");
        }
        if self.addresses.is_empty() || self.addresses.len() > MAX_ADDRESSES {
            bail!("invalid discovery advertisement address count");
        }
        if self.capabilities.len() > MAX_CAPABILITIES {
            bail!("invalid discovery capability count");
        }
        if self.expires_at <= self.issued_at
            || self.expires_at - self.issued_at > MAX_TTL_SECS
            || self.expires_at < now
            || self.issued_at > now.saturating_add(CLOCK_SKEW_SECS)
        {
            bail!("expired or out-of-window discovery advertisement");
        }
        for address in &self.addresses {
            validate_address(address, allow_private)?;
        }
        let public_key = PublicKey::try_decode_protobuf(&self.public_key)
            .map_err(|e| anyhow!("invalid discovery public key: {e}"))?;
        let peer_id = public_key.to_peer_id();
        if peer_id.to_string() != self.peer_id {
            bail!("discovery PeerID does not match advertised public key");
        }
        if !public_key.verify(&self.signing_bytes()?, &self.signature) {
            bail!("invalid discovery advertisement signature");
        }
        Ok(peer_id)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let encoded = serde_json::to_vec(self)?;
        if encoded.len() > MAX_ADVERTISEMENT_BYTES {
            bail!("discovery advertisement exceeds size bound");
        }
        Ok(encoded)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() > MAX_ADVERTISEMENT_BYTES {
            bail!("discovery advertisement exceeds size bound");
        }
        Ok(serde_json::from_slice(bytes)?)
    }
}

pub fn bounded_addresses<I>(addresses: I, allow_private: bool) -> Vec<String>
where
    I: IntoIterator<Item = Multiaddr>,
{
    addresses
        .into_iter()
        .filter_map(|address| {
            let text = address.to_string();
            validate_address(&text, allow_private).ok()?;
            Some(text)
        })
        .take(MAX_ADDRESSES)
        .collect()
}

pub fn default_capabilities(is_provider: bool) -> Vec<String> {
    let mut capabilities = vec!["mesh".to_string(), "gossipsub-discovery".to_string()];
    if is_provider {
        capabilities.push("bootstrap-rendezvous".to_string());
    }
    capabilities
}

#[cfg(test)]
mod tests {
    use super::*;
    use libp2p::identity::Keypair;

    fn signed_advertisement(keypair: &Keypair) -> WorldDiscoveryAdvertisement {
        let peer_id = PeerId::from(keypair.public());
        WorldDiscoveryAdvertisement::unsigned(
            "ghostwire-one",
            peer_id,
            "did:ghost:test",
            keypair.public().encode_protobuf(),
            vec!["/ip4/8.8.8.8/tcp/4811".to_string()],
            1,
            100,
            200,
            vec!["mesh".to_string()],
        )
        .sign_with(|bytes| keypair.sign(bytes).ok())
        .unwrap()
    }

    #[test]
    fn signed_advertisement_round_trips_and_verifies() {
        let keypair = Keypair::generate_ed25519();
        let ad = signed_advertisement(&keypair);
        let bytes = ad.to_bytes().unwrap();
        let decoded = WorldDiscoveryAdvertisement::from_bytes(&bytes).unwrap();
        assert_eq!(
            decoded.verify("ghostwire-one", 150, false).unwrap(),
            PeerId::from(keypair.public())
        );
    }

    #[test]
    fn tampering_or_scope_change_fails_closed() {
        let keypair = Keypair::generate_ed25519();
        let mut ad = signed_advertisement(&keypair);
        ad.did = "did:ghost:tampered".into();
        assert!(ad.verify("ghostwire-one", 150, false).is_err());
        let ad = signed_advertisement(&keypair);
        assert!(ad.verify("another-network", 150, false).is_err());
    }

    #[test]
    fn private_endpoints_are_blocked_by_default() {
        assert!(validate_address("/ip4/192.168.1.7/tcp/4811", false).is_err());
        assert!(validate_address("/ip4/192.168.1.7/tcp/4811", true).is_ok());
        assert!(validate_address("/ip4/8.8.8.8/tcp/4811", false).is_ok());
        assert!(validate_address("/ip4/127.0.0.1/tcp/4811", true).is_err());
        let _ = Ipv4Addr::UNSPECIFIED;
    }
}
