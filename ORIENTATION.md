# GhostWire — Orientation Brief

**Read this FIRST, every session, before touching anything.** It exists because
this project has a specific and repeating failure mode: code that looks
finished, is documented as working, and is not actually wired in. Three
separate audits have now found it. Trust nothing on sight — verify the call
path.

Last verified: 2026-09-29 · branch `rescue/sep-disk-tree` · suite 478 passing

---

## 1. What this project actually is

A Rust/libp2p anonymous mesh. Peer-to-peer encrypted messaging with multiple
transports, a Sphinx mixnet layer, and a plugin-style transport stack. It is
run by one person (Michael / phantomojo) as security infrastructure, not a demo.

**Owner:** Michael Irungu Muriithi (phantomojo), Nairobi.
**Canonical checkout:** `/home/phantomojo/rescue/gw-push` (branch `rescue/sep-disk-tree`).
**Crate root:** `/home/phantomojo/rescue/gw-push/ghostwire`

Do NOT work in the 1TB `/mnt/tb1/@home/ph/GhostWire-*` checkouts. They are
dangling worktrees with a stale gitdir pointer. Any note claiming "git is down"
or "the -newest checkout is canonical" is FALSE and out of date.

---

## 2. The trap: plausible code that is never called

This is the single most important thing to know.

| Module | Looks like | Reality |
|---|---|---|
| `obfs4_transport.rs` | obfs4 obfuscation | **No handshake, no framing, not in the transport chain.** Its "public key" was literally `secret.to_vec()`. |
| `traffic_armor.rs` → `CoverTrafficScheduler` | idle cover traffic | Built at `mesh.rs:775`, bound to `_traffic_armor`, **never started. Zero callers.** |
| `armor.rs` → `ErasureArmor` | Reed-Solomon message coding | Constructed on `Core`, `encode()`/`reconstruct()` **never called.** |
| `binary_hardening.rs` → `SecureBuffer`/`lock_memory` | mlocked key memory | Correct code, **zero callers.** Keys can still reach swap. |
| `network/presence_obfuscation.rs` | intersection-attack defence | 282 lines, **zero callers.** |
| QUIC provider (older) | working QUIC | Was a dead inner wrapper; fixed in `f893453`. |

**The check that catches all of them:**
```
grep -rn '<symbol>' src/ --include=*.rs | grep -v '<its own file>'
```
Zero hits outside the defining file = dead code, regardless of how good it is.

Also check for a leading underscore in a binding (`let _x = ...`) — that is
"constructed and deliberately dropped".

**And check the docs against reality.** `docs/ARCHITECTURE.md:89` claims
"Transport 8/10 ✅ Working — libp2p, Iroh, BT, Obfs4". Iroh and BT are not in
the tree. `CONFIGURATION.md:225` says `GHOSTWIRE_OBFS4_ENABLED` enables obfs4
DPI evasion; it enables nothing. `ARCHITECTURE.md:198` contradicts both by
correctly marking obfs4 as "No".

---

## 3. What genuinely works (verified, not assumed)

- **Message padding** — `pad_message`/`unpad_message` really are on the send and
  receive path (`mesh.rs:1637`, `mesh.rs:2774`). Fixed 512-byte blocks.
- **QUIC** — proven end-to-end on two physical machines with TCP deliberately
  unreachable (see §6).
- **RNS send routing** — a 32-hex Reticulum address now routes to RNS instead of
  failing a PeerId parse (`src/core/reticulum.rs::is_reticulum_address`).
- **Release hardening** — `[profile.release]` has `lto`, `codegen-units=1`,
  `strip`, `panic=abort`, `opt-level=2`. Note `debug = true` is under
  `[profile.dev]` and is correct there.
- **Anti-debug** — `binary_hardening::detect_debugger()` reads
  `/proc/self/status` TracerPid and exits(1); called from `main.rs:358`.

---

## 4. Build, test, verify

```bash
cd /home/phantomojo/rescue/gw-push/ghostwire
cargo build --lib 2>&1 | grep -E '^error' -A8      # fast check
cargo clippy --all-targets 2>&1 | grep -cE '^error' # must be 0
export GHOSTWIRE_HOME=$(mktemp -d)                  # tests need an isolated profile
cargo test --no-fail-fast                           # full: ~478 tests, ~3 min
cargo test --test <name>                            # single integration test
```

**Green means nothing on its own.** A test that never exercises the broken path
passes forever. Prove the test bites by reverting the fix and watching it go red
(fail-first) before believing it.

Conventions: `anyhow::Result` in fallible paths, never `unwrap()` in production;
`tracing::*` never `println!`; `Arc<RwLock<T>>` for multi-reader state.

**Careful with `src/core/mesh.rs`.** It has multiple `impl` blocks and two
`pub async fn new` (line ~300 `MeshManager::new`, line ~3107 one that delegates
to it). A text replacement matching a similar nearby block will silently edit
the wrong struct literal and you will chase a phantom error. Re-read the region
after every edit and let the compiler's line number confirm the site.

---

## 5. Multi-agent coordination

There is a shared blackboard at `/home/phantomojo/.hermes/blackboard/`, exposed to
both Hermes and OpenCode as MCP tools `post` / `read` / `wait` / `topics`.
One append-only JSONL file per topic. Either agent posts; the other picks it up
on its next `wait`.

Current topics: `anti-re` (the anti-reverse-engineering audit backlog).

```bash
# see the conversation live, as the human
tail -f /home/phantomojo/.hermes/blackboard/anti-re.jsonl
```

`wait` long-polls, so it wakes as soon as the other agent posts. It is **not**
push: an agent only learns of a new message when it next calls a blackboard tool.
Coordinate deliberately at the start of a task, not only at the end.

---

## 6. Live-test topology (two real machines)

- **A** — this box, 192.168.100.7, Tailscale 100.64.21.17. Web 9950.
- **B** — Elitebook, 192.168.100.196, Tailscale 100.69.164.79, user `elite`,
  CachyOS. `ssh elitbox`. Web 9951, mesh 4001, QUIC udp/8081.

B runs the same binary from a synced copy at `/home/elite/ghostwire-test/`.
Its login shell is **fish**, so `VAR=x cmd` and `$VAR` expansions fail silently —
wrap remote work in `bash -lc '...'` or a script file. B's live logs:
`/tmp/qB3.log`; profile `/tmp/gw-qB3` (token in `/tmp/gw-qB3/token`).

**QUIC proof (reproduce rather than re-derive):** start B with
`--mesh-port 4999`, a port not opened in its firewall, so TCP is impossible
while udp/8081 stays open. A message still arrives and B holds zero established
TCP sockets. That is how QUIC was proven to carry real traffic.

**Do not** try to capture packets with sudo/tcpdump — no passwordless sudo on
either host, and passwords must not be pasted into chat. Ask the human to run
the capture themselves.

---

## 7. Open work, in priority order

1. **Anti-RE wiring** (audit on the `anti-re` board topic): start
   `CoverTrafficScheduler`, put key material in `SecureBuffer`/mlock, wire
   `presence_obfuscation`, fix the docs.
2. **RNS delivery** — routing works, but a real send fails
   `DroppedMissingDestinationIdentity` because the peer's announce is not heard
   over multicast. Suspect multicast filtering or announce timing. Unresolved.
3. **Mesh driver supervision** — a `libp2p` 0.56 `request-response` panic
   (`ConnectionClosed` + stale `connections.len()` during deferred event
   processing) kills the swarm. `2508adf` only made it loud; `1fa9680` added
   `MeshManager::rebuild_inputs`/`rebuild` so a supervisor loop can be written.
   The supervisor itself is not implemented yet.
4. **obfs4** — must be built or the claim removed. See the ledger for why
   obfs4 alone is not a long-term answer and why metadata leaks matter more.

**Ledger:** `~/GHOSTWIRE_LEDGER.md` (1800 lines) is the record of every finding,
correction and non-verification. Append an entry before calling a task done.

---

## 8. Rules that are not negotiable

- Never claim a fix works without a command and its real output.
- Report what is **not** verified as prominently as what is.
- If an audit claim turns out wrong, say so in the ledger and correct it.
  (Two audit claims were wrong and both are recorded.)
- Secrets: never echo keys into chat, commits, skills, or logs.
- Do not `git push` without asking. Several commits are local-only.
