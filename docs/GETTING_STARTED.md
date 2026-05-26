# GhostWire — Getting Started

> Complete guide to installing, configuring, and running GhostWire on any platform.

---

## Table of Contents

- [Quick Start (30 Seconds)](#quick-start-30-seconds)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Starting GhostWire](#starting-ghostwire)
- [Verification](#verification)
- [Configuration](#configuration)
- [Management](#management)
- [Troubleshooting](#troubleshooting)
- [Next Steps](#next-steps)

---

## Quick Start (30 Seconds)

```bash
git clone https://github.com/Phantomojo/GhostWire-secure-mesh-communication.git
cd GhostWire-secure-mesh-communication
./scripts/RUN_GHOSTWIRE.sh
```

That's it. The script handles everything:
- Checks if services are running
- Installs dependencies (first run only)
- Starts the backend automatically
- Opens the web interface in your browser

Press **Ctrl+C** to stop.

---

## Prerequisites

### Required

| Dependency | Minimum Version | Check Command |
|-----------|----------------|---------------|
| **Rust** | 1.70+ | `rustc --version` |
| **Cargo** | 1.70+ | `cargo --version` |
| **Git** | — | `git --version` |

### Linux

```bash
# Build tools and OpenSSL
sudo apt install build-essential pkg-config libssl-dev
# or: sudo dnf groupinstall "Development Tools"
```

### Windows

Download from:
- **Rust**: https://rustup.rs/
- **Git**: https://git-scm.com/

### macOS

```bash
xcode-select --install
```

### Ports Required

| Port | Purpose |
|------|---------|
| **9097** | Web UI / REST API |
| **4001** | P2P mesh networking (libp2p) |

Check availability:

```bash
netstat -tulpn | grep -E ':(9097|4001)'
```

---

## Installation

### Quick Install (All Platforms)

```bash
git clone https://github.com/Phantomojo/GhostWire-secure-mesh-communication.git
cd GhostWire-secure-mesh-communication
```

#### Linux / macOS

```bash
# Install dependencies
./scripts/install-deps.sh

# Interactive install (recommended)
./install.sh

# Silent install
./install.sh --silent --username alice

# System-wide with service
sudo ./install.sh --system --service
```

#### Windows (PowerShell)

```powershell
# Install dependencies
.\scripts\install-deps.ps1

# Interactive install
.\install.ps1

# Silent install
.\install.ps1 -Silent -Username alice
```

### Installation Options

| Option | Description |
|--------|-------------|
| `--user` / `-User` | Install for current user (default) |
| `--system` / `-System` | Install system-wide in `/opt/ghostwire` |
| `--dir DIR` / `-InstallDir DIR` | Custom installation directory |
| `--service` / `-Service` | Install as system service (auto-start) |
| `--no-service` / `-NoService` | Don't install service (default) |
| `--silent` / `-Silent` | No prompts, non-interactive |
| `--username NAME` / `-Username NAME` | Set node identity name |

### Build from Source

```bash
cd ghostwire

# Debug build (faster compile)
cargo build

# Release build (optimized)
cargo build --release
```

---

## Starting GhostWire

### Method 1: Quick Script (Recommended)

```bash
./scripts/RUN_GHOSTWIRE.sh
```

Starts the backend and opens the web UI. Handles dependency checks automatically.

### Method 2: Cargo (Full Control)

```bash
# Default start
cargo run -- web --port 9097

# With all options
cargo run -- web \
  --api-host 0.0.0.0 \
  --mesh-host 0.0.0.0 \
  --port 9097 \
  --profile default

# With custom bootstrap nodes
cargo run -- web --port 9097 \
  --bootstrap /dnsaddr/bootstrap.libp2p.io/p2p/QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN
```

#### CLI Options

| Option | Description | Default |
|--------|-------------|---------|
| `--api-host <HOST>` | API bind address | `127.0.0.1` |
| `--mesh-host <HOST>` | Mesh network bind address | `0.0.0.0` |
| `--port <PORT>` | API port | `9097` |
| `--bootstrap <ADDR>` | Bootstrap node address (repeatable) | — |
| `--profile <NAME>` | Profile name | `default` |

### Method 3: Start Script

```bash
# Make executable (first time only)
chmod +x start-backend.sh

# Run
./start-backend.sh
```

This script checks for a pre-built binary, falls back to debug build, and auto-builds if needed.

---

## Verification

### API Health Check

```bash
curl http://localhost:9097/api/status
```

**Expected response:**
```json
{
  "status": "ok",
  "version": "0.1.0"
}
```

### Process Check

```bash
ps aux | grep ghostwire
```

### Port Check

```bash
netstat -tulpn | grep ghostwire
```

**Expected:**
```
tcp  0  0 0.0.0.0:9097  0.0.0.0:*  LISTEN  12345/ghostwire
tcp  0  0 0.0.0.0:4001  0.0.0.0:*  LISTEN  12345/ghostwire
```

### Web UI

Open **http://localhost:9097** in your browser.

Expected: GhostWire dashboard loads, no CORS errors in the browser console (F12).

### Postman Test

1. Import collection: `tests/api/GhostWire_API.postman_collection.json`
2. Open **Auth & Bootstrap** → **Get Dev Token**
3. Click **Send**
4. Expected: 200 OK with token

---

## Configuration

GhostWire creates configuration files on first run:

| Platform | Config Directory |
|----------|-----------------|
| Linux / macOS | `~/.local/ghostwire/config/` |
| Windows | `%LOCALAPPDATA%\GhostWire\config\` |

### identity.toml

```toml
[identity]
nickname = "ghost"  # Your display name
# Keypair auto-generated on first run
```

### network.toml

```toml
[network]
listen_addr = "/ip4/0.0.0.0/tcp/4001"

# NAT traversal for internet connectivity
enable_relay = true
enable_autonat = true
enable_dcutr = true

# Bootstrap nodes for peer discovery
bootstrap_nodes = [
    "/dnsaddr/bootstrap.libp2p.io/p2p/QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN",
]

[web]
host = "0.0.0.0"
port = 9097
```

### Finding Nodes by Username

GhostWire uses a DHT-based username system. Other nodes can find you by username instead of IP:

```rust
let peer = mesh.find_peer_by_username("alice").await?;
mesh.connect_to_peer(peer).await?;
```

---

## Management

### Starting / Stopping

```bash
# Using the manager script
./ghostwire-manager.sh start
./ghostwire-manager.sh stop
./ghostwire-manager.sh restart
./ghostwire-manager.sh status
./ghostwire-manager.sh logs
./ghostwire-manager.sh logs -f   # Follow logs
```

```bash
# Manual start
~/.local/ghostwire/bin/ghostwire-launch

# View logs
tail -f ~/.local/ghostwire/logs/ghostwire.log

# Edit config
nano ~/.local/ghostwire/config/network.toml
```

### Backups

```bash
# Create backup
./ghostwire-manager.sh backup

# Restore from backup
./ghostwire-manager.sh restore backup-file.tar.gz
```

### Updating

```bash
# Automatic (recommended)
./ghostwire-manager.sh update
# or
./update.sh

# Manual
git pull
cd ghostwire && cargo build --release && cd ..
cd webui && npm install && cd ..
./ghostwire-manager.sh restart
```

### Uninstalling

```bash
./uninstall.sh
```

The uninstaller stops the service, removes files, and optionally removes config/data.

### Running as a System Service

**Linux (systemd):**
```bash
sudo ./install.sh --system --service
sudo systemctl start ghostwire
sudo systemctl enable ghostwire
```

**macOS (launchd):**
```bash
./install.sh --service
```

**Windows (Service):**
```powershell
.\install.ps1 -System -Service
```

### Custom Relay Server

```bash
# Run your own relay
./scripts/run-relay.sh
```

Then add it to `network.toml`:
```toml
relay_servers = [
    "/ip4/your-server-ip/tcp/4001/p2p/YourPeerID",
]
```

---

## Troubleshooting

### Database Lock Error

```
Error: IO error: could not acquire lock on ...
```

**Causes:** Another GhostWire instance is already running, or a previous instance crashed without releasing the file lock.

**Solutions:**

A. **Kill existing processes:**
```bash
pkill -9 ghostwire
sleep 3
ps aux | grep ghostwire  # Verify none remain
```

B. **Remove stale lock file:**
```bash
rm -rf ~/.local/ghostwire/vault/*.lock
```

C. **Wait:** Locks auto-release after ~30–60 seconds.

### Port Already in Use

```
Error: Address already in use (os error 98)
```

**Solutions:**

A. **Find and kill the process using the port:**
```bash
lsof -i :9097
kill -9 <PID>
```

B. **Use a different port:**
```bash
cargo run -- web --port 9098
```

### Permission Denied

```
Permission denied (os error 13)
```

- Don't use `sudo` unless installing system-wide
- Ports below 1024 require root — stick with 9097
- On Linux, set capabilities as an alternative:
  ```bash
  sudo setcap 'cap_net_bind_service=+ep' target/debug/ghostwire
  ```

### Build Errors

```
error[E0432]: unresolved import
```

**Solutions:**

```bash
# Update Rust
rustup update stable
rustup default stable

# Clean and rebuild
cargo clean
cargo build

# Update dependencies
cargo update
cargo build
```

### Web UI Shows CORS Errors

**Symptoms:** UI loads but shows "Failed to fetch"; console shows CORS errors.

**Solutions:**

1. Make sure the backend is running on port 9097
2. In the browser console (F12), set the backend URL:
   ```javascript
   localStorage.setItem('ghostwire_backend_url', 'http://localhost:9097/api');
   location.reload();
   ```

---

## Next Steps

1. **Explore the TUI:** Run GhostWire with `cargo run -- web --port 9097` and use the terminal interface
2. **Secure your node:** Review [Security docs](security/SECURITY.md)
3. **Connect multiple nodes:** Deploy on two machines — they auto-discover via mDNS on the same LAN
4. **Build on GhostWire:** Check the [Developer guides](development/README.md)
5. **Read the protocol specs:** [Specifications](spec/PROTOCOL.md)
6. **Join the community:** [GitHub Discussions](https://github.com/Phantomojo/GhostWire-secure-mesh-communication/discussions)

---

## Startup Sequence

When GhostWire starts, this is what happens:

```
1. Initialize logging
   ↓
2. Load configuration
   ↓
3. Check for updates (if enabled)
   ↓
4. Load identity from vault
   ↓
5. Load encryption keys
   ↓
6. Initialize security manager
   ↓
7. Initialize transport registry
   ↓
8. Start network monitor
   ↓
9. Start mesh network (auto)
   ↓
10. Start web server
   ↓
✅ Ready!
```

Typical startup time: **3–5 seconds**.

---

## Best Practices

### Do's ✅

- Kill processes cleanly: `pkill ghostwire`
- Wait 2–3 seconds before restarting
- Use different profiles for testing: `--profile test1`
- Check logs with `RUST_LOG=debug cargo run` when debugging
- Backup vault before major changes

### Don'ts ❌

- Don't run multiple instances on the same profile
- Don't use `kill -9` unless necessary
- Don't delete the vault while GhostWire is running
- Don't ignore lock errors
- Don't run as root unless installing system-wide

---

## Reference

| Command | Description |
|---------|-------------|
| `./scripts/RUN_GHOSTWIRE.sh` | Start everything (recommended) |
| `cargo run -- web --port 9097` | Start backend on port 9097 |
| `pkill -9 ghostwire` | Kill all GhostWire processes |
| `curl http://localhost:9097/api/status` | Health check |
| `netstat -tulpn \| grep ghostwire` | Check listening ports |
| `RUST_LOG=debug cargo run -- web --port 9097` | Start with debug logging |

---

**Last Updated:** May 26, 2026
**Version:** 0.1.0
**Status:** Production-Ready
