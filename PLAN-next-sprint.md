# GhostWire — Next Sprint Plan

**Date**: 2026-06-24
**Status**: PENDING APPROVAL
**Scope**: All quick wins + wiring gaps, estimated <1 day total

---

## What Already Exists (Surprise)

The codebase is MORE complete than expected:

| Component | Status | Lines |
|-----------|--------|-------|
| Full CLI (clap subcommands) | ✅ Built | 706 |
| Full TUI (ratatui, 4 views) | ✅ Built | 813 |
| Username Discovery (DHT) | ✅ Built | 349 |
| Mesh publish_username / lookup_username | ✅ Built | in mesh.rs |
| Identity nickname system | ✅ Built | in core_identity.rs |
| Wizard (first-run setup) | ✅ Built | wizard.rs |
| Daemon HTTP API for TUI | ✅ Built | daemon_client.rs |

**The problem isn't "we don't have usernames" — it's "the CLI doesn't wire them in."**

---

## Phase 1: Wire Username System (20 lines, ~15 min)

The username infrastructure exists end-to-end. The CLI just doesn't connect the pieces.

**Gap**: `ghostwire init --username Michael` prints the name but doesn't:
1. Store it in `IdentityMetadata.nickname` via `update_metadata()`
2. Publish it to DHT via `publish_username()`

**Fix**: Edit `cli/mod.rs` `init_identity()` (~15 lines):
```rust
// After line 213: if let Some(name) = username {
//   1. Store in identity metadata
core.identity.update_metadata(Some(name.clone()), None, None, None);
//   2. Publish to DHT when mesh starts
//   (publish_username is called in mesh start, just needs the name set)
```

**Verification**: `cargo build` → run `ghostwire init --username Michael` → check identity file has nickname.

---

## Phase 2: Wire Anonymous Intro (15 lines, ~10 min)

`anonymous_intro.rs` (351 lines) is compiled but never called. It's the SURB-based anonymous contact system.

**Gap**: No code calls `generate_intro_invite()` or `receive_introduction()`.

**Fix**: Wire into mesh.rs message handler (~15 lines):
- In the incoming message handler, check if message type is `Introduction` → call `receive_introduction()`
- In CLI `connect` command, offer `--anonymous` flag → call `generate_intro_invite()`

**Verification**: `cargo build` → unit test passes.

---

## Phase 3: Wire Function Calling (20 lines, ~15 min)

`function_calling.rs` (1013 lines) — NL→tool calling via Ollama. 8 tool schemas, all handlers stub.

**Gap**: `FunctionGemmaClient` exists but is never instantiated in Core.

**Fix**:
1. Add `function_gemma_client: Option<Arc<FunctionGemmaClient>>` to Core struct (~3 lines)
2. Initialize in `Core::new()` if Ollama available (~5 lines)
3. Wire 8 stub handlers to real components (~50 lines, but stubs already return mock data — just connect the plumbing)

**Note**: The stubs returning mock data is FINE for now. The point is the plumbing exists so when we have real handlers, they plug in.

**Verification**: `cargo build` → no new warnings.

---

## Phase 4: Wire Coercion / Duress Mode (30 lines, ~20 min)

`coercion.rs` (261 lines) — Duress PIN + decoy profiles. Constant-time, Argon2id, ZeroizeOnDrop.

**Gap**: `CoercionConfig` never loaded, duress password never checked at unlock.

**Fix** (~30 lines across 4 spots):
1. Load `CoercionConfig` from persistence on startup
2. In identity unlock: try real password first, then duress password → route to decoy namespace
3. In EncryptedStore: add namespace routing (real vs decoy)
4. DuressAlert channel trigger on duress unlock

**Verification**: `cargo build` → test duress flow manually.

---

## Phase 5: Nervous System — Self-Healing (40 lines, ~30 min)

Currently monitors but doesn't react. Add 3 autonomic responses:

**5a. Peer Health Heartbeat** (~15 lines):
- Add periodic ping to connected peers (every 30s)
- If no response in 3 pings → mark peer offline, trigger reconnection

**5b. Self-Healing Reconnection** (~15 lines):
- On peer disconnect → auto-retry with exponential backoff (1s, 2s, 4s, max 30s)
- On transport failure → fallback to next transport via AutoTransportSwitcher

**5c. Rate Limiting** (~10 lines):
- Per-peer message rate limiter (already exists in `stealth_tcp.rs`, reuse pattern)
- If peer exceeds limit → temporary ban (60s)

**Verification**: `cargo build` → test by killing/restarting a peer.

---

## Phase 6: CLI Polish (10 lines, ~10 min)

Minor CLI fixes:

**6a. `init` should auto-start mesh** (~3 lines):
- After init, prompt: "Start mesh now? [Y/n]"
- Or `ghostwire init --start` flag

**6b. `peers` should show usernames** (~5 lines):
- Currently shows peer IDs, resolve via `username_discovery.get_username_for_peer()`

**6c. `send` should show username resolution** (~2 lines):
- Already works via `@username` but output doesn't show the resolved name

**Verification**: `cargo build` → run each command.

---

## Phase 7: typed_crypto Consolidation (50 lines, ~30 min)

`typed_crypto.rs` (314 lines) — `CryptoKind`, `KeyBundle`, wire format. Zero callers.

**Option A (recommended)**: Merge `CryptoKind` + `KeyBundle` into `encryption.rs`, delete file (~50 lines).
**Option B**: Leave as-is (it's not hurting anything, just dead code).

**Recommendation**: Option B — YAGNI. The types might be needed when we add new crypto backends. Not worth the risk of breaking encryption.rs for no immediate benefit.

---

## Summary

| Phase | What | Lines | Time | Risk |
|-------|------|-------|------|------|
| 1 | Wire username system | ~20 | 15 min | Low — connects existing code |
| 2 | Wire anonymous intro | ~15 | 10 min | Low — adds message type handler |
| 3 | Wire function calling | ~20 | 15 min | Low — plumbing only |
| 4 | Wire coercion/duress | ~30 | 20 min | Medium — touches identity unlock |
| 5 | Nervous system self-healing | ~40 | 30 min | Medium — new tokio tasks |
| 6 | CLI polish | ~10 | 10 min | Low — display changes |
| 7 | typed_crypto consolidation | ~50 | 30 min | Skip for now (YAGNI) |
| **Total** | | **~135** | **~100 min** | |

---

## What I Will NOT Do

- **PRoPHET v2 upgrade** — v1 works, v2 is an optimization, not a fix
- **Nostr + Tor** — significant feature, separate sprint
- **TUI improvements** — TUI already works, polish later
- **typed_crypto consolidation** — YAGNI, leave as-is
- **Main binary test harness** — CLI already exists, test harness is fine

---

## Approval Needed

Approve this plan and I'll execute phases 1-6 in order, building after each phase.

Or: "do phase X first" / "skip phase Y" / "add Z" — your call.
