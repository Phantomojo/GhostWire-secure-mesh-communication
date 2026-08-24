# ghostwire-dtn

[![Crates.io](https://img.shields.io/crates/v/ghostwire-dtn.svg)](https://crates.io/crates/ghostwire-dtn)
[![Documentation](https://docs.rs/ghostwire-dtn/badge.svg)](https://docs.rs/ghostwire-dtn)
[![License](https://img.shields.io/crates/l/ghostwire-dtn)](https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication)

Delay-Tolerant Networking with Spray-and-Wait + PRoPHET epidemic routing.

## Features

- **Spray-and-Wait** — efficient bundle distribution in intermittently connected networks
- **PRoPHET** — delivery predictability with time decay and transitivity
- **Bloom filter sync** — efficient set reconciliation during encounters
- **Bundle encryption** — X25519 ECDH + HKDF + XChaCha20-Poly1305
- **Bounded buffer** — 50MB max with LRU eviction
- **GNN-enhanced routing** — optional ML-based forwarding decisions

## Installation

```toml
[dependencies]
ghostwire-dtn = "0.1"
```

## Quick Start

```rust
use ghostwire_dtn::{DtnStore, DtnBundle};

let store = DtnStore::new([0x01u8; 32]);
let bundle = DtnBundle::new(vec![1, 2, 3, 4], [0x02u8; 32], None);
store.store(bundle).await;
```

## Reference

- Spyropoulos, T., et al. "Spray and Wait"
- Lindgren, A., et al. "Probabilistic Routing in Intermittently Connected Networks"

## License

Licensed under either of Apache License 2.0 or MIT license.
