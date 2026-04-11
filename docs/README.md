# GhostWire Documentation

> Central hub for all GhostWire documentation.

---

## Protocol Specifications

Formal specifications that define the GhostWire Protocol — any implementation that follows these specs is protocol-compatible.

| Document | Description |
|----------|-------------|
| **[Protocol Overview](spec/PROTOCOL.md)** | GhostWire Protocol v0.2.0 — layer architecture, comparison to other protocols |
| **[Wire Format](spec/WIRE_FORMAT.md)** | Frame structure, serialization, version negotiation, replay protection |
| **[Cryptography](spec/CRYPTO.md)** | XChaCha20-Poly1305, X25519, Ed25519, ML-KEM-768, forward secrecy, key management |
| **[Sphinx Routing](spec/SPHINX.md)** | Onion routing spec — hop structure, encryption layers, SURBs, cover traffic |
| **[Peer Discovery](spec/DISCOVERY.md)** | mDNS, Kademlia DHT, rendezvous points, device pairing, manual introduction |
| **[AI Routing](spec/ROUTING.md)** | 4-layer ML stack — LightGBM, Gemma 4, GNN, PRoPHET fallback |
| **[Delay-Tolerant Networking](spec/DTN.md)** | PRoPHET epidemic routing, Spray-and-Wait, SyncBloom, buffer management |
| **[Identity](spec/IDENTITY.md)** | DID format, key management, device pairing, Shamir recovery, coercion resistance |
| **[Social Protocol](spec/SOCIAL.md)** | Connection state machine, Web of Trust, trust scoring, event bus |
| **[Pluggable Transports](spec/TRANSPORT.md)** | Transport interface, censorship resistance, traffic armor, auto-switching |

---

## Building on GhostWire

Guides for developers who want to build applications on the GhostWire Protocol.

| Document | Description |
|----------|-------------|
| **[Overview](build/OVERVIEW.md)** | What you can build, protocol vs implementation distinction |
| **[Quick Start](build/QUICKSTART.md)** | Add GhostWire to a Rust project, send your first mesh message |
| **[Embedding Guide](build/EMBEDDING.md)** | Run GhostWire as a background service in your application |
| **[Custom Transport](build/CUSTOM_TRANSPORT.md)** | Implement the Transport trait for new networking mediums |
| **[Protocol Bridges](build/PROTOCOL_BRIDGES.md)** | Build bridges to Matrix, Meshtastic, Briar, Reticulum |
| **[API Reference](build/API_REFERENCE.md)** | Complete REST API + WebSocket event reference |

---

## Protocol Governance

| Document | Description |
|----------|-------------|
| **[Governance](GOVERNANCE.md)** | GhostWire Implementation Proposals (GIPs), versioning, decision making |

---

## Implementation Guides

Documentation for the GhostWire reference implementation (Rust).

| Document | Description |
|----------|-------------|
| **[Architecture](ARCHITECTURE.md)** | System internals, component diagram, message lifecycle |
| **[Installation](INSTALL.md)** | Detailed setup for Linux, Docker, from source |
| **[Quick Start](QUICK_START.md)** | Get running in 5 minutes |
| **[How to Start](HOW_TO_START_GHOSTWIRE.md)** | Running GhostWire — CLI flags, configuration |
| **[CLI](CLI.md)** | Command-line interface reference |
| **[TUI](TUI.md)** | Terminal user interface guide |
| **[Web UI](WEB_UI.md)** | Web interface documentation |
| **[Testing](TESTING.md)** | Running tests, writing new tests |
| **[Development](DEVELOPMENT.md)** | Development workflow, hot-reloading, debugging |

---

## Security

| Document | Description |
|----------|-------------|
| **[Security Policy](SECURITY.md)** | Vulnerability reporting, security model, best practices |
| **[Security Audit](SECURITY_AUDIT.md)** | Full audit report — all critical issues resolved |
| **[Audit Summary](AUDIT_SUMMARY.md)** | Executive summary of security audit findings |

---

## Protocol Knowledge

| Document | Description |
|----------|-------------|
| **[Protocol Deep Dive](protocol-knowledge/PROTOCOL_DEEP_DIVE.md)** | libp2p stack, cryptography, stealth transport, mesh networking details |

---

## Project Information

| Document | Description |
|----------|-------------|
| **[First Ping](FIRST_PING.md)** | First cross-machine mesh communication record (April 5, 2026) |
| **[Changelog](CHANGELOG.md)** | Version history |
| **[Roadmap](ROADMAP.md)** | Development roadmap |
| **[Contributing](CONTRIBUTING.md)** | How to contribute to the project |
| **[Code of Conduct](CODE_OF_CONDUCT.md)** | Community guidelines |

---

## Presentations

Slide decks for talks and presentations about GhostWire.

| Document | Description |
|----------|-------------|
| [01: Project Overview](presentations/01_project_overview.md) | Introduction to GhostWire |
| [02: Getting Started](presentations/02_getting_started.md) | Installation and first run |
| [03: Architecture](presentations/03_architecture.md) | System design |
| [04: Transports](presentations/04_transports.md) | Networking transports |
| [05: Security](presentations/05_security.md) | Security model |
| [06: Adapters](presentations/06_adapters.md) | Protocol bridges |
| [07: FAQ](presentations/07_faq.md) | Frequently asked questions |
| [08: Contributing](presentations/08_contributing.md) | How to contribute |
| [09: Deployment & Operations](presentations/09_deployment_operations.md) | Production deployment |
| [10: Advanced Security](presentations/10_advanced_security.md) | Deep security topics |
| [11: Performance & Scaling](presentations/11_performance_scaling.md) | Performance optimization |
| [12: Developer Guide](presentations/12_developer_guide.md) | Building on GhostWire |
| [13: Case Studies](presentations/13_case_studies.md) | Real-world deployments |
| [14: Glossary](presentations/14_glossary_reference.md) | Terminology reference |

---

**Last Updated:** April 12, 2026
**Maintained By:** Team GhostWire
