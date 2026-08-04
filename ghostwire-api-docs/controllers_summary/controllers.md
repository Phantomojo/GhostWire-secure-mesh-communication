# GhostWire Controllers Summary

This document summarizes the various controllers in the GhostWire backend and their responsibilities.

## Controller Files

1. **ark.rs** - Knowledge Ark functionality
   - Collections management
   - Entry creation and retrieval
   - Search and tagging
   - Statistics

2. **auth.rs** - Authentication
   - Login/logout
   - Token validation
   - Dev token generation (debug only)

3. **channel.rs** - Ghost Channels
   - Channel creation/management
   - Join/leave channels
   - Publish/subscribe messages
   - Channel membership

4. **health.rs** - Health endpoints
   - Basic health checks
   - Ready/live probes

5. **identity.rs** - User identity and profile
   - Profile creation/listing/deletion
   - Profile selection
   - Public key retrieval
   - Profile locking/unlocking
   - Metadata updates (nickname, bio, avatar)
   - Username updates

6. **messaging.rs** - Messaging functionality
   - Send/receive messages
   - Message history
   - Broadcasting

7. **network.rs** - Network and mesh health
   - Network status
   - QR contact generation/scanning
   - Peer pinging
   - Pairing functionality

8. **peer_controller.rs** & **peer.rs** - Peer management
   - Peer listing (nearby, all)
   - Connection requests
   - Peer blocking
   - Profile retrieval for peers
   - Disconnection

9. **profiles_debug.rs** - Debug profile endpoints (likely for development)

10. **profiles.rs** - Profile management (similar to identity.rs but possibly different implementation)

11. **recovery.rs** - Shamir Secret Sharing recovery
    - Share setup
    - Share listing
    - Key reconstruction

12. **system.rs** - System administration
    - System stats
    - Security audit
    - Key rotation
    - Lockdown mode
    - System backup
    - Log retrieval
    - Encryption upgrades
    - Firewall configuration
    - Security scans
    - Threat hunting
    - Communication analysis
    - User management
    - Settings

13. **trust.rs** - Trust management
    - Peer trust listing
    - Trust verification
    - Peer blocking

14. **ws_handler.rs** - WebSocket handling
    - Real-time communication
    - Event broadcasting

## Key Controller Responsibilities

### Authentication (auth.rs)
- Validates authentication tokens
- Issues JWT tokens for API access
- Provides development tokens in debug builds

### Identity & Profile (identity.rs)
- Manages user identity profiles
- Handles profile CRUD operations
- Manages public keys and DIDs
- Controls profile locking for security

### Messaging (messaging.rs)
- Sends and receives direct messages
- Manages message history
- Supports broadcasting to all peers

### Peer Management (peer_controller.rs, peer.rs)
- Discovers and lists nearby peers
- Manages connection requests
- Handles peer blocking/trust
- Retrieves peer profiles

### Network (network.rs)
- Monitors network connectivity
- Generates/scans QR codes for contact sharing
- Implements peer pinging
- Handles pairing protocols

### Knowledge Ark (ark.rs)
- Manages knowledge collections
- Stores and retrieves knowledge entries
- Provides search and tagging capabilities
- Tracks usage statistics

### Ghost Channels (channel.rs)
- Creates and manages communication channels
- Handles channel membership
- Manages message publishing/subscription
- Supports both public and private channels

### Recovery (recovery.rs)
- Implements Shamir's Secret Sharing
- Manages share distribution
- Enables key reconstruction from shares

### System (system.rs)
- Provides system monitoring and administration
- Handles security audits and scans
- Manages encryption and keys
- Controls system lockdown/backup
- Manages firewall settings
- Processes user administration

### Trust (trust.rs)
- Manages trust relationships between peers
- Tracks verified/trusted peers
- Implements trust-based blocking mechanisms

### WebSocket (ws_handler.rs)
- Maintains real-time connections
- Broadcasts events to connected clients
- Handles message streaming and presence updates

## Inter-Controller Relationships

1. **Identity <-> Auth**: Profiles require authentication tokens
2. **Peer <-> Identity**: Peer profiles reference user identities
3. **Messaging <-> Peer**: Messages are sent to/from specific peers
4. **Network <-> Peer**: Network status affects peer connectivity
5. **Channel <-> Peer**: Channels consist of peer members
6. **Trust <-> Peer**: Trust scores modify peer interactions
7. **Ark <-> Identity**: Knowledge entries are owned by users
8. **Recovery <-> Identity**: Recovery shares protect identity keys
9. **System <-> All**: System settings affect all other components
10. **WebSocket <-> All**: WebSocket broadcasts updates from all controllers

This modular architecture allows for clear separation of concerns while enabling rich, interconnected functionality in the GhostWire mesh network.
