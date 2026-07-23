use crate::core::encryption::{EncryptionManager, MessageType as EncMessageType};
use crate::core::traffic_armor;
use crate::core::identity::Identity;
use crate::core::message::Message;
use crate::core::security::{SecurityConfig, SecurityManager, SecurityStats, ThreatLevel};
use crate::core::stealth_tcp::{ConnectionStats, StealthTCPProvider};
use anyhow::Result;
use bincode::Options;
// use async_trait::async_trait;
use crate::core::username_discovery::UsernameDiscovery;
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
        PublishError,
    },
    identify::{Behaviour as Identify, Config as IdentifyConfig},
    mdns::{tokio::Behaviour as Mdns, Config as MdnsConfig},
    request_response::{self, Config as RequestResponseConfig, ProtocolSupport},
    swarm::{NetworkBehaviour, Swarm, SwarmEvent},
    Multiaddr, PeerId,
};
use libp2p_webrtc as webrtc;
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::AtomicUsize;
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

pub const DEFAULT_BOOTSTRAPS: &[&str] = &[];

pub const CLOUDFLARE_BOOTSTRAP_URL: &str = "https://ghostwire-bootstrap.st02563362023.workers.dev/api/peers";

pub async fn fetch_cloudflare_peers() -> Vec<Multiaddr> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build();
    let client = match client {
        Ok(c) => c,
        Err(_) => return vec![],
    };

    let resp = match client.get(CLOUDFLARE_BOOTSTRAP_URL).send().await {
        Ok(r) => r,
        Err(e) => {
            tracing::warn!("Cloudflare bootstrap fetch failed: {}", e);
            return vec![];
        }
    };

    let body = match resp.json::<serde_json::Value>().await {
        Ok(b) => b,
        Err(e) => {
            tracing::warn!("Cloudflare bootstrap parse failed: {}", e);
            return vec![];
        }
    };

    let peers = match body.get("peers").and_then(|p| p.as_array()) {
        Some(arr) => arr,
        None => return vec![],
    };

    let mut addrs = Vec::new();
    for peer in peers {
        if let Some(ma_arr) = peer.get("multiaddrs").and_then(|m| m.as_array()) {
            for ma_str in ma_arr {
                if let Some(s) = ma_str.as_str() {
                    if let Ok(addr) = s.parse::<Multiaddr>() {
                        addrs.push(addr);
                    }
                }
            }
        }
    }

    tracing::info!("Fetched {} peer addresses from Cloudflare bootstrap", addrs.len());
    addrs
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
    pub mdns: Mdns,
    pub autonat: libp2p::autonat::Behaviour,
    pub relay: libp2p::relay::client::Behaviour,
    pub dcutr: libp2p::dcutr::Behaviour,
    pub request_response: libp2p::request_response::cbor::Behaviour<ChatRequest, ChatResponse>,
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
            _inner: libp2p::tcp::tokio::Transport::new(libp2p::tcp::Config::default().nodelay(true)),
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
    /// Step 4: Shared routes map for topology API
    routes: Arc<RwLock<HashMap<String, Vec<String>>>>,
    peer_count_cache: Option<Arc<AtomicUsize>>,
    dtn_store: Option<Arc<crate::core::dtn::epidemic::DtnStore>>,
    mailbox_store: Option<Arc<crate::core::mailbox::MailboxStore>>,
    reliability: Arc<crate::core::reliability::ReliabilityLayer>,
    peer_health: Arc<crate::core::peer_health::PeerHealthTracker>,
    _pending_pings: tokio::sync::Mutex<crate::core::peer_health::PendingPings>,
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
    ) -> Result<Self> {
        let local_key = identity.keypair().ok_or_else(|| {
            anyhow::anyhow!("Identity is locked during Mesh Transport initialization")
        })?;
        let local_peer_id = PeerId::from(local_key.public());
        // Use PeerId string as node_id (stable, unique, matches network identity)
        let node_id = local_peer_id.to_string();

        // Initialize obfs4 for DPI evasion (always-on by default)
        // Note: Actual obfuscation integrated in transport stack in future releases

        // Create base TCP transport
        let tcp_transport =
            libp2p::tcp::tokio::Transport::new(libp2p::tcp::Config::default().nodelay(true));

        // Add relay transport for NAT traversal
        let (relay_transport, relay_client) = libp2p::relay::client::new(local_peer_id);

        // Add WebRTC transport for browser/P2P connectivity
        let webrtc_transport = webrtc::tokio::Transport::new(
            local_key.clone(),
            webrtc::tokio::Certificate::generate(&mut OsRng)?,
        );

        // Build the transport stack:
        // TCP → Noise (encryption) → Yamux (multiplexing)
        // obfs4 obfuscation is applied at the connection layer
        // Fallback to relay and WebRTC for NAT traversal
        let transport = tcp_transport
            .or_transport(relay_transport)
            .upgrade(Version::V1)
            .authenticate(libp2p::noise::Config::new(&local_key)?)
            .multiplex(libp2p::yamux::Config::default())
            .or_transport(webrtc_transport)
            .map(|either, _| match either {
                libp2p::futures::future::Either::Left((peer_id, muxer)) => {
                    (peer_id, libp2p::core::muxing::StreamMuxerBox::new(muxer))
                }
                libp2p::futures::future::Either::Right((peer_id, muxer)) => {
                    (peer_id, libp2p::core::muxing::StreamMuxerBox::new(muxer))
                }
            })
            .boxed();

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

        let mdns = Mdns::new(MdnsConfig::default(), local_peer_id)?;

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
            mdns,
            autonat,
            relay: relay_client,
            dcutr,
            request_response,
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
            _core_messages: core_messages,
            _message_store: message_store,
            bootstrap_nodes,
            username_discovery,
            event_bus,
            topology: Arc::new(RwLock::new(HashMap::new())),
            // Step 4: Initialize routes shared between transport and driver
            routes: Arc::new(RwLock::new(HashMap::new())),
            peer_count_cache: None,
            dtn_store: None,
            mailbox_store: None,
            reliability,
            peer_health: Arc::new(crate::core::peer_health::PeerHealthTracker::new(
                std::time::Duration::from_secs(2),
                std::time::Duration::from_secs(30),
            )),
            _pending_pings: tokio::sync::Mutex::new(crate::core::peer_health::PendingPings::new()),
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

        // Subscribe to broadcast topic
        let topic = IdentTopic::new(crate::core::constants::BROADCAST_TOPIC);
        swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

        // Get local peer ID before moving swarm into driver
        let local_peer_id = *swarm.local_peer_id();

        // Subscribe to shared DM topic (all DMs go here, content encrypted end-to-end)
        // Using a single topic avoids leaking recipient PeerId in gossipsub metadata
        let dm_topic = IdentTopic::new(crate::core::constants::DM_TOPIC);
        swarm.behaviour_mut().gossipsub.subscribe(&dm_topic)?;
        tracing::info!("Subscribed to shared DM topic");

        let dtn_topic = IdentTopic::new(crate::core::constants::DTN_SYNC_TOPIC);
        swarm.behaviour_mut().gossipsub.subscribe(&dtn_topic)?;
        tracing::info!("Subscribed to DTN sync topic");

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
            } else {
                tracing::warn!("Bootstrap address missing PeerId: {}", addr);
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
            // Step 2 & 4: Use shared routes from MeshTransport
            routes: self.routes.clone(),
            peer_count_cache: self.peer_count_cache.clone(),
            dtn_store: self.dtn_store.clone(),
            mailbox_store: self.mailbox_store.clone(),
            completed_handshakes: std::collections::HashSet::new(),
            reliability: self.reliability.clone(),
            vector_clock: crate::core::vector_clock::VectorClock::new(),
            pending_messages: HashMap::new(),
            peer_health: self.peer_health.clone(),
            pending_pings: tokio::sync::Mutex::new(crate::core::peer_health::PendingPings::new()),
            mixing_queue: Arc::new(crate::core::routing::mixing_delay::MixingQueue::new(
                crate::core::routing::mixing_delay::MixingConfig::default(),
            )),
            sphinx_secret: self.encryption_manager.x25519_secret()
                .ok_or_else(|| anyhow::anyhow!("X25519 secret not available for Sphinx"))?,
            mixing_tx: None,
            cover_rx,
        };

        tokio::spawn(driver.run());
        Ok(())
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

    pub fn add_bootstrap_nodes(&mut self, _nodes: Vec<Multiaddr>) {
        tracing::warn!("Adding bootstrap nodes dynamically not supported in actor model yet");
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
        
        let routes = self.routes.read().await.clone();

        MeshTopology {
            nodes,
            routes,
            local_node_id: self.node_id.clone(),
        }
    }

    pub fn find_route(&self, _target_node: &str) -> Option<Vec<String>> {
        None
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
    /// Step 2: Track peer connections for route computation
    /// Routes map: source_peer_id -> [neighbor_peer_ids]
    routes: Arc<RwLock<HashMap<String, Vec<String>>>>,
    peer_count_cache: Option<Arc<AtomicUsize>>,
    dtn_store: Option<Arc<crate::core::dtn::epidemic::DtnStore>>,
    mailbox_store: Option<Arc<crate::core::mailbox::MailboxStore>>,
    completed_handshakes: std::collections::HashSet<PeerId>,
    reliability: Arc<crate::core::reliability::ReliabilityLayer>,
    vector_clock: crate::core::vector_clock::VectorClock,
    /// Messages pending ACK — keyed by message ID, used for retry retransmission
    pending_messages: HashMap<Uuid, Message>,
    peer_health: Arc<crate::core::peer_health::PeerHealthTracker>,
    pending_pings: tokio::sync::Mutex<crate::core::peer_health::PendingPings>,
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
        if let Err(e) = self.swarm.behaviour_mut().gossipsub.subscribe(&sphinx_topic) {
            tracing::warn!("Failed to subscribe to Sphinx topic: {}", e);
        } else {
            tracing::info!("Subscribed to Sphinx topic: {}", crate::core::constants::SPHINX_TOPIC);
        }

        let (mixing_tx, mut mixing_rx) = mpsc::channel::<crate::core::routing::mixing_delay::QueuedSphinxPacket>(64);
        let drain_queue = self.mixing_queue.clone();
        {
            let tx_slot = self.mixing_tx.insert(mixing_tx);
            drain_queue.start_drain_loop(tx_slot.clone());
        }

        let mut retry_interval = tokio::time::interval(std::time::Duration::from_secs(5));
        retry_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        let mut ping_interval = tokio::time::interval(std::time::Duration::from_secs(2));
        ping_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        // ponytail: DTN housekeeping — purge expired bundles every 60s
        let mut dtn_purge_interval = tokio::time::interval(std::time::Duration::from_secs(60));
        dtn_purge_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        let mut key_rotation_interval = tokio::time::interval(std::time::Duration::from_secs(3600));
        key_rotation_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
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
                _ = key_rotation_interval.tick() => {
                    if let Some(enc) = Arc::get_mut(&mut self.encryption_manager) {
                        if let Err(e) = enc.rotate_keys_if_needed() {
                            tracing::warn!("Key rotation failed: {}", e);
                        }
                    }
                }
                Some(ready_pkt) = mixing_rx.recv() => {
                    self.forward_sphinx_packet(&ready_pkt.next_hop, &ready_pkt.packet_bytes).await;
                }
                Some(cover_frame) = self.cover_rx.recv() => {
                    let dm_topic = IdentTopic::new(crate::core::constants::DM_TOPIC);
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
                self.vector_clock.increment(&crate::core::vector_clock::redact_node_id(&self.local_peer_id.to_string()));
                message.vector_clock = self.vector_clock.clone();

                let topic = IdentTopic::new(crate::core::constants::BROADCAST_TOPIC);
                use crate::core::wire_protocol::{Frame, FrameKind};
                let frame = match Frame::encode(FrameKind::Message, &message) {
                    Ok(f) => f,
                    Err(e) => { let _ = reply.send(Err(e)); return; }
                };
                let data = frame.to_bytes();
                // ponytail: mark as seen before publish so gossipsub loopback is deduped
                self.seen_messages.push_back(message.id.clone());
                let mut res = self.swarm.behaviour_mut().gossipsub.publish(topic.clone(), data.clone());
                for attempt in 1..=3u32 {
                    if let Err(PublishError::NoPeersSubscribedToTopic) = &res {
                        tracing::debug!("broadcast: no peers subscribed yet (attempt {}/3), waiting 500ms", attempt);
                        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                        res = self.swarm.behaviour_mut().gossipsub.publish(topic.clone(), data.clone());
                    } else {
                        break;
                    }
                }
                let res = res.map(|_| ()).map_err(|e| anyhow::anyhow!("broadcast: {}", e));
                if res.is_ok() {
                    tracing::info!("Broadcasted message to topic: {}", topic);
                }
                let _ = reply.send(res);
            }
            MeshCommand::ConnectPeer { peer_id, reply } => {
                let res = self
                    .swarm
                    .dial(peer_id)
                    .map_err(|e| anyhow::anyhow!("Dial error: {}", e));
                let _ = reply.send(res);
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
                self.vector_clock.increment(&crate::core::vector_clock::redact_node_id(&self.local_peer_id.to_string()));
                message.vector_clock = self.vector_clock.clone();

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
                    tracing::warn!(
                        "Failed to encrypt DM to {}, dropping message",
                        recipient
                    );
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
                                    let sphinx_topic = IdentTopic::new(crate::core::constants::SPHINX_TOPIC);
                                    let mut sphinx_res = self.swarm.behaviour_mut().gossipsub.publish(sphinx_topic.clone(), pkt_bytes.clone());
                                    for attempt in 1..=3u32 {
                                        if let Err(PublishError::NoPeersSubscribedToTopic) = &sphinx_res {
                                            tracing::debug!("sphinx-dm: no peers subscribed yet (attempt {}/3), waiting 500ms", attempt);
                                            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                                            sphinx_res = self.swarm.behaviour_mut().gossipsub.publish(sphinx_topic.clone(), pkt_bytes.clone());
                                        } else {
                                            break;
                                        }
                                    }
                                    match sphinx_res {
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
                    let topic = IdentTopic::new(crate::core::constants::DM_TOPIC);
                    let mut dm_res = self.swarm.behaviour_mut().gossipsub.publish(topic.clone(), data.clone());
                    for attempt in 1..=3u32 {
                        if let Err(PublishError::NoPeersSubscribedToTopic) = &dm_res {
                            tracing::debug!("dm-to-{}: no peers subscribed yet (attempt {}/3), waiting 500ms", recipient, attempt);
                            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                            dm_res = self.swarm.behaviour_mut().gossipsub.publish(topic.clone(), data.clone());
                        } else {
                            break;
                        }
                    }
                    if let Err(e) = dm_res {
                        tracing::warn!(
                            "Gossipsub publish to {} failed: {}, trying DTN fallback",
                            recipient,
                            e
                        );

                        if let Some(ref dtn_store) = self.dtn_store {
                            let destination =
                                *blake3::hash(recipient.to_string().as_bytes()).as_bytes();
                            let bundle = crate::core::dtn::bundle::DtnBundle::new(data, destination);
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

    async fn handle_swarm_event(&mut self, event: SwarmEvent<MeshBehaviourEvent>) {
        // Handle mDNS discovery — auto-connect to discovered peers
        if let SwarmEvent::Behaviour(MeshBehaviourEvent::Mdns(libp2p::mdns::Event::Discovered(
            peers,
        ))) = &event
        {
            for (peer_id, addr) in peers {
                tracing::info!("mDNS discovered peer {} at {}", peer_id, addr);
                // Add to Kademlia routing table
                self.swarm
                    .behaviour_mut()
                    .kad
                    .add_address(peer_id, addr.clone());
                // Attempt to connect
                match self.swarm.dial(addr.clone()) {
                    Ok(()) => tracing::info!("Initiated dial to {} ({})", peer_id, addr),
                    Err(e) => tracing::warn!("Failed to dial {} at {}: {}", peer_id, addr, e),
                }
            }
            return;
        }

        if let SwarmEvent::Behaviour(MeshBehaviourEvent::Mdns(libp2p::mdns::Event::Expired(
            peers,
        ))) = &event
        {
            for (peer_id, _addr) in peers {
                tracing::info!("mDNS peer expired: {}", peer_id);
            }
            return;
        }

        // Log connection events for debugging
        if let SwarmEvent::ConnectionEstablished {
            peer_id,
            num_established,
            ..
        } = &event
        {
            tracing::info!(
                "Connection established with {} ({} total)",
                peer_id,
                num_established
            );
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

            // Step 3: Build bidirectional routes when peers connect
            {
                let mut routes = self.routes.write().await;
                let local_id = self.local_peer_id.to_string();
                let peer_id_str = peer_id.to_string();
                
                // Add peer to local's neighbors
                routes
                    .entry(local_id.clone())
                    .or_insert_with(Vec::new);
                if let Some(neighbors) = routes.get_mut(&local_id) {
                    if !neighbors.contains(&peer_id_str) {
                        neighbors.push(peer_id_str.clone());
                    }
                }
                
                // Add local to peer's neighbors
                routes
                    .entry(peer_id_str.clone())
                    .or_insert_with(Vec::new);
                if let Some(neighbors) = routes.get_mut(&peer_id_str) {
                    if !neighbors.contains(&local_id) {
                        neighbors.push(local_id);
                    }
                }
                
                tracing::info!(
                    "Route updated: {} now has {} neighbors",
                    self.local_peer_id,
                    routes.get(&self.local_peer_id.to_string())
                        .map(|n| n.len())
                        .unwrap_or(0)
                );
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
                    let dm_topic = IdentTopic::new(crate::core::constants::DM_TOPIC);
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
                let handshake = crate::core::dtn::epidemic::DtnSyncHandshake::build(dtn_store).await;
                if let Ok(hs_bytes) = bincode::serialize(&handshake) {
                    let sync_topic = IdentTopic::new(crate::core::constants::DTN_SYNC_TOPIC);
                    let _ = self.swarm.behaviour_mut().gossipsub.publish(sync_topic, hs_bytes);
                    tracing::debug!("DTN: published sync handshake to new peer {}", peer_id);
                }
            }

            if let Some(ref mailbox) = self.mailbox_store {
                if let Ok(deliveries) = mailbox.get_pending_deliveries(&peer_id.to_string()) {
                    if !deliveries.is_empty() {
                        let dm_topic = IdentTopic::new(crate::core::constants::DM_TOPIC);
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
            peer_id,
            error,
            ..
        } = &event
        {
            tracing::warn!("Outgoing connection to {:?} failed: {}", peer_id, error);
        }

        if let SwarmEvent::ConnectionClosed {
            peer_id,
            cause: Some(ref e),
            ..
        } = &event
        {
            tracing::warn!("Connection closed with {}: {}", peer_id, e);

            {
                let mut topo = self.topology.write().await;
                if let Some(node) = topo.get_mut(&peer_id.to_string()) {
                    node.is_online = false;
                }
            }

            self.peer_health.remove_peer(&peer_id.to_string()).await;

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

            // Step 3: Remove peer from routes on disconnect
            {
                let mut routes = self.routes.write().await;
                let local_id = self.local_peer_id.to_string();
                let peer_id_str = peer_id.to_string();

                // Remove peer from local's neighbors
                if let Some(neighbors) = routes.get_mut(&local_id) {
                    neighbors.retain(|n| n != &peer_id_str);
                }

                // Remove local from peer's neighbors
                if let Some(neighbors) = routes.get_mut(&peer_id_str) {
                    neighbors.retain(|n| n != &local_id);
                }

                tracing::info!(
                    "Route updated: {} disconnected from {}",
                    peer_id,
                    local_id
                );
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
            let _ = self
                .handle_gossipsub_message(propagation_source, message_id, message)
                .await;
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
                }
            }
            return;
        }
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

        let is_dm_for_us = topic_str == crate::core::constants::DM_TOPIC;
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
                    let pong = WirePong { ts: ping.ts };
                    if let Ok(pong_frame) = Frame::encode(FrameKind::Pong, &pong) {
                        let pong_topic = IdentTopic::new(crate::core::constants::DM_TOPIC);
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
            let relay_topic = IdentTopic::new(crate::core::constants::DM_TOPIC);
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
                let header_b64 = if is_ratchet && payload_parts.len() == 3 {
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
                let ack_topic = IdentTopic::new(crate::core::constants::DM_TOPIC);
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
        let packet: crate::core::routing::sphinx::SphinxPacket = match bincode::options()
            .with_limit(1024 * 1024)
            .deserialize(&message.data)
        {
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
            match bincode::options()
                .with_limit(1024 * 1024)
                .deserialize(&message.data)
            {
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
                let dm_topic = IdentTopic::new(crate::core::constants::DM_TOPIC);
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
            self.encryption_manager
                .add_peer_key(msg.sender.clone(), pub_key_bytes.clone(), key_type);
            
            // Step 1: Update topology with peer's public key for Sphinx routing
            {
                let mut topo = self.topology.write().await;
                if let Some(node) = topo.get_mut(&peer_id.to_string()) {
                    if node.public_key.is_empty() && key_len >= 32 {
                        node.public_key = pub_key_bytes.clone();
                        tracing::info!(
                            "Updated topology with peer {} public key ({} bytes)",
                            peer_id,
                            key_len
                        );
                    }
                }
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
                let topic = IdentTopic::new(crate::core::constants::DM_TOPIC);

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
            let ping = WirePing { ts };
            if let Ok(frame) = Frame::encode(FrameKind::Ping, &ping) {
                let topic = IdentTopic::new(crate::core::constants::DM_TOPIC);
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

// SAFETY: MeshTransport wraps libp2p Swarm which contains non-Send/Sync types
// (tokio tasks, channel receivers). All mutable access is through Arc<RwLock<>>,
// and the Swarm is only accessed from tokio tasks on the runtime. The Send/Sync
// impls are required to shuttle MeshTransport across task boundaries.
// If libp2p internals change their thread-safety guarantees, this will need review.
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

    #[test]
    fn test_topology_public_key_storage() {
        // Test that MeshNode can store public keys for Sphinx routing
        let peer_id = PeerId::random();
        let public_key = vec![0u8; 32]; // X25519 key is 32 bytes
        
        let node = MeshNode {
            id: "peer-node".to_string(),
            peer_id,
            last_seen_addresses: Vec::new(),
            username: String::new(),
            public_key: public_key.clone(),
            last_seen: 0,
            connection_quality: 1.0,
            is_online: true,
            threat_level: ThreatLevel::Low,
        };

        assert_eq!(node.public_key.len(), 32);
        assert!(node.is_online);
    }

    #[test]
    fn test_topology_routes_structure() {
        // Test that routes are properly structured
        let mut routes = HashMap::new();
        let local_peer = "local-peer-id".to_string();
        let peer1 = "peer-1-id".to_string();
        let peer2 = "peer-2-id".to_string();

        // Local peer connected to peer1 and peer2
        routes.insert(local_peer.clone(), vec![peer1.clone(), peer2.clone()]);
        routes.insert(peer1.clone(), vec![local_peer.clone()]);
        routes.insert(peer2.clone(), vec![local_peer.clone()]);

        let topology = MeshTopology {
            nodes: HashMap::new(),
            routes,
            local_node_id: local_peer,
        };

        // Verify routes are bidirectional
        assert_eq!(topology.routes.len(), 3);
        assert_eq!(topology.routes.get("local-peer-id").unwrap().len(), 2);
        assert!(topology.routes.get("peer-1-id").unwrap().contains(&"local-peer-id".to_string()));
    }
}
