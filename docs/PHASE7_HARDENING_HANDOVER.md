# GhostWire Phase 7 Hardening and Research Handover

**Branch:** `feat/security-hardening-and-optimizations`
**Publication status:** Local commits are complete. A temporary GitHub credential has been configured for publication, but the sandbox currently cannot connect to `api.github.com`; publication must resume when GitHub network reachability returns.
**Continuation objective:** Publish the branch/PR after credentials or connectivity are restored, then continue the P0 transport-policy/private-direct-delivery redesign from the decision register.

## Verified branch state

The branch contains the earlier stabilization, Core-stack refactor, identity/CRL/causal/QUIC hardening, property testing, Docker playground, and libp2p–Iroh offline design work. This phase added the following commits:

| Commit | Purpose |
|---|---|
| `2d06f1c8` | Restored compilation of the node-to-node integration fixture after the mesh constructor change and removed direct test dependence on process-global persistence configuration. |
| `6bdfc71a` | Added a privacy-safe public error/fault foundation; corrected API dispatch truthfulness; protected and rewired WebSocket command dispatch. |
| `d14cb15a` | Added the architecture audit, Kenya-centered payment design, licensing audit, error index, test matrix, and architectural limits decision register. |
| `2676a85f` | Made private direct-message dispatch fail closed without a Sphinx route, added cancellation-safe reliability tracking, and added a deterministic transport privacy-policy foundation. |
| `09151eff` | Recorded the new containment and remaining shared-ingress limitations in the architecture decision register. |
| `5145de2a` | Disables legacy direct-topic recipient forwarding and prevents generic retransmission from exposing completed Sphinx sends. |
| `ef17edcc` | Exposes privacy-route unavailability as a stable safe HTTP/WebSocket error contract with focused controller coverage. |
| `e59d237e` | Replaces non-blocking aggregate CI testing with required deterministic library, all-target compile, integration-compile, and named non-live integration gates. |
| `190a32aa` | Clarifies that public broadcasts remain encrypted at rest, adds a safe storage API name, and isolates store tests from global vault locks. |
| `8ffec185` | Makes the Reticulum profile compile-safe but explicitly runtime-unsupported, replacing the non-interoperable prototype's false privacy/transport claim. |

## Implemented improvements

### Public faults and error taxonomy

`ghostwire/src/error_catalogue.rs` now contains an expanded stable `ErrorCode` registry, `FaultCategory`, explicit `RetryDirective`, `PublicFault`, and a protected internal `GhostWireError` representation. Internal peer and route context are skipped during serialization and public conversion returns only a stable safe code/category/retry/message. Five focused tests confirm registry uniqueness, metadata completeness, retry policy, display compatibility, and redaction.

This is a **foundation**, not a completed migration. Controllers, core errors, WebSocket events, and TUI responses must incrementally adopt `PublicFault`; no raw source chain should cross a client boundary. See [`ERROR_INDEX.md`](ERROR_INDEX.md) and `LIM-011` in [`ARCHITECTURE_LIMITS_REGISTER.md`](ARCHITECTURE_LIMITS_REGISTER.md).

### API dispatch containment

`MessagingController` now clones the swappable `Arc<Core>` under a short read lock before awaiting calls. It uses the actual dispatched message’s UUID, validates byte length and empty payloads, rejects unsupported `msg_type` rather than silently ignoring it, logs dispatch source errors locally, and uses `BROADCAST_DISPATCHED` instead of falsely calling a local dispatch a queued or delivered recipient outcome.

A durable lifecycle/acknowledgement store is still required. The API currently reports only local dispatcher acceptance. See `LIM-006` and `LIM-007`.

### WebSocket containment and command wiring

The `/ws` route now passes through existing daemon authentication instead of bypassing it. The handler no longer claims to provide custom per-session encryption with an undistributed key/unencrypted fallback. It relies on the authenticated local daemon transport, clones Core snapshots before awaits, accounts for subscribers, validates bounded input, sends direct/broadcast messages through Core, and returns `MessageDispatched` rather than recipient delivery.

Before any non-loopback exposure, add authenticated WebSocket integration tests, explicit browser-compatible authentication/origin policy, shared API schema, stable event filtering, TLS deployment rules, and an event payload privacy review. Direct privacy-route failures now map to `PRIVATE_DELIVERY_UNAVAILABLE` without raw source detail; the broader typed-fault migration remains open. See `LIM-005` and `LIM-011`.

### Fail-closed direct-message privacy policy

The mesh no longer downgrades a private direct-message operation to the shared `DM_TOPIC` or epidemic DTN when a Sphinx route cannot be established. It returns `PRIVATE_DELIVERY_UNAVAILABLE`, removes the pending message from local retransmission state, and cancels both mesh and Core reliability entries without fabricating an acknowledgement. This prevents a later retry from leaking through the legacy metadata-visible carrier.

`core::transport_policy` establishes the required long-term contract: every adapter must declare what delivery semantics and network-observable privacy class it actually provides. The deterministic coordinator selects only available candidates that meet the immutable operation requirement. In particular, a relationship-metadata-protected direct message rejects the current shared GossipSub pattern rather than silently downgrading to it. Five focused tests cover ordering, unavailability, capability mismatch, public broadcast selection, and privacy mismatch.

This is **containment rather than full direct-message support**. The legacy `DM_TOPIC` receiver now discards frames whose embedded recipient is another node and no longer relays shared-topic private frames; successful Sphinx sends also do not enter the legacy-topic retry path. Compatibility/control uses of the topic and the general ingress paths must still be disentangled behind the shared validator design in `LIM-003`.

### Node-to-node test repair and discovered limit

The node-to-node target now compiles with `cargo test --test node_to_node --no-run`. The fixture uses explicit persistence directories and guards the legacy environment-driven identity creation only for its short construction window. When run as a live concurrent mesh scenario, the test still timed out after the fixture repair. This is a useful verified architecture/reliability finding: it must not be hidden by raising timeouts or permanently ignored.

The recommended two-track path is documented in `LIM-009`: make deterministic simulated/adversarial adapter tests blocking now, while rebuilding the live libp2p harness around readiness signals, fully injected identity/storage, supervised driver teardown, deterministic ports/resources, and leak checks. The live harness should become blocking only after it passes repeatedly under parallel execution.

## Verification evidence

| Command | Result |
|---|---|
| `cargo test error_catalogue --lib -- --nocapture` | Passed: 5 error-catalogue tests. |
| `cargo check --lib` after WebSocket change | Passed, with pre-existing unused-field warnings. |
| `cargo test --test node_to_node --no-run` | Passed: integration target compiles. |
| `cargo check --all-targets` after phase-nine transport work | Passed across default build targets; only the known unused-field warnings remain. |
| `cargo test controllers::messaging::tests --lib -- --nocapture` | Passed: 2 public privacy-route error mapping regressions. |
| `cargo test core::store::tests --lib -- --nocapture` | Passed: 2 isolated encrypted storage regressions. |
| `cargo test --lib -- --nocapture` before phase-nine privacy changes | Passed: **276 passed, 0 failed, 1 ignored**. |
| `cargo test --lib -- --nocapture` after phase-ten storage hardening | Passed: **285 passed, 0 failed, 1 ignored**. |
| `cargo test --test node_to_node -- --nocapture` | Does not yet pass reliably: live concurrent harness timed out. This is tracked, not suppressed. |
| Aggregate non-live integration target attempt | Did not complete because the sandbox terminated a linker process with signal 15 while building `mesh_isolate`; this was a resource/command-lifetime interruption, not a failed test assertion. Run each target in CI on a standard runner before making it blocking. |
| `cargo check --features reticulum` | Passed after the fail-closed Reticulum boundary repair. This is compile-only evidence; runtime initialization and connection deliberately return `RETICULUM_UNSUPPORTED`. |
| `cargo check --no-default-features --features reticulum` | Still fails because the no-default profile globally assumes web/mesh modules in several callers; this is a broader feature-gating issue, not evidence of a working Reticulum bridge. |
| GitHub publication preflight | Temporary credential configured, but a direct authenticated GitHub API request fails because the sandbox cannot connect to `api.github.com`. |

The full repository formatting check currently reports pre-existing formatting debt in unrelated TUI/test files and the prior QUIC change. It was not mass-formatted during this phase to avoid unrelated churn. `git diff --check` was clean before commits.

## Documentation package

| Document | Purpose |
|---|---|
| [`ARCHITECTURE_AUDIT.md`](ARCHITECTURE_AUDIT.md) | Evidence-based protocol/wiring audit and priority roadmap. |
| [`ARCHITECTURE_LIMITS_REGISTER.md`](ARCHITECTURE_LIMITS_REGISTER.md) | Living decision register for every verified limit, containment, long-term solution, and acceptance evidence. |
| [`ERROR_INDEX.md`](ERROR_INDEX.md) | Public fault ranges, retry rules, redaction policy, and migration plan. |
| [`TEST_MATRIX.md`](TEST_MATRIX.md) | Required CI, property, fuzz, multi-node, Docker, and feature-profile checks. |
| [`PAYMENT_INTEGRATION_DESIGN.md`](PAYMENT_INTEGRATION_DESIGN.md) | Non-custodial payment-request-only design with Kenya-centered legal/privacy risk boundaries. |
| [`LICENSE_AUDIT.md`](LICENSE_AUDIT.md) | Partial lockfile audit, AGPL/MPL analysis, and release gates. |

## Priority continuation order

1. Restore GitHub credentials/connectivity, push this branch, and open a PR that explicitly discloses the open P0 limits and the live-harness limitation.
2. Connect the new deterministic `TransportCoordinator` capability model to every concrete adapter, then add no-downgrade, lifecycle, retry, and three-node observer privacy tests.
3. Replace remaining legacy fixed-topic control/direct delivery and route all carriers through one bounded ingress pipeline. The current send-side direct-message downgrade is contained, but full private delivery requires a supported Sphinx/mailbox/direct-stream path.
4. Complete WebSocket/API shared command schema and test it end-to-end with protected auth/origin behavior and redacted public faults.
5. Replace global identity storage lookup with injectable/in-memory identity/storage interfaces; then rebuild the live libp2p harness and make it a blocking gate.
6. Reticulum now compiles as an explicit fail-closed scaffold, not a supported bridge. Replace it only with a real bounded external RNS bridge that transports opaque envelopes, binds GhostWire identity to destination identity, enforces MTU/backpressure/expiry, and has contract tests. Keep Iroh feature-gated/ciphertext-only under signed recipient-bound offers.
7. Re-run the license audit online for all feature profiles, generate notices/SBOM, and make the release gate blocking. The deterministic CI gate structure is now committed, but its first hosted execution still needs review after publication.

## Publication recovery commands

After GitHub network connectivity is restored, use the following sequence from the repository root:

```bash
gh auth status
git push -u origin feat/security-hardening-and-optimizations
gh pr create \
  --repo Phantomojo/GhostWire-secure-mesh-communication \
  --base main \
  --head feat/security-hardening-and-optimizations \
  --title "Hardening: protocol audit, privacy-safe faults, and secure WebSocket dispatch" \
  --body-file /tmp/ghostwire-pr-body.md
```

The PR body must call out the open direct-message metadata problem, incomplete live mesh test, unsupported Reticulum feature, and incomplete online license verification. Do not describe those capabilities as solved or production-ready.
