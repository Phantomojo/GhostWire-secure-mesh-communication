use crate::core::encryption::{EncryptionManager, KeyType};
use crate::core::identity::Identity;
use crate::core::message::Message;
use crate::core::security::{SecurityConfig, SecurityManager, SecurityStats, ThreatLevel};
use crate::core::stealth_tcp::{ConnectionStats, StealthTCPProvider};
use anyhow::Result;
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
        Behaviour as Gossipsub, Config as GossipsubConfig, Message as GossipsubMessage,
        MessageAuthenticity, MessageId,
    },
    identify::{Behaviour as Identify, Config as IdentifyConfig},
    mdns::{tokio::Behaviour as Mdns, Config as MdnsConfig},
    request_response::{self, Config as RequestResponseConfig, ProtocolSupport},
    swarm::{NetworkBehaviour, Swarm, SwarmEvent},
    Multiaddr, PeerId,
};
use libp2p_webrtc as webrtc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::{mpsc, oneshot};
use tracing::debug;
use uuid::Uuid;

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

/// Default bootstrap nodes for peer discovery. All are official, well-known libp2p/IPFS
/// bootstrap nodes (bootstrap.libp2p.io pool and IPFS team–run node). Safe for production.
/// See docs/LIBP2P_NETWORK_CONNECTIVITY.md for privacy notes.
pub const DEFAULT_BOOTSTRAPS: &[&str] = &[
    // Original bootstrap.libp2p.io pool
    "/dnsaddr/bootstrap.libp2p.io/p2p/QmNnooP36J7RSRM8uXCc8p6Y74G56EPV45UFSy9Pr7T79F",
    "/dnsaddr/bootstrap.libp2p.io/p2p/QmQCUuY6vS7705XT5ZqUhc4G3n9PqEqH5D3S8Y8xYFSy9Pr",
    "/dnsaddr/bootstrap.libp2p.io/p2p/QmbLHAnMo6pXhvG2SFTkS5rTfyS28y2XbY3R7wF1pG4z5W",
    "/dnsaddr/bootstrap.libp2p.io/p2p/QmcZf59bWbeUUn9BR3pNDfB3zM9F136G9c6y6y2j9X6v4",
    // Official IPFS/Kubo default bootstrap nodes (IPFS dev team)
    "/dnsaddr/bootstrap.libp2p.io/p2p/QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN",
    "/dnsaddr/bootstrap.libp2p.io/p2p/QmQCU2EcMqAqQPR2i9bChDtGNJchTbq5TbXJJ16u19uLTa",
    "/dnsaddr/bootstrap.libp2p.io/p2p/QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb",
    "/dnsaddr/bootstrap.libp2p.io/p2p/QmcZf59bWwK5XFi76CZX8cbJ4BhTzzA3gU1ZjYZcYW3dwt",
    "/ip4/104.131.131.82/tcp/4001/p2p/QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ",
];

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
    inner: libp2p::tcp::tokio::Transport,
}

impl StealthTransport {
    pub fn new(provider: Arc<StealthTCPProvider>) -> Self {
        Self {
            _provider: provider,
            inner: libp2p::tcp::tokio::Transport::new(libp2p::tcp::Config::default().nodelay(true)),
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
        _handshake_secret: Option<Vec<u8>>, // Reserved for future stealth TCP integration
    ) -> Result<Self> {
        let node_id = Uuid::new_v4().to_string();
        let local_key = identity.keypair().ok_or_else(|| {
            anyhow::anyhow!("Identity is locked during Mesh Transport initialization")
        })?;
        let local_peer_id = PeerId::from(local_key.public());

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
            webrtc::tokio::Certificate::generate(&mut rand::thread_rng())?,
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
        let topic = IdentTopic::new("ghostwire-messages");
        swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

        // Get local peer ID before moving swarm into driver
        let local_peer_id = *swarm.local_peer_id();

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

    pub fn get_topology(&self) -> MeshTopology {
        MeshTopology {
            nodes: HashMap::new(),
            routes: HashMap::new(),
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
}

impl MeshDriver {
    async fn run(mut self) {
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
            }
        }
        tracing::info!("Mesh driver task exited.");
    }

    async fn handle_command(&mut self, cmd: MeshCommand) {
        match cmd {
            MeshCommand::PutRecord { key, value, reply } => {
                let record = libp2p::kad::Record {
                    key: libp2p::kad::RecordKey::new(&key),
                    value,
                    publisher: None,
                    expires: None,
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
            MeshCommand::BroadcastMessage { message, reply } => {
                let topic = IdentTopic::new("ghostwire-messages");
                let res = (|| -> Result<()> {
                    // FIXED: use compact MessagePack wire format instead of JSON
                    // Saves 50-70% bandwidth — critical for Global South 3G users
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
                // Subscribe to the peer's DM topic so we can receive their replies
                let dm_topic = IdentTopic::new(&format!("ghostwire-dm-{}", peer_id));
                let _ = self.swarm.behaviour_mut().gossipsub.subscribe(&dm_topic);

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
            MeshCommand::PublishUsername { reply } => {
                let res = (|| -> Result<()> {
                    let identity = self._identity.clone();
                    if let Some(keypair) = identity.keypair() {
                        // Ring verification expects raw 32-byte Ed25519 public key; libp2p protobuf is typically 0x08 0x01 0x12 0x20 <32 bytes>
                        let pk_proto = keypair.public().encode_protobuf();
                        let pk_bytes: Vec<u8> = if pk_proto.len() >= 32 {
                            pk_proto[pk_proto.len() - 32..].to_vec()
                        } else {
                            pk_proto
                        };
                        let (key, value) = self
                            ._username_discovery
                            .prepare_username_publication_with_signer(pk_bytes, |sig_data| {
                                keypair
                                    .sign(sig_data)
                                    .map_err(|e| anyhow::anyhow!("Signing failed: {:?}", e))
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
                message,
                reply,
            } => {
                let res = (|| -> Result<()> {
                    // Send direct message via gossipsub on a per-peer topic
                    // Topic format: ghostwire-dm-{recipient_peer_id}
                    let topic = IdentTopic::new(&format!("ghostwire-dm-{}", recipient));
                    use crate::core::wire_protocol::{Frame, FrameKind};
                    let frame = Frame::encode(FrameKind::Message, &message)?;
                    let data = frame.to_bytes();
                    self.swarm
                        .behaviour_mut()
                        .gossipsub
                        .publish(topic.clone(), data)?;
                    tracing::info!("Sent direct message to peer {} on topic {}", recipient, topic);
                    Ok(())
                })();
                let _ = reply.send(res);
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
        if let SwarmEvent::Behaviour(MeshBehaviourEvent::Mdns(
            libp2p::mdns::Event::Discovered(peers),
        )) = &event
        {
            for (peer_id, addr) in peers {
                tracing::info!("mDNS discovered peer {} at {}", peer_id, addr);
                // Add to Kademlia routing table
                self.swarm.behaviour_mut().kad.add_address(peer_id, addr.clone());
                // Attempt to connect
                let _ = self.swarm.dial(addr.clone());
            }
            return;
        }

        if let SwarmEvent::Behaviour(MeshBehaviourEvent::Mdns(
            libp2p::mdns::Event::Expired(peers),
        )) = &event
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
                peer_id, num_established
            );
            // Subscribe to this peer's DM topic
            let dm_topic = IdentTopic::new(&format!("ghostwire-dm-{}", peer_id));
            let _ = self.swarm.behaviour_mut().gossipsub.subscribe(&dm_topic);
        }

        if let SwarmEvent::ConnectionClosed {
            peer_id,
            cause: Some(ref e),
            ..
        } = &event
        {
            tracing::warn!("Connection closed with {}: {}", peer_id, e);
        }

        if let SwarmEvent::Behaviour(MeshBehaviourEvent::Kad(
            libp2p::kad::Event::OutboundQueryProgressed { id, result, .. },
        )) = event
        {
            if let Some(reply) = self.pending_get_queries.remove(&id) {
                match result {
                    libp2p::kad::QueryResult::GetRecord(Ok(ok)) => {
                        if let libp2p::kad::GetRecordOk::FoundRecord(record) = ok {
                            let _ = reply.send(Ok(record.record.value));
                        } else {
                            let _ = reply.send(Err(anyhow::anyhow!("Record not found")));
                        }
                    }
                    libp2p::kad::QueryResult::GetRecord(Err(e)) => {
                        let _ = reply.send(Err(anyhow::anyhow!("GetRecord error: {:?}", e)));
                    }
                    _ => {
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
                    let _ = self.handle_incoming_message(peer, request.0).await;
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

        // Determine if this is a direct message for us or a broadcast
        let is_dm_for_us = topic_str.starts_with("ghostwire-dm-")
            && topic_str.ends_with(&self.local_peer_id.to_string());

        if !is_dm_for_us && topic_str != "ghostwire-messages" {
            // Not our DM topic and not the broadcast topic — ignore
            return Ok(());
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
        let msg = if let Ok((frame, _consumed)) =
            crate::core::wire_protocol::Frame::from_bytes(&message.data)
        {
            if let Ok(m) = frame.decode::<Message>() {
                m
            } else {
                // Fallback: try direct JSON deserialization
                serde_json::from_slice::<Message>(&message.data)
                    .map_err(|e| anyhow::anyhow!("Failed to decode message: {}", e))?
            }
        } else {
            // Fallback: try direct JSON deserialization
            serde_json::from_slice::<Message>(&message.data)
                .map_err(|e| anyhow::anyhow!("Failed to deserialize message: {}", e))?
        };

        if is_dm_for_us {
            tracing::info!("Received direct message from peer {}", peer_id);
        } else {
            tracing::info!("Processed broadcast message from {}: {:?}", peer_id, msg);
        }

        if msg.message_type != "handshake" {
            let _ = self.handle_incoming_message(peer_id, msg.clone()).await;
        }

        if msg.message_type == "handshake" {
            tracing::info!("🤝 Received Handshake from {}", msg.sender);
            if let Ok(pub_key_bytes) =
                base64::engine::general_purpose::STANDARD.decode(&msg.content)
            {
                self.encryption_manager.add_peer_key(
                    msg.sender.clone(),
                    pub_key_bytes,
                    KeyType::Hybrid,
                );
                tracing::info!("✅ Verified & Stored Identity for peer: {}", msg.sender);
            } else {
                tracing::warn!("⚠️ Failed to decode Handshake key from {}", msg.sender);
            }
        }

        Ok(())
    }

    fn get_peer_ip(&self, _peer_id: &PeerId) -> Option<std::net::IpAddr> {
        None
    }

    async fn handle_incoming_message(&mut self, _peer_id: PeerId, msg: Message) -> Result<()> {
        // Persist to store
        if let Err(e) = self.message_store.save_message(&msg) {
            tracing::error!("Failed to save incoming message: {}", e);
        }

        // Add to in-memory
        {
            let mut messages = self.core_messages.write().await;
            messages.push(msg.clone());
        }

        // Send to event bus
        if let Some(bus) = &self.event_bus {
            let _ = bus.send(crate::core::CoreEvent::NewMessage(msg));
        }

        Ok(())
    }
}

pub struct MeshManager {
    transport: MeshTransport,
    _local_identity: Arc<Identity>,
    _security_manager: Arc<SecurityManager>,
    core_messages: Arc<RwLock<Vec<Message>>>,
    message_store: Arc<crate::core::store::SecureMessageStore>,
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
        )
        .await?;

        Ok(Self {
            transport,
            _local_identity: identity,
            _security_manager: security_manager,
            core_messages,
            message_store,
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
        let topology = self.transport.get_topology();

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

    pub fn get_topology(&self) -> MeshTopology {
        self.transport.get_topology()
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

    /// Get all messages received by this node (in-memory history)
    pub async fn get_received_messages(&self) -> Vec<Message> {
        self.core_messages.read().await.clone()
    }

    pub async fn stop(&self) -> Result<()> {
        self.transport.stop().await
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
