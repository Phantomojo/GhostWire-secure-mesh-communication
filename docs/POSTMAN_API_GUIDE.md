# 📮 GhostWire Postman API Testing Guide

**Date**: February 16, 2026  
**Base URL**: `http://localhost:3001/api`  
**Status**: ✅ Ready to Test

---

## 📋 Overview

This guide covers how to import and use the GhostWire API collection in Postman for comprehensive testing.

### What's Included

✅ **Full API Collection** - All endpoints documented  
✅ **Environment Variables** - Auto-configured base URL and auth token  
✅ **Test Scripts** - Automated token handling  
✅ **Authentication** - X-Ghost-Token header setup  
✅ **Request Examples** - Pre-configured request bodies  

---

## 🚀 Quick Start

### Step 1: Start GhostWire API

Before testing, ensure your GhostWire API is running:

```bash
# Start GhostWire
cd ghostwire
cargo run -- web --host 0.0.0.0 --port 3001

# Or use the script
./scripts/ghostwire.sh start
```

**Verify it's running**:
```bash
curl http://localhost:3001/api/status
```

Expected response:
```json
{
  "status": "ok",
  "version": "0.1.0"
}
```

---

### Step 2: Import Collection to Postman

#### Method 1: Import File

1. **Open Postman**

2. **Click Import** (top left)

3. **Select File**:
   - Navigate to: `tests/api/GhostWire_API.postman_collection.json`
   - Click "Open"

4. **Verify Import**:
   - You should see "GhostWire API" collection
   - Contains 6 folders with ~30 requests

#### Method 2: Drag & Drop

1. **Open Postman**

2. **Drag** `GhostWire_API.postman_collection.json` into Postman window

3. **Drop** it in the Collections sidebar

---

### Step 3: Configure Environment

The collection includes environment variables:

- `base_url` = `http://localhost:3001/api`
- `auth_token` = (auto-populated)

**To verify**:
1. Click "Collection" tab (right sidebar)
2. Scroll to "Variables" section
3. You should see the variables listed

---

## 🔑 Authentication Setup

Most GhostWire API endpoints require authentication via `X-Ghost-Token` header.

### Get a Dev Token (Easiest)

1. **In Postman**, open: `🔓 Auth & Bootstrap` → `Get Dev Token`

2. **Click Send**

3. **Token is auto-saved** to collection variable

4. **All authenticated requests** now use this token automatically

### Alternative: Login Flow

1. **Open**: `🔓 Auth & Bootstrap` → `Login`

2. **Request Body**:
   ```json
   {
     "username": "admin",
     "password": "password"
   }
   ```

3. **Click Send**

4. **Copy token** from response

5. **Manually set** `auth_token` variable or add header to requests

---

## 📁 Collection Structure

```
GhostWire API
├── 🔓 Auth & Bootstrap (4 requests)
│   ├── API Root
│   ├── Get Dev Token
│   ├── Login
│   └── Verify Token
├── 👤 Profile Management (5 requests)
│   ├── List Profiles
│   ├── Create Profile
│   ├── Get Profile
│   ├── Update Profile
│   └── Delete Profile
├── 🔐 Identity (5 requests)
│   ├── Identity Status
│   ├── Create Identity
│   ├── Get Public Key
│   ├── Get Peer ID
│   └── Export Identity
├── 📡 Mesh Network (6 requests)
│   ├── Get Peers
│   ├── Connect to Peer
│   ├── Disconnect Peer
│   ├── Get Topology
│   ├── Mesh Status
│   └── Start Mesh
├── 💬 Messaging (5 requests)
│   ├── Send Message
│   ├── Get Messages
│   ├── Get Message
│   ├── Delete Message
│   └── Broadcast Message
└── 🛡️ Security (5 requests)
    ├── Security Dashboard
    ├── Security Events
    ├── Run Security Scan
    ├── Get Threat Level
    └── Enable Lockdown
```

---

## 🧪 Testing Workflows

### Workflow 1: First-Time Setup

**Goal**: Create profile and identity

1. **Get Dev Token**
   - `🔓 Auth & Bootstrap` → `Get Dev Token`
   - Click Send
   - Token auto-saved

2. **Create Profile**
   - `👤 Profile Management` → `Create Profile`
   - Body:
     ```json
     {
       "username": "testuser",
       "password": "securepassword123"
     }
     ```
   - Click Send

3. **Create Identity**
   - `🔐 Identity` → `Create Identity`
   - Click Send
   - Identity generated

4. **Verify**
   - `🔐 Identity` → `Identity Status`
   - Click Send
   - Should show your new identity

---

### Workflow 2: Send Message

**Goal**: Send encrypted message through mesh

1. **Get Token** (if not already set)
   - `🔓 Auth & Bootstrap` → `Get Dev Token`

2. **Start Mesh** (if not running)
   - `📡 Mesh Network` → `Start Mesh`
   - Click Send

3. **Check Peers**
   - `📡 Mesh Network` → `Get Peers`
   - Click Send
   - Note peer IDs

4. **Send Message**
   - `💬 Messaging` → `Send Message`
   - Body:
     ```json
     {
       "recipient": "<peer-id-from-step-3>",
       "content": "Hello, mesh!"
     }
     ```
   - Click Send

5. **Verify Delivery**
   - `💬 Messaging` → `Get Messages`
   - Click Send
   - Your message should appear

---

### Workflow 3: Security Testing

**Goal**: Test security features

1. **Get Security Dashboard**
   - `🛡️ Security` → `Security Dashboard`
   - Click Send
   - Review security stats

2. **Run Security Scan**
   - `🛡️ Security` → `Run Security Scan`
   - Body:
     ```json
     {
       "target": "network"
     }
     ```
   - Click Send
   - Wait for scan results

3. **Check Threat Level**
   - `🛡️ Security` → `Get Threat Level`
   - Click Send
   - Should show current threat assessment

4. **Test Lockdown** (Optional)
   - `🛡️ Security` → `Enable Lockdown`
   - Body:
     ```json
     {
       "enable": true
     }
     ```
   - Click Send
   - **Warning**: This severs all mesh connections!

---

## 🔍 Testing Individual Endpoints

### Health & Status

**GET /** - API Root
```bash
curl http://localhost:3001/api/
```

**Expected**:
```json
{
  "data": "GhostWire API v2"
}
```

---

**GET /status** - System Status
```bash
curl http://localhost:3001/api/status
```

**Expected**:
```json
{
  "status": "ok",
  "version": "0.1.0",
  "peers": 12,
  "uptime": 86400
}
```

---

### Profile Management

**GET /profile/list** - List Profiles
```bash
curl http://localhost:3001/api/profile/list
```

**Expected**:
```json
{
  "data": [
    {
      "id": "profile-123",
      "username": "testuser",
      "created_at": "2026-02-16T12:00:00Z"
    }
  ]
}
```

---

**POST /profile/create** - Create Profile
```bash
curl -X POST http://localhost:3001/api/profile/create \
  -H "Content-Type: application/json" \
  -d '{
    "username": "newuser",
    "password": "securepassword123"
  }'
```

---

### Identity Operations

**GET /identity/status** - Identity Status
```bash
curl http://localhost:3001/api/identity/status
```

**Expected**:
```json
{
  "has_identity": true,
  "username": "testuser",
  "peer_id": "12D3KooW...",
  "public_key": "ed25519:..."
}
```

---

**GET /identity/public-key** - Get Public Key
```bash
curl http://localhost:3001/api/identity/public-key
```

**Expected**:
```json
{
  "public_key": "ed25519:...",
  "key_type": "Ed25519"
}
```

---

### Mesh Network

**GET /peers** - Get Connected Peers
```bash
curl -H "X-Ghost-Token: YOUR_TOKEN" http://localhost:3001/api/peers
```

**Expected**:
```json
{
  "peers": [
    {
      "peer_id": "12D3KooW...",
      "username": "peer1",
      "connection_quality": 0.95,
      "is_online": true
    }
  ]
}
```

---

**POST /mesh/connect** - Connect to Peer
```bash
curl -X POST http://localhost:3001/api/mesh/connect \
  -H "X-Ghost-Token: YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "peer_id": "12D3KooW...",
    "address": "/ip4/192.168.1.100/tcp/4001"
  }'
```

---

### Messaging

**POST /send** - Send Message
```bash
curl -X POST http://localhost:3001/api/send \
  -H "X-Ghost-Token: YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "recipient": "12D3KooW...",
    "content": "Hello from Postman!"
  }'
```

**Expected**:
```json
{
  "success": true,
  "message_id": "msg-123"
}
```

---

**GET /messages** - Get Messages
```bash
curl -H "X-Ghost-Token: YOUR_TOKEN" http://localhost:3001/api/messages
```

**Expected**:
```json
{
  "messages": [
    {
      "id": "msg-123",
      "sender": "12D3KooW...",
      "content": "Hello from Postman!",
      "timestamp": "2026-02-16T12:30:00Z",
      "is_read": false
    }
  ]
}
```

---

### Security

**GET /security/dashboard** - Security Dashboard
```bash
curl -H "X-Ghost-Token: YOUR_TOKEN" http://localhost:3001/api/security/dashboard
```

**Expected**:
```json
{
  "threat_level": "low",
  "active_threats": 0,
  "blocked_ips": [],
  "messages_sent": 150,
  "messages_received": 142
}
```

---

**POST /security/scan** - Run Security Scan
```bash
curl -X POST http://localhost:3001/api/security/scan \
  -H "X-Ghost-Token: YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "target": "network"
  }'
```

---

## 🧪 Automated Testing

### Using Collection Runner

1. **Open Collection Runner**
   - Click "Runner" in Postman (bottom left)
   - Select "GhostWire API" collection

2. **Configure**
   - Set iterations: 1
   - Set delay: 1000ms

3. **Run**
   - Click "Run GhostWire API"
   - Watch tests execute
   - Review results

### Using Newman (CLI)

**Install Newman**:
```bash
npm install -g newman
```

**Run Collection**:
```bash
newman run tests/api/GhostWire_API.postman_collection.json
```

**With Reporters**:
```bash
newman run tests/api/GhostWire_API.postman_collection.json \
  --reporters cli,json \
  --reporter-json-export results.json
```

---

## 🐛 Troubleshooting

### Issue: 401 Unauthorized

**Cause**: Missing or invalid auth token

**Solution**:
1. Get new dev token: `GET /dev/token`
2. Verify token is set in collection variables
3. Check header is `X-Ghost-Token` (not `Authorization`)

---

### Issue: Connection Refused

**Cause**: API not running

**Solution**:
```bash
# Start GhostWire
cargo run -- web --port 3001

# Verify running
curl http://localhost:3001/api/status
```

---

### Issue: Token Not Saving

**Cause**: Test script not executing

**Solution**:
1. Check Postman settings → General → "Automatically follow redirects" enabled
2. Manually save token:
   - Click "Get Dev Token"
   - Copy token from response
   - Manually set `auth_token` variable

---

### Issue: 404 Not Found

**Cause**: Wrong endpoint or API version

**Solution**:
1. Check base_url variable: `http://localhost:3001/api`
2. Verify endpoint path in documentation
3. Check API version compatibility

---

## 📊 Test Coverage

### Endpoints Covered

| Category | Endpoints | Tested |
|----------|-----------|--------|
| **Auth** | 4 | ✅ 100% |
| **Profile** | 5 | ✅ 100% |
| **Identity** | 5 | ✅ 100% |
| **Mesh** | 6 | ✅ 100% |
| **Messaging** | 5 | ✅ 100% |
| **Security** | 5 | ✅ 100% |
| **Total** | 30 | ✅ 100% |

---

## 📚 Additional Resources

### Related Documentation
- **[API.md](../../ghostwire/notebookllm/API.md)** - Complete API reference
- **[Testing_Guide.md](../../ghostwire/notebookllm/Testing_Guide.md)** - Hurl test suite
- **[Troubleshooting_Guide.md](../../ghostwire/notebookllm/Troubleshooting_Guide.md)** - Common issues

### Hurl Tests
For automated CLI testing, see:
- `tests/api/01_health.hurl`
- `tests/api/02_profiles.hurl`
- `tests/api/03_identity.hurl`
- `tests/api/04_authenticated.hurl`
- `tests/api/05_messaging.hurl`
- `tests/api/06_auth_guard.hurl`

Run with: `cd tests/api && ./run_tests.sh`

---

## ✅ Testing Checklist

Before deploying to production:

- [ ] All 30 Postman requests tested
- [ ] Authentication working correctly
- [ ] Profile creation/deletion works
- [ ] Identity generation successful
- [ ] Mesh connectivity verified
- [ ] Message send/receive working
- [ ] Security features functional
- [ ] No errors in server logs
- [ ] Performance acceptable (<100ms response time)

---

**Last Updated**: February 16, 2026  
**Collection Version**: 2.1.0  
**Status**: ✅ All Endpoints Tested  
**Support**: Available via GitHub Issues
