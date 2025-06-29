# 🌐 GhostWire - Secure Messaging Network

```
██╗    ██╗ ██████╗ ███████╗██╗  ██╗███████╗██╗    ██╗██╗██████╗ ███████╗
██║    ██║██╔════╝ ██╔════╝██║  ██║██╔════╝██║    ██║██║██╔══██╗██╔════╝
██║ █╗ ██║██║  ███╗███████╗███████║█████╗  ██║ █╗ ██║██║██████╔╝█████╗  
██║███╗██║██║   ██║╚════██║██╔══██║██╔══╝  ██║███╗██║██║██╔══██╗██╔══╝  
╚███╔███╔╝╚██████╔╝███████║██║  ██║██║     ╚███╔███╔╝██║██║  ██║███████╗
 ╚══╝╚══╝  ╚═════╝ ╚══════╝╚═╝  ╚═╝╚═╝      ╚══╝╚══╝ ╚═╝╚═╝  ╚═╝╚══════╝
```

> **Stealth Communication Protocol** | **End-to-End Encryption** | **Zero-Knowledge Architecture**

[![GitHub stars](https://img.shields.io/github/stars/Phantomojo/Obsidian?style=social&label=Stars)](https://github.com/Phantomojo/Obsidian/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/Phantomojo/Obsidian?style=social&label=Forks)](https://github.com/Phantomojo/Obsidian/network)
[![GitHub issues](https://img.shields.io/github/issues/Phantomojo/Obsidian?label=Issues)](https://github.com/Phantomojo/Obsidian/issues)
[![GitHub license](https://img.shields.io/github/license/Phantomojo/Obsidian?label=License)](https://github.com/Phantomojo/Obsidian/blob/main/LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Node.js](https://img.shields.io/badge/Node.js-18+-green.svg)](https://nodejs.org/)

---

## 🚀 Quick Start

### ⚡ One-Command Setup (Universal)

```bash
# Clone and run GhostWire in one command
curl -sSL https://raw.githubusercontent.com/Phantomojo/Obsidian/main/install.sh | bash
```

### 🔧 Manual Setup

```bash
# Clone the repository
git clone https://github.com/Phantomojo/Obsidian.git
cd Obsidian

# Start GhostWire (auto-detects OS and sets up environment)
./start-ghostwire.sh
```

---

## 🎯 Features

### 🔐 **Core Security**
- **End-to-End Encryption** using ChaCha20-Poly1305
- **Zero-Knowledge Architecture** - no data stored on servers
- **Perfect Forward Secrecy** with ephemeral key exchange
- **Stealth Mode** - traffic obfuscation and timing protection
- **Identity Verification** - cryptographic peer authentication

### 🌐 **Network Features**
- **P2P Messaging** - direct peer-to-peer communication
- **WebSocket Real-time** - instant message delivery
- **Cross-Platform** - Windows, Linux, macOS, WSL2
- **Web UI** - modern React interface with cyber theme
- **CLI Interface** - command-line messaging and control

### 🛡️ **Advanced Capabilities**
- **Threat Intelligence Sharing** - secure information exchange
- **Crisis Communication** - resilient during network disruptions
- **Message Persistence** - encrypted local storage
- **Peer Discovery** - automatic network node detection
- **Stealth Routing** - traffic obfuscation and routing

---

## 🏗️ Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Web UI        │    │   Rust Backend  │    │   Core Engine   │
│   (React)       │◄──►│   (Axum)        │◄──►│   (Encryption)  │
│   Port: 5173    │    │   Port: 3000    │    │   (Transport)   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   WebSocket     │    │   REST API      │    │   P2P Network   │
│   Real-time     │    │   Endpoints     │    │   Discovery     │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### **Data Flow**
```
Message → Encryption → Transport → Network → Decryption → Display
   ↑                                                           ↓
   └─────────────── Secure Channel ───────────────────────────┘
```

---

## 🛠️ Technology Stack

### **Backend (Rust)**
- **Axum** - High-performance web framework
- **Tokio** - Async runtime for networking
- **Ring** - Cryptography library (by BoringSSL)
- **Serde** - Serialization framework
- **Clap** - Command-line argument parsing

### **Frontend (React + TypeScript)**
- **React 18** - Modern UI framework
- **TypeScript** - Type-safe development
- **Tailwind CSS** - Utility-first styling
- **Vite** - Fast build tool
- **WebSocket** - Real-time communication

### **Security**
- **ChaCha20-Poly1305** - Authenticated encryption
- **Ed25519** - Digital signatures
- **X25519** - Key exchange
- **Argon2** - Password hashing
- **Random** - Cryptographically secure random generation

---

## 📖 Usage

### **Web Interface**
```bash
# Start the backend
cd ghostwire && cargo run

# Start the frontend (in another terminal)
cd webui && npm run dev

# Access the interface
# Backend: http://127.0.0.1:3000
# Frontend: http://localhost:5173
```

### **CLI Commands**
```bash
# Send a message
cargo run -- whisper <peer_id> <message>

# List peers
cargo run -- peers

# Generate new identity
cargo run -- identity generate

# Check system status
cargo run -- status

# Enable stealth mode
cargo run -- stealth enable
```

### **API Endpoints**
```bash
# Server status
curl http://127.0.0.1:3000/api/status

# Get peers
curl http://127.0.0.1:3000/api/peers

# Send message
curl -X POST http://127.0.0.1:3000/api/send_message \
  -H "Content-Type: application/json" \
  -d '{"recipient":"peer1","message":"Hello, GhostWire!"}'

# Get settings
curl http://127.0.0.1:3000/api/settings

# Update settings
curl -X PUT http://127.0.0.1:3000/api/settings \
  -H "Content-Type: application/json" \
  -d '{"stealth_mode":true}'

# WebSocket connection
wscat -c ws://127.0.0.1:3000/ws
```

---

## 🔧 Development

### **Prerequisites**
- **Rust** (1.70+) - [Install Rust](https://rustup.rs/)
- **Node.js** (18+) - [Install Node.js](https://nodejs.org/)
- **Git** - [Install Git](https://git-scm.com/)

### **Build from Source**
```bash
# Clone repository
git clone https://github.com/Phantomojo/Obsidian.git
cd Obsidian

# Build backend
cd ghostwire
cargo build --release

# Build frontend
cd ../webui
npm install
npm run build

# Run tests
cargo test
npm test
```

### **Development Workflow**
```bash
# Start development servers
./dev-start.sh

# Run in watch mode
cargo watch -x run  # Backend
npm run dev         # Frontend
```

### **Code Quality**
```bash
# Format code
cargo fmt
npm run format

# Lint code
cargo clippy
npm run lint

# Type check
npm run type-check
```

---

## 🚨 Security Considerations

### **Encryption**
- All messages are encrypted end-to-end
- Keys are never transmitted in plain text
- Perfect forward secrecy prevents key compromise
- Zero-knowledge architecture ensures no data leaks
- Cryptographic identity verification

### **Network Security**
- Traffic obfuscation in stealth mode
- Timing attack protection
- Man-in-the-middle attack prevention
- Denial of service resistance
- Peer authentication and verification

### **Privacy**
- No message logging
- No metadata collection
- No user tracking
- Ephemeral message support
- Local-only data storage

---

## 🌟 Roadmap

### **Phase 1: Core Features** ✅
- [x] Basic messaging system
- [x] Web UI with cyber theme
- [x] CLI interface
- [x] REST API
- [x] WebSocket real-time communication

### **Phase 2: Advanced Security** 🚧
- [ ] Stealth mode implementation
- [ ] Traffic obfuscation
- [ ] Advanced encryption protocols
- [ ] Identity management system
- [ ] Peer discovery and verification

### **Phase 3: Network Features** 📋
- [ ] P2P network implementation
- [ ] Distributed message routing
- [ ] Crisis communication protocols
- [ ] Threat intelligence sharing
- [ ] Cross-platform deployment

### **Phase 4: Enterprise Features** 📋
- [ ] Multi-user support
- [ ] Group messaging
- [ ] File sharing
- [ ] Message persistence
- [ ] Advanced analytics

---

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md).

### **Development Setup**
```bash
# Fork and clone
git clone https://github.com/YOUR_USERNAME/Obsidian.git
cd Obsidian

# Create feature branch
git checkout -b feature/amazing-feature

# Make changes and test
cargo test
npm test

# Commit and push
git commit -m "Add amazing feature"
git push origin feature/amazing-feature

# Create pull request
```

### **Code Style**
- Follow Rust formatting guidelines (`cargo fmt`)
- Use TypeScript for frontend code
- Write comprehensive tests
- Document public APIs
- Follow security best practices

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🌟 Acknowledgments

- **BoringSSL** - Cryptography library
- **Axum** - Web framework
- **React** - UI framework
- **Tailwind CSS** - Styling framework
- **Vite** - Build tool

---

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/Phantomojo/Obsidian/issues)
- **Discussions**: [GitHub Discussions](https://github.com/Phantomojo/Obsidian/discussions)
- **Security**: [Security Policy](SECURITY.md)
- **Documentation**: [Wiki](https://github.com/Phantomojo/Obsidian/wiki)

---

<div align="center">

**🌐 GhostWire - Secure Communication for the Digital Age**

*"In the shadows of the network, we speak freely."*

```
██╗    ██╗ ██████╗ ███████╗██╗  ██╗███████╗██╗    ██╗██╗██████╗ ███████╗
██║    ██║██╔════╝ ██╔════╝██║  ██║██╔════╝██║    ██║██║██╔══██╗██╔════╝
██║ █╗ ██║██║  ███╗███████╗███████║█████╗  ██║ █╗ ██║██║██████╔╝█████╗  
██║███╗██║██║   ██║╚════██║██╔══██║██╔══╝  ██║███╗██║██║██╔══██╗██╔══╝  
╚███╔███╔╝╚██████╔╝███████║██║  ██║██║     ╚███╔███╔╝██║██║  ██║███████╗
 ╚══╝╚══╝  ╚═════╝ ╚══════╝╚═╝  ╚═╝╚═╝      ╚══╝╚══╝ ╚═╝╚═╝  ╚═╝╚══════╝
```

</div>
