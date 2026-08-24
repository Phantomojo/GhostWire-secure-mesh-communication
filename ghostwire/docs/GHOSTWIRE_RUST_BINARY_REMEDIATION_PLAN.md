# GhostWire Rust and Binary Remediation Plan

## Immediate defects

The newest checkout compiles, but `cargo clippy -- -D warnings` fails at project scale. The dominant issues are stale/deprecated constants, unused fields and variables, unnecessary mutability, broad error/string conversions, and mixed module responsibilities. These should be fixed in batches rather than hidden with blanket allows.

The updater build path originally had a naming and secret-boundary defect. It now uses `GHOSTWIRE_RELEASE_VERIFYING_KEY_HEX`, parses it as a public key, and ignores the deprecated signing-key variable. The private release-signing key remains outside the application and must never reach `rustc-env`. A private signing key belongs only in an isolated release signer or CI secret store and must never reach `rustc-env`.

## Rust architecture priorities

The core should expose small typed policy objects instead of passing raw byte vectors and strings across boundaries. Crypto profile IDs, transport plans, peer identities, epochs, sequence numbers, and message IDs should be distinct types where confusion is security-relevant. Fallible parsing should return structured errors with stable categories; logging should add context without including secrets or raw plaintext.

Large managers should be split by responsibility: state machine, persistence adapter, transport codec, policy engine, and observability. Avoid holding synchronous mutex guards across await points. Prefer immutable configuration plus explicit state transitions. Use iterator-based transformations where they improve clarity, but do not trade readable security checks for compressed one-liners.

Unsafe code should be absent from protocol logic unless a narrowly documented boundary is unavoidable. Each unsafe block must have a safety comment and a focused test. Sensitive buffers should use `Zeroize`/`Zeroizing` consistently, but wiping a wrapper does not prove that copies, logs, allocator buffers, or serialized forms are absent.

## Build and supply-chain priorities

Add CI execution for `cargo fmt --all -- --check`, `cargo check --all-features`, focused security tests, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo deny check`, and an advisory scan. Every advisory ignore requires an owner, rationale, affected path, and review date. Add reproducible-build metadata, a release manifest containing artifact hash and verification key ID, and a post-build self-verification step.

Release profiles should keep LTO, one codegen unit, panic abort, and stripping where appropriate, but hardening flags must be selected for the target toolchain and verified from the produced ELF rather than assumed from Cargo settings. Inspect RELRO, PIE, NX, stack protector, fortify settings, symbol visibility, build ID, and debug leakage. Obfuscation such as `obfstr` only hides selected literals; it is not a substitute for secret management, symbol control, anti-tamper, or a threat-modelled binary-hardening strategy.

## References

[1]: https://doc.rust-lang.org/book/ "The Rust Programming Language"
[2]: https://doc.rust-lang.org/reference/unsafe-keyword.html "Rust Reference: Unsafe code"
[3]: https://doc.rust-lang.org/cargo/commands/cargo-clippy.html "Cargo Clippy"
[4]: https://embarkstudios.github.io/cargo-deny/ "cargo-deny"
[5]: https://doc.rust-lang.org/cargo/reference/profiles.html "Cargo Profiles"
