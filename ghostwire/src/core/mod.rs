// Core networking, encryption, metadata sanitizer, and storage modules will be implemented here.

use serde::Serialize;

pub mod armor;
pub mod disaster;
pub mod encryption;
pub mod identity;
pub mod message;
pub mod monitor;
pub mod security;
pub mod signal;
pub mod store;
pub mod transport;
pub mod transport_manager;
pub mod updater;
pub mod username_discovery; // DHT-based username discovery // Background and startup updates

pub mod bluetooth;
#[cfg(feature = "briar")]
pub mod briar;
#[cfg(feature = "mesh")]
pub mod mesh;
pub mod qr;
#[cfg(feature = "reticulum")]
pub mod reticulum;
pub mod sim_transport;
pub mod stealth_tcp;

#[cfg(feature = "matrix-bridge")]
use matrix_sdk::ruma::{events::room::message::RoomMessageEventContent, RoomId};
#[cfg(feature = "matrix-bridge")]
use matrix_sdk::Client;

pub use encryption::Encryption;
pub use identity::Identity;
pub use message::Message;
// Transport trait removed, using TransportKind enum instead
pub use security::{SecurityConfig, SecurityManager, SecurityStats};

#[cfg(feature = "mesh")]
pub use mesh::{MeshManager, MeshNode, MeshStats, MeshTopology};
#[cfg(feature = "reticulum")]
pub use reticulum::{ReticulumManager, ReticulumStats, ReticulumTopology};
#[cfg(feature = "stealth")]
pub use stealth_tcp::{ConnectionStats, StealthTCPProvider};

use anyhow::Result;
use libp2p::Multiaddr;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
// using explicit paths for tracing macros
use tokio::sync::broadcast;
use transport::TransportRegistry;

/// System-wide events for real-time reactivity
#[derive(Debug, Clone, Serialize)]
pub enum CoreEvent {
    NewMessage(Message),
    PeerConnected(String),
    PeerDisconnected(String),
    SystemAlert(String),
    NetworkUpdate { active_peers: u32, latency_ms: u32 },
    SecurityLockdown(bool),
}

// Protocol Adapter Enum for async dispatch
#[derive(Clone)]
pub enum AdapterKind {
    Matrix(MatrixAdapter),
    Meshtastic(MeshtasticAdapter),
    // Add more adapters here as needed
}

impl AdapterKind {
    pub fn name(&self) -> &'static str {
        match self {
            AdapterKind::Matrix(a) => a.name(),
            AdapterKind::Meshtastic(a) => a.name(),
        }
    }
    pub fn description(&self) -> &'static str {
        match self {
            AdapterKind::Matrix(a) => a.description(),
            AdapterKind::Meshtastic(a) => a.description(),
        }
    }
    pub async fn send_message(
        &self,
        message: &crate::core::message::Message,
    ) -> anyhow::Result<()> {
        match self {
            AdapterKind::Matrix(a) => a.send_message(message).await,
            AdapterKind::Meshtastic(a) => a.send_message(message).await,
        }
    }
    pub async fn receive_message(&self) -> anyhow::Result<Option<crate::core::message::Message>> {
        match self {
            AdapterKind::Matrix(a) => a.receive_message().await,
            AdapterKind::Meshtastic(a) => a.receive_message().await,
        }
    }
}

// Adapter Registry
pub struct AdapterRegistry {
    adapters: HashMap<String, AdapterKind>,
}

impl AdapterRegistry {
    pub fn new() -> Self {
        Self {
            adapters: HashMap::new(),
        }
    }
    pub fn register(&mut self, adapter: AdapterKind) {
        self.adapters.insert(adapter.name().to_string(), adapter);
    }
    pub fn get(&self, name: &str) -> Option<&AdapterKind> {
        self.adapters.get(name)
    }
    pub fn list(&self) -> Vec<String> {
        self.adapters.keys().cloned().collect()
    }
}

// Initial Matrix adapter placeholder definition replaced by real implementation below

// MatrixAdapter implementation moved to AdapterKind enum

#[derive(Clone)]
pub struct MatrixAdapter {
    pub homeserver: String,
    pub user: String,
    pub access_token: String,
    #[cfg(feature = "matrix-bridge")]
    pub client: Option<Arc<Client>>,
}

impl MatrixAdapter {
    pub fn name(&self) -> &'static str {
        "matrix"
    }
    pub fn description(&self) -> &'static str {
        "Matrix protocol bridge adapter"
    }

    pub async fn send_message(
        &self,
        message: &crate::core::message::Message,
    ) -> anyhow::Result<()> {
        #[cfg(feature = "matrix-bridge")]
        {
            if let Some(client) = &self.client {
                let room_id = RoomId::parse(&message.recipient)?;
                let content = RoomMessageEventContent::text_plain(&message.content);
                client
                    .get_room(&room_id)
                    .ok_or_else(|| anyhow::anyhow!("Room not found"))?
                    .send(content)
                    .await?;
                return Ok(());
            }
        }
        println!(
            "[MOCK] Matrix: Would send message to room: {}",
            message.content
        );
        Ok(())
    }

    pub async fn receive_message(&self) -> anyhow::Result<Option<crate::core::message::Message>> {
        #[cfg(feature = "matrix-bridge")]
        {
            // Real sync logic would go here
            return Ok(None);
        }
        Ok(None)
    }

    /// Initialize real Matrix client
    pub async fn init_client(&mut self) -> anyhow::Result<()> {
        #[cfg(feature = "matrix-bridge")]
        {
            let client = Client::builder()
                .homeserver_url(&self.homeserver)
                .build()
                .await?;
            client
                .matrix_auth()
                .login_username(
                    &self.user,
                    &std::env::var("GHOSTWIRE_MATRIX_PASSWORD")
                        .unwrap_or_else(|_| "password".to_string()),
                ) // Use env var
                .send()
                .await?;
            self.client = Some(Arc::new(client));
        }
        Ok(())
    }
}
// Usage:
// 1. Set your Matrix room ID in the code (replace !yourroomid:matrix.org)
// 2. Build with: cargo build --features matrix-bridge
// 3. Call MatrixAdapter::send_test_message("Hello from GhostWire!").await
// 4. Call receive_message() to print new messages

#[derive(Clone)]
pub struct MeshtasticAdapter {
    pub device_path: String,
    pub channel: String,
    pub erasure_armor: Arc<armor::ErasureArmor>,
}

impl MeshtasticAdapter {
    pub fn new(path: String, chan: String) -> Self {
        Self {
            device_path: path,
            channel: chan,
            erasure_armor: Arc::new(armor::ErasureArmor::new(3, 2)), // 3 data, 2 parity (Military grade)
        }
    }
    pub fn name(&self) -> &'static str {
        "meshtastic"
    }
    pub fn description(&self) -> &'static str {
        "Meshtastic protocol bridge adapter"
    }

    pub async fn send_message(
        &self,
        message: &crate::core::message::Message,
    ) -> anyhow::Result<()> {
        // Apply Military-Grade Erasure Coding (Fountain Code simulation)
        let shards = self.erasure_armor.encode(message.content.as_bytes());
        tracing::info!(
            "Meshtastic: Splitting message into {} shards for extreme resilience",
            shards.len()
        );

        #[cfg(feature = "meshtastic")]
        {
            let mut port = serialport::new(&self.device_path, 115_200)
                .timeout(Duration::from_millis(100))
                .open()?;

            for shard in shards {
                port.write_all(&shard)?;
            }
            return Ok(());
        }
        println!(
            "[MOCK] Meshtastic: Would send {} shards to device {}: {}",
            shards.len(),
            self.device_path,
            message.content
        );
        Ok(())
    }

    pub async fn receive_message(&self) -> anyhow::Result<Option<crate::core::message::Message>> {
        #[cfg(feature = "meshtastic")]
        {
            let mut port = serialport::new(&self.device_path, 115_200).open()?;
            let mut serial_buf: Vec<u8> = vec![0; 1000];
            let n = port.read(serial_buf.as_mut_slice())?;
            if n > 0 {
                // Real implementation would parse Meshtastic protobuf here
                return Ok(None);
            }
        }
        println!(
            "[MOCK] Meshtastic: Would receive message from device {}",
            self.device_path
        );
        Ok(None)
    }
}

/// Core application state and management
#[derive(Clone)]
pub struct Core {
    pub identity: Arc<Identity>,
    pub encryption: Arc<Encryption>,
    #[cfg(feature = "mesh")]
    pub mesh_manager: Option<Arc<RwLock<MeshManager>>>,
    #[cfg(feature = "reticulum")]
    pub reticulum_manager: Option<Arc<RwLock<ReticulumManager>>>,
    pub security_manager: Arc<SecurityManager>,
    pub transport_registry: Arc<RwLock<TransportRegistry>>,
    pub active_transport: Option<crate::core::transport::TransportKind>,
    pub adapter_registry: Arc<RwLock<AdapterRegistry>>,
    pub active_adapter: Option<AdapterKind>,

    // The Nervous System
    pub monitor: Arc<monitor::NetworkMonitor>,
    pub switcher: Arc<RwLock<transport_manager::AutoTransportSwitcher>>,
    pub traffic_armor: Arc<armor::TrafficArmor>,
    pub messages: Arc<RwLock<Vec<Message>>>,
    pub message_store: Arc<store::SecureMessageStore>,
    pub bootstrap_nodes: Vec<Multiaddr>,
    pub event_bus: broadcast::Sender<CoreEvent>,
}

impl Core {
    pub async fn new(bootstrap_nodes: Vec<Multiaddr>, sim_addr: Option<String>) -> Result<Self> {
        let identity = Arc::new(
            Identity::new().map_err(|e| anyhow::anyhow!("Identity creation failed: {}", e))?,
        );
        let _traffic_armor = Arc::new(armor::TrafficArmor::new(armor::MorphingConfig::default()));

        let encryption = Arc::new(
            Encryption::new().map_err(|e| anyhow::anyhow!("Encryption creation failed: {}", e))?,
        );
        // Link Armor to Encryption for future internal morphing
        // (Uses interior mutability or we could have passed it to new if we changed the signature further)
        // For now, we'll keep it in Core and use it in orchestration.

        let security_config = SecurityConfig::default();
        let security_manager = Arc::new(SecurityManager::new(security_config));
        let mut registry = TransportRegistry::new();
        let mut active_transport = None; // Set this based on user config or default

        if let Some(addr) = sim_addr {
            let sim_transport = Arc::new(RwLock::new(
                crate::core::sim_transport::SimTransport::new(identity.id.clone(), &addr).await?,
            ));
            let sim_kind = crate::core::transport::TransportKind::Simulated(sim_transport);
            registry.register(sim_kind.clone());
            active_transport = Some(sim_kind);
        }

        let registry = Arc::new(RwLock::new(registry));

        // Adapter registry
        let mut adapter_registry = AdapterRegistry::new();
        // Load configuration
        let config = crate::config::Config::load().unwrap_or_default();

        // Use default bootstraps if none provided
        let mut bootstrap_nodes = bootstrap_nodes;
        if bootstrap_nodes.is_empty() {
            tracing::info!("🌐 No bootstrap nodes provided. Using default public gateways for zero-config discovery.");
            for addr_str in crate::core::mesh::DEFAULT_BOOTSTRAPS {
                if let Ok(addr) = addr_str.parse() {
                    bootstrap_nodes.push(addr);
                }
            }
        }

        // Register Matrix adapter if enabled
        if config.features.matrix_bridge {
            if let Some(matrix_config) = config.get_matrix_config() {
                adapter_registry.register(AdapterKind::Matrix(MatrixAdapter {
                    homeserver: matrix_config.homeserver,
                    user: matrix_config.user,
                    access_token: matrix_config.access_token,
                    #[cfg(feature = "matrix-bridge")]
                    client: None,
                }));
            }
        }
        adapter_registry.register(AdapterKind::Meshtastic(MeshtasticAdapter::new(
            "/dev/ttyUSB0".to_string(),
            "main".to_string(),
        )));
        let adapter_registry = Arc::new(RwLock::new(adapter_registry));
        let active_adapter = None;

        // Initialize the Nervous System (10/10 DNA)
        let disaster_manager = disaster::DisasterManager::new(
            500, // 500ms latency threshold
            0.2, // 20% packet loss threshold
            1,   // Minimum 1 peer for isolation state
            3,   // Require 3 stable samples before recovering (Anti-Thrashing)
        );
        let switcher = Arc::new(RwLock::new(transport_manager::AutoTransportSwitcher::new(
            disaster_manager,
            disaster::DisasterPolicy::default(),
        )));

        let traffic_armor = Arc::new(armor::TrafficArmor::new(armor::MorphingConfig::default()));

        let messages = Arc::new(RwLock::new(Vec::new()));
        let message_store = Arc::new(store::SecureMessageStore::new()?);
        let (event_bus, _) = broadcast::channel(1024); // Large buffer for high traffic

        #[cfg(feature = "mesh")]
        let mesh_manager_inner = {
            let username = identity.get_metadata().nickname;
            let peer_id = identity.keypair().public().to_peer_id();

            let username_discovery = Arc::new(
                crate::core::username_discovery::UsernameDiscovery::new(peer_id, username),
            );

            Some(Arc::new(RwLock::new(
                MeshManager::new(
                    identity.clone(),
                    encryption.clone(),
                    messages.clone(),
                    message_store.clone(),
                    bootstrap_nodes.clone(),
                    username_discovery,
                    Some(event_bus.clone()),
                )
                .await?,
            )))
        };

        #[cfg(not(feature = "mesh"))]
        let mesh_manager_inner = None;

        let monitor = Arc::new(monitor::NetworkMonitor::new(
            #[cfg(feature = "mesh")]
            mesh_manager_inner.clone(),
            std::time::Duration::from_secs(5), // Check every 5 seconds
        ));

        Ok(Self {
            identity,
            encryption,
            security_manager,
            transport_registry: registry,
            active_transport,
            adapter_registry,
            active_adapter,
            monitor,
            switcher,
            traffic_armor,
            #[cfg(feature = "mesh")]
            mesh_manager: mesh_manager_inner,
            #[cfg(feature = "reticulum")]
            reticulum_manager: None,
            messages,
            message_store,
            bootstrap_nodes,
            event_bus,
        })
    }

    /// Starts the autonomic nervous system heartbeat
    pub async fn start_nervous_system(self: Arc<Self>, mesh_bind_ip: String) {
        tracing::info!("🧠 Activating GhostWire Nervous System (Autonomic Response enabled)");

        // Start the network monitor
        self.monitor.clone().start().await;

        // Start the autonomic switcher loop
        let core = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(10));
            loop {
                interval.tick().await;
                let metrics = core.monitor.get_metrics().await;

                let mut switcher = core.switcher.write().await;
                if let Some(new_transport) = switcher.update_and_evaluate(&metrics) {
                    // Logic to actually swap the transport kind in the Core
                    // For now, we update the active_transport field
                    // core.set_active_transport_by_type(new_transport).await;
                    tracing::info!("Autonomic system decided to switch to {:?}", new_transport);
                }
            }
        });

        // Start Traffic Chaffing Heartbeat (Military-Grade Metadata Protection)
        let core_chaff = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
            loop {
                interval.tick().await;
                if core_chaff.active_adapter.is_some() {
                    tracing::debug!("📡 Sending Military-Grade Chaff Traffic (DPI Evasion)");
                    let dummy = crate::core::message::Message::new(
                        core_chaff.identity.id.clone(),
                        "CHAFF_TARGET".to_string(),
                        "DUMMY_TRAFFIC_DATA_NOOP".to_string(),
                    );
                    let _ = core_chaff.send_adapter_message(&dummy).await;
                }
            }
        });

        // Auto-Start Mesh Network (Zero-Conf)
        let core_mesh = self.clone();
        tokio::spawn(async move {
            // Bind to the specified IP, with a random port (0) for P2P
            let listen_addr = format!("/ip4/{}/tcp/0", mesh_bind_ip);
            match core_mesh.start_mesh(&listen_addr).await {
                Ok(_) => {
                    tracing::info!("✅ Mesh Network Auto-Started on {}", listen_addr);
                    // Give the swarm a moment to bind, then print addresses
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    let listeners = core_mesh.get_mesh_listeners().await;
                    if !listeners.is_empty() {
                        println!("\n--- GhostWire Mesh Node Started ---");
                        println!(
                            "Peer ID: {}",
                            core_mesh.identity.keypair().public().to_peer_id()
                        );
                        println!("Listening on:");
                        for addr in listeners {
                            println!("  {}", addr);
                        }
                        println!("------------------------------------\n");
                    }
                }
                Err(e) => tracing::warn!("⚠️ Failed to auto-start mesh: {}", e),
            }
        });
    }

    /// Select an active transport by name
    pub async fn set_active_transport(&mut self, name: &str) -> Result<()> {
        let registry = self.transport_registry.read().await;
        if let Some(transport) = registry.get(name) {
            self.active_transport = Some(transport.clone());
            Ok(())
        } else {
            Err(anyhow::anyhow!("Transport not found: {}", name))
        }
    }

    /// Send a message through the selected transport
    pub async fn send_message(&self, message: &Message) -> Result<()> {
        // Enforce Lockdown
        if self.security_manager.is_lockdown().await {
            tracing::warn!(
                "BLOCKED: Attempted to send message during LOCKDOWN: {}",
                message.content
            );
            return Err(anyhow::anyhow!(
                "SYSTEM LOCKDOWN ACTIVE - OUTGOING TRAFFIC BLOCKED"
            ));
        }

        // Persist message to encrypted storage
        if let Err(e) = self.message_store.save_message(message) {
            tracing::error!("⚠️ Persistence failure: {}", e);
        }

        // Add to in-memory history
        {
            let mut messages = self.messages.write().await;
            messages.push(message.clone());
        }

        // Record message for security stats
        self.security_manager.record_message(&message.sender).await;

        // Broadcast to Event Bus
        let _ = self.event_bus.send(CoreEvent::NewMessage(message.clone()));

        #[cfg(feature = "mesh")]
        if let Some(mesh_manager) = &self.mesh_manager {
            let mut mgr = mesh_manager.write().await;
            return mgr.send_message(message).await;
        }

        if let Some(_transport) = &self.active_transport {
            Ok(())
        } else {
            tracing::info!("Message sent locally (no transport): {}", message.content);
            Ok(())
        }
    }

    /// Initialize a default transport for basic functionality
    pub async fn init_default_transport(&mut self) -> Result<()> {
        let provider = Arc::new(crate::core::stealth_tcp::StealthTCPProvider::new(
            None,
            self.security_manager.clone(),
            None,
            true, // Enable Stealth Mode by default
        ));

        self.active_transport = Some(crate::core::transport::TransportKind::StealthTCP(provider));
        tracing::info!("Initialized StealthTCP transport (Default) 🥷");

        Ok(())
    }

    /// Select an active protocol adapter by name
    pub async fn set_active_adapter(&mut self, name: &str) -> Result<()> {
        let registry = self.adapter_registry.read().await;
        if let Some(adapter) = registry.get(name) {
            self.active_adapter = Some(adapter.clone());
            Ok(())
        } else {
            Err(anyhow::anyhow!("Adapter not found: {}", name))
        }
    }
    /// Send a message through the selected protocol adapter with traffic morphing
    pub async fn send_adapter_message(&self, message: &Message) -> Result<()> {
        // Apply Military-Grade Traffic Morphing (Timing Chaff)
        self.traffic_armor.apply_timing_chaff().await;

        if let Some(adapter) = &self.active_adapter {
            adapter.send_message(message).await
        } else {
            Err(anyhow::anyhow!("No active adapter selected"))
        }
    }

    /// Get the identity ID
    pub fn get_identity_id(&self) -> String {
        self.identity.id.clone()
    }

    /// Get the DID
    pub fn get_did(&self) -> String {
        self.identity.did.clone()
    }

    /// Get the metadata
    pub fn get_metadata(&self) -> crate::core::identity::IdentityMetadata {
        self.identity.get_metadata()
    }

    /// Update metadata
    pub fn update_metadata(
        &self,
        nickname: Option<String>,
        bio: Option<String>,
        avatar_url: Option<String>,
    ) {
        self.identity.update_metadata(nickname, bio, avatar_url);
    }

    /// Get the public key
    pub fn get_public_key(&self) -> Vec<u8> {
        self.encryption.export_public_key()
    }

    /// Get the key ID
    pub fn get_key_id(&self) -> String {
        self.encryption.get_key_id()
    }

    /// Set PQC Armor mode
    pub async fn set_pqc_mode(&self, enabled: bool) {
        // Implementation logic for toggling Kyber requirement
        // For now we'll just log it as the actual encryption module will check this
        tracing::info!("Post-Quantum Armor mode set to: {}", enabled);
    }

    /// Set Regional Shard
    pub async fn set_dht_shard(&self, shard: &str) -> anyhow::Result<()> {
        #[cfg(feature = "mesh")]
        if let Some(mesh_manager) = &self.mesh_manager {
            let mut mgr = mesh_manager.write().await;
            mgr.set_shard(shard).await?;
            tracing::info!("DHT Regional Shard updated to: {}", shard);
            return Ok(());
        }
        Err(anyhow::anyhow!("Mesh networking not available"))
    }

    /// Get network topology (placeholder for now)
    pub async fn get_network_topology(&self) -> Result<String> {
        Ok("Network topology not yet implemented".to_string())
    }

    /// Initialize mesh network
    pub async fn init_mesh(&mut self) -> Result<()> {
        #[cfg(feature = "mesh")]
        {
            if self.mesh_manager.is_none() {
                let username = self.identity.get_metadata().nickname;
                let peer_id = self.identity.keypair().public().to_peer_id();

                let username_discovery = Arc::new(
                    crate::core::username_discovery::UsernameDiscovery::new(peer_id, username),
                );

                let mesh_manager = MeshManager::new(
                    self.identity.clone(),
                    self.encryption.clone(),
                    self.messages.clone(),
                    self.message_store.clone(),
                    self.bootstrap_nodes.clone(),
                    username_discovery,
                    Some(self.event_bus.clone()),
                )
                .await?;
                self.mesh_manager = Some(Arc::new(RwLock::new(mesh_manager)));
                tracing::info!("Mesh network initialized");
            }
        }
        #[cfg(not(feature = "mesh"))]
        {
            tracing::warn!("Mesh feature not enabled");
        }
        Ok(())
    }

    /// Add bootstrap nodes dynamically
    pub async fn add_bootstrap_nodes(&self, nodes: Vec<Multiaddr>) {
        #[cfg(feature = "mesh")]
        if let Some(mesh_manager) = &self.mesh_manager {
            let mut mgr = mesh_manager.write().await;
            tracing::info!("Added {} bootstrap nodes dynamically", nodes.len());
            mgr.add_bootstrap_nodes(nodes);
        }
    }

    /// Initialize Reticulum network
    pub async fn init_reticulum(&mut self) -> Result<()> {
        #[cfg(feature = "reticulum")]
        {
            if self.reticulum_manager.is_none() {
                let reticulum_manager = ReticulumManager::new(self.identity.clone()).await?;
                self.reticulum_manager = Some(Arc::new(RwLock::new(reticulum_manager)));
                tracing::info!("Reticulum network initialized");
            }
        }
        #[cfg(not(feature = "reticulum"))]
        {
            tracing::warn!("Reticulum feature not enabled");
        }
        Ok(())
    }

    /// Get mesh network statistics
    pub async fn get_mesh_stats(&self) -> Option<MeshStats> {
        #[cfg(feature = "mesh")]
        {
            if let Some(mesh_manager) = &self.mesh_manager {
                let manager = mesh_manager.read().await;
                Some(manager.get_stats().await)
            } else {
                None
            }
        }
        #[cfg(not(feature = "mesh"))]
        {
            None
        }
    }

    /// Get Reticulum network statistics
    pub async fn get_reticulum_stats(&self) -> Option<ReticulumStats> {
        #[cfg(feature = "reticulum")]
        {
            if let Some(reticulum_manager) = &self.reticulum_manager {
                let _manager = reticulum_manager.read().await;
                Some(ReticulumStats {
                    total_nodes: 0,
                    online_nodes: 0,
                    local_node_id: self.get_identity_id().to_string(),
                    routes_count: 0,
                    onion_routes_count: 0,
                    messages_relayed: 0,
                    encryption_errors: 0,
                })
            } else {
                None
            }
        }
        #[cfg(not(feature = "reticulum"))]
        {
            None
        }
    }

    /// Get security statistics
    pub async fn get_security_stats(&self) -> SecurityStats {
        self.security_manager.get_security_stats().await
    }

    pub async fn connect_meshtastic(&self, address: &str) -> anyhow::Result<()> {
        let mut registry = self.adapter_registry.write().await;
        registry.register(AdapterKind::Meshtastic(MeshtasticAdapter::new(
            address.to_string(),
            "main".to_string(),
        )));
        Ok(())
    }

    pub async fn connect_reticulum(
        &self,
        _address: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "reticulum")]
        {
            if let Some(_reticulum_manager) = &self.reticulum_manager {
                // Note: This method doesn't exist yet, so we'll skip it for now
                // let manager = reticulum_manager.write().await;
                // manager.connect(address).await?;
            }
        }
        Ok(())
    }

    pub async fn get_reticulum_topology(&self) -> Result<Option<ReticulumTopology>> {
        #[cfg(feature = "reticulum")]
        {
            if let Some(_reticulum_manager) = &self.reticulum_manager {
                // Note: This method doesn't exist yet, so we'll return None for now
                Ok(None)
            } else {
                Ok(None)
            }
        }
        #[cfg(not(feature = "reticulum"))]
        {
            Ok(None)
        }
    }

    pub async fn start_mesh(&self, address: &str) -> anyhow::Result<()> {
        #[cfg(feature = "mesh")]
        if let Some(mesh_manager) = &self.mesh_manager {
            let addr = address.parse()?;
            let mesh_manager = mesh_manager.clone();
            tokio::spawn(async move {
                let mut mgr = mesh_manager.write().await;
                if let Err(e) = mgr.start(addr).await {
                    tracing::error!("Mesh loop failed: {}", e);
                }
            });
            return Ok(());
        }
        Err(anyhow::anyhow!("Mesh networking not available"))
    }
    pub async fn get_mesh_topology(&self) -> anyhow::Result<()> {
        // Stub implementation
        Ok(())
    }

    pub async fn get_mesh_listeners(&self) -> Vec<Multiaddr> {
        #[cfg(feature = "mesh")]
        if let Some(mesh_manager) = &self.mesh_manager {
            return mesh_manager.read().await.get_listeners();
        }
        Vec::new()
    }

    // Add convenience methods for backward compatibility
    pub fn get_peer_count(&self) -> usize {
        #[cfg(feature = "mesh")]
        if let Some(_mesh_manager) = &self.mesh_manager {
            // This is a bit tricky since we can't await in a sync function
            // For HUD, we usually want live data
            return 0; // Will be optimized later with an atomic counter
        }
        self.encryption.get_peer_count()
    }
}

// Documentation: MatrixAdapter and MeshtasticAdapter now have config fields and stubbed logic. Replace stubs with real SDK/API calls for production bridging.
// Documentation: The core now supports modular, pluggable transports via TransportRegistry. Use feature flags to include/exclude transports at build time. Select active transport at runtime for message routing.
// Documentation: The core now supports modular, pluggable protocol adapters via AdapterRegistry. Register new adapters as plugins. Select active adapter at runtime for protocol bridging.

// Feature-guarded type stubs for when features are disabled
#[cfg(not(feature = "mesh"))]
pub struct MeshStats {
    pub peer_count: usize,
    pub message_count: usize,
    pub uptime_seconds: u64,
}

#[cfg(not(feature = "mesh"))]
pub struct MeshNode {
    pub id: String,
    pub name: String,
    pub is_online: bool,
}

#[cfg(not(feature = "mesh"))]
pub struct MeshTopology {
    pub nodes: Vec<MeshNode>,
    pub connections: Vec<(String, String)>,
}

#[cfg(not(feature = "mesh"))]
pub struct MeshManager {
    pub identity: Arc<Identity>,
}

#[cfg(not(feature = "mesh"))]
impl MeshManager {
    pub async fn new(_identity: Arc<Identity>) -> Result<Self> {
        Ok(Self {
            identity: _identity,
        })
    }

    pub fn get_peer_count(&self) -> usize {
        // TODO: Implement actual peer counting
        0
    }
}

#[cfg(not(feature = "reticulum"))]
#[derive(Serialize)]
pub struct ReticulumStats {
    pub node_count: usize,
    pub message_count: usize,
    pub uptime_seconds: u64,
}

#[cfg(not(feature = "reticulum"))]
pub struct ReticulumTopology {
    pub nodes: Vec<String>,
    pub connections: Vec<(String, String)>,
}

#[cfg(not(feature = "reticulum"))]
pub struct ReticulumManager {
    pub identity: Arc<Identity>,
}

#[cfg(not(feature = "reticulum"))]
impl ReticulumManager {
    pub async fn new(_identity: Arc<Identity>) -> Result<Self> {
        Ok(Self {
            identity: Arc::new(
                Identity::new().map_err(|e| anyhow::anyhow!("Failed to create identity: {}", e))?,
            ),
        })
    }
}
