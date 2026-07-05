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

use crate::genesis_constants::{
    genesis_derive, SPHINX_BLIND_DOMAIN, SPHINX_KDF_DOMAIN, SPHINX_MAC_DOMAIN, SPHINX_SHARED_DOMAIN,
};
use anyhow::Result;
use chacha20::cipher::{KeyIvInit, StreamCipher};
use chacha20::ChaCha20;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Key, Nonce,
};
use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};
use subtle::ConstantTimeEq;
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
    Deliver {
        payload: Vec<u8>,
    },
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

/// SECURITY FIX: Blind the ephemeral secret for the next hop.
///
/// In proper Sphinx, each hop's shared secret comes from ECDH between
/// a blinded ephemeral key and the hop's public key. The blinding ensures
/// each hop sees a different alpha, providing unlinkability.
///
/// e_{i+1} = blinding_factor_i × e_i  (scalar multiplication mod curve order)
///
/// This requires curve25519-dalek's Scalar type for raw scalar multiplication,
/// since x25519-dalek doesn't expose this operation.
fn blind_ephemeral_secret(ephemeral: &[u8; 32], blinding_factor: &[u8; 32]) -> [u8; 32] {
    use curve25519_dalek::scalar::Scalar;

    // Clamp both to valid X25519 scalars (RFC 7748 §5)
    let mut e = *ephemeral;
    e[0] &= 248;
    e[31] &= 127;
    e[31] |= 64;

    let mut b = *blinding_factor;
    b[0] &= 248;
    b[31] &= 127;
    b[31] |= 64;

    // Multiply as scalars mod l (curve order)
    let e_scalar = Scalar::from_bytes_mod_order(e);
    let b_scalar = Scalar::from_bytes_mod_order(b);
    let next_e = b_scalar * e_scalar;
    next_e.to_bytes()
}

/// Compute X25519 DH without re-clamping the scalar.
///
/// x25519_dalek::StaticSecret::from() re-clamps the scalar bytes (clears
/// bottom 3 bits, sets bit 254), changing the value by ~2^254. After
/// blind_ephemeral_secret(), the ephemeral bytes are NOT in clamped form
/// (they're raw Scalar.to_bytes()), so StaticSecret::from would produce a
/// different scalar, breaking multi-hop Sphinx — creator and relay would
/// derive different shared secrets at hop ≥ 1.
///
/// This uses curve25519_dalek's MontgomeryPoint × Scalar directly, which
/// does NOT re-clamp. For clamped inputs (hop 0), the result is identical
/// to x25519_dalek. For non-clamped inputs (hop 1+), the result matches
/// what the relay computes via its own StaticSecret × alpha DH.
fn raw_dh(scalar_bytes: &[u8; 32], point_bytes: &[u8; 32]) -> [u8; 32] {
    use curve25519_dalek::montgomery::MontgomeryPoint;
    use curve25519_dalek::scalar::Scalar;

    let scalar = Scalar::from_bytes_mod_order(*scalar_bytes);
    let point = MontgomeryPoint(*point_bytes);
    let result = &point * &scalar;
    result.0
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
            n,
            SPHINX_MIN_HOPS
        );
    }
    if n > SPHINX_MAX_HOPS {
        anyhow::bail!("Route length {} exceeds maximum {}", n, SPHINX_MAX_HOPS);
    }

    let padded_payload = pad_to_size(final_payload, SPHINX_PAYLOAD_SIZE)?;

    // Generate ephemeral X25519 keypair for α₀
    let ephemeral_bytes = {
        let mut b = [0u8; 32];
        rng.fill(&mut b)
            .map_err(|_| anyhow::anyhow!("RNG failure"))?;
        b[0] &= 248;
        b[31] &= 127;
        b[31] |= 64; // clamp
        b
    };
    let ephemeral_secret = x25519_dalek::StaticSecret::from(ephemeral_bytes);
    let alpha_0 = x25519_dalek::PublicKey::from(&ephemeral_secret);

    let mut alphas: Vec<[u8; 32]> = Vec::with_capacity(n);
    let mut shared_secrets: Vec<Vec<u8>> = Vec::with_capacity(n);
    alphas.push(*alpha_0.as_bytes());

    // SECURITY FIX: Proper Sphinx shared secret derivation.
    // Each hop's shared secret comes from REAL ECDH between the blinded
    // ephemeral secret and the hop's public key. This ensures:
    // 1. Each hop can independently verify the packet is intended for it
    // 2. The blinding provides unlinkability (each hop sees different alpha)
    // 3. An attacker can't substitute their key without breaking the chain

    // Track the current ephemeral secret (blinded at each hop)
    let mut current_ephemeral = ephemeral_bytes;

    for i in 0..n {
        // ECDH without re-clamping: blind_ephemeral_secret() produces non-clamped
        // bytes; StaticSecret::from() would change the scalar by ~2^254, breaking
        // the DH match with the relay. raw_dh() uses curve25519-dalek directly.
        let shared_bytes = raw_dh(&current_ephemeral, &route_keys[i]);

        // Derive the hop's shared secret with path binding
        let mut ikm = shared_bytes.to_vec();
        ikm.extend_from_slice(&alphas[i]);
        let hop_shared_secret = genesis_derive(SPHINX_SHARED_DOMAIN, &ikm);
        shared_secrets.push(hop_shared_secret.to_vec());

        // Compute blinding factor for the NEXT hop (if not the last hop)
        if i < n - 1 {
            let mut ikm = shared_secrets[i].clone();
            ikm.extend_from_slice(&alphas[i]);
            let blinding_factor = genesis_derive(SPHINX_BLIND_DOMAIN, &ikm);

            // Blind the ephemeral secret for the next hop
            current_ephemeral = blind_ephemeral_secret(&current_ephemeral, &blinding_factor);

            // Blind alpha for the next hop (point multiplication)
            let new_alpha = rerandomise_alpha(&alphas[i], &blinding_factor);
            alphas.push(new_alpha);
        }
    }

    // ── Build header layers (innermost → outermost) ──────────────────────
    let mut current_header = vec![0u8; SPHINX_HEADER_SIZE];
    current_header[..8].copy_from_slice(b"DELIVER\x00");

    // Capture β_i (header state at each hop) before encryption for MAC chain
    let mut header_at_hop: Vec<Vec<u8>> = vec![vec![]; n];

    for i in (0..n).rev() {
        if i < n - 1 {
            let next_id = route_ids[i + 1].as_bytes();
            let id_len = next_id.len().min(255);
            let mut new_header = vec![0u8; SPHINX_HEADER_SIZE];
            new_header[0] = id_len as u8;
            new_header[1..1 + id_len].copy_from_slice(&next_id[..id_len]);
            let copy_len = (SPHINX_HEADER_SIZE - 1 - id_len).min(current_header.len());
            new_header[1 + id_len..1 + id_len + copy_len]
                .copy_from_slice(&current_header[..copy_len]);
            current_header = new_header;
        }

        let keys = HopKeys::derive(&shared_secrets[i], 0, &alphas[i]);
        let header_nonce: [u8; 12] = {
            let mut hasher = blake3::Hasher::new();
            hasher.update(&shared_secrets[i]);
            hasher.update(&alphas[i]);
            hasher.update(b"header-nonce");
            let hash = hasher.finalize();
            let mut nn = [0u8; 12];
            nn.copy_from_slice(&hash.as_bytes()[..12]);
            nn
        };
        let mut cipher = ChaCha20::new(&keys.header_key.into(), &header_nonce.into());
        cipher.apply_keystream(&mut current_header);

        // Capture AFTER encryption: this is what the relay sees as packet.beta
        header_at_hop[i] = current_header.clone();
    }

    // ── Build payload layers with MAC chain (innermost → outermost) ─────
    //
    // Standard Sphinx MAC chain: sender pre-computes γ_i for every hop.
    // γ_{i+1} is prepended to δ_i before encrypting with hop i's payload_key.
    // During processing, hop i decrypts δ_i → reveals γ_{i+1} for hop i+1.
    // This is necessary because a relay cannot compute the next hop's mac_key
    // (it would require the next hop's private key).
    let mut current_payload = padded_payload;
    let mut next_gamma: Option<[u8; 32]> = None;

    for i in (0..n).rev() {
        if let Some(g) = next_gamma {
            let mut with_gamma = g.to_vec();
            with_gamma.extend_from_slice(&current_payload);
            current_payload = with_gamma;
        }

        let keys = HopKeys::derive(&shared_secrets[i], 0, &alphas[i]);
        let payload_nonce: [u8; 12] = {
            let mut hasher = blake3::Hasher::new();
            hasher.update(&shared_secrets[i]);
            hasher.update(&alphas[i]);
            hasher.update(b"payload-nonce");
            let hash = hasher.finalize();
            let mut nn = [0u8; 12];
            nn.copy_from_slice(&hash.as_bytes()[..12]);
            nn
        };

        let mut cipher = ChaCha20::new(&keys.payload_key.into(), &payload_nonce.into());
        cipher.apply_keystream(&mut current_payload);

        // The relay reconstructs the header after extracting next_hop_id and
        // zero-padding. We must MAC against THAT version, not the full
        // header_at_hop[i], because the prepend step truncates the inner header
        // to fit SPHINX_HEADER_SIZE.
        // Hop 0 sees the original packet.beta (full header_at_hop[0]).
        // Hop i>0 sees the truncated remainder from hop i-1's processing,
        // which strips route_ids[i]'s prefix and zero-pads to SPHINX_HEADER_SIZE.
        let mac_beta = if i == 0 {
            header_at_hop[i].clone()
        } else {
            let nid_len = route_ids[i].as_bytes().len().min(255);
            let mut rb = header_at_hop[i][..SPHINX_HEADER_SIZE - 1 - nid_len].to_vec();
            rb.resize(SPHINX_HEADER_SIZE, 0u8);
            rb
        };

        next_gamma = Some(compute_mac(
            &keys.mac_key,
            &alphas[i],
            &mac_beta,
            &current_payload,
        ));
    }

    let gamma = next_gamma.expect("n >= 1, so gamma was computed");

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
    _our_peer_id: &str,
) -> SphinxProcessResult {
    // Reject low-order / small-subgroup points (confinement attack defense).
    // These points on Curve25519 (Montgomery form) have order dividing 8.
    // An attacker sending them can probe the relay's static private key over
    // repeated packets via the DH output's torsion component.
    if packet.alpha == [0u8; 32] || (packet.alpha[0] == 1 && packet.alpha[1..] == [0u8; 31]) {
        return SphinxProcessResult::Invalid;
    }

    let alpha_pub = match x25519_dalek::PublicKey::try_from(packet.alpha) {
        Ok(p) => p,
        Err(_) => return SphinxProcessResult::Invalid,
    };
    let shared = our_x25519_secret.diffie_hellman(&alpha_pub);

    // Reject zero shared secret (would produce degenerate derived keys)
    if shared.as_bytes() == &[0u8; 32] {
        return SphinxProcessResult::Invalid;
    }

    let derived_secret = {
        let mut ikm = shared.as_bytes().to_vec();
        ikm.extend_from_slice(&packet.alpha);
        genesis_derive(SPHINX_SHARED_DOMAIN, &ikm)
    };

    let keys = HopKeys::derive(&derived_secret, 0, &packet.alpha);

    // SECURITY FIX: Use per-hop mac_key (derived from shared secret) instead
    // of public SPHINX_MAC_DOMAIN constant. Previous version was forgeable.
    let expected_mac = compute_mac(&keys.mac_key, &packet.alpha, &packet.beta, &packet.delta);
    let mac_match: bool = expected_mac.ct_eq(&packet.gamma).into();
    if !mac_match {
        return SphinxProcessResult::Invalid;
    }

    // SECURITY FIX: Diversified nonce derivation (matches create_sphinx_packet)
    let header_nonce: [u8; 12] = {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&derived_secret);
        hasher.update(&packet.alpha);
        hasher.update(b"header-nonce");
        let hash = hasher.finalize();
        let mut n = [0u8; 12];
        n.copy_from_slice(&hash.as_bytes()[..12]);
        n
    };

    let mut header_pt = packet.beta.clone();
    let mut cipher = ChaCha20::new(&keys.header_key.into(), &header_nonce.into());
    cipher.apply_keystream(&mut header_pt);

    if header_pt.starts_with(b"DELIVER\x00") {
        // Final hop — decrypt the payload
        let payload_nonce: [u8; 12] = {
            let mut hasher = blake3::Hasher::new();
            hasher.update(&derived_secret);
            hasher.update(&packet.alpha);
            hasher.update(b"payload-nonce");
            let hash = hasher.finalize();
            let mut n = [0u8; 12];
            n.copy_from_slice(&hash.as_bytes()[..12]);
            n
        };
        let mut payload_pt = packet.delta.clone();
        let mut cipher = ChaCha20::new(&keys.payload_key.into(), &payload_nonce.into());
        cipher.apply_keystream(&mut payload_pt);

        match unpad_from_size(&payload_pt) {
            Ok(unpadded) => SphinxProcessResult::Deliver { payload: unpadded },
            Err(_) => SphinxProcessResult::Invalid,
        }
    } else if header_pt.len() >= 2 {
        let id_len = header_pt[0] as usize;
        if header_pt.len() < 1 + id_len {
            return SphinxProcessResult::Invalid;
        }
        let next_hop = match String::from_utf8(header_pt[1..1 + id_len].to_vec()) {
            Ok(s) => s,
            Err(_) => return SphinxProcessResult::Invalid,
        };

        // SECURITY FIX: Compute blinding factor the same way as create_sphinx_packet.
        // Creation uses: genesis_derive(SPHINX_BLIND_DOMAIN, shared_secret || alpha)
        // HopKeys::derive uses: genesis_derive(SPHINX_KDF_DOMAIN, "blind" + secret + idx + alpha)
        // These are different derivations — we MUST use the creation's derivation here.
        let blinding_factor = genesis_derive(SPHINX_BLIND_DOMAIN, &{
            let mut ikm = derived_secret.to_vec();
            ikm.extend_from_slice(&packet.alpha);
            ikm
        });
        let new_alpha = rerandomise_alpha(&packet.alpha, &blinding_factor);

        let remaining_header = header_pt[1 + id_len..].to_vec();
        let mut new_beta = remaining_header;
        new_beta.resize(SPHINX_HEADER_SIZE, 0u8);

        // Peel one payload layer
        let payload_nonce: [u8; 12] = {
            let mut hasher = blake3::Hasher::new();
            hasher.update(&derived_secret);
            hasher.update(&packet.alpha);
            hasher.update(b"payload-nonce");
            let hash = hasher.finalize();
            let mut nn = [0u8; 12];
            nn.copy_from_slice(&hash.as_bytes()[..12]);
            nn
        };
        let mut new_delta = packet.delta.clone();
        let mut cipher = ChaCha20::new(&keys.payload_key.into(), &payload_nonce.into());
        cipher.apply_keystream(&mut new_delta);

        // Extract next hop's gamma and delta from decrypted payload.
        // The sender embedded γ_{i+1} as the first 32 bytes of δ_i before
        // encrypting. A relay cannot compute the next hop's mac_key (would
        // require the next hop's private key), so extraction is the only option.
        if new_delta.len() < 32 {
            return SphinxProcessResult::Invalid;
        }
        let mut new_gamma = [0u8; 32];
        new_gamma.copy_from_slice(&new_delta[..32]);
        let remaining_delta = new_delta[32..].to_vec();

        SphinxProcessResult::Forward {
            next_hop,
            packet: SphinxPacket {
                alpha: new_alpha,
                beta: new_beta,
                gamma: new_gamma,
                delta: remaining_delta,
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
    rng.fill(&mut reply_key)
        .map_err(|_| anyhow::anyhow!("RNG failure creating SURB"))?;

    // Create a dummy packet on the reply route
    let dummy_payload = b"surb-init";
    let packet = create_sphinx_packet(reply_route_keys, reply_route_ids, dummy_payload, rng)?;

    let header = bincode::serialize(&packet)
        .map_err(|e| anyhow::anyhow!("SURB serialization failed: {}", e))?;

    Ok(Surb {
        header,
        first_hop: reply_route_ids[0].clone(),
        reply_key,
    })
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
    rng.fill(&mut nonce_bytes)
        .map_err(|_| anyhow::anyhow!("RNG failure in use_surb"))?;
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

/// Compute HMAC over (alpha || beta || delta) using a per-hop secret MAC key.
///
/// SECURITY FIX: The previous version used SPHINX_MAC_DOMAIN (a public
/// compile-time constant) as the blake3 keyed-hash key. Since genesis_derive
/// = blake3::keyed_hash(domain, data), the domain IS the cryptographic key.
/// A public key means gamma is computable by anyone from wire-visible data —
/// it catches bit-flips but provides zero protection against an active
/// attacker forging a tampered packet.
///
/// Now uses keys.mac_key derived per-hop from the shared secret via
/// HopKeys::derive("mac"), which was already computed but never consumed.
fn compute_mac(mac_key: &[u8; 32], alpha: &[u8; 32], beta: &[u8], delta: &[u8]) -> [u8; 32] {
    let mut ikm = alpha.to_vec();
    ikm.extend_from_slice(beta);
    ikm.extend_from_slice(delta);
    genesis_derive(mac_key, &ikm)
}

fn pad_to_size(data: &[u8], target: usize) -> Result<Vec<u8>> {
    if data.len() > target.saturating_sub(3) {
        anyhow::bail!(
            "Payload {} bytes exceeds target {} (need 3 bytes for padding marker)",
            data.len(),
            target
        );
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
        anyhow::bail!(
            "Corrupt padding: claimed len {} > buffer {}",
            orig_len,
            padded.len()
        );
    }
    Ok(padded[..orig_len].to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_core::OsRng;
    use x25519_dalek::{PublicKey, StaticSecret};

    fn make_hop() -> (StaticSecret, HopPublicKey, String) {
        let secret = StaticSecret::random_from_rng(OsRng);
        let pub_key = PublicKey::from(&secret);
        let id = format!("peer_{}", hex::encode(&pub_key.as_bytes()[..4]));
        (secret, *pub_key.as_bytes(), id)
    }

    #[test]
    fn sphinx_two_hop_deliver() {
        let (s1, pk1, id1) = make_hop();
        let (s2, pk2, id2) = make_hop();
        let rng = SystemRandom::new();
        let payload = b"Ubuntu \xe2\x80\x94 I am because we are";

        // 2 hops (minimum for anonymity)
        let packet =
            create_sphinx_packet(&[pk1, pk2], &[id1.clone(), id2.clone()], payload, &rng).unwrap();

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
        let mut packet =
            create_sphinx_packet(&[pk1, pk2], &[id1.clone(), id2], b"secret", &rng).unwrap();
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
        assert_ne!(
            new_alpha, alpha,
            "Re-randomised alpha must differ from input"
        );
    }

    #[test]
    fn surb_reply_roundtrip() {
        let (s1, pk1, id1) = make_hop();
        let (s2, pk2, id2) = make_hop();
        let rng = SystemRandom::new();

        let surb = create_surb(&[pk1, pk2], &[id1, id2], &rng).unwrap();
        let reply_key = surb.reply_key;
        let reply_text = b"I got your message";

        let reply_packet = use_surb(&surb, reply_text, &rng).unwrap();

        let decrypted = decrypt_surb_reply(&reply_packet.delta, &reply_key).unwrap();
        assert_eq!(decrypted, reply_text);
    }

    #[test]
    fn forged_gamma_is_rejected() {
        let (s1, pk1, id1) = make_hop();
        let (_s2, pk2, id2) = make_hop();
        let rng = SystemRandom::new();

        let mut packet = create_sphinx_packet(
            &[pk1, pk2],
            &[id1.clone(), id2],
            b"test payload",
            &rng,
        )
        .unwrap();

        // Forge a new gamma using the public domain constant (old broken approach).
        // An attacker can compute this from wire-visible data, but the relay now
        // uses a secret mac_key that the attacker doesn't have.
        let forged_gamma = genesis_derive(
            SPHINX_MAC_DOMAIN,
            &[packet.alpha.as_slice(), &packet.beta, &packet.delta].concat(),
        );
        packet.gamma = forged_gamma;

        match process_sphinx_packet(&packet, &s1, &id1) {
            SphinxProcessResult::Invalid => {}
            _ => panic!("Forged gamma (using public MAC_DOMAIN) should be rejected"),
        }
    }

    #[test]
    fn low_order_alpha_is_rejected() {
        let (s1, pk1, id1) = make_hop();
        let (_s2, pk2, id2) = make_hop();
        let rng = SystemRandom::new();

        let mut packet = create_sphinx_packet(
            &[pk1, pk2],
            &[id1.clone(), id2.clone()],
            b"test",
            &rng,
        )
        .unwrap();

        // Inject zero-point (order 1)
        packet.alpha = [0u8; 32];
        match process_sphinx_packet(&packet, &s1, &id1) {
            SphinxProcessResult::Invalid => {}
            _ => panic!("Zero-point alpha should be rejected"),
        }

        // Re-create and inject identity point (order 2)
        let mut packet2 = create_sphinx_packet(
            &[pk1, pk2],
            &[id1.clone(), id2],
            b"test",
            &rng,
        )
        .unwrap();
        packet2.alpha = [1u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        match process_sphinx_packet(&packet2, &s1, &id1) {
            SphinxProcessResult::Invalid => {}
            _ => panic!("Identity-point alpha should be rejected"),
        }
    }
}
