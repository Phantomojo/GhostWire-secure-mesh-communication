# GhostWire Passphrase Persistence Handoff

## Target

The implementation is installed in Omen’s canonical dirty checkout:

`/home/ph/GhostWire-secure-mesh-communication-newest/ghostwire`

The remote endpoint was verified as `ph` on an `OMEN by HP Gaming Laptop 16-k0xxx`. Production PID `932431` and ports `9097`, `4001`, and `8081` were not restarted or modified.

## Installed files

- `src/core/passphrase_keyslot.rs`
- `src/core/persistence_manager.rs`
- `src/core/encryption.rs`
- `src/core/mod.rs`

## Verification

- Red reproducer failed before implementation because the passphrase constructor did not exist.
- Focused persistence module: `9 passed; 0 failed`.
- Aggregate library suite: `344 passed; 0 failed; 1 ignored`.
- `cargo check --lib`: passed.
- `cargo fmt --all -- --check`: passed after formatting the deployed source.
- `git diff --check`: passed.

## Security model

A random 32-byte persistence key is wrapped in a `GWKS` version-1 keyslot using Argon2id and XChaCha20-Poly1305. The keyslot is profile-bound through AAD and written atomically with restrictive permissions. The active record envelopes use the unwrapped key and bind profile, tree, and record key through AAD.

Legacy machine-key stores are migrated only through `migrate_legacy_with_verified_passphrase`, which must be called after identity password verification. Core now starts persistence locked, unlocks/migrates alongside identity unlock, re-locks identity if persistence unlock fails, and rotates the keyslot passphrase during `save_to_disk`.

## Remaining blockers

The running production process uses the previous binary and has not activated this lifecycle. A controlled isolated restart/migration test is required before any production restart. The separate identity and `EncryptionManager` persistence artifacts remain separate audit boundaries and should not be described as covered by this change.

A complete report is in `GHOSTWIRE_PASSPHRASE_PERSISTENCE_IMPLEMENTATION_REPORT.md`.
