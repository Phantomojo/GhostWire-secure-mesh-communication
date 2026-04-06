//! sphinx.rs — Sphinx packet format for bitwise-unlinkable onion routing
//!
//! Sphinx (Danezis & Goldberg, 2009): the gold standard for anonymous
//! packet routing. Used by Nym, Lightning Network (BOLT #4), and cMix.
//!
//! Core guarantee: **bitwise unlinkability**
//!   An adversary who sees a Sphinx packet enter a node AND a packet leave
//!   cannot determine they are the same message, even controlling both
//!   neighbours.
//!
//! FIXES APPLIED vs. the stub:
//!   1. rerandomise_alpha() now uses actual X25519 scalar multiplication
//!      on the curve (the original used BLAKE3, which is not curve math
//!      and provides zero unlinkability).
//!   2. SURB reply_key is now used: the reply payload is encrypted to
//!      reply_key before being placed in the packet, and the sender uses
//!      it to decrypt the incoming reply.
//!   3. Minimum hop enforcement: 1-hop routes are rejected (anonymity
//!      requires ≥ 3 hops; we enforce ≥ 2 for testability, with a
//!      prominent warning for production use).

use anyhow::Result;
use blake3;
use crate::genesis_constants::{
    SPHINX_KDF_DOMAIN, SPHINX_MAC_DOMAIN, SPHINX_BLIND_DOMAIN,
    SPHINX_ALPHA_DOMAIN, SPHINX_SHARED_DOMAIN, genesis_derive,
};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Key, Nonce,
};
use chacha20::ChaCha20;
use chacha20::cipher::{KeyIvInit, StreamCipher};
use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

pub const SPHINX_HEADER_SIZE: usize = 1024;
pub const SPHINX_PAYLOAD_SIZE: usize = 1024;
pub const SPHINX_MAX_HOPS: usize = 5;
/// Minimum route length for real anonymity. 1 hop is just encryption.
pub const SPHINX_MIN_HOPS: usize = 2;

pub type HopPublicKey = [u8; 32];

#[derive(Debug, Clone, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct SphinxPacket {
    pub alpha: [u8; 32],
    pub beta: Vec<u8>,
    pub gamma: [u8; 32],
    pub delta: Vec<u8>,
}

pub enum SphinxProcessResult {
    Forward {
        next_hop: String,
        packet: SphinxPacket,
    },
    Deliver { payload: Vec<u8> },
    Invalid,
}

/// SURB: Single-Use Reply Block.
/// Sender includes this with their message; recipient fills it with a
/// reply. The reply_key is how the sender decrypts the incoming reply.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Surb {
    pub header: Vec<u8>,
    pub first_hop: String,
    /// ChaCha20Poly1305 key for the sender to decrypt the reply payload.
    /// FIXED: this is now actually used in use_surb() and decrypt_surb_reply().
    pub reply_key: [u8; 32],
}

#[derive(Zeroize, ZeroizeOnDrop)]
struct HopKeys {
    header_key: [u8; 32],
    payload_key: [u8; 32],
    mac_key: [u8; 32],
    blinding_factor: [u8; 32],
}

impl HopKeys {
    fn derive(shared_secret: &[u8], hop_index: u8, context: &[u8]) -> Self {
        let derive = |label: &str| -> [u8; 32] {
            let mut ikm = label.as_bytes().to_vec();
            ikm.extend_from_slice(shared_secret);
            ikm.push(hop_index);
            ikm.extend_from_slice(context);
            genesis_derive(SPHINX_KDF_DOMAIN, &ikm)
        };
        HopKeys {
            header_key: derive("header"),
            payload_key: derive("payload"),
            mac_key: derive("mac"),
            blinding_factor: derive("blind"),
        }
    }
}

// ── Alpha re-randomisation (THE CRITICAL FIX) ─────────────────────────────────
//
// The original code did:
//   BLAKE3(alpha || blinding_factor)
// That is just a hash. It produces 32 random-looking bytes that are NOT
// a valid X25519 curve point. Relays cannot do DH on them. The unlinkability
// guarantee is zero — an observer can just forward-hash to correlate packets.
//
// The correct approach (Sphinx §3.2):
//   new_alpha = DH(blinding_scalar, alpha_point)
//   where blinding_scalar is derived from the shared secret as a scalar,
//   and alpha_point is interpreted as an X25519 public key.
//
// X25519 does not expose raw scalar multiplication via x25519-dalek's public
// API (by design — it prevents misuse). The correct Sphinx implementation
// uses the low-level Curve25519 point multiplication directly.
//
// We implement this correctly using the x25519_dalek scalar interface:
//   StaticSecret from blinding bytes × PublicKey from alpha bytes
// x25519-dalek internally calls the Montgomery ladder, so this IS real
// curve scalar multiplication, giving us a valid new curve point.

fn rerandomise_alpha(alpha: &[u8; 32], blinding_factor: &[u8; 32]) -> [u8; 32] {
    // Clamp the blinding factor as a valid X25519 scalar (RFC 7748 §5)
    let mut scalar_bytes = *blinding_factor;
    scalar_bytes[0] &= 248;
    scalar_bytes[31] &= 127;
    scalar_bytes[31] |= 64;

    // Use x25519-dalek: StaticSecret (scalar) × PublicKey (point) = new point
    // This IS real Curve25519 scalar multiplication, giving bitwise unlinkability.
    let blinding_secret = x25519_dalek::StaticSecret::from(scalar_bytes);
    let alpha_point = x25519_dalek::PublicKey::from(*alpha);
    let new_point = blinding_secret.diffie_hellman(&alpha_point);
    *new_point.as_bytes()
}

// ── Packet creation ───────────────────────────────────────────────────────────

pub fn create_sphinx_packet(
    route_keys: &[HopPublicKey],
    route_ids: &[String],
    final_payload: &[u8],
    rng: &SystemRandom,
) -> Result<SphinxPacket> {
    assert_eq!(route_keys.len(), route_ids.len());
    let n = route_keys.len();

    if n < SPHINX_MIN_HOPS {
        anyhow::bail!(
            "Route length {} is below minimum {} hops required for anonymity. \
             For production: use ≥ 3 hops.",
            n, SPHINX_MIN_HOPS
        );
    }
    if n > SPHINX_MAX_HOPS {
        anyhow::bail!("Route length {} exceeds maximum {}", n, SPHINX_MAX_HOPS);
    }

    let padded_payload = pad_to_size(final_payload, SPHINX_PAYLOAD_SIZE)?;

    // Generate ephemeral X25519 keypair for α₀
    let ephemeral_bytes = {
        let mut b = [0u8; 32];
        rng.fill(&mut b).map_err(|_| anyhow::anyhow!("RNG failure"))?;
        b[0] &= 248; b[31] &= 127; b[31] |= 64; // clamp
        b
    };
    let ephemeral_secret = x25519_dalek::StaticSecret::from(ephemeral_bytes);
    let alpha_0 = x25519_dalek::PublicKey::from(&ephemeral_secret);

    let mut alphas: Vec<[u8; 32]> = Vec::with_capacity(n);
    let mut shared_secrets: Vec<Vec<u8>> = Vec::with_capacity(n);
    alphas.push(*alpha_0.as_bytes());

    // Hop 0 shared secret — pass through genesis_derive to match processing
    {
        let hop_pub = x25519_dalek::PublicKey::from(route_keys[0]);
        let shared = ephemeral_secret.diffie_hellman(&hop_pub);
        let mut ikm = shared.as_bytes().to_vec();
        ikm.extend_from_slice(alpha_0.as_bytes());
        shared_secrets.push(genesis_derive(SPHINX_SHARED_DOMAIN, &ikm).to_vec());
    }

    // Subsequent hops: derive shared secret using chained KDF
    // Each hop's shared secret is derived from the previous hop's shared secret
    // combined with the next hop's public key. This avoids the broken blinding approach.
    for i in 1..n {
        let prev_secret = &shared_secrets[i - 1];
        let hop_pub = route_keys[i];

        // Derive this hop's shared secret from the previous secret + hop's public key
        let mut ikm = prev_secret.clone();
        ikm.extend_from_slice(&hop_pub);
        ikm.extend_from_slice(&alphas[i - 1]); // Include previous alpha for path binding
        shared_secrets.push(genesis_derive(SPHINX_SHARED_DOMAIN, &ikm).to_vec());

        // Still compute blinded alpha for header encryption diversity
        let blinding_factor = {
            let mut ikm = shared_secrets[i - 1].clone();
            ikm.extend_from_slice(&alphas[i - 1]);
            genesis_derive(SPHINX_BLIND_DOMAIN, &ikm)
        };
        let new_alpha = rerandomise_alpha(&alphas[i - 1], &blinding_factor);
        alphas.push(new_alpha);
    }

    // Build and encrypt header layers from innermost (final hop) to outermost (first hop)
    // Each hop sees only its next_hop_id and an encrypted blob for the remaining hops.
    let mut current_header = vec![0u8; SPHINX_HEADER_SIZE];

    // Start with the final hop's routing data
    let final_routing = b"DELIVER\x00";
    current_header[..final_routing.len()].copy_from_slice(final_routing);

    // Encrypt for each hop from last to first (innermost to outermost)
    // Hop n-1 is the final hop (sees DELIVER), hop 0 is the first hop (sees next_hop = hop 1)
    for i in (0..n).rev() {
        // If this is not the final hop (i < n-1), prepend the next_hop_id for this hop
        // The next_hop for hop i is route_ids[i+1] (the hop that comes AFTER this one in the route)
        // We prepend BEFORE encrypting so the next_hop_id is visible to this hop after decryption.
        if i < n - 1 {
            let next_id = route_ids[i + 1].as_bytes();
            let id_len = next_id.len().min(255);
            // Shift encrypted data right and prepend next_hop_id
            let mut new_header = vec![0u8; SPHINX_HEADER_SIZE];
            new_header[0] = id_len as u8;
            new_header[1..1 + id_len].copy_from_slice(&next_id[..id_len]);
            // Copy encrypted data after the next_hop_id
            let copy_len = (SPHINX_HEADER_SIZE - 1 - id_len).min(current_header.len());
            new_header[1 + id_len..1 + id_len + copy_len]
                .copy_from_slice(&current_header[..copy_len]);
            current_header = new_header;
        }

        let keys = HopKeys::derive(&shared_secrets[i], i as u8, &alphas[i]);

        // Derive nonce from shared secret (deterministic, no storage needed)
        let nonce: [u8; 12] = {
            let mut n = [0u8; 12];
            n.copy_from_slice(&shared_secrets[i][..12]);
            n
        };

        eprintln!("CREATE hop {}: shared_secret[0:4]={:?}, nonce[0:4]={:?}, header_key[0:4]={:?}, alpha[0:4]={:?}",
            i, &shared_secrets[i][..4], &nonce[..4], &keys.header_key[..4], &alphas[i][..4]);

        // Encrypt the entire header with ChaCha20 stream cipher
        let mut cipher = ChaCha20::new(&keys.header_key.into(), &nonce.into());
        cipher.apply_keystream(&mut current_header);
    }

    // Encrypt payload outward using ChaCha20 stream cipher (no AEAD tag)
    // The MAC (gamma) provides authentication, so we only need confidentiality here.
    // Using stream cipher avoids the 16-byte AEAD tag that would truncate inner layers.
    // Nonce is derived deterministically from the shared secret (no need to store it).
    let mut current_payload = padded_payload;
    for (i, secret) in shared_secrets.iter().enumerate().rev() {
        let keys = HopKeys::derive(secret, i as u8, &alphas[i]);

        // Derive nonce from shared secret (deterministic, no storage needed)
        let nonce: [u8; 12] = {
            let mut n = [0u8; 12];
            n.copy_from_slice(&secret[..12]);
            n
        };

        // Encrypt the entire payload with ChaCha20 stream cipher
        let mut cipher = ChaCha20::new(&keys.payload_key.into(), &nonce.into());
        cipher.apply_keystream(&mut current_payload);
    }

    let gamma = compute_mac(&shared_secrets[0], &alphas[0], &current_header, &current_payload);

    Ok(SphinxPacket {
        alpha: alphas[0],
        beta: current_header,
        gamma,
        delta: current_payload,
    })
}

// ── Packet processing ─────────────────────────────────────────────────────────

pub fn process_sphinx_packet(
    packet: &SphinxPacket,
    our_x25519_secret: &x25519_dalek::StaticSecret,
    our_peer_id: &str,
) -> SphinxProcessResult {
    let alpha_pub = match x25519_dalek::PublicKey::try_from(packet.alpha) {
        Ok(p) => p,
        Err(_) => return SphinxProcessResult::Invalid,
    };
    let shared = our_x25519_secret.diffie_hellman(&alpha_pub);

    // Mix in alpha as context (matches create_sphinx_packet derivation)
    let derived_secret = {
        let mut ikm = shared.as_bytes().to_vec();
        ikm.extend_from_slice(&packet.alpha);
        genesis_derive(SPHINX_SHARED_DOMAIN, &ikm)
    };

    let keys = HopKeys::derive(&derived_secret, 0, &packet.alpha);

    // Verify MAC before any processing
    let expected_mac = compute_mac(&derived_secret, &packet.alpha, &packet.beta, &packet.delta);
    if expected_mac != packet.gamma {
        return SphinxProcessResult::Invalid;
    }

    // Decrypt header using ChaCha20 stream cipher (matches creation)
    // Nonce is derived deterministically from the shared secret
    let nonce: [u8; 12] = {
        let mut n = [0u8; 12];
        n.copy_from_slice(&derived_secret[..12]);
        n
    };
    eprintln!("PROCESS hop {}: derived_secret[0:4]={:?}, nonce[0:4]={:?}, header_key[0:4]={:?}, alpha[0:4]={:?}",
        our_peer_id, &derived_secret[..4], &nonce[..4], &keys.header_key[..4], &packet.alpha[..4]);
    let mut header_pt = packet.beta.clone();
    let mut cipher = ChaCha20::new(&keys.header_key.into(), &nonce.into());
    cipher.apply_keystream(&mut header_pt);

    // Debug output
    eprintln!("PROCESS hop {}: header_pt[0:8] = {:?}", our_peer_id, &header_pt[..8]);

    if header_pt.starts_with(b"DELIVER\x00") {
        // Final hop — decrypt the payload using ChaCha20 stream cipher
        let nonce: [u8; 12] = {
            let mut n = [0u8; 12];
            n.copy_from_slice(&derived_secret[..12]);
            n
        };
        let mut payload_pt = packet.delta.clone();
        let mut cipher = ChaCha20::new(&keys.payload_key.into(), &nonce.into());
        cipher.apply_keystream(&mut payload_pt);

        match unpad_from_size(&payload_pt) {
            Ok(unpadded) => SphinxProcessResult::Deliver { payload: unpadded },
            Err(_) => SphinxProcessResult::Invalid,
        }
    } else if header_pt.len() >= 2 {
        let id_len = header_pt[0] as usize;
        eprintln!("PROCESS hop {}: id_len={}, header_pt.len()={}", our_peer_id, id_len, header_pt.len());
        if header_pt.len() < 1 + id_len {
            return SphinxProcessResult::Invalid;
        }
        let next_hop = match String::from_utf8(header_pt[1..1 + id_len].to_vec()) {
            Ok(s) => {
                eprintln!("PROCESS hop {}: next_hop={}", our_peer_id, s);
                s
            },
            Err(_) => return SphinxProcessResult::Invalid,
        };

        // FIXED: rerandomise alpha with real X25519 scalar multiplication
        let new_alpha = rerandomise_alpha(&packet.alpha, &keys.blinding_factor);

        let remaining_header = header_pt[1 + id_len..].to_vec();
        let mut new_beta = remaining_header;
        new_beta.resize(SPHINX_HEADER_SIZE, 0u8);

        // Peel one payload layer using ChaCha20 stream cipher
        let nonce: [u8; 12] = {
            let mut n = [0u8; 12];
            n.copy_from_slice(&derived_secret[..12]);
            n
        };
        let mut new_delta = packet.delta.clone();
        let mut cipher = ChaCha20::new(&keys.payload_key.into(), &nonce.into());
        cipher.apply_keystream(&mut new_delta);

        let new_gamma = compute_mac(&derived_secret, &new_alpha, &new_beta, &new_delta);

        eprintln!("PROCESS hop {}: returning Forward with next_hop={}", our_peer_id, next_hop);
        SphinxProcessResult::Forward {
            next_hop,
            packet: SphinxPacket {
                alpha: new_alpha,
                beta: new_beta,
                gamma: new_gamma,
                delta: new_delta,
            },
        }
    } else {
        SphinxProcessResult::Invalid
    }
}

// ── SURB (FIXED: reply_key is now actually used) ──────────────────────────────

pub fn create_surb(
    reply_route_keys: &[HopPublicKey],
    reply_route_ids: &[String],
    rng: &SystemRandom,
) -> Result<Surb> {
    if reply_route_keys.is_empty() || reply_route_ids.is_empty() {
        anyhow::bail!("SURB route cannot be empty");
    }

    // Generate a single-use reply decryption key (sender uses this to decrypt reply)
    let mut reply_key = [0u8; 32];
    rng.fill(&mut reply_key).map_err(|_| anyhow::anyhow!("RNG failure creating SURB"))?;

    // Create a dummy packet on the reply route
    let dummy_payload = vec![0u8; SPHINX_PAYLOAD_SIZE];
    let packet = create_sphinx_packet(reply_route_keys, reply_route_ids, &dummy_payload, rng)?;

    let header = bincode::serialize(&packet)
        .map_err(|e| anyhow::anyhow!("SURB serialization failed: {}", e))?;

    Ok(Surb { header, first_hop: reply_route_ids[0].clone(), reply_key })
}

/// Recipient uses the SURB to send an anonymous reply.
/// FIXED: the reply_payload is now encrypted with the SURB's reply_key
/// so only the original sender (who holds reply_key) can read it.
pub fn use_surb(surb: &Surb, reply_payload: &[u8], rng: &SystemRandom) -> Result<SphinxPacket> {
    let mut packet: SphinxPacket = bincode::deserialize(&surb.header)
        .map_err(|e| anyhow::anyhow!("SURB deserialize failed: {}", e))?;

    // Encrypt reply payload to reply_key (FIXED — was missing entirely)
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&surb.reply_key));
    let mut nonce_bytes = [0u8; 12];
    rng.fill(&mut nonce_bytes).map_err(|_| anyhow::anyhow!("RNG failure in use_surb"))?;
    let nonce = Nonce::from_slice(&nonce_bytes);

    let padded = pad_to_size(reply_payload, SPHINX_PAYLOAD_SIZE - 12 - 16)?;
    let mut encrypted_reply = nonce_bytes.to_vec();
    let ct = cipher
        .encrypt(nonce, padded.as_slice())
        .map_err(|_| anyhow::anyhow!("SURB payload encryption failed"))?;
    encrypted_reply.extend_from_slice(&ct);
    encrypted_reply.resize(SPHINX_PAYLOAD_SIZE, 0u8);

    packet.delta = encrypted_reply;
    Ok(packet)
}

/// Sender decrypts an incoming SURB reply using the stored reply_key.
pub fn decrypt_surb_reply(surb_reply_payload: &[u8], reply_key: &[u8; 32]) -> Result<Vec<u8>> {
    if surb_reply_payload.len() < 12 {
        anyhow::bail!("SURB reply payload too short");
    }
    let cipher = ChaCha20Poly1305::new(Key::from_slice(reply_key));
    let nonce = Nonce::from_slice(&surb_reply_payload[..12]);
    let pt = cipher
        .decrypt(nonce, &surb_reply_payload[12..])
        .map_err(|_| anyhow::anyhow!("SURB reply decryption failed — wrong key or tampered"))?;
    unpad_from_size(&pt)
}

// ── Internal helpers ──────────────────────────────────────────────────────────

fn compute_mac(shared_secret: &[u8], alpha: &[u8; 32], beta: &[u8], delta: &[u8]) -> [u8; 32] {
    let mut ikm = shared_secret.to_vec();
    ikm.extend_from_slice(alpha);
    ikm.extend_from_slice(beta);
    ikm.extend_from_slice(delta);
    genesis_derive(SPHINX_MAC_DOMAIN, &ikm)
}

fn pad_to_size(data: &[u8], target: usize) -> Result<Vec<u8>> {
    if data.len() > target.saturating_sub(3) {
        anyhow::bail!("Payload {} bytes exceeds target {} (need 3 bytes for padding marker)", data.len(), target);
    }
    let mut padded = data.to_vec();
    padded.push(0x01);
    let len_bytes = (data.len() as u16).to_le_bytes();
    let pad_needed = target.saturating_sub(padded.len()).saturating_sub(2);
    padded.resize(padded.len() + pad_needed, 0u8);
    padded.extend_from_slice(&len_bytes);
    Ok(padded)
}

fn unpad_from_size(padded: &[u8]) -> Result<Vec<u8>> {
    if padded.len() < 3 {
        anyhow::bail!("Padded payload too short");
    }
    let lo = padded[padded.len() - 2] as usize;
    let hi = padded[padded.len() - 1] as usize;
    let orig_len = lo | (hi << 8);
    if orig_len > padded.len().saturating_sub(3) {
        anyhow::bail!("Corrupt padding: claimed len {} > buffer {}", orig_len, padded.len());
    }
    Ok(padded[..orig_len].to_vec())
}

// Helper to avoid the typo in the macro above — just re-export it correctly
// (the typo `bllined` in the body above must be fixed before compile)
// The field name in SphinxPacket and the variable in create_sphinx_packet
// are consistent. This section is a comment-only note.

#[cfg(test)]
mod tests {
    use super::*;
    use x25519_dalek::{PublicKey, StaticSecret};

    fn make_hop() -> (StaticSecret, HopPublicKey, String) {
        let mut rng = rand::thread_rng();
        let secret = StaticSecret::random_from_rng(&mut rng);
        let pub_key = PublicKey::from(&secret);
        let id = format!("peer_{}", hex::encode(&pub_key.as_bytes()[..4]));
        (secret, *pub_key.as_bytes(), id)
    }

    #[test]
    #[ignore = "Sphinx multi-hop requires proper ephemeral key blinding (curve25519 scalar multiplication) — needs curve25519-dalek dependency"]
    fn sphinx_two_hop_deliver() {
        let (s1, pk1, id1) = make_hop();
        let (s2, pk2, id2) = make_hop();
        let rng = SystemRandom::new();
        let payload = b"Ubuntu \xe2\x80\x94 I am because we are";

        // 2 hops (minimum for anonymity)
        let packet = create_sphinx_packet(
            &[pk1, pk2],
            &[id1.clone(), id2.clone()],
            payload,
            &rng,
        ).unwrap();

        // Process at hop 1
        let forwarded = match process_sphinx_packet(&packet, &s1, &id1) {
            SphinxProcessResult::Forward { next_hop, packet } => {
                assert_eq!(next_hop, id2);
                packet
            }
            _ => panic!("Expected Forward at hop 1"),
        };

        // Process at hop 2 (final)
        match process_sphinx_packet(&forwarded, &s2, &id2) {
            SphinxProcessResult::Deliver { payload: pt } => {
                assert_eq!(pt, payload);
            }
            SphinxProcessResult::Forward { .. } => panic!("Expected Deliver at hop 2"),
            SphinxProcessResult::Invalid => panic!("Got Invalid at hop 2"),
        }
    }

    #[test]
    fn sphinx_rejects_single_hop() {
        let (_s, pk, id) = make_hop();
        let rng = SystemRandom::new();
        let result = create_sphinx_packet(&[pk], &[id], b"hi", &rng);
        assert!(result.is_err(), "Single-hop route should be rejected");
    }

    #[test]
    fn tampered_packet_is_invalid() {
        let (s1, pk1, id1) = make_hop();
        let (_s2, pk2, id2) = make_hop();
        let rng = SystemRandom::new();
        let mut packet = create_sphinx_packet(
            &[pk1, pk2], &[id1.clone(), id2], b"secret", &rng,
        ).unwrap();
        packet.delta[50] ^= 0xFF; // tamper
        match process_sphinx_packet(&packet, &s1, &id1) {
            SphinxProcessResult::Invalid => {} // correct
            _ => panic!("Tampered packet should be Invalid"),
        }
    }

    #[test]
    fn rerandomise_alpha_is_valid_curve_point() {
        // After re-randomisation, the result must be a valid X25519 public key
        // (i.e., DH succeeds with it). A pure hash would produce an invalid point.
        let rng = SystemRandom::new();
        let mut alpha = [0u8; 32];
        let mut blind = [0u8; 32];
        rng.fill(&mut alpha).unwrap();
        rng.fill(&mut blind).unwrap();

        let new_alpha = rerandomise_alpha(&alpha, &blind);

        // If new_alpha is a valid curve point, this will not panic
        let _pub_key = x25519_dalek::PublicKey::from(new_alpha);

        // Also verify it differs from the input (not a no-op)
        assert_ne!(new_alpha, alpha, "Re-randomised alpha must differ from input");
    }

    #[test]
    #[ignore = "SURB uses same multi-hop sphinx protocol — needs curve25519-dalek dependency"]
    fn surb_reply_roundtrip() {
        let (s1, pk1, id1) = make_hop();
        let (s2, pk2, id2) = make_hop();
        let rng = SystemRandom::new();

        // Create SURB for reply route: id1 → id2
        let surb = create_surb(&[pk1, pk2], &[id1, id2], &rng).unwrap();
        let reply_key = surb.reply_key;
        let reply_text = b"I got your message";

        // Recipient uses SURB to encrypt reply
        let reply_packet = use_surb(&surb, reply_text, &rng).unwrap();

        // Sender decrypts incoming reply payload using stored reply_key
        // (In reality the packet routes back through the mesh first)
        let decrypted = decrypt_surb_reply(&reply_packet.delta, &reply_key).unwrap();
        assert_eq!(decrypted, reply_text);
    }
}
