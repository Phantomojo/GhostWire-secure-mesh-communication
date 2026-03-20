# GhostWire - Project Context

**Last Updated:** March 2026  
**Repository:** https://github.com/Phantomojo/GhostWire-secure-mesh-communication  
**License:** AGPL-3.0-or-later OR Commercial License (dual-licensed)

---

## 📖 Project Overview

**GhostWire** is a high-performance, decentralized communication platform built for resilience in adverse environments. It combines peer-to-peer mesh networking with a tactical interface for secure information sharing when traditional infrastructure fails.

### Core Features
- **End-to-End Encryption** - AES-256-GCM, Ed25519/X25519, Perfect Forward Secrecy
- **libp2p Mesh Network** - Kademlia DHT, Gossipsub, Noise protocol
- **Stealth Transport** - Custom TCP handshake appearing as generic traffic
- **Multi-Profile Support** - Ephemeral identities with cryptographic key separation
- **Tactical HUD Interface** - Real-time monitoring, network radar, messaging
- **Heterogeneous Transports** - WiFi Direct, BLE, LoRa support (via adapters)

### Architecture
```
Frontend (React/TypeScript) ←→ Backend (Rust/Axum) ←→ libp2p Swarm ←→ Transports
                                    ↓
                            Security Layer
                            Identity Manager
                            DHT/Kademlia
```

---

## 🏗️ Project Structure

```
GhostWire-secure-mesh-communication/
├── ghostwire/              # Main Rust backend
│   ├── src/
│   │   ├── core/           # Core modules (identity, mesh, security, transport)
│   │   ├── controllers/    # API controllers (auth, messaging, profiles, system)
│   │   ├── cli/            # Command-line interface
│   │   ├── web.rs          # Axum web server & WebSocket handlers
│   │   ├── api_v2_*.rs     # REST API v2 endpoints & types
│   │   ├── genesis_constants.rs  # Network identity constants
│   │   └── main.rs         # Application entry point
│   ├── Cargo.toml          # Rust dependencies & features
│   └── build.rs            # Build script
│
├── webui/                  # React frontend (Vite + TypeScript)
│   ├── src/
│   │   ├── components/     # React components (panels, layout, shared)
│   │   ├── services/       # API client (axios-based)
│   │   ├── context/        # React context providers
│   │   ├── hooks/          # Custom React hooks
│   │   └── App.tsx         # Main application component
│   ├── package.json        # Node.js dependencies
│   └── vite.config.ts      # Vite build configuration
│
├── src-tauri/              # Tauri desktop/mobile wrapper
├── gw-cli/                 # Standalone CLI tool
├── docs/                   # Documentation (architecture, security, guides)
├── scripts/                # Shell scripts (deployment, testing, utilities)
├── config/                 # Configuration files & examples
├── tests/                  # Integration & end-to-end tests
├── release/                # Release packages & deployment configs
├── bruno-collection/       # API testing collection
└── [versioned-archives]/   # Historical snapshots (ghostwire-v*, etc.)
```

### Key Directories
- **`ghostwire/`** - Primary Rust backend (active development)
- **`webui/`** - React frontend (active development)
- **`docs/`** - Comprehensive documentation
- **`scripts/`** - Automation and deployment scripts
- **`ghostwire-v*/`**, **`ghostwire-complete-v*/`** - Historical snapshots (read-only archives)

---

## 🛠️ Technology Stack

### Backend (Rust)
| Category | Technologies |
|----------|-------------|
| **Web Framework** | Axum 0.7, Tower, Tower-HTTP |
| **P2P Network** | libp2p 0.56 (tokio, tcp, dns, noise, yamux, kad, gossipsub, mdns, identify, autonat, relay, dcutr) |
| **Cryptography** | ed25519-dalek, x25519-dalek, aes-gcm, chacha20poly1305, pqcrypto-mlkem, vodozemac, argon2, pbkdf2, sha2, blake3 |
| **Async Runtime** | Tokio (full features) |
| **Serialization** | Serde, Serde JSON, Serde Bytes, CBOR, Bincode |
| **Storage** | Sled (embedded KV), Dashmap |
| **Rate Limiting** | Governor, Tower-Governor |
| **JWT** | jsonwebtoken 9.2.0 |
| **CLI** | Clap 4.0 |
| **Logging** | Tracing, Tracing-subscriber |

### Frontend (TypeScript)
| Category | Technologies |
|----------|-------------|
| **Framework** | React 19, React Router 7 |
| **Build Tool** | Vite 7, TypeScript 5.8 |
| **Styling** | Tailwind CSS 3, Framer Motion |
| **HTTP Client** | Axios |
| **Testing** | Vitest, Playwright, React Testing Library |
| **Desktop** | Tauri 2.x |

### DevOps & Tooling
- **CI/CD:** GitHub Actions (Rust CI: test, lint, security audit)
- **Containerization:** Docker, Docker Compose
- **Pre-commit:** pre-commit hooks with Rustfmt, Clippy
- **API Testing:** Bruno collection, Newman (Postman)

---

## 🚀 Building and Running

### Prerequisites
- **Rust:** Stable toolchain (edition 2021)
- **Node.js:** v25+ (for frontend)
- **Git:** For version control

### Backend (Rust)
```bash
cd ghostwire

# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run with default features
cargo run --bin ghostwire --features web

# Run with specific features
cargo run --bin ghostwire --features "web,cli,mesh,security,stealth,bluetooth,qr"

# Run tests
cargo test --lib --tests --all-features

# Lint
cargo fmt --all -- --check
cargo clippy -- -D warnings

# Security audit
cargo audit
```

### Frontend (React)
```bash
cd webui

# Install dependencies
npm install

# Development server
npm run dev

# Production build
npm run build

# Run tests
npm test
npm run test:watch
npm run test:ui

# E2E tests
npx playwright test
```

### Full System
```bash
# From project root
# Start backend (debug mode)
./start-ghostwire-debug.sh

# Or start in background
./start-ghostwire-bg.sh

# Access Web UI
# http://localhost:8080  (or port configured in backend)
```

### Docker
```bash
# Build and run with Docker Compose
docker-compose up -d

# View logs
docker-compose logs -f ghostwire

# Stop
docker-compose down
```

### Feature Flags
The backend supports optional features via Cargo:
- `web` - Web UI and API server
- `cli` - Command-line interface
- `mesh` - libp2p mesh networking
- `security` - Security features (threat detection, encryption)
- `stealth` - Stealth TCP transport
- `bluetooth` - BLE support
- `qr` - QR code generation
- `matrix-bridge` - Matrix protocol bridge
- `meshtastic` - Meshtastic adapter
- `minimal` - Minimal feature set

---

## 🧪 Testing

### Backend Tests
```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --tests

# All features
cargo test --all-features

# With coverage (requires cargo-tarpaulin)
cargo tarpaulin --all-features --out Html
```

### Frontend Tests
```bash
# Unit tests
npm test

# Watch mode
npm run test:watch

# E2E tests
npx playwright test

# Test with UI
npm run test:ui
```

### API Testing
```bash
# Using Bruno collection
# Import bruno-collection/ into Bruno app

# Using Newman (Postman)
./test-api-newman.sh

# Backend CLI tests
./test-backend-cli.sh
```

---

## 📐 Development Conventions

### Code Style
- **Rust:** Follows Rustfmt defaults, enforced via CI
- **TypeScript:** ESLint + Prettier (config in webui/)
- **Commit Messages:** Conventional Commits format preferred

### Security Practices
- All cryptographic keys use `zeroize` for secure memory clearing
- Sensitive types wrapped with `secrecy` crate
- JWT tokens for API authentication
- Rate limiting on all endpoints (Governor)
- CORS configured for localhost development

### Branching Strategy
- `main` - Stable branch (protected)
- `develop` - Development branch
- Feature branches: `feature/<name>`
- Fix branches: `fix/<name>`

### Pull Request Process
1. Create feature branch from `develop`
2. Make changes with tests
3. Run linters locally (`cargo fmt`, `cargo clippy`, `npm run lint`)
4. Submit PR to `develop`
5. CI runs: tests, lints, security audit
6. Code review by maintainer
7. Merge after approval

### Licensing Requirements
- All new files must include copyright header:
  ```rust
  // Copyright (C) 2025-2026 Michael Irungu
  // SPDX-License-Identifier: AGPL-3.0-or-later
  ```
- Third-party dependencies must be AGPL-3.0 compatible
- No Commons Clause or similar restrictions
- Contributions are dual-licensed (AGPL-3.0 + Commercial)

---

## 📚 Documentation

### Key Documents
| File | Purpose |
|------|---------|
| `README.md` | Project overview and quick start |
| `docs/ARCHITECTURE.md` | System architecture deep dive |
| `docs/SECURITY.md` | Security policy and vulnerability reporting |
| `docs/CONTRIBUTING.md` | Contribution guidelines |
| `docs/CLA.md` | Contributor License Agreement |
| `LICENSE_EXPLANATION.md` | Dual licensing model explanation |
| `COMMERCIAL_LICENSE.md` | Commercial licensing terms |
| `NOTICE` | Cryptographic constants notice |
| `QUICK_START.md` | Live testing guide |

### API Documentation
- `docs/API_QUICK_REFERENCE.md` - API endpoints overview
- `docs/api/DEEP_DIVE.md` - Detailed API documentation
- `docs/PROFILE_IDENTITY_LIFECYCLE.md` - Profile/identity management

### Architecture Guides
- `docs/LIBP2P_NETWORK_CONNECTIVITY.md` - libp2p setup
- `docs/IMPLEMENTATION_GUIDE.md` - Implementation details
- `docs/presentations/` - Slide deck format guides

---

## 🔐 Security & Licensing

### Dual Licensing Model
GhostWire is dual-licensed:
1. **AGPL-3.0-or-later** - For open source, community, personal use
2. **Commercial License** - For proprietary, closed-source, SaaS deployments

**AGPL-3.0 Use Cases (Free):**
- Personal/research/educational use
- Open source projects (released under AGPL-3.0)
- Internal deployments with source disclosure

**Commercial License Required:**
- Proprietary products
- SaaS without source disclosure
- Commercial hosting services
- OEM integration

**Contact:** mirungu015@proton.me for commercial licensing

### Security Reporting
- **Email:** mirungu015@proton.me
- **GitHub:** Create private security advisory
- **DO NOT** create public issues for vulnerabilities

### Cryptographic Notice
The `NOTICE` file documents that GhostWire contains irreproducible cryptographic protocol constants derived from a two-party commitment ceremony (2025). These constants are required for network participation.

---

## 🎯 Current Development Focus

### Active Work
- ✅ Core components built (functional prototype)
- 🔄 Security hardening (patching vulnerabilities)
- 🔄 Test coverage increase (<10% → 50%+)
- ✅ Dual licensing framework established

### Roadmap
- Enhanced security features (threat detection)
- Mobile support (iOS/Android via Tauri)
- Plugin system (extensible architecture)
- UI/UX improvements
- Analytics dashboard
- Protocol adapters (Briar, Meshtastic, Matrix bridges)

---

## 📞 Contact & Support

**Author:** Michael Irungu  
**Email:** mirungu015@proton.me  
**Location:** Nairobi, Kenya  

**Community:**
- GitHub Issues: Bug reports, feature requests
- GitHub Discussions: General questions
- Security: Private reporting via email or GitHub Security Advisories

**Commercial Inquiries:** mirungu015@proton.me

---

## ⚠️ Important Notes for AI Assistants

1. **License Compliance:** Always preserve copyright headers when editing files
2. **Dual Licensing:** Remember contributions are dual-licensed
3. **Security First:** This is a security-critical project - don't introduce vulnerabilities
4. **Historical Archives:** `ghostwire-v*/`, `ghostwire-complete-v*/` directories are read-only snapshots
5. **Active Development:** Focus on `ghostwire/` and `webui/` directories
6. **No Commons Clause:** This restriction is incompatible with AGPL-3.0 (already researched)
7. **Cryptographic Constants:** Don't modify `genesis_constants.rs` values (network identity)

---

## 🔧 Common Tasks

### Add New API Endpoint
1. Add handler in `ghostwire/src/api_v2_endpoints.rs` or `ghostwire/src/v2_routes.rs`
2. Add type definitions in `ghostwire/src/api_v2_types.rs`
3. Update controller in `ghostwire/src/controllers/`
4. Add frontend service in `webui/src/services/`
5. Add tests

### Add New Transport
1. Implement transport trait in `ghostwire/src/core/transport.rs`
2. Add feature flag in `Cargo.toml`
3. Update adapter registry
4. Add configuration options
5. Document in `docs/`

### Update Frontend Component
1. Edit component in `webui/src/components/`
2. Update types in `webui/src/services/api.ts`
3. Test with `npm run test`
4. Build with `npm run build`

### Run Security Audit
```bash
cargo audit
npm audit
gh api /repos/Phantomojo/GhostWire-secure-mesh-communication/security-and-analysis
```

---

**End of QWEN.md**
