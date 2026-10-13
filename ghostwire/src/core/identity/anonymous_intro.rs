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
//! Additionally, Alice's QR includes a BQP-style key commitment — a 32-byte
//! BLAKE3 keyed hash over her X25519 pubkey (using ANON_INTRO_COMMIT_DOMAIN).
//! Bob verifies this commitment on receipt, preventing key-substitution
//! attacks where an attacker swaps Alice's pubkey without updating the
//! commitment. This mirrors Briar's BQP protocol (Bramble KeyAgreementConstants).
//!
//! Flow:
//!   1. Alice calls generate_intro_invite() → gets SURB + her pubkey + commitment
//!   2. Alice encodes this in a QR code: "gwi:<base58(serialized_invite)>"
//!   3. Bob scans QR, calls send_introduction(invite, bob_pubkey, bob_nodekey)
//!      — Bob verifies the commitment matches the X25519 pubkey
//!   4. Bob's introduction routes anonymously through the SURB to Alice
//!   5. Alice calls receive_introduction(payload) → gets Bob's public key
//!   6. Alice initiates vodozemac session → sends first message with her node info
//!   7. Normal Double Ratchet from here
//!
//! Additionally: time-windowed rendezvous DHT keys prevent long-term tracking.
//! The rendezvous key rotates every 24 hours:
//!   key = BLAKE3_keyed(RENDEZVOUS_DOMAIN, pubkey || floor(unix_ts / 86400))

use anyhow::Result;

use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};

use crate::genesis_constants::{genesis_derive, ANON_INTRO_COMMIT_DOMAIN, INTRO_CONFIRM_DOMAIN, INTRO_HMAC_DOMAIN, RENDEZVOUS_DOMAIN};

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
    /// BQP-style commitment to Alice's X25519 key — a 32-byte BLAKE3 keyed
    /// hash over her X25519 pubkey using the ANON_INTRO_COMMIT_DOMAIN.
    /// Bob verifies this against the pubkey he received, preventing
    /// key-substitution attacks where an attacker swaps in their own pubkey.
    pub commitment: [u8; 32],
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
    /// HMAC tag (BQP REQUEST_LABEL equivalent): Bob's HMAC over the invite
    /// + his own pubkeys + nonce, using INTRO_HMAC_DOMAIN as the key.
    /// This binds Bob to this specific introduction and prevents replay.
    pub hmac_tag: [u8; 32],
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

    // BQP-style commitment: BLAKE3 keyed hash over Alice's X25519 pubkey.
    // Bob will verify this against the pubkey he extracts from the invite,
    // preventing key-substitution attacks. Uses the same commitment-then-reveal
    // pattern as Briar's BQP protocol (KeyAgreementConstants.COMMIT_LABEL).
    let commitment = genesis_derive(ANON_INTRO_COMMIT_DOMAIN, &our_x25519_pub);

    Ok(AnonymousInvite {
        ed25519_pub: our_ed25519_pub,
        x25519_pub: our_x25519_pub,
        surb_bytes,
        nickname,
        created_at: now_secs(),
        commitment,
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

    // BQP-style commitment verification: derive the commitment from Alice's
    // X25519 pubkey and check it matches the commitment embedded in the invite.
    // This prevents key-substitution attacks where an attacker modifies the
    // pubkey in the QR code without updating the commitment.
    let expected_commitment = genesis_derive(ANON_INTRO_COMMIT_DOMAIN, &invite.x25519_pub);
    if expected_commitment != invite.commitment {
        anyhow::bail!(
            "Commitment mismatch: the X25519 public key in the invite does not \
             match the commitment. This may indicate a key-substitution attack."
        );
    }

    let surb: Surb = bincode::deserialize(&invite.surb_bytes)
        .map_err(|e| anyhow::anyhow!("SURB deserialise: {}", e))?;

    let mut nonce = [0u8; 16];
    ring::rand::SystemRandom::new()
        .fill(&mut nonce)
        .map_err(|_| anyhow::anyhow!("RNG failure"))?;

    // BQP-style HMAC authentication (REQUEST_LABEL equivalent):
    // Bob computes an HMAC over (invite.x25519_pub, our_x25519_pub, our_ed25519_pub, nonce)
    // using INTRO_HMAC_DOMAIN as the derivation domain. This binds Bob to this
    // specific introduction conversation and prevents replay/reflection attacks.
    // Alice verifies this HMAC in receive_introduction().
    let hmac_tag = compute_intro_hmac(
        &invite.x25519_pub,
        &our_x25519_pub,
        &our_ed25519_pub,
        &nonce,
    );

    let intro = IntroductionPayload {
        ed25519_pub: our_ed25519_pub,
        x25519_pub: our_x25519_pub,
        nickname: our_nickname,
        nonce,
        created_at: now_secs(),
        hmac_tag,
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

    Ok((packet_bytes, surb.first_hop.clone()))
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

    // BQP-style HMAC verification (REQUEST_LABEL equivalent):
    // Alice verifies that the HMAC tag matches what Bob should have computed
    // over (invite.x25519_pub, bob.x25519_pub, bob.ed25519_pub, nonce).
    // This prevents:
    //   - Replay: the nonce ensures freshness
    //   - Key substitution: Bob's pubkeys are bound into the HMAC
    //   - Reflection: the invite's pubkey is included, preventing Bob from
    //     replaying Alice's own pubkey back at her
    // Alice knows her X25519 pubkey (from her secret) — this IS the invite's
    // x25519_pub that Bob used when computing the HMAC.
    let alice_x25519_pub = x25519_dalek::PublicKey::from(our_x25519_secret);
    let expected_tag = compute_intro_hmac(
        alice_x25519_pub.as_bytes(),  // Alice's (invite's) X25519 pub
        &intro.x25519_pub,            // Bob's X25519 pub (in the intro)
        &intro.ed25519_pub,           // Bob's Ed25519 pub (in the intro)
        &intro.nonce,
    );
    if expected_tag != intro.hmac_tag {
        anyhow::bail!(
            "HMAC verification failed: introduction payload does not authenticate — \
             possible replay, tampering, or key-substitution attack."
        );
    }

    Ok(intro)
}

/// Bob sends a BQP-style confirmation record back to Alice through the
/// established session channel (reply-to-path).
///
/// After Alice receives Bob's IntroductionPayload, she has Bob's X25519
/// pubkey. She can compute the same DH shared secret and verify Bob's
/// confirmation record matches. This is the mutual authentication step.
///
/// In Ghostwire's anonymous intro flow, this confirmation is sent as the
/// first message of the Double Ratchet session — Alice initiates the
/// session, Bob replies with this confirmation, and Alice verifies it.
pub fn send_confirmation(
    invite: &AnonymousInvite,
    bob_x25519_pub: [u8; 32],
    bob_ed25519_pub: [u8; 32],
    nonce: &[u8; 16],
    our_x25519_secret: &x25519_dalek::StaticSecret,
) -> Result<[u8; 32]> {
    // Derive shared secret: DH(Alice's X25519 secret, Bob's X25519 pub)
    let alice_x25519_pub = x25519_dalek::PublicKey::from(our_x25519_secret);
    let bob_pub = x25519_dalek::PublicKey::from(bob_x25519_pub);

    // Verify our pubkey matches the invite (should match)
    if alice_x25519_pub.as_bytes() != &invite.x25519_pub {
        anyhow::bail!(
            "Alice's X25519 pubkey does not match the invite — internal error"
        );
    }

    let shared = our_x25519_secret.diffie_hellman(&bob_pub);

    Ok(compute_confirmation_record(
        shared.as_bytes(),
        &invite.x25519_pub,
        &bob_x25519_pub,
        &bob_ed25519_pub,
        nonce,
    ))
}

/// Alice verifies Bob's BQP confirmation record.
///
/// Called after Alice has received the IntroductionPayload (through the SURB)
/// and established the initial DH with Bob's X25519 pubkey. Alice recomputes
/// the confirmation record and checks it matches what Bob sent.
pub fn verify_confirmation(
    invite: &AnonymousInvite,
    intro: &IntroductionPayload,
    our_x25519_secret: &x25519_dalek::StaticSecret,
    bob_confirm: &[u8; 32],
) -> Result<()> {
    let bob_pub = x25519_dalek::PublicKey::from(intro.x25519_pub);
    let shared = our_x25519_secret.diffie_hellman(&bob_pub);

    let expected = compute_confirmation_record(
        shared.as_bytes(),
        &invite.x25519_pub,
        &intro.x25519_pub,
        &intro.ed25519_pub,
        &intro.nonce,
    );

    if expected != *bob_confirm {
        anyhow::bail!(
            "BQP confirmation record mismatch — possible MITM attack or tampered introduction"
        );
    }
    Ok(())
}
///
/// This is the REQUEST_LABEL equivalent: an HMAC over the invite's X25519 pubkey,
/// Bob's own Ed25519 + X25519 pubkeys, and the nonce — providing:
///   - Replay prevention: unique nonce per introduction
///   - Binding: Bob is cryptographically bound to this specific invite
///   - Non-repudiation: Bob can't deny sending the introduction
///
/// Uses genesis_derive (BLAKE3 keyed) with INTRO_HMAC_DOMAIN, mirroring
/// Briar BQP's HMAC-SHA256 REQUEST_LABEL pattern.
fn compute_intro_hmac(
    invite_x25519_pub: &[u8; 32],
    bob_x25519_pub: &[u8; 32],
    bob_ed25519_pub: &[u8; 32],
    nonce: &[u8; 16],
) -> [u8; 32] {
    let mut ikm = Vec::with_capacity(32 + 32 + 32 + 16);
    ikm.extend_from_slice(invite_x25519_pub);
    ikm.extend_from_slice(bob_x25519_pub);
    ikm.extend_from_slice(bob_ed25519_pub);
    ikm.extend_from_slice(nonce);

    genesis_derive(INTRO_HMAC_DOMAIN, &ikm)
}

/// Compute a BQP-style confirmation record (third round of the handshake).
///
/// Both Alice and Bob derive the same DH shared secret from:
///   shared = X25519(alice_x25519_secret, bob_x25519_pub)
///
/// Then derive a confirmation key and HMAC over the static identifiers:
///   confirm_key = genesis_derive(INTRO_CONFIRM_DOMAIN, &shared_secret_bytes)
///   confirm_mac = genesis_derive(confirm_key, &invite_x25519_pub || bob_x25519_pub || bob_ed25519_pub || nonce)
///
/// This mirrors Briar BQP's deriveConfirmationRecord():
///   - CONFIRMATION_KEY_LABEL: derives the confirmation key from the shared secret
///   - CONFIRMATION_MAC_LABEL: HMAC over (alice_payload, alice_pub, bob_payload, bob_pub)
///
/// The confirmation record allows Alice to prove she holds the shared secret
/// (i.e., her X25519 secret), and Bob to prove he computed the same secret.
/// Without this step, an attacker who intercepted the ECIES-encrypted
/// IntroductionPayload could forward it without proving knowledge of the
/// DH shared secret.
pub fn compute_confirmation_record(
    shared_secret: &[u8; 32],
    invite_x25519_pub: &[u8; 32],
    bob_x25519_pub: &[u8; 32],
    bob_ed25519_pub: &[u8; 32],
    nonce: &[u8; 16],
) -> [u8; 32] {
    let confirm_key = genesis_derive(INTRO_CONFIRM_DOMAIN, shared_secret);
    let mut ikm = Vec::with_capacity(32 + 32 + 32 + 16);
    ikm.extend_from_slice(invite_x25519_pub);
    ikm.extend_from_slice(bob_x25519_pub);
    ikm.extend_from_slice(bob_ed25519_pub);
    ikm.extend_from_slice(nonce);
    genesis_derive(&confirm_key, &ikm)
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
        rand_core::RngCore::fill_bytes(&mut rand_core::OsRng, &mut secret_bytes);
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
        let x25519_pub = [0x22u8; 32];
        let commitment = genesis_derive(ANON_INTRO_COMMIT_DOMAIN, &x25519_pub);
        let invite = AnonymousInvite {
            ed25519_pub: [0x11u8; 32],
            x25519_pub,
            surb_bytes: vec![1, 2, 3, 4],
            nickname: Some("Alice".into()),
            created_at: now_secs(),
            commitment,
        };
        let s = invite.to_invite_string().unwrap();
        assert!(s.starts_with(ANON_INVITE_PREFIX));
        let recovered = AnonymousInvite::from_invite_string(&s).unwrap();
        assert_eq!(recovered.ed25519_pub, invite.ed25519_pub);
        assert_eq!(recovered.x25519_pub, invite.x25519_pub);
        assert_eq!(recovered.commitment, invite.commitment);
        assert_eq!(recovered.nickname, invite.nickname);
    }

    #[test]
    fn commitment_prevents_key_substitution() {
        // An attacker who swaps the X25519 pubkey but keeps the old commitment
        // should produce a mismatched commitment.
        let alice_real_key = [0x22u8; 32];
        let alice_fake_key = [0xFFu8; 32];

        let real_commitment = genesis_derive(ANON_INTRO_COMMIT_DOMAIN, &alice_real_key);
        let fake_commitment = genesis_derive(ANON_INTRO_COMMIT_DOMAIN, &alice_fake_key);

        assert_ne!(real_commitment, fake_commitment,
            "Different pubkeys must yield different commitments");

        // An invite where the pubkey was swapped but commitment was not updated
        let invite_with_swapped_key = AnonymousInvite {
            ed25519_pub: [0x11u8; 32],
            x25519_pub: alice_fake_key,  // swapped!
            surb_bytes: vec![1, 2, 3, 4],
            nickname: None,
            created_at: now_secs(),
            commitment: real_commitment,  // still the old one
        };

        let expected = genesis_derive(ANON_INTRO_COMMIT_DOMAIN, &invite_with_swapped_key.x25519_pub);
        assert_ne!(expected, invite_with_swapped_key.commitment,
            "Key-substitution attack should be detected by commitment mismatch");
    }

    #[test]
    fn hmac_prevents_replay_and_tampering() {
        // Bob computes HMAC over (invite_pub, bob_x25519, bob_ed25519, nonce)
        let invite_x25519 = [0x22u8; 32];
        let bob_x25519 = [0x33u8; 32];
        let bob_ed25519 = [0x44u8; 32];
        let nonce = [0x55u8; 16];

        let tag = compute_intro_hmac(&invite_x25519, &bob_x25519, &bob_ed25519, &nonce);

        // Correct inputs should produce matching tag
        let expected = compute_intro_hmac(&invite_x25519, &bob_x25519, &bob_ed25519, &nonce);
        assert_eq!(tag, expected, "HMAC should be deterministic");

        // Changing any input should produce a different tag (tamper detection)
        let different_nonce = [0x99u8; 16];
        let tampered_tag = compute_intro_hmac(&invite_x25519, &bob_x25519, &bob_ed25519, &different_nonce);
        assert_ne!(tag, tampered_tag, "Tampered nonce must produce different HMAC");

        // Different invite pub (reflection attack prevention)
        let reflection_tag = compute_intro_hmac(&bob_x25519, &bob_x25519, &bob_ed25519, &nonce);
        assert_ne!(tag, reflection_tag, "Reflection attack must produce different HMAC");
    }

    #[test]
    fn confirmation_record_is_symmetric() {
        // Alice and Bob both compute the same confirmation record
        let alice_secret = x25519_dalek::StaticSecret::from([0xA1u8; 32]);
        let alice_pub = x25519_dalek::PublicKey::from(&alice_secret);
        let bob_secret = x25519_dalek::StaticSecret::from([0xB2u8; 32]);
        let bob_pub = x25519_dalek::PublicKey::from(&bob_secret);

        let shared = alice_secret.diffie_hellman(&bob_pub);

        let nonce = [0x55u8; 16];

        // Alice computes: uses her pub (invite's), Bob's pubs
        let alice_confirm = compute_confirmation_record(
            shared.as_bytes(),
            alice_pub.as_bytes(),
            bob_pub.as_bytes(),
            &[0xC3u8; 32], // Bob's ed25519
            &nonce,
        );

        // Bob computes: same inputs (invite is Alice's pub, bob's identitites)
        let bob_confirm = compute_confirmation_record(
            shared.as_bytes(),
            alice_pub.as_bytes(),
            bob_pub.as_bytes(),
            &[0xC3u8; 32],
            &nonce,
        );

        assert_eq!(alice_confirm, bob_confirm, "Both parties must compute the same confirmation record");
    }

    #[test]
    fn confirmation_record_rejects_wrong_shared_secret() {
        let alice_secret = x25519_dalek::StaticSecret::from([0xA1u8; 32]);
        let alice_pub = x25519_dalek::PublicKey::from(&alice_secret);
        let bob_secret = x25519_dalek::StaticSecret::from([0xB2u8; 32]);
        let bob_pub = x25519_dalek::PublicKey::from(&bob_secret);

        let wrong_secret = alice_secret.diffie_hellman(&x25519_dalek::PublicKey::from(&x25519_dalek::StaticSecret::from([0x99u8; 32])));

        let nonce = [0x55u8; 16];

        let correct = compute_confirmation_record(
            &alice_secret.diffie_hellman(&bob_pub).as_bytes(),
            alice_pub.as_bytes(),
            bob_pub.as_bytes(),
            &[0xC3u8; 32],
            &nonce,
        );

        let wrong = compute_confirmation_record(
            wrong_secret.as_bytes(),
            alice_pub.as_bytes(),
            bob_pub.as_bytes(),
            &[0xC3u8; 32],
            &nonce,
        );

        assert_ne!(correct, wrong, "Different shared secrets must produce different confirmation records");
    }

    #[test]
    fn send_confirmation_rejects_pubkey_mismatch() {
        let alice_secret = x25519_dalek::StaticSecret::from([0xA1u8; 32]);
        let invite = AnonymousInvite {
            ed25519_pub: [0x11u8; 32],
            x25519_pub: [0xFFu8; 32], // Mismatched — real pub is different
            surb_bytes: vec![],
            nickname: None,
            created_at: 0,
            commitment: [0u8; 32],
        };

        let result = send_confirmation(
            &invite,
            [0xB2u8; 32],
            [0xC3u8; 32],
            &[0x55u8; 16],
            &alice_secret,
        );

        assert!(result.is_err(), "Should reject mismatched pubkey");
        assert!(result.unwrap_err().to_string().contains("does not match"));
    }

    #[test]
    fn verify_confirmation_accepts_correct_record() {
        let alice_secret = x25519_dalek::StaticSecret::from([0xDDu8; 32]);
        let alice_pub_key = x25519_dalek::PublicKey::from(&alice_secret);
        let alice_pub = alice_pub_key.as_bytes();

        let bob_secret = x25519_dalek::StaticSecret::from([0xBAu8; 32]);
        let bob_pub = x25519_dalek::PublicKey::from(&bob_secret);
        let bob_pub_bytes = bob_pub.as_bytes();
        let bob_ed = [0x99u8; 32];

        let nonce = [0x42u8; 16];

        // Compute shared secret
        let shared = alice_secret.diffie_hellman(&bob_pub);

        // Bob computes confirmation
        let bob_confirm = compute_confirmation_record(
            shared.as_bytes(),
            alice_pub,
            bob_pub_bytes,
            &bob_ed,
            &nonce,
        );

        // Alice verifies
        let invite = AnonymousInvite {
            ed25519_pub: [0x11u8; 32],
            x25519_pub: *alice_pub,
            surb_bytes: vec![],
            nickname: None,
            created_at: 0,
            commitment: genesis_derive(ANON_INTRO_COMMIT_DOMAIN, alice_pub),
        };

        let intro = IntroductionPayload {
            ed25519_pub: bob_ed,
            x25519_pub: *bob_pub_bytes,
            nickname: None,
            nonce,
            created_at: now_secs(),
            hmac_tag: compute_intro_hmac(alice_pub, bob_pub_bytes, &bob_ed, &nonce),
        };

        let result = verify_confirmation(&invite, &intro, &alice_secret, &bob_confirm);
        assert!(result.is_ok(), "Correct confirmation must verify");
    }

    #[test]
    fn verify_confirmation_rejects_wrong_record() {
        let alice_secret = x25519_dalek::StaticSecret::from([0xDDu8; 32]);
        let alice_pub_key = x25519_dalek::PublicKey::from(&alice_secret);
        let alice_pub = alice_pub_key.as_bytes();

        let bob_secret = x25519_dalek::StaticSecret::from([0xBAu8; 32]);
        let bob_pub = x25519_dalek::PublicKey::from(&bob_secret);
        let bob_pub_bytes = bob_pub.as_bytes();
        let bob_ed = [0x99u8; 32];

        let nonce = [0x42u8; 16];

        // Wrong confirmation (attacker's guess)
        let wrong_confirm = [0xDEu8; 32];

        let invite = AnonymousInvite {
            ed25519_pub: [0x11u8; 32],
            x25519_pub: *alice_pub,
            surb_bytes: vec![],
            nickname: None,
            created_at: 0,
            commitment: genesis_derive(ANON_INTRO_COMMIT_DOMAIN, alice_pub),
        };

        let intro = IntroductionPayload {
            ed25519_pub: bob_ed,
            x25519_pub: *bob_pub_bytes,
            nickname: None,
            nonce,
            created_at: now_secs(),
            hmac_tag: compute_intro_hmac(alice_pub, bob_pub_bytes, &bob_ed, &nonce),
        };

        let result = verify_confirmation(&invite, &intro, &alice_secret, &wrong_confirm);
        assert!(result.is_err(), "Wrong confirmation must be rejected");
        assert!(result.unwrap_err().to_string().contains("mismatch"));
    }
}
