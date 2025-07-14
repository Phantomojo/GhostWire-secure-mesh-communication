// Core networking, encryption, metadata sanitizer, and storage modules will be implemented here.

pub mod identity;
pub mod message;
pub mod encryption;
pub mod transport;
pub mod mesh;
pub mod reticulum;
pub mod briar;
pub mod stealth_tcp;
pub mod security;

#[cfg(feature = "matrix-bridge")]
use matrix_sdk::{Client, ruma::RoomId, config::SyncSettings, Room, ruma::events::room::message::RoomMessageEventContent};

pub use identity::Identity;
pub use message::Message;
pub use encryption::Encryption;
pub use transport::Transport;
pub use mesh::{MeshManager, MeshStats, MeshNode, MeshTopology};
pub use reticulum::{ReticulumManager, ReticulumStats, ReticulumTopology};
pub use stealth_tcp::{StealthTCPProvider, ConnectionStats};
pub use security::{SecurityManager, SecurityConfig, SecurityStats};

use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;
use transport::TransportRegistry;

// Protocol Adapter Trait
#[async_trait::async_trait]
pub trait ProtocolAdapter: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str { "" }
    async fn send_message(&self, message: &crate::core::message::Message) -> anyhow::Result<()>;
    async fn receive_message(&self) -> anyhow::Result<Option<crate::core::message::Message>>;
}

// Adapter Registry
pub struct AdapterRegistry {
    adapters: HashMap<String, Arc<dyn ProtocolAdapter>>,
}

impl AdapterRegistry {
    pub fn new() -> Self {
        Self { adapters: HashMap::new() }
    }
    pub fn register<T: ProtocolAdapter + 'static>(&mut self, adapter: T) {
        self.adapters.insert(adapter.name().to_string(), Arc::new(adapter));
    }
    pub fn get(&self, name: &str) -> Option<Arc<dyn ProtocolAdapter>> {
        self.adapters.get(name).cloned()
    }
    pub fn list(&self) -> Vec<String> {
        self.adapters.keys().cloned().collect()
    }
}

// Matrix Adapter with config
pub struct MatrixAdapter {
    pub homeserver: String,
    pub user: String,
    pub access_token: String,
}

#[async_trait::async_trait]
impl ProtocolAdapter for MatrixAdapter {
    fn name(&self) -> &'static str { "matrix" }
    fn description(&self) -> &'static str { "Matrix protocol bridge adapter" }
    async fn send_message(&self, message: &crate::core::message::Message) -> anyhow::Result<()> {
        #[cfg(feature = "matrix-bridge")]
        {
            // Send a message to a Matrix room using matrix-sdk
            let client = Client::builder()
                .homeserver_url(self.homeserver.clone())
                .build()
                .await?;
            
            // Login with access token
            client.restore_login_with_access_token(
                self.user.clone(),
                self.access_token.clone(),
                None,
            ).await?;
            
            // Get room and send message
            let room_id = RoomId::parse("!yourroomid:matrix.org")?;
            client.room_send(
                &room_id,
                RoomMessageEventContent::text_plain(message.content.clone()),
                None,
            ).await?;
            
            Ok(())
        }
        #[cfg(not(feature = "matrix-bridge"))]
        {
            println!("[MOCK] Matrix: Would send message to room: {}", message.content);
            Ok(())
        }
    }
    async fn receive_message(&self) -> anyhow::Result<Option<crate::core::message::Message>> {
        #[cfg(feature = "matrix-bridge")]
        {
            // Receive messages from Matrix rooms
            let client = Client::builder()
                .homeserver_url(self.homeserver.clone())
                .build()
                .await?;
            
            // Login with access token
            client.restore_login_with_access_token(
                self.user.clone(),
                self.access_token.clone(),
                None,
            ).await?;
            
            // Sync and get messages
            client.sync_once(SyncSettings::default()).await?;
            
            // Check for new messages in joined rooms
            for (room_id, room) in client.rooms().joined() {
                let timeline = room.timeline().await?;
                for event in timeline.events() {
                    if let matrix_sdk::deserialized_responses::TimelineEvent::Event(ev) = event {
                        if let Some(content) = ev.event.deserialize_as::<RoomMessageEventContent>().ok() {
                            return Ok(Some(crate::core::message::Message {
                                id: ev.event.event_id().to_string(),
                                sender: ev.event.sender().to_string(),
                                recipient: room_id.to_string(),
                                content: content.body().to_string(),
                                timestamp: ev.event.origin_server_ts().as_secs(),
                                encrypted: false,
                            }));
                        }
                    }
                }
            }
            
            Ok(None)
        }
        #[cfg(not(feature = "matrix-bridge"))]
        {
            println!("[MOCK] Matrix: Would receive message");
            Ok(None)
        }
    }
}

// Example test function for MatrixAdapter
impl MatrixAdapter {
    /// Send a test message to a Matrix room
    pub async fn send_test_message(&self, content: &str) -> anyhow::Result<()> {
        let msg = crate::core::message::Message {
            id: uuid::Uuid::new_v4(),
            sender: self.user.clone(),
            recipient: "!yourroomid:matrix.org".to_string(), // Replace with your room ID
            content: content.to_string(),
            timestamp: chrono::Utc::now().timestamp() as u64,
            encrypted: false,
        };
        self.send_message(&msg).await
    }
}
// Usage:
// 1. Set your Matrix room ID in the code (replace !yourroomid:matrix.org)
// 2. Build with: cargo build --features matrix-bridge
// 3. Call MatrixAdapter::send_test_message("Hello from GhostWire!").await
// 4. Call receive_message() to print new messages

// Meshtastic Adapter with config
pub struct MeshtasticAdapter {
    pub device_path: String,
    pub channel: String,
}

#[async_trait::async_trait]
impl ProtocolAdapter for MeshtasticAdapter {
    fn name(&self) -> &'static str { "meshtastic" }
    fn description(&self) -> &'static str { "Meshtastic protocol bridge adapter" }
    async fn send_message(&self, message: &crate::core::message::Message) -> anyhow::Result<()> {
        // TODO: Integrate serialport or meshtastic-rust for real message sending
        println!("[MeshtasticAdapter] Would send message to {}: {}", self.device_path, message.content);
        Ok(())
    }
    async fn receive_message(&self) -> anyhow::Result<Option<crate::core::message::Message>> {
        // TODO: Integrate serialport or meshtastic-rust for real message receiving
        println!("[MeshtasticAdapter] Would poll for new messages on {}", self.device_path);
        Ok(None)
    }
}

/// Core application state and management
pub struct Core {
    pub identity: Arc<Identity>,
    pub encryption: Arc<Encryption>,
    pub mesh_manager: Option<Arc<RwLock<MeshManager>>>,
    pub reticulum_manager: Option<Arc<RwLock<ReticulumManager>>>,
    pub security_manager: Arc<SecurityManager>,
    pub transport_registry: Arc<RwLock<TransportRegistry>>,
    pub active_transport: Option<Arc<tokio::sync::Mutex<dyn Transport>>>,
    pub adapter_registry: Arc<RwLock<AdapterRegistry>>,
    pub active_adapter: Option<Arc<dyn ProtocolAdapter>>,
}

impl Core {
    pub async fn new() -> Result<Self> {
        let identity = Arc::new(Identity::new().map_err(|e| anyhow::anyhow!("Identity creation failed: {}", e))?);
        let encryption = Arc::new(Encryption::new().map_err(|e| anyhow::anyhow!("Encryption creation failed: {}", e))?);
        let security_config = SecurityConfig::default();
        let security_manager = Arc::new(SecurityManager::new(security_config));
        let mut registry = TransportRegistry::new();

        // Register available transports using feature flags
        #[cfg(feature = "mesh-transport")]
        {
            let mesh = MeshManager::new(identity.clone()).await?;
            registry.register(mesh.transport);
        }
        #[cfg(feature = "stealth-tcp-transport")]
        {
            let stealth = StealthTCPProvider::new(None, security_manager.clone(), None, true);
            registry.register(stealth);
        }
        #[cfg(feature = "briar-transport")]
        {
            let briar = BriarManager::new(identity.clone()).await?;
            registry.register(briar);
        }
        #[cfg(feature = "reticulum-transport")]
        {
            let reticulum = ReticulumManager::new(identity.clone()).await?;
            registry.register(reticulum);
        }

        let registry = Arc::new(RwLock::new(registry));
        let active_transport = None; // Set this based on user config or default

        // Adapter registry
        let mut adapter_registry = AdapterRegistry::new();
        // Register Matrix adapter with your credentials
        // SECURITY: Keep your access token private! If you need to change it, update here.
        adapter_registry.register(MatrixAdapter {
            homeserver: "https://matrix.org".to_string(),
            user: "@phantomojo:matrix.org".to_string(),
            access_token: "mat_Ey06RMa7JU5uBN5509fzeFWnogdOdE_bObsn3".to_string(), // <-- Update here if you change your token
        });
        adapter_registry.register(MeshtasticAdapter {
            device_path: "/dev/ttyUSB0".to_string(),
            channel: "main".to_string(),
        });
        let adapter_registry = Arc::new(RwLock::new(adapter_registry));
        let active_adapter = None;

        Ok(Self {
            identity,
            encryption,
            mesh_manager: None,
            reticulum_manager: None,
            security_manager,
            transport_registry: registry,
            active_transport,
            adapter_registry,
            active_adapter,
        })
    }

    /// Select an active transport by name
    pub async fn set_active_transport(&mut self, name: &str) -> Result<()> {
        let registry = self.transport_registry.read().await;
        if let Some(transport) = registry.get(name) {
            self.active_transport = Some(transport);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Transport not found: {}", name))
        }
    }

    /// Send a message through the selected transport
    pub async fn send_message(&self, message: &Message) -> Result<()> {
        if let Some(transport) = &self.active_transport {
            let mut guard = transport.lock().await;
            guard.send_message(message).await
        } else {
            Err(anyhow::anyhow!("No active transport selected"))
        }
    }

    /// Select an active protocol adapter by name
    pub async fn set_active_adapter(&mut self, name: &str) -> Result<()> {
        let registry = self.adapter_registry.read().await;
        if let Some(adapter) = registry.get(name) {
            self.active_adapter = Some(adapter);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Adapter not found: {}", name))
        }
    }
    /// Send a message through the selected protocol adapter
    pub async fn send_adapter_message(&self, message: &Message) -> Result<()> {
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

    /// Get the public key
    pub fn get_public_key(&self) -> Vec<u8> {
        self.encryption.export_public_key()
    }
    
    /// Get the key ID
    pub fn get_key_id(&self) -> String {
        self.encryption.get_key_id()
    }

    /// Get network topology (placeholder for now)
    pub async fn get_network_topology(&self) -> Result<String> {
        Ok("Network topology not yet implemented".to_string())
    }

    pub async fn init_mesh(&mut self) -> Result<()> {
        if self.mesh_manager.is_none() {
            let mesh_manager = MeshManager::new(self.identity.clone()).await?;
            self.mesh_manager = Some(Arc::new(RwLock::new(mesh_manager)));
        }
        Ok(())
    }

    pub async fn init_reticulum(&mut self) -> Result<()> {
        if self.reticulum_manager.is_none() {
            let reticulum_manager = ReticulumManager::new(self.identity.clone()).await?;
            self.reticulum_manager = Some(Arc::new(RwLock::new(reticulum_manager)));
        }
        Ok(())
    }

    pub async fn get_mesh_stats(&self) -> Option<MeshStats> {
        if let Some(mesh_manager) = &self.mesh_manager {
            let manager = mesh_manager.read().await;
            Some(manager.get_stats().await)
        } else {
            None
        }
    }

    pub async fn get_reticulum_stats(&self) -> Option<ReticulumStats> {
        if let Some(reticulum_manager) = &self.reticulum_manager {
            let manager = reticulum_manager.read().await;
            Some(manager.get_stats().await)
        } else {
            None
        }
    }

    pub async fn get_security_stats(&self) -> SecurityStats {
        self.security_manager.get_security_stats()
    }

    pub async fn connect_meshtastic(&self, _address: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation for Meshtastic connection
        Ok(())
    }

    pub async fn connect_reticulum(&self, _address: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(_reticulum_manager) = &self.reticulum_manager {
            // Note: This method doesn't exist yet, so we'll skip it for now
            // let manager = reticulum_manager.write().await;
            // manager.connect(address).await?;
        }
        Ok(())
    }

    pub async fn get_reticulum_topology(&self) -> Result<Option<ReticulumTopology>> {
        if let Some(_reticulum_manager) = &self.reticulum_manager {
            // Note: This method doesn't exist yet, so we'll return None for now
            Ok(None)
        } else {
            Ok(None)
        }
    }

    pub async fn start_mesh(&self, _address: &str) -> anyhow::Result<()> {
        // Stub implementation
        Ok(())
    }
    pub async fn get_mesh_topology(&self) -> anyhow::Result<()> {
        // Stub implementation
        Ok(())
    }

    // Add convenience methods for backward compatibility
    pub fn get_peer_count(&self) -> usize {
        self.encryption.get_peer_count()
    }
}

// Documentation: MatrixAdapter and MeshtasticAdapter now have config fields and stubbed logic. Replace stubs with real SDK/API calls for production bridging.
// Documentation: The core now supports modular, pluggable transports via TransportRegistry. Use feature flags to include/exclude transports at build time. Select active transport at runtime for message routing.
// Documentation: The core now supports modular, pluggable protocol adapters via AdapterRegistry. Register new adapters as plugins. Select active adapter at runtime for protocol bridging.
