//! anonymous_intro.rs — SURB-based anonymous contact establishment
//!
//! The problem: standard contact exchange (QR scan, invite link) leaks
//! that two specific identities are establishing a relationship. Any DHT
//! node watching the rendezvous can map the social graph.
//!
//! The fix: Alice embeds a SURB in her QR code. Bob sends his introduction
//! through the SURB — it routes anonymously back to Alice via the Sphinx
//! mixnet. Bob never learns Alice's node address until Alice explicitly
//! responds. The DHT sees nothing about either identity.
//!
//! Flow:
//!   1. Alice calls generate_intro_invite() → gets SURB + her public key
//!   2. Alice encodes this in a QR code: "gwi:<b58(surb_bytes)>:<b58(alice_pubkey)>"
//!   3. Bob scans QR, calls send_introduction(invite, bob_pubkey, bob_nodekey)
//!   4. Bob's introduction routes anonymously through the SURB to Alice
//!   5. Alice calls receive_introduction(payload) → gets Bob's public key
//!   6. Alice initiates vodozemac session → sends first message with her node info
//!   7. Normal Double Ratchet from here
//!
//! Additionally: time-windowed rendezvous DHT keys prevent long-term tracking.
//! The rendezvous key rotates every 24 hours:
//!   key = BLAKE3_keyed(RENDEZVOUS_DOMAIN, pubkey || floor(unix_ts / 86400))

use anyhow::Result;
use rand_core::OsRng;
use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};

use crate::genesis_constants::{genesis_derive, RENDEZVOUS_DOMAIN};

/// The invite prefix for anonymous introductions
pub const ANON_INVITE_PREFIX: &str = "gwi:";

/// An anonymous introduction invite — encoded into a QR code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnonymousInvite {
    /// Alice's Ed25519 public key (for the recipient to encrypt their intro to)
    pub ed25519_pub: [u8; 32],
    /// Alice's X25519 public key (for session establishment)
    pub x25519_pub: [u8; 32],
    /// A pre-built SURB for routing the introduction back to Alice
    pub surb_bytes: Vec<u8>,
    /// Alice's optional nickname (shown during contact request UI)
    pub nickname: Option<String>,
    /// Creation timestamp (invite expires after 48 hours)
    pub created_at: u64,
}

impl AnonymousInvite {
    /// Encode to a shareable string: "gwi:<base58(serialised_invite)>"
    pub fn to_invite_string(&self) -> Result<String> {
        let bytes =
            bincode::serialize(self).map_err(|e| anyhow::anyhow!("Invite serialise: {}", e))?;
        Ok(format!(
            "{}{}",
            ANON_INVITE_PREFIX,
            bs58::encode(bytes).into_string()
        ))
    }

    /// Decode from an invite string
    pub fn from_invite_string(s: &str) -> Result<Self> {
        let stripped = s
            .strip_prefix(ANON_INVITE_PREFIX)
            .ok_or_else(|| anyhow::anyhow!("Not a Ghostwire anonymous invite"))?;
        let bytes = bs58::decode(stripped)
            .into_vec()
            .map_err(|e| anyhow::anyhow!("Base58 decode: {}", e))?;
        bincode::deserialize(&bytes).map_err(|e| anyhow::anyhow!("Invite deserialise: {}", e))
    }

    /// Whether this invite has expired (48-hour window)
    pub fn is_expired(&self) -> bool {
        now_secs() > self.created_at + (48 * 3600)
    }
}

/// The introduction payload that Bob sends through the SURB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntroductionPayload {
    /// Bob's Ed25519 public key
    pub ed25519_pub: [u8; 32],
    /// Bob's X25519 public key (for session establishment)
    pub x25519_pub: [u8; 32],
    /// Bob's optional nickname
    pub nickname: Option<String>,
    /// A nonce to prevent replay (random 16 bytes)
    pub nonce: [u8; 16],
    /// Timestamp
    pub created_at: u64,
}

/// Generate an anonymous introduction invite for this node.
/// The invite contains a SURB that routes introductions back to us.
///
/// `route_keys` — X25519 public keys of Sphinx relay nodes for the SURB path
/// `route_ids`  — peer IDs for the SURB relay nodes
/// `our_ed25519_pub` — our Ed25519 public key
/// `our_x25519_pub`  — our X25519 public key  
/// `nickname`   — optional display name
pub fn generate_intro_invite(
    route_keys: &[[u8; 32]],
    route_ids: &[String],
    our_ed25519_pub: [u8; 32],
    our_x25519_pub: [u8; 32],
    nickname: Option<String>,
) -> Result<AnonymousInvite> {
    use crate::core::routing::sphinx::{create_surb, SPHINX_MIN_HOPS};

    if route_keys.len() < SPHINX_MIN_HOPS {
        anyhow::bail!(
            "Anonymous invite requires at least {} relay hops for anonymity",
            SPHINX_MIN_HOPS
        );
    }

    let rng = SystemRandom::new();
    let surb = create_surb(route_keys, route_ids, &rng)?;
    let surb_bytes =
        bincode::serialize(&surb).map_err(|e| anyhow::anyhow!("SURB serialise: {}", e))?;

    Ok(AnonymousInvite {
        ed25519_pub: our_ed25519_pub,
        x25519_pub: our_x25519_pub,
        surb_bytes,
        nickname,
        created_at: now_secs(),
    })
}

/// Bob calls this to send an introduction through Alice's SURB.
/// Returns the Sphinx packet to send to `surb.first_hop`.
pub fn send_introduction(
    invite: &AnonymousInvite,
    our_ed25519_pub: [u8; 32],
    our_x25519_pub: [u8; 32],
    our_nickname: Option<String>,
) -> Result<(Vec<u8>, String)> {
    use crate::core::routing::sphinx::{use_surb, Surb};

    if invite.is_expired() {
        anyhow::bail!("Invite has expired (> 48 hours old)");
    }

    let surb: Surb = bincode::deserialize(&invite.surb_bytes)
        .map_err(|e| anyhow::anyhow!("SURB deserialise: {}", e))?;

    let mut nonce = [0u8; 16];
    ring::rand::SystemRandom::new()
        .fill(&mut nonce)
        .map_err(|_| anyhow::anyhow!("RNG failure"))?;

    let intro = IntroductionPayload {
        ed25519_pub: our_ed25519_pub,
        x25519_pub: our_x25519_pub,
        nickname: our_nickname,
        nonce,
        created_at: now_secs(),
    };

    let intro_bytes =
        bincode::serialize(&intro).map_err(|e| anyhow::anyhow!("Intro serialise: {}", e))?;

    // Encrypt introduction to Alice's X25519 key (ECIES)
    // so only Alice can read it even if a relay peeks
    let sealed = ecies_seal(&invite.x25519_pub, &intro_bytes)?;
    let sealed_bytes =
        bincode::serialize(&sealed).map_err(|e| anyhow::anyhow!("Sealed serialise: {}", e))?;

    let rng = SystemRandom::new();
    let sphinx_packet = use_surb(&surb, &sealed_bytes, &rng)?;
    let packet_bytes = bincode::serialize(&sphinx_packet)
        .map_err(|e| anyhow::anyhow!("Packet serialise: {}", e))?;

    Ok((packet_bytes, surb.first_hop))
}

/// Alice calls this when she receives an introduction through her SURB.
pub fn receive_introduction(
    payload: &[u8],
    our_x25519_secret: &x25519_dalek::StaticSecret,
) -> Result<IntroductionPayload> {
    // Decrypt the ECIES sealed payload
    let sealed: EciesSealedPayload =
        bincode::deserialize(payload).map_err(|e| anyhow::anyhow!("Sealed deserialise: {}", e))?;

    let intro_bytes = ecies_unseal(&sealed, our_x25519_secret)?;

    let intro: IntroductionPayload = bincode::deserialize(&intro_bytes)
        .map_err(|e| anyhow::anyhow!("Intro deserialise: {}", e))?;

    // Basic replay protection: reject introductions older than 24 hours
    if now_secs().saturating_sub(intro.created_at) > 86400 {
        anyhow::bail!("Introduction payload is too old (> 24 hours)");
    }

    Ok(intro)
}

/// Time-windowed rendezvous DHT key — rotates every 24 hours
/// Prevents long-term tracking via static rendezvous addresses
///
/// key = BLAKE3_keyed(RENDEZVOUS_DOMAIN, ed25519_pub || day_number)
pub fn windowed_rendezvous_key(ed25519_pub: &[u8; 32]) -> [u8; 32] {
    let day = now_secs() / 86400; // Unix day number
    let mut ikm = ed25519_pub.to_vec();
    ikm.extend_from_slice(&day.to_le_bytes());
    genesis_derive(RENDEZVOUS_DOMAIN, &ikm)
}

/// Generate the rendezvous key for the previous day (so contacts
/// connecting from the previous day window can still find us during
/// the transition period)
pub fn windowed_rendezvous_key_prev(ed25519_pub: &[u8; 32]) -> [u8; 32] {
    let day = (now_secs() / 86400).saturating_sub(1);
    let mut ikm = ed25519_pub.to_vec();
    ikm.extend_from_slice(&day.to_le_bytes());
    genesis_derive(RENDEZVOUS_DOMAIN, &ikm)
}

// ── ECIES sealed payload (minimal) ───────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EciesSealedPayload {
    ephemeral_pub: [u8; 32],
    nonce: [u8; 12],
    ciphertext: Vec<u8>,
}

fn ecies_seal(recipient_x25519_pub: &[u8; 32], plaintext: &[u8]) -> Result<EciesSealedPayload> {
    use crate::genesis_constants::{derive_with_domain, SEALED_KDF_DOMAIN};
    use chacha20poly1305::{
        aead::{Aead, KeyInit},
        ChaCha20Poly1305, Key, Nonce,
    };

    let mut ephemeral_bytes = [0u8; 32];
    ring::rand::SystemRandom::new()
        .fill(&mut ephemeral_bytes)
        .map_err(|_| anyhow::anyhow!("RNG failure"))?;
    ephemeral_bytes[0] &= 248;
    ephemeral_bytes[31] &= 127;
    ephemeral_bytes[31] |= 64;

    let ephemeral_secret = x25519_dalek::StaticSecret::from(ephemeral_bytes);
    let ephemeral_pub = x25519_dalek::PublicKey::from(&ephemeral_secret);
    let recipient_key = x25519_dalek::PublicKey::from(*recipient_x25519_pub);
    let shared = ephemeral_secret.diffie_hellman(&recipient_key);

    let sym_key = derive_with_domain(SEALED_KDF_DOMAIN, shared.as_bytes());
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&sym_key));

    let mut nonce_bytes = [0u8; 12];
    ring::rand::SystemRandom::new()
        .fill(&mut nonce_bytes)
        .map_err(|_| anyhow::anyhow!("RNG failure"))?;
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|_| anyhow::anyhow!("ECIES encrypt failed"))?;

    Ok(EciesSealedPayload {
        ephemeral_pub: *ephemeral_pub.as_bytes(),
        nonce: nonce_bytes,
        ciphertext,
    })
}

fn ecies_unseal(
    sealed: &EciesSealedPayload,
    our_secret: &x25519_dalek::StaticSecret,
) -> Result<Vec<u8>> {
    use crate::genesis_constants::{derive_with_domain, SEALED_KDF_DOMAIN};
    use chacha20poly1305::{
        aead::{Aead, KeyInit},
        ChaCha20Poly1305, Key, Nonce,
    };

    let sender_pub = x25519_dalek::PublicKey::from(sealed.ephemeral_pub);
    let shared = our_secret.diffie_hellman(&sender_pub);
    let sym_key = derive_with_domain(SEALED_KDF_DOMAIN, shared.as_bytes());

    let cipher = ChaCha20Poly1305::new(Key::from_slice(&sym_key));
    let nonce = Nonce::from_slice(&sealed.nonce);

    cipher
        .decrypt(nonce, sealed.ciphertext.as_slice())
        .map_err(|_| anyhow::anyhow!("ECIES decrypt failed — wrong key or tampered"))
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
    fn windowed_rendezvous_key_changes_daily() {
        let pub_key = [0x42u8; 32];
        let today = windowed_rendezvous_key(&pub_key);
        let yesterday = windowed_rendezvous_key_prev(&pub_key);
        assert_ne!(today, yesterday, "Key should differ between days");
    }

    #[test]
    fn windowed_key_is_deterministic_within_day() {
        let pub_key = [0x55u8; 32];
        let k1 = windowed_rendezvous_key(&pub_key);
        let k2 = windowed_rendezvous_key(&pub_key);
        assert_eq!(k1, k2, "Same day should produce same key");
    }

    #[test]
    fn ecies_roundtrip() {
        let mut secret_bytes = [0u8; 32];
        rand_core::RngCore::fill_bytes(&mut OsRng, &mut secret_bytes);
        secret_bytes[0] &= 248;
        secret_bytes[31] &= 127;
        secret_bytes[31] |= 64;
        let secret = x25519_dalek::StaticSecret::from(secret_bytes);
        let pubkey = x25519_dalek::PublicKey::from(&secret);

        let plaintext = b"Anonymous introduction from Bob";
        let sealed = ecies_seal(pubkey.as_bytes(), plaintext).unwrap();
        let recovered = ecies_unseal(&sealed, &secret).unwrap();
        assert_eq!(recovered, plaintext);
    }

    #[test]
    fn invite_string_roundtrip() {
        let invite = AnonymousInvite {
            ed25519_pub: [0x11u8; 32],
            x25519_pub: [0x22u8; 32],
            surb_bytes: vec![1, 2, 3, 4],
            nickname: Some("Alice".into()),
            created_at: now_secs(),
        };
        let s = invite.to_invite_string().unwrap();
        assert!(s.starts_with(ANON_INVITE_PREFIX));
        let recovered = AnonymousInvite::from_invite_string(&s).unwrap();
        assert_eq!(recovered.ed25519_pub, invite.ed25519_pub);
        assert_eq!(recovered.nickname, invite.nickname);
    }
}
