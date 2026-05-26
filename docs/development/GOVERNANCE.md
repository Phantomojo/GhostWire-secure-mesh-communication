# GhostWire Governance

This document describes how the GhostWire protocol evolves, how decisions are made, and how contributors can propose changes.

---

## GhostWire Implementation Proposals (GIPs)

GhostWire uses **GIPs** (GhostWire Implementation Proposals) to evolve the protocol, similar to Nostr's NIPs or Python's PEPs. A GIP is a design document that provides information to the GhostWire community about a new feature, protocol change, or process.

### GIP Lifecycle

```
┌─────────┐    ┌────────────┐    ┌───────────┐    ┌──────────────┐    ┌────────┐
│  Draft  │───>│ Discussion │───>│ Accepted  │───>│ Implemented  │───>│ Final  │
│         │    │  (14 days) │    │           │    │              │    │        │
└─────────┘    └────────────┘    └───────────┘    └──────────────┘    └────────┘
```

| Stage | Description |
|-------|-------------|
| **Draft** | Initial proposal. Author writes the GIP document and submits it for review. |
| **Discussion** | 14-day public discussion period. Community reviews, suggests changes, and debates the proposal. |
| **Accepted** | The proposal is approved and scheduled for implementation. |
| **Implemented** | The proposal has been implemented in the reference implementation. |
| **Final** | The proposal is stable, implemented, and part of the protocol specification. |

A GIP may also be **Rejected** or **Withdrawn** at any stage.

---

## GIP Template

Create a new GIP as a markdown file in `docs/gips/GIP-NNN-title.md`:

```markdown
# GIP-NNN: Title

| Field | Value |
|-------|-------|
| **GIP** | NNN |
| **Title** | Short descriptive title |
| **Author** | Name <email> |
| **Status** | Draft |
| **Created** | YYYY-MM-DD |
| **Discussion** | Link to GitHub Discussion / Issue |

## Abstract

One-paragraph summary of what this GIP proposes.

## Motivation

Why is this change needed? What problem does it solve?

## Specification

Detailed technical specification. Include:
- Protocol changes (wire format, message types)
- New algorithms or cryptographic constructions
- API changes
- Backward compatibility considerations

## Reference Implementation

Link to or describe the implementation in the reference codebase.

## Security Considerations

What are the security implications? New attack surfaces? Mitigations?

## Backward Compatibility

How does this affect existing implementations? Migration path?

## Copyright

This GIP is licensed under CC-BY-4.0.
```

---

## Current GIPs

| GIP | Title | Status | Description |
|-----|-------|--------|-------------|
| **001** | Initial Specification | Final | Core protocol specification: identity, messaging, mesh networking, cryptography |
| **002** | ML-KEM-768 Post-Quantum Hybrid | Final | Adds ML-KEM-768 (Kyber-768) hybrid key exchange alongside X25519 for post-quantum security |
| **003** | Pluggable Transports | Final | Censorship-resistant transports: WebSocket Camouflage, Shadowsocks AEAD, TLS Camouflage, obfs4 |

---

## Versioning

GhostWire uses **semantic versioning** (semver): `MAJOR.MINOR.PATCH`

| Component | Bumped When |
|-----------|-------------|
| **MAJOR** | Breaking protocol changes (wire format changes, incompatible crypto changes) |
| **MINOR** | New features, new endpoints, backward-compatible protocol extensions |
| **PATCH** | Bug fixes, security patches, performance improvements |

### Wire Format Version

The GhostWire wire format includes a **version byte** in the protocol header:

```
[version: 1 byte][payload: variable][signature: 64 bytes]
```

Current wire format version: `0x02` (v0.2.0)

This allows nodes running different versions to detect incompatibility and gracefully degrade rather than silently corrupting data.

### Version Compatibility

| Your Version | Can Communicate With |
|-------------|---------------------|
| 0.2.x | 0.2.x (full), 0.1.x (limited -- missing v2 features) |
| 0.1.x | 0.1.x, 0.2.x (v1-compatible subset only) |

When a major version bump occurs (e.g., 0.x -> 1.0), nodes running the old version **cannot** communicate with nodes running the new version. A migration period is announced with a timeline.

---

## Decision Making

GhostWire follows a **benevolent dictator** model:

- **Benevolent Dictator:** Michael Irungu Muriithi ([@Phantomojo](https://github.com/Phantomojo))
- **Role:** Final authority on protocol design, GIP acceptance, and release decisions
- **Responsibilities:** Review GIPs, resolve technical disputes, set roadmap priorities, approve releases

### How Decisions Are Made

1. **Community Discussion:** Proposals are discussed openly on GitHub Discussions or Issues.
2. **Consensus Seeking:** The community works toward consensus on technical merits.
3. **Dictator Decision:** If consensus cannot be reached, the benevolent dictator makes the final call.
4. **Appeal:** Decisions can be appealed with new technical evidence or implementation results.

The benevolent dictator model is chosen because:
- Protocol design requires coherent, consistent decisions
- Fast decisions are needed for security patches
- The project lead has the deepest understanding of the system's architecture and threat model

This may evolve as the project matures and more core contributors emerge.

---

## Contributing

### Code Contributions

1. Fork the repository
2. Create a feature branch
3. Write tests for your changes
4. Submit a pull request
5. Address review feedback

### Documentation Contributions

Documentation improvements are always welcome. Follow the same process as code contributions.

### Security Contributions

See [Security Disclosure Process](#security-disclosure-process) below.

---

## Security Disclosure Process

GhostWire takes security seriously. If you discover a vulnerability:

### Responsible Disclosure

1. **DO NOT** open a public issue or discussion
2. **Email** the security report to the project maintainer
3. **Include** in your report:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if you have one)

### Response Timeline

| Stage | Timeline |
|-------|----------|
| **Acknowledgment** | Within 48 hours |
| **Initial Assessment** | Within 7 days |
| **Fix Development** | Within 30 days (critical: within 7 days) |
| **Public Disclosure** | After fix is released, or 90 days after initial report (whichever comes first) |

### Severity Classification

| Severity | Response Time | Examples |
|----------|--------------|----------|
| **Critical** | 7 days | Remote code execution, key extraction, authentication bypass |
| **High** | 14 days | Message decryption, identity forgery, DoS |
| **Medium** | 30 days | Information disclosure, protocol downgrade |
| **Low** | 90 days | Minor information leak, UI spoofing |

### Security Advisories

After a fix is released, a security advisory is published with:
- CVE identifier (if applicable)
- Description of the vulnerability
- Affected versions
- Fixed version
- Mitigation for users who cannot upgrade immediately

---

## Release Process

1. **Feature Freeze:** No new features accepted to the release branch
2. **Code Freeze:** Only bug fixes and security patches
3. **Testing:** Full test suite run, manual testing of critical paths
4. **Release Candidate:** Published for community testing
5. **Final Release:** Tagged and published to crates.io
6. **Announcement:** Release notes published on GitHub

### Release Checklist

- [ ] All tests pass
- [ ] Security audit completed (or no new findings since last audit)
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Version bumped in Cargo.toml
- [ ] Git tag created
- [ ] crates.io published (for library crates)
- [ ] Release notes published

---

## Code of Conduct

All participants in the GhostWire project are expected to follow the [Code of Conduct](../CODE_OF_CONDUCT.md). Respectful, constructive communication is required in all discussions.

---

## License

- **Application (ghostwire binary):** AGPL-3.0-or-later
- **Library crates (sphinx-rs, ghostwire-dtn, etc.):** MIT OR Apache-2.0
- **Protocol specifications and GIPs:** CC-BY-4.0
- **Documentation:** CC-BY-4.0

---

*This governance document is itself subject to change via the GIP process. To propose changes to governance, submit a GIP.*
