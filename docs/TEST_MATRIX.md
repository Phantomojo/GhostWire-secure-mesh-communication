# GhostWire Security, Reliability, and Release Test Matrix

**Status:** Required verification roadmap.
**Principle:** A passing library suite is necessary but not sufficient. Every supported delivery path, build profile, and externally visible claim needs an executable test boundary.

## Current test baseline

At the current audited revision, the default library suite passes (**322 passed, 0 failed, 1 ignored**). The full optional Reticulum-feature library profile also passes (**324 passed, 0 failed, 1 ignored**). The current branch additionally has focused passing tests for the external-RNS bridge, policy-bound RNS packet submission, carrier-specific fragmentation, HKDF transport/interface identity separation, and native multi-carrier policy eligibility. `cargo check --all-targets` also passes on this revision with two pre-existing unused-field warnings. The node-to-node integration target also compiles again with `cargo test --test node_to_node --no-run`, but its live concurrent two-node scenario still times out and cannot yet be a blocking runtime gate. `cargo check --no-default-features --features reticulum` still fails with 27 unrelated feature-gating errors. The default test count must therefore never be described as full project verification.

The immediate objective is to make required testing representative, deterministic, privacy-safe, and actionable. Test names should express the threat/property being proven. Tests that need real network/hardware must be isolated with clear prerequisities, resource limits, and ownership—not hidden with a global non-blocking CI setting.

## Required CI tiers

| Tier | Command/category | Required status | Purpose |
|---|---|---|---|
| Fast correctness | `cargo fmt --check`, `cargo clippy`, default compile/tests | Blocking | Fast feedback for source/API regressions. |
| Standard target compilation | `cargo check --all-targets` | Blocking | Covers default binaries, integration targets, and test-target compilation without coupling release confidence to the known unstable live-mesh runtime. |
| Default security properties | Core property/parser/CRL/causal/crypto tests | Blocking | Protects the highest-risk message/identity invariants. |
| Supported feature compilation | Declared feature-profile matrix | Blocking per supported profile | Prevents optional-code rot. |
| Non-live integration | Named API/crypto/protocol/postbox/AI integration targets | Blocking | Exercises deterministic integration behavior without the live-mesh harness. |
| Multi-node integration | Deterministic local 2/3-node scenarios | Blocking once stabilized | Proves dispatch/lifecycle/privacy behavior. |
| Fuzz/property regression | Corpus/property jobs with bounded time | Blocking on PR or scheduled, depending runtime | Finds parser/state-machine edge cases. |
| Docker playground | Compose startup/smoke test | Blocking on release/nightly | Verifies documented operator flow. |
| Supply chain | Locked license/SBOM/advisory checks | Blocking on release, scheduled otherwise | Prevents unaudited dependency drift. |
| Hardware/live network | Bluetooth/Meshtastic/real relay smoke | Explicitly quarantined | Valuable evidence but never hidden as passing core CI. |

## Security-property matrix

| Domain | Property to prove | Test class | Determinism controls | Release gate |
|---|---|---|---|---|
| Framing | Peer cannot force an allocation above each declared frame/decoded-envelope limit. | Unit, property, fuzz corpus. | Fixed arbitrary generators; allocation cap assertions. | P0 |
| Parsing | Non-canonical, malformed, unknown-version, and truncated messages fail before any side effect. | Negative corpus/property. | Pure codec; no network. | P0 |
| CRL | Invalid, future-dated, duplicate, and valid revocations have correct outcomes; revoked identities cannot send/connect. | Unit + multi-node integration. | Fake time; isolated store. | P0 |
| Trust/identity | Wrong identity binding, invalid signature, and failed handshake cannot reach command/delivery logic. | State-machine/integration. | Deterministic test keys. | P0 |
| Ratchet | Failed authentication does not commit provisional session state; skipped-key store stays bounded. | Unit/property. | State snapshot comparison. | P0 |
| Replay | Duplicate direct/gossip/Sphinx/mailbox frames are processed at most once. | Integration/property. | Controlled message IDs/nonces. | P0 |
| Causal ordering | Gap buffering, dependency wait, stale replay discard, and per-peer capacity policies are stable. | Existing deterministic/property suite expanded. | Fake clock, fixed scheduler. | P0 |
| Direct-message privacy | A third observer cannot receive/decrypt private direct delivery; no fixed shared direct topic is used. | Three-node black-box. | Local in-memory transports/adapters. | P0 |
| Privacy downgrade | Anonymity-required traffic queues/fails if no compliant route exists; it never silently switches to public gossip. | Coordinator simulation. | Fake adapters/capabilities. | P0 |
| Mailbox/DTN | Ciphertext-only persistence, per-recipient cap, expiry, idempotent retry, ACK deletion. | Store/integration fault injection. | Temp vault, fake clock. | P0 |
| Encrypted storage | Direct message persistence failure never invokes plaintext fallback. | Fault-injection regression. | Failpoint storage backend. | P0 |
| WebSocket | Unauthenticated clients cannot upgrade/subscribe; malformed command and untrusted origin reject; send command is not a no-op. | Axum WebSocket integration. | Loopback test server. | P0 |
| HTTP command service | One canonical message ID; byte budget; status/lifecycle truth; API and WS map same faults. | Contract/integration. | Schema snapshots. | P1 |
| Lifecycle | State transition graph is monotonic, idempotent, and only calls acknowledgement “delivered.” | Model/state-machine tests. | Test clock + deterministic persistence. | P1 |
| Core swap | Long-running request cannot block profile/core replacement indefinitely. | Concurrency test. | Barrier/timeout instrumentation. | P1 |
| Metrics/logging | Public fault/event serializations contain no peer, route, secret, payment, or plaintext data. | Snapshot/property. | Forbidden-string corpus. | P1 |
| Iroh offer | Signed offer recipient binding, nonce/replay, expiry, ALPN, and byte ceiling validation. | Codec/integration. | Fake endpoint/relay. | P3 |
| Reticulum bridge | The external bridge starts the official RNS runtime, creates distinct destinations, exchanges opaque bounded payloads in both directions, and exposes receipt semantics without contaminating its JSON control channel. | Black-box two-bridge interoperability plus lifecycle/negative tests. | Isolated RNS config/state, TCP-only fixture, timeout, no shared process state. | P1 |
| RNS hybrid packet submission | Only already protected opaque bytes with a valid RNS destination, truthful `ContentConfidential` capability, operating-profile eligibility, and a 383-byte direct plan reach the manager; bridge acceptance is not remote/app receipt. | RNS hybrid unit/model tests plus Reticulum-feature compile. | Fixed opaque payloads and local interface/profile declarations; no public RNS network. | P0 |
| Transport/interface identity | Root recovery material is never reused as a carrier identity; carrier domain, private interface instance, and rotation epoch produce distinct deterministic GhostWire-side child keys. | Unit/property. | Fixed root seed, opaque private interface IDs, no hardware/network identifiers in derivation context. | P0 |
| Carrier eligibility | Disabled, unsupported, permission-denied, background-ineligible, or non-opted-in interfaces cannot be selected; relay requires explicit opt-in. | Coordinator unit/model tests. | Pure capability declarations and no live radio dependency. | P0 |
| Cross-carrier fragmentation | A protected object takes only a direct/native-resource/private-block path; oversize RNS packet data and generic chunking are rejected. | Unit/property and adapter contract. | Fixed payload ceilings and bounded object sizes. | P0 |
| Private bulk foundation | Local block staging rejects invalid sizes, oversize objects/blocks, malformed manifests, unbounded wantlists, and excess route-scoped availability hints; output ordering is deterministic. | Unit/property. | In-memory store, fixed limits, no live mesh/DHT, no public provider discovery. | P0 |
| Operating profiles | Mobile defaults endpoint-only; relay needs explicit role plus background eligibility; battery/thermal/metered state can only reduce work; radio gateways reject generic bulk; resource refusal preserves message privacy intent. | Unit/model plus future platform integration. | Fixed local state fixtures; no live hardware for policy unit tests. | P0 |

## Feature profile matrix

Only features that compile and satisfy their minimum contract are **supported**. A feature flag alone is not a support statement.

| Profile | Current audit state | Required command | Minimum acceptance test |
|---|---|---|---|
| Default | Library suite and all-target compilation green; live node-to-node runtime remains unstable. | `cargo test --lib`, `cargo check --all-targets`, named non-live integration tests, and `cargo test --tests --no-run` | HTTP/API, mesh, postbox, protocol/crypto/AI targets run deterministically; node-to-node compiles until its runtime redesign is complete. |
| Minimal / no-default | Must be declared explicitly. | `cargo check --no-default-features` | Startup config validation and error surface. |
| Reticulum | **Experimental bounded bridge, not full transport support:** default-plus-Reticulum compiles; Core starts an external bridge to the official RNS runtime from explicit configuration. Isolated two-bridge tests prove bidirectional opaque 383-byte packet delivery and RNS receipt behavior. The no-default combination remains globally broken by unrelated web/mesh feature-gating assumptions. | `cargo check --features reticulum` plus `python3 scripts/test_reticulum_bridge.py` in an environment with the official RNS runtime | No fabricated RNS crypto/routing; isolated destination identities; bounded opaque envelopes; negative startup/MTU tests. Link/Resource, shared ingress, multi-interface handoff, and production support remain out of scope. |
| Matrix bridge | Unverified in this audit. | Dedicated no-default matrix command. | Adapter initialization + disabled-by-default behavior. |
| Meshtastic / serial | Platform/native dependencies. | Linux job and targeted feature command. | Bounded MTU/codec and unavailable-device error. |
| Bluetooth | Linux D-Bus environment. | Supported OS feature command. | Permission/device-unavailable fault; no secret leakage. |
| Iroh future feature | Design only. | Feature-specific compile/test job. | Offer validation and ciphertext-only transfer. |

## API and client contract tests

HTTP, WebSocket, TUI daemon client, and web UI must consume a shared versioned schema. Test the following response rules at every client boundary:

| Contract rule | Required assertion |
|---|---|
| Canonical ID | Returned operation/message ID equals the persisted/dispatched message’s ID. |
| Message type | Unsupported type rejects; supported type is explicitly dispatched/validated. |
| Byte budget | Multi-byte UTF-8 and encryption/frame overhead conditions have coherent failures. |
| Lifecycle truth | `Queued` is returned only after durable queue acceptance; `Dispatched` is not `Acknowledged`. |
| Error mapping | Each public failure maps to a stable error code/retry directive, not raw source text. |
| Redaction | Serializations exclude peer ID, multiaddr, IP, ciphertext/plaintext, payment data, and secret material. |
| Auth | Every mutation and WebSocket event subscription requires the configured authenticated local session. |
| Versioning | Older/newer schema payloads negotiate or fail with `PROTOCOL_VERSION_UNSUPPORTED`. |

## Fuzzing and property-testing backlog

Rust property tests should target pure state transitions and codec invariants. Fuzz harnesses should run against parsers/decoders that take attacker-controlled bytes. Every crash, timeout, excess allocation, or nondeterministic state transition becomes a minimized regression corpus item.

| Harness | Seed inputs | Invariants |
|---|---|---|
| Wire envelope decode | Random/truncated/oversized bytes; valid serialized corpus. | No panic; bounded allocation; canonical round-trip only. |
| CRL gossip | Signed/unsigned, future/past, duplicate, wrong issuer. | No invalid trust state; bounded store; monotonic valid merge. |
| Causal buffer | Vector clocks, sender IDs, capacity. | No duplicate delivery; correct readiness; bounded per-peer/global growth. |
| Payment request future codec | Scheme payloads, expiry, recipient binding. | No external action; no arbitrary URI/script; replay and malformed rejection. |
| DataPlaneOffer future codec | Tickets, ALPN, byte caps, signatures. | No unbound endpoint authority; no replay; cap enforced. |
| API/WebSocket inputs | JSON fields, headers, origins, token forms. | No panic; no auth bypass; no raw fault/metadata disclosure. |

## Docker playground verification

The playground should become a reproducible integration fixture, not only a demo. Each run must create isolated storage and deterministic test identifiers. The smoke script should wait for health, extract bootstrap identity, verify three-node membership, send a public test broadcast, assert expected lifecycle events, and preserve sanitized logs on failure.

A later privacy scenario must prove the negative property: an observer node can see only public/group traffic allowed by policy and cannot subscribe to or receive a private direct-delivery path. Do not use real personal data, production bootstrap nodes, public relays, or persistent volumes in CI.

## Test ownership and expiry policy

Every quarantined/flaky test requires a failure reason, owner, issue reference, and expiry date. A permanently `continue-on-error` test is not a reliability signal. CI must fail if a quarantine expires without explicit renewal. Build/test artifacts should include target triple, feature profile, lockfile hash, test command, and sanitized logs so a result is reproducible.

## First implementation batch

1. Rebuild the live node-to-node harness around injected identity/storage, deterministic readiness, and supervised teardown; then promote its runtime scenario to a blocking gate.
2. Add a storage failpoint test demonstrating that direct-message persistence failure prevents dispatch and never chooses a secondary non-private storage/delivery path.
3. Add WebSocket authentication/origin/command integration tests, including a `SendMessage` dispatch assertion and private-route error contract.
4. Connect the tested `TransportCoordinator` capability model to concrete adapters and durable lifecycle records, including native identity/discovery/fragmentation/interface-eligibility declarations.
5. Add a three-node direct-message metadata/privacy regression test for both missing-route refusal and a future supported private carrier.
6. Promote the black-box official-RNS two-bridge packet test to a separate environment-provisioned gate; keep Link/Resource, real radios, and Internet/Tailscale tests as explicit hardware/live jobs.
7. Keep the new deterministic library, all-target compilation, integration-target compilation, and non-live integration CI gates blocking; preserve live/hardware jobs as explicit separate status.
