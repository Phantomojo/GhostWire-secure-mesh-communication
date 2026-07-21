# Quick Start: Build Your First GhostWire App

Get a minimal GhostWire node running and sending messages in under 5 minutes.

---

## Prerequisites

- **Rust 1.75+** (check with `rustc --version`)
- **Cargo** (comes with Rust)
- **Tokio** runtime (added as a dependency)

Install Rust if you haven't already:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## Step 1: Create a New Project

```bash
cargo new my-mesh-app
cd my-mesh-app
```

---

## Step 2: Add Dependencies

Edit `Cargo.toml`:

```toml
[package]
name = "my-mesh-app"
version = "0.1.0"
edition = "2021"

[dependencies]
ghostwire = { git = "https://github.com/Phantomojo/GhostWire-secure-mesh-communication" }
tokio = { version = "1", features = ["full"] }
anyhow = "1"
tracing = "0.1"
tracing-subscriber = "0.3"
```

> **Note:** The first build will take a few minutes as it compiles libp2p, cryptographic libraries, and the full GhostWire stack. Subsequent builds are much faster.

---

## Step 3: Create a Node

Replace `src/main.rs` with:

```rust
use ghostwire::config::Config;
use ghostwire::core::{Core, CoreEvent};
use anyhow::Result;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_target(true)
        .with_thread_ids(true)
        .init();

    tracing::info!("Starting my-mesh-app...");

    // Load default configuration
    let config = Config::default();

    // No bootstrap nodes = use default public gateways for zero-config discovery
    let bootstrap = vec![];

    // Create the full GhostWire protocol stack
    let core = Core::new(config, bootstrap, None, None).await?;
    let core = Arc::new(core);

    // Print our identity
    let my_id = core.get_identity_id().await;
    tracing::info!("My node ID: {}", my_id);

    // Start the nervous system (mesh networking, AI routing, DTN, etc.)
    core.clone()
        .start_nervous_system("0.0.0.0".to_string(), 0)
        .await;

    tracing::info!("Mesh networking started. Waiting for peers...");

    // Subscribe to the event bus
    let mut event_rx = core.event_bus.subscribe();

    // Run event listener in background
    let event_handle = tokio::spawn(async move {
        while let Ok(event) = event_rx.recv().await {
            match event {
                CoreEvent::NewMessage(msg) => {
                    tracing::info!("Message from {}: {}", msg.sender, msg.content);
                }
                CoreEvent::PeerConnected(peer_id) => {
                    tracing::info!("Peer connected: {}", peer_id);
                }
                CoreEvent::PeerDisconnected(peer_id) => {
                    tracing::info!("Peer disconnected: {}", peer_id);
                }
                CoreEvent::NetworkUpdate { active_peers, latency_ms } => {
                    tracing::info!("Network: {} peers, {}ms latency", active_peers, latency_ms);
                }
                _ => {}
            }
        }
    });

    // Keep the main task alive
    // In a real app, you'd handle user input, HTTP requests, etc.
    tokio::signal::ctrl_c().await?;
    tracing::info!("Shutting down...");

    event_handle.abort();
    Ok(())
}
```

---

## Step 4: Run Two Nodes

Open **two terminal windows**.

**Terminal 1:**
```bash
cargo run
```

Note the node ID printed at startup, e.g.:
```
My node ID: a1b2c3d4e5f6...
```

**Terminal 2:**
```bash
# Run a second instance on a different port
GHOSTWIRE_PORT=8081 cargo run
```

Both nodes will discover each other via mDNS on the local network. You should see:
```
Peer connected: <peer-id>
```

---

## Step 5: Send a Message

Add this code before the `ctrl_c().await` line in `main.rs`:

```rust
// Clone core for the send task
let core_send = core.clone();

// Wait a moment for peers to connect
tokio::time::sleep(Duration::from_secs(5)).await;

// Get connected peers
let peer_count = core_send.get_peer_count();
tracing::info!("Connected peers: {}", peer_count);

if peer_count > 0 {
    // In a real app, you'd get the peer ID from the peer list
    // For now, broadcast to all connected peers
    let msg = ghostwire::core::message::Message::new(
        core_send.get_identity_id().await,
        "broadcast".to_string(),
        "Hello from my-mesh-app!".to_string(),
    );

    match core_send.send_message(&msg).await {
        Ok(_) => tracing::info!("Message sent!"),
        Err(e) => tracing::error!("Send failed: {}", e),
    }
}
```

Run both nodes again. After ~5 seconds, the second terminal should show:
```
Message sent!
```

And the first terminal should show:
```
Message from <sender-id>: Hello from my-mesh-app!
```

---

## Step 6: Receive Messages via Event Stream

The event listener from Step 3 already handles incoming messages. The `CoreEvent::NewMessage` variant fires for every received message.

For a more structured approach, you can also query the message store:

```rust
// Get last 50 messages from all peers
let messages = core.message_store.get_history("all", 50);

for msg in &messages {
    println!("[{}] {}: {}", msg.timestamp, msg.sender, msg.content);
}
```

---

## Running as a Daemon

For production use, run GhostWire as a background daemon:

```bash
# Start the daemon
ghostwire start

# Check status
ghostwire status

# Open the TUI
ghostwire tui

# Stop the daemon
ghostwire stop
```

The daemon listens on port 8080 by default and serves the full REST API.

---

## Troubleshooting

### Build fails with "edition 2024" error

Some dependencies require Rust edition 2024 support. Update your toolchain:

```bash
rustup update stable
```

### No peers discovered

GhostWire uses mDNS for local discovery. Ensure:
- Both nodes are on the same network
- Firewall allows UDP port 5353 (mDNS)
- No Docker network isolation blocking mDNS

For cross-network connections, provide bootstrap nodes:

```rust
let bootstrap: Vec<Multiaddr> = vec![
    "/ip4/100.64.0.1/tcp/4001/p2p/QmPeerId".parse()?,
];
let core = Core::new(config, bootstrap, None, None).await?;
```

### "Identity creation failed"

On first run, GhostWire creates a new identity. If this fails:
- Ensure the `.ghostwire_db` directory is writable
- Check disk space (identity storage requires minimal space)

### Port already in use

Change the daemon port:

```bash
ghostwire start --port 9090
```

Or for library usage, set `GHOSTWIRE_PORT` environment variable.

### Memory usage is high

GhostWire loads AI models into memory. Disable AI features if not needed:

```toml
[dependencies]
ghostwire = { git = "...", default-features = false, features = ["web", "cli", "mesh", "security"] }
```

### WebSocket connection refused

The WebSocket endpoint is at `/ws` (not `/api/ws`):

```bash
wscat -c ws://localhost:9097/ws
```

---

## What's Next?

- **[Embedding Guide](./EMBEDDING.md)** -- Add configuration, custom event handling, database integration
- **[API Reference](./API_REFERENCE.md)** -- Full REST API documentation for daemon mode
- **[Custom Transports](./CUSTOM_TRANSPORT.md)** -- Add BLE, LoRa, or your own transport
