use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    #[serde(with = "uuid::serde::compact")]
    pub id: Uuid,
    pub sender: String,
    pub recipient: String,
    pub content: String,
    pub timestamp: u64,
    pub encrypted: bool,
} 