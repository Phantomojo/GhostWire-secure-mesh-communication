# Building Protocol Bridges

This file contains bridge design notes and historical examples. **It is not evidence that a named bridge is interoperable, privacy-reviewed, or safe to deploy.** Consult the authoritative [transport and bridge support status](../TRANSPORT_SUPPORT_STATUS.md), [carrier contract](../CARRIER_CONTRACT.md), and [inter-transport handoff contract](../INTER_TRANSPORT_HANDOFF.md) before implementing or enabling a bridge.

> A bridge must carry an already protected GhostWire envelope across an explicit security boundary. It must not decrypt user plaintext merely to translate networks, merge native identities, claim a carrier receipt is end-to-end delivery, or silently fall back to a lower-privacy path.

---

## Available Bridges

| Bridge | Status | Feature Flag | Description |
|--------|--------|-------------|-------------|
| **Matrix** | Planned / unaudited | `matrix-bridge` | Module/feature surface exists; no supported interoperable or privacy-reviewed bridge claim |
| **Meshtastic** | Planned / unaudited | `meshtastic` | Module/feature surface exists; no supported radio bridge, erasure-coding, or delivery claim |
| **Briar** | Planned / unavailable | `briar` | No supported Briar bridge |
| **Reticulum / RNS** | Experimental bounded adapter | `reticulum` | External official-RNS bridge proves isolated bidirectional opaque packet delivery at a 383-byte ceiling; not general Reticulum or cross-carrier support |

---

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                  GhostWire Core                     │
│  ┌───────────────────────────────────────────────┐  │
│  │            Adapter Registry                   │  │
│  │  ┌──────────┬──────────┬───────┬──────────┐   │  │
│  │  │  Matrix  │Meshtastic│ Briar │Reticulum │   │  │
│  │  │ Adapter  │ Adapter  │Adapter │ Adapter  │   │  │
│  │  └──────────┴──────────┴───────┴──────────┘   │  │
│  └───────────────────────────────────────────────┘  │
├─────────────────────────────────────────────────────┤
│         External Networks                           │
│  Matrix HS    Meshtastic Nodes    Briar Peers       │
└─────────────────────────────────────────────────────┘
```

The diagram is a target architecture, not proof that the adapter registry currently routes all listed bridges. Any future adapter must first publish a factual carrier declaration, pass the shared ingress checks, and use the protected-envelope handoff lifecycle. A `send_message()` method or feature flag alone is not a delivery guarantee.

---

## Matrix Bridge

**Status: planned / unaudited. Do not deploy using the following historical example.** A future Matrix bridge requires an explicit external-service, identity-linkage, metadata, retention, consent, and encrypted-envelope review. The example below is retained as development reference only and must not be treated as a supported configuration.

### Configuration

Enable the bridge in `ghostwire.toml`:

```toml
[features]
matrix_bridge = true

[matrix]
homeserver = "https://matrix.org"
user = "@ghostwire:matrix.org"
# access_token is set via environment variable
```

Set the access token:

```bash
export GHOSTWIRE_MATRIX_PASSWORD="your-matrix-password"
export MATRIX_ACCESS_TOKEN="your-matrix-access-token"
```

### Building

```bash
cargo build --features matrix-bridge
```

### Usage

The Matrix adapter sends and receives messages through Matrix rooms:

```rust
use ghostwire::core::{AdapterKind, MatrixAdapter};

// Create the Matrix adapter
let adapter = AdapterKind::Matrix(MatrixAdapter {
    homeserver: "https://matrix.org".to_string(),
    user: "@ghostwire:matrix.org".to_string(),
    access_token: std::env::var("MATRIX_ACCESS_TOKEN").unwrap(),
    client: None, // Will be initialized
});

// Initialize the real Matrix client
let mut adapter = match adapter {
    AdapterKind::Matrix(mut a) => {
        a.init_client().await?;
        AdapterKind::Matrix(a)
    }
    other => other,
};

// Send a GhostWire message to a Matrix room
let message = ghostwire::core::message::Message::new(
    "sender-id".to_string(),
    "!roomid:matrix.org".to_string(), // Matrix room ID as recipient
    "Hello from GhostWire!".to_string(),
);
adapter.send_message(&message).await?;

// Receive messages from Matrix
if let Some(msg) = adapter.receive_message().await? {
    println!("From Matrix: {}", msg.content);
}
```

### How It Works

1. GhostWire connects to the Matrix homeserver using the provided credentials.
2. Messages sent to Matrix room IDs are forwarded as Matrix room messages.
3. Incoming Matrix messages are converted to GhostWire `Message` objects.
4. End-to-end encryption is maintained: GhostWire encrypts the payload before it reaches Matrix, so the Matrix server cannot read message contents.

---

## Meshtastic Bridge

The Meshtastic bridge connects GhostWire to LoRa radio devices via serial connection.

### Configuration

```toml
[features]
meshtastic_bridge = true

[meshtastic]
device_path = "/dev/ttyUSB0"
channel = "main"
```

Environment variables:

```bash
export GHOSTWIRE_MESHTASTIC_BRIDGE=1
export MESHTASTIC_DEVICE_PATH="/dev/ttyUSB0"
export MESHTASTIC_CHANNEL="main"
```

### Building

```bash
cargo build --features meshtastic
```

### Usage with Erasure Coding

The Meshtastic adapter automatically applies erasure coding to messages, splitting them into shards that can survive packet loss:

```rust
use ghostwire::core::{AdapterKind, MeshtasticAdapter};
use ghostwire::core::armor::ErasureArmor;
use std::sync::Arc;

// Create the Meshtastic adapter with erasure armor
// 3 data shards, 2 parity shards (can recover from any 2 lost shards)
let adapter = MeshtasticAdapter::new(
    "/dev/ttyUSB0".to_string(),
    "main".to_string(),
);

// Send a message (automatically split into shards)
let message = ghostwire::core::message::Message::new(
    "sender-id".to_string(),
    "broadcast".to_string(),
    "Emergency alert: flood warning".to_string(),
);
adapter.send_message(&message).await?;

// The adapter internally:
// 1. Applies ErasureArmor::encode() to split the message
// 2. Sends each shard over the serial port
// 3. The receiving end can reconstruct from any 3 of 5 shards
```

### Erasure Coding Details

```
Original message: "Emergency alert: flood warning"
         |
         v
ErasureArmor::new(3, 2)  // 3 data + 2 parity
         |
         v
[Shard 0] [Shard 1] [Shard 2] [Parity 0] [Parity 1]
         |
         v
  Sent over LoRa (each shard fits in 242-byte payload)
         |
         v
  Receiver needs any 3 of 5 shards to reconstruct
```

This is critical for LoRa because:
- LoRa has a maximum payload of 242 bytes per packet
- Radio links are unreliable (interference, range limits)
- Erasure coding ensures message delivery even when some shards are lost

---

## Building a Custom Bridge

### The BridgeAdapter Trait

All bridges implement the `AdapterKind` pattern. To add a new bridge:

1. **Define the adapter struct:**

```rust
#[derive(Clone)]
pub struct MyBridgeAdapter {
    pub endpoint: String,
    pub api_key: String,
    // ... your bridge-specific configuration
}
```

2. **Implement the adapter methods:**

```rust
impl MyBridgeAdapter {
    pub fn name(&self) -> &'static str {
        "my-bridge"
    }

    pub fn description(&self) -> &'static str {
        "My custom protocol bridge"
    }

    pub async fn send_message(
        &self,
        message: &ghostwire::core::message::Message,
    ) -> anyhow::Result<()> {
        // Convert GhostWire message to your bridge's format
        // Send it to the external network
        tracing::info!(
            "MyBridge: Sending to {}: {}",
            message.recipient,
            message.content
        );
        Ok(())
    }

    pub async fn receive_message(
        &self,
    ) -> anyhow::Result<Option<ghostwire::core::message::Message>> {
        // Poll the external network for messages
        // Convert them to GhostWire Message format
        Ok(None) // Return Some(message) when you have one
    }
}
```

3. **Register in the AdapterKind enum:**

```rust
pub enum AdapterKind {
    Matrix(MatrixAdapter),
    Meshtastic(MeshtasticAdapter),
    MyBridge(MyBridgeAdapter),  // Add your adapter
}

impl AdapterKind {
    pub fn name(&self) -> &'static str {
        match self {
            AdapterKind::Matrix(a) => a.name(),
            AdapterKind::Meshtastic(a) => a.name(),
            AdapterKind::MyBridge(a) => a.name(),
        }
    }

    pub async fn send_message(&self, message: &Message) -> anyhow::Result<()> {
        match self {
            AdapterKind::Matrix(a) => a.send_message(message).await,
            AdapterKind::Meshtastic(a) => a.send_message(message).await,
            AdapterKind::MyBridge(a) => a.send_message(message).await,
        }
    }

    pub async fn receive_message(&self) -> anyhow::Result<Option<Message>> {
        match self {
            AdapterKind::Matrix(a) => a.receive_message().await,
            AdapterKind::Meshtastic(a) => a.receive_message().await,
            AdapterKind::MyBridge(a) => a.receive_message().await,
        }
    }
}
```

4. **Register at startup:**

```rust
let mut adapter_registry = AdapterRegistry::new();

adapter_registry.register(AdapterKind::MyBridge(MyBridgeAdapter {
    endpoint: "https://my-bridge.example.com".to_string(),
    api_key: std::env::var("MY_BRIDGE_API_KEY").unwrap(),
}));
```

---

## Security Considerations

### Opt-In Trust

Bridges are **opt-in**. A GhostWire node does not automatically forward traffic to external networks. Bridges must be explicitly enabled in configuration:

```toml
[features]
matrix_bridge = true    # Must be explicitly enabled
meshtastic_bridge = true
```

### Signed Relay Messages

When a message crosses a bridge boundary, it should be signed to prove its origin:

```rust
// Sign the message before sending through a bridge
let message_bytes = serde_json::to_vec(&message)?;
let signature = identity.sign(&message_bytes);

// Include the signature in the bridged message
let bridged_message = BridgedMessage {
    original: message,
    signature,
    bridge_name: "matrix".to_string(),
    timestamp: SystemTime::now(),
};
```

The receiving node can verify the signature to ensure the message was not tampered with by the bridge.

### Whitelist-Based Bridge Trust

Configure which bridges are trusted for which types of traffic:

```toml
[bridge_trust]
# Only allow Matrix for text messages (no file transfers)
matrix.allowed_message_types = ["text"]
matrix.max_message_size = 4096

# Meshtastic allows all types (it's a trusted local device)
meshtastic.allowed_message_types = ["text", "file", "control"]
meshtastic.max_message_size = 242  # LoRa MTU

# Briar is not trusted for any traffic
briar.enabled = false
```

### Encryption Across Bridges

GhostWire messages are encrypted end-to-end before they reach any bridge. The bridge operator (Matrix server, Meshtastic device, etc.) cannot read message contents:

```
GhostWire Node A
  |
  | [XChaCha20-Poly1305 encrypted payload]
  | [Sphinx onion-routed header]
  v
Matrix Server (cannot decrypt)
  |
  | [Still encrypted]
  v
GhostWire Node B (decrypts with recipient's key)
```

The bridge only sees encrypted blobs. Decryption happens at the destination GhostWire node.

---

## Bridge API Endpoints

When running in daemon mode, bridges can be managed via REST API:

```bash
# List available bridges
curl -H "X-Ghost-Token: $TOKEN" http://localhost:8080/api/v2/network/status

# Check Matrix bridge status
curl -H "X-Ghost-Token: $TOKEN" http://localhost:8080/api/v2/network/status | jq '.data.transport_mode'
```

---

## Next Steps

- **[Custom Transports](./CUSTOM_TRANSPORT.md)** -- Add new transport layers
- **[API Reference](./API_REFERENCE.md)** -- REST API for bridge management
- **[Pluggable Transports Spec](../spec/TRANSPORT.md)** -- Full transport specification
