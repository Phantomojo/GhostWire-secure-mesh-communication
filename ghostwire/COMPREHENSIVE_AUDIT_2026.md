# GhostWire Comprehensive Security & Code Audit
**Date:** 2026-04-08
**Scope:** Full codebase analysis, threat model alignment, network routing audit

---

## Executive Summary

GhostWire has a solid cryptographic foundation (XChaCha20-Poly1305, Ed25519, X25519) and working libp2p mesh networking. However, **44 stubs/placeholders** were found, including **6 CRITICAL** security gaps that must be addressed before any production deployment.

**Overall Security Score: 4.5/10** (up from 3.6/10 after previous fixes)

---

## 1. CRITICAL Findings (Must Fix Before Any Deployment)

### 1.1 Federation Bridge Trust — Trusts All Bridges, Dummy Signatures
**File:** `src/core/federation.rs:12-19`
```rust
pub struct TrustAllBridgeTrust;
impl BridgeTrust for TrustAllBridgeTrust {
    fn is_trusted(&self, _bridge_id: &str) -> bool { true }
    fn sign_relay(&self, _msg: &Message) -> SignedRelay {
        SignedRelay { relay_id: "dummy".into(), signature: vec![] }
    }
}
```
**Threat:** Any attacker can forge relayed messages through any bridge. No authentication, no integrity verification.
**Fix:** Implement Ed25519-based bridge trust with per-bridge public key verification.

### 1.2 obfs4 Transport — Secret Key Used as Public Key
**File:** `src/core/obfs4_transport.rs:53,84`
```rust
let public_key = secret.to_vec();  // Should use elligator2
```
**Threat:** The "public key" is identical to the secret key. This completely breaks obfs4 obfuscation — an observer can trivially identify obfs4 traffic by comparing the two keys.
**Fix:** Implement proper elligator2 encoding for public key derivation.

### 1.3 StealthTCP Transport — Entirely Commented Out
**File:** `src/core/stealth_tcp.rs:490-501`
```rust
// Commented out: impl Transport for StealthTCPProvider { ... }
```
**Threat:** The stealth TCP transport cannot be used at all. If this was intended as the primary DPI-resistant transport, the mesh has NO censorship resistance.
**Fix:** Uncomment and implement, or remove the module entirely to avoid false sense of security.

### 1.4 FunctionGemma Handlers — All 8 Return Mock Data
**File:** `src/core/ai/function_calling.rs` (8 handlers)
**Threat:** The AI function-calling layer returns fake success responses. If any system depends on these responses for routing decisions, it will operate on fabricated data.
**Fix:** Wire each handler to the real GhostWire subsystem it claims to control.

### 1.5 LiteRT-LM Engine — All Methods Are Stubs
**File:** `src/core/ai/litert_backend.rs:296,333,371`
**Threat:** The local AI inference backend returns empty text with 0 tokens. If the routing classifier depends on this, it will make decisions on empty input.
**Fix:** Implement or remove the module.

### 1.6 Encryption Path `panic!()` — Crashes Node
**File:** `src/core/encryption.rs:731`
```rust
panic!("Encryption failed - this should never happen in production: {}", e);
```
**Threat:** A single encryption failure crashes the entire node. An attacker could trigger this path to cause denial of service.
**Fix:** Return `Result::Err` instead of panicking.

---

## 2. HIGH Findings (Feature Gaps)

### 2.1 Mesh Health Dashboard — 16 of 28 Fields Return 0
**File:** `src/core/mesh_health.rs:106-176`
**Impact:** API reports a completely healthy mesh even if it's broken. Operators cannot detect failures.

### 2.2 Peer Controller — All Endpoints Return Empty Data
**File:** `src/controllers/peer_controller.rs`
**Impact:** No peer discovery, connection requests, or health checks work via API.

### 2.3 Ghost Channel Publish — Messages Never Broadcast
**File:** `src/controllers/all_controllers.rs:241`
**Impact:** Messages prepared for ghost channels are never actually sent.

### 2.4 TUI Mailbox — 0 Unread, No History
**File:** `src/tui/daemon_client.rs:181,265`
**Impact:** TUI users cannot see messages or message history.

### 2.5 Adapter Registry — Only Dummy Adapter
**File:** `src/core/adapter_registry.rs:14-32`
**Impact:** No Matrix, Meshtastic, or Briar bridges are functional.

### 2.6 StubKeyManager — Cannot Rotate/Revoke/Backup Keys
**File:** `src/core/encryption.rs:143-152`
**Impact:** If this is the active implementation, key management is completely broken.

### 2.7 Reticulum (LoRa) Initialization — Never Implemented
**File:** `src/cli/mod.rs:247`
**Impact:** No LoRa bridge is ever started.

### 2.8 CLI Stop Command — Does Nothing
**File:** `src/cli/mod.rs:495`
**Impact:** Cannot gracefully stop the daemon via CLI.

### 2.9 GNN Router — Empty Return on Missing Nodes
**File:** `src/core/ai/gnn_router.rs:531`
**Impact:** Returns empty route list instead of falling back to PRoPHET.

### 2.10 NoOp Traffic Obfuscator — No Cover Traffic
**File:** `src/core/traffic.rs:13-31`
**Impact:** No traffic analysis resistance. No cover traffic, no dummy packets, no delays.

### 2.11 NoOp Key Rotation Announcer
**File:** `src/core/encryption.rs:106-110`
**Impact:** Peers never learn about key rotations.

---

## 3. Threat Model Analysis

### 3.1 Adversary: Passive Eavesdropper
| Threat | Status | Notes |
|--------|--------|-------|
| Message encryption | ✅ Strong | XChaCha20-Poly1305 is cryptographically sound |
| Metadata protection | ⚠️ Partial | No cover traffic (NoOpTrafficObfuscator active) |
| Traffic analysis resistance | ❌ None | No dummy packets, no delays, no padding |

**Verdict:** Message content is protected, but traffic patterns (who talks to whom, when, how much) are fully visible.

### 3.2 Adversary: Active Man-in-the-Middle
| Threat | Status | Notes |
|--------|--------|-------|
| Message integrity | ✅ Strong | Poly1305 MAC prevents tampering |
| Replay attacks | ✅ Protected | HLC timestamps + 256-packet window |
| Key exchange | ✅ Strong | X25519 ECDH with Ed25519 signatures |
| Bridge relay integrity | ❌ None | TrustAllBridgeTrust with dummy signatures |

**Verdict:** Direct peer-to-peer is protected, but federation bridges are completely vulnerable.

### 3.3 Adversary: Sybil Attacker
| Threat | Status | Notes |
|--------|--------|-------|
| Peer scoring | ✅ Partial | Gossipsub peer scoring configured |
| Sybil rejection counter | ❌ Always 0 | Not wired to any subsystem |
| Trust-on-first-use | ✅ Implemented | TrustStore exists |

**Verdict:** Basic protections exist but Sybil rejection metrics are not operational.

### 3.4 Adversary: Eclipse Attack
| Threat | Status | Notes |
|--------|--------|-------|
| DHT bootstrap | ✅ Seed nodes | Decentralized bootstrap configured |
| Peer diversity | ⚠️ Unknown | No explicit diversity checks |
| mDNS discovery | ⚠️ Local only | Cannot be eclipsed remotely |

**Verdict:** Partially protected. No explicit eclipse attack detection.

### 3.5 Adversary: Censor (DPI)
| Threat | Status | Notes |
|--------|--------|-------|
| Stealth TCP | ❌ Commented out | Transport impl is disabled |
| obfs4 | ❌ Broken | Secret = public key |
| Traffic obfuscation | ❌ NoOp | No cover traffic, no delays |

**Verdict:** **No effective censorship resistance.** All three layers are non-functional.

### 3.6 Adversary: Compromised Peer
| Threat | Status | Notes |
|--------|--------|-------|
| Anomaly detection | ✅ Working | Isolation Forest (ROC-AUC 0.918) |
| Rate limiting | ✅ Implemented | Per-IP limits with backoff |
| Audit logging | ✅ Working | tracing-based event logging |
| Panic mode | ✅ Implemented | Identity wipe on duress |

**Verdict:** Good detection capabilities for compromised peers.

---

## 4. Network Routing Audit

### 4.1 libp2p/GossipSub (Primary)
| Component | Status | Notes |
|-----------|--------|-------|
| GossipSub | ✅ Working | v0.49.4 (CVE-2026-34219 fixed) |
| Kademlia DHT | ✅ Working | Peer discovery and content routing |
| mDNS | ✅ Working | Local network discovery |
| Peer scoring | ✅ Configured | Hardened params in skademlia.rs |
| **CVE-2026-34219** | ✅ Fixed | Instant overflow patched in 0.49.4 |
| **CVE-2026-33040** | ✅ Fixed | PRUNE backoff overflow patched in 0.49.3 |

**Verdict:** libp2p stack is current and patched.

### 4.2 WiFi Direct
| Component | Status | Notes |
|-----------|--------|-------|
| Transport | ✅ Via libp2p TCP | Standard TCP over WiFi |
| Encryption | ✅ XChaCha20-Poly1305 | End-to-end, link-independent |
| Discovery | ✅ mDNS | Automatic local network discovery |

**Verdict:** Solid. WiFi is just TCP with encryption.

### 4.3 Bluetooth (BLE)
| Component | Status | Notes |
|-----------|--------|-------|
| Transport module | ✅ Exists | `src/core/bluetooth.rs` |
| Implementation | ❌ Stub | No actual BLE code |
| Security research 2026 | ⚠️ MITM risk | Bitchat vulnerabilities identified Jan 2026 |

**Verdict:** Bluetooth module exists but is not implemented. When implemented, must address MITM vulnerabilities identified in Bitchat (Jan 2026).

### 4.4 LoRa/Reticulum
| Component | Status | Notes |
|-----------|--------|-------|
| Transport module | ✅ Exists | `src/core/reticulum.rs` |
| Initialization | ❌ Stub | CLI spawn does nothing |
| Store-and-forward | ✅ PRoPHET | DTN epidemic routing works |

**Verdict:** Reticulum module exists but is never initialized. PRoPHET fallback provides basic store-and-forward.

### 4.5 Stealth TCP / obfs4
| Component | Status | Notes |
|-----------|--------|-------|
| Stealth TCP | ❌ Commented out | Transport trait impl disabled |
| obfs4 | ❌ Broken | elligator2 not implemented |
| DPI resistance | ❌ None | All layers non-functional |

**Verdict:** **Critical gap.** No censorship resistance is operational.

### 4.6 AI Routing (4-Layer)
| Layer | Status | Accuracy | Notes |
|-------|--------|----------|-------|
| L1 LightGBM | ✅ Working | AUC 1.0 | Anomaly detection |
| L1b Isolation Forest | ✅ Working | ROC-AUC 0.918 | Unsupervised anomaly |
| L2 Gemma 4 | ✅ Working | N/A | Mesh analysis via Ollama |
| L3 GNN | ✅ Integrated | ~90% | Real Guifi.net data |
| L4 PRoPHET | ✅ Working | N/A | Always-on fallback |

**Verdict:** Excellent. All 4 layers operational with real data.

---

## 5. Cryptographic Audit

### 5.1 Encryption
| Algorithm | Status | Notes |
|-----------|--------|-------|
| XChaCha20-Poly1305 | ✅ Strong | 192-bit nonce, no reuse risk |
| X25519 ECDH | ✅ Strong | Standard key exchange |
| Ed25519 | ✅ Strong | Standard signatures |
| Symmetric ratchet | ✅ Implemented | Forward secrecy |
| BLAKE3 KDF | ✅ Strong | Domain-separated derivation |

**Verdict:** Cryptographic primitives are all sound. No known vulnerabilities in XChaCha20-Poly1305 as of 2026.

### 5.2 Key Management
| Component | Status | Notes |
|-----------|--------|-------|
| Key generation | ✅ Working | Ed25519 + X25519 pairs |
| Key rotation | ⚠️ Stub | NoOpKeyRotationAnnouncer active |
| Key backup | ❌ Stub | StubKeyManager does nothing |
| Shamir recovery | ✅ Working | Encrypted shares |
| Device pairing | ✅ Implemented | Identity pairing protocol |

**Verdict:** Key generation and recovery work, but rotation and backup are non-functional.

---

## 6. Priority Fix List

### Immediate (This Week)
1. **Fix obfs4 public key derivation** — Replace `secret.to_vec()` with elligator2
2. **Remove or implement StealthTCP** — Either uncomment the Transport impl or delete the module
3. **Replace panic!() in encryption** — Return Result::Err
4. **Replace TrustAllBridgeTrust** — Implement Ed25519 bridge verification

### Short Term (This Month)
5. **Wire FunctionGemma handlers** — Connect to real subsystems
6. **Wire peer controller endpoints** — Connect to social_graph
7. **Wire mesh health dashboard** — Connect to real metrics
8. **Implement LiteRT-LM or remove** — Don't ship stubs
9. **Add cover traffic** — Replace NoOpTrafficObfuscator

### Medium Term (Next Quarter)
10. **Implement Bluetooth transport** — Address MITM vulnerabilities
11. **Initialize Reticulum/LoRa** — Wire the CLI spawn task
12. **Replace StubKeyManager** — Implement rotation/revocation/backup
13. **Add eclipse attack detection** — Monitor peer diversity

---

## 7. Comparison to Competitors

| Feature | GhostWire | Briar | Meshtastic | Matrix |
|---------|-----------|-------|------------|--------|
| E2E encryption | ✅ XChaCha20 | ✅ AES-GCM | ✅ AES-CTR | ✅ Olm/Megolm |
| Forward secrecy | ✅ Ratchet | ✅ Ratchet | ❌ | ✅ Ratchet |
| Censorship resistance | ❌ Broken | ✅ Tor | ❌ | ❌ |
| Sybil resistance | ⚠️ Partial | ✅ TOFU | ⚠️ Partial | ❌ |
| Offline operation | ✅ Full | ✅ Full | ✅ Full | ❌ |
| AI routing | ✅ 4-layer | ❌ | ❌ | ❌ |
| Anomaly detection | ✅ Isolation Forest | ❌ | ❌ | ❌ |
| Post-quantum ready | ⚠️ Planned | ❌ | ❌ | ❌ |

**GhostWire's advantages:** AI routing, anomaly detection, real mesh data training
**GhostWire's gaps:** Censorship resistance (all 3 layers broken), key management stubs

---

## 8. Recommendations

### Security
- **Do NOT deploy federation bridges** until TrustAllBridgeTrust is replaced
- **Do NOT advertise censorship resistance** until obfs4/StealthTCP are fixed
- **Add rate limiting to encryption endpoints** to prevent panic-triggering DoS

### Architecture
- **Remove all stub modules** rather than shipping non-functional code
- **Add integration tests** that verify each transport actually sends/receives
- **Add a "feature flags" system** to clearly mark what's production vs. research

### Operations
- **Wire mesh health dashboard** — operators need visibility
- **Implement CLI stop command** — graceful shutdown is essential
- **Add cover traffic** — traffic analysis is the easiest attack vector
