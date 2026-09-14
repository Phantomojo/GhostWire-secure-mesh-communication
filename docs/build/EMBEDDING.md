# Embedding GhostWire in Your Application

This guide shows how to integrate the GhostWire protocol stack into your own application -- whether that's a desktop app, mobile app, server, or IoT device.

---

## Architecture

```
┌──────────────────────────────────────────────────┐
│                   Your Application               │
│  (UI, business logic, domain-specific features)  │
├──────────────────────────────────────────────────┤
│              GhostWire Core (Core)               │
│  ┌──────────┬──────────┬──────────┬──────────┐   │
│  │ Identity │  Mesh    │  DTN     │  AI      │   │
│  │ Manager  │ Manager  │  Store   │ Routing  │   │
│  └──────────┴──────────┴──────────┴──────────┘   │
│  ┌──────────┬──────────┬──────────┬──────────┐   │
│  │ Security │ Transport│  Sphinx  │  Social  │   │
│  │ Manager  │ Registry │  Router  │  Graph   │   │
│  └──────────┴──────────┴──────────┴──────────┘   │
├──────────────────────────────────────────────────┤
│              libp2p Swarm Layer                  │
│  (GossipSub, Kademlia, mDNS, Noise, Yamux)       │
├──────────────────────────────────────────────────┤
│              Transport Layer                     │
│  (TCP, WebRTC, BLE, LoRa, Reticulum, obfs4)      │
└──────────────────────────────────────────────────┘
```

The `Core` struct is the single entry point. It owns all subsystems and coordinates them through an event bus.

---

## Basic Embedding

### Minimal Setup

```rust
use ghostwire::config::Config;
use ghostwire::core::{Core, CoreEvent};
use std::sync::Arc;
use anyhow::Result;

pub struct MyApp {
    core: Arc<Core>,
    event_handle: Option<tokio::task::JoinHandle<()>>,
}

impl MyApp {
    pub async fn new() -> Result<Self> {
        let config = Config::default();
        let bootstrap = vec![]; // Use default public gateways

        let core = Core::new(config, bootstrap, None, None).await?;
        let core = Arc::new(core);

        // Start mesh networking
        core.clone()
            .start_nervous_system("0.0.0.0".to_string(), 0)
            .await;

        Ok(Self {
            core,
            event_handle: None,
        })
    }

    /// Start listening for GhostWire events
    pub fn start_event_listener(&mut self) {
        let mut rx = self.core.event_bus.subscribe();
        let core = self.core.clone();

        self.event_handle = Some(tokio::spawn(async move {
            while let Ok(event) = rx.recv().await {
                Self::handle_event(&core, event).await;
            }
        }));
    }

    async fn handle_event(core: &Core, event: CoreEvent) {
        match event {
            CoreEvent::NewMessage(msg) => {
                // Your application logic here
                println!("Received: {}", msg.content);
            }
            CoreEvent::PeerConnected(peer_id) => {
                println!("Peer online: {}", peer_id);
            }
            CoreEvent::PeerDisconnected(peer_id) => {
                println!("Peer offline: {}", peer_id);
            }
            CoreEvent::NetworkUpdate { active_peers, latency_ms } => {
                println!("Mesh: {} peers, {}ms", active_peers, latency_ms);
            }
            CoreEvent::SecurityLockdown(enabled) => {
                if enabled {
                    eprintln!("SECURITY LOCKDOWN ACTIVE");
                }
            }
            _ => {}
        }
    }

    /// Send a message to a specific peer
    pub async fn send_message(&self, recipient: &str, content: &str) -> Result<()> {
        let sender_id = self.core.get_identity_id().await;
        let msg = ghostwire::core::message::Message::new(
            sender_id,
            recipient.to_string(),
            content.to_string(),
        );
        self.core.send_direct_message(recipient.to_string(), &msg).await?;
        Ok(())
    }

    /// Broadcast a message to all connected peers
    pub async fn broadcast(&self, content: &str) -> Result<()> {
        let sender_id = self.core.get_identity_id().await;
        let msg = ghostwire::core::message::Message::new(
            sender_id,
            "broadcast".to_string(),
            content.to_string(),
        );
        self.core.send_message(&msg).await?;
        Ok(())
    }

    /// Get our identity information
    pub async fn get_identity(&self) -> (String, String, Vec<u8>) {
        (
            self.core.get_identity_id().await,
            self.core.get_did().await,
            self.core.get_public_key().to_vec(),
        )
    }

    /// Get connected peer count
    pub fn peer_count(&self) -> usize {
        self.core.get_peer_count()
    }
}
```

---

## Configuration Options

### Programmatic Configuration

```rust
use ghostwire::config::Config;

let mut config = Config::default();

// Server settings
config.server.host = "127.0.0.1".to_string();
config.server.port = 8080;
config.server.web_enabled = true;

// Network settings
config.network.mesh_enabled = true;
config.network.max_peers = 100;
config.network.connection_timeout_secs = 30;

// Enable obfs4 for censorship resistance
config.network.obfs4_enabled = true;

// Security settings
config.security.max_connections_per_ip = 10;
config.security.max_messages_per_minute = 100;
config.security.blacklist_enabled = true;
config.security.threat_detection_enabled = true;

// Feature flags
config.features.matrix_bridge = false;
config.features.meshtastic_bridge = false;
config.features.ai_routing = true;
config.features.ai_threat_detection = true;

// AI model selection (auto-detects device capability)
config.features.auto_detect_device = true;
config.features.gemma_version = "4".to_string();
config.features.inference_backend = "ollama".to_string();
config.features.ollama_url = "http://localhost:11434".to_string();
```

### TOML Configuration File

```toml
# ghostwire.toml
[server]
host = "127.0.0.1"
port = 8080
web_enabled = true
cli_enabled = true
auto_update = true

[network]
mesh_enabled = true
reticulum_enabled = false
briar_enabled = false
stealth_tcp_enabled = false
max_peers = 50
connection_timeout_secs = 30
obfs4_enabled = true

[security]
max_connections_per_ip = 10
max_messages_per_minute = 100
blacklist_enabled = true
threat_detection_enabled = true
key_rotation_interval_days = 7

[encryption]
algorithm = "AES-256-GCM"
key_size = 32
session_timeout_hours = 1

[features]
matrix_bridge = false
meshtastic_bridge = false
web_ui = true
cli = true
api = true
ai_routing = true
ai_threat_detection = true
gemma_version = "4"
inference_backend = "ollama"
ollama_url = "http://localhost:11434"
auto_detect_device = true
```

Load from file:

```rust
let config = Config::load_from_path("ghostwire.toml")?;
// Or auto-detect: tries ghostwire.toml, config/ghostwire.toml, ~/.config/ghostwire/config.toml
let config = Config::load().unwrap_or_else(|_| Config::default());
```

### Environment Variables

```bash
GHOSTWIRE_HOST=127.0.0.1
GHOSTWIRE_PORT=8080
GHOSTWIRE_MAX_CONNECTIONS_PER_IP=10
GHOSTWIRE_MATRIX_BRIDGE=1        # Set to enable Matrix bridge
GHOSTWIRE_MESHTASTIC_BRIDGE=1    # Set to enable Meshtastic bridge
GHOSTWIRE_AUTO_UPDATE=true
GHOSTWIRE_OBFS4_ENABLED=true
GHOSTWIRE_STEALTH_SECRET=<hex>   # Shared secret for stealth TCP handshake
```

---

## Running as a Separate Process with REST API

For applications that cannot embed Rust (Python, JavaScript, mobile apps), run GhostWire as a daemon and communicate via REST API + WebSocket.

### Start the Daemon

```bash
# Start with default settings
ghostwire start

# Start with custom port
ghostwire start --port 9090

# Start with Tailscale bootstrap
ghostwire start --tailscale-bootstrap 100.64.0.1/QmPeerId
```

### Authenticate

```bash
# Get the auth token (debug builds)
TOKEN=$(curl -s http://localhost:8080/api/dev/token | jq -r '.data')

# Or login with the token
JWT=$(curl -s -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d "{\"token\": \"$TOKEN\"}" | jq -r '.data.token')
```

### Interact from Any Language

**Python:**
```python
import requests
import websockets
import json

TOKEN = "your-auth-token"
BASE = "http://localhost:8080/api"
HEADERS = {"X-Ghost-Token": TOKEN}

# Get identity
resp = requests.get(f"{BASE}/v2/profile", headers=HEADERS)
profile = resp.json()["data"]
print(f"Node ID: {profile['id']}")

# Send message
resp = requests.post(f"{BASE}/v2/messages/send", headers=HEADERS, json={
    "recipient": "4tK9mN2p...",
    "content": "Hello from Python!"
})

# Get messages
resp = requests.get(f"{BASE}/v2/messages", headers=HEADERS, params={"limit": 50})
messages = resp.json()["data"]

# Real-time events via WebSocket
async with websockets.connect("ws://localhost:8080/ws") as ws:
    async for message in ws:
        event = json.loads(message)
        print(f"Event: {event}")
```

**JavaScript/TypeScript:**
```typescript
const TOKEN = "your-auth-token";
const BASE = "http://localhost:8080/api";

// Get identity
const profile = await fetch(`${BASE}/v2/profile`, {
  headers: { "X-Ghost-Token": TOKEN }
}).then(r => r.json());

// Send message
await fetch(`${BASE}/v2/messages/send`, {
  method: "POST",
  headers: {
    "X-Ghost-Token": TOKEN,
    "Content-Type": "application/json"
  },
  body: JSON.stringify({
    recipient: "4tK9mN2p...",
    content: "Hello from JavaScript!"
  })
});

// Real-time events
const ws = new WebSocket("ws://localhost:8080/ws");
ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log("GhostWire event:", data);
};
```

---

## Custom Event Handling

The `CoreEvent` enum provides real-time notifications for all significant protocol events:

```rust
use ghostwire::core::CoreEvent;

async fn handle_custom_events(core: &Core) {
    let mut rx = core.event_bus.subscribe();

    while let Ok(event) = rx.recv().await {
        match event {
            CoreEvent::NewMessage(msg) => {
                // Process incoming message
                // msg.sender, msg.content, msg.timestamp
            }
            CoreEvent::PeerConnected(peer_id) => {
                // Update UI, sync data, etc.
            }
            CoreEvent::PeerDisconnected(peer_id) => {
                // Mark peer offline, queue messages for DTN
            }
            CoreEvent::SystemAlert(alert) => {
                // System-level notifications
                tracing::warn!("System alert: {}", alert);
            }
            CoreEvent::NetworkUpdate { active_peers, latency_ms } => {
                // Update network status display
            }
            CoreEvent::SecurityLockdown(enabled) => {
                if enabled {
                    // Emergency lockdown: all connections severed
                    // Notify user, log the event
                }
            }
        }
    }
}
```

### Subscribing Multiple Times

The event bus is a `tokio::sync::broadcast` channel. You can subscribe multiple times from different parts of your application:

```rust
// UI thread subscribes for display updates
let mut ui_rx = core.event_bus.subscribe();

// Storage thread subscribes for persistence
let mut storage_rx = core.event_bus.subscribe();

// Analytics thread subscribes for metrics
let mut analytics_rx = core.event_bus.subscribe();
```

Each subscriber receives all events independently. The channel buffer is 1024 events.

---

## Database Integration Pattern

GhostWire uses `sled` for local storage. Your application can access the same database or integrate with its own database.

### Accessing GhostWire's Internal Stores

```rust
// Message store (encrypted message history)
let messages = core.message_store.get_history("all", 100);

// Mailbox (offline message delivery)
let inbox = core.mailbox_store.drain_inbox()?;

// Knowledge Ark (content-addressed knowledge base)
let collections = core.knowledge_ark.list_collections().await;
let entries = core.knowledge_ark.search("search term").await;

// Trust store (web of trust scores)
let trust_score = core.trust_store.trust_score(&peer_id).await;
let all_peers = core.trust_store.all_peers().await;

// Ghost Channels
let channels = core.ghost_channels.list().await;
let channel_messages = core.ghost_channels.get_messages(&topic_id).await;
```

### Integrating with Your Own Database

If your application has its own database, you can mirror GhostWire events into it:

```rust
use ghostwire::core::CoreEvent;

async fn mirror_events_to_db(core: &Core, db: &MyDatabase) -> Result<()> {
    let mut rx = core.event_bus.subscribe();

    while let Ok(event) = rx.recv().await {
        match event {
            CoreEvent::NewMessage(msg) => {
                db.insert_message(&msg.sender, &msg.content, msg.timestamp).await?;
            }
            CoreEvent::PeerConnected(peer_id) => {
                db.set_peer_status(&peer_id, "online").await?;
            }
            CoreEvent::PeerDisconnected(peer_id) => {
                db.set_peer_status(&peer_id, "offline").await?;
            }
            _ => {}
        }
    }

    Ok(())
}
```

### Persistence Manager

GhostWire's `PersistenceManager` handles encrypted local storage. You can use it for your own application data:

```rust
use ghostwire::core::persistence_manager::PersistenceManager;

let persistence = PersistenceManager::new(None)?; // None = default profile

// Store encrypted data
persistence.store("my_app_key", &my_data_bytes).await?;

// Retrieve encrypted data
let data = persistence.retrieve("my_app_key").await?;
```

---

## Feature Flags

Control which subsystems are compiled into your application:

```toml
[dependencies]
ghostwire = {
    git = "https://github.com/Phantomojo/GhostWire-secure-mesh-communication",
    default-features = false,
    features = [
        "web",       # REST API + Web UI
        "cli",       # Command-line interface
        "mesh",      # libp2p mesh networking
        "security",  # Security manager, rate limiting
        "stealth",   # Stealth TCP transport
        "bluetooth", # BLE transport
        "qr",        # QR code generation
        # Optional bridges:
        # "matrix-bridge",
        # "meshtastic",
    ]
}
```

| Feature | Description | Adds |
|---------|-------------|------|
| `web` | REST API server + embedded Web UI | axum, rust-embed |
| `cli` | Command-line interface | clap |
| `mesh` | libp2p mesh networking | libp2p, libp2p-webrtc |
| `security` | Security manager, audit logging | governor, tower-governor |
| `stealth` | Stealth TCP transport | - |
| `matrix-bridge` | Matrix protocol bridge | matrix-sdk |
| `meshtastic` | Meshtastic serial bridge | serialport |
| `bluetooth` | BLE transport | bluer |
| `qr` | QR code generation | qrcode-generator |
| `minimal` | Minimal build (no optional features) | - |

---

## Next Steps

- **[Custom Transports](./CUSTOM_TRANSPORT.md)** -- Add BLE, LoRa, or your own transport
- **[Protocol Bridges](./PROTOCOL_BRIDGES.md)** -- Connect to Matrix, Meshtastic, Briar
- **[API Reference](./API_REFERENCE.md)** -- Complete REST API documentation
