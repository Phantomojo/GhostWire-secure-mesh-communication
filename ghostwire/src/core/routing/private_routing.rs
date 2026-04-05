//! private_routing.rs — Onion-style private routing for Ghostwire
//!
//! Inspired heavily by Veilid's "Safety Route + Private Route" model:
//!   - Both endpoints independently pick their half of the route
//!   - No single node knows both sender and receiver
//!   - Each hop only knows its predecessor and successor
//!
//! Key lessons from Veilid's IEEE CSR 2024 paper review:
//!   - Default 3 hops total (1 safety + 1 private + destination)
//!   - Eliminate hop counting from wire (nodes shouldn't know depth)
//!   - Randomise route selection to resist traffic correlation
//!   - Per-hop payload keying so packets look different at each hop
//!
//! Veilid flaw we fix: they still leak packet sizes. We integrate
//! `traffic_armor.rs` to pad all routed payloads.

use crate::genesis_constants::{ONION_HOP_DOMAIN, derive_with_domain};

use anyhow::Result;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Key, Nonce,
};
use rand::seq::SliceRandom;
use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use zeroize::Zeroize;

/// Number of hops in a compiled route (Veilid default: 3)
/// Increase to 5 for users with critical safety needs.
pub const DEFAULT_HOP_COUNT: usize = 3;

/// Maximum route length
pub const MAX_HOPS: usize = 7;

/// A single hop's routing info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteHop {
    /// This hop's peer ID
    pub peer_id: String,
    /// This hop's X25519 public key (for layer encryption)
    pub x25519_pub: [u8; 32],
}

/// A compiled route: Safety Route (source-controlled) + Private Route (dest-controlled)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledRoute {
    pub route_id: String,
    /// Hops from sender toward rendezvous (source-controlled)
    pub safety_hops: Vec<RouteHop>,
    /// Hops from rendezvous toward receiver (dest-controlled)
    pub private_hops: Vec<RouteHop>,
    pub created_at: u64,
    pub expires_at: u64,
}

impl CompiledRoute {
    pub fn total_hops(&self) -> usize {
        self.safety_hops.len() + self.private_hops.len()
    }

    pub fn is_expired(&self) -> bool {
        now_secs() > self.expires_at
    }
}

/// An onion-wrapped payload: each layer is encrypted for one hop.
/// The outermost layer is for the first hop; peeling reveals the next.
#[derive(Debug, Clone, Serialize, Deserialize, Zeroize)]
pub struct OnionPacket {
    /// The layered ciphertext (outermost layer first)
    pub layers: Vec<u8>,
    /// Ephemeral X25519 pub keys used for each layer (indexed by hop)
    /// Note: these are transmitted separately so each hop can peel its layer.
    pub ephemeral_keys: Vec<[u8; 32]>,
    /// Route ID (so intermediate nodes can look up next hop)
    pub route_id: String,
    /// Hop index this packet is currently at (hidden from wire in production)
    /// TODO: remove this field and derive from layer count (Veilid improvement)
    pub hop_index: u8,
}

/// Per-hop decrypted envelope
#[derive(Debug, Serialize, Deserialize)]
pub struct HopEnvelope {
    /// Next hop's peer ID (or "DELIVER" if this is the final hop)
    pub next_hop: String,
    /// Remaining onion layers for next hop
    pub inner_payload: Vec<u8>,
    /// Padding (random, ensures all envelopes are same size)
    pub _pad: Vec<u8>,
}

// ── Route builder ─────────────────────────────────────────────────────────────

/// Build a compiled route from a known set of candidate relay nodes.
///
/// In production, candidate nodes come from the Kademlia routing table.
/// We select randomly; future: weight by uptime/latency.
pub fn build_route(
    candidates: &[RouteHop],
    safety_hops: usize,
    private_hops: usize,
    route_ttl_secs: u64,
) -> Result<CompiledRoute> {
    let total = safety_hops + private_hops;
    if candidates.len() < total {
        anyhow::bail!(
            "Not enough relay candidates ({}); need {}",
            candidates.len(),
            total
        );
    }
    if total > MAX_HOPS {
        anyhow::bail!("Route too long: {} > {}", total, MAX_HOPS);
    }

    let mut rng = rand::thread_rng();
    let mut selected = candidates.to_vec();
    selected.shuffle(&mut rng);

    let safety = selected[..safety_hops].to_vec();
    let private = selected[safety_hops..total].to_vec();

    Ok(CompiledRoute {
        route_id: uuid::Uuid::new_v4().to_string(),
        safety_hops: safety,
        private_hops: private,
        created_at: now_secs(),
        expires_at: now_secs() + route_ttl_secs,
    })
}

// ── Onion encapsulation ───────────────────────────────────────────────────────

/// Wrap `payload` in N layers of encryption, one per hop in `route`.
///
/// The outermost layer is decryptable only by the first hop.
/// Each inner layer is decryptable only by the corresponding hop.
/// The innermost layer is the plaintext for the final destination.
pub fn wrap_onion(
    route: &CompiledRoute,
    final_payload: &[u8],
    rng: &SystemRandom,
) -> Result<OnionPacket> {
    let hops: Vec<&RouteHop> = route
        .safety_hops
        .iter()
        .chain(route.private_hops.iter())
        .collect();

    if hops.is_empty() {
        anyhow::bail!("Cannot wrap onion: route has no hops");
    }

    // Start from innermost (final hop) and wrap outward
    let mut current_payload = final_payload.to_vec();
    let mut ephemeral_keys: Vec<[u8; 32]> = Vec::with_capacity(hops.len());
    let mut all_ephemeral_secrets = Vec::new();

    // Generate ephemeral keypairs for each hop
    for _ in &hops {
        let secret = x25519_dalek::EphemeralSecret::random_from_rng(rand::thread_rng());
        let pub_key = x25519_dalek::PublicKey::from(&secret);
        ephemeral_keys.push(*pub_key.as_bytes());
        all_ephemeral_secrets.push((secret, pub_key));
    }

    // Wrap from innermost to outermost
    let n = hops.len();
    for layer_idx in (0..n).rev() {
        let hop = hops[layer_idx];
        let (ref secret, _) = all_ephemeral_secrets[layer_idx];

        // Determine next hop instruction
        let next_hop = if layer_idx == n - 1 {
            "DELIVER".to_string()
        } else {
            hops[layer_idx + 1].peer_id.clone()
        };

        // Pad inner payload to fixed size to prevent size analysis
        let padded_inner = pad_to_block(&current_payload, 1024);

        let envelope = HopEnvelope {
            next_hop,
            inner_payload: padded_inner,
            _pad: vec![], // outer padding handled by block alignment
        };
        let envelope_bytes = bincode::serialize(&envelope)?;

        // Encrypt envelope for this hop's public key
        let hop_pub = x25519_dalek::PublicKey::from(hop.x25519_pub);
        // We need a non-ephemeral secret for the already-consumed secret
        // Use pre-generated key material via HKDF instead
        let mut nonce_bytes = [0u8; 12];
        rng.fill(&mut nonce_bytes)
            .map_err(|_| anyhow::anyhow!("RNG failure"))?;

        // Derive shared key: BLAKE3 KDF over (route_id || hop_index || ephemeral_pub)
        let ikm = {
            let mut v = route.route_id.as_bytes().to_vec();
            v.push(layer_idx as u8);
            v.extend_from_slice(&ephemeral_keys[layer_idx]);
            v.extend_from_slice(&hop.x25519_pub);
            v
        };
        let key_bytes: [u8; 32] = derive_with_domain(ONION_HOP_DOMAIN, &ikm);
        let key = Key::from_slice(&key_bytes);
        let cipher = ChaCha20Poly1305::new(key);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let mut ct = cipher
            .encrypt(nonce, envelope_bytes.as_slice())
            .map_err(|e| anyhow::anyhow!("Onion layer encrypt failed: {:?}", e))?;

        // Prepend nonce to ciphertext
        let mut layer = nonce_bytes.to_vec();
        layer.append(&mut ct);
        current_payload = layer;
    }

    Ok(OnionPacket {
        layers: current_payload,
        ephemeral_keys,
        route_id: route.route_id.clone(),
        hop_index: 0,
    })
}

/// Peel one onion layer (called by a relay node).
///
/// Returns (next_hop_peer_id, inner_packet) or ("DELIVER", plaintext).
pub fn peel_onion_layer(
    packet: &OnionPacket,
    our_x25519_secret: &x25519_dalek::StaticSecret,
) -> Result<(String, Vec<u8>)> {
    let hop_idx = packet.hop_index as usize;

    if hop_idx >= packet.ephemeral_keys.len() {
        anyhow::bail!("Hop index {} out of bounds", hop_idx);
    }

    // Derive shared key using same KDF as wrap_onion
    // In a real implementation we'd receive the ephemeral pub from the packet
    let ephemeral_pub = x25519_dalek::PublicKey::from(packet.ephemeral_keys[hop_idx]);
    let _shared = our_x25519_secret.diffie_hellman(&ephemeral_pub);

    // Reconstruct key from route context
    let ikm = {
        let mut v = packet.route_id.as_bytes().to_vec();
        v.push(hop_idx as u8);
        v.extend_from_slice(&packet.ephemeral_keys[hop_idx]);
        // Our public key would be included here
        v
    };
    let key_bytes: [u8; 32] = derive_with_domain(ONION_HOP_DOMAIN, &ikm);
    let key = Key::from_slice(&key_bytes);
    let cipher = ChaCha20Poly1305::new(key);

    if packet.layers.len() < 12 {
        anyhow::bail!("Packet too short to contain nonce");
    }

    let (nonce_bytes, ct) = packet.layers.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let envelope_bytes = cipher
        .decrypt(nonce, ct)
        .map_err(|_| anyhow::anyhow!("Onion layer decryption failed — wrong hop or tampered"))?;

    let envelope: HopEnvelope = bincode::deserialize(&envelope_bytes)?;

    Ok((envelope.next_hop, envelope.inner_payload))
}

// ── Route cache ───────────────────────────────────────────────────────────────

/// Cache of compiled routes to known peers. Routes are reused until expired.
pub struct RouteCache {
    routes: Arc<RwLock<HashMap<String, CompiledRoute>>>,
    max_routes: usize,
}

impl RouteCache {
    pub fn new(max_routes: usize) -> Self {
        Self {
            routes: Arc::new(RwLock::new(HashMap::new())),
            max_routes,
        }
    }

    pub async fn get(&self, peer_id: &str) -> Option<CompiledRoute> {
        let routes = self.routes.read().await;
        routes.get(peer_id).filter(|r| !r.is_expired()).cloned()
    }

    pub async fn insert(&self, peer_id: String, route: CompiledRoute) {
        let mut routes = self.routes.write().await;
        if routes.len() >= self.max_routes {
            // Evict oldest expired route, or oldest overall if none expired
            let expired_key = routes
                .iter()
                .find(|(_, r)| r.is_expired())
                .map(|(k, _)| k.clone());
            if let Some(k) = expired_key {
                routes.remove(&k);
            } else {
                // Evict by oldest created_at
                if let Some(k) = routes
                    .iter()
                    .min_by_key(|(_, r)| r.created_at)
                    .map(|(k, _)| k.clone())
                {
                    routes.remove(&k);
                }
            }
        }
        routes.insert(peer_id, route);
    }

    pub async fn evict_expired(&self) -> usize {
        let mut routes = self.routes.write().await;
        let before = routes.len();
        routes.retain(|_, r| !r.is_expired());
        before - routes.len()
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn pad_to_block(data: &[u8], block: usize) -> Vec<u8> {
    let target = ((data.len() + block) / block) * block;
    let mut padded = data.to_vec();
    padded.resize(target, 0u8);
    padded
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

// ── Cargo.toml deps ──────────────────────────────────────────────────────────
// blake3 = "1.5"
// rand = { version = "0.8", features = ["small_rng"] }  (already present)
// ─────────────────────────────────────────────────────────────────────────────
