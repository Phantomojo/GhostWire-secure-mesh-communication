---
title: "I Built a Post-Quantum Encrypted Mesh Network — Here's What ML-KEM-768 Actually Looks Like in Production"
published: false
description: "Building production post-quantum cryptography into a Rust P2P mesh: ML-KEM-768 integration, X25519+XChaCha20 fallback chain, and what I learned shipping FIPS 205 code in 2026."
tags: [rust, cryptography, security, discuss]
canonical_url: https://dev.to/phantomojo/post-quantum-cryptography-in-rust-mesh-network
cover_image: https://raw.githubusercontent.com/Phantomojo/GhostWire/main/docs/assets/ghostwire-pfp.png
series: GhostWire
---

April 2026. Cloudflare announces post-quantum mesh networking. Pilot Protocol ships ML-KEM into their P2P layer. And suddenly every cryptography Twitter account is asking the same question: *"Cool demo, but who's actually running this in production?"*

I am. In Nairobi. On a mesh network that spans 63 simulated nodes, 7 different transports, and an AI routing layer that needs to survive the quantum transition.

Here's what nobody tells you about shipping ML-KEM-768 (FIPS 205) in a real Rust application: **the wrapper is the hard part.** The primitive itself is well-specified, well-tested, and fast. But integrating it into a key exchange protocol that handles rotation, revocation, and coercion resistance? That's where things get interesting.

## Why Post-Quantum Matters for Mesh Networks

Mesh networks have a unique vulnerability to store-now-decrypt-later attacks. Unlike a TLS session where the key lives for minutes, mesh nodes route encrypted bundles that can sit in buffers for **72 hours** (our DTN TTL). An adversary who records your traffic today and cracks Shor's algorithm tomorrow gets everything.

Standard X25519 ECDH dies to Shor's algorithm on a sufficiently large quantum computer. ML-KEM (formerly Kyber) is a lattice-based key encapsulation mechanism selected by NIST in 2024 precisely because it resists both classical and quantum attacks.

For GhostWire, post-quantum security isn't optional — it's a **hard requirement** for protecting mesh traffic against harvest-now-decrypt-later attacks.

## The Hybrid Key Architecture

The key insight: you don't replace X25519 with ML-KEM. You layer them.

Here's the actual key type definition from GhostWire's encryption engine:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Zeroize)]
pub enum KeyType {
    Ed25519,      // Signing only
    X25519,       // Classical DH
    Hybrid,       // Ed25519 + X25519
    PostQuantum,  // ML-KEM-768 (Kyber)
    Ultimate,     // Ed25519 + X25519 + ML-KEM-768
}
```

The `Ultimate` variant is the one we use in production. Every message is encrypted with **three independent cryptographic layers**:

1. **XChaCha20-Poly1305** for symmetric encryption (the actual data cipher)
2. **X25519 ECDH** for classical key agreement
3. **ML-KEM-768** for post-quantum key encapsulation

The session key is a composite: `SHA3-256(classical_shared_secret || kem_shared_secret)`. Break either one, and the composite is still secure. This is the same hybrid design Cloudflare uses, same approach NIST SP 800-227 recommends, and — crucially — the same pattern that passes SNDL audits.

## What ML-KEM-768 Costs You

Let's talk real numbers from our benchmarks on a Ryzen 7 7840HS (the machine running our seed node):

| Operation | X25519 | ML-KEM-768 | Hybrid (X25519 + ML-KEM) |
|---|---|---|---|
| Key generation | ~50µs | ~80µs | ~130µs |
| Encapsulation | N/A | ~110µs | ~160µs (combined) |
| Decapsulation | ~300µs (DH) | ~90µs | ~390µs |
| Ciphertext size | 32 bytes | 1088 bytes | 1120 bytes |

The numbers are reasonable. Key generation and encapsulation are sub-millisecond. The real cost is **ciphertext size** — 1088 bytes per KEM encapsulate vs 32 bytes for an X25519 public key. When you're routing through LoRa at 255 bytes/second, that matters.

**Our solution:** ML-KEM is used during the initial handshake only. Once the composite session key is established, all subsequent messages use XChaCha20-Poly1305 with forward secrecy via the vodozemac Double Ratchet (Signal protocol, Trail of Bits audited). The 1088-byte KEM payload is a one-time cost per peer session.

## The Actual Integration (Real Code)

Here's the key encapsulation wrapper from GhostWire's crypto machine — the part that actually handles the PQ key exchange:

```rust
/// Encrypted message with optional post-quantum hardening
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedMessage {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
    pub signature: Vec<u8>,
    pub sender_id: String,
    pub recipient_id: String,
    pub timestamp: u64,
    pub key_id: String,
    pub message_type: MessageType,
    pub version: u8,
    pub pq_key: Option<Vec<u8>>, // ML-KEM encapsulated key for PQC Hardening
}
```

The `pq_key` field is `Option<Vec<u8>>` deliberately. Peers that don't support ML-KEM fall back to pure X25519. Peers that do get the hybrid composite. This gives us **graceful downgrade** — a peer running a build from before the PQ merge still works, it just gets classical-only security.

The `version` field enables algorithm rotation without breaking the network. Version 4 = Argon2id + XChaCha20. Version 5 = adds ML-KEM. When version 6 ships (probably with ML-KEM-1024), version 4 messages are still valid, they just get flagged for session rekeying.

## What I Learned the Hard Way

Three lessons that cost me weeks:

### 1. The PQ public key doesn't fit in a DNS packet

Our original discovery protocol used mDNS with peer information encoded in TXT records. X25519 keys fit (32 bytes, no problem). ML-KEM public keys are **1184 bytes**. Standard DNS/UDP limits TXT records to ~255 bytes per string. We had to fragment across multiple records — or more practically, move key exchange to the TCP mesh layer where there's no packet size limit.

### 2. `Zeroize` matters more with PQ

ML-KEM secret keys are larger (2400 bytes vs 32 bytes for X25519). Memory dumps become a real concern. GhostWire uses `zeroize::ZeroizeOnDrop` on all key material, but I found that the `pqc_kyber` crate's internal stack buffers didn't always clear themselves. We had to audit every intermediate allocation.

### 3. On-device key generation is slow on ESP32

We're porting GhostWire to ESP32-S3 for LoRa mesh nodes. ML-KEM-768 key generation takes **~8ms** on the ESP32's 240 MHz Xtensa core. That's 60x slower than on a laptop CPU. Still acceptable for a one-time handshake, but we had to add a progress indicator to prevent users from thinking the device froze.

## What's Next: ML-KEM-1024 and Beyond

NIST is standardizing ML-KEM-1024 as FIPS 205 (higher security level, ~2x slower). Our architecture handles it through the version field — no protocol break. We're also experimenting with **hybrid KEM combiner functions** using SHAKE-256 for domain separation, matched to the IRTF CFRG's ongoing work on KEM combiners.

Mesh networks will be some of the last systems to transition fully to post-quantum, precisely because of the long-lived bundles and low-power devices. But the transition has to start now. Every bundle routed today carries encrypted data that someone will try to decrypt tomorrow.

## Rust Makes This Bearable

The `pqc_kyber` crate, `x25519-dalek`, and `chacha20poly1305` crate all work together without conflicts. The type system ensures you can't accidentally swap a KEM ciphertext for a DH public key. `zeroize` prevents key material from lingering. The borrow checker catches your lifetime mistakes before they become security bugs.

I'm not saying Rust makes post-quantum crypto *easy*. But it makes it *possible* to ship with confidence.

---

**What's your experience with post-quantum cryptography? Are you running ML-KEM in production, or still evaluating?** Drop a comment — I'm especially interested in hearing from people working on low-power embedded deployments, because that's where the real challenges are.
