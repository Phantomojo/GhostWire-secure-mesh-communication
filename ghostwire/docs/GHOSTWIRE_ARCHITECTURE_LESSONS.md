---
name: ghostwire-architecture-lessons
description: Use when working on GhostWire wireframes, integration tests, or routing decisions.
category: software-development
---

# GhostWire Architecture Lessons

## Standing Constraint (every change, every layer)

At every stage — design, research, wiring, testing — evaluate:
- **Security**: new attack surfaces, weakened guarantees, input validation, key handling
- **Privacy**: metadata leaks (sender, recipient, timestamp, size), peer relationship exposure, correlation
- **Anonymity**: DID-PeerId linking, network topology exposure, traffic analysis
- **Anti-censorship**: can this be blocked/filtered by an authority? Multiple paths, obfuscation, decentralization, store-and-forward

Not a phase. A lens. Every change, every layer, every time.

## Anti-Censorship as a Core Goal

GhostWire is anti-censorship. This means:
- Multiple transport paths (so if one is blocked, another works)
- Traffic obfuscation (so censors can't identify GhostWire traffic)
- Decentralized discovery (no central point to block)
- Store-and-forward (so messages survive intermittent connectivity)
- Cover traffic (to hide usage patterns)
- Stealth mode (to avoid detection)

Compensating controls for anti-censorship:
- mDNS discovery (weak: detectable) → covered by RNS encrypted announces
- RNS transport (weak: identifiable) → covered by cover traffic + obfuscation
- Store-and-forward (weak: intermediate nodes see metadata) → covered by sealed sender + padding
- Single transport path (weak: blockable) → covered by multi-transport fallback

## Key Architectural Insights

### 1. The algorithm IS the protocol
The routing decision (which transport, how many hops, when to store-and-forward) is the protocol specification. Transports are pipes. The adaptive selector (Thompson Sampling + hysteresis + disaster levels) IS GhostWire's intelligence. libp2p and RNS are carriers, not competitors.

### 2. RNS is primary, libp2p is bootstrap
RNS runs over radio, serial, TCP, UDP, BLE. It works when IP is down. libp2p assumes IP connectivity. For a mission of "communication regardless of the odds," RNS carries messages and libp2p handles discovery/bootstrap. The tradeoff is not "slow vs fast" — it's "slow vs silence."

### 3. Scaffolding is design, not placeholders
Every unwired module represents a completed design decision. The transport selector, disaster manager, sealed sender crypto, wire protocol, spec with conformance vectors — these are not stubs. They're implementations waiting for integration. The hard thinking is done. The wiring is mechanical.

### 4. Compensating controls, not perfection
No single feature is strong in all three dimensions (security, privacy, anonymity). The system becomes strong by combining features whose weaknesses are covered by other features' strengths:
- mDNS (weak anonymity) → feeds into RNS encrypted announces
- RNS (weak anonymity) → carries sealed sender envelopes
- Sealed sender (weak security) → hides sender in RNS envelopes
- Sphinx (weak anonymity at 2 hops) → adds multi-hop unlinkability
Each layer compensates for the layer below. Defense in depth.

### 5. The armory, not the weapon
Not everyone needs maximum privacy in every situation. Default to middle ground (L1: encrypted content, no sealed sender, libp2p primary). Escalate to L3 (sealed + cover + Sphinx + RNS) on demand. Degrade gracefully when resources are constrained. Never force maximum privacy (kills usability) or minimum (kills security). The privacy tier (L0-L3) and disaster level (Normal-Blackout) are orthogonal axes.

### 6. Platform independence via RNS interface trait
The RNS interface trait (process_incoming, process_outgoing, spawned) is platform-agnostic. Carrier logic (KISS/HDLC framing, MTU chunking) is shared Rust. Only the I/O shim is platform-specific:
- WiFi: TCP sockets (universal Rust, no platform code)
- Serial/Radio: std::io (universal)
- BLE: ~90% universal carrier + ~10% platform shim (BlueZ on Linux, JNI/Tauri on Android/iOS)
Write the carrier once. Write a small shim per platform.

### 7. Dependency chain (not strictly sequential)
Transport → Routing → Privacy → UX → Quality. But wiring can happen in parallel — you can wire Layer 2's existing code while Layer 1 is still in progress. The dependency is on *testing*, not *coding*. You can't test Layer 2 without Layer 1 working, but you can wire Layer 2 while Layer 1 is in progress.

### 8. Truth ledger as honesty mechanism
The truth ledger honestly tracks what's real vs claimed. It downgrades marketing language. It lists release blockers. This is GhostWire's competitive advantage — not the code, not the crypto, the honesty. GhostWire doesn't lie to itself.

### 9. Security/privacy/anonymity per feature
Every feature has a strength and a weakness. Document both. Don't claim a feature is strong in all dimensions. Build compensating controls for the weaknesses. The system becomes strong by combining features whose weaknesses are covered.

### 10. Resilience vs speed is dynamic, not absolute
When IPs are down, "slow RNS" vs "no connection" is not a real choice. The tradeoff isn't "slow vs fast" — it's "slow vs silence." But when internet is up and no censorship, speed matters too. The balance depends on environmental state (internet available, latency, censorship detected) AND user preference. The algorithm weighs both. RNS over radio works when everything else fails. That's the mission.

## Specific Concerns Per Layer

### Transport
- BLE/WiFi discovery leaks proximity
- RNS announces leak network topology
- mDNS leaks presence on LAN
- Cross-transport discovery links RNS address to PeerId

### Routing
- Sphinx hop IDs are real PeerIds (privacy leak)
- Cross-transport discovery links RNS address to PeerId (correlation)
- DTN bundles expose destination hashes
- Store-and-forward leaks metadata to intermediate nodes

### Privacy
- Sealed sender unwired (no AAD, no replay, no expiry)
- Metadata (sender/recipient/timestamp/size) visible on shared DM topic
- DID-PeerId correlation is a known defect
- Recipient hint is BLAKE3(pubkey) — pseudonymous, not anonymous

### UX
- Message read receipts leak activity patterns
- Typing indicators leak presence
- Unread counts leak communication frequency
- mDNS broadcasts presence on LAN

### Quality
- Unwrap/panic can crash daemon and expose state
- Unbounded collections enable DoS
- Error messages leak internal details
- Transport switch logs leak network conditions

## Anti-patterns to avoid
- Creating modules without integration tests
- Starting new features before wiring the previous one
- Claiming features in README/truth ledger that aren't tested end-to-end
- Research subagent → design doc → module → move on (produces scaffolding, not software)
- Treating security/privacy as a phase instead of a lens
- Forcing maximum privacy (kills usability) or minimum (kills security)

## The Rule
No feature is "done" without a node-to-node integration test that:
- Starts two daemon instances
- Performs the actual operation
- Verifies the result on the receiving node
- Fails if the feature is removed

No exceptions.