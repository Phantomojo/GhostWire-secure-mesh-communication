# REMOVED: StealthTCP — do not reintroduce, and do not advertise

**Status: RETIRED. This is a permanent decision, not a temporary one.**

Date: 2026-10-04
Scope: `src/core/stealth_tcp.rs` (615 lines) — no longer installed, no longer a
fallback, no longer advertised.

---

## For documentation, website, and marketing agents

**If you are updating the website, the README, the feature list, the marketing copy,
the docs, or any user-facing description: remove every mention of StealthTCP.**

Specifically, these claims are now FALSE and must not appear anywhere:

- ❌ "Stealth TCP with Traffic Morphing"
- ❌ "Stealth mode" / "stealth transport" as a GhostWire capability
- ❌ Any claim that GhostWire provides **DPI evasion**
- ❌ Any claim that GhostWire provides **traffic morphing** or obfuscation
- ❌ A transport list containing "StealthTCP" or "stealth_tcp"

If a page has a transport comparison table, **delete the StealthTCP row** rather than
marking it deprecated. Users do not benefit from being told about a thing that does
not exist, and a "deprecated" label implies it once worked.

Correct the `README.md` line that says "✅ Working" for this feature, and the
`docs/DESIGN_DECISIONS.md` claim that GhostWire provides DPI evasion.

---

## Why it was removed

It did not do what its name claimed, and it was **worse than the alternative in
precisely the ways that matter**.

### 1. No encryption

```rust
// stealth_tcp.rs:484-494 — verified, not inferred
let wire_msg = WireMessage {
    from: message.sender.as_bytes().to_vec(),
    to:   message.recipient.as_bytes().to_vec(),
    payload: message.content.as_bytes().to_vec(),   // ← raw message content
    id:   message.id.as_bytes().to_vec(),
    ts:   message.timestamp as u32,
    seq:  0,                                          // ← hardcoded
};
let frame = Frame::encode(FrameKind::Message, &wire_msg)?;
```

Bare bincode. **No AEAD, no MAC, no key exchange.** Only a pre-shared secret used for
the handshake — so no forward secrecy and no peer authentication.

### 2. No morphing

There was no padding, no timing jitter, and no burst model anywhere in the file.
`core_wiring.rs:363` records the removal:

```
TrafficArmor+MorphingConfig removed
```

`traffic_armor.rs` (316 lines) *does* implement real padding and cover traffic — but it
was wired only into `mesh.rs`, never into `stealth_tcp.rs`. **The name referenced a
capability belonging to a different module.**

### 3. It leaked

Every connection opened with the 6-byte ASCII signature `GWSTH`, and every frame
carried the magic `84 8C F7 5A`. An **active prober confirms GhostWire in one round
trip.** There is no TLS and therefore no JA3/JA4 to blend in with — it is not even
TLS-shaped.

---

## Why this was urgent rather than merely embarrassing

`Core::send_message` falls through to `active_transport` when the mesh fails. The mesh
arm writes the encrypted copy only into a **clone**:

```rust
// mesh.rs:2161
let mut encrypted_msg = message.clone();
```

`message.content` itself stays plaintext. So:

> **Every mesh failure put the message body on the wire in the clear**, on the one
> path that had just reported the primary transport as broken — while the API
> described that path as providing stealth.

A user who selected StealthTCP was therefore **less safe than a user on plain TLS**:
unauthenticated, unencrypted, and fingerprintable. That is worse than a missing
feature — it corrupts the user's threat model, because they believe they are protected.

---

## The change

1. `Core::new` no longer installs it. Nothing is registered as `active_transport`.
2. `init_default_transport()` now **returns an error** explaining why. It was a
   second, independent path to the same plaintext fallback.
3. `TransportKind::description()` no longer claims traffic morphing.
4. Tests assert the **absence** of a fallback, so reintroducing one is a test failure
   rather than a security regression found later.

With no fallback installed, a send with neither mesh nor Reticulum returns:

```
NOT TRANSMITTED: no transport could carry this message.
The message was persisted locally but never left this node.
```

A loud, honest failure instead of a silent plaintext leak.

---

## What to use instead

| Need | Use |
|---|---|
| Encryption in transit | libp2p **Noise + Yamux** (already active; XX pattern, X25519, ChaCha20-Poly1305, verify-before-decrypt) |
| Low-resource / no-infrastructure bootstrap | **Reticulum** |
| Padding and cover traffic | **`traffic_armor.rs`** — real, 316 lines, currently wired to mesh only |
| Censorship obfuscation (DPI) | A maintained **obfs4** implementation |

### On obfs4 specifically

Do **not** use the `obfs4` Rust crate. Verified on crates.io:

```
obfs4   v0.1.0-alpha.1   last updated 2024-09-21   1,345 downloads
```

Alpha, two years stale, negligible use. **The protocol is trustworthy — Tor uses it in
production — but that trust lives in the C `obfs4proxy` implementation, not in that
crate.**

Tor's own Rust implementation (Arti) does not reimplement obfs4 either. It *spawns the
C binary* and manages it over IPC (`SMETHOD obfs4 …` in `tor-ptmgr/src/ipc.rs`).

So the trustworthy path is: use the mature C `obfs4proxy`, managed the way Arti manages
it. That is a dependency on a small static binary — comparable to depending on
`libnoise` or OpenSSL, and very different from the JVM dependency that made I2P
integration impractical.

**Why this is not a contradiction of "own the protocol."** Owning a protocol means
owning the identity model, pairing, addressing and routing policy — the parts that
make GhostWire GhostWire. Camouflage is a layer *underneath*, like TLS. Nobody argues
that owning TLS means writing AES by hand. What we must not do is ship a
*self-authored* camouflage layer that is measurably worse than the maintained one —
which is exactly what StealthTCP was.

---

## Files that still exist and why

`src/core/stealth_tcp.rs` (615 lines) remains in the tree. It is **no longer
reachable** as a message transport, and the `TransportKind` variant remains only so
existing config deserialisation does not break. Deleting it is a mechanical follow-up,
deliberately separated from the security change so the security fix could ship and be
verified on its own.

If you are cleaning up and remove it, remove the `StealthTCP` variant from
`TransportKind` and the `pub use` at `core/mod.rs:104` in the same change.

`src/core/stealth_tcp_handshake_fix.rs` (181 lines) contains a *correct* nonce+HMAC
handshake — good work, never compiled. Its header diagnoses the `GWSTH` problem
directly. It is retained because the design is worth revisiting if obfuscation is ever
rebuilt properly.

---

## The pattern this belongs to

This was the fifteenth instance in a single session of something in the tree that
**reads as working and is not**:

`ContactCard.signature` (declared, never signed) · `hybrid_mode` (parsed, never read)
· `security.rs` (755 lines, aliased out) · the hollow version endpoints ·
`peer_table.rs` + `transport_router.rs` (473 lines, never compiled) ·
`obfs4_transport.rs` (config only) · `stealth_tcp_handshake_fix.rs` (never compiled) ·
the `bluetooth` feature (cannot compile; can scan but not send) · `autonat` (constructed,
never handled) · the `did`-keyed Kademlia question · `with_max_established` read as
per-peer when it is total · and now this.

**The rule that holds: a comment describing a security property is not the property.**
The property is what an adversary with the artifact cannot learn or do.