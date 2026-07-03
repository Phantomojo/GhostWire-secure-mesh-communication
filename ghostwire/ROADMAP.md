# GhostWire Development Roadmap

> **Last Updated**: 2026-07-03 | **Current Version**: 0.2.0 | **Target**: v1.0 Stable

---

## 🎯 Vision

GhostWire is a decentralized P2P encrypted mesh communication system built in Rust. The goal is v1.0 stable: a reliable, secure, production-ready messaging platform.

**Current Score**: 7/10 (B-) — 4 critical gaps identified.

---

## 📊 Current State (2026-07-03)

### What Works ✅

| Layer | Status | Score |
|-------|--------|-------|
| Transport (libp2p, Iroh, BT, Obfs4) | Working | 8/10 |
| Encryption (AES, ChaCha, X25519, ML-KEM) | Working | 8/10 |
| Security (SecurityManager, Audit, RateLimit, CVE patches) | Working | 8/10 |
| Storage (MessageStore, Mailbox, Block) | Working | 7/10 |
| API (Axum, WebSocket, JWT) | Working | 7/10 |
| Application (TUI, CLI, Web UI) | Working | 7/10 |
| Health Monitoring (Ping/Pong, RTT, Liveness) | Working | 7/10 |

### What's In Progress 🔄

| Layer | Status | Score |
|-------|--------|-------|
| Identity (key revocation, rotation) | Partial | 6/10 |
| Privacy (adaptive cover traffic) | Static | 6/10 |
| Network (DHT, peer scoring) | Stub | 6/10 |
| Social (groups, reputation) | Basic | 5/10 |
| AI/ML (routing accuracy) | Broken | 5/10 |

### What's Done ✅

| Layer | Status | Score |
|-------|--------|-------|
| Message (ACK, ordering, retry) | Complete | 7/10 |
| Forward Secrecy (symmetric ratchet) | Complete | 8/10 |
| Health Monitoring (ping/pong) | Complete | 7/10 |
| Security Hardening (CVE patches) | Complete | 8/10 |

---

## 🚀 v1.0 Stable Plan (remaining)

### Phase 1: Message Reliability ✅ (2026-06-24)

**Goal**: Messages are never lost, duplicated, or reordered.

- [x] **ACK/Retry** — ACK/NACK frames implemented
- [x] **Message Ordering** — HLC timestamps for causal ordering
- [x] **Deduplication** — Bloom filter for duplicate detection
- [x] **Message TTL** — Expiration for old messages
- [ ] **Message Chunking** — Large message support (still needed)

### Phase 2: Key Security ✅ (2026-07-03)

**Goal**: Compromised keys can be revoked. Forward secrecy works.

- [x] **Forward Secrecy** — Symmetric ratchet with chain key aliasing
- [x] **Secure Key Storage** — Encrypted key store
- [ ] **Key Revocation** — Gossip-based CRL propagation (still needed)
- [ ] **Key Rotation** — Automatic rotation on schedule (still needed)

- [ ] **Key Revocation** — Gossip-based CRL propagation
- [ ] **Forward Secrecy** — Per-session key rotation
- [ ] **Secure Key Storage** — Encrypted key store
- [ ] **Key Rotation** — Automatic rotation on schedule

### Phase 3: Network Hardening (2-3 weeks)

**Goal**: Network is resilient to Sybil attacks and congestion.

- [ ] **Peer Scoring** — Reputation-based peer ranking
- [ ] **Bandwidth Estimation** — Congestion awareness
- [ ] **PrivacyParams** — Device-aware privacy tiers
- [ ] **DHT Integration** — Wire Kademlia for peer discovery

### Phase 4: Testing & Documentation (2-3 weeks)

**Goal**: Unknown vulnerabilities are found and documented.

- [ ] **Fuzzing Setup** — cargo-fuzz for critical paths
- [ ] **API Documentation** — OpenAPI 3.1 spec
- [ ] **SECURITY.md Update** — Threat model
- [ ] **Property-based Tests** — proptest for invariants
- [ ] **Integration Tests** — API end-to-end tests

### Phase 5: Polish (1-2 weeks)

**Goal**: Production-ready release.

- [ ] **Contact Verification** — Trust establishment
- [ ] **Message Compression** — Bandwidth savings
- [ ] **Storage Backup/Restore** — Data recovery
- [ ] **Performance Benchmarks** — Latency, throughput

---

## 📅 Release Strategy

### Versioning (Semantic)

```
v0.2.0  →  v1.0.0-alpha  →  v1.0.0-beta  →  v1.0.0
(current)   (Phase 1-2)     (Phase 3-4)     (Phase 5)
```

### Release Schedule

| Version | Target | Content |
|---------|--------|---------|
| v0.3.0 | Week 4 | Message reliability (Phase 1) |
| v0.4.0 | Week 8 | Key security (Phase 2) |
| v0.5.0 | Week 11 | Network hardening (Phase 3) |
| v1.0.0-alpha | Week 13 | Testing complete |
| v1.0.0-beta | Week 15 | Documentation complete |
| v1.0.0 | Week 16 | Stable release |

---

## 🔮 Post v1.0 (v1.1+)

### v1.1 — Advanced Privacy (Q4 2026)

- [ ] Poisson timing for cover traffic
- [ ] LARMix τ parameter
- [ ] PARSAN-Mix α parameter
- [ ] Bandwidth-adaptive mixing

### v1.2 — Multi-device (Q1 2027)

- [ ] KERI integration
- [ ] Multi-device support
- [ ] Identity migration
- [ ] Social recovery

### v2.0 — Ecosystem (Q2 2027)

- [ ] Mobile apps (Android/iOS)
- [ ] Desktop app (Tauri)
- [ ] Plugin system
- [ ] WebRTC transport
- [ ] Group messaging
- [ ] File sharing

---

## 🎯 Success Metrics

### v1.0 Requirements

| Metric | Target | Current |
|--------|--------|---------|
| Message delivery | 99.9% | Unknown |
| Key revocation | Working | ❌ |
| Forward secrecy | Per-session | ❌ |
| Peer scoring | Working | ❌ |
| Fuzzing coverage | Critical paths | ❌ |
| Test coverage | >80% | ~60% |
| Documentation | Complete | Partial |

### Performance Targets

| Metric | Target |
|--------|--------|
| Message latency | <100ms |
| Cover traffic overhead | <20% |
| Battery impact (mobile) | <5%/hr |
| Memory usage | <512MB |
| Binary size | <50MB |

---

## 📋 Technical Priorities

### Critical (v1.0 Blockers)

1. **Message ACK/Retry** — Messages are silently lost
2. **Key Revocation** — Compromised keys can't be revoked
3. **Forward Secrecy** — Session compromise exposes all past
4. **Peer Scoring** — Sybil attacks trivial
5. **Fuzzing** — Unknown vulnerabilities

### High (v1.0 Important)

1. **PrivacyParams** — Mobile battery waste
2. **Bandwidth Estimation** — No congestion awareness
3. **API Documentation** — Third-party integration
4. **DHT Integration** — Peer discovery incomplete
5. **Message Chunking** — Large messages fail

### Medium (Post v1.0)

1. **Poisson Timing** — Privacy improvement
2. **KERI** — Advanced key lifecycle
3. **Multi-device** — User convenience
4. **Mobile Apps** — Platform reach
5. **Plugin System** — Extensibility

---

## 📞 Community

### How to Contribute

- **Code**: Submit PRs for v1.0 items
- **Testing**: Test releases, report bugs
- **Documentation**: Improve guides
- **Security**: Report vulnerabilities (security@ghostwire.dev)

### Resources

- **GitHub**: https://github.com/Phantomojo/GhostWire
- **Docs**: `ghostwire/docs/`
- **Security**: `ghostwire/SECURITY.md`
- **Changelog**: `ghostwire/CHANGELOG.md`

---

**Note**: This roadmap is updated weekly. Last update: 2026-07-01.