# GhostWire Project Overview & Master Reference

---

## 1. **Project Vision**
GhostWire aims to be the universal, modular, and privacy-focused mesh communication platform. It will bridge multiple mesh protocols (Bluetooth, WiFi, LoRa, WebRTC, TCP/IP, etc.), support cross-network channels, and provide robust security, privacy, and usability. The goal is to become the backbone for decentralized, censorship-resistant, and resilient communication—usable by anyone, anywhere, on any device.

---

## 2. **Current State (as of this document)**

### **A. Codebase Structure**
- **ghostwire/**: Rust core, CLI, and backend logic
  - `src/core/`: Core modules (encryption, identity, mesh, message, store, transport)
  - `src/cli/`: CLI commands
  - `src/web.rs`: Web API
  - `main.rs`: Entry point
- **webui/**: TypeScript/React web interface
  - `src/`: UI components, state, services
  - `src/services/`: API and WebSocket clients
- **webui/src-tauri/**: Tauri integration for desktop/mobile
- **meshtastic-rust/**, **meshtastic-web/**: (To be reviewed for integration or legacy)

### **B. Features Implemented**
- Basic mesh networking (Rust core)
- CLI for node management and messaging
- Web UI for chat, settings, onboarding
- Encryption primitives (in progress)
- Modular code structure for future transports
- Documentation: README, SECURITY, CONTRIBUTING, ROADMAP, etc.

### **C. Features Incomplete or Missing**
- Modular, pluggable transport layer (Bluetooth, WiFi, LoRa, WebRTC, etc.)
- Protocol adapters for bitchat, Briar, Meshtastic, etc.
- Universal relay/channel system
- Advanced privacy (cover traffic, forward secrecy, emergency wipe)
- Message compression, batching, deduplication
- Store-and-forward for offline delivery
- Adaptive power modes (for mobile)
- Comprehensive test coverage
- Robust error handling and diagnostics
- Mobile app wrappers (Tauri, React Native, etc.)

### **D. Known Issues & Technical Debt**
- Build errors and trait mismatches in Rust modules
- Incomplete or stubbed methods in core logic
- Struct field mismatches and import conflicts
- Web UI and backend API integration gaps
- Large codebase (2GB+) with legacy/unused files
- Documentation sometimes out of sync with code

---

## 3. **Architectural Vision**
- **Transport Abstraction:** All network transports are modular and hot-swappable.
- **Protocol Adapters:** Bridge messages between mesh protocols (bitchat, Briar, Meshtastic, etc.).
- **Universal Channels:** Channels span multiple networks and protocols, with deduplication and relay logic.
- **Security & Privacy:** End-to-end encryption, forward secrecy, cover traffic, emergency wipe, zero-knowledge group membership.
- **Open API/SDK:** REST/WebSocket API for third-party integration.
- **Modern UI/UX:** Responsive web/mobile UI, easy onboarding, QR invites.

---

## 4. **Development Principles**
- **Security First:** All code is reviewed for vulnerabilities and privacy risks.
- **Modularity:** New features are added as independent modules/crates.
- **Interoperability:** Prioritize compatibility with other mesh chat protocols.
- **Documentation:** All features and APIs are documented and kept up to date.
- **Community:** Encourage contributions, feedback, and open discussion.

---

## 5. **Ongoing Tasks & TODOs**
- [ ] Refactor transport layer for modularity
- [ ] Implement Bluetooth, WiFi, LoRa, WebRTC transports
- [ ] Add protocol adapters (bitchat, Briar, Meshtastic)
- [ ] Build universal relay/channel logic
- [ ] Enhance security (cover traffic, forward secrecy, emergency wipe)
- [ ] Add message compression, batching, deduplication
- [ ] Improve error handling and diagnostics
- [ ] Integrate and test mobile/desktop wrappers
- [ ] Audit and clean up legacy/unused code
- [ ] Expand and update documentation

---

## 6. **How to Use This Document**
- **Reference for all planning and implementation.**
- **Update as features are added, changed, or removed.**
- **Log major architectural or design decisions.**
- **Track known issues and technical debt.**
- **Ensure alignment with project vision and goals.**

---

## 7. **Next Steps**
- Deep-dive codebase audit: Identify dead code, legacy modules, and integration gaps.
- Prioritize modular transport refactor and protocol adapter design.
- Set up regular documentation and code review cycles.
- Engage community for feedback and contributions.

---

## 8. **2024+ Roadmap & Implementation Plan (Updated)**

### **A. Leverage Existing Technologies**
- **Networking:** Use libp2p for modular, multi-transport mesh networking (TCP, WebRTC, Bluetooth, etc.).
- **Overlay Mesh:** Integrate Yggdrasil or Cjdns for IPv6 mesh overlay (optional, advanced).
- **LoRa:** Interface with Meshtastic protocol/hardware for long-range comms.
- **Protocol Adapters:** Use/adapt Matrix bridges for interoperability (Matrix, IRC, XMPP, etc.).
- **Briar/Meshtastic:** Reuse open-source logic for offline queuing, group messaging, and file sharing.
- **Encryption:** Use Noise Protocol (snow crate), MLS (mls-rs), and hybrid post-quantum crypto (Kyber + X25519).
- **Store-and-Forward:** Optionally use IPFS/Dat for decentralized, offline message storage.
- **Web UI:** Use Tauri for compact desktop/mobile wrappers; React + Chakra/MUI for modern UI.

### **B. Advanced & Covert Features**
- **Pluggable Transports:** Integrate obfs4/meek for censorship resistance.
- **Traffic Obfuscation:** Implement cover traffic, timing randomization, and packet padding.
- **Steganography:** Prototype message hiding in media (images/audio).
- **Panic Wipe:** Secure memory zeroization and file deletion.

### **C. Compactness & Modularity**
- **Target Size:** <100MB for mobile/desktop core; advanced features as optional modules/plugins.
- **Feature Flags:** Use Rust features to enable/disable transports, protocols, and UI modules.
- **Dynamic Loading:** Design plugin interfaces for future expansion.

### **D. Testing, Automation, and Documentation**
- **Testing:** Use cargo-fuzz, proptest, and distributed simulation for robustness.
- **Diagnostics:** Integrate OpenTelemetry for tracing and diagnostics.
- **Automation:** CI/CD for builds, tests, and releases.
- **Docs:** Use mdBook for living documentation.

### **E. Community & Contribution**
- **Open Roadmap:** Maintain public roadmap and RFC process.
- **Modular Codebase:** Encourage third-party plugins and protocol adapters.

---

## 9. **Implementation Priorities & Milestones**

1. **Modular Transport Refactor**
   - Refactor networking to use libp2p and pluggable transports.
   - Integrate obfs4/meek for covert/censorship-resistant comms.
2. **Protocol Adapters**
   - Implement Matrix, Meshtastic, and Briar adapters.
   - Expose all features via CLI and Web API.
3. **Security Enhancements**
   - Add cover traffic, panic wipe, and hybrid crypto.
4. **Compactness & Platform Support**
   - Optimize binary/UI size; use feature flags for optional features.
   - Build Tauri wrappers for desktop/mobile.
5. **Testing & Automation**
   - Add fuzzing, property testing, and distributed simulation.
   - Set up CI/CD and automated docs.
6. **Documentation & Community**
   - Keep docs and roadmap up to date; encourage contributions.

---

## 10. **Size & Platform Strategy**
- **Mobile Target:** <100MB core app, with optional modules for advanced features.
- **Desktop Target:** <200MB with all features enabled.
- **Plugin System:** Allow power users to add advanced features as plugins.

---

**This document is the single source of truth for GhostWire’s direction, state, and priorities. Always update it as the project evolves.** 

---

## Codebase Audit Log (2024-xx-xx)

### **Dead/Legacy Code & Unused Modules**
- **core/store.rs**: `MessageCache` and `MessageStore` are not referenced in the main core, mesh, reticulum, or briar modules. These may be legacy or experimental and can likely be removed or refactored if not used elsewhere.
- **core/transport.rs**: `MockTransport` and `LocalTransport` are not used in the main application flow. The `MessageTransport` trait is not implemented by any production code. These are likely scaffolding or test code and should be reviewed for removal or moved to a test/dev module.
- **core/transport.rs (p2p mod)**: The `p2p` module is a placeholder and not integrated. Remove or implement as part of the modular transport refactor.
- **core/identity.rs**: `EphemeralIdentity` struct is not referenced in the main `Identity` struct or core logic. If not used for ephemeral sessions, consider removing or integrating.

### **Incomplete or Stubbed Features**
- **core/mod.rs**: Many methods in `Core` are stubbed or placeholders (e.g., `get_network_topology`, `start_mesh`, `get_mesh_topology`, `connect_meshtastic`, `connect_reticulum`, `get_reticulum_topology`). These need full implementations or should be removed if not required.
- **core/mesh.rs**: `MeshTransport`'s `send_message` and `receive_message` are stubbed. `get_topology` returns empty data. `find_route` and `update_connection_quality` are no-ops. These are critical for mesh operation and need implementation.
- **core/reticulum.rs**: Many advanced features (onion routing, store-and-forward, message relay) are stubbed or only partially implemented. Integration with the main core is incomplete.
- **core/briar.rs**: Contact discovery, message sending, and verification are present but not integrated with the main core or exposed via CLI/Web API.

### **Integration Gaps**
- **Transport Abstraction**: Only mesh and stealth TCP are implemented. No Bluetooth, WiFi, LoRa, or WebRTC transports are present. The `Transport` trait is not fully leveraged.
- **Protocol Adapters**: No actual adapters for bitchat, Briar, or Meshtastic exist. The architecture is ready, but implementation is missing.
- **Store-and-Forward**: Not fully implemented in any transport or protocol manager.
- **Web API**: Not all features (especially advanced mesh/reticulum/briar operations) are exposed via the web API.
- **Testing**: No evidence of comprehensive integration or unit tests for these modules.

### **Recommendations**
- **Remove or refactor**: Unused structs, traits, and modules (store.rs, transport.rs scaffolding, EphemeralIdentity) unless needed for future features.
- **Prioritize implementation**: Complete all stubbed methods in core, mesh, reticulum, and briar modules.
- **Modularize transports**: Refactor all networking to use the `Transport` trait and plan for new transports.
- **Implement protocol adapters**: Begin with bitchat or Meshtastic for maximum impact.
- **Expose all features via API**: Ensure CLI and Web API can access all core functionality.
- **Add tests**: Write integration and unit tests for all modules.

---

### **2024-xx-xx: Major Transport Registry Refactor & Build Fixes**
- Refactored `TransportRegistry` to store `Arc<tokio::sync::Mutex<dyn Transport>>` for safe, mutable, async access to all registered transports.
- Updated all usages of the transport registry and `active_transport` to lock the mutex before calling mutating methods (e.g., `send_message`).
- Fixed architectural issue where `send_message` required `&mut self` but registry used `Arc<dyn Transport>`, which only allowed immutable access.
- Updated TCP transport instantiation in mesh networking to use `TcpTransport::<libp2p_tcp::tokio::Tcp>::new(libp2p_tcp::Config::default().nodelay(true))` as required by libp2p 0.44.0.
- Ensured all trait objects are `Send + Sync` and compatible with async/multithreaded use.
- All build errors related to transport mutability, trait bounds, and libp2p provider types are resolved. Project builds cleanly.
- This refactor future-proofs the codebase for modular, pluggable, async transports and multithreaded runtime.

*This audit log will be updated as the codebase is cleaned up and refactored. All future code cleanup and planning should reference this section.* 