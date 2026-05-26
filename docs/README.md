# GhostWire Documentation

> Central hub for all GhostWire documentation.

---

## 📖 Getting Started

| Document | Description |
|----------|-------------|
| **[Getting Started](GETTING_STARTED.md)** | Install, configure, run, and troubleshoot GhostWire in one guide |

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

## Architecture

System design, interfaces, and component documentation.

| Document | Description |
|----------|-------------|
| **[Architecture](architecture/ARCHITECTURE.md)** | System internals, component diagram, message lifecycle |
| **[Architecture Research](architecture/RESEARCH.md)** | Design research and decisions |
| _(CLI, TUI, and Web UI content is documented in the main README and ghostwire README)_ | |

---

## Security

| Document | Description |
|----------|-------------|
| **[Security Policy](security/SECURITY.md)** | Vulnerability reporting, security model, best practices |
| **[Security Audit](security/SECURITY_AUDIT.md)** | Full audit report — all critical issues resolved |
| **[Audit Summary](security/AUDIT_SUMMARY.md)** | Executive summary of security audit findings |
| **[Audit Report 2026](security/SECURITY_AUDIT_REPORT_2026.md)** | Detailed security audit report |
| **[API Security](security/API_SECURITY.md)** | API authentication and authorization patterns |

---

## API Reference

Full REST API and WebSocket documentation.

| Document | Description |
|----------|-------------|
| **[API Reference](development/build/API_REFERENCE.md)** | Complete REST API + WebSocket event reference |
| **[API Testing Guide](api/API_TESTING_GUIDE.md)** | Comprehensive curl examples for every endpoint |
| **[API Quick Reference](api/API_QUICK_REFERENCE.md)** | Cheat-sheet of common API calls |
| **[API Versioning](api/API_VERSIONING.md)** | Versioning policy and deprecation strategy |
| **[Postman Guide](api/POSTMAN_API_GUIDE.md)** | Postman collection usage |
| **[Backend Communication Test](api/BACKEND_COMMUNICATION_TEST.md)** | Multi-node test walkthrough |
| **[API Deep Dive](api/DEEP_DIVE.md)** | Detailed API internals |

---

## Building on GhostWire

Guides for developers building applications on top of the GhostWire Protocol.

| Document | Description |
|----------|-------------|
| **[Overview](development/build/OVERVIEW.md)** | What you can build, protocol vs implementation distinction |
| **[Quick Start](development/build/QUICKSTART.md)** | Add GhostWire to a Rust project, send your first mesh message |
| **[Embedding Guide](development/build/EMBEDDING.md)** | Run GhostWire as a background service in your application |
| **[Custom Transport](development/build/CUSTOM_TRANSPORT.md)** | Implement the Transport trait for new networking mediums |
| **[Protocol Bridges](development/build/PROTOCOL_BRIDGES.md)** | Build bridges to Matrix, Meshtastic, Briar, Reticulum |

---

## Development

Developer workflow, testing, and contribution guides.

| Document | Description |
|----------|-------------|
| **[Contributing](development/CONTRIBUTING.md)** | How to contribute to the project |
| **[Governance](development/GOVERNANCE.md)** | GIP process, versioning, decision making |
| **[CLA](development/CLA.md)** | Contributor License Agreement |
| **[Brand Guidelines](development/BRAND_GUIDELINES.md)** | Branding and identity guidelines |
| **[Branding Integration](development/BRANDING_INTEGRATION.md)** | Brand implementation guide |
| **[libp2p Connectivity](development/LIBP2P_NETWORK_CONNECTIVITY.md)** | Network connectivity reference |
| **[Vibe Setup](development/VIBE_SETUP_GUIDE.md)** | Development environment setup |

---

## Knowledge Base

Deep dives, reference materials, and project knowledge.

| Document | Description |
|----------|-------------|
| **[Wiki](knowledge/wiki/index.md)** | Community wiki — getting started, FAQ, architecture, security |
| **[GhostWire Codebase](knowledge/wiki/GhostWire_Codebase.md)** | 7000+ line generated codebase reference (LLM-optimized) |
| **[Comprehensive Code Review](knowledge/COMPREHENSIVE_CODE_REVIEW.md)** | 1500+ line code review document |
| **[Implementation Guide](knowledge/IMPLEMENTATION_GUIDE.md)** | Implementation journal and log |
| **[Hardware BOM](knowledge/HARDWARE_SPEC_BOM.md)** | Hardware bill of materials |
| **[30-Day libp2p Curriculum](knowledge/protocol-knowledge/30_DAY_LIBP2P_CURRICULUM.md)** | Structured libp2p learning path |
| **[Protocol Deep Dive](knowledge/protocol-knowledge/PROTOCOL_DEEP_DIVE.md)** | libp2p stack, cryptography, stealth transport |
| **[Global Knowledge Sources](knowledge/GLOBAL_KNOWLEDGE_SOURCES.md)** | External knowledge references |
| **[GhostWire Manifesto](knowledge/GHOSTWIRE_MANIFESTO.md)** | Project vision and manifesto |
| **[Manifesto Framework](knowledge/MANIFESTO_FRAMEWORK.md)** | Manifesto development framework |
| **[Recommendations](knowledge/RECOMMENDATIONS.md)** | Project recommendations |
| **[Review Summary](knowledge/REVIEW_SUMMARY.md)** | Code review summary |
| **[First Ping](knowledge/FIRST_PING.md)** | First cross-machine mesh communication record |
| **[Changelog](knowledge/CHANGELOG.md)** | Version history |

---

## Deployment

Production deployment and operations.

| Document | Description |
|----------|-------------|
| **[Multi-Node Deployment](api/BACKEND_COMMUNICATION_TEST.md)** | Multi-machine mesh deployment |
| **[ML Pipeline](deployment/ML_PIPELINE.md)** | ML model deployment pipeline |
| **[Installer Enhancements](deployment/INSTALLER_ENHANCEMENTS.md)** | Installer development notes |

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

## Legacy

Historical documentation preserved for reference.

| Directory | Contents |
|-----------|----------|
| **[legacy/](legacy/)** | 6 historical docs from earlier development phases |

---

## Archive

Deprecated or superseded documentation.

| Directory | Contents |
|-----------|----------|
| **[archive/](archive/)** | Merged, replaced, or out-of-date documents |

---

**Last Updated:** May 26, 2026
**Maintained By:** Team GhostWire
