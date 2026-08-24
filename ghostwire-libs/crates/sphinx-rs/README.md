# sphinx-rs

[![Crates.io](https://img.shields.io/crates/v/sphinx-rs.svg)](https://crates.io/crates/sphinx-rs)
[![Documentation](https://docs.rs/sphinx-rs/badge.svg)](https://docs.rs/sphinx-rs)
[![License](https://img.shields.io/crates/l/sphinx-rs)](https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication)
[![Rust](https://img.shields.io/badge/rust-1.75+-blue.svg)](https://www.rust-lang.org)

A production-grade implementation of the [Sphinx](https://www.freehaven.net/anonbib/cache/sphinx.pdf) onion routing protocol with Single-Use Reply Blocks (SURBs).

## Features

- **Multi-hop onion routing** — Up to 5 hops with X25519 alpha re-randomization for bitwise unlinkability
- **SURBs (Single-Use Reply Blocks)** — Anonymous reply channels for bidirectional communication
- **XChaCha20-Poly1305** — Authenticated encryption for SURB payloads
- **BLAKE3 KDF** — Fast, 256-bit security key derivation
- **Constant-time MAC** — Timing-safe comparison via `subtle` crate
- **Zeroize on drop** — All intermediate secrets wiped from memory
- **Configurable domain constants** — No hardcoded ceremony values; works out of the box
- **Typed errors** — Full `thiserror` enum for pattern-matching on failure modes
- **Generic RNG** — Accepts any `RngCore + CryptoRng` (not tied to `ring`)

## Installation

```toml
[dependencies]
sphinx-rs = "0.1"
```

Requires Rust 1.75+.

## Quick Start

```rust
use sphinx_rs::{SphinxConfig, create_sphinx_packet, process_sphinx_packet, SphinxProcessResult};
use x25519_dalek::{StaticSecret, PublicKey};
use rand::thread_rng;

// Generate keypairs for 3 hops
let mut rng = thread_rng();
let config = SphinxConfig::default();

let hops: Vec<_> = (0..3).map(|i| {
    let secret = StaticSecret::random_from_rng(&mut rng);
    let public = PublicKey::from(&secret);
    (secret, public, format!("hop_{}", i))
}).collect();

let route_keys: Vec<[u8; 32]> = hops.iter().map(|(_, pk, _)| *pk.as_bytes()).collect();
let route_ids: Vec<String> = hops.iter().map(|(_, _, id)| id.clone()).collect();

// Create and route a Sphinx packet
let packet = create_sphinx_packet(&route_keys, &route_ids, b"secret", &mut rng, &config).unwrap();

// Process at each hop
let mut current = packet;
for (secret, _, _) in &hops {
    match process_sphinx_packet(&current, secret, &config) {
        SphinxProcessResult::Forward { next_hop, packet } => {
            println!("Forwarding to {}", next_hop);
            current = packet;
        }
        SphinxProcessResult::Deliver { payload } => {
            println!("Delivered: {:?}", payload);
        }
        SphinxProcessResult::Invalid => panic!("Invalid packet!"),
    }
}
```

Run the full example:

```bash
cargo run --example basic_usage
```

## SURB (Anonymous Replies)

```rust
use sphinx_rs::{create_surb, use_surb, decrypt_surb_reply, SphinxConfig};

// Sender creates a SURB along with their message
let surb = create_surb(&reply_route_keys, &reply_route_ids, &mut rng, &config)?;

// Recipient uses the SURB to send an anonymous reply
let reply_packet = use_surb(&surb, b"reply message", &mut rng)?;

// ... route reply_packet through the SURB route ...

// Sender decrypts the incoming reply
let decrypted = decrypt_surb_reply(&reply_payload, &surb.reply_key)?;
```

## Security Model

Sphinx provides **anonymity** (sender/receiver unlinkability) through:

1. **Layered encryption** — Each hop decrypts exactly one layer
2. **Alpha re-randomization** — Each hop sees a different ephemeral key (real X25519 scalar multiplication)
3. **Fixed packet size** — All packets are the same size (hides message length)
4. **Constant-time MAC** — No timing leaks on verification

### Threat Model

| Attack | Mitigation |
|---|---|
| Traffic correlation (entry/exit) | Alpha re-randomization (unlinkable) |
| Packet size analysis | Fixed-size packets (1024 + 1024 bytes) |
| Replay attacks | Higher-layer nonce/epoch (not built-in) |
| Timing analysis | Cover traffic (not built-in) |
| Compromised hops | Need ALL hops to deanonymize |

### Recommendations

- Use **≥ 3 hops** for production anonymity (2 is minimum for testing)
- Add **cover traffic** to defeat timing analysis
- Use **constant-size packets** (already enforced)
- Implement **replay protection** at your application layer

## Comparison to Other Implementations

| Feature | sphinx-rs | Nym mixnet-sphinx | Lightning BOLT 4 |
|---|---|---|---|
| Curve | X25519 | X25519 | secp256k1 |
| KDF | BLAKE3 | HKDF-SHA256 | SHA256 |
| Encryption | ChaCha20 + XChaCha20-Poly1305 | ChaCha20-Poly1305 | ChaCha20+HMAC |
| SURB | ✅ Built-in | ❌ | ❌ |
| Max hops | 5 | Configurable | 20 |
| Constant-time MAC | ✅ | ✅ | ✅ |
| Typed errors | ✅ (thiserror) | ✅ (thiserror) | ✅ (thiserror) |
| Configurable domains | ✅ | ❌ (hardcoded) | ❌ (hardcoded) |

**Unique advantage:** Only Rust Sphinx with SURB support. BLAKE3 is faster than SHA256.

## Feature Flags

| Feature | Default | Description |
|---|---|---|
| `std` | ✅ | Enable std library features |
| `surb` | ✅ | Enable Single-Use Reply Blocks |

To disable SURBs (smaller binary):

```toml
[dependencies]
sphinx-rs = { version = "0.1", default-features = false, features = ["std"] }
```

## API Reference

See [docs.rs/sphinx-rs](https://docs.rs/sphinx-rs) for full documentation.

### Core Functions

- [`create_sphinx_packet()`](https://docs.rs/sphinx-rs/latest/sphinx_rs/fn.create_sphinx_packet.html) — Create a multi-hop anonymous packet
- [`process_sphinx_packet()`](https://docs.rs/sphinx-rs/latest/sphinx_rs/fn.process_sphinx_packet.html) — Process at a hop (Forward/Deliver/Invalid)
- [`create_surb()`](https://docs.rs/sphinx-rs/latest/sphinx_rs/fn.create_surb.html) — Create a Single-Use Reply Block
- [`use_surb()`](https://docs.rs/sphinx-rs/latest/sphinx_rs/fn.use_surb.html) — Send an anonymous reply via SURB
- [`decrypt_surb_reply()`](https://docs.rs/sphinx-rs/latest/sphinx_rs/fn.decrypt_surb_reply.html) — Decrypt an incoming SURB reply

### Types

- [`SphinxPacket`](https://docs.rs/sphinx-rs/latest/sphinx_rs/struct.SphinxPacket.html) — The wire-format packet
- [`SphinxProcessResult`](https://docs.rs/sphinx-rs/latest/sphinx_rs/enum.SphinxProcessResult.html) — Forward / Deliver / Invalid
- [`Surb`](https://docs.rs/sphinx-rs/latest/sphinx_rs/struct.Surb.html) — Single-Use Reply Block
- [`SphinxConfig`](https://docs.rs/sphinx-rs/latest/sphinx_rs/struct.SphinxConfig.html) — Configurable domain constants
- [`SphinxError`](https://docs.rs/sphinx-rs/latest/sphinx_rs/enum.SphinxError.html) — Typed error enum

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Reference

- Danezis, G., & Goldberg, I. (2009). [Sphinx: A Compact and Provably Secure Mix Format](https://www.freehaven.net/anonbib/cache/sphinx.pdf). IEEE Symposium on Security and Privacy.
