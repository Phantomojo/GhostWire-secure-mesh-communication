# GhostWire

**Decentralized, Secure, and Stealth Mesh Networking & Messaging Platform**

[![Rust](https://img.shields.io/badge/Rust-1.70+-blue.svg)](https://www.rust-lang.org/)
[![Node.js](https://img.shields.io/badge/Node.js-18+-green.svg)](https://nodejs.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Security](https://img.shields.io/badge/Security-First-red.svg)](SECURITY.md)

---

## 🚀 Project Vision
GhostWire is a universal, privacy-first mesh communication platform. It bridges multiple mesh protocols (Bluetooth, WiFi, LoRa, WebRTC, TCP/IP, etc.), supports cross-network channels, and provides robust security, privacy, and usability. The goal: censorship-resistant, resilient, and decentralized communication—usable by anyone, anywhere, on any device.

---

## 🌟 Key Features
- **End-to-End Encrypted Messaging** (AES-256-GCM, Ed25519/X25519)
- **Zero-Trust Architecture**: No server or relay can decrypt or correlate user data
- **Stealth/Disguised Chats & Panic Mode**: For sensitive situations
- **Peer-to-Peer & Federated Networking**: Resilient, decentralized, and offline-capable
- **Store-and-Forward**: Asynchronous delivery for unreliable networks
- **Modular Transports**: Bluetooth, WiFi, LoRa, WebRTC, TCP/IP, and more
- **Protocol Adapters**: Bridge to Briar, Meshtastic, Matrix, etc.
- **Traffic Obfuscation**: Cover traffic, timing randomization, and packet padding
- **Open API & Modern Web UI**: REST/WebSocket API, React/TypeScript frontend
- **Security-First**: Threat detection, Sybil defense, quotas, blacklists, disaster triggers, reputation, federation

---

## 🏗️ Architecture Overview
- **Rust Core**: Modular, async, and secure
- **Web UI**: React/TypeScript, Tauri wrappers for desktop/mobile
- **Transport Abstraction**: All network transports are modular and hot-swappable
- **Protocol Adapters**: Bridge messages between mesh protocols
- **Universal Channels**: Deduplication and relay logic across networks
- **Security & Privacy**: End-to-end encryption, forward secrecy, cover traffic, emergency wipe

See [PROJECT_OVERVIEW.md](./PROJECT_OVERVIEW.md) for a deep dive.

---

## ⚡ Quickstart

### Universal Installation (All Platforms)
```bash
curl -fsSL https://raw.githubusercontent.com/Phantomojo/Obsidian/main/install.sh | bash
```

### Manual Installation
```bash
git clone https://github.com/Phantomojo/Obsidian.git
cd Obsidian
# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Install Node.js (if not installed)
# Download from https://nodejs.org/
# Build and run backend
cd ghostwire && cargo run -- --host 0.0.0.0 --port 3001
# Build and run frontend
cd ../webui && npm install && npm run dev
```

### Multi-PC Setup
- Run the installer on each PC
- Each PC gets unique ports automatically
- Set usernames in the Peers tab
- Click "SCAN NETWORK" to discover other PCs
- Start chatting—it's that simple!

---

## 🔒 Security & Privacy
- **End-to-End Encryption**: AES-256-GCM, Ed25519/X25519
- **Perfect Forward Secrecy**: Ephemeral keys for each session
- **Threat Detection**: Real-time monitoring, Sybil defense, quotas, blacklists
- **Traffic Obfuscation**: Cover traffic, dummy nodes, timing randomization
- **Disaster Mode**: Fallback to LoRa/Bluetooth, panic wipe, emergency triggers
- **Zero-Knowledge Proofs**: Cryptographic proofs for identity verification
- **Audit Logging**: Comprehensive event logging and analysis

See [docs/threat_model.md](./docs/threat_model.md) for details.

---

## 🧩 Project Structure
```
ghostwire/      # Rust core, CLI, backend
webui/          # React/TypeScript frontend
webui/src-tauri/ # Tauri desktop/mobile wrappers
meshtastic-*    # (To be reviewed for integration)
docs/           # Roadmap, threat model, use cases, etc.
```

---

## 📚 Documentation
- [Project Overview](./PROJECT_OVERVIEW.md)
- [Roadmap](./docs/roadmap.md)
- [Threat Model](./docs/threat_model.md)
- [Use Cases](./docs/use_cases.md)
- [Changelog](./ghostwire/CHANGELOG.md)
- [Contributing](./CONTRIBUTING.md)

---

## 🛠️ Development
### Backend
```bash
cd ghostwire
cargo run -- --host 0.0.0.0 --port 3001
```
### Frontend
```bash
cd webui
npm run dev
```
### Testing
```bash
cd ghostwire && cargo test
cd webui && npm test
```

---

## 🌍 API Endpoints (Sample)
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET    | /api/status         | Check server status |
| GET    | /api/peers          | Get list of connected peers |
| POST   | /api/send_message   | Send encrypted message |
| GET    | /api/settings       | Get current settings |
| PUT    | /api/settings       | Update settings |
| GET    | /api/public_key     | Get server's public key |
| WS     | /ws                 | WebSocket for real-time messaging |

---

## 🤝 Contributing
1. **Fork** the repository
2. **Create** a feature branch
3. **Make** your changes
4. **Test** thoroughly
5. **Submit a pull request**

See [CONTRIBUTING.md](./CONTRIBUTING.md) for more.

---

## 🗺️ Roadmap & Status
- [x] Modular security/trust modules (Sybil, quota, blacklist, disaster, reputation)
- [x] Store-and-forward with quotas
- [x] Disaster mode triggers and fallback
- [x] Local reputation and federation trust
- [x] First integration and security tests
- [x] Initial docs: threat model, use cases, API
- [ ] Traffic obfuscation (cover traffic, dummy nodes)
- [ ] Advanced federation/bridge trust
- [ ] Signed score exchange, key revocation propagation
- [ ] Mobile/desktop wrappers (Tauri)
- [ ] Global reputation, plugin adapters
- [ ] Expanded docs: deployment, advanced usage

See [docs/roadmap.md](./docs/roadmap.md) for details.

---

## 📢 Community & Support
- **GitHub Issues**: Bug reports and feature requests
- **Discord/Forum**: Community chat and support (link coming soon)
- **Security**: Direct contact for vulnerabilities (see SECURITY.md)

---

## 📝 License
MIT License. See [LICENSE](./LICENSE).

---

*GhostWire is open source, community-driven, and welcomes all contributors. Help us build the future of secure, decentralized communication!*
