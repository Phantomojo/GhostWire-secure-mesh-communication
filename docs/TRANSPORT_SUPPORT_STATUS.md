# GhostWire Transport and Bridge Support Status

**Status:** Authoritative implementation-status record for the current branch.
**Last verified baseline:** `cargo test --lib` — **310 passed, 0 failed, 1 ignored**; `cargo check --all-targets` passes with two pre-existing unused-field warnings.
**Rule:** A feature flag, module, design document, or UI label is **not** a support claim. A path is supported only when its concrete adapter, security boundary, lifecycle semantics, and required tests are all present.

> **GhostWire is becoming an inter-transport security switchboard. It is not yet a universal, automatic transport converter.** A node may eventually relay a protected GhostWire envelope from one eligible carrier to another, but it must not silently downgrade privacy, reuse one identity across networks, or expose carrier-native metadata as application metadata.

## Support Levels

| Level | Meaning |
|---|---|
| **Supported foundation** | Code is used by the current runtime and passes its declared test boundary. It does not mean every product workflow is complete. |
| **Experimental bounded adapter** | A narrow real implementation has passed a stated contract, but production lifecycle, ingress, and/or cross-carrier behavior remains incomplete. Not a general availability claim. |
| **Policy/design foundation** | Deterministic logic and documentation exist, but concrete runtime adapters have not been wired through it. |
| **Planned / unavailable** | A feature flag, module, documentation concept, or future design exists, but GhostWire users must not rely on it for delivery. |

## Carrier and Bridge Matrix

| Carrier or bridge | Current level | What is actually verified | What is **not** claimed |
|---|---|---|---|
| **libp2p mesh** | Supported foundation | The Rust node compiles and its existing mesh/control-plane components are covered by the normal library and all-target build gates. | Anonymous/private direct delivery, stable live multi-node lifecycle, and automatic cross-carrier failover. |
| **Direct private delivery** | Contained, not supported | A direct send fails with `PRIVATE_DELIVERY_UNAVAILABLE` when no private Sphinx route exists; it no longer falls back to the legacy shared direct topic. | A complete private route, durable delivery receipt, or general direct-stream replacement. |
| **Sphinx** | Supported protocol component; limited GhostWire integration | `sphinx-rs` and bounded route handling are present. | A fully unified ingress path, complete SURB acknowledgement lifecycle, or proof that every GhostWire carrier inherits anonymity. |
| **RNS / Reticulum** | Experimental bounded adapter | The `reticulum` feature compiles. The policy-checked `Core::submit_rns_protected_packet()` accepts only an already protected opaque envelope, validates destination/policy/profile/383-byte packet bounds, and submits it to an external official RNS runtime. The black-box test exchanges opaque payloads both ways between two isolated real-RNS TCP configurations, with RNS receipts. | A native Rust RNS implementation, full RNS interface support, Link/Resource bulk transfer, shared ingress, automatic handoff, mobile radio support, remote application delivery, or production readiness. |
| **Wi-Fi Direct** | Policy/design foundation | Carrier semantics, interface eligibility, identity separation, handoff constraints, and operating-profile restrictions are modeled and tested. | A Wi-Fi Direct adapter, Android/iOS desktop support, automatic group-owner behavior, or live delivery. |
| **Bluetooth LE / GATT** | Policy/design foundation | Carrier semantics, relay opt-in, MTU-aware planning, device eligibility, and operating-profile restrictions are modeled and tested. | BLE discovery, pairing, advertisement, GATT transfer, background relay, or a privacy/anonymity claim. |
| **RNode / LoRa / KISS / Meshtastic radio paths** | Planned / unavailable | Low-rate carrier constraints, RNS interface facts, fragmentation limits, and airtime caution are documented. | A production radio bridge, generic erasure-coded delivery, automatic retries/flooding, or universal 242-byte packet behavior. |
| **WebRTC / stealth transports** | Existing modules require individual audit | Code/modules may exist in the repository. | That they are selected by the new policy coordinator, provide anonymity, or are production-grade default fallbacks. |
| **Store-and-forward / DTN** | Supported foundation with limits | Local encrypted message persistence and DTN primitives exist. | A privacy-preserving public mailbox/provider discovery system or end-to-end delivery guarantee. |
| **Private bulk transfer** | Policy/design foundation plus bounded local staging | Carrier-specific fragmentation planning, a safe handoff planner, and a bounded in-memory content-addressed block store are tested. | BitTorrent/IPFS/Bitswap compatibility, public swarming, public provider announcements, exposed wantlists, durable encrypted file storage, or a network transfer API. |
| **Matrix, Briar, Meshtastic bridges** | Planned / individually unaudited | Feature/module placeholders and historical documentation may exist. | Interoperable, privacy-reviewed, or release-supported bridge behavior. |

## What “Inter-Transport” Means

A correct cross-carrier relay has this shape:

```text
protected GhostWire envelope
    ↓ carrier A
GhostWire node validates CRL/trust/auth/replay/causal/expiry policy
    ↓ policy-approved carrier B
same protected GhostWire envelope
```

The carrier changes its own framing, route/session behavior, MTU handling, and local receipt semantics. **GhostWire does not merge a libp2p peer ID, RNS destination, BLE address, Wi-Fi Direct group identity, radio identity, or root GhostWire identity.** The only valid shared item is the authenticated GhostWire envelope and its end-to-end operation identifier.

The deterministic planner currently rejects the following rather than pretending delivery succeeded:

| Rejected condition | Correct outcome |
|---|---|
| No carrier meets the required privacy class | Explicit typed route/policy failure; no downgrade. |
| Carrier lacks owner enablement, device support, OS permission, or background eligibility | Not selectable. |
| A relay-capable carrier lacks explicit relay opt-in | Not selectable for cross-carrier forwarding. |
| A carrier’s declared native identity does not match its family | Not selectable. |
| RNS single-packet payload exceeds 383 bytes | Reject; do not generic-fragment. |
| A stream object exceeds its direct logical frame limit | Plan a separately authorized private block exchange; do not invent a generic chunk protocol. |
| Source and destination are the same carrier | Not a handoff; preserve normal per-carrier lifecycle semantics. |

## Device Profiles

The deterministic [Operating Profiles](OPERATING_PROFILES.md) policy foundation is implemented and unit-tested. It defaults mobile nodes to endpoints, requires explicit relay role plus platform background eligibility, and can only reduce work under battery/thermal/metered constraints. Runtime configuration, telemetry, mesh/queue/adapter enforcement, and a mobile client are still pending.

Every GhostWire installation is a node. The node’s **capability profile** determines whether it is only an endpoint or may relay over an individual interface.

| Profile | Intended posture |
|---|---|
| Desktop/server, for example **Omen** or `cerberus-command` | May run multiple supervised carriers and explicitly opt in to relay duties, subject to bounded queue, connection, and privacy policy. |
| Mobile/constrained, for example a future Android/iOS GhostWire node | Endpoint by default; individual carrier relay requires explicit opt-in plus OS permission, background allowance, energy/radio budget, and carrier-specific support. |
| Low-rate radio gateway | Supervised, carrier-specific relay role only; no automatic generic retry, flooding, or bulk transfer. |

The formal adaptive profile implementation is still in progress. The contract exists so a future mobile app cannot accidentally behave like an unrestricted desktop relay.

## Required Reading for Contributors

| Document | Scope |
|---|---|
| [Carrier contract](CARRIER_CONTRACT.md) | Native carrier family, identity, discovery, fragmentation, and eligibility declarations. |
| [Inter-transport handoff](INTER_TRANSPORT_HANDOFF.md) | Protected-envelope handoff, per-leg receipts, and private bulk rules. |
| [Transport identity derivation](TRANSPORT_IDENTITY_DERIVATION.md) | HKDF domain separation for GhostWire-side carrier bindings. |
| [RNS/multi-carrier research evidence](RESEARCH_EVIDENCE_RETICULUM_MULTICARRIER.md) | Source-backed Reticulum, libp2p, BLE, Wi-Fi Direct, and radio design evidence. |
| [Architecture limits register](ARCHITECTURE_LIMITS_REGISTER.md) | Open acceptance conditions and explicit non-claims. |
| [Test matrix](TEST_MATRIX.md) | Required verification gates and supported-profile definitions. |

## Non-Negotiable Product Language

Public documentation, UI text, API responses, and release notes must use the following language until the relevant acceptance criteria pass.

| Do not say | Say instead |
|---|---|
| “GhostWire automatically switches between BLE, Wi-Fi, LoRa, and RNS.” | “GhostWire has a tested multi-carrier policy foundation; concrete automatic switching is still being wired and tested.” |
| “Reticulum is supported.” | “GhostWire has an experimental external RNS bridge with verified bounded packet interoperability.” |
| “All nodes relay automatically.” | “Every node may become a relay only when its device profile and individual carrier policy explicitly permit it.” |
| “Secure file sharing via Bitswap.” | “GhostWire has bounded local private-bulk staging and a private-transfer design; no public swarm/file-transfer transport is supported.” |
| “BLE/Wi-Fi/LoRa works offline.” | “BLE, Wi-Fi Direct, and radio carrier contracts are designed but concrete GhostWire adapters are not yet supported.” |
