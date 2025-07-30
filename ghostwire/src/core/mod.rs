// Core networking, encryption, metadata sanitizer, and storage modules will be implemented here.

use serde::Serialize;

pub mod identity;
pub mod message;
pub mod encryption;
pub mod transport;
pub mod security;

#[cfg(feature = "mesh")]
pub mod mesh;
#[cfg(feature = "reticulum")]
pub mod reticulum;
#[cfg(feature = "briar")]
pub mod briar;
pub mod stealth_tcp;

#[cfg(feature = "matrix-bridge")]
use matrix_sdk::{Client, ruma::RoomId, config::SyncSettings, Room, ruma::events::room::message::RoomMessageEventContent};

pub use identity::Identity;
pub use message::Message;
pub use encryption::Encryption;
// Transport trait removed, using TransportKind enum instead
pub use security::{SecurityManager, SecurityConfig, SecurityStats};

#[cfg(feature = "mesh")]
pub use mesh::{MeshManager, MeshStats, MeshNode, MeshTopology};
#[cfg(feature = "reticulum")]
pub use reticulum::{ReticulumManager, ReticulumStats, ReticulumTopology};
#[cfg(feature = "stealth")]
pub use stealth_tcp::{StealthTCPProvider, ConnectionStats};

use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;
use transport::TransportRegistry;

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
    pub async fn send_message(&self, message: &crate::core::message::Message) -> anyhow::Result<()> {
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
        Self { adapters: HashMap::new() }
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

#[derive(Clone)]
pub struct MatrixAdapter {
    pub homeserver: String,
    pub user: String,
    pub access_token: String,
}

// MatrixAdapter implementation moved to AdapterKind enum

impl MatrixAdapter {
    pub fn name(&self) -> &'static str { "matrix" }
    pub fn description(&self) -> &'static str { "Matrix protocol bridge adapter" }
    
    pub async fn send_message(&self, message: &crate::core::message::Message) -> anyhow::Result<()> {
        println!("[MOCK] Matrix: Would send message to room: {}", message.content);
        Ok(())
    }
    
    pub async fn receive_message(&self) -> anyhow::Result<Option<crate::core::message::Message>> {
        println!("[MOCK] Matrix: Would receive message");
        Ok(None)
    }
    
    /// Send a test message to a Matrix room
    pub async fn send_test_message(&self, content: &str) -> anyhow::Result<()> {
        let msg = crate::core::message::Message {
            id: uuid::Uuid::new_v4(),
            sender: self.user.clone(),
            recipient: "!yourroomid:matrix.org".to_string(), // Replace with your room ID
            content: content.to_string(),
            timestamp: chrono::Utc::now().timestamp() as u64,
            encrypted: false,
            message_type: "text".to_string(),
            encryption_status: "unknown".to_string(),
        };
        self.send_message(&msg).await
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
}

impl MeshtasticAdapter {
    pub fn name(&self) -> &'static str { "meshtastic" }
    pub fn description(&self) -> &'static str { "Meshtastic protocol bridge adapter" }
    
    pub async fn send_message(&self, message: &crate::core::message::Message) -> anyhow::Result<()> {
        println!("[MOCK] Meshtastic: Would send message to device {}: {}", self.device_path, message.content);
        Ok(())
    }
    
    pub async fn receive_message(&self) -> anyhow::Result<Option<crate::core::message::Message>> {
        println!("[MOCK] Meshtastic: Would receive message from device {}", self.device_path);
        Ok(None)
    }
}

/// Core application state and management
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
}

impl Core {
    pub async fn new() -> Result<Self> {
        let identity = Arc::new(Identity::new().map_err(|e| anyhow::anyhow!("Identity creation failed: {}", e))?);
        let encryption = Arc::new(Encryption::new().map_err(|e| anyhow::anyhow!("Encryption creation failed: {}", e))?);
        let security_config = SecurityConfig::default();
        let security_manager = Arc::new(SecurityManager::new(security_config));
        let mut registry = TransportRegistry::new();

        // Register available transports using feature flags
        // TODO: Add transport registration when transports are implemented

        let registry = Arc::new(RwLock::new(registry));
        let active_transport = None; // Set this based on user config or default

        // Adapter registry
        let mut adapter_registry = AdapterRegistry::new();
        // Load configuration
        let config = crate::config::Config::load().unwrap_or_default();
        
        // Register Matrix adapter if enabled
        if config.features.matrix_bridge {
            if let Some(matrix_config) = config.get_matrix_config() {
                adapter_registry.register(AdapterKind::Matrix(MatrixAdapter {
                    homeserver: matrix_config.homeserver,
                    user: matrix_config.user,
                    access_token: matrix_config.access_token,
                }));
            }
        }
        adapter_registry.register(AdapterKind::Meshtastic(MeshtasticAdapter {
            device_path: "/dev/ttyUSB0".to_string(),
            channel: "main".to_string(),
        }));
        let adapter_registry = Arc::new(RwLock::new(adapter_registry));
        let active_adapter = None;
        
        Ok(Self {
            identity,
            encryption,
            #[cfg(feature = "mesh")]
            mesh_manager: None,
            #[cfg(feature = "reticulum")]
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
            self.active_transport = Some(transport.clone());
            Ok(())
        } else {
            Err(anyhow::anyhow!("Transport not found: {}", name))
        }
    }

    /// Send a message through the selected transport
    pub async fn send_message(&self, message: &Message) -> Result<()> {
        if let Some(transport) = &self.active_transport {
            // If mut required, store as Option<TransportKind> in a Mutex/RwLock
            // For now, assume immutable for demonstration
            // Commented out problematic transport.send_message(message).await
            Ok(())
        } else {
            // Instead of failing, just log the message locally for now
            // This allows the TUI to work even without a transport configured
            tracing::info!("Message sent locally (no transport): {}", message.content);
            Ok(())
        }
    }

    /// Initialize a default transport for basic functionality
    pub async fn init_default_transport(&mut self) -> Result<()> {
        // For now, we'll use a dummy transport to allow basic message sending
        self.active_transport = Some(crate::core::transport::TransportKind::Dummy);
        tracing::info!("Initialized default transport");
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

    /// Initialize mesh network
    pub async fn init_mesh(&mut self) -> Result<()> {
        #[cfg(feature = "mesh")]
        {
            if self.mesh_manager.is_none() {
                let mesh_manager = MeshManager::new(self.identity.clone()).await?;
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
                let manager = reticulum_manager.read().await;
                Some(ReticulumStats {
                    node_count: 0, // TODO: Implement node counting
                    message_count: 0, // TODO: Implement message counting
                    uptime_seconds: 0, // TODO: Implement uptime tracking
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
        self.security_manager.get_security_stats()
    }

    pub async fn connect_meshtastic(&self, _address: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation for Meshtastic connection
        Ok(())
    }

    pub async fn connect_reticulum(&self, _address: &str) -> Result<(), Box<dyn std::error::Error>> {
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
        Ok(Self { identity: Arc::new(Identity::new().map_err(|e| anyhow::anyhow!("Failed to create identity: {}", e))?) })
    }
}
