# trust-store

[![Crates.io](https://img.shields.io/crates/v/trust-store.svg)](https://crates.io/crates/trust-store)
[![Documentation](https://docs.rs/trust-store/badge.svg)](https://docs.rs/trust-store)
[![License](https://img.shields.io/crates/l/trust-store)](https://github.com/Phantomojo/GhostWire-secure-mesh-communication)

Web of Trust with TOFU (Trust-On-First-Use) key verification for P2P networks.

## Features

- **TOFU key pinning** — pin a peer's public key on first contact
- **Key change detection** — flag potential MITM attacks
- **Social graph attestation** — peers vouch for each other's keys
- **Computed trust scores** — bubble trust through the graph with decay
- **Ed25519 signature verification** — cryptographically verify attestations

## Installation

```toml
[dependencies]
trust-store = "0.1"
```

## Quick Start

```rust
use trust_store::TrustStore;

let store = TrustStore::new();
let key = vec![0x01u8; 32];
let is_new = store.verify_or_pin("peer1", &key).await.unwrap();
assert!(is_new);
```

## License

Licensed under either of Apache License 2.0 or MIT license.
