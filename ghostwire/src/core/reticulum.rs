use crate::core::identity::Identity;
use crate::core::message::Message;
use anyhow::Result;
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::{mpsc, RwLock};
use tracing::{info, warn};

#[cfg(feature = "reticulum")]
use rns_transport::hash::AddressHash;

/// Reticulum's standard local auto-discovery bind address.
///
/// 29716 is the conventional Reticulum UDP/TCP discovery port. Binding it is
/// what makes the default "AutoInterface" actually able to find LAN peers.
const RNS_AUTO_BIND: &str = "0.0.0.0:29716";
#[cfg(feature = "reticulum")]
use rns_transport::identity::PrivateIdentity;
#[cfg(feature = "reticulum")]
use rns_transport::packet::{
    ContextFlag, DestinationType, Header, IfacFlag, Packet, PacketContext, PacketDataBuffer,
    PacketType, PropagationType,
};
#[cfg(feature = "reticulum")]
use rns_transport::transport::{
    AnnounceEvent, ReceivedData, SendPacketOutcome, Transport, TransportConfig,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReticulumNode {
    pub id: String,
    pub public_key: Vec<u8>,
    pub address: String,
    pub last_seen: u64,
    pub connection_quality: f32,
    pub is_online: bool,
    pub onion_routing_capable: bool,
}

// ─────────────────────────────────────────────────────────────────────────────
// Cross-transport discovery: two phases
// ─────────────────────────────────────────────────────────────────────────────
//
// RNS is the cold-start mechanism for libp2p. Two nodes that have never met, on
// networks with no shared subnet and no bootstrap address, can still find each
// other because RNS announces are routed across the backbone. Once connected,
// libp2p carries the real traffic.
//
// The phases are deliberately asymmetric, and that is the whole security design:
//
//   PHASE 1 — public announce. Identity only: who I am, that I am GhostWire,
//     and how to reach me over RNS. NO libp2p ADDRESSES. Announce packets are
//     broadcast to every hop on the backbone, so anything placed here is public.
//     Publishing the libp2p address would hand every RNS hop a direct route to
//     the node, letting an observer bypass RNS entirely and attack the libp2p
//     endpoint directly — which defeats the purpose of discovering over RNS.
//
//   PHASE 2 — authenticated exchange. Only after the peer's phase-1 signature
//     has been verified against its registered key do the libp2p addresses cross,
//     over the RNS channel that is already encrypted. The addresses never touch
//     the public backbone.
//
// The invariant, stated so it can be checked rather than trusted: an
// advertisement handed to the mesh for dialing must never carry addresses that
// arrived via the public announce path. `DiscoveryAnnounce` has no `addresses`
// field at all, so the unsafe shape cannot be expressed in that type.

/// Public, backbone-visible announcement. Identity only, by construction.
///
/// Deliberately NOT `WorldDiscoveryAdvertisement`: that struct carries
/// `addresses`, and using it here would put every node's libp2p endpoints on
/// every RNS hop. See the phase notes above.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiscoveryAnnounce {
    /// Wire-format version, so a future change is rejected rather than misparsed.
    pub version: u16,
    /// Scopes the announcement to one GhostWire network, so a node on another
    /// network cannot inject itself into this one.
    pub network_id: String,
    /// Stable libp2p peer id, base58.
    pub peer_id: String,
    pub did: String,
    /// Ed25519 public key that `signature` is verified against.
    pub public_key: Vec<u8>,
    pub issued_at: u64,
    pub expires_at: u64,
    /// Monotonic counter, so a replayed older announce is distinguishable.
    pub sequence: u64,
    pub signature: Vec<u8>,
}

pub const DISCOVERY_ANNOUNCE_VERSION: u16 = 1;

/// Longest peer id / DID accepted from the network.
const MAX_ID_LEN: usize = 256;
/// Bound on signature length, matching world_discovery's own limit.
const MAX_SIGNATURE_LEN: usize = 128;

/// Liveness window for an announce. Short, because an announce is a routing hint
/// and a stale one costs nothing but a wasted dial.
pub const DISCOVERY_ANNOUNCE_TTL_SECS: u64 = 300;

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

impl DiscoveryAnnounce {
    pub fn new(
        network_id: &str,
        peer_id: &str,
        did: &str,
        public_key: [u8; 32],
        sequence: u64,
    ) -> Self {
        let now = now_unix();
        Self {
            version: DISCOVERY_ANNOUNCE_VERSION,
            network_id: network_id.to_string(),
            peer_id: peer_id.to_string(),
            did: did.to_string(),
            public_key: public_key.to_vec(),
            issued_at: now,
            expires_at: now.saturating_add(DISCOVERY_ANNOUNCE_TTL_SECS),
            sequence,
            signature: Vec::new(),
        }
    }

    /// Canonical bytes to sign: everything except the signature itself.
    pub fn signing_bytes(&self) -> Result<Vec<u8>> {
        let mut unsigned = self.clone();
        unsigned.signature.clear();
        Ok(bincode::serialize(&unsigned)?)
    }

    pub fn sign(mut self, secret: &[u8; 32]) -> Result<Self> {
        let bytes = self.signing_bytes()?;
        use ed25519_dalek::{Signer, SigningKey};
        self.signature = SigningKey::from_bytes(secret)
            .sign(&bytes)
            .to_bytes()
            .to_vec();
        Ok(self)
    }

    /// Verify structure, network scope, expiry, and signature.
    ///
    /// The trusted public key is the caller's out-of-band anchor (the peer's
    /// registered key). The signature is checked against THAT key, and the
    /// announce's own `public_key` field must equal it — otherwise a valid
    /// signature by some other key would authenticate an arbitrary `peer_id`.
    ///
    /// Returns the authenticated peer id. Nothing downstream should act on this
    /// announcement before this returns `Ok`.
    pub fn verify(
        &self,
        expected_network_id: &str,
        now: u64,
        trusted_key: &[u8; 32],
    ) -> Result<String> {
        if self.version != DISCOVERY_ANNOUNCE_VERSION {
            anyhow::bail!("unsupported discovery announce version {}", self.version);
        }
        if self.network_id != expected_network_id {
            anyhow::bail!("discovery announce network scope mismatch");
        }
        if self.peer_id.is_empty() || self.peer_id.len() > MAX_ID_LEN {
            anyhow::bail!("invalid discovery announce peer id");
        }
        if self.did.is_empty() || self.did.len() > MAX_ID_LEN {
            anyhow::bail!("invalid discovery announce did");
        }
        if self.signature.is_empty() || self.signature.len() > MAX_SIGNATURE_LEN {
            anyhow::bail!("invalid discovery announce signature");
        }
        if self.public_key.as_slice() != trusted_key.as_slice() {
            anyhow::bail!("discovery announce key does not match the trusted key");
        }
        if self.expires_at <= self.issued_at || now >= self.expires_at {
            anyhow::bail!("discovery announce expired or has a malformed window");
        }

        use ed25519_dalek::{Signature, Verifier, VerifyingKey};
        let vk = VerifyingKey::from_bytes(trusted_key)
            .map_err(|e| anyhow::anyhow!("bad announce key: {e}"))?;
        let sig_bytes: [u8; 64] = self
            .signature
            .as_slice()
            .try_into()
            .map_err(|_| anyhow::anyhow!("unexpected signature length {}", self.signature.len()))?;
        vk.verify(&self.signing_bytes()?, &Signature::from_bytes(&sig_bytes))
            .map_err(|_| anyhow::anyhow!("discovery announce signature invalid"))?;

        Ok(self.peer_id.clone())
    }
}

/// Phase 2: the authenticated address exchange.
///
/// Sent over the RNS channel only after phase 1 verified the peer. This is the
/// ONLY type by which a libp2p address may travel between nodes, which is what
/// makes the two-phase boundary enforceable rather than a convention.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AddressExchange {
    pub version: u16,
    pub network_id: String,
    /// Peer id this exchange is FROM, so the receiver can check both directions.
    pub peer_id: String,
    pub did: String,
    /// libp2p multiaddrs, e.g. /ip4/203.0.113.5/tcp/4001.
    pub addresses: Vec<String>,
    pub issued_at: u64,
    pub expires_at: u64,
    pub sequence: u64,
    pub signature: Vec<u8>,
}

pub const ADDRESS_EXCHANGE_VERSION: u16 = 1;
/// Bound on addresses accepted from the exchange.
pub const MAX_EXCHANGE_ADDRESSES: usize = 16;
/// Exchange entries are used to dial promptly, so they expire sooner.
pub const ADDRESS_EXCHANGE_TTL_SECS: u64 = 120;

impl AddressExchange {
    pub fn new(
        network_id: &str,
        peer_id: &str,
        did: &str,
        addresses: Vec<String>,
        sequence: u64,
    ) -> Self {
        let now = now_unix();
        Self {
            version: ADDRESS_EXCHANGE_VERSION,
            network_id: network_id.to_string(),
            peer_id: peer_id.to_string(),
            did: did.to_string(),
            addresses,
            issued_at: now,
            expires_at: now.saturating_add(ADDRESS_EXCHANGE_TTL_SECS),
            sequence,
            signature: Vec::new(),
        }
    }

    pub fn signing_bytes(&self) -> Result<Vec<u8>> {
        let mut unsigned = self.clone();
        unsigned.signature.clear();
        Ok(bincode::serialize(&unsigned)?)
    }

    pub fn sign(mut self, secret: &[u8; 32]) -> Result<Self> {
        let bytes = self.signing_bytes()?;
        use ed25519_dalek::{Signer, SigningKey};
        self.signature = SigningKey::from_bytes(secret)
            .sign(&bytes)
            .to_bytes()
            .to_vec();
        Ok(self)
    }

    /// Verify, then return ONLY the addresses that are safe to dial.
    ///
    /// Each address is validated via `world_discovery::validate_address` with
    /// `allow_private: false`, so a peer cannot make this node dial loopback,
    /// multicast, unspecified, or an RFC1918 address it does not control.
    ///
    /// Returns `(peer_id, addresses)`. The caller dials exactly these and
    /// nothing else — this is the single point where an address becomes
    /// dialable, which is what makes the phase-1/phase-2 boundary enforceable.
    pub fn verify(
        &self,
        expected_network_id: &str,
        now: u64,
        trusted_key: &[u8; 32],
    ) -> Result<(String, Vec<String>)> {
        if self.version != ADDRESS_EXCHANGE_VERSION {
            anyhow::bail!("unsupported address exchange version {}", self.version);
        }
        if self.network_id != expected_network_id {
            anyhow::bail!("address exchange network scope mismatch");
        }
        if self.peer_id.is_empty() || self.peer_id.len() > MAX_ID_LEN {
            anyhow::bail!("invalid address exchange peer id");
        }
        if self.did.is_empty() || self.did.len() > MAX_ID_LEN {
            anyhow::bail!("invalid address exchange did");
        }
        if self.signature.is_empty() || self.signature.len() > MAX_SIGNATURE_LEN {
            anyhow::bail!("invalid address exchange signature");
        }
        if self.addresses.is_empty() || self.addresses.len() > MAX_EXCHANGE_ADDRESSES {
            anyhow::bail!("invalid address exchange address count");
        }
        if self.expires_at <= self.issued_at || now >= self.expires_at {
            anyhow::bail!("address exchange expired or has a malformed window");
        }

        use ed25519_dalek::{Signature, Verifier, VerifyingKey};
        let vk = VerifyingKey::from_bytes(trusted_key)
            .map_err(|e| anyhow::anyhow!("bad exchange key: {e}"))?;
        let sig_bytes: [u8; 64] = self
            .signature
            .as_slice()
            .try_into()
            .map_err(|_| anyhow::anyhow!("unexpected signature length"))?;
        vk.verify(&self.signing_bytes()?, &Signature::from_bytes(&sig_bytes))
            .map_err(|_| anyhow::anyhow!("address exchange signature invalid"))?;

        let accepted: Vec<String> = self
            .addresses
            .iter()
            .filter(|a| crate::core::world_discovery::validate_address(a, false).is_ok())
            .cloned()
            .collect();
        if accepted.is_empty() {
            anyhow::bail!("address exchange carried no dialable public address");
        }
        Ok((self.peer_id.clone(), accepted))
    }
}
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReticulumTopology {
    pub nodes: HashMap<String, ReticulumNode>,
    pub routes: HashMap<String, Vec<String>>,
    pub local_node_id: String,
    pub onion_routes: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReticulumMessage {
    pub id: String,
    pub sender: String,
    pub recipient: String,
    pub content: Vec<u8>,
    pub onion_layers: Vec<Vec<u8>>,
    pub timestamp: u64,
    pub ttl: u32,
    pub hop_count: u32,
    pub max_hops: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReticulumStats {
    pub total_nodes: usize,
    pub online_nodes: usize,
    pub local_node_id: String,
    pub routes_count: usize,
    pub onion_routes_count: usize,
    pub messages_relayed: u64,
    pub encryption_errors: u64,
    pub initialized: bool,
    pub degraded: bool,
    pub interfaces_configured: usize,
    pub messages_sent: u64,
    pub messages_received: u64,
    pub send_failures: u64,
    pub last_error: Option<String>,
}

const RNS_ENVELOPE_VERSION: u8 = 1;
const RNS_ENVELOPE_DOMAIN: &str = "ghostwire.rns.message";

#[derive(Debug, Serialize, Deserialize)]
struct ReticulumEnvelope {
    version: u8,
    domain: String,
    message: Message,
}

impl ReticulumEnvelope {
    fn from_message(message: &Message) -> Self {
        Self {
            version: RNS_ENVELOPE_VERSION,
            domain: RNS_ENVELOPE_DOMAIN.to_string(),
            message: message.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// Real Reticulum transport (feature = "reticulum")
// ---------------------------------------------------------------------------
#[cfg(feature = "reticulum")]
pub struct ReticulumTransport {
    transport: Option<Transport>,
    local_address: String,
    local_address_hash: AddressHash,
    rx: mpsc::Receiver<Message>,
    _rx_task: tokio::task::JoinHandle<()>,
    interfaces_configured: usize,
    degraded: bool,
    messages_sent: u64,
    messages_received: u64,
    send_failures: u64,
    last_error: Option<String>,
}

#[cfg(feature = "reticulum")]
impl ReticulumTransport {
    /// Create a new Reticulum transport.
    ///
    /// If `master_seed` is provided, the Reticulum identity is deterministically
    /// derived from it (same master seed → same Reticulum address every time).
    /// Otherwise falls back to a random identity.
    pub async fn new(
        interfaces_ini: &str,
        node_name: &str,
        master_seed: Option<[u8; 32]>,
    ) -> Result<Self> {
        let identity = match master_seed {
            Some(seed) => {
                let sign_key = ed25519_dalek::SigningKey::from_bytes(&seed);
                let static_secret = x25519_dalek::StaticSecret::from(seed);
                PrivateIdentity::new(static_secret, sign_key)
            }
            None => {
                // ponytail: persist RNS identity to disk so it survives restarts.
                // Without this, every restart generates a new random RNS address.
                let identity_path = dirs::config_dir()
                    .unwrap_or_default()
                    .join("ghostwire")
                    .join("rns_identity.bin");

                match std::fs::read(&identity_path) {
                    Ok(bytes) if bytes.len() == rns_transport::identity::PRIVATE_KEY_LENGTH => {
                        match PrivateIdentity::from_private_key_bytes(&bytes) {
                            Ok(id) => {
                                info!(
                                    "Loaded persisted RNS identity from {:?} (address: {})",
                                    identity_path,
                                    id.address_hash().to_hex_string()
                                );
                                id
                            }
                            Err(e) => {
                                warn!(
                                    "Failed to deserialize RNS identity: {}, generating random",
                                    e
                                );
                                let id = PrivateIdentity::new_from_rand(OsRng);
                                let _ = std::fs::create_dir_all(identity_path.parent().unwrap());
                                let _ = std::fs::write(&identity_path, id.to_private_key_bytes());
                                id
                            }
                        }
                    }
                    _ => {
                        let id = PrivateIdentity::new_from_rand(OsRng);
                        info!(
                            "Generated new RNS identity (address: {}), persisting to {:?}",
                            id.address_hash().to_hex_string(),
                            identity_path
                        );
                        if let Some(parent) = identity_path.parent() {
                            let _ = std::fs::create_dir_all(parent);
                        }
                        let _ = std::fs::write(&identity_path, id.to_private_key_bytes());
                        id
                    }
                }
            }
        };
        let local_address = identity.address_hash().to_hex_string();
        let local_address_hash = *identity.address_hash();

        let mut config = TransportConfig::new(node_name, &identity, true);
        config.set_transport_enabled(true);
        config.set_broadcast(true);
        config.set_link_idle_timeout_secs(900);

        // Validate the operator's interface config up front so a malformed file
        // fails loudly here rather than silently yielding an RNS transport that
        // can reach nobody.
        //
        // This call's RESULT used to be assigned to `_rns_config` and dropped.
        // The parsed `interfaces` vec was never read, so "AutoInterface" -- the
        // one interface the default config names -- was never constructed
        // anywhere in src/. Kept for validation only; the interface list is
        // honoured explicitly in the spawn loop below.
        rns_transport::config::Config::from_ini(interfaces_ini)?;

        // Wrap transport creation and announce in a timeout. The default
        // GhostWire configuration uses local AutoInterface only; any WAN
        // interface is explicit operator configuration and must not block the
        // rest of the node when unreachable.
        let init_timeout = std::time::Duration::from_secs(15);

        // Count interfaces we ACTUALLY spawn, not lines in a config string.
        //
        // This used to be:
        //     interfaces_ini.lines().filter(|l| l.starts_with("interface ")).count()
        // which counted the text "interface 0 = AutoInterface" in the DEFAULT
        // config and reported 1. But the only spawn loop below reads `target `
        // lines, and the default config has none -- the parsed `Config` was even
        // discarded as `let _rns_config`. So the default node spawned ZERO
        // interfaces while reporting 1.
        //
        // That number is load-bearing and dishonest: mesh_health.rs computes
        //     rns_ready = initialized && !degraded && interfaces_configured > 0
        // and reports "ready" -- the best possible state -- for a transport
        // with no interfaces, and mod.rs feeds it into bootstrap readiness. A
        // config string was being counted as a bound socket.
        //
        // Now it is incremented at the point of a real spawn, so it is
        // arithmetically incapable of over-reporting.
        let mut interfaces_configured: usize = 0;
        let mut degraded = false;
        let (transport, mut rx_data) = match tokio::time::timeout(init_timeout, async {
            let t = Transport::new(config);
            {
                let iface_mgr = t.iface_manager();
                let mut mgr = iface_mgr.lock().await;
                for line in interfaces_ini.lines() {
                    let line = line.trim();

                    // "interface N = AutoInterface" is what the DEFAULT config
                    // ships (config.rs default_reticulum_interfaces). It was
                    // parsed and then thrown away as `let _rns_config`, so the
                    // one interface the default configuration names was never
                    // constructed anywhere in src/. The only loop that spawned
                    // anything read `target ` lines, which the default config
                    // does not have -- so a default node created a Transport
                    // with an identity, a destination and an announce, and zero
                    // network interfaces.
                    //
                    // Map it to the crate's multicast UDP interface, which is
                    // how Reticulum does local peer discovery. Without this the
                    // "AutoInterface" in the default config is a fiction.
                    let is_auto_interface = line
                        .strip_prefix("interface ")
                        .and_then(|rest| rest.split_once('=').map(|(_, v)| v.trim()))
                        .map(|value| {
                            value.eq_ignore_ascii_case("autointerface")
                                || value.eq_ignore_ascii_case("auto_interface")
                        })
                        .unwrap_or(false);
                    if is_auto_interface {
                        info!("Spawning AutoInterface (multicast UDP {} )", RNS_AUTO_BIND);
                        let (_hash, _routing, _status) =
                            rns_transport::iface::udp::spawn_multicast_udp_with_status(
                                &mut mgr,
                                RNS_AUTO_BIND.to_string(),
                                None,
                            );
                        interfaces_configured += 1;
                        continue;
                    }

                    if let Some(addr) = line
                        .strip_prefix("target ")
                        .and_then(|s| s.split_once('=').map(|(_, v)| v.trim()))
                    {
                        info!("Registering TCP client interface to {}", addr);
                        let tcp_client = rns_transport::iface::tcp_client::TcpClient::new(addr);
                        mgr.spawn(
                            tcp_client,
                            rns_transport::iface::tcp_client::TcpClient::spawn,
                        );
                        interfaces_configured += 1;
                    }
                }
                if interfaces_configured == 0 {
                    warn!(
                        "Reticulum configured zero interfaces: the node has an RNS identity \
                         and destination but cannot reach any RNS peer. Configure an \
                         'interface N = AutoInterface' or 'target N = host:port' entry, or \
                         set network.reticulum_enabled = false so health does not imply \
                         readiness."
                    );
                }
            }

            let rx_data = t.received_data_events();

            let _local_dest = t
                .add_destination(
                    identity.clone(),
                    rns_transport::destination::DestinationName::new(node_name, "mesh"),
                )
                .await;

            info!(
                "RNS destination registered as \"{}\" (address: {})",
                node_name, local_address
            );
            t.send_announce(&_local_dest, None).await;

            (t, rx_data)
        })
        .await
        {
            Ok(result) => result,
            Err(_elapsed) => {
                warn!(
                    "Reticulum transport init timed out after {}s — node will run in degraded RNS mode",
                    init_timeout.as_secs()
                );
                degraded = true;
                // Create a minimal transport without the unavailable configured backbone.
                let t = Transport::new(TransportConfig::new(node_name, &identity, true));
                let rx_data = t.received_data_events();
                let _local_dest = t
                    .add_destination(
                        identity.clone(),
                        rns_transport::destination::DestinationName::new("ghostwire", "mesh"),
                    )
                    .await;
                (t, rx_data)
            }
        };
        let (tx, rx) = mpsc::channel::<Message>(256);
        let rx_task = tokio::spawn(async move {
            loop {
                match rx_data.recv().await {
                    Ok(received) => {
                        if let Some(msg) = decode_received_data(&received) {
                            if tx.send(msg).await.is_err() {
                                break;
                            }
                        }
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                        warn!("Reticulum receive channel lagged, dropped {} messages", n);
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                }
            }
        });

        info!(
            "Reticulum transport created for {} (address: {})",
            node_name, local_address
        );

        Ok(Self {
            transport: Some(transport),
            local_address,
            local_address_hash,
            rx,
            _rx_task: rx_task,
            interfaces_configured,
            degraded,
            messages_sent: 0,
            messages_received: 0,
            send_failures: 0,
            last_error: None,
        })
    }

    pub async fn send_message(&mut self, message: &Message) -> Result<()> {
        let payload = serde_json::to_vec(&ReticulumEnvelope::from_message(message))?;

        let recipient_hash = parse_address_hash(&message.recipient).ok_or_else(|| {
            anyhow::anyhow!(
                "Invalid Reticulum address hash in recipient field: {}",
                message.recipient
            )
        })?;

        let packet = Packet {
            header: Header {
                ifac_flag: IfacFlag::Open,
                header_type: rns_transport::packet::HeaderType::Type1,
                context_flag: ContextFlag::Unset,
                propagation_type: PropagationType::Transport,
                destination_type: DestinationType::Single,
                packet_type: PacketType::Data,
                hops: 0,
            },
            ifac: None,
            destination: recipient_hash,
            transport: None,
            context: PacketContext::None,
            data: PacketDataBuffer::new_from_slice(&payload),
        };

        let transport = self
            .transport
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Reticulum transport is stopped"))?;
        let trace = transport.send_packet_with_trace(packet).await;
        match trace.outcome {
            SendPacketOutcome::SentDirect | SendPacketOutcome::SentBroadcast => {
                self.messages_sent = self.messages_sent.saturating_add(1);
                info!("Queued message {} via Reticulum", message.id);
                Ok(())
            }
            outcome => {
                self.send_failures = self.send_failures.saturating_add(1);
                let detail = format!("Reticulum send outcome: {outcome:?}");
                self.last_error = Some(detail.clone());
                Err(anyhow::anyhow!(detail))
            }
        }
    }

    pub async fn send_broadcast(&mut self, message: &Message) -> Result<()> {
        let payload = serde_json::to_vec(&ReticulumEnvelope::from_message(message))?;

        let packet = Packet {
            header: Header {
                ifac_flag: IfacFlag::Open,
                header_type: rns_transport::packet::HeaderType::Type1,
                context_flag: ContextFlag::Unset,
                propagation_type: PropagationType::Broadcast,
                destination_type: DestinationType::Single,
                packet_type: PacketType::Data,
                hops: 0,
            },
            ifac: None,
            destination: AddressHash::new_empty(),
            transport: None,
            context: PacketContext::None,
            data: PacketDataBuffer::new_from_slice(&payload),
        };

        let transport = self
            .transport
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Reticulum transport is stopped"))?;
        let trace = transport.send_packet_broadcast_with_trace(packet).await;
        match trace.outcome {
            SendPacketOutcome::SentBroadcast | SendPacketOutcome::SentDirect => {
                self.messages_sent = self.messages_sent.saturating_add(1);
                info!("Queued broadcast message {} via Reticulum", message.id);
                Ok(())
            }
            outcome => {
                self.send_failures = self.send_failures.saturating_add(1);
                let detail = format!("Reticulum broadcast outcome: {outcome:?}");
                self.last_error = Some(detail.clone());
                Err(anyhow::anyhow!(detail))
            }
        }
    }

    pub async fn receive_message(&mut self) -> Result<Option<Message>> {
        match self.rx.try_recv() {
            Ok(msg) => {
                self.messages_received = self.messages_received.saturating_add(1);
                Ok(Some(msg))
            }
            Err(mpsc::error::TryRecvError::Empty) => Ok(None),
            Err(mpsc::error::TryRecvError::Disconnected) => {
                anyhow::bail!("Reticulum receive channel closed")
            }
        }
    }

    pub fn local_address(&self) -> String {
        self.local_address.clone()
    }

    pub fn local_address_hash(&self) -> AddressHash {
        self.local_address_hash
    }

    pub async fn connect_tcp(&mut self, address: &str) -> Result<()> {
        let address = address.trim();
        if address.is_empty() || address.len() > 256 {
            anyhow::bail!("Invalid Reticulum TCP endpoint");
        }
        let port = address
            .rsplit_once(':')
            .and_then(|(_, port)| port.parse::<u16>().ok())
            .filter(|port| *port != 0)
            .ok_or_else(|| anyhow::anyhow!("Reticulum endpoint must be host:port"))?;
        let transport = self
            .transport
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Reticulum transport is stopped"))?;
        let iface_mgr = transport.iface_manager();
        let mut mgr = iface_mgr.lock().await;
        let tcp_client = rns_transport::iface::tcp_client::TcpClient::new(address);
        mgr.spawn(
            tcp_client,
            rns_transport::iface::tcp_client::TcpClient::spawn,
        );
        self.interfaces_configured = self.interfaces_configured.saturating_add(1);
        info!("Registered Reticulum TCP client interface at {address} (port {port})");
        Ok(())
    }

    pub async fn subscribe_announces(
        &self,
    ) -> Result<tokio::sync::broadcast::Receiver<AnnounceEvent>> {
        let transport = self
            .transport
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Reticulum transport is stopped"))?;
        Ok(transport.recv_announces().await)
    }

    pub async fn path_status(&self, address: &str) -> Result<(bool, Option<String>, Option<u8>)> {
        let hash = parse_address_hash(address)
            .ok_or_else(|| anyhow::anyhow!("Invalid Reticulum address hash"))?;
        let transport = self
            .transport
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Reticulum transport is stopped"))?;
        let status = transport.path_status(&hash).await;
        Ok((
            status.path_found,
            status.next_hop.map(|hop| hop.to_hex_string()),
            status.hops,
        ))
    }

    pub async fn stop(&mut self) -> Result<()> {
        self._rx_task.abort();
        self.transport.take();
        info!("Reticulum transport stopped and interface tasks released");
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Stub transport (feature not enabled)
// ---------------------------------------------------------------------------
#[cfg(not(feature = "reticulum"))]
pub struct ReticulumTransport;

#[cfg(not(feature = "reticulum"))]
impl ReticulumTransport {
    pub async fn new(
        _interfaces_ini: &str,
        _node_name: &str,
        _master_seed: Option<[u8; 32]>,
    ) -> Result<Self> {
        Err(anyhow::anyhow!(
            "Reticulum feature not enabled — compile with --features reticulum"
        ))
    }

    pub async fn send_message(&mut self, _message: &Message) -> Result<()> {
        Err(anyhow::anyhow!("Reticulum feature not enabled"))
    }

    pub async fn send_broadcast(&mut self, _message: &Message) -> Result<()> {
        Err(anyhow::anyhow!("Reticulum feature not enabled"))
    }

    pub async fn receive_message(&mut self) -> Result<Option<Message>> {
        Err(anyhow::anyhow!("Reticulum feature not enabled"))
    }

    pub fn local_address(&self) -> String {
        String::new()
    }

    pub async fn stop(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(not(feature = "reticulum"))]
pub fn compute_reticulum_address(_master_seed: &[u8; 32]) -> String {
    String::new()
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Decode a `ReceivedData` event into a GhostWire `Message`.
/// The transport delivers the raw payload bytes; we JSON-deserialize them.
#[cfg(feature = "reticulum")]
fn decode_received_data(received: &ReceivedData) -> Option<Message> {
    let bytes = received.data.as_slice();
    if bytes.is_empty() || bytes.len() > 16 * 1024 {
        warn!(
            "Rejected Reticulum payload outside GhostWire bounds: {} bytes",
            bytes.len()
        );
        return None;
    }
    match serde_json::from_slice::<ReticulumEnvelope>(bytes) {
        Ok(envelope)
            if envelope.version == RNS_ENVELOPE_VERSION
                && envelope.domain == RNS_ENVELOPE_DOMAIN =>
        {
            Some(envelope.message)
        }
        Ok(_) => {
            warn!("Rejected Reticulum payload with unsupported GhostWire envelope");
            None
        }
        Err(e) => {
            warn!(
                "Rejected Reticulum payload that is not a GhostWire envelope: {}",
                e
            );
            None
        }
    }
}

/// Parse a hex-encoded Reticulum address hash string into `AddressHash`.
#[cfg(feature = "reticulum")]
fn parse_address_hash(hex_str: &str) -> Option<AddressHash> {
    let hex = hex_str.trim().trim_start_matches("0x");
    let bytes = hex::decode(hex).ok()?;
    Some(AddressHash::new_from_slice(&bytes))
}

/// Whether a recipient string is a Reticulum destination address.
///
/// STRICT on purpose, and deliberately narrower than `parse_address_hash`.
/// That helper accepts ANY hex of ANY length, so it happily "parses" a
/// libp2p PeerId or any other hex-looking id into a truncated 16-byte
/// `AddressHash::new_from_slice`. If this were used to decide which transport
/// a recipient belongs to, a mistyped or non-RNS id would be silently sent to
/// RNS and encrypted to a destination nobody holds the key for.
///
/// A real Reticulum address hash is exactly 16 bytes, i.e. 32 hex characters
/// (ADDRESS_HASH_SIZE in reticulum-rs-transport). PeerIds are base58 and start
/// with `12D3KooW`, so they cannot collide with this.
pub fn is_reticulum_address(recipient: &str) -> bool {
    let hex = recipient.trim().trim_start_matches("0x");
    hex.len() == rns_transport::hash::ADDRESS_HASH_SIZE * 2
        && hex.chars().all(|c| c.is_ascii_hexdigit())
        // A bare 32-hex-digit string is ambiguous in principle; require it not
        // to look like the hex form of anything else we route.
        && !hex.starts_with("12D3Koo")
}

// ---------------------------------------------------------------------------
// Manager — orchestrates transport lifecycle
// ---------------------------------------------------------------------------
pub struct ReticulumManager {
    _identity: Arc<Identity>,
    topology: Arc<RwLock<ReticulumTopology>>,
    stats: Arc<RwLock<ReticulumStats>>,
    transport: Arc<RwLock<ReticulumTransport>>,
    topology_task: Option<tokio::task::JoinHandle<()>>,
}

impl ReticulumManager {
    pub async fn new(
        identity: Arc<Identity>,
        interfaces_ini: &str,
        destination_name: &str,
    ) -> Result<Self> {
        let node_id = destination_name.to_string();
        let transport_seed = identity.derive_transport_key("reticulum");
        let transport =
            ReticulumTransport::new(interfaces_ini, destination_name, Some(transport_seed)).await?;

        let topology = Arc::new(RwLock::new(ReticulumTopology {
            nodes: HashMap::new(),
            routes: HashMap::new(),
            local_node_id: node_id.clone(),
            onion_routes: HashMap::new(),
        }));

        let stats = Arc::new(RwLock::new(ReticulumStats {
            total_nodes: 0,
            online_nodes: 0,
            local_node_id: node_id,
            routes_count: 0,
            onion_routes_count: 0,
            messages_relayed: 0,
            encryption_errors: 0,
            initialized: true,
            degraded: false,
            interfaces_configured: 0,
            messages_sent: 0,
            messages_received: 0,
            send_failures: 0,
            last_error: None,
        }));

        let topology_task = {
            #[cfg(feature = "reticulum")]
            {
                let mut announce_rx = transport.subscribe_announces().await?;
                let topology_for_task = topology.clone();
                let local_address = transport.local_address();
                Some(tokio::spawn(async move {
                    loop {
                        match announce_rx.recv().await {
                            Ok(event) => {
                                let destination = event.destination.lock().await;
                                let address = destination.desc.address_hash.to_hex_string();
                                if address == local_address {
                                    continue;
                                }
                                let now = SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .unwrap_or_default()
                                    .as_secs();
                                let node = ReticulumNode {
                                    id: address.clone(),
                                    public_key: destination.identity.public_key_bytes().to_vec(),
                                    address: address.clone(),
                                    last_seen: now,
                                    connection_quality: 1.0 / (1.0 + f32::from(event.hops)),
                                    is_online: true,
                                    onion_routing_capable: false,
                                };
                                let mut topology = topology_for_task.write().await;
                                topology.nodes.insert(address.clone(), node);
                                topology.routes.insert(
                                    address,
                                    vec![format!("rns:{}", hex::encode(event.interface))],
                                );
                            }
                            Err(tokio::sync::broadcast::error::RecvError::Lagged(count)) => {
                                tracing::warn!(
                                    "RNS announce topology consumer lagged by {count} events"
                                );
                            }
                            Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                        }
                    }
                }))
            }
            #[cfg(not(feature = "reticulum"))]
            {
                None
            }
        };

        Ok(Self {
            _identity: identity,
            topology,
            stats,
            transport: Arc::new(RwLock::new(transport)),
            topology_task,
        })
    }

    pub async fn send_message(&self, message: &Message) -> Result<()> {
        let mut transport = self.transport.write().await;
        transport.send_message(message).await?;

        let mut stats = self.stats.write().await;
        stats.messages_relayed += 1;
        Ok(())
    }

    pub async fn send_broadcast(&self, message: &Message) -> Result<()> {
        let mut transport = self.transport.write().await;
        transport.send_broadcast(message).await?;

        let mut stats = self.stats.write().await;
        stats.messages_relayed += 1;
        Ok(())
    }

    /// Whether the RNS transport is currently usable.
    ///
    /// A recipient that is a Reticulum address can only be delivered if this is
    /// true, so the send path checks it rather than assuming a registered
    /// transport is running.
    pub async fn is_alive(&self) -> bool {
        self.transport.read().await.transport.is_some()
    }

    pub async fn receive_message(&self) -> Result<Option<Message>> {
        let mut transport = self.transport.write().await;
        transport.receive_message().await
    }

    pub async fn connect(&mut self, address: &str) -> Result<()> {
        let mut transport = self.transport.write().await;
        transport.connect_tcp(address).await
    }

    async fn prune_stale_topology(&self) {
        const TOPOLOGY_TTL_SECS: u64 = 900;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let mut topology = self.topology.write().await;
        let local_id = topology.local_node_id.clone();
        topology.nodes.retain(|id, node| {
            now.saturating_sub(node.last_seen) <= TOPOLOGY_TTL_SECS || id == &local_id
        });
        let valid_node_ids: std::collections::HashSet<String> =
            topology.nodes.keys().cloned().collect();
        topology.routes.retain(|id, _| valid_node_ids.contains(id));
        topology
            .onion_routes
            .retain(|id, _| valid_node_ids.contains(id));
    }

    pub async fn get_stats(&self) -> ReticulumStats {
        self.prune_stale_topology().await;
        let mut stats = self.stats.read().await.clone();
        let topology = self.topology.read().await;
        let transport = self.transport.read().await;
        stats.total_nodes = topology.nodes.len();
        stats.online_nodes = topology.nodes.values().filter(|n| n.is_online).count();
        stats.routes_count = topology.routes.len();
        stats.onion_routes_count = topology.onion_routes.len();
        stats.initialized = transport.transport.is_some();
        stats.degraded = transport.degraded;
        stats.interfaces_configured = transport.interfaces_configured;
        stats.messages_sent = transport.messages_sent;
        stats.messages_received = transport.messages_received;
        stats.send_failures = transport.send_failures;
        stats.last_error = transport.last_error.clone();
        stats
    }

    pub async fn get_topology(&self) -> ReticulumTopology {
        self.prune_stale_topology().await;
        self.topology.read().await.clone()
    }

    pub async fn local_address(&self) -> String {
        let transport = self.transport.read().await;
        transport.local_address()
    }

    pub fn transport_arc(&self) -> Arc<RwLock<ReticulumTransport>> {
        self.transport.clone()
    }

    pub async fn add_node(&mut self, node: ReticulumNode) -> Result<()> {
        let mut topology = self.topology.write().await;
        topology.nodes.insert(node.id.clone(), node);
        Ok(())
    }

    pub async fn update_node_status(
        &mut self,
        node_id: &str,
        is_online: bool,
        quality: f32,
    ) -> Result<()> {
        let mut topology = self.topology.write().await;
        if let Some(node) = topology.nodes.get_mut(node_id) {
            node.is_online = is_online;
            node.connection_quality = quality.clamp(0.0, 1.0);
            node.last_seen = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
        }
        Ok(())
    }

    pub async fn stop(&mut self) -> Result<()> {
        if let Some(task) = self.topology_task.take() {
            task.abort();
        }
        let mut transport = self.transport.write().await;
        transport.stop().await?;
        Ok(())
    }
}

/// Compute a Reticulum address hex string from a master seed without starting a transport.
/// Used by the mesh handshake to include our reticulum address in peer exchange.
#[cfg(feature = "reticulum")]
pub fn compute_reticulum_address(master_seed: &[u8; 32]) -> String {
    let sign_key = ed25519_dalek::SigningKey::from_bytes(master_seed);
    let static_secret = x25519_dalek::StaticSecret::from(*master_seed);
    let identity = PrivateIdentity::new(static_secret, sign_key);
    identity.address_hash().to_hex_string()
}

#[cfg(test)]
mod discovery_tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};

    /// A deterministic keypair for "the peer we trust".
    fn trusted() -> (SigningKey, [u8; 32]) {
        let sk = SigningKey::from_bytes(&[7u8; 32]);
        let pk = sk.verifying_key().to_bytes();
        (sk, pk)
    }

    /// A DIFFERENT keypair, standing in for an attacker. Must not collide with
    /// `trusted()`, or the key-mismatch tests would pass for the wrong reason —
    /// the same trap opencode hit with a "locked node" test that was green with
    /// the fix reverted.
    fn attacker() -> (SigningKey, [u8; 32]) {
        let sk = SigningKey::from_bytes(&[99u8; 32]);
        let pk = sk.verifying_key().to_bytes();
        (sk, pk)
    }

    // PHASE 1 — the public announce must not be able to carry an address at all.
    // This is the invariant Michael identified: an address in a public announce
    // is broadcast to every RNS hop, letting an observer bypass RNS and dial the
    // libp2p endpoint directly.

    #[test]
    fn discovery_announce_has_no_address_field() {
        // Compile-time proof, not a runtime check: if someone adds `addresses`
        // to DiscoveryAnnounce, this destructuring stops compiling.
        let (sk, pk) = trusted();
        let a = DiscoveryAnnounce::new("net1", "12D3KooWabc", "did:ghost:x", pk, 1).sign(&sk.to_bytes()).unwrap();
        let DiscoveryAnnounce {
            version,
            network_id,
            peer_id,
            did,
            public_key,
            issued_at,
            expires_at,
            sequence,
            signature,
        } = a.clone();
        assert_eq!(version, DISCOVERY_ANNOUNCE_VERSION);
        assert_eq!(network_id, "net1");
        assert_eq!(peer_id, "12D3KooWabc");
        assert_eq!(did, "did:ghost:x");
        assert_eq!(public_key, pk.to_vec());
        assert!(issued_at > 0);
        assert!(expires_at > issued_at);
        assert_eq!(sequence, 1);
        assert!(!signature.is_empty());
    }

    #[test]
    fn signed_announce_verifies_against_its_key() {
        let (sk, pk) = trusted();
        let a = DiscoveryAnnounce::new("net1", "12D3KooWabc", "did:ghost:x", pk, 1)
            .sign(&sk.to_bytes())
            .unwrap();
        let now = now_unix();
        assert_eq!(a.verify("net1", now, &pk).unwrap(), "12D3KooWabc");
    }

    #[test]
    fn announce_from_another_network_is_refused() {
        let (sk, pk) = trusted();
        let a = DiscoveryAnnounce::new("attacker-net", "12D3KooWabc", "did:ghost:x", pk, 1)
            .sign(&sk.to_bytes())
            .unwrap();
        // This is what stops a node on another network injecting itself here.
        assert!(a.verify("our-net", now_unix(), &pk).is_err());
    }

    #[test]
    fn announce_signed_by_a_different_key_is_refused() {
        let (_, pk) = trusted();
        let (attacker_sk, attacker_pk) = attacker();
        let a = DiscoveryAnnounce::new("net1", "12D3KooWevil", "did:ghost:evil", attacker_pk, 1)
            .sign(&attacker_sk.to_bytes())
            .unwrap();
        // Trusted key is pk, but the announce claims attacker_pk and is signed by
        // the attacker. Must be refused.
        assert!(a.verify("net1", now_unix(), &pk).is_err());
    }

    #[test]
    fn expired_announce_is_refused() {
        let (sk, pk) = trusted();
        let a = DiscoveryAnnounce::new("net1", "12D3KooWabc", "did:ghost:x", pk, 1)
            .sign(&sk.to_bytes())
            .unwrap();
        assert!(a.verify("net1", a.expires_at + 1, &pk).is_err());
    }

    #[test]
    fn tampered_announce_is_refused() {
        let (sk, pk) = trusted();
        let a = DiscoveryAnnounce::new("net1", "12D3KooWabc", "did:ghost:x", pk, 1)
            .sign(&sk.to_bytes())
            .unwrap();
        // Attacker swaps the peer id after signing.
        let mut bad = a.clone();
        bad.peer_id = "12D3KooWevil".to_string();
        assert!(bad.verify("net1", now_unix(), &pk).is_err());
    }

    // PHASE 2 — the only path by which an address becomes dialable.

    #[test]
    fn address_exchange_yields_validated_addresses() {
        let (sk, pk) = trusted();
        let x = AddressExchange::new(
            "net1",
            "12D3KooWabc",
            "did:ghost:x",
            vec!["/ip4/203.0.113.5/tcp/4001".to_string()],
            1,
        )
        .sign(&sk.to_bytes())
        .unwrap();
        let (peer, addrs) = x.verify("net1", now_unix(), &pk).unwrap();
        assert_eq!(peer, "12D3KooWabc");
        assert_eq!(addrs, vec!["/ip4/203.0.113.5/tcp/4001".to_string()]);
    }

    #[test]
    fn exchange_cannot_make_us_dial_loopback_or_private_addresses() {
        let (sk, pk) = trusted();
        // allow_private: false, so these must all be dropped. If any survive, a
        // peer could point us at our own host or an internal network.
        let hostile = vec![
            "/ip4/127.0.0.1/tcp/4001".to_string(),
            "/ip4/10.0.0.5/tcp/4001".to_string(),
            "/ip4/192.168.1.9/tcp/4001".to_string(),
            "/ip4/172.16.4.4/tcp/4001".to_string(),
            "/ip4/0.0.0.0/tcp/4001".to_string(),
            "/ip4/224.0.0.1/tcp/4001".to_string(),
        ];
        let x = AddressExchange::new("net1", "12D3KooWabc", "did:ghost:x", hostile, 1)
            .sign(&sk.to_bytes())
            .unwrap();
        // Nothing dialable remains, so verify must fail rather than return empty.
        assert!(x.verify("net1", now_unix(), &pk).is_err());
    }

    #[test]
    fn exchange_with_only_hostile_addresses_is_refused_even_with_a_public_one() {
        let (sk, pk) = trusted();
        let mixed = vec![
            "/ip4/203.0.113.5/tcp/4001".to_string(),
            "/ip4/127.0.0.1/tcp/4001".to_string(),
        ];
        let x = AddressExchange::new("net1", "12D3KooWabc", "did:ghost:x", mixed, 1)
            .sign(&sk.to_bytes())
            .unwrap();
        let (_, addrs) = x.verify("net1", now_unix(), &pk).unwrap();
        // The good address survives; the loopback one is dropped, not dialled.
        assert_eq!(addrs, vec!["/ip4/203.0.113.5/tcp/4001".to_string()]);
        assert!(!addrs.iter().any(|a| a.contains("127.0.0.1")));
    }

    #[test]
    fn expired_exchange_is_refused() {
        let (sk, pk) = trusted();
        let x = AddressExchange::new(
            "net1",
            "12D3KooWabc",
            "did:ghost:x",
            vec!["/ip4/203.0.113.5/tcp/4001".to_string()],
            1,
        )
        .sign(&sk.to_bytes())
        .unwrap();
        assert!(x.verify("net1", x.expires_at + 1, &pk).is_err());
    }

    #[test]
    fn tampered_exchange_address_is_refused() {
        let (sk, pk) = trusted();
        let x = AddressExchange::new(
            "net1",
            "12D3KooWabc",
            "did:ghost:x",
            vec!["/ip4/203.0.113.5/tcp/4001".to_string()],
            1,
        )
        .sign(&sk.to_bytes())
        .unwrap();
        // Swap the address after signing: signature must no longer verify.
        let mut bad = x.clone();
        bad.addresses = vec!["/ip4/198.51.100.9/tcp/4001".to_string()];
        assert!(bad.verify("net1", now_unix(), &pk).is_err());
    }

    #[test]
    fn exchange_from_another_network_is_refused() {
        let (sk, pk) = trusted();
        let x = AddressExchange::new(
            "other-net",
            "12D3KooWabc",
            "did:ghost:x",
            vec!["/ip4/203.0.113.5/tcp/4001".to_string()],
            1,
        )
        .sign(&sk.to_bytes())
        .unwrap();
        assert!(x.verify("net1", now_unix(), &pk).is_err());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_reticulum_address_deterministic() {
        let seed = [0x42u8; 32];
        let addr1 = compute_reticulum_address(&seed);
        let addr2 = compute_reticulum_address(&seed);
        assert_eq!(addr1, addr2);
        assert!(!addr1.is_empty());
    }

    #[test]
    fn compute_reticulum_address_different_seeds() {
        let seed_a = [0x01u8; 32];
        let seed_b = [0x02u8; 32];
        let addr_a = compute_reticulum_address(&seed_a);
        let addr_b = compute_reticulum_address(&seed_b);
        assert_ne!(addr_a, addr_b);
    }

    #[test]
    fn reticulum_envelope_is_domain_separated() {
        assert_eq!(RNS_ENVELOPE_VERSION, 1);
        assert_eq!(RNS_ENVELOPE_DOMAIN, "ghostwire.rns.message");
    }

    #[test]
    fn reticulum_stats_default() {
        let stats = ReticulumStats {
            total_nodes: 0,
            online_nodes: 0,
            local_node_id: "test".into(),
            routes_count: 0,
            onion_routes_count: 0,
            messages_relayed: 0,
            encryption_errors: 0,
            initialized: false,
            degraded: false,
            interfaces_configured: 0,
            messages_sent: 0,
            messages_received: 0,
            send_failures: 0,
            last_error: None,
        };
        assert_eq!(stats.total_nodes, 0);
        assert_eq!(stats.messages_relayed, 0);
    }

    #[test]
    fn reticulum_node_serialization() {
        let node = ReticulumNode {
            id: "node1".into(),
            public_key: vec![1, 2, 3],
            address: "abc123".into(),
            last_seen: 1000,
            connection_quality: 0.95,
            is_online: true,
            onion_routing_capable: false,
        };
        let json = serde_json::to_string(&node).unwrap();
        let parsed: ReticulumNode = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.id, "node1");
        assert!(parsed.is_online);
        assert!((parsed.connection_quality - 0.95).abs() < f32::EPSILON);
    }

    #[test]
    fn reticulum_message_serialization() {
        let msg = ReticulumMessage {
            id: "msg1".into(),
            sender: "alice".into(),
            recipient: "bob".into(),
            content: b"hello".to_vec(),
            onion_layers: vec![],
            timestamp: 1000,
            ttl: 60,
            hop_count: 0,
            max_hops: 10,
        };
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: ReticulumMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.sender, "alice");
        assert_eq!(parsed.content, b"hello");
    }

    #[test]
    fn reticulum_topology_default() {
        let topo = ReticulumTopology {
            nodes: HashMap::new(),
            routes: HashMap::new(),
            local_node_id: "local".into(),
            onion_routes: HashMap::new(),
        };
        assert!(topo.nodes.is_empty());
        assert!(topo.routes.is_empty());
    }
}

#[cfg(test)]
mod routing_tests {
    use super::is_reticulum_address;

    // A real Reticulum address hash is 16 bytes -> 32 hex chars.
    const RNS: &str = "adf74d1c1b1e5c1a3f9c1e2b3d4f5a6b";
    // A real libp2p PeerId, base58, from a live node.
    const PEER: &str = "12D3KooWWHvd9Z3VQdjtjWe42xSP9TFNSDeD4K73zfVxTdCXAUFY9";

    #[test]
    fn a_reticulum_address_is_routed_to_rns() {
        assert!(is_reticulum_address(RNS));
    }

    #[test]
    fn a_libp2p_peer_id_is_never_treated_as_a_reticulum_address() {
        // The whole point of the strict check. parse_address_hash accepts ANY
        // hex of ANY length, so an over-permissive test would misroute a PeerId
        // (or any hex-looking id) to RNS and encrypt it to a destination nobody
        // holds the key for.
        assert!(!is_reticulum_address(PEER));
    }

    #[test]
    fn wrong_length_hex_is_rejected() {
        assert!(!is_reticulum_address("abcd"));
        assert!(!is_reticulum_address("adf74d1c1b1e5c1a3f9c1e2b3d4f5a6bcd"));
    }

    #[test]
    fn non_hex_is_rejected() {
        // Same length as a real address, but not hex.
        assert!(!is_reticulum_address("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz"));
    }

    #[test]
    fn prefixed_and_padded_addresses_are_tolerated() {
        assert!(is_reticulum_address(
            "  0xadf74d1c1b1e5c1a3f9c1e2b3d4f5a6b "
        ));
    }

    #[test]
    fn an_empty_recipient_is_rejected() {
        assert!(!is_reticulum_address(""));
    }
}
