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

use crate::core::persistence_manager::PersistenceManager;
use crate::genesis_constants::{derive_with_domain, MAILBOX_KDF_DOMAIN};
use base64::Engine;

use anyhow::Result;
use chacha20poly1305::{
    aead::{Aead, KeyInit, Payload},
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
const MAILBOX_AAD_DOMAIN: &[u8] = b"ghostwire/mailbox-envelope/v1";
const MAILBOX_STORAGE_DOMAIN: &str = "ghostwire/mailbox-store/v1";
const PERSISTENCE_MAGIC: &[u8] = b"GWPE";
const PERSISTENCE_VERSION: u8 = 1;

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
    persistence: Arc<PersistenceManager>,
    rng: SystemRandom,
}

fn mailbox_aad(
    recipient_id: &str,
    ephemeral_pub: &str,
    nonce: &str,
    created_at: u64,
    message_id: &str,
) -> Vec<u8> {
    // Bind immutable metadata. attempts is mutable retry state and is excluded
    // so delivery retries do not invalidate an already-sealed ciphertext.
    let mut aad = Vec::with_capacity(MAILBOX_AAD_DOMAIN.len() + 128);
    aad.extend_from_slice(MAILBOX_AAD_DOMAIN);
    for field in [
        recipient_id.as_bytes(),
        ephemeral_pub.as_bytes(),
        nonce.as_bytes(),
        message_id.as_bytes(),
    ] {
        aad.extend_from_slice(&(field.len() as u32).to_le_bytes());
        aad.extend_from_slice(field);
    }
    aad.extend_from_slice(&created_at.to_le_bytes());
    aad
}

impl MailboxStore {
    pub fn open(db: Arc<Db>, persistence: Arc<PersistenceManager>) -> Result<Self> {
        Ok(Self {
            db,
            persistence,
            rng: SystemRandom::new(),
        })
    }

    fn ensure_user_unlocked(&self) -> Result<()> {
        self.persistence.ensure_user_unlocked()
    }

    fn encrypt_record(&self, key: &str, plaintext: &[u8]) -> Result<Vec<u8>> {
        self.persistence
            .encrypt_user_record(MAILBOX_STORAGE_DOMAIN, key, plaintext)
    }

    fn decrypt_record(&self, key: &str, stored: &[u8]) -> Result<Vec<u8>> {
        self.persistence
            .decrypt_user_record(MAILBOX_STORAGE_DOMAIN, key, stored)
    }

    fn is_wrapped_record(value: &[u8]) -> bool {
        value.len() >= PERSISTENCE_MAGIC.len() + 1
            && &value[..PERSISTENCE_MAGIC.len()] == PERSISTENCE_MAGIC
            && value[PERSISTENCE_MAGIC.len()] == PERSISTENCE_VERSION
    }

    /// Re-wrap pre-keyslot mailbox records after the operator has explicitly
    /// unlocked or migrated the profile. Raw legacy records are never exposed
    /// while the profile is locked, and malformed records abort migration.
    pub fn migrate_legacy_records(&self) -> Result<usize> {
        self.ensure_user_unlocked()?;
        let mut updates = Vec::new();
        for item in self.db.scan_prefix(MAILBOX_PREFIX.as_bytes()) {
            let (key, value) = item?;
            if Self::is_wrapped_record(&value) {
                continue;
            }
            let key_string = String::from_utf8(key.to_vec())
                .map_err(|_| anyhow::anyhow!("Mailbox key is not valid UTF-8"))?;
            if key_string.contains("pending:") || key_string.contains("inbox:") {
                bincode::deserialize::<MailboxEnvelope>(&value)
                    .map_err(|_| anyhow::anyhow!("Malformed legacy mailbox envelope"))?;
            } else if value.len() > 256 * 1024 * 1024 {
                anyhow::bail!("Legacy mailbox frame exceeds maximum size");
            }
            let encrypted = self.encrypt_record(&key_string, &value)?;
            updates.push((key, encrypted));
        }
        for (key, value) in &updates {
            self.db.insert(key, value.as_slice())?;
        }
        if !updates.is_empty() {
            self.db.flush()?;
        }
        Ok(updates.len())
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

        let message_id = uuid::Uuid::new_v4().to_string();
        let created_at = now_secs();
        let ephemeral_pub_b64 =
            base64::engine::general_purpose::STANDARD.encode(ephemeral_pub.as_bytes());
        let nonce_b64 = base64::engine::general_purpose::STANDARD.encode(nonce_bytes);
        let aad = mailbox_aad(
            recipient_id,
            &ephemeral_pub_b64,
            &nonce_b64,
            created_at,
            &message_id,
        );
        let ciphertext = cipher
            .encrypt(
                nonce,
                Payload {
                    msg: plaintext,
                    aad: &aad,
                },
            )
            .map_err(|e| anyhow::anyhow!("Encryption failed: {:?}", e))?;

        // Zero shared secret
        drop(shared_bytes);

        let envelope = MailboxEnvelope {
            recipient_id: recipient_id.to_string(),
            ephemeral_pub: ephemeral_pub_b64,
            ciphertext,
            nonce: nonce_b64,
            created_at,
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
        if nonce_bytes.len() != 12 {
            anyhow::bail!(
                "Invalid nonce length: expected 12 bytes, got {}",
                nonce_bytes.len()
            );
        }
        let nonce = Nonce::from_slice(&nonce_bytes);
        let aad = mailbox_aad(
            &envelope.recipient_id,
            &envelope.ephemeral_pub,
            &envelope.nonce,
            envelope.created_at,
            &envelope.message_id,
        );

        cipher
            .decrypt(
                nonce,
                Payload {
                    msg: envelope.ciphertext.as_slice(),
                    aad: &aad,
                },
            )
            .map_err(|_| anyhow::anyhow!("Decryption failed — wrong key or tampered ciphertext"))
    }

    /// Store a pending outgoing envelope
    fn store_pending(&self, envelope: &MailboxEnvelope) -> Result<()> {
        self.ensure_user_unlocked()?;
        let key = format!("{}pending:{}", MAILBOX_PREFIX, envelope.message_id);
        let val = bincode::serialize(envelope)?;
        let encrypted = self.encrypt_record(&key, &val)?;
        self.db.insert(key.as_bytes(), encrypted)?;
        Ok(())
    }

    /// Mark an envelope as delivered (remove from pending)
    pub fn mark_delivered(&self, message_id: &str) -> Result<()> {
        self.ensure_user_unlocked()?;
        let key = format!("{}pending:{}", MAILBOX_PREFIX, message_id);
        self.db.remove(key.as_bytes())?;
        Ok(())
    }

    /// Get all pending (undelivered) outgoing envelopes
    pub fn get_pending(&self) -> Result<Vec<MailboxEnvelope>> {
        self.ensure_user_unlocked()?;
        let prefix = format!("{}pending:", MAILBOX_PREFIX);
        let mut envelopes = Vec::new();
        for item in self.db.scan_prefix(prefix.as_bytes()) {
            let (key, val) = item?;
            let key = String::from_utf8(key.to_vec())?;
            let plaintext = self.decrypt_record(&key, &val)?;
            let envelope: MailboxEnvelope = bincode::deserialize(&plaintext)?;
            envelopes.push(envelope);
        }
        Ok(envelopes)
    }

    /// Store an incoming envelope in our inbox
    pub fn store_incoming(&self, envelope: &MailboxEnvelope) -> Result<()> {
        self.ensure_user_unlocked()?;
        // Check inbox size limit
        let prefix = format!("{}inbox:", MAILBOX_PREFIX);
        let count = self.db.scan_prefix(prefix.as_bytes()).count();
        if count >= MAX_MAILBOX_SIZE {
            // Sled orders these keys by UUID, not by message creation time.
            // Decode envelopes and select the oldest timestamp deterministically.
            let mut oldest: Option<(u64, String, sled::IVec)> = None;
            let mut malformed_key: Option<sled::IVec> = None;
            for item in self.db.scan_prefix(prefix.as_bytes()) {
                let (key, value) = item?;
                let key_string = String::from_utf8(key.to_vec())?;
                let plaintext = self.decrypt_record(&key_string, &value)?;
                match bincode::deserialize::<MailboxEnvelope>(&plaintext) {
                    Ok(existing) => {
                        let candidate = (existing.created_at, existing.message_id.clone(), key);
                        if oldest
                            .as_ref()
                            .map(|current| (candidate.0, &candidate.1) < (current.0, &current.1))
                            .unwrap_or(true)
                        {
                            oldest = Some(candidate);
                        }
                    }
                    Err(_) if malformed_key.is_none() => malformed_key = Some(key),
                    Err(_) => {}
                }
            }
            if let Some((_, _, key)) = oldest {
                self.db.remove(key)?;
            } else if let Some(key) = malformed_key {
                self.db.remove(key)?;
            }
        }

        let key = format!("{}inbox:{}", MAILBOX_PREFIX, envelope.message_id);
        let val = bincode::serialize(envelope)?;
        let encrypted = self.encrypt_record(&key, &val)?;
        self.db.insert(key.as_bytes(), encrypted)?;
        Ok(())
    }

    /// Read all messages from inbox without removing them
    pub fn peek_inbox(&self) -> Result<Vec<MailboxEnvelope>> {
        self.ensure_user_unlocked()?;
        let prefix = format!("{}inbox:", MAILBOX_PREFIX);
        let mut envelopes = Vec::new();
        for result in self.db.scan_prefix(prefix.as_bytes()) {
            let (key, val) = result?;
            let key = String::from_utf8(key.to_vec())?;
            let plaintext = self.decrypt_record(&key, &val)?;
            let envelope: MailboxEnvelope = bincode::deserialize(&plaintext)?;
            envelopes.push(envelope);
        }
        Ok(envelopes)
    }

    /// Drain all messages from our inbox
    pub fn drain_inbox(&self) -> Result<Vec<MailboxEnvelope>> {
        self.ensure_user_unlocked()?;
        let prefix = format!("{}inbox:", MAILBOX_PREFIX);
        let mut envelopes = Vec::new();
        let keys: Vec<_> = self
            .db
            .scan_prefix(prefix.as_bytes())
            .collect::<std::result::Result<Vec<_>, _>>()?;

        for (key, val) in &keys {
            let key_string = String::from_utf8(key.to_vec())?;
            let plaintext = self.decrypt_record(&key_string, val)?;
            let envelope: MailboxEnvelope = bincode::deserialize(&plaintext)?;
            envelopes.push(envelope);
            self.db.remove(key)?;
        }
        Ok(envelopes)
    }

    /// Increment delivery attempt counter
    pub fn increment_attempts(&self, message_id: &str) -> Result<()> {
        self.ensure_user_unlocked()?;
        let key = format!("{}pending:{}", MAILBOX_PREFIX, message_id);
        if let Some(val) = self.db.get(key.as_bytes())? {
            let plaintext = self.decrypt_record(&key, &val)?;
            let mut envelope: MailboxEnvelope = bincode::deserialize(&plaintext)?;
            envelope.attempts += 1;
            let updated = bincode::serialize(&envelope)?;
            let encrypted = self.encrypt_record(&key, &updated)?;
            self.db.insert(key.as_bytes(), encrypted)?;
        }
        Ok(())
    }

    /// Purge messages older than `max_age_secs`
    pub fn purge_expired(&self, max_age_secs: u64) -> Result<usize> {
        self.ensure_user_unlocked()?;
        let now = now_secs();
        let prefix = MAILBOX_PREFIX.as_bytes();
        let mut removed = 0;
        let keys: Vec<_> = self
            .db
            .scan_prefix(prefix)
            .collect::<std::result::Result<Vec<_>, _>>()?;

        for (key, val) in keys {
            let key_string = String::from_utf8(key.to_vec())?;
            if key_string.contains("pending:") || key_string.contains("inbox:") {
                let plaintext = self.decrypt_record(&key_string, &val)?;
                let envelope = bincode::deserialize::<MailboxEnvelope>(&plaintext)?;
                if now.saturating_sub(envelope.created_at) > max_age_secs {
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

    fn test_store() -> (MailboxStore, tempfile::TempDir) {
        let home = tempfile::tempdir().unwrap();
        let persistence = PersistenceManager::new_with_base_dir_and_passphrase(
            home.path().join("vault"),
            Some("mailbox-test".to_string()),
            "mailbox-test-passphrase",
        )
        .unwrap();
        let db = Arc::new(sled::Config::new().temporary(true).open().unwrap());
        (MailboxStore::open(db, Arc::new(persistence)).unwrap(), home)
    }

    #[test]
    fn seal_and_open_roundtrip() {
        let recipient_secret = StaticSecret::random_from_rng(OsRng);
        let recipient_pub: PublicKey = (&recipient_secret).into();

        let (store, _home) = test_store();

        let plaintext = b"Hello from the shadow realm";
        let envelope = store
            .seal_for_peer("peer123", recipient_pub.as_bytes(), plaintext)
            .unwrap();

        let decrypted = store.open_envelope(&envelope, &recipient_secret).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn envelope_metadata_tampering_is_rejected() {
        let recipient_secret = StaticSecret::random_from_rng(OsRng);
        let recipient_pub: PublicKey = (&recipient_secret).into();
        let (store, _home) = test_store();
        let envelope = store
            .seal_for_peer("peer123", recipient_pub.as_bytes(), b"metadata")
            .unwrap();
        let mut tampered = envelope.clone();
        tampered.recipient_id = "different-peer".to_string();
        assert!(store.open_envelope(&tampered, &recipient_secret).is_err());
        let mut tampered = envelope.clone();
        tampered.created_at = tampered.created_at.saturating_add(1);
        assert!(store.open_envelope(&tampered, &recipient_secret).is_err());
    }

    #[test]
    fn malformed_nonce_is_rejected_without_panic() {
        let recipient_secret = StaticSecret::random_from_rng(OsRng);
        let recipient_pub: PublicKey = (&recipient_secret).into();
        let (store, _home) = test_store();
        let mut envelope = store
            .seal_for_peer("peer123", recipient_pub.as_bytes(), b"nonce")
            .unwrap();
        envelope.nonce = base64::engine::general_purpose::STANDARD.encode([1u8; 4]);
        let error = store
            .open_envelope(&envelope, &recipient_secret)
            .unwrap_err();
        assert!(error.to_string().contains("nonce length"));
    }

    #[test]
    fn mailbox_pending_record_is_not_raw_bincode_at_rest() {
        let (store, _home) = test_store();
        let recipient_secret = StaticSecret::random_from_rng(OsRng);
        let recipient_pub: PublicKey = (&recipient_secret).into();
        let envelope = store
            .seal_for_peer("peer123", recipient_pub.as_bytes(), b"at-rest-red")
            .unwrap();
        let key = format!("{}pending:{}", MAILBOX_PREFIX, envelope.message_id);
        let raw = store.db.get(key.as_bytes()).unwrap().unwrap();
        assert!(
            raw.starts_with(b"GWPE"),
            "mailbox records must use authenticated persistence envelopes at rest"
        );
    }

    #[test]
    fn inbox_size_limit_evicts_oldest_timestamp_not_uuid_order() {
        let (store, _home) = test_store();
        let make = |message_id: &str, created_at: u64| MailboxEnvelope {
            recipient_id: "peer".into(),
            ephemeral_pub: String::new(),
            ciphertext: Vec::new(),
            nonce: String::new(),
            created_at,
            message_id: message_id.into(),
            attempts: 0,
        };
        for index in 0..MAX_MAILBOX_SIZE {
            store
                .store_incoming(&make(&format!("z-{index:06}"), 100 + index as u64))
                .unwrap();
        }
        // This UUID sorts before existing keys but is newer; UUID order must
        // not decide the eviction victim.
        store.store_incoming(&make("a-newer", 10_000)).unwrap();
        let inbox = store.peek_inbox().unwrap();
        assert_eq!(inbox.len(), MAX_MAILBOX_SIZE);
        assert!(!inbox.iter().any(|entry| entry.message_id == "z-000000"));
        assert!(inbox.iter().any(|entry| entry.message_id == "a-newer"));
    }

    #[test]
    fn mailbox_records_are_wrapped_and_locked_access_fails_closed() {
        let (store, _home) = test_store();
        let recipient_secret = StaticSecret::random_from_rng(OsRng);
        let recipient_pub: PublicKey = (&recipient_secret).into();
        let envelope = store
            .seal_for_peer("peer123", recipient_pub.as_bytes(), b"at-rest")
            .unwrap();
        let key = format!("{}pending:{}", MAILBOX_PREFIX, envelope.message_id);
        let raw = store.db.get(key.as_bytes()).unwrap().unwrap();
        assert!(MailboxStore::is_wrapped_record(&raw));
        assert!(bincode::deserialize::<MailboxEnvelope>(&raw).is_err());

        store.persistence.lock().unwrap();
        assert!(store.get_pending().is_err());
        assert!(store.mark_delivered(&envelope.message_id).is_err());
    }

    #[test]
    fn explicit_legacy_mailbox_migration_wraps_existing_records() {
        let (store, _home) = test_store();
        let envelope = MailboxEnvelope {
            recipient_id: "peer".into(),
            ephemeral_pub: String::new(),
            ciphertext: Vec::new(),
            nonce: String::new(),
            created_at: 100,
            message_id: "legacy-message".into(),
            attempts: 0,
        };
        let key = format!("{}inbox:{}", MAILBOX_PREFIX, envelope.message_id);
        store
            .db
            .insert(key.as_bytes(), bincode::serialize(&envelope).unwrap())
            .unwrap();
        assert!(store.peek_inbox().is_err());
        assert_eq!(store.migrate_legacy_records().unwrap(), 1);
        let raw = store.db.get(key.as_bytes()).unwrap().unwrap();
        assert!(MailboxStore::is_wrapped_record(&raw));
        assert_eq!(store.peek_inbox().unwrap()[0].message_id, "legacy-message");
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
        self.ensure_user_unlocked()?;
        let key = format!("{}outgoing:{}:{}", MAILBOX_PREFIX, recipient_id, message_id);
        let encrypted = self.encrypt_record(&key, frame_bytes)?;
        self.db.insert(key.as_bytes(), encrypted)?;
        Ok(())
    }

    pub fn get_pending_deliveries(&self, recipient_id: &str) -> Result<Vec<(String, Vec<u8>)>> {
        self.ensure_user_unlocked()?;
        let prefix = format!("{}outgoing:{}:", MAILBOX_PREFIX, recipient_id);
        let mut results = Vec::new();
        for item in self.db.scan_prefix(prefix.as_bytes()) {
            let (key, val) = item?;
            let key_str = String::from_utf8(key.to_vec())?;
            let plaintext = self.decrypt_record(&key_str, &val)?;
            if let Some(msg_id) = key_str.rsplit(':').next() {
                results.push((msg_id.to_string(), plaintext));
            }
        }
        Ok(results)
    }

    /// Drain all pending frames for `recipient_id` (returns frames + their message IDs, removes from queue).
    pub fn drain_deliveries(&self, recipient_id: &str) -> Result<Vec<(String, Vec<u8>)>> {
        self.ensure_user_unlocked()?;
        let prefix = format!("{}outgoing:{}:", MAILBOX_PREFIX, recipient_id);
        let mut results = Vec::new();
        let keys: Vec<_> = self
            .db
            .scan_prefix(prefix.as_bytes())
            .collect::<std::result::Result<Vec<_>, _>>()?;

        for (key, val) in &keys {
            let key_str = String::from_utf8(key.to_vec())?;
            let plaintext = self.decrypt_record(&key_str, val)?;
            if let Some(msg_id) = key_str.rsplit(':').next() {
                results.push((msg_id.to_string(), plaintext));
            }
            self.db.remove(key)?;
        }
        Ok(results)
    }

    pub fn remove_delivery(&self, recipient_id: &str, message_id: &str) -> Result<()> {
        self.ensure_user_unlocked()?;
        let key = format!("{}outgoing:{}:{}", MAILBOX_PREFIX, recipient_id, message_id);
        self.db.remove(key.as_bytes())?;
        Ok(())
    }

    pub fn delivery_count(&self) -> Result<usize> {
        self.ensure_user_unlocked()?;
        let prefix = format!("{}outgoing:", MAILBOX_PREFIX);
        Ok(self.db.scan_prefix(prefix.as_bytes()).count())
    }
}

// NOTE: `impl Default for MailboxStore` was removed 2026-09-14 (fail-closed
// fix). It built a temporary on-disk store encrypted with a passphrase
// published in source ("temporary-mailbox-fallback-passphrase") and silently
// received mail when the real store failed to open. Tests construct stores
// via `MailboxStore::open` (see tests::test_store) and are unaffected.
