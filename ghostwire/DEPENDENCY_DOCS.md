# GhostWire Dependency Documentation Index

## Official Documentation Links

### Core Crates (with Links to docs.rs)

| Crate         | Version    | Docs URL                                               | Description                                 |
| ------------- | ---------- | ------------------------------------------------------ | ------------------------------------------- |
| **vodozemac** | 0.7        | [docs.rs](https://docs.rs/vodozemac/latest/vodozemac/) | Olm/Megolm Double Ratchet (Signal Protocol) |
| **libp2p**    | 0.56       | [docs.rs](https://docs.rs/libp2p/latest/lib2p/)        | Peer-to-peer networking                     |
| **tokio**     | 1.51       | [docs.rs](https://docs.rs/tokio/latest/tokio/)         | Async runtime                               |
| **axum**      | 0.7        | [docs.rs](https://docs.rs/axum/latest/axum/)           | Web framework                               |
| **ring**      | 0.17.12    | [docs.rs](https://docs.rs/ring/latest/ring/)           | Crypto primitives                           |
| **ort**       | 2.0.0-rc.4 | [docs.rs](https://docs.rs/ort/latest/ort/)             | ONNX Runtime AI                             |
| **ratatui**   | 0.29       | [docs.rs](https://docs.rs/ratatui/latest/ratatui/)     | Terminal UI                                 |
| **sled**      | 0.34       | [docs.rs](https://docs.rs/sled/latest/sled/)           | Embedded database                           |
| **sentry**    | 0.47       | [docs.rs](https://docs.rs/sentry/latest/sentry/)       | Error tracking                              |

## vodozemac Deep Dive

### What is Olm/Megolm?

- **Olm**: Double Ratchet algorithm (Signal Protocol) for 1:1 messaging
- **Megolm**: Single ratchet for group chats (shared key, no per-recipient encryption)

### Key Resources

- Main Docs: https://docs.rs/vodozemac/latest/vodozemac/
- GitHub: https://github.com/matrix-org/vodozemac
- Olm Spec: https://gitlab.matrix.org/matrix-org/olm/-/blob/master/docs/olm.md
- Megolm Spec: https://gitlab.matrix.org/matrix-org/olm/-/blob/master/docs/megolm.md
- Security Audit: https://matrix.org/media/Least%20Authority%20-%20Matrix%20vodozemac%20Final%20Audit%20Report.pdf

### Usage in GhostWire

```rust
use vodozemac::olm::{Account, Session, SessionConfig};

// Create account with identity keys
let account = Account::new();

// Generate one-time prekeys for peer discovery
account.generate_one_time_keys(5);
account.mark_keys_as_published();

// Create outbound session with peer's identity + OTK
let mut session = account.create_outbound_session(
    SessionConfig::version_1(),
    peer_identity_key,
    peer_one_time_key
)?;

// Encrypt message
let encrypted = session.encrypt("Hello")?;

// Decrypt message (on peer's side)
let plaintext = session.decrypt(&encrypted)?;
```

## libp2p Deep Dive

### Key Resources

- Main Docs: https://docs.rs/libp2p/latest/libp2p/
- Website: https://libp2p.io
- GitHub: https://github.com/libp2p/rust-libp2p

### GhostWire Usage

- GossipSub for pub/sub messaging
- Kademlia DHT for peer discovery
- mDNS for local discovery
- Noise for encrypted transport
- Yamux for stream multiplexing

## tokio Deep Dive

### Key Resources

- Main Docs: https://docs.rs/tokio/latest/tokio/
- Website: https://tokio.rs/tokio
- GitHub: https://github.com/tokio-rs/tokio

### GhostWire Usage

- Multi-threaded async runtime
- TCP/UDP networking
- Channels for message passing
- Timers and intervals
