---
title: "Building Censorship Resistance Into a Mesh Protocol — Duress PINs, Panic Wipe, and Coercion-Proof Identity"
published: false
description: "How GhostWire implements physical coercion resistance through parallel Argon2 namespaces, duress PINs that open decoy profiles, and a Sphinx onion routing layer that makes traffic analysis computationally infeasible."
tags: [security, privacy, rust, discuss]
canonical_url: https://dev.to/phantomojo/censorship-resistance-mesh-protocol-duress-pin
cover_image: https://raw.githubusercontent.com/Phantomojo/GhostWire/main/docs/assets/ghostwire-pfp.png
series: GhostWire
---

In 2013, Sarah (not her real name) was pulled aside at the airport and told to unlock her laptop. She typed her password. The officer scrolled through her messages, found a contact from a journalist she'd interviewed, and flagged her file. She spent six hours in a detention room while they "verified" her identity.

Sarah wasn't a criminal. She was a reporter who'd written about local government corruption in a country that doesn't call itself a surveillance state but behaves like one.

The technology that failed her wasn't encryption. She had that. It was **coercion resistance** — the fundamental inability to prove to a coercive actor that you have nothing to hide, because hiding the existence of your data is cryptographically harder than encrypting it.

I built a mesh network called GhostWire. It runs on Rust, uses libp2p for transport, and can route messages through LoRa relays across 10km of open terrain. But the feature I'm proudest of isn't the GNN router or the post-quantum key exchange. It's the duress PIN.

## The Architecture of Coercion

The idea is simple: two passwords, two identities, one application.

```
Real password  -> Argon2id(real_password, salt_a)  -> real identity namespace
Duress password -> Argon2id(duress_password, salt_b) -> decoy identity namespace
```

The EncryptedStore supports namespaces. This module manages two parallel namespaces behind two separate Argon2 derivations. **Neither key can decrypt the other's namespace** — they are cryptographically independent.

From the perspective of an attacker holding your device: they see one unlock screen, get one result. They cannot tell which namespace they unlocked without already knowing which password they used.

Here's how it's implemented:

```rust
/// Which profile a given password decrypts
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileKind {
    Real,
    Decoy,
}

/// The result of an unlock attempt
#[derive(Debug)]
pub struct UnlockResult {
    pub kind: ProfileKind,
    pub namespace_key: DerivedKey,     // 32-byte symmetric key
    pub namespace: String,             // namespace for EncryptedStore
}
```

The `DerivedKey` uses `ZeroizeOnDrop` — when the key goes out of scope, the memory is zeroed immediately. This prevents forensic recovery from RAM dumps.

## How Two Passwords Become Two Identities

The Argon2id parameters are tuned for ~1 second on a mid-range phone:

```rust
fn argon2_params() -> Params {
    Params::new(
        65536,  // 64 MB memory
        3,      // 3 iterations
        4,      // 4 parallel lanes
        Some(32), // 32-byte output
    ).expect("valid Argon2 params")
}
```

Why 64 MB? Memory-hard functions like Argon2id resist GPU and ASIC parallelization by requiring large memory per attempt. If an attacker wants to crack the password, each guess costs 64 MB of memory. At 3 iterations and 4 lanes, that's roughly 200ms per attempt on a modern CPU — thousands of years to brute force a 12-character password with mixed case and symbols.

The coercion config stores the salts and BLAKE3 verifiers (not the keys themselves):

```rust
pub struct CoercionConfig {
    pub real_salt_hex: String,
    pub duress_salt_hex: String,
    pub real_verifier_hex: String,   // BLAKE3(derived_key || domain_string)
    pub duress_verifier_hex: String,
    pub duress_enabled: bool,
    pub decoy_display_name: String,
}
```

The verifier is computed as: `BLAKE3(derived_key || "ghostwire coercion verifier v1")`. This lets the application check whether the correct password was entered without storing the key on disk. The verifier reveals nothing about the key — BLAKE3 is preimage-resistant.

## Building a Believable Decoy Profile

A decoy profile that's *empty* is suspicious. "Why does your messaging app have no messages?"

The decoy namespace is pre-populated with plausible content: 3-4 innocuous conversations with fictional contacts, a few "Hey, how are you?" messages, a weather query from last week. Nothing incriminating. Nothing that suggests *any* encrypted communication is happening. Just a messaging app that someone barely uses.

This is the hardest part of the design, because the decoy has to be *believable under scrutiny.* An attacker who sees a messaging app with zero messages and zero contacts will immediately know something is hidden. A decoy with carefully crafted mundane activity passes the "coerced unlock" test.

## Defeating Traffic Analysis

A duress PIN protects against physical coercion. But what about network-level surveillance?

GhostWire routes all traffic through **Sphinx packets** — the same onion routing format used by Nym and Lightning Network (BOLT #4). Sphinx (Danezis & Goldberg, 2009) provides **bitwise unlinkability**: an adversary who sees a packet enter a node AND a packet leave a node cannot determine they are the same message, even if they control both neighboring nodes.

```rust
/// Sphinx packet with bitwise unlinkability
pub struct SphinxPacket {
    pub alpha: [u8; 32],  // Ephemeral DH public key (rerandomized per hop)
    pub beta: Vec<u8>,    // Encrypted routing information
    pub gamma: [u8; 32],  // Integrity check tag
    pub delta: Vec<u8>,   // Encrypted payload (wrapped in layers)
}
```

Each hop decrypts one layer of the onion, learns only the next hop, and re-randomizes the packet's appearance. The per-hop key derivation uses BLAKE3 with domain-separated labels:

```rust
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
```

Minimum route length is 2 hops (we warn that production anonymity requires 3+). The SURB (Single-Use Reply Block) mechanism lets recipients reply without the sender revealing their identity — crucial for journalist-source communication.

## Pluggable Transports for Protocol Obfuscation

Encrypted payloads are identifiable as encrypted payloads. Traffic morphing makes them look like something else.

```rust
pub struct MorphingConfig {
    pub enable_padding: bool,          // Random packet padding
    pub min_packet_size: usize,        // Pad to at least this size
    pub max_padding: usize,            // Maximum random padding bytes
    pub enable_timing_chaff: bool,     // Random delays between packets
    pub max_chaff_delay_ms: u64,       // Maximum random delay
}
```

Default configuration: pad packets to at least 512 bytes, add up to 256 bytes of random padding, and introduce random delays of up to 100ms between packets. This defeats packet-size fingerprinting and timing correlation attacks.

The padding uses a CSPRNG (not `rand::thread_rng`) — predictable padding is worse than no padding because an adversary who can predict your padding patterns can filter them out.

## What I Learned About Building Coercion-Resistant Systems

### 1. Coercion resistance is not a feature. It's a threat model.

You can't add duress PINs as an afterthought. The entire storage layer must be namespace-aware. The UI must be identical for both profiles. The decoy profile needs maintenance (new messages have timestamps). Every detail matters because the adversary has your device in their hands and all the time they want.

### 2. Traffic morphing is harder than encryption.

Encryption is mathematically well-defined. Traffic analysis countermeasures are adversarial games. Our first padding implementation used `rand::thread_rng()` — a PRNG seeded with the current time. An attacker who knew the approximate time of packet creation could narrow down the possible pad values. We switched to `getrandom` (kernel CSPRNG) for all cryptographic operations.

### 3. Sphinx is production-ready but poorly documented

The Nym implementation is the best reference. The academic paper is precise but misses implementation details (key derivation domain separation, SURB format). I spent two weeks reconciling the paper with the Nym codebase.

### 4. The protocol is only as strong as the weakest client

If the desktop app has coercion resistance but the mobile port doesn't, the adversary knows to look at the phone. Every client must implement the same protections.

---

**What's your threat model?** Are you building tools for high-risk users, or shipping for convenience with security as a checkbox? The distinction matters more than most developers realize. Drop a comment — I'm especially interested in hearing from people working on journalist safety tools and censorship circumvention.
