# GhostWire Persistence Security Hardening Handoff

**Date:** 2026-08-17 UTC
**Author:** Manus AI
**Target:** Omen — `/home/ph/GhostWire-secure-mesh-communication-newest/ghostwire`
**Scope:** Identity auto-unlock boundary, MailboxStore durable records, EncryptionManager private-key lifecycle, isolated runtime validation, and production-preservation verification.

## Objective

The objective was to remove the remaining machine-trust persistence weaknesses identified after the passphrase-wrapped `PersistenceManager` tranche. Specifically, GhostWire had to stop treating a machine-recoverable identity sidecar as an implicit user authorization, stop leaving mailbox records outside the user-keyed persistence boundary, and stop allowing the separate EncryptionManager machine-local master key to remain an undeclared bypass around the user passphrase.

The user-visible security claim is now narrower and more accurate: **normal startup does not auto-unlock a persisted identity or private encryption state solely because the machine can read local sidecars. Explicit user authorization is required for normal unlock and migration.**

## Evidence before change

The identity implementation previously loaded `.identity.key` during `Identity::load_or_create()`, derived its decryption key from machine/path material, and unlocked the identity without interactive user authorization. New identity creation also wrote `.initial-password` in plaintext with restrictive file permissions. This was a measured source-level defect and was reproduced with an isolated red test asserting that a default load must remain locked when only the legacy sidecar is present.

`MailboxStore` persisted durable sled values outside `PersistenceManager`’s user lock state. The red test inspected the stored value and demonstrated that the old path could leave raw bincode at rest. The final mailbox tests now verify wrapped records, fail-closed locked access, authenticated metadata, malformed nonce rejection, and explicit legacy migration.

`EncryptionManager` maintained a separate `master.key` and stored private hybrid key material in a structure that did not distinguish public metadata from private unlock state. The user-keyed migration red test initially failed because the migration API did not exist. The final lifecycle test verifies user-keyed rewrap, removal of the machine master key, public-only restart, explicit activation, and rejection of private operations while locked.

## Authority and boundaries

`PersistenceManager` and its passphrase keyslot own user authorization for durable private state. `Identity` owns identity lock/unlock and explicit legacy recovery. `MailboxStore` may not override the persistence lock with direct sled reads. `EncryptionManager` may expose public metadata while locked, but it may not use a machine-local key to reconstruct private state after user-key migration.

The changes affect cryptographic persistence and migration semantics. They were applied only to the canonical Omen checkout through the existing Cloudflare SSH tunnel. The live production process and production identity were outside the authorization scope and were not restarted, replaced, or mutated.

## Root causes and changes

### Identity auto-unlock

The default identity startup path previously conflated machine possession with user authorization. The default path now leaves persisted identities locked when only a legacy `.identity.key` exists. Legacy sidecar recovery remains available only through the explicit recovery path and is not called during normal startup. New identities no longer write `.initial-password`. The Core recovery path migrates the verified legacy identity and associated persistence state, mailbox records, and encryption state as one explicit operation; failures re-lock the profile.

### MailboxStore durable state

Mailbox durable records are now wrapped with authenticated encryption through the user-keyed persistence boundary. The implementation preserves queue semantics and adds explicit legacy migration rather than silently interpreting raw records as already protected. Locked access fails closed. Tests cover pending records, inbox records, envelope metadata, tamper rejection, malformed nonce handling, eviction ordering, and migration.

### EncryptionManager private state

`EncryptionManager` now separates public metadata from private key material held in zeroizing user-unlock state. User-keyed state is stored in `encryption.user.gwpe` through `PersistenceManager` record encryption. A public stub permits startup to expose identity metadata without private key material. After migration, the legacy `master.key` is removed and machine-key helper operations reject user-keyed profiles instead of silently creating or using a machine fallback. `activate_user_keyed()` explicitly restores private state after the profile is unlocked. `lock_private_state()` clears the in-memory private material during Core lock and unlock/migration rollback. User-keyed rotation is exposed through `rotate_keys_if_needed_with_persistence()`; the old rotation path rejects user-keyed profiles rather than saving through the machine envelope.

## Modified source files

| File | Semantic change |
|---|---|
| `src/core/identity/core_identity.rs` | Default legacy-sidecar auto-unlock removed; explicit legacy recovery retained; new plaintext initial-password write removed; identity migration gate and locked-default tests added. |
| `src/core/mailbox.rs` | Durable mailbox records use user-keyed authenticated envelopes; locked access fails closed; explicit legacy record migration added; raw-at-rest and queue regression tests added. |
| `src/core/encryption.rs` | Public/private state separation, user-keyed state activation, public stub, machine-key retirement and fallback guard, user-keyed rotation API, private-state clearing, and lifecycle tests. |
| `src/core/persistence_manager.rs` | Existing GWPE/GWKS user-passphrase lifecycle and migration helpers used as the authoritative boundary for mailbox and encryption state. |
| `src/core/mod.rs` | Locked startup and explicit unlock/recovery orchestration; mailbox and EncryptionManager migration hooks; lock and rollback now clear private encryption state and re-lock persistence/identity. |
| `tests/ai_routing_tests.rs` | Existing repository-local dirty work preserved; not reset or cleaned. |
| `tests/crypto_tests.rs` | Existing repository-local dirty work preserved; not reset or cleaned. |
| `tests/node_to_node.rs` | Existing repository-local dirty work preserved; not reset or cleaned. |
| `tests/protocol_integration.rs` | Existing repository-local dirty work preserved; not reset or cleaned. |

Timestamped backup files created by the existing hardening workflow remain in the checkout. They were not deleted because the project rule forbids cleaning dirty work.

## Verification

| Check | Command or probe | Result |
|---|---|---|
| Identity red/green reproducer | Isolated `default_load_does_not_auto_unlock_from_legacy_sidecar` test | Pass; default startup remains locked. |
| Identity focused tests | `GHOSTWIRE_HOME=<isolated> cargo test --lib core::identity::core_identity::tests -- --test-threads=1` | Pass after the migration gate and round-trip test correction. |
| Mailbox focused tests | `GHOSTWIRE_HOME=<isolated> cargo test --lib core::mailbox::tests -- --test-threads=1` | Pass; wrapped records, locked failure, migration, tamper, nonce, and queue semantics verified. |
| Encryption focused tests | `GHOSTWIRE_HOME=<isolated> cargo test --lib core::encryption::tests -- --test-threads=1` | **8 passed, 0 failed.** |
| Encryption restart lifecycle | `user_keyed_restart_starts_public_only_until_explicit_activation` | **1 passed, 0 failed.** |
| Aggregate library suite before final Core lock refinement | `GHOSTWIRE_HOME=<isolated> cargo test --lib -- --test-threads=1` | **351 passed, 0 failed, 1 ignored.** |
| Aggregate library suite after final Core lock refinement | Same command with a fresh isolated home | **351 passed, 0 failed, 1 ignored**, completed in about 201 seconds. |
| Formatting | `cargo fmt --all -- --check` | Pass. |
| Diff validation | `git diff --check` | Pass. |
| Build | `cargo build` | Pass. Warnings remain, primarily deprecated constants, unused mutability, an unused `SecureMessageStore.encryption` field, and an unused `quic_port` variable. No compile errors. |
| Isolated daemon smoke test | Debug daemon on API `9697`, mesh `4601`, QUIC `8681`, temporary `GHOSTWIRE_HOME` | Process reached readiness; HTTP root returned `200`; mesh and API sockets bound; process was shut down cleanly. |
| Authenticated-route boundary smoke test | `/api/v2/network/status` and `/api/v2/system/mesh-health` on isolated API | Returned `401` without credentials, confirming the route auth boundary is active. Unauthenticated `/api/status` and `/api/v2/network/transport-health` returned `404` because those exact paths are not registered. |
| Production preservation | `ps -p 932431`; `ss` on `9097`, `4001`, `8081` | PID `932431` remained `./target/release/ghostwire --daemon --port 9097 --mesh-port 4001`; API `127.0.0.1:9097`, mesh `0.0.0.0:4001`, and QUIC `0.0.0.0:8081` remained bound throughout. |
| Temporary runtime cleanup | Smoke-test trap and post-cleanup checks | Isolated daemon process and temporary smoke-test homes were removed. No matching final hardening temp homes remained. |

The isolated daemon emitted one unrelated warning because `https://releases.ghostwire.app/latest.json` could not resolve during the sandboxed update check. This did not prevent startup, socket binding, API response, or clean shutdown.

## Process and identity safety

All changes were made on Omen through `ssh.ghostwire.cc` using the existing cached Cloudflare SSH proxy. The canonical checkout was never reset, cleaned, or replaced. The production process `932431` was never restarted or signalled. Ports `9097`, `4001`, and `8081` were never used by the isolated tests. Isolated tests used temporary `GHOSTWIRE_HOME` directories under `/tmp` and separate ports. No production identity was regenerated.

The live smoke daemon used a new temporary profile named `live-api-smoke` and a temporary home. It created no production files and was terminated by a cleanup trap after API and socket probes.

## Deferred risks

The implementation is materially stronger, but it is not complete enough to claim a finished production rollout.

First, the live daemon smoke test validated startup, socket binding, unauthenticated route rejection, and clean shutdown, but it did not perform an authenticated API unlock transaction against a running isolated daemon. That needs a controlled authenticated test using a disposable profile and password, not production credentials.

Second, `EncryptionManager::secure_save()` and `secure_load()` remain as compatibility primitives for the legacy machine envelope. The final guard prevents their machine-key path from silently operating after user-key migration, but all call sites should be audited and either removed, explicitly marked legacy-only, or routed through `PersistenceManager`.

Third, Core’s startup behavior with a locked identity and locked private encryption state needs a full authenticated client workflow test. Some routes and background services may still assume private signing material is immediately available. The unit suite proves the cryptographic state transitions, not every daemon subsystem’s behavior while locked.

Fourth, mailbox migration is explicit, but rollback atomicity across identity, persistence, mailbox, and encryption stores is not a single transaction. A crash between stores can leave a partially migrated profile. The next hardening tranche should add a durable migration journal or versioned migration marker with recovery semantics.

Fifth, the repository still contains substantial pre-existing warnings and dirty test/source files. The warnings do not block compilation, but the unused `SecureMessageStore.encryption` field and broad deprecated-constant usage are evidence that the Rust quality batch remains unfinished.

Sixth, the production binary still needs a controlled restart and post-restart validation. That action is intentionally deferred because the user required explicit authorization before touching PID `932431` or production ports.

## Next action

The next exact implementation step is an authenticated isolated daemon lifecycle test. Use a fresh temporary `GHOSTWIRE_HOME`, non-production ports `9797/4701/8781`, a disposable profile, and the existing local API authentication mechanism. Exercise: create or enroll the profile with a disposable passphrase, restart, verify locked identity and public-only encryption state, unlock with the passphrase, perform one authenticated status operation, lock again, verify private operations fail, and cleanly terminate the daemon. Do not use the production profile or password.

Only after that test passes should the operator decide whether to authorize a controlled production restart. No restart is authorized by this handoff.

## Status block

```text
STATUS REPORT
─────────────
✅ Done:      Identity default auto-unlock gate, mailbox user-keyed durable records, EncryptionManager user-keyed private state, machine-key fallback guard, Core lock rollback clearing, isolated daemon smoke test, 351-library-test aggregate, formatting, diff validation, and build verification.
⚠️  Issue:    Authenticated daemon unlock workflow and cross-store crash-recovery journal are not yet validated or implemented. Legacy machine-envelope compatibility helpers remain in the source but reject user-keyed profiles.
❌ Deferred:  Production restart and deployment; authenticated live workflow; migration journal; full warning/clippy cleanup; complete call-site retirement of legacy machine-envelope primitives.
📋 Next:      Run the authenticated isolated daemon lifecycle test on ports 9797/4701/8781 with a disposable profile and temporary GHOSTWIRE_HOME. Then request explicit authorization before any production restart.

HANDOFF — 2026-08-17
Compile status: clean; warnings remain, no errors
Files modified: src/core/identity/core_identity.rs, src/core/mailbox.rs, src/core/encryption.rs, src/core/persistence_manager.rs, src/core/mod.rs; existing dirty test files preserved
Files needing attention: authenticated daemon lifecycle; migration journal; legacy secure_save/secure_load call-site audit; production restart authorization; Rust warning/clippy batch
Resume from: authenticated isolated daemon lifecycle test on ports 9797/4701/8781
```

## References

[1]: /home/ph/GhostWire-secure-mesh-communication-newest/ghostwire/src/core/identity/core_identity.rs "GhostWire identity implementation"

[2]: /home/ph/GhostWire-secure-mesh-communication-newest/ghostwire/src/core/mailbox.rs "GhostWire MailboxStore implementation"

[3]: /home/ph/GhostWire-secure-mesh-communication-newest/ghostwire/src/core/encryption.rs "GhostWire EncryptionManager implementation"

[4]: /home/ph/GhostWire-secure-mesh-communication-newest/ghostwire/src/core/persistence_manager.rs "GhostWire PersistenceManager implementation"

[5]: /home/ph/GhostWire-secure-mesh-communication-newest/ghostwire/src/core/mod.rs "GhostWire Core orchestration"

[6]: /home/ph/GhostWire-secure-mesh-communication-newest/ghostwire/.ai/AGENT_HANDOFF_TEMPLATE.md "GhostWire handoff template"
