# GhostWire × Freenet Research Report

**Date**: July 28, 2026
**Scope**: Both Classic Freenet (Hyphanet) and New Freenet (Locutus)

---

## PART 1: THE TWO FREENETS — CRITICAL DISTINCTION

| | **Hyphanet** (hyphanet.org) | **New Freenet** (freenet.org) |
|---|---|---|
| **Name history** | Original Freenet (1999–2023), renamed Hyphanet June 2023 | Codename *Locutus* (2019–2023), rebranded to "Freenet" March 2023 |
| **Codebase** | Java, ~192K LOC, 0.7.5 build 1506 (Feb 2026) | Rust, ~45K+ LOC, freenet-core |
| **Primary goal** | Extreme anonymity, censorship-resistant file storage | Decentralized real-time app platform (no anonymity goal) |
| **Network type** | Friend-to-friend darknet + opennet hybrid | Small-world ring, structured DHT |
| **State model** | Immutable CHK blocks, versioned SSK/USK | WASM contracts with commutative merge (join-semilattice) |
| **Live size** | Unknown (legacy) | ~440 active peers (whitepaper measurement) |

---

## PART 2: HYPANET (CLASSIC FREENET) — ARCHITECTURE

### Darknet vs Opennet
- **Darknet**: Nodes connect ONLY to trusted friends. Social graph = network graph. Small-world properties at degree >= 20.
- **Opennet**: Random peer bootstrapping when darknet connections < 20.

### Key Types (Four-Part System)
- **CHK** (Content Hash Keys): SHA256(data) + AES_key. Content-addressed. Storer CANNOT decrypt.
- **SSK** (Signed Subspace Keys): Asymmetric keypair + symmetric encryption. Plausible deniability.
- **USK** (Updatable Subspace Keys): SSK wrapper with auto-incrementing version.
- **KSK** (Keyword Signed Keys): Human-readable, spammable.

### Data Store Architecture
- 50% cache (LRU) + 50% permanent store
- Encrypted atomic blocks — nodes never see plaintext
- Splitfiles for >32KB files with Vandermonde FEC
- "Cache as you go" — successful responses cached at every hop

### Routing Protocol (v0.7)
- Small-world ring model with location-based routing
- Greedy forwarding toward target location
- HTL counter bounds search depth

### Metadata Protection Strengths
- Requestor anonymity (multi-hop)
- Content deniability (encrypted storage)
- Insert anonymity
- Source/destination unlinkability

### Known Weaknesses
- Statistical disclosure attacks (intersection attacks)
- Timing correlation
- Content poisoning
- Passive logging

---

## PART 3: NEW FREENET (LOCUTUS) — ARCHITECTURE

### Core Primitives
- **Contracts** (WASM, runs on untrusted peers): State space + validity predicate + commutative merge
- **Delegates** (WASM, local only): Private key management, secrets
- **UI** (web, WebSocket): Standard web tech

### Consistency Model: Join-Semilattice
```
merge(a, b) = b ⊕ a           (commutative)
a ⊕ (b ⊕ c) = (a ⊕ b) ⊕ c   (associative)  
a ⊕ a = a                     (idempotent)
```

### Summary/Delta Sync
1. Peer A sends compact summary to Peer B
2. Peer B computes delta (what B has that A doesn't)
3. B sends delta; A applies merge
4. Convergent state with minimal transfer

### Small-World Ring Topology
- All peers/contracts on [0, 1) ring
- Peer location from IP hash, contract location from key hash
- 25-200 connections per peer
- Greedy forwarding with HTL cap of 10

### Adaptive Routing (Isotonic Regression)
- Per-peer response time model (monotonically increasing with distance)
- Failure probability model
- Transfer rate model
- Expected cost = (success_prob × time) + (failure_prob × penalty × time)

### FrTP Transport
- UDP-based with NAT hole punching (symmetric + double-NAT)
- X25519 + ChaCha20Poly1305 handshake, AES-128-GCM data
- Congestion control: FixedRate, LEDBAT++, BBRv3
- Lock-free fragment reassembly

---

## PART 4: GHOSTWIRE ACTIONABLE TAKEAWAYS

### Techniques GhostWire Could Adopt

| Technique | Source | Application | Priority |
|---|---|---|---|
| Summary/delta sync | New Freenet | DTN bundles: exchange compact summaries, only transfer deltas | **High** |
| Join-semilattice state model | New Freenet | CRDT-style data (reputation, routing, identity) | **High** |
| Routing key ≠ decryption key (CHK pattern) | Hyphanet | Content-addressed storage where storer can't decrypt | **High** |
| Adaptive peer scoring (isotonic regression) | New Freenet | Baseline fallback when ML models unavailable | Medium |
| LRU + permanent storage tiers | Hyphanet | DTN store-carry-forward with retention policy | Medium |
| Splitfile + FEC (Vandermonde) | Hyphanet | Large DTN bundles: split blocks, reassemble from N-of-M | Medium |
| Sneakernet bridging (Shoeshop) | Hyphanet | Physical transport for partitioned networks | Low |

### Techniques GhostWire May Be Reinventing

1. **Content addressing**: If message IDs are GUIDs not hashes, consider switching for auto-dedup + integrity
2. **Plausible deniability**: If nodes can read stored messages, missing a key defense
3. **Cache-along-path**: Successful routes should be cached for reuse
4. **Backtracking search**: DTN should backtrack and try other nodes on dead ends

### Freenet Weaknesses GhostWire Should Avoid

| Weakness | GhostWire Solution |
|---|---|
| No Sybil resistance | DHT identity + proof-of-work/stake |
| LRU eviction loses rare content | Explicit retention policy per message class |
| No incentive to contribute | Reputation scoring for relay participation |
| Timing analysis leaks metadata | Cover traffic + dummy messages for DTN |
| No search capability | DHT-based discovery (already has this) |
| Congestion control immature | Test under mesh conditions |
| No anonymity in New Freenet | Keep Sphinx + DTN metadata protection — DON'T retreat |

### Cross-Pollination Ideas

1. **Message as Contract**: DTN bundles as Freenet-style contracts with AI routing hints
2. **Sphinx over FrTP**: Onion routing on mesh-tuned transport
3. **DHT Identity from Hyphanet's ARK**: Updatable reachability info under stable key
4. **Subscription Tree for Gossipsub**: Structured alternative to flooding for high-volume channels

---

## TOP 3 FOR GHOSTWIRE

1. **Adopt summary/delta sync for DTN bundles** — cuts bandwidth on lossy mesh links
2. **Separate routing address from decryption key** (CHK pattern) — nodes route without reading
3. **Don't retreat on anonymity** — Sphinx + post-quantum + DTN metadata protection is a differentiator
