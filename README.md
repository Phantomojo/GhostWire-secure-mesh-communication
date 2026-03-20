<!-- SPDX-License-Identifier: AGPL-3.0-or-later OR Commercial License -->
# 🚀 **GhostWire** - Secure Mesh Networking & Communication

![GhostWire - Afrofuturistic Secure Mesh](webui/public/brand-header.png)

[![GitHub license](https://img.shields.io/github/license/Phantomojo/GhostWire-secure-mesh-communication)](LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/Phantomojo/GhostWire-secure-mesh-communication?style=social)](https://github.com/Phantomojo/GhostWire-secure-mesh-communication/stargazers)
![Status](https://img.shields.io/badge/Status-Active%20Development-blue?style=for-the-badge)
![Tech Stack](https://img.shields.io/badge/Tech-Rust%20|%20libp2p%20|%20React-orange?style=for-the-badge)
[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL--3.0-blue.svg)](LICENSE)
[![Commercial License](https://img.shields.io/badge/License-Commercial-available.svg)](COMMERCIAL_LICENSE.md)

**GhostWire** is a high-performance, decentralized communication platform built for resilience in adverse environments. It combines high-speed peer-to-peer mesh networking with a tactical interface designed for coordination and secure information sharing when traditional infrastructure fails.

---

## 📜 Licensing

**GhostWire is dual-licensed:**

- **🆓 AGPL-3.0-or-later** - For open source, community, and personal use
- **🏢 Commercial License** - For proprietary, closed-source, and SaaS deployments

**Which license do I need?**

| Use Case | License |
|----------|---------|
| Personal/Research/Education | ✅ AGPL-3.0 (free) |
| Open Source Projects | ✅ AGPL-3.0 (free) |
| Internal Tools (with source disclosure) | ✅ AGPL-3.0 (free) |
| Proprietary Products | 🏢 Commercial |
| SaaS without source disclosure | 🏢 Commercial |
| Commercial Hosting | 🏢 Commercial |

📧 **Commercial inquiries**: mirungu015@proton.me  
📄 **Full terms**: See [LICENSE](LICENSE), [COMMERCIAL_LICENSE.md](COMMERCIAL_LICENSE.md), [LICENSE_EXPLANATION.md](LICENSE_EXPLANATION.md)

---

## ⚡ **Key Pillars**

### **🛡️ Security-First Architecture**
- **Stealth Transport**: Custom TCP handshake layer that appears as generic traffic to unauthorized scanners.
- **End-to-End Encryption**: All messages are encrypted by default; metadata is minimized.
- **Disposable Identities**: Support for multiple ephemeral profiles with cryptographic key separation.

### **🔍 Decentralized Resilience**
- **Heterogeneous Mesh**: Support for WiFi Direct, BLE, and LoRa (via Tiered Hardware adapters).
- **libp2p P2P Stack**: Utilizing Gossipsub for efficient message propagation and Kademlia DHT for peer discovery.
- **One-Click Scaling**: Spin up node swarms for testing or deployment using optimized shell controllers.

### **🛰️ Tactical HUD Interface**
- **Real-time Monitoring**: Glowing status indicators for CPU, RAM, and Security posture.
- **Network Radar**: Dynamic, interactive mesh topology visualization.
- **Modern Messaging**: Split-view, high-density communication interface optimized for coordination.

---

## 🏗️ **Technical Architecture**

```mermaid
graph TD
    A[Frontend: React/TS] <--> B[Backend API: Rust/Axum]
    B <--> C[Security Layer: Stealth Handshake]
    C <--> D[Mesh Protocol: libp2p]
    D <--> E[Transports: WiFi/BLE/LoRa]
    B <--> F[Local Storage: Identity/Chain]
```

### **Core Components**
- **Backend (Rust)**: High-performance core handling encryption, state management, and the p2p stack.
- **Frontend (TypeScript)**: Responsive, tactical interface using modern React patterns and real-time WebSocket updates.
- **Mesh Layer**: Pluggable transport architecture supporting a variety of hardware interfaces.

---

## 🚀 **Getting Started**

### **One-Line Installation (Linux/macOS)**
```bash
git clone https://github.com/Phantomojo/GhostWire-secure-mesh-communication.git && cd GhostWire-secure-mesh-communication && ./scripts/install.sh
```

### **Manual Build**
1. **Backend**: `cd ghostwire && cargo build --release`
2. **Frontend**: `cd webui && npm install && npm run build`
3. **Launch**: `./scripts/ghostwire-manager.sh start`

---

## 👥 **Multi-Profile Support**

Maintain separate identities for different operational contexts. Launch with a specific profile:

```bash
./scripts/ghostwire-manager.sh start --profile delta_force
```
Each profile manages its own identity keypair and peer database in `~/.ghostwire/`.

---

## 📚 **Documentation**

Comprehensive guides are located in the [docs/](docs/) directory:

- [🏗️ ARCHITECTURE.md](docs/ARCHITECTURE.md) - Deep dive into system internals.
- [🛠️ INSTALL.md](docs/INSTALL.md) - Detailed setup for various platforms.
- [🔒 SECURITY.md](docs/SECURITY.md) - Disclosure policy and security model overview.
- [🧪 TESTING_GUIDE.md](docs/TESTING_GUIDE.md) - How to run network and security simulations.
- [🌐 LIBP2P_NETWORK_CONNECTIVITY.md](docs/LIBP2P_NETWORK_CONNECTIVITY.md) - Connecting to other libp2p networks and bootstrap nodes.
- [👤 PROFILE_IDENTITY_LIFECYCLE.md](docs/PROFILE_IDENTITY_LIFECYCLE.md) - Profile/identity from creation to deletion (API and storage).

---

## 🏆 **Mission Roadmap**

- [x] **Phase 1: Foundation** - Rust core + libp2p basic integration.
- [x] **Phase 2: Tactical HUD** - Real-time UI overhaul (Node Cluster V2).
- [/] **Phase 3: Deep Resilience** - LoRa integration and Micro-Raiden state channels.
- [ ] **Phase 4: Knowledge Ark** - Offline content serving and ZK-privacy layers.

---

## 📞 **Community & Contribution**

We welcome contributions from everyone! Whether you're fixing bugs, adding features, or just asking questions, you're part of our community.

### **Get Started**
- **🐛 Bug Reports**: [Create an issue](https://github.com/Phantomojo/GhostWire-secure-mesh-communication/issues)
- **💬 Discussions**: [Join the conversation](https://github.com/Phantomojo/GhostWire-secure-mesh-communication/discussions)
- **📖 Documentation**: [Read the docs](https://github.com/Phantomojo/GhostWire-secure-mesh-communication/tree/main/docs)
- **🤝 Contributing**: [See our guidelines](docs/CONTRIBUTING.md)
- **📋 Code of Conduct**: [Community standards](CODE_OF_CONDUCT.md)
- **📞 Support**: [Get help](SUPPORT.md)

### **Licensing & Legal**
- **📜 License**: AGPL-3.0-or-later + Commercial License
- **📄 LICENSE**: [Full license text](LICENSE)
- **💼 Commercial Use**: [Commercial licensing](COMMERCIAL_LICENSE.md)
- **📖 Explanation**: [Licensing guide](LICENSE_EXPLANATION.md)
- **✍️ CLA**: [Contributor License Agreement](docs/CLA.md) (coming soon)

### **Security**
- **🔒 Security Policy**: [SECURITY.md](docs/SECURITY.md)
- **🚨 Report Vulnerability**: [mirungu015@proton.me](mailto:mirungu015@proton.me)
- **🛡️ Security Audit**: [Audit report](docs/SECURITY_AUDIT.md)

---

## 👤 **Author**

**Author:** Michael Irungu  
**Contact:** mirungu015@proton.me  
**Location:** Nairobi, Kenya  
**License:** AGPL-3.0-or-later with commercial licensing available

---

<div align="center">

**🏆 Licensed under [AGPL-3.0-or-later](LICENSE) or [Commercial License](COMMERCIAL_LICENSE.md)**

*Built for connection. Designed for resilience.*  
*Connecting the disconnected, regardless of the odds.*

</div>
