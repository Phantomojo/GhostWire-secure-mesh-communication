# 🚀 How to Start GhostWire - Complete Guide

**Date**: February 16, 2026  
**Status**: Production-Ready

---

## 📋 Table of Contents

1. [Understanding the Issue](#understanding-the-issue)
2. [Prerequisites](#prerequisites)
3. [Method 1: Quick Start](#method-1-quick-start)
4. [Method 2: Manual Start](#method-2-manual-start)
5. [Method 3: Using Scripts](#method-3-using-scripts)
6. [Troubleshooting](#troubleshooting)
7. [Verification](#verification)

---

## 🧠 Understanding the Issue

### The Lock Error

When you see this error:
```
Error: IO error: could not acquire lock on "
```

**What it means:**
- Another GhostWire process is already running
- The database (`sled`) uses file locks to prevent corruption
- Only ONE GhostWire instance can run per profile at a time

**Why this happens:**
- You started GhostWire in another terminal
- A background process is still running
- The process didn't shut down cleanly (crash, kill -9, etc.)

---

## ✅ Prerequisites

### Check Rust Installation
```bash
rustc --version
cargo --version
```

**Required:**
- Rust 1.70+ 
- Cargo 1.70+

### Check Ports
```bash
# Check if ports are in use
netstat -tulpn | grep -E ':(3001|8080|4001)'
```

**Required ports:**
- **3001** or **8080** - Web UI / API
- **4001** - P2P mesh networking

---

## 🔧 Method 1: Quick Start (Recommended)

### Step 1: Kill All Running Instances

```bash
# Kill any GhostWire processes
pkill -9 ghostwire

# Wait for locks to release
sleep 3

# Verify nothing is running
ps aux | grep ghostwire
```

**Expected output:** No ghostwire processes (except the grep itself)

---

### Step 2: Remove Lock Files (If Needed)

```bash
# Remove the lock file
rm -rf 

# Or remove entire vault (will regenerate)
rm -rf 
```

---

### Step 3: Start GhostWire

```bash
cd 

# Start on port 8080 (default for web UI)
cargo run -- web --api-host 0.0.0.0 --mesh-host 0.0.0.0 --port 8080
```

---

### Step 4: Verify It's Running

You should see:
```
2026-02-16T16:40:00.000Z  INFO ghostwire: 🚀 Starting GhostWire...
2026-02-16T16:40:01.000Z  INFO ghostwire: ✅ Repository is up to date
2026-02-16T16:40:02.000Z  INFO ghostwire: 🔑 Loaded persistent identity
2026-02-16T16:40:03.000Z  INFO ghostwire: 🔐 Loaded encryption keys
2026-02-16T16:40:04.000Z  INFO ghostwire: 🌐 Using default bootstrap nodes
2026-02-16T16:40:05.000Z  INFO ghostwire: ✅ Mesh Network Auto-Started
Peer ID: 12D3KooW...
Listening on:
  /ip4/0.0.0.0/tcp/8080
```

**✅ Success!** Your API is now running on `http://localhost:9097`

---

## 🛠️ Method 2: Manual Start (Full Control)

### Step 1: Navigate to Project

```bash
cd 
```

---

### Step 2: Build (If Needed)

```bash
# Debug build (faster, includes debug info)
cargo build

# Release build (optimized, slower to build)
cargo build --release
```

---

### Step 3: Run with Options

```bash
# Basic start (port 8080)
cargo run -- web --port 8080

# Specify all options
cargo run -- web \
  --api-host 0.0.0.0 \
  --mesh-host 0.0.0.0 \
  --port 8080 \
  --profile default

# With custom bootstrap nodes
cargo run -- web --port 8080 \
  --bootstrap /dnsaddr/bootstrap.libp2p.io/p2p/QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN
```

---

### Step 4: Available Options

```bash
# Show help
cargo run -- web --help
```

**Options:**
- `--api-host <HOST>` - API bind address (default: 127.0.0.1)
- `--mesh-host <HOST>` - Mesh network bind address (default: 0.0.0.0)
- `--port <PORT>` - API port (default: 8080)
- `--bootstrap <ADDR>` - Bootstrap node addresses (multiple allowed)
- `--profile <NAME>` - Profile name (default: default)

---

## 📜 Method 3: Using Scripts

### Option A: start-backend.sh

```bash
cd 

# Make executable (first time only)
chmod +x start-backend.sh

# Run
./start-backend.sh
```

**What it does:**
- Checks for pre-built binary
- Falls back to debug binary
- Auto-builds if needed
- Starts on port 3001 by default

---

### Option B: Custom Script

Create your own startup script:

```bash
#!/bin/bash
# File: start-ghostwire.sh

set -e

echo "🚀 Starting GhostWire..."

# Configuration
PORT=${GHOSTWIRE_PORT:-8080}
API_HOST=${GHOSTWIRE_API_HOST:-0.0.0.0}
MESH_HOST=${GHOSTWIRE_MESH_HOST:-0.0.0.0}
PROFILE=${GHOSTWIRE_PROFILE:-default}

# Kill existing processes
pkill -9 ghostwire 2>/dev/null || true
sleep 2

# Start GhostWire
cd 
cargo run -- web \
  --api-host $API_HOST \
  --mesh-host $MESH_HOST \
  --port $PORT \
  --profile $PROFILE
```

Make executable:
```bash
chmod +x start-ghostwire.sh
```

Run:
```bash
./start-ghostwire.sh
```

---

## 🐛 Troubleshooting

### Issue 1: Database Lock Error

**Error:**
```
Error: IO error: could not acquire lock on "
```

**Solutions:**

**A. Find and Kill Process:**
```bash
# Find process
ps aux | grep ghostwire

# Kill it
kill -9 <PID>

# Or kill all
pkill -9 ghostwire
```

**B. Remove Lock File:**
```bash
rm 
```

**C. Wait:**
Sometimes locks release automatically after 30-60 seconds.

---

### Issue 2: Port Already in Use

**Error:**
```
Error: Address already in use (os error 98)
```

**Solutions:**

**A. Find Process Using Port:**
```bash
# Check port 8080
lsof -i :9097
netstat -tulpn | grep 8080
```

**B. Kill Process:**
```bash
kill -9 <PID>
```

**C. Use Different Port:**
```bash
cargo run -- web --port 8081
```

---

### Issue 3: Permission Denied

**Error:**
```
Permission denied (os error 13)
```

**Solutions:**

**A. Run as Regular User:**
```bash
# Don't use sudo unless necessary
cargo run -- web --port 8080
```

**B. Use Higher Port:**
```bash
# Ports < 1024 require root
cargo run -- web --port 8080  # OK
```

**C. Set Capabilities (Linux):**
```bash
sudo setcap 'cap_net_bind_service=+ep' target/debug/ghostwire
```

---

### Issue 4: Build Errors

**Error:**
```
error[E0432]: unresolved import
```

**Solutions:**

**A. Update Rust:**
```bash
rustup update stable
rustup default stable
```

**B. Clean Build:**
```bash
cargo clean
cargo build
```

**C. Check Dependencies:**
```bash
cargo update
cargo build
```

---

### Issue 5: Web UI Shows CORS Errors

**Symptoms:**
- Web UI loads but shows "Failed to fetch"
- Browser console shows CORS errors

**Solutions:**

**A. Start Backend:**
```bash
cargo run -- web --port 8080
```

**B. Check Port Match:**
- Web UI expects port 8080 by default
- Make sure backend runs on same port

**C. Update Web UI Config:**
```javascript
// In browser console (F12)
localStorage.setItem('ghostwire_backend_url', 'http://localhost:9097/api');
location.reload();
```

---

## ✅ Verification

### Test 1: API Health Check

```bash
curl http://localhost:9097/api/status
```

**Expected:**
```json
{
  "status": "ok",
  "version": "0.1.0"
}
```

---

### Test 2: Check Processes

```bash
ps aux | grep ghostwire
```

**Expected:**
```
ph  12345  0.5  0.2  1234567  89012  pts/0  Sl+  16:40   0:00  target/debug/ghostwire web ...
```

---

### Test 3: Check Ports

```bash
netstat -tulpn | grep ghostwire
```

**Expected:**
```
tcp  0  0 0.0.0.0:9097  0.0.0.0:*  LISTEN  12345/ghostwire
tcp  0  0 0.0.0.0:4001  0.0.0.0:*  LISTEN  12345/ghostwire
```

---

### Test 4: Web UI Access

Open browser: `http://localhost:3001` or `http://localhost:9097`

**Expected:**
- GhostWire login screen loads
- No CORS errors in console (F12)
- Can interact with UI

---

### Test 5: Postman Test

**In Postman:**
1. Import collection: `tests/api/GhostWire_API.postman_collection.json`
2. Open: `🔓 Auth & Bootstrap` → `Get Dev Token`
3. Click **Send**
4. Should get 200 OK with token

---

## 📊 Startup Sequence

When GhostWire starts, here's what happens:

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

**Typical startup time:** 3-5 seconds

---

## 🎯 Best Practices

### Do's ✅

- Always kill processes cleanly: `pkill ghostwire`
- Wait 2-3 seconds before restarting
- Use different profiles for testing: `--profile test1`
- Check logs for errors: `RUST_LOG=debug cargo run`
- Backup vault before major changes

### Don'ts ❌

- Don't run multiple instances on same profile
- Don't kill -9 unless necessary
- Don't delete vault while running
- Don't ignore lock errors
- Don't run as root unless necessary

---

## 📚 Additional Resources

- **[Deployment_Guide.md](../docs/Deployment_Guide.md)** - Production deployment
- **[Troubleshooting_Guide.md](../ghostwire/notebookllm/Troubleshooting_Guide.md)** - Common issues
- **[POSTMAN_API_GUIDE.md](../docs/POSTMAN_API_GUIDE.md)** - API testing

---

## 🆘 Quick Reference Commands

```bash
# Kill all instances
pkill -9 ghostwire

# Start on port 8080
cargo run -- web --api-host 0.0.0.0 --mesh-host 0.0.0.0 --port 8080

# Start on port 3001
cargo run -- web --api-host 0.0.0.0 --mesh-host 0.0.0.0 --port 3001

# Check if running
ps aux | grep ghostwire

# Check ports
netstat -tulpn | grep -E ':(8080|3001|4001)'

# Test API
curl http://localhost:9097/api/status

# View logs (debug mode)
RUST_LOG=debug cargo run -- web --port 8080
```

---

**Last Updated**: February 16, 2026  
**Version**: 0.1.0  
**Status**: ✅ Production-Ready

---

*This guide covers all methods to start GhostWire correctly. Follow Method 1 for the quickest setup.*
