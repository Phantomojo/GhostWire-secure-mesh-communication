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

pub use encryption::EncryptionManager;
pub use message::Message;
pub use store::MessageCache;
pub use identity::EphemeralIdentity;
pub use transport::{Transport, LocalTransport};

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Core {
    pub encryption: Arc<EncryptionManager>,
    pub identity: Arc<RwLock<EphemeralIdentity>>,
    pub store: Arc<MessageCache>,
    pub transport: Arc<dyn Transport>,
}

impl Core {
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
    
    pub fn add_peer_key(&self, peer_id: String, public_key: Vec<u8>) {
        self.encryption.add_peer_key(peer_id, public_key);
    }
    
    pub fn get_peer_count(&self) -> usize {
        self.encryption.get_peer_count()
    }
}
