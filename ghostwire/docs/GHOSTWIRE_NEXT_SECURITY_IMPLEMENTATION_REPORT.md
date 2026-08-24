# GhostWire Next Security Implementation Report

## Executive result

The next security tranche was implemented against Omen’s canonical checkout at `/home/ph/GhostWire-secure-mesh-communication-newest/ghostwire` through the existing Cloudflare SSH tunnel. The dirty checkout was preserved. No reset, clean, commit, push, identity regeneration, website publication, production restart, or production configuration change was performed.

The implementation materially improves active persistence integrity, QUIC profile negotiation, WebRTC fragmentation, and release-key boundaries. It does **not** justify claiming that GhostWire is finished cryptographically. The active at-rest key is still machine-protected rather than user-passphrase-wrapped, only QUIC has the new profile negotiation, WebRTC fragmentation is hardened but not yet wired into every transport, and the release ELF verifier correctly rejects the current Rust binary because stack-protector evidence is absent.

## Changes implemented

| Area | Implementation | Verification |
|---|---|---|
| Active persistence | Added `GWPE` version 1 envelopes with XChaCha20-Poly1305, fresh nonces, profile/tree/key-bound AAD, size limits, strict parsing, legacy rejection, and fail-closed batch loading. | 7 persistence tests passed, including round-trip, version, legacy, tampering, and record-key substitution tests. |
| Encryption helper | Added `EncryptionManager::encrypt_bytes_with_aad` and `decrypt_bytes_with_aad` using the existing machine-protected master key. | Feature compile passed. |
| Crypto profile | Added strict canonical capability decoding and round-trip coverage. | Crypto-profile tests passed. |
| QUIC | Added client/server capability exchange, strongest-common profile selection, minimum rank enforcement, canonical capability transcript binding, selected-profile response verification, and `GWDT` profile-bound data frames. | QUIC tests passed: 2/2. Feature compile passed. |
| WebRTC fragmentation | Replaced sequence-only chunks with authenticated `GWF1` fragments carrying logical length, whole-message SHA-256 digest, HMAC-SHA256 tag, strict bounds, final-flag validation, 30-second reassembly expiry, duplicate consistency checks, and fail-closed errors. | WebRTC tests passed: 5/5. |
| Reticulum quality | Fixed a mutable/immutable borrow conflict in topology pruning by snapshotting valid node IDs before route cleanup. | Aggregate library test compilation and execution passed. |
| Release verification | Corrected updater to read `GHOSTWIRE_RELEASE_VERIFYING_KEY_BYTES`, the exact public variable injected by `build.rs`. Added `scripts/verify_release_binary.sh`. | Release build completed. Verifier passed PIE, RELRO, BIND_NOW, and NX checks, then correctly failed on missing stack-protector symbols. |

## At-rest security assessment

The active sled persistence path now uses a versioned authenticated envelope. The stored representation is no longer an ambiguous raw `[nonce][ciphertext]` blob. Its logical structure is:

```text
GWPE || version || XChaCha20 nonce || ciphertext+Poly1305 tag
```

The AEAD associated data includes a domain label, envelope version, profile name, tree name, and record key. Consequently, copying a valid encrypted value into another record key or tree fails authentication. `load_all_secure` now rejects malformed or unauthenticated entries rather than silently skipping them.

This is an improvement, but it is **not yet passphrase-protected at rest**. The active key source remains GhostWire’s machine-local `master.key` mechanism. That protects against casual database disclosure on the same machine but does not provide portable passphrase unlock, protection against an attacker who obtains the master key file, or a clean profile-lock lifecycle. The next required design is a user-controlled passphrase-wrapped keyslot integrated with the existing identity unlock flow. A hidden derivation from identity material or environment variables would be insecure and was deliberately not introduced.

The existing `save_message_plain` name remains misleading: it stores through `store_secure` and is encrypted by the persistence manager, despite the method and log wording suggesting plaintext. That naming debt should be fixed in the next Rust quality batch.

## QUIC profile assessment

QUIC now performs an explicit profile handshake on the first bidirectional stream. Each side sends bounded canonical capability bytes. The selected profile, negotiated frame size, fragmentation flag, and ordered client/server capability bytes are hashed under a GhostWire-specific transcript domain. The server returns the binding, and the client rejects any mismatch.

Application data frames now carry a magic value, wire version, selected profile ID, and bounded payload length. A peer cannot silently change the profile after negotiation or inject a frame claiming a different selected profile. QUIC currently advertises a frame size that avoids fragmentation; if a future capability configuration requires fragmentation, the handshake rejects it rather than pretending that fragmentation support exists.

The limitation is important: this integrates policy negotiation and profile binding into QUIC, but it does not yet make the selected profile an application-level E2EE implementation. QUIC TLS supplies transport confidentiality, while GhostWire’s separate message encryption paths still need a complete, transport-independent envelope integration.

## WebRTC fragmentation assessment

The old WebRTC fragmenter allocated a vector based on attacker-controlled `total_chunks`, accepted inconsistent totals, overwrote duplicate fragments, trusted the final flag, had no reassembly expiry, and had no authentication. The replacement requires a per-session 32-byte HMAC key. Each fragment authenticates the stream ID, fragment count and index, final flag, logical message length, whole-message digest, and payload.

Reassembly is bounded to 1,024 fragments and 16 MiB, expires incomplete states after 30 seconds, rejects conflicting duplicates, checks exact logical length, validates the digest after reassembly, and rejects wrong-key or tampered fragments. The HMAC is an additional fragment-integrity layer; it is not a substitute for the outer session AEAD.

The remaining integration requirement is to derive and supply the HMAC key from the negotiated session context on every transport that uses this fragmenter. Until that is wired, the hardened type is available but not universal protocol enforcement.

## Release binary assessment

The release profile produced an ELF PIE binary with the following verified properties:

| Property | Result |
|---|---|
| ELF type / PIE | Passed: `ET_DYN` |
| GNU RELRO | Passed |
| Immediate binding | Passed: `BIND_NOW` and `NOW` present |
| Non-executable stack | Passed: `GNU_STACK` is `RW`, not `RWE` |
| Debug sections | Script check available; current run stopped earlier at stack-protector failure |
| Stack-protector evidence | **Failed: no `__stack_chk_fail` or `__stack_chk_guard` symbols** |
| Public release-key boundary | Corrected; updater now consumes the injected public-key bytes variable |

The current installed Rust compiler exposes no `-C stack-protector` code-generation option. Therefore the project cannot honestly mark stack protection as verified merely by setting a Cargo profile flag. The release pipeline needs either a supported compiler/toolchain configuration that emits stack protection, a reviewed native hardening wrapper, or a revised policy that treats Rust’s memory-safety model separately from C ABI stack-canary evidence. Until one of those is established, the verifier should remain fail-closed for release publication.

## Verification record

| Command/check | Result |
|---|---|
| `cargo fmt --all -- --check` | Passed |
| `git diff --check` | Passed |
| `cargo check --features "web cli mesh security stealth qr reticulum"` | Passed |
| Persistence focused tests | Passed: 7/7 |
| Crypto-profile focused tests | Passed |
| QUIC focused tests | Passed: 2/2 |
| WebRTC focused tests | Passed: 5/5 |
| Aggregate library suite | Passed: 336 passed, 0 failed, 1 ignored |
| Release build | Completed successfully with `CARGO_BUILD_JOBS=2` |
| ELF hardening verifier | Correctly failed at stack-protector evidence |
| Production PID | Preserved: `932431` |
| Production TCP ports | Preserved: `9097`, `4001` |
| Production UDP port | Preserved: `8081` |

## Modified files in this tranche

The active implementation files changed in this tranche are `src/core/encryption.rs`, `src/core/persistence_manager.rs`, `src/core/crypto_profile.rs`, `src/core/quic_transport.rs`, `src/core/transport_ext/webrtc_transport.rs`, `src/core/reticulum.rs`, `src/core/updater_fixed.rs`, and `scripts/verify_release_binary.sh`. Timestamped backups were created on Omen before replacing active files. The checkout contains many unrelated pre-existing dirty files and backups; they were not cleaned.

## Residual risks

The largest unresolved risk is that the active persistence key is machine-local rather than passphrase-wrapped. The second is incomplete transport-wide profile enforcement: QUIC is integrated, but libp2p, Stealth TCP, Reticulum, mailbox, and other adapters still need the same handshake/AAD policy. The third is that fallback and broadcast paths need a complete classification matrix so “public,” “encrypted at rest,” “transport encrypted,” and “end-to-end encrypted” cannot be conflated. The fourth is release hardening: stack-protector verification is currently a hard failure. Finally, the aggregate library suite is green, but the full workspace/integration/doctest suite still requires a separate run and the historical Clippy/dependency-audit debt remains.

## Recommended next action

The next implementation should be the passphrase-wrapped persistence keyslot. It should use the existing identity password lifecycle, preserve the machine-key migration path explicitly, reject empty passwords, avoid retaining plaintext passwords in long-lived structs, and include crash-safe migration plus lock/unlock tests. After that, the QUIC profile and AAD policy should be extracted into a shared envelope module and integrated into the remaining transports one at a time.

## References

[1]: https://doc.rust-lang.org/cargo/reference/profiles.html "Cargo profiles"
[2]: https://doc.rust-lang.org/cargo/commands/cargo-test.html "Cargo test"
[3]: https://doc.rust-lang.org/cargo/commands/cargo-check.html "Cargo check"
[4]: https://doc.rust-lang.org/book/ "The Rust Programming Language"
[5]: https://www.rfc-editor.org/rfc/rfc2104 "HMAC: Keyed-Hashing for Message Authentication"
[6]: https://www.rfc-editor.org/rfc/rfc9180 "Hybrid Public Key Encryption"
