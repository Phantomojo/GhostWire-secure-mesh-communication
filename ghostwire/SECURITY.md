# Security Policy

## Read this first

GhostWire is **pre-1.0 and has had no external security review.** Treat every
security claim in this repository as a design intent, not a verified property.

`spec/README.md` lists what is **not** promised as prominently as what is. Read it
before relying on anything here.

There is **no issue tracker enabled** on this repository. That gap is the entire
Reticulum lesson, and it is worth stating plainly:

> Reticulum's author stepped down. Its repository has `has_issues: false`. A
> critical remote-shell flaw was fixed in 1.3.9 alongside a promised
> post-mortem that will never arrive, because there was no channel through which
> anyone could report the *next* one. **A protocol with no errata channel is one
> where you can never learn that you are insecure.**

Enabling an issue tracker and publishing a reachable reporting address are the two
changes that convert this project from a prototype into something a user can
depend on. Until then, contact the maintainer directly.

## Supported versions

| Version | Supported |
|---|---|
| `rescue/sep-disk-tree` (current) | yes |
| Anything older | no — no deployed network exists |

No released version. Breaking changes to normative encodings cost nothing today
and are made deliberately (`spec/README.md` §7).

## Reporting a vulnerability

Contact the maintainer directly. **Do not** rely on an issue tracker existing —
assume it does not.

Please include: description, steps to reproduce, impact assessment, OS and
GhostWire version, and a proof of concept if you have one.

Do not test against nodes you are not authorised to test. Do not publicly
disclose before a fix exists.

## What GhostWire protects

- **Content confidentiality and integrity** between paired peers.
- **Unlinkability of addresses** until pairing completes, *within* a transport.
- **Authorisation**: a message from a DID requires a pairing record that verifies
  against it. Possession of a transport address is not authorisation.

## What GhostWire does NOT protect

The properties a user is most likely to assume, and is wrong about.

### Your IP address

Every libp2p connection exposes both endpoints' IP addresses to each other, to
their network operators, and to anyone on the path. `DCUtR` actively works
against address hiding **by design**.

Your network operator can see that you participate and roughly when. Message
metadata is not protected; padding is per-message, not per-flow, so an idle
connection is itself a signal.

### Communication patterns

No anonymity against a global passive observer. Content is unlinkable,
participation is pseudonymous, traffic patterns are neither.

### A compromised key

A `Revocation` is signed by the key being revoked and never expires — a key
compromised last year is still compromised. But a self-signed revocation **cannot
stop an attacker who already holds the key** from signing their own statements.
Revocation buys you this: honest holders of a stale address learn the truth
without trusting a third party. It does not buy key-theft recovery.

### Reticulum path selection

Path selection is **nearest-wins by construction**, using a timestamp the
announcing node itself asserts. That is the textbook DTN blackhole shape. Do not
assume a Reticulum path is an independent witness.

## Known open defects

Security-relevant and user-visible. Stated here rather than only in the spec.

### 1. PeerId → DID is invertible — UNRESOLVED

libp2p uses an **identity multihash** for keys of 42 bytes or fewer, so the
SHA-256 branch is unreachable and every `12D3KooW…` PeerId **literally contains
the public key**.

> **Anyone holding a PeerId can compute the corresponding DID.**

Verified by decoding a PeerId from libp2p's own specification text and recovering
the key.

This makes the PeerId a **permanent cross-transport correlator**, in direct
tension with "one identity, many transports, addresses unlinkable until
pairing". The `identify` protocol compounds it by pushing the public key on every
connection, so one libp2p operator sees every GhostWire node's DID with no pairing
involved.

Options, none taken unilaterally:

1. Disable `identify` and replace its address-discovery role. Costs discovery,
   preserves the property.
2. Adopt a non-identity multihash. Blocked in practice: `identify` rejects any
   peer whose key does not derive their claimed ID.
3. Scope it out and state plainly that libp2p peer identity is pseudonymous but
   **correlatable**.

The specification's requirement stands: *an implementation MUST NOT claim
cross-transport unlinkability while a standards-compliant libp2p PeerId is in
use.*

### 2. Typed pairing was broken until `58fa4a5` — FIXED

`encode` used `ALPHABET[(byte as usize) % 32]`, one character per byte, keeping
only the low 5 bits. Not injective, so no decoder could exist. `from_string`
therefore hashed the string with blake3 instead of decoding, and the two sides of
a typed pairing derived **different session keys**.

Every typed pairing produced a safety-number mismatch — `579841` versus `677922`.
Pairing at any distance did not work. Fixed with real base32 and an exact inverse
decoder, and pinned by three conformance vectors.

**If you paired against a build before `58fa4a5`, that pairing is untrustworthy.
Re-pair.**

### 3. Discovery signed a Rust struct layout — FIXED in `0c5c877`

`DiscoveryAnnounce` and `AddressExchange` signed `bincode::serialize(...)`, so
signed bytes depended on field order and widths. No second implementation could
have reproduced them. Fixed with a length-prefixed, domain-separated canonical
form, pinned by `spec/vectors.json`.

### 4. P1 — `pqcrypto-mlkem` 0.1.1 depends on archived PQClean

**Advisory**: RUSTSEC-2026-0161 · **Status**: Open — requires migration

`pqcrypto-mlkem` 0.1.1 depends on PQClean, archived July 2026. It will not
receive security patches.

**Remediation**: migrate to the maintained [`ml-kem`](https://crates.io/crates/ml-kem)
crate (ML-KEM-768 compatible API). Confined to `encryption.rs`; `pqcrypto-traits`
can then be removed.

**Risk if deferred**: no immediate CVE, but future PQClean-targeting attacks go
unpatched. Acceptable in development; must be fixed before production.

### 5. P2 — no CI dependency audit

`cargo-audit` is not installed and no CI pipeline runs automated dependency
vulnerability checks. Regressions are possible on `cargo update`.

**Remediation**: add `cargo audit` to CI.

**Risk if deferred**: new dependency CVEs are caught only by sporadic manual
audit.

## Cryptographic posture

| Concern | Status |
|---|---|
| Identity keys | ed25519, verified at pairing |
| Signed structures | ed25519 over a canonical, length-prefixed encoding |
| Transport | libp2p Noise (default); QUIC available |
| Key agreement | ML-KEM-768 hybrid where negotiated |
| Revocation | self-signed, never expires, **unverified revocations ignored** |

**No primitives are home-rolled.** Each is a vetted library used as documented.
The one custom construction is the *canonical signing form*
(`spec/README.md` §3), and its properties — field-boundary unambiguity and domain
separation — are pinned by vectors rather than asserted.

`is_revoked` ignores revocations that do not verify. Obeying them would let anyone
revoke anyone by injecting a garbage record — a denial of service on the whole
network.

## Resolved findings (2026-07-03 audit)

- **CVE-2024-58262** — curve25519-dalek timing attack; pinned `>=4.1.3`
- **CVE-2023-42811** — aes-gcm plaintext exposure; pinned `>=0.10.3`
- **CVE-2025-0805** — reqwest HTTP request smuggling; pinned `>=0.11.23`
- **CVE-2025-62514** — X25519 small-order point attack; all-zero DH output check
  added in `encryption.rs:703`

## Auditing the claims in this file

Two commands, both cheap:

```
cargo test --lib --no-fail-fast          # 494 unit tests
cargo test --test conformance_vectors    # 9 vectors
```

The vector executor is deliberately sensitive: changing a normative encoding
fails with *"this is a PROTOCOL change, not just a refactor"*. That is the point.
A protocol whose encodings can drift silently is an application wearing a
protocol's name.

## Cryptographic bugs

For a bug that breaks confidentiality, authenticity, or the canonical encoding,
contact the maintainer directly and **do not** rely on an issue tracker existing.