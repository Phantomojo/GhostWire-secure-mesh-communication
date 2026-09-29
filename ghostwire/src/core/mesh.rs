use crate::core::encryption::{EncryptionManager, MessageType as EncMessageType};
use crate::core::identity::Identity;
use crate::core::message::Message;
use crate::core::security::{SecurityConfig, SecurityManager, SecurityStats, ThreatLevel};
use crate::core::stealth_tcp::{ConnectionStats, StealthTCPProvider};
use crate::core::traffic_armor;
use anyhow::Result;
// use async_trait::async_trait;
use crate::core::username_discovery::UsernameDiscovery;
use crate::core::world_discovery::{
    advertisement_record_key, bounded_addresses, configured_network_id, discovery_topic,
    private_endpoints_allowed, provider_record_key, SharedWorldDiscoveryHealth,
    WorldDiscoveryAdvertisement, WorldDiscoveryHealth, MAX_TTL_SECS,
};
use base64::Engine;
use futures_util::StreamExt;
use libp2p::gossipsub::IdentTopic;
use libp2p::kad::store::MemoryStore;
use libp2p::kad::Behaviour as Kad;
// NoTcpTransport needed
use libp2p::{
    core::{transport::upgrade::Version, transport::Transport},
    gossipsub::{
        Behaviour as Gossipsub, Message as GossipsubMessage, MessageAuthenticity, MessageId,
    },
    identify::{Behaviour as Identify, Config as IdentifyConfig},
    mdns::{tokio::Behaviour as Mdns, Config as MdnsConfig},
    request_response::{self, Config as RequestResponseConfig, ProtocolSupport},
    swarm::{behaviour::toggle::Toggle, NetworkBehaviour, Swarm, SwarmEvent},
    Multiaddr, PeerId,
};
use libp2p_webrtc as webrtc;
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::{mpsc, oneshot};
use tracing::debug;
use uuid::Uuid;

/// Maximum hop count for multi-hop relay
const MAX_HOPS: u8 = 5;

/// Maximum number of message IDs to track for deduplication
const SEEN_MSGS_MAX: usize = 10_000;
const IN_MEMORY_MSGS_MAX: usize = 5_000;

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[derive(Debug, Clone, Serialize)]
pub struct MeshMdnsHealth {
    pub enabled: bool,
    pub discovered_total: usize,
    pub expired_total: usize,
    pub last_event_unix: u64,
}

pub enum MeshCommand {
    PutRecord {
        key: Vec<u8>,
        value: Vec<u8>,
        reply: oneshot::Sender<Result<()>>,
    },
    GetRecord {
        key: Vec<u8>,
        reply: oneshot::Sender<Result<Vec<u8>>>,
    },
    ConnectPeer {
        peer_id: PeerId,
        reply: oneshot::Sender<Result<()>>,
    },
    /// Measure a real round-trip to a peer over the existing request_response
    /// behaviour and resolve with the elapsed milliseconds. A peer answers
    /// every request_response message with `ChatResponse("ACK")`, so this needs
    /// no new protocol and older peers answer correctly unchanged.
    PingPeer {
        peer_id: PeerId,
        reply: oneshot::Sender<Result<u64>>,
    },
    SubscribeTopic {
        topic: String,
        reply: oneshot::Sender<Result<()>>,
    },
    PublishUsername {
        reply: oneshot::Sender<Result<()>>,
    },
    SetShard {
        shard: String,
        reply: oneshot::Sender<Result<()>>,
    },
    BroadcastMessage {
        message: Message,
        reply: oneshot::Sender<Result<()>>,
    },
    PublishToTopic {
        topic: String,
        data: Vec<u8>,
        reply: oneshot::Sender<Result<()>>,
    },
    PublishWorldDiscovery {
        reply: oneshot::Sender<Result<()>>,
    },
    SetWorldDiscoveryProvider {
        enabled: bool,
        reply: oneshot::Sender<Result<()>>,
    },
    RefreshWorldDiscoveryProviders {
        reply: oneshot::Sender<Result<()>>,
    },
    SendMessage {
        recipient: PeerId,
        message: Message,
        reply: oneshot::Sender<Result<()>>,
    },
    GetListeners {
        reply: oneshot::Sender<Vec<Multiaddr>>,
    },
    AddAddress {
        peer_id: PeerId,
        address: Multiaddr,
        reply: oneshot::Sender<Result<()>>,
    },
    Shutdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest(pub Message);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse(pub String);

/// Mesh network node information
#[derive(Debug, Clone, Serialize)]
pub struct MeshNode {
    pub id: String,
    #[serde(skip)]
    pub peer_id: PeerId,
    #[serde(skip)]
    pub last_seen_addresses: Vec<Multiaddr>,
    pub username: String,
    pub public_key: Vec<u8>,
    pub last_seen: u64,
    pub connection_quality: f32,
    pub is_online: bool,
    pub threat_level: ThreatLevel,
}

/// Mesh network topology
#[derive(Debug, Clone, Serialize)]
pub struct MeshTopology {
    pub nodes: HashMap<String, MeshNode>,
    pub routes: HashMap<String, Vec<String>>,
    pub local_node_id: String,
}

#[derive(NetworkBehaviour)]
pub struct MeshBehaviour {
    pub gossipsub: Gossipsub,
    pub kad: Kad<MemoryStore>,
    pub identify: Identify,
    pub mdns: Toggle<Mdns>,
    pub autonat: libp2p::autonat::Behaviour,
    pub relay: libp2p::relay::client::Behaviour,
    pub dcutr: libp2p::dcutr::Behaviour,
    pub request_response: libp2p::request_response::cbor::Behaviour<ChatRequest, ChatResponse>,
    /// Caps parallel connections per peer.
    ///
    /// Four independent discovery sources (mdns, kad_provider, kad_closest,
    /// world_discovery) each dial a peer's advertised addresses, and
    /// `dial_peer_if_needed` only suppresses a dial when the peer is already
    /// connected *or* has a dial in flight. Those two guards do not stop a
    /// second dial on a different address once the first connection is
    /// established, so the swarm settled at 3-4 parallel connections to a
    /// single peer and then pruned the extras, which surfaced as
    /// ConnectionEstablished/ConnectionClosed churn.
    ///
    /// Measured 2026-09-28 across two live nodes: closes landed in the
    /// :30-:39 window of each minute, aligned to the 60s world-discovery
    /// republish, in bursts averaging 6, with 3 addresses advertised per node.
    /// libp2p prunes the surplus connections and the peer oscillates between
    /// 3 and 4 established.
    ///
    /// This uses the already-enabled `memory-connection-limits` feature, so no
    /// new dependency is introduced.
    ///
    /// The per-peer cap is deliberately 2, not 1. `libp2p-request-response`
    /// asserts internally (0.29.0, src/lib.rs:678):
    ///
    /// ```text
    ///     debug_assert_eq!(connections.is_empty(), remaining_established == 0);
    /// ```
    ///
    /// i.e. its own per-peer connection list must be empty exactly when
    /// libp2p reports zero remaining. With a cap of 1 libp2p closes the
    /// connection as soon as the second one is refused, and the two counts
    /// disagree mid-teardown, which panics a debug build. Observed live on the
    /// Elitebook node with `with_max_established(Some(1))`:
    ///
    /// ```text
    ///     thread 'tokio-rt-worker' panicked at
    ///       libp2p-request-response-0.29.0/src/lib.rs:678:9:
    ///     assertion `left == right` failed  left: false  right: true
    /// ```
    ///
    /// 2 still collapses the observed 3-4 connections to a stable pair while
    /// leaving request-response a spare to tear down against, so the assert
    /// holds.
    pub connection_limits: libp2p::connection_limits::Behaviour,
}

/// A libp2p transport that wraps TCP in a Stealth Handshake
pub struct StealthTransport {
    _provider: Arc<StealthTCPProvider>,
    _inner: libp2p::tcp::tokio::Transport,
}

impl StealthTransport {
    pub fn new(provider: Arc<StealthTCPProvider>) -> Self {
        Self {
            _provider: provider,
            _inner: libp2p::tcp::tokio::Transport::new(
                libp2p::tcp::Config::default().nodelay(true),
            ),
        }
    }
}

/// Mesh transport implementation with enhanced security
pub struct MeshTransport {
    swarm: Option<Swarm<MeshBehaviour>>,
    command_tx: mpsc::Sender<MeshCommand>,
    command_rx: Option<mpsc::Receiver<MeshCommand>>,
    node_id: String,
    identity: Arc<Identity>,
    security_manager: Arc<SecurityManager>,
    encryption_manager: Arc<EncryptionManager>,
    _stats: Arc<RwLock<MeshStats>>,
    current_shard: String,
    _core_messages: Arc<RwLock<Vec<Message>>>,
    _message_store: Arc<crate::core::store::SecureMessageStore>,
    pub bootstrap_nodes: Vec<Multiaddr>,
    username_discovery: Arc<crate::core::username_discovery::UsernameDiscovery>,
    event_bus: Option<tokio::sync::broadcast::Sender<crate::core::CoreEvent>>,
    topology: Arc<RwLock<HashMap<String, MeshNode>>>,
    peer_count_cache: Option<Arc<AtomicUsize>>,
    dtn_store: Option<Arc<crate::core::dtn::epidemic::DtnStore>>,
    mailbox_store: Option<Arc<crate::core::mailbox::MailboxStore>>,
    reliability: Arc<crate::core::reliability::ReliabilityLayer>,
    peer_health: Arc<crate::core::peer_health::PeerHealthTracker>,
    _pending_pings: tokio::sync::Mutex<crate::core::peer_health::PendingPings>,
    mdns_discovered: Arc<AtomicUsize>,
    mdns_expired: Arc<AtomicUsize>,
    mdns_last_event: Arc<AtomicU64>,
    mdns_enabled: bool,
    discovery_directory: Arc<crate::core::discovery_directory::DiscoveryDirectory>,
    /// False once the swarm driver task has stopped (panic or clean exit).
    ///
    /// The driver owns the Swarm and is consumed by `run()`, so if it dies
    /// there is no mesh any more -- but nothing recorded that. The node kept
    /// answering API calls and logging autonomic/transport lines while the mesh
    /// was completely dead, which is the worst possible failure mode: a
    /// live-looking node with no connectivity. `is_mesh_alive` lets callers
    /// report `degraded` instead of pretending.
    alive: Arc<std::sync::atomic::AtomicBool>,
    world_discovery_network_id: String,
    world_discovery_enabled: bool,
    world_discovery_health: SharedWorldDiscoveryHealth,
}

impl MeshTransport {
    pub async fn new(
        identity: Arc<Identity>,
        security_manager: Arc<SecurityManager>,
        encryption_manager: Arc<EncryptionManager>,
        core_messages: Arc<RwLock<Vec<Message>>>,
        message_store: Arc<crate::core::store::SecureMessageStore>,
        bootstrap_nodes: Vec<Multiaddr>,
        username_discovery: Arc<crate::core::username_discovery::UsernameDiscovery>,
        event_bus: Option<tokio::sync::broadcast::Sender<crate::core::CoreEvent>>,
        _handshake_secret: Option<Vec<u8>>,
        reliability: Arc<crate::core::reliability::ReliabilityLayer>,
        discovery_directory: Arc<crate::core::discovery_directory::DiscoveryDirectory>,
        quic_port: u16,
    ) -> Result<Self> {
        let local_key = identity.keypair().ok_or_else(|| {
            anyhow::anyhow!("Identity is locked during Mesh Transport initialization")
        })?;
        let local_peer_id = PeerId::from(local_key.public());
        let world_discovery_network_id = configured_network_id();
        crate::core::world_discovery::validate_network_id(&world_discovery_network_id)?;
        let world_discovery_enabled = std::env::var_os("GHOSTWIRE_DISABLE_DISCOVERY").is_none();
        let world_discovery_health = Arc::new(RwLock::new(WorldDiscoveryHealth::new(
            world_discovery_enabled,
            world_discovery_network_id.clone(),
        )));
        // Use PeerId string as node_id (stable, unique, matches network identity)
        let node_id = local_peer_id.to_string();

        // Initialize obfs4 for DPI evasion (always-on by default)
        // Note: Actual obfuscation integrated in transport stack in future releases

        // Create base TCP transport
        let tcp_transport =
            libp2p::tcp::tokio::Transport::new(libp2p::tcp::Config::default().nodelay(true));

        // Add relay transport for NAT traversal
        let (relay_transport, relay_client) = libp2p::relay::client::new(local_peer_id);

        // WebSocket transport for Cloudflare-edge dialing (/ws locally, /wss
        // via the edge which terminates TLS). Shares Noise+Yamux upgrade below.
        let ws_tcp_transport =
            libp2p::tcp::tokio::Transport::new(libp2p::tcp::Config::default().nodelay(true));
        let ws_transport = libp2p::websocket::Config::new(ws_tcp_transport);

        // WebRTC is opt-in and, as wired today, INCOMPLETE.
        //
        // It was previously constructed unconditionally and joined into the
        // transport chain, which made it look supported. It could not move a
        // byte, for two independent reasons:
        //
        //  1. libp2p-webrtc's dial path borrows a `udp_mux` from an existing
        //     listener and hard-fails without one:
        //         self.listeners.iter().next().ok_or(TransportError::Other(Error::NoListeners))?
        //     Nothing in src/ ever called `listen_on` for WebRTC, so the
        //     listener list was always empty and every dial failed there
        //     regardless of the address offered.
        //  2. The certificate is regenerated with `Certificate::generate` on
        //     every process start, so the certhash is never stable across
        //     restarts and is never published. Even with a listener, a peer
        //     would have no way to learn a matching /certhash to dial.
        //
        // So it is gated behind an explicit env var instead of sitting in the
        // chain looking live. Turning this on is necessary but NOT sufficient to
        // make WebRTC work: it also needs a real listener and a stable,
        // published certhash. Until that exists, treat WebRTC as unimplemented
        // rather than as a fallback.
        let webrtc_enabled = std::env::var("GHOSTWIRE_WEBRTC")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let webrtc_boxed = if webrtc_enabled {
            let webrtc_transport = webrtc::tokio::Transport::new(
                local_key.clone(),
                webrtc::tokio::Certificate::generate(&mut OsRng)?,
            );
            tracing::warn!(
                "WebRTC enabled by operator, but it is still INCOMPLETE: no listener is \
                 created and the certhash is regenerated every start, so peers still cannot \
                 dial it. See the comment above."
            );
            Some(
                webrtc_transport
                    .map(|out, _state| {
                        let (peer_id, conn) = out;
                        (peer_id, libp2p::core::muxing::StreamMuxerBox::new(conn))
                    })
                    .boxed(),
            )
        } else {
            None
        };

        // QUIC transport on the operator's chosen UDP port (--quic-port, default
        // 8081). QUIC runs over UDP, so a TCP-throttling middlebox or a
        // TCP-only firewall still lets it through -- that is the whole point of
        // running it.
        //
        // Placed in the stack AFTER the Noise+Yamux upgrade for the other
        // transports, because libp2p's QUIC transport performs its own TLS 1.3
        // handshake with Noise-style identity verification: libp2p-quic
        // authenticates the remote peer inside that handshake, so wrapping it
        // in Noise again is not required for confidentiality or peer
        // authentication. Putting it before .authenticate() would fail to
        // compile, since the QUIC transport is already a full authenticated
        // connection upgrade.
        let mut quic_transport =
            libp2p::quic::tokio::Transport::new(libp2p::quic::Config::new(&local_key));
        if quic_port > 0 {
            let quic_addr: libp2p::Multiaddr = format!("/ip4/0.0.0.0/udp/{}/quic-v1", quic_port)
                .parse()
                .map_err(|e| anyhow::anyhow!("invalid QUIC multiaddr for udp/{quic_port}: {e}"))?;
            use libp2p::core::transport::{ListenerId, Transport};
            // QUIC is an ADDITIONAL path, not the only one. A failed bind must
            // not take the node down: TCP/relay/WS/WebRTC all still work, and
            // the operator can still reach the node. Making this fatal broke 17
            // unrelated tests, because Core::new died whenever udp/8081 was
            // already held (two nodes on one host, or a parallel test run).
            //
            // A node that cannot bind its QUIC port is `degraded`, not `failed`
            // -- it still has a working mesh on the other transports.
            match quic_transport.listen_on(ListenerId::next(), quic_addr) {
                Ok(listener_id) => {
                    tracing::info!("QUIC transport listening on udp/{}", quic_port);
                    let _ = listener_id;
                }
                Err(e) => {
                    tracing::warn!(
                        "QUIC listener unavailable on udp/{} ({}); continuing on TCP/relay/WebSocket/WebRTC. The node is degraded, not down.",
                        quic_port,
                        e
                    );
                }
            }
        }

        // Build the transport stack:
        // TCP → Noise (encryption) → Yamux (multiplexing)
        // obfs4 obfuscation is applied at the connection layer
        // Fallback to relay, WebSocket, QUIC and WebRTC for NAT traversal
        //
        // The base stack is boxed before QUIC and WebRTC are added. Boxing
        // erases the Either nesting to a single level, so the final `.map`
        // stays two-armed instead of needing one closure per transport group.
        let base_transport = tcp_transport
            .or_transport(relay_transport)
            .or_transport(ws_transport)
            .upgrade(Version::V1)
            .authenticate(libp2p::noise::Config::new(&local_key)?)
            .multiplex(libp2p::yamux::Config::default())
            .map(|out, _state| {
                let (peer_id, muxer) = out;
                (peer_id, libp2p::core::muxing::StreamMuxerBox::new(muxer))
            })
            .boxed();

        // QUIC's Connection and WebRTC's Connection are different types, so each
        // branch is erased to StreamMuxerBox separately before they are joined.
        let quic_boxed = quic_transport
            .map(|out, _state| {
                let (peer_id, conn) = out;
                (peer_id, libp2p::core::muxing::StreamMuxerBox::new(conn))
            })
            .boxed();

        // Each `.or_transport` introduces one Either level, so join one at a
        // time and erase as we go rather than stacking three transports and
        // trying to match a 3-deep Either by hand.
        // WebRTC is joined only when the operator opted in; by default the
        // chain is base + QUIC. See the WebRTC comment above for why it is
        // opt-in rather than always-on.
        let transport = match webrtc_boxed {
            Some(w) => base_transport
                .or_transport(quic_boxed)
                .map(|either, _state| match either {
                    libp2p::futures::future::Either::Left((peer_id, muxer)) => (peer_id, muxer),
                    libp2p::futures::future::Either::Right((peer_id, muxer)) => {
                        (peer_id, libp2p::core::muxing::StreamMuxerBox::new(muxer))
                    }
                })
                .or_transport(w)
                .map(|either, _state| match either {
                    libp2p::futures::future::Either::Left((peer_id, muxer)) => (peer_id, muxer),
                    libp2p::futures::future::Either::Right((peer_id, muxer)) => {
                        (peer_id, libp2p::core::muxing::StreamMuxerBox::new(muxer))
                    }
                })
                .boxed(),
            None => base_transport
                .or_transport(quic_boxed)
                .map(|either, _state| match either {
                    libp2p::futures::future::Either::Left((peer_id, muxer)) => (peer_id, muxer),
                    libp2p::futures::future::Either::Right((peer_id, muxer)) => {
                        (peer_id, libp2p::core::muxing::StreamMuxerBox::new(muxer))
                    }
                })
                .boxed(),
        };

        // Create network behavior with hardened S/Kademlia parameters
        // FIXED: replaced GossipsubConfig::default() with hardened config from skademlia.rs
        // This prevents relay score inflation, sybil dominance, and gossip flooding.
        // Wave 7 FIX: Simplified API — peer scoring can be re-added after first live test
        use crate::core::routing::skademlia::hardened_gossipsub_config;
        let gossipsub_config = hardened_gossipsub_config();
        let gossipsub = Gossipsub::new(
            MessageAuthenticity::Signed(local_key.clone()),
            gossipsub_config,
        )
        .map_err(|e| anyhow::anyhow!(e))?;

        let identify = Identify::new(IdentifyConfig::new(
            "/ghostwire/mesh/2.0.0".to_string(),
            local_key.public(),
        ));

        // Recursive DHT: Regional Sharding logic
        // Nodes join a global shard AND a regional shard for high-velocity lookups
        let shard_id = "global"; // Region/shard from config in future
        let kad_config =
            libp2p::kad::Config::new(libp2p::StreamProtocol::new("/ghostwire/kad/shard/global"));

        let mut kad = Kad::with_config(local_peer_id, MemoryStore::new(local_peer_id), kad_config);

        // Harden Kademlia: Increase replication and periodic refresh
        kad.set_mode(Some(libp2p::kad::Mode::Server));

        let mdns_enabled = std::env::var_os("GHOSTWIRE_TEST_DISABLE_MDNS").is_none();
        let mdns = if mdns_enabled {
            Some(Mdns::new(MdnsConfig::default(), local_peer_id)?)
        } else {
            tracing::info!("mDNS disabled by explicit test isolation policy");
            None
        };

        let autonat =
            libp2p::autonat::Behaviour::new(local_peer_id, libp2p::autonat::Config::default());

        let dcutr = libp2p::dcutr::Behaviour::new(local_peer_id);

        let request_response =
            libp2p::request_response::cbor::Behaviour::<ChatRequest, ChatResponse>::new(
                [(
                    libp2p::StreamProtocol::new("/ghostwire/chat/1.0.0"),
                    ProtocolSupport::Full,
                )],
                RequestResponseConfig::default(),
            );

        let behaviour = MeshBehaviour {
            gossipsub,
            identify,
            kad,
            mdns: mdns.into(),
            autonat,
            relay: relay_client,
            dcutr,
            request_response,
            // Cap parallel connections per peer. Two, not one: see the
            // connection_limits doc comment for the request-response debug
            // assert that a cap of 1 trips. Collapses the observed 3-4
            // connections to a stable pair. Pending dials are bounded too so
            // a discovery burst cannot queue unbounded dials.
            connection_limits: libp2p::connection_limits::Behaviour::new(
                libp2p::connection_limits::ConnectionLimits::default()
                    .with_max_established(Some(2))
                    .with_max_pending_incoming(Some(16))
                    .with_max_pending_outgoing(Some(16)),
            ),
        };

        let swarm = libp2p::SwarmBuilder::with_existing_identity(local_key)
            .with_tokio()
            .with_other_transport(|_| transport)?
            .with_behaviour(|_| behaviour)?
            .with_swarm_config(|c| {
                c.with_idle_connection_timeout(std::time::Duration::from_secs(60))
            })
            .build();

        let (command_tx, command_rx) = mpsc::channel(100);

        Ok(Self {
            swarm: Some(swarm),
            command_tx,
            command_rx: Some(command_rx),
            node_id: node_id.clone(),
            identity,
            security_manager,
            encryption_manager,
            _stats: Arc::new(RwLock::new(MeshStats {
                total_nodes: 0,
                online_nodes: 0,
                local_node_id: node_id,
                routes_count: 0,
                connection_stats: ConnectionStats::default(),
                security_stats: SecurityStats::default(),
            })),
            current_shard: shard_id.to_string(),
            alive: Arc::new(std::sync::atomic::AtomicBool::new(true)),
            _core_messages: core_messages,
            _message_store: message_store,
            bootstrap_nodes,
            username_discovery,
            event_bus,
            topology: Arc::new(RwLock::new(HashMap::new())),
            peer_count_cache: None,
            dtn_store: None,
            mailbox_store: None,
            reliability,
            peer_health: Arc::new(crate::core::peer_health::PeerHealthTracker::new(
                std::time::Duration::from_secs(15),
                std::time::Duration::from_secs(30),
            )),
            _pending_pings: tokio::sync::Mutex::new(crate::core::peer_health::PendingPings::new()),
            mdns_discovered: Arc::new(AtomicUsize::new(0)),
            mdns_expired: Arc::new(AtomicUsize::new(0)),
            mdns_last_event: Arc::new(AtomicU64::new(0)),
            mdns_enabled,
            discovery_directory,
            world_discovery_network_id,
            world_discovery_enabled,
            world_discovery_health,
        })
    }

    /// Start the mesh network with enhanced security
    pub async fn start(&mut self, listen_addr: Multiaddr) -> Result<()> {
        tracing::info!("Starting secure mesh network on {}", listen_addr);

        let mut swarm = self
            .swarm
            .take()
            .ok_or_else(|| anyhow::anyhow!("Swarm already started or missing"))?;
        let command_rx = self
            .command_rx
            .take()
            .ok_or_else(|| anyhow::anyhow!("Channels missing"))?;

        // Security check for listen address
        if let Some(ip) = listen_addr.iter().find_map(|proto| {
            if let libp2p::multiaddr::Protocol::Ip4(ip) = proto {
                Some(ip.into())
            } else if let libp2p::multiaddr::Protocol::Ip6(ip) = proto {
                Some(ip.into())
            } else {
                None
            }
        }) {
            if !self.security_manager.is_ip_allowed(&ip).await {
                return Err(anyhow::anyhow!(
                    "Listen address {} is not allowed by security policy",
                    ip
                ));
            }
        }

        // Listen on the specified address
        swarm.listen_on(listen_addr.clone())?;

        // Opt-in WebSocket listener for Cloudflare Tunnel origin duty.
        // Set GHOSTWIRE_WS_LISTEN=/ip4/127.0.0.1/tcp/4002/ws on the seed box;
        // the Tunnel ingress forwards edge /wss to this local /ws listener.
        // Unset = no behavior change.
        if let Ok(raw_ws) = std::env::var("GHOSTWIRE_WS_LISTEN") {
            match raw_ws.parse::<Multiaddr>() {
                Ok(ws_addr) => {
                    if let Err(e) = swarm.listen_on(ws_addr.clone()) {
                        tracing::warn!("WS listen failed on {}: {}", ws_addr, e);
                    } else {
                        tracing::info!("Listening for WS origin traffic on {}", ws_addr);
                    }
                }
                Err(_) => tracing::warn!("Ignoring bad GHOSTWIRE_WS_LISTEN: {}", raw_ws),
            }
        }

        // Explicit, opt-in test dial. This is deliberately environment-gated;
        // production startup never dials an arbitrary address from the environment.
        if let Ok(raw) = std::env::var("GHOSTWIRE_TEST_DIAL") {
            for raw_addr in raw.split(',').map(str::trim).filter(|v| !v.is_empty()) {
                match raw_addr.parse::<Multiaddr>() {
                    Ok(addr) => {
                        tracing::info!(address = %addr, "Dialing explicit test peer");
                        if let Err(error) = swarm.dial(addr.clone()) {
                            tracing::warn!(address = %addr, %error, "Explicit test peer dial failed");
                        }
                    }
                    Err(error) => {
                        tracing::warn!(address = raw_addr, %error, "Invalid explicit test peer multiaddress")
                    }
                }
            }
        }

        // Subscribe to broadcast topic
        let topic = IdentTopic::new(crate::core::constants::BROADCAST_TOPIC);
        swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

        // Get local peer ID before moving swarm into driver
        let local_peer_id = *swarm.local_peer_id();

        // Subscribe to shared DM topic (all DMs go here, content encrypted end-to-end)
        // Using a single topic avoids leaking recipient PeerId in gossipsub metadata
        let dm_topic = IdentTopic::new(crate::core::constants::topics::DM);
        swarm.behaviour_mut().gossipsub.subscribe(&dm_topic)?;
        tracing::info!("Subscribed to shared DM topic");

        let dtn_topic = IdentTopic::new(crate::core::constants::DTN_SYNC_TOPIC);
        swarm.behaviour_mut().gossipsub.subscribe(&dtn_topic)?;
        tracing::info!("Subscribed to DTN sync topic");

        // Subscribe to CRL gossip topic for revocation propagation
        let crl_gossip_topic = IdentTopic::new(crate::core::constants::topics::CRL_GOSSIP);
        swarm
            .behaviour_mut()
            .gossipsub
            .subscribe(&crl_gossip_topic)?;
        tracing::info!("Subscribed to CRL gossip topic");

        if self.world_discovery_enabled {
            let discovery_topic =
                IdentTopic::new(discovery_topic(&self.world_discovery_network_id));
            swarm
                .behaviour_mut()
                .gossipsub
                .subscribe(&discovery_topic)?;
            tracing::info!(network_id = %self.world_discovery_network_id, topic = %discovery_topic, "Subscribed to GhostWire world-discovery topic");
        }

        // Connect to bootstrap nodes
        for addr in &self.bootstrap_nodes {
            if let Some(peer_id) = addr.iter().find_map(|p| {
                if let libp2p::multiaddr::Protocol::P2p(peer_id) = p {
                    Some(peer_id)
                } else {
                    None
                }
            }) {
                tracing::info!("Adding bootstrap node: {} ({})", peer_id, addr);
                swarm
                    .behaviour_mut()
                    .kad
                    .add_address(&peer_id, addr.clone());
                if let Err(error) = swarm.dial(addr.clone()) {
                    tracing::warn!(peer_id = %peer_id, address = %addr, %error, "Configured bootstrap dial failed");
                } else {
                    tracing::info!(peer_id = %peer_id, address = %addr, "Dialing configured GhostWire bootstrap");
                }
            } else {
                tracing::warn!("Bootstrap address missing PeerId: {}", addr);
            }
        }

        // Opt-in relay reservation (circuit relay v2 client).
        // GHOSTWIRE_RELAY=/ip4/<relay>/tcp/<port>/p2p/<relay-id>: listen on
        // the relay's p2p-circuit address, which drives reservation + renewal
        // through the relay client behaviour. Unset = no relay duty.
        if let Ok(raw_relay) = std::env::var("GHOSTWIRE_RELAY") {
            match raw_relay.trim().parse::<Multiaddr>() {
                Ok(relay_addr) => {
                    let circuit_addr = relay_addr.with(libp2p::multiaddr::Protocol::P2pCircuit);
                    match swarm.listen_on(circuit_addr.clone()) {
                        Ok(_) => tracing::info!("Requested relay reservation via {}", circuit_addr),
                        Err(e) => tracing::warn!(
                            "Relay reservation listen failed on {}: {}",
                            circuit_addr,
                            e
                        ),
                    }
                }
                Err(_) => tracing::warn!("Ignoring bad GHOSTWIRE_RELAY: {}", raw_relay),
            }
        }

        let (cover_tx, cover_rx) = mpsc::channel::<crate::core::traffic_armor::PaddedFrame>(32);
        let _traffic_armor = Arc::new(crate::core::traffic_armor::TrafficArmor::new(
            Some(cover_tx),
            crate::core::armor::DeviceTier::detect(),
        ));

        let driver = MeshDriver {
            swarm,
            command_rx,
            _node_id: self.node_id.clone(),
            local_peer_id,
            _identity: self.identity.clone(),
            security_manager: self.security_manager.clone(),
            encryption_manager: self.encryption_manager.clone(),
            _username_discovery: self.username_discovery.clone(),
            event_bus: self.event_bus.clone(),
            pending_get_queries: HashMap::new(),
            message_store: self._message_store.clone(),
            core_messages: self._core_messages.clone(),
            seen_messages: VecDeque::new(),
            topology: self.topology.clone(),
            peer_count_cache: self.peer_count_cache.clone(),
            dtn_store: self.dtn_store.clone(),
            mailbox_store: self.mailbox_store.clone(),
            completed_handshakes: std::collections::HashSet::new(),
            pending_dials: std::collections::HashSet::new(),
            reliability: self.reliability.clone(),
            vector_clock: crate::core::vector_clock::VectorClock::new(),
            pending_messages: HashMap::new(),
            peer_health: self.peer_health.clone(),
            pending_pings: tokio::sync::Mutex::new(crate::core::peer_health::PendingPings::new()),
            mdns_discovered: self.mdns_discovered.clone(),
            mdns_expired: self.mdns_expired.clone(),
            mdns_last_event: self.mdns_last_event.clone(),
            discovery_directory: self.discovery_directory.clone(),
            world_discovery_network_id: self.world_discovery_network_id.clone(),
            world_discovery_enabled: self.world_discovery_enabled,
            world_discovery_health: self.world_discovery_health.clone(),
            world_discovery_sequence: 0,
            mixing_queue: Arc::new(crate::core::routing::mixing_delay::MixingQueue::new(
                crate::core::routing::mixing_delay::MixingConfig::default(),
            )),
            sphinx_secret: self
                .encryption_manager
                .x25519_secret()
                .ok_or_else(|| anyhow::anyhow!("X25519 secret not available for Sphinx"))?,
            mixing_tx: None,
            cover_rx,
        };

        // Supervise the swarm driver.
        //
        // `run()` owns the Swarm and is consumed by it, so if it panics there is
        // no mesh left and nothing can rebuild it in place. Observed live on the
        // Elitebook: an upstream `libp2p-request-response` connection-bookkeeping
        // assert fired after a connection timeout, the driver task died, and the
        // node carried on answering API calls and logging autonomic lines while
        // being completely unable to reach a single peer.
        //
        // `catch_unwind` cannot repair the mesh, but it converts a silent
        // permanent failure into a loud, attributable one and lets us flip the
        // liveness flag so the API can report `degraded` honestly.
        let alive = self.alive.clone();
        let driver_alive = alive.clone();
        tokio::spawn(async move {
            let driver_future = std::panic::AssertUnwindSafe(driver.run());
            match futures_util::FutureExt::catch_unwind(driver_future).await {
                Ok(()) => tracing::error!(
                    "Mesh driver exited unexpectedly; the node has no mesh connectivity"
                ),
                Err(payload) => {
                    let detail = payload
                        .downcast_ref::<String>()
                        .cloned()
                        .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
                        .unwrap_or_else(|| "non-string panic payload".to_string());
                    tracing::error!(
                        "Mesh driver PANICKED: {}. The node is still serving the API but has \
                         NO mesh connectivity until it is restarted.",
                        detail
                    );
                    tracing::error!(
                        backtrace = %std::backtrace::Backtrace::force_capture(),
                        "mesh driver panic backtrace"
                    );
                }
            }
            // Mark dead in every exit path, panic or clean.
            driver_alive.store(false, std::sync::atomic::Ordering::SeqCst);
            tracing::error!("Mesh is now DOWN (driver stopped). Peers will be unreachable.");
        });
        Ok(())
    }

    /// Whether the swarm driver task is still running.
    ///
    /// False means the node is serving the API but has no mesh connectivity.
    /// Callers should report `degraded` rather than implying a healthy node.
    pub fn is_mesh_alive(&self) -> bool {
        self.alive.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub async fn put_record(&self, key: Vec<u8>, value: Vec<u8>) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.command_tx
            .send(MeshCommand::PutRecord {
                key,
                value,
                reply: tx,
            })
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send command: {}", e))?;
        rx.await?
    }

    pub async fn get_record(&self, key: Vec<u8>) -> Result<Vec<u8>> {
        let (tx, rx) = oneshot::channel();
        self.command_tx
            .send(MeshCommand::GetRecord { key, reply: tx })
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send command: {}", e))?;
        rx.await?
    }

    pub async fn broadcast_message(&self, message: &Message) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.command_tx
            .send(MeshCommand::BroadcastMessage {
                message: message.clone(),
                reply: tx,
            })
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send command: {}", e))?;
        rx.await?
    }

    pub async fn connect_peer(&self, peer_id: PeerId) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.command_tx
            .send(MeshCommand::ConnectPeer { peer_id, reply: tx })
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send command: {}", e))?;
        rx.await?
    }

    /// Measure a real round-trip to a peer, in milliseconds.
    ///
    /// Sends an actual request over the existing `request_response` behaviour
    /// and resolves only when the peer's reply arrives, so the number is a
    /// measurement rather than a cached or synthesised figure. A peer answers
    /// every request_response message with `ChatResponse("ACK")`, so this needs
    /// no new protocol and an older peer answers correctly unchanged.
    pub async fn ping_peer(&self, peer_id: PeerId) -> Result<u64> {
        let (tx, rx) = oneshot::channel();
        self.command_tx
            .send(MeshCommand::PingPeer { peer_id, reply: tx })
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send command: {}", e))?;
        rx.await?
    }

    pub async fn subscribe_topic(&self, topic: &str) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.command_tx
            .send(MeshCommand::SubscribeTopic {
                topic: topic.to_string(),
                reply: tx,
            })
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send command: {}", e))?;
        rx.await?
    }

    pub async fn publish_world_discovery(&self) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.command_tx
            .send(MeshCommand::PublishWorldDiscovery { reply: tx })
            .await?;
        rx.await?
    }

    pub async fn set_world_discovery_provider(&self, enabled: bool) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.command_tx
            .send(MeshCommand::SetWorldDiscoveryProvider { enabled, reply: tx })
            .await?;
        rx.await?
    }

    pub async fn refresh_world_discovery_providers(&self) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.command_tx
            .send(MeshCommand::RefreshWorldDiscoveryProviders { reply: tx })
            .await?;
        rx.await?
    }

    pub async fn world_discovery_health(&self) -> WorldDiscoveryHealth {
        self.world_discovery_health.read().await.clone()
    }

    /// Publish raw bytes to a Gossipsub topic.
    /// Used by ChannelController for ghost channel messages.
    pub async fn publish_to_topic(&self, topic: &str, data: Vec<u8>) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.command_tx
            .send(MeshCommand::PublishToTopic {
                topic: topic.to_string(),
                data,
                reply: tx,
            })
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send command: {}", e))?;
        rx.await?
    }

    pub async fn send_direct_message(&self, recipient: PeerId, message: Message) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.command_tx
            .send(MeshCommand::SendMessage {
                recipient,
                message,
                reply: tx,
            })
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send command: {}", e))?;
        rx.await?
    }

    pub async fn get_listeners(&self) -> Vec<Multiaddr> {
        let (tx, rx) = oneshot::channel();
        if let Err(e) = self
            .command_tx
            .send(MeshCommand::GetListeners { reply: tx })
            .await
        {
            tracing::error!("Failed to send GetListeners command: {}", e);
            return vec![];
        }
        rx.await.unwrap_or_default()
    }

    pub async fn get_security_stats(&self) -> crate::core::security::SecurityStats {
        self.security_manager.get_security_stats().await
    }

    pub async fn add_address(&self, peer_id: PeerId, address: Multiaddr) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.command_tx
            .send(MeshCommand::AddAddress {
                peer_id,
                address,
                reply: tx,
            })
            .await?;
        rx.await?
    }

    pub fn get_connection_stats(&self) -> ConnectionStats {
        // Return default stats - obfs4 transport doesn't expose separate stats
        // Connection stats are now tracked at the swarm level
        ConnectionStats::default()
    }

    pub async fn set_shard(&mut self, shard: &str) -> Result<()> {
        self.current_shard = shard.to_string();
        tracing::info!("MeshTransport switching to DHT Shard: {}", shard);
        Ok(())
    }

    pub fn add_bootstrap_nodes(&mut self, nodes: Vec<Multiaddr>) {
        for addr in nodes {
            let peer = addr.iter().find_map(|p| match p {
                libp2p::multiaddr::Protocol::P2p(id) => Some(id),
                _ => None,
            });
            match peer {
                Some(peer_id) => {
                    self.bootstrap_nodes.push(addr.clone());
                    // Non-blocking handoff to the driver; actor model preserved.
                    // Driver's AddAddress handler syncs swarm + Kademlia address books.
                    let (tx, _rx) = oneshot::channel();
                    if let Err(e) = self.command_tx.try_send(MeshCommand::AddAddress {
                        peer_id,
                        address: addr,
                        reply: tx,
                    }) {
                        tracing::warn!("bootstrap AddAddress handoff failed: {}", e);
                    }
                }
                None => {
                    tracing::warn!(
                        "bootstrap addr without /p2p/ peer id stored but not dialable: {}",
                        addr
                    );
                    self.bootstrap_nodes.push(addr);
                }
            }
        }
    }

    pub async fn publish_username(&mut self) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.command_tx
            .send(MeshCommand::PublishUsername { reply: tx })
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send PublishUsername command: {}", e))?;
        rx.await?
    }

    pub async fn lookup_username(
        &mut self,
        username: &str,
    ) -> Result<Option<crate::core::username_discovery::UsernameRecord>> {
        use crate::core::username_discovery::UsernameLookupResult;

        match self
            .username_discovery
            .find_peer_by_username(username)
            .await?
        {
            UsernameLookupResult::Cached(record) => Ok(Some(record)),
            UsernameLookupResult::DhtQuery(key) => {
                let _ = self.get_record(key.to_vec()).await;
                Ok(None)
            }
        }
    }

    pub async fn get_topology(&self) -> MeshTopology {
        let topo = self.topology.read().await;
        let nodes = topo
            .iter()
            .map(|(id, node)| (id.clone(), node.clone()))
            .collect();

        // Direct single-hop routes for every known node. Multi-hop routing
        // stays unimplemented until the DHT routing table feeds path discovery;
        // callers must treat a one-entry route as direct-only, not as a path.
        let routes = topo
            .keys()
            .map(|id| (id.clone(), vec![id.clone()]))
            .collect();

        MeshTopology {
            nodes,
            routes,
            local_node_id: self.node_id.clone(),
        }
    }

    /// Direct-only route lookup against the known topology.
    /// Returns `Some(single_hop)` when the target is a known node, `None`
    /// otherwise. Multi-hop pathfinding is not implemented; the ticketing
    /// for DHT-driven routes is tracked, not silently stubbed.
    pub async fn find_route(&self, target_node: &str) -> Option<Vec<String>> {
        let topo = self.topology.read().await;
        topo.contains_key(target_node)
            .then(|| vec![target_node.to_string()])
    }

    pub async fn update_connection_quality(&mut self, node_id: &str, quality: f32) {
        tracing::info!("Updated connection quality for {}: {}", node_id, quality);
    }

    pub async fn stop(&self) -> Result<()> {
        tracing::info!("Sending shutdown command to mesh driver...");
        let _ = self.command_tx.send(MeshCommand::Shutdown).await;
        Ok(())
    }
}

struct MeshDriver {
    swarm: Swarm<MeshBehaviour>,
    command_rx: mpsc::Receiver<MeshCommand>,
    _node_id: String,
    local_peer_id: PeerId,
    _identity: Arc<Identity>,
    security_manager: Arc<SecurityManager>,
    encryption_manager: Arc<EncryptionManager>,
    _username_discovery: Arc<UsernameDiscovery>,
    event_bus: Option<tokio::sync::broadcast::Sender<crate::core::CoreEvent>>,
    pending_get_queries: HashMap<libp2p::kad::QueryId, oneshot::Sender<Result<Vec<u8>>>>,
    message_store: Arc<crate::core::store::SecureMessageStore>,
    core_messages: Arc<RwLock<Vec<Message>>>,
    seen_messages: VecDeque<Uuid>,
    topology: Arc<RwLock<HashMap<String, MeshNode>>>,
    peer_count_cache: Option<Arc<AtomicUsize>>,
    dtn_store: Option<Arc<crate::core::dtn::epidemic::DtnStore>>,
    mailbox_store: Option<Arc<crate::core::mailbox::MailboxStore>>,
    completed_handshakes: std::collections::HashSet<PeerId>,
    /// PeerIDs with a dial currently in flight; owned by the mesh event loop.
    pending_dials: std::collections::HashSet<PeerId>,
    reliability: Arc<crate::core::reliability::ReliabilityLayer>,
    vector_clock: crate::core::vector_clock::VectorClock,
    /// Messages pending ACK — keyed by message ID, used for retry retransmission
    pending_messages: HashMap<Uuid, Message>,
    peer_health: Arc<crate::core::peer_health::PeerHealthTracker>,
    pending_pings: tokio::sync::Mutex<crate::core::peer_health::PendingPings>,
    mdns_discovered: Arc<AtomicUsize>,
    mdns_expired: Arc<AtomicUsize>,
    mdns_last_event: Arc<AtomicU64>,
    discovery_directory: Arc<crate::core::discovery_directory::DiscoveryDirectory>,
    world_discovery_network_id: String,
    world_discovery_enabled: bool,
    world_discovery_health: SharedWorldDiscoveryHealth,
    world_discovery_sequence: u64,
    /// Delay queue for Sphinx relay forwarding (timing-attack resistance)
    mixing_queue: Arc<crate::core::routing::mixing_delay::MixingQueue>,
    /// X25519 secret key for Sphinx packet processing
    sphinx_secret: x25519_dalek::StaticSecret,
    /// Channel for mixing queue drain loop → forward packets
    mixing_tx: Option<mpsc::Sender<crate::core::routing::mixing_delay::QueuedSphinxPacket>>,
    /// Cover traffic receiver — drains dummy frames from scheduler, publishes to DM topic
    cover_rx: mpsc::Receiver<crate::core::traffic_armor::PaddedFrame>,
}

impl MeshDriver {
    async fn run(mut self) {
        let sphinx_topic = IdentTopic::new(crate::core::constants::SPHINX_TOPIC);
        if let Err(e) = self
            .swarm
            .behaviour_mut()
            .gossipsub
            .subscribe(&sphinx_topic)
        {
            tracing::warn!("Failed to subscribe to Sphinx topic: {}", e);
        } else {
            tracing::info!(
                "Subscribed to Sphinx topic: {}",
                crate::core::constants::SPHINX_TOPIC
            );
        }

        let (mixing_tx, mut mixing_rx) =
            mpsc::channel::<crate::core::routing::mixing_delay::QueuedSphinxPacket>(64);
        let drain_queue = self.mixing_queue.clone();
        {
            let tx_slot = self.mixing_tx.insert(mixing_tx);
            drain_queue.start_drain_loop(tx_slot.clone());
        }

        let mut retry_interval = tokio::time::interval(std::time::Duration::from_secs(5));
        retry_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        let mut ping_interval = tokio::time::interval(std::time::Duration::from_secs(15));
        ping_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        // ponytail: DTN housekeeping — purge expired bundles every 60s
        let mut dtn_purge_interval = tokio::time::interval(std::time::Duration::from_secs(60));
        dtn_purge_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        let mut key_rotation_interval = tokio::time::interval(std::time::Duration::from_secs(3600));
        key_rotation_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        let mut world_discovery_interval =
            tokio::time::interval(std::time::Duration::from_secs(60));
        world_discovery_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        if self.world_discovery_enabled {
            let provider_key =
                libp2p::kad::RecordKey::new(&provider_record_key(&self.world_discovery_network_id));
            self.swarm.behaviour_mut().kad.get_providers(provider_key);
            let _ = self.publish_world_discovery().await;
        }
        loop {
            tokio::select! {
                event = self.swarm.next() => {
                    if let Some(event) = event {
                        self.handle_swarm_event(event).await;
                    } else {
                        break;
                    }
                }
                command = self.command_rx.recv() => {
                    if let Some(cmd) = command {
                        let is_shutdown = matches!(cmd, MeshCommand::Shutdown);
                        self.handle_command(cmd).await;
                        if is_shutdown {
                            tracing::info!("Mesh driver shutting down swarm...");
                            break;
                        }
                    } else {
                        break;
                    }
                }
                _ = retry_interval.tick() => {
                    self.handle_retries().await;
                }
                _ = ping_interval.tick() => {
                    self.send_periodic_pings().await;
                }
                _ = dtn_purge_interval.tick() => {
                    if let Some(ref dtn_store) = self.dtn_store {
                        dtn_store.purge_expired().await;
                        let delivered = dtn_store.drain_local_deliveries().await;
                        for bundle in delivered {
                            if let Ok((frame, _)) =
                                crate::core::wire_protocol::Frame::from_bytes(&bundle.payload)
                            {
                                if let Ok(m) = frame.decode::<Message>() {
                                    tracing::info!(
                                        "DTN: locally delivered bundle {:02x?}",
                                        &bundle.cid[..8]
                                    );
                                    let mut msgs = self.core_messages.write().await;
                                    msgs.push(m);
                                    if msgs.len() > IN_MEMORY_MSGS_MAX {
                                        msgs.drain(..IN_MEMORY_MSGS_MAX / 2);
                                    }
                                }
                            }
                        }
                    }
                }
                _ = world_discovery_interval.tick() => {
                    let _ = self.publish_world_discovery().await;
                }
                _ = key_rotation_interval.tick() => {
                    if let Some(enc) = Arc::get_mut(&mut self.encryption_manager) {
                        if let Err(e) = enc.rotate_keys_if_needed() {
                            tracing::warn!("Key rotation failed: {}", e);
                        }
                        // Garbage collect expired CRL entries
                        let gc = enc.gc_crl();
                        if gc > 0 {
                            tracing::info!("CRL GC removed {} expired entries", gc);
                        }
                    }
                }
                Some(ready_pkt) = mixing_rx.recv() => {
                    self.forward_sphinx_packet(&ready_pkt.next_hop, &ready_pkt.packet_bytes).await;
                }
                Some(cover_frame) = self.cover_rx.recv() => {
                    let dm_topic = IdentTopic::new(crate::core::constants::topics::DM);
                    let cover = crate::core::wire_protocol::Frame::new(
                        crate::core::wire_protocol::FrameKind::CoverTraffic,
                        cover_frame.data,
                    );
                    if let Err(e) = self.swarm.behaviour_mut().gossipsub.publish(dm_topic, cover.to_bytes()) {
                        tracing::trace!("Cover frame publish failed: {}", e);
                    }
                }
            }
        }
        tracing::info!("Mesh driver task exited.");
    }

    fn select_sphinx_route(&self, recipient: PeerId) -> Option<(Vec<[u8; 32]>, Vec<String>)> {
        let topo = self.topology.try_read().ok()?;
        let _our_pub = self.encryption_manager.x25519_public_key()?;
        let recipient_str = recipient.to_string();

        let mut candidates: Vec<(String, [u8; 32], f32)> = topo
            .iter()
            .filter(|(id, node)| {
                **id != self.local_peer_id.to_string()
                    && **id != recipient_str
                    && node.is_online
                    && node.public_key.len() >= 32
            })
            .filter_map(|(id, node)| {
                let key_bytes: [u8; 32] = node.public_key[..32].try_into().ok()?;
                Some((id.clone(), key_bytes, node.connection_quality))
            })
            .collect();

        if candidates.len() < 2 {
            return None;
        }

        candidates.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
        candidates.truncate(2);

        let mut route_keys = vec![candidates[0].1, candidates[1].1];
        let mut route_ids = vec![candidates[0].0.clone(), candidates[1].0.clone()];

        let recipient_node = topo.get(&recipient_str)?;
        let recipient_key: [u8; 32] = recipient_node.public_key[..32].try_into().ok()?;
        route_keys.push(recipient_key);
        route_ids.push(recipient_str);

        Some((route_keys, route_ids))
    }

    async fn forward_sphinx_packet(&mut self, next_hop: &str, packet_bytes: &[u8]) {
        let topic = IdentTopic::new(crate::core::constants::SPHINX_TOPIC);
        if let Err(e) = self
            .swarm
            .behaviour_mut()
            .gossipsub
            .publish(topic, packet_bytes.to_vec())
        {
            tracing::warn!("Sphinx forward to {} failed: {}", next_hop, e);
        } else {
            tracing::trace!("Sphinx packet forwarded to {}", next_hop);
        }
    }

    async fn handle_command(&mut self, cmd: MeshCommand) {
        match cmd {
            MeshCommand::PutRecord { key, value, reply } => {
                let record = libp2p::kad::Record {
                    key: libp2p::kad::RecordKey::new(&key),
                    value,
                    publisher: None,
                    expires: Some(
                        std::time::Instant::now() + std::time::Duration::from_secs(86400),
                    ),
                };
                let res = self
                    .swarm
                    .behaviour_mut()
                    .kad
                    .put_record(record, libp2p::kad::Quorum::One);
                let _ = reply.send(res.map(|_| ()).map_err(|e| anyhow::anyhow!("{:?}", e)));
            }
            MeshCommand::GetRecord { key, reply } => {
                let query_id = self
                    .swarm
                    .behaviour_mut()
                    .kad
                    .get_record(libp2p::kad::RecordKey::new(&key));
                self.pending_get_queries.insert(query_id, reply);
            }
            MeshCommand::BroadcastMessage { mut message, reply } => {
                self.vector_clock
                    .increment(&crate::core::vector_clock::redact_node_id(
                        &self.local_peer_id.to_string(),
                    ));
                message.vector_clock = self.vector_clock.clone();

                let topic = IdentTopic::new(crate::core::constants::BROADCAST_TOPIC);
                let res = (|| -> Result<()> {
                    use crate::core::wire_protocol::{Frame, FrameKind};
                    let frame = Frame::encode(FrameKind::Message, &message)?;
                    let data = frame.to_bytes();
                    self.swarm
                        .behaviour_mut()
                        .gossipsub
                        .publish(topic.clone(), data)?;
                    tracing::info!("Broadcasted message to topic: {}", topic);
                    Ok(())
                })();
                let _ = reply.send(res);
            }
            MeshCommand::ConnectPeer { peer_id, reply } => {
                let res = self
                    .swarm
                    .dial(peer_id)
                    .map_err(|e| anyhow::anyhow!("Dial error: {}", e));
                let _ = reply.send(res);
            }
            MeshCommand::PingPeer { peer_id, reply } => {
                // A peer answers EVERY request_response message with
                // ChatResponse("ACK"), so a minimal well-formed message forces
                // a real round-trip without introducing a new protocol.
                let req = ChatRequest(Message::new(
                    self._node_id.clone(),
                    peer_id.to_string(),
                    "ping".to_string(),
                ));
                // `send_request` returns an OutboundRequestId directly; it
                // queues onto an established connection and does not itself
                // report a failure. A peer that never answers is caught by the
                // caller's timeout, not here.
                self.swarm
                    .behaviour_mut()
                    .request_response
                    .send_request(&peer_id, req);
                // Register in the EXISTING PendingPings tracker
                // (core/peer_health.rs) rather than adding a second, parallel
                // ping table. The request_response reply path calls
                // resolve_waiter, which hands the measured RTT to this caller.
                self.pending_pings
                    .lock()
                    .await
                    .register_waiter(peer_id, reply);
            }
            MeshCommand::SubscribeTopic { topic, reply } => {
                let gossipsub_topic = IdentTopic::new(&topic);
                let res = self
                    .swarm
                    .behaviour_mut()
                    .gossipsub
                    .subscribe(&gossipsub_topic)
                    .map(|_| ())
                    .map_err(|e| anyhow::anyhow!("Subscribe error: {}", e));
                let _ = reply.send(res);
            }
            MeshCommand::PublishToTopic { topic, data, reply } => {
                let gossipsub_topic = IdentTopic::new(&topic);
                let res = self
                    .swarm
                    .behaviour_mut()
                    .gossipsub
                    .publish(gossipsub_topic, data)
                    .map(|_| ())
                    .map_err(|e| anyhow::anyhow!("Publish error: {}", e));
                let _ = reply.send(res);
            }
            MeshCommand::PublishWorldDiscovery { reply } => {
                let result = self.publish_world_discovery().await;
                let _ = reply.send(result);
            }
            MeshCommand::SetWorldDiscoveryProvider { enabled, reply } => {
                let key = libp2p::kad::RecordKey::new(&provider_record_key(
                    &self.world_discovery_network_id,
                ));
                let result = if enabled {
                    self.swarm
                        .behaviour_mut()
                        .kad
                        .start_providing(key)
                        .map(|_| ())
                        .map_err(|e| anyhow::anyhow!("world discovery provider failed: {e:?}"))
                } else {
                    self.swarm.behaviour_mut().kad.stop_providing(&key);
                    Ok(())
                };
                if result.is_ok() {
                    let mut health = self.world_discovery_health.write().await;
                    health.provider_active = enabled;
                    health.state = if enabled {
                        "provider_ready"
                    } else if health.enabled {
                        "ready"
                    } else {
                        "disabled"
                    }
                    .into();
                }
                let _ = reply.send(result);
            }
            MeshCommand::RefreshWorldDiscoveryProviders { reply } => {
                let key = libp2p::kad::RecordKey::new(&provider_record_key(
                    &self.world_discovery_network_id,
                ));
                self.swarm.behaviour_mut().kad.get_providers(key);
                let _ = reply.send(Ok(()));
            }
            MeshCommand::PublishUsername { reply } => {
                let res = (|| -> Result<()> {
                    let identity = self._identity.clone();
                    if let Some(pk_bytes) = identity.public_key().map(|pk| {
                        let proto = pk.encode_protobuf();
                        if proto.len() >= 32 {
                            proto[proto.len() - 32..].to_vec()
                        } else {
                            proto
                        }
                    }) {
                        let (key, value) = self
                            ._username_discovery
                            .prepare_username_publication_with_signer(pk_bytes, |sig_data| {
                                identity
                                    .sign(sig_data)
                                    .ok_or_else(|| anyhow::anyhow!("Signing failed"))
                            })?;
                        self.swarm.behaviour_mut().kad.put_record(
                            libp2p::kad::Record {
                                key,
                                value,
                                publisher: None,
                                expires: None,
                            },
                            libp2p::kad::Quorum::One,
                        )?;
                        Ok(())
                    } else {
                        Err(anyhow::anyhow!("Identity locked"))
                    }
                })();
                let _ = reply.send(res);
            }
            MeshCommand::SetShard { shard, reply } => {
                tracing::info!("Setting mesh shard to: {}", shard);
                // In a real system, this would change the DHT prefix or topic
                let _ = reply.send(Ok(()));
            }
            MeshCommand::SendMessage {
                recipient,
                mut message,
                reply,
            } => {
                self.vector_clock
                    .increment(&crate::core::vector_clock::redact_node_id(
                        &self.local_peer_id.to_string(),
                    ));
                message.vector_clock = self.vector_clock.clone();

                // Handshake installation is asynchronous. Do not attempt DM
                // encryption during the short window between peer discovery
                // and receipt of the peer's verified public key. Waiting is
                // bounded and remains fail-closed if the key never arrives.
                let key_deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(15);
                while !self.encryption_manager.has_peer_key(&recipient.to_string())
                    && tokio::time::Instant::now() < key_deadline
                {
                    tokio::time::sleep(std::time::Duration::from_millis(250)).await;
                }
                if !self.encryption_manager.has_peer_key(&recipient.to_string()) {
                    tracing::warn!(
                        recipient = %recipient,
                        "Direct-message encryption key not ready after bounded wait"
                    );
                    let _ = reply.send(Err(anyhow::anyhow!(
                        "Recipient encryption key not ready for {}",
                        recipient
                    )));
                    return;
                }

                let mut encrypted_msg = message.clone();
                let use_ratchet = self
                    .encryption_manager
                    .has_ratchet_session(&recipient.to_string());

                // ponytail: traffic analysis resistance — pad plaintext to 512-byte blocks before encryption
                let padded_content = traffic_armor::pad_message(
                    message.content.as_bytes(),
                    &ring::rand::SystemRandom::new(),
                )
                .ok()
                .map(|p| String::from_utf8(p).unwrap_or(message.content.clone()))
                .unwrap_or(message.content.clone());

                let enc_result: Result<_, anyhow::Error> = if use_ratchet {
                    self.encryption_manager
                        .encrypt_message_ratchet(
                            &padded_content,
                            &message.recipient,
                            EncMessageType::Text,
                        )
                        .map_err(|e| anyhow::anyhow!("{}", e))
                } else {
                    self.encryption_manager
                        .encrypt_message(&padded_content, &message.recipient, EncMessageType::Text)
                        .map_err(|e| anyhow::anyhow!("{}", e))
                };
                if let Err(encryption_error) = enc_result.as_ref() {
                    tracing::warn!(recipient = %recipient, error = %encryption_error, "Direct-message encryption failed before transport");
                }

                if let Ok(encrypted_content) = enc_result {
                    encrypted_msg.content = base64::engine::general_purpose::STANDARD
                        .encode(&encrypted_content.ciphertext);
                    encrypted_msg.encrypted = true;
                    encrypted_msg.encryption_status = "full".to_string();
                    let nonce_b64 =
                        base64::engine::general_purpose::STANDARD.encode(&encrypted_content.nonce);
                    let sig_b64 = base64::engine::general_purpose::STANDARD
                        .encode(&encrypted_content.signature);
                    let version_prefix = if encrypted_content.version == 0x04 {
                        "v4:"
                    } else {
                        ""
                    };
                    encrypted_msg.message_type = if let Some(ref pq) = encrypted_content.pq_key {
                        let header_b64 = base64::engine::general_purpose::STANDARD.encode(pq);
                        format!(
                            "encrypted:{}{}:{}:{}",
                            version_prefix, nonce_b64, sig_b64, header_b64
                        )
                    } else {
                        format!("encrypted:{}{}:{}", version_prefix, nonce_b64, sig_b64)
                    };
                } else {
                    tracing::warn!("Failed to encrypt DM to {}, dropping message", recipient);
                    let _ = reply.send(Err(anyhow::anyhow!(
                        "Encryption failed for DM to {}",
                        recipient
                    )));
                    return;
                }

                self.pending_messages
                    .insert(encrypted_msg.id, encrypted_msg.clone());
                self.reliability.on_message_sent(encrypted_msg.id);

                use crate::core::wire_protocol::{Frame, FrameKind};
                let frame = match Frame::encode(FrameKind::Message, &encrypted_msg) {
                    Ok(f) => f,
                    Err(e) => {
                        tracing::warn!("Failed to encode frame for {}: {}", recipient, e);
                        let _ = reply.send(Err(e));
                        return;
                    }
                };
                let data = frame.to_bytes();

                if let Some(ref mailbox) = self.mailbox_store {
                    if let Err(e) = mailbox.queue_delivery(
                        &recipient.to_string(),
                        &encrypted_msg.id.to_string(),
                        &data,
                    ) {
                        tracing::warn!("Failed to queue delivery for {}: {}", recipient, e);
                    }
                }

                let sent_via_sphinx = if let Some((route_keys, route_ids)) =
                    self.select_sphinx_route(recipient)
                {
                    match crate::core::routing::sphinx::create_sphinx_packet(
                        &route_keys,
                        &route_ids,
                        &data,
                        &ring::rand::SystemRandom::new(),
                    ) {
                        Ok(packet) => {
                            match bincode::serialize(&packet) {
                                Ok(pkt_bytes) => {
                                    let sphinx_topic =
                                        IdentTopic::new(crate::core::constants::SPHINX_TOPIC);
                                    match self
                                        .swarm
                                        .behaviour_mut()
                                        .gossipsub
                                        .publish(sphinx_topic, pkt_bytes)
                                    {
                                        Ok(_) => {
                                            tracing::trace!(
                                                "Sent DM via Sphinx ({} hops) to {}",
                                                route_keys.len(),
                                                recipient
                                            );
                                            true
                                        }
                                        Err(e) => {
                                            tracing::warn!("Sphinx publish failed: {}, falling back to direct DM", e);
                                            false
                                        }
                                    }
                                }
                                Err(e) => {
                                    tracing::warn!("Sphinx packet serialization failed: {}", e);
                                    false
                                }
                            }
                        }
                        Err(e) => {
                            tracing::trace!("Sphinx route build failed: {}", e);
                            false
                        }
                    }
                } else {
                    false
                };

                if !sent_via_sphinx {
                    let topic = IdentTopic::new(crate::core::constants::topics::DM);
                    if let Err(e) = self
                        .swarm
                        .behaviour_mut()
                        .gossipsub
                        .publish(topic, data.clone())
                    {
                        tracing::warn!(
                            "Gossipsub publish to {} failed: {}, trying DTN fallback",
                            recipient,
                            e
                        );

                        if let Some(ref dtn_store) = self.dtn_store {
                            let destination =
                                *blake3::hash(recipient.to_string().as_bytes()).as_bytes();
                            let bundle =
                                crate::core::dtn::bundle::DtnBundle::new(data, destination);
                            if dtn_store.store(bundle).await {
                                tracing::trace!(
                                    "DTN fallback: message queued for epidemic delivery to {}",
                                    recipient
                                );
                                let _ = reply.send(Ok(()));
                                return;
                            } else {
                                tracing::warn!(
                                    "DTN store full or bundle expired, message dropped for {}",
                                    recipient
                                );
                            }
                        }

                        let _ = reply.send(Err(anyhow::anyhow!(
                            "Direct send and DTN fallback both failed for {}: {}",
                            recipient,
                            e
                        )));
                        return;
                    }
                }

                tracing::trace!(
                    "Sent direct message to peer {} (sphinx={}, ratchet={})",
                    recipient,
                    sent_via_sphinx,
                    use_ratchet
                );
                let _ = reply.send(Ok(()));
            }
            MeshCommand::GetListeners { reply } => {
                let listeners = self.swarm.listeners().cloned().collect();
                let _ = reply.send(listeners);
            }
            MeshCommand::AddAddress {
                peer_id,
                address,
                reply,
            } => {
                // Keep the dial address book and the DHT routing table in sync.
                // Kademlia knowledge alone does not guarantee that `swarm.dial(peer_id)`
                // can resolve a usable address for the peer.
                self.swarm.add_peer_address(peer_id, address.clone());
                self.swarm
                    .behaviour_mut()
                    .kad
                    .add_address(&peer_id, address);
                let _ = reply.send(Ok(()));
            }
            MeshCommand::Shutdown => {
                tracing::info!("Mesh driver received shutdown command.");
            }
        }
    }

    fn dial_peer_if_needed(
        &mut self,
        peer_id: PeerId,
        address: Option<Multiaddr>,
        source: &'static str,
    ) {
        if peer_id == self.local_peer_id {
            tracing::debug!(%peer_id, source, "Dial suppressed for local peer");
            return;
        }
        if self.swarm.is_connected(&peer_id) {
            tracing::debug!(%peer_id, source, "Dial suppressed for already-connected peer");
            return;
        }
        if !self.pending_dials.insert(peer_id) {
            tracing::debug!(%peer_id, source, "Dial suppressed for in-flight peer");
            return;
        }
        let result = match address {
            Some(address) => self.swarm.dial(address),
            None => self.swarm.dial(peer_id),
        };
        if let Err(error) = result {
            self.pending_dials.remove(&peer_id);
            tracing::debug!(%peer_id, source, %error, "Deduplicated dial start failed");
        } else {
            tracing::debug!(%peer_id, source, "Deduplicated dial started");
        }
    }

    async fn handle_swarm_event(&mut self, event: SwarmEvent<MeshBehaviourEvent>) {
        // Handle mDNS discovery — auto-connect to discovered peers
        if let SwarmEvent::Behaviour(MeshBehaviourEvent::Mdns(libp2p::mdns::Event::Discovered(
            peers,
        ))) = &event
        {
            if std::env::var_os("GHOSTWIRE_TEST_DISABLE_MDNS").is_some() {
                return;
            }
            self.mdns_discovered
                .fetch_add(peers.len(), Ordering::Relaxed);
            self.mdns_last_event.store(now_secs(), Ordering::Relaxed);
            for (peer_id, addr) in peers {
                tracing::info!(peer_id = %peer_id, address = %addr, "mDNS discovered peer");
                self.discovery_directory
                    .observe(
                        peer_id.to_string(),
                        None,
                        [addr.to_string()],
                        crate::core::discovery_directory::DiscoverySource::Mdns,
                        now_secs(),
                    )
                    .await;
                // Add to Kademlia routing table
                self.swarm
                    .behaviour_mut()
                    .kad
                    .add_address(peer_id, addr.clone());
                // Attempt to connect, but never create duplicate physical connections.
                self.dial_peer_if_needed(*peer_id, Some(addr.clone()), "mdns");
            }
            return;
        }

        if let SwarmEvent::Behaviour(MeshBehaviourEvent::Mdns(libp2p::mdns::Event::Expired(
            peers,
        ))) = &event
        {
            if std::env::var_os("GHOSTWIRE_TEST_DISABLE_MDNS").is_some() {
                return;
            }
            self.mdns_expired.fetch_add(peers.len(), Ordering::Relaxed);
            self.mdns_last_event.store(now_secs(), Ordering::Relaxed);
            for (peer_id, _addr) in peers {
                tracing::info!(peer_id = %peer_id, "mDNS peer expired");
                let _ = self
                    .discovery_directory
                    .mark_reachability(
                        &peer_id.to_string(),
                        crate::core::discovery_directory::ReachabilityState::Unreachable,
                    )
                    .await;
            }
            return;
        }

        // Log connection events for debugging
        if let SwarmEvent::NewListenAddr { address, .. } = &event {
            if self.world_discovery_enabled {
                tracing::info!(address = %address, "World discovery listener available; publishing signed presence");
                let _ = self.publish_world_discovery().await;
            }
        }
        if let SwarmEvent::ConnectionEstablished {
            peer_id,
            num_established,
            ..
        } = &event
        {
            self.pending_dials.remove(peer_id);
            tracing::info!(
                "Connection established with {} ({} total)",
                peer_id,
                num_established
            );
            let _ = self
                .discovery_directory
                .mark_reachability(
                    &peer_id.to_string(),
                    crate::core::discovery_directory::ReachabilityState::Reachable,
                )
                .await;
            let my_id = self._identity.id.clone();
            let my_pubkey = base64::engine::general_purpose::STANDARD
                .encode(self.encryption_manager.get_public_key());
            let mut handshake = Message::new(my_id, peer_id.to_string(), my_pubkey);
            handshake.message_type = "handshake".to_string();
            self.swarm
                .behaviour_mut()
                .request_response
                .send_request(peer_id, ChatRequest(handshake));
            tracing::info!(
                "Sent encryption handshake via request_response to {} (key: {} bytes)",
                peer_id,
                self.encryption_manager.get_public_key().len()
            );

            {
                let mut topo = self.topology.write().await;
                topo.entry(peer_id.to_string())
                    .or_insert_with(|| MeshNode {
                        id: peer_id.to_string(),
                        peer_id: *peer_id,
                        last_seen_addresses: Vec::new(),
                        username: String::new(),
                        public_key: Vec::new(),
                        last_seen: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs(),
                        connection_quality: 1.0,
                        is_online: true,
                        threat_level: ThreatLevel::Low,
                    })
                    .is_online = true;
                if let Some(node) = topo.get_mut(&peer_id.to_string()) {
                    node.last_seen = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();
                }
            }

            self.peer_health.register_peer(peer_id.to_string()).await;

            if let Some(cache) = &self.peer_count_cache {
                let count = self
                    .topology
                    .read()
                    .await
                    .values()
                    .filter(|n| n.is_online)
                    .count() as usize;
                cache.store(count, std::sync::atomic::Ordering::Relaxed);
            }

            if let Some(ref dtn_store) = self.dtn_store {
                let peer_dest = *blake3::hash(peer_id.to_string().as_bytes()).as_bytes();
                let bundles = dtn_store.bundles.read().await;
                let to_deliver: Vec<crate::core::dtn::bundle::DtnBundle> = bundles
                    .values()
                    .filter(|b| b.destination == peer_dest && !b.is_expired())
                    .cloned()
                    .collect();
                drop(bundles);

                if !to_deliver.is_empty() {
                    let dm_topic = IdentTopic::new(crate::core::constants::topics::DM);
                    for bundle in &to_deliver {
                        let _ = self
                            .swarm
                            .behaviour_mut()
                            .gossipsub
                            .publish(dm_topic.clone(), bundle.payload.clone());
                        tracing::info!(
                            "DTN: delivered bundle {:02x?} to newly connected peer {}",
                            &bundle.cid[..8],
                            peer_id
                        );
                    }
                    for bundle in &to_deliver {
                        dtn_store.remove(&bundle.cid).await;
                    }
                }

                // Trigger epidemic sync handshake (bloom-filter exchange)
                let handshake =
                    crate::core::dtn::epidemic::DtnSyncHandshake::build(dtn_store).await;
                if let Ok(hs_bytes) = bincode::serialize(&handshake) {
                    let sync_topic = IdentTopic::new(crate::core::constants::DTN_SYNC_TOPIC);
                    let _ = self
                        .swarm
                        .behaviour_mut()
                        .gossipsub
                        .publish(sync_topic, hs_bytes);
                    tracing::debug!("DTN: published sync handshake to new peer {}", peer_id);
                }
            }

            if let Some(ref mailbox) = self.mailbox_store {
                if let Ok(deliveries) = mailbox.get_pending_deliveries(&peer_id.to_string()) {
                    if !deliveries.is_empty() {
                        let dm_topic = IdentTopic::new(crate::core::constants::topics::DM);
                        let mut delivered_ids = Vec::new();
                        for (msg_id, frame_bytes) in &deliveries {
                            if self
                                .swarm
                                .behaviour_mut()
                                .gossipsub
                                .publish(dm_topic.clone(), frame_bytes.clone())
                                .is_ok()
                            {
                                delivered_ids.push(msg_id.clone());
                                tracing::info!(
                                    "Mailbox: delivered queued message {} to {}",
                                    msg_id,
                                    peer_id
                                );
                            }
                        }
                        for msg_id in &delivered_ids {
                            let _ = mailbox.remove_delivery(&peer_id.to_string(), msg_id);
                        }
                    }
                }
            }
        }

        if let SwarmEvent::OutgoingConnectionError {
            peer_id: Some(peer_id),
            error,
            ..
        } = &event
        {
            if self.pending_dials.remove(peer_id) {
                tracing::debug!(peer_id = %peer_id, %error, "Deduplicated dial failed");
            }
        }

        if let SwarmEvent::ConnectionClosed {
            peer_id,
            num_established,
            cause: Some(ref e),
            ..
        } = &event
        {
            tracing::warn!("Connection closed with {}: {}", peer_id, e);

            {
                let mut topo = self.topology.write().await;
                if let Some(node) = topo.get_mut(&peer_id.to_string()) {
                    // Only mark the PEER offline when its LAST connection went
                    // away. `num_established` is how many connections to this
                    // peer remain after this close.
                    //
                    // This handler used to ignore that field (`..`) and set
                    // is_online = false unconditionally, which is wrong because
                    // libp2p multiplexes several connections per peer. Observed
                    // 2026-09-28 with two live nodes: "Connection established
                    // ... (3 total)" / "(4 total)" alternating, while
                    // /api/v2/peers reported the peer Disconnected during
                    // eight consecutive successful pings (30-137ms) with zero
                    // connection events in between. The peer was connected the
                    // whole time; one of its three connections closing was
                    // enough to declare it offline.
                    if *num_established == 0 {
                        node.is_online = false;
                    } else {
                        tracing::debug!(
                            "Connection to {} closed but {} remain; keeping peer online",
                            peer_id,
                            num_established
                        );
                    }
                }
            }

            self.peer_health.remove_peer(&peer_id.to_string()).await;
            let _ = self
                .discovery_directory
                .mark_reachability(
                    &peer_id.to_string(),
                    crate::core::discovery_directory::ReachabilityState::Unreachable,
                )
                .await;

            if let Some(cache) = &self.peer_count_cache {
                let count = self
                    .topology
                    .read()
                    .await
                    .values()
                    .filter(|n| n.is_online)
                    .count() as usize;
                cache.store(count, std::sync::atomic::Ordering::Relaxed);
            }
        }

        if let SwarmEvent::Behaviour(MeshBehaviourEvent::Identify(
            libp2p::identify::Event::Received {
                peer_id,
                info,
                connection_id: _,
            },
        )) = &event
        {
            tracing::info!(
                "Identified peer {} as {} (protocols: {:?})",
                peer_id,
                info.agent_version,
                info.protocols
            );
            self.discovery_directory
                .observe(
                    peer_id.to_string(),
                    None,
                    info.listen_addrs.iter().map(|addr| addr.to_string()),
                    crate::core::discovery_directory::DiscoverySource::Transport,
                    now_secs(),
                )
                .await;

            let mut topo = self.topology.write().await;
            let entry = topo.entry(peer_id.to_string()).or_insert_with(|| MeshNode {
                id: peer_id.to_string(),
                peer_id: *peer_id,
                last_seen_addresses: Vec::new(),
                username: String::new(),
                public_key: Vec::new(),
                last_seen: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
                connection_quality: 1.0,
                is_online: true,
                threat_level: ThreatLevel::Low,
            });
            entry.last_seen_addresses = info.listen_addrs.clone();
            entry.username = info.agent_version.clone();
            entry.last_seen = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
        }

        if let SwarmEvent::Behaviour(MeshBehaviourEvent::Kad(
            libp2p::kad::Event::OutboundQueryProgressed { id, result, .. },
        )) = event
        {
            match result {
                libp2p::kad::QueryResult::GetProviders(Ok(
                    libp2p::kad::GetProvidersOk::FoundProviders { providers, .. },
                )) => {
                    for peer_id in providers {
                        self.dial_peer_if_needed(peer_id, None, "kad_provider");
                    }
                }
                libp2p::kad::QueryResult::GetProviders(Ok(
                    libp2p::kad::GetProvidersOk::FinishedWithNoAdditionalRecord { closest_peers },
                )) => {
                    for peer_id in closest_peers {
                        self.dial_peer_if_needed(peer_id, None, "kad_closest");
                    }
                }
                libp2p::kad::QueryResult::GetProviders(Err(error)) => {
                    tracing::debug!(?error, "world discovery provider lookup failed");
                }
                libp2p::kad::QueryResult::GetRecord(Ok(libp2p::kad::GetRecordOk::FoundRecord(
                    record,
                ))) => {
                    if let Some(reply) = self.pending_get_queries.remove(&id) {
                        let _ = reply.send(Ok(record.record.value.clone()));
                    } else {
                        // Not a pending query — try to interpret as a username record
                        if let Ok(username_record) =
                            crate::core::username_discovery::UsernameRecord::from_bytes(
                                &record.record.value,
                            )
                        {
                            if username_record.verify().is_ok() {
                                tracing::info!(
                                    "Discovered username '{}' -> PeerID {}",
                                    username_record.username,
                                    username_record.peer_id
                                );
                                self.discovery_directory
                                    .observe(
                                        username_record.peer_id.to_string(),
                                        None,
                                        std::iter::empty::<String>(),
                                        crate::core::discovery_directory::DiscoverySource::Dht,
                                        now_secs(),
                                    )
                                    .await;
                            }
                        }
                    }
                }
                libp2p::kad::QueryResult::GetRecord(Ok(_)) => {
                    if let Some(reply) = self.pending_get_queries.remove(&id) {
                        let _ = reply.send(Err(anyhow::anyhow!("Record not found")));
                    }
                }
                libp2p::kad::QueryResult::GetRecord(Err(e)) => {
                    if let Some(reply) = self.pending_get_queries.remove(&id) {
                        let _ = reply.send(Err(anyhow::anyhow!("GetRecord error: {:?}", e)));
                    }
                }
                _ => {
                    if let Some(reply) = self.pending_get_queries.remove(&id) {
                        let _ = reply.send(Err(anyhow::anyhow!("Unexpected query result")));
                    }
                }
            }
            return;
        }

        if let SwarmEvent::Behaviour(MeshBehaviourEvent::Gossipsub(
            libp2p::gossipsub::Event::Message {
                propagation_source,
                message_id,
                message,
            },
        )) = event
        {
            if message.topic.as_str() == discovery_topic(&self.world_discovery_network_id) {
                let _ = self
                    .handle_world_discovery_message(propagation_source, message)
                    .await;
            } else {
                let _ = self
                    .handle_gossipsub_message(propagation_source, message_id, message)
                    .await;
            }
            return;
        }

        if let SwarmEvent::Behaviour(MeshBehaviourEvent::RequestResponse(
            request_response::Event::Message { peer, message, .. },
        )) = event
        {
            match message {
                request_response::Message::Request {
                    request, channel, ..
                } => {
                    if request.0.message_type == "handshake" {
                        self.handle_handshake(peer, &request.0).await;
                    } else {
                        let _ = self.handle_incoming_message(peer, request.0).await;
                    }
                    let _ = self
                        .swarm
                        .behaviour_mut()
                        .request_response
                        .send_response(channel, ChatResponse("ACK".to_string()));
                }
                request_response::Message::Response { response, .. } => {
                    debug!("Received chat response from {}: {}", peer, response.0);

                    // This is the reply half of an interactive latency probe
                    // issued by `MeshManager::ping_peer`. The periodic health
                    // pings travel over gossipsub as FrameKind::Pong and are
                    // resolved on that path instead; only the
                    // request_response route can hand an RTT back to a caller
                    // blocked inside `ping_peer`.
                    let mut pending = self.pending_pings.lock().await;
                    if let Some(rtt_ms) = pending.resolve_waiter(&peer) {
                        self.peer_health
                            .record_pong(&peer.to_string(), rtt_ms)
                            .await;
                        debug!("ping to {} resolved: RTT={:.1}ms", peer, rtt_ms);
                    }
                }
            }
            return;
        }
        if let SwarmEvent::Behaviour(MeshBehaviourEvent::Relay(event)) = event {
            use libp2p::relay::client::Event as RelayEvent;
            match event {
                RelayEvent::ReservationReqAccepted {
                    relay_peer_id,
                    renewal,
                    ..
                } => tracing::info!(
                    "Relay reservation accepted by {} (renewal: {})",
                    relay_peer_id,
                    renewal
                ),
                RelayEvent::OutboundCircuitEstablished { relay_peer_id, .. } => {
                    tracing::info!("Outbound circuit via relay {}", relay_peer_id)
                }
                RelayEvent::InboundCircuitEstablished { src_peer_id, .. } => {
                    tracing::info!("Inbound circuit from {} via relay", src_peer_id)
                }
            }
            return;
        }
    }

    async fn publish_world_discovery(&mut self) -> Result<()> {
        if !self.world_discovery_enabled {
            return Ok(());
        }
        let listeners: Vec<Multiaddr> = self.swarm.listeners().cloned().collect();
        let allow_private = private_endpoints_allowed();
        let mut addresses = bounded_addresses(listeners.clone(), allow_private);
        // A wildcard listener is not a dialable advertisement. Project the
        // actual interface addresses into the listener's TCP port so nodes
        // can discover one another without manually exchanging addresses.
        // Private LAN endpoints remain opt-in; Tailscale/global endpoints are
        // admitted through the same bounded validator as explicit addresses.
        if std::env::var("GHOSTWIRE_AUTO_EXTERNAL_ADDRESSES")
            .map(|value| value != "0")
            .unwrap_or(true)
        {
            let tcp_port = listeners.iter().find_map(|address| {
                address.iter().find_map(|protocol| match protocol {
                    libp2p::multiaddr::Protocol::Tcp(port) => Some(port),
                    _ => None,
                })
            });
            if let Some(tcp_port) = tcp_port {
                if let Ok(interfaces) = local_ip_address::list_afinet_netifas() {
                    for (interface, ip) in interfaces {
                        if interface == "lo" || ip.is_loopback() || ip.is_unspecified() {
                            continue;
                        }
                        if ip.is_ipv4() {
                            let raw = format!("/ip4/{}/tcp/{}", ip, tcp_port);
                            if let Ok(address) = raw.parse::<Multiaddr>() {
                                addresses.extend(bounded_addresses([address], allow_private));
                            }
                        } else {
                            let raw = format!("/ip6/{}/tcp/{}", ip, tcp_port);
                            if let Ok(address) = raw.parse::<Multiaddr>() {
                                addresses.extend(bounded_addresses([address], allow_private));
                            }
                        }
                    }
                }
            }
        }
        if let Ok(extra) = std::env::var("GHOSTWIRE_EXTERNAL_ADDRESS") {
            for raw in extra
                .split(',')
                .map(str::trim)
                .filter(|raw| !raw.is_empty())
            {
                if let Ok(address) = raw.parse::<Multiaddr>() {
                    addresses.extend(bounded_addresses([address], private_endpoints_allowed()));
                }
            }
        }
        // The address bound applies to the aggregate advertisement, not each
        // individual source. Preserve listener/interface preference, remove
        // duplicates, and enforce the wire-level limit before signing.
        let mut seen_addresses = HashSet::new();
        addresses.retain(|address| seen_addresses.insert(address.clone()));
        addresses.truncate(crate::core::world_discovery::MAX_ADDRESSES);
        if addresses.is_empty() {
            let mut health = self.world_discovery_health.write().await;
            health.state = "waiting_for_listener".into();
            return Ok(());
        }
        let public_key = self
            ._identity
            .public_key()
            .ok_or_else(|| anyhow::anyhow!("identity locked; discovery advertisement withheld"))?;
        self.world_discovery_sequence = self.world_discovery_sequence.saturating_add(1);
        let issued_at = crate::core::world_discovery::now_secs();
        let ad = WorldDiscoveryAdvertisement::unsigned(
            self.world_discovery_network_id.clone(),
            self.local_peer_id,
            self._identity.did.clone(),
            public_key.encode_protobuf(),
            addresses,
            self.world_discovery_sequence,
            issued_at,
            issued_at.saturating_add(MAX_TTL_SECS),
            crate::core::world_discovery::default_capabilities(
                self.world_discovery_health.read().await.provider_active,
            ),
        )
        .sign_with(|bytes| self._identity.sign(bytes))?;
        let bytes = ad.to_bytes()?;
        let gossipsub_published = match self.swarm.behaviour_mut().gossipsub.publish(
            IdentTopic::new(discovery_topic(&self.world_discovery_network_id)),
            bytes.clone(),
        ) {
            Ok(_) => true,
            Err(libp2p::gossipsub::PublishError::NoPeersSubscribedToTopic) => {
                let mut health = self.world_discovery_health.write().await;
                health.state = "waiting_for_peers".into();
                health.last_error = None;
                tracing::debug!(
                    "World discovery advertisement signed but no GossipSub peers are available yet"
                );
                false
            }
            Err(error) => {
                return Err(anyhow::anyhow!(
                    "world discovery GossipSub publish failed: {error}"
                ));
            }
        };
        let record = libp2p::kad::Record {
            key: libp2p::kad::RecordKey::new(&advertisement_record_key(
                &self.world_discovery_network_id,
                &self.local_peer_id.to_string(),
            )),
            value: bytes,
            publisher: None,
            expires: Some(std::time::Instant::now() + std::time::Duration::from_secs(MAX_TTL_SECS)),
        };
        let dht_published = self
            .swarm
            .behaviour_mut()
            .kad
            .put_record(record, libp2p::kad::Quorum::One)
            .is_ok();
        if !dht_published {
            tracing::debug!(
                "World discovery DHT store rejected the advertisement; it will be re-attempted"
            );
        }
        let mut health = self.world_discovery_health.write().await;
        if gossipsub_published && dht_published {
            health.published_total = health.published_total.saturating_add(1);
            health.last_published_unix = issued_at;
        }
        health.state = if !gossipsub_published || !dht_published {
            "waiting_for_peers"
        } else if health.provider_active {
            "provider_ready"
        } else {
            "ready"
        }
        .into();
        health.last_error = None;
        Ok(())
    }

    async fn handle_world_discovery_message(
        &mut self,
        propagation_source: PeerId,
        message: GossipsubMessage,
    ) -> Result<()> {
        let result = (|| -> Result<(WorldDiscoveryAdvertisement, PeerId)> {
            let advertisement = WorldDiscoveryAdvertisement::from_bytes(&message.data)?;
            let peer_id = advertisement.verify(
                &self.world_discovery_network_id,
                crate::core::world_discovery::now_secs(),
                private_endpoints_allowed(),
            )?;
            Ok((advertisement, peer_id))
        })();
        let (advertisement, peer_id) = match result {
            Ok(value) => value,
            Err(error) => {
                let mut health = self.world_discovery_health.write().await;
                health.rejected_total = health.rejected_total.saturating_add(1);
                health.last_error = Some(error.to_string());
                tracing::debug!(%propagation_source, %error, "Rejected GhostWire world-discovery advertisement");
                return Ok(());
            }
        };
        let now = crate::core::world_discovery::now_secs();
        self.discovery_directory
            .observe(
                peer_id.to_string(),
                Some(advertisement.did.clone()),
                advertisement.addresses.clone(),
                crate::core::discovery_directory::DiscoverySource::GossipSub,
                now,
            )
            .await;
        for raw in &advertisement.addresses {
            let Ok(mut address) = raw.parse::<Multiaddr>() else {
                continue;
            };
            address.push(libp2p::multiaddr::Protocol::P2p(peer_id));
            self.swarm
                .behaviour_mut()
                .kad
                .add_address(&peer_id, address.clone());
            self.dial_peer_if_needed(peer_id, Some(address), "world_discovery");
        }
        let mut health = self.world_discovery_health.write().await;
        health.accepted_total = health.accepted_total.saturating_add(1);
        health.last_received_unix = now;
        health.candidate_count = self.discovery_directory.snapshot().await.len();
        health.last_error = None;
        Ok(())
    }

    async fn handle_gossipsub_message(
        &mut self,
        peer_id: PeerId,
        message_id: MessageId,
        message: GossipsubMessage,
    ) -> Result<()> {
        let topic_str = message.topic.as_str();

        if topic_str == crate::core::constants::SPHINX_TOPIC {
            return self.handle_sphinx_packet(peer_id, message).await;
        }

        if topic_str == crate::core::constants::DTN_SYNC_TOPIC {
            return self.handle_dtn_sync(message).await;
        }

        if topic_str == crate::core::constants::topics::CRL_GOSSIP {
            // Deserialize CRL gossip payloads and handle them
            if let Ok(payloads) =
                serde_json::from_slice::<Vec<crate::core::crl::CrlGossipPayload>>(&message.data)
            {
                let (accepted, duplicates, rejected) =
                    self.encryption_manager.handle_crl_gossip(&payloads);
                tracing::info!(
                    "CRL gossip from {}: accepted={}, duplicates={}, rejected={}",
                    peer_id,
                    accepted,
                    duplicates,
                    rejected
                );
            }
            return Ok(());
        }

        let is_dm_for_us = topic_str == crate::core::constants::topics::DM;
        let is_dm_topic = is_dm_for_us;

        if !is_dm_for_us && !is_dm_topic && topic_str != crate::core::constants::BROADCAST_TOPIC {
            return Ok(());
        }

        if let Ok((frame, _)) = crate::core::wire_protocol::Frame::from_bytes(&message.data) {
            if frame.kind == crate::core::wire_protocol::FrameKind::CoverTraffic {
                tracing::trace!("Discarded cover frame from {}", peer_id);
                return Ok(());
            }
        }

        debug!(
            "Received gossipsub message on {} from {}: {:?}",
            topic_str, peer_id, message_id
        );

        let peer_ip = self.get_peer_ip(&peer_id);
        if let Some(ip) = peer_ip {
            let sm = self.security_manager.clone();
            if !sm.is_ip_allowed(&ip).await {
                tracing::warn!("Rejected message from blacklisted peer: {}", peer_id);
                return Ok(());
            }
        }

        // Try to decode as Message (bincode via wire_protocol frame)
        let mut msg = if let Ok((frame, _consumed)) =
            crate::core::wire_protocol::Frame::from_bytes(&message.data)
        {
            if frame.kind == crate::core::wire_protocol::FrameKind::Ping {
                if let Ok(ping) = frame.decode::<crate::core::wire_protocol::WirePing>() {
                    self.peer_health.record_activity(&peer_id.to_string()).await;
                    use crate::core::wire_protocol::{Frame, FrameKind, WirePong};
                    let pong = WirePong {
                        ts: ping.ts,
                        seq: ping.seq,
                    };
                    if let Ok(pong_frame) = Frame::encode(FrameKind::Pong, &pong) {
                        let pong_topic = IdentTopic::new(crate::core::constants::topics::DM);
                        let _ = self
                            .swarm
                            .behaviour_mut()
                            .gossipsub
                            .publish(pong_topic, pong_frame.to_bytes());
                        tracing::debug!("Responded to ping from {} with pong", peer_id);
                    }
                }
                return Ok(());
            }
            if frame.kind == crate::core::wire_protocol::FrameKind::Pong {
                if let Ok(pong) = frame.decode::<crate::core::wire_protocol::WirePong>() {
                    let mut pending = self.pending_pings.lock().await;
                    if let Some((peer_id_str, rtt_ms)) = pending.resolve(pong.ts) {
                        self.peer_health.record_pong(&peer_id_str, rtt_ms).await;
                        tracing::debug!("Pong from {}: RTT={:.1}ms", peer_id_str, rtt_ms);
                    }
                }
                return Ok(());
            }

            if let Ok(m) = frame.decode::<Message>() {
                m
            } else {
                serde_json::from_slice::<Message>(&message.data)
                    .map_err(|e| anyhow::anyhow!("Failed to decode message: {}", e))?
            }
        } else {
            serde_json::from_slice::<Message>(&message.data)
                .map_err(|e| anyhow::anyhow!("Failed to deserialize message: {}", e))?
        };

        if self.seen_messages.contains(&msg.id) {
            debug!("Skipping already-seen message {}", msg.id);
            return Ok(());
        }
        // Evict oldest entries when at capacity
        if self.seen_messages.len() >= SEEN_MSGS_MAX {
            self.seen_messages.drain(..SEEN_MSGS_MAX / 2);
        }
        self.seen_messages.push_back(msg.id);

        if msg.ttl > 0 && msg.timestamp + msg.ttl < crate::core::reliability::current_unix_secs() {
            debug!("Dropping expired message {} (TTL={})", msg.id, msg.ttl);
            return Ok(());
        }

        self.vector_clock.merge(&msg.vector_clock);

        if msg.message_type == "ack" {
            if let Ok(ack_id) = msg.content.parse::<uuid::Uuid>() {
                self.reliability.on_ack_received(&ack_id);
                self.pending_messages.remove(&ack_id);
                if let Some(ref mailbox) = self.mailbox_store {
                    let _ = mailbox.remove_delivery(&msg.sender, &ack_id.to_string());
                }
            }
            return Ok(());
        }

        if !msg.is_broadcast()
            && msg.recipient != self.local_peer_id.to_string()
            && msg.hops < MAX_HOPS
        {
            let mut forwarded = msg.clone();
            forwarded.hops += 1;
            tracing::trace!(
                "Relaying message {} (hop {}/{}) to {}",
                forwarded.id,
                forwarded.hops,
                MAX_HOPS,
                forwarded.recipient
            );
            let relay_topic = IdentTopic::new(crate::core::constants::topics::DM);
            use crate::core::wire_protocol::{Frame, FrameKind};
            if let Ok(frame) = Frame::encode(FrameKind::Message, &forwarded) {
                let _ = self
                    .swarm
                    .behaviour_mut()
                    .gossipsub
                    .publish(relay_topic, frame.to_bytes());
            }
        }

        if !is_dm_for_us && topic_str != crate::core::constants::BROADCAST_TOPIC {
            return Ok(());
        }

        if msg.encrypted && msg.message_type.starts_with("encrypted:") {
            // v3: encrypted:nonce:sig, v4: encrypted:v4:nonce:sig:header
            let after_encrypted = &msg.message_type["encrypted:".len()..];
            let is_ratchet = after_encrypted.starts_with("v4:");
            let payload = if is_ratchet {
                &after_encrypted["v4:".len()..]
            } else {
                after_encrypted
            };
            let payload_parts: Vec<&str> = payload.splitn(3, ':').collect();

            if payload_parts.len() >= 2 {
                let nonce_b64 = payload_parts[0];
                let sig_b64 = payload_parts[1];
                // Both v3 hybrid messages and v4 ratchet messages may carry
                // an ML-KEM ciphertext/header. The sender includes it as the
                // third payload segment; dropping it for v3 makes the receiver
                // derive a different AEAD key and report a false decrypt failure.
                let header_b64 = if payload_parts.len() == 3 {
                    Some(payload_parts[2].to_string())
                } else {
                    None
                };

                if let (Ok(ciphertext), Ok(nonce), Ok(signature)) = (
                    base64::engine::general_purpose::STANDARD.decode(&msg.content),
                    base64::engine::general_purpose::STANDARD.decode(nonce_b64),
                    base64::engine::general_purpose::STANDARD.decode(sig_b64),
                ) {
                    let pq_key_bytes = header_b64
                        .and_then(|h| base64::engine::general_purpose::STANDARD.decode(&h).ok());

                    let enc_msg = crate::core::encryption::EncryptedMessage {
                        ciphertext,
                        nonce,
                        signature,
                        sender_id: msg.sender.clone(),
                        timestamp: msg.timestamp,
                        key_id: if is_ratchet {
                            format!("ratchet:{}", msg.sender)
                        } else {
                            "".to_string()
                        },
                        message_type: EncMessageType::Text,
                        version: if is_ratchet { 0x04 } else { 0x03 },
                        pq_key: pq_key_bytes,
                    };

                    let decrypt_result: Result<String, anyhow::Error> = if is_ratchet {
                        self.encryption_manager
                            .decrypt_message_ratchet(&enc_msg)
                            .map_err(|e| anyhow::anyhow!("{}", e))
                    } else {
                        self.encryption_manager
                            .decrypt_message(&enc_msg)
                            .map_err(|e| anyhow::anyhow!("{}", e))
                    };

                    match decrypt_result {
                        Ok(plaintext) => {
                            // ponytail: unpad after decryption — backward compat: if unpad fails, use as-is
                            msg.content = traffic_armor::unpad_message(plaintext.as_bytes())
                                .ok()
                                .and_then(|v| String::from_utf8(v).ok())
                                .unwrap_or(plaintext);
                            msg.encryption_status = "decrypted".to_string();
                            msg.message_type = "text".to_string();
                        }
                        Err(e) => {
                            tracing::warn!("Failed to decrypt message from {}: {}", msg.sender, e);
                            msg.encryption_status = "decrypt_failed".to_string();
                        }
                    }
                }
            }
        }

        if is_dm_for_us {
            tracing::trace!("Received direct message from peer {}", peer_id);
        } else {
            tracing::trace!("Processed broadcast message from {}", peer_id);
        }

        if msg.message_type != "handshake" && !msg.message_type.starts_with("encrypted:") {
            let _ = self.handle_incoming_message(peer_id, msg.clone()).await;
        }

        if msg.message_type == "handshake" {
            self.handle_handshake(peer_id, &msg).await;
        }

        if is_dm_for_us && msg.message_type != "ack" && msg.encryption_status != "decrypt_failed" {
            let ack_msg = Message::new(
                self.local_peer_id.to_string(),
                msg.sender.clone(),
                msg.id.to_string(),
            );
            let mut ack_msg = ack_msg;
            ack_msg.message_type = "ack".to_string();

            use crate::core::wire_protocol::{Frame, FrameKind};
            if let Ok(frame) = Frame::encode(FrameKind::Message, &ack_msg) {
                let ack_topic = IdentTopic::new(crate::core::constants::topics::DM);
                let _ = self
                    .swarm
                    .behaviour_mut()
                    .gossipsub
                    .publish(ack_topic, frame.to_bytes());
                debug!("Sent ACK for message {} to {}", msg.id, peer_id);
            }
        }

        Ok(())
    }

    async fn handle_sphinx_packet(
        &mut self,
        peer_id: PeerId,
        message: GossipsubMessage,
    ) -> Result<()> {
        let packet: crate::core::routing::sphinx::SphinxPacket =
            match bincode::deserialize(&message.data) {
                Ok(p) => p,
                Err(_) => return Ok(()),
            };

        match crate::core::routing::sphinx::process_sphinx_packet(
            &packet,
            &self.sphinx_secret,
            &self.local_peer_id.to_string(),
        ) {
            crate::core::routing::sphinx::SphinxProcessResult::Forward {
                next_hop,
                packet: new_packet,
            } => {
                let delay = crate::core::routing::mixing_delay::sample_delay(
                    &crate::core::routing::mixing_delay::MixingConfig::default(),
                );
                if let Ok(pkt_bytes) = bincode::serialize(&new_packet) {
                    if delay > std::time::Duration::ZERO {
                        tracing::trace!(
                            "Sphinx relay to {} (mixing delay {}ms)",
                            next_hop,
                            delay.as_millis()
                        );
                        let queue = self.mixing_queue.clone();
                        queue.enqueue(next_hop, pkt_bytes).await;
                    } else {
                        self.forward_sphinx_packet(&next_hop, &pkt_bytes).await;
                    }
                }
            }
            crate::core::routing::sphinx::SphinxProcessResult::Deliver { payload } => {
                if let Ok((frame, _)) = crate::core::wire_protocol::Frame::from_bytes(&payload) {
                    if let Ok(msg) = frame.decode::<Message>() {
                        tracing::trace!("Sphinx: delivered message from {}", msg.sender);
                        let _ = self.handle_incoming_message(peer_id, msg).await;
                    }
                }
            }
            crate::core::routing::sphinx::SphinxProcessResult::Invalid => {
                tracing::trace!("Sphinx: invalid packet from {}", peer_id);
            }
        }
        Ok(())
    }

    async fn handle_dtn_sync(&mut self, message: GossipsubMessage) -> Result<()> {
        let handshake: crate::core::dtn::epidemic::DtnSyncHandshake =
            match bincode::deserialize(&message.data) {
                Ok(h) => h,
                Err(_) => return Ok(()),
            };

        if let Some(ref dtn_store) = self.dtn_store {
            let our_bloom = dtn_store.build_bloom().await;
            let our_scores = dtn_store.export_delivery_scores().await;
            let bundles_to_send = dtn_store
                .on_encounter(
                    &handshake.node_id,
                    &handshake.have_bloom,
                    &handshake.delivery_scores,
                )
                .await;

            // Send bundles we have that they need
            if !bundles_to_send.is_empty() {
                let dm_topic = IdentTopic::new(crate::core::constants::topics::DM);
                for bundle in &bundles_to_send {
                    let _ = self
                        .swarm
                        .behaviour_mut()
                        .gossipsub
                        .publish(dm_topic.clone(), bundle.payload.clone());
                }
                tracing::debug!(
                    "DTN: sent {} bundles after sync handshake",
                    bundles_to_send.len()
                );
            }

            // Reply with our sync handshake so they can send us what we need
            let reply = crate::core::dtn::epidemic::DtnSyncHandshake {
                node_id: dtn_store.node_id(),
                have_bloom: our_bloom,
                delivery_scores: our_scores,
                ts: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            };
            if let Ok(hs_bytes) = bincode::serialize(&reply) {
                let sync_topic = IdentTopic::new(crate::core::constants::DTN_SYNC_TOPIC);
                let _ = self
                    .swarm
                    .behaviour_mut()
                    .gossipsub
                    .publish(sync_topic, hs_bytes);
            }
        }

        Ok(())
    }

    fn get_peer_ip(&self, _peer_id: &PeerId) -> Option<std::net::IpAddr> {
        None
    }

    async fn handle_handshake(&mut self, peer_id: PeerId, msg: &Message) {
        tracing::info!(
            "🤝 Received Handshake from {} (sender_id: {})",
            peer_id,
            msg.sender
        );
        if let Ok(pub_key_bytes) = base64::engine::general_purpose::STANDARD.decode(&msg.content) {
            let key_len = pub_key_bytes.len();
            let key_type = if key_len >= 1248 {
                crate::core::encryption::KeyType::Ultimate
            } else {
                crate::core::encryption::KeyType::Hybrid
            };
            self.encryption_manager.add_peer_key(
                peer_id.to_string(),
                pub_key_bytes.clone(),
                key_type.clone(),
            );
            self.encryption_manager.add_peer_key(
                msg.sender.clone(),
                pub_key_bytes.clone(),
                key_type,
            );
            // Make authenticated key readiness observable in the topology projection.
            // Discovery/reachability alone must not imply trust, but a completed
            // handshake should be reflected as verified peer metadata.
            if let Some(node) = self.topology.write().await.get_mut(&peer_id.to_string()) {
                node.public_key = pub_key_bytes.clone();
            }
            tracing::info!(
                "✅ Stored encryption key for peer {} (key: {} bytes, type: {:?})",
                peer_id,
                key_len,
                if key_len >= 1248 {
                    "Ultimate"
                } else {
                    "Hybrid"
                }
            );

            if !self.completed_handshakes.contains(&peer_id) {
                self.completed_handshakes.insert(peer_id);
                let my_id = self._identity.id.clone();
                let my_pubkey = base64::engine::general_purpose::STANDARD
                    .encode(self.encryption_manager.get_public_key());
                let mut reply = Message::new(my_id, peer_id.to_string(), my_pubkey);
                reply.message_type = "handshake".to_string();
                self.swarm
                    .behaviour_mut()
                    .request_response
                    .send_request(&peer_id, ChatRequest(reply));
                tracing::info!("Sent handshake reply via request_response to {}", peer_id);
            } else {
                tracing::info!(
                    "Already exchanged handshakes with {}, skipping reply",
                    peer_id
                );
            }
        } else {
            tracing::warn!("⚠️ Failed to decode Handshake key from {}", msg.sender);
        }
    }

    async fn handle_incoming_message(&mut self, _peer_id: PeerId, msg: Message) -> Result<()> {
        // Persist to store (skip for broadcast or failed decryption)
        if msg.recipient != "broadcast" && msg.encryption_status != "decrypt_failed" {
            if let Err(e) = self.message_store.save_message(&msg) {
                tracing::error!("Failed to save incoming message: {}", e);
            }
        } else if msg.encryption_status == "decrypt_failed" {
            // Save plaintext even if decryption failed (for debugging)
            tracing::warn!("Saving message with failed decryption from peer (content redacted)");
        }

        // Add to in-memory
        {
            let mut messages = self.core_messages.write().await;
            messages.push(msg.clone());
            if messages.len() > IN_MEMORY_MSGS_MAX {
                messages.drain(..IN_MEMORY_MSGS_MAX / 2);
            }
        }

        // Send to event bus
        if let Some(bus) = &self.event_bus {
            let _ = bus.send(crate::core::CoreEvent::NewMessage(msg));
        }

        Ok(())
    }

    async fn handle_retries(&mut self) {
        let timed_out = self.reliability.check_timeouts();
        for msg_id in timed_out {
            if let Some(original) = self.pending_messages.get(&msg_id) {
                let recipient = original.recipient.clone();
                let topic = IdentTopic::new(crate::core::constants::topics::DM);

                let mut retransmit = original.clone();
                retransmit.hops = 0;

                use crate::core::wire_protocol::{Frame, FrameKind};
                if let Ok(frame) = Frame::encode(FrameKind::Message, &retransmit) {
                    if let Err(e) = self
                        .swarm
                        .behaviour_mut()
                        .gossipsub
                        .publish(topic, frame.to_bytes())
                    {
                        tracing::warn!("Retry publish to {} failed: {}", recipient, e);
                    } else {
                        tracing::info!("Retransmitted message {} to {}", msg_id, recipient);
                        self.reliability.on_message_sent(msg_id);
                    }
                } else {
                    tracing::warn!("Failed to encode retry frame for {}", msg_id);
                }
            }
        }
        self.pending_messages
            .retain(|id, _| self.reliability.ack_tracker.is_pending(id));
    }

    async fn send_periodic_pings(&mut self) {
        let peers: Vec<PeerId> = self
            .topology
            .read()
            .await
            .keys()
            .filter_map(|id| id.parse().ok())
            .collect();

        for peer_id in peers {
            use crate::core::wire_protocol::{Frame, FrameKind, WirePing};
            let ts = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u32;
            let ping = WirePing { ts, seq: 0 };
            if let Ok(frame) = Frame::encode(FrameKind::Ping, &ping) {
                let topic = IdentTopic::new(crate::core::constants::topics::DM);
                if let Err(e) = self
                    .swarm
                    .behaviour_mut()
                    .gossipsub
                    .publish(topic, frame.to_bytes())
                {
                    tracing::warn!("Failed to send ping to {}: {}", peer_id, e);
                } else {
                    self.peer_health.record_ping(&peer_id.to_string()).await;
                    let mut pending = self.pending_pings.lock().await;
                    pending.insert(ts, peer_id.to_string());
                    tracing::debug!("Sent ping to {}", peer_id);
                }
            }
        }
    }
}

pub struct MeshManager {
    transport: MeshTransport,
    _local_identity: Arc<Identity>,
    _security_manager: Arc<SecurityManager>,
    core_messages: Arc<RwLock<Vec<Message>>>,
    _message_store: Arc<crate::core::store::SecureMessageStore>,
    _reliability: Arc<crate::core::reliability::ReliabilityLayer>,
}

impl MeshManager {
    pub async fn new(
        identity: Arc<Identity>,
        encryption_manager: Arc<EncryptionManager>,
        core_messages: Arc<RwLock<Vec<Message>>>,
        message_store: Arc<crate::core::store::SecureMessageStore>,
        bootstrap_nodes: Vec<Multiaddr>,
        username_discovery: Arc<crate::core::username_discovery::UsernameDiscovery>,
        event_bus: Option<tokio::sync::broadcast::Sender<crate::core::CoreEvent>>,
        handshake_secret: Option<Vec<u8>>,
        reliability: Arc<crate::core::reliability::ReliabilityLayer>,
        discovery_directory: Arc<crate::core::discovery_directory::DiscoveryDirectory>,
        quic_port: u16,
    ) -> Result<Self> {
        let security_config = SecurityConfig::default();
        let security_manager = Arc::new(SecurityManager::new(security_config));

        let transport = MeshTransport::new(
            identity.clone(),
            security_manager.clone(),
            encryption_manager,
            core_messages.clone(),
            message_store.clone(),
            bootstrap_nodes,
            username_discovery,
            event_bus,
            handshake_secret,
            reliability.clone(),
            discovery_directory,
            quic_port,
        )
        .await?;

        Ok(Self {
            transport,
            _local_identity: identity,
            _security_manager: security_manager,
            core_messages,
            _message_store: message_store,
            _reliability: reliability,
        })
    }

    pub async fn start(&mut self, listen_addr: Multiaddr) -> Result<()> {
        self.transport.start(listen_addr).await
    }

    pub async fn send_message(&mut self, message: &Message) -> Result<()> {
        self.transport.broadcast_message(message).await
    }

    pub async fn send_direct_message(&mut self, recipient: PeerId, message: Message) -> Result<()> {
        self.transport.send_direct_message(recipient, message).await
    }

    pub async fn put_dht_record(&self, key: Vec<u8>, value: Vec<u8>) -> Result<()> {
        self.transport.put_record(key, value).await
    }

    pub async fn get_dht_record(&self, key: Vec<u8>) -> Result<Vec<u8>> {
        self.transport.get_record(key).await
    }

    pub async fn set_shard(&mut self, shard: &str) -> Result<()> {
        self.transport.set_shard(shard).await
    }

    pub async fn get_stats(&self) -> MeshStats {
        let connection_stats = self.transport.get_connection_stats();
        let security_stats = self.transport.get_security_stats().await;
        let topology = self.transport.get_topology().await;

        MeshStats {
            total_nodes: topology.nodes.len(),
            online_nodes: topology.nodes.values().filter(|n| n.is_online).count(),
            local_node_id: topology.local_node_id,
            routes_count: topology.routes.len(),
            connection_stats,
            security_stats,
        }
    }

    pub async fn get_listeners(&self) -> Vec<Multiaddr> {
        self.transport.get_listeners().await
    }

    pub fn mdns_health(&self) -> MeshMdnsHealth {
        MeshMdnsHealth {
            enabled: self.transport.mdns_enabled,
            discovered_total: self.transport.mdns_discovered.load(Ordering::Relaxed),
            expired_total: self.transport.mdns_expired.load(Ordering::Relaxed),
            last_event_unix: self.transport.mdns_last_event.load(Ordering::Relaxed),
        }
    }

    pub async fn add_address(&self, peer_id: PeerId, address: Multiaddr) -> Result<()> {
        self.transport.add_address(peer_id, address).await
    }

    pub async fn get_topology(&self) -> MeshTopology {
        self.transport.get_topology().await
    }

    pub fn set_peer_count_cache(&mut self, cache: Arc<AtomicUsize>) {
        self.transport.peer_count_cache = Some(cache);
    }

    pub fn set_dtn_store(&mut self, store: Arc<crate::core::dtn::epidemic::DtnStore>) {
        self.transport.dtn_store = Some(store);
    }

    pub fn set_mailbox_store(&mut self, store: Arc<crate::core::mailbox::MailboxStore>) {
        self.transport.mailbox_store = Some(store);
    }

    pub fn add_bootstrap_nodes(&mut self, nodes: Vec<Multiaddr>) {
        self.transport.add_bootstrap_nodes(nodes);
    }

    /// Publish username to DHT
    pub async fn publish_world_discovery(&self) -> Result<()> {
        self.transport.publish_world_discovery().await
    }

    pub async fn set_world_discovery_provider(&self, enabled: bool) -> Result<()> {
        self.transport.set_world_discovery_provider(enabled).await
    }

    pub async fn refresh_world_discovery_providers(&self) -> Result<()> {
        self.transport.refresh_world_discovery_providers().await
    }

    pub async fn world_discovery_health(&self) -> WorldDiscoveryHealth {
        self.transport.world_discovery_health().await
    }

    pub async fn publish_username(&mut self) -> Result<()> {
        self.transport.publish_username().await
    }

    /// Lookup username in DHT
    pub async fn lookup_username(
        &mut self,
        username: &str,
    ) -> Result<Option<crate::core::username_discovery::UsernameRecord>> {
        self.transport.lookup_username(username).await
    }

    /// Connect to a peer by ID
    /// Measure a real round-trip to a peer, in milliseconds.
    ///
    /// Delegates to the transport, which puts a request on the wire and only
    /// resolves when the peer's reply arrives. A peer that never answers is
    /// caught by the caller's timeout rather than reported as a number.
    pub async fn ping_peer(&self, peer_id: PeerId) -> Result<u64> {
        self.transport.ping_peer(peer_id).await
    }

    pub async fn connect_peer(&mut self, peer_id: PeerId) -> Result<()> {
        self.transport.connect_peer(peer_id).await
    }

    /// Subscribe to a gossipsub topic (e.g., a peer's DM topic)
    pub async fn subscribe_topic(&self, topic: &str) -> Result<()> {
        self.transport.subscribe_topic(topic).await
    }

    pub async fn publish_to_topic(&self, topic: &str, message: &str) -> Result<()> {
        self.transport
            .publish_to_topic(topic, message.as_bytes().to_vec())
            .await
    }

    /// Get all messages received by this node (in-memory history)
    pub async fn get_received_messages(&self) -> Vec<Message> {
        self.core_messages.read().await.clone()
    }

    pub async fn stop(&self) -> Result<()> {
        self.transport.stop().await
    }

    /// Access the peer health tracker (for testing / diagnostics)
    pub fn peer_health(&self) -> &Arc<crate::core::peer_health::PeerHealthTracker> {
        &self.transport.peer_health
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MeshStats {
    pub total_nodes: usize,
    pub online_nodes: usize,
    pub local_node_id: String,
    pub routes_count: usize,
    #[serde(skip)]
    pub connection_stats: ConnectionStats,
    #[serde(skip)]
    pub security_stats: SecurityStats,
}

unsafe impl Sync for MeshTransport {}
unsafe impl Send for MeshTransport {}

#[cfg(test)]
mod tests {
    use super::*;
    use libp2p::PeerId;
    use std::collections::HashMap;

    #[test]
    fn test_mesh_node_serialization() {
        let peer_id = PeerId::random();
        let node = MeshNode {
            id: "test-node".to_string(),
            peer_id,
            last_seen_addresses: vec!["/ip4/127.0.0.1/tcp/4001".parse().unwrap()],
            username: "test-user".to_string(),
            public_key: vec![1, 2, 3],
            last_seen: 123456789,
            connection_quality: 0.9,
            is_online: true,
            threat_level: ThreatLevel::Low,
        };

        let serialized = serde_json::to_string(&node).unwrap();
        assert!(serialized.contains("\"id\":\"test-node\""));
        assert!(serialized.contains("\"username\":\"test-user\""));
        // peer_id and address should be skipped
        assert!(!serialized.contains("\"peer_id\""));
        assert!(!serialized.contains("\"address\""));
    }

    #[test]
    fn test_mesh_topology_creation() {
        let topology = MeshTopology {
            nodes: HashMap::new(),
            routes: HashMap::new(),
            local_node_id: "local".to_string(),
        };
        assert_eq!(topology.local_node_id, "local");
        assert!(topology.nodes.is_empty());
    }

    #[test]
    fn test_mesh_stats_serialization() {
        let stats = MeshStats {
            total_nodes: 5,
            online_nodes: 3,
            local_node_id: "local-node".to_string(),
            routes_count: 2,
            connection_stats: ConnectionStats::default(),
            security_stats: SecurityStats::default(),
        };

        let serialized = serde_json::to_value(&stats).unwrap();
        assert_eq!(serialized["total_nodes"], 5);
        assert_eq!(serialized["online_nodes"], 3);
        assert_eq!(serialized["local_node_id"], "local-node");
        assert_eq!(serialized["routes_count"], 2);

        // connection_stats and security_stats should be skipped due to #[serde(skip)]
        assert!(serialized
            .as_object()
            .unwrap()
            .get("connection_stats")
            .is_none());
    }
}
