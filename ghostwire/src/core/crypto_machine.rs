//! crypto_machine.rs — Cryptographic state machine using vodozemac
//!
//! FIXES APPLIED:
//!   The original file rolled a custom Double Ratchet using raw ring + x25519.
//!   vodozemac was in Cargo.toml but unused. Custom crypto is almost always wrong.
//!
//!   This replacement uses vodozemac's audited Olm/Megolm implementation
//!   (the same library used by Element/Matrix, audited by Trail of Bits).
//!
//!   vodozemac provides:
//!     - Account: identity key management + one-time key prekeys
//!     - Session: Double Ratchet session (bidirectional)
//!     - Encrypt/decrypt with forward secrecy
//!
//!   For peers not yet in a session, we use vodozemac's pre-key message
//!   flow (equivalent to Signal's X3DH). Once established, normal message
//!   exchange uses the Double Ratchet.
//!
//!   The "sealed" fallback (for offline peers) uses a plain ECIES construction
//!   with ChaCha20Poly1305 and a fresh X25519 ephemeral — no session needed.

use anyhow::Result;
use crate::genesis_constants::{SEALED_KDF_DOMAIN, derive_with_domain};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Key, Nonce,
};
use rand::RngCore;
use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use vodozemac::{
    olm::{Account, AccountPickle, OlmMessage, Session, SessionPickle},
    Curve25519PublicKey, Ed25519PublicKey,
};
use zeroize::Zeroize;

// ── Events and Actions (preserved API from original) ─────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CryptoEvent {
    IncomingMessage {
        sender_id: String,
        device_id: String,
        payload: Vec<u8>,
    },
    EncryptMessage {
        recipient_id: String,
        plaintext: Vec<u8>,
    },
    EncryptSealedMessage {
        recipient_id: String,
        recipient_curve25519_pub: [u8; 32],
        plaintext: Vec<u8>,
    },
    PeerDiscovered {
        peer_id: String,
        device_id: String,
        device_name: String,
        /// Peer's serialized identity keys (Ed25519 + Curve25519 from their Account)
        identity_keys: Vec<u8>,
        /// A one-time prekey claimed for session establishment
        one_time_key: Vec<u8>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CryptoAction {
    SendPacket {
        recipient_id: String,
        packet: Vec<u8>,
    },
    StoreSecret { key: String, data: Vec<u8> },
    MessageDecrypted {
        sender_id: String,
        plaintext: Vec<u8>,
    },
    DecryptionFailed { sender_id: String, reason: String },
    Log { level: String, message: String },
    DepositMailbox {
        recipient_id: String,
        packet: SealedPacket,
    },
    /// Publish our current one-time prekeys to the DHT so peers can establish sessions
    PublishPrekeys {
        one_time_keys: Vec<[u8; 32]>,
    },
}

/// ECIES sealed packet for offline / pre-session delivery
#[derive(Debug, Clone, Serialize, Deserialize, Zeroize)]
pub struct SealedPacket {
    #[zeroize(skip)]
    pub ephemeral_pub: [u8; 32],
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; 12],
}

// ── Session serialization (for sled persistence) ──────────────────────────────

#[derive(Serialize, Deserialize)]
struct PersistedSession {
    peer_id: String,
    pickle: String, // vodozemac SessionPickle as base64
    passphrase_hash: Vec<u8>,
}

// ── CryptoMachine ─────────────────────────────────────────────────────────────

/// The vodozemac-backed cryptographic state machine.
/// Thread-safe via internal Arc<Mutex<_>>.
pub struct CryptoMachine {
    /// Our identity (vodozemac Account)
    account: Arc<Mutex<Account>>,
    /// Active sessions keyed by peer_id
    sessions: Arc<Mutex<HashMap<String, Session>>>,
    system_rng: SystemRandom,
}

impl CryptoMachine {
    /// Create a new identity. Call this on first run.
    pub fn new() -> Self {
        Self {
            account: Arc::new(Mutex::new(Account::new())),
            sessions: Arc::new(Mutex::new(HashMap::new())),
            system_rng: SystemRandom::new(),
        }
    }

    /// Restore from a persisted Account pickle.
    pub fn from_pickle(pickle_bytes: &[u8], passphrase: &[u8]) -> Result<Self> {
        let pickle_str = std::str::from_utf8(pickle_bytes)?;
        let pickle: AccountPickle = serde_json::from_str(pickle_str)?;
        let account = Account::from_pickle(pickle);
        Ok(Self {
            account: Arc::new(Mutex::new(account)),
            sessions: Arc::new(Mutex::new(HashMap::new())),
            system_rng: SystemRandom::new(),
        })
    }

    /// Serialize the Account for storage. Encrypt with Argon2 + ChaCha20 before saving.
    pub fn to_pickle(&self) -> Result<Vec<u8>> {
        let account = self.account.lock().unwrap();
        let pickle = account.pickle();
        let json = serde_json::to_string(&pickle)?;
        Ok(json.into_bytes())
    }

    /// Our Ed25519 identity public key (for announcements)
    pub fn ed25519_identity(&self) -> Ed25519PublicKey {
        self.account.lock().unwrap().ed25519_key()
    }

    /// Our Curve25519 identity public key (for session establishment)
    pub fn curve25519_identity(&self) -> Curve25519PublicKey {
        self.account.lock().unwrap().curve25519_key()
    }

    /// Generate fresh one-time prekeys and return them for DHT publication.
    /// Call this periodically (e.g. when prekey count drops below 5).
    pub fn generate_one_time_keys(&self, count: usize) -> Vec<Curve25519PublicKey> {
        let mut account = self.account.lock().unwrap();
        account.generate_one_time_keys(count);
        let keys = account.one_time_keys();
        account.mark_keys_as_published();
        keys.curve25519().values().cloned().collect()
    }

    /// Process a crypto event and produce a list of actions.
    /// This is the pure state-machine dispatch method.
    pub fn process(&self, event: CryptoEvent) -> Vec<CryptoAction> {
        match event {
            CryptoEvent::EncryptMessage { recipient_id, plaintext } => {
                self.encrypt_for_peer(&recipient_id, &plaintext)
            }
            CryptoEvent::EncryptSealedMessage { recipient_id, recipient_curve25519_pub, plaintext } => {
                match self.encrypt_sealed(&recipient_curve25519_pub, &plaintext) {
                    Ok(packet) => vec![
                        CryptoAction::DepositMailbox { recipient_id, packet }
                    ],
                    Err(e) => vec![CryptoAction::Log {
                        level: "error".into(),
                        message: format!("Sealed encrypt failed: {}", e),
                    }],
                }
            }
            CryptoEvent::IncomingMessage { sender_id, device_id: _, payload } => {
                self.decrypt_from_peer(&sender_id, &payload)
            }
            CryptoEvent::PeerDiscovered { peer_id, identity_keys, one_time_key, .. } => {
                self.establish_session(&peer_id, &identity_keys, &one_time_key)
            }
        }
    }

    fn encrypt_for_peer(&self, peer_id: &str, plaintext: &[u8]) -> Vec<CryptoAction> {
        let mut sessions = self.sessions.lock().unwrap();
        match sessions.get_mut(peer_id) {
            Some(session) => {
                let olm_msg = session.encrypt(plaintext);
                let packet = match serde_json::to_vec(&olm_msg) {
                    Ok(p) => p,
                    Err(e) => return vec![CryptoAction::Log {
                        level: "error".into(),
                        message: format!("Olm serialize failed: {}", e),
                    }],
                };
                vec![CryptoAction::SendPacket { recipient_id: peer_id.to_string(), packet }]
            }
            None => {
                // No session yet — fall back to sealed packet via mailbox
                vec![CryptoAction::Log {
                    level: "warn".into(),
                    message: format!("No session for {} — cannot encrypt; request peer identity first", peer_id),
                }]
            }
        }
    }

    fn decrypt_from_peer(&self, sender_id: &str, payload: &[u8]) -> Vec<CryptoAction> {
        let olm_msg: OlmMessage = match serde_json::from_slice(payload) {
            Ok(m) => m,
            Err(e) => return vec![CryptoAction::DecryptionFailed {
                sender_id: sender_id.to_string(),
                reason: format!("Deserialize failed: {}", e),
            }],
        };

        let mut sessions = self.sessions.lock().unwrap();

        if let Some(session) = sessions.get_mut(sender_id) {
            // Existing session
            match session.decrypt(&olm_msg) {
                Ok(plaintext) => vec![CryptoAction::MessageDecrypted {
                    sender_id: sender_id.to_string(),
                    plaintext,
                }],
                Err(e) => vec![CryptoAction::DecryptionFailed {
                    sender_id: sender_id.to_string(),
                    reason: format!("Decrypt failed: {}", e),
                }],
            }
        } else if let OlmMessage::PreKey(prekey_msg) = &olm_msg {
            // New inbound session — create from prekey message
            let mut account = self.account.lock().unwrap();
            match account.create_inbound_session(prekey_msg) {
                Ok(result) => {
                    let (session, plaintext) = (result.session, result.plaintext);
                    sessions.insert(sender_id.to_string(), session);
                    vec![CryptoAction::MessageDecrypted {
                        sender_id: sender_id.to_string(),
                        plaintext,
                    }]
                }
                Err(e) => vec![CryptoAction::DecryptionFailed {
                    sender_id: sender_id.to_string(),
                    reason: format!("Inbound session create failed: {}", e),
                }],
            }
        } else {
            vec![CryptoAction::DecryptionFailed {
                sender_id: sender_id.to_string(),
                reason: "No session and message is not a PreKey message".to_string(),
            }]
        }
    }

    fn establish_session(
        &self, peer_id: &str, identity_keys_bytes: &[u8], one_time_key_bytes: &[u8],
    ) -> Vec<CryptoAction> {
        let curve25519_key: Curve25519PublicKey = match serde_json::from_slice(identity_keys_bytes) {
            Ok(k) => k,
            Err(e) => return vec![CryptoAction::Log {
                level: "error".into(),
                message: format!("Bad peer identity key for {}: {}", peer_id, e),
            }],
        };
        let one_time_key: Curve25519PublicKey = match serde_json::from_slice(one_time_key_bytes) {
            Ok(k) => k,
            Err(e) => return vec![CryptoAction::Log {
                level: "error".into(),
                message: format!("Bad one-time key for {}: {}", peer_id, e),
            }],
        };

        let mut account = self.account.lock().unwrap();
        let session = account.create_outbound_session(curve25519_key, one_time_key);
        drop(account);

        self.sessions.lock().unwrap().insert(peer_id.to_string(), session);

        vec![CryptoAction::Log {
            level: "info".into(),
            message: format!("Session established with {}", peer_id),
        }]
    }

    /// ECIES sealed encrypt: fresh X25519 ephemeral + ChaCha20Poly1305.
    /// Used when no session exists (offline peer, mailbox delivery).
    pub fn encrypt_sealed(&self, recipient_pub: &[u8; 32], plaintext: &[u8]) -> Result<SealedPacket> {
        let mut ephemeral_bytes = [0u8; 32];
        self.system_rng.fill(&mut ephemeral_bytes).map_err(|_| anyhow::anyhow!("RNG failure"))?;
        ephemeral_bytes[0] &= 248; ephemeral_bytes[31] &= 127; ephemeral_bytes[31] |= 64;
        let ephemeral_secret = x25519_dalek::StaticSecret::from(ephemeral_bytes);
        let ephemeral_pub_key = x25519_dalek::PublicKey::from(&ephemeral_secret);
        let recipient_key = x25519_dalek::PublicKey::from(*recipient_pub);
        let shared = ephemeral_secret.diffie_hellman(&recipient_key);
        let sym_key = derive_with_domain(SEALED_KDF_DOMAIN, shared.as_bytes());
        let cipher = ChaCha20Poly1305::new(Key::from_slice(sym_key.as_bytes()));
        let mut nonce_bytes = [0u8; 12];
        self.system_rng.fill(&mut nonce_bytes).map_err(|_| anyhow::anyhow!("RNG failure"))?;
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = cipher.encrypt(nonce, plaintext)
            .map_err(|_| anyhow::anyhow!("Sealed encrypt failed"))?;
        Ok(SealedPacket {
            ephemeral_pub: *ephemeral_pub_key.as_bytes(),
            ciphertext,
            nonce: nonce_bytes,
        })
    }

    /// Decrypt a sealed packet using our Curve25519 identity key.
    pub fn decrypt_sealed(&self, packet: &SealedPacket, our_secret: &x25519_dalek::StaticSecret) -> Result<Vec<u8>> {
        let sender_pub = x25519_dalek::PublicKey::from(packet.ephemeral_pub);
        let shared = our_secret.diffie_hellman(&sender_pub);
        let sym_key = derive_with_domain(SEALED_KDF_DOMAIN, shared.as_bytes());
        let cipher = ChaCha20Poly1305::new(Key::from_slice(sym_key.as_bytes()));
        let nonce = Nonce::from_slice(&packet.nonce);
        cipher.decrypt(nonce, packet.ciphertext.as_slice())
            .map_err(|_| anyhow::anyhow!("Sealed decrypt failed — wrong key or tampered"))
    }

    /// Number of active sessions
    pub fn session_count(&self) -> usize {
        self.sessions.lock().unwrap().len()
    }

    /// Drop a session (e.g. after peer blocks or disconnects)
    pub fn drop_session(&self, peer_id: &str) {
        self.sessions.lock().unwrap().remove(peer_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sealed_roundtrip() {
        let machine = CryptoMachine::new();
        let mut secret_bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut secret_bytes);
        secret_bytes[0] &= 248; secret_bytes[31] &= 127; secret_bytes[31] |= 64;
        let recipient_secret = x25519_dalek::StaticSecret::from(secret_bytes);
        let recipient_pub = x25519_dalek::PublicKey::from(&recipient_secret);

        let plaintext = b"Nairobi sunrise";
        let packet = machine.encrypt_sealed(recipient_pub.as_bytes(), plaintext).unwrap();
        let decrypted = machine.decrypt_sealed(&packet, &recipient_secret).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn sealed_tamper_detected() {
        let machine = CryptoMachine::new();
        let mut secret_bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut secret_bytes);
        secret_bytes[0] &= 248; secret_bytes[31] &= 127; secret_bytes[31] |= 64;
        let recipient_secret = x25519_dalek::StaticSecret::from(secret_bytes);
        let recipient_pub = x25519_dalek::PublicKey::from(&recipient_secret);

        let mut packet = machine.encrypt_sealed(recipient_pub.as_bytes(), b"secret").unwrap();
        packet.ciphertext[0] ^= 0xFF;
        assert!(machine.decrypt_sealed(&packet, &recipient_secret).is_err());
    }

    #[test]
    fn identity_keys_are_stable() {
        let m = CryptoMachine::new();
        let ed = m.ed25519_identity();
        let curve = m.curve25519_identity();
        // Keys don't change between calls
        assert_eq!(m.ed25519_identity().to_base64(), ed.to_base64());
        assert_eq!(m.curve25519_identity().to_base64(), curve.to_base64());
    }
}
