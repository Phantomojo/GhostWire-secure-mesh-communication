# Changelog

All notable changes to GhostWire will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### September 2026

#### Mailbox Fail-Closed Boot (2026-09-12)
- Removed `Default` trait from `MailboxStore` — node now requires encryption keys at startup
- If keys missing, node panics instead of silently running unencrypted
- Files changed: `gw-pipe/src/mailbox.rs`, `gw-pipe/src/mailbox/mod.rs`

#### gw-* Crate Ports (2026-09-09 to 2026-09-12)
- `gw-reliability`: AckTracker, RetryQueue, TTL-based delivery — standalone crate, 8/8 tests pass
- `gw-routing`: peer_table, peer_health, Skademlia DHT — standalone crate, 20/20 tests pass
- gw-pipe, gw-vault, gw-identity, gw-trust, gw-discovery, gw-quic, gw-msg: ported to source files from newest/ checkout (not yet standalone crates)

#### Codeberg Migration (2026-09-14)
- Public repo moved from GitHub to Codeberg (nonprofit, no AI training)
- All links updated across README, website, docs

### July 2026

#### Security Hardening (2026-07-03)
- Pinned minimum safe versions: curve25519-dalek >=4.1.3, aes-gcm >=0.10.3, reqwest >=0.11.23
- X25519 small-order point defense: all-zero DH output check after key exchange
- PeerHealthTracker: ping/pong with RTT measurement, rolling 60-sample window
- Documentation overhaul: INSTALL.md, CONFIGURATION.md, DEPLOYMENT.md, TROUBLESHOOTING.md, DESIGN_DECISIONS.md

### June 2026

#### Phase 1: TUI & Backend Integration (2026-06-24)
- Wired TUI to real `/api/v2/messages` endpoint
- Added auth token support
- PeerController returns real peer data
- Removed `/api/send` from auth whitelist

#### Phase 2: Compile Error Cleanup (2026-06-24)
- 25 compile errors eliminated: rand compatibility, missing imports, wrong module paths, API changes in ring 0.17

### January 2026

#### v0.1.0 — Initial Release
- Core architecture: Ed25519/X25519 identity, AES-256-GCM encryption, libp2p mesh networking
- Stealth TCP transport with connection obfuscation
- CLI and Web interfaces
- Reticulum-inspired LoRa support
- AGPL-3.0 license

### Known Limitations
- Not independently audited yet
- Strong privacy, not full anonymity (traffic analysis risk)
- Physical device seizure may expose local data
- gw-* crate ports: only gw-reliability and gw-routing are standalone crates with passing tests
