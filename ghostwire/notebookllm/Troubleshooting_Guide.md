# 🔧 GhostWire Troubleshooting Guide

**Date**: February 16, 2026  
**Status**: ✅ Production-Ready

---

## 📋 Quick Reference

| Issue | Quick Fix |
|-------|-----------|
| **Won't start** | Check ports, logs |
| **Can't connect** | Firewall, NAT |
| **High memory** | Reduce max_peers |
| **Slow mesh** | Check NAT traversal |
| **Auth fails** | Verify JWT token |

---

## 🚨 Common Issues

### 1. Service Won't Start

#### Symptom
```bash
$ ./ghostwire web
Error: Address already in use (os error 98)
```

#### Causes
- Port 3001 or 4001 already in use
- Permission denied
- Missing dependencies

#### Solutions

**Check Port Usage**:
```bash
# Find process using port
lsof -i :3001
netstat -tulpn | grep 3001

# Kill process
kill -9 <PID>

# Or use different port
./ghostwire web --port 3002
```

**Check Permissions**:
```bash
# Run as non-root (recommended)
./ghostwire web

# If binding to privileged ports (<1024), use:
sudo setcap 'cap_net_bind_service=+ep' ./ghostwire
```

**Check Dependencies**:
```bash
# Verify Rust installation
rustc --version

# Rebuild if needed
cargo clean
cargo build --release
```

---

### 2. Can't Connect to Mesh

#### Symptom
```
No peers connected after 5 minutes
```

#### Causes
- Firewall blocking ports
- NAT not configured
- Bootstrap nodes unreachable

#### Solutions

**Check Firewall**:
```bash
# UFW status
sudo ufw status

# Allow ports
sudo ufw allow 3001/tcp
sudo ufw allow 4001/tcp
sudo ufw allow 4001/udp

# Verify
sudo ufw status | grep ghostwire
```

**Check NAT Configuration**:
```bash
# Check if behind NAT
curl ifconfig.me

# Enable port forwarding on router
# Forward ports 3001, 4001 to your local IP
```

**Test Bootstrap Connectivity**:
```bash
# Test connection to bootstrap node
curl -v https://bootstrap.libp2p.io

# If blocked, use alternative bootstraps
./ghostwire web --bootstrap /dns4/your-node.com/tcp/4001/p2p/YOUR_PEER_ID
```

---

### 3. High Memory Usage

#### Symptom
```bash
$ ps aux | grep ghostwire
ghostwire  2.1GB  1.2%  ghostwire
```

#### Causes
- Too many peers
- Memory leak
- Large routing tables

#### Solutions

**Reduce Max Peers**:
```toml
# config.toml
[network]
max_peers = 20  # Default: 50
```

**Check for Memory Leaks**:
```bash
# Install heaptrack
sudo apt install heaptrack

# Profile
heaptrack ghostwire web

# Analyze
heaptrack_gui heaptrack.ghostwire.*.gz
```

**Restart Periodically**:
```bash
# Add to systemd service
[Service]
Restart=always
RestartSec=86400  # Restart daily
```

---

### 4. Slow Mesh Performance

#### Symptom
```
Message delivery takes >5 seconds
```

#### Causes
- Poor NAT traversal
- High latency peers
- Network congestion

#### Solutions

**Enable AutoNAT** (should be default in v0.56):
```toml
# config.toml
[network]
enable_autonat = true
enable_dcutr = true
enable_relay = true
```

**Check NAT Traversal Success**:
```bash
# Check logs for NAT type
journalctl -u ghostwire | grep -i nat

# Should show: "NAT: Public or Symmetric"
# If "Restricted", port forwarding needed
```

**Monitor Peer Quality**:
```bash
# Check peer list
curl http://localhost:3001/api/peers

# Look for high latency peers
# Consider blocking if consistently slow
```

---

### 5. Authentication Failures

#### Symptom
```json
{
  "error": "Invalid token",
  "code": 401
}
```

#### Causes
- Expired JWT token
- Invalid token format
- Clock skew

#### Solutions

**Get New Token**:
```bash
# Login
curl -X POST http://localhost:3001/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"testuser","password":"securepassword"}'

# Use new token
export GHOSTWIRE_TOKEN="eyJhbGciOiJIUzI1NiIs..."

# Test protected endpoint
curl http://localhost:3001/api/peers \
  -H "X-Ghost-Token: $GHOSTWIRE_TOKEN"
```

**Check System Clock**:
```bash
# Sync time
sudo timedatectl set-ntp true

# Verify
timedatectl status
```

---

### 6. Build Failures

#### Symptom
```bash
$ cargo build --release
error[E0432]: unresolved import
```

#### Causes
- Outdated dependencies
- Rust version mismatch
- Corrupted build cache

#### Solutions

**Update Rust**:
```bash
rustup update stable
rustup default stable
```

**Clean Build**:
```bash
cargo clean
cargo update
cargo build --release
```

**Check Rust Version**:
```bash
rustc --version
# Should be 1.70+

# If older:
rustup install stable
```

---

### 7. Web UI Not Loading

#### Symptom
```
Browser shows: "This site can't be reached"
```

#### Causes
- Web server not running
- Wrong port
- CORS issues

#### Solutions

**Verify Server Running**:
```bash
# Check process
ps aux | grep ghostwire

# Check port
netstat -tulpn | grep 3001

# Test endpoint
curl http://localhost:3001/api/status
```

**Check CORS Configuration**:
```toml
# config.toml
[web]
cors_origins = ["*"]  # For testing only
# In production, specify your domain:
# cors_origins = ["https://your-domain.com"]
```

**Clear Browser Cache**:
```bash
# Or use incognito mode
# Or force refresh: Ctrl+Shift+R
```

---

### 8. Mobile App Issues (Android)

#### Symptom
```
App crashes on startup
```

#### Causes
- Missing permissions
- Incompatible Android version
- Build configuration error

#### Solutions

**Check Permissions**:
```xml
<!-- AndroidManifest.xml should include: -->
<uses-permission android:name="android.permission.INTERNET" />
<uses-permission android:name="android.permission.ACCESS_NETWORK_STATE" />
<uses-permission android:name="android.permission.ACCESS_WIFI_STATE" />
```

**Check Android Version**:
```bash
# Minimum SDK: 24 (Android 7.0)
# Check device Android version
adb shell getprop ro.build.version.sdk
```

**Rebuild App**:
```bash
cd src-tauri
cargo tauri android clean
cargo tauri android build
```

---

## 📊 Diagnostic Commands

### System Health

```bash
# Check all GhostWire processes
ps aux | grep ghostwire

# Check memory usage
ps -o pid,rss,command -p $(pgrep ghostwire)

# Check open files
lsof -p $(pgrep ghostwire)

# Check network connections
netstat -tulpn | grep ghostwire
```

### Log Analysis

```bash
# View recent logs
journalctl -u ghostwire -n 100

# Follow logs
journalctl -u ghostwire -f

# Filter by level
journalctl -u ghostwire | grep -i error

# Search for specific issue
journalctl -u ghostwire | grep -i "peer\|connect\|error"
```

### Network Diagnostics

```bash
# Test local connectivity
curl http://localhost:3001/api/status

# Test external connectivity
curl ifconfig.me

# Check port accessibility
nc -zv your-public-ip 4001

# Trace route to peer
traceroute peer-address
```

---

## 🧪 Testing Fixes

### Verify Port Forwarding

```bash
# From external machine
nc -zv your-public-ip 4001

# Should show: succeeded!
# If fails, check router configuration
```

### Verify NAT Traversal

```bash
# Check logs for AutoNAT status
journalctl -u ghostwire | grep autonat

# Should show: "Public address: your-ip:4001"
```

### Verify Mesh Connectivity

```bash
# Check peer count
curl http://localhost:3001/api/peers | jq '.peers | length'

# Should show: > 0 (unless first node)
```

---

## 📞 Getting Help

### Collect Debug Information

```bash
# System info
uname -a
rustc --version
cargo --version

# GhostWire version
./ghostwire --version

# Recent logs
journalctl -u ghostwire -n 200 > ghostwire_logs.txt

# Network config
netstat -tulpn | grep ghostwire > network.txt

# Send to support
# Attach: ghostwire_logs.txt, network.txt
```

### Support Channels

- **GitHub Issues**: [Report Bug](https://github.com/Phantomojo/GhostWire-secure-mesh-communication/issues)
- **Discussions**: [Ask Question](https://github.com/Phantomojo/GhostWire-secure-mesh-communication/discussions)
- **Documentation**: [Browse Docs](https://github.com/Phantomojo/GhostWire-secure-mesh-communication/tree/main/ghostwire/notebookllm)

---

## 🎯 Prevention Best Practices

### Regular Maintenance

```bash
# Weekly: Check logs
journalctl -u ghostwire --since "1 week ago" | grep -i error

# Monthly: Update
cargo update
cargo build --release

# Quarterly: Review config
cat ~/.config/ghostwire/config.toml
```

### Monitoring Setup

```bash
# Install monitoring
sudo apt install prometheus-node-exporter

# Set up alerts
# Alert if:
# - Service down for > 5 minutes
# - Memory usage > 90%
# - Peer count = 0 for > 1 hour
```

---

**Last Updated**: February 16, 2026  
**Maintained By**: GhostWire Core Team  
**Support**: Available via GitHub Issues
