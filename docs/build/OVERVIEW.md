# Build on GhostWire

**GhostWire is a protocol, not just an application.** The GhostWire app (CLI, TUI, Web UI) is merely the reference implementation. The protocol itself -- encrypted messaging, anonymous routing, delay-tolerant networking, AI-powered path selection, pluggable transports -- can be embedded into any application that needs resilient, censorship-resistant peer-to-peer communication.

---

## What Is GhostWire?

GhostWire is a decentralized networking protocol written in Rust. The repository is active development: a documented feature, crate, module, or flag is not necessarily a production-supported delivery path. See the authoritative [transport and bridge support status](../TRANSPORT_SUPPORT_STATUS.md) before building on any carrier or adapter.

| Layer | Current implementation boundary |
|-------|-----------|
| **Mesh Networking** | libp2p GossipSub, Kademlia DHT, mDNS discovery |
| **Carrier policy foundation** | Tested capability, eligibility, identity-separation, fragmentation, and protected-envelope handoff planning; concrete automatic switching remains in progress |
| **Cryptography** | XChaCha20-Poly1305, X25519, Ed25519, ML-KEM-768 (post-quantum), forward secrecy ratchet |
| **Anonymous Routing** | Sphinx onion routing with Single-Use Reply Blocks (SURBs) |
| **Delay-Tolerant Networking** | Spray-and-Wait + PRoPHET epidemic routing, store-carry-forward |
| **AI Routing** | 4-layer ML stack: LightGBM -> Gemma 4 -> GNN -> PRoPHET fallback |
| **Identity** | Self-sovereign DIDs, device pairing, Shamir secret recovery, coercion resistance |
| **Knowledge Ark** | Local content-addressed foundations; no supported public swarm/provider discovery claim |
| **Ghost Channels** | Proof-of-work protected broadcast channels |

---

## What You Can Build

| Application | Description | GhostWire Features Used |
|------------|-------------|------------------------|
| **Encrypted Messenger** | End-to-end encrypted chat over mesh networks | Sphinx routing, identity, messaging API |
| **Anonymous Tip Line** | Whistleblower platform with SURB-based anonymous replies | Sphinx SURBs, coercion resistance |
| **Disaster Communications** | Architecture and future carrier goals for emergency responders | Current mesh/DTN foundations; BLE and LoRa adapters are not yet supported |
| **Offline Knowledge Distribution** | Share medical guides, legal info, maps without internet | Knowledge Ark, DTN bundles |
| **Private Bulk Transfer (foundation)** | Bounded local content-addressed staging and a private-transfer design | No supported Bitswap/IPFS compatibility, public swarm, or network file-transfer API |
| **Activist Safety Platform** | Panic mode, duress PIN, dead man's switch for at-risk users | Coercion resistance, Shamir recovery |
| **Mesh Social Network** | Decentralized social feed over local mesh | Ghost Channels, social graph, WoT |
| **IoT Sensor Network** | Collect and route sensor data across mesh | DTN, lightweight transports |
| **Censorship-Resistant Gateway (research)** | Future bridge/camouflage design direction | Individual stealth transports require explicit adapter and privacy validation |
| **Protocol Bridge** | Experimental bounded RNS interoperability; other bridges require independent audit | Matrix, Briar, Meshtastic are not supported bridge claims |

---

## Three Ways to Use GhostWire

### 1. Rust Library (Recommended)

Add the `ghostwire` crate to your project and embed the full protocol stack directly:

```toml
[dependencies]
ghostwire = { git = "https://github.com/Phantomojo/GhostWire-secure-mesh-communication" }
tokio = { version = "1", features = ["full"] }
anyhow = "1"
```

```rust
use ghostwire::core::Core;
use ghostwire::config::Config;
use libp2p::Multiaddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::default();
    let bootstrap: Vec<Multiaddr> = vec![]; // Empty = use default public gateways

    // Initialize the full GhostWire protocol stack
    let core = Core::new(config, bootstrap, None, None).await?;

    // Start mesh networking, AI routing, DTN, etc.
    core.start_nervous_system("0.0.0.0".to_string(), 0).await;

    // Send a message
    let msg = ghostwire::core::message::Message::new(
        core.get_identity_id().await,
        "recipient-peer-id".to_string(),
        "Hello from my app!".to_string(),
    );
    core.send_direct_message("recipient-peer-id".to_string(), &msg).await?;

    // Subscribe to the event bus for incoming messages
    let mut rx = core.event_bus.subscribe();
    while let Ok(event) = rx.recv().await {
        match event {
            ghostwire::core::CoreEvent::NewMessage(msg) => {
                println!("From {}: {}", msg.sender, msg.content);
            }
            ghostwire::core::CoreEvent::PeerConnected(peer_id) => {
                println!("Peer connected: {}", peer_id);
            }
            _ => {}
        }
    }

    Ok(())
}
```

### 2. REST API

Run GhostWire as a daemon and interact via HTTP:

```bash
# Start the daemon
ghostwire start

# Get an auth token (debug builds only)
TOKEN=$(curl -s http://localhost:8080/api/dev/token | jq -r '.data')

# Check health
curl http://localhost:8080/api/v2/health

# Get your identity
curl -H "X-Ghost-Token: $TOKEN" http://localhost:8080/api/v2/profile

# Send a message
curl -X POST http://localhost:8080/api/v2/messages/send \
  -H "X-Ghost-Token: $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"recipient": "4tK9mN2p...", "content": "Hello!"}'

# List peers
curl -H "X-Ghost-Token: $TOKEN" http://localhost:8080/api/v2/peers

# Subscribe to real-time events via WebSocket
wscat -c ws://localhost:8080/ws
```

Any language with an HTTP client can use the REST API -- Python, JavaScript, Go, C, etc.

### 3. Individual Crates

GhostWire publishes standalone crates to crates.io that can be used independently:

| Crate | Purpose | Use When |
|-------|---------|----------|
| [`sphinx-rs`](https://crates.io/crates/sphinx-rs) | Sphinx onion routing with SURBs | You need anonymous routing without the full stack |
| [`ghostwire-dtn`](https://crates.io/crates/ghostwire-dtn) | Delay-tolerant networking (PRoPHET) | You need store-carry-forward for intermittent connectivity |
| `hlc-rs` | Hybrid Logical Clocks | You need causal ordering across distributed nodes |
| `trust-store` | Web of Trust + TOFU key verification | You need decentralized trust management |

```toml
[dependencies]
sphinx-rs = "0.1"
ghostwire-dtn = "0.1"
```

```rust
use sphinx_rs::{create_sphinx_packet, process_sphinx_packet, SphinxConfig};
use rand::thread_rng;

let config = SphinxConfig::default();
let mut rng = thread_rng();

// Create a 3-hop anonymous route
let route_keys: Vec<[u8; 32]> = vec![hop1_pub, hop2_pub, hop3_pub];
let route_ids = vec!["hop1".into(), "hop2".into(), "hop3".into()];

let packet = create_sphinx_packet(&route_keys, &route_ids, b"secret message", &mut rng, &config)?;

// Process at hop 1
match process_sphinx_packet(&packet, &hop1_secret, &config) {
    sphinx_rs::SphinxProcessResult::Forward { next_hop, packet } => {
        println!("Forward to {}", next_hop);
    }
    sphinx_rs::SphinxProcessResult::Deliver { payload } => {
        println!("Delivered: {:?}", payload);
    }
    sphinx_rs::SphinxProcessResult::Invalid => {
        eprintln!("Invalid packet");
    }
}
```

---

## Protocol vs Implementation

It is important to understand the distinction:

- **The GhostWire Protocol** is specified in the `/docs/spec/` directory. It defines wire formats, cryptographic constructions, routing algorithms, and identity formats. Anyone can implement it in any language.
- **The GhostWire Reference Implementation** is the Rust code in this repository. It is one implementation of the protocol, providing the `ghostwire` binary and library crate.
- **The Standalone Crates** (`sphinx-rs`, `ghostwire-dtn`, etc.) are protocol components published to crates.io. They can be consumed by any Rust project, including projects that are not GhostWire nodes.

You do not need to run the GhostWire application to use GhostWire protocol components. You can embed `sphinx-rs` in a Python service (via PyO3), use `ghostwire-dtn` in an IoT firmware build, or connect to a running GhostWire daemon via REST API from a JavaScript frontend.

---

## Next Steps

1. **[Quick Start](./QUICKSTART.md)** -- Get a minimal GhostWire node running in 5 minutes
2. **[Embedding Guide](./EMBEDDING.md)** -- Integrate GhostWire into your application
3. **[Transport Support Status](../TRANSPORT_SUPPORT_STATUS.md)** -- Verified, experimental, planned, and unsupported carrier/bridge boundaries
4. **[Custom Transports](./CUSTOM_TRANSPORT.md)** -- Carrier-contract design guidance; not proof of existing adapter support
5. **[Protocol Bridges](./PROTOCOL_BRIDGES.md)** -- Bridge status and required security boundary
6. **[API Reference](./API_REFERENCE.md)** -- Complete REST API documentation

---

*GhostWire v0.2.0 | AGPL-3.0-or-later (application) / MIT+Apache-2.0 (library crates)*
