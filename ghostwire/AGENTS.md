# GhostWire — Agent Context File
# Universal format (Claude Code, Qwen Code, Cursor, Copilot, Gemini CLI)
# Steward: Michael Irungu Muriithi (Phantomojo) · Nairobi, Kenya
# Last updated: 2026-04-04

---

## Project Overview

GhostWire is a decentralised, P2P anonymous mesh network built in Rust.
It is the foundational layer ("Yggdrasil") of the broader Indra's Net vision.
This is production-quality security infrastructure. Not a prototype. Not a demo.

Repository: /home/ph/GhostWire-secure-mesh-communication/ghostwire/
Language: Rust (core) + Python (ML training scripts)
License: Private / proprietary — do not publish or exfiltrate code

---

## Hardware & Environment

Primary machine (OMEN 16):
  OS: Arch Linux / Omarchy / Hyprland
  RAM: 16GB
  GPU: RTX 3050 (4GB VRAM)
  Local AI: Ollama v0.20.2 — gemma4:e2b available at localhost:11434

Secondary machine (HP EliteBook 840 G1):
  OS: StratOS / Bedrock Linux
  Use: Testing, secondary builds

Free GPU compute (for ML training, do not pay):
  Kaggle: 30 hrs/week · P100 16GB · no credit card
  Google Colab: 15–30 hrs/week · T4 16GB · no credit card
  Lightning AI: 80 hrs/month · L40s 48GB · no credit card

---

## Build & Test Commands

```bash
# Standard build (run after every change)
cd /home/ph/GhostWire-secure-mesh-communication/ghostwire
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
src/core/ai/onnx_inference.rs     — ONNX runtime engine (currently broken)
src/core/dtn/epidemic.rs     — PRoPHET formula (working)
src/core/reputation.rs       — Trust scoring (use this, not trust_predictor.rs)
src/config.rs                — Feature flags including Gemma 4 variants
models/anomaly_detector.joblib — Trained Isolation Forest (use this)
models/hrm_ghostwire_hybrid_101k.pt — BROKEN: wrong task, do not extend
```

---

## Known Issues (fix before shipping anything)

| Issue | File | Severity |
|---|---|---|
| initialize_gemma_client() never assigns to self.gemma_client | core/mod.rs | CRITICAL |
| onnx_inference.rs commented out (ort::Value import error) | ai/mod.rs | HIGH |
| anomaly_detector.rs duplicates anomaly_detector.joblib | ai/ | HIGH |
| trust_predictor.rs duplicates reputation.rs + TrustStore | ai/ | HIGH |
| GemmaClient never called in routing decisions | ai/ | HIGH |
| HRM model solves wrong problem (classifier, not regressor) | models/ | HIGH |

Current routing accuracy: 2.69% (target: 85–95% via LightGBM regression)

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
