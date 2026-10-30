# Information Theory for GhostWire Encryption

## Purpose

Apply information-theoretic bounds to GhostWire's encryption and
anonymity claims. Entropy, mutual information, and channel capacity
provide formal limits on what an adversary can learn.

## Key Concepts

### Entropy
Shannon entropy measures uncertainty in a random variable:
```
H(X) = -Σ p(x) log₂ p(x)
```
GhostWire's keys must have full entropy (256 bits for AES-256).
Any deviation reduces security.

### Mutual Information
Measures how much knowing X reduces uncertainty about Y:
```
I(X;Y) = H(X) - H(X|Y)
```
For perfect secrecy: I(plaintext; ciphertext) = 0 (Shannon's theorem).
GhostWire's encryption should approach this bound.

### Channel Capacity
Maximum reliable transmission rate over a noisy channel:
```
C = B log₂(1 + SNR)
```
GhostWire's transport selector should not exceed channel capacity
for any given link.

### Kullback-Leibler Divergence
Measures how one probability distribution differs from another:
```
D_KL(P||Q) = Σ p(x) log(p(x)/q(x))
```
Used to measure how much GhostWire's traffic pattern deviates from
"normal" traffic (cover traffic should minimize this).

## Application to GhostWire

### 1. Key Entropy Verification
- All keys must be generated from a CSPRNG with ≥ 256 bits entropy
- Verify with NIST SP 800-90B (entropy assessment)
- No key reuse (forward secrecy via ratcheting)

### 2. Traffic Analysis Resistance
- Cover traffic should make P(traffic pattern) ≈ P(normal traffic)
- Minimize D_KL between GhostWire traffic and background traffic
- Measure and report in mesh health

### 3. Anonymity Set Size
- Anonymity is bounded by the size of the anonymity set
- For k-anonymity: adversary can narrow to k candidates
- GhostWire's Sphinx routing should maintain k ≥ 3 for all messages

### 4. Information Leakage Bounds
- Metadata (sender, recipient, timestamp, size) leaks information
- Sealed sender reduces sender leakage to O(log n) bits
- Padding reduces size leakage to O(1) bits
- Timing analysis: use fixed intervals (ReliabilityMode::Anonymous)

### 5. Channel Capacity for Transport Selection
- Each transport has a maximum throughput (channel capacity)
- Transport selector should not request more than capacity
- BBR-style congestion control (see BBR_CONGESTION_CONTROL_DESIGN.md)
  models this automatically

## Formal Security Claims

| Claim | Bound | Mechanism |
|-------|-------|-----------|
| Key secrecy | 256-bit | AES-256-GCM + X25519 + ML-KEM-768 |
| Perfect secrecy | I(P;C) ≈ 0 | Authenticated encryption + random nonce |
| Forward secrecy | Per-message | Double Ratchet (Signal protocol) |
| Metadata privacy | O(log n) bits | Sealed sender + padding |
| Anonymity | k ≥ 3 | Sphinx routing (3+ hops) |
| Traffic analysis | D_KL < ε | Cover traffic + fixed intervals |

## References

- Shannon 1948: "A Mathematical Theory of Communication"
- Shannon 1949: "Communication Theory of Secrecy Systems"
- Cover & Thomas 2006: "Elements of Information Theory"
- NIST SP 800-90B: "Recommendation for the Entropy Sources"
EOF
echo "created"