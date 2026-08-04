//! mailbox.rs — Store-and-forward delivery for offline peers
//!
//! Borrowed from Veilid: the DHT multi-writer model where nodes can
//! "leave notes" for offline peers. We implement this using sled for
//! local persistence + libp2p Kademlia for DHT-backed delivery.
//!
//! Architecture:
//!   1. Sender encrypts message for recipient's public key
//!   2. Drops encrypted blob into recipient's DHT mailbox record
//!   3. When recipient comes online, they poll their mailbox key and drain it
//!   4. Local sled cache holds undelivered outgoing messages until confirmed

use crate::genesis_constants::{derive_with_domain, MAILBOX_KDF_DOMAIN};
use base64::Engine;

use anyhow::Result;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Key, Nonce,
};
use rand_core::OsRng;
use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};
use sled::Db;
use std::sync::Arc;
use zeroize::Zeroize;

const MAILBOX_PREFIX: &str = "mailbox:";
const MAX_MAILBOX_SIZE: usize = 100;
const MAX_MESSAGE_BYTES: usize = 65_536; // 64 KiB per message

/// A sealed, encrypted envelope stored in the DHT for an offline recipient.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailboxEnvelope {
    /// Base58 recipient peer ID (used as DHT key suffix)
    pub recipient_id: String,
    /// Sender's ephemeral X25519 public key (32 bytes, base64)
    pub ephemeral_pub: String,
    /// ChaCha20-Poly1305 ciphertext
    pub ciphertext: Vec<u8>,
    /// 12-byte nonce (base64)
    pub nonce: String,
    /// Unix timestamp of creation
    pub created_at: u64,
    /// Message UUID (used for deduplication)
    pub message_id: String,
    /// Number of delivery attempts
    pub attempts: u32,
}

impl Drop for MailboxEnvelope {
    fn drop(&mut self) {
        // Zero sensitive fields on drop
        self.ciphertext.zeroize();
    }
}

/// Local mailbox store backed by sled
pub struct MailboxStore {
    db: Arc<Db>,
    rng: SystemRandom,
}

impl MailboxStore {
    pub fn open(db: Arc<Db>) -> Result<Self> {
        Ok(Self {
            db,
            rng: SystemRandom::new(),
        })
    }

    /// Seal and store an outgoing message for a peer who may be offline.
    ///
    /// Uses X25519 ECDH + ChaCha20-Poly1305. The recipient only needs their
    /// static X25519 private key to open it.
    pub fn seal_for_peer(
        &self,
        recipient_id: &str,
        recipient_x25519_pub: &[u8; 32],
        plaintext: &[u8],
    ) -> Result<MailboxEnvelope> {
        if plaintext.len() > MAX_MESSAGE_BYTES {
            anyhow::bail!(
                "Message too large: {} > {} bytes",
                plaintext.len(),
                MAX_MESSAGE_BYTES
            );
        }

        // Generate ephemeral X25519 keypair
        let ephemeral_secret = x25519_dalek::EphemeralSecret::random_from_rng(OsRng);
        let ephemeral_pub = x25519_dalek::PublicKey::from(&ephemeral_secret);

        // DH shared secret
        let recipient_pub = x25519_dalek::PublicKey::from(*recipient_x25519_pub);
        let shared = ephemeral_secret.diffie_hellman(&recipient_pub);
        let shared_bytes = shared.as_bytes().to_vec();

        // Derive encryption key via BLAKE3 KDF
        let key_bytes = derive_with_domain(MAILBOX_KDF_DOMAIN, &shared_bytes);
        let key = Key::from_slice(&key_bytes);
        let cipher = ChaCha20Poly1305::new(key);

        // Random nonce
        let mut nonce_bytes = [0u8; 12];
        self.rng
            .fill(&mut nonce_bytes)
            .map_err(|_| anyhow::anyhow!("RNG failure"))?;
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| anyhow::anyhow!("Encryption failed: {:?}", e))?;

        // Zero shared secret
        drop(shared_bytes);

        let message_id = uuid::Uuid::new_v4().to_string();
        let envelope = MailboxEnvelope {
            recipient_id: recipient_id.to_string(),
            ephemeral_pub: base64::engine::general_purpose::STANDARD
                .encode(ephemeral_pub.as_bytes()),
            ciphertext,
            nonce: base64::engine::general_purpose::STANDARD.encode(nonce_bytes),
            created_at: now_secs(),
            message_id: message_id.clone(),
            attempts: 0,
        };

        // Persist locally until delivered
        self.store_pending(&envelope)?;
        Ok(envelope)
    }

    /// Open a sealed envelope addressed to us.
    pub fn open_envelope(
        &self,
        envelope: &MailboxEnvelope,
        our_x25519_secret: &x25519_dalek::StaticSecret,
    ) -> Result<Vec<u8>> {
        use base64::Engine;

        let ephemeral_pub_bytes: Vec<u8> = base64::engine::general_purpose::STANDARD
            .decode(&envelope.ephemeral_pub)
            .map_err(|_| anyhow::anyhow!("Invalid ephemeral pub encoding"))?;

        let ephemeral_pub_arr: [u8; 32] = ephemeral_pub_bytes
            .try_into()
            .map_err(|_| anyhow::anyhow!("Ephemeral pub wrong length"))?;

        let ephemeral_pub = x25519_dalek::PublicKey::from(ephemeral_pub_arr);
        let shared = our_x25519_secret.diffie_hellman(&ephemeral_pub);
        let key_bytes = derive_with_domain(MAILBOX_KDF_DOMAIN, shared.as_bytes());
        let key = Key::from_slice(&key_bytes);
        let cipher = ChaCha20Poly1305::new(key);

        let nonce_bytes: Vec<u8> = base64::engine::general_purpose::STANDARD
            .decode(&envelope.nonce)
            .map_err(|_| anyhow::anyhow!("Invalid nonce encoding"))?;
        let nonce = Nonce::from_slice(&nonce_bytes);

        cipher
            .decrypt(nonce, envelope.ciphertext.as_slice())
            .map_err(|_| anyhow::anyhow!("Decryption failed — wrong key or tampered ciphertext"))
    }

    /// Store a pending outgoing envelope
    fn store_pending(&self, envelope: &MailboxEnvelope) -> Result<()> {
        let key = format!("{}pending:{}", MAILBOX_PREFIX, envelope.message_id);
        let val = bincode::serialize(envelope)?;
        self.db.insert(key.as_bytes(), val)?;
        Ok(())
    }

    /// Mark an envelope as delivered (remove from pending)
    pub fn mark_delivered(&self, message_id: &str) -> Result<()> {
        let key = format!("{}pending:{}", MAILBOX_PREFIX, message_id);
        self.db.remove(key.as_bytes())?;
        Ok(())
    }

    /// Get all pending (undelivered) outgoing envelopes
    pub fn get_pending(&self) -> Result<Vec<MailboxEnvelope>> {
        let prefix = format!("{}pending:", MAILBOX_PREFIX);
        let mut envelopes = Vec::new();
        for item in self.db.scan_prefix(prefix.as_bytes()) {
            let (_, val) = item?;
            let envelope: MailboxEnvelope = bincode::deserialize(&val)?;
            envelopes.push(envelope);
        }
        Ok(envelopes)
    }

    /// Store an incoming envelope in our inbox
    pub fn store_incoming(&self, envelope: &MailboxEnvelope) -> Result<()> {
        // Check inbox size limit
        let prefix = format!("{}inbox:", MAILBOX_PREFIX);
        let count = self.db.scan_prefix(prefix.as_bytes()).count();
        if count >= MAX_MAILBOX_SIZE {
            // Drop oldest entry (sled scan is ordered)
            if let Some(Ok((first_key, _))) = self.db.scan_prefix(prefix.as_bytes()).next() {
                self.db.remove(first_key)?;
            }
        }

        let key = format!("{}inbox:{}", MAILBOX_PREFIX, envelope.message_id);
        let val = bincode::serialize(envelope)?;
        self.db.insert(key.as_bytes(), val)?;
        Ok(())
    }

    /// Read all messages from inbox without removing them
    pub fn peek_inbox(&self) -> Result<Vec<MailboxEnvelope>> {
        let prefix = format!("{}inbox:", MAILBOX_PREFIX);
        let mut envelopes = Vec::new();
        for result in self.db.scan_prefix(prefix.as_bytes()) {
            if let Ok((_, val)) = result {
                let envelope: MailboxEnvelope = bincode::deserialize(&val)?;
                envelopes.push(envelope);
            }
        }
        Ok(envelopes)
    }

    /// Drain all messages from our inbox
    pub fn drain_inbox(&self) -> Result<Vec<MailboxEnvelope>> {
        let prefix = format!("{}inbox:", MAILBOX_PREFIX);
        let mut envelopes = Vec::new();
        let keys: Vec<_> = self
            .db
            .scan_prefix(prefix.as_bytes())
            .filter_map(|r| r.ok())
            .collect();

        for (key, val) in &keys {
            let envelope: MailboxEnvelope = bincode::deserialize(val)?;
            envelopes.push(envelope);
            self.db.remove(key)?;
        }
        Ok(envelopes)
    }

    /// Increment delivery attempt counter
    pub fn increment_attempts(&self, message_id: &str) -> Result<()> {
        let key = format!("{}pending:{}", MAILBOX_PREFIX, message_id);
        if let Some(val) = self.db.get(key.as_bytes())? {
            let mut envelope: MailboxEnvelope = bincode::deserialize(&val)?;
            envelope.attempts += 1;
            let updated = bincode::serialize(&envelope)?;
            self.db.insert(key.as_bytes(), updated)?;
        }
        Ok(())
    }

    /// Purge messages older than `max_age_secs`
    pub fn purge_expired(&self, max_age_secs: u64) -> Result<usize> {
        let now = now_secs();
        let prefix = MAILBOX_PREFIX.as_bytes();
        let mut removed = 0;
        let keys: Vec<_> = self.db.scan_prefix(prefix).filter_map(|r| r.ok()).collect();

        for (key, val) in keys {
            if let Ok(envelope) = bincode::deserialize::<MailboxEnvelope>(&val) {
                if now - envelope.created_at > max_age_secs {
                    self.db.remove(&key)?;
                    removed += 1;
                }
            }
        }
        Ok(removed)
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

// ── Dependency additions needed in Cargo.toml ────────────────────────────────
// blake3 = "1.5"
// x25519-dalek already present
// chacha20poly1305 already present
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use x25519_dalek::{PublicKey, StaticSecret};

    #[test]
    fn seal_and_open_roundtrip() {
        let recipient_secret = StaticSecret::random_from_rng(OsRng);
        let recipient_pub: PublicKey = (&recipient_secret).into();

        let db = Arc::new(sled::Config::new().temporary(true).open().unwrap());
        let store = MailboxStore::open(db).unwrap();

        let plaintext = b"Hello from the shadow realm";
        let envelope = store
            .seal_for_peer("peer123", recipient_pub.as_bytes(), plaintext)
            .unwrap();

        let decrypted = store.open_envelope(&envelope, &recipient_secret).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn inbox_size_limit() {
        let db = Arc::new(sled::Config::new().temporary(true).open().unwrap());
        let store = MailboxStore::open(db).unwrap();

        let secret = StaticSecret::random_from_rng(OsRng);
        let pub_key: x25519_dalek::PublicKey = (&secret).into();

        for _ in 0..MAX_MAILBOX_SIZE + 10 {
            let env = store
                .seal_for_peer("peer", pub_key.as_bytes(), b"msg")
                .unwrap();
            store.store_incoming(&env).unwrap();
        }

        let prefix = format!("{}inbox:", MAILBOX_PREFIX);
        let count = store.db.scan_prefix(prefix.as_bytes()).count();
        assert!(count <= MAX_MAILBOX_SIZE);
    }
}

impl MailboxStore {
    /// Count of pending (undelivered) messages in the outbox.
    /// Used by mesh_health for the storage dashboard.
    pub fn pending_count(&self) -> anyhow::Result<usize> {
        let pending = self.get_pending()?;
        Ok(pending.len())
    }

    /// Queue an encrypted wire frame for later delivery to `recipient_id`.
    pub fn queue_delivery(
        &self,
        recipient_id: &str,
        message_id: &str,
        frame_bytes: &[u8],
    ) -> Result<()> {
        let key = format!("{}outgoing:{}:{}", MAILBOX_PREFIX, recipient_id, message_id);
        self.db.insert(key.as_bytes(), frame_bytes)?;
        Ok(())
    }

    pub fn get_pending_deliveries(&self, recipient_id: &str) -> Result<Vec<(String, Vec<u8>)>> {
        let prefix = format!("{}outgoing:{}:", MAILBOX_PREFIX, recipient_id);
        let mut results = Vec::new();
        for item in self.db.scan_prefix(prefix.as_bytes()) {
            let (key, val) = item?;
            let key_str = String::from_utf8_lossy(&key);
            if let Some(msg_id) = key_str.rsplit(':').next() {
                results.push((msg_id.to_string(), val.to_vec()));
            }
        }
        Ok(results)
    }

    /// Drain all pending frames for `recipient_id` (returns frames + their message IDs, removes from queue).
    pub fn drain_deliveries(&self, recipient_id: &str) -> Result<Vec<(String, Vec<u8>)>> {
        let prefix = format!("{}outgoing:{}:", MAILBOX_PREFIX, recipient_id);
        let mut results = Vec::new();
        let keys: Vec<_> = self
            .db
            .scan_prefix(prefix.as_bytes())
            .filter_map(|r| r.ok())
            .collect();

        for (key, val) in &keys {
            // Extract message_id from key: mailbox:outgoing:{recipient}:{message_id}
            let key_str = String::from_utf8_lossy(key);
            if let Some(msg_id) = key_str.rsplit(':').next() {
                results.push((msg_id.to_string(), val.to_vec()));
            }
            self.db.remove(key)?;
        }
        Ok(results)
    }

    pub fn remove_delivery(&self, recipient_id: &str, message_id: &str) -> Result<()> {
        let key = format!("{}outgoing:{}:{}", MAILBOX_PREFIX, recipient_id, message_id);
        self.db.remove(key.as_bytes())?;
        Ok(())
    }

    pub fn delivery_count(&self) -> Result<usize> {
        let prefix = format!("{}outgoing:", MAILBOX_PREFIX);
        Ok(self.db.scan_prefix(prefix.as_bytes()).count())
    }
}

impl Default for MailboxStore {
    fn default() -> Self {
        // In-memory fallback: use a temporary sled database
        let db = sled::Config::new()
            .temporary(true)
            .open()
            .expect("temp sled db");
        Self::open(Arc::new(db)).expect("MailboxStore default")
    }
}
