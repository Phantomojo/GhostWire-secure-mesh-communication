# GhostWire Installation Guide

Complete guide for installing GhostWire on Linux, macOS, and Windows.

## Table of Contents

- [Quick Install](#quick-install)
- [Prerequisites](#prerequisites)
- [Installation Methods](#installation-methods)
  - [Linux / macOS](#linux--macos)
  - [Windows](#windows)
- [Configuration](#configuration)
- [Multi-PC Deployment](#multi-pc-deployment)
- [Troubleshooting](#troubleshooting)
- [Management](#management)

---

## Quick Install

### Linux / macOS (One-liner)

```bash
git clone https://github.com/Phantomojo/GhostWire-secure-mesh-communication.git
cd GhostWire-secure-mesh-communication
./install.sh
```

### Windows (PowerShell)

```powershell
git clone https://github.com/Phantomojo/GhostWire-secure-mesh-communication.git
cd GhostWire-secure-mesh-communication
.\install.ps1
```

---

## Prerequisites

GhostWire requires the following dependencies:

### All Platforms
- **Rust** 1.70+ (with Cargo)
- **Node.js** 16+ (with npm)
- **Git** (for updates)

### Linux Additional Requirements
- Build tools (gcc, make, pkg-config)
- OpenSSL development libraries

### Windows Additional Requirements
- Visual C++ Build Tools (installed automatically)

---

## Installation Methods

### Linux / macOS

#### 1. Install Dependencies Automatically

```bash
./scripts/install-deps.sh
```

This will detect your OS and install all required dependencies.

#### 2. Run the Installer

**Interactive Installation (Recommended):**
```bash
./install.sh
```

**Silent Installation:**
```bash
./install.sh --silent --username alice
```

**System-wide Installation:**
```bash
sudo ./install.sh --system --service
```

**Custom Directory:**
```bash
./install.sh --dir /custom/path
```

#### Installation Options

| Option            | Description                                                 |
| ----------------- | ----------------------------------------------------------- |
| `--user`          | Install for current user (default, recommended for privacy) |
| `--system`        | Install system-wide in `/opt/ghostwire`                     |
| `--dir DIR`       | Custom installation directory                               |
| `--service`       | Install as system service (auto-start)                      |
| `--no-service`    | Don't install system service (default)                      |
| `--silent`        | Silent mode, no prompts                                     |
| `--username NAME` | Set username for node identity                              |
| `--help`          | Show help message                                           |

---

### Windows

#### 1. Install Dependencies

**Option A: Automatic (Recommended)**
```powershell
.\scripts\install-deps.ps1
```

**Option B: Manual**
- Download Rust: https://rustup.rs/
- Download Node.js: https://nodejs.org/

#### 2. Run the Installer

**Interactive Installation:**
```powershell
.\install.ps1
```

**Silent Installation:**
```powershell
.\install.ps1 -Silent -Username alice
```

**System-wide Installation (Requires Admin):**
```powershell
.\install.ps1 -System -Service
```

#### Installation Options

| Option            | Description                                  |
| ----------------- | -------------------------------------------- |
| `-User`           | Install for current user (default)           |
| `-System`         | Install system-wide (requires Administrator) |
| `-InstallDir DIR` | Custom installation directory                |
| `-Service`        | Install as Windows Service                   |
| `-NoService`      | Don't install service (default)              |
| `-Silent`         | Silent mode, no prompts                      |
| `-Username NAME`  | Set username for node identity               |
| `-Help`           | Show help message                            |

---

## Configuration

After installation, GhostWire creates configuration files in:
- **Linux/macOS**: `~/.local/ghostwire/config/`
- **Windows**: `%LOCALAPPDATA%\GhostWire\config\`

### Key Configuration Files

#### `identity.toml`
Your node identity and nickname:
```toml
[identity]
nickname = "ghost"  # Set your display name
# Keypair auto-generated on first run
```

#### `network.toml`
Network and NAT traversal settings:
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
port = 3001
```

### Finding Nodes by Username

GhostWire uses a DHT-based username system. Other nodes can find you by your username instead of IP:

```rust
// Find and connect to a node by username
let peer = mesh.find_peer_by_username("alice").await?;
mesh.connect_to_peer(peer).await?;
```

---

## Multi-PC Deployment

### Local Network Setup

1. **Install on all PCs** using the installer
2. **Launch GhostWire** on each PC
3. **Automatic Discovery**: Nodes will find each other via mDNS

### Internet-Wide Mesh

GhostWire supports internet-wide mesh networking using:
- **NAT Traversal**: Automatic hole punching
- **Relay Servers**: For nodes behind strict NATs
- **DHT Discovery**: Find nodes across the internet by username

#### Manual Remote Deployment (Future Feature)

```bash
# Deploy to multiple hosts via SSH
./scripts/deploy-remote.sh host1.example.com host2.example.com host3.example.com
```

---

## Management

### Using GhostWire Manager

The `ghostwire-manager.sh` script provides easy management:

```bash
# Start GhostWire
./ghostwire-manager.sh start

# Stop GhostWire
./ghostwire-manager.sh stop

# Restart
./ghostwire-manager.sh restart

# Check status
./ghostwire-manager.sh status

# View logs
./ghostwire-manager.sh logs
./ghostwire-manager.sh logs -f  # Follow logs

# Create backup
./ghostwire-manager.sh backup

# Update to latest version
./ghostwire-manager.sh update

# Restore from backup
./ghostwire-manager.sh restore backup-file.tar.gz
```

### Manual Management

**Start GhostWire:**
```bash
~/.local/ghostwire/bin/ghostwire-launch
```

**View Logs:**
```bash
tail -f ~/.local/ghostwire/logs/ghostwire.log
```

**Edit Configuration:**
```bash
nano ~/.local/ghostwire/config/network.toml
```

---

## Troubleshooting

### Installation Issues

**Missing Dependencies:**
```bash
./scripts/install-deps.sh
```

**Build Fails:**
```bash
# Clean and rebuild
cd ghostwire
cargo clean
cargo build --release
```

**Permission Denied:**
```bash
chmod +x install.sh
./install.sh
```

### Runtime Issues

**Port Already in Use:**
Edit `~/.local/ghostwire/config/network.toml` and change ports:
```toml
[network]
listen_addr = "/ip4/0.0.0.0/tcp/4002"  # Changed from 4001

[web]
port = 3002  # Changed from 3001
```

**Cannot Connect to Other Nodes:**
1. Check firewall settings
2. Verify NAT traversal is enabled in `network.toml`
3. Check logs: `./ghostwire-manager.sh logs`

**Web UI Not Loading:**
```bash
# Check if GhostWire is running
./ghostwire-manager.sh status

# View logs for errors
./ghostwire-manager.sh logs
```

### Getting Help

**View Logs:**
```bash
./ghostwire-manager.sh logs -f
```

**Check Configuration:**
```bash
cat ~/.local/ghostwire/config/network.toml
```

**Test Network Connectivity:**
```bash
# Test P2P port
nc -zv localhost 4001

# Test Web UI port
curl http://localhost:3001/api/status
```

---

## Updating

### Automatic Update

```bash
./update.sh
```

Or using the manager:
```bash
./ghostwire-manager.sh update
```

The update process:
1. ✅ Creates automatic backup
2. ✅ Pulls latest changes
3. ✅ Rebuilds backend and frontend
4. ✅ Installs updated files
5. ✅ Rolls back automatically on failure

### Manual Update

```bash
# Pull latest changes
git pull

# Rebuild
cd ghostwire && cargo build --release && cd ..
cd webui && npm install && cd ..

# Restart
./ghostwire-manager.sh restart
```

---

## Uninstalling

```bash
./uninstall.sh
```

The uninstaller will:
1. Stop GhostWire service
2. Remove installation files
3. Remove from PATH
4. Optionally remove config/data (prompts you)

---

## Next Steps

After installation:

1. **Start GhostWire**: `./ghostwire-manager.sh start`
2. **Access Web UI**: Open http://localhost:3001
3. **Configure Identity**: Edit `~/.local/ghostwire/config/identity.toml`
4. **Test Connectivity**: Try connecting to other nodes
5. **Read Documentation**: Check `docs/` for more guides

---

## Advanced Topics

### Running as System Service

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

**Windows (Windows Service):**
```powershell
.\install.ps1 -System -Service
```

### Custom Relay Server

To run your own relay server:
```bash
./scripts/run-relay.sh
```

Then add it to `network.toml`:
```toml
relay_servers = [
    "/ip4/your-server-ip/tcp/4001/p2p/YourPeerID",
]
```

---

## License

GhostWire is licensed under the AGPL-3.0 License. See [LICENSE](../LICENSE) for details.

---

## Support

- **Issues**: https://github.com/Phantomojo/GhostWire-secure-mesh-communication/issues
- **Documentation**: https://github.com/Phantomojo/GhostWire-secure-mesh-communication/wiki
- **Community**: https://github.com/Phantomojo/GhostWire-secure-mesh-communication/discussions
