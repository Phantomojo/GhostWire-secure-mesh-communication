# Implementing Custom Transports

GhostWire is building a capability-governed multi-carrier system, not a generic transport registry that makes every network interchangeable. This guide is **design and contributor guidance**: a type, trait sketch, or enum variant does not mean an adapter is supported or selected by the current runtime. Read the authoritative [transport support status](../TRANSPORT_SUPPORT_STATUS.md), [carrier contract](../CARRIER_CONTRACT.md), and [inter-transport handoff contract](../INTER_TRANSPORT_HANDOFF.md) before implementing a carrier.

---

## The Transport Architecture

```
┌─────────────────────────────────────────────┐
│           GhostWire Core                    │
│  (Messaging, Routing, Identity)             │
├─────────────────────────────────────────────┤
│         Transport Registry                  │
│  ┌─────────┬─────────┬─────────┬─────────┐  │
│  │Stealth  │  WebRTC │   BLE   │  YOUR   │  │
│  │  TCP    │         │         │TRANSPORT│  │
│  └─────────┴─────────┴─────────┴─────────┘  │
├─────────────────────────────────────────────┤
│         Physical Medium                     │
│  (Ethernet, WiFi, Radio, Serial, etc.)      │
└─────────────────────────────────────────────┘
```

The current runtime does **not** yet automatically switch every concrete transport. The implemented `TransportCoordinator` is a deterministic policy foundation: a future adapter must declare its native carrier family, identity namespace, discovery model, fragmentation rules, local eligibility, and relay policy before it can be selected. Failure must return an explicit operation result or authorized encrypted store-and-forward decision; it must never trigger an implicit lower-privacy fallback.

---

## The Transport Trait

All transports in GhostWire implement a common interface. The key methods are:

```rust
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
```

### Transport Types

The following is a historical/illustrative carrier-type sketch. It is **not** a supported-adapter list and must not be used to infer runtime availability:

```rust
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
    // Add your custom type here
}
```

### Transport Metrics

Every transport reports standardized metrics:

```rust
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

Metrics can inform a future bounded health monitor, but they are insufficient authority to switch carriers. Selection must also satisfy the immutable delivery/privacy requirement, native carrier semantics, local device eligibility, relay opt-in, and carrier-specific fragmentation policy. A faster or healthier metadata-visible carrier cannot replace an unavailable privacy-preserving route.

---

## Example: Serial Port Transport

Here is a complete example of implementing a serial port transport for direct node-to-node communication over UART:

```rust
use anyhow::Result;
use async_trait::async_trait;
use libp2p::Multiaddr;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Serial port transport for direct UART communication
pub struct SerialTransport {
    port_path: String,
    baud_rate: u32,
    active_connections: Arc<RwLock<u32>>,
    bytes_sent: Arc<RwLock<u64>>,
    bytes_received: Arc<RwLock<u64>>,
    start_time: std::time::Instant,
}

impl SerialTransport {
    pub fn new(port_path: String, baud_rate: u32) -> Self {
        Self {
            port_path,
            baud_rate,
            active_connections: Arc::new(RwLock::new(0)),
            bytes_sent: Arc::new(RwLock::new(0)),
            bytes_received: Arc::new(RwLock::new(0)),
            start_time: std::time::Instant::now(),
        }
    }

    pub fn transport_type(&self) -> TransportType {
        TransportType::Serial  // Add this variant to the enum
    }

    pub fn is_available(&self) -> bool {
        // Check if the serial port exists and is accessible
        std::path::Path::new(&self.port_path).exists()
    }

    pub fn metrics(&self) -> TransportMetrics {
        TransportMetrics {
            bytes_sent: *self.bytes_sent.blocking_read(),
            bytes_received: *self.bytes_received.blocking_read(),
            active_connections: *self.active_connections.blocking_read(),
            total_connections: *self.bytes_sent.blocking_read(), // Simplified
            avg_latency_ms: 5.0, // Serial is typically low-latency
            packet_loss_rate: 0.01,
            uptime_seconds: self.start_time.elapsed().as_secs(),
            last_error: None,
        }
    }

    pub fn name(&self) -> &str {
        "serial"
    }

    pub fn supported_protocols(&self) -> &[&str] {
        &["/serial"]
    }

    pub async fn listen(&self, _address: Multiaddr) -> Result<Listener> {
        // Open the serial port for listening
        #[cfg(feature = "serialport")]
        {
            let port = serialport::new(&self.port_path, self.baud_rate)
                .timeout(std::time::Duration::from_millis(100))
                .open()?;

            *self.active_connections.write().await += 1;
            // Return a listener that reads from the serial port
            Ok(Listener::Serial(port))
        }

        #[cfg(not(feature = "serialport"))]
        anyhow::bail!("serialport feature not enabled")
    }

    pub async fn dial(&self, _address: Multiaddr) -> Result<Connection> {
        // Open the serial port for dialing
        #[cfg(feature = "serialport")]
        {
            let port = serialport::new(&self.port_path, self.baud_rate)
                .timeout(std::time::Duration::from_millis(100))
                .open()?;

            *self.active_connections.write().await += 1;
            // Return a connection that writes to the serial port
            Ok(Connection::Serial(port))
        }

        #[cfg(not(feature = "serialport"))]
        anyhow::bail!("serialport feature not enabled")
    }
}
```

---

## Registering a Custom Transport

### Via Configuration

Add your transport to the `ghostwire.toml` configuration:

```toml
[network]
# Enable your custom transport
custom_transports = ["serial"]

[transports.serial]
port_path = "/dev/ttyUSB0"
baud_rate = 115200
```

### Programmatically

Register the transport with the `TransportRegistry`:

```rust
use ghostwire::core::transport::TransportRegistry;
use ghostwire::core::transport::TransportKind;

// Create your transport
let serial = SerialTransport::new("/dev/ttyUSB0".to_string(), 115200);

// Wrap it in a TransportKind
let transport_kind = TransportKind::Serial(Arc::new(RwLock::new(serial)));

// Register with the core
let core = Core::new(config, bootstrap, None, None).await?;
core.transport_registry.write().await.register(transport_kind);
```

---

## Censorship Resistance: Pluggable Transports

GhostWire includes four censorship-resistant transport modes that wrap your traffic to resist deep packet inspection:

### Transport Modes

```rust
pub enum TransportMode {
    /// Raw TCP with StealthTCP HMAC handshake
    Stealth,
    /// Wrap in WebSocket upgrade headers (passes most firewalls)
    WebSocket,
    /// Shadowsocks-style full-stream AEAD encryption (looks like random bytes)
    ShadowsocksAead,
    /// TLS 1.3 with a convincing server certificate (looks like HTTPS)
    TlsCamouflage,
}
```

### Using Pluggable Transports

The `PluggableTransportConfig` controls how your traffic appears to network observers:

```rust
use ghostwire::core::pluggable_transport::{
    PluggableTransportConfig, TransportMode, TransportPsk,
};

// Default: WebSocket camouflage (passes most corporate firewalls)
let config = PluggableTransportConfig::default();

// High censorship: Shadowsocks AEAD (traffic looks like random bytes)
let config = PluggableTransportConfig::high_censorship()?;

// Low censorship: WebSocket (lower overhead)
let config = PluggableTransportConfig::low_censorship();
```

### Shadowsocks AEAD Mode

In Shadowsocks mode, the entire byte stream is encrypted with XChaCha20-Poly1305:

```rust
use ghostwire::core::pluggable_transport::{
    ss_encrypt_chunk, ss_decrypt_chunk, TransportPsk,
};
use ring::rand::{SecureRandom, SystemRandom};

// Generate a pre-shared key (share out-of-band with your peer)
let psk = TransportPsk::generate()?;
let psk_hex = hex::encode(psk.as_bytes());
// Share psk_hex with your peer securely

// Encrypt a chunk before sending
let rng = SystemRandom::new();
let plaintext = b"GhostWire protocol data";
let encrypted = ss_encrypt_chunk(plaintext, &psk, &rng)?;
// Send `encrypted` over the wire

// Decrypt a chunk on receive
let (decrypted, bytes_consumed) = ss_decrypt_chunk(&received_data, &psk)?;
// `decrypted` is the original plaintext
```

Format: `[2-byte chunk_len LE][12-byte nonce][ciphertext+tag]`

Each chunk is independently encrypted with a fresh nonce, so there is no stream cipher state to track.

### WebSocket Camouflage Mode

WebSocket mode wraps GhostWire traffic in standard WebSocket frames that appear as normal web browsing:

```rust
use ghostwire::core::pluggable_transport::{
    ws_client_upgrade, ws_server_upgrade, ws_frame_encode, ws_frame_decode,
};

// Client side: perform WebSocket handshake
ws_client_upgrade(&mut tcp_stream).await?;

// Server side: accept WebSocket handshake
ws_server_upgrade(&mut tcp_stream).await?;

// Send: encode as WebSocket binary frame
let frame = ws_frame_encode(&data, true); // true = client (masked)
tcp_stream.write_all(&frame).await?;

// Receive: decode WebSocket frame
let (payload, consumed) = ws_frame_decode(&buffer)?;
```

The handshake uses realistic HTTP headers:
```
GET /ws/v2/live HTTP/1.1
Host: cdn-notifications.example.com
Upgrade: websocket
Connection: Upgrade
Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==
Sec-WebSocket-Version: 13
User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36
```

To a DPI system, this is indistinguishable from a legitimate WebSocket connection.

### Traffic Armor

Beyond transport-level obfuscation, GhostWire provides Traffic Armor with three additional protections:

| Protection | Mechanism | Effect |
|-----------|-----------|--------|
| **Payload Morphing** | XOR with PRNG stream | Payload appears as random data |
| **Timing Chaff** | 0-100ms random delay per packet | Disrupts timing correlation |
| **Erasure Coding** | Reed-Solomon encoding | Resists packet loss/censorship |

---

## Carrier Selection and Handoff

**Automatic cross-carrier delivery is not yet enabled.** The current policy foundation intentionally refuses to invent performance-only fallback behavior. A future carrier adapter must report its actual native constraints and a future handoff pipeline must select only a compliant next leg.

| Required decision input | Why a generic health score is unsafe |
|---|---|
| Immutable delivery/privacy requirement | Low latency cannot compensate for relationship-metadata or anonymity loss. |
| Native identity and discovery model | A libp2p peer ID, RNS destination, BLE address, Wi-Fi Direct group, and radio interface are not interchangeable identities. |
| Local interface eligibility | User enablement, device capability, OS permission, background policy, and energy/radio budget can forbid a carrier even if it is reachable. |
| Relay opt-in | A GhostWire node can be an endpoint without agreeing to forward other nodes’ traffic. |
| Carrier fragmentation/resource contract | RNS packet mode, BLE, low-rate radio, streams, and private block exchange have distinct safe object limits. |
| Per-leg receipt semantics | A local write or carrier receipt is not recipient delivery. |

A carrier-changing request must carry one protected GhostWire envelope ID across legs, validate the source through the common ingress pipeline, and require a destination carrier that meets all the above conditions. No adapter may decrypt user plaintext to convert carriers. See [Inter-Transport Handoff](../INTER_TRANSPORT_HANDOFF.md).

Hysteresis, route scoring, energy scheduling, and performance adaptation can be added only as **bounded secondary policy** after this fail-closed eligibility check. They must never override a privacy requirement or make a disabled/mobile interface relay unexpectedly.

---

## Testing Your Transport

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_serial_transport_available() {
        // Create a virtual serial port for testing
        let transport = SerialTransport::new("/dev/pty0".to_string(), 115200);
        assert!(transport.is_available());
    }

    #[tokio::test]
    async fn test_serial_transport_metrics() {
        let transport = SerialTransport::new("/dev/pty0".to_string(), 115200);
        let metrics = transport.metrics();
        assert_eq!(metrics.active_connections, 0);
        assert_eq!(metrics.bytes_sent, 0);
        assert_eq!(metrics.name, "serial");
    }

    #[tokio::test]
    async fn test_serial_transport_roundtrip() {
        // Set up a pseudo-terminal pair
        let (master, slave) = pty::openpty()?;

        let tx = SerialTransport::new(master.path().to_string_lossy().to_string(), 115200);
        let rx = SerialTransport::new(slave.path().to_string_lossy().to_string(), 115200);

        // Send data through master, receive on slave
        let data = b"test message";
        // ... implement send/recv test
    }
}
```

### Integration Tests

Test your transport with a real GhostWire node:

```rust
#[tokio::test]
async fn test_transport_with_ghostwire_core() {
    let mut config = Config::default();
    config.network.mesh_enabled = true;

    let core = Core::new(config, vec![], None, None).await.unwrap();

    // Register your transport
    let serial = SerialTransport::new("/dev/pty0".to_string(), 115200);
    let kind = TransportKind::Serial(Arc::new(RwLock::new(serial)));
    core.transport_registry.write().await.register(kind);

    // Verify transport is registered
    let registry = core.transport_registry.read().await;
    assert!(registry.all_transports().iter().any(|t| t.name() == "serial"));
}
```

---

## Reference Implementation

See the following modules in the GhostWire codebase:

| Module | Path |
|--------|------|
| Pluggable transport (WebSocket, Shadowsocks, TLS) | `ghostwire/src/core/pluggable_transport.rs` |
| obfs4 transport | `ghostwire/src/core/obfs4_transport.rs` |
| Stealth TCP | `ghostwire/src/core/stealth_tcp.rs` |
| Traffic Armor | `ghostwire/src/core/traffic_armor.rs` |
| Transport registry | `ghostwire/src/core/transport/` |
| Meshtastic (serial) adapter | `ghostwire/src/core/mod.rs` (MeshtasticAdapter) |

---

## Next Steps

- **[Protocol Bridges](./PROTOCOL_BRIDGES.md)** -- Connect GhostWire to other networks
- **[API Reference](./API_REFERENCE.md)** -- REST API for monitoring transport status
- **[Pluggable Transports Spec](../spec/TRANSPORT.md)** -- Full protocol specification
