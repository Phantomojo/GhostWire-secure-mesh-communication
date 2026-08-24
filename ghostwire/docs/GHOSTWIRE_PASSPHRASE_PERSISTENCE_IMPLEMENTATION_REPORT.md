# GhostWire User-Passphrase Persistence Protection

**Author:** Manus AI
**Target:** Omen, `/home/ph/GhostWire-secure-mesh-communication-newest/ghostwire`
**Date:** 2026-08-17
**Scope:** Active persistence protection only. Production process `932431` and production ports were not restarted or modified.

## Executive assessment

The active GhostWire persistence path now supports a user-passphrase-wrapped data-encryption key. The passphrase does not directly encrypt every sled record. Instead, GhostWire generates a random 256-bit persistence data-encryption key, wraps it in a versioned authenticated keyslot using an Argon2id-derived key and XChaCha20-Poly1305, and then uses the unwrapped data key for the existing authenticated persistence envelopes.

This is the correct key hierarchy for the requested feature. It avoids repeatedly deriving a key for every record, permits passphrase rotation without re-encrypting the database, keeps the passphrase out of long-lived state, and binds the keyslot to the active profile through authenticated associated data.

> **Important trust boundary:** Persistence cannot independently determine whether a passphrase is the GhostWire identity password. Legacy migration is therefore exposed through `migrate_legacy_with_verified_passphrase`, which must be called only after the identity layer has successfully verified the password. The low-level persistence layer deliberately refuses to treat an arbitrary passphrase as proof of identity.

## Implemented design

| Component | Implementation | Security purpose |
|---|---|---|
| Keyslot magic | `GWKS` | Prevents accidental interpretation of unrelated files as keyslots |
| Keyslot version | Version `1` | Enables strict future format migration |
| Salt | Random 16-byte per-keyslot salt | Prevents precomputed passphrase-key reuse |
| Passphrase KDF | Argon2id, 64 MiB, 3 iterations, one lane | Makes offline passphrase guessing materially more expensive |
| Wrapped data key | Random 32-byte persistence key | Separates passphrase-derived key from record encryption key |
| Key wrapping AEAD | XChaCha20-Poly1305 with a fresh 24-byte nonce | Provides confidentiality and integrity for the keyslot |
| Keyslot AAD | Domain, version, and profile name | Prevents cross-profile and cross-format keyslot substitution |
| Record AEAD | Existing XChaCha20-Poly1305 persistence envelope using the unwrapped key | Protects individual records with fresh nonces |
| Record AAD | Domain, envelope version, profile, tree name, and record key | Prevents record relocation and profile/tree/key substitution |
| Key memory | `zeroize::Zeroizing` wrappers | Clears key material when the owning value is dropped |
| Keyslot write | Temporary file, `fsync`, rename, mode `0600` | Reduces partial-write and permission exposure risk |

The keyslot filename is profile-scoped as `ghost_<profile>.persistence.keyslot`; the default profile uses `ghost_default.persistence.keyslot`. Profile names should still be constrained at a higher configuration boundary because filesystem-derived profile components must never accept path traversal input.

## Lifecycle behavior

A new Core instance now constructs `PersistenceManager::new_locked(...)`. A database marked as user mode opens locked and refuses private record reads, writes, key listing, batch loads, and tree clearing until the profile is unlocked. `Core::unlock(password)` first verifies the identity password, then either unwraps the existing persistence keyslot or performs the verified legacy migration. If persistence unlock fails, the identity is re-locked to avoid an inconsistent partially-unlocked state.

`Core::lock()` clears the active persistence key from the manager before locking the identity. `Core::save_to_disk(password)` rotates the persistence keyslot passphrase when the persistence key is already unlocked. If the store is still legacy, it performs the identity-authenticated migration before saving the identity.

Legacy machine-key databases remain available through the compatibility constructor for tests and controlled migration. They are not silently accepted as user-mode databases. Migration decrypts the old authenticated envelopes using a stable compatibility-key snapshot, re-encrypts every record under a new random persistence key, writes a user-mode marker into a staged sled database, atomically swaps the database directory, and removes the old legacy directory after success. A timestamped source backup is retained under the repository’s existing `.manus-backups/` area before source installation; the runtime migration itself does not silently overwrite the old database until the staged replacement is ready.

## Red/green verification

A red reproducer was installed against the active Omen checkout before implementation. The exact selector failed at compile time because `new_with_base_dir_and_passphrase` did not exist:

```text
red_status=101
error[E0599]: no associated function or constant named `new_with_base_dir_and_passphrase`
```

After implementation, the focused test passed:

```text
running 1 test
test core::persistence_manager::tests::test_user_passphrase_wraps_active_persistence_key ... ok
test result: ok. 1 passed; 0 failed
```

The complete persistence-manager module then passed:

```text
running 9 tests
test result: ok. 9 passed; 0 failed; 0 ignored
```

The aggregate Omen library suite passed after fixing a real migration race exposed by concurrent tests. The final result was:

```text
cargo check --lib                 passed
cargo fmt --all -- --check        passed
 git diff --check                  passed
cargo test --lib                  344 passed; 0 failed; 1 ignored
```

The aggregate suite initially found one failure in the legacy migration test. The root cause was not cryptographic weakness in the new keyslot; the compatibility constructor reread the global machine-key path while other tests temporarily changed `GHOSTWIRE_HOME`. The implementation was corrected to snapshot the legacy machine key once into zeroizing memory. The migration test and aggregate suite then passed consistently.

The keyslot-specific tests cover correct round-trip, wrong-passphrase rejection, profile binding, ciphertext tampering, and weak-passphrase rejection. Persistence tests cover locked access, explicit migration, keyslot reopen, legacy-format rejection, AAD record-key binding, envelope tampering, list/load-all behavior, and normal round-trips.

## Modified active files

| File | Change |
|---|---|
| `src/core/passphrase_keyslot.rs` | New `GWKS` keyslot format, Argon2id derivation, XChaCha20-Poly1305 wrapping, profile-bound AAD, atomic `0600` writes, strict parsing, and zeroized key handling |
| `src/core/persistence_manager.rs` | Locked/user/machine key-source state, passphrase unlock, explicit verified legacy migration, keyslot rotation, stable legacy-key snapshot, user-mode marker, and key-bound record AEAD |
| `src/core/encryption.rs` | Explicit caller-key AEAD wrappers and a narrow machine-key snapshot API; existing machine-key APIs remain intact |
| `src/core/mod.rs` | Registers the keyslot module, starts persistence through the locked constructor, and wires Core lock/unlock/save lifecycle behavior |

The synchronized exact formatted source files are included in the attached handoff archive.

## Production safety check

At final verification time:

| Check | Result |
|---|---|
| Remote identity | `whoami=ph`; canonical checkout present |
| Hardware identity | `OMEN by HP Gaming Laptop 16-k0xxx` |
| Production PID | `932431 ghostwire Sl` still running |
| API port | `127.0.0.1:9097` still owned by PID `932431` |
| Mesh TCP port | `0.0.0.0:4001` still owned by PID `932431` |
| QUIC UDP port | `0.0.0.0:8081` still owned by PID `932431` |
| Production restart | Not performed |
| Production data migration | Not performed |

The new code was compiled and tested in isolation against the checkout. It has **not** been activated inside the running production process. This is intentional: activating the new persistence lifecycle requires a controlled GhostWire restart and a user-password migration decision.

## Residual risks and required next actions

The largest operational risk is that the running production binary predates this implementation. The new behavior will begin only when GhostWire is deliberately restarted on the new build. Before doing so, the operator must ensure the identity password is available and that a tested backup of the relevant `GHOSTWIRE_HOME` data exists.

The system still contains a separate `EncryptionManager` machine-key persistence lifecycle. This implementation does not retroactively convert every encryption-manager artifact into the user-passphrase key hierarchy. The active `PersistenceManager` record store is protected by the user-wrapped key after migration; identity and encryption-manager storage remain separate boundaries and need their own explicit lifecycle audit.

Passphrase strength is still an operational dependency. Argon2id raises offline attack cost but cannot rescue a weak or reused password. The current minimum is 12 bytes, not a guarantee of high entropy. A later UX layer should provide a password-strength estimate without logging or persisting the passphrase.

Crash recovery around directory replacement is substantially safer than direct overwrite, but a power loss between keyslot creation and database swap can leave a recoverable migration artifact. The next hardening pass should add a durable migration journal with explicit states, startup recovery, and a user-visible recovery command. The current implementation preserves the old database until the replacement is staged, but it does not yet implement a complete journal protocol.

The next controlled action should be a **non-production isolated migration test using a dedicated `GHOSTWIRE_HOME` and ports**, followed by a planned production restart only after the user explicitly authorizes it.

## Rollback points

Before source installation, the following timestamped backup directory was created on Omen:

```text
/home/ph/GhostWire-secure-mesh-communication-newest/ghostwire/.manus-backups/passphrase-20260817T101132Z/
```

It contains the pre-install versions of `src/core/mod.rs`, `src/core/encryption.rs`, `src/core/persistence_manager.rs`, and the prior keyslot file if one existed. The checkout’s pre-existing dirty state was not cleaned or reset.

## Final assessment

This request is implemented at the active persistence boundary and is no longer merely a machine-key envelope claim. The new path has a real user-passphrase keyslot, profile-bound authenticated wrapping, zeroized in-memory data keys, explicit lock state, verified identity-gated migration, passphrase rotation, and passing focused plus aggregate tests.

It is still not safe to claim that **every** GhostWire persistent artifact is passphrase-protected. The accurate claim is narrower: **the active `PersistenceManager` record store now supports user-passphrase-wrapped persistence keys; legacy data requires explicit identity-authenticated migration; separate identity/encryption-manager stores remain separate audit targets.**
