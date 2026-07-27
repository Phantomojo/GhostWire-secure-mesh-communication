# GhostWire Upgrade Map — From BitChat Debugging Session

**Generated:** 2026-07-26
**Source:** Live BLE debugging session with bitchat-plugin (Rust) ↔ BitChat Android (Kotlin)
**Status:** Ideation / Planning

---

## Part 1: BLE Transport Upgrades

### P0 — Critical (Blocker)

| # | Upgrade | Source | Why |
|---|---------|--------|-----|
| 1 | Replace `btleplug` with `bluer` | BLE session | btleplug 0.11 is central-only on Linux. `bluer` gives central + peripheral via BlueZ D-Bus. Required for mesh. |
| 2 | Use `AcquireNotify` over `StartNotify` | BLE session | BlueZ 5.86 `StartNotify` silently succeeds but never writes CCCD to remote GATT server. `AcquireNotify` (fd-based) actually works. |
| 3 | Noise handshake peer ID tiebreaker | BLE session | When both sides initiate simultaneously, lower hex peer ID wins. Without this, both become responders → deadlock. |

### P1 — High (Feature Parity)

| # | Upgrade | Source | Why |
|---|---------|--------|-----|
| 4 | GATT server + advertising + scanning | bitchat-plugin | GhostWire's `bluetooth.rs` is scan-only. Need all four roles. |
| 5 | BLE message fragmentation | BitChat | Protocol-level FragmentStart/Continue/End for payloads > MTU. |
| 6 | Delivery acknowledgments | BitChat | DeliveryAck (0x0A) + ReadReceipt (0x0C) for reliable delivery over unreliable BLE. |
| 7 | Auto-subscribe on connection | Android GATT server | Auto-add connected devices to subscribed list for resilience. |

### P2 — Medium (Enhancement)

| # | Upgrade | Source | Why |
|---|---------|--------|-----|
| 8 | BLE file transfer | BitChat | Voice notes, images over BLE with store-carry-forward. |
| 9 | Connection quality monitoring | Session analysis | Per-transport health metrics for auto-failover. |
| 10 | Fixed-size message padding | BitChat | Pad messages to resist traffic analysis on BLE. |
| 11 | ProtocolAck for BLE reliability | BitChat | ACK each received message for reliability without TCP. |

### P3 — Low (Documentation)

| # | Upgrade | Source | Why |
|---|---------|--------|-----|
| 12 | Expand BLE transport spec | Session analysis | Current spec is 12 lines. Add CCCD mechanics, AcquireNotify, notification lifecycle. |

---

## Part 2: Geohash Location Channels

### What BitChat Does
- Users pick geohash precision: Building(8), Block(7), Neighborhood(6), City(5), Province(4), Region(2)
- Messages scoped to geohash cell — only peers in same area receive them
- Geohash bookmarks with geocoder-friendly name resolution
- `LocationNotesManager` — kind=1 text notes tagged with geohash
- Geohash aliases for DM routing by location

### GhostWire Integration
- **Geohash Channel Layer** in protocol spec — messages scoped by precision
- Geohash prefixes as Kademlia DHT keys — location-based peer clustering
- Location Notes as first-class message type
- Privacy model: precision = privacy dial (precision 2 = region, precision 8 = building)

---

## Part 3: Nostr Bridge Transport

### What BitChat Does
- Full Nostr client with relay manager
- Ephemeral events (kind=20000) for geohash channel chat
- Presence events (kind=20001) for peer discovery
- Direct messages via NIP-04
- PoW validation for spam resistance
- BLE mesh + Nostr relay as parallel transports

### GhostWire Integration
- **Nostr bridge as first-class transport** — internet fallback when BLE mesh out of range
- Automatic transport failover: BLE → Nostr → BLE
- Geohash-scoped Nostr events map to DTN layer (store-carry-forward)
- PoW for spam resistance alongside reputation scoring

---

## Part 4: Protocol & Crypto Improvements

| Feature | BitChat Approach | GhostWire Approach | Recommendation |
|---------|-----------------|-------------------|----------------|
| **Key exchange** | Noise XX | Noise XX + X3DH | Use appropriate ratchet per transport |
| **Forward secrecy** | Noise session (simple) | Double Ratchet (full) | BLE = Noise session (5-min timeout). TCP = Double Ratchet |
| **Peer ID** | Ephemeral (random/session) | Persistent (key-derived) | Hybrid: persistent for routing, ephemeral for BLE |
| **Message padding** | Fixed-size padding | Payload morphing + timing chaff | BLE = fixed padding. TCP = full traffic armor |
| **Compression** | Per-message compression | Not explicit | Add compression for BLE payloads |
| **Identity storage** | EncryptedSharedPreferences | AES-256-GCM local store | Both equivalent, no change needed |
| **QR verification** | bitchat://verify URI | qr.rs exists | Follow bitchat://verify pattern |
| **Spam resistance** | PoW difficulty | Reputation scoring | Add PoW alongside reputation |

---

## Part 5: Message Types to Add

BitChat's 23 message types that GhostWire should support on BLE:

```
FragmentStart (0x05)    — Large message fragmentation
FragmentContinue (0x06) — Middle fragment
FragmentEnd (0x07)      — Final fragment
ChannelAnnounce (0x08)  — Channel discovery
DeliveryAck (0x0A)      — Delivery confirmation
ReadReceipt (0x0C)      — Read confirmation
FileTransfer (0x22)     — Binary file transfer
RequestSync (0x21)      — GCS-based sync request
```

---

## Part 6: GhostWire as Transport & Armor for Other Apps

### The Vision
GhostWire becomes a **universal encrypted transport layer** that apps like BitChat, Briar, Meshtastic, and custom mesh apps can plug into.

### Architecture
```
┌─────────────────────────────────────────┐
│  Third-Party Apps (BitChat, etc.)        │
├─────────────────────────────────────────┤
│  GhostWire Transport API                │
│  Transport trait + Metrics + Health      │
├─────────────────────────────────────────┤
│  GhostWire Core                          │
│  Noise XX │ Double Ratchet │ Sphinx     │
│  AI Routing │ DTN │ Traffic Armor        │
├─────────────────────────────────────────┤
│  Pluggable Transports                    │
│  BLE │ TCP │ WebRTC │ LoRa │ Nostr      │
└─────────────────────────────────────────┘
```

### Advantages
- Apps get encrypted mesh for free — just implement the Transport trait
- GhostWire's AI routing optimizes path selection across all connected apps
- Sphinx anonymous routing available to any app
- DTN (store-carry-forward) works across transport boundaries
- Single security audit covers all apps using GhostWire

### Disadvantages
- Coupling risk — GhostWire bugs affect all downstream apps
- API stability burden — breaking changes affect ecosystem
- Larger attack surface — more code, more vectors
- Performance overhead — abstraction layers add latency
- Harder to reason about — security properties become compositional

### Precedent
- **libp2p** does this for IPFS, Filecoin, Polkadot — transport abstraction works
- **Reticulum** does this for Meshtastic, Nostr, custom apps — simpler but proven
- **Tor** does this for onion services — transport isolation is critical

### Recommendation
GhostWire should expose a **minimal Transport SDK** — not the full stack. Apps choose what they need:
- Just encryption? → Noise XX + session management
- Just routing? → Kademlia DHT + AI path selection
- Just armor? → Traffic morphing + padding + timing chaff
- Full mesh? → Everything

This keeps the coupling loose and the API surface small.

---

## Part 7: Overlapping Ideas

### Idea A: GhostWire as BitChat Backend
GhostWire replaces BitChat's custom BLE mesh with GhostWire's full mesh stack. BitChat becomes a UI layer only.
- **Advantage**: BitChat gets AI routing, Sphinx, DTN for free
- **Disadvantage**: BitChat loses independence, becomes coupled to GhostWire releases

### Idea B: GhostWire Protocol Bridge
GhostWire nodes act as bridges between BitChat mesh and GhostWire mesh. Both ecosystems interoperate.
- **Advantage**: Neither project loses independence
- **Disadvantage**: Protocol translation overhead, version drift

### Idea C: GhostWire SDK for Android/iOS
GhostWire publishes a mobile SDK that apps like BitChat can integrate. Same core, different UIs.
- **Advantage**: Ecosystem growth, shared security audits
- **Disadvantage**: SDK maintenance burden, platform-specific quirks

### Idea D: GhostWire as Transport Plugin for Reticulum
GhostWire becomes a Reticulum transport — Reticulum handles routing, GhostWire handles encryption + armor.
- **Advantage**: Reticulum's proven routing + GhostWire's security
- **Disadvantage**: Reticulum already has encryption (Curve25519 + AES)

---

## Priority Summary

| Priority | Count | Theme |
|----------|-------|-------|
| P0 | 3 | BLE basics that must work |
| P1 | 4 | Feature parity with BitChat |
| P2 | 4 | Enhancements |
| P3 | 1 | Documentation |
| Ideation | 4 | GhostWire as transport layer |

**Total:** 16 upgrades + 4 architectural ideas

---

## Part 8: Algorithm Deep Dives — PRoPHET, Sphinx, DTN (DETAILED)

### 8.1 PRoPHET — Complete Reference

**Current state**: PRoPHET formula in `src/core/dtn/epidemic.rs` (609 lines). Uses v1 parameters: γ=0.98, P_init=0.75, β=0.25. Wired into mesh.rs via `dtn_store.on_encounter()` but no actual encounter loop triggers it.

#### PRoPHET v1 Core (Lindgren & Doria, 2003)

Three operations on every encounter:

**Update on direct encounter:**
```
P(a,b) = P(a,b)_old + (1 - P(a,b)_old) × P_encounter
```

**Aging** (exponential decay):
```
P(a,b) = P(a,b)_old × γ^k
```

**Transitivity:**
```
P(a,c) = P(a,c)_old + (1 - P(a,c)_old) × P(a,b) × P(b,c) × β
```

**Forwarding rule**: Forward bundle for destination D from A to B iff P(B,D) > P(A,D).

#### PRoPHET v2 Changes (RFC 6693, Grasic et al. 2011)

**The "Parking Lot Problem"**: When many nodes cluster (e.g., parking lot), every encounter triggers transitive updates. P(a,c) for *every* known node rises monotonically even if none of those nodes were actually met. Creates false high DPs for distant nodes, destroying the routing gradient.

**v2 fixes:**
- Direct encounter: `P_encounter = P_encounter_max × (IntvlB / I_typ)` when IntvlB < I_typ (reduced boost for frequent encounters)
- Transitive update changed from additive to max-based: `P(a,i) = max(P(a,i)_old, P(a,b) × P(b,i) × β)`
- `P_encounter_first` set to 0.5 (was 0.75 in v1)
- `P_encounter_max` caps maximum DP increment
- Explicit δ parameter: max DP = 1 - δ
- β=0.9 recommended (was 0.25 in some v1 specs)

#### State-of-the-Art Improvements (2024-2026)

| Improvement | Concept | Gain | Source |
|---|---|---|---|
| **R-PRoPHET** | Dynamic I_typ from weighted moving average of inter-encounter intervals. Forwarding uses ≥ instead of > | +20% delivery, -16% latency | Arteaga-Arce, Mitton 2024 |
| **TES-PRoPHET** | Triple exponential smoothing over DP time series | Higher delivery, lower delay vs v2 | Peng et al. 2024 |
| **LH-PRoPHET** | Trajectory prediction from position history | Higher delivery, lower resource use | Matsubara, Hayashibara 2025 |
| **MLProph** | Decision tree + shallow NN for forwarding decisions | Matches PRoPHET with less state | Sharma et al. 2016 |
| **AIPropRF/XGBoost** | Random Forest / XGBoost for optimal next-hop | Better than PRoPHET in heterogeneous | Chourasia et al. 2024 |
| **ANN param predictor** | 4-layer feedforward (64→32→16→3) predicts α, β, γ | 99% accuracy, ~100 params | Kumar et al. 2023 |

#### Known Weaknesses & Fixes

1. **Random Waypoint mobility kills PRoPHET** — DPs never converge. Fix: detect mobility entropy, fall back to Spray-and-Wait
2. **Slow dissemination** — no forwarding when DPs are zero. Fix: epidemic boost for first N copies
3. **No buffer management** — doesn't consider remaining buffer/TTL. Fix: multi-factor scheduling (DP > hop count > traffic load)
4. **Single metric fragility** — DP alone doesn't capture energy/bandwidth. Fix: weighted deliverability score
5. **Parameter sensitivity** — α, β, γ are static. Fix: ANN prediction or R-PRoPHET dynamic estimation
6. **No security** — malicious nodes can advertise fake DPs. Fix: Sign DP exchanges with Ed25519

#### Data to Collect for ML Routing

| Field | Type | Why |
|---|---|---|
| `timestamp` | u64 | Time of encounter |
| `peer_id` | [u8; 32] | Node identity |
| `encounter_duration_s` | f32 | How long contact lasted |
| `throughput_bps` | f32 | Historical throughput |
| `buffer_occupancy` | f32 | % buffer full |
| `battery_level` | f32 | 0.0–1.0 remaining |
| `dp_after` | f32 | DP value after update |
| `inter_encounter_interval_s` | f32 | Time since last encounter |
| `num_hops` | u8 | Hops of bundles exchanged |
| `delivery_success` | bool | Label: did bundle reach destination? |
| `transports_active` | u8 | Bitmask (BLE=1, WiFi=2, LoRa=4, TCP=8) |

Most predictive features: DP value history, inter-encounter interval variance, encounter duration, node popularity, transitivity depth, throughput, battery trend, time-of-day patterns.

#### Algorithm Comparison

| Protocol | Type | Delivery | Overhead | Best For |
|---|---|---|---|---|
| Epidemic | Flooding | Highest | Highest | Small nets, low buffer |
| PRoPHET v2 | Probabilistic | Medium-High | Medium | Predictable mobility |
| Spray-and-Wait | Quota-based | Medium-High | **Lowest** | Resource-constrained |
| MaxProp | Priority-based | High | Medium | Buffer-aware routing |
| Spray-and-Focus | Hybrid | **Highest** | Low | General purpose |

**GhostWire decision**: Use PRoPHET v2 as base, with R-PRoPHET's dynamic I_typ + ML enhancement (AIPropRF for Layer 1 routing). Fall back to Spray-and-Wait when mobility looks random.

#### Multi-Transport PRoPHET

Run **separate DP tables per transport interface**:
- P_wifi[N][dest] — based on TCP/WiFi encounters
- P_ble[N][dest] — based on BLE encounters
- P_lora[N][dest] — based on LoRa encounters

Cross-transport inference: If A meets B via WiFi, slightly increase P_ble(A, B) too (shared mobility signal).

Nostr as transitive accelerator: Relays act as "meeting places" for DP exchange even when nodes are never in physical proximity — effectively giving global transitive knowledge. This breaks PRoPHET's assumption about physical encounters but could be a feature for internet-connected nodes.

#### Rust Implementations Available

| Project | Status | Features |
|---|---|---|
| **ghostwire-dtn** (ours) | Active v0.1.0 | PRoPHET + Spray-and-Wait, bloom filter sync, XChaCha20 encryption, GNN routing |
| **dtn7-rs** | Active v0.21.0 | BPv7, epidemic/sprayandwait, external routing API |
| **hardy** | Active | Full BPv7 + BPSec, Time-Variant Routing, filter pipeline |
| **murmuration-routing** | Active v0.2.0 | DTN routing evaluation toolkit with UCB1 bandit, Q-routing |

#### PRoPHET v2 Implementation Checklist for GhostWire

```rust
// Minimal PRoPHET v2 core
pub struct ProphetRouter {
    dp: HashMap<[u8; 32], HashMap<[u8; 32], f64>>,  // peer -> {dest -> P}
    last_encounter: HashMap<(TransportType, [u8; 32]), Instant>,
    last_aged: Instant,
    // Config
    p_encounter_first: f64,  // 0.5
    p_encounter_max: f64,    // 0.7
    beta: f64,               // 0.9
    gamma: f64,              // 0.999
    delta: f64,              // 0.01
    i_typ: Duration,         // transport-specific
}
```

### 8.2 Sphinx — Anonymous Routing Origin (DETAILED)

**Current state**: In-tree `routing/sphinx.rs` (809 lines) is FULLY WIRED into mesh.rs. Standalone `sphinx-rs` crate (970 lines) is installed but NOT used. Two competing implementations.

#### How Sphinx Originates Messages

Sphinx is a packet format, not a network. The originator:
1. Creates a Sphinx packet with layered encryption (onion layers)
2. Each layer = one hop's routing info + encryption key
3. Packet travels through mix nodes, each peeling one layer
4. Final hop delivers to destination payload (encrypted with destination's key)

#### GhostWire's In-Tree Sphinx (what's actually wired)

From `routing/sphinx.rs`:
- Real X25519 scalar multiplication for alpha re-randomization
- SURBs (Single-Use Reply Blocks) with AEAD reply payload encryption
- MAC chain verification with constant-time comparisons
- Small-subgroup attack defense
- `create_packet()` / `process_packet()` — used by mesh.rs `select_sphinx_route()` (line 831)
- Known issue: MAC domain is public constant (`SPHINX_MAC_DOMAIN`), making gamma forgeable by any observer

#### Standalone sphinx-rs Crate (NOT used)

From `ghostwire-libs/crates/sphinx-rs/lib.rs`:
- Configurable `SphinxConfig` with domain separation for all 5 key domains (rho, mu, blind, shared, kdf)
- Proper Sphinx backward header construction with fixed-size routing blocks (128 bytes each)
- Known bug: 3-hop test marked `#[ignore]` — "Fix requires redesigning header construction to avoid truncation"

#### For GhostWire to ORIGINATE + BE A MIX NODE

1. **Originator mode**: Build Sphinx packets, route through GhostWire mix nodes
2. **Mix node mode**: Accept Sphinx packets, shuffle with cover traffic, forward
3. **SURBs**: Generate SURBs, send to destination. Destination encrypts reply payload, sends back through reverse Sphinx path
4. **Hub/switch/converter**: Accept packets from ANY format (BitChat, Meshtastic), wrap in Sphinx, route, unwrap at destination, convert back

#### What Needs Wiring

- `routing/private_routing.rs` (353 lines): Veilid-style onion routing — COMPLETE but NOT WIRED (zero callers)
- `routing/mixing_delay.rs` (308 lines): Exponential mixing delays — PARTIALLY WIRED (queue exists, delay not applied)
- `routing/skademlia.rs` (509 lines): S/Kademlia defenses — COMPLETE but NOT WIRED (zero callers)
- Cover traffic generator exists (`Frame::cover_traffic()`) but no idle loop produces it

### 8.3 DTN — Universal Store-Carry-Forward (DETAILED)

**Current state**: In-tree `epidemic.rs` (609 lines) + `bundle.rs` (135 lines). Wired into Core and mesh.rs. Standalone `ghostwire-dtn` crate (777 lines) duplicates code + adds X25519 bundle encryption.

#### What's Implemented

- `DtnStore`: HashMap-based bundle storage, LRU-by-delivery-probability eviction at 50MB
- `DeliveryTable`: Time-decayed DP (γ=0.98, P_init=0.75, β=0.25)
- `SyncBloom`: Bloom filter for set reconciliation
- `DtnSyncHandshake`: Data structure exists but no over-wire exchange
- Spray-and-Wait binary copy split
- AI enhancement: `Core::handle_dtn_encounter()` supports GNN-enhanced forwarding with PRoPHET fallback

#### What's Missing

1. **No encounter loop**: `on_encounter()` exists but nothing triggers it — no background task that detects BLE proximity and calls it
2. **No over-wire protocol**: `DtnSyncHandshake` struct exists but mesh.rs only builds it — no actual sync exchange
3. **No persistence**: Bundles live in memory, restart loses everything
4. **Duplicate code**: In-tree epidemic.rs vs standalone ghostwire-dtn crate are ~80% identical
5. **DTN not wired to transports**: Spec says "DTN is a PROPERTY of all transports" but it's only accessible via mesh.rs gossipsub

#### Multi-Transport DTN Design

| Transport | DTN Behavior | Buffer Strategy |
|---|---|---|
| BLE | Store when peer disconnects, carry until reconnect | In-memory, 100MB |
| TCP | Store when connection drops | SQLite, persistent |
| LoRa | Store when out of range, carry via mobile nodes | SQLite, compressed |
| Nostr | Post to relay, relay stores until peer subscribes | Relay's storage |
| WebRTC | Store when peer goes offline | SQLite, persistent |

#### Bundle Encryption (from ghostwire-dtn crate, not in-tree)

The standalone crate has `EncryptedBundlePayload` using X25519. This should be folded into the in-tree version.

### 8.4 GhostWire as Hub/Switch/Converter (DETAILED)

**The vision**: GhostWire becomes the **universal translator** for mesh networks.

```
BitChat app ──→ GhostWire ──→ Sphinx mix ──→ GhostWire ──→ Meshtastic app
                    │                              │
                    ├── BLE transport               ├── LoRa transport
                    ├── TCP transport               ├── TCP transport
                    ├── Nostr relay                 └── Nostr relay
                    └── LoRa transport
```

**Capabilities:**
1. **Protocol translation** — receive BitChat packets, convert to GhostWire format, route
2. **Transport switching** — BLE → TCP → Nostr → LoRa as available
3. **Anonymization** — wrap any packet in Sphinx layers
4. **Store-carry-forward** — DTN on every transport
5. **AI routing** — 4-layer stack optimizes path across ALL transports
6. **Traffic armor** — padding, morphing, timing chaff on all outgoing traffic

**No other project does ALL of this.** libp2p does transport abstraction. Reticulum does multi-transport. Tor does anonymity. GhostWire does all three + AI routing + DTN.

---

## Part 9: Nostr — What Is It? (DETAILED)

**Nostr** = "Notes and Other Stuff Transmitted by Relays"

**It is NOT Tor.** It's simpler:
- **Tor**: Encrypted circuit routing through volunteer nodes. Provides anonymity.
- **Nostr**: Signed events posted to relays. Provides censorship resistance, NOT anonymity.

#### Nostr Basics

**Event Structure (NIP-01):**
```json
{
  "id": "<SHA-256 hash>",
  "pubkey": "<schnorr secp256k1 pubkey>",
  "created_at": <unix timestamp>,
  "kind": <integer>,
  "tags": [["e", ...], ["p", ...], ["g", ...]],
  "content": "<string>",
  "sig": "<64-byte schnorr sig>"
}
```

**Event Kind Ranges:**
| Range | Behavior | Use |
|-------|----------|-----|
| 0–9999 | Regular (stored) | Text notes, DMs |
| 10000–19999 | Replaceable | Only latest per pubkey kept |
| **20000–29999** | **Ephemeral** | **Not stored — forwarded to live subscribers only** |
| 30000–39999 | Addressable | Latest per pubkey+kind+d tag |

#### Key NIPs for GhostWire

**NIP-44 v2** (current standard): ChaCha20 + HMAC-SHA256, custom padding, ECDH key agreement. Replaces flawed NIP-04 (AES-CBC). Maximum plaintext: 64KB.

**NIP-59 Gift Wrap** (sender anonymity): Three-layer encapsulation:
1. Rumor — unsigned event with real content
2. Seal (kind 13) — rumor encrypted via NIP-44 to recipient
3. Gift Wrap (kind 1059/21059) — seal encrypted with ephemeral key

Relays see only ephemeral pubkey + recipient p-tag. Cannot identify real sender. Use `kind:21059` for live mesh bridge traffic, `kind:1059` for store-and-forward.

**NIP-65 Relay List Metadata** (kind 10002): Users advertise which relays they read from/write to. Keep lists small (2-4 per category). Publish to well-known indexer relays.

**NIP-13 PoW**: Mine event id to have N leading zero bits. Recommended: 8-12 for mesh-transit, 16-20 for direct relay submissions.

**NIP-BE** (official Nostr BLE spec, merged 2025): GATT server/client roles by UUID comparison. DEFLATE compression, chunking for BLE MTU. Exactly the Nostr+BLE hybrid GhostWire needs.

#### Relay Infrastructure (2026)

- Total known: ~5,735
- Reachable: ~1,341 (23.4%)
- Geographic spread: 50 countries, 151 autonomous systems
- No single country or AS controls >25% of relays
- 290+ BitChat-compatible (support kind 20000)

#### BitChat's Nostr Usage (Model for GhostWire)

**Ephemeral Geohash Events (kind=20000)**: Location-based chat using geohash-precision channels. Tagged with `["g", geohash]`. Ephemeral (relays don't store long-term).

**Presence Heartbeats (kind=20001)**: Empty content, just `["g", geohash]` tag. Broadcast every 40-80s (randomized). Privacy: only broadcast at precision ≤ 5. Per-broadcast random delays prevent temporal correlation.

**Geohash-Derived Identities**: Each geohash channel gets ephemeral pubkey derived from master key + geohash string. Fresh identity per location, prevents cross-location correlation.

**Dual Transport Fallback**:
1. BLE mesh (direct Noise session) — fastest, most private
2. Nostr fallback (NIP-17 gift-wrapped DM via global relays)
3. Smart queuing — messages stored until transport available

#### Rust Nostr Libraries

**`nostr-sdk`** (v0.44.1 stable): High-level client SDK with NIP-44, NIP-59, Tor proxy support, built-in PoW, relay pool management.

```rust
use nostr_sdk::prelude::*;
let client = Client::builder().signer(keys).build();
client.add_relay("wss://relay.damus.io").await?;
// Publish with PoW
EventBuilder::new(Kind::Custom(20007), "encrypted-payload").pow(16);
// Gift wrap
client.gift_wrap(&recipient_pubkey, unsigned_event, []).await?;
```

Feature flags for GhostWire: `nip44`, `nip59`, `tor`, `pow-multi-thread`, `nip65`.

#### Nostr + GhostWire Architecture

```
┌─────────────────────────────────────────────────┐
│  Transport Layer (transport_selector)            │
│  ┌──────────────┐  ┌─────────────────────────┐  │
│  │ BLE Mesh     │  │ Nostr (internet bridge) │  │
│  │ (NIP-BE)     │  │ • Kind 20007 (encrypted)│  │
│  │ • Noise P2P  │  │ • Kind 20008 (receipts) │  │
│  │ • 7-hop mesh │  │ • Kind 21059 (giftwrap) │  │
│  └──────────────┘  └─────────────────────────┘  │
├─────────────────────────────────────────────────┤
│  Encryption Layer                                │
│  ┌──────────────┐  ┌─────────────────────────┐  │
│  │ Noise Proto  │  │ NIP-44 + NIP-59         │  │
│  │ (mesh)       │  │ (internet)               │  │
│  └──────────────┘  └─────────────────────────┘  │
└─────────────────────────────────────────────────┘
```

**Key design decisions:**
1. Use kind 20000–29999 for mesh-transit events (ephemeral)
2. Separate inbox kind (e.g., kind 20007) for store-and-forward
3. NIP-44 + NIP-59 for all internet-transit messages
4. Geohash tags with geohash-derived pubkeys for location channels
5. NIP-65 relay lists for peer inbox discovery
6. Hybrid courier: BLE nodes carry messages for disconnected peers, deposit to Nostr when internet available

---

## Part 10: GhostWire as Transport & Armor for Other Apps

### The Architecture

```
┌─────────────────────────────────────────────┐
│  Third-Party Apps                            │
│  BitChat │ Meshtastic │ Briar │ Custom      │
├─────────────────────────────────────────────┤
│  GhostWire SDK (3 crates)                   │
│  ghostwire-crypto │ ghostwire-transport     │
│  ghostwire-mesh                             │
├─────────────────────────────────────────────┤
│  Core Services                              │
│  Noise XX │ Sphinx │ DTN │ AI Routing       │
│  Traffic Armor │ Geohash Channels           │
├─────────────────────────────────────────────┤
│  Pluggable Transports                       │
│  BLE │ TCP │ WebRTC │ LoRa │ Nostr │ obfs4  │
└─────────────────────────────────────────────┘
```

### How Apps Use GhostWire

**Level 1 — Just Encryption:**
```rust
use ghostwire_crypto::{NoiseSession, encrypt, decrypt};
// App handles its own transport
```

**Level 2 — Transport + Encryption:**
```rust
use ghostwire_transport::{BleTransport, TcpTransport, TransportManager};
// GhostWire handles BLE/TCP, app handles routing
```

**Level 3 — Full Mesh:**
```rust
use ghostwire_mesh::{MeshNode, Dht, SphinxRouter, DtnManager};
// GhostWire handles everything, app is just UI
```

### The Armor Layer

GhostWire can sit between ANY app and its transport:

```
BitChat app → [GhostWire Armor] → BLE
                ├── Message padding (fixed-size)
                ├── Traffic morphing (random bytes)
                ├── Timing chaff (random delays)
                ├── Sphinx wrapping (anonymity)
                └── DTN buffering (store-carry-forward)
```

This means BitChat gets GhostWire's security without changing its code. Just route traffic through GhostWire.

### Advantages

1. **One security audit** covers all apps
2. **Network effects** — more nodes = better mesh = more value
3. **Specialization** — GhostWire focuses on security, apps focus on UX
4. **Interoperability** — BitChat talks to Meshtastic via GhostWire
5. **Reduced duplication** — every mesh app reimplements Noise, DHT, fragmentation

### Disadvantages

1. **API surface** — every feature exposed is a feature to maintain
2. **Version coupling** — breaking changes affect all downstream apps
3. **Security surface** — more code = more attack vectors
4. **Performance overhead** — abstraction layers add latency
5. **Governance** — who decides the API? who reviews PRs?

### Recommendation

Ship as **3 crates**:
1. `ghostwire-crypto` — Noise XX, Sphinx, ML-KEM. Zero deps.
2. `ghostwire-transport` — Transport trait + implementations. Depends on crypto.
3. `ghostwire-mesh` — DHT, GossipSub, DTN, AI routing. Depends on transport.

Apps choose their level. Loose coupling. Small APIs.

---

## Part 11: Codebase Gap Analysis — Spec vs Implementation

### Implementation Status Matrix

| Feature | Status | Lines | Wired? | Gap |
|---------|--------|-------|--------|-----|
| DTN epidemic (in-tree) | ✅ IMPLEMENTED | 609 | Partially | No encounter loop, no over-wire protocol, no persistence |
| ghostwire-dtn crate | ⚠️ DUPLICATE | 777 | Not used | Has bundle encryption in-tree doesn't |
| Sphinx routing (in-tree) | ✅ IMPLEMENTED | 809 | Yes | 3-hop broken; MAC domain public |
| Sphinx (sphinx-rs crate) | ⚠️ UNUSED | 970 | No | Installed but not imported anywhere |
| Private routing (onion) | ✅ IMPLEMENTED | 353 | No | Zero callers |
| S/Kademlia defenses | ✅ IMPLEMENTED | 509 | No | Zero callers, mesh.rs uses default gossipsub |
| Mixing delays | ⚠️ PARTIAL | 308 | Partial | Queue exists, delay not applied |
| AI routing classifiers | ⚠️ LOADED | N/A | No | ONNX/GNN loaded but never consulted |
| Transport trait | ❌ ABSENT | 0 | N/A | Enum-based dispatch, no trait, no plugin model |
| Transport switching | ⚠️ ARCHITECTURE | 54 | No | AutoTransportSwitcher exists but never invoked |
| TransportKind/Type mismatch | ❌ BROKEN | — | — | Two enums, no conversion between them |
| Double Ratchet | ✅ IMPLEMENTED | 567 | Partial | In Encryption struct, not in message path |
| Wire Protocol | ✅ IMPLEMENTED | 424 | No | FrameParser never called |
| Cover Traffic | ⚠️ MINIMAL | ~10 | No | Generator exists, no idle loop |
| Reliability layer | ✅ IMPLEMENTED | 530 | Yes | No periodic timeout check loop |
| core_wiring.rs | ⚠️ SPEC ONLY | 367 | No | All comments, zero executable code |

### Highest-Impact Fixes (by effort/impact ratio)

1. **Wire S/Kademlia** — 3 lines in gossipsub construction → Sybil attack defense
2. **Wire AutoTransportSwitcher** — periodic metrics polling → actual transport switching
3. **Reconcile DTN** — fold bundle encryption from ghostwire-dtn into in-tree epidemic.rs
4. **Add FrameKind::SphinxPacket/DtnBundle** — use FrameParser in mesh.rs
5. **Wire mixing delays** — one `await` call before Sphinx forwarding
6. **Add cover traffic loop** — background task using existing Frame::cover_traffic()
7. **Populate WireMessage.seq** — per-peer monotonic counter for ReplayWindow

### Duplicate Code to Reconcile

- In-tree `epidemic.rs` vs standalone `ghostwire-dtn` crate: ~80% identical
- In-tree `routing/sphinx.rs` vs standalone `sphinx-rs` crate: different implementations
- `TransportKind` (transport.rs) vs `TransportType` (disaster.rs): same purpose, no conversion

---

## Updated Priority Summary

| Priority | Count | Theme |
|----------|-------|-------|
| P0 | 3 | BLE basics |
| P1 | 4 | Feature parity |
| P2 | 4 | Enhancements |
| P3 | 1 | Documentation |
| Algorithm | 3 | PRoPHET v2+ML, Sphinx originate+mix, DTN universal |
| Architecture | 3 | Hub/switch/converter, 3-crate SDK, armor layer |
| Codebase Gap | 7 | Wire S/Kademlia, transport switching, reconcile duplicates |

**Total:** 25 action items across 7 categories

---

*This file is a living document. Update as implementation progresses.*
*Last updated: 2026-07-26 — Added deep research: PRoPHET v2 algorithm details, Nostr NIPs/relay infra, codebase gap analysis*
