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
use uuid::Uuid;

#[cfg(feature = "reticulum")]
use rns_transport::hash::AddressHash;
#[cfg(feature = "reticulum")]
use rns_transport::identity::PrivateIdentity;
#[cfg(feature = "reticulum")]
use rns_transport::packet::{
    ContextFlag, DestinationType, Header, IfacFlag, Packet, PacketContext, PacketDataBuffer,
    PacketType, PropagationType,
};
#[cfg(feature = "reticulum")]
use rns_transport::transport::{ReceivedData, Transport, TransportConfig};

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
}

// ---------------------------------------------------------------------------
// Real Reticulum transport (feature = "reticulum")
// ---------------------------------------------------------------------------
#[cfg(feature = "reticulum")]
pub struct ReticulumTransport {
    transport: Transport,
    local_address: String,
    local_address_hash: AddressHash,
    rx: mpsc::Receiver<Message>,
    _rx_task: tokio::task::JoinHandle<()>,
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

        let _rns_config = rns_transport::config::Config::from_ini(interfaces_ini)?;

        // ponytail: wrap transport creation + announce in a timeout.
        // rmap.world:4242 TCP can hang forever (firewall/stateful inspection issues).
        // Without timeout, a single unreachable backbone blocks the entire node startup.
        let init_timeout = std::time::Duration::from_secs(15);

        let (transport, mut rx_data) = match tokio::time::timeout(init_timeout, async {
            let t = Transport::new(config);
            {
                let iface_mgr = t.iface_manager();
                let mut mgr = iface_mgr.lock().await;
                for line in interfaces_ini.lines() {
                    let line = line.trim();
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
                    }
                }
            }

            let mut rx_data = t.received_data_events();

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
                    "Reticulum transport init timed out after {}s — node will run without RNS backbone",
                    init_timeout.as_secs()
                );
                // Create a minimal transport without backbone so the node can still run
                // with AutoInterface-only for local LAN discovery
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
            transport,
            local_address,
            local_address_hash,
            rx,
            _rx_task: rx_task,
        })
    }

    pub async fn send_message(&mut self, message: &Message) -> Result<()> {
        let payload = serde_json::to_vec(message)?;

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

        self.transport.send_packet(packet).await;
        info!("Sent message {} via Reticulum", message.id);
        Ok(())
    }

    pub async fn send_broadcast(&self, message: &Message) -> Result<()> {
        let payload = serde_json::to_vec(message)?;

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

        self.transport.send_broadcast(packet, None).await;
        info!("Broadcast message {} via Reticulum", message.id);
        Ok(())
    }

    pub async fn receive_message(&mut self) -> Result<Option<Message>> {
        match self.rx.try_recv() {
            Ok(msg) => Ok(Some(msg)),
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

    pub async fn stop(&self) -> Result<()> {
        // Transport's CancellationToken fires on drop, cleaning up interface tasks
        info!("Reticulum transport stopped");
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

    pub async fn send_broadcast(&self, _message: &Message) -> Result<()> {
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
    if bytes.is_empty() {
        return None;
    }
    match serde_json::from_slice::<Message>(bytes) {
        Ok(msg) => Some(msg),
        Err(e) => {
            warn!("Failed to decode Reticulum payload as Message: {}", e);
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

// ---------------------------------------------------------------------------
// Manager — orchestrates transport lifecycle
// ---------------------------------------------------------------------------
pub struct ReticulumManager {
    _identity: Arc<Identity>,
    topology: Arc<RwLock<ReticulumTopology>>,
    stats: Arc<RwLock<ReticulumStats>>,
    transport: Arc<RwLock<ReticulumTransport>>,
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
            ReticulumTransport::new(interfaces_ini, destination_name, transport_seed).await?;

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
        }));

        Ok(Self {
            _identity: identity,
            topology,
            stats,
            transport: Arc::new(RwLock::new(transport)),
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
        let transport = self.transport.read().await;
        transport.send_broadcast(message).await?;

        let mut stats = self.stats.write().await;
        stats.messages_relayed += 1;
        Ok(())
    }

    pub async fn receive_message(&self) -> Result<Option<Message>> {
        let mut transport = self.transport.write().await;
        transport.receive_message().await
    }

    pub async fn get_stats(&self) -> ReticulumStats {
        let mut stats = self.stats.read().await.clone();
        let topology = self.topology.read().await;
        stats.total_nodes = topology.nodes.len();
        stats.online_nodes = topology.nodes.values().filter(|n| n.is_online).count();
        stats.routes_count = topology.routes.len();
        stats.onion_routes_count = topology.onion_routes.len();
        stats
    }

    pub async fn get_topology(&self) -> ReticulumTopology {
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

    pub async fn stop(&self) -> Result<()> {
        let transport = self.transport.read().await;
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
    fn reticulum_stats_default() {
        let stats = ReticulumStats {
            total_nodes: 0,
            online_nodes: 0,
            local_node_id: "test".into(),
            routes_count: 0,
            onion_routes_count: 0,
            messages_relayed: 0,
            encryption_errors: 0,
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
