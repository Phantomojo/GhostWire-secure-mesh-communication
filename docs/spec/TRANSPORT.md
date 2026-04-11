# Pluggable Transports Specification

**GIP:** Pending
**Version:** 0.2.0-draft
**Date:** April 12, 2026

---

## Overview

This specification defines the pluggable transport layer of GhostWire, enabling communication over diverse network technologies with built-in censorship resistance. Transports are abstracted behind a common interface, allowing seamless switching between TCP, WebRTC, BLE, LoRa, Reticulum, and censorship-resistant transports (WebSocket Camouflage, Shadowsocks AEAD, TLS Camouflage, obfs4). The transport layer provides Traffic Armor for resistance against deep packet inspection and timing analysis.

---

## Transport Trait

All transports implement the common Transport trait:

```rust
pub trait Transport: Send + Sync {
    /// Unique transport type identifier
    fn transport_type(&self) -> TransportType;

    /// Start listening on the given address
    fn listen(&self, address: Multiaddr) -> impl Future<Output = Result<Listener>>;

    /// Dial a remote peer
    fn dial(&self, address: Multiaddr) -> impl Future<Output = Result<Connection>>;

    /// Whether this transport is currently available
    fn is_available(&self) -> bool;

    /// Transport performance metrics
    fn metrics(&self) -> TransportMetrics;

    /// Human-readable transport name
    fn name(&self) -> &str;

    /// Supported address protocols
    fn supported_protocols(&self) -> &[&str];
}

pub enum TransportType {
    StealthTcp,
    WebRtc,
    Ble,
    Lora,
    Reticulum,
    WebSocketCamouflage,
    ShadowsocksAead,
    TlsCamouflage,
    Obfs4,
}

pub struct TransportMetrics {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub active_connections: u32,
    pub total_connections: u64,
    pub avg_latency_ms: f64,
    pub packet_loss_rate: f64,
    pub uptime_seconds: u64,
    pub last_error: Option<String>,
}
```

---

## Transport Types

### StealthTCP

Standard TCP with obfuscation to avoid simple port-based blocking.

| Property | Value |
|----------|-------|
| **Protocol** | TCP |
| **Default port** | 9735 (configurable) |
| **Obfuscation** | Random port selection; looks like generic TCP |
| **Use case** | General mesh networking, unrestricted networks |

**Stealth mode:** When enabled, StealthTCP listens on a random high port (10000-65535) and advertises the port via mDNS/DHT. This avoids simple port-based blocking.

### WebRTC

Peer-to-peer communication via WebRTC data channels, enabling NAT traversal.

| Property | Value |
|----------|-------|
| **Protocol** | WebRTC Data Channels (SCTP over DTLS) |
| **NAT traversal** | STUN/TURN + ICE |
| **Encryption** | DTLS 1.2 (built into WebRTC) |
| **Use case** | Cross-NAT communication, browser compatibility |

**Signaling:** WebRTC signaling is performed via the GhostWire DHT (offer/answer exchange) or direct mDNS exchange on local networks.

**TURN fallback:** If direct P2P connection fails, a user-configured TURN relay is used. TURN traffic is encrypted (DTLS), so the relay cannot read message contents.

### BLE (Bluetooth Low Energy)

Short-range communication for opportunistic mesh networking.

| Property | Value |
|----------|-------|
| **Protocol** | Bluetooth 4.0+ (BLE) |
| **Range** | ~10-100 meters (environment dependent) |
| **MTU** | 512 bytes (negotiable up to 4096) |
| **Power consumption** | Low (advertising interval: 1 second) |
| **Use case** | Proximity networking, disaster zones, offline areas |

**See DTN spec** for BLE sync handshake details.

### LoRa

Long-range, low-bandwidth communication for rural and remote areas.

| Property | Value |
|----------|-------|
| **Protocol** | LoRa (Chirp Spread Spectrum) |
| **Frequency** | Region-dependent (868 MHz EU, 915 MHz US, 433 MHz Asia) |
| **Range** | 2-15 km (line of sight) |
| **Bandwidth** | 0.3-50 kbps |
| **Max payload** | 242 bytes per packet |
| **Use case** | Rural mesh networks, emergency communication, IoT |

**Fragmentation:** Messages exceeding LoRa's payload limit are fragmented at the DTN layer (see DTN spec).

**Duty cycle:** LoRa transmissions comply with regional duty cycle regulations (e.g., 1% in EU 868 MHz band).

### Reticulum

Reticulum Network Stack for resilient communication over any interface.

| Property | Value |
|----------|-------|
| **Protocol** | Reticulum (autonomous network stack) |
| **Interfaces** | Any (TCP, UDP, BLE, LoRa, serial, I2P, etc.) |
| **Routing** | Autonomous pathfinding |
| **Encryption** | Built-in (Curve25519 + AES) |
| **Use case** | Multi-interface mesh networks, fallback connectivity |

Reticulum acts as a meta-transport, automatically selecting the best underlying interface for communication.

---

## Censorship Resistance

GhostWire provides four pluggable censorship-resistant transports.

### WebSocket Camouflage

WebSocket traffic disguised as standard web browsing.

| Property | Value |
|----------|-------|
| **Protocol** | WebSocket (RFC 6455) over TLS |
| **Port** | 443 (standard HTTPS) |
| **Appearance** | Indistinguishable from normal WebSocket traffic |
| **Use case** | Networks that allow HTTPS but block P2P protocols |

**Implementation:**
- WebSocket connection to a user-operated or community-operated bridge.
- Traffic is wrapped in standard WebSocket frames.
- The bridge forwards traffic to the GhostWire network.
- DPI (Deep Packet Inspection) sees only standard WebSocket traffic.

### Shadowsocks AEAD

Encrypted proxy protocol with AEAD ciphers.

| Property | Value |
|----------|-------|
| **Protocol** | Shadowsocks AEAD (SIP004) |
| **Cipher** | ChaCha20-IETF-Poly1305 |
| **Port** | Configurable (default: 8388) |
| **Use case** | Censored networks that perform active probing |

**Implementation:**
- Shadowsocks server runs on an uncensored node (bridge).
- GhostWire client connects to the Shadowsocks server as a proxy.
- All traffic is encrypted with the Shadowsocks AEAD cipher.
- Active probing is resisted by the AEAD authentication step.

### TLS Camouflage

TLS traffic that appears as standard HTTPS to passive observers.

| Property | Value |
|----------|-------|
| **Protocol** | TLS 1.3 with custom ALPN |
| **Port** | 443 |
| **Certificate** | Self-signed or Let's Encrypt |
| **Use case** | Networks that allow HTTPS but inspect SNI/ALPN |

**Implementation:**
- TLS handshake with a certificate matching the expected domain.
- Custom ALPN extension for GhostWire protocol negotiation.
- After TLS handshake, GhostWire protocol runs inside the encrypted tunnel.
- To a passive observer, traffic appears as standard HTTPS.

### obfs4

Obfuscation protocol that makes traffic indistinguishable from random noise.

| Property | Value |
|----------|-------|
| **Protocol** | obfs4 (Tor Project) |
| **Port** | Configurable (default: 443) |
| **Use case** | Highly censored networks with active DPI and probing |

**Implementation:**
- obfs4 uses the Elligator2 mapping to make public keys indistinguishable from random.
- The handshake produces traffic that appears as random bytes.
- After handshake, traffic is encrypted with a shared secret.
- Active probing is resisted: obfs4 servers respond with random data to unauthenticated probes.

**Bridge distribution:** obfs4 bridges are distributed via:
- BridgeDB (email-based distribution).
- DHT (encrypted bridge addresses).
- Manual distribution (trusted contacts).

---

## Traffic Armor

Traffic Armor provides additional resistance against traffic analysis and deep packet inspection.

### Payload Morphing

Payloads are morphed to resist signature-based detection:

```
morphed_payload = xor(original_payload, prng_stream(seed))
```

Where `prng_stream` is a deterministic pseudo-random stream derived from a shared seed. The morphing is reversible at the receiving end.

**Effect:** The payload appears as random data to passive observers, even before encryption is applied.

### Timing Chaff

Random delays are added to packet transmission to resist timing correlation attacks.

| Property | Value |
|----------|-------|
| **Delay distribution** | Uniform random |
| **Min delay** | 0 ms |
| **Max delay** | 100 ms (configurable) |
| **Application** | Per-packet, independent |

```
actual_send_time = scheduled_send_time + uniform_random(0, 100ms)
```

**Effect:** Timing correlation between incoming and outgoing packets is disrupted, making it harder for an observer to link packets across a node.

### Erasure Coding

Messages are encoded with erasure coding to resist packet loss and censorship.

| Property | Value |
|----------|-------|
| **Algorithm** | Reed-Solomon |
| **Data shards** | Original message split into N shards |
| **Parity shards** | M additional parity shards |
| **Recovery** | Any N shards can reconstruct the original message |

**Example:** A message is split into 4 data shards with 2 parity shards (6 total). Any 4 of the 6 shards can reconstruct the message. This provides resilience against up to 2 lost or censored shards.

**Use case:** Erasure coding is enabled for Critical and Expedited priority bundles (see DTN spec). It is optional for Normal and Bulk priority.

---

## Auto-Transport Switching

The transport registry automatically selects the best available transport based on network conditions.

### Disaster Levels

Network conditions are classified into four levels:

| Level | Description | Indicators |
|-------|-------------|------------|
| **Normal** | All transports functional | Low latency, low packet loss, high bandwidth |
| **Congested** | Some transports degraded | Moderate latency, moderate packet loss |
| **Isolation** | Most transports unavailable | High latency, high packet loss, few peers reachable |
| **Blackout** | No standard transports available | No peers reachable via TCP/WebRTC |

### Transport Selection by Disaster Level

| Disaster Level | Preferred Transports |
|----------------|---------------------|
| Normal | StealthTCP, WebRTC |
| Congested | WebRTC, WebSocket Camouflage |
| Isolation | WebSocket Camouflage, Shadowsocks AEAD, obfs4 |
| Blackout | BLE, LoRa, Reticulum (any available interface) |

### Hysteresis (Anti-Thrashing)

To prevent rapid oscillation between transports:

| Property | Value |
|----------|-------|
| **Minimum transport hold time** | 60 seconds |
| **Switch threshold** | Current transport fails 3 consecutive times |
| **Re-evaluation interval** | 5 minutes |
| **Hysteresis margin** | New transport must be 20% better than current to switch |

**Switch decision:**
```
fn should_switch_transport(current_metrics, candidate_metrics) -> bool {
    if current_metrics.is_healthy() {
        return false;
    }
    if candidate_metrics.score > current_metrics.score * 1.2 {
        return true;
    }
    false
}
```

---

## Transport Registry

The transport registry manages all available transports and their state.

```rust
pub struct TransportRegistry {
    pub transports: HashMap<TransportType, Box<dyn Transport>>,
    pub active_transport: TransportType,
    pub fallback_order: Vec<TransportType>,
    pub disaster_level: DisasterLevel,
    pub last_switch_time: u64,
    pub switch_count: u64,
}

impl TransportRegistry {
    /// Register a new transport
    pub fn register(&mut self, transport: Box<dyn Transport>);

    /// Select the best available transport based on current conditions
    pub fn select_transport(&mut self) -> TransportType;

    /// Update disaster level based on network metrics
    pub fn update_disaster_level(&mut self);

    /// Get the current active transport
    pub fn active(&self) -> &dyn Transport;

    /// Get all available transports
    pub fn all_transports(&self) -> Vec<&dyn Transport>;
}
```

### Fallback Chain

If the active transport fails, the registry falls back through the configured order:

```
Default fallback order:
1. StealthTCP
2. WebRTC
3. WebSocket Camouflage
4. Shadowsocks AEAD
5. TLS Camouflage
6. obfs4
7. BLE
8. LoRa
9. Reticulum
```

**User configuration:** Users can customize the fallback order based on their threat model and network environment.

### Transport Health Check

Each transport is periodically health-checked:

```
health_check(transport):
    metrics = transport.metrics()
    if metrics.packet_loss_rate > 0.5:
        return Unhealthy
    if metrics.avg_latency_ms > 5000:
        return Degraded
    if metrics.active_connections == 0:
        return NoPeers
    return Healthy
```

**Check interval:** Every 30 seconds for the active transport; every 5 minutes for inactive transports.

---

## Security Properties

| Property | Mechanism |
|----------|-----------|
| **Transport encryption** | All transports encrypt traffic (DTLS for WebRTC, XChaCha20-Poly1305 for others) |
| **Censorship resistance** | 4 pluggable censorship-resistant transports |
| **Traffic analysis resistance** | Timing chaff (0-100ms), payload morphing |
| **Packet loss resilience** | Erasure coding (Reed-Solomon) |
| **Active probing resistance** | obfs4 (random response to unauthenticated probes), Shadowsocks AEAD (authentication required) |
| **DPI resistance** | WebSocket Camouflage (looks like standard WS), TLS Camouflage (looks like HTTPS) |
| **Anti-thrashing** | Hysteresis mechanism prevents rapid transport oscillation |

### Threat Model

| Adversary | Capability | Protection |
|-----------|-----------|------------|
| **Port blocking** | Blocks known P2P ports | Random port selection (StealthTCP); port 443 transports (WebSocket, TLS) |
| **DPI** | Inspects packet contents | All transports encrypt; payload morphing makes traffic appear random |
| **Active probing** | Probes servers to identify protocol | obfs4 responds with random data; Shadowsocks requires authentication |
| **Timing correlation** | Correlates packet timing | Timing chaff (0-100ms random delay) |
| **Transport-specific blocking** | Blocks specific protocols (e.g., WebRTC) | Auto-transport switching to alternative transports |
| **Bridge compromise** | Operates malicious bridge | End-to-end encryption (XChaCha20-Poly1305); bridge cannot read message contents |

---

## Reference Implementation

See the following modules in the GhostWire reference implementation:

| Module | Path |
|--------|------|
| Transport trait | `src/core/transport/traits.rs` |
| StealthTCP | `src/core/transport/stealth_tcp.rs` |
| WebRTC | `src/core/transport/webrtc.rs` |
| BLE | `src/core/transport/ble.rs` |
| LoRa | `src/core/transport/lora.rs` |
| Reticulum | `src/core/transport/reticulum.rs` |
| WebSocket Camouflage | `src/core/transport/websocket_camo.rs` |
| Shadowsocks AEAD | `src/core/transport/shadowsocks.rs` |
| TLS Camouflage | `src/core/transport/tls_camo.rs` |
| obfs4 | `src/core/transport/obfs4.rs` |
| Traffic Armor | `src/core/transport/traffic_armor.rs` |
| Transport registry | `src/core/transport/registry.rs` |
| Auto-transport switching | `src/core/transport/auto_switch.rs` |
