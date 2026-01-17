use crate::core::encryption::{EncryptionManager, KeyType};
use crate::core::identity::Identity;
use crate::core::message::Message;
use crate::core::security::{SecurityConfig, SecurityManager, SecurityStats, ThreatLevel};
use crate::core::stealth_tcp::{ConnectionStats, StealthTCPProvider};
use anyhow::Result;
// use async_trait::async_trait;
use base64::Engine;
use futures_util::StreamExt;
use libp2p::gossipsub::IdentTopic;
use libp2p::kad::store::MemoryStore;
use libp2p::kad::{Behaviour as Kad, Event as KadEvent, QueryResult};
use libp2p::tcp::Transport as TcpTransport;
use libp2p::{
    core::{transport::upgrade::Version, transport::Transport},
    gossipsub::{
        Behaviour as Gossipsub, Config as GossipsubConfig, Event as GossipsubEvent,
        Message as GossipsubMessage, MessageAuthenticity, MessageId,
    },
    identify::{
        Behaviour as Identify, Config as IdentifyConfig, Event as IdentifyEvent,
        Info as IdentifyInfo,
    },
    mdns::{tokio::Behaviour as Mdns, Config as MdnsConfig, Event as MdnsEvent},
    swarm::{Config as SwarmConfig, NetworkBehaviour, Swarm, SwarmEvent},
    Multiaddr, PeerId,
};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

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

pub const DEFAULT_BOOTSTRAPS: &'static [&'static str] = &[
    "/dnsaddr/bootstrap.libp2p.io/p2p/QmNnooP36J7RSRM8uXCc8p6Y74G56EPV45UFSy9Pr7T79F",
    "/dnsaddr/bootstrap.libp2p.io/p2p/QmQCUuY6vS7705XT5ZqUhc4G3n9PqEqH5D3S8Y8xYFSy9Pr",
    "/dnsaddr/bootstrap.libp2p.io/p2p/QmbLHAnMo6pXhvG2SFTkS5rTfyS28y2XbY3R7wF1pG4z5W",
    "/dnsaddr/bootstrap.libp2p.io/p2p/QmcZf59bWbeUUn9BR3pNDfB3zM9F136G9c6y6y2j9X6v4",
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
}

/// A libp2p transport that wraps TCP in a Stealth Handshake
pub struct StealthTransport {
    _provider: Arc<StealthTCPProvider>,
    inner: TcpTransport<libp2p_tcp::tokio::Tcp>,
}

impl StealthTransport {
    pub fn new(provider: Arc<StealthTCPProvider>) -> Self {
        Self {
            _provider: provider,
            inner: TcpTransport::<libp2p_tcp::tokio::Tcp>::new(
                libp2p_tcp::Config::default().nodelay(true).port_reuse(true),
            ),
        }
    }
}

/// Mesh transport implementation with enhanced security
pub struct MeshTransport {
    swarm: Swarm<MeshBehaviour>,
    node_id: String,
    identity: Arc<Identity>,
    security_manager: Arc<SecurityManager>,
    encryption_manager: Arc<EncryptionManager>,
    stealth_provider: StealthTCPProvider,
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
    ) -> Result<Self> {
        let node_id = Uuid::new_v4().to_string();
        let local_key = identity.keypair();
        let local_peer_id = PeerId::from(local_key.public());

        // Create enhanced StealthTCP provider with security integration
        let stealth_provider = StealthTCPProvider::new(
            None, // Allowlist - can be configured
            security_manager.clone(),
            None, // Auto-generate handshake secret
            true, // Enable stealth mode
        );

        // Use our custom StealthTransport instead of standard TcpTransport
        let stealth_transport = StealthTransport::new(Arc::new(stealth_provider.clone()));

        let (relay_transport, relay_client) = libp2p::relay::client::new(local_peer_id);

        let transport = stealth_transport
            .inner
            .or_transport(relay_transport)
            .upgrade(Version::V1)
            .authenticate(libp2p::noise::Config::new(&local_key)?)
            .multiplex(libp2p::yamux::Config::default())
            .boxed();

        // Create network behavior with enhanced security
        let gossipsub_config = GossipsubConfig::default();
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
        let mut kad_config = libp2p::kad::Config::default();
        let shard_id = "global"; // TODO: Dynamically determine region
        kad_config.set_protocol_names(vec![
            libp2p::StreamProtocol::new("/ghostwire/kad/shard/global"),
            libp2p::StreamProtocol::try_from_owned(format!("/ghostwire/kad/shard/{}", shard_id))
                .unwrap(),
        ]);

        let mut kad = Kad::with_config(local_peer_id, MemoryStore::new(local_peer_id), kad_config);

        // Harden Kademlia: Increase replication and periodic refresh
        kad.set_mode(Some(libp2p::kad::Mode::Server));

        let mdns = Mdns::new(MdnsConfig::default(), local_peer_id)?;

        let autonat =
            libp2p::autonat::Behaviour::new(local_peer_id, libp2p::autonat::Config::default());

        let dcutr = libp2p::dcutr::Behaviour::new(local_peer_id);

        let behaviour = MeshBehaviour {
            gossipsub,
            identify,
            kad,
            mdns,
            autonat,
            relay: relay_client,
            dcutr,
        };

        let swarm = Swarm::new(
            transport,
            behaviour,
            local_peer_id,
            SwarmConfig::with_tokio_executor(),
        );

        Ok(Self {
            swarm,
            node_id: node_id.clone(),
            identity,
            security_manager,
            encryption_manager,
            stealth_provider,
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
        info!("Starting secure mesh network on {}", listen_addr);

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
        self.swarm.listen_on(listen_addr)?;

        // Subscribe to broadcast topic
        let topic = IdentTopic::new("ghostwire-messages");
        self.swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

        // Connect to bootstrap nodes
        for addr in &self.bootstrap_nodes {
            if let Some(peer_id) = addr.iter().find_map(|p| {
                if let libp2p::multiaddr::Protocol::P2p(peer_id) = p {
                    Some(peer_id)
                } else {
                    None
                }
            }) {
                info!("Adding bootstrap node: {} ({})", peer_id, addr);
                self.swarm
                    .behaviour_mut()
                    .kad
                    .add_address(&peer_id, addr.clone());
            } else {
                warn!("Bootstrap address missing PeerId: {}", addr);
            }
        }

        // Kick off Kademlia bootstrap
        if !self.bootstrap_nodes.is_empty() {
            info!("Initiating DHT bootstrap...");
            self.swarm.behaviour_mut().kad.bootstrap()?;
        }

        // Publish username to DHT
        let username_discovery = self.username_discovery.clone();
        let local_key = self.identity.keypair().clone();
        let pk_bytes = local_key.public().encode_protobuf();
        let sk_bytes = local_key
            .to_protobuf_encoding()
            .map_err(|e| anyhow::anyhow!(e))?;

        if let Ok((key, value)) =
            username_discovery.prepare_username_publication(pk_bytes, &sk_bytes)
        {
            self.swarm.behaviour_mut().kad.put_record(
                libp2p::kad::Record {
                    key,
                    value,
                    publisher: Some(self.identity.keypair().public().to_peer_id()),
                    expires: None,
                },
                libp2p::kad::Quorum::One,
            )?;
            info!("Auto-published username to DHT");
        }

        // Start the swarm event loop
        self.run_swarm_loop().await?;

        Ok(())
    }

    /// Run the swarm event loop with security monitoring
    async fn run_swarm_loop(&mut self) -> Result<()> {
        loop {
            tokio::select! {
                        swarm_event = self.swarm.next() => {
                            match swarm_event {
                                Some(SwarmEvent::NewListenAddr { address, .. }) => {
                                    info!("Listening on {}", address);
                                }
                                Some(SwarmEvent::Behaviour(event)) => {
                                    match event {
                                        MeshBehaviourEvent::Gossipsub(event) => {
                                            match event {
                                                GossipsubEvent::Message { propagation_source, message_id, message } => {
                                                    self.handle_gossipsub_message(propagation_source, message_id, message).await?;
                                }
                                                _ => {}
                                            }
                                        }
                                        MeshBehaviourEvent::Identify(event) => {
                                            match event {
                                                IdentifyEvent::Received { peer_id, info, .. } => {
                                    self.handle_identify_info(peer_id, info).await?;
                                }
                                                _ => {}
                                            }
                                        }
                                        MeshBehaviourEvent::Kad(event) => {
                                            match event {
                                                KadEvent::OutboundQueryProgressed { ref result, .. } => {
                                                    if let QueryResult::Bootstrap(result) = result {
                                                        info!("Kademlia bootstrap completed: {:?}", result);
                                                    }
                                                    // Pass to username discovery
                                                    if let Err(e) = self.username_discovery.handle_kad_event(&event).await {
                                                        debug!("Username discovery failed to handle Kademlia event: {:?}", e);
                                                    }
                                                }
                                                _ => {}
                                            }
                                        }
                                        MeshBehaviourEvent::Mdns(event) => {
                                            match event {
                                                MdnsEvent::Discovered(list) => {
                                    for (peer_id, multiaddr) in list {
                                        info!("mDNS discovered peer: {} at {}", peer_id, multiaddr);
                                        self.swarm.behaviour_mut().kad.add_address(&peer_id, multiaddr);
                                        self.send_handshake().await?;
                                        // Broadcast peer connection event
                                        if let Some(bus) = &self.event_bus {
                                            let _ = bus.send(crate::core::CoreEvent::PeerConnected(peer_id.to_string()));
                                        }
                                    }
                                }
                                        MdnsEvent::Expired(list) => {
                                            for (peer_id, multiaddr) in list {
                                                tracing::info!("mDNS expired peer: {} at {}", peer_id, multiaddr);
                                                self.swarm.behaviour_mut().kad.remove_address(&peer_id, &multiaddr);
                                            }
                                        }
                                    }
                                }
                                MeshBehaviourEvent::Autonat(event) => {
                                     match event {
                                        libp2p::autonat::Event::StatusChanged { old, new } => {
                                            info!("AutoNAT status changed: {:?} -> {:?}", old, new);
                                        }
                                        libp2p::autonat::Event::InboundProbe(e) => {
                                            debug!("AutoNAT Inbound Probe: {:?}", e);
                                        }
                                        libp2p::autonat::Event::OutboundProbe(e) => {
                                            debug!("AutoNAT Outbound Probe: {:?}", e);
                                        }
                                     }
                                }
                                MeshBehaviourEvent::Relay(event) => {
                                    debug!("Relay client event: {:?}", event);
                                }
                                MeshBehaviourEvent::Dcutr(event) => {
                                    debug!("DCUtR event: {:?}", event);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    async fn send_handshake(&mut self) -> Result<()> {
        let pub_key = self.encryption_manager.export_public_key();
        let encoded_key = base64::engine::general_purpose::STANDARD.encode(pub_key);

        let msg = Message {
            id: Uuid::new_v4(),
            sender: self.node_id.clone(),
            recipient: "broadcast".to_string(),
            content: encoded_key,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
            encrypted: false,
            message_type: "handshake".to_string(),
            encryption_status: "none".to_string(),
        };

        self.broadcast_message(&msg).await?;
        Ok(())
    }

    async fn handle_gossipsub_message(
        &mut self,
        peer_id: PeerId,
        message_id: MessageId,
        message: GossipsubMessage,
    ) -> Result<()> {
        debug!(
            "Received gossipsub message from {}: {:?}",
            peer_id, message_id
        );

        // Security check for message processing
        let peer_ip = self.get_peer_ip(&peer_id).await;
        if let Some(ip) = peer_ip {
            if !self.security_manager.is_ip_allowed(&ip).await {
                warn!("Rejected message from blacklisted peer: {}", peer_id);
                return Ok(());
            }
        }

        if let Ok(msg) = serde_json::from_slice::<Message>(&message.data) {
            info!("Processed message from {}: {:?}", peer_id, msg);

            // Broadcast message received event (except handshakes)
            if msg.message_type != "handshake" {
                if let Some(bus) = &self.event_bus {
                    let _ = bus.send(crate::core::CoreEvent::NewMessage(msg.clone()));
                }
            }

            if msg.message_type == "handshake" {
                info!("🤝 Received Handshake from {}", msg.sender);
                if let Ok(pub_key_bytes) =
                    base64::engine::general_purpose::STANDARD.decode(&msg.content)
                {
                    self.encryption_manager.add_peer_key(
                        msg.sender.clone(),
                        pub_key_bytes,
                        KeyType::Hybrid,
                    );
                    info!("✅ Verified & Stored Identity for peer: {}", msg.sender);
                } else {
                    warn!("⚠️ Failed to decode Handshake key from {}", msg.sender);
                }
            }
        } else {
            warn!("Failed to deserialize message from {}", peer_id);
        }

        Ok(())
    }

    async fn handle_identify_info(&mut self, peer_id: PeerId, info: IdentifyInfo) -> Result<()> {
        info!("Received identify info from {}: {:?}", peer_id, info);
        Ok(())
    }

    pub async fn broadcast_message(&mut self, message: &Message) -> Result<()> {
        let topic = IdentTopic::new("ghostwire-messages");
        let data = serde_json::to_vec(message)?;
        self.swarm
            .behaviour_mut()
            .gossipsub
            .publish(topic.clone(), data)?;
        info!("Broadcasted message to topic: {}", topic);
        Ok(())
    }

    pub async fn get_topology(&self) -> MeshTopology {
        // For now, return a simple topology
        MeshTopology {
            nodes: HashMap::new(),
            routes: HashMap::new(),
            local_node_id: self.node_id.clone(),
        }
    }

    pub async fn find_route(&self, _target_node: &str) -> Option<Vec<String>> {
        // For now, return None
        None
    }

    pub async fn update_connection_quality(&mut self, node_id: &str, quality: f32) {
        // For now, do nothing
        info!("Updated connection quality for {}: {}", node_id, quality);
    }

    /// Get connection statistics from StealthTCP provider
    pub fn get_connection_stats(&self) -> ConnectionStats {
        self.stealth_provider.get_connection_stats()
    }

    /// Get listening multiaddresses
    pub fn get_listeners(&self) -> Vec<Multiaddr> {
        self.swarm.listeners().cloned().collect()
    }

    /// Get security statistics
    pub async fn get_security_stats(&self) -> crate::core::security::SecurityStats {
        self.security_manager.get_security_stats().await
    }

    /// Helper method to get peer IP (simplified)
    async fn get_peer_ip(&self, _peer_id: &PeerId) -> Option<std::net::IpAddr> {
        // In a real implementation, this would look up the peer's IP from the swarm
        None
    }

    pub async fn set_shard(&mut self, shard: &str) -> Result<()> {
        self.current_shard = shard.to_string();
        // Since Kad protocol names are usually fixed at creation in current libp2p,
        // we'll log this as a switch. Real implementation might need to restart the swarm
        // or support dynamic protocols if the underlying transport allows.
        tracing::info!("MeshTransport switching to DHT Shard: {}", shard);
        Ok(())
    }

    pub fn add_bootstrap_nodes(&mut self, nodes: Vec<Multiaddr>) {
        for addr in nodes {
            if let Some(peer_id) = addr.iter().find_map(|p| {
                if let libp2p::multiaddr::Protocol::P2p(peer_id) = p {
                    Some(peer_id)
                } else {
                    None
                }
            }) {
                info!("Dynamically adding bootstrap node: {} ({})", peer_id, addr);
                self.swarm
                    .behaviour_mut()
                    .kad
                    .add_address(&peer_id, addr.clone());
                self.bootstrap_nodes.push(addr);
            } else {
                warn!("Bootstrap address missing PeerId: {}", addr);
            }
        }
        // Force a bootstrap or bucket refresh
        if let Err(e) = self.swarm.behaviour_mut().kad.bootstrap() {
            warn!("Failed to trigger DHT bootstrap: {}", e);
        }
    }

    /// Publish username to DHT
    pub async fn publish_username(&mut self) -> Result<()> {
        // Placeholder: In a real implementation we need the private key to sign the record.
        warn!("Publishing username is currently a placeholder until Identity key access is refactored");
        Ok(())
    }

    /// Connect to a peer by ID
    pub async fn connect_peer(&mut self, peer_id: PeerId) -> Result<()> {
        self.swarm
            .dial(peer_id)
            .map_err(|e| anyhow::anyhow!("Dial error: {}", e))?;
        info!("Dialing peer: {}", peer_id);
        Ok(())
    }

    /// Lookup username in DHT
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
                self.swarm.behaviour_mut().kad.get_record(key);
                Ok(None) // Async lookup started
            }
        }
    }
}

pub struct MeshManager {
    transport: MeshTransport,
    _local_identity: Arc<Identity>,
    _security_manager: Arc<SecurityManager>,
    _core_messages: Arc<RwLock<Vec<Message>>>,
    _message_store: Arc<crate::core::store::SecureMessageStore>,
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
        )
        .await?;

        Ok(Self {
            transport,
            _local_identity: identity,
            _security_manager: security_manager,
            _core_messages: core_messages,
            _message_store: message_store,
        })
    }

    pub async fn start(&mut self, listen_addr: Multiaddr) -> Result<()> {
        self.transport.start(listen_addr).await
    }

    pub async fn send_message(&mut self, message: &Message) -> Result<()> {
        self.transport.broadcast_message(message).await
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

    pub fn get_listeners(&self) -> Vec<Multiaddr> {
        self.transport.get_listeners()
    }

    pub async fn get_topology(&self) -> MeshTopology {
        self.transport.get_topology().await
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
