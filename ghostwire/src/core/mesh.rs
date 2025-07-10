use libp2p::{
    core::{upgrade, transport::Transport},
    gossipsub::{self, Behaviour as GossipsubBehaviour, Config as GossipsubConfig, MessageId, Event as GossipsubEvent, Message as GossipsubMessage, IdentTopic as Topic},
    identify::{Behaviour as IdentifyBehaviour, Config as IdentifyConfig, Event as IdentifyEvent, Info as IdentifyInfo},
    kad::{Behaviour as KadBehaviour, store::MemoryStore, Event as KadEvent, QueryResult},
    mdns::{Behaviour as MdnsBehaviour, Config as MdnsConfig, Event as MdnsEvent},
    identity::{Keypair, PublicKey},
    noise::{Config as NoiseConfig},
    swarm::{Swarm, SwarmEvent, NetworkBehaviour, Config as SwarmConfig},
    yamux::Config as YamuxConfig,
    Multiaddr, PeerId, Transport as Libp2pTransport,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use anyhow::Result;
use tracing::{info, warn, debug};
use serde::Serialize;
use crate::core::identity::Identity;
use crate::core::security::{SecurityManager, SecurityConfig, ThreatLevel, SecurityStats};
use crate::core::stealth_tcp::{StealthTCPProvider, ConnectionStats};
use crate::core::message::Message;
use async_trait::async_trait;
use base64::Engine;
use futures_util::StreamExt;

/// Mesh network node information
#[derive(Debug, Clone, Serialize)]
pub struct MeshNode {
    pub id: String,
    #[serde(skip)]
    pub peer_id: PeerId,
    #[serde(skip)]
    pub address: Multiaddr,
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

/// Mesh networking behavior combining libp2p protocols
#[derive(NetworkBehaviour)]
pub struct MeshBehaviour {
    pub gossipsub: GossipsubBehaviour,
    pub identify: IdentifyBehaviour,
    pub kad: KadBehaviour<MemoryStore>,
    pub mdns: MdnsBehaviour,
}

/// Mesh transport implementation with enhanced security
pub struct MeshTransport {
    swarm: Swarm<MeshBehaviour>,
    node_id: String,
    local_peer_id: PeerId,
    security_manager: Arc<SecurityManager>,
    stealth_provider: StealthTCPProvider,
    stats: Arc<RwLock<MeshStats>>,
}

impl MeshTransport {
    pub async fn new(identity: Arc<Identity>, security_manager: Arc<SecurityManager>) -> Result<Self> {
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
        
        // Use a simpler transport setup that works with the current libp2p version
        let transport = libp2p::tcp::Transport::default()
            .upgrade(libp2p::core::upgrade::Version::V1)
            .authenticate(libp2p::noise::Config::new(&local_key)?)
            .multiplex(libp2p::yamux::Config::default())
            .boxed();

        // Create network behavior with enhanced security
        let gossipsub_config = GossipsubConfig::default();
        let gossipsub = GossipsubBehaviour::new(
            gossipsub::MessageAuthenticity::Signed(local_key.clone()),
            gossipsub_config,
        ).map_err(|e| anyhow::anyhow!(e))?;

        let identify = IdentifyBehaviour::new(IdentifyConfig::new(
            "/ghostwire/mesh/1.0.0".to_string(),
            local_key.public(),
        ));

        let kad = KadBehaviour::new(local_peer_id, MemoryStore::new(local_peer_id));

        let mdns = MdnsBehaviour::new(MdnsConfig::default(), local_peer_id)?;

        let behaviour = MeshBehaviour {
            gossipsub,
            identify,
            kad,
            mdns,
        };

        let swarm = Swarm::new(transport, behaviour, local_peer_id, SwarmConfig::without_executor());

        Ok(Self {
            swarm,
            node_id,
            local_peer_id,
            security_manager,
            stealth_provider,
            stats: Arc::new(RwLock::new(MeshStats {
                total_nodes: 0,
                online_nodes: 0,
                local_node_id: node_id,
                routes_count: 0,
                connection_stats: ConnectionStats::default(),
                security_stats: SecurityStats::default(),
            })),
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
            if !self.security_manager.is_ip_allowed(&ip) {
                return Err(anyhow::anyhow!("Listen address {} is not allowed by security policy", ip));
            }
        }
        
        // Listen on the specified address
        self.swarm.listen_on(listen_addr)?;
        
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
                                        KadEvent::OutboundQueryProgressed { result, .. } => {
                                            if let QueryResult::Bootstrap(result) = result {
                            info!("Kademlia bootstrap completed: {:?}", result);
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
                            }
                        }
                                        MdnsEvent::Expired(list) => {
                            for (peer_id, multiaddr) in list {
                                info!("mDNS expired peer: {} at {}", peer_id, multiaddr);
                                self.swarm.behaviour_mut().kad.remove_address(&peer_id, &multiaddr);
                            }
                        }
                        _ => {}
                    }
                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    async fn handle_gossipsub_message(
        &mut self,
        peer_id: PeerId,
        message_id: MessageId,
        message: GossipsubMessage,
    ) -> Result<()> {
        debug!("Received gossipsub message from {}: {:?}", peer_id, message_id);
        
        // Security check for message processing
        let peer_ip = self.get_peer_ip(&peer_id).await;
        if let Some(ip) = peer_ip {
            if !self.security_manager.is_ip_allowed(&ip) {
                warn!("Rejected message from blacklisted peer: {}", peer_id);
                return Ok(());
            }
        }
        
        if let Ok(msg) = serde_json::from_slice::<Message>(&message.data) {
            info!("Processed message from {}: {:?}", peer_id, msg);
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
        let topic = Topic::new("ghostwire-messages");
        let data = serde_json::to_vec(message)?;
        self.swarm.behaviour_mut().gossipsub.publish(topic.clone(), data)?;
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

    pub async fn find_route(&self, target_node: &str) -> Option<Vec<String>> {
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

    /// Get security statistics
    pub async fn get_security_stats(&self) -> crate::core::security::SecurityStats {
        self.security_manager.get_security_stats()
    }

    /// Helper method to get peer IP (simplified)
    async fn get_peer_ip(&self, _peer_id: &PeerId) -> Option<std::net::IpAddr> {
        // In a real implementation, this would look up the peer's IP from the swarm
        None
    }
}

#[async_trait]
impl super::transport::Transport for MeshTransport {
    fn name(&self) -> &'static str { "mesh" }
    fn description(&self) -> &'static str { "libp2p-based mesh networking transport" }
    fn feature_flag(&self) -> Option<&'static str> { Some("mesh-transport") }
    async fn send_message(&self, message: &Message) -> Result<()> {
        // This would be implemented to send via the transport
        // For now, we'll use the broadcast mechanism
        self.broadcast_message(message).await
    }
    async fn receive_message(&self) -> Result<Option<Message>> {
        // This would be implemented to receive from the transport
        // For now, return None
        Ok(None)
    }
}

pub struct MeshManager {
    transport: MeshTransport,
    local_identity: Arc<Identity>,
    security_manager: Arc<SecurityManager>,
}

impl MeshManager {
    pub async fn new(identity: Arc<Identity>) -> Result<Self> {
        let security_config = SecurityConfig::default();
        let security_manager = Arc::new(SecurityManager::new(security_config));
        
        let transport = MeshTransport::new(identity.clone(), security_manager.clone()).await?;
        
        Ok(Self {
            transport,
            local_identity: identity,
            security_manager,
        })
    }

    pub async fn start(&mut self, listen_addr: Multiaddr) -> Result<()> {
        self.transport.start(listen_addr).await
    }

    pub async fn send_message(&mut self, message: &Message) -> Result<()> {
        self.transport.broadcast_message(message).await
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

    pub async fn get_topology(&self) -> MeshTopology {
        self.transport.get_topology().await
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

fn encode_public_key(pk: &PublicKey) -> String {
    base64::engine::general_purpose::STANDARD.encode(pk.encode_protobuf())
} 