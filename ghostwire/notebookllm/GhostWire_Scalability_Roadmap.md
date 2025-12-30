# 🛰️ GhostWire: Scalability & Expansion Roadmap

This document outlines the strategic path to scale GhostWire from small-scale clusters to a global secure mesh network capable of supporting thousands of concurrent nodes.

## 1. Protocol Layer Optimization
Current GhostWire messages use JSON over the mesh. This is the first major bottleneck.

- **✅ Transition to Protobuf**: Migrate from `serde_json` to `prost` or `capnp`. This reduces message size by up to 60-80% and significantly decreases CPU cycles spent on serialization.
- **✅ Canonical Message Compression**: Implement Zstd compression for message payloads exceeding 1KB.
- **✅ Delta-Syncing**: Instead of sending full topology snapshots, implement delta-updates to synchronization only what has changed.

## 2. Hierarchical Mesh (Supernodes)
A flat mesh becomes unstable beyond a certain density. We should introduce a tiered architecture.

- **✅ Supernode Election**: Dynamically elect nodes with high uptime, stable IP addresses, and significant bandwidth to act as "Relay Hubs."
- **✅ Auto-Relay (AutoNAT)**: Automatically detect symmetric NATs and route traffic through Supernodes only when direct P2P is impossible.
- **✅ Regional DHT Sharding**: Instead of one global Kademlia DHT, use regional sharding to keep discovery latency low for local clusters while maintaining a global "backbone."

## 3. Adaptive Gossip Tuning
Gossipsub is powerful but can be noisy if not tuned.

- **✅ Density-Aware Gossip**: Automatically adjust the `mesh_n` (target number of peers) based on the total node count. In dense environments, `mesh_n` should be lowered to reduce redundancy.
- **✅ Topic Filtering**: Implement server-side (node-side) topic interest filtering to ensure nodes only receive packets for channels they are actually subscribed to.

## 4. Multi-Transport Offloading
GhostWire already has Matrix and Meshtastic adapters. We can use these for more than just bridging.

- **✅ Backbone Offloading**: Allow nodes to bridge "High-Latency" traffic over Matrix or Tor while keeping "Low-Latency" traffic on the StealthTCP mesh.
- **✅ LoRa Heartbeats**: Use Meshtastic/LoRa as a "Control Plane" to maintain network connectivity even when the Primary IP network is down (OOB Management).

## 5. Incentivation Model (Future)
To ensure nodes route traffic reliably:
- **✅ Proof-of-Relay**: A cryptographic way to prove a node has successfully forwarded a packet.
- **✅ Reputation Credits**: Nodes gain "Relay Credits" by forwarding traffic, which they can then spend to prioritize their own outgoing packets during high network congestion.

---

## 📈 Scalability Targets

| Phase       | Target Nodes | Tech Milestone                              |
| :---------- | :----------- | :------------------------------------------ |
| **Current** | 10 - 50      | Stable StealthTCP + mDNS                    |
| **V2.2**    | 100 - 500    | Protobuf + Kademlia DHT Optimization        |
| **V3.0**    | 1,000+       | Hierarchical Supernodes + Regional Sharding |
| **V4.0**    | 10,000+      | P2P Incentive Layer + Global Backbone       |
