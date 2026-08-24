# Dev.to Blog Article

## Title
Building a Decentralized Mesh Network in Rust — Lessons from the Global South

## Cover Image
Screenshot of the website homepage or the how-it-works.html diagram

## Tags
#rust #opensource #privacy #meshnetwork #gcd4f #decentralized #africa

## Article Body

---

## The Problem

2.6 billion people lack reliable internet access. When disasters strike,
infrastructure fails, or communities are remote — traditional communication
breaks down precisely when coordination is most critical.

I'm a cybersecurity student in Nairobi, Kenya. I've seen what happens when
communities lose connectivity: families can't check on each other after floods,
rescue teams can't coordinate, and activists can't organize safely.

So I built **GhostWire** — a decentralized, censorship-resistant mesh
communication platform that works without any central servers.

---

## What Is GhostWire?

GhostWire is a peer-to-peer encrypted communication platform written in Rust.
Instead of connecting to a server, devices connect directly to each other.
Messages hop from node to node through whatever path is available.

If a node goes offline, the mesh routes around it. If the internet goes down,
GhostWire switches to WiFi Direct, Bluetooth, or even LoRa radio.

**Live site:** https://phantomojo.github.io/GhostWire-secure-mesh-communication/
**GitHub:** https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication

---

## The Architecture

### Networking: libp2p

We use [libp2p](https://libp2p.io/) — the same P2P networking stack used by
IPFS and Ethereum. It handles peer discovery, connection establishment, and
multiplexing. On top of that, we run a S/Kademlia-hardened DHT for routing
and Gossipsub for message propagation.

```rust
// Simplified peer discovery
let swarm = SwarmBuilder::with_new_identity()
    .with_tokio()
    .with_tcp(
        tcp::Config::default(),
        noise::Config::new,
        yamux::Config::default,
    )?
    .with_behaviour(|_| Behaviour::new())?
    .build();
```

### Encryption: Defense in Depth

Every message is encrypted end-to-end before it leaves your device:

| Layer | Algorithm | Purpose |
|-------|-----------|---------|
| Symmetric | AES-256-GCM | Message encryption + integrity |
| Key Exchange | X25519 | Perfect forward secrecy |
| Signatures | Ed25519 | Identity verification |
| Post-Quantum | ML-KEM-768 | Future-proof (planned) |

No server, no relay, no intermediate node ever sees plaintext.

### AI-Powered Routing

This is where GhostWire gets interesting. Instead of using fixed routing rules,
we trained AI models on **real mesh network data** from Barcelona's GuifiSants
— one of the world's largest community mesh networks.

- **L1 — LightGBM anomaly detector:** AUC 1.0, 76.7μs inference, exported as
  ONNX and wired into Rust via ONNX Runtime
- **L3 — Graph Neural Network:** Trained on 7,931 samples across 63 nodes over
  31 days. Learns which paths work best in real conditions.

```python
# Training pipeline (simplified)
model = lgb.LGBMRegressor(
    n_estimators=500,
    learning_rate=0.05,
    max_depth=7,
    num_leaves=31
)
model.fit(X_train, y_train)
onnx_model = convert_lightgbm(model, initial_types=initial_type)
onnx.save(onnx_model, "ghostwire_routing.onnx")
```

The model runs in **76.7 microseconds** — fast enough for real-time routing
decisions on a Raspberry Pi.

### 7 Transport Layers

| Transport | Range | Use Case |
|-----------|-------|----------|
| WiFi Direct | ~100m | Urban mesh, device-to-device |
| Bluetooth LE | ~50m | Indoor, low-power |
| LoRa | ~15km | Rural, long-range |
| WebRTC | Internet | Bridge across networks |
| TCP/IP | Internet | Standard connectivity |
| Reticulum | Multi-hop | Amateur radio mesh |
| Briar | Bluetooth/WiFi | Activist communication |

GhostWire selects the best available path automatically. No internet? It falls
back to RF mesh. No WiFi? Bluetooth. The mesh adapts.

---

## Why Rust?

Three reasons:

1. **Memory safety without GC** — GhostWire runs on resource-constrained devices
   (Raspberry Pi, old laptops). We can't afford a garbage collector pause during
   emergency communication.

2. **Fearless concurrency** — The networking stack handles hundreds of
   simultaneous peer connections. Rust's ownership model means we don't worry
   about data races.

3. **Performance** — The LightGBM inference runs in 76.7μs. The crypto is
   hardware-accelerated. Rust lets us squeeze every microsecond out of the
   hardware.

---

## The Human Side

GhostWire isn't just a technical project. It's built on a philosophy:

> In Hindu and Buddhist cosmology, Indra's Net is an infinite web. At each
> intersection hangs a jewel. Each reflected in all others. No jewel is more
> important. The net has no center. The net has no edge.
>
> The original internet architects independently rediscovered what African
> philosophy had encoded for millennia: systems built on mutual relationship
> rather than central authority are more resilient, more equitable, and more
> aligned with existence itself.

We're building this as part of the **GCD4F 2026** competition (Global Climate
Data for Future) under the "AI for Society" track, representing the Open
University of Kenya.

---

## We Need Contributors

GhostWire is AGPL-3.0 licensed and actively seeking contributors:

- **Rust developers** — libp2p networking, transport layers, crypto
- **AI/ML engineers** — GNN model training, routing optimization
- **Security researchers** — independent audit, threat modeling
- **Frontend developers** — React/TypeScript dashboard
- **Documentation writers** — guides, tutorials, translations

**Good first issues** are labeled on GitHub. Our CONTRIBUTING.md has detailed
setup instructions.

---

## Links

- **Website:** https://phantomojo.github.io/GhostWire-secure-mesh-communication/
- **GitHub:** https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication
- **Docs:** https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication/tree/main/docs
- **Security:** https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication/blob/main/docs/SECURITY.md

---

*Built in Nairobi, for the world. 🇰🇪*

## Publishing Tips
• Publish on Tuesday or Wednesday morning
• Add the cover image (screenshot of website or diagram)
• Engage with every comment in the first 24 hours
• Cross-post to Hashnode and Medium after 1 week
• Share the Dev.to link on Twitter/Mastodon with #rustlang #opensource
