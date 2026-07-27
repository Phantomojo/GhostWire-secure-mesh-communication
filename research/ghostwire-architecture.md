# GhostWire Architecture — Schematic Map

**Last updated**: 2026-06-24
**Analogy**: Like a biological nervous system — sensory organs (adapters), nerves (transport), brain (Core), immune system (security), and autonomous reflexes (AI routing).

---

## 1. The Big Picture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         GHOSTWIRE NODE (One Device)                         │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                     🧠 CORE (Brain)                                 │    │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────────────┐   │    │
│  │  │ Identity │  │Encryption│  │ Security │  │  Event Bus       │   │    │
│  │  │ "Who am  │  │ "How do  │  │ "Is this │  │  (broadcast      │   │    │
│  │  │  I?"     │  │  I lock  │  │  safe?"  │  │   channel)       │   │    │
│  │  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────────┬─────────┘   │    │
│  │       │              │              │                  │             │    │
│  │  ┌────┴──────────────┴──────────────┴──────────────────┴─────────┐  │    │
│  │  │              Message Flow Controller                          │  │    │
│  │  │   send_message() → send_direct_message() → mesh.send()       │  │    │
│  │  └──────────────────────────┬────────────────────────────────────┘  │    │
│  │                             │                                       │    │
│  │  ┌──────────────────────────┴────────────────────────────────────┐  │    │
│  │  │                    Nervous System                              │  │    │
│  │  │  ┌─────────────┐  ┌──────────────┐  ┌─────────────────────┐  │  │    │
│  │  │  │   Monitor   │  │   Switcher   │  │   Disaster Manager  │  │  │    │
│  │  │  │ "How are    │  │ "Switch to   │  │ "Something broke"   │  │  │    │
│  │  │  │  the nerves │  │  backup"     │  │  (latency, loss,    │  │  │    │
│  │  │  │  doing?"    │  │              │  │   isolation)        │  │  │    │
│  │  │  └──────┬──────┘  └──────┬───────┘  └──────────┬──────────┘  │  │    │
│  │  └─────────┼────────────────┼──────────────────────┼─────────────┘  │    │
│  └────────────┼────────────────┼──────────────────────┼────────────────┘    │
│               │                │                      │                     │
│  ┌────────────┴────────────────┴──────────────────────┴────────────────┐    │
│  │                     🔌 TRANSPORT LAYER                               │    │
│  │                                                                      │    │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐  │    │
│  │  │   QUIC   │ │ Simulated│ │ Stealth  │ │   BLE    │ │  (More)  │  │    │
│  │  │  (Real   │ │  (Test   │ │   TCP    │ │ (Classic │ │          │  │    │
│  │  │   P2P)   │ │  Mode)   │ │ (Obfuscat│ │  BT via  │ │          │  │    │
│  │  │          │ │          │ │   ed)    │ │  BlueZ)  │ │          │  │    │
│  │  └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘  │    │
│  └───────┼────────────┼────────────┼─────────────┼────────────┼────────┘    │
│          │            │            │             │            │              │
│  ┌───────┴────────────┴────────────┴─────────────┴────────────┴────────┐    │
│  │                     📡 MESH NETWORK (libp2p swarm)                  │    │
│  │                                                                      │    │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                  │    │
│  │  │   Kademlia  │  │    Gossip   │  │   Sphinx    │                  │    │
│  │  │     DHT     │  │     Sub     │  │   Routing   │                  │    │
│  │  │ "Find peer" │  │ "Broadcast" │  │ "Onion      │                  │    │
│  │  │             │  │             │  │  packets"   │                  │    │
│  │  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘                  │    │
│  └─────────┼────────────────┼────────────────┼─────────────────────────┘    │
│            │                │                │                              │
│  ┌─────────┴────────────────┴────────────────┴─────────────────────────┐    │
│  │                     💾 STORAGE LAYER                                 │    │
│  │                                                                      │    │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                  │    │
│  │  │  Message    │  │  Mailbox    │  │  Block      │                  │    │
│  │  │  Store      │  │  Store      │  │  Store      │                  │    │
│  │  │ (Encrypted  │  │ (Offline    │  │ (Content-   │                  │    │
│  │  │  messages)  │  │  delivery)  │  │  addressed) │                  │    │
│  │  └──────┬──────────┘  └──────┬──────┘  └──────┬──────┘              │    │
│  └─────────┼────────────────┼────────────────┼─────────────────────────┘    │
│            │                │                │                              │
│  ┌─────────┴────────────────┴────────────────┴─────────────────────────┐    │
│  │                     🧬 AI/ML LAYER (Autonomic)                       │    │
│  │                                                                      │    │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                  │    │
│  │  │   Gemma 4   │  │   ONNX      │  │   GNN       │                  │    │
│  │  │   (LLM)     │  │  (Fast      │  │  (Graph     │                  │    │
│  │  │             │  │   routing)  │  │   routing)  │                  │    │
│  │  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘                  │    │
│  └─────────┼────────────────┼────────────────┼─────────────────────────┘    │
│            │                │                │                              │
│  ┌─────────┴────────────────┴────────────────┴─────────────────────────┐    │
│  │                     🌐 ADAPTER LAYER (Protocol Bridges)              │    │
│  │                                                                      │    │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                  │    │
│  │  │   Matrix    │  │  Meshtastic │  │    IRC      │                  │    │
│  │  │  (Homeserver│  │  (LoRa      │  │  (Legacy    │                  │    │
│  │  │   bridge)   │  │   radio)    │  │   chat)     │                  │    │
│  │  └─────────────┘  └─────────────┘  └─────────────┘                  │    │
│  └──────────────────────────────────────────────────────────────────────┘    │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Message Flow — How a Message Travels

### Outgoing Message (You type "Hello")

```
You type "Hello"
       │
       ▼
┌──────────────────┐
│  Core::send_     │
│  direct_message()│
│                  │
│  1. Check        │  ← SecurityManager: "Is system locked down?"
│     lockdown     │
│  2. Record ACK   │  ← ReliabilityLayer: "Track this message for retry"
│  3. Persist      │  ← SecureMessageStore: "Save encrypted copy"
│  4. Add to       │  ← In-memory Vec<Message>
│     history      │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│  MeshManager::   │
│  send_direct_    │
│  message()       │
│                  │
│  1. Parse        │  ← "Is recipient a valid PeerId?"
│     PeerId       │
│  2. Send via     │  ← MeshCommand::SendMessage
│     command_tx   │     (channel to MeshDriver)
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│  MeshDriver      │
│  (Background     │
│   Actor)         │
│                  │
│  1. Encrypt      │  ← EncryptionManager::encrypt_message()
│     message      │     (XChaCha20-Poly1305)
│  2. Build Sphinx │  ← create_sphinx_packet(route, payload)
│     packet       │     (Onion encryption through 2+ hops)
│  3. Mix delay    │  ← MixingQueue: "Wait random 50-200ms"
│     (timing      │     (Prevents timing analysis)
│     protection)  │
│  4. Forward to   │  ← swarm.send_message(next_hop, packet)
│     next hop     │     (libp2p transports to the wire)
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│  QUIC / BLE /    │
│  Stealth TCP     │
│                  │
│  Bytes on the    │  ← Actual network I/O
│  wire            │
└──────────────────┘
```

### Incoming Message (Someone sends you "Hello")

```
Bytes arrive on QUIC/BLE/TCP
       │
       ▼
┌──────────────────┐
│  libp2p swarm    │
│  receives data   │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│  MeshDriver      │
│  processes       │
│  behaviour       │
│  events          │
│                  │
│  Is it a Sphinx  │
│  packet?         │
│  YES →           │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│  process_sphinx  │
│  _packet()       │
│                  │
│  1. Peel one     │  ← Decrypt one layer with our X25519 key
│     layer        │
│  2. Check MAC    │  ← "Is this packet really for me?"
│  3. If relay:    │
│     forward to   │  ← Next hop in Sphinx route
│     next hop     │
│  4. If final:    │
│     extract      │  ← Get plaintext payload
│     payload      │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│  Message         │
│  Deserialization │
│                  │
│  1. Decrypt      │  ← EncryptionManager::decrypt_message()
│     payload      │     (XChaCha20-Poly1305)
│  2. Verify       │  ← Check signature if present
│     signature    │
│  3. Deduplicate  │  ← MessageDeduplicator: "Seen this UUID?"
│  4. Store        │  ← SecureMessageStore::save_message()
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│  Event Bus       │
│                  │
│  broadcast(      │
│    CoreEvent::   │
│    NewMessage)   │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│  UI / TUI /      │
│  API Handler     │
│                  │
│  Display to user │
└──────────────────┘
```

---

## 3. Component Deep Dive — What Each Part Does

### 🧠 Core (The Brain)

| Field | Type | Purpose |
|-------|------|---------|
| `identity` | `Arc<RwLock<Identity>>` | Who am I? (Ed25519 keypair, DID, metadata) |
| `encryption` | `Arc<EncryptionManager>` | All crypto operations (XChaCha20, Ed25519, X25519) |
| `security_manager` | `Arc<SecurityManager>` | Rate limiting, lockdown, audit logging |
| `event_bus` | `broadcast::Sender<CoreEvent>` | System-wide event distribution (pub/sub) |
| `config` | `Config` | Feature flags, network settings, profile |

### 🔌 Transport (The Nerves)

| Transport | When Used | How It Works |
|-----------|-----------|--------------|
| **QUIC** | Default on internet | UDP-based, multiplexed streams, TLS 1.3 built-in |
| **Simulated** | Testing | In-process channel, no real network |
| **Stealth TCP** | Censorship evasion | Disguises traffic as normal HTTPS |
| **BLE** | Local mesh | Classic Bluetooth via BlueZ D-Bus |

**AutoTransportSwitcher** monitors network health and switches transports automatically:
- Normal → QUIC (best performance)
- Degraded → Stealth TCP (censorship resistance)
- Isolated → BLE (local mesh only)

### 📡 Mesh Network (The Spinal Cord)

| Component | Purpose |
|-----------|---------|
| **libp2p swarm** | P2P connection management, protocol negotiation |
| **Kademlia DHT** | Distributed peer lookup ("Find node with ID X") |
| **Gossipsub** | Topic-based pub/sub broadcast ("Everyone hear this") |
| **Sphinx routing** | Onion encryption for anonymous packet forwarding |

### 💾 Storage (The Memory)

| Store | Purpose | Backend |
|-------|---------|---------|
| **SecureMessageStore** | Encrypted message history | Sled DB |
| **MailboxStore** | Offline message delivery (store-and-forward) | Sled DB |
| **BlockStore** | Content-addressed blob storage | In-memory LRU |
| **KnowledgeArk** | Network topology knowledge | In-memory |
| **TrustStore** | Peer reputation scores | In-memory |
| **DtnStore** | Delay-tolerant networking bundles | In-memory |

### 🧬 AI/ML (The Autonomic Nervous System)

```
Layer 1: ONNX (Fast Path)     → <1ms routing decisions (LightGBM)
Layer 2: Gemma 4 (Analytical) → 50-200ms every 30s (threat analysis)
Layer 3: GNN (Graph Opt.)     → offline every 5min (topology optimization)
Layer 4: PRoPHET (Fallback)   → <0.01ms always available (DTN formula)
Security: Isolation Forest    → anomaly detection → Gemma 4 threat analysis
```

### 🌐 Adapters (The Senses)

| Adapter | Connects To | Purpose |
|---------|-------------|---------|
| **Matrix** | Matrix homeserver | Bridge to Matrix/Element users |
| **Meshtastic** | LoRa radio | Bridge to Meshtastic mesh devices |
| **IRC** | IRC server | Bridge to legacy IRC chat |

---

## 4. Security Model — The Immune System

```
┌─────────────────────────────────────────────────────────────┐
│                    SECURITY LAYERS                           │
│                                                              │
│  Layer 1: Transport Encryption                               │
│  ┌─────────────────────────────────────────────────────┐    │
│  │  QUIC: TLS 1.3 built-in                             │    │
│  │  BLE: AES-CCM link encryption                       │    │
│  │  Stealth TCP: Obfuscated framing                    │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                              │
│  Layer 2: Sphinx Onion Routing                               │
│  ┌─────────────────────────────────────────────────────┐    │
│  │  Each hop only knows previous + next                 │    │
│  │  No single node sees full route                      │    │
│  │  Timing protection via mixing delays                 │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                              │
│  Layer 3: End-to-End Encryption                              │
│  ┌─────────────────────────────────────────────────────┐    │
│  │  XChaCha20-Poly1305 for message payloads             │    │
│  │  Ed25519 signatures for authenticity                 │    │
│  │  X25519 key exchange for forward secrecy             │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                              │
│  Layer 4: Application Security                               │
│  ┌─────────────────────────────────────────────────────┐    │
│  │  Rate limiting (FailedAttemptTracker)                │    │
│  │  Lockdown mode (block all traffic)                   │    │
│  │  Panic mode (nuke keys + decoy profiles)             │    │
│  │  Audit logging (all security events)                 │    │
│  │  CRL checking (revoked certificates)                 │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                              │
│  Layer 5: Metadata Protection                                │
│  ┌─────────────────────────────────────────────────────┐    │
│  │  SURB (Single-Use Reply Blocks) for anonymous replies│    │
│  │  Cover traffic (traffic_armor.rs)                    │    │
│  │  Padded frames (prevent size analysis)               │    │
│  └─────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

---

## 5. Identity System — The DNA

```
┌─────────────────────────────────────────────────────────────┐
│                    IDENTITY LIFECYCLE                         │
│                                                              │
│  Creation:                                                   │
│  ┌─────────────────────────────────────────────────────┐    │
│  │  Generate Ed25519 keypair                            │    │
│  │  Derive X25519 key (for DH key exchange)             │    │
│  │  Create DID (Decentralized Identifier)               │    │
│  │  Store in PersistenceManager (encrypted)             │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                              │
│  Pairing (Device → Device):                                  │
│  ┌─────────────────────────────────────────────────────┐    │
│  │  1. Device A generates QR code with public key       │    │
│  │  2. Device B scans QR, sends handshake               │    │
│  │  3. Both derive shared secret (X25519 DH)            │    │
│  │  4. Exchange encrypted identity packets              │    │
│  │  5. Add to TrustStore (TOFU model)                   │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                              │
│  Panic Mode:                                                 │
│  ┌─────────────────────────────────────────────────────┐    │
│  │  Trigger: Duress PIN or panic button                 │    │
│  │  Action: Destroy real keys, load decoy identity      │    │
│  │  Effect: System appears normal, real data gone       │    │
│  └─────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

---

## 6. DTN (Delay-Tolerant Networking) — The Reflexes

```
┌─────────────────────────────────────────────────────────────┐
│                    DTN MESSAGE FLOW                          │
│                                                              │
│  When direct connection fails:                               │
│                                                              │
│  1. Bundle Creation:                                         │
│     ┌─────────────────────────────────────────────────┐     │
│     │  Message → DtnBundle (with TTL, priority)        │     │
│     │  Store in DtnStore                                │     │
│     └─────────────────────────────────────────────────┘     │
│                                                              │
│  2. Encounter (Two nodes meet):                              │
│     ┌─────────────────────────────────────────────────┐     │
│     │  Exchange Bloom filters ("What do you have?")    │     │
│     │  Compare delivery scores (PRoPHET routing)       │     │
│     │  Forward bundles where P(deliver) is higher      │     │
│     └─────────────────────────────────────────────────┘     │
│                                                              │
│  3. Delivery:                                                │
│     ┌─────────────────────────────────────────────────┐     │
│     │  Destination receives bundle                     │     │
│     │  Decrypt, verify, deliver to application         │     │
│     │  Remove from DtnStore                            │     │
│     └─────────────────────────────────────────────────┘     │
│                                                              │
│  PRoPHET Formula:                                            │
│     P(A→B) = P(A→B)_old + (1 - P(A→B)_old) × β             │
│     where β = delivery predictability constant               │
└─────────────────────────────────────────────────────────────┘
```

---

## 7. Module Dependency Graph

```
                    ┌──────────┐
                    │   Core   │
                    └────┬─────┘
                         │
        ┌────────────────┼────────────────┐
        │                │                │
        ▼                ▼                ▼
┌───────────┐    ┌───────────┐    ┌───────────┐
│  Identity │    │Encryption │    │ Security  │
└─────┬─────┘    └─────┬─────┘    └─────┬─────┘
      │                │                │
      │                │                │
      ▼                ▼                ▼
┌───────────┐    ┌───────────┐    ┌───────────┐
│  Pairing  │    │   Sphinx  │    │   CRL     │
│  Manager  │    │  Routing  │    │  Store    │
└───────────┘    └─────┬─────┘    └───────────┘
                       │
                       ▼
                 ┌───────────┐
                 │   Mesh    │
                 │  Manager  │
                 └─────┬─────┘
                       │
          ┌────────────┼────────────┐
          │            │            │
          ▼            ▼            ▼
    ┌───────────┐ ┌───────────┐ ┌───────────┐
    │    DHT    │ │  Gossip   │ │   DTN     │
    │  (Kademlia)│ │  (Sub)   │ │  Store    │
    └───────────┘ └───────────┘ └───────────┘
```

---

## 8. Key Design Decisions

| Decision | Rationale |
|----------|-----------|
| **Sphinx over custom onion routing** | Battle-tested, formal security proofs, forward secrecy built-in |
| **XChaCha20-Poly1305** | Better nonce handling than AES-GCM, no need for separate AEAD nonce management |
| **libp2p as transport** | Mature P2P networking stack, QUIC + Kademlia + Gossipsub built-in |
| **Sled for persistence** | Embedded, crash-safe, zero-copy reads, good for mobile |
| **PRoPHET for DTN** | Proven algorithm for delay-tolerant networks (used in SWIM/DTN research) |
| **4-layer AI routing** | Fast path (ONNX) for every decision, deep analysis (Gemma) periodically |
| **Panic mode** | Critical for security: duress PIN destroys real keys, loads decoy identity |

---

## 9. What's Wired vs. What's Dead

### ✅ Wired and Working
- Core struct initialization (all 40+ fields)
- Mesh networking (libp2p swarm, Kademlia, Gossipsub)
- Sphinx onion routing (create + process packets)
- DTN epidemic routing (PRoPHET)
- QUIC transport (default)
- Message persistence (Sled)
- Security (rate limiting, audit logging, CRL)
- AI routing (ONNX fast path + Gemma analytical)
- Anomaly detection (Isolation Forest)
- Peer health tracking
- Traffic armor (cover traffic)
- Mailbox (offline delivery)
- Trust scoring

### 🟡 Wired but Unexercised
- anonymous_intro.rs — SURB-based anonymous contacts (needs Sphinx delivery integration)
- coercion.rs — Duress PIN + decoy profiles (needs identity unlock hook)
- function_calling.rs — NL→tool calling via Ollama (needs real handler wiring)
- typed_crypto.rs — CryptoKind + KeyBundle (needs consolidation into encryption.rs)

### 🔴 Dead (Deleted)
- private_routing.rs — CRITICAL BUG: onion keys derivable from public data
- network_scanner.rs — Zero callers
- obfs4_transport.rs — Zero callers
- pluggable_transport.rs — Orphan duplicate

---

## 10. Quick Reference — File → Purpose

| File | Lines | Purpose |
|------|-------|---------|
| `core/mod.rs` | 2278 | Core struct, initialization, message flow |
| `core/mesh.rs` | 2444 | libp2p swarm, MeshDriver actor, Sphinx processing |
| `core/encryption.rs` | 1100+ | XChaCha20, Ed25519, X25519, key management |
| `core/routing/sphinx.rs` | 800+ | Sphinx packet create/process (onion routing) |
| `core/dtn/epidemic.rs` | 600+ | PRoPHET DTN routing, DtnStore |
| `core/security_v2.rs` | 500+ | Rate limiting, audit logging, lockdown |
| `core/ai/mod.rs` | 400+ | AI module (Gemma, ONNX, GNN, anomaly) |
| `core/store.rs` | 158 | Encrypted message persistence |
| `core/mailbox.rs` | 392 | Offline message delivery |
| `core/reputation.rs` | 402 | Trust scoring |
| `core/peer_health.rs` | 308 | Peer connection quality tracking |
| `core/traffic_armor.rs` | 316 | Cover traffic + padded frames |
| `core/identity.rs` | 400+ | Identity lifecycle, key management |
| `core/identity_pairing/` | 500+ | Device pairing (QR-based) |
| `core/anonymous_intro.rs` | 351 | SURB-based anonymous contacts |
| `core/coercion.rs` | 261 | Duress PIN + decoy profiles |
| `core/function_calling.rs` | 1013 | NL→tool calling via Ollama |
