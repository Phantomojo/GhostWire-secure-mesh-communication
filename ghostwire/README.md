# GhostWire - Secure Mesh Networking and Messaging

[![Rust](https://img.shields.io/badge/Rust-1.70+-blue.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Security](https://img.shields.io/badge/Security-First-red.svg)](SECURITY.md)

## 🚀 Overview

GhostWire is a cutting-edge secure mesh networking and messaging platform built in Rust, designed for privacy-conscious users who need reliable, encrypted communication without centralized infrastructure. The project implements advanced security features including end-to-end encryption, threat detection, anonymity networks, and decentralized peer discovery.

## 🔒 Security Features

### Core Security Architecture
- **End-to-End Encryption**: AES-256-GCM encryption with hybrid key pairs (Ed25519 + X25519)
- **Stealth TCP Transport**: Custom transport layer with stealth handshakes and connection obfuscation
- **Threat Detection**: Real-time security monitoring with IP reputation and behavioral analysis
- **Anonymity Networks**: Integration with Tor and other anonymity networks
- **Zero-Knowledge Proofs**: Cryptographic proofs for identity verification without revealing data

### Advanced Security Components
- **Security Manager**: Centralized security policy enforcement and threat response
- **Connection Rate Limiting**: Protection against DDoS and brute force attacks
- **IP Allowlisting**: Granular control over network access
- **Audit Logging**: Comprehensive security event logging and analysis
- **Key Rotation**: Automatic cryptographic key rotation and management

## 🏗️ Architecture

### Core Components

#### 1. Identity Management (`src/core/identity.rs`)
- **Ephemeral Identities**: Temporary identities for enhanced privacy
- **Key Pair Generation**: Ed25519 for signatures, X25519 for encryption
- **Identity Verification**: Cryptographic proof of identity without revealing secrets

#### 2. Encryption Engine (`src/core/encryption.rs`)
- **Hybrid Encryption**: Combines symmetric and asymmetric encryption
- **Key Derivation**: PBKDF2-based key derivation with high iteration counts
- **Perfect Forward Secrecy**: Ephemeral keys for each session
- **Message Authentication**: HMAC-SHA256 for message integrity

#### 3. Mesh Networking (`src/core/mesh.rs`)
- **libp2p Integration**: Built on the Rust libp2p ecosystem
- **Gossip Protocol**: Efficient message propagation across the network
- **Peer Discovery**: mDNS and Kademlia DHT for peer discovery
- **Topology Management**: Dynamic network topology tracking and optimization

#### 4. Stealth TCP Transport (`src/core/stealth_tcp.rs`)
- **Stealth Handshakes**: Custom handshake protocol to evade detection
- **Connection Obfuscation**: Traffic pattern obfuscation and timing randomization
- **Rate Limiting**: Per-IP connection rate limiting with exponential backoff
- **Threat Scoring**: Real-time threat assessment for incoming connections

#### 5. Security Manager (`src/core/security.rs`)
- **Threat Detection**: Multi-layered threat detection and response
- **Policy Enforcement**: Configurable security policies and rules
- **Event Correlation**: Security event analysis and threat intelligence
- **Incident Response**: Automated response to security incidents

### Network Protocols

#### Mesh Protocol
- **GossipSub**: Efficient message propagation with topic-based routing
- **Identify**: Peer information exchange and capability negotiation
- **Kademlia DHT**: Distributed hash table for peer discovery and content routing
- **mDNS**: Local network peer discovery

#### Reticulum Protocol (Inspired)
- **LoRa Integration**: Long-range radio communication support
- **Store-and-Forward**: Message persistence and delivery guarantees
- **Resource Management**: Efficient bandwidth and power usage
- **Multi-Hop Routing**: Dynamic routing through intermediate nodes

#### Briar Protocol (Inspired)
- **Contact-Based Messaging**: Direct messaging between trusted contacts
- **Offline Capability**: Message queuing and offline delivery
- **Group Messaging**: Secure group communication with member management
- **File Sharing**: Encrypted file transfer with integrity verification

## 🛠️ Installation

### Prerequisites
- Rust 1.70 or higher
- Cargo (Rust package manager)
- Git

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/ghostwire.git
cd ghostwire

# Build the project
cargo build --release

# Run tests
cargo test

# Install globally (optional)
cargo install --path .
```

### Configuration

Create a configuration file at `~/.config/ghostwire/config.toml`:

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
key_rotation_interval = 86400  # 24 hours
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

## 🚀 Usage

### Command Line Interface

```bash
# Start GhostWire in web mode (default)
ghostwire

# Start in CLI mode
ghostwire cli

# Start web server on specific host and port
ghostwire web --host 0.0.0.0 --port 9000

# Initialize a new identity
ghostwire init --username "alice"

# Send a message to a peer
ghostwire send --recipient "peer-id" --message "Hello, secure world!"

# Enable stealth mode
ghostwire cloak --enable

# Drop content with TTL
ghostwire drop --content "secret-data" --ttl 3600

# Fetch content by key
ghostwire fetch --key "content-hash"

# List connected peers
ghostwire peers --verbose

# Set trust score for a peer
ghostwire trust --peer "peer-id" --score 0.8
```

### Web Interface

Start the web server and navigate to `http://localhost:8080`:

- **Dashboard**: Overview of network status and security metrics
- **Identity Management**: View and manage your identity and keys
- **Peer Discovery**: Discover and connect to peers on the network
- **Message Center**: Send and receive encrypted messages
- **Security Monitor**: Real-time security events and threat analysis
- **Network Topology**: Visual representation of the mesh network

### API Endpoints

#### Core API
- `GET /api/status` - Get system status
- `GET /api/identity` - Get identity information
- `GET /api/public-key` - Get public key
- `POST /api/send` - Send a message

#### Mesh Network API
- `POST /api/mesh/init` - Initialize mesh network
- `POST /api/mesh/start` - Start mesh networking
- `POST /api/mesh/send` - Send mesh message
- `GET /api/mesh/stats` - Get mesh statistics
- `GET /api/mesh/topology` - Get network topology

#### Security API
- `GET /api/security/stats` - Get security statistics
- `POST /api/security/scan` - Perform security scan
- `GET /api/security/events` - Get security events

#### Reticulum API
- `POST /api/reticulum/init` - Initialize Reticulum network
- `POST /api/reticulum/send` - Send Reticulum message
- `GET /api/reticulum/stats` - Get Reticulum statistics

## 🔧 Development

### Project Structure

```
ghostwire/
├── src/
│   ├── main.rs              # Application entry point
│   ├── cli/                 # Command-line interface
│   │   ├── mod.rs           # CLI module
│   │   └── commands.rs      # CLI commands
│   ├── core/                # Core networking components
│   │   ├── mod.rs           # Core module
│   │   ├── identity.rs      # Identity management
│   │   ├── encryption.rs    # Encryption engine
│   │   ├── message.rs       # Message types
│   │   ├── transport.rs     # Transport layer
│   │   ├── mesh.rs          # Mesh networking
│   │   ├── reticulum.rs     # Reticulum networking
│   │   ├── briar.rs         # Briar-inspired messaging
│   │   ├── stealth_tcp.rs   # Stealth TCP transport
│   │   └── security.rs      # Security manager
│   └── web.rs               # Web interface
├── vendor/                  # Vendored dependencies
├── Cargo.toml              # Rust dependencies
├── README.md               # This file
├── SECURITY.md             # Security policy
├── CONTRIBUTING.md         # Contribution guidelines
└── LICENSE                 # MIT License
```

### Key Dependencies

#### Core Networking
- **libp2p**: Peer-to-peer networking framework
- **tokio**: Asynchronous runtime
- **futures**: Future and stream utilities

#### Security & Cryptography
- **ring**: Cryptographic primitives
- **aes-gcm**: AES-GCM encryption
- **blake2b_simd**: Fast hashing
- **getrandom**: Secure random number generation

#### Web Framework
- **axum**: Web framework
- **tower-http**: HTTP middleware
- **serde**: Serialization

#### CLI & Utilities
- **clap**: Command-line argument parsing
- **tracing**: Logging and diagnostics
- **anyhow**: Error handling

### Building and Testing

```bash
# Development build
cargo build

# Release build with optimizations
cargo build --release

# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with logging
RUST_LOG=debug cargo run

# Check for security vulnerabilities
cargo audit

# Format code
cargo fmt

# Lint code
cargo clippy
```

### Adding New Features

1. **Create a new module** in `src/core/` for your feature
2. **Add tests** in the module file or create `tests/` directory
3. **Update documentation** in this README and module comments
4. **Add CLI commands** in `src/cli/commands.rs` if needed
5. **Add web API endpoints** in `src/web.rs` if needed
6. **Update dependencies** in `Cargo.toml` if needed

## 🔒 Security Considerations

### Threat Model

GhostWire is designed to protect against:
- **Network Surveillance**: Traffic analysis and packet inspection
- **Man-in-the-Middle Attacks**: Connection interception and modification
- **DDoS Attacks**: Distributed denial of service attacks
- **Identity Correlation**: Linking multiple identities to the same user
- **Metadata Analysis**: Analysis of communication patterns and timing

### Security Best Practices

1. **Regular Updates**: Keep GhostWire and dependencies updated
2. **Strong Passwords**: Use strong, unique passwords for identity creation
3. **Network Security**: Use VPNs or Tor for additional network privacy
4. **Key Management**: Regularly rotate encryption keys
5. **Audit Logs**: Monitor security logs for suspicious activity
6. **Peer Verification**: Verify peer identities before establishing trust

### Reporting Security Issues

If you discover a security vulnerability, please report it responsibly:

1. **Do not** disclose the issue publicly
2. **Email** security@ghostwire.dev with details
3. **Include** steps to reproduce the issue
4. **Provide** any relevant logs or error messages

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

```bash
# Fork and clone the repository
git clone https://github.com/yourusername/ghostwire.git
cd ghostwire

# Create a feature branch
git checkout -b feature/your-feature-name

# Make your changes
# Add tests
# Update documentation

# Run tests and checks
cargo test
cargo fmt
cargo clippy

# Commit your changes
git commit -m "Add feature: description"

# Push to your fork
git push origin feature/your-feature-name

# Create a pull request
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **libp2p Team**: For the excellent peer-to-peer networking framework
- **Rust Community**: For the amazing language and ecosystem
- **Security Researchers**: For their work on privacy and security
- **Open Source Contributors**: For their valuable contributions

## 📞 Support

- **Documentation**: [docs.ghostwire.dev](https://docs.ghostwire.dev)
- **Issues**: [GitHub Issues](https://github.com/yourusername/ghostwire/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/ghostwire/discussions)
- **Email**: support@ghostwire.dev

---

**GhostWire** - Secure, Private, Decentralized Communication 