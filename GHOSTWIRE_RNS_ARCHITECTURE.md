# GhostWire + Reticulum Integration — Architecture & Vision Document

**Date**: 2026-08-03
**Author**: PHANTOMOJO (Michael Irungu) + AI Architecture Session
**Status**: Vision & Architecture Finalized — Ready for Implementation

---

## 1. Executive Summary

This document captures the complete architectural vision for integrating Reticulum (RNS) into GhostWire, establishing GhostWire as a **Universal Mesh Aggregator Hub** that bridges multiple disparate mesh networks (libp2p, Reticulum, Meshtastic, Briar) into a single, intelligent, censorship-resistant communication platform.

**Key Decision**: GhostWire will run **both** `libp2p` AND `rns-rs` (Rust port of Reticulum) simultaneously, using the existing `TransportRegistry` hub architecture to route traffic across both stacks seamlessly.

---

## 2. What GhostWire Actually Is

GhostWire is NOT just a mesh network. It is a **Layer 7 Overlay Router, Universal Mesh Aggregator, and Threat Intelligence Platform**.

Think of it like an octopus:
- **The Brain**: The TUI, AI/ML threat detection, application logic, identity management
- **The Tentacles**: The transports (libp2p, Reticulum, BLE, WiFi Direct, Meshtastic, Briar, etc.)

GhostWire is **transport-agnostic**. It doesn't care HOW a packet gets from Node A to Node B. The `TransportRegistry` dispatches messages across whichever transport is available.

### What Makes GhostWire Unique (vs. Reticulum alone)
- Reticulum connects the world at the protocol level
- NomadNet lets humans talk over Reticulum
- **GhostWire makes the network intelligent and multi-modal**

Reticulum CANNOT:
- Run AI/ML Threat Intelligence
- Talk to libp2p (IPFS, Ethereum, WebRTC) networks
- Bridge between Meshtastic, Briar, and other isolated protocols
- Stream high-bandwidth data efficiently
- Provision ESP32 firmware

**Only GhostWire can do all of these**, because it sits above all transport networks as the universal hub.

---

## 3. The Hybrid Architecture (libp2p + RNS)

### Why Both?

| Feature | libp2p | Reticulum (RNS) |
|---------|--------|-----------------|
| High-bandwidth Internet | ✅ Excellent | ❌ Not designed for it |
| WebRTC browser connections | ✅ Native | ❌ Not supported |
| Off-grid radio (LoRa) | ❌ Cannot | ✅ Native |
| Zero-IP networking | ❌ Requires IP | ✅ No IP needed |
| Raw WiFi frames (no router) | ❌ Cannot | ✅ AutoInterface |
| BLE mesh routing | ❌ Limited | ✅ Native |
| NAT traversal | ✅ STUN/TURN | ✅ Built-in |

### How They Coexist

```
┌─────────────────────────────────────────────────┐
│              GhostWire Application               │
│  (TUI, AI Immune System, Identity, Contacts)     │
├─────────────────────────────────────────────────┤
│              TransportRegistry (Hub)             │
├──────────┬──────────┬──────────┬────────────────┤
│  libp2p  │   RNS    │   BLE   │  Meshtastic    │
│ (Internet│ (Off-grid │(Proximity│  (LoRa Radio)  │
│  TCP/UDP)│  Radio)  │  Mesh)  │                │
└──────────┴──────────┴──────────┴────────────────┘
```

- **Internet alive**: GhostWire uses `libp2p` for maximum bandwidth
- **Internet dead, WiFi router available**: `libp2p` mDNS on local network still works
- **Internet dead, NO routers**: GhostWire fails over to Reticulum (raw WiFi frames, BLE, LoRa)
- **Total blackout**: Reticulum over serial/LoRa radio, BLE hops

### Implementation Approach

1. Add `rns-rs` to `Cargo.toml` as a library dependency
2. Create `reticulum_transport.rs` implementing the existing GhostWire `Transport` trait
3. Add `TransportKind::Reticulum(Arc<ReticulumTransport>)` variant
4. Write fallback logic: "If IP available → libp2p; if IP dead → Reticulum"

**Estimated effort**: 2-4 hours of focused pair programming

---

## 4. Multi-Hop Cross-Medium Routing

GhostWire's killer feature: seamlessly routing a single message across multiple physical mediums.

### Example Scenario
1. **Bob (on Internet)** sends a message to **Alice (off-grid on LoRa)**
2. GhostWire checks the routing table → finds **John** advertises a path to Alice
3. GhostWire sends the packet over the Internet to John's node
4. John's node receives it via Internet, realizes Alice is reachable via LoRa
5. John's node blasts it out over the 900MHz LoRa radio
6. **Alice receives the message** on her LoRa antenna

Neither Bob nor Alice knew what transport the other was using. GhostWire figured it out automatically.

---

## 5. The "Airport" Hub Model (Open Network + GhostWire Badge)

### Design Philosophy
GhostWire does NOT isolate itself from the public mesh. It operates as an **open airport**:

1. **The Public Terminal (Open Routing)**: GhostWire nodes sit on the public Reticulum mesh. Random Reticulum users can route packets through GhostWire nodes. GhostWire provides critical infrastructure to the masses.

2. **The VIP Lounge (Genesis Constants)**: When GhostWire users communicate with each other, their data is packed using the `FRAME_MAGIC` and `CHANNEL_INVITE_PREFIX` from the Genesis Ceremony. To any normal Reticulum user, it looks like standard encrypted traffic. But GhostWire nodes recognize the "badge."

3. **Airport Security (AI Immune System)**: Because GhostWire processes public traffic, the AI can analyze routing behavior, traffic patterns, and anomalies passing through it.

### Using Non-GhostWire Users as Courier Pigeons
GhostWire can route encrypted packets through standard Reticulum or Meshtastic users who have never heard of GhostWire:
- The packet is end-to-end encrypted (ML-KEM-768) before leaving the sender
- The outer transport layer (Reticulum/Meshtastic) wraps it in its own encryption
- Intermediate nodes see a standard-looking packet and blindly route it
- Only the destination GhostWire node can decrypt the inner payload

The entire global Reticulum and Meshtastic infrastructure becomes free, anonymous relay infrastructure for GhostWire.

---

## 6. Bridging Other Protocols (Meshtastic, Briar, etc.)

### How GhostWire Bridges Isolated Networks

GhostWire acts as a **protocol translator**. The developers of Meshtastic and Briar will never add a "Send via GhostWire" button. Instead, GhostWire impersonates a native user on each network.

**Hardware/API Plugins:**
- **Meshtastic**: USB serial radio plugged into GhostWire server → Meshtastic Serial API
- **Briar**: Headless Briar instance running in background → Briar API
- **Reticulum**: `rns-rs` embedded directly in GhostWire Rust binary

**Method A: Gateway Bot (For non-GhostWire users)**
- Alice (Meshtastic) sends: `/send bob@briar Hey Bob!`
- GhostWire receives it via Meshtastic radio, translates, injects into Briar API
- Bob receives on Briar: "[From Alice on Meshtastic]: Hey Bob!"

**Method B: Seamless Routing (For GhostWire users)**
- User adds Alice's Meshtastic ID and Bob's Briar ID to GhostWire contacts
- GhostWire automatically routes to the correct network per contact
- User never thinks about which protocol their contacts are on

---

## 7. Nested Encryption (Security Across Mediums)

GhostWire uses **double-layer (onion) encryption**:

### Inner Layer (GhostWire's Own)
- ML-KEM-768 (Post-Quantum, from Genesis constants)
- Applied BEFORE the message leaves the app
- Survives even if the transport encryption is cracked

### Outer Layer (Transport Medium)
- **libp2p**: Noise Protocol (similar to TLS)
- **Reticulum**: Ed25519/X25519
- **Meshtastic**: AES-256

Even if a government cracks Meshtastic's AES-256, they find GhostWire's post-quantum encryption sitting inside. They are completely blocked.

---

## 8. Fragmentation Across Mediums

### The MTU Problem
- Internet (libp2p): megabytes per packet ✅
- Reticulum: ~500 bytes per packet
- Meshtastic/LoRa: ~230 bytes per packet
- GhostWire ML-KEM-768 encrypted payload: ~1,080+ bytes

### The Solution (Already Built)
GhostWire's **Fragmenter/Reassembler** (referenced in `TRANSPORT_IMPLEMENTATION_PLAN.md`) handles this:
1. **Fragment**: Chop the 1,080-byte payload into 5 pieces of 230 bytes each
2. **Sequence**: Header on each piece: "I am piece 1 of 5", "piece 2 of 5", etc.
3. **Transmit**: Send pieces individually over the constrained medium
4. **Reassemble**: Destination node stitches pieces back together, then decrypts

The user never notices the fragmentation.

---

## 9. Universal GhostWire Identity System

### The Problem
Without a unified identity, users get fragmented across networks:
- libp2p → PeerId
- Reticulum → Destination Hash
- Meshtastic → Device ID

### The Solution: Single Master Identity

When a user first launches GhostWire, the app generates ONE master cryptographic keypair using ML-KEM-768 + Ed25519 (tied to Genesis constants):

```
GW:7f3a9c21e8b4d056...
```

From this master key, GhostWire derives child keys for each transport using HKDF:

```
Master Key (GW:7f3a9c21...)
    ├── derives → libp2p PeerId
    ├── derives → Reticulum Destination Hash
    ├── derives → Meshtastic Device Key
    └── derives → Briar Identity
```

### Properties
- **Permanent**: Survives reboots, IP changes, network switches, hardware changes
- **Universal**: One identity across every network
- **Private**: Each network only sees the derived child key; cannot correlate across networks
- **Recoverable**: Backup master keyfile → restore on new device → all child keys regenerate

---

## 10. Human-Readable Naming System

### Display Names with Discriminators

```
Display Name:    PHANTOMOJO
GhostWire Tag:   PHANTOMOJO#7F3A
Full ID:         GW:7f3a9c21e8b4d056...
```

- Chatting: Users see "PHANTOMOJO"
- Adding contacts: Search "PHANTOMOJO" → pick the right one via `#7F3A` discriminator
- Under the hood: Always routes to the full `GW:` hash

### Name Resolution (Decentralized, No DNS)
- Nodes broadcast Announce packets: "PHANTOMOJO = GW:7f3a..."
- Other nodes cache the mapping locally
- Searching queries local cache, then asks nearby nodes

### Anonymity Modes
- **Public Mode**: Name broadcast in Announces. Anyone can find you by searching.
- **Ghost Mode**: Name NEVER broadcast. Only findable via QR code or direct ID share.
- **Contacts-Only Mode**: Name visible only to existing contacts.

### Name Changes
- User updates display name → next Announce broadcasts new name
- Contact lists auto-update
- Underlying `GW:` ID never changes
- All message history and trust remain intact

---

## 11. Reserved Names (Founder Privilege)

### PHANTOMOJO is Permanently Reserved

A `reserved_identities.rs` file in the codebase hardcodes founder names:

```rust
pub const RESERVED_NAMES: &[ReservedIdentity] = &[
    ReservedIdentity {
        name: "PHANTOMOJO",
        bound_to: "GW:7f3a9c21e8b4d056...", // Genesis keypair
        reason: "Founder",
    },
];
```

### Fuzzy Matching (All Variations Blocked)
Normalization before checking:
- Strip spaces, underscores, hyphens, dots
- Convert to lowercase
- Homoglyph detection (O vs 0 vs Ø)

Blocked for everyone except the founder:
- `PHANTOMOJO`, `phantomojo`, `PhAnToMoJo`, `PHANTOM_MOJO`
- `PHANTOM.MOJO`, `P H A N T O M O J O`, `PHANTØM0JO`

### Verified Badge
```
🔒 PHANTOMOJO  ← regular user (self-claimed name)
⚡ PHANTOMOJO  ← Genesis-verified founder (cryptographically proven)
```

The Genesis Ceremony keypair provides mathematical proof of founder status.

---

## 12. Protest / Blackout Resilience

### Scenario: Government Internet Kill-Switch

1. **DPI Censorship (Internet throttled)**: GhostWire's StealthTCP + craftls browser fingerprint parroting defeats Deep Packet Inspection
2. **Total Internet Blackout**: libp2p dies → GhostWire detects the drop → instantly fails over to:
   - Raw WiFi frames (Reticulum AutoInterface, no router needed)
   - Bluetooth Low Energy hops
   - LoRa radio (via Meshtastic or Reticulum serial interface)
3. **Mass Accessibility**: ESP32 nodes act as WiFi Access Points with captive portals — random people connect to "GhostWire-Free-Mesh" and get a web-based chat interface without downloading any app

### The "Cell Tower" Strategy
Deploy cheap ESP32 + LoRa + solar panel nodes on rooftops across a city. They act as autonomous relay towers, bouncing messages 10+ miles across the city without any internet infrastructure.

---

## 13. Mass Adoption / UX Design Principles

### Zero-Configuration
- Install app → Open → Start typing
- No "Enter IP address" or "Configure Gateway" screens
- Reticulum AutoInterface auto-discovers nearby nodes silently

### QR Codes for Everything
- Adding contacts: Display QR code → scan with phone → done
- No need to type cryptographic hashes

### Captive Portal (No App Required)
- ESP32 nodes broadcast WiFi AP
- Phone connects → captive portal pops up → lightweight web chat
- User is on the mesh without downloading anything

### "Signal" UX
- Green checkmarks for delivery
- Simple contact list
- No visible logs unless "Advanced Mode" toggled on

---

## 14. Reticulum Infrastructure Setup (Completed)

### Current Live Services on This Machine

| Service | Status | Details |
|---------|--------|---------|
| `rnsd.service` | ✅ Active (systemd) | Core Reticulum routing daemon |
| `nomadnet.service` | ✅ Active (systemd) | LXMF node hosting |
| Node Name | GHOSTWIRE | Visible globally on mesh |
| Peer Identity | PHANTOMOJO | Personal display name |
| Backbone | rmap.world:4242 | TCP interface to global mesh |

### Config Files
- Reticulum: `~/.reticulum/config` (RMAP TCP interface added)
- NomadNet: `~/.nomadnetwork/config` (node enabled, name set)
- Display Name: `~/.nomadnetwork/storage/displayname`

### Mobile Access
- **Sideband** app (Android/iOS/Desktop) for GUI access
- Recommended: Create separate identity on phone (e.g., "PHANTOMOJO Mobile")
- Do NOT clone laptop keypair to phone (causes routing conflicts)

---

## 15. Implementation Roadmap

### Phase 1: Reticulum Integration (2-4 hours)
1. Add `rns-rs` to `Cargo.toml`
2. Create `reticulum_transport.rs` implementing `Transport` trait
3. Add `TransportKind::Reticulum` variant to `TransportRegistry`
4. Write fallback logic (IP available → libp2p; IP dead → Reticulum)

### Phase 2: Universal Identity System (1-2 days)
1. Implement master keypair generation using Genesis constants
2. Implement HKDF child key derivation per transport
3. Implement display name system with discriminators
4. Implement reserved names registry with fuzzy matching

### Phase 3: Protocol Bridges (1-2 weeks)
1. Meshtastic serial API bridge
2. Briar headless API bridge
3. Gateway bot command parsing (`/send user@network message`)

### Phase 4: Mass UX (2-3 weeks)
1. QR code contact exchange
2. ESP32 captive portal web chat
3. Signal-style GUI (Sideband integration or custom)

---

## Sources & References

- GhostWire Genesis Ceremony: `NOTICE` file in project root
- GhostWire Transport Architecture: `TRANSPORT_IMPLEMENTATION_PLAN.md`
- GhostWire Threat Model: `THREAT_MODEL.md`
- Reticulum Protocol: https://reticulum.network
- rns-rs (Rust port): https://github.com/lelloman/rns-rs
- Sideband (Mobile client): Available on F-Droid / GitHub
- NomadNet: https://github.com/markqvist/NomadNet
