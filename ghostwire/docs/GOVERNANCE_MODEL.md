# GhostWire Governance Model

## Overview

GhostWire uses a two-tier governance model inspired by the user's insight:
"dictator for critical (genesis), community for day-to-day (code)."

## Tier 1: Genesis (Constitution)

**Authority**: Project owner (dictator)
**Scope**: Network identity, crypto floor, governance rules, successor list
**Change process**: Genesis ceremony (signed document, version increment)
**Frequency**: Rare (protocol-level changes only)

### What requires Genesis change
- Network name or discovery topic changes
- Minimum encryption requirements
- PQ hybrid mandate
- Governance model itself
- Successor authorization
- Privacy floor changes

### What does NOT require Genesis change
- Transport implementation details
- Privacy tier defaults (within the floor)
- Discovery intervals
- UI/UX changes
- Performance optimizations

## Tier 2: Code (Legislation)

**Authority**: Community (maintainers + contributors)
**Scope**: Everything not in Tier 1
**Change process**: Standard PR review + merge
**Frequency**: Regular (feature development)

### Maintainer Responsibilities
1. Review PRs for security/privacy/anonymity/anti-censorship impact
2. Ensure tests pass (unit + integration)
3. Verify no regression in existing features
4. Update documentation for significant changes
5. Tag releases with semantic versioning

### Contributor Guidelines
1. All changes must pass the 4-lens evaluation (security, privacy, anonymity, anti-censorship)
2. New features require node-to-node integration tests
3. No new MCP tools or dependencies without maintainer approval
4. Breaking changes require genesis version bump

## Decision Making

### For Critical Changes (Genesis)
```
Author proposes → Author signs → Community reviews → Author applies → Transition period → Old version deprecated
```

### For Day-to-Day Changes (Code)
```
Contributor proposes → Maintainer reviews → CI passes → Merge → Release
```

### Conflict Resolution
- If community disagrees with author on critical change: author has final say
- If author is unavailable: successor list determines authority
- If no successor available: community can fork (open source)

## Succession Planning

### Triggers for Succession
1. Author voluntarily transfers (preferred)
2. Author becomes unavailable (health, legal, etc.)
3. Author authorizes emergency successor

### Transfer Process
1. Author creates new genesis document with successor's pubkey
2. Author signs and distributes
3. Successor receives private key (out of band)
4. Successor creates next genesis version
5. Old author's key is retired (kept in history)

### Successor Selection Criteria
- Technical competence in Rust, crypto, networking
- Alignment with GhostWire's mission (privacy, resilience, anti-censorship)
- Trust established through prior contributions
- Willingness to maintain long-term

## Anti-Censorship Governance

Special consideration for anti-censorship features:

1. **No single point of failure**: Multiple maintainers in different jurisdictions
2. **No central infrastructure**: All bootstrap is peer-to-peer
3. **Plausible deniability**: Maintainers can claim they don't control the network
4. **Open source**: Anyone can audit, fork, or run their own instance

### Legal Risk Mitigation
- Framing: "privacy tool for journalists" not "anti-censorship tool to evade government"
- Zero-knowledge architecture: maintainers cannot read user messages
- Genesis ceremony: verifiable proof that no backdoor can be added
- No central servers: nothing to seize or subpoena

## Metrics for Governance Health

| Metric | Target | Measurement |
|--------|--------|-------------|
| PR review time | < 7 days | GitHub analytics |
| Test coverage | > 80% | cargo tarpaulin |
| Security audit frequency | Quarterly | Manual audit |
| Genesis version stability | < 1/year | Version history |
| Community contributions | > 50%/year | PR author analysis |
| Successor readiness | Always 1+ | Successor list |

## Emergency Procedures

### If Author Key is Compromised
1. Author revokes key (publishes revocation)
2. Successor creates new genesis with new author key
3. All nodes update to new genesis
4. Old genesis is blacklisted

### If Network is Under Attack
1. Disaster level escalates automatically
2. Transport switches to RNS/BLE/WiFi
3. Cover traffic increases
4. Store-and-forward activates
5. Community coordinates via out-of-band channels

### If Maintainer is Coerced
1. Maintainer can push a "duress" commit (pre-arranged signal)
2. Community detects and responds
3. Successor takes over if needed
4. Genesis ceremony provides plausible deniability
