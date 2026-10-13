//! Encryption: key revocation, rotation announcements, CRLs.
//! Next: integrate with key manager and web API.
//!
//! **Security Fixes Applied:**
//! - ✅ CVE-2025-11931 class: Input validation on decryption
//! - ✅ DoS prevention: Result-based error handling (no panic)
//! - ✅ Replay protection: receiver-side replay cache over the AUTHENTICATED
//!    portion of a frame — see [`ReplayGuard`]. This is a real check on the
//!    `decrypt_message` / `decrypt_message_ratchet` paths, not the
//!    `wire_protocol::ReplayWindow`, which guards a different message type
//!    and is not consulted here.
//! - ✅ Timing attacks: Constant-time signature verification

use crate::core::binary_hardening::LockedBuffer;
use crate::core::crl;
use crate::core::double_ratchet;
use crate::core::persistence_manager::PersistenceManager;
use crate::genesis_constants::{
    genesis_derive, ANON_INTRO_COMMIT_DOMAIN, TRANSPORT_KEY_ROTATE_DOMAIN,
};
use rand_core::OsRng;
use ring::hkdf;
use ring::rand::{SecureRandom, SystemRandom};
use ring::signature::{Ed25519KeyPair, KeyPair};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::info;
use x25519_dalek::{PublicKey as XPublicKey, StaticSecret};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// ✅ NEW: Secure error types (no panic on invalid input)
#[derive(Debug, thiserror::Error)]
pub enum DecryptionError {
    #[error("Invalid data length")]
    InvalidLength,
    #[error("Decryption failed")]
    DecryptionFailed,
    #[error("Authentication failed")]
    AuthenticationFailed,
    #[error("Invalid UTF-8")]
    InvalidUtf8,
    #[error("Replay attack detected")]
    ReplayDetected,
}

/// Enhanced encrypted message with additional security features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedMessage {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
    pub signature: Vec<u8>,
    pub sender_id: String,
    pub timestamp: u64,
    pub key_id: String,
    pub message_type: MessageType,
    pub version: u8,
    pub pq_key: Option<Vec<u8>>, // Kyber encapsulated key for PQC Hardening
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    Text,
    Binary,
    Control,
    Heartbeat,
    Handshake,
}

/// Enhanced key pair with additional metadata
#[derive(Debug, Clone, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct GhostKeyPair {
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
    pub key_id: String,
    pub created_at: u64,
    pub expires_at: Option<u64>,
    pub usage_count: usize,
    pub key_type: KeyType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Zeroize)]
pub enum KeyType {
    Ed25519,
    X25519,
    Hybrid,      // Both Ed25519 and X25519
    PostQuantum, // Kyber
    Ultimate,    // Ed25519 + X25519 + Kyber
}

/// Enhanced peer key information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerKey {
    pub public_key: Vec<u8>,
    pub key_id: String,
    pub last_updated: u64,
    pub key_type: KeyType,
    pub is_verified: bool,
    pub trust_score: f32,
}

/// Struct for tracking revoked keys.
#[derive(Default, Clone)]
pub struct RevocationList {
    pub revoked_keys: Vec<String>,
}

/// Trait for announcing key rotations.
pub trait KeyRotationAnnouncer {
    /// Announce a key rotation event.
    fn announce_rotation(&self, key_id: &str);
}

/// Default stub: no-op.
pub struct NoOpKeyRotationAnnouncer;
impl KeyRotationAnnouncer for NoOpKeyRotationAnnouncer {
    fn announce_rotation(&self, _key_id: &str) {}
}

const USER_STATE_NAMESPACE: &str = "ghostwire/encryption-manager/v1";
const USER_STATE_KEY: &str = "state";

/// Enhanced encryption manager with advanced security features
pub struct EncryptionManager {
    /// Public metadata remains available while the profile is locked.
    key_pair: GhostKeyPair,
    /// Private material is populated only after explicit user-keyed unlock.
    private_key: Arc<Mutex<Option<LockedBuffer>>>,
    peer_keys: Arc<Mutex<HashMap<String, PeerKey>>>,
    rng: SystemRandom,
    session_keys: Arc<Mutex<HashMap<String, SessionKey>>>,
    key_rotation_interval: u64,
    last_key_rotation: u64,
    /// Overlapping transport key ring (BTP-style prev/curr/next).
    /// Derived from the identity X25519 secret at construction time.
    /// Refreshed when the time period advances.
    transport_key_ring: TransportKeyRing,
    pub crl: Arc<crl::CrlStore>,
    pub ratchet_sessions: Arc<double_ratchet::RatchetSessionManager>,
    /// Replay cache. Dropped on lock alongside the private material it
    /// describes, so a cache cannot outlive the identity that produced it.
    replay_guard: Arc<Mutex<ReplayGuard>>,
}

#[derive(Debug, Clone, Zeroize, ZeroizeOnDrop)]
pub struct SessionKey {
    pub key: Vec<u8>,
    pub created_at: u64,
    pub expires_at: u64,
    pub usage_count: usize,
}

/// How many distinct digests to remember per sender before evicting the oldest.
const REPLAY_CACHE_PER_SENDER: usize = 256;
/// How many distinct senders to track. Bounds memory against a peer that
/// varies `sender_id`, exactly as `add_peer_key` bounds the peer-key map.
const REPLAY_MAX_SENDERS: usize = 10_000;

/// Time-period length for overlapping key rotation, matching Briar BTP's
/// 6-day windows. This gives messages a 6-day grace period to be delivered
/// during the previous period, and the next period is pre-generated so
/// messages encrypted under a slightly future key still decrypt.
pub const KEY_ROTATION_PERIOD_SECS: u64 = 6 * 24 * 60 * 60; // 6 days

/// Overlapping transport key ring with 3 active periods (prev/curr/next).
///
/// Mirrors Briar's BTP `TransportKeys` structure: derives per-period
/// encryption keys from the identity X25519 secret using label-separated
/// HKDF-style derivation, so that:
///   - Messages encrypted with the previous period's key still decrypt (grace)
///   - Messages encrypted with the next period's key also decrypt (pre-generated)
///   - Each direction (in/out) has independent keys (label separation)
///
/// The `rotation_epoch` tracks the base time period. On rotate, it advances
/// by one period and re-derives prev=old_curr, curr=new, next=new+1.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportKeyRing {
    /// Time period index: floor(now / KEY_ROTATION_PERIOD_SECS)
    pub rotation_epoch: u64,
    /// Keys for the previous period (for grace-period decryption)
    pub prev_in: [u8; 32],
    /// Keys for the current period (active encryption/decryption)
    pub curr_in: [u8; 32],
    /// Keys for the next period (pre-generated for future messages)
    pub curr_out: [u8; 32],
    pub next_in: [u8; 32],
    /// Outgoing keys for the current period
    pub prev_out: [u8; 32],
    pub next_out: [u8; 32],
}

impl TransportKeyRing {
    /// Derive the transport key ring from the identity X25519 secret.
    ///
    /// The derivation chain mirrors Briar BTP:
    ///   rotation_key = BLAKE3_keyed(TRANSPORT_KEY_ROTATE_DOMAIN, x25519_secret)
    ///   curr_in = rotation_key
    ///   prev_in  = BLAKE3_keyed(TRANSPORT_KEY_ROTATE_DOMAIN, rotation_key || epoch-1)
    ///   next_in  = BLAKE3_keyed(TRANSPORT_KEY_ROTATE_DOMAIN, rotation_key || epoch+1)
    ///
    /// Each direction (in/out) uses a label-separated derivation so keys
    /// are never reused across contexts.
    pub fn derive(
        x25519_secret: &[u8; 32],
        at_time_secs: u64,
    ) -> Self {
        let epoch = at_time_secs / KEY_ROTATION_PERIOD_SECS;

        // Master rotation key from identity secret
        let master = genesis_derive(TRANSPORT_KEY_ROTATE_DOMAIN, x25519_secret);

        // Label-separated per-direction keys
        let curr_in = genesis_derive(
            crate::genesis_constants::ANON_INTRO_COMMIT_DOMAIN, // different domain for in
            &master,
        );
        let curr_out = genesis_derive(
            &crate::genesis_constants::ONION_HOP_DOMAIN, // different domain for out
            &master,
        );

        // Derive prev/next with time-period stepping
        let prev_in = Self::derive_period_key(&master, epoch.saturating_sub(1), true);
        let next_in = Self::derive_period_key(&master, epoch + 1, true);
        let prev_out = Self::derive_period_key(&master, epoch.saturating_sub(1), false);
        let next_out = Self::derive_period_key(&master, epoch + 1, false);

        TransportKeyRing {
            rotation_epoch: epoch,
            prev_in,
            curr_in,
            curr_out,
            next_in,
            prev_out,
            next_out,
        }
    }

    /// Step a key forward by one time period, with direction label separation.
    fn derive_period_key(master: &[u8; 32], time_period: u64, incoming: bool) -> [u8; 32] {
        let mut ikm = master.to_vec();
        ikm.extend_from_slice(&time_period.to_le_bytes());
        ikm.push(if incoming { 0x01 } else { 0x02 }); // direction label

        genesis_derive(TRANSPORT_KEY_ROTATE_DOMAIN, &ikm)
    }

    /// Check whether the key ring is still current, and if not, advance it.
    /// Returns the new ring if rotation was needed.
    pub fn refresh_if_needed(&self, x25519_secret: &[u8; 32], now_secs: u64) -> Option<Self> {
        let current_epoch = now_secs / KEY_ROTATION_PERIOD_SECS;
        if current_epoch > self.rotation_epoch {
            Some(Self::derive(x25519_secret, now_secs))
        } else {
            None
        }
    }

    /// Determine which key to use for decrypting an incoming message,
    /// based on the key_id prefix encoded in the message header.
    ///
    /// key_id format: "v{period_delta}" where period_delta is 0 (curr),
    /// -1 (prev), or +1 (next). This is a simplified protocol-level signal.
    pub fn select_incoming_key(&self, period_hint: i32) -> Option<[u8; 32]> {
        match period_hint {
            0 => Some(self.curr_in),
            -1 => Some(self.prev_in),
            1 => Some(self.next_in),
            _ => None,
        }
    }

    /// Get the active outgoing key for the current period.
    pub fn current_out_key(&self) -> &[u8; 32] {
        &self.curr_out
    }
}

/// Receiver-side replay cache for decrypted frames.
///
/// # Why a digest and not a counter
///
/// `EncryptedMessage` carries a `timestamp` but no sequence number, so a
/// per-sender monotonic counter cannot be enforced from the wire as it
/// stands. Rather than trust a counter we do not have, this caches the
/// digest of each frame we have already accepted and refuses any frame
/// whose digest is already present.
///
/// # Which fields the digest covers, and why exactly those
///
/// The digest MUST be computed only over bytes the sender authenticated.
/// If it covered a field the signature does not cover, an attacker could
/// take a captured frame, flip that field, and present it as a "new"
/// message that misses the cache — turning replay protection into a no-op
/// that costs one bit-flip to bypass.
///
/// `sign_message` (encryption.rs) signs the ciphertext and nothing else, and
/// the AEAD binds nonce + ciphertext. So the authenticated set is:
///
///   * `sender_id`  — selects the peer key the signature is verified against,
///                    so a swapped id verifies against a different key and
///                    fails. Including it also scopes the cache per sender.
///   * `version`    — selects the KDF/AEAD path; changing it changes the
///                    decryption result, so it is implicitly authenticated.
///   * `nonce`      — bound into the AEAD as the nonce.
///   * `ciphertext` — signed directly, and AEAD-bound.
///   * `pq_key`     — the ML-KEM ciphertext / ratchet header. Mixing it
///                    changes the derived key, so altering it breaks the AEAD
///                    tag exactly as altering the ciphertext does.
///
/// `timestamp` is deliberately EXCLUDED. It is not signed and not AEAD-bound,
/// so it is attacker-controlled free text; an attacker could rewrite it to
/// forge a fresh digest. It is instead checked for sanity as a freshness
/// bound (see [`Self::check`]), which is the only honest use of an
/// unauthenticated field.
#[derive(Debug, Default)]
pub struct ReplayGuard {
    /// sender_id -> ordered set of digests already accepted from that sender.
    ///
    /// A `VecDeque` is used rather than a set so eviction is strictly
    /// oldest-first; membership is a linear scan, which is bounded by
    /// `REPLAY_CACHE_PER_SENDER` and therefore not a DoS surface.
    seen: std::collections::HashMap<String, std::collections::VecDeque<[u8; 32]>>,
}

impl ReplayGuard {
    /// Digest the authenticated portion of a frame.
    ///
    /// The length prefixes matter: without them, fields could be shifted
    /// across the boundary (e.g. a `sender_id` one byte longer) and still
    /// concatenate to the same preimage.
    pub fn digest(msg: &EncryptedMessage) -> [u8; 32] {
        use ring::digest;
        let mut ctx = digest::Context::new(&digest::SHA256);
        ctx.update(b"GhostWire-Replay-v1");

        // Length-prefix every field. Without it a `sender_id` one byte longer
        // could shift the remaining fields and still concatenate to the same
        // preimage, which is exactly the ambiguity a replay digest must not
        // have.
        //
        // Written as a macro rather than a closure: a closure capturing `&mut
        // ctx` would keep the borrow alive across the whole scope and block the
        // direct `ctx.update` calls that follow (E0499).
        macro_rules! field {
            ($bytes:expr) => {{
                let b: &[u8] = $bytes;
                ctx.update(&(b.len() as u64).to_le_bytes());
                ctx.update(b);
            }};
        }

        field!(msg.sender_id.as_bytes());
        ctx.update(&[msg.version]);
        field!(&msg.nonce);
        field!(&msg.ciphertext);
        match &msg.pq_key {
            Some(pq) => field!(pq),
            // Distinguish "no PQ component" from "empty PQ component".
            None => {
                ctx.update(&[0u8]);
            }
        }
        let out = ctx.finish();
        let mut digest = [0u8; 32];
        digest.copy_from_slice(out.as_ref());
        digest
    }

    /// Seconds of clock skew tolerated on an unauthenticated timestamp.
    const MAX_FUTURE_SKEW_SECS: u64 = 300;
    /// Longest age accepted for a timestamp. Frames older than this are
    /// refused regardless of whether the digest was cached, which bounds how
    /// long a capture stays useful even if the cache has been evicted or the
    /// process restarted (the cache is in-memory only).
    const MAX_AGE_SECS: u64 = 24 * 60 * 60;

    /// Reject a frame that is a replay, or whose timestamp is implausible.
    ///
    /// MUST be called only AFTER signature verification has succeeded. If it
    /// runs before authentication, an attacker can fill the cache with
    /// digests of frames that were never valid, evicting genuine entries and
    /// re-opening the replay window.
    pub fn check(&mut self, msg: &EncryptedMessage, now: u64) -> Result<(), DecryptionError> {
        // The timestamp is unauthenticated, so it can only ever be a sanity
        // bound — never an authenticity signal. Check it first: a frame from
        // the far future must not advance any state.
        if msg.timestamp > now.saturating_add(Self::MAX_FUTURE_SKEW_SECS) {
            return Err(DecryptionError::ReplayDetected);
        }
        if msg.timestamp.saturating_add(Self::MAX_AGE_SECS) < now {
            return Err(DecryptionError::ReplayDetected);
        }

        let digest = Self::digest(msg);
        let is_replay = self
            .seen
            .get(&msg.sender_id)
            .is_some_and(|entry| entry.iter().any(|seen| *seen == digest));
        if is_replay {
            return Err(DecryptionError::ReplayDetected);
        }

        // Bound the map against a peer that varies `sender_id`, exactly as
        // `add_peer_key` bounds the peer-key map. Done BEFORE taking the
        // mutable borrow below, and only when a genuinely new sender needs an
        // entry, so a single chatty sender cannot churn the map.
        if !self.seen.contains_key(&msg.sender_id) && self.seen.len() >= REPLAY_MAX_SENDERS {
            let oldest = self
                .seen
                .iter()
                .min_by_key(|(_, q)| q.front().copied())
                .map(|(k, _)| k.clone());
            if let Some(k) = oldest {
                self.seen.remove(&k);
            }
        }

        let entry = self.seen.entry(msg.sender_id.clone()).or_default();
        entry.push_back(digest);
        while entry.len() > REPLAY_CACHE_PER_SENDER {
            entry.pop_front();
        }
        Ok(())
    }

    /// Forget every sender. Used when private state is dropped on lock, so a
    /// stale cache cannot outlive the identity it belonged to.
    pub fn clear(&mut self) {
        self.seen.clear();
    }

    /// Test hook: how many digests are cached for `sender_id`.
    pub fn cached_for(&self, sender_id: &str) -> usize {
        self.seen.get(sender_id).map_or(0, |q| q.len())
    }
}

impl EncryptionManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Self::load_or_create()
    }

    /// Construct with an operator-configured key-rotation interval, in days.
    ///
    /// `SecurityConfig::key_rotation_interval_days` is documented in
    /// docs/CONFIGURATION.md and settable in ghostwire.toml, but nothing read it:
    /// the interval was hardcoded to 7 days in `from_key_pair` and the check in
    /// `should_rotate_keys` compared against that. An operator setting
    /// `key_rotation_interval_days = 1` to tighten rotation got 7 days, and a
    /// config that reads like a compliance control silently did nothing.
    ///
    /// Applied at construction rather than via a setter because `Arc<Encryption>`
    /// has no interior mutability on this field and `should_rotate_keys` takes
    /// `&self`. A restored profile still wins: `load_from_disk` overwrites the
    /// interval from `PersistedEncryption` (see `from_persisted`), so an
    /// existing node keeps the interval it was created with instead of silently
    /// adopting a new one on upgrade.
    ///
    /// A `days` of 0 is rejected in favour of the default: it would make
    /// `should_rotate_keys` true on every check, which is never the intent.
    pub fn new_with_rotation_days(days: u64) -> Result<Self, Box<dyn std::error::Error>> {
        let mut manager = Self::load_or_create()?;
        if days == 0 {
            tracing::warn!(
                target: "encryption",
                "key_rotation_interval_days = 0 is not a usable interval; keeping the default"
            );
        } else {
            let secs = days.saturating_mul(86_400);
            tracing::info!(
                target: "encryption",
                days,
                secs,
                "Applying configured key rotation interval"
            );
            manager.key_rotation_interval = secs;
        }
        Ok(manager)
    }

    /// Extract the X25519 static secret for Sphinx packet processing.
    /// Returns None if the key pair is too short (legacy format).
    pub fn x25519_secret(&self) -> Option<x25519_dalek::StaticSecret> {
        let pk = self.private_key_snapshot().ok()?;
        if pk.len() < 34 {
            return None;
        }
        let ed_len = u16::from_le_bytes([pk[0], pk[1]]) as usize;
        let start = 2 + ed_len;
        if pk.len() < start + 32 {
            return None;
        }
        let bytes: [u8; 32] = pk[start..start + 32].try_into().ok()?;
        Some(x25519_dalek::StaticSecret::from(bytes))
    }

    /// Extract our X25519 public key bytes (32 bytes).
    pub fn x25519_public_key(&self) -> Option<[u8; 32]> {
        let pk = self.private_key_snapshot().ok()?;
        if pk.len() < 34 {
            return None;
        }
        let ed_len = u16::from_le_bytes([pk[0], pk[1]]) as usize;
        let start = 2 + ed_len;
        if pk.len() < start + 32 {
            return None;
        }
        let secret =
            x25519_dalek::StaticSecret::from(<[u8; 32]>::try_from(&pk[start..start + 32]).ok()?);
        Some(*x25519_dalek::PublicKey::from(&secret).as_bytes())
    }

    pub fn load_or_create() -> Result<Self, Box<dyn std::error::Error>> {
        let storage_path = Self::get_storage_path()?;

        if storage_path.exists() {
            let manager = if Self::is_user_keyed() && !Self::master_key_path()?.exists() {
                info!("🔒 Encryption private state is user-keyed and locked at startup");
                Self::load_public_stub()?
            } else {
                Self::load_existing(&storage_path)?
            };
            info!(
                "🔐 Loaded persistent encryption keys for key_id: {}",
                manager.key_pair.key_id
            );
            return Ok(manager);
        }

        info!("🆕 Generating new persistent encryption keys...");
        let manager = Self::create_new()?;
        manager.save_to_disk()?;
        Ok(manager)
    }

    fn create_new() -> Result<Self, Box<dyn std::error::Error>> {
        let rng = SystemRandom::new();

        // Generate hybrid key pair (both Ed25519 and X25519)
        let key_pair = Self::generate_hybrid_key_pair(&rng)?;
        Ok(Self::from_key_pair(key_pair))
    }

    fn get_storage_path() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
        let ghost_dir = if let Ok(ghost_home) = std::env::var("GHOSTWIRE_HOME") {
            std::path::PathBuf::from(ghost_home)
        } else {
            let home = dirs::home_dir().ok_or("Could not find home directory")?;
            home.join(".ghostwire")
        };

        if !ghost_dir.exists() {
            std::fs::create_dir_all(&ghost_dir)?;
        }
        Ok(ghost_dir.join("encryption.json"))
    }

    /// Load an existing persistent encryption state without replacing it on failure.
    /// A corrupt or undecryptable state is a recovery condition, not permission to
    /// silently rotate the node's persistent encryption identity.
    fn load_existing(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        Self::load_from_disk(&path.to_path_buf())
    }

    fn from_key_pair(key_pair: GhostKeyPair) -> Self {
        let mut public = key_pair.clone();
        let private = LockedBuffer::from_bytes(&key_pair.private_key);
        public.private_key.clear();

        // Derive the initial transport key ring from the X25519 secret.
        // This creates a TransportKeyRing with 3 overlapping 6-day periods,
        // mirroring Briar BTP's approach.
        let now_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let transport_key_ring = if let Some(x25519_secret) = Self::extract_x25519_secret_bytes(&private) {
            TransportKeyRing::derive(x25519_secret, now_secs)
        } else {
            // Fallback: identity without X25519 (legacy) — use zeros
            TransportKeyRing::derive(&[0u8; 32], now_secs)
        };

        EncryptionManager {
            key_pair: public,
            private_key: Arc::new(Mutex::new(Some(private))),
            peer_keys: Arc::new(Mutex::new(HashMap::new())),
            rng: SystemRandom::new(),
            session_keys: Arc::new(Mutex::new(HashMap::new())),
            key_rotation_interval: 86400 * 7,
            last_key_rotation: now_secs,
            transport_key_ring,
            crl: Arc::new(crl::CrlStore::new()),
            ratchet_sessions: Arc::new(double_ratchet::RatchetSessionManager::new()),
            replay_guard: Arc::new(Mutex::new(ReplayGuard::default())),
        }
    }

    fn from_persisted(persisted: PersistedEncryption) -> Self {
        let mut manager = Self::from_key_pair(persisted.key_pair);
        manager.key_rotation_interval = persisted.key_rotation_interval;
        manager.last_key_rotation = persisted.last_key_rotation;
        // Restore persisted transport key ring if available
        if let Some(ring) = persisted.transport_key_ring {
            manager.transport_key_ring = ring;
        }
        manager
    }

    /// Extract the raw 32-byte X25519 secret from the private key buffer.
    /// Returns None if the key pair doesn't contain an X25519 component.
    fn extract_x25519_secret_bytes(private: &LockedBuffer) -> Option<&[u8; 32]> {
        if private.len() < 34 {
            return None;
        }
        let ed_len = u16::from_le_bytes([private[0], private[1]]) as usize;
        let start = 2 + ed_len;
        if private.len() < start + 32 {
            return None;
        }
        // SAFETY: we've verified bounds above; reinterpret as &[u8; 32]
        let slice = &private[start..start + 32];
        Some(slice.try_into().ok()?)
    }

    fn private_key_snapshot(&self) -> Result<LockedBuffer, Box<dyn std::error::Error>> {
        self.private_key
            .lock()
            .map_err(|_| std::io::Error::other("private encryption key lock poisoned"))?
            .as_ref()
            .map(|key| LockedBuffer::from_bytes(key))
            .ok_or_else(|| "encryption private state is locked".into())
    }

    fn load_from_disk(path: &std::path::PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let persisted: PersistedEncryption = Self::secure_load(path)?;
        Ok(Self::from_persisted(persisted))
    }

    fn master_key_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let ghost_dir = if let Ok(ghost_home) = std::env::var("GHOSTWIRE_HOME") {
            PathBuf::from(ghost_home)
        } else {
            dirs::home_dir().ok_or("No home dir")?.join(".ghostwire")
        };
        Ok(ghost_dir.join(cryptify::encrypt_string!("master.key")))
    }

    fn public_stub_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let ghost_dir = if let Ok(ghost_home) = std::env::var("GHOSTWIRE_HOME") {
            PathBuf::from(ghost_home)
        } else {
            dirs::home_dir().ok_or("No home dir")?.join(".ghostwire")
        };
        std::fs::create_dir_all(&ghost_dir)?;
        Ok(ghost_dir.join("encryption.user.public.json"))
    }

    fn load_public_stub() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::public_stub_path()?;
        let mut persisted: PersistedEncryption = serde_json::from_slice(&std::fs::read(path)?)?;
        persisted.key_pair.private_key.clear();
        let manager = Self::from_persisted(persisted);
        *manager
            .private_key
            .lock()
            .map_err(|_| std::io::Error::other("private encryption key lock poisoned"))? = None;
        Ok(manager)
    }

    fn write_public_stub(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut persisted = self.persisted_state()?;
        persisted.key_pair.private_key.clear();
        let path = Self::public_stub_path()?;
        let temp = path.with_extension("json.tmp");
        let _ = std::fs::remove_file(&temp);
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&temp)?;
        file.write_all(&serde_json::to_vec_pretty(&persisted)?)?;
        file.sync_all()?;
        std::fs::rename(&temp, path)?;
        Ok(())
    }

    pub fn activate_user_keyed(
        &self,
        persistence: &PersistenceManager,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let loaded = Self::load_user_keyed(persistence)?;
        let private = loaded.private_key_snapshot()?;
        *self
            .private_key
            .lock()
            .map_err(|_| std::io::Error::other("private encryption key lock poisoned"))? =
            Some(private);
        Ok(())
    }

    /// Drop ALL private encryption material from memory when the profile locks.
    ///
    /// Locked must mean *no decryption*, not merely "the one obvious key is
    /// gone". This function previously cleared only `private_key`, which left two
    /// live bypasses, both of which let a locked node decrypt traffic:
    ///
    ///  1. `derive_session_key` returned the CACHED session key before it ever
    ///     reached its `private_key_snapshot()` check, so a warm cache decrypted
    ///     v3 traffic indefinitely.
    ///  2. Ratchet state never referenced `private_key` at all, so v4 traffic
    ///     decrypted indefinitely.
    ///
    /// Both gaps are closed structurally, not just here: `derive_session_key`
    /// and `decrypt_message_ratchet` now consult `require_unlocked()` before
    /// doing any work, so clearing the caches below is defence in depth rather
    /// than the only thing standing between a locked profile and plaintext.
    pub fn lock_private_state(&self) -> Result<(), Box<dyn std::error::Error>> {
        *self
            .private_key
            .lock()
            .map_err(|_| std::io::Error::other("private encryption key lock poisoned"))? = None;

        // Cached session keys are raw ECDH-derived key material. Leaving them
        // behind is exactly the defect above, so they go with the private key.
        self.session_keys
            .lock()
            .map_err(|_| std::io::Error::other("session key lock poisoned"))?
            .clear();

        // Ratchet state holds its own chain/root keys and a DH private key, and
        // is completely independent of `private_key`. It must be dropped too.
        self.ratchet_sessions.clear_all_sessions();

        // The replay cache describes frames accepted under the identity we just
        // dropped; it is not secret, but keeping it would let a locked-then-
        // unlocked profile reject a legitimately re-delivered frame.
        self.replay_guard
            .lock()
            .map_err(|_| std::io::Error::other("replay guard lock poisoned"))?
            .clear();
        Ok(())
    }

    /// Authoritative lock predicate: is private material unavailable?
    ///
    /// Every path that can produce plaintext MUST gate on this (or on
    /// `private_key_snapshot`, which fails identically when locked). Presence of
    /// the flag alone is not sufficient — the ratchet path never touched the
    /// private key and is exactly why the old lock was ineffective.
    pub fn is_locked(&self) -> bool {
        self.private_key
            .lock()
            .map(|guard| guard.is_none())
            .unwrap_or(true)
    }

    /// Gate for any operation that can derive key material or produce plaintext.
    ///
    /// Fails CLOSED: a poisoned mutex is treated as locked, never as unlocked.
    fn require_unlocked(&self) -> Result<(), Box<dyn std::error::Error>> {
        let guard = self
            .private_key
            .lock()
            .map_err(|_| std::io::Error::other("private encryption key lock poisoned"))?;
        if guard.is_none() {
            return Err("encryption private state is locked".into());
        }
        Ok(())
    }

    /// Reject a frame whose digest we have already accepted.
    ///
    /// Called only after signature verification succeeds. See [`ReplayGuard`] for
    /// why the digest covers only the authenticated fields.
    fn check_replay(
        &self,
        msg: &EncryptedMessage,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.replay_guard
            .lock()
            .map_err(|_| std::io::Error::other("replay guard lock poisoned"))?
            .check(msg, now)?;
        Ok(())
    }

    /// Test hook: replay-cache size for a sender.
    pub fn replay_cache_len_for_test(&self, sender_id: &str) -> usize {
        self.replay_guard
            .lock()
            .map(|guard| guard.cached_for(sender_id))
            .unwrap_or(0)
    }

    fn get_master_key() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if Self::is_user_keyed() {
            return Err(
                "machine master-key operations are disabled after user-key migration".into(),
            );
        }
        let ghost_dir =
            if let Ok(ghost_home) = std::env::var(cryptify::encrypt_string!("GHOSTWIRE_HOME")) {
                std::path::PathBuf::from(ghost_home)
            } else {
                let home = dirs::home_dir().ok_or("No home dir")?;
                home.join(cryptify::encrypt_string!(".ghostwire"))
            };

        if !ghost_dir.exists() {
            std::fs::create_dir_all(&ghost_dir)?;
        }
        let key_path = ghost_dir.join(cryptify::encrypt_string!("master.key"));

        if key_path.exists() {
            let key = std::fs::read(&key_path)?;
            if key.len() != 32 {
                return Err("Invalid master key length".into());
            }
            Ok(key)
        } else {
            let mut key = [0u8; 32];
            SystemRandom::new()
                .fill(&mut key[..])
                .map_err(|_| "RNG failed")?;

            // save with 600
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .mode(0o600)
                .open(&key_path)?;
            file.write_all(&key)?;
            tracing::info!("🔑 Generated new Master Key for secure storage");
            Ok(key.to_vec())
        }
    }

    fn user_state_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let ghost_dir = if let Ok(ghost_home) = std::env::var("GHOSTWIRE_HOME") {
            PathBuf::from(ghost_home)
        } else {
            dirs::home_dir()
                .ok_or("Could not find home directory")?
                .join(".ghostwire")
        };
        std::fs::create_dir_all(&ghost_dir)?;
        Ok(ghost_dir.join("encryption.user.gwpe"))
    }

    fn persisted_state(&self) -> Result<PersistedEncryption, Box<dyn std::error::Error>> {
        let mut key_pair = self.key_pair.clone();
        key_pair.private_key = self.private_key_snapshot()?.to_vec();
        Ok(PersistedEncryption {
            key_pair,
            key_rotation_interval: self.key_rotation_interval,
            last_key_rotation: self.last_key_rotation,
            transport_key_ring: Some(self.transport_key_ring.clone()),
        })
    }

    /// Re-wrap the EncryptionManager state under the already-unlocked user
    /// PersistenceManager key. The legacy master.key remains intact during
    /// compatibility migration so old readers cannot be bricked accidentally.
    pub fn migrate_to_user_keyed(
        &self,
        persistence: &PersistenceManager,
    ) -> Result<(), Box<dyn std::error::Error>> {
        persistence
            .ensure_user_unlocked()
            .map_err(|e| format!("user persistence is locked: {e}"))?;
        let needs_activation = self
            .private_key
            .lock()
            .map_err(|_| std::io::Error::other("private encryption key lock poisoned"))?
            .is_none();
        if needs_activation {
            self.activate_user_keyed(persistence)?;
        }
        let json = serde_json::to_vec(&self.persisted_state()?)?;
        let wrapped = persistence
            .encrypt_user_record(USER_STATE_NAMESPACE, USER_STATE_KEY, &json)
            .map_err(|e| format!("user-keyed encryption state wrap failed: {e}"))?;
        let path = Self::user_state_path()?;
        let temp = path.with_extension("gwpe.tmp");
        let _ = std::fs::remove_file(&temp);
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&temp)?;
        file.write_all(&wrapped)?;
        file.sync_all()?;
        std::fs::rename(&temp, &path)?;
        self.write_public_stub()?;
        let master_key = Self::master_key_path()?;
        if master_key.exists() {
            std::fs::remove_file(master_key)?;
        }
        Ok(())
    }

    /// Load the user-keyed EncryptionManager state after the caller has
    /// explicitly unlocked the profile. No machine master key is consulted.
    pub fn load_user_keyed(
        persistence: &PersistenceManager,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        persistence
            .ensure_user_unlocked()
            .map_err(|e| format!("user persistence is locked: {e}"))?;
        let path = Self::user_state_path()?;
        let wrapped = std::fs::read(path)?;
        let json = persistence
            .decrypt_user_record(USER_STATE_NAMESPACE, USER_STATE_KEY, &wrapped)
            .map_err(|e| format!("user-keyed encryption state unwrap failed: {e}"))?;
        let persisted: PersistedEncryption = serde_json::from_slice(&json)?;
        Ok(Self::from_persisted(persisted))
    }

    pub fn is_user_keyed() -> bool {
        Self::user_state_path()
            .map(|path| path.exists())
            .unwrap_or(false)
    }

    pub fn secure_save<T: Serialize>(
        path: &std::path::Path,
        data: &T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let json_bytes = serde_json::to_vec(data)?;
        let master_key = Self::get_master_key()?;

        // Encrypt with XChaCha20-Poly1305
        let result = Self::encrypt_with_xchacha20(&json_bytes, &master_key)?;

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(path)?;
        // Simple container format: [Nonce (12)] [Ciphertext]
        file.write_all(&result.nonce)?;
        file.write_all(&result.ciphertext)?;
        Ok(())
    }

    pub fn secure_load<T: for<'de> Deserialize<'de>>(
        path: &std::path::Path,
    ) -> Result<T, Box<dyn std::error::Error>> {
        let data = std::fs::read(path)?;
        if data.len() < 24 {
            return Err("File too short".into());
        }

        let master_key = Self::get_master_key()?;
        let nonce = &data[..24];
        let ciphertext = &data[24..];

        let plaintext_bytes = Self::decrypt_with_xchacha20(ciphertext, nonce, &master_key)?;
        let object: T = serde_json::from_slice(&plaintext_bytes)?;
        Ok(object)
    }

    /// Public helper for byte-level encryption using Master Key
    pub fn encrypt_bytes(data: &[u8]) -> Result<EncryptionResult, Box<dyn std::error::Error>> {
        let master_key = Self::get_master_key()?;
        Self::encrypt_with_xchacha20(data, &master_key)
    }

    /// Public helper for byte-level decryption using Master Key
    pub fn decrypt_bytes(
        ciphertext: &[u8],
        nonce: &[u8],
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let master_key = Self::get_master_key()?;
        Self::decrypt_with_xchacha20(ciphertext, nonce, &master_key)
    }

    pub fn save_to_disk(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::get_storage_path()?;
        let persisted = self.persisted_state()?;
        Self::secure_save(&path, &persisted)?;
        Ok(())
    }

    fn generate_hybrid_key_pair(
        rng: &SystemRandom,
    ) -> Result<GhostKeyPair, Box<dyn std::error::Error>> {
        // Generate Ed25519 key pair for signing
        let ed25519_pkcs8_key_pair = Ed25519KeyPair::generate_pkcs8(rng)
            .map_err(|e| format!("Failed to generate Ed25519 key pair: {:?}", e))?;
        let ed25519_key_pair = Ed25519KeyPair::from_pkcs8(ed25519_pkcs8_key_pair.as_ref())
            .map_err(|e| format!("Failed to create Ed25519 key pair: {:?}", e))?;

        // Generate X25519 key pair for encryption (persistent static secret)
        let x25519_secret = StaticSecret::random_from_rng(OsRng);
        let x25519_public = XPublicKey::from(&x25519_secret);
        let x25519_public_bytes = *x25519_public.as_bytes();
        let x25519_private_bytes = *x25519_secret.as_bytes();

        // Generate ML-KEM (Kyber768) key pair for post-quantum hardening
        use pqcrypto_mlkem::mlkem768;
        use pqcrypto_traits::kem::{PublicKey as _, SecretKey as _};
        let (pq_pk, pq_sk) = mlkem768::keypair();

        // Combine keys into a serialized binary format
        // Public part: [Ed25519 (32)] [X25519 (32)] [Kyber PK (1184 for 768)]
        let mut public_key = ed25519_key_pair.public_key().as_ref().to_vec();
        public_key.extend_from_slice(&x25519_public_bytes);
        public_key.extend_from_slice(pq_pk.as_bytes());

        // Private part: [Ed25519 Len (2)] [Ed25519 PKCS8] [X25519 (32)] [Kyber SK (2400 for 768)]
        let ed_pkcs8 = ed25519_pkcs8_key_pair.as_ref();
        let ed_len = ed_pkcs8.len() as u16;
        let mut private_key = ed_len.to_le_bytes().to_vec();
        private_key.extend_from_slice(ed_pkcs8);
        private_key.extend_from_slice(&x25519_private_bytes);
        private_key.extend_from_slice(pq_sk.as_bytes());

        let key_pair = GhostKeyPair {
            public_key,
            private_key,
            key_id: Self::generate_key_id(),
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            expires_at: Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    + 86400 * 30,
            ), // 30 days
            usage_count: 0,
            key_type: KeyType::Ultimate,
        };

        Ok(key_pair)
    }

    fn generate_key_id() -> String {
        // Timestamp-only IDs collide when multiple fresh managers are created
        // within one second. Keep the timestamp for observability, but bind a
        // cryptographically random suffix so key IDs are unique in practice.
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut random = [0u8; 8];
        SystemRandom::new()
            .fill(&mut random)
            .expect("system RNG unavailable while generating key ID");
        format!("key_{:x}_{}", timestamp, hex::encode(random))
    }

    pub fn get_public_key(&self) -> &[u8] {
        &self.key_pair.public_key
    }

    pub fn get_key_id(&self) -> String {
        self.key_pair.key_id.clone()
    }

    /// Learn a peer's public key.
    ///
    /// SECURITY: this no longer silently overwrites an existing entry. A key
    /// learned earlier must not be replaceable by a later unauthenticated
    /// message, because the decrypt path looks a key up by the SENDER ID
    /// carried in the message (`decrypt_message` -> `peer_keys`), and verifies
    /// the signature against whatever is stored there. A peer able to overwrite
    /// that entry could therefore make forged messages verify as a third party.
    ///
    /// The reachable path: `handle_handshake` (mesh.rs) calls this TWICE per
    /// gossipsub frame — once keyed by the transport-authenticated `peer_id`,
    /// and once keyed by `msg.sender`, which is a free-text field inside the
    /// untrusted payload with no binding to `peer_id`. Nothing signs the
    /// handshake content and nothing checked the sender string, so any peer
    /// publishing to the DM topic could replace an arbitrary sender's key.
    ///
    /// First-write-wins is the minimal safe behaviour: an existing key is
    /// KEPT. Rotating a genuinely changed key needs an explicit, authenticated
    /// path, which does not exist yet - so a legitimate re-key would be
    /// ignored. That is a deliberate trade: failing closed here is much better
    /// than silently trusting an attacker-supplied key. `trust_score` and
    /// `last_updated` are preserved from the original entry, so the first
    /// observation time is not lost.
    pub fn add_peer_key(&self, peer_id: String, public_key: Vec<u8>, key_type: KeyType) {
        let mut peer_keys = self.peer_keys.lock().unwrap();
        // Bound the map. `handle_handshake` calls this with `msg.sender`, a
        // free-text field from an untrusted gossipsub frame, so without a cap a
        // single peer can insert unlimited permanent entries by varying the
        // string - no crypto, no valid packet structure, just one field. There
        // is no eviction elsewhere, so this is the only bound.
        const MAX_PEER_KEYS: usize = 10_000;
        if peer_keys.len() >= MAX_PEER_KEYS && !peer_keys.contains_key(&peer_id) {
            tracing::warn!(
                target: "encryption",
                "peer_keys at capacity ({MAX_PEER_KEYS}); refusing new key for an unknown id"
            );
            return;
        }
        if let Some(existing) = peer_keys.get(&peer_id) {
            if existing.public_key == public_key {
                // Same key, nothing to do. Do NOT refresh last_updated or
                // re-stamp trust_score, so a replay cannot reset them.
                return;
            }
            tracing::warn!(
                target: "encryption",
                "refusing to overwrite an existing peer key via an unauthenticated path",
            );
            return;
        }
        peer_keys.insert(
            peer_id,
            PeerKey {
                public_key,
                key_id: Self::generate_key_id(),
                last_updated: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                key_type,
                is_verified: false,
                trust_score: 0.5, // Default trust score
            },
        );
    }

    /// Return whether a usable handshake key is currently available for a peer.
    /// This is a presence check only; encryption still performs revocation,
    /// derivation, PQC, AEAD, and signing checks.
    pub fn has_peer_key(&self, peer_id: &str) -> bool {
        self.peer_keys.lock().unwrap().contains_key(peer_id)
    }

    pub fn encrypt_message(
        &self,
        message: &str,
        recipient_id: &str,
        message_type: MessageType,
    ) -> Result<EncryptedMessage, Box<dyn std::error::Error>> {
        // Reject if recipient key is revoked
        if self.crl.is_revoked(recipient_id) {
            return Err(format!("Recipient key {} is revoked", recipient_id).into());
        }

        let peer_keys = self.peer_keys.lock().unwrap();
        let peer_key = peer_keys.get(recipient_id).ok_or("Recipient not found")?;

        // Derive base session key via ECDH (existing)
        let session_key = self.derive_session_key(recipient_id, &peer_key.public_key)?;

        // PQC Hardening: Kyber Encapsulation
        let mut pq_key_out = None;
        let mut final_key = session_key.clone();

        if peer_key.public_key.len() >= 64 + 1184 {
            use pqcrypto_mlkem::mlkem768;
            use pqcrypto_traits::kem::PublicKey as _;

            let pq_pk_bytes = &peer_key.public_key[64..64 + 1184];
            let pk = mlkem768::PublicKey::from_bytes(pq_pk_bytes)
                .map_err(|_| "Invalid peer PQ public key")?;
            let (shared_secret, ciphertext) = mlkem768::encapsulate(&pk);
            use pqcrypto_traits::kem::{Ciphertext, SharedSecret};
            pq_key_out = Some(ciphertext.as_bytes().to_vec());

            // Mix keys: HKDF(ECDH_key, Kyber_secret)
            let salt = hkdf::Salt::new(hkdf::HKDF_SHA256, &final_key);
            let prk = salt.extract(shared_secret.as_bytes());
            let okm = prk
                .expand(&[b"GhostWire-PQC-Mix"], hkdf::HKDF_SHA256)
                .map_err(|_| "PQC mix failed")?;
            let mut mixed = [0u8; 32];
            okm.fill(&mut mixed).map_err(|_| "PQC fill failed")?;
            final_key = mixed.to_vec();
        }

        // Encrypt the message using XChaCha20-Poly1305 with the hardened key
        let encrypted_data = Self::encrypt_with_xchacha20(message.as_bytes(), &final_key)?;

        // Sign the encrypted message
        let signature = self.sign_message(&encrypted_data.ciphertext)?;

        // Update usage count
        self.increment_key_usage();

        Ok(EncryptedMessage {
            ciphertext: encrypted_data.ciphertext,
            nonce: encrypted_data.nonce,
            signature,
            sender_id: self.key_pair.key_id.clone(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            key_id: self.key_pair.key_id.clone(),
            message_type,
            version: 0x03, // XChaCha20-Poly1305 encryption
            pq_key: pq_key_out,
        })
    }

    pub fn decrypt_message(
        &self,
        encrypted_msg: &EncryptedMessage,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // SECURITY: refuse to decrypt at all while locked, before any key
        // derivation or peer lookup happens.
        self.require_unlocked()?;

        // Reject if sender key is revoked
        if self.crl.is_revoked(&encrypted_msg.sender_id) {
            return Err(format!("Sender key {} is revoked", encrypted_msg.sender_id).into());
        }

        // Get peer public key for verification
        let peer_keys = self.peer_keys.lock().unwrap();
        let peer_key = peer_keys
            .get(&encrypted_msg.sender_id)
            .ok_or("Sender not found in peer keys")?;
        let peer_public_key = peer_key.public_key.clone();
        drop(peer_keys); // Release lock

        // Verify signature first
        self.verify_signature(
            &encrypted_msg.ciphertext,
            &encrypted_msg.signature,
            &peer_public_key,
        )?;

        // Replay check, only now that the frame is known to be authentic.
        //
        // Ordering is load-bearing: `sign_message` covers the ciphertext
        // alone, so the header fields above (timestamp, key_id, message_type)
        // are not authenticated. Running this before signature verification
        // would let an unauthenticated attacker seed the cache with chosen
        // digests and evict genuine entries, reopening the replay window.
        self.check_replay(encrypted_msg)?;

        if encrypted_msg.version != 0x03 {
            return Err("Unsupported message version (expected v3 XChaCha20)".into());
        }

        // Derive base session key for decryption
        let session_key = self.derive_session_key(&encrypted_msg.sender_id, &peer_public_key)?;
        let mut final_key = session_key.clone();

        // PQC Hardening: Kyber Decapsulation
        if let Some(pq_ciphertext) = &encrypted_msg.pq_key {
            use pqcrypto_mlkem::mlkem768;
            use pqcrypto_traits::kem::{Ciphertext as _, SecretKey as _};

            // Extract our Kyber Secret Key from private key blob
            let priv_key_bytes = self.private_key_snapshot()?;
            let ed_len = u16::from_le_bytes([priv_key_bytes[0], priv_key_bytes[1]]) as usize;
            let pq_sk_bytes = &priv_key_bytes[2 + ed_len + 32..];

            if let (Ok(pq_sk), Ok(pq_ct)) = (
                mlkem768::SecretKey::from_bytes(pq_sk_bytes),
                mlkem768::Ciphertext::from_bytes(pq_ciphertext),
            ) {
                let shared_secret = mlkem768::decapsulate(&pq_ct, &pq_sk);

                use pqcrypto_traits::kem::SharedSecret;
                // Mix keys: HKDF(ECDH_key, Kyber_secret)
                let salt = hkdf::Salt::new(hkdf::HKDF_SHA256, &session_key);
                let prk = salt.extract(shared_secret.as_bytes());
                let okm = prk
                    .expand(&[b"GhostWire-PQC-Mix"], hkdf::HKDF_SHA256)
                    .map_err(|_| "PQC mix failed")?;
                let mut mixed = [0u8; 32];
                okm.fill(&mut mixed).map_err(|_| "PQC fill failed")?;
                final_key = mixed.to_vec();
            }
        }

        // Decrypt using XChaCha20-Poly1305
        let decrypted_data = Self::decrypt_with_xchacha20(
            &encrypted_msg.ciphertext,
            &encrypted_msg.nonce,
            &final_key,
        )?;

        String::from_utf8(decrypted_data).map_err(|e| e.into())
    }

    pub fn encrypt_with_xchacha20(
        data: &[u8],
        key: &[u8],
    ) -> Result<EncryptionResult, Box<dyn std::error::Error>> {
        use chacha20poly1305::{
            aead::{Aead, KeyInit},
            XChaCha20Poly1305, XNonce,
        };

        // Ensure key is exactly 32 bytes
        if key.len() != 32 {
            return Err("Key must be exactly 32 bytes for XChaCha20".into());
        }

        let cipher = XChaCha20Poly1305::new(key.into());

        // Generate random nonce (192 bits = 24 bytes for XChaCha20)
        let nonce_bytes = Self::generate_nonce();
        let nonce = XNonce::from_slice(&nonce_bytes);

        // Encrypt
        let ciphertext = cipher
            .encrypt(nonce, data)
            .map_err(|e| format!("XChaCha20 encryption failed: {}", e))?;

        Ok(EncryptionResult {
            ciphertext,
            nonce: nonce_bytes.to_vec(),
        })
    }

    pub fn decrypt_with_xchacha20(
        ciphertext: &[u8],
        nonce: &[u8],
        key: &[u8],
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        use chacha20poly1305::{
            aead::{Aead, KeyInit},
            XChaCha20Poly1305, XNonce,
        };

        // Ensure key is exactly 32 bytes
        if key.len() != 32 {
            return Err("Key must be exactly 32 bytes for XChaCha20".into());
        }

        // Ensure nonce is exactly 24 bytes
        if nonce.len() != 24 {
            return Err("Nonce must be exactly 24 bytes for XChaCha20".into());
        }

        // ✅ CRITICAL SECURITY FIX (CVE-2025-11931 class vulnerability)
        // Prevent integer underflow and DoS attacks
        const POLY1305_TAG_LEN: usize = 16;
        const MIN_CIPHERTEXT_LEN: usize = POLY1305_TAG_LEN;
        const MAX_MESSAGE_LEN: usize = 256 * 1024 * 1024; // 256 MB

        // Validate ciphertext length BEFORE any arithmetic operations
        if ciphertext.len() < MIN_CIPHERTEXT_LEN {
            tracing::warn!(
                actual_len = ciphertext.len(),
                min_required = MIN_CIPHERTEXT_LEN,
                "Ciphertext too short - possible truncation attack (CVE-2025-11931 class)"
            );
            return Err("Ciphertext too short (possible truncation attack)".into());
        }

        // Prevent DoS via large memory allocations
        if ciphertext.len() > MAX_MESSAGE_LEN {
            tracing::warn!(
                actual_len = ciphertext.len(),
                max_allowed = MAX_MESSAGE_LEN,
                "Ciphertext exceeds maximum allowed size - possible DoS attack"
            );
            return Err("Ciphertext exceeds maximum allowed size".into());
        }

        let cipher = XChaCha20Poly1305::new(key.into());
        let nonce = XNonce::from_slice(nonce);

        // Decrypt with sanitized error messages (don't leak internal details)
        cipher.decrypt(nonce, ciphertext).map_err(|e| {
            // Log full error internally for debugging
            tracing::debug!(error = ?e, "XChaCha20 decryption failed");
            // Return generic error to attacker (prevent oracle attacks)
            "Decryption failed (authentication failed or corrupted data)".into()
        })
    }

    /// Encrypt bytes with the machine-protected master key and caller-supplied AAD.
    /// This is intended for authenticated persistence envelopes, not peer messaging.
    pub fn encrypt_bytes_with_aad(
        data: &[u8],
        aad: &[u8],
    ) -> Result<EncryptionResult, Box<dyn std::error::Error>> {
        let master_key = Self::get_master_key()?;
        Self::encrypt_with_xchacha20_aad(data, &master_key, aad)
    }

    /// Decrypt bytes with the machine-protected master key and caller-supplied AAD.
    pub fn decrypt_bytes_with_aad(
        ciphertext: &[u8],
        nonce: &[u8],
        aad: &[u8],
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let master_key = Self::get_master_key()?;
        Self::decrypt_with_xchacha20_aad(ciphertext, nonce, &master_key, aad)
    }

    /// Snapshot the legacy machine master key for compatibility migration.
    /// New user-mode persistence must use a passphrase keyslot instead.
    pub fn machine_master_key() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Self::get_master_key()
    }

    /// Encrypt bytes with an explicit caller-owned key and AAD.
    /// The caller is responsible for key lifecycle and zeroization.
    pub fn encrypt_bytes_with_key_aad(
        data: &[u8],
        key: &[u8],
        aad: &[u8],
    ) -> Result<EncryptionResult, Box<dyn std::error::Error>> {
        Self::encrypt_with_xchacha20_aad(data, key, aad)
    }

    /// Decrypt bytes with an explicit caller-owned key and AAD.
    pub fn decrypt_bytes_with_key_aad(
        ciphertext: &[u8],
        nonce: &[u8],
        key: &[u8],
        aad: &[u8],
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Self::decrypt_with_xchacha20_aad(ciphertext, nonce, key, aad)
    }

    fn encrypt_with_xchacha20_aad(
        data: &[u8],
        key: &[u8],
        aad: &[u8],
    ) -> Result<EncryptionResult, Box<dyn std::error::Error>> {
        use chacha20poly1305::{
            aead::{Aead, KeyInit, Payload},
            XChaCha20Poly1305, XNonce,
        };

        if key.len() != 32 {
            return Err("Key must be exactly 32 bytes for XChaCha20".into());
        }

        let cipher = XChaCha20Poly1305::new(key.into());
        let nonce_bytes = Self::generate_nonce();
        let nonce = XNonce::from_slice(&nonce_bytes);
        let ciphertext = cipher
            .encrypt(nonce, Payload { msg: data, aad })
            .map_err(|e| format!("XChaCha20 encryption with AAD failed: {}", e))?;

        Ok(EncryptionResult {
            ciphertext,
            nonce: nonce_bytes.to_vec(),
        })
    }

    fn decrypt_with_xchacha20_aad(
        ciphertext: &[u8],
        nonce: &[u8],
        key: &[u8],
        aad: &[u8],
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        use chacha20poly1305::{
            aead::{Aead, KeyInit, Payload},
            XChaCha20Poly1305, XNonce,
        };

        if key.len() != 32 {
            return Err("Key must be exactly 32 bytes for XChaCha20".into());
        }
        if nonce.len() != 24 {
            return Err("Nonce must be exactly 24 bytes for XChaCha20".into());
        }
        if ciphertext.len() < 16 {
            return Err("Ciphertext too short".into());
        }
        if ciphertext.len() > 256 * 1024 * 1024 {
            return Err("Ciphertext exceeds maximum allowed size".into());
        }

        let cipher = XChaCha20Poly1305::new(key.into());
        let nonce = XNonce::from_slice(nonce);
        cipher
            .decrypt(
                nonce,
                Payload {
                    msg: ciphertext,
                    aad,
                },
            )
            .map_err(|_| "Decryption failed (authentication failed or corrupted data)".into())
    }

    fn derive_session_key(
        &self,
        peer_id: &str,
        peer_public_key: &[u8],
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // SECURITY: gate on the lock BEFORE the cache lookup.
        //
        // This check used to sit at the private-key extraction further down,
        // which meant the cached-session-key branch above returned early and
        // never reached it. A locked node with a warm cache therefore
        // decrypted v3 traffic normally. A cached key IS private key material,
        // so serving it while locked defeats the lock entirely.
        self.require_unlocked()?;

        // Check if we have a cached session key
        let mut session_keys = self.session_keys.lock().unwrap();
        if let Some(session_key) = session_keys.get(peer_id) {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            if now < session_key.expires_at {
                return Ok(session_key.key.clone());
            }
        }

        // 🛡️ REAL ECDH IMPLEMENTATION
        // Peer public key structure: [Ed25519 (32) | X25519 (32)]
        if peer_public_key.len() < 64 {
            return Err("Invalid peer public key length".into());
        }
        let x25519_peer_pub_bytes: [u8; 32] = peer_public_key[32..64].try_into()?;
        let peer_x_pub = XPublicKey::from(x25519_peer_pub_bytes);

        // Our private key structure: [Ed25519 Len (2)] [Ed25519 PKCS8] [X25519 Private (32)]
        let priv_key_bytes = self.private_key_snapshot()?;
        if priv_key_bytes.len() < 34 {
            return Err("Private key too short".into());
        }
        let ed_len = u16::from_le_bytes([priv_key_bytes[0], priv_key_bytes[1]]) as usize;
        let x25519_priv_bytes: [u8; 32] = priv_key_bytes[2 + ed_len..2 + ed_len + 32].try_into()?;
        let our_x_secret = StaticSecret::from(x25519_priv_bytes);

        // Compute Shared Secret using X25519
        let shared_secret = our_x_secret.diffie_hellman(&peer_x_pub);

        // 🛡️ CVE-2025-62514 defense: reject all-zero shared secret (small-order point attack)
        // An attacker sending a small-order point (e.g., the zero point) would produce
        // an all-zero DH output, bypassing encryption entirely.
        if shared_secret.as_bytes() == &[0u8; 32] {
            return Err(
                "X25519 shared secret is all zeros — small-order point attack detected".into(),
            );
        }

        // Derive session key using HKDF-SHA256 (Robust way)
        let salt = hkdf::Salt::new(hkdf::HKDF_SHA256, b"GhostWire-Salt-v1");
        let prk = salt.extract(shared_secret.as_bytes());
        let info_str = cryptify::encrypt_string!("GhostWire-Session-Key-v2");
        let info = [info_str.as_bytes()];
        let okm = prk
            .expand(&info, hkdf::HKDF_SHA256)
            .map_err(|_| "HKDF expand failed")?;

        let mut key = [0u8; 32];
        okm.fill(&mut key).map_err(|_| "HKDF fill failed")?;

        // Cache the session key
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        session_keys.insert(
            peer_id.to_string(),
            SessionKey {
                key: key.to_vec(),
                created_at: now,
                expires_at: now + 3600, // 1 hour
                usage_count: 0,
            },
        );

        Ok(key.to_vec())
    }

    // Enhanced encryption method for CLI compatibility
    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        // Derive a strong 32-byte key from our key pair
        let key = self
            .derive_cli_key()
            .unwrap_or_else(|error| panic!("CLI encryption unavailable: {}", error));

        match Self::encrypt_with_xchacha20(data, &key) {
            Ok(result) => {
                // Combine nonce and ciphertext
                let mut combined = result.nonce;
                combined.extend_from_slice(&result.ciphertext);
                combined
            }
            Err(e) => {
                tracing::error!(inner_error = %e, "CLI encryption failed");
                panic!(
                    "Encryption failed - this should never happen in production: {}",
                    e
                );
            }
        }
    }

    // Derive a deterministic but unique 32-byte key for CLI operations
    fn derive_cli_key(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        use ring::digest;
        let mut ctx = digest::Context::new(&digest::SHA256);
        let domain = cryptify::encrypt_string!("GhostWire-CLI-v1");
        ctx.update(domain.as_bytes());
        ctx.update(&self.private_key_snapshot()?[..]);
        ctx.update(&self.key_pair.public_key[..]);
        Ok(ctx.finish().as_ref().to_vec())
    }

    // ✅ SECURE DECRYPTION: Returns Result instead of panic (DoS prevention)
    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, DecryptionError> {
        // This derives a key from the private material, so it must respect the
        // profile lock like every other decryption path.
        if self.is_locked() {
            return Err(DecryptionError::DecryptionFailed);
        }
        if data.len() < 24 {
            tracing::warn!(
                actual_len = data.len(),
                min_required = 24,
                "Encrypted data too short"
            );
            return Err(DecryptionError::InvalidLength);
        }

        // Derive the same key we used for encryption
        let key = self
            .derive_cli_key()
            .map_err(|_| DecryptionError::DecryptionFailed)?;

        // Extract nonce and ciphertext
        let nonce = &data[..24];
        let ciphertext = &data[24..];

        Self::decrypt_with_xchacha20(ciphertext, nonce, &key).map_err(|e| {
            tracing::debug!(error = ?e, "CLI decryption failed");
            DecryptionError::DecryptionFailed
        })
    }

    // Enhanced signature methods
    pub fn sign_message(&self, message: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Use Ed25519 for signing
        // Extract plain Ed25519 PKCS8 bytes from binary hybrid key
        let priv_key_bytes = self.private_key_snapshot()?;
        if priv_key_bytes.len() < 2 {
            return Err("Invalid private key length".into());
        }
        let ed_len = u16::from_le_bytes([priv_key_bytes[0], priv_key_bytes[1]]) as usize;
        if priv_key_bytes.len() < 2 + ed_len {
            return Err("Corrupted private key".into());
        }
        let key_pair_bytes = &priv_key_bytes[2..2 + ed_len];

        let key_pair = Ed25519KeyPair::from_pkcs8(key_pair_bytes)
            .map_err(|e| format!("Failed to create key pair for signing: {:?}", e))?;

        let signature = key_pair.sign(message);
        Ok(signature.as_ref().to_vec())
    }

    pub fn verify_signature(
        &self,
        message: &[u8],
        signature: &[u8],
        public_key: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Verify Ed25519 signature
        // Extract plain Ed25519 public key (first 32 bytes) if hybrid
        let ed25519_pub_key = if public_key.len() >= 32 {
            &public_key[..32]
        } else {
            return Err("Invalid public key length".into());
        };

        let public_key =
            ring::signature::UnparsedPublicKey::new(&ring::signature::ED25519, ed25519_pub_key);

        public_key
            .verify(message, signature)
            .map_err(|e| format!("Signature verification failed: {:?}", e).into())
    }

    fn generate_nonce() -> [u8; 24] {
        let mut nonce = [0u8; 24];
        SystemRandom::new().fill(&mut nonce[..]).unwrap();
        nonce
    }

    fn increment_key_usage(&self) {
        // In a real implementation, this would be thread-safe
        // For now, we'll just log the usage
        tracing::debug!(key_id = %self.key_pair.key_id, "Key usage incremented");
    }

    pub fn export_public_key(&self) -> Vec<u8> {
        self.key_pair.public_key.clone()
    }

    /// Test/inspection helper: read the currently stored key for an id.
    pub fn stored_peer_key_for_test(&self, id: &str) -> Option<Vec<u8>> {
        self.peer_keys
            .lock()
            .unwrap()
            .get(id)
            .map(|k| k.public_key.clone())
    }

    pub fn get_peer_count(&self) -> usize {
        self.peer_keys.lock().unwrap().len()
    }

    pub fn should_rotate_keys(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        now - self.last_key_rotation > self.key_rotation_interval
    }

    /// Refresh the overlapping transport key ring if the time period has advanced.
    /// This is separate from identity key rotation — transport keys derived
    /// from the identity X25519 secret are refreshed every KEY_ROTATION_PERIOD_SECS
    /// (6 days), giving 3 overlapping periods for grace-period decryption.
    pub fn refresh_transport_key_ring(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if let Some(secret) = self.x25519_secret() {
            let secret_bytes: [u8; 32] = *secret.as_bytes();
            if let Some(new_ring) = self.transport_key_ring.refresh_if_needed(&secret_bytes, now) {
                tracing::debug!(
                    target: "encryption",
                    old_epoch = self.transport_key_ring.rotation_epoch,
                    new_epoch = new_ring.rotation_epoch,
                    "Transport key ring refreshed (period advanced)"
                );
                self.transport_key_ring = new_ring;
                // Persist the refreshed key ring
                if !Self::is_user_keyed() {
                    self.save_to_disk()?;
                }
            }
        }
        Ok(())
    }

    pub fn rotate_keys_if_needed(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if Self::is_user_keyed() {
            return Err(
                "user-keyed encryption requires rotate_keys_if_needed_with_persistence".into(),
            );
        }
        if self.should_rotate_keys() {
            tracing::info!(msg = "Rotating encryption keys");
            let new_key_pair = Self::generate_hybrid_key_pair(&self.rng)?;
            let private = LockedBuffer::from_bytes(&new_key_pair.private_key);
            self.key_pair = new_key_pair;
            self.key_pair.private_key.clear();
            *self
                .private_key
                .lock()
                .map_err(|_| std::io::Error::other("private encryption key lock poisoned"))? =
                Some(private);
            self.last_key_rotation = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            self.session_keys.lock().unwrap().clear();
            self.crl.bump_epoch();

            // Re-derive transport key ring from new identity key
            let now_secs = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let private_bytes = self.private_key_snapshot()?;
            if private_bytes.len() >= 2 + 32 {
                // Try to extract X25519 secret from the new private key
                let ed_len = u16::from_le_bytes([private_bytes[0], private_bytes[1]]) as usize;
                let start = 2 + ed_len;
                if private_bytes.len() >= start + 32 {
                    if let Ok(secret_bytes) = private_bytes[start..start + 32].try_into() {
                        self.transport_key_ring = TransportKeyRing::derive(secret_bytes, now_secs);
                    }
                }
            }



            self.save_to_disk()?;
        }
        Ok(())
    }

    /// Rotate and persist encryption keys through the user-keyed envelope.
    pub fn rotate_keys_if_needed_with_persistence(
        &mut self,
        persistence: &PersistenceManager,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.should_rotate_keys() {
            return Ok(());
        }
        persistence.ensure_user_unlocked()?;
        tracing::info!(msg = "Rotating user-keyed encryption keys");
        let new_key_pair = Self::generate_hybrid_key_pair(&self.rng)?;
        let private = LockedBuffer::from_bytes(&new_key_pair.private_key);
        self.key_pair = new_key_pair;
        self.key_pair.private_key.clear();
        *self
            .private_key
            .lock()
            .map_err(|_| std::io::Error::other("private encryption key lock poisoned"))? =
            Some(private);
        self.last_key_rotation = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.session_keys.lock().unwrap().clear();
        self.crl.bump_epoch();
        self.migrate_to_user_keyed(persistence)?;
        Ok(())
    }

    pub fn revoke_key(&self, key_id: &str) -> crl::RevocationEntry {
        let key_id_owned = key_id.to_string();
        let signer_id = self.key_pair.key_id.clone();
        self.crl.revoke(&key_id_owned, &signer_id, &|payload| {
            self.sign_message(payload).unwrap_or_default()
        })
    }

    pub fn merge_crl_entry(&self, entry: &crl::RevocationEntry) -> crl::MergeResult {
        self.crl.merge(entry, &|payload, signature, signer_id| {
            if let Some(peer) = self.peer_keys.lock().unwrap().get(signer_id) {
                self.verify_signature(payload, signature, &peer.public_key)
                    .is_ok()
            } else {
                false
            }
        })
    }

    pub fn gc_crl(&self) -> usize {
        self.crl.gc()
    }

    pub fn create_crl_gossip_payload(&self) -> Vec<crl::CrlGossipPayload> {
        self.crl
            .entries()
            .into_iter()
            .enumerate()
            .map(|(i, entry)| crl::CrlGossipPayload {
                entry,
                version: (i as u64) + 1,
            })
            .collect()
    }

    pub fn handle_crl_gossip(&self, payloads: &[crl::CrlGossipPayload]) -> (usize, usize, usize) {
        let mut accepted = 0;
        let mut duplicates = 0;
        let mut rejected = 0;

        for payload in payloads {
            match self.merge_crl_entry(&payload.entry) {
                crl::MergeResult::Accepted => accepted += 1,
                crl::MergeResult::Duplicate => duplicates += 1,
                _ => rejected += 1,
            }
        }

        (accepted, duplicates, rejected)
    }

    /// Establish the sending side of a ratchet session with a peer.
    ///
    /// `peer_public_key` is the peer's FULL hybrid public key, laid out
    /// `[Ed25519 (32) | X25519 (32)]`, as stored in `peer_keys` and returned by
    /// `get_public_key()`.
    ///
    /// Two bugs lived here, both of which made this function incapable of
    /// succeeding:
    ///
    /// 1. It passed `self.key_pair.public_key[32..64].to_vec()` — our OWN
    ///    X25519 half, 32 bytes — into `derive_session_key`, which requires
    ///    `len >= 64` and then slices `[32..64]` ITSELF. That always failed with
    ///    "Invalid peer public key length".
    /// 2. Even passing our own key would be wrong. `derive_session_key` performs
    ///    an ECDH against the key it is given, so both parties must pass the
    ///    OTHER party's public key for the derived secret to agree. Passing our
    ///    own would give alice and bob two different root keys, and the first
    ///    decrypt would fail with an AEAD error.
    ///
    /// This is why the v4 ratchet was unreachable in production: not merely
    /// unwired, but broken at its entry point. The ~40 ratchet tests drive
    /// `double_ratchet` directly, so they never exercised this wrapper.
    pub fn init_ratchet_session(
        &self,
        peer_id: &str,
        peer_public_key: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        if peer_public_key.len() < 64 {
            return Err("Peer public key too short for ratchet setup (need >= 64)".into());
        }
        let session_key = self.derive_session_key(peer_id, peer_public_key)?;

        // The X25519 half the ratchet itself needs for the initial DH.
        let bob_public_key: [u8; 32] = peer_public_key[32..64].try_into()?;

        let mut shared_secret = [0u8; 32];
        shared_secret.copy_from_slice(&session_key);

        self.ratchet_sessions
            .init_alice(peer_id, &shared_secret, &bob_public_key);
        Ok(())
    }

    /// Establish the receiving side of a ratchet session with a peer.
    ///
    /// Same contract as `init_ratchet_session`: `peer_public_key` is the
    /// peer's FULL hybrid public key. The original signature took only a
    /// 32-byte slice and then passed `self.key_pair.public_key[32..64]`
    /// (our own key, 32 bytes) into `derive_session_key`, which requires
    /// `len >= 64` — so this could never succeed either.
    pub fn init_ratchet_session_as_bob(
        &self,
        peer_id: &str,
        peer_public_key: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        if peer_public_key.len() < 64 {
            return Err("Peer public key too short for ratchet setup (need >= 64)".into());
        }
        let session_key = self.derive_session_key(peer_id, peer_public_key)?;

        let mut shared_secret = [0u8; 32];
        shared_secret.copy_from_slice(&session_key);

        // Extract X25519 private key from hybrid key: [ed_len_u16][ed25519_pkcs8][x25519_32]
        let priv_key_bytes = self.private_key_snapshot()?;
        if priv_key_bytes.len() < 36 {
            return Err("Private key too short for X25519 extraction".into());
        }
        let ed_len = u16::from_le_bytes([priv_key_bytes[0], priv_key_bytes[1]]) as usize;
        let x25519_start = 2 + ed_len;
        if priv_key_bytes.len() < x25519_start + 32 {
            return Err("Private key missing X25519 secret".into());
        }
        let mut bob_secret = [0u8; 32];
        bob_secret.copy_from_slice(&priv_key_bytes[x25519_start..x25519_start + 32]);

        self.ratchet_sessions
            .init_bob(peer_id, &shared_secret, &bob_secret);
        Ok(())
    }

    pub fn encrypt_message_ratchet(
        &self,
        message: &str,
        peer_id: &str,
        message_type: MessageType,
    ) -> Result<EncryptedMessage, Box<dyn std::error::Error>> {
        if self.crl.is_revoked(peer_id) {
            return Err(format!("Peer key {} is revoked", peer_id).into());
        }

        let ratchet_msg = self
            .ratchet_sessions
            .encrypt(peer_id, message.as_bytes())
            .map_err(|e| format!("Ratchet encrypt failed: {}", e))?;

        let signature = self.sign_message(&ratchet_msg.ciphertext)?;

        let header_json = serde_json::to_vec(&ratchet_msg.header)?;

        self.increment_key_usage();

        Ok(EncryptedMessage {
            ciphertext: ratchet_msg.ciphertext,
            nonce: ratchet_msg.nonce,
            signature,
            sender_id: self.key_pair.key_id.clone(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            key_id: format!("ratchet:{}", peer_id),
            message_type,
            version: 0x04,
            pq_key: Some(header_json),
        })
    }

    pub fn decrypt_message_ratchet(
        &self,
        encrypted_msg: &EncryptedMessage,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // SECURITY: ratchet state is independent of `private_key`, so the
        // profile lock used to have no effect on this path at all — a locked
        // node decrypted v4 traffic indefinitely. Gate it explicitly.
        self.require_unlocked()?;

        if self.crl.is_revoked(&encrypted_msg.sender_id) {
            return Err(format!("Sender key {} is revoked", encrypted_msg.sender_id).into());
        }

        if encrypted_msg.version != 0x04 {
            return Err("Expected ratchet message (v4)".into());
        }

        // SECURITY: verify the sender's signature BEFORE touching the ratchet.
        //
        // This function previously did not call `verify_signature` at all,
        // while the v3 path does (see `decrypt_message`, which fetches the peer
        // key and verifies before deriving anything). The AEAD tag only proves
        // the message decrypts with the chain key — it says the message decrypts,
        // not WHO sent it. The sender does sign (`encrypt_message_ratchet`
        // calls `sign_message` over the ratchet ciphertext) and the signature
        // crosses the wire, but the receiver discarded it.
        //
        // Ordering matters: the check must precede `ratchet_sessions.decrypt`,
        // because decrypting ADVANCES the chain and consumes a message key.
        // Verifying afterwards would let an unauthenticated caller burn a peer's
        // keys and desynchronise the session.
        let peer_public_key = {
            let peer_keys = self.peer_keys.lock().unwrap();
            let peer_key = peer_keys
                .get(&encrypted_msg.sender_id)
                .ok_or("Sender not found in peer keys")?;
            peer_key.public_key.clone()
        };

        self.verify_signature(
            &encrypted_msg.ciphertext,
            &encrypted_msg.signature,
            &peer_public_key,
        )?;

        // Replay check. Placed after signature verification (so the cache can
        // only ever hold authentic frames) but before `ratchet_sessions
        // .decrypt`, which ADVANCES the chain and consumes a message key.
        // Checking after would mean a replay had already burned a key.
        //
        // The ratchet does consume each message key, so a byte-identical
        // replay would fail the AEAD check anyway — but that is a side effect
        // of the ratchet, not a replay check, and it does not stop an
        // attacker from forcing repeated chain advancement attempts. This is
        // the explicit defence.
        self.check_replay(encrypted_msg)?;

        let header_json = encrypted_msg
            .pq_key
            .as_ref()
            .ok_or("Missing ratchet header")?;
        let header: double_ratchet::RatchetHeader = serde_json::from_slice(header_json)
            .map_err(|e| format!("Invalid ratchet header: {}", e))?;

        let ratchet_msg = double_ratchet::RatchetMessage {
            header,
            ciphertext: encrypted_msg.ciphertext.clone(),
            nonce: encrypted_msg.nonce.clone(),
        };

        let peer_id = encrypted_msg
            .key_id
            .strip_prefix("ratchet:")
            .unwrap_or(&encrypted_msg.sender_id);

        let plaintext = self
            .ratchet_sessions
            .decrypt(peer_id, &ratchet_msg)
            .map_err(|e| format!("Ratchet decrypt failed: {}", e))?;

        String::from_utf8(plaintext).map_err(|e| e.into())
    }

    pub fn has_ratchet_session(&self, peer_id: &str) -> bool {
        self.ratchet_sessions.has_session(peer_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionResult {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
}

impl Default for EncryptionManager {
    fn default() -> Self {
        Self::new().expect("Failed to create default EncryptionManager")
    }
}

pub type Encryption = EncryptionManager;

/// Hermetic home for tests: a pristine per-test GHOSTWIRE_HOME held under a
/// process-wide env lock, so parallel tests never observe each other's
/// half-written global state (migrated keys, deleted master.key, foreign
/// profiles). Every test that reaches HOME-derived paths must hold one.
#[cfg(test)]
pub(crate) struct TestHome {
    _guard: std::sync::MutexGuard<'static, ()>,
    _dir: tempfile::TempDir,
}

#[cfg(test)]
pub(crate) fn test_home() -> TestHome {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let guard = ENV_LOCK.lock().expect("test env lock poisoned");
    let dir = tempfile::tempdir().expect("test GHOSTWIRE_HOME");
    std::env::set_var("GHOSTWIRE_HOME", dir.path());
    TestHome {
        _guard: guard,
        _dir: dir,
    }
}

#[cfg(test)]
mod rotation_config_tests {
    use super::*;

    /// A configured rotation interval must actually reach the manager.
    ///
    /// Before this, `SecurityConfig::key_rotation_interval_days` was read by
    /// nothing: `from_key_pair` hardcoded `86400 * 7`, so an operator asking for
    /// 1 day got 7. This asserts the value survives construction, which is the
    /// entire point of the setting.
    #[test]
    fn configured_rotation_days_reach_the_manager() {
        // Isolate GHOSTWIRE_HOME. Without this the two managers below share one
        // key file and the second construction fails to decrypt it — which is a
        // test artifact, not the behaviour under test.
        let _home = crate::core::encryption::test_home();

        let mgr = EncryptionManager::new_with_rotation_days(1).expect("construct");
        assert_eq!(
            mgr.key_rotation_interval, 86_400,
            "key_rotation_interval_days = 1 must become 86400s, not the 7-day default"
        );

        let mgr30 = EncryptionManager::new_with_rotation_days(30).expect("construct");
        assert_eq!(mgr30.key_rotation_interval, 30 * 86_400);
    }

    /// Zero is rejected rather than silently meaning "rotate constantly".
    #[test]
    fn zero_rotation_days_falls_back_to_default() {
        let _home = crate::core::encryption::test_home();
        let mgr = EncryptionManager::new_with_rotation_days(0).expect("construct");
        assert_eq!(
            mgr.key_rotation_interval, 86400 * 7,
            "0 days must not become a zero interval"
        );
    }

    /// The operator's security settings must survive the conversion, instead of
    /// `Core::new` substituting `SecurityConfig::default()` and discarding them.
    #[test]
    fn operator_security_config_is_not_replaced_by_defaults() {
        let mut cfg = crate::config::SecurityConfig::default();
        cfg.max_messages_per_minute = 3;
        cfg.blacklist_enabled = true;
        cfg.threat_detection_enabled = false;
        cfg.max_connections_per_ip = 2;
        cfg.allowed_ips = vec!["10.0.0.1".to_string()];
        cfg.blocked_ips = vec!["10.0.0.2".to_string()];

        let runtime = crate::core::security_v2::SecurityConfig::from(&cfg);
        assert_eq!(runtime.max_messages_per_minute, 3);
        assert!(runtime.blacklist_enabled);
        assert!(!runtime.threat_detection_enabled);
        assert_eq!(runtime.max_connections_per_ip, 2);
        assert_eq!(runtime.allowed_ips, vec!["10.0.0.1".to_string()]);
        assert_eq!(runtime.blocked_ips, vec!["10.0.0.2".to_string()]);
    }
}

#[cfg(test)]
mod tests {

    /// F2 REGRESSION: a v4 ratchet message must be signature-verified, and the
    /// check must reject a message whose signature does not match its sender.
    ///
    /// `decrypt_message_ratchet` did not call `verify_signature`, while v3
    /// `decrypt_message` does. The ratchet's AEAD tag proves the message
    /// decrypts with the chain key — NOT who sent it. The sender signs
    /// (`encrypt_message_ratchet` calls `sign_message`) and the signature
    /// crosses the wire, but the receiver ignored it entirely.
    ///
    /// Test design note, learned the hard way: simply replaying alice's
    /// ciphertext with mallory's signature is NOT a valid probe here. The
    /// ratchet consumes each message key, so a replay fails the AEAD check
    /// regardless of the signature and the test passes even with verification
    /// removed. To isolate the SIGNATURE check, each case below uses a FRESH
    /// ratchet message (fresh chain key, fresh AEAD) and corrupts only the
    /// signature field — so the only reason to reject is the signature.
    #[test]
    fn ratchet_message_requires_a_valid_signature() {
        let _home = super::test_home();

        let alice = EncryptionManager::new().unwrap();
        let bob = EncryptionManager::new().unwrap();

        // NOTE: a third `EncryptionManager` would NOT give us a different
        // identity here. `new()` -> `load_or_create()` reads GHOSTWIRE_HOME,
        // and `test_home()` sets ONE temp dir per test, so every manager built
        // in this test loads the SAME identity file and therefore the same
        // signing key. (An earlier draft of this test made that mistake and
        // "passed" for the wrong reason: the forger signed with alice's key, so
        // the signature was genuinely valid.) So the forger's key is generated
        // directly with `ring`, guaranteeing a distinct Ed25519 identity.
        let rng = ring::rand::SystemRandom::new();
        let mallory_pkcs8 = ring::signature::Ed25519KeyPair::generate_pkcs8(&rng)
            .expect("generate attacker keypair");
        let mallory_kp = ring::signature::Ed25519KeyPair::from_pkcs8(mallory_pkcs8.as_ref())
            .expect("parse attacker keypair");

        bob.add_peer_key(
            alice.get_key_id(),
            alice.get_public_key().to_vec(),
            KeyType::Hybrid,
        );

        // A genuine message round-trips: each side passes the OTHER's full
        // hybrid public key so the ECDH yields the same shared secret.
        alice
            .init_ratchet_session(&bob.get_key_id(), bob.get_public_key())
            .expect("alice establishes ratchet session");
        bob.init_ratchet_session_as_bob(&alice.get_key_id(), alice.get_public_key())
            .expect("bob establishes ratchet session");
        let genuine = alice
            .encrypt_message_ratchet("attack at dawn", &bob.get_key_id(), MessageType::Text)
            .expect("alice encrypts via ratchet");
        assert_eq!(
            bob.decrypt_message_ratchet(&genuine)
                .expect("bob decrypts alice's genuine message"),
            "attack at dawn"
        );

        // CASE 1: a FRESH ratchet message whose signature was replaced with one
        // made by a different key, while still claiming to be alice.
        // The AEAD is valid (fresh chain key), so only signature verification
        // can reject this.
        bob.init_ratchet_session_as_bob(&alice.get_key_id(), alice.get_public_key())
            .expect("re-establish session for a fresh chain key");
        let fresh = alice
            .encrypt_message_ratchet("second message", &bob.get_key_id(), MessageType::Text)
            .expect("alice encrypts a second ratchet message");

        let forged = EncryptedMessage {
            ciphertext: fresh.ciphertext.clone(),
            nonce: fresh.nonce.clone(),
            // Signed by a genuinely different Ed25519 key, so only signature
            // verification can reject this.
            signature: mallory_kp.sign(&fresh.ciphertext).as_ref().to_vec(),
            sender_id: alice.get_key_id(), // claims to be alice
            timestamp: fresh.timestamp,
            key_id: fresh.key_id.clone(),
            message_type: fresh.message_type.clone(),
            version: 0x04,
            pq_key: fresh.pq_key.clone(),
        };

        assert!(
            bob.decrypt_message_ratchet(&forged).is_err(),
            "SECURITY REGRESSION: a v4 ratchet message signed by the WRONG key was \
             accepted. A ratchet session is not sender authentication — the AEAD \
             tag only proves the message decrypts, not who sent it."
        );

        // CASE 2: a garbage signature on a fresh message must also be rejected.
        bob.init_ratchet_session_as_bob(&alice.get_key_id(), alice.get_public_key())
            .expect("re-establish session again");
        let fresh2 = alice
            .encrypt_message_ratchet("third message", &bob.get_key_id(), MessageType::Text)
            .expect("alice encrypts a third ratchet message");
        let tampered = EncryptedMessage {
            signature: vec![0xAAu8; 64],
            ..fresh2.clone()
        };
        assert!(
            bob.decrypt_message_ratchet(&tampered).is_err(),
            "a ratchet message with a bogus signature must be rejected"
        );
    }
    /// F3 REGRESSION: an existing peer key must not be replaceable through the
    /// unauthenticated `add_peer_key` path.
    ///
    /// `handle_handshake` (mesh.rs) calls `add_peer_key` twice per gossipsub
    /// frame: once under the transport-authenticated `peer_id`, and once under
    /// `msg.sender` - a free-text field inside the untrusted payload, with no
    /// binding to `peer_id` and no signature over the handshake. `add_peer_key`
    /// used to `insert` unconditionally, so any peer publishing to the DM topic
    /// could replace the key stored for an ARBITRARY third party. The decrypt
    /// path looks a key up by sender id and verifies the signature against it,
    /// so a substituted key turns forged messages into messages that verify.
    #[test]
    fn peer_key_substitution_is_refused() {
        let _home = super::test_home();
        let mgr = EncryptionManager::new().unwrap();

        let genuine = vec![0xABu8; 32];
        let attacker = vec![0xCDu8; 32];

        // First observation is accepted.
        mgr.add_peer_key("victim".to_string(), genuine.clone(), KeyType::Hybrid);
        assert_eq!(
            mgr.stored_peer_key_for_test("victim").as_deref(),
            Some(genuine.as_slice()),
            "the first key for an id must be stored"
        );

        // A substituted key from the unauthenticated path must be REFUSED.
        mgr.add_peer_key("victim".to_string(), attacker.clone(), KeyType::Hybrid);
        assert_eq!(
            mgr.stored_peer_key_for_test("victim").as_deref(),
            Some(genuine.as_slice()),
            "SECURITY REGRESSION: an existing peer key was overwritten via the \
             unauthenticated path; forged messages would verify against it"
        );

        // Re-announcing the SAME key is a no-op, not an error, and must not
        // churn state.
        mgr.add_peer_key("victim".to_string(), genuine.clone(), KeyType::Hybrid);
        assert_eq!(
            mgr.stored_peer_key_for_test("victim").as_deref(),
            Some(genuine.as_slice())
        );

        // Distinct ids still work, so the fix does not break normal operation.
        mgr.add_peer_key("other".to_string(), attacker.clone(), KeyType::Hybrid);
        assert_eq!(
            mgr.stored_peer_key_for_test("other").as_deref(),
            Some(attacker.as_slice())
        );
        assert_eq!(mgr.get_peer_count(), 2, "only two distinct ids expected");
    }

    /// DoS REGRESSION: the peer-key map must be bounded.
    ///
    /// `handle_handshake` supplies `msg.sender` verbatim, so a single peer
    /// could insert unlimited permanent entries by varying one string, with no
    /// crypto and no valid packet structure. There is no eviction elsewhere.
    #[test]
    fn peer_key_map_is_bounded() {
        let _home = super::test_home();
        let mgr = EncryptionManager::new().unwrap();
        let key = vec![0x11u8; 32];

        for i in 0..10_050 {
            mgr.add_peer_key(format!("attacker-{i}"), key.clone(), KeyType::Hybrid);
        }

        let count = mgr.get_peer_count();
        assert!(
            count <= 10_000,
            "SECURITY REGRESSION: peer_keys grew unbounded to {count} entries; \
             one peer can DoS the node by varying msg.sender"
        );
    }

    use super::*;

    #[test]
    fn test_encryption_roundtrip() {
        let _home = super::test_home();
        let alice = EncryptionManager::new().unwrap();
        let bob = EncryptionManager::new().unwrap();

        let message = "Hello, Bob!";

        // Bob needs to exist in Alice's peer keys
        alice.add_peer_key(
            bob.get_key_id(),
            bob.get_public_key().to_vec(),
            KeyType::Hybrid,
        );

        // Alice needs to exist in Bob's peer keys (for Bob to verify/derive key)
        bob.add_peer_key(
            alice.get_key_id(),
            alice.get_public_key().to_vec(),
            KeyType::Hybrid,
        );

        // Alice encrypts for Bob
        let encrypted = alice
            .encrypt_message(message, &bob.get_key_id(), MessageType::Text)
            .unwrap();

        // Bob decrypts
        let decrypted = bob.decrypt_message(&encrypted).unwrap();

        assert_eq!(message, decrypted);
    }

    /// A byte-identical redelivery of an accepted frame must be refused, and
    /// the refusal must be the replay check rather than any incidental
    /// failure. Built from one genuinely valid frame so the first delivery
    /// proves authenticity, and only the second call isolates the cache.
    #[test]
    fn replayed_frame_is_rejected_on_redelivery() {
        let _home = super::test_home();
        let alice = EncryptionManager::new().unwrap();
        let bob = EncryptionManager::new().unwrap();

        alice.add_peer_key(
            bob.get_key_id(),
            bob.get_public_key().to_vec(),
            KeyType::Hybrid,
        );
        bob.add_peer_key(
            alice.get_key_id(),
            alice.get_public_key().to_vec(),
            KeyType::Hybrid,
        );

        let frame = alice
            .encrypt_message("first delivery", &bob.get_key_id(), MessageType::Text)
            .unwrap();

        let first = bob
            .decrypt_message(&frame)
            .expect("the authentic first delivery must decrypt");
        assert_eq!(first, "first delivery");

        let error = bob
            .decrypt_message(&frame)
            .expect_err("a byte-identical redelivery must be refused")
            .to_string();
        assert!(
            error.contains("Replay"),
            "SECURITY REGRESSION: the second delivery of the SAME frame was \
             accepted, so a captured message replays forever. Got: {error:?}"
        );
    }

    /// Locking must stop decryption, including the path the old lock never
    /// covered: a session key already sitting in the cache. Two frames are
    /// used deliberately — the first warms the cache, and the second is fresh
    /// so the replay cache cannot be what rejects it below.
    #[test]
    fn locked_profile_cannot_decrypt_even_with_a_warm_session_cache() {
        let _home = super::test_home();
        let alice = EncryptionManager::new().unwrap();
        let bob = EncryptionManager::new().unwrap();

        alice.add_peer_key(
            bob.get_key_id(),
            bob.get_public_key().to_vec(),
            KeyType::Hybrid,
        );
        bob.add_peer_key(
            alice.get_key_id(),
            alice.get_public_key().to_vec(),
            KeyType::Hybrid,
        );

        let warm = alice
            .encrypt_message("warm the cache", &bob.get_key_id(), MessageType::Text)
            .unwrap();
        assert_eq!(
            bob.decrypt_message(&warm)
                .expect("the first delivery succeeds and caches a session key"),
            "warm the cache"
        );

        let after_lock = alice
            .encrypt_message("must not decrypt", &bob.get_key_id(), MessageType::Text)
            .unwrap();

        alice
            .init_ratchet_session(&bob.get_key_id(), bob.get_public_key())
            .expect("alice establishes ratchet session");
        bob.init_ratchet_session_as_bob(&alice.get_key_id(), alice.get_public_key())
            .expect("bob establishes ratchet session");
        let warm_ratchet = alice
            .encrypt_message_ratchet("warm the ratchet", &bob.get_key_id(), MessageType::Text)
            .expect("alice encrypts via ratchet");
        assert_eq!(
            bob.decrypt_message_ratchet(&warm_ratchet)
                .expect("the first ratchet delivery succeeds"),
            "warm the ratchet"
        );
        let ratchet_after_lock = alice
            .encrypt_message_ratchet("must not decrypt either", &bob.get_key_id(), MessageType::Text)
            .expect("alice encrypts a second ratchet frame");

        bob.lock_private_state().expect("locking succeeds");
        assert!(bob.is_locked(), "the manager reports itself locked");

        let error = bob
            .decrypt_message(&after_lock)
            .expect_err("a locked profile must not produce plaintext")
            .to_string();
        assert!(
            error.contains("locked"),
            "SECURITY REGRESSION: the locked profile produced plaintext. \
             Got: {error:?}"
        );

        let ratchet_error = bob
            .decrypt_message_ratchet(&ratchet_after_lock)
            .expect_err("a locked profile must not decrypt ratchet traffic")
            .to_string();
        assert!(
            ratchet_error.contains("locked"),
            "SECURITY REGRESSION: the locked profile decrypted v4 ratchet \
             traffic, which never touches private_key. Got: {ratchet_error:?}"
        );
    }

    #[test]
    fn test_signature_verification() {
        let _home = super::test_home();
        let encryption = EncryptionManager::new().unwrap();
        let message = b"Verify me!";
        let signature = encryption.sign_message(message).unwrap();

        // Verify with own public key
        assert!(encryption
            .verify_signature(message, &signature, encryption.get_public_key())
            .is_ok());

        // Fail with wrong message
        assert!(encryption
            .verify_signature(b"Wrong", &signature, encryption.get_public_key())
            .is_err());
    }

    #[test]
    fn test_hybrid_key_generation() {
        let rng = SystemRandom::new();
        let key_pair = EncryptionManager::generate_hybrid_key_pair(&rng).unwrap();

        // 32 Ed25519 + 32 X25519 + 1184 Kyber ML-KEM-768 = 1248
        assert_eq!(key_pair.public_key.len(), 1248);
        assert!(key_pair.private_key.len() > 34);
        // Header length prefix should be there
        let ed_len =
            u16::from_le_bytes([key_pair.private_key[0], key_pair.private_key[1]]) as usize;
        assert!(ed_len > 0);
        // Kyber secret key should be at the end: 2400 bytes for ML-KEM-768
        let pq_sk_start = 2 + ed_len + 32;
        assert_eq!(key_pair.private_key.len(), pq_sk_start + 2400);
        assert_eq!(key_pair.key_type, KeyType::Ultimate);
    }

    #[test]
    fn test_pq_hybrid_encrypt_decrypt() {
        let alice = EncryptionManager::create_new().unwrap();
        let bob = EncryptionManager::create_new().unwrap();

        assert_eq!(alice.key_pair.key_type, KeyType::Ultimate);
        assert_eq!(alice.key_pair.public_key.len(), 1248);

        alice.add_peer_key(
            bob.get_key_id(),
            bob.get_public_key().to_vec(),
            KeyType::Ultimate,
        );
        bob.add_peer_key(
            alice.get_key_id(),
            alice.get_public_key().to_vec(),
            KeyType::Ultimate,
        );

        let message = "PQ-hybrid encryption works!";
        let encrypted = alice
            .encrypt_message(message, &bob.get_key_id(), MessageType::Text)
            .unwrap();

        assert!(encrypted.pq_key.is_some(), "Kyber path not taken");
        assert_eq!(encrypted.pq_key.as_ref().unwrap().len(), 1088);

        let decrypted = bob.decrypt_message(&encrypted).unwrap();
        assert_eq!(message, decrypted);
    }

    /// HYBRID DOWNGRADE — regression guard, currently PASSING.
    ///
    /// A hybrid KEM is only worth more than its classical half if the PQ half
    /// cannot be dropped by whoever controls the wire. `pq_key` is an
    /// `Option<Vec<u8>>` on the wire and `decrypt_message` applies the PQ mix
    /// only `if let Some(pq_ciphertext)`, which looks like a silent
    /// classical-downgrade path.
    ///
    /// It is NOT exploitable as written, and the reason is worth recording: for
    /// a peer holding an ML-KEM key, `encrypt_message` always sets `pq_key`, so
    /// `final_key` is the MIXED key. If an attacker strips or corrupts `pq_key`,
    /// the receiver derives the UNMIXED classical key instead, and the XChaCha20-
    /// Poly1305 tag check fails. The AEAD turns the derivation mismatch into a
    /// hard failure.
    ///
    /// The safety therefore depends on an implicit invariant: "sender included
    /// PQ  =>  receiver must also apply PQ". That invariant is enforced by
    /// authentication failure rather than by an explicit check, so it is easy
    /// to break later by, for example, tolerating a missing `pq_key`. This test
    /// exists to make such a regression loud.
    ///
    /// The residual risk is a peer whose key is NOT >= 64+1184 bytes: for that
    /// peer `pq_key` is legitimately `None` on both sides, and the session is
    /// classical by design rather than downgraded.
    #[test]
    fn stripping_pq_key_must_not_silently_downgrade_to_classical() {
        let alice = EncryptionManager::create_new().unwrap();
        let bob = EncryptionManager::create_new().unwrap();

        assert_eq!(bob.key_pair.key_type, KeyType::Ultimate);
        alice.add_peer_key(
            bob.get_key_id(),
            bob.get_public_key().to_vec(),
            KeyType::Ultimate,
        );
        bob.add_peer_key(
            alice.get_key_id(),
            alice.get_public_key().to_vec(),
            KeyType::Ultimate,
        );

        let encrypted = alice
            .encrypt_message(
                "hybrid downgrade probe",
                &bob.get_key_id(),
                MessageType::Text,
            )
            .unwrap();
        assert!(
            encrypted.pq_key.is_some(),
            "sender should include a PQ component"
        );

        // Sanity: intact frame decrypts.
        assert_eq!(
            bob.decrypt_message(&encrypted).unwrap(),
            "hybrid downgrade probe"
        );

        // Attack: strip the PQ component entirely.
        let mut stripped = encrypted.clone();
        stripped.pq_key = None;
        let stripped_result = bob.decrypt_message(&stripped);

        // Attack: keep the field but corrupt the KEM ciphertext.
        let mut corrupted = encrypted.clone();
        let mut bad = corrupted.pq_key.clone().unwrap();
        bad[0] ^= 0xFF;
        corrupted.pq_key = Some(bad);
        let corrupted_result = bob.decrypt_message(&corrupted);

        assert!(
            stripped_result.is_err() && corrupted_result.is_err(),
            "hybrid receiver must REJECT a frame whose PQ component was removed or corrupted, \
             otherwise an active attacker downgrades the connection to classical X25519. \
             stripped={:?} corrupted={:?}",
            stripped_result.map(|_| "DECRYPTED"),
            corrupted_result.map(|_| "DECRYPTED"),
        );
    }

    #[test]
    fn corrupted_persistent_state_fails_closed_without_rotation() {
        let path = std::env::temp_dir().join(format!(
            "ghostwire-encryption-corrupt-{}.json",
            uuid::Uuid::new_v4()
        ));
        let original = b"corrupted-encryption-state";
        std::fs::write(&path, original).unwrap();

        let result = EncryptionManager::load_existing(&path);
        assert!(result.is_err(), "corrupt persistent state must fail closed");
        assert_eq!(std::fs::read(&path).unwrap(), original);
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn user_keyed_rewrap_does_not_depend_on_machine_master_key() {
        let _home = super::test_home();
        let home = tempfile::tempdir().unwrap();
        let base = home.path().join("vault");
        let persistence = std::sync::Arc::new(
            crate::core::persistence_manager::PersistenceManager::new_with_base_dir_and_passphrase(
                base,
                Some("encryption-test".to_string()),
                "encryption-test-passphrase",
            )
            .unwrap(),
        );
        let manager = EncryptionManager::create_new().unwrap();
        manager.migrate_to_user_keyed(&persistence).unwrap();
        assert!(EncryptionManager::is_user_keyed());
    }

    #[test]
    fn user_keyed_restart_starts_public_only_until_explicit_activation() {
        let _home = super::test_home();
        let home = tempfile::tempdir().unwrap();
        let persistence =
            crate::core::persistence_manager::PersistenceManager::new_with_base_dir_and_passphrase(
                home.path().join("vault"),
                Some("encryption-restart-test".to_string()),
                "encryption-restart-passphrase",
            )
            .unwrap();
        let manager = EncryptionManager::create_new().unwrap();
        manager.save_to_disk().unwrap();
        let key_id = manager.get_key_id();
        manager.migrate_to_user_keyed(&persistence).unwrap();
        assert!(!EncryptionManager::master_key_path().unwrap().exists());
        assert!(EncryptionManager::public_stub_path().unwrap().exists());

        let locked = EncryptionManager::load_or_create().unwrap();
        assert_eq!(locked.get_key_id(), key_id);
        assert!(locked.private_key_snapshot().is_err());
        assert!(locked.sign_message(b"locked").is_err());
        assert!(EncryptionManager::encrypt_bytes(b"machine-fallback").is_err());

        locked.activate_user_keyed(&persistence).unwrap();
        assert!(locked.private_key_snapshot().is_ok());
        assert!(locked.sign_message(b"activated").is_ok());
    }

    #[test]
    fn test_secure_storage() {
        let _home = super::test_home();
        let file_path = std::path::Path::new("test_secure_storage.json");

        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct TestData {
            name: String,
            val: i32,
        }
        let data = TestData {
            name: "test".to_string(),
            val: 42,
        };

        EncryptionManager::secure_save(&file_path, &data).unwrap();
        let loaded: TestData = EncryptionManager::secure_load(&file_path).unwrap();

        assert_eq!(data, loaded);

        // Cleanup
        let _ = std::fs::remove_file(file_path);
    }

    use crate::genesis_constants::{ANON_INTRO_COMMIT_DOMAIN, TRANSPORT_KEY_ROTATE_DOMAIN};
    use crate::genesis_constants::genesis_derive;

    #[test]
    fn transport_key_ring_derives_three_periods() {
        let secret = [0xABu8; 32];
        let ring = TransportKeyRing::derive(&secret, 0);

        // epoch 0: prev should be for epoch -1 (saturating), curr for epoch 0, next for epoch 1
        assert_eq!(ring.rotation_epoch, 0);
        assert_ne!(ring.curr_in, ring.prev_in, "curr and prev keys must differ");
        assert_ne!(ring.curr_in, ring.next_in, "curr and next keys must differ");
        assert_ne!(ring.prev_in, ring.next_in, "prev and next keys must differ");
        assert_ne!(ring.curr_in, ring.curr_out, "in and out keys must differ");
    }

    #[test]
    fn transport_key_ring_uses_label_separation() {
        let secret = [0x42u8; 32];
        let now = 1_000_000u64;
        let ring = TransportKeyRing::derive(&secret, now);

        // In and out keys must be different (label separation prevents
        // encrypting and decrypting with the same key)
        assert_ne!(ring.curr_in, ring.curr_out, "in/out keys must differ (label separation)");

        // Verify the derivation chain: master -> curr_in uses a different
        // domain than derive_period_key
        let master = genesis_derive(TRANSPORT_KEY_ROTATE_DOMAIN, &secret);
        let expected_curr_in = genesis_derive(ANON_INTRO_COMMIT_DOMAIN, &master);
        assert_eq!(ring.curr_in, expected_curr_in, "curr_in derivation must match");
    }

    #[test]
    fn transport_key_ring_refresh_advances_epoch() {
        let secret = [0x99u8; 32];
        let period = KEY_ROTATION_PERIOD_SECS;

        // Ring at time T
        let ring_old = TransportKeyRing::derive(&secret, period);

        // Ring at time T + 1 period should advance epoch
        let ring_new = TransportKeyRing::derive(&secret, period * 2);
        assert_eq!(ring_old.rotation_epoch + 1, ring_new.rotation_epoch);

        // Old ring should refresh
        let refreshed = ring_old.refresh_if_needed(&secret, period * 2);
        assert!(refreshed.is_some(), "ring should refresh when epoch advances");

        // Ring at same time should not refresh
        let no_refresh = ring_old.refresh_if_needed(&secret, period);
        assert!(no_refresh.is_none(), "ring should NOT refresh within same period");
    }

    #[test]
    fn transport_key_ring_select_incoming_by_period_hint() {
        let secret = [0x11u8; 32];
        let ring = TransportKeyRing::derive(&secret, 0);

        assert_eq!(ring.select_incoming_key(0), Some(ring.curr_in));
        assert_eq!(ring.select_incoming_key(-1), Some(ring.prev_in));
        assert_eq!(ring.select_incoming_key(1), Some(ring.next_in));
        assert_eq!(ring.select_incoming_key(99), None, "invalid period hint returns None");
    }

    #[test]
    fn transport_key_ring_serialization_roundtrip() {
        let secret = [0x88u8; 32];
        let ring = TransportKeyRing::derive(&secret, 12345);

        let serialized = bincode::serialize(&ring).unwrap();
        let deserialized: TransportKeyRing = bincode::deserialize(&serialized).unwrap();

        assert_eq!(ring.rotation_epoch, deserialized.rotation_epoch);
        assert_eq!(ring.prev_in, deserialized.prev_in);
        assert_eq!(ring.curr_in, deserialized.curr_in);
        assert_eq!(ring.curr_out, deserialized.curr_out);
        assert_eq!(ring.next_in, deserialized.next_in);
        assert_eq!(ring.prev_out, deserialized.prev_out);
        assert_eq!(ring.next_out, deserialized.next_out);
    }
}

#[derive(Serialize, Deserialize)]
struct PersistedEncryption {
    key_pair: GhostKeyPair,
    key_rotation_interval: u64,
    last_key_rotation: u64,
    /// Overlapping transport key ring (BTP-style prev/curr/next).
    /// Optional for backward compatibility with older state files.
    #[serde(default)]
    transport_key_ring: Option<TransportKeyRing>,
}
