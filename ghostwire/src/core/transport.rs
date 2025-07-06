use async_trait::async_trait;
use crate::core::message::Message;
use anyhow::Result;
use tokio::sync::mpsc;
use uuid::Uuid;

#[async_trait]
pub trait Transport: Send + Sync {
    async fn send_message(&self, message: &Message) -> Result<()>;
    async fn receive_message(&self) -> Result<Option<Message>>;
}

pub struct MockTransport {
    peer_id: String,
    message_queue: mpsc::UnboundedReceiver<(String, Vec<u8>)>,
    _sender: mpsc::UnboundedSender<(String, Vec<u8>)>,
}

impl MockTransport {
    pub async fn new() -> Result<Self> {
        let peer_id = Uuid::new_v4().to_string();
        let (sender, receiver) = mpsc::unbounded_channel();
        
        Ok(Self {
            peer_id,
            message_queue: receiver,
            _sender: sender,
        })
    }

    pub fn peer_id(&self) -> &str {
        &self.peer_id
    }
    
    // Add send method for CLI compatibility
    pub async fn send(&self, peer_id: &str, data: &[u8]) -> Result<()> {
        println!("MockTransport: Sending {} bytes to {}", data.len(), peer_id);
        Ok(())
    }
}

#[async_trait]
impl Transport for MockTransport {
    async fn send_message(&self, message: &Message) -> Result<()> {
        println!("MockTransport: Sending message with ID: {}", message.id);
        Ok(())
    }
    
    async fn receive_message(&self) -> Result<Option<Message>> {
        // In a real implementation, this would receive from the queue
        Ok(None)
    }
}

impl Clone for MockTransport {
    fn clone(&self) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        Self {
            peer_id: self.peer_id.clone(),
            message_queue: receiver,
            _sender: sender,
        }
    }
}

pub struct LocalTransport {
    // For now, this is a simple local transport
}

impl LocalTransport {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Transport for LocalTransport {
    async fn send_message(&self, message: &Message) -> Result<()> {
        // For now, just log the message
        println!("[LOCAL] Sending message with ID: {}", message.id);
        Ok(())
    }
    
    async fn receive_message(&self) -> Result<Option<Message>> {
        // For now, return None (no messages to receive)
        Ok(None)
    }
}

// --- P2P Networking Scaffold ---
#[cfg(feature = "p2p")]
pub mod p2p {
    // Placeholder for libp2p-based transport
    pub fn start_p2p_node() {
        // TODO: Implement P2P node startup using libp2p
        println!("[P2P] Starting P2P node (libp2p placeholder)");
    }
}

// Abstraction for message transport
pub trait MessageTransport {
    fn send_message(&self, recipient: &str, message: &[u8]) -> Result<(), String>;
    fn receive_message(&self) -> Option<(String, Vec<u8>)>;
} 