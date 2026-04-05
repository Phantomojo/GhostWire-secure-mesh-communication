---
name: ghostwire-security-audit
description: >
  Use when reviewing GhostWire code for security vulnerabilities, cryptographic
  correctness, P2P privacy properties, or anonymity guarantees. Triggers on:
  "security review", "audit", "crypto", "anonymity", "privacy", "onion routing",
  "Sphinx", "Kademlia", "post-quantum", "unsafe block", "CVE", "cargo audit".
  Also triggers when adding any new network-facing code.
---

# GhostWire Security Audit Skill

## Core Security Properties to Preserve

Every change must not degrade these guarantees:

1. **Traffic analysis resistance** — packet sizes must be padded to fixed sizes
2. **Forward secrecy** — ephemeral keys only, no long-term session keys in data path
3. **Plausible deniability** — nodes cannot prove they didn't relay a given packet
4. **Metadata minimisation** — no unnecessary identifiers in wire format
5. **Post-quantum readiness** — CRYSTALS-Kyber for KEM, Dilithium for signatures

## Audit Commands

```bash
# Check for known CVEs in dependencies
cargo audit

# Check for unsafe blocks (must all be reviewed)
grep -rn "unsafe" src/ --include="*.rs"

# Check for unwrap() in non-test code (should be zero)
grep -rn "\.unwrap()" src/ --include="*.rs" | grep -v "#\[cfg(test)\]" | grep -v "tests/"

# Check for hardcoded addresses or keys (should be zero)
grep -rn "localhost\|127\.0\.0\|0\.0\.0\.0\|hardcode\|TODO.*key\|TODO.*secret" src/

# Check for println! (must use tracing instead)
grep -rn "println!\|eprintln!" src/ --include="*.rs"

# Dependency tree for supply chain review
cargo tree --edges features | head -80
```

## Cryptographic Review Checklist

Before approving any crypto-adjacent change:
- [ ] No custom crypto implementations — use audited crates only
- [ ] RNG: `rand::rngs::OsRng` only — never `rand::thread_rng()` in crypto paths
- [ ] Key material: zeroed on drop — use `zeroize` crate
- [ ] Comparison: constant-time — use `subtle` crate for key/MAC comparison
- [ ] No key material in log output — search for tracing::debug with key variables
- [ ] No key material in error messages — anyhow errors get propagated upward

## Crates That Are Pre-Approved (do not need extra review)

- `ring` — Google's crypto primitives
- `x25519-dalek` — X25519 ECDH
- `ed25519-dalek` — Ed25519 signatures  
- `chacha20poly1305` — AEAD encryption
- `pqcrypto-kyber` — CRYSTALS-Kyber post-quantum KEM
- `zeroize` — secure memory zeroing
- `subtle` — constant-time comparisons

## Crates That Need Review Before Adding

- Any new networking crate — check for DNS leaks, plain-text fallbacks
- Any serialisation crate — check for memory safety with untrusted input
- Any FFI crate — review unsafe surface area carefully

## Unsafe Block Protocol

If you must add an unsafe block:
```rust
// SAFETY: [explain precisely why this is safe]
// Reviewed by: [who reviewed it]
// Date: [date]
// Alternative considered: [why safe Rust couldn't be used]
unsafe {
    // ...
}
```

No unsafe block without all four fields in the comment.
