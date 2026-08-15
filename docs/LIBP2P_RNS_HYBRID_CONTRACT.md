# libp2p–RNS Hybrid Transport Contract

**Status:** An **experimental bounded RNS packet leg** is implemented. It is not a general automatic hybrid mesh, private-direct delivery system, RNS Link/Resource transfer, or universal carrier bridge.

> GhostWire is the policy/security switchboard. libp2p and RNS remain independent native networks. GhostWire moves only a pre-authenticated, already encrypted **protected envelope** between approved legs; it does not merge peer IDs, RNS destinations, routing tables, or application plaintext.

## What Is Implemented

| Component | Current behavior | Explicit non-claim |
|---|---|---|
| libp2p runtime | Existing GhostWire mesh/control plane. Its direct peer relationship is not automatically anonymous. | That libp2p direct streams satisfy relationship-metadata protection. |
| External RNS runtime | `ReticulumManager` launches the official RNS Python runtime in a dedicated configuration/identity directory and validates the local RNS destination at startup. | A native Rust RNS implementation or a GhostWire-derived RNS identity. |
| RNS bridge process | Bounded JSON-lines control pipe; official RNS owns packet encryption, path discovery, routing, native interface configuration, and RNS receipts. | That GhostWire reimplements or predicts RNS routing/interface behavior. |
| Packet hybrid boundary | `rns_hybrid::plan_rns_packet_submission()` accepts only already protected opaque bytes, validates an explicit 32-hex RNS destination, applies carrier policy and operating profile, and allows only one packet of at most **383 bytes**. | Plaintext `Message` conversion, generic fragmentation, Link/Resource bulk transfer, or a privacy upgrade. |
| Bridge submission API | `Core::submit_rns_protected_packet()` exists only with the `reticulum` feature and routes through the manager’s policy-checked method. | Routing normal direct messages through RNS, a local bridge write as remote receipt, or recipient application delivery. |
| Real RNS verification | `scripts/test_reticulum_bridge.py` starts two isolated official-RNS TCP configurations and proves bounded opaque payload delivery plus RNS receipts in both directions. | Multi-interface hardware, public relay, mobile, or cross-machine test coverage. |

## Data-Plane Boundary

The only currently authorized RNS operation is a low-level pre-protected packet submission:

```text
caller has already authenticated/encrypted a GhostWire envelope
    ↓ immutable delivery/privacy requirement + opaque payload bytes
rns_hybrid planner
    ├─ validates RNS destination format
    ├─ restricts interface using device/role/runtime profile
    ├─ requires truthful Reticulum native-identity declaration
    ├─ rejects unmet privacy or relay requirements
    └─ requires a direct 1–383 byte packet plan
    ↓
ReticulumManager local JSON bridge write
    ↓
official RNS encrypted packet/path/receipt machinery
```

The policy boundary purposefully does not accept `Message`. A `Message` is plaintext in memory and its ordinary direct dispatch is still controlled by the existing Sphinx/private-route containment. This prevents an accidental “RNS fallback” from bypassing encrypted persistence, private-route failure behavior, or message-level protection.

## Carrier Semantics

| Property | libp2p leg | Experimental RNS packet leg | Switchboard rule |
|---|---|---|---|
| Native identity | `PeerId` | RNS destination hash | Keep namespaces distinct; never substitute a GhostWire root identity. |
| Discovery/route model | Overlay connections, DHT/mDNS/relay behavior as configured | RNS path discovery/interface configuration as configured by RNS | Do not infer anonymity from either transport’s encryption. |
| Payload/resource model | Carrier-specific stream/frame behavior | Official RNS encrypted packet mode, **383-byte** payload ceiling | No generic ciphertext chunks. Larger objects require an explicit native resource or separate private block-transfer contract. |
| Local receipt | Per-libp2p/lifecycle semantics | Local bridge acceptance and potentially RNS packet receipt | Neither equals recipient application delivery. |
| Privacy label | Varies by path, generally no higher than `ContentConfidential` for an ordinary direct link | `ContentConfidential` for the current explicit-destination packet bridge | A higher requirement must fail rather than downgrade. |

## Enforced Refusals

| Condition | Result |
|---|---|
| Invalid RNS destination | `RNS_HYBRID_INVALID_DESTINATION` before bridge use. |
| Message needs relationship-metadata protection | Carrier policy rejects current RNS packet mode; it is not relabeled as anonymous/private direct transport. |
| Payload is empty or exceeds 383 bytes | Fragmentation policy refuses it; no application-side split/retry. |
| Mobile endpoint tries to relay | Operating profile removes relay eligibility; policy rejects it. |
| Low battery, critical thermal state, metered/limited profile rejects new work | Typed operating-profile refusal; no lower-privacy route is tried. |
| Bridge unavailable/uninitialized or write fails | Explicit RNS bridge error; no synthetic local simulation or generic fallback. |

## Verification

| Gate | Latest result | Meaning |
|---|---|---|
| `cargo test --lib rns_hybrid --features reticulum` | 5 passed | Valid destination, policy no-downgrade, 383-byte bound, mobile relay refusal, and opaque-plan behavior. |
| `cargo check --features reticulum` | Passes | The real bridge manager and Core protected-packet API compile under the feature. |
| `python3 scripts/test_reticulum_bridge.py` | Passes | Two independent official-RNS TCP instances exchange bounded opaque packets and RNS returns delivery receipts both ways. |
| Default library suite | 317 passed, 0 failed, 1 ignored before this API’s focused feature verification | General deterministic regression baseline; it does not replace a live RNS or multi-machine acceptance test. |

## Required Before Any Broader Hybrid Claim

| Gap | Required acceptance condition |
|---|---|
| Shared ingress | Every RNS `incoming` event passes bounded decode, envelope authentication, CRL/trust, replay, expiry, causal ordering, and safe application delivery before it leaves the manager queue. |
| Protected-envelope codec | A versioned message-level ciphertext envelope is available for the RNS API; application plaintext is never submitted. |
| RNS Link/Resource | Official RNS Link/Resource lifecycle, resource limits, abort/receipt semantics, and privacy review are implemented and tested. |
| Concrete carrier registry | libp2p, RNS interface instances, Wi-Fi Direct, BLE, radio, and store-and-forward adapters publish factual capabilities to one runtime coordinator. |
| Cross-carrier relay | A validated ingress envelope can be handed off through an explicitly opted-in node with per-leg receipts and no plaintext bridge boundary. |
| Device enforcement | `OperatingProfile` limits are consumed by mesh connection, queue, carrier registry, and RNS lifecycle—not only the submission API. |
| Real-machine validation | A temporary Tailscale test path connects the sandbox to an isolated GhostWire test account on Omen/EliteKali/`cerberus-command` and tests the declared routes only. |

## References

[1]: https://reticulum.network/manual/reference.html "Reticulum API Reference"
[2]: https://reticulum.network/manual/interfaces.html "Reticulum Interface Configuration"
[3]: https://github.com/libp2p/specs/blob/master/connections/README.md "libp2p Connection Establishment Specification"
[4]: RESEARCH_EVIDENCE_RETICULUM_MULTICARRIER.md "GhostWire research evidence and implementation consequences"
