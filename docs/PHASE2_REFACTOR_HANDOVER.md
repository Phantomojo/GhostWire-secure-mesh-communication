# GhostWire Phase 2 Refactor Handover

## Scope and verification state

This handover records the current **Phase 2: Core Decoupling** implementation. The prior `Core` god-struct has been separated into explicit ownership domains, and the branch has been compiled and tested after that migration.

| Area | Current state | Verification |
|---|---|---|
| Core structure | `Core` delegates identity, network, security, storage, AI, and social responsibilities to dedicated stack structs. | `cargo test --lib` completed with **273 passed, 0 failed, 1 ignored**. |
| Runtime configuration | `ArcSwap<Config>` provides atomic read access for configuration snapshots. | Compiled and covered by the full library build. |
| Mesh runtime references | `ArcSwapOption` is used for mesh and Reticulum manager replacement. | Compiled and covered by the full library build. |
| Key revocation | CRL enforcement remains integrated in mesh processing and rejects revocations beyond a five-minute permitted sender clock lead. | Unit and manager-level gossip acceptance, duplicate, expiry, signature, and future-timestamp rejection tests pass. |
| Causal ordering | `CausalBuffer` buffers messages until vector-clock dependencies are satisfiable and discards stale/replayed sender counters. | Deterministic same-sender gap, cross-sender dependency, and replay-discard tests pass. |
| Sensitive identity storage | `EncryptedIdentity` uses `ZeroizeOnDrop`; Argon2-derived identity keys use `Zeroizing`; identity locking retains its explicit keypair-consumption path. | Identity encryption, recovery, malformed nonce, and lock tests pass. |
| Error taxonomy | `core::error::GhostError` provides the typed-error foundation and transparently wraps `IdentityCryptoError`. | The identity-crypto boundary no longer returns `anyhow` errors. |

## Core ownership boundaries

| Stack | Owns | Notes |
|---|---|---|
| `IdentityStack` | Identity handle, identity cache, pairing manager | Identity state remains lock-protected because unlock and lock transitions are stateful. |
| `NetworkStack` | Mesh/Reticulum handles, transport and adapter state, monitor, bootstrap nodes, handshake secret | Manager handles use atomic swap semantics. The handshake secret is zeroized during network-stack teardown. |
| `SecurityStack` | Encryption, security manager, trust store, audit and panic components | Security policy decisions remain centralized. |
| `StorageStack` | Persistence, encrypted message store, mailbox, block and DTN stores | `PersistenceManager::open_at` now permits explicit isolated vault paths. |
| `AiStack` | Model handles and feature extractor | Keeps optional model integrations away from transport and identity logic. |
| `SocialStack` | Social graph and Ghost Channels registry | Social notifications are broadcast events, rather than direct UI state mutation. |

## Channel migration: safe next increment

`RwLock` is not a blanket anti-pattern: read-mostly immutable handles and short, in-process state transitions can remain lock-protected. The next channel migration should target the **mutable, async mesh runtime**, which is presently accessed through `RwLock<MeshManager>`.

1. Introduce a `MeshCommand` enum for send, direct-send, query-stats, start, and shutdown operations.
2. Run the mutable `MeshManager` as the sole owner in one Tokio task, with a bounded `mpsc` receiver.
3. Use one-shot reply channels for command results and enforce backpressure with a documented queue capacity.
4. Expose a lightweight `MeshHandle` from `NetworkStack` instead of exposing the manager lock to `Core` consumers.
5. Preserve a subscription-only broadcast channel for non-sensitive UI/monitor events; do not broadcast private message payloads indiscriminately.
6. Add cancellation and shutdown tests before removing the direct manager reference.

This strategy should be introduced separately from the completed stack split to keep behavioral regressions reviewable.

## Test-suite upgrades in this branch

The branch adds `proptest` and property-oriented coverage for vector-clock invariants. It also eliminates several flaky assumptions uncovered during full-suite execution:

- Message age tests now respect the intentional ±30-second timestamp privacy jitter.
- Social-graph tests no longer assume that inbound request notifications arrive before peer-discovery notifications.
- GNN topology tests no longer depend on non-deterministic `HashMap` node indexing.
- Persistence tests use `PersistenceManager::open_at` with per-test vault directories instead of sharing the process-global `GHOSTWIRE_HOME` environment variable.
- CRL gossip tests prove a signed entry is accepted once, recognized as a duplicate on replay, and rejects implausibly future-dated entries.
- Causal-buffer tests prove a same-sender gap is held and released in order, cross-sender dependencies wait, stale replays are discarded rather than occupying the bounded buffer, and capacity evicts the oldest buffered message deterministically.
- Identity and recovery code reject malformed XChaCha20 nonce lengths before a slice-construction panic can occur.

## Docker playground

`playground/` defines a three-node local mesh: one bootstrap node and two peer nodes. Its launcher waits for the bootstrap node's runtime-generated peer ID, then supplies a valid `/ip4/.../tcp/.../p2p/...` Multiaddr through `GHOSTWIRE_BOOTSTRAP_NODES` to the peer nodes.

The standard-library Python smoke test validates API health, peer visibility, and broadcast delivery from node 1 to node 2. It deliberately does not claim to prove anonymity, adversarial resilience, causal ordering under reordering, direct-message routing, or revocation propagation.

> Docker Engine was unavailable in the current build environment, so the playground configuration was reviewed against GhostWire's CLI and configuration code but could not be executed end-to-end here.

## Publication status

The local branch currently includes these relevant commits:

1. `chore: remove tracked Cargo build artifacts` removes approximately 592 MB of accidentally versioned Cargo output and adds nested-target ignore coverage.
2. `refactor: harden core stacks and add mesh simulation` contains the stack refactor, CRL/causal-ordering wiring, hardening foundation, tests, and playground.
3. `docs: add Phase 2 refactor handover` records architecture boundaries and continuation steps.
4. `hardening: type identity crypto failures and validate revocations` adds typed identity-crypto failures, derived-key zeroization, nonce validation, and CRL clock-skew rejection.
5. `test: cover causal gaps and CRL gossip propagation` adds manager-level CRL gossip coverage and deterministic causal release tests.
6. `hardening: reject stale causal message replays` tightens causal sender-counter validation and discards stale buffered replays.
7. `test: cover causal buffer capacity eviction` proves the bounded buffer evicts its oldest entry and retains later causal messages.

GitHub API connectivity was unavailable when publication was attempted. The branch is committed locally and ready to push/create a pull request once connectivity resumes.

## Recommended next work

1. Publish and review the current branch before broadening the actor/channel migration.
2. Migrate `MeshManager` to a bounded command actor as outlined above.
3. Adopt `GhostError` module-by-module, starting with the wire protocol and persistence boundaries.
4. Add simulated-time tests for TTL expiry, CRL garbage collection, and causal-buffer capacity/eviction behavior.
5. Build a containerized CI job that runs the playground smoke test where Docker-in-Docker or a service runner is available.
6. Continue the zeroization audit for long-lived transport, ratchet, and pairing secrets.
