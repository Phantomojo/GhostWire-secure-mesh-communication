# GhostWire — Capability Status Spine

> **Generated:** 2026-09-28 · **Repo state:** `rescue/sep-disk-tree` @ `e4028f2` (2026-09-22) · **Build:** `cargo check --lib --bins` clean
> **Tests:** 368 passed, 0 failed, 1 ignored (`cargo test --lib`, verified 2026-09-28)
> **Size:** 58,166 lines across 151 `.rs` files in `ghostwire/src/` · 66,116 lines crate-wide

This file is the **single source of truth for what GhostWire actually does.** Every capability claim in the Bible, the website, the README, and any funder document should point here rather than restating status in prose. That removes the contradiction class of error entirely: two chapters cannot disagree if neither is the source of truth.

**Re-verify before reuse.** If this file is more than ~30 days older than the repo tip, treat every row as `stale` and re-run the evidence commands in the appendix.

---

## Status vocabulary

Fixed set. No free-text status — free text is how "functional" and "in progress" come to mean the same thing on different pages.

| Status | Meaning |
|---|---|
| `verified` | Implemented AND exercised by a test or live run. Evidence is a command that passes. |
| `implemented-unproven` | Code exists and compiles; no passing test or live evidence yet. |
| `prototype` | Partial, research-grade, or explicitly scoped as experimental. |
| `scaffold` | Deliberate placeholder with honest in-code comments. Not a feature. |
| `planned` | Designed, not built. No implementation. |
| `absent` | Does not exist. Remove from all documents. |

---

## 1. Cryptography — the strongest part of the project

| Capability | Status | Evidence |
|---|---|---|
| XChaCha20-Poly1305 AEAD encryption | `verified` | `core/encryption.rs`; nonce handling per-message random 192-bit |
| Ed25519 identity signatures | `verified` | `crypto_tests.rs` `test_message_signing`, `test_signature_tamper_detection` — pass |
| X25519 key exchange + forward-secrecy ratchet | `verified` | `crypto_tests.rs` 11/11 pass |
| **ML-KEM-768 hybrid post-quantum on live message path** | **`verified`** | `encryption.rs:206` calls `generate_hybrid_key_pair`; 1,248-byte composite key (32 Ed + 32 X25519 + 1,184 ML-KEM). `mesh.rs:1295` → `encrypt_message` → real `mlkem768::encapsulate` at `encryption.rs:658-680`, HKDF-mixed with ECDH. `mesh.rs:1316` serialises 1,088-byte `pq_key` as the **third wire segment** of `encrypted:v4:`. `mesh.rs:2339-2355` parses it back. Test `test_pq_hybrid_encrypt_decrypt` **passes**, asserting the Kyber branch is taken. |
| Sphinx onion routing with SURBs | `verified` | `routing/sphinx.rs` — 6 pub fns incl. SURB create/use/decrypt, 7 tests. `mesh.rs:908-924` subscribes `SPHINX_TOPIC`; `:1032` `select_sphinx_route`; `:1359-1371` sends via Sphinx **in preference to** direct gossipsub; `:1015` relay forwarding through a `mixing_delay` queue. |
| Binary integrity check | `absent` (disabled) | Build prints `GHOSTWIRE_BINARY_HASH not set`. Config unset in current builds. **Never claim active.** |
| Signed auto-update | `implemented-unproven` | Updater verification code exists; `GHOSTWIRE_RELEASE_VERIFYING_KEY_HEX not set` at build. Default key is a zero placeholder. **Release blocker.** |

### Post-quantum key establishment cost — measured 2026-09-28

Added `bench_mlkem768` to `benches/hot_paths.rs` because **no benchmark in the repo previously measured PQ at all**, and three chapters quoted a figure with no benchmark behind it.

```
mlkem768/keypair      time: [10.509 µs 10.548 µs 10.586 µs]
mlkem768/encapsulate  time: [10.489 µs 10.508 µs 10.527 µs]
mlkem768/decapsulate  time: [11.117 µs 11.229 µs 11.362 µs]
```

Reproduce: `cargo bench --bench hot_paths -- mlkem768`

| Prior claim | Where | Reality |
|---|---|---|
| "ML-KEM encapsulation completes in 890 µs on-device" | ch05, ch11, ch14 | **Falsified — off by ~85×.** Measured encapsulation is **10.5 µs**, not 890 µs. The only source for the old number (`Performance_Benchmarks.md:143-147`) states an **Intel i7-10700K** — desktop silicon described as "on-device". |
| "77 µs LightGBM inference" | ch11, ch16, website | **Never measured.** The figure exists only as a code comment at `onnx_inference.rs:153`; the bench harness never covers the ONNX path. Treat as unknown until benchmarked. |

> **Caveat that must travel with the corrected number:** these are desktop x86 figures measured on the development machine. Constrained hardware (ARM SBC, embedded radio) will be slower — plausibly 5–20×, which still lands well under the 890 µs claim. Do **not** restate these as on-device figures. Until a target-class benchmark exists, say: *"measured at 10.5 µs on x86_64 development hardware."*
>
> The benchmark also asserts a PQ round-trip (encapsulate → decapsulate recovers the same shared secret), so these timings cannot be silently measuring a no-op.


> **Note on PQ wording:** ML-KEM-768 is genuinely integrated and tested. The Bible previously listed it as *planned* — that understates it. Conversely, the dev.to article lists it as *planned* while the website lists it as shipped. **Both should read: implemented and tested, not yet independently audited.**

---

## 2. Revocation and trust — the Sep 22 milestone

| Capability | Status | Evidence |
|---|---|---|
| CRL epoch anti-rollback (P0) | `verified` | `crl.rs:53` `epoch` field; `:189-199` rejects `epoch < high-water` with `MergeResult::StaleEpoch`; `:203-208` signature covers the epoch; `:231-235` per-issuer high-water marks. 14 tests. |
| Fail-closed revocation gates | `verified` | Gates at **both** encrypt and decrypt paths. |
| CRL gossip between nodes | `verified` | `constants.rs:208` `CRL_GOSSIP` topic; `mesh.rs:490-492` subscribes; `mesh.rs:2168-2173` handles inbound → `encryption.rs:1277` `handle_crl_gossip`. |
| `PUT /rotate-keys` endpoint | `verified` | `web.rs:496` → `controllers/system.rs:158` → `encryption.rs:1214` `rotate_keys_if_needed_with_persistence`. |
| Session revocation covers the static API token | `absent` | `AUTH_TOKEN` has no `session_epoch`, so `revoke_sessions()` **cannot invalidate the primary credential**. Open design gap. |

---

## 3. Coercion resistance — complete in code, lightly wired

| Capability | Status | Evidence |
|---|---|---|
| Duress PIN → decoy identity namespace | `implemented-unproven` | `identity/coercion.rs` — Argon2 duress path → `ProfileKind::Decoy` (`:166-172`). 5 tests pass. **No controller or API route exposes it.** |
| Decoy profiles | `implemented-unproven` | Same subsystem. |
| Dead man's switch | `verified` | `identity/panic.rs` — timer loop is real. |
| Panic wipe actually wipes | `prototype` | `panic.rs:151-155` explicitly does **not** wipe — comment: "Caller is responsible for the actual wipe." `execute_panic` has **no production caller**, only its own test at `panic.rs:285`. `PanicManager` is constructed at `core/mod.rs:726`. **Do not claim panic wipe works.** |

---

## 4. Transports — this is where the Bible is wrong

`TransportKind` (`core/peer_table.rs:28-35`) is exactly: `Quic | StealthTcp | Mesh | Bluetooth | WiFi | Reticulum`.

| Transport | Status | Evidence |
|---|---|---|
| Stealth TCP | `verified` | 615 lines, wired at `transport.rs:12` |
| WebRTC | `verified` | `transport_ext/webrtc_transport.rs` (19 KB); real `webrtc::tokio::Transport` in the swarm at `mesh.rs:262-277` |
| Reticulum / NomadNet | `verified` | 883 lines, wired at `transport.rs:17` |
| QUIC | `implemented-unproven` | Present in `TransportKind`; downgrade refusal wired at `quic_transport.rs:16-17,153-154` only |
| **Bluetooth LE** | `scaffold` | `bluetooth.rs:35-39` — `send_ble_data` logs "Skeleton" and returns `Ok(())`; `stop_scanning` is a no-op. `/ble/*` API routes commented out at `web.rs:552-554`. 63 lines total. |
| **LoRa** | `absent` | `disaster.rs:65` has a `LoRa` variant but **no LoRa transport implementation exists** — zero `struct *Lora*` hits. It is only a selector weight (`transport_selector.rs:110`). `TODO.md:186`: *"Protocol is defined but no hardware integration."* |
| **WiFi Direct** | `scaffold` | `medium_adapter.rs:111-130` `WifiDirectAdapter` returns `unsupported_snapshot`, reason: "Platform adapter boundary exists; OS provider is not installed". |
| Transport downgrade refusal | `implemented-unproven` | `crypto_profile.rs:271` returns `NegotiationError::DowngradeRejected`, test at `:329`. **Wired into QUIC only — not the libp2p mesh handshake.** |

> **Every "five transports are functional" / "nine live transports" claim must be deleted.** LoRa does not exist; WiFi Direct returns unsupported; BLE is a logging stub. Three are real: stealth TCP, WebRTC, Reticulum.

---

## 5. Routing and delay tolerance

| Capability | Status | Evidence |
|---|---|---|
| Delay-tolerant store-and-forward | `verified` (as fallback) | `dtn/bundle.rs` (135 L) + `dtn/epidemic.rs` (611 L, 9 tests). `core/mod.rs:1816,1870` call `set_dtn_store(...)`, so the `None` at `mesh.rs:384` is overwritten on the real path. **Engages only when gossipsub publish fails** (`mesh.rs:1408-1436`) — a fallback, not the primary route. |
| PRoPHET epidemic formula | `verified` | `dtn/epidemic.rs`, 9 tests |
| ONNX routing classifier | `implemented-unproven` | `core/mod.rs:781` `RoutingClassifier::new("models/ghostwire_routing.onnx")`; `:411` `onnx_router: Option<...>`. **Not called from routing decisions** — known issue, HIGH, open since 2026-06-24. |
| GNN router (Layer 3) | `prototype` | `ai/gnn_router.rs` — constants and structure present; no trained model wired into live path |
| Four-layer AI routing (LightGBM → Gemma 4 → GNN → PRoPHET) | `prototype` | Layers 2-4 are structure. **Layer 1 exists but is unused in routing decisions.** Describe as "privacy-preserving intelligence architecture under development", never as shipped. |

---

## 6. Eclipse / Sybil defences — WRITTEN BUT NEVER CALLED

This is the most dangerous category in the project, because the documentation implies protection that the running system does not provide.

| Defence | Status | Evidence |
|---|---|---|
| `validate_node_id` | `implemented-unproven` | `routing/skademlia.rs` — **zero call sites.** Only mention is a doc comment at `:19`. |
| `enforce_ip_diversity` | `implemented-unproven` | Same. Doc comment only. |
| `disjoint_lookup` | `implemented-unproven` | Defined at `skademlia.rs:275`. **Zero call sites.** |
| `hardened_peer_score_params` | `implemented-unproven` | Appears **only inside comments** (`skademlia.rs:426,431`, `core_wiring.rs:261,265`). Never invoked. |

> The threat model v1.2 (2026-09-22) explicitly names the **NAT-path/eclipse adversary**. The mitigations for it exist as code and are not deployed. Either wire them into peer admission, or remove the protection claim from every document. **A documented-but-absent defence is worse than an absent one, because it reads as coverage.**

---

## 7. Bridges and interoperability

| Bridge | Status | Evidence |
|---|---|---|
| Briar | `scaffold` | `briar.rs:1` self-describes as "Briar-**inspired**". `BriarManager` (`:79`) is defined and **never instantiated anywhere**; the only registration call is commented out at `:411`. Behind non-default `briar` feature flag. |
| Meshtastic | `absent` | No meshtastic module. Erasure-coding code exists for LoRa framing; no bridge. |
| Matrix | `absent` | No matrix module. |
| Reticulum | `verified` | This one is real — a first-party transport, not a bridge. |

---

## 8. Application surface

| Capability | Status | Evidence |
|---|---|---|
| Axum HTTP API + WebSocket | `implemented-unproven` | `web.rs` (972 L). **0% test coverage** — 24 controller/API files at 0.00%. |
| TUI (Ratatui) | `implemented-unproven` | `tui/app.rs` 1,231 L. 0% coverage. |
| Auth token comparison | `verified` (fixed 2026-09-28) | `web.rs:333` uses `subtle` constant-time. `controllers/auth.rs:25` **was** plain `==` — fixed to `ct_eq` this session. Both paths now constant-time. |
| CLI | `implemented-unproven` | `cli/mod.rs` |

---

## 9. Corrections to prior public claims

| Prior claim | Where it appears | Reality |
|---|---|---|
| "114,000+ lines, 231 source files" | OTF post, possibly site | **58,166 lines / 151 files** in `src/`; 66,116 crate-wide. No counting method reaches 114k. |
| "42,000 lines" | Concept note | Same origin — a repo-wide `find` that swept in `.audit-backups/`, `.manus-backups/`, and `tui.pre-*` snapshots. |
| "six-node network test" | OTF post, site | `tests/node_to_node.rs` builds nodes A and B only. `delivery_rate.rs` is a 2-node and 3-node harness. No 6-node test exists. |
| "per-message deletion" | OTF post, commit `ff3fc7b` message | **Does not exist.** `message.rs:67` is `- [ ] Add deletion tombstones` in a TODO list. `ff3fc7b` contains no deletion code. |
| "300/300 tests" (deck slide 9) | Slide deck | **368 passing, 0 failed.** |
| "83% research / 13% build" | Ledger, OTF post | No such build profile exists in any `Cargo.toml`. The figure describes **session mix**, correctly: **1,859 research vs 291 build sessions** (85:15), measured across 2,271 sessions. Attribute it that way or drop it. |
| "ESP32 / Heltec / LilyGo verified" | Concept note §5 | No firmware, no `esp-idf` dependency, zero Rust references. `TODO.md:186` contradicts directly. |
| "Briar bridge functional" | OTF post, Lorna guide | `scaffold`, not functional. |
| "Telegram delivers 1%" | Concept note §3 | Retracted in the deck, still asserted in the note. Pick one. |

---

## 10. Test coverage — the shape matters more than the number

**41.7% lines / 44.5% regions.** Branch coverage was never collected.

| Module | Coverage | Reading |
|---|---|---|
| `crl.rs` | 88% | Best-tested security code in the repo |
| `double_ratchet.rs` | 77% | Strong |
| `sphinx.rs` | 75% | Strong |
| `mesh.rs` | **4.19%** | 2,962 lines, the core of the system, essentially untested |
| `web.rs` + all 24 controller/API files | **0.00%** | Entire HTTP/auth surface untested |
| `tui/app.rs`, `daemon.rs` | 0.00% | |

> **The honest framing:** cryptographic primitives are well tested; the network and API surface is not. Do not report a single coverage percentage without this table beside it.

**Crypto test flakiness — resolved 2026-09-28.** `crypto_tests` failed 3–10 of 11 tests non-deterministically. Root cause: `EncryptionManager::new()` → `load_or_create()` persists to `GHOSTWIRE_HOME`, and cargo runs the binary's tests on parallel threads, so they raced on shared on-disk state. **Not a crypto bug.** Fixed by serialising with a process-global mutex and a per-test temp home. Proof: without the guard, 6 consecutive runs failed 3–10 tests; with it, 11/11 pass, repeatedly. The repo already had an unused `TestHome` helper at `encryption.rs:1436-1454` documented "Every test that reaches HOME-derived paths must hold one" — **zero tests used it.** Prefer that helper for future work.

---

## 11. Other known gaps

- **~5,850 lines across 11 modules sit outside the compilation graph** — never declared in any `mod.rs`. Includes `crypto_machine.rs`, a documented vodozemac (audited-Olm) migration. The repo ships a hand-rolled ratchet while an audited replacement sits uncompiled.
- **`core_wiring.rs`** — 367 lines of which **356 are comments**. It reads as integration wiring but is an instruction document. Actively misleading.
- **`src/tui.pre-20260817T223241Z/`** — a timestamped copy of `src/tui/` still inside the module tree.
- **561 `unwrap()`**, 29 `expect()`, 10 `panic!()` (111/24/3 in non-test code), with `panic = "abort"` in release. 156 `println!` despite the "never println!" convention. `[lints.clippy]` has 5 entries, all `cast_*` at warn — no `unwrap_used`.
- **`ROADMAP.md` claims ~60% coverage** and is three months stale. `AGENTS.md:74-78` build status likewise.
- **CI was reporting green on a red suite** — `continue-on-error: true` on the test job. **Fixed 2026-09-28** (removed from tests; deliberately retained on coverage with a written rationale).

---

## Appendix — reproducing this file

```bash
cd ghostwire
cargo check --lib --bins                              # expect: Finished, 0 errors
GHOSTWIRE_HOME=$(mktemp -d) cargo test --lib          # expect: 368 passed, 0 failed
find src -name '*.rs' | wc -l                         # 151
find src -name '*.rs' -exec cat {} + | wc -l          # 58166
grep -rn 'validate_node_id\|enforce_ip_diversity' --include=*.rs src/   # doc comments only
grep -c 'LoRa' src/core/peer_table.rs                 # 0 — no such variant
```

**Regenerate this spine whenever the repo tip moves.** A status file that silently rots is worse than no status file.
