# �� GhostWire - Secure Multi-PC Messaging Network

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://rustup.rs/)
[![Node.js](https://img.shields.io/badge/Node.js-18+-green.svg)](https://nodejs.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Multi-PC](https://img.shields.io/badge/Multi--PC-Ready-purple.svg)](https://github.com/Phantomojo/Obsidian)
[![Auto-Discovery](https://img.shields.io/badge/Auto--Discovery-Enabled-cyan.svg)](https://github.com/Phantomojo/Obsidian)

> **🔐 End-to-End Encrypted • 🛡️ Zero-Knowledge • ⚡ Real-time • 🌐 Multi-PC Ready**

GhostWire is a revolutionary secure messaging system that enables **automatic peer discovery** and **encrypted communication** across multiple PCs on the same network. Built with Rust for performance and React for a modern UI.

## 🚀 Features

### 🔍 **Automatic Peer Discovery**
- **Network Scanning**: Automatically finds other GhostWire nodes on your local network
- **Username-Based**: See "Alice" and "Bob" instead of IP addresses
- **Real-time Updates**: Instantly discover new peers as they join the network
- **Cross-Platform**: Works on Windows, macOS, Linux, and WSL

### 🔐 **Military-Grade Security**
- **ChaCha20-Poly1305**: State-of-the-art encryption for messages
- **Ed25519**: Secure key generation and verification
- **Zero-Knowledge**: No central server stores your messages
- **End-to-End**: Only you and your recipient can read messages

### ⚡ **Real-time Communication**
- **WebSocket**: Instant message delivery
- **Multi-PC Support**: Connect multiple computers seamlessly
- **Auto-Reconnection**: Automatically reconnects if connection is lost
- **Status Monitoring**: Real-time connection status and peer health

### 🎯 **Universal Installation**
- **One-Command Setup**: `./install.sh` works on any OS
- **Auto-Port Detection**: Automatically finds available ports
- **Network Configuration**: Detects your IP and configures networking
- **Windows Support**: PowerShell script for easy Windows deployment

## 🛠️ Quick Start

### **Universal Installation (All Platforms)**

```bash
# Download and run the universal installer
curl -fsSL https://raw.githubusercontent.com/Phantomojo/Obsidian/main/install.sh | bash
```

### **Windows Quick Start**

```powershell
# Run the PowerShell script
.\start-all.ps1
```

### **Manual Installation**

```bash
# Clone the repository
git clone https://github.com/Phantomojo/Obsidian.git
cd Obsidian

# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js (if not installed)
# Download from https://nodejs.org/

# Build and run
cd ghostwire && cargo run -- --host 0.0.0.0 --port 3001
cd ../webui && npm install && npm run dev
```

## 🌐 Multi-PC Setup

### **Automatic Setup**
1. **Run the installer** on each PC: `./install.sh`
2. **Each PC gets unique ports** automatically
3. **Set usernames** in the Peers tab
4. **Click "SCAN NETWORK"** to discover other PCs
5. **Start chatting** - it's that simple!

### **Manual Multi-PC Setup**

#### **PC 1 (Port 3001)**
```bash
cd ghostwire
cargo run -- --host 0.0.0.0 --port 3001
```

#### **PC 2 (Port 3002)**
```bash
cd ghostwire
cargo run -- --host 0.0.0.0 --port 3002
```

#### **Frontend Configuration**
Update `webui/src/services/api.ts` on each PC:
```typescript
// PC 1
const API_BASE_URL = 'http://localhost:3001/api';

// PC 2  
const API_BASE_URL = 'http://localhost:3002/api';
```

## 🔍 Network Discovery

### **Automatic Discovery**
- **Network Scanner**: Scans your local network (192.168.1.x)
- **Port Detection**: Checks ports 3001-3005 for GhostWire nodes
- **Username Retrieval**: Gets usernames from discovered nodes
- **Auto-Connection**: Automatically connects to found peers

### **Manual Discovery**
```bash
# Run the network discovery script
./discover-peers.sh
```

## 🏗️ Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   PC 1          │    │   PC 2          │    │   PC 3          │
│  (Port 3001)    │    │  (Port 3002)    │    │  (Port 3003)    │
│                 │    │                 │    │                 │
│  ┌───────────┐  │    │  ┌───────────┐  │    │  ┌───────────┐  │
│  │ Frontend  │  │    │  │ Frontend  │  │    │  │ Frontend  │  │
│  │ (React)   │  │    │  │ (React)   │  │    │  │ (React)   │  │
│  └───────────┘  │    │  └───────────┘  │    │  └───────────┘  │
│        │        │    │        │        │    │        │        │
│  ┌───────────┐  │    │  ┌───────────┐  │    │  ┌───────────┐  │
│  │ Backend   │◄─┼────┼─►│ Backend   │◄─┼────┼─►│ Backend   │  │
│  │ (Rust)    │  │    │  │ (Rust)    │  │    │  │ (Rust)    │  │
│  └───────────┘  │    │  └───────────┘  │    │  └───────────┘  │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## 🔧 API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/status` | Check server status |
| `GET` | `/api/peers` | Get list of connected peers |
| `POST` | `/api/send_message` | Send encrypted message |
| `GET` | `/api/settings` | Get current settings |
| `PUT` | `/api/settings` | Update settings |
| `GET` | `/api/public_key` | Get server's public key |
| `GET` | `/api/scan_network` | Scan for other GhostWire nodes |
| `POST` | `/api/register_peer` | Register with another node |
| `GET` | `/api/get_username` | Get current username |
| `POST` | `/api/set_username` | Set username |
| `WS` | `/ws` | WebSocket for real-time messaging |

## 🛡️ Security Features

### **Encryption**
- **ChaCha20-Poly1305**: High-performance authenticated encryption
- **Ed25519**: Elliptic curve digital signatures
- **Key Derivation**: Secure key generation from random entropy
- **Perfect Forward Secrecy**: Each session uses unique keys

### **Network Security**
- **Local Network Only**: No internet exposure required
- **Firewall Friendly**: Uses standard HTTP/WebSocket ports
- **No Central Server**: Decentralized peer-to-peer architecture
- **Zero Data Collection**: No logs, no tracking, no metadata

## 🚀 Performance

- **Rust Backend**: Sub-millisecond message processing
- **React Frontend**: Smooth 60fps UI updates
- **WebSocket**: Real-time bidirectional communication
- **Auto-Scaling**: Handles multiple concurrent connections

## 📱 Browser Support

- **Chrome/Edge**: Full support
- **Firefox**: Full support  
- **Safari**: Full support
- **Mobile Browsers**: Responsive design

## 🔧 Development

### **Backend Development**
```bash
cd ghostwire
cargo run -- --host 0.0.0.0 --port 3001
```

### **Frontend Development**
```bash
cd webui
npm run dev
```

### **Testing**
```bash
# Backend tests
cd ghostwire && cargo test

# Frontend tests
cd webui && npm test
```

## 📦 Dependencies

### **Backend (Rust)**
- `axum`: Web framework
- `tokio`: Async runtime
- `ring`: Cryptography
- `serde`: Serialization
- `reqwest`: HTTP client

### **Frontend (React)**
- `react`: UI framework
- `typescript`: Type safety
- `tailwindcss`: Styling
- `vite`: Build tool

## 🤝 Contributing

1. **Fork** the repository
2. **Create** a feature branch
3. **Make** your changes
4. **Test** thoroughly
5. **Submit** a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🌟 Star History

[![Star History Chart](https://api.star-history.com/svg?repos=Phantomojo/Obsidian&type=Date)](https://star-history.com/#Phantomojo/Obsidian&Date)

## 🔗 Links

- **GitHub**: https://github.com/Phantomojo/Obsidian
- **Issues**: https://github.com/Phantomojo/Obsidian/issues
- **Discussions**: https://github.com/Phantomojo/Obsidian/discussions

---

**🌐 GhostWire - Secure Multi-PC Communication for the Digital Age**

*Built with ❤️ using Rust and React*

```
██╗    ██╗ ██████╗ ███████╗██╗  ██╗███████╗██╗    ██╗██╗██████╗ ███████╗
██║    ██║██╔════╝ ██╔════╝██║  ██║██╔════╝██║    ██║██║██╔══██╗██╔════╝
██║ █╗ ██║██║  ███╗███████╗███████║█████╗  ██║ █╗ ██║██║██████╔╝█████╗  
██║███╗██║██║   ██║╚════██║██╔══██║██╔══╝  ██║███╗██║██║██╔══██╗██╔══╝  
╚███╔███╔╝╚██████╔╝███████║██║  ██║██║     ╚███╔███╔╝██║██║  ██║███████╗
 ╚══╝╚══╝  ╚═════╝ ╚══════╝╚═╝  ╚═╝╚═╝      ╚══╝╚══╝ ╚═╝╚═╝  ╚═╝╚══════╝
```

</rewritten_file>