# GhostWire Scopes A–C Implementation Report

## Scope and safety boundary

The work was performed against Omen’s newest canonical checkout at `/home/ph/GhostWire-secure-mesh-communication-newest/ghostwire`. The checkout was already dirty; no reset, clean, identity regeneration, production restart, commit, push, or website publication was performed. Production PID `932431` remained active, with TCP `9097`, TCP `4001`, and UDP `8081` verified after the changes.

## Implemented changes

### 1. Truth and capability documentation

Added `docs/GHOSTWIRE_TRUTH_LEDGER.md`, `docs/GHOSTWIRE_CRYPTOGRAPHIC_SECURITY_MODEL.md`, and `docs/GHOSTWIRE_RUST_BINARY_REMEDIATION_PLAN.md`. Updated active installation, configuration, and deployment documentation to use `GHOSTWIRE_RELEASE_VERIFYING_KEY_HEX` rather than the ambiguous signing-key name.

The ledger explicitly downgrades unsupported claims: post-quantum protection remains a roadmap item; “nine live transports” requires a tested transport matrix; “no logs/no traces,” censorship resistance, coercion resistance, and universal AI routing are not unconditional verified guarantees.

### 2. Transport-aware crypto-profile scaffolding

Added `src/core/crypto_profile.rs` and exported it from `src/core/mod.rs`. It provides a bounded `CryptoCapabilities` structure, canonical capability bytes for handshake transcript binding, a typed `CryptoProfileId`, a conservative `GW0-SESSION-1` profile, deterministic strongest-common selection, minimum security rank enforcement, and explicit errors for protocol mismatch, no common profile, downgrade rejection, and insufficient fragmentation support.

The module is deliberately scaffolding, not a claim that every transport now negotiates the profile. Transport integrations still need to bind capability bytes and selected profile IDs into their authenticated handshake transcripts and envelope AAD before the profile can be called operational.

### 3. Release verification-key boundary

Changed `build.rs` and `src/core/updater_fixed.rs` so the application consumes a public `GHOSTWIRE_RELEASE_VERIFYING_KEY_HEX` value. The old `GHOSTWIRE_RELEASE_SIGNING_KEY_HEX` variable is deprecated and ignored. The private release-signing key must remain in an isolated release signer and never enter `rustc-env` or the application binary.

The current build still has no configured public verification key or binary hash, so automatic update and binary-integrity verification remain disabled in the current development environment. That is now an explicit and truthful state instead of an ambiguous “signing key” contract.

## Cryptographic audit findings

The active code contains several separate crypto domains: vodozemac session state, sealed mailbox encryption, encrypted storage abstractions, identity persistence, transport framing, Sphinx, and experimental ratchet/adapter code. They are not yet governed by one authenticated profile policy. In particular, transport selection currently exists separately from crypto selection, so an encryption switch must be wired through a policy layer rather than allowing each transport to choose an algorithm ad hoc.

The sealed fallback is authenticated by AEAD but is not equivalent to a ratcheted session. It requires explicit recipient identity, message ID, expiry, direction, profile, replay, and fragmentation binding in associated data. The current audit found that these guarantees are not uniformly evidenced across every fallback and adapter.

The vodozemac-oriented `crypto_machine.rs` file is not reachable from the active `src/core/mod.rs` module graph, while `CryptoProcessor` persists `GhostCryptoMachine` state with bincode. A passphrase-aware encrypted pickle prototype was intentionally removed after discovering it was dead/unwired code. The active persistence boundary still needs a proper versioned authenticated envelope and a real passphrase/unlock lifecycle before at-rest encryption can be claimed.

## Binary and Rust audit findings

The release profile already uses LTO, one codegen unit, panic abort, and stripping. Those are useful but do not prove ELF hardening. The project lacks a verified release pipeline for PIE, RELRO, NX, stack protection, symbol/debug leakage, reproducibility, and artifact self-verification. `obfstr` hides selected literals only; it is not secret protection or a substitute for threat-modelled binary hardening.

`cargo-deny` policy exists, but `cargo-deny`, `cargo-audit`, and `cargo-vet` were not installed on Omen during the audit. The advisory ignore list is large and requires current advisory review, ownership, and expiry dates. Clippy remains far from warning-clean, with stale constants, unused fields, unnecessary mutability, and other design debt.

The correct Rust refactor direction is typed policy objects, bounded parsers, structured errors, explicit state transitions, no secrets in logs, no mutex guards across await points, and small modules with one responsibility. Blanket `allow` attributes or aggressive one-line rewrites would conceal rather than fix the current quality debt.

## Verification results

| Check | Result |
|---|---|
| `cargo fmt --all -- --check` | Passed |
| `git diff --check` | Passed |
| Crypto-profile unit tests | Passed: 4/4 |
| Feature compile (`web cli mesh security stealth qr reticulum`) | Passed |
| Aggregate existing suite | Not rerun as green; prior audit recorded node-to-node and doctest failures |
| `cargo clippy -- -D warnings` | Not green; prior audit recorded approximately 415 errors |
| Production PID/ports | Preserved and verified |

The first focused test attempt matched zero tests because the module insertion anchor was wrong. That was diagnosed, corrected, and the actual four crypto-profile tests were then executed successfully. This is recorded to avoid presenting a false green result.

## Rollback points

Timestamped backups exist on Omen for `src/core/mod.rs`, `src/core/crypto_profile.rs`, `build.rs`, `src/core/updater_fixed.rs`, and the modified active documentation files. The repository’s pre-existing dirty files and unrelated backups were not reset or cleaned.

## Residual risks and required next work

The transport profile must be integrated into the handshake and every transport codec before it provides real downgrade resistance. The active `CryptoProcessor` persistence path needs a user-controlled passphrase lifecycle and authenticated migration tests. Sealed mailbox associated-data binding and replay/expiry enforcement need a transport-by-transport matrix. The release process needs a real public verification key, manifest test vectors, binary hash generation, and ELF hardening verification. Dependency audits need to run in CI, and clippy failures need to be fixed by subsystem.

The public website has not been changed. A website update should be prepared only after the truth ledger, transport test matrix, cryptographic integration, and release-status wording are accepted. The website must stop presenting research modules, feature flags, and placeholders as universally deployed capabilities.

## References

[1]: https://datatracker.ietf.org/doc/rfc9180/ "RFC 9180: Hybrid Public Key Encryption"
[2]: https://noiseprotocol.org/noise.html "The Noise Protocol Framework"
[3]: https://doc.rust-lang.org/book/ "The Rust Programming Language"
[4]: https://doc.rust-lang.org/cargo/commands/cargo-clippy.html "Cargo Clippy"
[5]: https://embarkstudios.github.io/cargo-deny/ "cargo-deny"
[6]: https://doc.rust-lang.org/cargo/reference/profiles.html "Cargo Profiles"
