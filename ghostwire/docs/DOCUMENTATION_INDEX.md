# GhostWire Documentation Index

> **Last Updated**: 2026-07-03 | **Project Version**: 0.2.0

---

## Quick Navigation

| Category | Document | Status | Location |
|----------|----------|--------|----------|
| **Getting Started** | README | ✅ Current | `ghostwire/README.md` |
| | Install Guide | ✅ Current | `docs/INSTALL.md` |
| | Configuration | ✅ Current | `docs/CONFIGURATION.md` |
| **Architecture** | Architecture Guide | ✅ Current | `docs/ARCHITECTURE.md` |
| | Design Decisions | ✅ Current | `docs/DESIGN_DECISIONS.md` |
| **Security** | Security Policy | ✅ Current | `ghostwire/SECURITY.md` |
| | Threat Model | ✅ Current | `docs/THREAT_MODEL.md` |
| | Key Management | ✅ Current | `docs/KEY_MANAGEMENT.md` |
| **Messaging** | Message Delivery Spec | ✅ Current | `docs/MESSAGE_DELIVERY_SPEC.md` |
| | Privacy Tiers | ✅ Current | `docs/PRIVACY_TIERS.md` |
| | Device Tiers | ✅ Current | `docs/DEVICE_TIERS.md` |
| **API** | API Documentation | ✅ Current | `docs/API.md` |
| **Deployment** | Deployment Guide | ✅ Current | `docs/DEPLOYMENT.md` |
| | Troubleshooting | ✅ Current | `docs/TROUBLESHOOTING.md` |
| **Development** | Contributing | ✅ Current | `ghostwire/CONTRIBUTING.md` |
| | Code of Conduct | ✅ Current | `ghostwire/CODE_OF_CONDUCT.md` |
| | Changelog | ✅ Current | `ghostwire/CHANGELOG.md` |
| | Roadmap | ✅ Current | `ghostwire/ROADMAP.md` |
| **Performance** | Performance Benchmarks | ✅ Current | `docs/Performance_Benchmarks.md` |
| **Reference** | Evolution Diagram | ✅ Current | `docs/GhostWire_Evolution_Diagram.md` |
| | Tauri Mobile Guide | ✅ Current | `docs/Tauri_Mobile_App_Guide.md` |

---

## Documentation Standards

### File Naming

- Use `UPPER_SNAKE_CASE.md` for root-level docs
- Use `lowercase-with-hyphens.md` for subdirectory docs (except legacy UPPER_SNAKE_CASE files)
- Prefix audit docs with `AUDIT_` or `SECURITY_AUDIT_`

### Content Standards

- Every doc must have a `Last Updated` date
- Architecture diagrams use ASCII art (no external image dependencies)
- Code examples must compile (no placeholder/snippet-only examples)
- Security docs must reference specific CVEs where applicable

### Maintenance Schedule

- **Monthly**: Review ROADMAP.md for accuracy
- **Per release**: Update CHANGELOG.md, bump version badges
- **Quarterly**: Review SECURITY.md, THREAT_MODEL.md
- **Ad hoc**: Update docs when adding/removing modules

---

## Missing Documentation (Candidates for Creation)

| Document | Priority | Description |
|----------|----------|-------------|
| **Wire Protocol Spec** | HIGH | Message framing, byte layout, version negotiation. NOTE: wire_protocol.rs exists with full binary frame format (magic, kind, payload_len, bincode, BTP stream encryption, replay protection, cover traffic). No standalone spec document yet — implementation is the spec. |
| **LINDDUN Privacy Analysis** | MEDIUM | STRIDE + LINDDUN privacy threat extension |
| **Formal Verification** | LOW | Tamarin prover specs for crypto protocols |
| **SDK Reference** | LOW | Auto-generated API reference via rustdoc |
