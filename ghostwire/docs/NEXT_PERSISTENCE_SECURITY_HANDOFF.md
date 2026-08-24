# GhostWire Next Persistence Security Handoff

**Target:** Omen, `/home/ph/GhostWire-secure-mesh-communication-newest/ghostwire`
**Date:** 2026-08-17
**Scope:** Isolated passphrase lifecycle validation, remaining persistence-boundary audit, and fail-closed EncryptionManager recovery.

## Executive result

The passphrase-wrapped `PersistenceManager` lifecycle was validated end to end in an isolated Omen environment. A legacy machine-key store was created, populated, reopened locked, rejected access while locked, migrated through the verified migration API, reopened with the migrated user keyslot, rotated to a new passphrase, locked, rejected the old passphrase, and successfully reopened with the new passphrase.

A separate concrete security/correctness defect was fixed in `EncryptionManager`: corrupted or undecryptable `encryption.json` state no longer causes silent generation and persistence of a new encryption identity. Existing state now fails closed and remains untouched, forcing an explicit recovery decision instead of silently changing the node’s cryptographic identity.

## Isolated passphrase validation

The temporary test used a dedicated `GHOSTWIRE_HOME` under `/tmp` and the public environment-based constructors used by Core. It did not use production data, the production identity, or production ports.

The verified cycle was:

| Stage | Observed result |
|---|---|
| Create legacy machine-key store | Passed |
| Write and flush private record | Passed |
| Reopen through locked constructor | Passed |
| Read while locked | Rejected as expected |
| Identity-authenticated legacy migration | Passed |
| Read migrated record | Original secret preserved |
| Rotate passphrase | Passed |
| Lock after rotation | Passed; access rejected |
| Reopen user-mode store | Passed |
| Old passphrase | Rejected |
| New passphrase | Accepted; original secret recovered |

The exact isolated reproducer passed:

```text
running 1 test
test isolated_legacy_migration_unlock_rotation_and_lock_cycle ... ok
test result: ok. 1 passed; 0 failed
```

The temporary test file and temporary home were removed after execution.

## EncryptionManager fix

Before the change, `EncryptionManager::load_or_create()` attempted to load `encryption.json`; if loading or decryption failed, it logged a warning and fell through to `create_new()` followed by `save_to_disk()`. That behavior could silently rotate the node’s persistent signing/encryption identity after corruption, wrong machine state, truncated storage, or a keyfile problem. It is unacceptable for security infrastructure because the caller receives a fresh identity without an explicit recovery decision.

The implementation now uses a strict existing-state loader:

```rust
if storage_path.exists() {
    let manager = Self::load_existing(&storage_path)?;
    return Ok(manager);
}
```

`load_existing()` delegates to the existing authenticated loader and propagates errors. The original artifact is not overwritten on failure. New key generation remains available only when the persistence file does not exist.

The red reproducer failed before implementation with:

```text
error[E0599]: no associated function or constant named `load_existing`
```

After implementation, the same reproducer passed:

```text
running 1 test
test core::encryption::tests::corrupted_persistent_state_fails_closed_without_rotation ... ok
test result: ok. 1 passed; 0 failed
```

## Final verification matrix

The final Omen regression run used a dedicated temporary `GHOSTWIRE_HOME` and completed with these results:

| Command/check | Result |
|---|---|
| `cargo fmt --all -- --check` | Passed |
| `git diff --check` | Passed |
| `GHOSTWIRE_HOME=<temporary> cargo check --lib` | Passed |
| `GHOSTWIRE_HOME=<temporary> cargo build` | Passed |
| Fail-closed EncryptionManager test | Passed, 1/1 |
| PersistenceManager tests | Passed, 9/9 |
| Passphrase keyslot tests | Passed, 3/3 |
| Aggregate library tests | Passed, 345 passed, 0 failed, 1 ignored |
| Isolated migration test cleanup | Passed |

The aggregate suite ran with `--test-threads=1` and completed in approximately 190 seconds. Existing warnings remain; no new warning policy was introduced by this work.

## Remaining persistence boundaries

### Identity auto-unlock remains a real trust-boundary problem

`Identity::load_or_create()` automatically decrypts `.identity.key` and uses the recovered password to unlock the identity. The keyfile’s wrapping key is derived from the hostname and identity path, neither of which is a secret. File permissions are `0600`, which is useful against other unprivileged local users, but anyone who can read the user’s home and determine the hostname/path can recover the identity password from the sidecar.

New identities also write `.initial-password` in plaintext with mode `0600`. This is an operational convenience, not high-grade password protection. I did not remove or change this behavior in the current block because doing so without a replacement user-facing unlock/recovery flow would risk locking the operator out of the identity and would alter startup behavior beyond the requested isolated validation.

The next correct implementation is an explicit opt-in migration gate: stop creating new plaintext `.initial-password` files, stop auto-unlocking from `.identity.key` by default, preserve legacy sidecars only for a user-authorized migration, and provide an explicit recovery command. This needs a deliberate UX/recovery design before code changes.

### MailboxStore remains outside passphrase gating

`MailboxStore` opens a separate sled database under `GHOSTWIRE_HOME/mailbox/<profile>/db`. It stores sealed envelopes, recipient IDs, message IDs, timestamps, retry counters, and queued outgoing frame bytes directly with bincode. The normal message payload is recipient-sealed, but local mailbox metadata and queued frames are not controlled by `PersistenceManager::lock()` and are not passphrase-gated.

This is not equivalent to plaintext message storage, but it is still a durable metadata and ciphertext-cache boundary. The correct fix is to integrate mailbox records into the authenticated `PersistenceManager` or give the mailbox store an explicit key lifecycle. A blind wrapper around the current sled database would risk breaking queue recovery and concurrent delivery behavior.

### EncryptionManager remains a separate key hierarchy

The fail-closed load behavior is fixed, but `EncryptionManager` still uses a separate machine-local `master.key` to protect `encryption.json`; it is not yet wrapped by the user passphrase keyslot. This is now a clearly bounded residual risk rather than a silent rotation risk.

## Process and identity safety

Final Omen safety state:

```text
932431 ghostwire Sl
udp  0.0.0.0:8081  owned by ghostwire PID 932431
tcp  127.0.0.1:9097 owned by ghostwire PID 932431
tcp  0.0.0.0:4001  owned by ghostwire PID 932431
```

Production was not restarted, stopped, migrated, or otherwise mutated. The isolated test used temporary data and no production ports. The canonical dirty checkout was preserved; no reset or clean operation was performed.

## Modified files in this block

| File | Change |
|---|---|
| `src/core/encryption.rs` | Added fail-closed `load_existing()` path and regression test for corrupted persistent state |
| `tests/isolated_passphrase_migration.rs` | Temporary isolated reproducer, installed only for the test and removed afterward |
| `docs/NEXT_PERSISTENCE_SECURITY_HANDOFF.md` | This handoff report |

The exact formatted EncryptionManager source is synchronized in the attached continuation archive.

## Resume point

The next best implementation is the **identity auto-unlock migration gate**, but it must be designed as an explicit operator-facing recovery change. The precise first red test should assert that default `Identity::load_or_create(profile)` leaves a persisted identity locked when only a legacy `.identity.key` sidecar exists, while an explicit compatibility flag preserves legacy auto-unlock behavior during migration. No production restart should occur until that isolated test and a recovery walkthrough pass.

```text
STATUS REPORT
─────────────
✅ Done:      Isolated passphrase migration, rotation, lock/reopen validation; EncryptionManager corruption now fails closed; build, formatting, focused, aggregate, and production-safety checks passed.
⚠️  Issue:    Identity auto-unlock still stores a machine-recoverable password sidecar; mailbox and EncryptionManager remain separate passphrase boundaries.
❌ Deferred:  Identity auto-unlock migration gate and mailbox key-lifecycle integration; deferred to avoid silently changing recovery behavior.
📋 Next:     Build the identity auto-unlock red test, then implement an explicit compatibility/migration gate in isolation.

HANDOFF — 2026-08-17
Compile status: clean; 345 library tests passed, 1 ignored
Files modified: `src/core/encryption.rs`, temporary isolated test removed, handoff documentation added
Files needing attention: `src/core/identity/core_identity.rs` for auto-unlock migration; `src/core/mailbox.rs` for passphrase-gated local cache
Resume from: Add and run the identity auto-unlock red reproducer under a dedicated GHOSTWIRE_HOME.
```
