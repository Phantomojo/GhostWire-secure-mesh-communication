use crate::core::crypto_machine::SealedPacket;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

/// A request to a mailbox provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MailboxRequest {
    /// Deposit a message for a peer
    Deposit {
        recipient_id: String,
        packet: SealedPacket,
    },
    /// Fetch messages for ourselves
    Fetch { recipient_id: String },
}

/// A response from a mailbox provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MailboxResponse {
    /// Operation succeeded
    Ok,
    /// Messages retrieved from mailbox
    Messages(Vec<SealedPacket>),
    /// Operation failed
    Error(String),
}

/// Manages local storage for the mailbox swarm
pub struct MailboxManager {
    /// recipient_id -> Vec<(timestamp, packet)>
    storage: Arc<DashMap<String, Vec<(SystemTime, SealedPacket)>>>,
    max_messages_per_peer: usize,
    ttl: Duration,
}

impl MailboxManager {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(DashMap::new()),
            max_messages_per_peer: 100,
            ttl: Duration::from_secs(60 * 60 * 24 * 7), // 7 days
        }
    }

    /// Deposit a message into the mailbox
    pub fn deposit(&self, recipient_id: String, packet: SealedPacket) -> Result<(), String> {
        let mut messages = self.storage.entry(recipient_id).or_insert_with(Vec::new);

        // Prune expired messages
        self.prune_expired(&mut messages);

        if messages.len() >= self.max_messages_per_peer {
            return Err("Mailbox full for this peer".into());
        }

        messages.push((SystemTime::now(), packet));
        Ok(())
    }

    /// Fetch and remove messages from the mailbox
    pub fn fetch(&self, recipient_id: &str) -> Vec<SealedPacket> {
        if let Some((_, mut messages)) = self.storage.remove(recipient_id) {
            self.prune_expired(&mut messages);
            messages.into_iter().map(|(_, p)| p).collect()
        } else {
            Vec::new()
        }
    }

    fn prune_expired(&self, messages: &mut Vec<(SystemTime, SealedPacket)>) {
        let now = SystemTime::now();
        let ttl = self.ttl;
        messages.retain(|(ts, _)| {
            if let Ok(elapsed) = now.duration_since(*ts) {
                elapsed < ttl
            } else {
                false
            }
        });
    }

    /// Periodic cleanup of all storage
    pub fn cleanup(&self) {
        self.storage.retain(|_, messages| {
            self.prune_expired(messages);
            !messages.is_empty()
        });
    }
}
