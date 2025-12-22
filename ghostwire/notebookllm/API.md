# GhostWire API Documentation

## Overview

This document provides comprehensive API documentation for GhostWire, covering all public interfaces, data structures, and usage examples.

## Core API

### Identity Management

#### `Identity`

The core identity structure for GhostWire users.

```rust
pub struct Identity {
    pub id: String,
    pub username: String,
    pub public_key: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub is_ephemeral: bool,
}
```

**Methods:**

- `new(username: String) -> Result<Self>`: Create a new identity
- `from_keypair(keypair: KeyPair) -> Self`: Create identity from existing keypair
- `verify_proof(&self, proof: &Proof) -> bool`: Verify identity proof
- `generate_proof(&self) -> Proof`: Generate identity proof

#### `KeyPair`

Cryptographic key pair for encryption and signing.

```rust
pub struct KeyPair {
    pub signing_key: Ed25519KeyPair,
    pub encryption_key: X25519StaticSecret,
}
```

**Methods:**

- `generate() -> Self`: Generate new key pair
- `from_seed(seed: &[u8]) -> Result<Self>`: Create from seed
- `public_key(&self) -> PublicKey`: Get public key
- `sign(&self, message: &[u8]) -> Signature`: Sign message
- `verify(&self, message: &[u8], signature: &Signature) -> bool`: Verify signature

### Encryption

#### `EncryptionEngine`

Core encryption functionality.

```rust
pub struct EncryptionEngine {
    algorithm: EncryptionAlgorithm,
    key_derivation_iterations: u32,
}
```

**Methods:**

- `new() -> Self`: Create new encryption engine
- `encrypt(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>>`: Encrypt data
- `decrypt(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>>`: Decrypt data
- `derive_key(&self, password: &str, salt: &[u8]) -> Result<Vec<u8>>`: Derive key from password

#### `Message`

Encrypted message structure.

```rust
pub struct Message {
    pub id: String,
    pub sender: String,
    pub recipient: String,
    pub content: Vec<u8>,
    pub timestamp: DateTime<Utc>,
    pub message_type: MessageType,
    pub signature: Option<Vec<u8>>,
}
```

**Methods:**

- `new(sender: String, recipient: String, content: Vec<u8>) -> Self`: Create new message
- `encrypt(&self, engine: &EncryptionEngine, key: &[u8]) -> Result<Vec<u8>>`: Encrypt message
- `decrypt(&self, engine: &EncryptionEngine, key: &[u8]) -> Result<Vec<u8>>`: Decrypt message
- `sign(&self, keypair: &KeyPair) -> Result<()>`: Sign message
- `verify_signature(&self, public_key: &[u8]) -> bool`: Verify message signature

### Mesh Networking

#### `MeshNetwork`

Core mesh networking functionality.

```rust
pub struct MeshNetwork {
    pub local_peer_id: PeerId,
    pub swarm: Swarm<MeshBehaviour>,
    pub event_sender: mpsc::Sender<MeshEvent>,
}
```

**Methods:**

- `new(identity: &Identity) -> Result<Self>`: Create new mesh network
- `start(&mut self) -> Result<()>`: Start mesh networking
- `stop(&mut self) -> Result<()>`: Stop mesh networking
- `send_message(&mut self, message: Message) -> Result<()>`: Send message
- `get_peers(&self) -> Vec<PeerId>`: Get connected peers
- `get_topology(&self) -> NetworkTopology`: Get network topology

#### `MeshBehaviour`

Custom behavior for mesh networking.

```rust
pub struct MeshBehaviour {
    pub gossipsub: Gossipsub,
    pub identify: Identify,
    pub kademlia: Kademlia,
    pub mdns: Mdns,
}
```

**Methods:**

- `new(local_peer_id: PeerId) -> Result<Self>`: Create new behavior
- `handle_event(&mut self, event: SwarmEvent) -> Result<()>`: Handle swarm events
- `get_peer_info(&self, peer_id: &PeerId) -> Option<PeerInfo>`: Get peer information

### Security

#### `SecurityManager`

Comprehensive security management.

```rust
pub struct SecurityManager {
    pub threat_level: ThreatLevel,
    pub policies: SecurityPolicies,
    pub stats: SecurityStats,
    pub event_sender: mpsc::Sender<SecurityEvent>,
}
```

**Methods:**

- `new() -> Self`: Create new security manager
- `update_threat_level(&mut self, level: ThreatLevel)`: Update threat level
- `add_policy(&mut self, policy: SecurityPolicy)`: Add security policy
- `scan_connection(&mut self, connection: &ConnectionInfo) -> ThreatScore`: Scan connection
- `get_stats(&self) -> SecurityStats`: Get security statistics
- `log_event(&mut self, event: SecurityEvent)`: Log security event

#### `SecurityPolicy`

Configurable security policies.

```rust
pub struct SecurityPolicy {
    pub name: String,
    pub rules: Vec<SecurityRule>,
    pub actions: Vec<SecurityAction>,
    pub enabled: bool,
}
```

**Methods:**

- `new(name: String) -> Self`: Create new policy
- `add_rule(&mut self, rule: SecurityRule)`: Add security rule
- `add_action(&mut self, action: SecurityAction)`: Add security action
- `evaluate(&self, context: &SecurityContext) -> bool`: Evaluate policy

### Transport

#### `StealthTCP`

Advanced stealth TCP transport.

```rust
pub struct StealthTCP {
    pub config: StealthConfig,
    pub rate_limiter: RateLimiter,
    pub threat_scorer: ThreatScorer,
    pub stats: ConnectionStats,
}
```

**Methods:**

- `new(config: StealthConfig) -> Self`: Create new stealth TCP transport
- `create_connection(&mut self, addr: SocketAddr) -> Result<Connection>`: Create connection
- `accept_connection(&mut self, stream: TcpStream) -> Result<Connection>`: Accept connection
- `rate_limit(&mut self, addr: &IpAddr) -> bool`: Check rate limit
- `score_threat(&mut self, connection: &ConnectionInfo) -> ThreatScore`: Score connection threat

## Web API

### REST Endpoints

#### Core API

##### `GET /api/status`
Get system status.

**Response:**
```json
{
  "status": "running",
  "version": "0.1.0",
  "uptime": 3600,
  "peers": 5,
  "messages_sent": 100,
  "messages_received": 95
}
```

##### `GET /api/identity`
Get current identity information.

**Response:**
```json
{
  "id": "peer-id",
  "username": "alice",
  "public_key": "base64-encoded-key",
  "created_at": "2024-01-15T10:30:00Z",
  "is_ephemeral": false
}
```

##### `POST /api/send`
Send a message.

**Request:**
```json
{
  "recipient": "peer-id",
  "content": "Hello, world!",
  "message_type": "text"
}
```

**Response:**
```json
{
  "message_id": "msg-123",
  "status": "sent",
  "timestamp": "2024-01-15T10:30:00Z"
}
```

#### Mesh Network API

##### `POST /api/mesh/init`
Initialize mesh network.

**Request:**
```json
{
  "host": "127.0.0.1",
  "port": 8080,
  "enable_stealth": true
}
```

**Response:**
```json
{
  "status": "initialized",
  "peer_id": "peer-id",
  "addresses": ["/ip4/127.0.0.1/tcp/8080"]
}
```

##### `GET /api/mesh/stats`
Get mesh network statistics.

**Response:**
```json
{
  "peers_connected": 5,
  "messages_sent": 100,
  "messages_received": 95,
  "network_topology": {
    "nodes": 6,
    "edges": 15,
    "average_degree": 5.0
  }
}
```

##### `GET /api/mesh/topology`
Get network topology.

**Response:**
```json
{
  "nodes": [
    {
      "id": "peer-1",
      "addresses": ["/ip4/192.168.1.1/tcp/8080"],
      "connections": 3
    }
  ],
  "edges": [
    {
      "from": "peer-1",
      "to": "peer-2",
      "latency": 10
    }
  ]
}
```

#### Security API

##### `GET /api/security/stats`
Get security statistics.

**Response:**
```json
{
  "threat_level": "low",
  "connections_scanned": 1000,
  "threats_detected": 5,
  "policies_active": 10,
  "events_logged": 500
}
```

##### `POST /api/security/scan`
Perform security scan.

**Request:**
```json
{
  "target": "peer-id",
  "scan_type": "full"
}
```

**Response:**
```json
{
  "scan_id": "scan-123",
  "status": "completed",
  "threat_score": 0.1,
  "findings": [
    {
      "severity": "low",
      "description": "High connection frequency",
      "recommendation": "Monitor connection patterns"
    }
  ]
}
```

##### `GET /api/security/events`
Get security events.

**Query Parameters:**
- `limit`: Number of events to return (default: 100)
- `severity`: Filter by severity level
- `since`: Filter events since timestamp

**Response:**
```json
{
  "events": [
    {
      "id": "event-123",
      "timestamp": "2024-01-15T10:30:00Z",
      "severity": "medium",
      "type": "connection_attempt",
      "description": "Suspicious connection attempt from 192.168.1.100",
      "source_ip": "192.168.1.100",
      "action_taken": "rate_limited"
    }
  ],
  "total": 500,
  "has_more": true
}
```

### WebSocket API

#### Connection
Connect to WebSocket endpoint: `ws://localhost:8080/ws`

#### Message Types

##### Security Events
```json
{
  "type": "security_event",
  "data": {
    "event_id": "event-123",
    "severity": "high",
    "description": "Threat detected",
    "timestamp": "2024-01-15T10:30:00Z"
  }
}
```

##### Network Events
```json
{
  "type": "network_event",
  "data": {
    "event_type": "peer_connected",
    "peer_id": "peer-123",
    "timestamp": "2024-01-15T10:30:00Z"
  }
}
```

##### Message Events
```json
{
  "type": "message_event",
  "data": {
    "event_type": "message_received",
    "message_id": "msg-123",
    "sender": "peer-123",
    "timestamp": "2024-01-15T10:30:00Z"
  }
}
```

## CLI API

### Command Structure

```bash
ghostwire [COMMAND] [OPTIONS]
```

### Commands

#### `init`
Initialize a new GhostWire identity.

```bash
ghostwire init --username "alice" [OPTIONS]
```

**Options:**
- `--username <USERNAME>`: Username for the identity
- `--ephemeral`: Create ephemeral identity
- `--output <FILE>`: Output file for identity

#### `web`
Start the web server.

```bash
ghostwire web [OPTIONS]
```

**Options:**
- `--host <HOST>`: Host to bind to (default: 127.0.0.1)
- `--port <PORT>`: Port to bind to (default: 8080)
- `--config <FILE>`: Configuration file path

#### `send`
Send a message to a peer.

```bash
ghostwire send --recipient <PEER_ID> --message <MESSAGE> [OPTIONS]
```

**Options:**
- `--recipient <PEER_ID>`: Recipient peer ID
- `--message <MESSAGE>`: Message content
- `--type <TYPE>`: Message type (text, file, group)
- `--encrypt`: Encrypt message (default: true)

#### `peers`
List connected peers.

```bash
ghostwire peers [OPTIONS]
```

**Options:**
- `--verbose`: Show detailed peer information
- `--format <FORMAT>`: Output format (json, table)

#### `security`
Security management commands.

```bash
ghostwire security <SUBCOMMAND>
```

**Subcommands:**
- `scan`: Perform security scan
- `stats`: Show security statistics
- `events`: Show security events
- `policy`: Manage security policies

#### `mesh`
Mesh network management.

```bash
ghostwire mesh <SUBCOMMAND>
```

**Subcommands:**
- `start`: Start mesh networking
- `stop`: Stop mesh networking
- `status`: Show mesh status
- `topology`: Show network topology

## Configuration

### Configuration File Format

```toml
[network]
host = "127.0.0.1"
port = 8080
enable_stealth = true
enable_tor = false

[security]
threat_level = "medium"
max_connections = 100
rate_limit_per_second = 10
enable_audit_logging = true

[encryption]
key_rotation_interval = 86400
encryption_algorithm = "aes-256-gcm"
key_derivation_iterations = 100000

[mesh]
gossip_topic = "ghostwire-messages"
peer_discovery_timeout = 30
max_peers = 50

[reticulum]
enable_lora = false
lora_frequency = 915000000
lora_power = 14
```

### Environment Variables

- `GHOSTWIRE_CONFIG`: Path to configuration file
- `GHOSTWIRE_LOG_LEVEL`: Logging level (debug, info, warn, error)
- `GHOSTWIRE_HOST`: Default host for web server
- `GHOSTWIRE_PORT`: Default port for web server
- `GHOSTWIRE_SECURITY_LEVEL`: Security level (low, medium, high)

## Error Handling

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum GhostWireError {
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Security error: {0}")]
    Security(String),
    
    #[error("Encryption error: {0}")]
    Encryption(String),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
}
```

### Error Responses

#### HTTP Error Responses

**400 Bad Request:**
```json
{
  "error": "validation_error",
  "message": "Invalid request parameters",
  "details": {
    "field": "recipient",
    "issue": "Peer ID is required"
  }
}
```

**401 Unauthorized:**
```json
{
  "error": "authentication_error",
  "message": "Authentication required",
  "details": {
    "required": "valid_identity"
  }
}
```

**403 Forbidden:**
```json
{
  "error": "security_error",
  "message": "Access denied",
  "details": {
    "reason": "threat_detected",
    "threat_score": 0.8
  }
}
```

**500 Internal Server Error:**
```json
{
  "error": "internal_error",
  "message": "An internal error occurred",
  "request_id": "req-123"
}
```

## Examples

### Basic Usage

#### Initialize Identity
```rust
use ghostwire::core::identity::Identity;

let identity = Identity::new("alice".to_string())?;
println!("Created identity: {}", identity.id);
```

#### Send Encrypted Message
```rust
use ghostwire::core::{Message, EncryptionEngine};

let engine = EncryptionEngine::new();
let message = Message::new(
    "alice".to_string(),
    "bob".to_string(),
    "Hello, Bob!".as_bytes().to_vec()
);

let encrypted = message.encrypt(&engine, &key)?;
```

#### Start Mesh Network
```rust
use ghostwire::core::mesh::MeshNetwork;

let mut mesh = MeshNetwork::new(&identity)?;
mesh.start()?;

// Send message
mesh.send_message(message)?;
```

### Web API Usage

#### JavaScript Example
```javascript
// Get system status
const response = await fetch('/api/status');
const status = await response.json();
console.log('System status:', status);

// Send message
const messageResponse = await fetch('/api/send', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
  },
  body: JSON.stringify({
    recipient: 'peer-id',
    content: 'Hello, world!',
    message_type: 'text'
  })
});
const result = await messageResponse.json();
```

#### Python Example
```python
import requests

# Get identity
response = requests.get('http://localhost:8080/api/identity')
identity = response.json()
print(f"Identity: {identity['username']}")

# Send message
message_data = {
    'recipient': 'peer-id',
    'content': 'Hello from Python!',
    'message_type': 'text'
}
response = requests.post('http://localhost:8080/api/send', json=message_data)
result = response.json()
```

### CLI Examples

#### Initialize and Start
```bash
# Initialize identity
ghostwire init --username alice

# Start web server
ghostwire web --host 0.0.0.0 --port 9000

# Send message
ghostwire send --recipient peer-id --message "Hello, world!"

# Check peers
ghostwire peers --verbose

# Security scan
ghostwire security scan --target peer-id
```

## Best Practices

### Security
- Always use strong passwords for identity creation
- Enable stealth mode for enhanced privacy
- Regularly update security policies
- Monitor security events and logs
- Use ephemeral identities for sensitive communications

### Performance
- Configure appropriate connection limits
- Monitor network performance metrics
- Use efficient message serialization
- Implement proper error handling and recovery
- Optimize for your specific use case

### Development
- Follow Rust coding standards
- Write comprehensive tests
- Document all public APIs
- Handle errors gracefully
- Use async/await for I/O operations

---

For more detailed information, see the [main documentation](README.md) and [contributing guidelines](CONTRIBUTING.md). 