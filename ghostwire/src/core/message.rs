use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: Uuid,
    pub timestamp: u64,
    pub payload: Vec<u8>, // Encrypted
    pub signature: Vec<u8>,
    // Optionally: ephemeral sender ID, TTL, etc.
} 