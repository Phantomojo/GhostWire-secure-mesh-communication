# notebookllm/ Directory Assessment

**Date:** 2026-07-03
**Files:** 37 markdown files
**Origin:** AI-generated documentation batch, August 2025 + February 2026 updates

---

## Verdict

**Archive the entire directory.** Move it to `docs/archive/notebookllm/` and stop maintaining it.

---

## Why

### 1. Massive Duplication with docs/

The top-level `docs/` directory (18 files) already covers the same ground with more accurate, manually maintained content:

| notebookllm file | docs/ equivalent | Which is better? |
|-----------------|-----------------|------------------|
| `SECURITY.md` | `docs/SECURITY.md` | docs/ (has THREAT_MODEL.md too) |
| `API.md` | `docs/API.md` | docs/ |
| `CONTRIBUTING.md` | `docs/CONTRIBUTING.md` | docs/ |
| `ROADMAP.md` | `docs/ROADMAP.md` | docs/ |
| `CHANGELOG.md` | `CHANGELOG.md` (root) | root |
| `GHOSTWIRE_MANUAL.md` | `docs/GHOSTWIRE_MANUAL.md` | docs/ |
| `Deployment_Guide.md` | `docs/DEPLOYMENT.md` | docs/ |
| `Testing_Guide.md` | `docs/` (implied) | docs/ |
| `Troubleshooting_Guide.md` | `docs/TROUBLESHOOTING.md` | docs/ |

That's 9 direct duplicates. Half the directory is copies of files that exist better-maintained elsewhere.

### 2. Outdated Content

The `DOCUMENTATION_INDEX.md` says "Last Updated: February 16, 2026" and references libp2p v0.56. The codebase has moved past that. Key indicators:

- `GhostWire_Codebase.md` is a 7,097-line code dump that includes old module paths (e.g., `adapter_registry.rs` with a `ProtocolAdapter` trait that doesn't exist in the current codebase). The current `Core` struct in `core/mod.rs` has fields and modules this doc doesn't mention.
- `Cryptography_Implementation.md` describes the crypto system accurately but at a high level. The new `docs/DESIGN_DECISIONS.md` covers the same ground with more depth and real code references.
- `README.md` in notebookllm describes "AES-256-GCM" as the primary cipher. The actual codebase uses XChaCha20-Poly1305 for all message encryption.

### 3. LLM-Generated Fluff

Several files are speculative or aspirational, not reflecting the actual codebase state:

- `GhostWire_Future_Scopes.md` — Lists features that don't exist yet (satellite mesh, quantum key distribution). Useful as a brainstorm, not as documentation.
- `GhostWire_Scalability_Roadmap.md` — Theoretical scaling strategies. No code backs this up.
- `Project_Walkthrough.md` — Describes an idealized architecture that doesn't match the current module structure.
- `Competitor_Analysis_Report.md` — Market analysis, not technical documentation.

### 4. What's Actually Unique (3 files worth keeping)

Only three files in notebookllm contain content not found elsewhere:

| File | Unique Content | Recommendation |
|------|---------------|----------------|
| `Tauri_Mobile_App_Guide.md` | Android app architecture, Tauri 2.0 setup, Kotlin background service | Move to `docs/MOBILE.md` |
| `Performance_Benchmarks.md` | Specific benchmark numbers (v0.53 vs v0.56), test methodology | Move to `docs/BENCHMARKS.md` |
| `GhostWire_Evolution_Diagram.md` | Timeline visualization of project phases | Move to `docs/EVOLUTION.md` |

These three are worth preserving. The rest can be archived or deleted.

### 5. Maintenance Burden

The `DOCUMENTATION_INDEX.md` defines a quarterly review cycle and a 5-step update process. Nobody is doing this. The index itself is stale (references a deprecated `GhostWire_Interface_TUI.md` marked as removed). Maintaining 37 files that duplicate 18 other files is wasted effort.

---

## Recommended Action

```
1. Create docs/archive/notebookllm/
2. Move the entire notebookllm/ directory there
3. Extract these 3 files to docs/ before archiving:
   - Tauri_Mobile_App_Guide.md → docs/MOBILE.md
   - Performance_Benchmarks.md → docs/BENCHMARKS.md
   - GhostWire_Evolution_Diagram.md → docs/EVOLUTION.md
4. Update docs/DOCUMENTATION_INDEX.md to point to the new locations
5. Delete notebookllm/ after extraction
```

**Do not consolidate.** The unique content from the 3 files above is small enough to move manually. Merging 37 files into docs/ would just bloat the already-maintained documentation set.

---

## Files to Delete (no unique value)

- `API.md` — duplicates `docs/API.md`
- `AUDIO_OVERVIEW_GUIDE.md` — meta-documentation about audio overviews
- `AUDIO_OVERVIEW_SCRIPT.md` — script for generating audio overviews
- `CHANGELOG.md` — duplicates root `CHANGELOG.md`
- `Competitor_Analysis_Report.md` — market analysis, not technical docs
- `CONTRIBUTING.md` — duplicates `docs/CONTRIBUTING.md`
- `Cryptography_Implementation.md` — superseded by `docs/DESIGN_DECISIONS.md`
- `CURATED_READING_ORDER.md` — meta-index, not content
- `Deployment_Guide.md` — duplicates `docs/DEPLOYMENT.md`
- `DOCUMENTATION_INDEX.md` — stale index of stale docs
- `February_2026_Changes.md` — changelog fragment
- `GhostWire_Architecture_Audit.md` — duplicates `docs/THREAT_MODEL.md`
- `GhostWire_Big_Bang_Deep_Dive.md` — narrative history, not reference
- `GhostWire_Codebase.md` — 7K-line code dump, outdated
- `GhostWire_Deep_Audit_Report.md` — audit narrative
- `GhostWire_Future_Scopes.md` — speculative, no code backing
- `GhostWire_Multimodal_Uniqueness.md` — transport overview, better covered in DESIGN_DECISIONS.md
- `GhostWire_Ops_Tests.md` — duplicates `docs/` testing guidance
- `GhostWire_Scalability_Roadmap.md` — theoretical, no implementation
- `GhostWire_System_Overview.md` — duplicates `docs/ARCHITECTURE.md`
- `Implementation_Plan.md` — project planning, not docs
- `libp2p_v0.56_Migration_Guide.md` — migration already complete
- `Project_Implementation_Plan.md` — duplicate of Implementation_Plan.md
- `Project_Task_List.md` — task tracking, not docs
- `Project_Walkthrough.md` — outdated narrative
- `README.md` — outdated, incorrect cipher reference
- `ROADMAP.md` — duplicates `docs/ROADMAP.md`
- `SECURITY_AUDIT_RESOLVED.md` — resolved audit, historical only
- `SECURITY.md` — duplicates `docs/SECURITY.md`
- `TASK_TRACKING.md` — task tracking, not docs
- `Testing_Guide.md` — duplicates `docs/` guidance
- `Troubleshooting_Guide.md` — duplicates `docs/TROUBLESHOOTING.md`
- `UPDATE_SUMMARY_FEB_2026.md` — changelog fragment
- `GHOSTWIRE_MANUAL.md` — duplicates `docs/GHOSTWIRE_MANUAL.md`

---

*Assessment based on reading 37 notebookllm files + 18 docs/ files + Cargo.toml + core source modules.*
