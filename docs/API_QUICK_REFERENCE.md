# 📮 GhostWire API Quick Reference Card

**For**: Postman, curl, or any HTTP client  
**Base URL**: `http://localhost:3001/api`  
**Auth Header**: `X-Ghost-Token: <your-token>`

---

## 🔑 Quick Start (3 Steps)

```bash
# 1. Get Dev Token
curl http://localhost:3001/api/dev/token
# Response: {"data": "eyJhbGciOiJIUzI1NiIs..."}

# 2. Save Token
export GHOSTWIRE_TOKEN="eyJhbGciOiJIUzI1NiIs..."

# 3. Test Authenticated Endpoint
curl -H "X-Ghost-Token: $GHOSTWIRE_TOKEN" http://localhost:3001/api/peers
```

---

## 📋 Essential Endpoints

### 🔓 Authentication (No Auth Required)

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/` | GET | API health check |
| `/status` | GET | System status |
| `/dev/token` | GET | Get development token |
| `/auth/login` | POST | Login with credentials |
| `/profile/list` | GET | List all profiles |

---

### 👤 Profile Management

| Endpoint | Method | Auth | Body |
|----------|--------|------|------|
| `/profile/create` | POST | ❌ | `{"username": "...", "password": "..."}` |
| `/profile/{id}` | GET | ✅ | - |
| `/profile/{id}` | PUT | ✅ | `{"username": "..."}` |
| `/profile/{id}` | DELETE | ✅ | - |

---

### 🔐 Identity

| Endpoint | Method | Auth | Purpose |
|----------|--------|------|---------|
| `/identity/status` | GET | ❌ | Check if identity exists |
| `/identity/create` | POST | ✅ | Generate new identity |
| `/identity/public-key` | GET | ❌ | Get public key |
| `/identity/peer-id` | GET | ❌ | Get Peer ID |
| `/identity/export` | POST | ✅ | Export identity (encrypted) |

---

### 📡 Mesh Network

| Endpoint | Method | Auth | Body |
|----------|--------|------|------|
| `/peers` | GET | ✅ | - |
| `/mesh/connect` | POST | ✅ | `{"peer_id": "...", "address": "..."}` |
| `/mesh/disconnect` | POST | ✅ | `{"peer_id": "..."}` |
| `/mesh/topology` | GET | ✅ | - |
| `/mesh/status` | GET | ✅ | - |
| `/mesh/start` | POST | ✅ | - |

---

### 💬 Messaging

| Endpoint | Method | Auth | Body |
|----------|--------|------|------|
| `/send` | POST | ✅ | `{"recipient": "...", "content": "..."}` |
| `/messages` | GET | ✅ | - |
| `/messages/{id}` | GET | ✅ | - |
| `/messages/{id}` | DELETE | ✅ | - |
| `/broadcast` | POST | ✅ | `{"content": "..."}` |

---

### 🛡️ Security

| Endpoint | Method | Auth | Body |
|----------|--------|------|------|
| `/security/dashboard` | GET | ✅ | - |
| `/security/events` | GET | ✅ | - |
| `/security/scan` | POST | ✅ | `{"target": "network"}` |
| `/security/threat-level` | GET | ✅ | - |
| `/security/lockdown` | POST | ✅ | `{"enable": true/false}` |

---

## 🔧 curl Examples

### Get System Status
```bash
curl http://localhost:3001/api/status
```

### Get Dev Token
```bash
curl http://localhost:3001/api/dev/token
```

### Send Message
```bash
curl -X POST http://localhost:3001/api/send \
  -H "X-Ghost-Token: YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"recipient": "12D3KooW...", "content": "Hello!"}'
```

### Get Connected Peers
```bash
curl -H "X-Ghost-Token: YOUR_TOKEN" http://localhost:3001/api/peers
```

### Run Security Scan
```bash
curl -X POST http://localhost:3001/api/security/scan \
  -H "X-Ghost-Token: YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"target": "network"}'
```

---

## 📊 Response Formats

### Success Response
```json
{
  "success": true,
  "data": {...},
  "message": "Operation completed"
}
```

### Error Response
```json
{
  "success": false,
  "error": "Error message",
  "code": 400
}
```

### Authentication Error
```json
{
  "error": "Invalid or missing authentication token",
  "code": 401
}
```

---

## 🔍 Status Codes

| Code | Meaning | When |
|------|---------|------|
| 200 | OK | Request succeeded |
| 201 | Created | Resource created (e.g., profile) |
| 400 | Bad Request | Invalid input |
| 401 | Unauthorized | Missing/invalid token |
| 403 | Forbidden | Insufficient permissions |
| 404 | Not Found | Resource not found |
| 500 | Server Error | Internal server error |

---

## 🧪 Testing Checklist

**Quick Test Sequence**:

1. ✅ `GET /` - API is running
2. ✅ `GET /dev/token` - Get auth token
3. ✅ `GET /identity/status` - Check identity
4. ✅ `POST /identity/create` - Create if needed
5. ✅ `GET /peers` - Check mesh connections
6. ✅ `POST /send` - Send test message
7. ✅ `GET /messages` - Verify message received
8. ✅ `GET /security/dashboard` - Check security status

---

## 🐛 Common Issues

### 401 Unauthorized
```bash
# Get new token
curl http://localhost:3001/api/dev/token

# Use correct header format
curl -H "X-Ghost-Token: YOUR_TOKEN" http://localhost:3001/api/peers
```

### Connection Refused
```bash
# Start GhostWire
cargo run -- web --port 3001

# Verify running
curl http://localhost:3001/api/status
```

### 404 Not Found
```bash
# Check base URL
# Should be: http://localhost:3001/api
# Not: http://localhost:3001/api/v1 (no v1 prefix)
```

---

## 📚 Full Documentation

- **[POSTMAN_API_GUIDE.md](POSTMAN_API_GUIDE.md)** - Complete Postman guide
- **[API.md](../../ghostwire/notebookllm/API.md)** - Full API reference
- **[Testing_Guide.md](../../ghostwire/notebookllm/Testing_Guide.md)** - Hurl test suite

---

**Quick Reference v1.0** | **February 2026** | **GhostWire v0.1.0**
