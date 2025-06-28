use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc;
use uuid::Uuid;

#[async_trait]
pub trait Transport {
    async fn send(&self, peer_id: &str, data: &[u8]) -> Result<()>;
    async fn receive(&self) -> Option<(String, Vec<u8>)>;
    fn name(&self) -> &'static str;
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
}

#[async_trait]
impl Transport for MockTransport {
    async fn send(&self, peer_id: &str, data: &[u8]) -> Result<()> {
        println!("MockTransport: Sending {} bytes to peer {}", data.len(), peer_id);
        Ok(())
    }

    async fn receive(&self) -> Option<(String, Vec<u8>)> {
        // In a real implementation, this would receive from the queue
        None
    }

    fn name(&self) -> &'static str {
        "mock"
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