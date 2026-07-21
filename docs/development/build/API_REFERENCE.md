# REST API Reference

Complete documentation of the GhostWire REST API.

**Base URL:** `http://localhost:9097/api`
**API Version:** v2
**Default Port:** 8080 (configurable via `--port` or `GHOSTWIRE_PORT`)

---

## Authentication

### Auth Methods

GhostWire supports two authentication methods:

1. **Token Auth** -- Send the `X-Ghost-Token` header with every request
2. **JWT Auth** -- Send `Authorization: Bearer <jwt>` header

### Getting a Token

**Debug builds only:**
```bash
curl http://localhost:9097/api/dev/token
# Response: {"version":"2.0","data":"<token>","timestamp":"...","request_id":"..."}
```

**All builds:**
```bash
curl -X POST http://localhost:9097/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"token": "<your-auth-token>"}'
# Response: {"version":"2.0","data":{"token":"<jwt>"},"timestamp":"...","request_id":"..."}
```

The auth token is stored at `~/.config/ghostwire/token` with permissions `0o600`.

### Public Endpoints (No Auth Required)

These endpoints do not require authentication:

| Endpoint | Method |
|----------|--------|
| `/api/dev/token` | GET |
| `/api/auth/login` | POST |
| `/api/status` | GET |
| `/api/v2/health` | GET |
| `/api/v2/ready` | GET |
| `/api/v2/live` | GET |
| `/api/profile/*` | GET |
| `/api/identity/*` | GET |
| `/api/v2/profile` | GET |
| `/api/ble/*` | GET, POST |

All other endpoints require the `X-Ghost-Token` or `Authorization: Bearer` header.

---

## Response Format

All v2 responses use a standardized envelope:

```json
{
  "version": "2.0",
  "timestamp": "2026-04-12T10:30:00Z",
  "request_id": "550e8400-e29b-41d4-a716-446655440000",
  "data": { ... },
  "pagination": {
    "total": 100,
    "page": 1,
    "per_page": 20,
    "pages": 5
  },
  "links": {
    "self": "/api/v2/resource"
  },
  "rate_limit": {
    "limit": 200,
    "remaining": 199,
    "reset": "2026-04-12T10:31:00Z"
  }
}
```

### Error Response

```json
{
  "version": "2.0",
  "timestamp": "2026-04-12T10:30:00Z",
  "request_id": "550e8400-e29b-41d4-a716-446655440000",
  "error": {
    "code": "INVALID_INPUT",
    "message": "Username must be 1-32 characters",
    "details": null,
    "help_url": null,
    "trace_id": null
  }
}
```

### Standard Error Codes

| Code | Meaning |
|------|---------|
| `PROFILE_LOAD_FAILED` | Could not load user profile |
| `INVALID_CREDENTIALS` | Wrong password or token |
| `CORE_INITIALIZATION_FAILED` | Core system failed to start |
| `UNAUTHORIZED` | Missing or invalid authentication |
| `NOT_FOUND` | Resource does not exist |
| `INTERNAL_ERROR` | Unexpected server error |
| `INVALID_INPUT` | Request validation failed |
| `IDENTITY_LOCKED` | Profile is locked, unlock first |
| `RATE_LIMITED` | Too many requests |

---

## Rate Limiting

| Endpoint Group | Rate Limit | Burst |
|---------------|-----------|-------|
| General API | 200 req/s per IP | 500 |
| Auth endpoints | 5 req/s per IP | 10 |
| WebSocket | No limit | N/A |

Rate limit headers are included in responses when applicable:

```
X-RateLimit-Limit: 200
X-RateLimit-Remaining: 195
X-RateLimit-Reset: 2026-04-12T10:31:00Z
```

---

## Endpoint Groups

### Health

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| GET | `/api/v2/health` | No | Health check |
| GET | `/api/v2/ready` | No | Readiness check |
| GET | `/api/v2/live` | No | Liveness check |

**GET /api/v2/health**
```bash
curl http://localhost:9097/api/v2/health
```
```json
{
  "version": "2.0",
  "data": {
    "status": "healthy",
    "version": "0.2.0",
    "message": "GhostWire API is running",
    "timestamp": "2026-04-12T10:30:00Z"
  }
}
```

---

### Identity & Profile

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| GET | `/api/v2/profile` | No | Get full profile/identity |
| PUT | `/api/v2/profile/username` | Yes | Update username |
| PUT | `/api/v2/profile/metadata` | Yes | Update profile metadata |
| GET | `/api/v2/profile/public-key` | No | Get public key |
| POST | `/api/v2/profile/lock` | Yes | Lock profile |
| POST | `/api/v2/profile/unlock` | Yes | Unlock profile |
| POST | `/api/identity/setup` | No | First-time identity setup |

**GET /api/v2/profile**
```bash
curl http://localhost:9097/api/v2/profile
```
```json
{
  "version": "2.0",
  "data": {
    "id": "a1b2c3d4...",
    "did": "did:ghost:abc123...",
    "username": "phantomojo",
    "nickname": "phantomojo",
    "bio": "",
    "public_key": "<base64-encoded-ed25519-pubkey>",
    "key_id": "key-1",
    "created_at": "2026-04-12T10:00:00Z",
    "dht_published": false,
    "security_score": 85,
    "setup_complete": true
  },
  "links": { "self": "/api/v2/profile" }
}
```

**POST /api/identity/setup**
```bash
curl -X POST http://localhost:9097/api/identity/setup \
  -H "Content-Type: application/json" \
  -d '{"nickname": "my-node", "password": "secure-password-123"}'
```

**POST /api/v2/profile/unlock**
```bash
curl -X POST http://localhost:9097/api/v2/profile/unlock \
  -H "X-Ghost-Token: $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"password": "your-password"}'
```

---

### Messaging

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| GET | `/api/v2/messages` | Yes | List messages |
| GET | `/api/messages` | Yes | List messages (v1 compat) |

**GET /api/v2/messages**
```bash
curl -H "X-Ghost-Token: $TOKEN" \
  "http://localhost:9097/api/v2/messages?peer_id=all&limit=50"
```

Query parameters:
- `peer_id` -- Filter by peer ID (default: `all`)
- `limit` -- Max messages to return (default: 50, max: 1000)

```json
{
  "version": "2.0",
  "data": [
    {
      "id": "msg-uuid",
      "sender": "sender-peer-id",
      "recipient": "my-peer-id",
      "content": "Hello!",
      "timestamp": 1712920200,
      "msg_type": "text"
    }
  ],
  "links": { "self": "/api/v2/messages" }
}
```

---

### Peers

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| GET | `/api/v2/peers` | Yes | List connected peers (paginated) |
| GET | `/api/peers` | Yes | List peers (v1 compat) |
| GET | `/api/v2/peers/nearby` | Yes | List nearby discoverable peers |
| POST | `/api/v2/peers/connect` | Yes | Request connection to peer |
| POST | `/api/v2/peers/requests` | Yes | List pending connection requests |
| POST | `/api/v2/peers/requests/respond` | Yes | Accept/reject connection request |
| DELETE | `/api/v2/peers/:id` | Yes | Disconnect from peer |
| POST | `/api/v2/peers/:id/block` | Yes | Block a peer |
| POST | `/api/v2/peers/ping` | Yes | Ping a peer |
| POST | `/api/connect_peer` | Yes | Request connect (v1 compat) |
| POST | `/api/disconnect_peer` | Yes | Disconnect (v1 compat) |
| POST | `/api/ping_peer` | Yes | Ping (v1 compat) |

**GET /api/v2/peers**
```bash
curl -H "X-Ghost-Token: $TOKEN" \
  "http://localhost:9097/api/v2/peers?page=1&per_page=20"
```

Query parameters:
- `page` -- Page number (default: 1)
- `per_page` -- Items per page (default: 20, max: 100)

```json
{
  "version": "2.0",
  "data": {
    "count": 3,
    "peers": []
  },
  "pagination": {
    "total": 3,
    "page": 1,
    "per_page": 20,
    "pages": 1
  },
  "links": { "self": "/api/v2/peers?page=1&per_page=20" }
}
```

---

### Connection Requests

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| GET | `/api/v2/peers/requests` | Yes | List pending requests |
| POST | `/api/v2/peers/requests/respond` | Yes | Accept or reject a request |

**POST /api/v2/peers/requests/respond**
```bash
curl -X POST http://localhost:9097/api/v2/peers/requests/respond \
  -H "X-Ghost-Token: $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"request_id": "req-uuid", "action": "accept"}'
```

---

### Trust

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| GET | `/api/v2/trust` | Yes | List all trusted peers |
| GET | `/api/v2/trust/:peer_id` | Yes | Get trust score for a peer |
| POST | `/api/v2/trust/:peer_id/verify` | Yes | Mark a peer as verified |
| POST | `/api/v2/trust/:peer_id/block` | Yes | Block a peer |

**GET /api/v2/trust/:peer_id**
```bash
curl -H "X-Ghost-Token: $TOKEN" http://localhost:9097/api/v2/trust/4tK9mN2p...
```
```json
{
  "version": "2.0",
  "data": {
    "peer_id": "4tK9mN2p...",
    "trust_score": 0.85,
    "record": { ... }
  }
}
```

---

### Knowledge Ark

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| GET | `/api/v2/ark/collections` | Yes | List knowledge collections |
| POST | `/api/v2/ark/collections` | Yes | Create a collection |
| POST | `/api/v2/ark/entries` | Yes | Add an entry |
| GET | `/api/v2/ark/entries/:cid` | Yes | Get entry by content ID |
| DELETE | `/api/v2/ark/entries/:cid` | Yes | Delete an entry |
| GET | `/api/v2/ark/search?q=...` | Yes | Search entries |
| GET | `/api/v2/ark/tags/:tag` | Yes | Search by tag |
| GET | `/api/v2/ark/stats` | Yes | Ark statistics |

**POST /api/v2/ark/entries**
```bash
curl -X POST http://localhost:9097/api/v2/ark/entries \
  -H "X-Ghost-Token: $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Emergency Procedures",
    "body": "In case of flood, move to high ground...",
    "collection": "safety",
    "tags": ["emergency", "flood"],
    "kind": "article",
    "access": "public"
  }'
```

**GET /api/v2/ark/search**
```bash
curl -H "X-Ghost-Token: $TOKEN" \
  "http://localhost:9097/api/v2/ark/search?q=emergency"
```

---

### Ghost Channels

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| GET | `/api/v2/channels` | Yes | List joined channels |
| POST | `/api/v2/channels` | Yes | Create a channel |
| POST | `/api/v2/channels/:id/join` | Yes | Join a channel |
| POST | `/api/v2/channels/:id/leave` | Yes | Leave a channel |
| POST | `/api/v2/channels/:id/publish` | Yes | Publish a message |
| GET | `/api/v2/channels/:id/messages` | Yes | Get channel messages |

**POST /api/v2/channels**
```bash
curl -X POST http://localhost:9097/api/v2/channels \
  -H "X-Ghost-Token: $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"name": "general"}'
```
```json
{
  "version": "2.0",
  "data": {
    "channel": { ... },
    "invite": "gw-ch:abc123...",
    "topic": "ghostwire-channel-<hex-topic-id>"
  }
}
```

**POST /api/v2/channels/:id/publish**
```bash
curl -X POST http://localhost:9097/api/v2/channels/<channel-id>/publish \
  -H "X-Ghost-Token: $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"text": "Hello channel!"}'
```

---

### Device Pairing

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| POST | `/api/v2/pair/generate` | Yes | Generate pairing code |
| GET | `/api/v2/pair/contact-card` | Yes | Get contact card |
| GET | `/api/v2/pair/rendezvous` | Yes | Get DHT rendezvous address |

**POST /api/v2/pair/generate**
```bash
curl -X POST http://localhost:9097/api/v2/pair/generate \
  -H "X-Ghost-Token: $TOKEN"
```
```json
{
  "version": "2.0",
  "data": {
    "code": "ABCD-EFGH-JKLM-NPQR",
    "safety_number": "123456",
    "session_id": "session-uuid",
    "expires_in_secs": 600
  }
}
```

---

### Recovery (Shamir Secret Sharing)

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| POST | `/api/v2/recovery/setup` | Yes | Create recovery shares |
| GET | `/api/v2/recovery/my-shares` | Yes | List my recovery shares |
| POST | `/api/v2/recovery/reconstruct` | Yes | Reconstruct key from shares |

**POST /api/v2/recovery/setup**
```bash
curl -X POST http://localhost:9097/api/v2/recovery/setup \
  -H "X-Ghost-Token: $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "threshold": 3,
    "total": 5,
    "guardians": [
      {
        "peer_id": "guardian-peer-id",
        "name": "Alice",
        "x25519_pub_hex": "abc123..."
      }
    ]
  }'
```

---

### Network & Mesh Health

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| GET | `/api/v2/network/status` | Yes | Network status |
| GET | `/api/v2/network/qr-contact` | Yes | Generate contact QR code |
| POST | `/api/v2/network/scan-qr` | Yes | Parse a contact QR code |

**GET /api/v2/network/status**
```bash
curl -H "X-Ghost-Token: $TOKEN" http://localhost:9097/api/v2/network/status
```
```json
{
  "version": "2.0",
  "data": {
    "internet_reachable": true,
    "latency_ms": 15,
    "packet_loss_pct": 0.5,
    "active_peers": 3,
    "last_update": 1712920200,
    "transport_mode": "websocket",
    "network_state": "full_mesh"
  }
}
```

---

### Security Dashboard

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| GET | `/api/v2/security/dashboard` | Yes | Security metrics |

**GET /api/v2/security/dashboard**
```bash
curl -H "X-Ghost-Token: $TOKEN" http://localhost:9097/api/v2/security/dashboard
```
```json
{
  "version": "2.0",
  "data": {
    "score": 85,
    "encryption": {
      "method": "AES-256-GCM",
      "active": true,
      "messages_encrypted": 1234
    },
    "sybil_defense": {
      "method": "Proof-of-Work",
      "difficulty": 16,
      "verified_peers": 12
    },
    "rate_limiting": {
      "active": true,
      "violations_today": 0
    },
    "blacklist": {
      "entries": 0,
      "recent_additions": 0
    },
    "last_scan": "2026-04-12T10:30:00Z"
  },
  "links": { "self": "/api/v2/security/dashboard" }
}
```

---

### System & Settings

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| GET | `/api/status` | No | System status |
| GET | `/api/settings` | Yes | Get settings |
| PUT | `/api/settings` | Yes | Update settings |
| GET | `/api/system/stats` | Yes | System statistics |
| GET | `/api/system/audit` | Yes | Security audit log |

**GET /api/status**
```bash
curl http://localhost:9097/api/status
```
```json
{
  "version": "2.0",
  "data": {
    "status": "OPERATIONAL",
    "uptime_seconds": 3600,
    "local_ip": "192.168.1.100",
    "peer_count": 3,
    "mesh_active": true,
    "threat_level": "Low",
    "blocked_connections": 0,
    "timestamp": "2026-04-12T10:30:00Z"
  }
}
```

---

### BLE Transport

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| GET | `/api/ble/status` | No | BLE transport status |
| POST | `/api/ble/connect` | No | Connect to BLE device |

---

## WebSocket Events

Connect to `ws://localhost:9097/ws` for real-time event streaming.

```bash
wscat -c ws://localhost:9097/ws
```

### Event Types

The WebSocket forwards `CoreEvent` variants as JSON:

```json
{
  "NewMessage": {
    "id": "msg-uuid",
    "sender": "sender-peer-id",
    "recipient": "my-peer-id",
    "content": "Hello!",
    "timestamp": 1712920200,
    "msg_type": "text"
  }
}
```

```json
{
  "PeerConnected": "4tK9mN2pQ7rS8vW1xY3zA5bC6dE7fG8h"
}
```

```json
{
  "PeerDisconnected": "4tK9mN2pQ7rS8vW1xY3zA5bC6dE7fG8h"
}
```

```json
{
  "NetworkUpdate": {
    "active_peers": 5,
    "latency_ms": 12
  }
}
```

```json
{
  "SystemAlert": "High packet loss detected on transport"
}
```

```json
{
  "SecurityLockdown": true
}
```

---

## Security Headers

All API responses include these security headers:

```
Strict-Transport-Security: max-age=31536000; includeSubDomains
X-Frame-Options: DENY
X-Content-Type-Options: nosniff
X-XSS-Protection: 1; mode=block
Referrer-Policy: strict-origin-when-cross-origin
```

---

## CORS

CORS is configured to allow origins from:
- `localhost` / `127.0.0.1`
- `192.168.x.x` (local network)
- `172.x.x.x` (Docker/Podman networks)

Allowed methods: `GET`, `POST`, `PUT`, `DELETE`, `OPTIONS`

Allowed headers: `Content-Type`, `Authorization`, `Accept`, `Origin`, `X-Ghost-Token`

---

## Next Steps

- **[Quick Start](./QUICKSTART.md)** -- Get started with the GhostWire library
- **[Embedding Guide](./EMBEDDING.md)** -- Integrate GhostWire into your app
- **[API Security](../API_SECURITY.md)** -- Security architecture details
