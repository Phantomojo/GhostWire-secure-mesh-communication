use crate::core::message::Message;
use crate::core::identity::Identity;
use anyhow::Result;
use async_trait::async_trait;
use futures_util::StreamExt;
use libp2p::{
    core::{upgrade, transport::Transport},
    gossipsub::{Gossipsub, GossipsubConfig, GossipsubEvent, GossipsubMessage, ValidationMode},
    identify::{Identify, IdentifyConfig, IdentifyEvent},
    kad::{Kademlia, KademliaConfig, KademliaEvent, QueryResult},
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    noise,
    swarm::{NetworkBehaviour, Swarm, SwarmEvent},
    tcp, yamux, PeerId, Multiaddr,
};
use meshtastic::{
    api::{StreamApi, ConnectedStreamApi},
    packet::{PacketDestination, PacketRouter},
    protobufs,
    types::{NodeId, MeshChannel},
    utils::stream::build_tcp_stream,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tracing::{info, warn, error, debug};
use uuid::Uuid;

/// Mesh network node information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshNode {
    pub id: String,
    pub peer_id: PeerId,
    pub address: Multiaddr,
    pub username: String,
    pub public_key: Vec<u8>,
    pub last_seen: u64,
    pub connection_quality: f32,
    pub is_online: bool,
}

/// Mesh network topology
#[derive(Debug, Clone)]
pub struct MeshTopology {
    pub nodes: HashMap<String, MeshNode>,
    pub routes: HashMap<String, Vec<String>>,
    pub local_node_id: String,
}

/// Mesh networking behavior combining libp2p and Meshtastic concepts
#[derive(NetworkBehaviour)]
pub struct MeshBehaviour {
    pub gossipsub: Gossipsub,
    pub identify: Identify,
    pub kad: Kademlia,
    pub mdns: Mdns,
}

/// Mesh transport implementation
pub struct MeshTransport {
    swarm: Swarm<MeshBehaviour>,
    node_id: String,
    identity: Arc<Identity>,
    topology: Arc<RwLock<MeshTopology>>,
    message_queue: mpsc::UnboundedReceiver<Message>,
    _sender: mpsc::UnboundedSender<Message>,
    meshtastic_api: Option<ConnectedStreamApi>,
}

impl MeshTransport {
    pub async fn new(identity: Arc<Identity>) -> Result<Self> {
        let node_id = Uuid::new_v4().to_string();
        
        // Create libp2p transport
        let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
            .into_authentic(&identity.get_keypair())
            .expect("Signing libp2p-noise static DH keypair failed.");

        let transport = tcp::TokioTcpConfig::new()
            .nodelay(true)
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::NoiseConfig::xx(noise_keys).into_authenticated())
            .multiplex(yamux::YamuxConfig::default())
            .boxed();

        // Create network behavior
        let mut gossipsub_config = GossipsubConfig::default();
        gossipsub_config.validation_mode = ValidationMode::Strict;
        gossipsub_config.message_id_fn = Arc::new(|message: &GossipsubMessage| {
            let mut s = DefaultHasher::new();
            message.data.hash(&mut s);
            libp2p::gossipsub::MessageId::from(s.finish().to_string())
        });

        let gossipsub = Gossipsub::new(
            libp2p::gossipsub::MessageAuthenticity::Signed(identity.get_keypair()),
            gossipsub_config,
        )?;

        let identify = Identify::new(IdentifyConfig::new(
            "/ghostwire/mesh/1.0.0".to_string(),
            identity.get_public_key(),
        ));

        let kad_config = KademliaConfig::default();
        let kad = Kademlia::new(PeerId::from_public_key(&identity.get_public_key()), kad_config);

        let mdns = Mdns::new(MdnsConfig::default())?;

        let behaviour = MeshBehaviour {
            gossipsub,
            identify,
            kad,
            mdns,
        };

        let peer_id = PeerId::from_public_key(&identity.get_public_key());
        let swarm = Swarm::new(transport, behaviour, peer_id);

        let topology = Arc::new(RwLock::new(MeshTopology {
            nodes: HashMap::new(),
            routes: HashMap::new(),
            local_node_id: node_id.clone(),
        }));

        let (sender, receiver) = mpsc::unbounded_channel();

        Ok(Self {
            swarm,
            node_id,
            identity,
            topology,
            message_queue: receiver,
            _sender: sender,
            meshtastic_api: None,
        })
    }

    /// Start the mesh network
    pub async fn start(&mut self, listen_addr: Multiaddr) -> Result<()> {
        info!("Starting mesh network on {}", listen_addr);
        
        // Listen on the specified address
        self.swarm.listen_on(listen_addr)?;
        
        // Start the swarm event loop
        self.run_swarm_loop().await?;
        
        Ok(())
    }

    /// Run the swarm event loop
    async fn run_swarm_loop(&mut self) -> Result<()> {
        loop {
            tokio::select! {
                swarm_event = self.swarm.next() => {
                    match swarm_event {
                        Some(SwarmEvent::NewListenAddr { address, .. }) => {
                            info!("Listening on {}", address);
                        }
                        Some(SwarmEvent::Behaviour(MeshBehaviourEvent::Gossipsub(GossipsubEvent::Message {
                            propagation_source: peer_id,
                            message_id: id,
                            message,
                        }))) => {
                            self.handle_gossipsub_message(peer_id, id, message).await?;
                        }
                        Some(SwarmEvent::Behaviour(MeshBehaviourEvent::Identify(IdentifyEvent::Received {
                            peer_id,
                            info,
                        }))) => {
                            self.handle_identify_info(peer_id, info).await?;
                        }
                        Some(SwarmEvent::Behaviour(MeshBehaviourEvent::Kad(KademliaEvent::OutboundQueryCompleted {
                            result: QueryResult::Bootstrap(result),
                            ..
                        }))) => {
                            info!("Kademlia bootstrap completed: {:?}", result);
                        }
                        Some(SwarmEvent::Behaviour(MeshBehaviourEvent::Mdns(MdnsEvent::Discovered(list)))) => {
                            for (peer_id, multiaddr) in list {
                                info!("mDNS discovered peer: {} at {}", peer_id, multiaddr);
                                self.swarm.behaviour_mut().kad.add_address(&peer_id, multiaddr);
                            }
                        }
                        Some(SwarmEvent::Behaviour(MeshBehaviourEvent::Mdns(MdnsEvent::Expired(list)))) => {
                            for (peer_id, multiaddr) in list {
                                info!("mDNS expired peer: {} at {}", peer_id, multiaddr);
                                self.swarm.behaviour_mut().kad.remove_address(&peer_id, &multiaddr);
                            }
                        }
                        _ => {}
                    }
                }
                message = self.message_queue.next() => {
                    if let Some(msg) = message {
                        self.broadcast_message(&msg).await?;
                    }
                }
            }
        }
    }

    /// Handle incoming gossipsub messages
    async fn handle_gossipsub_message(
        &mut self,
        peer_id: PeerId,
        message_id: libp2p::gossipsub::MessageId,
        message: GossipsubMessage,
    ) -> Result<()> {
        debug!("Received gossipsub message from {}: {:?}", peer_id, message_id);
        
        // Decode the message
        if let Ok(msg) = serde_json::from_slice::<Message>(&message.data) {
            // Update topology with sender info
            let mut topology = self.topology.write().await;
            if let Some(node) = topology.nodes.get_mut(&msg.sender_id) {
                node.last_seen = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                node.is_online = true;
            }
            
            info!("Received mesh message from {}: {}", msg.sender_id, msg.content);
        }
        
        Ok(())
    }

    /// Handle identify protocol info
    async fn handle_identify_info(&mut self, peer_id: PeerId, info: libp2p::identify::Info) -> Result<()> {
        debug!("Received identify info from {}: {:?}", peer_id, info);
        
        // Add discovered addresses to Kademlia
        for addr in info.listen_addrs {
            self.swarm.behaviour_mut().kad.add_address(&peer_id, addr);
        }
        
        Ok(())
    }

    /// Broadcast a message to the mesh network
    pub async fn broadcast_message(&mut self, message: &Message) -> Result<()> {
        let topic = libp2p::gossipsub::IdentTopic::new("ghostwire-messages");
        
        // Subscribe to the topic if not already subscribed
        self.swarm.behaviour_mut().gossipsub.subscribe(&topic)?;
        
        // Serialize and publish the message
        let data = serde_json::to_vec(message)?;
        let message_id = self.swarm.behaviour_mut().gossipsub.publish(topic, data)?;
        
        info!("Broadcasted message with ID: {:?}", message_id);
        Ok(())
    }

    /// Connect to a Meshtastic device
    pub async fn connect_meshtastic(&mut self, address: &str) -> Result<()> {
        info!("Connecting to Meshtastic device at {}", address);
        
        let stream = build_tcp_stream(address).await?;
        let (api, _packet_receiver) = StreamApi::new().connect(stream).await?;
        let configured_api = api.configure().await?;
        
        self.meshtastic_api = Some(configured_api);
        info!("Connected to Meshtastic device");
        
        Ok(())
    }

    /// Send message through Meshtastic mesh
    pub async fn send_meshtastic_message(&mut self, message: &Message) -> Result<()> {
        if let Some(api) = &mut self.meshtastic_api {
            let data = serde_json::to_vec(message)?;
            
            // Send as broadcast to the mesh
            api.send_mesh_packet(
                PacketDestination::Broadcast,
                MeshChannel::Primary,
                data.into(),
            ).await?;
            
            info!("Sent message through Meshtastic mesh");
        } else {
            warn!("No Meshtastic connection available");
        }
        
        Ok(())
    }

    /// Get current mesh topology
    pub async fn get_topology(&self) -> MeshTopology {
        self.topology.read().await.clone()
    }

    /// Find route to a specific node
    pub async fn find_route(&self, target_node: &str) -> Option<Vec<String>> {
        let topology = self.topology.read().await;
        topology.routes.get(target_node).cloned()
    }

    /// Update connection quality for a node
    pub async fn update_connection_quality(&mut self, node_id: &str, quality: f32) {
        let mut topology = self.topology.write().await;
        if let Some(node) = topology.nodes.get_mut(node_id) {
            node.connection_quality = quality;
        }
    }
}

#[async_trait]
impl super::transport::Transport for MeshTransport {
    async fn send_message(&self, message: &Message) -> Result<()> {
        // This would be called by the core system
        // For now, we'll use the internal sender
        if let Some(sender) = &self._sender {
            let _ = sender.send(message.clone());
        }
        Ok(())
    }

    async fn receive_message(&self) -> Result<Option<Message>> {
        // Messages are handled in the swarm loop
        // This could be enhanced to return messages from a queue
        Ok(None)
    }
}

/// Mesh network manager
pub struct MeshManager {
    transport: MeshTransport,
    local_identity: Arc<Identity>,
}

impl MeshManager {
    pub async fn new(identity: Arc<Identity>) -> Result<Self> {
        let transport = MeshTransport::new(identity.clone()).await?;
        
        Ok(Self {
            transport,
            local_identity: identity,
        })
    }

    /// Start the mesh network
    pub async fn start(&mut self, listen_addr: Multiaddr) -> Result<()> {
        self.transport.start(listen_addr).await
    }

    /// Connect to Meshtastic device
    pub async fn connect_meshtastic(&mut self, address: &str) -> Result<()> {
        self.transport.connect_meshtastic(address).await
    }

    /// Send message through mesh
    pub async fn send_message(&mut self, message: &Message) -> Result<()> {
        // Try libp2p mesh first
        self.transport.broadcast_message(message).await?;
        
        // Also try Meshtastic if available
        self.transport.send_meshtastic_message(message).await?;
        
        Ok(())
    }

    /// Get mesh statistics
    pub async fn get_stats(&self) -> MeshStats {
        let topology = self.transport.get_topology().await;
        
        MeshStats {
            total_nodes: topology.nodes.len(),
            online_nodes: topology.nodes.values().filter(|n| n.is_online).count(),
            local_node_id: topology.local_node_id,
            routes_count: topology.routes.len(),
        }
    }
}

/// Mesh network statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshStats {
    pub total_nodes: usize,
    pub online_nodes: usize,
    pub local_node_id: String,
    pub routes_count: usize,
}

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher}; 