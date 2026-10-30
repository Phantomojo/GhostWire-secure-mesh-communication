# Shamir Threshold Secret Sharing — Design for GhostWire

## Purpose

Design (k,n) threshold secret sharing for GhostWire's IFAC seed and genesis
secret. This enables:
- No single point of failure for root secrets
- Successor transfer via threshold re-sharing
- Genesis ceremony rotation without full key replacement
- Information-theoretic security (k-1 shares reveal nothing)

## Mathematical Foundation

### Shamir's (k,n) Threshold Scheme

Given a secret `s ∈ F_p` (prime field), construct a polynomial of degree k-1:

```
f(x) = s + a_1*x + a_2*x^2 + ... + a_{k-1}*x^{k-1}  (mod p)
```

Where `a_1, ..., a_{k-1}` are random coefficients in F_p.

**Shares**: For n participants, compute `s_i = f(i)` for i = 1, ..., n.

**Reconstruction**: Any k shares can reconstruct `s` via Lagrange interpolation:
```
s = Σ_{i=1}^{k} s_i * Π_{j≠i} (0 - j) / (i - j)  (mod p)
```

**Security**: Any k-1 shares reveal nothing about `s` (information-theoretic).

### Parameters for GhostWire

- **Prime**: Use a large prime (e.g., 2^256 - 189, a 256-bit prime) or
  work in GF(2^256) for byte-aligned operations
- **Default (k,n)**: (3,5) — any 3 of 5 shares reconstruct; 2 reveal nothing
- **IFAC seed**: 32 bytes → split into 32 independent Shamir instances
  (one per byte) or use a larger field
- **Genesis secret**: Ed25519 private key → split into 32-byte chunks

## Application to GhostWire

### 1. IFAC Seed Sharing

The IFAC seed (`~/.ghostwire/ifac_seed`) is a 32-byte random value that
determines the node's RNS overlay. If lost, the node's RNS identity changes.

**Design**:
- Generate IFAC seed (32 bytes)
- Split into (3,5) Shamir shares
- Distribute 5 shares to 5 trusted peers/guardians
- Any 3 can reconstruct the seed
- Store shares encrypted at rest (Argon2id + AES-256-GCM)

**Rotation**: When a guardian leaves, re-share with new (3,5) scheme.
Old shares become useless.

### 2. Genesis Secret Sharing

The genesis ceremony author's private key is the root of trust. If compromised,
the entire network's trust model breaks.

**Design**:
- Author generates genesis private key (Ed25519)
- Splits into (3,5) Shamir shares
- Distributes to 5 successors/guardians
- Any 3 can reconstruct for emergency succession
- Author retains 1 share for day-to-day operations

**Succession**: When author transfers control:
1. Author creates new genesis with successor's pubkey
2. Author signs and distributes
3. Successor receives their Shamir share (out of band)
4. Successor can now reconstruct the key with 2 other guardians

### 3. Threshold Re-sharing (Rotation)

When rotating guardians or changing (k,n):
1. Reconstruct secret from k old shares
2. Generate new polynomial with new random coefficients
3. Distribute new shares
4. Old shares become useless (different polynomial)

This is **proactive secret sharing** — periodic rotation prevents
accumulation of compromised shares.

## Implementation Plan

### Phase 1: Shamir Library
1. Create `src/core/shamir.rs` with (k,n) threshold scheme
2. Use 256-bit prime field or GF(2^256)
3. Implement split() and reconstruct()
4. Add zeroization for shares
5. Add tests for (k,n) properties

### Phase 2: IFAC Seed Sharing
1. Add `split_ifac_seed()` and `reconstruct_ifac_seed()` to PersistenceManager
2. Add API endpoints: POST /v2/ifac/share, POST /v2/ifac/reconstruct
3. Add guardian management (list, add, remove)
4. Store shares encrypted at rest

### Phase 3: Genesis Secret Sharing
1. Add Shamir splitting to genesis ceremony
2. Add successor share distribution
3. Add reconstruction for emergency succession
4. Document in GENESIS_CEREMONY_DESIGN.md

### Phase 4: Proactive Re-sharing
1. Add periodic re-sharing (e.g., monthly)
2. Add guardian rotation
3. Add audit log for share operations

## Security Properties

- **Information-theoretic**: k-1 shares reveal nothing about the secret
- **No single point of failure**: any k guardians can reconstruct
- **Forward secrecy**: re-sharing invalidates old shares
- **Auditability**: all share operations logged

## Code Sketch

```rust
pub struct ShamirShare {
    pub index: u8,
    pub value: [u8; 32],
}

pub fn split(secret: &[u8; 32], k: u8, n: u8) -> Result<Vec<ShamirShare>, ShamirError> {
    // For each byte of the secret, create a polynomial of degree k-1
    // Evaluate at points 1..=n
    // Return n shares, each containing one evaluation per byte
}

pub fn reconstruct(shares: &[ShamirShare], k: u8) -> Result<[u8; 32], ShamirError> {
    // Lagrange interpolation for each byte
    // Return reconstructed secret
}
```

## References

- Shamir 1979: "How to Share a Secret"
- Feldman 1987: "A Practical Scheme for Non-interactive Verifiable Secret Sharing"
- Herzberg et al. 1995: "Proactive Secret Sharing"
- ITU-T Y.3808: Quantum key distribution networks
EOF
echo "created"