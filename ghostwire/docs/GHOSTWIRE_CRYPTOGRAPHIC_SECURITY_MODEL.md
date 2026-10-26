# GhostWire Cryptographic Security Model

## Scope

This document records the cryptographic boundaries that GhostWire must enforce. Names such as “encrypted,” “secure,” or “post-quantum” are not sufficient evidence; each claim must identify the primitive, key origin, nonce domain, authenticated metadata, replay rule, and lifecycle.

## Session path

Interactive peer sessions use the vodozemac-backed account/session path. Its identity, one-time prekeys, and ratcheted messages are distinct from the mailbox fallback. The session transcript must bind peer identity, device identity, negotiated profile, protocol version, and direction. A session packet must not be accepted merely because it decrypts; the sender and session context must also be valid.

## Sealed mailbox path

The sealed fallback uses an ephemeral X25519-style exchange with an AEAD payload. It is not equivalent to a Double Ratchet session. It must bind recipient identity, sender or envelope identity where available, message ID, expiry, profile ID, and direction through associated data. It needs an explicit replay/expiry policy and must never silently downgrade an established session into a weaker fallback.

## At-rest account state

The newest checkout contains a vodozemac-oriented `crypto_machine.rs` prototype with plaintext pickle methods, but that file is not reachable from the active `src/core/mod.rs` module graph. The active `CryptoProcessor` persistence path currently serializes `GhostCryptoMachine` with bincode and does not derive an encryption key from its unused passphrase concept. Therefore at-rest protection is **not verified as active**. The required fix is to integrate a versioned passphrase-protected envelope at the active persistence boundary, with an Argon2-derived key, ChaCha20-Poly1305 authentication, random salt and nonce, explicit associated data, empty-passphrase rejection, migration gating, and wrong-passphrase tests. Until that integration is complete, documentation must not claim encrypted account persistence.

## Transport-aware protection

Transport selection and cryptographic selection are separate. A transport may constrain frame size, ordering, fragmentation, retry, or store-and-forward behavior, but it must not choose a weaker key exchange, authentication scheme, KDF, or AEAD merely because it is convenient. The negotiated profile is authenticated in the handshake transcript and bound into application-envelope associated data.

The first profile is `GW0-SESSION-1`, with classical X25519 plus authenticated Ed25519 identity binding, a domain-separated KDF, an audited ChaCha20-family AEAD, counter-based nonce discipline, and explicit replay windows. ML-KEM-768 (pqcrypto-mlkem) IS now integrated in encryption.rs with HKDF-SHA256 mixing and downgrade prevention (pq_key included in signature). The hybrid (X25519 + ML-KEM-768) path is active in encrypt/decrypt. What remains: formal audit, cross-implementation interoperability testing, and PQ profile selection for at-rest state.

## Binary and release integrity

Release verification requires a public Ed25519 verification key and a separately produced binary hash. A private signing key must never be injected into or compiled into the application. The build variable must be named and documented as a public verification key. Release builds without configured verification key and binary hash must be marked development/unverified, not presented as integrity-protected releases.

## Security invariants

1. No key or passphrase is logged, serialized in plaintext, or reused across domains without an explicit derivation context.
2. Every AEAD nonce is unique for its key and direction.
3. Every ciphertext authenticates the metadata required to prevent identity, profile, direction, replay, and fragmentation confusion.
4. Fallbacks are explicit and observable in policy state; they are never silent downgrades.
5. Unknown mandatory profile IDs and malformed capability messages are rejected with bounded parsing.
6. Legacy formats require an explicit compatibility gate and are rewrapped before forwarding.

## References

[1]: https://datatracker.ietf.org/doc/rfc9180/ "RFC 9180: Hybrid Public Key Encryption"
[2]: https://noiseprotocol.org/noise.html "The Noise Protocol Framework"
[3]: https://doc.rust-lang.org/book/ "The Rust Programming Language"
