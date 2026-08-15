//! postbox.rs — GhostPostbox: Anonymous store-and-forward for asynchronous mesh delivery
//!
//! GhostPostbox extends the DTN (Delay-Tolerant Networking) layer by providing
//! high-uptime "anchor" nodes that store encrypted Sphinx packets for offline
//! peers. This bridges the gap when two peers are never online at the same time.
//!
//! Features:
//!   1. **Anonymity**: Postboxes store packets by `MailboxID` (a hash), not PeerID.
//!   2. **Persistence**: Backed by sled for reliable storage across restarts.
//!   3. **Quota-limited**: Per-mailbox storage limits to prevent DoS.
//!   4. **Time-limited**: Automatic expiry of old packets.

use crate::core::routing::sphinx::SphinxPacket;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sled::{Db, Tree};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// Default packet expiry: 7 days
pub const POSTBOX_EXPIRY_SECS: u64 = 7 * 24 * 3600;
/// Default quota: 50 packets per mailbox
pub const POSTBOX_QUOTA: usize = 50;

/// Unique identifier for a mailbox, derived from the recipient's identity
/// but hashed to prevent the Postbox from knowing the recipient's PeerID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct MailboxId(pub [u8; 32]);

/// A stored entry in the GhostPostbox
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostboxEntry {
    pub packet: SphinxPacket,
    pub received_at: u64,
    pub expires_at: u64,
}

pub struct GhostPostbox {
    db: Arc<Db>,
    store: Tree,
}

impl GhostPostbox {
    pub fn open(db: Arc<Db>) -> Result<Self> {
        let store = db.open_tree("ghostpostbox")?;
        Ok(Self { db, store })
    }

    /// Deposit a Sphinx packet into a specific mailbox.
    pub fn deposit(&self, mailbox_id: MailboxId, packet: SphinxPacket) -> Result<()> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
        let now_secs = now.as_secs();
        let now_nanos = now.subsec_nanos();

        let entry = PostboxEntry {
            packet,
            received_at: now_secs,
            expires_at: now_secs + POSTBOX_EXPIRY_SECS,
        };

        let key = self.make_key(mailbox_id, now_secs, now_nanos);
        let value = bincode::serialize(&entry)?;

        // Check quota before inserting
        let existing_count = self.count_packets(mailbox_id)?;
        if existing_count >= POSTBOX_QUOTA {
            // Evict oldest if quota reached
            self.evict_oldest(mailbox_id)?;
        }

        self.store.insert(key, value)?;
        Ok(())
    }

    /// Retrieve all packets for a mailbox since a specific timestamp.
    pub fn retrieve(&self, mailbox_id: MailboxId, since: u64) -> Result<Vec<SphinxPacket>> {
        let prefix = self.make_prefix(mailbox_id);
        let mut packets = Vec::new();

        for item in self.store.scan_prefix(prefix) {
            let (_, value) = item?;
            let entry: PostboxEntry = bincode::deserialize(&value)?;

            if entry.received_at >= since {
                packets.push(entry.packet);
            }
        }

        Ok(packets)
    }

    /// Delete all packets for a mailbox (after retrieval).
    pub fn drain(&self, mailbox_id: MailboxId) -> Result<()> {
        let prefix = self.make_prefix(mailbox_id);
        let mut keys_to_delete = Vec::new();

        for item in self.store.scan_prefix(prefix) {
            let (key, _) = item?;
            keys_to_delete.push(key);
        }

        for key in keys_to_delete {
            self.store.remove(key)?;
        }

        Ok(())
    }

    /// Periodic cleanup of expired packets across all mailboxes.
    pub fn cleanup(&self) -> Result<usize> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        let mut count = 0;
        let mut keys_to_delete = Vec::new();

        for item in self.store.iter() {
            let (key, value) = item?;
            let entry: PostboxEntry = bincode::deserialize(&value)?;
            if entry.expires_at < now {
                keys_to_delete.push(key);
                count += 1;
            }
        }

        for key in keys_to_delete {
            self.store.remove(key)?;
        }

        Ok(count)
    }

    fn count_packets(&self, mailbox_id: MailboxId) -> Result<usize> {
        let prefix = self.make_prefix(mailbox_id);
        Ok(self.store.scan_prefix(prefix).count())
    }

    fn evict_oldest(&self, mailbox_id: MailboxId) -> Result<()> {
        let prefix = self.make_prefix(mailbox_id);
        if let Some(item) = self.store.scan_prefix(prefix).next() {
            let (key, _) = item?;
            self.store.remove(key)?;
        }
        Ok(())
    }

    fn make_prefix(&self, mailbox_id: MailboxId) -> Vec<u8> {
        let mut prefix = Vec::with_capacity(32);
        prefix.extend_from_slice(&mailbox_id.0);
        prefix
    }

    fn make_key(&self, mailbox_id: MailboxId, timestamp: u64, nanos: u32) -> Vec<u8> {
        let mut key = Vec::with_capacity(44);
        key.extend_from_slice(&mailbox_id.0);
        key.extend_from_slice(&timestamp.to_be_bytes());
        key.extend_from_slice(&nanos.to_be_bytes());
        key
    }
}
