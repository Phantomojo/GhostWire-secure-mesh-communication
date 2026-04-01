# 🚀 GhostWire Deployment Guide

**Date**: February 16, 2026  
**Status**: ✅ Production-Ready

---

## 📋 Overview

This guide covers deployment of GhostWire across different environments: development, production, and mobile.

---

## 🖥️ System Requirements

### Minimum Requirements

| Component | Requirement |
|-----------|-------------|
| **CPU** | 2 cores (x86_64 or ARM64) |
| **RAM** | 512MB (1GB recommended) |
| **Storage** | 100MB |
| **Network** | TCP/UDP ports open |

### Recommended Requirements

| Component | Specification |
|-----------|---------------|
| **CPU** | 4+ cores |
| **RAM** | 2GB+ |
| **Storage** | 1GB SSD |
| **Network** | Public IP or port forwarding |

---

## 🔧 Installation Methods

### Method 1: Quick Install (Linux/macOS)

```bash
# One-line installation
git clone https://github.com/Phantomojo/GhostWire-secure-mesh-communication.git
cd GhostWire-secure-mesh-communication
./scripts/install.sh

# Start GhostWire
./scripts/ghostwire.sh start
```

### Method 2: Manual Build

```bash
# Clone repository
git clone https://github.com/Phantomojo/GhostWire-secure-mesh-communication.git
cd GhostWire-secure-mesh-communication/ghostwire

# Build release
cargo build --release

# Run
./target/release/ghostwire web --host 0.0.0.0 --port 3001
```

### Method 3: Docker

```bash
# Using docker-compose
cd GhostWire-secure-mesh-communication
docker-compose up -d

# Check status
docker-compose ps

# View logs
docker-compose logs -f ghostwire
```

### Method 4: Tauri Mobile (Android)

```bash
# Install Tauri CLI
cargo install tauri-cli

# Build Android APK
cd src-tauri
cargo tauri android build

# Install on device
adb install app/build/outputs/apk/debug/app-debug.apk
```

---

## ⚙️ Configuration

### Environment Variables

```bash
# Required
GHOSTWIRE_HOST=0.0.0.0          # Listen address
GHOSTWIRE_PORT=3001              # Web UI port
GHOSTWIRE_P2P_PORT=4001          # P2P port

# Optional
GHOSTWIRE_LOG_LEVEL=info         # trace, debug, info, warn, error
GHOSTWIRE_PROFILE=default        # User profile name
GHOSTWIRE_AUTO_UPDATE=false      # Enable auto-updates
```

### Configuration File

**Location**: `~/.config/ghostwire/config.toml`

```toml
[server]
host = "0.0.0.0"
port = 3001

[network]
p2p_port = 4001
max_peers = 50
enable_relay = true

[security]
enable_encryption = true
enable_stealth = true
max_connections_per_ip = 10

[logging]
level = "info"
log_to_file = true
```

---

## 🌐 Network Configuration

### Port Requirements

| Port | Protocol | Purpose |
|------|----------|---------|
| 3001 | TCP | Web UI / API |
| 4001 | TCP/UDP | P2P mesh networking |
| 4002 | UDP | QUIC transport (optional) |

### Firewall Rules

```bash
# UFW (Ubuntu)
sudo ufw allow 3001/tcp
sudo ufw allow 4001/tcp
sudo ufw allow 4001/udp

# firewalld (CentOS/RHEL)
sudo firewall-cmd --add-port=3001/tcp --permanent
sudo firewall-cmd --add-port=4001/tcp --permanent
sudo firewall-cmd --add-port=4001/udp --permanent
sudo firewall-cmd --reload

# iptables
sudo iptables -A INPUT -p tcp --dport 3001 -j ACCEPT
sudo iptables -A INPUT -p tcp --dport 4001 -j ACCEPT
sudo iptables -A INPUT -p udp --dport 4001 -j ACCEPT
```

### NAT Port Forwarding

For best mesh connectivity, forward ports on your router:

1. Access router admin panel (usually 192.168.1.1)
2. Find "Port Forwarding" or "Virtual Server"
3. Add rules:
   - Port 3001 → Your server IP:3001
   - Port 4001 → Your server IP:4001
4. Save and restart router

---

## 🚀 Production Deployment

### Systemd Service (Linux)

**File**: `/etc/systemd/system/ghostwire.service`

```ini
[Unit]
Description=GhostWire Secure Mesh Node
After=network.target

[Service]
Type=simple
User=ghostwire
WorkingDirectory=/opt/ghostwire
ExecStart=/opt/ghostwire/ghostwire web --host 0.0.0.0 --port 3001
Restart=always
RestartSec=10
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
```

**Enable and Start**:
```bash
# Create user
sudo useradd -r -s /bin/false ghostwire

# Copy binary
sudo cp target/release/ghostwire /opt/ghostwire/
sudo chown -R ghostwire:ghostwire /opt/ghostwire

# Install service
sudo cp ghostwire.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable ghostwire
sudo systemctl start ghostwire

# Check status
sudo systemctl status ghostwire
```

### Docker Production

**docker-compose.prod.yml**:
```yaml
version: '3.8'

services:
  ghostwire:
    image: ghostwire/ghostwire:latest
    container_name: ghostwire
    restart: unless-stopped
    ports:
      - "3001:3001"
      - "4001:4001/tcp"
      - "4001:4001/udp"
    volumes:
      - ghostwire_data:/app/data
      - ghostwire_config:/app/config
    environment:
      - GHOSTWIRE_HOST=0.0.0.0
      - GHOSTWIRE_LOG_LEVEL=warn
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3001/api/status"]
      interval: 30s
      timeout: 10s
      retries: 3

volumes:
  ghostwire_data:
  ghostwire_config:
```

---

## 📱 Mobile Deployment (Android)

### Prerequisites

```bash
# Install Android SDK
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Tauri CLI
cargo install tauri-cli

# Install Android target
rustup target add aarch64-linux-android
rustup target add x86_64-linux-android
```

### Build APK

```bash
cd src-tauri

# Debug build
cargo tauri android build

# Release build (sign with your key)
cargo tauri android build --release --keystore my-release-key.keystore
```

### Install on Device

```bash
# Via ADB
adb install app/build/outputs/apk/debug/app-debug.apk

# Or transfer APK and install manually
```

---

## 🔐 Security Hardening

### Production Checklist

- [ ] Change default ports (optional but recommended)
- [ ] Enable firewall
- [ ] Configure fail2ban
- [ ] Set up SSL/TLS for web UI
- [ ] Enable automatic security updates
- [ ] Configure log rotation
- [ ] Set up monitoring/alerting
- [ ] Regular backups of identity keys

### SSL/TLS Setup

```bash
# Using Caddy (automatic HTTPS)
# Install Caddy
sudo apt install caddy

# /etc/caddy/Caddyfile
mesh.example.com {
    reverse_proxy localhost:3001
}

# Start Caddy
sudo systemctl start caddy
```

### fail2ban Configuration

**File**: `/etc/fail2ban/jail.local`

```ini
[ghostwire]
enabled = true
port = 3001,4001
filter = ghostwire
logpath = /var/log/ghostwire/*.log
maxretry = 5
bantime = 3600
```

---

## 📊 Monitoring

### Health Check Endpoint

```bash
# Check status
curl http://localhost:3001/api/status

# Expected response
{
  "status": "ok",
  "version": "0.1.0",
  "peers": 12,
  "uptime": 86400
}
```

### Prometheus Metrics (Future)

```yaml
# prometheus.yml
scrape_configs:
  - job_name: 'ghostwire'
    static_configs:
      - targets: ['localhost:3001']
    metrics_path: '/api/metrics'
```

### Log Aggregation

```bash
# View logs
journalctl -u ghostwire -f

# Or with Docker
docker-compose logs -f ghostwire
```

---

## 🔄 Updates

### Manual Update

```bash
# Stop service
sudo systemctl stop ghostwire

# Backup
cp -r /opt/ghostwire /opt/ghostwire.backup

# Update binary
cargo build --release
cp target/release/ghostwire /opt/ghostwire/

# Restart
sudo systemctl start ghostwire
```

### Automatic Updates

```bash
# Enable in config
GHOSTWIRE_AUTO_UPDATE=true
GHOSTWIRE_UPDATE_INTERVAL=86400  # 24 hours

# Or via script
./scripts/update.sh
```

---

## 🚨 Troubleshooting

### Service Won't Start

```bash
# Check logs
journalctl -u ghostwire -n 50

# Common issues:
# - Port already in use
# - Permission denied
# - Missing dependencies
```

### Can't Connect to Mesh

```bash
# Check firewall
sudo ufw status

# Verify ports are open
netstat -tulpn | grep ghostwire

# Test connectivity
curl http://localhost:3001/api/status
```

### High Memory Usage

```bash
# Check memory
ps aux | grep ghostwire

# Reduce max_peers in config
# Restart service
sudo systemctl restart ghostwire
```

---

## 📚 Additional Resources

- **[Testing_Guide.md](Testing_Guide.md)** - Test deployment
- **[Troubleshooting_Guide.md](Troubleshooting_Guide.md)** - Common issues
- **[Security_Best_Practices.md](Security_Best_Practices.md)** - Security hardening

---

**Last Updated**: February 16, 2026  
**Maintained By**: GhostWire Core Team  
**Deployment Status**: ✅ Production-Ready
