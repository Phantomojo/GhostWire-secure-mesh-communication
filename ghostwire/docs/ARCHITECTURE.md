# GhostWire Architecture Guide

> **Version**: 1.1 | **Last Updated**: 2026-07-03 | **Score**: 7/10 → Target 8/10

---

## 1. System Overview

GhostWire is a decentralized P2P encrypted mesh communication system built in Rust. No central servers. No single point of failure.

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                         │
│  ┌─────────┐  ┌─────┐  ┌──────────┐  ┌──────────────────┐  │
│  │   TUI   │  │ CLI │  │  Web UI  │  │  Android (planned)│  │
│  └────┬────┘  └──┬──┘  └────┬─────┘  └────────┬─────────┘  │
├───────┼──────────┼──────────┼──────────────────┼────────────┤
│       │    API Layer (Axum 0.8 + Tower)        │            │
│       │  ┌───────┴──────────┴──────────┐       │            │
│       │  │ REST + WebSocket + JWT Auth │       │            │
│       │  │ Biscuit tokens + Rate Limit │       │            │
│       │  └────────────┬────────────────┘       │            │
├───────────────────────┼────────────────────────────────────┤
│              Security Layer                                 │
│  ┌────────────────────┴────────────────────────┐            │
│  │  SecurityManager · AuditLogger · RateLimit  │            │
│  │  Threat Detection · Peer Scoring            │            │
│  └────────────────────┬────────────────────────┘            │
├───────────────────────┼────────────────────────────────────┤
│             Encryption Layer                                │
│  ┌────────────────────┴────────────────────────┐            │
│  │  Ed25519 (sign) · X25519 (key agreement)    │            │
│  │  AES-256-GCM (encrypt) · ML-KEM (post-Q)   │            │
│  │  HKDF-SHA256 (KDF)                          │            │
│  └────────────────────┬────────────────────────┘            │
├───────────────────────┼────────────────────────────────────┤
│              Identity Layer                                 │
│  ┌────────────────────┴────────────────────────┐            │
│  │  Ed25519 keypairs · Biscuit tokens          │            │
│  │  Trust scores · Social graph                │            │
│  └────────────────────┬────────────────────────┘            │
├───────────────────────┼────────────────────────────────────┤
│              Privacy Layer                                  │
│  ┌────────────────────┴────────────────────────┐            │
│  │  Sphinx packets (2252B fixed)               │            │
│  │  Cover traffic (512B blocks)                │            │
│  │  Poisson timing (planned)                   │            │
│  │  PARSAN-Mix α · LARMix τ                    │            │
│  └────────────────────┬────────────────────────┘            │
├───────────────────────┼────────────────────────────────────┤
│            Transport Layer (pluggable)                      │
│  ┌──────┐ ┌──────┐ ┌────┐ ┌──────┐ ┌────────┐            │
│  │libp2p│ │RNS │ │Obfs4✗│ │Circuit │              │
│  │TCP   │ │QUIC  │ │BLE │ │      │ │Relay v2│            │
│  └──┬───┘ └──┬───┘ └─┬──┘ └──┬───┘ └───┬────┘            │
├─────┼────────┼────────┼───────┼─────────┼──────────────────┤
│             Network Layer                                   │
│  ┌────────────────────┴────────────────────────┐            │
│  │  GossipSub (pub/sub) · Kademlia DHT         │            │
│  │  mDNS (local) · Peer scoring                │            │
│  └────────────────────┬────────────────────────┘            │
├───────────────────────┼────────────────────────────────────┤
│              Storage Layer                                  │
│  ┌────────────────────┴────────────────────────┐            │
│  │  MessageStore · BlockStore · Mailbox         │            │
│  │  Encrypted at rest (AES-256-GCM)            │            │
│  └────────────────────┬────────────────────────┘            │
├───────────────────────┼────────────────────────────────────┤
│              Message Layer                                  │
│  ┌────────────────────┴────────────────────────┐            │
│  │  ACK/Retry · Ordering (vector clocks)        │            │
│  │  Dedup (bloom filter) · TTL · Chunking       │            │
│  └────────────────────┬────────────────────────┘            │
├───────────────────────┼────────────────────────────────────┤
│              Social Layer                                   │
│  ┌────────────────────┴────────────────────────┐            │
│  │  Groups · Channels · Contact verification   │            │
│  │  Reputation · Trust propagation             │            │
│  └─────────────────────────────────────────────┘            │
└─────────────────────────────────────────────────────────────┘
```

---

## 2. Layer Audit Scores

| # | Layer | Score | Status | Notes |
|---|-------|-------|--------|-------|
| 1 | Transport | 5/10 | ⚠️ Partial | libp2p (TCP+QUIC) and Reticulum work. Obfs4 is NOT implemented (no handshake/framing, not in the transport chain). Iroh and Bluetooth are not in the tree. |
| 2 | Privacy | 4/10 | ⚠️ Incomplete | Message padding works, and cover traffic IS emitted (verified: 512B frames, scheduler started by `TrafficArmor::new`). The real gap is `presence_obfuscation` — zero callers, so intersection attacks are unmitigated. Note cover traffic rides gossipsub on the DM topic; that is padding, not wire-layer obfuscation, and does not survive an active prober. |
| 3 | Encryption | 8/10 | ✅ Working | AES-256-GCM, X25519, ML-KEM, small-order defense |
| 4 | Identity | 6/10 | ⚠️ Partial | Keys work, revocation missing |
| 5 | Security | 8/10 | ✅ Working | SecurityManager, Audit, RateLimit, CVE patches |
| 6 | Storage | 7/10 | ✅ Working | MessageStore, Mailbox, BlockStore |
| 7 | Network | 6/10 | ⚠️ Stub | DHT exists, peer scoring weak |
| 8 | Message | 7/10 | ✅ Working | ACKs/NACKs, HLC ordering, ping/pong health |
| 9 | Social | 5/10 | ⚠️ Basic | Groups exist, reputation weak |
| 10 | AI/ML | 5/10 | ❌ Broken | Routing accuracy 2.69% (target 85-95%) |
| 11 | API | 7/10 | ✅ Working | Axum + WebSocket + JWT |
| 12 | Application | 7/10 | ✅ Working | TUI, CLI, Web UI |
| 13 | Health Monitoring | 7/10 | ✅ Working | Ping/pong, RTT, liveness detection |

**Overall: 7/10 (B-)** — 4 critical gaps before v1.0.

---

## 3. Trust Boundaries

| Boundary | Trusted Side | Untrusted Side | Enforcement |
|----------|-------------|----------------|-------------|
| **B1: Network** | Local node | P2P network | Noise (libp2p). Iroh NOT in tree. |
| **B2: API** | Authenticated client | HTTP requests | Biscuit token + rate limit |
| **B3: Storage** | Encrypted store | Disk | AES-256-GCM at rest |
| **B4: Identity** | Private keys | Public keys | Ed25519 signatures |
| **B5: Peer** | Verified peers | Unknown peers | Trust scoring + reputation |

---

## 4. Data Flow: Sending a Message

```
1. User types message in TUI/Web UI
2. API handler validates input + checks Biscuit token
3. SecurityManager verifies sender not rate-limited
4. EncryptionManager:
   a. Looks up recipient public key
   b. Derives session key (X25519 ECDH)
   c. Encrypts payload (AES-256-GCM)
   d. Signs with Ed25519
5. PrivacyLayer:
   a. Wraps in Sphinx packet (fixed 2252B)
   b. Adds padding (512B blocks) — WORKS. Cover traffic (512B random frames on the DM topic) also runs.
   c. Applies Poisson timing (planned)
6. TransportManager:
   a. Selects best transport (libp2p TCP+QUIC, Reticulum). Obfs4 NOT implemented.
   b. Routes via GossipSub or direct connection
7. Recipient receives, decrypts, verifies, ACKs
```

---

## 5. Core Components

### 5.1 Core Struct (30+ fields)

```rust
pub struct Core {
    // Configuration
    pub config: Arc<RwLock<Config>>,
    pub device_class: DeviceClass,
    pub privacy_params: PrivacyParams,

    // Identity & Encryption
    pub identity: Arc<RwLock<Identity>>,
    pub encryption: Arc<RwLock<Encryption>>,

    // Networking
    pub mesh_manager: Option<Arc<MeshManager>>,
    pub reticulum_manager: Option<Arc<ReticulumManager>>,
    pub transport_registry: Arc<RwLock<TransportRegistry>>,
    pub adapter_registry: Arc<RwLock<AdapterRegistry>>,

    // Security
    pub security_manager: Arc<RwLock<SecurityManager>>,
    pub reputation: Arc<RwLock<ReputationManager>>,

    // Storage
    pub persistence: Arc<RwLock<PersistenceManager>>,
    pub block_store: Arc<RwLock<BlockStore>>,
    pub mailbox: Arc<RwLock<Mailbox>>,

    // Privacy
    pub traffic_armor: Arc<RwLock<TrafficArmor>>,
    pub monitor: Arc<RwLock<Monitor>>,

    // Messaging
    pub channel_manager: Arc<RwLock<ChannelManager>>,
    pub social_graph: Arc<RwLock<SocialGraph>>,
    pub trust_store: Arc<RwLock<TrustStore>>,

    // AI/ML
    pub ai_manager: Option<Arc<AIManager>>,

    // DTN
    pub dtn: Arc<RwLock<DTNManager>>,
}
```

### 5.2 Module Map

| Module | Purpose | Feature-gated |
|--------|---------|---------------|
| `encryption` | AES-256-GCM, X25519, key derivation, small-order defense | No |
| `identity` | Ed25519 keypairs, identity management | No |
| `security_v2` | Threat detection, audit, rate limiting | No |
| `transport` | Pluggable transport abstraction | No |
| ~~`stealth_tcp`~~ | REMOVED — see `REMOVED_STEALTHTCP.md`. |sport | No |
| `obfs4_transport` | Tor pluggable transport | No |
| `mesh` | GossipSub + Kademlia + health monitoring | `mesh` |
| `peer_health` | Ping/pong, RTT tracking, liveness detection | No |
| `mesh_health` | Health metrics aggregation | No |
| `briar` | Briar protocol integration | `briar` |
| `bluetooth` | BLE controller | `bluetooth` |
| `reticulum` | Reticulum network | `reticulum` |
| `dtn` | Delay-tolerant networking (PRoPHET) | No |
| `traffic_armor` | Cover traffic generation | No |
| `reputation` | Peer trust scoring | No |
| `social` | Social graph, groups, channels | No |
| `ai` | ML routing, anomaly detection, Gemma 4 | No |
| `wire_protocol` | Binary frame protocol (Ping, Pong, ACK, etc.) | No |

---

## 6. Cryptographic Primitives

| Purpose | Algorithm | Library | Security Level |
|---------|-----------|---------|---------------|
| Signing | Ed25519 | `ed25519-dalek` | 128-bit |
| Key Agreement | X25519 | `x25519-dalek` | 128-bit |
| Symmetric | AES-256-GCM | `aes-gcm` | 256-bit |
| Hashing | SHA-256 | `sha2` | 128-bit collision |
| KDF | HKDF-SHA256 | `hkdf` | 128-bit |
| Post-Quantum | ML-KEM-768 | `ml-kem` | 192-bit quantum |
| Sphinx | Fixed 2252B packets | `sphinx-rs` | anonymity sets |

---

## 7. Transport Stack

```
Application Message
    ↓
Sphinx Packet (2252B fixed)
    ↓
Cover Traffic Padding (512B blocks)
    ↓
Transport Selection:
  ├── libp2p TCP + Noise + yamux (primary)
  ├── Reticulum RNS (AutoInterface / TCP targets) — WORKING
  ├── Circuit Relay v2 (fallback)
  ├── Bluetooth BLE (local mesh) — NOT IMPLEMENTED
  ├── Obfs4 (censorship circumvention) — NOT IMPLEMENTED
  └── TURN (last resort)
```

---

## 8. v1.0 Gaps to Close

| Gap | Impact | Phase | Status |
|-----|--------|-------|--------|
| Message ACK/Retry | Messages silently lost | Phase 1 | ✅ Done |
| Message Ordering | Out-of-order delivery | Phase 1 | ✅ Done (HLC) |
| Message Deduplication | Duplicate messages | Phase 1 | ✅ Done (bloom filter) |
| Ping/Pong Health | No liveness detection | Phase 1 | ✅ Done |
| Key Revocation | Compromised keys can't be revoked | Phase 2 | ❌ Missing |
| Forward Secrecy | Past messages at risk | Phase 2 | ✅ Done (symmetric ratchet) |
| Peer Scoring | Sybil attacks trivial | Phase 3 | ⚠️ Weak |
| PrivacyParams | Mobile battery waste | Phase 3 | ⚠️ Static |
| Two-Machine Test | No verified end-to-end test | Phase 3 | ❌ Missing |
| Fuzzing | Unknown vulnerabilities | Phase 4 | ❌ Missing |

---

## 9. References

- [THREAT_MODEL.md](./THREAT_MODEL.md) — STRIDE threat analysis
- [KEY_MANAGEMENT.md](./KEY_MANAGEMENT.md) — Key lifecycle
- [MESSAGE_DELIVERY_SPEC.md](./MESSAGE_DELIVERY_SPEC.md) — Message reliability
- [PRIVACY_TIERS.md](./PRIVACY_TIERS.md) — Privacy parameters
- [DEVICE_TIERS.md](./DEVICE_TIERS.md) — Device capabilities
- [DESIGN_DECISIONS.md](./DESIGN_DECISIONS.md) — Why we chose X over Y
- [SECURITY.md](../SECURITY.md) — Security policy and open findings
