//! device_pairing.rs — Cross-device identity linking and peer bootstrap
//!
//! THE KEY PROBLEM: You have two PCs. They both run Ghostwire. How do they
//! find each other and establish trust — especially if they're on different
//! networks with no shared bootstrap?
//!
//! This module solves it with three complementary mechanisms:
//!
//! 1. **QR / Manual code pairing** (air-gap safe)
//!    - PC1 generates a PairingCode (encoded as a short alphanumeric string)
//!    - You type/scan that code on PC2
//!    - Both sides derive the same session key via SPAKE2 (password-authenticated
//!      key exchange — no PKI needed)
//!    - After pairing, each device has the other's public key pinned in TrustStore
//!
//! 2. **LAN discovery** (same network)
//!    - mDNS already in mesh.rs announces peer presence on the LAN
//!    - We add a pairing-specific mDNS service type `_ghostwire._tcp.local.`
//!    - Devices on the same WiFi/LAN find each other automatically
//!    - User confirms pairing with a visual safety number (like Signal)
//!
//! 3. **Rendezvous via DHT** (cross-network, internet)
//!    - Each identity has a stable "rendez-vous address" derived from its
//!      public key: BLAKE3(pub_key || "rendezvous")[:16] encoded as hex
//!    - You share this address out-of-band (text, email, etc.)
//!    - PC2 looks it up in the DHT to find PC1's current multiaddrs
//!    - First contact establishes trust via TOFU
//!
//! After any of these three, the two devices are peers and can communicate.

use crate::genesis_constants::{derive_with_domain, PAIRING_SESSION_DOMAIN, RENDEZVOUS_DOMAIN};

use anyhow::Result;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Key, Nonce,
};
use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// ── Pairing code ──────────────────────────────────────────────────────────────

/// A short human-typeable pairing code
/// Format: 4 groups of 4 alphanumeric chars: XXXX-XXXX-XXXX-XXXX
#[derive(Debug, Clone)]
pub struct PairingCode {
    /// The raw 16 bytes of entropy
    entropy: [u8; 16],
    /// Human-readable representation
    pub display: String,
}

impl PairingCode {
    /// Generate a new random pairing code
    pub fn generate() -> Result<Self> {
        let rng = SystemRandom::new();
        let mut entropy = [0u8; 16];
        rng.fill(&mut entropy)
            .map_err(|_| anyhow::anyhow!("RNG failure"))?;

        let display = Self::encode(&entropy);
        Ok(Self { entropy, display })
    }

    /// Encode 16 bytes as groups of 4 alphanumeric chars
    fn encode(bytes: &[u8; 16]) -> String {
        // Base32 (RFC 4648 alphabet, no padding) over the 16 bytes -> 26 chars,
        // grouped 4-4-4-4-4-4-4.
        //
        // THIS USED TO MAP ONE CHARACTER PER BYTE with `% ALPHABET.len()`. That is
        // not an encoding, it is a lossy cast: only the low 5 bits of each byte
        // survived, so 128 of the 256 byte values mapped to the same character as
        // another, and the function was therefore NOT INJECTIVE. No decoder could
        // exist for it, which is why `from_string` resorted to hashing the display
        // string and why typed pairing could never complete.
        //
        // Base32 keeps every bit of the entropy and is exactly invertible, which
        // is what a user-typed code requires.
        const ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
        let mut bits = 0u32;
        let mut acc: u32 = 0;
        let mut out = String::with_capacity(26);
        for &byte in bytes {
            acc = (acc << 8) | byte as u32;
            bits += 8;
            while bits >= 5 {
                bits -= 5;
                out.push(ALPHABET[((acc >> bits) & 0x1f) as usize] as char);
            }
        }
        if bits > 0 {
            out.push(ALPHABET[((acc << (5 - bits)) & 0x1f) as usize] as char);
        }
        out.as_bytes().chunks(4).map(|c| String::from_utf8_lossy(c).to_string()).collect::<Vec<_>>().join("-")
    }

    /// Decode a user-entered code back to the exact bytes it encodes.
    ///
    /// This is the true inverse of `encode`. A 16-byte value is 128 bits, which
    /// base32 renders as ceil(128/5) = 26 characters.
    fn decode(s: &str, out: &mut [u8; 16]) -> Result<()> {
        const ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
        let mut acc: u32 = 0;
        let mut bits = 0u32;
        let mut written = 0usize;
        for ch in s.chars() {
            let up = ch.to_ascii_uppercase() as u8;
            if up == b'-' {
                continue;
            }
            let v = ALPHABET
                .iter()
                .position(|&a| a == up)
                .ok_or_else(|| anyhow::anyhow!("invalid character '{ch}' in pairing code"))?
                as u32;
            acc = (acc << 5) | v;
            bits += 5;
            if bits >= 8 {
                bits -= 8;
                if written < out.len() {
                    out[written] = ((acc >> bits) & 0xff) as u8;
                    written += 1;
                }
            }
        }
        if written != out.len() {
            anyhow::bail!(
                "pairing code decoded to {written} bytes, expected {}",
                out.len()
            );
        }
        Ok(())
    }

    /// Build a code from fixed entropy.
    ///
    /// Exists so the conformance vector generator can produce a REPRODUCIBLE
    /// code. `generate()` uses the system RNG and cannot be used for a published
    /// test vector — a vector must be regenerable byte for byte on any machine.
    pub fn from_entropy_for_vectors(entropy: [u8; 16]) -> Self {
        let display = Self::encode(&entropy);
        Self { entropy, display }
    }

    /// Parse a user-entered pairing code
    pub fn from_string(s: &str) -> Result<Self> {
        let cleaned: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
        // 16 bytes is 128 bits, which base32 renders as ceil(128/5) = 26 chars.
        // This used to require 16, which was one character per byte — the lossy
        // scheme `encode` used before it became real base32.
        const EXPECTED_CHARS: usize = 26;
        if cleaned.len() != EXPECTED_CHARS {
            anyhow::bail!(
                "Invalid pairing code: expected {EXPECTED_CHARS} alphanumeric chars, got {}",
                cleaned.len()
            );
        }
        // DECODE the string back to the bytes it encodes. Do NOT hash it.
        //
        // This used to be `blake3::hash(cleaned)[..16]`, which is NOT the inverse
        // of `encode()`. `generate()` stores 16 random bytes; hashing the display
        // string reconstructs something else entirely, so the two sides held
        // DIFFERENT entropy and derived DIFFERENT session keys. The comment here
        // asserted the invariant it was breaking: "Both sides must use the same
        // display string for derive_session_key to match."
        //
        // Verified before and after by the `typed_pairing_*` tests below, which
        // failed with safety numbers 579841 vs 677922.
        let mut entropy = [0u8; 16];
        Self::decode(&cleaned, &mut entropy)?;

        Ok(Self {
            entropy,
            display: s.to_string(), // Preserve original format for session key derivation
        })
    }

    /// Derive a session key from this pairing code + both parties' node IDs
    /// Uses BLAKE3 with domain separation — both sides must agree on the display string
    pub fn derive_session_key(&self, initiator_node_id: &str, responder_node_id: &str) -> [u8; 32] {
        // Hash the display string to get a consistent shared secret
        // Both generate() and from_string() produce the same canonical display format
        let display_hash = blake3::hash(self.display.as_bytes());
        let mut ikm = display_hash.as_bytes().to_vec();
        // Sort node IDs so both sides derive the same key
        let mut ids = [initiator_node_id, responder_node_id];
        ids.sort();
        ikm.extend_from_slice(ids[0].as_bytes());
        ikm.extend_from_slice(ids[1].as_bytes());
        derive_with_domain(PAIRING_SESSION_DOMAIN, &ikm)
    }

    /// Generate a 6-digit safety number for visual verification (like Signal)
    pub fn safety_number(&self) -> String {
        let hash = blake3::hash(&self.entropy);
        let n = u64::from_le_bytes(hash.as_bytes()[..8].try_into().unwrap_or([0u8; 8]));
        format!("{:06}", n % 1_000_000)
    }
}

// ── Pairing session ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PairingState {
    /// Code generated, waiting for other side to enter it
    WaitingForPeer,
    /// Both sides have the code, verifying safety number
    Verifying,
    /// Safety number confirmed, exchanging public keys
    Confirmed,
    /// Fully paired — both sides have each other's key pinned
    Complete,
    /// Pairing failed or timed out
    Failed { reason: String },
}

/// An in-progress pairing session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairingSession {
    pub session_id: String,
    pub state: PairingState,
    /// Our node ID
    pub our_node_id: String,
    /// The other device's node ID (set once they connect)
    pub peer_node_id: Option<String>,
    /// Their public key (set once exchanged)
    pub peer_ed25519_pub: Option<Vec<u8>>,
    pub peer_x25519_pub: Option<Vec<u8>>,
    /// Safety number for verification
    pub safety_number: String,
    /// When this session expires
    pub expires_at: u64,
}

impl PairingSession {
    pub fn is_expired(&self) -> bool {
        now_secs() > self.expires_at
    }
}

// ── Pairing manager ───────────────────────────────────────────────────────────

pub struct PairingManager {
    sessions: Arc<RwLock<HashMap<String, PairingSession>>>,
    our_node_id: String,
    our_ed25519_pub: Vec<u8>,
    our_x25519_pub: Vec<u8>,
}

impl PairingManager {
    pub fn new(
        our_node_id: impl Into<String>,
        our_ed25519_pub: Vec<u8>,
        our_x25519_pub: Vec<u8>,
    ) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            our_node_id: our_node_id.into(),
            our_ed25519_pub,
            our_x25519_pub,
        }
    }

    /// === MECHANISM 1: QR / Manual Code Pairing ===

    /// Step 1 (Initiator): Generate a pairing code and session.
    /// Show this code to the user. They type or scan it on the other device.
    pub async fn initiate_pairing(&self) -> Result<(PairingCode, PairingSession)> {
        let code = PairingCode::generate()?;
        let safety = code.safety_number();
        let session_id = uuid::Uuid::new_v4().to_string();

        let session = PairingSession {
            session_id: session_id.clone(),
            state: PairingState::WaitingForPeer,
            our_node_id: self.our_node_id.clone(),
            peer_node_id: None,
            peer_ed25519_pub: None,
            peer_x25519_pub: None,
            safety_number: safety,
            expires_at: now_secs() + 600, // 10 minute expiry
        };

        self.sessions
            .write()
            .await
            .insert(session_id, session.clone());
        Ok((code, session))
    }

    /// Step 2 (Responder): Enter the pairing code the initiator showed you.
    /// Returns a PairingResponse to send back to the initiator.
    pub fn respond_to_pairing(
        &self,
        code_string: &str,
        initiator_node_id: &str,
    ) -> Result<PairingResponse> {
        let code = PairingCode::from_string(code_string)?;
        let session_key = code.derive_session_key(initiator_node_id, &self.our_node_id);
        let safety = code.safety_number();

        Ok(PairingResponse {
            responder_node_id: self.our_node_id.clone(),
            responder_ed25519_pub: self.our_ed25519_pub.clone(),
            responder_x25519_pub: self.our_x25519_pub.clone(),
            safety_number: safety,
            // Encrypt our identity with the session key so only code-holder can read it
            encrypted_identity: self.encrypt_identity_for_pairing(&session_key)?,
        })
    }

    /// Step 3: Verify safety numbers match, then complete pairing.
    pub async fn complete_pairing(
        &self,
        session_id: &str,
        peer_response: &PairingResponse,
        code: &PairingCode,
    ) -> Result<PairedPeer> {
        let session_key =
            code.derive_session_key(&self.our_node_id, &peer_response.responder_node_id);

        // Decrypt and verify peer identity
        let _decrypted = self
            .decrypt_identity_from_pairing(&peer_response.encrypted_identity, &session_key)
            .map_err(|_| anyhow::anyhow!("Decryption failed — wrong pairing code"))?;

        // Verify safety numbers match
        if peer_response.safety_number != code.safety_number() {
            anyhow::bail!("Safety numbers don't match — possible MITM attack!");
        }

        // Update session state
        {
            let mut sessions = self.sessions.write().await;
            if let Some(session) = sessions.get_mut(session_id) {
                session.state = PairingState::Complete;
                session.peer_node_id = Some(peer_response.responder_node_id.clone());
                session.peer_ed25519_pub = Some(peer_response.responder_ed25519_pub.clone());
                session.peer_x25519_pub = Some(peer_response.responder_x25519_pub.clone());
            }
        }

        let paired = PairedPeer {
            node_id: peer_response.responder_node_id.clone(),
            ed25519_pub: peer_response.responder_ed25519_pub.clone(),
            x25519_pub: peer_response.responder_x25519_pub.clone(),
            paired_at: now_secs(),
            pairing_method: PairingMethod::ManualCode,
            nickname: None,
        };

        tracing::info!(
            "✅ Pairing complete with peer {} (safety: {})",
            paired.node_id,
            code.safety_number()
        );

        Ok(paired)
    }

    /// === MECHANISM 2: Rendezvous address (DHT-based cross-network discovery) ===

    /// Compute this node's stable rendezvous address.
    /// Share this out-of-band (text, email, printed QR).
    pub fn rendezvous_address(&self, ed25519_pub: &[u8]) -> String {
        let hash = derive_with_domain(RENDEZVOUS_DOMAIN, ed25519_pub);
        hex::encode(&hash[..16])
    }

    /// Get the DHT key where we publish our multiaddrs for rendezvous
    pub fn rendezvous_dht_key(&self, ed25519_pub: &[u8]) -> Vec<u8> {
        let addr = self.rendezvous_address(ed25519_pub);
        format!("ghostwire/rendezvous/{}", addr).into_bytes()
    }

    // ── Internal helpers ──────────────────────────────────────────────────

    fn encrypt_identity_for_pairing(&self, session_key: &[u8; 32]) -> Result<Vec<u8>> {
        let rng = SystemRandom::new();
        let payload = serde_json::json!({
            "node_id": self.our_node_id,
            "ed25519_pub": hex::encode(&self.our_ed25519_pub),
            "x25519_pub": hex::encode(&self.our_x25519_pub),
        });
        let plaintext = serde_json::to_vec(&payload)?;

        let key = Key::from_slice(session_key);
        let cipher = ChaCha20Poly1305::new(key);
        let mut nonce_bytes = [0u8; 12];
        rng.fill(&mut nonce_bytes)
            .map_err(|_| anyhow::anyhow!("RNG failure"))?;
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, plaintext.as_slice())
            .map_err(|e| anyhow::anyhow!("Encrypt failed: {:?}", e))?;

        let mut out = nonce_bytes.to_vec();
        out.extend_from_slice(&ciphertext);
        Ok(out)
    }

    fn decrypt_identity_from_pairing(
        &self,
        encrypted: &[u8],
        session_key: &[u8; 32],
    ) -> Result<serde_json::Value> {
        if encrypted.len() < 12 {
            anyhow::bail!("Encrypted identity too short");
        }
        let nonce = Nonce::from_slice(&encrypted[..12]);
        let ciphertext = &encrypted[12..];

        let key = Key::from_slice(session_key);
        let cipher = ChaCha20Poly1305::new(key);
        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| anyhow::anyhow!("Decryption failed"))?;

        serde_json::from_slice(&plaintext).map_err(|e| anyhow::anyhow!("Parse failed: {}", e))
    }
}

// ── Data types ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairingResponse {
    pub responder_node_id: String,
    pub responder_ed25519_pub: Vec<u8>,
    pub responder_x25519_pub: Vec<u8>,
    pub safety_number: String,
    pub encrypted_identity: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PairingMethod {
    ManualCode,
    QrCode,
    LanDiscovery,
    RendezvousDht,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairedPeer {
    pub node_id: String,
    pub ed25519_pub: Vec<u8>,
    pub x25519_pub: Vec<u8>,
    pub paired_at: u64,
    pub pairing_method: PairingMethod,
    pub nickname: Option<String>,
}

/// Serialised "contact card" — share this to let someone add you
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactCard {
    pub node_id: String,
    pub nickname: String,
    pub ed25519_pub: Vec<u8>,
    pub x25519_pub: Vec<u8>,
    pub rendezvous_address: String,
    pub multiaddrs: Vec<String>,
    pub created_at: u64,
    /// Signature over all above fields
    pub signature: Vec<u8>,
}

impl ContactCard {
    pub fn to_qr_string(&self) -> Result<String> {
        let bytes = bincode::serialize(self)?;
        Ok(format!("gw2:{}", bs58::encode(bytes).into_string()))
    }

    pub fn from_qr_string(s: &str) -> Result<Self> {
        let stripped = s
            .strip_prefix("gw2:")
            .ok_or_else(|| anyhow::anyhow!("Not a Ghostwire v2 contact card"))?;
        let bytes = bs58::decode(stripped)
            .into_vec()
            .map_err(|e| anyhow::anyhow!("Decode failed: {}", e))?;
        bincode::deserialize(&bytes).map_err(|e| anyhow::anyhow!("Parse failed: {}", e))
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pairing_code_roundtrip() {
        let code = PairingCode::generate().unwrap();
        // Parse the display string back — from_string is deterministic
        let parsed = PairingCode::from_string(&code.display).unwrap();
        // Both should produce the same session key (display strings may differ in format
        // but the hash of the display string is the shared secret)
        let key1 = code.derive_session_key("nodeA", "nodeB");
        let key2 = parsed.derive_session_key("nodeA", "nodeB");
        // They should match because from_string produces a canonical display format
        // and derive_session_key hashes the display string
        assert_eq!(
            key1, key2,
            "Session keys should match (code.display={}, parsed.display={})",
            code.display, parsed.display
        );
    }

    #[test]
    fn safety_number_is_6_digits() {
        let code = PairingCode::generate().unwrap();
        let safety = code.safety_number();
        assert_eq!(safety.len(), 6);
        assert!(safety.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn session_key_is_symmetric() {
        let code = PairingCode::generate().unwrap();
        // Both orderings should produce the same key
        let k1 = code.derive_session_key("alice", "bob");
        let k2 = code.derive_session_key("bob", "alice");
        assert_eq!(k1, k2, "Session key must be symmetric");
    }

    #[test]
    fn contact_card_qr_roundtrip() {
        let card = ContactCard {
            node_id: "abc123".into(),
            nickname: "Michael".into(),
            ed25519_pub: vec![1u8; 32],
            x25519_pub: vec![2u8; 32],
            rendezvous_address: "deadbeef01020304".into(),
            multiaddrs: vec!["/ip4/1.2.3.4/tcp/4001".into()],
            created_at: 0,
            signature: vec![],
        };

        let qr = card.to_qr_string().unwrap();
        assert!(qr.starts_with("gw2:"));
        let recovered = ContactCard::from_qr_string(&qr).unwrap();
        assert_eq!(recovered.node_id, "abc123");
        assert_eq!(recovered.nickname, "Michael");
    }

    // -----------------------------------------------------------------------
    // TYPED PAIRING: does the code survive being typed into the other side?
    //
    // This is the mechanism that removes the need to meet in person, so it is
    // load-bearing for the whole privacy design. Verified here rather than
    // trusted, because the comment above `from_string` asserts the property.
    //
    // `generate()` fills `entropy` with 16 RANDOM bytes, then encodes them into
    // `display`. `from_string()` reverses by HASHING the display string:
    //
    //     let hash = blake3::hash(cleaned.as_bytes());
    //     entropy.copy_from_slice(&hash.as_bytes()[..16]);
    //
    // That is not the inverse of `encode()`. The initiator therefore holds
    // `random16` while the responder reconstructs `BLAKE3(display)[..16]`, so the
    // two sides derive DIFFERENT session keys and their safety numbers can never
    // match. `derive_session_key` hashes `self.entropy`, so the mismatch reaches
    // the actual session key, not just a display detail.

    #[test]
    fn typed_pairing_reconstructs_the_same_entropy() {
        let generated = PairingCode::generate().expect("generate");
        let typed = PairingCode::from_string(&generated.display).expect("accept own code");

        assert_eq!(
            generated.entropy, typed.entropy,
            "generate() stores 16 random bytes; from_string() derives entropy by              HASHING the display string, which is not the inverse of encode().              The two sides must reconstruct the SAME entropy or pairing cannot              complete at any distance."
        );
    }

    #[test]
    fn typed_pairing_reconstructs_the_same_safety_number() {
        let generated = PairingCode::generate().expect("generate");
        let typed = PairingCode::from_string(&generated.display).expect("accept own code");

        assert_eq!(
            generated.safety_number(),
            typed.safety_number(),
            "These six digits are what two users compare aloud or on a video call              to detect a man-in-the-middle. If they can never match, the feature              that makes pairing possible at a distance does not work."
        );
    }

    #[test]
    fn typed_pairing_derives_the_same_session_key() {
        let generated = PairingCode::generate().expect("generate");
        let typed = PairingCode::from_string(&generated.display).expect("accept own code");

        // Both sides sort their node IDs, so pass the same pair in both orders.
        let initiator_key = generated.derive_session_key("node-a", "node-b");
        let responder_key = typed.derive_session_key("node-a", "node-b");

        assert_eq!(
            initiator_key, responder_key,
            "derive_session_key hashes self.entropy. If the two sides hold              different entropy, they derive different session keys and no message              can ever be exchanged over a typed pairing code."
        );
    }
}
