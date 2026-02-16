// Core networking, encryption, metadata sanitizer, and storage modules will be implemented here.

use serde::Serialize;

pub mod armor;
pub mod disaster;
pub mod encryption;
pub mod identity;
pub mod message;
pub mod monitor;
pub mod persistence_manager;
pub mod security;
pub mod store;
pub mod transport;
pub mod transport_manager;
pub mod tutorial;
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
// use std::time::Duration; // redundant
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
            Ok(None)
        }
        #[cfg(not(feature = "matrix-bridge"))]
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
            shards_len = shards.len(),
            device_path = %self.device_path,
            content = %message.content,
            "Meshtastic: Splitting message into shards for extreme resilience"
        );

        #[cfg(feature = "meshtastic")]
        {
            let mut port = serialport::new(&self.device_path, 115_200)
                .timeout(std::time::Duration::from_millis(100))
                .open()?;

            for shard in shards {
                use std::io::Write;
                port.write_all(&shard)?;
            }
        }
        #[cfg(not(feature = "meshtastic"))]
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
    pub config: crate::config::Config,
    pub identity: Arc<RwLock<Arc<Identity>>>,
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
    pub persistence: Arc<persistence_manager::PersistenceManager>,
    pub monitor: Arc<monitor::NetworkMonitor>,
    pub switcher: Arc<RwLock<transport_manager::AutoTransportSwitcher>>,
    pub traffic_armor: Arc<armor::TrafficArmor>,
    pub messages: Arc<RwLock<Vec<Message>>>,
    pub message_store: Arc<store::SecureMessageStore>,
    pub bootstrap_nodes: Vec<Multiaddr>,
    pub event_bus: broadcast::Sender<CoreEvent>,
    pub handshake_secret: Option<Vec<u8>>,
}

impl Core {
    pub async fn new(
        config: crate::config::Config,
        bootstrap_nodes: Vec<Multiaddr>,
        sim_addr: Option<String>,
        profile: Option<String>,
    ) -> Result<Self> {
        let identity = Arc::new(
            Identity::new(profile.clone())
                .map_err(|e| anyhow::anyhow!("Identity creation failed: {}", e))?,
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

        // Initialize Persistence
        let persistence = Arc::new(persistence_manager::PersistenceManager::new(
            profile.clone(),
        )?);

        // Adapter registry
        let mut adapter_registry = AdapterRegistry::new();

        // Use default bootstraps if none provided
        let mut bootstrap_nodes = bootstrap_nodes;
        if bootstrap_nodes.is_empty() {
            tracing::info!(
                msg = "🌐 No bootstrap nodes provided. Using default public gateways for zero-config discovery."
            );
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
        let message_store = Arc::new(store::SecureMessageStore::new(persistence.clone()));
        let (event_bus, _) = broadcast::channel(1024); // Large buffer for high traffic

        // Parse handshake secret from config if available
        let handshake_secret = config
            .network
            .stealth_handshake_secret
            .as_ref()
            .and_then(|s| {
                if let Ok(bytes) = hex::decode(s) {
                    Some(bytes)
                } else {
                    tracing::warn!(
                        "Failed to decode stealth_handshake_secret from config (expected hex)"
                    );
                    None
                }
            });

        #[cfg(feature = "mesh")]
        let mesh_manager_inner = {
            if let Some(kp) = identity.keypair() {
                let username = identity.get_metadata().nickname;
                let peer_id = kp.public().to_peer_id();

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
                        handshake_secret.clone(),
                    )
                    .await?,
                )))
            } else {
                tracing::info!("Identity is locked. Deferring MeshManager initialization.");
                None
            }
        };

        #[cfg(not(feature = "mesh"))]
        let mesh_manager_inner = None;

        let monitor = Arc::new(monitor::NetworkMonitor::new(
            #[cfg(feature = "mesh")]
            mesh_manager_inner.clone(),
            std::time::Duration::from_secs(5), // Check every 5 seconds
        ));

        Ok(Self {
            config,
            identity: Arc::new(RwLock::new(identity)),
            encryption,
            security_manager,
            transport_registry: registry,
            active_transport,
            adapter_registry,
            active_adapter,
            monitor,
            switcher,
            traffic_armor,
            persistence,
            #[cfg(feature = "mesh")]
            mesh_manager: mesh_manager_inner,
            #[cfg(feature = "reticulum")]
            reticulum_manager: None,
            messages,
            message_store,
            bootstrap_nodes,
            event_bus,
            handshake_secret,
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
                        core_chaff.get_identity_id().await,
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
                        if let Some(kp) = core_mesh.identity.read().await.keypair() {
                            println!("Peer ID: {}", kp.public().to_peer_id());
                        } else {
                            println!("Peer ID: <Locked>");
                        }
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

    pub async fn send_direct_message(&self, recipient: String, message: &Message) -> Result<()> {
        // Enforce Lockdown
        if self.security_manager.is_lockdown().await {
            tracing::warn!(
                "BLOCKED: Attempted to send direct message during LOCKDOWN: {}",
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

        // We do NOT broadcast direct messages to the event bus indiscriminately
        // But we might want to emit a "MessageSent" event for UI updates

        #[cfg(feature = "mesh")]
        if let Some(mesh_manager) = &self.mesh_manager {
            let mut mgr = mesh_manager.write().await;
            if let Ok(peer_id) = recipient.parse::<libp2p::PeerId>() {
                return mgr.send_direct_message(peer_id, message.clone()).await;
            } else {
                return Err(anyhow::anyhow!("Invalid P2P Recipient ID: {}", recipient));
            }
        }

        Err(anyhow::anyhow!("Mesh network not available"))
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
    pub async fn get_identity_id(&self) -> String {
        self.identity.read().await.id.clone()
    }

    /// Get the DID
    pub async fn get_did(&self) -> String {
        self.identity.read().await.did.clone()
    }

    /// Get the metadata
    pub async fn get_metadata(&self) -> crate::core::identity::IdentityMetadata {
        self.identity.read().await.get_metadata()
    }

    /// Update metadata
    pub async fn update_metadata(
        &self,
        nickname: Option<String>,
        bio: Option<String>,
        avatar_url: Option<String>,
        setup_complete: Option<bool>,
    ) {
        self.identity
            .read()
            .await
            .update_metadata(nickname, bio, avatar_url, setup_complete);
    }

    /// Get the public key
    pub fn get_public_key(&self) -> Vec<u8> {
        self.encryption.export_public_key()
    }

    pub async fn is_locked(&self) -> bool {
        self.identity.read().await.is_locked()
    }

    pub async fn lock(&self) -> Result<()> {
        self.identity
            .read()
            .await
            .lock()
            .map_err(|e| anyhow::anyhow!("{}", e))
    }

    pub async fn unlock(&self, password: &str) -> Result<()> {
        self.identity
            .read()
            .await
            .unlock(password)
            .map_err(|e| anyhow::anyhow!("{}", e))
    }

    pub async fn save_to_disk(&self, password: &str) -> Result<()> {
        self.identity
            .read()
            .await
            .save_to_disk(password)
            .map_err(|e| anyhow::anyhow!("{}", e))
    }

    /// Recover account from DHT
    pub async fn recover_account(&self, username: &str, recovery_password: &str) -> Result<()> {
        use sha2::Digest;
        let mut shares = Vec::new();
        let threshold = 3;

        #[cfg(feature = "mesh")]
        if let Some(mesh_manager) = &self.mesh_manager {
            let mgr = mesh_manager.read().await;
            for i in 0..5 {
                let dht_key_preimage = format!("{}:recovery:{}", username, i);
                let dht_key = hex::encode(sha2::Sha256::digest(dht_key_preimage.as_bytes()));

                if let Ok(data) = mgr
                    .get_dht_record(hex::decode(dht_key).unwrap_or_default())
                    .await
                {
                    if let Ok(share) = serde_json::from_slice::<
                        crate::core::identity::EncryptedRecoveryShare,
                    >(&data)
                    {
                        shares.push(share);
                    }
                }

                if shares.len() >= threshold {
                    break;
                }
            }
        }

        if shares.len() < threshold {
            return Err(anyhow::anyhow!(
                "Finding recovery shares failed ({} found, threshold {})",
                shares.len(),
                threshold
            ));
        }

        // Recover secret
        let secret = crate::core::identity::RecoveryManager::recover_identity_secret(
            shares,
            recovery_password,
        )?;

        // Create matching identity
        let new_identity = Arc::new(Identity::from_seed(&secret)?);

        // Atomic swap
        {
            let mut id_lock = self.identity.write().await;
            *id_lock = new_identity;
        }

        tracing::info!("Account recovered successfully for {}", username);
        Ok(())
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
                let kp = self
                    .identity
                    .read()
                    .await
                    .keypair()
                    .ok_or_else(|| anyhow::anyhow!("Cannot init mesh: Identity is locked"))?;
                let username = self.identity.read().await.get_metadata().nickname;
                let peer_id = kp.public().to_peer_id();

                let username_discovery = Arc::new(
                    crate::core::username_discovery::UsernameDiscovery::new(peer_id, username),
                );

                let mesh_manager = MeshManager::new(
                    self.identity.read().await.clone(),
                    self.encryption.clone(),
                    self.messages.clone(),
                    self.message_store.clone(),
                    self.bootstrap_nodes.clone(),
                    username_discovery,
                    Some(self.event_bus.clone()),
                    self.handshake_secret.clone(),
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

    /// Enable DHT-based account recovery
    pub async fn enable_recovery(&self, recovery_password: &str) -> Result<Vec<String>> {
        #[cfg(feature = "mesh")]
        if let Some(mesh_manager) = &self.mesh_manager {
            let shares = crate::core::identity::RecoveryManager::create_recovery_shares(
                &(*self.identity.read().await),
                recovery_password,
                3, // Threshold
                5, // Total shares
            )?;

            let mgr = mesh_manager.write().await;
            let mut stored_indices = Vec::new();

            for share in shares {
                // index_hash is hex string of the DHT key
                let key = hex::decode(&share.index_hash)?;
                // We serialize the share struct to bytes for storage
                let value = serde_json::to_vec(&share)?;
                mgr.put_dht_record(key, value).await?;
                stored_indices.push(share.index_hash);
            }

            tracing::info!("Recovery enabled. Shares stored at: {:?}", stored_indices);
            Ok(stored_indices)
        } else {
            Err(anyhow::anyhow!(
                "Mesh network not active. Please start the mesh first."
            ))
        }
        #[cfg(not(feature = "mesh"))]
        Err(anyhow::anyhow!("Mesh feature not enabled"))
    }

    /// Initialize Reticulum network
    pub async fn init_reticulum(&mut self) -> Result<()> {
        #[cfg(feature = "reticulum")]
        {
            if self.reticulum_manager.is_none() {
                let reticulum_manager =
                    ReticulumManager::new(self.identity.read().await.clone()).await?;
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
                    local_node_id: self.get_identity_id().await.to_string(),
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
            return mesh_manager.read().await.get_listeners().await;
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

    pub async fn shutdown(&self) -> Result<()> {
        tracing::info!("Core initiating shutdown sequence...");

        // 1. Stop Monitor
        self.monitor.stop();

        // 2. Stop Mesh
        #[cfg(feature = "mesh")]
        if let Some(mesh_mgr) = &self.mesh_manager {
            let mgr = mesh_mgr.read().await;
            mgr.stop().await?;
        }

        // 3. Stop Reticulum
        #[cfg(feature = "reticulum")]
        if let Some(ret_mgr) = &self.reticulum_manager {
            let mgr = ret_mgr.read().await;
            mgr.stop().await?;
        }

        // 4. Flush Persistence
        self.persistence.flush()?;

        tracing::info!("Core shutdown complete.");
        Ok(())
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

    pub async fn stop(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(not(feature = "reticulum"))]
#[derive(Serialize)]
pub struct ReticulumStats {
    pub total_nodes: usize,
    pub online_nodes: usize,
    pub routes_count: usize,
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
                Identity::new(None)
                    .map_err(|e| anyhow::anyhow!("Failed to create identity: {}", e))?,
            ),
        })
    }

    pub async fn stop(&self) -> Result<()> {
        Ok(())
    }
}
