# GhostWire — Agent Context File
# Universal format (Claude Code, Qwen Code, Cursor, Copilot, Gemini CLI)
# Steward: Michael Irungu Muriithi (Phantomojo) · Nairobi, Kenya
# Last updated: 2026-06-24

---

## Project Overview

GhostWire is a decentralised, P2P anonymous mesh network built in Rust.
It is the foundational layer ("Yggdrasil") of the broader Indra's Net vision.
This is production-quality security infrastructure. Not a prototype. Not a demo.

Repository: ghostwire/
Language: Rust (core) + Python (ML training scripts)
License: Private / proprietary — do not publish or exfiltrate code

---

## Hardware & Environment

Primary machine:
  OS: Linux
  RAM: 16GB
  GPU: NVIDIA (4GB VRAM)
  Local AI: Ollama — gemma4:e2b available at localhost:11434

Secondary machine:
  OS: Linux
  Use: Testing, secondary builds

Free GPU compute (for ML training, do not pay):
  Kaggle: 30 hrs/week · P100 16GB · no credit card
  Google Colab: 15–30 hrs/week · T4 16GB · no credit card
  Lightning AI: 80 hrs/month · L40s 48GB · no credit card

---

## Build & Test Commands

```bash
# Standard build (run after every change)
cd ghostwire
cargo build 2>&1 | head -60

# Check only (faster, no linking)
cargo check 2>&1 | head -40

# Lint (run before declaring anything done)
cargo clippy -- -D warnings 2>&1 | head -40

# Run all tests
cargo test 2>&1 | tail -30

# Run specific module tests
cargo test ai::gemma_client::tests
cargo test ai::routing_classifier::tests

# Audit dependencies for CVEs
cargo audit

# Check for unused dependencies
cargo +nightly udeps

# Release build (only when explicitly requested)
cargo build --release
```

**Verification gate**: Nothing is "done" until `cargo build` passes clean.
If it doesn't compile, it isn't fixed.

## Build Status (2026-06-24)

- **Compile**: 0 errors (was 25 before Phase 2)
- **Tests**: 203/203 pass, 2 pre-existing failures (identity storage + social graph), 2 network-hang skips
- **Binary**: 254MB (debug), builds in ~6 minutes
- **Warnings**: 68 (all pre-existing, unused imports/variables)
- **Feature flags**: `bluetooth` disabled by default (ble_controller not implemented)

---

## Architecture — Four-Layer AI Routing System

```
Layer 1  Fast Path        LightGBM/ONNX      <1ms       every routing decision
Layer 2  Analytical       Gemma 4 (Ollama)   50-200ms   every 30 seconds
Layer 3  Graph Opt.       GNN (PyTorch Geo)  offline    every 5 minutes
Layer 4  Fallback         PRoPHET formula    <0.01ms    always available

Security  Isolation Forest → anomaly score → Gemma 4 threat analysis (if anomaly)
```

PRoPHET (Layer 4) is already working in `src/core/dtn/epidemic.rs`.
Do NOT reimplement it. Reference it.

---

## Key File Paths

```
src/core/mod.rs              — Core struct, module declarations
src/core/ai/mod.rs           — AI module definition
src/core/ai/gemma_client.rs  — Gemma 4 Ollama HTTP client
src/core/ai/routing_classifier.rs — ML routing classifier
src/core/ai/onnx_inference.rs     — ONNX runtime engine (working, ort 2.0)
src/core/dtn/epidemic.rs     — PRoPHET formula (working)
src/core/reputation.rs       — Trust scoring (trust_predictor.rs deleted, this is canonical)
src/config.rs                — Feature flags including Gemma 4 variants
models/anomaly_detector.joblib — Trained Isolation Forest for Python training
models/l1_anomaly_detector.onnx — ONNX Isolation Forest for Rust runtime inference
```

---

## Known Issues (fix before shipping anything)

| Issue | File | Severity | Status |
|---|---|---|---|
| ML routing classifier not wired into actual packet routing | routing/ | HIGH | Open — Layer 1 LightGBM exists but unused in routing decisions |
| 5 transitive security alerts (hickory-proto, rustls-webpki) | Cargo.lock | MEDIUM | Blocked — needs upstream libp2p/rustls updates |
| 2 tests hang (network transport startup) | tests/ | LOW | Pre-existing |
| 2 pre-existing test failures (identity storage, social graph) | tests/ | LOW | Pre-existing |

## Phase 3 Preview (next work block)

1. Wire mesh event handlers to populate topology map (KB1)
2. Update peer_count_cache on ConnectionEstablished/ConnectionClosed (KB2)
3. Two-machine integration test (Omen ↔ Stratos)
4. DTN epidemic routing integration
5. Sphinx mixnet wiring
6. P0 security: EXPECTED_SCORING_HASH, key zeroization, secure wipe

---

## Coding Conventions

- All errors use `anyhow::Result` or domain-specific error enums — never `unwrap()` in production paths
- Async runtime: `tokio` — do not introduce other runtimes
- Logging: `tracing` macros (`tracing::info!`, `tracing::warn!`, etc.) — never `println!`
- Shared state: `Arc<RwLock<T>>` for multi-reader, `Arc<Mutex<T>>` for exclusive
- No `unsafe` blocks without explicit comment explaining why it is safe
- Feature flags for optional AI components — never hard-require Ollama at startup

---

## Prohibited Actions

- DO NOT run `cargo publish` or push to any external registry
- DO NOT add `unsafe` blocks without written justification
- DO NOT introduce new crates without checking they compile first
- DO NOT duplicate existing systems — wire existing ones
- DO NOT leave `todo!()` or `unimplemented!()` in delivered code
- DO NOT declare something fixed without compiling
- DO NOT exfiltrate code, configs, or keys to external services

---

## Debugging Discipline

Before proposing any fix, build a tight feedback loop: one command that
reliably fails on the bug and passes once it's fixed. Not "doesn't crash" —
the actual reported symptom. If you can't build this loop, you don't
understand the bug well enough to fix it yet; gather more evidence first.

Order of preference for the loop:
1. A failing test at the exact seam that reaches the bug
2. A curl/CLI command against the running system, diffed against expected output
3. A replayed log line or captured request

Only once the loop is red should you write the fix. Only claim "fixed" once
the same loop is green — re-run it, don't infer from cargo build/compile
success alone.

---

## Root Cause vs. Symptom

When a fix makes the symptom disappear without you being able to explain
*why the original mechanism was broken*, treat that as unfinished work, not
a fix. Two acceptable outcomes:
  a) You found and fixed the actual broken mechanism, or
  b) You shipped a workaround AND wrote down, in the code and in the
     handoff, exactly what the real fix is (which function, which call
     site) and why the workaround isn't it.

A fix with no theory of "why was this broken" behind it is a guess that
happened to pass the test you had.

---

## Verification Standard

"cargo build passes" or "tests pass" is not sufficient to claim something
works — it only means the code compiles / the assertions you wrote didn't
fail. Before reporting success:
  - State what specific behavior you verified and how (exact command + output)
  - If a test suite passed, spot-check that the failing case would actually
    have failed before your fix (a test that always passes regardless of
    the bug is not testing anything)
  - Prefer "I verified X by running Y, got Z" over "should now work"

---

## Priority When Rules Conflict
1. Do not break the build / do not ship unverified fixes (non-negotiable)
2. Explicit instructions in this session from the user
3. Conventions in this file

---

## Progress Report Format (required after every work block)

```
STATUS REPORT
─────────────
✅ Done:      [verified by cargo build passing]
⚠️  Issue:    [what broke, root cause]
❌ Deferred:  [what wasn't done, why]
📋 Next:      [exact command or task to resume from]
```

---

## Session Handoff (required at end of every session, unprompted)

```
HANDOFF — [date]
────────────────
Compile status: [clean / N errors in which modules]
Files modified: [list]
Files needing attention: [list + reason]
Resume from: [exact next action]
```

---
## Embedded GhostWire policy layer

Before architectural, security, transport, identity, API, UI, storage, or intelligence work, read `docs/policy/PROJECT_DOCTRINE.md`, `docs/policy/FUTURE_ML_POLICY.md`, and `docs/policy/AGENT_HANDOFF_TEMPLATE.md`. These are repository-local normative policies. `.ai/memory.md` records the current architectural state and exact continuation point.
