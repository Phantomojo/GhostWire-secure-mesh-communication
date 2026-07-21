# GhostWire Threat Model

> **⚠️ LAWFUL USE ONLY**: This document describes the security properties of GhostWire for legitimate, lawful use. Users must comply with all applicable laws.

## Scope

This document covers the security properties of the GhostWire protocol and implementation, what is enforced receiver-side (fork-safe), known shared limitations, and open problems. It is maintained as part of the NLnet grant evidence trail.

## Design Assumptions

- Every peer is untrusted by default
- The network is adversarial (active MITM, Sybil, Eclipse attackers assumed)
- No single node's behavior can compromise the wider network, provided honest nodes enforce validation locally

## Fork Safety

Security must be enforced by honest nodes validating what they receive, not by trusting that every node runs unmodified code. A fork that strips a check weakens only that fork's own node, provided the check is receiver-enforced.

### Receiver-enforced (fork-safe by design)

| Property | Enforcement Point | Status |
|---|---|---|
| MAC/signature verification | Each hop verifies before decryption | Verified — constant-time via `subtle::ConstantTimeEq` |
| Replay protection | Transport-layer `ReplayWindow` (wire_protocol.rs:214-262) | Enforced locally, no sender trust required |
| Low-order point rejection | DH result validation at each hop (sphinx.rs:387-400) | Enforced on receiving side |
| Constant-size cells | 1024+1024 fixed padding | No payload-length leakage |
| Header re-randomization | X25519 scalar multiplication per hop | Bitwise unlinkability |
| Rate limiting / DoS caps | Per-node enforcement | Inherently node-local |

### Not naturally fork-safe (requires explicit design attention)

| Property | Risk | Mitigation |
|---|---|---|
| Sybil/eclipse resistance | Stripped fork spins up cheap fake identities affecting honest peers' routing | Planned: PoW in peer handshake, verified by receiver |
| Routing/GNN poisoning | Coordinated forked nodes feed bad routing signals | Planned: outlier clipping, reputation weighting, exclude unverified peers from training |
| Protocol downgrade | Forked client claims weaker transport, honest node silently falls back | Must hard-fail, not silently downgrade (audit needed) |
| Relay refusal/dropping | No cryptographic check prevents relay from refusing to forward | Mitigated by path diversity/multipath (accepted limitation shared with Tor) |

## Cryptographic Properties

### Sphinx / Onion Routing

- Shared-secret derivation: unique per hop (0..N) via `raw_dh(blinded_ephemeral, hop_pubkey)` + BLAKE3 domain separation
- MAC verification: constant-time, per-hop `mac_key` (not public domain constant)
- Header unlinkability: real X25519 scalar multiplication, not hash-based
- Key zeroization: `HopKeys` implements `ZeroizeOnDrop`
- Known issue: `shared_secrets: Vec<Vec<u8>>` in sphinx.rs not zeroized (heap leak of per-hop secrets)

### Post-quantum (ML-KEM-768)

- Crate: `pqcrypto-mlkem` v0.1.1 (PQClean C reference, FFI wrapper)
- Decapsulation: constant-time with implicit rejection (FIPS 203 compliant)
- Hybrid combiner: dual-PRF HKDF (salt=ECDH, IKM=Kyber, info="GhostWire-PQC-Mix")
- Known issue: silent PQ downgrade in `decrypt_message` — falls back to X25519-only if PQ deserialization fails

### Scalar / RNG hygiene

- All X25519 secrets clamped at every call site
- All crypto-relevant RNG uses CSPRNG (OsRng, ring::SystemRandom, getrandom)
- No nonce reuse across encryption calls
- HKDF info strings domain-separated per purpose (15 distinct genesis constants)

## Known Limitations (shared with other onion-routing systems)

1. **Relay refusal**: No cryptographic check prevents a relay from dropping or reordering packets. Mitigated by multipath routing.
2. **Traffic analysis**: No cover traffic or fixed-rate sending currently. Timing can leak path information.
3. **Sybil cost**: Currently near-zero. PoW/stake requirement is planned but not yet implemented.

## Open Work

- Sybil resistance via peer handshake PoW (receiver-verified)
- GNN robustness against adversarial training data
- Cover traffic for traffic analysis resistance
- `shared_secrets` zeroization in sphinx.rs
- PQ downgrade prevention in `decrypt_message`

## References

- Crypto audit report: `/tmp/ghostwire_wave2_crypto_audit.md`
- Kit playbook: `/tmp/kit/docs/00_AUDIT_AND_HYGIENE_PARTS_A-C.md` (Part C)
