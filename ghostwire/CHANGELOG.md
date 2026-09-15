# Changelog

All notable changes to GhostWire will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Codeberg Migration** (2026-09-14):
  - Public repo moved from GitHub to Codeberg (nonprofit, no AI training)
  - All links updated across README, website, docs
- **Mailbox Fail-Closed Boot** (2026-09-12):
  - Removed `Default` trait from `MailboxStore` — node now requires encryption keys at startup
  - If keys missing, node panics instead of silently running unencrypted
- **gw-* Crate Ports** (2026-09-09 to 2026-09-12):
  - `gw-reliability`: AckTracker, RetryQueue, TTL-based delivery (8/8 tests pass)
  - `gw-routing`: peer_table, peer_health, Skademlia DHT (20/20 tests pass)
  - gw-pipe, gw-vault, gw-identity, gw-trust, gw-discovery, gw-quic, gw-msg: ported to source files from newest/ checkout (not yet standalone crates)
- **Security Hardening** (2026-07-03):
  - Pinned minimum safe versions: curve25519-dalek >=4.1.3 (CVE-2024-58262), aes-gcm >=0.10.3 (CVE-2023-42811), reqwest >=0.11.23 (CVE-2025-0805)
  - X25519 small-order point defense: all-zero DH output check after key exchange (CVE-2025-62514)
  - USBGuard: permanently allowed CASUE USB KB (2a7a:959f)
  - SECURITY.md: documented open findings (pqcrypto-mlkem unmaintained, no CI cargo-audit)
- **Peer Health Monitoring** (2026-07-03):
  - `PeerHealthTracker`: ping/pong with RTT measurement, rolling 60-sample window
  - `PendingPings`: timestamp-keyed ping tracking with stale cleanup
  - Periodic pings every 2s in MeshDriver event loop
  - Incoming Ping → respond with Pong; incoming Pong → resolve RTT
  - `mesh_health.rs` module with health metrics
  - `peer_health()` accessor on MeshManager
- **Documentation Overhaul** (2026-07-03):
  - New: INSTALL.md, CONFIGURATION.md, DEPLOYMENT.md, TROUBLESHOOTING.md, DESIGN_DECISIONS.md
  - Rewritten: README.md, ARCHITECTURE.md, DOCUMENTATION_INDEX.md
  - Updated: SECURITY.md, THREAT_MODEL.md, KEY_MANAGEMENT.md
  - Archived notebookllm/ (38 files) to docs/archive/notebookllm/; 3 unique files extracted to docs/
- **Phase 1: TUI & Backend Integration Fixes** (11 fixes, 2026-06-24)
  - Wired TUI `get_messages()` to real `/api/v2/messages` endpoint (daemon_client.rs)
  - Added auth token support with `authenticated_get`/`authenticated_post` (daemon_client.rs)
  - Fixed `peer_count` field name mismatch with `active_peers` fallback (daemon_client.rs)
  - PeerController now returns real peer data from mesh (peer_controller.rs)
  - Added `peer_count` alias to NetworkController (all_controllers.rs)
  - Removed `/api/send` from auth whitelist — now requires authentication (web.rs)
  - Uncommented v2 send/broadcast routes (web.rs)
  - Messages load on contact Enter key in TUI (app.rs)
  - Added structured `tracing::warn!` to all TUI daemon error paths (daemon_client.rs)
  - Made `refresh_data()` concurrent with `tokio::join!` (app.rs)
  - Replaced hardcoded AI status with live `get_ai_status_summary()` (app.rs, daemon_client.rs)
- **Phase 2: Compile Error Cleanup** (25 errors → 0, 2026-06-24)
  - Downgraded `rand` from 0.9 to 0.8 (fixed 14 errors: rand + Distribution compat)
  - Commented out BLE routes + gated bluetooth module with `#[cfg(feature = "bluetooth")]`
  - Added `anyhow!` macro import in `coercion.rs`
  - Fixed `DtnBundle` import path (`dtn::epidemic` → `dtn::bundle`)
  - Fixed `ring::hkdf::Prk::expand` API for ring 0.17 (info slice + HKDF_SHA256 len)
  - Replaced `publish_to_topic` (nonexistent) with `tracing::warn!` stub
  - Added missing `threshold` argument to `recover_identity_secret` call
- Comprehensive documentation suite
- Security policy and code of conduct
- Development roadmap and changelog
- AGPL-3.0 License (Fixed empty file issue)
- **Security Hardening**:
  - Removed plaintext `ghostwire_token.txt` from repository
  - Updated `SECURITY.md` and `AUDIT_SUMMARY.md` to reflect current security posture
  - Patched critical vulnerabilities (Matrix SDK, Ring, pqcrypto replacement)
- **Decentralized Profile Management**: Secure first-time setup with password enforcement
  - New `/api/identity/setup` endpoint for secure profile initialization
  - `setup_complete` flag to trigger the "Sovereign Identity Phase" wizard
  - Automatic unlock with default password and re-encryption with user's new password
  - Refactored `IdentityController` to remove legacy `username.txt` dependency
- **Rate Limit Optimization**: Increased API rate limits (200/s, burst 100) to support frontend initialization

### Changed
- Refactored `TransportRegistry` to use `Arc<tokio::sync::Mutex<dyn Transport>>` for async, mutable access to all transports.
- Updated all code to lock the mutex before calling mutating methods on transports.
- Fixed architectural issue where `send_message` required `&mut self` but registry used `Arc<dyn Transport>`, which only allowed immutable access.
- Updated TCP transport instantiation in mesh networking to use the correct libp2p provider and config for compatibility with libp2p 0.44.0.
- All build errors related to transport mutability, trait bounds, and libp2p provider types are resolved. Project builds cleanly.
- Updated project structure and organization
- Enhanced documentation and guides

### Fixed
- **25 compile errors eliminated** (Phase 2): rand compatibility, missing imports, wrong module paths, API changes in ring 0.17, missing function arguments, nonexistent method calls
- Various documentation and formatting issues

## [0.1.0] - 2024-01-15

### Added
- **Core Architecture**: Complete project structure with modular design
- **Identity Management**: Ed25519/X25519 key pair generation and management
  - Ephemeral identity support
  - Identity verification and proof generation
  - Secure key storage and backup

- **Encryption Engine**: AES-256-GCM encryption implementation
  - Hybrid encryption (symmetric + asymmetric)
  - Key derivation with PBKDF2
  - Message authentication with HMAC-SHA256
  - Perfect forward secrecy support

- **Mesh Networking**: libp2p-based peer-to-peer networking
  - GossipSub protocol for message propagation
  - mDNS local network discovery
  - Kademlia DHT for global peer discovery
  - Network topology management

- **Stealth TCP Transport**: Custom transport layer with advanced security
  - Stealth handshake protocol
  - Connection obfuscation and timing randomization
  - Rate limiting and threat scoring
  - IP allowlisting and blacklisting

- **Security Manager**: Comprehensive security monitoring and threat detection
  - Real-time threat detection and response
  - Security policy enforcement
  - Audit logging and event correlation
  - Incident response automation

- **CLI Interface**: Command-line interface with full functionality
  - Identity management commands
  - Message sending and receiving
  - Network management and monitoring
  - Security configuration and monitoring

- **Web Interface**: Modern web-based user interface
  - RESTful API endpoints
  - Real-time security monitoring
  - Network topology visualization
  - Message center and peer management

- **Reticulum Integration**: Inspired by Reticulum protocol
  - LoRa radio communication support
  - Store-and-forward message delivery
  - Resource management and optimization
  - Multi-hop routing capabilities

- **Briar Integration**: Inspired by Briar protocol
  - Contact-based messaging
  - Offline message queuing
  - Group messaging support
  - File sharing capabilities

### Technical Features
- **Async Architecture**: Full async/await implementation with tokio
- **Error Handling**: Comprehensive error handling with anyhow
- **Serialization**: Serde integration for data serialization
- **Configuration**: TOML-based configuration system
- **Logging**: Structured logging with tracing
- **Testing**: Comprehensive test suite with high coverage

### Security Features
- **Cryptographic Security**: Industry-standard cryptographic algorithms
- **Network Security**: Advanced network security and privacy features
- **Threat Detection**: Multi-layered threat detection and response
- **Audit Logging**: Comprehensive security event logging
- **Key Management**: Secure key generation, storage, and rotation

### Dependencies
- **libp2p**: Peer-to-peer networking framework (v0.56.0)
- **tokio**: Asynchronous runtime
- **ring**: Cryptographic primitives
- **aes-gcm**: AES-GCM encryption
- **axum**: Web framework
- **serde**: Serialization
- **anyhow**: Error handling
- **tracing**: Logging and diagnostics

### Vendored Dependencies
- **libp2p-mdns**: Local patches for compatibility with libp2p v0.56.0
  - Made Provider trait and module public
  - Fixed import paths and API compatibility
  - Ensured mDNS functionality works with latest libp2p

## [0.0.1] - 2024-01-01

### Added
- Initial project setup and structure
- Basic Rust project configuration
- Core module organization
- Development environment setup

### Technical Foundation
- Cargo.toml configuration
- Basic source code structure
- Development tools and utilities
- Initial documentation

---

## Version History

### Version 0.1.0 (Current)
- **Status**: Stable Release
- **Focus**: Core functionality and security features
- **Target**: Production-ready core system

### Version 0.0.1 (Initial)
- **Status**: Development Release
- **Focus**: Project foundation and setup
- **Target**: Development environment and basic structure

## Release Notes

### Version 0.1.0 Release Notes

#### Breaking Changes
- None (first major release)

#### New Features
- Complete mesh networking implementation
- Advanced security features and threat detection
- Full CLI and web interfaces
- Comprehensive encryption and identity management

#### Security Improvements
- Industry-standard cryptographic implementation
- Advanced threat detection and response
- Secure key management and rotation
- Comprehensive audit logging

#### Performance Improvements
- Optimized async/await implementation
- Efficient memory management
- High-performance networking stack
- Minimal resource usage

#### Bug Fixes
- Fixed libp2p compatibility issues
- Resolved dependency conflicts
- Corrected API usage patterns
- Fixed serialization issues

#### Documentation
- Comprehensive API documentation
- User guides and tutorials
- Security documentation
- Development guidelines

### Known Issues

#### Version 0.1.0
- Some advanced libp2p features may require additional configuration
- Web interface requires modern browser with WebSocket support
- CLI interface may have limited functionality on Windows
- Performance may vary on resource-constrained systems

#### Planned Fixes
- Enhanced cross-platform compatibility
- Improved error handling and recovery
- Better performance on low-end devices
- More comprehensive testing coverage

## Migration Guide

### From Version 0.0.1 to 0.1.0
- No migration required (first major release)
- New configuration options available
- Enhanced security features enabled by default
- Improved API and interface design

## Support

### Version Support
- **Current Version**: 0.1.0 (Full Support)
- **Previous Versions**: Limited support
- **Future Versions**: Planned support

### Support Channels
- **Codeberg Issues**: Bug reports and feature requests
- **Documentation**: Comprehensive guides and tutorials
- **Security**: Direct security contact for vulnerabilities

---

**Note**: This changelog is maintained alongside the project and will be updated with each release. For detailed information about specific changes, please refer to the commit history and release notes. 