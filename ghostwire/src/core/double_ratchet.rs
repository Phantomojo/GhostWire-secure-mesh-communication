use parking_lot::RwLock;
use rand_core::OsRng;
use rand::Rng;
use rand_core::RngCore;
use ring::hmac;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::debug;
use x25519_dalek::{PublicKey as XPublicKey, StaticSecret};

const MAX_SKIP: u32 = 200;
const KEY_LEN: usize = 32;

#[derive(Clone)]
struct KdfOutput {
    chain_key: [u8; KEY_LEN],
    message_key: [u8; KEY_LEN],
}

fn kdf_chain(chain_key: &[u8; KEY_LEN], input: &[u8]) -> KdfOutput {
    let tag_next = hmac::sign(&hmac::Key::new(hmac::HMAC_SHA256, chain_key), &[0x01]);
    let mut next_chain_key = [0u8; KEY_LEN];
    next_chain_key.copy_from_slice(tag_next.as_ref());

    let mut msg_input = Vec::with_capacity(1 + input.len());
    msg_input.push(0x02);
    msg_input.extend_from_slice(input);
    let tag_msg = hmac::sign(&hmac::Key::new(hmac::HMAC_SHA256, chain_key), &msg_input);
    let mut message_key = [0u8; KEY_LEN];
    message_key.copy_from_slice(tag_msg.as_ref());

    KdfOutput {
        chain_key: next_chain_key,
        message_key,
    }
}

fn kdf_rk(root_key: &[u8; KEY_LEN], dh_output: &[u8]) -> ([u8; KEY_LEN], [u8; KEY_LEN]) {
    // ponytail: HMAC-based KDF matching kdf_chain pattern. Two tagged derivations
    // from the same key material, differentiated by domain separator byte.
    let key = hmac::Key::new(hmac::HMAC_SHA256, root_key);
    let mut rk_input = Vec::with_capacity(1 + dh_output.len());
    rk_input.push(0x01);
    rk_input.extend_from_slice(dh_output);
    let tag_rk = hmac::sign(&key, &rk_input);
    let mut new_root_key = [0u8; KEY_LEN];
    new_root_key.copy_from_slice(tag_rk.as_ref());
    let mut ck_input = Vec::with_capacity(1 + dh_output.len());
    ck_input.push(0x02);
    ck_input.extend_from_slice(dh_output);
    let tag_ck = hmac::sign(&key, &ck_input);
    let mut chain_key = [0u8; KEY_LEN];
    chain_key.copy_from_slice(tag_ck.as_ref());
    (new_root_key, chain_key)
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RatchetState {
    root_key: Vec<u8>,
    sending_chain_key: Vec<u8>,
    receiving_chain_key: Vec<u8>,
    dh_secret_bytes: Vec<u8>,
    dh_public_bytes: Vec<u8>,
    dh_remote_public: Vec<u8>,
    sending_message_number: u32,
    receiving_message_number: u32,
    previous_chain_length: u32,
    skipped_message_keys: Vec<SkippedKey>,
    pub last_ratchet_at: u64,
}

#[derive(Clone, Serialize, Deserialize)]
struct SkippedKey {
    dh_public: Vec<u8>,
    message_number: u32,
    key: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RatchetHeader {
    pub dh_public: Vec<u8>,
    pub previous_chain_length: u32,
    pub message_number: u32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RatchetMessage {
    pub header: RatchetHeader,
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
}

impl RatchetState {
    pub fn init_alice(shared_secret: &[u8], bob_public_key: &[u8; 32]) -> Self {
        let dh_secret = StaticSecret::random_from_rng(OsRng);
        let dh_public = XPublicKey::from(&dh_secret);

        let mut root_key = [0u8; KEY_LEN];
        root_key.copy_from_slice(shared_secret);

        let bob_pub = XPublicKey::from(*bob_public_key);
        let dh_output = dh_secret.diffie_hellman(&bob_pub);
        let (new_root_key, sending_chain_key) = kdf_rk(&root_key, dh_output.as_bytes());

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        RatchetState {
            root_key: new_root_key.to_vec(),
            sending_chain_key: sending_chain_key.to_vec(),
            receiving_chain_key: Vec::new(),
            dh_secret_bytes: dh_secret.as_bytes().to_vec(),
            dh_public_bytes: dh_public.as_bytes().to_vec(),
            dh_remote_public: bob_public_key.to_vec(),
            sending_message_number: 0,
            receiving_message_number: 0,
            previous_chain_length: 0,
            skipped_message_keys: Vec::new(),
            last_ratchet_at: now,
        }
    }

    pub fn init_bob(shared_secret: &[u8], bob_secret: &[u8; 32]) -> Self {
        let mut root_key = [0u8; KEY_LEN];
        root_key.copy_from_slice(shared_secret);

        let sk = StaticSecret::from(*bob_secret);
        let pk = XPublicKey::from(&sk);

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        RatchetState {
            root_key: root_key.to_vec(),
            sending_chain_key: Vec::new(),
            receiving_chain_key: Vec::new(),
            dh_secret_bytes: bob_secret.to_vec(),
            dh_public_bytes: pk.as_bytes().to_vec(),
            dh_remote_public: Vec::new(),
            sending_message_number: 0,
            receiving_message_number: 0,
            previous_chain_length: 0,
            skipped_message_keys: Vec::new(),
            last_ratchet_at: now,
        }
    }

    pub fn encrypt(&mut self, plaintext: &[u8]) -> Result<RatchetMessage, String> {
        if self.sending_chain_key.is_empty() {
            self.generate_dh_keypair();
        }

        let chain_input = self.sending_message_number.to_le_bytes();
        let output = kdf_chain(
            self.sending_chain_key
                .as_slice()
                .try_into()
                .map_err(|_| "Invalid chain key len")?,
            &chain_input,
        );

        self.sending_chain_key = output.chain_key.to_vec();

        use chacha20poly1305::{
            aead::{Aead, KeyInit},
            XChaCha20Poly1305, XNonce,
        };

        let cipher = XChaCha20Poly1305::new(output.message_key.as_slice().into());
        let mut nonce = [0u8; 24];
        OsRng.fill_bytes(&mut nonce);
        let ciphertext = cipher
            .encrypt(XNonce::from_slice(&nonce), plaintext)
            .map_err(|e| format!("Encrypt failed: {}", e))?;

        let header = RatchetHeader {
            dh_public: self.dh_public_bytes.clone(),
            previous_chain_length: self.previous_chain_length,
            message_number: self.sending_message_number,
        };

        self.sending_message_number += 1;

        debug!(msg_num = header.message_number, "Encrypted message");
        Ok(RatchetMessage {
            header,
            ciphertext,
            nonce: nonce.to_vec(),
        })
    }

    pub fn decrypt(&mut self, msg: &RatchetMessage) -> Result<Vec<u8>, String> {
        if msg.header.dh_public != self.dh_remote_public {
            self.dh_ratchet_step(&msg.header.dh_public)?;
        }

        self.try_skip_messages(msg)?;

        let chain_input = msg.header.message_number.to_le_bytes();
        let output = kdf_chain(
            self.receiving_chain_key
                .as_slice()
                .try_into()
                .map_err(|_| "Invalid receiving chain key")?,
            &chain_input,
        );

        self.receiving_chain_key = output.chain_key.to_vec();
        self.receiving_message_number = msg.header.message_number + 1;

        use chacha20poly1305::{
            aead::{Aead, KeyInit},
            XChaCha20Poly1305, XNonce,
        };

        let cipher = XChaCha20Poly1305::new(output.message_key.as_slice().into());
        let plaintext = cipher
            .decrypt(XNonce::from_slice(&msg.nonce), msg.ciphertext.as_slice())
            .map_err(|e| format!("Decrypt failed: {}", e))?;

        debug!(msg_num = msg.header.message_number, "Decrypted message");
        Ok(plaintext)
    }

    fn generate_dh_keypair(&mut self) {
        let dh_secret = StaticSecret::random_from_rng(OsRng);
        let dh_public = XPublicKey::from(&dh_secret);
        self.dh_secret_bytes = dh_secret.as_bytes().to_vec();
        self.dh_public_bytes = dh_public.as_bytes().to_vec();
    }

    fn dh_ratchet_step(&mut self, new_dh_remote: &[u8]) -> Result<(), String> {
        if self.dh_secret_bytes.is_empty() {
            self.generate_dh_keypair();
        }

        let mut secret_bytes = [0u8; 32];
        secret_bytes.copy_from_slice(&self.dh_secret_bytes);
        let our_secret = StaticSecret::from(secret_bytes);

        let mut remote_bytes = [0u8; 32];
        remote_bytes.copy_from_slice(new_dh_remote);
        let remote_pub = XPublicKey::from(remote_bytes);

        let dh_output = our_secret.diffie_hellman(&remote_pub);

        let mut root_key_bytes = [0u8; KEY_LEN];
        root_key_bytes.copy_from_slice(&self.root_key);
        let (new_root_key, receiving_chain_key) = kdf_rk(&root_key_bytes, dh_output.as_bytes());

        self.previous_chain_length = self.sending_message_number;
        self.sending_message_number = 0;
        self.receiving_message_number = 0;

        self.generate_dh_keypair();

        let mut new_secret_bytes = [0u8; 32];
        new_secret_bytes.copy_from_slice(&self.dh_secret_bytes);
        let new_secret = StaticSecret::from(new_secret_bytes);

        let dh_output_send = new_secret.diffie_hellman(&remote_pub);

        let (new_root_key2, sending_chain_key) = kdf_rk(&new_root_key, dh_output_send.as_bytes());

        self.root_key = new_root_key2.to_vec();
        self.receiving_chain_key = receiving_chain_key.to_vec();
        self.sending_chain_key = sending_chain_key.to_vec();
        self.dh_remote_public = new_dh_remote.to_vec();
        self.last_ratchet_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        debug!("DH ratchet step completed");
        Ok(())
    }

    fn try_skip_messages(&mut self, msg: &RatchetMessage) -> Result<(), String> {
        let msg_num = msg.header.message_number;

        if msg_num > self.receiving_message_number {
            let skip_count = msg_num - self.receiving_message_number;
            if skip_count > MAX_SKIP {
                return Err(format!(
                    "Message too far ahead: {} (max skip: {})",
                    skip_count, MAX_SKIP
                ));
            }

            let mut chain_key: [u8; KEY_LEN] = self
                .receiving_chain_key
                .as_slice()
                .try_into()
                .map_err(|_| "Invalid chain key")?;

            for i in 0..skip_count {
                let skip_input = (self.receiving_message_number + i).to_le_bytes();
                let output = kdf_chain(&chain_key, &skip_input);

                self.skipped_message_keys.push(SkippedKey {
                    dh_public: msg.header.dh_public.clone(),
                    message_number: self.receiving_message_number + i,
                    key: output.message_key.to_vec(),
                });

                chain_key = output.chain_key;
            }

            self.receiving_chain_key = chain_key.to_vec();
        }

        if msg_num < self.receiving_message_number {
            let already_skipped = self
                .skipped_message_keys
                .iter()
                .any(|sk| sk.message_number == msg_num && sk.dh_public == msg.header.dh_public);
            if already_skipped {
                return Err("Duplicate message (already decrypted)".into());
            }
            return Err("Message from previous chain, cannot decrypt".into());
        }

        Ok(())
    }

    pub fn our_public_key(&self) -> Vec<u8> {
        self.dh_public_bytes.clone()
    }
}

pub struct RatchetSessionManager {
    sessions: Arc<RwLock<HashMap<String, RatchetState>>>,
}

impl RatchetSessionManager {
    pub fn new() -> Self {
        RatchetSessionManager {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn init_alice(&self, peer_id: &str, shared_secret: &[u8], bob_public_key: &[u8; 32]) {
        let state = RatchetState::init_alice(shared_secret, bob_public_key);
        self.sessions.write().insert(peer_id.to_string(), state);
        debug!(peer_id, "Initialized Alice ratchet session");
    }

    pub fn init_bob(&self, peer_id: &str, shared_secret: &[u8], bob_secret: &[u8; 32]) {
        let state = RatchetState::init_bob(shared_secret, bob_secret);
        self.sessions.write().insert(peer_id.to_string(), state);
        debug!(peer_id, "Initialized Bob ratchet session");
    }

    pub fn encrypt(&self, peer_id: &str, plaintext: &[u8]) -> Result<RatchetMessage, String> {
        let mut sessions = self.sessions.write();
        let state = sessions
            .get_mut(peer_id)
            .ok_or_else(|| format!("No ratchet session for peer: {}", peer_id))?;
        state.encrypt(plaintext)
    }

    pub fn decrypt(&self, peer_id: &str, msg: &RatchetMessage) -> Result<Vec<u8>, String> {
        let mut sessions = self.sessions.write();
        let state = sessions
            .get_mut(peer_id)
            .ok_or_else(|| format!("No ratchet session for peer: {}", peer_id))?;
        state.decrypt(msg)
    }

    pub fn get_our_public_key(&self, peer_id: &str) -> Option<Vec<u8>> {
        let sessions = self.sessions.read();
        sessions.get(peer_id).map(|s| s.our_public_key())
    }

    pub fn has_session(&self, peer_id: &str) -> bool {
        self.sessions.read().contains_key(peer_id)
    }

    pub fn remove_session(&self, peer_id: &str) -> bool {
        self.sessions.write().remove(peer_id).is_some()
    }

    pub fn session_count(&self) -> usize {
        self.sessions.read().len()
    }

    pub fn export_state(&self, peer_id: &str) -> Option<String> {
        let sessions = self.sessions.read();
        sessions
            .get(peer_id)
            .and_then(|s| serde_json::to_string(s).ok())
    }

    pub fn import_state(&self, peer_id: &str, state_json: &str) -> Result<(), String> {
        let state: RatchetState = serde_json::from_str(state_json)
            .map_err(|e| format!("Failed to deserialize ratchet state: {}", e))?;
        self.sessions.write().insert(peer_id.to_string(), state);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kdf_chain_deterministic() {
        let chain_key = [0x42u8; KEY_LEN];
        let input = [0x00u8, 0x00, 0x00, 0x00];
        let out1 = kdf_chain(&chain_key, &input);
        let out2 = kdf_chain(&chain_key, &input);
        assert_eq!(out1.chain_key, out2.chain_key);
        assert_eq!(out1.message_key, out2.message_key);
        assert_ne!(out1.message_key, [0u8; KEY_LEN]);
    }

    #[test]
    fn test_kdf_chain_different_inputs() {
        let chain_key = [0x42u8; KEY_LEN];
        let out1 = kdf_chain(&chain_key, &[0x00, 0x00, 0x00, 0x00]);
        let out2 = kdf_chain(&chain_key, &[0x01, 0x00, 0x00, 0x00]);
        assert_ne!(out1.message_key, out2.message_key);
    }

    #[test]
    fn test_kdf_rk_deterministic() {
        let root_key = [0x11u8; KEY_LEN];
        let dh = [0x22u8; 32];
        let (rk1, ck1) = kdf_rk(&root_key, &dh);
        let (rk2, ck2) = kdf_rk(&root_key, &dh);
        assert_eq!(rk1, rk2);
        assert_eq!(ck1, ck2);
        assert_ne!(rk1, root_key);
    }

    #[test]
    fn test_alice_bob_encrypt_decrypt() {
        let alice_secret = StaticSecret::random_from_rng(OsRng);
        let bob_secret = StaticSecret::random_from_rng(OsRng);
        let bob_pub = XPublicKey::from(&bob_secret);

        let shared = alice_secret.diffie_hellman(&bob_pub);

        let mut alice = RatchetState::init_alice(shared.as_bytes(), bob_pub.as_bytes());
        let mut bob = RatchetState::init_bob(shared.as_bytes(), bob_secret.as_bytes());

        let msg = alice.encrypt(b"Hello, Bob!").unwrap();
        let plaintext = bob.decrypt(&msg).unwrap();
        assert_eq!(plaintext, b"Hello, Bob!");
    }

    #[test]
    fn test_bidirectional_messaging() {
        let alice_secret = StaticSecret::random_from_rng(OsRng);
        let bob_secret = StaticSecret::random_from_rng(OsRng);
        let bob_pub = XPublicKey::from(&bob_secret);

        let shared = alice_secret.diffie_hellman(&bob_pub);

        let mut alice = RatchetState::init_alice(shared.as_bytes(), bob_pub.as_bytes());
        let mut bob = RatchetState::init_bob(shared.as_bytes(), bob_secret.as_bytes());

        let msg1 = alice.encrypt(b"Hello from Alice").unwrap();
        let plain1 = bob.decrypt(&msg1).unwrap();
        assert_eq!(plain1, b"Hello from Alice");

        let msg2 = bob.encrypt(b"Hello from Bob").unwrap();
        let plain2 = alice.decrypt(&msg2).unwrap();
        assert_eq!(plain2, b"Hello from Bob");

        let msg3 = alice.encrypt(b"How are you?").unwrap();
        let plain3 = bob.decrypt(&msg3).unwrap();
        assert_eq!(plain3, b"How are you?");
    }

    #[test]
    fn test_forward_secrecy() {
        let alice_secret = StaticSecret::random_from_rng(OsRng);
        let bob_secret = StaticSecret::random_from_rng(OsRng);
        let bob_pub = XPublicKey::from(&bob_secret);

        let shared = alice_secret.diffie_hellman(&bob_pub);

        let mut alice = RatchetState::init_alice(shared.as_bytes(), bob_pub.as_bytes());
        let mut bob = RatchetState::init_bob(shared.as_bytes(), bob_secret.as_bytes());

        let msg1 = alice.encrypt(b"Message 1").unwrap();
        let pre_chain = alice.sending_chain_key.clone();
        let _ = bob.decrypt(&msg1).unwrap();

        let msg2 = bob.encrypt(b"Response 1").unwrap();
        let _ = alice.decrypt(&msg2).unwrap();

        let msg3 = alice.encrypt(b"Message 2").unwrap();
        assert_ne!(pre_chain, alice.sending_chain_key);
        assert_ne!(msg1.ciphertext, msg3.ciphertext);
    }

    #[test]
    fn test_session_manager() {
        let mgr = RatchetSessionManager::new();

        let alice_secret = StaticSecret::random_from_rng(OsRng);
        let bob_secret = StaticSecret::random_from_rng(OsRng);
        let bob_pub = XPublicKey::from(&bob_secret);

        let shared = alice_secret.diffie_hellman(&bob_pub);

        mgr.init_alice("bob", shared.as_bytes(), bob_pub.as_bytes());

        assert!(mgr.has_session("bob"));
        assert!(!mgr.has_session("alice"));

        let msg = mgr.encrypt("bob", b"test message").unwrap();
        assert!(!msg.ciphertext.is_empty());
        assert_eq!(msg.header.message_number, 0);

        assert!(mgr.remove_session("bob"));
        assert!(!mgr.has_session("bob"));
        assert_eq!(mgr.session_count(), 0);
    }

    #[test]
    fn test_state_export_import() {
        let mgr = RatchetSessionManager::new();

        let alice_secret = StaticSecret::random_from_rng(OsRng);
        let bob_secret = StaticSecret::random_from_rng(OsRng);
        let bob_pub = XPublicKey::from(&bob_secret);

        let shared = alice_secret.diffie_hellman(&bob_pub);
        mgr.init_alice("bob", shared.as_bytes(), bob_pub.as_bytes());

        let exported = mgr.export_state("bob").unwrap();
        assert!(!exported.is_empty());

        let mgr2 = RatchetSessionManager::new();
        mgr2.import_state("bob", &exported).unwrap();

        assert!(mgr2.has_session("bob"));
        let msg = mgr2.encrypt("bob", b"after import").unwrap();
        assert!(!msg.ciphertext.is_empty());
    }

    #[test]
    fn test_duplicate_message_rejected() {
        let alice_secret = StaticSecret::random_from_rng(OsRng);
        let bob_secret = StaticSecret::random_from_rng(OsRng);
        let bob_pub = XPublicKey::from(&bob_secret);

        let shared = alice_secret.diffie_hellman(&bob_pub);

        let mut alice = RatchetState::init_alice(shared.as_bytes(), bob_pub.as_bytes());
        let mut bob = RatchetState::init_bob(shared.as_bytes(), bob_secret.as_bytes());

        let msg = alice.encrypt(b"test").unwrap();
        let _ = bob.decrypt(&msg).unwrap();

        let result = bob.decrypt(&msg);
        assert!(result.is_err());
    }
}
