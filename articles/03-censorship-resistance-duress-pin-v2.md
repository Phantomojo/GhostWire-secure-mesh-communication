---
title: "What Happens When Someone Forces You to Unlock Your Phone — I Built a Cryptographic Answer"
published: false
description: "A duress PIN system that opens a decoy profile — cryptographically independent, forensically clean, and believable under scrutiny. Built in Rust for mesh networks."
tags: [security, rust, privacy, discuss]
canonical_url: https://dev.to/phantomojo/duress-pin-cryptographic-decoy-profile-rust
cover_image: https://raw.githubusercontent.com/Phantomojo/GhostWire-secure-mesh-communication/main/.github/social-preview.png
series: GhostWire
---

Sarah (not her real name) was pulled aside at the airport. She typed her password. The officer scrolled through her messages, found a contact from a journalist she'd interviewed, and flagged her file. She spent six hours in a detention room while they "verified" her identity.

Sarah wasn't a criminal. She was a reporter who'd written about local government corruption.

The technology that failed her wasn't encryption. She had that. It was something harder: **coercion resistance** — the ability to prove you have nothing to hide, even when you do.

I built a mesh network called GhostWire. I'm a cybersecurity student in Nairobi, Kenya — a city where internet shutdowns during protests are not hypothetical, they're historical fact. The feature I'm proudest of isn't the AI router or the post-quantum crypto. It's the duress PIN.

## The Core Idea: Two Passwords, Two Identities

```
Real password   → Argon2id(password, salt_a) → real identity
Duress password → Argon2id(password, salt_b) → decoy identity
```

Two separate Argon2 derivations. Two separate namespaces. **Neither key can decrypt the other's namespace.** The attacker sees one unlock screen, gets one result. They can't tell which namespace they unlocked.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileKind {
    Real,
    Decoy,
}

pub struct UnlockResult {
    pub kind: ProfileKind,
    pub namespace_key: DerivedKey,     // 32-byte symmetric key
    pub namespace: String,
}
```

`ZeroizeOnDrop` clears the key from memory immediately. No forensic recovery from RAM dumps.

## The Argon2 Parameters

Tuned for ~1 second on a mid-range phone:

```rust
fn argon2_params() -> Params {
    Params::new(
        65536,  // 64 MB memory
        3,      // 3 iterations
        4,      // 4 parallel lanes
        Some(32),
    ).expect("valid Argon2 params")
}
```

Why 64 MB? Each password guess costs 64 MB of memory. At 3 iterations and 4 lanes, that's ~200ms per attempt on a modern CPU. A 12-character password with mixed case and symbols takes thousands of years to brute force.

The coercion config stores salts and BLAKE3 verifiers — not the keys:

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

The verifier: `BLAKE3(derived_key || "ghostwire coercion verifier v1")`. Check if the password is correct without storing the key. BLAKE3 is preimage-resistant — the verifier reveals nothing about the key.

## The Hardest Part: Making the Decoy Believable

An empty decoy is suspicious. "Why does your messaging app have no messages?"

The decoy namespace is pre-populated: 3-4 innocuous conversations with fictional contacts, a few "Hey, how are you?" messages, a weather query from last week. Nothing incriminating. Nothing that suggests encrypted communication is happening. Just a messaging app that someone barely uses.

This is the design challenge. The decoy must be **believable under scrutiny.** An attacker who sees zero messages and zero contacts knows something is hidden. A decoy with mundane activity passes the "coerced unlock" test.

## Defeating Traffic Analysis

A duress PIN protects against physical coercion. Network surveillance needs something else.

GhostWire routes through **Sphinx packets** — the same onion routing used by Nym and Lightning Network. Sphinx provides **bitwise unlinkability**: an adversary who sees a packet enter AND leave a node cannot determine they're the same message.

```rust
pub struct SphinxPacket {
    pub alpha: [u8; 32],  // Ephemeral DH public key
    pub beta: Vec<u8>,    // Encrypted routing info
    pub gamma: [u8; 32],  // Integrity check tag
    pub delta: Vec<u8>,   // Encrypted payload (layered)
}
```

Each hop decrypts one layer, learns only the next hop, and re-randomizes the packet. The SURB (Single-Use Reply Block) lets recipients reply without revealing the sender's identity — crucial for journalist-source communication.

## Traffic Morphing

Encrypted payloads look like encrypted payloads. Traffic morphing makes them look like something else:

```rust
pub struct MorphingConfig {
    pub enable_padding: bool,
    pub min_packet_size: usize,
    pub max_padding: usize,
    pub enable_timing_chaff: bool,
    pub max_chaff_delay_ms: u64,
}
```

Default: pad to 512 bytes, add up to 256 bytes of random padding, random delays up to 100ms. This defeats packet-size fingerprinting and timing correlation.

The padding uses `getrandom` (kernel CSPRNG), not `rand::thread_rng()`. Predictable padding is worse than no padding.

## What I Learned

**1. Coercion resistance is a threat model, not a feature.**

You can't add duress PINs as an afterthought. The storage layer must be namespace-aware. The UI must be identical for both profiles. The decoy needs maintenance (new messages have timestamps). Every detail matters because the adversary has your device and all the time they want.

**2. Traffic morphing is harder than encryption.**

Encryption is mathematically well-defined. Traffic analysis countermeasures are adversarial games. Our first padding implementation used `rand::thread_rng()` — a PRNG seeded with the current time. An attacker who knew the approximate packet creation time could narrow down the pad values.

**3. Sphinx is production-ready but poorly documented.**

The Nym implementation is the best reference. The academic paper misses implementation details. I spent two weeks reconciling the paper with the Nym codebase.

**4. The protocol is only as strong as the weakest client.**

If the desktop app has coercion resistance but the mobile port doesn't, the adversary knows to look at the phone. Every client must implement the same protections.

---

GhostWire is open source (AGPL-3.0). If you're interested in the code, the duress PIN implementation lives in the `encrypted_store` module:

- **GitHub:** https://github.com/Phantomojo/GhostWire-secure-mesh-communication
- **Website:** https://phantomojo.github.io/GhostWire-secure-mesh-communication/

We're actively looking for contributors — especially security researchers who can help audit the coercion resistance model.

**What's your threat model?** Are you building tools for high-risk users, or shipping for convenience with security as a checkbox? Drop a comment — I'm especially interested in hearing from people working on journalist safety tools and censorship circumvention.
