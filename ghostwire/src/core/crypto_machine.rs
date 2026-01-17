//! Sans I/O Cryptographic State Machine
//! Inspired by matrix-rust-sdk's vodozemac/olm-machine approach.
//! This module contains pure logic and no side effects.

use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit};
use rand::RngCore;
use ring::hkdf::{Salt, HKDF_SHA256};
use ring::hmac;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use x25519_dalek::{PublicKey, StaticSecret};

/// Events that drive the state machine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CryptoEvent {
    /// Received a message from the network
    IncomingMessage {
        sender_id: String,
        device_id: String,
        payload: Vec<u8>,
    },
    /// Command to encrypt a message for a peer
    EncryptMessage {
        recipient_id: String,
        plaintext: Vec<u8>,
    },
    /// Command to encrypt a stateless sealed message (fallback)
    EncryptSealedMessage {
        recipient_id: String,
        recipient_public_key: [u8; 32],
        plaintext: Vec<u8>,
    },
    /// New peer discovered - try to establish a session
    PeerDiscovered {
        peer_id: String,
        device_id: String,
        device_name: String,
        public_key: Vec<u8>,
    },
}

/// Actions the state machine requests the caller to perform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CryptoAction {
    /// Send an encrypted packet over the network
    SendPacket {
        recipient_id: String,
        packet: Vec<u8>,
    },
    /// Store a session key or secret to persistent storage
    StoreSecret { key: String, data: Vec<u8> },
    /// Notify the UI of a decrypted message
    MessageDecrypted {
        sender_id: String,
        plaintext: Vec<u8>,
    },
    /// Handle a decryption failure
    DecryptionFailed { sender_id: String, reason: String },
    /// Log an internal warning/info (pure output)
    Log { level: String, message: String },
    /// Request decryption of a sealed packet (requires secret key)
    DecryptSealedPacket {
        sender_id: String,
        ephemeral_pub: [u8; 32],
        ciphertext: Vec<u8>,
    },
    /// Deposit a packet into the mailbox swarm
    DepositMailbox {
        recipient_id: String,
        packet: SealedPacket,
    },
}

/// Double Ratchet Message Packet (On-the-wire format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePacket {
    pub dh_pub: [u8; 32],
    pub n: u32,
    pub pn: u32,
    pub nonce: [u8; 12],
    pub prev_hash: [u8; 32],
    pub sender_device_id: String,
    pub ciphertext: Vec<u8>,
}

/// Stateless Sealed Packet for fallback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SealedPacket {
    pub ephemeral_pub: [u8; 32],
    pub ciphertext: Vec<u8>,
}

/// Double Ratchet Session State
#[derive(Serialize, Deserialize)]
pub struct RatchetSession {
    pub peer_id: String,
    pub root_key: [u8; 32],
    pub sending_chain_key: Option<[u8; 32]>,
    pub receiving_chain_key: Option<[u8; 32]>,
    pub dh_local_static: [u8; 32],
    pub dh_remote_static: [u8; 32],
    pub dh_ephemeral_private: [u8; 32],
    pub dh_ephemeral_public: [u8; 32],
    pub dh_remote_ephemeral: Option<[u8; 32]>,
    pub send_count: u32,
    pub recv_count: u32,
    pub prev_send_count: u32,
    pub skipped_message_keys: HashMap<(u32, [u8; 32]), [u8; 32]>,
    #[serde(default)]
    pub last_sent_hash: [u8; 32],
    #[serde(default)]
    pub last_recv_hash: [u8; 32],
}

/// The core trait for a Sans I/O crypto processor
pub trait CryptoMachine {
    fn handle_event(&mut self, event: CryptoEvent) -> Vec<CryptoAction>;
    fn tick(&mut self, timestamp: u64) -> Vec<CryptoAction>;
}

/// The main implementation of the GhostWire Crypto Machine
#[derive(Default, Serialize, Deserialize)]
pub struct GhostCryptoMachine {
    pub identity_id: String,
    pub master_public_key: Vec<u8>,
    /// peer_id -> device_id -> session
    pub sessions: HashMap<String, HashMap<String, RatchetSession>>,
    /// peer_id -> Vec<DeviceIdentity>
    pub authorized_devices: HashMap<String, Vec<crate::core::identity::DeviceIdentity>>,
}

impl GhostCryptoMachine {
    pub fn new(identity_id: String, master_public_key: Vec<u8>) -> Self {
        Self {
            identity_id,
            master_public_key,
            sessions: HashMap::new(),
            authorized_devices: HashMap::new(),
        }
    }

    /// KDF for the root chain (HKDF-SHA256)
    fn kdf_rk(root_key: &[u8; 32], dh_out: &[u8; 32]) -> ([u8; 32], [u8; 32]) {
        let salt = Salt::new(HKDF_SHA256, root_key.as_slice());
        let prk = salt.extract(dh_out.as_slice());
        let info = [b"GhostWire-RK".as_slice()];
        let okm = prk
            .expand(&info[..], HKDF_SHA256)
            .expect("HKDF expansion failed");

        let mut expanded = [0u8; 64];
        okm.fill(&mut expanded)
            .expect("Failed to fill expanded buffer");

        let mut rk = [0u8; 32];
        let mut ck = [0u8; 32];
        rk.copy_from_slice(&expanded[0..32]);
        ck.copy_from_slice(&expanded[32..64]);

        (rk, ck)
    }

    /// KDF for the sending/receiving chains (HMAC-SHA256)
    fn kdf_ck(chain_key: &[u8; 32]) -> ([u8; 32], [u8; 32]) {
        let key = hmac::Key::new(hmac::HMAC_SHA256, chain_key.as_slice());
        let tag_step = hmac::sign(&key, &[0x01][..]);
        let tag_msg = hmac::sign(&key, &[0x02][..]);

        let mut next_ck = [0u8; 32];
        let mut message_key = [0u8; 32];
        next_ck.copy_from_slice(tag_step.as_ref());
        message_key.copy_from_slice(tag_msg.as_ref());

        (next_ck, message_key)
    }

    /// Initialize a new session
    pub fn init_session(
        &mut self,
        peer_id: String,
        device_id: String,
        peer_pub: Vec<u8>,
    ) -> Vec<CryptoAction> {
        let mut actions = Vec::new();
        let initial_root_key = [0u8; 32];
        let (root_key, chain_key) = Self::kdf_rk(&initial_root_key, &[0u8; 32]);

        let alice_secret = StaticSecret::random_from_rng(rand::thread_rng());
        let alice_public = PublicKey::from(&alice_secret);

        let session = RatchetSession {
            peer_id: peer_id.clone(),
            root_key,
            sending_chain_key: Some(sending_chain_key),
            receiving_chain_key: Some(receiving_chain_key),
            dh_local_static: [0u8; 32],
            dh_remote_static: peer_pub_array,
            dh_ephemeral_private: [0u8; 32],
            dh_ephemeral_public: [0u8; 32],
            dh_remote_ephemeral: None,
            send_count: 0,
            recv_count: 0,
            prev_send_count: 0,
            skipped_message_keys: HashMap::new(),
            last_sent_hash: [0u8; 32],
            last_recv_hash: [0u8; 32],
        };

        self.sessions
            .entry(peer_id.clone())
            .or_insert_with(HashMap::new)
            .insert(device_id, session);

        actions.push(CryptoAction::Log {
            level: "INFO".into(),
            message: format!("Initialized Double Ratchet session for peer {}", peer_id),
        });

        actions
    }

    /// Advances the chain and handles skipped message keys
    fn skip_message_keys(session: &mut RatchetSession, until: u32) {
        if let Some(mut ck) = session.receiving_chain_key {
            while session.recv_count < until {
                let (next_ck, message_key) = Self::kdf_ck(&ck);
                ck = next_ck;
                let dh_pub = session.dh_remote_ephemeral.unwrap_or([0u8; 32]);
                session
                    .skipped_message_keys
                    .insert((session.recv_count, dh_pub), message_key);
                session.recv_count += 1;
            }
            session.receiving_chain_key = Some(ck);
        }
    }

    /// Advances the DH Ratchet
    fn dh_ratchet_step(session: &mut RatchetSession, remote_ephemeral: [u8; 32]) {
        session.dh_remote_ephemeral = Some(remote_ephemeral);

        // 1. Receiving step
        let alice_secret = StaticSecret::from(session.dh_ephemeral_private);
        let bob_public = PublicKey::from(remote_ephemeral);
        let dh_recv = alice_secret.diffie_hellman(&bob_public);

        let (rk, ck_recv) = Self::kdf_rk(&session.root_key, dh_recv.as_bytes());
        session.root_key = rk;
        session.receiving_chain_key = Some(ck_recv);
        session.recv_count = 0;

        // 2. Generate new ephemeral for sending
        let new_alice_secret = StaticSecret::random_from_rng(rand::thread_rng());
        let new_alice_public = PublicKey::from(&new_alice_secret);
        session.dh_ephemeral_private = *new_alice_secret.as_bytes();
        session.dh_ephemeral_public = *new_alice_public.as_bytes();
        session.prev_send_count = session.send_count;

        // 3. Sending step
        let dh_send = new_alice_secret.diffie_hellman(&bob_public);
        let (rk, ck_send) = Self::kdf_rk(&session.root_key, dh_send.as_bytes());
        session.root_key = rk;
        session.sending_chain_key = Some(ck_send);
        session.send_count = 0;
    }

    /// Hybrid Encryption/Decryption helper
    fn decrypt(key: &[u8; 32], ciphertext: &[u8], nonce: &[u8; 12]) -> Option<Vec<u8>> {
        let cipher_key = aes_gcm::Key::<Aes256Gcm>::from_slice(key.as_slice());
        let cipher = Aes256Gcm::new(cipher_key);
        let nonce = aes_gcm::Nonce::from_slice(nonce.as_slice());
        cipher.decrypt(nonce, ciphertext).ok()
    }

    fn encrypt(key: &[u8; 32], plaintext: &[u8], nonce: &[u8; 12]) -> Vec<u8> {
        let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key[..]);
        let cipher = Aes256Gcm::new(key);
        let nonce = aes_gcm::Nonce::from_slice(&nonce[..]);
        cipher
            .encrypt(nonce, plaintext)
            .expect("encryption failure")
    }

    pub fn hash_packet(&self, packet: &MessagePacket) -> [u8; 32] {
        use ring::digest;
        let mut ctx = digest::Context::new(&digest::SHA256);
        ctx.update(&packet.dh_pub);
        ctx.update(&packet.n.to_le_bytes());
        ctx.update(&packet.pn.to_le_bytes());
        ctx.update(&packet.nonce);
        ctx.update(&packet.prev_hash);
        ctx.update(packet.sender_device_id.as_bytes());
        ctx.update(&packet.ciphertext);
        let mut hash = [0u8; 32];
        hash.copy_from_slice(ctx.finish().as_ref());
        hash
    }
}

impl CryptoMachine for GhostCryptoMachine {
    fn handle_event(&mut self, event: CryptoEvent) -> Vec<CryptoAction> {
        let mut actions = Vec::new();
        match event {
            CryptoEvent::IncomingMessage {
                sender_id,
                device_id,
                payload,
            } => {
                // Try to deserialize as a Double Ratchet packet
                if let Ok(packet) = bincode::deserialize::<MessagePacket>(&payload) {
                    let session_map = self.sessions.entry(sender_id.clone()).or_default();
                    let session = match session_map.get_mut(&device_id) {
                        Some(s) => s,
                        None => {
                            actions.push(CryptoAction::Log {
                                level: "WARN".into(),
                                message: format!("No session found for peer {}:{}", sender_id, device_id),
                            });
                            return actions;
                        }
                    };

                    // Audit hash chain
                    if packet.prev_hash != session.last_recv_hash {
                        actions.push(CryptoAction::Log {
                            level: "WARN".into(),
                            message: format!(
                                "Hash chain gap detected for {}:{}: expected {}, got {}",
                                sender_id,
                                device_id,
                                hex::encode(session.last_recv_hash),
                                hex::encode(packet.prev_hash)
                            ),
                        });
                    }
                    session.last_recv_hash = self.hash_packet(&packet);

                    // 1. Try skipped keys
                    if let Some(mk) = session
                        .skipped_message_keys
                        .remove(&(packet.n, packet.dh_pub))
                    {
                        if let Some(plaintext) =
                            Self::decrypt(&mk, &packet.ciphertext, &packet.nonce)
                        {
                            actions.push(CryptoAction::MessageDecrypted {
                                sender_id: sender_id.clone(),
                                plaintext,
                            });
                        }
                        return actions;
                    }

                    // 2. Handle DH Ratchet shift
                    if Some(packet.dh_pub) != session.dh_remote_ephemeral {
                        Self::skip_message_keys(session, packet.pn);
                        Self::dh_ratchet_step(session, packet.dh_pub);
                    }

                    // 3. Catch up if n > recv_count
                    Self::skip_message_keys(session, packet.n);

                    // 4. Decrypt with current chain
                    if let Some(ck) = session.receiving_chain_key {
                        let (next_ck, message_key) = Self::kdf_ck(&ck);
                        session.receiving_chain_key = Some(next_ck);
                        session.recv_count += 1;

                        if let Some(plaintext) =
                            Self::decrypt(&message_key, &packet.ciphertext, &packet.nonce)
                        {
                            actions.push(CryptoAction::MessageDecrypted {
                                sender_id: sender_id.clone(),
                                plaintext,
                            });
                        } else {
                            actions.push(CryptoAction::DecryptionFailed {
                                sender_id: sender_id.clone(),
                                reason: "AEAD auth failure".into(),
                            });
                        }
                    }
                } else if let Ok(sealed) = bincode::deserialize::<SealedPacket>(&payload) {
                    // Falls back to SealedPacket decryption if it's not a Double Ratchet packet
                    actions.push(CryptoAction::DecryptSealedPacket {
                        sender_id: sender_id.clone(),
                        ephemeral_pub: sealed.ephemeral_pub,
                        ciphertext: sealed.ciphertext,
                    });
                }
            }
            CryptoEvent::EncryptMessage {
                recipient_id,
                plaintext,
            } => {
                let session_map = self.sessions.entry(recipient_id.clone()).or_default();
                if session_map.is_empty() {
                    actions.push(CryptoAction::Log {
                        level: "WARN".into(),
                        message: format!("No active sessions for {}, fallback to mailbox triggered", recipient_id),
                    });
                    // Logic to trigger sealed packet would go here in the processor
                    return actions;
                }

                for (device_id, session) in session_map.iter_mut() {
                    if let Some(ck) = session.sending_chain_key {
                        let (next_ck, message_key) = Self::kdf_ck(&ck);
                        session.sending_chain_key = Some(next_ck);

                        let mut nonce = [0u8; 12];
                        rand::thread_rng().fill_bytes(&mut nonce[..]);
                        let ciphertext = Self::encrypt(&message_key, &plaintext, &nonce);

                        let packet = MessagePacket {
                            dh_pub: session.dh_ephemeral_public,
                            n: session.send_count,
                            pn: session.prev_send_count,
                            nonce,
                            prev_hash: session.last_sent_hash,
                            sender_device_id: self.identity_id.clone(), // This node acts as its own device here for now
                            ciphertext,
                        };

                        session.last_sent_hash = self.hash_packet(&packet);
                        session.send_count += 1;

                        if let Ok(encoded) = bincode::serialize(&packet) {
                            actions.push(CryptoAction::SendPacket {
                                recipient_id: recipient_id.clone(),
                                packet: encoded,
                            });
                        }
                    }
                }
            }
            CryptoEvent::EncryptSealedMessage {
                recipient_id,
                recipient_public_key,
                plaintext,
            } => {
                use crypto_box::{ChaChaBox, PublicKey, SecretKey};

                let mut rng = rand::thread_rng();
                let mut ephemeral_sk_bytes = [0u8; 32];
                rng.fill_bytes(&mut ephemeral_sk_bytes);
                let ephemeral_sk = SecretKey::from(ephemeral_sk_bytes);
                let ephemeral_pk = PublicKey::from(&ephemeral_sk);

                let recipient_pk = PublicKey::from(recipient_public_key);
                let b = ChaChaBox::new(&recipient_pk, &ephemeral_sk);

                let mut nonce = [0u8; 24];
                rng.fill_bytes(&mut nonce);
                let nonce_val = crypto_box::Nonce::from_slice(&nonce);

                if let Ok(ciphertext_inner) = b.encrypt(nonce_val, plaintext.as_slice()) {
                    let mut ciphertext = Vec::new();
                    ciphertext.extend_from_slice(&nonce);
                    ciphertext.extend_from_slice(&ciphertext_inner);

                    let sealed = SealedPacket {
                        ephemeral_pub: *ephemeral_pk.as_bytes(),
                        ciphertext,
                    };

                    if let Ok(encoded) = bincode::serialize(&sealed) {
                        actions.push(CryptoAction::SendPacket {
                            recipient_id,
                            packet: encoded,
                        });
                    }
                }
            }
            CryptoEvent::PeerDiscovered {
                peer_id,
                device_id,
                device_name: _,
                public_key,
            } => {
                // Check if we already have a session for this device
                let session_map = self.sessions.entry(peer_id.clone()).or_default();
                if !session_map.contains_key(&device_id) {
                    let mut init_actions = self.init_session(peer_id, device_id, public_key);
                    actions.append(&mut init_actions);
                }
            }
        }
        actions
    }

    fn tick(&mut self, _timestamp: u64) -> Vec<CryptoAction> {
        Vec::new()
    }
}
                        // Actually, let's keep it simple: the processor can handle the x25519 work.
                    }
                } else {
                    actions.push(CryptoAction::Log {
                        level: "WARN".into(),
                        message: format!(
                            "No session state for {}, mailbox deposit might be needed",
                            recipient_id
                        ),
                    });
                }
                actions
            }
            CryptoEvent::EncryptSealedMessage {
                recipient_id,
                recipient_public_key,
                plaintext,
            } => {
                let mut actions = Vec::new();
                use crypto_box::{ChaChaBox, PublicKey, SecretKey};

                let mut rng = rand::thread_rng();
                let ephemeral_sk = SecretKey::generate(&mut rng);
                let ephemeral_pk = ephemeral_sk.public_key();

                let recipient_pk = PublicKey::from(recipient_public_key);
                let b = ChaChaBox::new(&recipient_pk, &ephemeral_sk);

                let mut nonce_bytes = [0u8; 24];
                rand::thread_rng().fill_bytes(&mut nonce_bytes[..]);
                let nonce = crypto_box::Nonce::from_slice(&nonce_bytes[..]);

                if let Ok(ciphertext) = b.encrypt(&nonce, plaintext.as_slice()) {
                    let mut full_payload = nonce_bytes.to_vec();
                    full_payload.extend_from_slice(&ciphertext);

                    let sealed = SealedPacket {
                        ephemeral_pub: *ephemeral_pk.as_bytes(),
                        ciphertext: full_payload.clone(),
                    };

                    actions.push(CryptoAction::SendPacket {
                        recipient_id: recipient_id.clone(),
                        packet: bincode::serialize(&sealed).unwrap_or_default(),
                    });

                    actions.push(CryptoAction::DepositMailbox {
                        recipient_id,
                        packet: sealed,
                    });
                }

                actions
            }
            CryptoEvent::PeerDiscovered {
                peer_id,
                public_key,
            } => {
                if !self.sessions.contains_key(&peer_id) {
                    self.init_session(peer_id, public_key)
                } else {
                    vec![]
                }
            }
        }
    }

    fn tick(&mut self, _timestamp: u64) -> Vec<CryptoAction> {
        vec![]
    }
}
