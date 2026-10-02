# GhostWire Technical Audit — `rescue/sep-disk-tree` @ `e4028f2`

**Date:** 2026-09-28 · **Host:** rustc 1.98.1 / cargo 1.98.1
**Method:** read source at HEAD (extracted via `git archive HEAD` to `/tmp/gwhead`, isolated from the dirty working tree), built and ran the test suite. No files in the repo were modified.

---

## 0. Headline findings

| # | Finding | Severity |
|---|---|---|
| 1 | `tests/delivery_rate.rs` has a **syntax error committed at HEAD** — the test suite does not compile | **Blocker** |
| 2 | CI runs tests with `continue-on-error: true` — the suite can be fully broken and stay green | **Blocker** |
| 3 | `crypto_tests` fails 3–10 of 11 on every run; root cause is **global on-disk state in the test harness**, not product code | **High** |
| 4 | S/Kademlia Sybil + eclipse defenses are fully implemented but **entirely unwired**; peer scoring is commented out | **High** |
| 5 | ~5,850 lines across 13 modules in `src/core/` are **not in the compilation graph at all** | **High** |
| 6 | `ROADMAP.md` claims ~60% test coverage; measured reality is **41.7% lines / 44.5% regions**, with `mesh.rs` at 4.2% | **High** (claim integrity) |
| 7 | Two divergent auth paths: one constant-time, one plain `==` | Medium |
| 8 | Commit `ff3fc7b` advertises "per-message deletion"; no such implementation exists | Medium (claim integrity) |

---

## 1. Build & test: the actual state

The working tree at `gw-push` does **not** compile, but for a reason unrelated to HEAD: an **untracked** `ghostwire/src/core/receive_keyring.rs` (plus a 2-line `core/mod.rs` edit that declares it) is syntactically truncated mid-struct. `git status` shows it as `??`. This is uncommitted local debris, not a defect in `e4028f2`.

HEAD itself, built cleanly with the `webui/dist` stub that CI creates:

```
cargo build --lib --all-targets   → lib + bin + node_to_node test compile
                                    21 warnings (lib), 45 (lib test)
                                    "ghostwire (lib) generated 21 warnings"
```

No compile errors at HEAD. `libghostwire.rlib` links. So the crate itself is sound.

**The test suite is not.**

```
tests/delivery_rate.rs:136:22  error: mismatched closing delimiter: `}`
  136 |  .connect_peer(PeerId::from(libp2p::identity::PublicKey::new([0u8; 32]).into())
      |                      ^ unclosed delimiter
error: could not compile `ghostwire` (test "delivery_rate") due to 2 previous errors
```

A missing paren, introduced in `ff3fc7b` (the commit that added the file). Because `cargo test` compiles all test targets, **`cargo test` cannot complete at HEAD**. This is masked in CI by `continue-on-error: true` on the `test` job (line 37 of `.github/workflows/rust-ci.yml`) and on the `coverage` job (line 102). A fully red test suite reports green.

### Test results (compiled subsets, run directly)

| Target | Result |
|---|---|
| `--lib` | **368 passed, 0 failed, 1 ignored** ✅ |
| `ai_routing_tests` | 7 passed ✅ |
| `api_integration` | 3 passed ✅ |
| `protocol_integration` | 1 passed ✅ |
| `reticulum_integration` | 3 passed ✅ |
| `crypto_tests` | **FAILED — 3–10 of 11 fail, varies per run** ❌ |
| `node_to_node` | **FAILED — 0 passed, 2 failed** ❌ |
| `delivery_rate` | does not compile ❌ |

### The `crypto_tests` flakiness is a harness bug, not a crypto bug

Failure counts across three identical runs: **6/5, 8/3, 4/7 passed**. Non-deterministic. Diagnosis:

`EncryptionManager` reads and writes **global on-disk state** — `~/.ghostwire/encryption.json` (persistent keypair) and `~/.ghostwire/master.key`. `cargo test` runs tests in parallel threads; each `EncryptionManager::new()` races on create/load/rotate of the same two files. Symptoms are consistent with that: `"Decryption failed (authentication failed or corrupted data)"` and a fresh manager unexpectedly reporting `should_rotate_keys()`.

Proof — with an isolated `GHOSTWIRE_HOME` the failures collapse:

```
default:            FAILED.  1 passed; 10 failed   (first run)
                    FAILED.  6 passed;  5 failed
                    FAILED.  8 passed;  3 failed
                    FAILED.  4 passed;  7 failed
GHOSTWIRE_HOME=tmp: FAILED. 10 passed;  1 failed
```

So the encryption layer is functionally correct; **the security test suite does not test it reliably**, and the one residual failure is environmental. There is no per-test isolation, no `tempfile` fixture (despite `tempfile` being a declared dev-dependency), and no CI step sets `GHOSTWIRE_HOME`. This is the single highest-value fix in the repo: the most security-critical tests are the least trustworthy ones.

`node_to_node` (real libp2p connectivity, 8s/19s timeouts) fails both tests — genuine functional failure or environment networking restriction; worth triaging directly.

---

## 2. Architecture as actually implemented

58,553 lines in `src/`, 103 modules. What is real:

**Transport / mesh — genuinely built.** `core/mesh.rs` (2,962 lines) is a real `libp2p` swarm with `#[derive(NetworkBehaviour)]` composing gossipsub, Kademlia, mDNS, identify, autonat, relay, DCUtR, and `request_response`. Noise transport, yamux, connection memory limits. Topics include a dedicated `CRL_GOSSIP` topic. Reticulum transport is behind the `reticulum` (default-on) feature. Sphinx onion routing **is** wired into the send path (`create_sphinx_packet` at `mesh.rs:1362`, forwarding at `:1077`, mixing queue at `:896`).

**Crypto — substantive and mostly sound.** `core/encryption.rs` (1,651 lines):
- XChaCha20-Poly1305, 24-byte **random** nonces via `SystemRandom` (correct choice at 192 bits — no counter-reuse risk)
- Real **ML-KEM-768** hybrid (`pqcrypto_mlkem::mlkem768` keypair/encapsulate/decapsulate) — not a stub
- AAD support bound to ciphertext; Ed25519 signature verification
- X25519 double ratchet with `MAX_SKIP`/`MAX_SKIPPED_KEYS` bounded at 1000, HMAC-SHA256 domain-separated KDF, `Zeroize`/`ZeroizeOnDrop` throughout
- Shamir secret sharing, vodozemac, argon2/pbkdf2 for at-rest

**CRL — the newest milestone is the strongest code in the repo.** `core/crl.rs` (537 lines) is genuinely well-built: per-issuer monotonic epoch high-water marks, signature covering the epoch (so peers cannot inflate the mark), TTL with GC, idempotent merge, and a proper `MergeResult` enum. **The loop is closed end-to-end**: `CrlStore` is owned by `EncryptionManager`, gated fail-closed at both encrypt (`encryption.rs:644`) and decrypt (`:710`) boundaries, and consumed from gossipsub via `handle_crl_gossip` (`encryption.rs:1277`) called at `mesh.rs:2171`. Rotate-keys (`PUT /rotate-keys`, `web.rs:496`; v2 `system.rs:158`) bumps the epoch. **This is real, working, wired code — the opposite of most of the rest of the security story.**

**Web API.** axum 0.8, tiered rate limiting via `tower_governor` (separate `auth` limiter on `/auth` routes), WebSocket, JWT with a `session_epoch` claim for global session revocation, persisted 0600 secrets generated with `ring` CSPRNG.

### What is *not* implemented — implemented-but-unwired, and unwritten

**S/Kademlia defenses exist in full and are called from nowhere.** `core/routing/skademlia.rs` implements all three documented S/Kademlia defenses (NodeId proof-of-work, disjoint-path lookups, /24 IP-diversity enforcement) plus `hardened_peer_score_params()` with aggressive decay and behaviour penalties. Verified call sites outside the defining file: **none.** `validate_node_id`, `enforce_ip_diversity`, and `disjoint_lookup` have zero callers. `hardened_peer_score_params()` has exactly one appearance — inside a **comment** in `core_wiring.rs:265`.

**`core_wiring.rs` is not wiring — it is an instruction document.** It is ~400 lines of commented-out "CHANGE 1: replace X with Y" prose addressed to a future agent, including the peer-scoring integration and Sphinx send-path work. Sphinx got done elsewhere; peer scoring did not. Any reader who greps for a feature and lands in this file will conclude it is integrated. **This is the most misleading artifact in the repo.**

**13 modules in `src/core/` are dead — never compiled, 5,854 lines:**

```
crypto_machine.rs (388)    crypto_processor.rs (137)   calls.rs (315)
chunking.rs (198)          peer_table.rs (326)         hlc.rs (426)
pluggable_transport.rs (392) transport_router.rs (147)  adapter_registry.rs (41)
stealth_tcp_handshake_fix.rs (181)  persistence_test.rs (3)
core_test.rs (108)         + mod.rs alias `security` → security_v2 (false positive)
```

`crypto_machine.rs` deserves emphasis: its own header says *"vodozemac was in Cargo.toml but unused. Custom crypto is almost always wrong. This replacement uses vodozemac's audited Olm/Megolm implementation."* It is a well-intentioned, well-documented migration to audited crypto that **was never declared in `core/mod.rs` and therefore never built.** `media/mod.rs` and `crypto_processor.rs` both `use crate::core::crypto_machine` — so had `media` been declared, it would not have compiled. `media` is itself undeclared. The module tree is internally inconsistent: files reference each other across a boundary that was never made real.

**Orphaned but at least declared** (compiled, never referenced): `db`, `dht_signatures`, `encrypted_store`, `medium_adapter`, `obfs4_transport`, `oem_battery_guide`, `transport_ext`, `transport_pluggable`, `typed_crypto`, `briar`, `qr`.

**Security-hardening posture is opt-in and off by default.** `build.rs` emits on every build:
```
GHOSTWIRE_RELEASE_VERIFYING_KEY_HEX not set. Automatic updates will be disabled at runtime.
GHOSTWIRE_BINARY_HASH not set. Binary integrity check disabled at runtime.
```
Self-update signature verification and binary integrity checking are both **disabled in any default build.** Fine for dev, but it means the release path's security properties are unverified by construction.

---

## 3. Code quality

**Positives:** zero `todo!()`/`unimplemented!()`. Dependencies pinned with explicit CVE-fix rationale comments (`aes-gcm >=0.10.3`, `curve25519-dalek >=4.1.3`, `reqwest >=0.11.23`). Clippy lint table in `Cargo.toml` covering the `cast_*` family. `zeroize`/`secrecy` used deliberately. `deny.toml`, `.gitleaks.toml`, pre-commit config, cargo-deny and gitleaks CI all present. The CRL module and the double ratchet are genuinely careful work. `src/core/crl.rs` and `routing/sphinx.rs` at 88% and 75% line coverage show real unit-test discipline in the newest code.

**Negatives:**

- **561 `unwrap()`, 29 `expect()`, 10 `panic!()` in `src/`.** 58 in just `encryption.rs`/`mesh.rs`/`message.rs`. Release profile is `panic = "abort"` — a panic is a full node crash, not a caught error. `expect("Time went backwards")` sits on the auth login path (`auth.rs:29`).
- **`cargo fmt --all -- --check` fails.** CI runs this check, so the lint job is red (or was never run).
- **~544 clippy diagnostics** on `--all-targets`. `cargo clippy --all-targets` in CI has no `-D warnings`, so it is advisory only — but 544 is not advisory noise, it is a signal nobody is reading.
- **No `rustfmt.toml`, no `clippy.toml`, no `deny.toml` tuning** — the quality gates that exist are at defaults.
- **Committed dead weight:** `src/tui.pre-20260817T223241Z/` (2,106 lines of a timestamped pre-migration TUI copy), `.manus-backups/` (720K), `.audit-backups/` (256K), plus root-level zero-byte files `404` and `429` and a 100KB `audio.mp3`. Also present: `web3` (a 3.5KB file with zero code files), `src-tauri` (81 lines, 3 files), and duplicate licence/contributing files in both root and `ghostwire/`. This is the visible residue of the AI-heavy workflow the owner described.
- `docs/presentations/` carries 18 files, 15 of them modified-but-uncommitted.

---

## 4. Test coverage: claimed vs. real

**Claim** (`ROADMAP.md:166`): `| Test coverage | >80% | ~60% |`

**Measured** (`ghostwire/coverage-report.txt`, llvm-cov, dated 2026-09-20 — three days before HEAD, and on the dirty tree):

```
TOTAL   regions 44.53%   functions 37.76%   lines 41.71%   branches n/a
```

Coverage is materially lower than claimed, and the gap is concentrated exactly where the risk is:

| Module | Region | Line | Note |
|---|---|---|---|
| `core/crl.rs` | 96.42% | 88.46% | ✅ genuinely well tested |
| `core/double_ratchet.rs` | 94.54% | 77.08% | ✅ |
| `core/routing/sphinx.rs` | 95.50% | 75.00% | ✅ |
| `core/crypto_profile.rs` | 86.16% | 90.00% | ✅ |
| `core/reliability.rs` | 86.51% | 83.33% | ✅ |
| `core/encryption.rs` | 64.91% | 51.35% | ⚠️ half the core crypto path untested |
| `core/persistence_manager.rs` | 77.48% | 56.25% | ⚠️ |
| `core/reputation.rs` | 54.92% | 43.10% | ⚠️ |
| `core/security_v2.rs` | 51.29% | 46.43% | ⚠️ |
| **`core/mesh.rs`** | **9.07%** | **4.19%** | ❌ the largest file, 2,962 lines |
| `web.rs` | 0.00% | 0.00% | ❌ |
| `tui/app.rs` | 0.00% | 0.00% | ❌ |

**24 of 103 measured files sit at literally 0%**, including every controller (`all_controllers`, `auth`, `identity`, `messaging`, `peer`, `profiles`, `system`, `health`), all of `api_v2_*`, `v2_routes.rs`, `daemon.rs`, and the entire TUI.

The headline: **the crypto primitives are well covered; the entire network and API surface — where the adversarial surface actually lives — is untested.** Branch coverage is reported as `0 / -` throughout, i.e. branch metrics were not collected at all, so the one dimension that would catch logic inversions in auth and rate limiting is absent. Fuzzing is `❌` in the roadmap and `fuzz/` is empty.

The `coverage-report.txt` itself is **untracked** (`??`) — the artifact backing the ~60% claim is not even in the repository.

---

## 5. Security-sensitive invariants

### Sound and verified in source
- **AEAD + random nonce discipline** — XChaCha20-Poly1305, 24-byte `SystemRandom` nonces throughout; 12-byte lengths asserted before use. AAD binds metadata to ciphertext.
- **CRL is fail-closed and loop-complete** — rejected at encrypt *and* decrypt; epoch rollback blocked per-issuer; signature covers the epoch; merge is idempotent and TTL-bounded; epoch bumped on rotation; consumed from real gossipsub traffic.
- **Session revocation works** — `session_epoch` is stamped at login, checked on every JWT path (`web.rs:343,354`), and `revoke_sessions()` is called from profile mutation handlers.
- **Secrets at rest** — 0600, CSPRNG-generated (`ring::SystemRandom`), 48-byte JWT secret; `zeroize`/`ZeroizeOnDrop` on key material.
- **Rate limiting** — `tower_governor` on all routes with a tighter limiter on `/auth`.
- **Constant-time token compare in the primary auth path** (`web.rs:333`, `subtle::ConstantTimeEq`).

### Real weaknesses

1. **Divergent auth comparison — timing side channel, second path.** `controllers/auth.rs:25` does `if payload.token == *AUTH_TOKEN` — a plain, short-circuiting byte comparison on the secret token, reachable from `POST /auth/login`. The middleware at `web.rs:333` correctly uses `ct_eq`. Two code paths, one safe. `payload.token == *AUTH_TOKEN` must be `ct_eq`. This is the clearest single-line fix in the audit.

2. **Same endpoint, two different issuance paths.** `auth.rs` accepts the raw static `AUTH_TOKEN`; `web.rs` accepts either the raw token *or* a JWT. The static token is a long-lived, never-expiring bearer credential with no `session_epoch`, so `revoke_sessions()` **cannot invalidate it** — session revocation is incomplete by construction for the primary credential. Combined with #1, the login endpoint is both timing-leaky and un-revocable.

3. **`Validation::default()` on all three JWT decode sites.** `jsonwebtoken::Validation::default()` in jsonwebtoken 10 validates `exp` but leaves the algorithm set permissive. These are HS256 self-signed tokens where the attacker supplies no key, so this is low risk today — but it is exactly the class of config that becomes critical the moment a public-key or JWKS path is added. `set_algorithms(&[Algorithm::HS256])` should be explicit.

4. **No per-test isolation for the crypto suite** (section 1). The security tests are the flaky ones. This is an assurance failure as much as a code one.

5. **Peer scoring disabled.** With `hardened_peer_score_params()` commented out and no IP-diversity enforcement at connection time, the mesh has **no Sybil resistance and no eclipse defense at runtime** — despite a 512-line module whose docstring opens by citing Baumgart & Mies 2007 and the 2024 eclipse-attack response. The threat model is documented; the mitigation is not deployed. Given the `e4028f2` commit headline explicitly names "NAT-path eclipse", this is the most consequential gap between stated and shipped security posture.

6. **No lockfile pin for the `sphinx-rs` / `ghostwire-dtn` / `hlc-rs` / `trust-store` crates.** All four are `"0.1"`/`"0.9.7"` version requirements from crates.io with path overrides commented out — unreviewed third-party code in the routing path, and the local sources that were meant to be tested against are disabled.

### Claim-integrity issues
- `ff3fc7b` is titled *"CRL gossip, per-message deletion, rotate-keys endpoint"*. CRL gossip ✅ real. Rotate-keys endpoint ✅ real (`PUT /rotate-keys`). **Per-message deletion: does not exist.** `core/message.rs:67` still carries `- [ ] Add deletion tombstones (for synced deletion)`. The commit's diff touches no deletion code.
- `e4028f2`'s "P0 epoch fail-closed" is accurate *for CRL merge* — but the epoch is not checked anywhere in `mesh.rs`, and `receive_keyring.rs` (the file that would consume it on the receive path) is untracked and non-compiling. The invariant holds on the encrypt/decrypt path and nowhere else.
- `ROADMAP.md` is dated 2026-07-03, still claims `Key Revocation — ❌` and `Forward Secrecy — Per-session ❌`, and has a **duplicated Phase 2 block** listing already-completed items as unchecked. It contradicts the code in both directions and has not been maintained across three months of substantial work.
- `ROADMAP.md:166` claims ~60% coverage; reality is 41.7%.

---

## 6. Recommended order of work

1. **Fix `tests/delivery_rate.rs:136`** (one paren) and **remove `continue-on-error: true`** from the test and coverage CI jobs. Until the suite is enforced, no coverage number means anything.
2. **Isolate `crypto_tests`** with a per-test `tempfile` + `GHOSTWIRE_HOME` fixture, and set an isolated home in CI. This converts the most security-critical tests from untrustworthy to trustworthy — highest assurance-per-effort in the repo.
3. **Fix the auth comparison** (`auth.rs:25` → `subtle::ConstantTimeEq`) and decide whether the static `AUTH_TOKEN` should be revocable, or retire it in favour of the epoch-checked JWT.
4. **Decide the fate of the 13 dead modules.** Either declare `crypto_machine`/`crypto_processor`/`media` (finishing the vodozemac migration that was clearly intended) or delete them. Shipping a hand-rolled ratchet while an audited-Olm replacement sits uncompiled in-tree is the worst of both worlds.
5. **Wire peer scoring + IP diversity** in `mesh.rs` at `ConnectionEstablished`, or delete `skademlia.rs` and remove the eclipse claims from the threat model. A documented-but-deployed-nowhere eclipse defense is worse than none, because it invites reliance.
6. **Delete committed debris** — `tui.pre-*`, `.manus-backups/`, `.audit-backups/`, `404`, `429`, `audio.mp3`, `web3`.
7. **Make `core_wiring.rs` either real wiring or a `docs/` design note.** Its current form actively misleads.
8. Run `cargo fmt`; decide whether the 544 clippy findings are advisory or blocking.
9. Reconcile `ROADMAP.md` against the code, and correct the ~60% coverage claim to 41.7% (or invest in the untested `mesh.rs` / `web.rs` surface, which is where the real risk is).

---

## Appendix — commands run

```bash
git archive HEAD | tar -x -C /tmp/gwhead          # isolated HEAD build
mkdir -p /tmp/gwhead/webui/dist                   # CI does this (build.rs RustEmbed needs it)
cargo build --lib --all-targets                    # clean at HEAD, 21 warnings
cargo test --lib                                   # 368 passed, 0 failed, 1 ignored
cargo test --test {crypto_tests,ai_routing_tests,api_integration,
                   protocol_integration,reticulum_integration,node_to_node}
GHOSTWIRE_HOME=$(mktemp -d) cargo test --test crypto_tests   # 10/11 pass
cargo fmt --all -- --check                         # fails (benches/hot_paths.rs)
cargo clippy --all-targets                         # ~544 diagnostics
```

No file in `/home/phantomojo/rescue/gw-push` was created, modified, or deleted. Scratch artifacts live in `/tmp/gwhead` and `~/.hermes/cache/scratch/gwt`.
