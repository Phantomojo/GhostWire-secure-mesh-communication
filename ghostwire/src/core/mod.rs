// Core networking, encryption, metadata sanitizer, and storage modules will be implemented here.

pub mod encryption;
/*
pub mod metadata_sanitizer;
pub mod storage;
*/
pub mod message;
pub mod identity;
pub mod store;
pub mod transport;
pub mod mesh;

pub use encryption::EncryptionManager;
pub use message::Message;
pub use store::MessageCache;
pub use identity::EphemeralIdentity;
pub use transport::{Transport, LocalTransport};
pub use mesh::{MeshManager, MeshTransport, MeshStats, MeshNode, MeshTopology};

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
#[allow(dead_code)]
pub struct Core {
    pub encryption: Arc<EncryptionManager>,
    pub identity: Arc<RwLock<EphemeralIdentity>>,
    pub store: Arc<MessageCache>,
    pub transport: Arc<dyn Transport>,
    pub mesh_manager: Option<Arc<RwLock<MeshManager>>>,
}

impl Core {
    #[allow(dead_code)]
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let encryption = Arc::new(EncryptionManager::new()?);
        let identity = Arc::new(RwLock::new(EphemeralIdentity::new()?));
        let store = Arc::new(MessageCache::new());
        let transport = Arc::new(LocalTransport::new());
        
        Ok(Core {
            encryption,
            identity,
            store,
            transport,
            mesh_manager: None,
        })
    }
    
    pub async fn send_message(&self, recipient: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Encrypt the message
        let encrypted = self.encryption.encrypt_message(content, recipient)?;
        let sender = self.identity.read().await.identity_id.clone();
        let message = Message {
            id: uuid::Uuid::new_v4(),
            sender,
            recipient: recipient.to_string(),
            content: serde_json::to_string(&encrypted)?,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            encrypted: true,
        };
        // self.store.store_message(&message).await?;
        self.transport.send_message(&message).await?;
        Ok(())
    }
    
    #[allow(dead_code)]
    pub async fn receive_message(&self, message: &Message) -> Result<String, Box<dyn std::error::Error>> {
        if message.encrypted {
            let encrypted: encryption::EncryptedMessage = serde_json::from_str(&message.content)?;
            self.encryption.decrypt_message(&encrypted)
        } else {
            Ok(message.content.clone())
        }
    }
    
    pub fn get_public_key(&self) -> Vec<u8> {
        self.encryption.export_public_key()
    }
    
    pub fn get_key_id(&self) -> String {
        self.encryption.get_key_id().to_string()
    }
    
    #[allow(dead_code)]
    pub fn add_peer_key(&self, peer_id: String, public_key: Vec<u8>) {
        self.encryption.add_peer_key(peer_id, public_key);
    }
    
    pub fn get_peer_count(&self) -> usize {
        self.encryption.get_peer_count()
    }

    /// Initialize mesh networking
    pub async fn init_mesh(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let identity = self.identity.read().await.clone();
        let mesh_manager = MeshManager::new(Arc::new(identity)).await?;
        self.mesh_manager = Some(Arc::new(RwLock::new(mesh_manager)));
        Ok(())
    }

    /// Start mesh network on specified address
    pub async fn start_mesh(&self, listen_addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(mesh_manager) = &self.mesh_manager {
            let mut manager = mesh_manager.write().await;
            let addr: libp2p::Multiaddr = listen_addr.parse()?;
            manager.start(addr).await?;
        }
        Ok(())
    }

    /// Connect to Meshtastic device
    pub async fn connect_meshtastic(&self, address: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(mesh_manager) = &self.mesh_manager {
            let mut manager = mesh_manager.write().await;
            manager.connect_meshtastic(address).await?;
        }
        Ok(())
    }

    /// Send message through mesh network
    pub async fn send_mesh_message(&self, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(mesh_manager) = &self.mesh_manager {
            let mut manager = mesh_manager.write().await;
            let sender = self.identity.read().await.identity_id.clone();
            let message = Message {
                id: uuid::Uuid::new_v4(),
                sender,
                recipient: "mesh".to_string(),
                content: content.to_string(),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                encrypted: false, // Will be encrypted by mesh layer
            };
            manager.send_message(&message).await?;
        }
        Ok(())
    }

    /// Get mesh network statistics
    pub async fn get_mesh_stats(&self) -> Option<MeshStats> {
        if let Some(mesh_manager) = &self.mesh_manager {
            let manager = mesh_manager.read().await;
            Some(manager.get_stats().await)
        } else {
            None
        }
    }

    /// Get mesh topology
    pub async fn get_mesh_topology(&self) -> Option<MeshTopology> {
        if let Some(mesh_manager) = &self.mesh_manager {
            let manager = mesh_manager.read().await;
            Some(manager.transport.get_topology().await)
        } else {
            None
        }
    }
}
