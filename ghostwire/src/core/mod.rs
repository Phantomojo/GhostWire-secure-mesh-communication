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

pub use identity::Identity;
pub use message::Message;
pub use encryption::Encryption;
pub use transport::Transport;
pub use mesh::{MeshManager, MeshStats, MeshNode, MeshTopology};
pub use reticulum::{ReticulumManager, ReticulumStats, ReticulumTopology};
pub use stealth_tcp::{StealthTCPProvider, ConnectionStats};
pub use security::{SecurityManager, SecurityConfig, SecurityStats};

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// Core application state and management
pub struct Core {
    pub identity: Arc<Identity>,
    pub encryption: Arc<Encryption>,
    pub mesh_manager: Option<Arc<RwLock<MeshManager>>>,
    pub reticulum_manager: Option<Arc<RwLock<ReticulumManager>>>,
    pub security_manager: Arc<SecurityManager>,
}

impl Core {
    pub async fn new() -> Result<Self> {
        let identity = Arc::new(Identity::new().map_err(|e| anyhow::anyhow!("Identity creation failed: {}", e))?);
        let encryption = Arc::new(Encryption::new().map_err(|e| anyhow::anyhow!("Encryption creation failed: {}", e))?);
        
        // Initialize security manager with default configuration
        let security_config = SecurityConfig::default();
        let security_manager = Arc::new(SecurityManager::new(security_config));
        
        Ok(Self {
            identity,
            encryption,
            mesh_manager: None,
            reticulum_manager: None,
            security_manager,
        })
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

    /// Send a message through the network
    pub async fn send_message(&self, message: &Message) -> Result<()> {
        // For now, just log the message
        // In a real implementation, this would route through the appropriate network
        info!("Sending message: {} -> {}: {}", message.sender, message.recipient, message.content);
        
        // If mesh is initialized, send through mesh
        if let Some(mesh_manager) = &self.mesh_manager {
            let manager = mesh_manager.read().await;
            // TODO: Implement actual message sending through mesh
        }
        
        // If reticulum is initialized, send through reticulum
        if let Some(reticulum_manager) = &self.reticulum_manager {
            let manager = reticulum_manager.read().await;
            // TODO: Implement actual message sending through reticulum
        }
        
        Ok(())
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
