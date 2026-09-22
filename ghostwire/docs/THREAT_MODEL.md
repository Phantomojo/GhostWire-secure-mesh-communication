# GhostWire Threat Model

> **Version**: 1.2 | **Status**: Draft | **Last Updated**: 2026-09-22
> **Standard**: STRIDE + NIST SP 800-57 | **Review Cycle**: Quarterly

---

## 1. Security Goals

| Goal | Description |
|------|-------------|
| **Confidentiality** | Message content is only readable by intended recipients |
| **Integrity** | Messages cannot be modified in transit without detection |
| **Authenticity** | Peers can verify each other's identity |
| **Availability** | Communication works despite network disruptions |
| **Anonymity** | Observers cannot determine who is communicating with whom |
| **Censorship Resistance** | No single point of failure can block communication |
| **Forward Secrecy** | Compromise of long-term keys does not expose past messages |

---

## 2. System Model

### 2.1 Architecture Layers

```
┌─────────────────────────────────────────────┐
│  Application Layer (TUI, CLI, Web UI)       │
├─────────────────────────────────────────────┤
│  API Layer (Axum 0.8 + Tower)               │
├─────────────────────────────────────────────┤
│  Security Layer (SecurityManager, Audit)    │
├─────────────────────────────────────────────┤
│  Encryption Layer (AES-256-GCM, X25519)     │
├─────────────────────────────────────────────┤
│  Identity Layer (Ed25519, Biscuit tokens)    │
├─────────────────────────────────────────────┤
│  Privacy Layer (Sphinx, cover traffic)       │
├─────────────────────────────────────────────┤
│  Transport Layer (libp2p, Iroh, BT, Obfs4)  │
├─────────────────────────────────────────────┤
│  Network Layer (GossipSub, Kademlia DHT)     │
├─────────────────────────────────────────────┤
│  Storage Layer (MessageStore, BlockStore)    │
└─────────────────────────────────────────────┘
```

### 2.2 Trust Boundaries

| Boundary | Trusted Side | Untrusted Side | Enforcement |
|----------|-------------|----------------|-------------|
| **B1: Network** | Local node | P2P network | Transport encryption (Noise/Iroh) |
| **B2: API** | Authenticated client | HTTP requests | Biscuit token + rate limiting |
| **B3: Storage** | Encrypted store | Disk/filesystem | AES-256-GCM at rest |
| **B4: Identity** | Private keys | Public keys | Ed25519 signatures |
| **B5: Peer** | Verified peers | Unknown peers | Trust scoring + reputation |

---

## 3. Trust Assumptions

| Assumption | Risk if Violated | Mitigation |
|------------|-----------------|------------|
| Local OS is trusted | Key extraction, session hijacking | Encrypted key storage, secure wipe |
| Cryptographic primitives are secure | Complete system compromise | NIST-approved algorithms, regular review |
| Random number generator is secure | Key prediction, nonce reuse | `rand` crate with OS entropy |
| At least one honest relay exists | Message delivery failure | Multiple transport fallbacks |
| Majority of DHT nodes are honest | Eclipse attack, routing manipulation | Peer scoring, diverse discovery |
| Honest outbound diversity — victim's outbound set is not fully attacker-controlled, including NAT'd (unreachable) peers | NAT-path eclipse: peerlist poisoning of reachable relays contaminates a NAT'd node's *only* connection surface; attacker needs public-IP Sybil nodes, not inbound to victim ([Shi et al., CCS 2026](https://arxiv.org/abs/2609.10260)) | Disjoint Kademlia lookups, per-/24 bucket caps, anchor connections to known-honest peers, CIDR-concentration self-check |

---

## 4. Threat Categories (STRIDE)

### 4.1 Spoofing

| ID | Threat | Likelihood | Impact | Status | Mitigation |
|----|--------|-----------|--------|--------|------------|
| S1 | Peer impersonation | Medium | Critical | ✅ Defended | Ed25519 signatures, Biscuit tokens |
| S2 | Man-in-the-middle | Medium | Critical | ✅ Defended | Noise/Iroh transport encryption |
| S3 | DNS/DHT poisoning | Low | High | ⚠️ Partial | Kademlia DHT with peer scoring |
| S4 | Replay attacks | Medium | High | ✅ Defended | Nonces + message ordering |

### 4.2 Tampering

| ID | Threat | Likelihood | Impact | Status | Mitigation |
|----|--------|-----------|--------|--------|------------|
| T1 | Message modification | Medium | Critical | ✅ Defended | AES-256-GCM authenticated encryption |
| T2 | Key tampering | Low | Critical | ✅ Defended | Ed25519 signatures on key material |
| T3 | Config tampering | Low | High | ✅ Defended | Signed configuration, integrity checks |
| T4 | Binary tampering | Low | Critical | ⚠️ Partial | Reproducible builds (planned) |

### 4.3 Repudiation

| ID | Threat | Likelihood | Impact | Status | Mitigation |
|----|--------|-----------|--------|--------|------------|
| R1 | Message denial | Medium | Medium | ✅ Defended | Signed messages, audit log |
| R2 | Action denial | Low | Medium | ✅ Defended | Audit trail with timestamps |

### 4.4 Information Disclosure

| ID | Threat | Likelihood | Impact | Status | Mitigation |
|----|--------|-----------|--------|--------|------------|
| I1 | Metadata leakage | High | High | ⚠️ Partial | Sphinx packets, cover traffic |
| I2 | Traffic analysis | High | Medium | ⚠️ Partial | Poisson timing (planned) |
| I3 | Key exposure | Low | Critical | ✅ Defended | Encrypted storage, secure wipe |
| I4 | Message content exposure | Low | Critical | ✅ Defended | E2E encryption |
| I5 | Peer list exposure | Medium | Medium | ⚠️ Partial | Encrypted peer exchange |

### 4.5 Denial of Service

| ID | Threat | Likelihood | Impact | Status | Mitigation |
|----|--------|-----------|--------|--------|------------|
| D1 | Rate limiting bypass | Medium | Medium | ✅ Defended | Governor rate limiter |
| D2 | Resource exhaustion | Medium | High | ⚠️ Partial | Memory limits, message TTL |
| D3 | Network flooding | High | High | ⚠️ Partial | GossipSub peer scoring |
| D4 | Storage filling | Low | Medium | ⚠️ Partial | Message TTL, storage limits |

### 4.6 Elevation of Privilege

| ID | Threat | Likelihood | Impact | Status | Mitigation |
|----|--------|-----------|--------|--------|------------|
| E1 | Unauthorized API access | Low | Critical | ✅ Defended | Biscuit token authentication |
| E2 | Privilege escalation | Low | Critical | ✅ Defended | Minimal permissions, sandboxing |
| E3 | Code injection | Low | Critical | ✅ Defended | Rust memory safety, no `unsafe` |

---

## 5. Detailed Threat Analysis

### 5.1 Key Compromise (Current)

**Threat**: Attacker obtains long-term signing key.

**Impact**: Can impersonate node, decrypt future messages.

**Mitigation**:
- Forward secrecy via per-session key derivation
- Key revocation via gossip-based CRL — **shipped** (`src/core/crl.rs`, CRL_GOSSIP in mesh; epoch fail-closed freshness in progress, see §5.7/P0-B)
- Secure key storage with encryption at rest

### 5.2 Key Compromise (Past Messages)

**Threat**: Attacker records encrypted traffic, then compromises key later.

**Impact**: Can decrypt all past messages.

**Mitigation**:
- Forward secrecy via X25519 ECDH
- Per-session key rotation (v1.0 Phase 2)
- No key reuse across sessions

### 5.3 Traffic Analysis

**Threat**: Observer correlates message timing/volume with peer activity.

**Impact**: Identifies communication patterns, active peers.

**Mitigation**:
- Sphinx fixed-size packets (2252 bytes)
- Cover traffic (512-byte blocks, 3-12s interval)
- Tiered privacy levels (L1/L2/L3)
- **Status**: Partially mitigated — Poisson timing planned

### 5.4 Sybil Attack

**Threat**: Attacker creates many fake identities to isolate target.

**Impact**: Eclipse attack, routing manipulation, deanonymization.

**Mitigation**:
- Peer scoring based on behavior
- Social graph trust propagation
- Proof-of-Work identity cost (planned)
- **Status**: Partially mitigated — scoring exists, needs hardening

Related: See §5.7 for the NAT-path adversary, a Sybil/eclipse class that specifically targets unreachable (NAT'd) peers — GhostWire's expected deployment shape in Nairobi.

### 5.7 NAT-Path Adversary / Eclipse via Peer-Discovery Poisoning

**Threat**: An adversary controls enough *reachable* (public-IP) mesh nodes to poison peerlists, then exploits the fact that NAT'd victims can only maintain outbound connections. Contaminated reachable relays feed the victim poisoned peer records during sync/handshake; the victim's entire outbound set becomes attacker-controlled. Because the victim cannot accept inbound connections, honest peers have no channel to heal the eclipse ([Shi et al., "Are Unreachable Nodes Truly Safe? Fully Eclipsing Monero's P2P Network!", ACM CCS 2026, arXiv:2609.10260](https://arxiv.org/abs/2609.10260) — attacks *Nyx* (slow network-wide poisoning) and *Moros* (fast bootstrap eclipse)).

**Why this matters for GhostWire**: Nairobi offline-first deployment assumes many nodes sit behind carrier NAT / CGNAT with no inbound reachability. That is exactly the preconditions class the CCS 2026 paper shows is *not* safer — it is often *more* exposed, since outbound-only peers must trust whoever answers their dials.

**Related results** (eclipse is a membership attack, not a forgery attack — signed records still verify when the attacker chooses *which* signed records you see):
- [Heilman et al., USENIX Security 2015](https://eprint.iacr.org/2015/263.pdf) — Bitcoin eclipse with 32×/24 or ~4600 bots (85%); deployed counters: deterministic random eviction, random table selection, test-before-evict (v0.17), feeler connections (v0.13), anchor connections (v0.21).
- [Marcus, Heilman, Goldberg, ePrint 2018](https://eprint.iacr.org/2018/236) — Ethereum Kademlia eclipse with **2 IPs**: XOR-distance predictability + free key generation; geth mitigated with max peers per /24.
- [Tran et al., USENIX Security 2021](https://www.usenix.org/system/files/sec21fall-tran.pdf) — Erebus/RAP: malicious-AS network-Sybil; routing-aware peering fails under <6% route-inference error (not realistic for us — no BGP visibility).
- [py-libp2p #1383 / #1399](https://github.com/libp2p/py-libp2p/issues/1383) — IP/subnet diversity in k-buckets (max 2 peers per /24 per bucket) + disjoint lookup paths (d≥2 independent lookups). Note: record signing authenticates content; eclipse attacks membership.

**Mitigation (planned / partial)**:
- Enable `disjoint_query_paths(true)` on Kademlia lookups (opt-in in rust-libp2p ≥0.20; multiplies attacker cost linearly) — **planned**
- Cap ~2 peers per /24 (IPv4) / /48 (IPv6) per k-bucket, exempting loopback/private/relay — **planned**
- Anchor connections to 2–3 known-honest peers (surviving sessions / operator-configured seeds) so a poisoned discovery path cannot fully replace the outbound set — **planned**
- Peer scoring based on behavior (existing) — partial
- Periodic eclipse self-check: alert if routing table shows high CIDR concentration vs. bootstrap diversity — **planned**
- **Status**: ⚠️ Partially mitigated — peer scoring exists; disjoint lookups, /24 caps, anchors, and CIDR self-check not yet implemented

**Non-goals** (rejected for GhostWire): proof-of-work peer IDs (libp2p interop break), centralized identity authority (contradicts permissionless mesh), full routing-aware peering / SABRE-style global relay fabric (needs BGP visibility or globally deployed infrastructure unavailable to an offline-first Nairobi mesh).

### 5.5 Censorship

**Threat**: Authority blocks IP/port ranges.

**Impact**: Nodes cannot communicate.

**Mitigation**:
- Multiple transport options (TCP, QUIC, BT, Obfs4)
- Stealth TCP mimics HTTPS traffic
- Circuit relay v2 for blocked nodes
- **Status**: Defended — multiple fallback paths

### 5.6 X25519 Small-Order Point Attack (CVE-2025-62514)

**Threat**: Attacker sends a crafted X25519 public key that produces a shared secret of all zeros, potentially bypassing key validation.

**Impact**: Could enable key recovery or session decryption if not detected.

**Mitigation**:
- All-zero shared secret check after `diffie_hellman()` in `encryption.rs:703`
- Rejects any DH output that is entirely zero bytes
- **Status**: ✅ Defended (fixed 2026-07-03)

---

## 6. What GhostWire Does NOT Defend Against

| Threat | Reason | Recommendation |
|--------|--------|----------------|
| **Endpoint compromise** | OS/hardware security problem | Use hardened OS, full-disk encryption |
| **Physical device seizure** | Beyond crypto scope | Dead man's switch, remote wipe |
| **Rubber-hose cryptanalysis** | Coercion is operational | Plausible deniability (planned) |
| **Supply chain attacks** | Requires reproducible builds | Reproducible builds (v1.1) |
| **Legal compulsion** | Operational/legal problem | Jurisdiction diversity |
| **Quantum computing** | Future threat | ML-KEM (Kyber) hybrid mode available |
| **Zero-day in dependencies** | Depends on upstream | `cargo audit`, minimal dependencies |
| **User operational security** | Human factor | Documentation, training |

---

## 7. Security Recommendations

### For Users

| Priority | Recommendation |
|----------|---------------|
| **Critical** | Keep software updated |
| **Critical** | Verify peer identities out-of-band |
| **High** | Use strong, unique passwords for encrypted storage |
| **High** | Enable maximum privacy tier for sensitive communication |
| **Medium** | Review connected peers periodically |
| **Medium** | Back up identity keys securely |

### For Operators

| Priority | Recommendation |
|----------|---------------|
| **Critical** | Run `cargo audit` before deployment |
| **Critical** | Monitor security audit logs |
| **High** | Implement key rotation schedule |
| **High** | Enable rate limiting and threat detection |
| **Medium** | Deploy behind firewall where possible |

---

## 8. Incident Response

### Severity Levels

| Level | Description | Response Time |
|-------|-------------|---------------|
| **P0 — Critical** | Remote code execution, key compromise | Immediate |
| **P1 — High** | Data exfiltration, authentication bypass | 24 hours |
| **P2 — Medium** | Information disclosure, DoS | 7 days |
| **P3 — Low** | Minor information leak, minor DoS | 30 days |

### Response Process

1. **Detection**: Security audit logs, user reports, automated monitoring
2. **Triage**: Assess severity using table above
3. **Containment**: Disable affected components, rotate compromised keys
4. **Eradication**: Patch vulnerability, deploy fix
5. **Recovery**: Restore from secure backups, verify integrity
6. **Post-mortem**: Document root cause, update threat model

### Contact

- **Security Email**: security@ghostwire.dev
- **Response SLA**: Initial response within 48 hours
- **Disclosure**: After fix is available, coordinated disclosure

---

## 9. Audit Status

| Audit | Date | Scope | Status |
|-------|------|-------|--------|
| Internal Review | 2026-06 | Full codebase | ✅ Complete |
| Dependency Audit | 2026-06 | `cargo audit` | ✅ Clean |
| External Audit | Planned | Crypto + transport | 🔜 Pending v1.0 |

---

## 10. References

- [NIST SP 800-57](https://csrc.nist.gov/publications/detail/sp/800-57-part-1/rev-5/final) — Key Management
- [OWASP Threat Modeling](https://owasp.org/www-community/Threat_Modeling)
- [STRIDE Model](https://learn.microsoft.com/en-us/azure/security/develop/stride-model)
- [Shi et al., CCS 2026 — Fully Eclipsing Monero via NAT](https://arxiv.org/abs/2609.10260) — NAT-path adversary (§5.7)
- [Heilman et al., USENIX 2015 — Bitcoin Eclipse](https://eprint.iacr.org/2015/263.pdf)
- [Marcus et al., ePrint 2018 — Ethereum Kademlia Eclipse](https://eprint.iacr.org/2018/236)
- [Tran et al., USENIX 2021 — Erebus / Routing-Aware Peering](https://www.usenix.org/system/files/sec21fall-tran.pdf)
- [py-libp2p #1383 — Kademlia eclipse hardening](https://github.com/libp2p/py-libp2p/issues/1383)
- [GhostWire SECURITY.md](../SECURITY.md)
- [GhostWire Layer Audit](~/Downloads/GhostWire_Layer_Audit_v1.md)
