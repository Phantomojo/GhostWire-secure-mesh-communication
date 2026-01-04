use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: Uuid,
    pub sender: String,
    pub recipient: String,
    pub content: String,
    pub timestamp: u64,
    pub encrypted: bool,
    pub message_type: String,
    pub encryption_status: String,
}

impl Message {
    pub fn new(sender: String, recipient: String, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            sender,
            recipient,
            content,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            encrypted: false,
            message_type: "text".to_string(),
            encryption_status: "none".to_string(),
        }
    }
}
