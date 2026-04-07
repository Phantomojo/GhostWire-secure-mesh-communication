# 🗺️ GhostWire: Future Enhancement Scopes

Following the successful implementation of the **Nervous System** and **Military-Grade Resilience**, we have several high-impact directions to take GhostWire to the next level.

---

## 🏗️ Scope A: GhostWire-Nucleus (Hardware Portability)
**Objective**: Shrink the GhostWire "DNA" to run on $1.00 microcontrollers (ESP32, STM32).

- **Current State**: Requires ~50MB RAM and 20MB Binary (Linux/Android target).
- **Nucleus Proposal**: 
    - Strip `libp2p` and `tokio` in favor of `no_std` alternatives (e.g., `embassy`, `smol`).
    - Implement a minimal **Gossip Light** protocol for low-power radio links.
    - Create a bridge between Nucleus nodes (sensors/IoT) and Core nodes (hubs/servers).
- **Value**: Makes GhostWire the global standard for $10 off-grid mesh devices.

## 🆔 Scope B: Sovereign Persona (Identity V2)
**Objective**: Move beyond simple keys to rich, decentralized, and anonymous profiles.

- **DIDs (Decentralized Identifiers)**: Integrate W3C DID standards for interoperable identity.
- **Private Profiles**: Implement Zero-Knowledge Proofs (ZKP) for attribute verification (e.g., "I am a verified rescue worker" without revealing my name).
- **Key Recovery**: "Social Recovery" mechanisms where a quorum of trusted peers can help restore a lost identity.
- **Value**: Establishes GhostWire as a platform for sovereign human interaction, not just a relay.

## 🕸️ Scope C: Recursive DHT (Web-Scale Mesh)
**Objective**: Scale from 100 nodes to 100,000+ nodes.

- **S/Kademlia Hardening**: Protect the DHT against Sybil and Eclipse attacks.
- **Hierarchical Sharding**: Divide the global mesh into regional shards to reduce routing table bloat on mobile devices.
- **Supernode Orchestration**: Dynamically promote high-uptime, high-bandwidth nodes to act as regional gateways.
- **Value**: Allows GhostWire to function as a "parallel internet" that scales with the population.

## 🛡️ Scope D: Post-Quantum "Armor" (Hyper-Hardening)
**Objective**: Protect GhostWire traffic against future quantum computer decryption.

- **Hybrid ML-KEM**: Integrate Kyber (NIST standard) alongside X25519.
- **PQ-Signatures**: Transition identity signing from Ed25519 to Dilithium (ML-DSA).
- **Algorithm Agility**: Implement a "Negotiated Armor" system where nodes can step down to legacy crypto only if the peer doesn't support PQC.
- **Value**: Ensures GhostWire communications remains secret for the next 50+ years.

## 🕹️ Scope E: The "War Room" UI (Advanced Visualization)
**Objective**: Enhance the MeshVisualizer into a professional tactical dashboard.

- **Real-time Heatmaps**: Show network congestion and "Disaster Zones" on a global or local map.
- **Transport Pulse**: Visual indicators of which messages are moving over LoRa vs WiFi in real-time.
- **Autonomic Logs**: A dedicated view showing the "Nervous System" decisions (e.g., "Switching to LoRa due to 40% packet loss on Mesh").
- **Value**: Makes the complex resilience logic visible and understandable for end users.

---

### 🗳️ Your Decision
Which scope aligns most with your current vision for GhostWire?
1. **The Core Evolution** (Scalability & Nucleus)
2. **The Sovereign Human** (Identity & Privacy)
3. **The Battle-Ready Armor** (PQC & Testing)
4. **The Tactical Vision** (Advanced UI)
