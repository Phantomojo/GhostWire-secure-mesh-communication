# libp2p crate documentation

## Overview

libp2p is a modular peer-to-peer networking framework written in Rust.

## License

MIT

## Source

https://github.com/libp2p/rust-libp2p

## Website

https://libp2p.io

---

## Core Concepts

### Transport Trait

The Transport trait provides connection-oriented communication between peers.

```rust
use libp2p::Transport;

// TCP transport
let tcp = libp2p::tcp::TcpTransport::new();

// WebSocket transport
let ws = libp2p::websocket::WsConfig::new();
```

### Swarm

The Swarm manages the network state and behaviors.

```rust
use libp2p::Swarm;

let mut swarm = Swarm::new(
    my_identity,
    my_behaviour,
);
```

### PeerId

Unique identifier for a peer in the network.

```rust
use libp2p::PeerId;

let peer_id = PeerId::from.public_key(&public_key);
```

### Multiaddr

Address format supporting multiple protocols.

```rust
use libp2p::Multiaddr;

let addr = "/ip4/127.0.0.1/tcp/4001".parse()?;
```

---

## Protocols Used in GhostWire

### Gossipsub (pub/sub messaging)

```rust
use libp2p::gossipsub::{Gossipsub, MessageAuthenticity};

let gossipsub = Gossipsub::new(
    MessageAuthenticity::Signed(keypair),
    config,
);
```

### Kademlia DHT (peer discovery)

```rust
use libp2p::kademlia::Kademlia;

let kad = Kademlia::new(
    peer_id,
    mem_store,
);
```

### mDNS (local discovery)

```rust
use libp2p::mdns::Mdns;

let mdns = Mdns::new()?;
```

### Noise (encrypted transport)

```rust
use libp2p::noise::NoiseConfig;

let noise = NoiseConfig::xx(keypair);
```

### Yamux (stream multiplexing)

```rust
use libp2p::yamux::YamuxConfig;
```

### Identify (peer protocol)

```rust
use libp2p::identify::Identify;
```

### Ping

```rust
use libp2p::ping::Ping;
```

---

## Feature Flags

| Feature          | Description                            |
| ---------------- | -------------------------------------- |
| tcp              | TCP transport                          |
| dns              | DNS resolution                         |
| websocket        | WebSocket support                      |
| noise            | Noise protocol for encrypted transport |
| yamux            | Stream multiplexing                    |
| kad              | Kademlia DHT                           |
| gossipsub        | Gossipsub pub/sub                      |
| mdns             | mDNS local discovery                   |
| identify         | Identify protocol                      |
| relay            | Circuit relay                          |
| ping             | Ping protocol                          |
| request-response | Request/response protocols             |

---

## Building a GhostWire Node

```rust
use libp2p::{
    swarm::SwarmBuilder,
    tcp::TcpTransport,
    noise::NoiseConfig,
    yamux::YamuxConfig,
    gossipsub::{Gossipsub, GossipsubConfig},
    identity::Keypair,
};

let keypair = Keypair::generate_ed25519();

let transport = TcpTransport::new(/)
    .upgrade(libp2p::core::upgrade::Version::V1)
    .authenticate(NoiseConfig::xx(keypair.clone()))
    .multiplex(YamuxConfig::default())
    .boxed();

let behaviour = GhostWireBehaviour::new();

let swarm = SwarmBuilder::new()
    .transport(transport)
    .authenticate(NoiseConfig::xx(keypair))
    .behaviour(behaviour)
    .swap_lazy_listeners()
    .build();
```

---

## Dependencies

- bytes
- futures
- multiaddr
- pin-project
- tokio
- tracing-subscriber
