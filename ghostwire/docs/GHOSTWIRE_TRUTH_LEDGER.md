# GhostWire Capability Truth Ledger

> Status: generated from the newest Omen checkout audit. This ledger is intended to become the source of truth for repository documentation and the public website. It must be reviewed before publication.

## Capability status

| Capability | Current evidence | Status | Required wording |
|---|---|---|---|
| Rust/libp2p mesh core | Compiles with the default feature set and contains active transport, routing, and security modules | Implemented but not release-ready | “Implemented prototype/core; interoperability and release verification remain in progress.” |
| End-to-end session encryption | vodozemac-backed account/session path is present; sealed mailbox fallback is separate | Partially implemented | Distinguish authenticated ratcheted sessions from offline sealed envelopes. |
| Transport switching | Selector and manager code exist; transport switching has tests and integration boundaries | Implemented in core, incomplete in operational proof | Do not claim every transport is production interoperable. |
| Sphinx/onion routing | Sphinx module and tests exist | Prototype/limited integration | Say “Sphinx routing prototype” until multi-hop deployment evidence exists. |
| Reticulum/RNS | Optional adapter and transport feature exist; `init_reticulum()` wired into `initialize_mesh_after_unlock()` (2026-10-26). RNS initializes on profile unlock, registers destination, starts receive pump. **RNS local address now exchanged in handshake** (2026-10-27). **GhostWire private RNS overlay (ghRNS)** designed: IFAC on all interfaces, destination name changed to ghostwire.mesh, default config includes network_name + passphrase. Cross-transport discovery tries RNS first if peer has RNS address, falls back to libp2p. Actual message delivery still uses libp2p gossipsub until RNS addresses populate topology. | Optional integration (initialized, handshake wired, IFAC configured, not yet carrying traffic) | Do not claim RNS is the primary transport. RNS is standby/fallback; libp2p carries all traffic. Address exchange and IFAC wired but delivery untested. |
| Post-quantum cryptography | ML-KEM-768 (pqcrypto-mlkem) IS integrated in encryption.rs with HKDF-SHA256 mixing; encrypt/decrypt path includes pq_key in signature for downgrade prevention. Production handshake uses hybrid (X25519 + ML-KEM-768). NOT verified as interoperable or formally audited. At-rest/PQ profile selection exists but untested cross-implementation. | Implemented in code, unverified for interoperability | Qualify as "hybrid (X25519 + ML-KEM-768) in code; interoperability and formal audit pending." |
| AI/ML intelligence | Intelligence workspace and heuristic/placeholder modules exist; no measured production ML pipeline | Research/protocol MVP | Describe as “privacy-preserving intelligence architecture under development.” |
| Binary integrity | Runtime hash hooks and updater verification code exist, but build hash/key configuration is unset in current builds | Disabled unless configured | Never state binary integrity is active by default. |
| Signed updates | Updater verification code exists, but current key configuration is ambiguous and default key is a zero placeholder | Not release-ready | Block release until public verification key and manifest tests are configured. |
| “No logs/no traces” | The system contains local state, metrics, errors, telemetry, and persistence paths | False as an absolute claim | Use a precise data-retention and operator-observability statement. |
| Censorship/coercion resistance | Architectural goals and traffic/transport features exist; no adversarial deployment evidence | Unverified claim | State threat-model goals, not achieved guarantees. |
| Nine live transports | Multiple transport modules/types exist, but “live” interoperability is not established for all | Overstated | Publish a transport matrix with tested status per adapter. |

## Release blockers

The aggregate test suite currently has a failing node-to-node integration path and failing doctests. Clippy with `-D warnings` fails at project scale. Release signing and binary hash configuration are absent from the current environment. These are release blockers, not cosmetic issues.

## Website correction policy

Every public capability claim must link to one of: a passing integration test, a reproducible deployment report, a protocol specification, or a clearly labelled roadmap item. Marketing language must not convert a module name, feature flag, or research stub into a verified operational guarantee.

## Required evidence fields

Each capability record should include the commit or build identifier, feature flags, test command, test result, environment, known limitations, and security classification. If any field is missing, the capability remains provisional.
