# GhostWire Protocol Specification

**Status:** DRAFT — not yet a standard.
**Spec version:** 1 (unreleased)
**Vectors:** [`vectors.json`](./vectors.json) · executor: `tests/conformance_vectors.rs`

---

## Read this first: what this document is, and is not

This is a **draft** specification for the parts of GhostWire that are genuinely
normative — the encodings a second implementation must reproduce byte for byte.

**It is not yet a protocol.** One implementation exists, and interoperability
across independent implementations is the defining property of a protocol. This
document is the work toward that, not the achievement.

It deliberately covers **less** than the codebase. Anything still
implementation-defined is left out rather than frozen, because a vector set that
enshrines a defect is worse than no vector set: the defect becomes normative by
reference.

### What is and is not covered

| Covered (normative) | Not covered — and why |
|---|---|
| DID derivation from an ed25519 key | libp2p PeerId derivation — that is **libp2p's** spec, not ours |
| Canonical signing form for all four signed structures | Wire framing that is still implementation-defined |
| The base32 pairing code and its round trip | Reticulum internals — a separate specification |
| Properties that must hold across implementations | Anything about deployment, UI, or policy |

### What is deliberately NOT promised

These are honest non-goals, and they are as much a part of the specification as
what is promised:

- **No anonymity against a global passive observer.** GhostWire provides
  unlinkable *content* and pseudonymous *participation*. It does not provide
  unlinkable *communication patterns*. Message padding is per-message, not
  per-flow, so an idle connection is itself a signal.
- **No transport anonymity.** Every libp2p connection exposes both endpoints'
  IP addresses to each other, to their network operators, and to anyone on the
  path. DCUtR actively works against address hiding by design.
- **No resistance to a sinkhole in Reticulum.** Path selection is
  nearest-wins by construction, using a timestamp the announcing node itself
  asserts. That is the textbook DTN blackhole shape.
- **No claim that the DID cannot be linked to the PeerId.** It can. See
  [§5](#5-the-correlation-problem-stated-in-full).

A specification that lists only its strengths is marketing. These are the
properties a user must not assume, and they are the ones most likely to be
assumed by accident.

---

## 1. Conventions

The key words **MUST**, **MUST NOT**, **SHOULD**, **SHOULD NOT** and **MAY** are
to be interpreted as in RFC 2119.

All integers are **unsigned, big-endian, fixed width** unless stated otherwise.
All byte strings are raw; no base64 on the wire. Hex is used only in test
vectors.

### Canonical encoding

Every structure that is signed uses one canonical byte form, defined in §3. It is
**not** any language's serialisation of a struct.

The reason is specific and was learned the hard way: `DiscoveryAnnounce` and
`AddressExchange` originally signed `bincode::serialize(&unsigned)`. bincode
encodes a *Rust struct*, so the signed bytes depended on field order, field widths,
and the serde derive's own choices. A second implementation — in another language,
or the same one after a field was added — computed different bytes and failed
verification, with no way to distinguish a reordering from an attack.

That is the defect that stops Reticulum-class protocols from being
specifications. It is why four-plus third parties have written competing
byte-level specs against different version targets and none of them interoperate.

---

## 2. Identity

### 2.1 DID

```
did:ghost:<64 lowercase hex characters>
```

where the hex is `SHA-256` of the ed25519 public key's **protobuf encoding**.

Verifiable by anyone. **Not invertible to a PeerId** — SHA-256 is one-way. A DID
is a verification anchor, never something you can compute a transport address
from.

### 2.2 Verification, not inversion

```
peer_id_matches_did(peer_id, did) -> bool
```

Decompose the PeerId to its public key, hash it, compare. That is the whole
operation. It answers "does this DID own this PeerId?" and nothing more.

It cannot answer "what PeerId does this DID own?" — that requires a known-peer
set, which is why Option B (publish the DID, disclose the PeerId only at pairing)
requires no directory.

---

## 3. The canonical signing form

```
canonical(domain, fields) :=
    domain
    || u32be(len(fields))
    || for each field: u32be(len(field)) || field

canonical_strings(list) :=
    u32be(len(list))
    || for each item: u32be(len(item)) || item
```

**Every field MUST be length-prefixed.** Without prefixes, `["ab","c"]` and
`["a","bc"]` concatenate identically, and an attacker can shift bytes across a
field boundary to produce a valid signature over different content. This is
pinned by the `address-boundary-unambiguity` vector.

**Every structure MUST use a distinct domain.** The signature over a public
announce MUST NOT verify as a signature over an address exchange. Without that, a
signed public announce could be replayed as an authenticated address exchange —
handing an attacker exactly the addresses the two-phase design exists to withhold.

Current domains:

| Structure | Domain |
|---|---|
| `DiscoveryAnnounce` | `GW/announce/v1` |
| `AddressExchange` | `GW/exchange/v1` |
| `PairingRecord` | version byte + length-prefixed fields |
| `Revocation` | version byte + length-prefixed fields |

---

## 4. Two-phase discovery

The core design. Two structures, deliberately separate.

### 4.1 Phase 1 — `DiscoveryAnnounce` (public)

Broadcast to every hop. **MUST NOT carry any address.**

This is not an oversight. A public announce must not be able to express "dial this
endpoint", because announces are seen by everyone. Addresses travel in phase 2.

Fields: `version`, `network_id`, `peer_id`, `did`, `public_key`, `issued_at`,
`expires_at`, `sequence`, `signature`.

A receiver MUST re-check `expires_at` at dial time, not only at receive time — a
queue can outlive the window it was verified in.

### 4.2 Phase 2 — `AddressExchange` (authenticated)

Sent over the channel established after phase 1 verified the peer. This is the
only structure permitted to carry a libp2p address.

Fields: `version`, `network_id`, `peer_id`, `did`, `addresses`, `issued_at`,
`expires_at`, `sequence`, `signature`.

### 4.3 The dial gate

`VerifiedPeerAddress` is `#[non_exhaustive]`, so no code outside the defining
module can construct one. Dial only what it contains.

---

## 5. The correlation problem, stated in full

**This is a known defect, not a solved property.**

A libp2p `PeerId` for an Ed25519 key is an **identity multihash**: keys of 42
bytes or fewer use `Multihash::wrap(0, …)`, so the SHA-256 branch is
unreachable and the PeerId is the protobuf-encoded public key. Every
`12D3KooW…` literally contains the key.

Therefore:

> **Anyone holding a PeerId can compute the corresponding DID.**

This was verified by decoding a `PeerId` from libp2p's own specification text and
recovering the key.

Consequence: the PeerId is a **permanent cross-transport correlator**. That is in
direct tension with "one identity, many transports, addresses unlinkable until
pairing" — which is this project's central claim.

It is also why `identify` is a problem. It pushes the public key on every
connection, so one libp2p operator sees every GhostWire node's key — and therefore
every DID — with no pairing involved.

**Options, none taken unilaterally:**

1. Disable `identify` and replace its address-discovery role. Costs discovery,
   preserves the property.
2. Adopt a non-identity multihash. Blocked in practice: `identify` rejects any
   peer whose key does not derive their claimed ID.
3. Scope it out: state plainly that libp2p-peer identity is pseudonymous but
   correlatable, and that unlinkability is claimed only within a transport.

**An implementation MUST NOT claim cross-transport unlinkability while a
standards-compliant libp2p PeerId is in use.** That is the requirement this
section exists to state.

---

## 6. Pairing

### 6.1 Records

`PairingRecord` is a signed claim that a DID controls a PeerId at a set of
addresses. Anyone holding the signer's public key can verify it with no
directory, no prior trust, and no network call.

`expires_at` **MUST** be signed, or the natural attack — "just extend my own
expiry" — succeeds.

### 6.2 Revocation

`Revocation` is signed by the key being revoked, and **never expires**. A key
compromised last year is still compromised; a TTL would let an attacker wait it
out.

`is_revoked` **MUST** ignore revocations that do not verify. Obeying them would
let anyone revoke anyone by injecting a garbage record — a denial of service on
the whole network.

A self-signed revocation cannot stop an attacker who already holds the key from
signing their own statements. What it buys is that honest holders of a stale
address learn the truth without trusting a third party.

### 6.3 The pairing code

```
16 bytes of entropy → base32 (RFC 4648 alphabet) → 26 characters, grouped 4-4-4-4-4-4-4
```

`encode()` and `decode()` **MUST** be inverses, and both sides of a typed pairing
**MUST** derive the same 6-digit safety number.

This is what makes pairing possible **without meeting in person**: the code is a
string a person reads aloud, types into a message, or scans as a QR. A
six-digit safety number compared on a voice or video call detects a
man-in-the-middle.

This was broken until recently and is now pinned by three vectors. It is stated
normatively because it is the usability property the whole privacy design rests
on — not an implementation detail.

---

## 7. Versioning and errata

**A change to any encoding in this document is a version bump**, not a refactor.
`spec/vectors.json` MUST be updated in the same commit as the change that caused
the mismatch, so the diff shows intent and effect together.

`SecurityConfiguration`/`SECURITY.md` MUST exist and the issue tracker MUST be
enabled.

That last point is the entire Reticulum lesson in two files. Reticulum's author
stepped down, the repository has `has_issues: false`, and a critical
remote-shell flaw fixed in 1.3.9 came with a promised post-mortem that will never
arrive. **A protocol with no errata channel is one where you cannot know it is
secure.**

---

## 8. Conformance

An implementation conforms when `tests/conformance_vectors.rs` passes, or its
equivalent:

```
cargo run --bin gen_vectors > spec/vectors.json
cargo test --test conformance_vectors
```

**Any second implementation MUST reproduce every vector byte for byte.**

---

## 9. What a second implementation would still have to guess

Honest gaps in this draft. Each is a place where two implementations would
currently disagree:

1. **Error semantics.** What does a verifier return on expiry versus a bad
   signature? Behaviour is specified; return values are not.
2. **Clock skew tolerance.** An announce with `expires_at` in the past by one
   second — accepted or refused? No rule exists.
3. **`sequence` semantics.** Monotonic per issuer, across restarts, across key
   rotation? Undefined.
4. **`network_id` derivation.** How is a network identified and scoped? Parsed
   and length-checked, never derived.
5. **Which transport carries which structure.** Phase 2 says "over the
   authenticated channel", which is not yet a wire format.

These are listed so that nobody mistakes the absence for a decision.
