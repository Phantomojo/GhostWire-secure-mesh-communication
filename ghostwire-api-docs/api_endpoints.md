# GhostWire API Documentation

## Overview
This document outlines all API endpoints available in GhostWire for UI consumption.

## API Version 2 Endpoints (from api_v2_endpoints.rs)

### Profile Management
- **GET** `/api/v2/profile` - Get full user profile/identity
- **PUT** `/api/v2/profile/username` - Update username
- **PUT** `/api/v2/profile/metadata` - Update profile metadata (nickname, bio, avatar_url)
- **GET** `/api/v2/profile/public-key` - Get public key with QR code
- **POST** `/api/v2/profile/lock` - Lock profile
- **POST** `/api/v2/profile/unlock` - Unlock profile
- **PUT** `/api/v2/profile/save` - Save profile metadata (alias for update_metadata)

### Security
- **PUT** `/api/v2/security/armor` - Toggle Post-Quantum Armor (PQC)
- **GET** `/api/v2/security/dashboard` - Get security dashboard stats

### Mesh Networking
- **PUT** `/api/v2/mesh/sharding` - Update DHT sharding region
- **GET** `/api/v2/peers` - Get paginated peer list
- **GET** `/api/v2/peers/nearby` - Get nearby peers
- **POST** `/api/v2/peers/connect` - Send peer connection request
- **GET** `/api/v2/peers/requests` - Get pending peer requests
- **POST** `/api/v2/peers/requests/respond` - Respond to peer request
- **DELETE** `/api/v2/peers/{id}` - Disconnect from peer
- **POST** `/api/v2/peers/{id}/block` - Block a peer
- **POST** `/api/v2/peers/ping` - Ping a peer
- **GET** `/api/v2/peers/{id}/profile` - Get peer profile

### Messaging
- **GET** `/api/v2/messages` - Get historical message list
- **POST** `/api/v2/messages/send` - Send a message

### Trust System
- **GET** `/api/v2/trust` - List trusted peers
- **GET** `/api/v2/trust/{peer_id}` - Get trust level for specific peer
- **POST** `/api/v2/trust/{peer_id}/verify` - Mark peer as verified
- **POST** `/api/v2/trust/{peer_id}/block` - Block peer in trust system

### Knowledge Ark
- **GET** `/api/v2/ark/collections` - List knowledge collections
- **POST** `/api/v2/ark/collections` - Create knowledge collection
- **POST** `/api/v2/ark/entries` - Create knowledge entry
- **GET** `/api/v2/ark/entries/{cid}` - Get knowledge entry
- **DELETE** `/api/v2/ark/entries/{cid}` - Delete knowledge entry
- **GET** `/api/v2/ark/search` - Search knowledge base
- **GET** `/api/v2/ark/tags/{tag}` - Search by tag
- **GET** `/api/v2/ark/stats` - Get ark statistics

### Ghost Channels
- **GET** `/api/v2/channels` - List channels
- **POST** `/api/v2/channels` - Create channel
- **POST** `/api/v2/channels/{id}/join` - Join channel
- **POST** `/api/v2/channels/{id}/leave` - Leave channel
- **POST** `/api/v2/channels/{id}/publish` - Publish to channel
- **GET** `/api/v2/channels/{id}/messages` - Get channel messages

### Recovery (Shamir Secret Sharing)
- **POST** `/api/v2/recovery/setup` - Setup recovery shares
- **GET** `/api/v2/recovery/my-shares` - List my recovery shares
- **POST** `/api/v2/recovery/reconstruct` - Reconstruct key from shares

### Network / Mesh Health
- **GET** `/api/v2/network/status` - Get network status
- **GET** `/api/v2/network/qr-contact` - Generate contact QR code
- **POST** `/api/v2/network/scan-qr` - Scan contact QR code

### Pairing
- **POST** `/api/v2/pair/generate` - Generate pairing code
- **GET** `/api/v2/pair/contact-card` - Get contact card for pairing
- **GET** `/api/v2/pair/rendezvous` - Get rendezvous address

### Health & System
- **GET** `/v2/health` - Health check
- **GET** `/v2/ready` - Readiness check
- **GET** `/v2/live` - Liveness check
- **GET** `/v2/system/mesh-health` - Mesh health status
- **GET** `/v2/profile/status` - Profile status
- **GET** `/v2/settings` - Get system settings
- **PUT** `/v2/settings` - Update system settings

## Authentication Endpoints (from web.rs)

### Auth Routes (nested under /auth)
- **POST** `/api/auth/login` - Login with token
- **POST** `/api/auth/verify` - Verify JWT token
- **GET** `/api/auth/dev-token` - Get development token (localhost only, debug builds)

### Identity Routes (nested under /identity)
- **GET** `/api/identity/` - Get current profile
- **GET** `/api/identity/status` - Get profile status
- **POST** `/api/identity/setup` - Setup new profile
- **GET** `/api/identity/public-key` - Get public key

### Profile Routes (nested under /profile)
- **GET** `/api/profile/list` - List profiles
- **POST** `/api/profile/create` - Create new profile
- **POST** `/api/profile/select` - Select profile
- **DELETE** `/api/profile/delete/{name}` - Delete profile

### System Routes (nested under /system)
- **GET** `/api/system/stats` - Get system statistics
- **GET** `/api/system/audit` - Get security audit
- **POST** `/api/system/wipe` - Wipe all profiles
- **PUT** `/api/system/rotate-keys` - Rotate encryption keys
- **POST** `/api/system/lockdown` - Enable lockdown mode
- **POST** `/api/system/backup` - Backup system
- **GET** `/api/system/logs` - Get system logs
- **POST** `/api/system/upgrade-encryption` - Upgrade encryption
- **POST** `/api/system/configure-firewall` - Configure firewall
- **POST** `/api/system/test-firewall` - Test firewall
- **GET** `/api/system/users` - Get auth users
- **POST** `/api/system/security-scan` - Run security scan
- **POST** `/api/system/threat-hunt` - Run threat hunt
- **POST** `/api/system/backup-security` - Backup security data
- **POST** `/api/system/analyze-communications` - Analyze communications

## WebSocket Endpoint
- **GET** `/api/ws` - WebSocket connection for real-time updates

## Standard API Response Format (from api_v2_types.rs)

All API v2 endpoints return responses in the following format:

```json
{
  "version": "2.0",
  "timestamp": "ISO 8601 timestamp",
  "request_id": "UUID",
  "data": { /* endpoint-specific data */ },
  "error": {
    "code": "ERROR_CODE",
    "message": "Human readable message",
    "details": {/* optional validation details */},
    "help_url": "Link to documentation",
    "trace_id": "Support trace ID"
  },
  "pagination": {
    "total": "Total items",
    "page": "Current page",
    "per_page": "Items per page",
    "pages": "Total pages"
  },
  "links": {
    "self": "Current endpoint URL",
    "next": "Next page URL (if applicable)",
    "prev": "Previous page URL (if applicable)",
    "related": {/* related resource links */}
  },
  "rate_limit": {
    "limit": "Request limit",
    "remaining": "Remaining requests",
    "reset": "ISO 8601 timestamp when limit resets"
  }
}
```

## Standard Error Codes
- PROFILE_LOAD_FAILED
- INVALID_CREDENTIALS
- CORE_INITIALIZATION_FAILED
- UNAUTHORIZED
- NOT_FOUND
- INTERNAL_ERROR
- INVALID_INPUT
- IDENTITY_LOCKED
- RATE_LIMITED

## Data Models (from api_v2_types.rs and endpoints)

### ProfileResponse
- id: String
- did: String (Decentralized Identifier)
- username: String
- nickname: String
- bio: String
- public_key: String (base64 encoded)
- key_id: String
- created_at: String (ISO 8601)
- dht_published: boolean
- security_score: u8 (0-100)

### SecurityDashboard
- score: u8 (0-100)
- encryption: {
    method: String,
    active: boolean,
    messages_encrypted: u64
  }
- sybil_defense: {
    method: String,
    difficulty: u8,
    verified_peers: u32
  }
- rate_limiting: {
    active: boolean,
    violations_today: u32
  }
- blacklist: {
    entries: u32,
    recent_additions: u32
  }
- last_scan: String (ISO 8601)

### PeerResponse
- id: String
- username: String
- status: "online" | "offline"
- last_seen: String (ISO 8601)
- trust_score: u8
- public_key: String (base64 encoded)

### Message (structure inferred from usage)
- id: String
- content: String
- timestamp: String
- sender_id: String
- recipient_id: String (or "broadcast")
- type: String (text, file, etc.)
- encrypted: boolean

### LoginRequest
- token: String (authentication token)

### LoginResponse
- token: String (JWT token)

## Web Interface Notes (from web.rs)

### Security Features
- Rate limiting on all endpoints
- CORS protection
- CSWSH (Cross-Site WebSocket Hijacking) protection
- Security headers: HSTS, X-Frame-Options, CSP, etc.
- Request body limit: 10MB
- Timeout: 30 seconds
- Response compression enabled

### Authentication Middleware
- All `/api` routes require authentication via JWT token in Authorization header
- Token validation uses HS256 algorithm
- Dev token endpoint restricted to localhost/Docker IPs in debug builds only

### Static File Serving
- Serves static assets from compiled frontend
- Fallback to index.html for SPA routing

## Real-time Features
- WebSocket endpoint at `/api/ws` for live updates
- Events likely include: new messages, peer status changes, security alerts, etc.

## UI Design Implications

Based on the API, the UI should include:

1. **Authentication Screen**
   - Token input field (for dev/testing)
   - Secure storage of JWT token

2. **Profile Management**
   - Display: username, nickname, bio, public key (with QR), DID
   - Edit profile: username, nickname, bio, avatar URL
   - Security score display
   - Profile lock/unlock functionality

3. **Security Dashboard**
   - Overall security score (0-100)
   - Encryption status (method, active, messages encrypted)
   - Sybil defense status (method, difficulty, verified peers)
   - Rate limiting violations
   - Blacklist entries
   - Last security scan timestamp

4. **Mesh Networking**
   - Peer list with pagination/search
   - Peer status indicators (online/offline)
   - Trust scores
   - Peer profile viewer
   - Connection request management (send/respond/cancel)
   - Block/unblock peer functionality
   - Nearby peers discovery
   - DHT sharding configuration

5. **Messaging**
   - Message history viewer
   - Message composition interface
   - Send message functionality
   - Peer selection for direct messages
   - Broadcast messaging

6. **Trust System**
   - Trust level management
   - Peer verification
   - Trust-based blocking

7. **Knowledge Ark**
   - Collection browser/manager
   - Entry creation/viewing/search
   - Tag-based filtering

8. **Ghost Channels**
   - Channel list
   - Channel creation/joining/leaving
   - Channel messaging
   - Channel publishing/subscribing

9. **Recovery**
   - Shamir Secret Sharing setup
   - Share management
   - Key reconstruction

10. **Network Diagnostics**
    - Network status display
    - QR contact generation/scanning
    - Ping/pairing functionality

11. **System Settings**
    - General configuration
    - Encryption settings
    - Firewall configuration
    - Security scan controls
    - Backup/restore functionality

12. **Real-time Updates**
    - WebSocket connection for live peer/status updates
    - Notification system for events
    - Auto-refreshing data displays

13. **Mobile/Desktop Responsiveness**
    - Touch-friendly controls
    - Adaptive layouts
    - Offline capability indicators

This API documentation should provide a comprehensive foundation for designing the GhostWire UI.
