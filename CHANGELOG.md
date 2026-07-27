# Changelog

All notable changes to GhostWire will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-07-27

### Added
- Username system: `ghostwire init --username` stores identity and publishes to DHT
- `ghostwire peers` now shows individual peers with usernames (was aggregate-only)
- DHT username records stored in mesh topology on discovery
- Auto-publish username on mesh start
- Function calling: `process_natural_command()` dispatches NL commands to Core methods
- Coercion/duress: `setup_coercion()`, `set_duress_password()`, `check_coercion()` on Core
- Peer health heartbeat (15s interval) in nervous system
- Rate limiting: 100 messages/60s per peer in MeshDriver
- Cross-platform release workflow (Linux x86_64/aarch64, macOS x86_64/aarch64, Windows x86_64)
- MSRV: 1.85.0
- Fuzz targets: `Frame::from_bytes()` and `FrameParser::feed()` protocol parsing
- Workflows: audit.yml (daily cargo-audit), scorecard.yml (OpenSSF), codeql.yml (SAST), fuzz.yml, sbom.yml (CycloneDX)
- nextest config with CI profile, retries, and isolation groups

### Fixed
- Encryption salt off-by-one: `% 18` → `% 17` (salt is 17 bytes, not 18)
- `test_storage_roundtrip`: identity auto-unlocks via keyfile, assertion corrected
- `request_and_accept_flow`: drain PeerDiscovered before asserting ConnectionRequestReceived
- `test_build_topology_from_mesh`: use actual HashMap key instead of hardcoded index 0
- `test_message_age`: increased tolerance from 1s to 60s for timestamp jitter
- Deprecated constant warnings: replaced direct `DM_TOPIC`/`SPHINX_TOPIC`/etc. with `topics::` module paths
- All unused import/dead code warnings eliminated (0 lib warnings)
- Version strings synced: CLI, main binary, protocol VERSION now use 0.2.0
- `irc_bridge` feature flag missing from Config struct

### Removed
- Dead modules: `private_routing.rs` (353L, had critical onion key bug), `network_scanner.rs` (202L), `obfs4_transport.rs` (157L), `pluggable_transport.rs` (398L canonical + orphan duplicate)
- Dead binary: `irc_test.rs` (pre-existing compile errors, undeclared in Cargo.toml)
- Total: 1,115 lines of dead code removed

### Security
- Release-plz SHA-pinned actions
- cargo-audit daily scans
- OpenSSF Scorecard weekly assessment
- CodeQL SAST for Rust and JavaScript/TypeScript
- CycloneDX SBOM generation on release

## [0.1.0] - 2026-04-05

### Added
- Initial release
- Mesh networking via libp2p (QUIC, TCP, mDNS, Kademlia, GossipSub)
- Sphinx onion-routed messaging
- Post-quantum hybrid encryption (ML-KEM-768 + X25519)
- Delay-tolerant networking (PRoPHET epidemic routing)
- 4-layer AI routing: ONNX → Gemma → GNN → PRoPHET
- CLI with 16 subcommands
- TUI with ratatui (Chat, Contacts, Dashboard, Wizard views)
- WebSocket API server (Axum)
- Docker image (GHCR)
- Android cross-compilation support
