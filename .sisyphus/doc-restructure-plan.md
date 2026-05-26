# GhostWire Docs Restructure Plan

> Generated: May 26, 2026
> Status: Directories created, GETTING_STARTED.md written, docs/README.md updated
> File moves: ❌ NOT YET EXECUTED (commands provided below)

---

## 1. New Directory Structure (Under `docs/`)

```
docs/
├── README.md                      # Main hub — UPDATED
├── GETTING_STARTED.md             # NEW — merged from INSTALL + QUICK_START + HOW_TO_START
│
├── spec/                          # EXISTING — 10 GIP protocol specs (no changes)
│   ├── PROTOCOL.md
│   ├── WIRE_FORMAT.md
│   ├── CRYPTO.md
│   ├── SPHINX.md
│   ├── DISCOVERY.md
│   ├── ROUTING.md
│   ├── DTN.md
│   ├── IDENTITY.md
│   ├── SOCIAL.md
│   └── TRANSPORT.md
│
├── architecture/                  # NEW FOLDER
│   ├── ARCHITECTURE.md
│   ├── RESEARCH.md                (was ARCHITECTURE_RESEARCH.md)
│   ├── CLI.md
│   ├── TUI.md
│   └── WEB_UI.md
│
├── security/                      # NEW FOLDER
│   ├── SECURITY.md
│   ├── SECURITY_AUDIT.md
│   ├── AUDIT_SUMMARY.md
│   ├── SECURITY_AUDIT_REPORT_2026.md
│   └── API_SECURITY.md
│
├── deployment/                    # NEW FOLDER
│   ├── BACKEND_COMMUNICATION_TEST.md
│   ├── ML_PIPELINE.md
│   └── INSTALLER_ENHANCEMENTS.md
│
├── api/                           # EXISTING folder — expanded
│   ├── DEEP_DIVE.md               (was api/DEEP_DIVE.md)
│   ├── API_TESTING_GUIDE.md
│   ├── API_QUICK_REFERENCE.md
│   ├── API_VERSIONING.md
│   ├── POSTMAN_API_GUIDE.md
│   └── BACKEND_COMMUNICATION_TEST.md
│
├── development/                   # NEW FOLDER — developer workflow
│   ├── DEVELOPMENT.md
│   ├── TESTING.md
│   ├── CONTRIBUTING.md
│   ├── GOVERNANCE.md
│   ├── CLA.md
│   ├── BRAND_GUIDELINES.md
│   ├── BRANDING_INTEGRATION.md
│   ├── LIBP2P_NETWORK_CONNECTIVITY.md
│   ├── VIBE_SETUP_GUIDE.md
│   ├── TRANSCRIPTION_SETUP.md
│   ├── TRANSCRIPTION_READY.md
│   ├── AUTOMATIC_UPDATES.md
│   └── build/                     (moved from docs/build/)
│       ├── OVERVIEW.md
│       ├── QUICKSTART.md
│       ├── EMBEDDING.md
│       ├── CUSTOM_TRANSPORT.md
│       ├── PROTOCOL_BRIDGES.md
│       └── API_REFERENCE.md
│
├── knowledge/                     # NEW FOLDER — deep dives, refs, wiki
│   ├── CHANGELOG.md
│   ├── FIRST_PING.md
│   ├── COMPREHENSIVE_CODE_REVIEW.md
│   ├── GHOSTWIRE_MANIFESTO.md
│   ├── MANIFESTO_FRAMEWORK.md     (was ghostwire-manifesto-framework.md)
│   ├── GLOBAL_KNOWLEDGE_SOURCES.md
│   ├── HARDWARE_SPEC_BOM.md
│   ├── IMPLEMENTATION_GUIDE.md
│   ├── RECOMMENDATIONS.md
│   ├── REVIEW_SUMMARY.md
│   ├── wiki/                      (moved from docs/wiki/)
│   │   ├── index.md
│   │   ├── adapters.md
│   │   ├── architecture.md
│   │   ├── contributing.md
│   │   ├── faq.md
│   │   ├── getting-started.md
│   │   ├── security.md
│   │   └── transports.md
│   └── protocol-knowledge/        (moved from docs/protocol-knowledge/)
│       ├── 30_DAY_LIBP2P_CURRICULUM.md
│       └── PROTOCOL_DEEP_DIVE.md
│
├── presentations/                 # EXISTING — 14 slide decks + PDFs (no changes)
│
├── legacy/                        # EXISTING — 6 historical docs (no changes)
│
├── archive/                       # EXISTING — destination for:
│   ├── INSTALL.md                 (merged into GETTING_STARTED)
│   ├── QUICK_START.md             (merged into GETTING_STARTED)
│   ├── HOW_TO_START_GHOSTWIRE.md  (merged into GETTING_STARTED)
│   └── notebookllm-copies/        (see §3)
│
└── (static files — leave in place)
    ├── _astro/
    ├── assets/
    ├── screenshots/
    ├── markdown-page/
    ├── index.html
    └── favicon.svg
```

---

## 2. File Move Commands

All commands run from project root (`GhostWire-secure-mesh-communication/`).

### architecture/

```bash
mv docs/ARCHITECTURE.md   docs/architecture/ARCHITECTURE.md
mv docs/ARCHITECTURE_RESEARCH.md  docs/architecture/RESEARCH.md
mv docs/CLI.md            docs/architecture/CLI.md
mv docs/TUI.md            docs/architecture/TUI.md
mv docs/WEB_UI.md         docs/architecture/WEB_UI.md
```

### security/

```bash
mv docs/SECURITY.md                 docs/security/SECURITY.md
mv docs/SECURITY_AUDIT.md           docs/security/SECURITY_AUDIT.md
mv docs/AUDIT_SUMMARY.md            docs/security/AUDIT_SUMMARY.md
mv docs/SECURITY_AUDIT_REPORT_2026.md  docs/security/SECURITY_AUDIT_REPORT_2026.md
mv docs/API_SECURITY.md             docs/security/API_SECURITY.md
```

### api/

```bash
mv docs/API_TESTING_GUIDE.md        docs/api/API_TESTING_GUIDE.md
mv docs/API_QUICK_REFERENCE.md      docs/api/API_QUICK_REFERENCE.md
mv docs/API_VERSIONING.md           docs/api/API_VERSIONING.md
mv docs/POSTMAN_API_GUIDE.md        docs/api/POSTMAN_API_GUIDE.md
mv docs/BACKEND_COMMUNICATION_TEST.md  docs/api/BACKEND_COMMUNICATION_TEST.md
# Note: api/DEEP_DIVE.md stays in place
```

### deployment/

```bash
mv docs/ML_PIPELINE.md              docs/deployment/ML_PIPELINE.md
mv docs/INSTALLER_ENHANCEMENTS.md   docs/deployment/INSTALLER_ENHANCEMENTS.md
# BACKEND_COMMUNICATION_TEST.md also goes to api/ (linked from both)
```

### development/

```bash
mv docs/DEVELOPMENT.md              docs/development/DEVELOPMENT.md
mv docs/TESTING.md                  docs/development/TESTING.md
mv docs/CONTRIBUTING.md             docs/development/CONTRIBUTING.md
mv docs/GOVERNANCE.md               docs/development/GOVERNANCE.md
mv docs/CLA.md                      docs/development/CLA.md
mv docs/BRAND_GUIDELINES.md         docs/development/BRAND_GUIDELINES.md
mv docs/BRANDING_INTEGRATION.md     docs/development/BRANDING_INTEGRATION.md
mv docs/LIBP2P_NETWORK_CONNECTIVITY.md  docs/development/LIBP2P_NETWORK_CONNECTIVITY.md
mv docs/VIBE_SETUP_GUIDE.md         docs/development/VIBE_SETUP_GUIDE.md
mv docs/TRANSCRIPTION_SETUP.md      docs/development/TRANSCRIPTION_SETUP.md
mv docs/TRANSCRIPTION_READY.md      docs/development/TRANSCRIPTION_READY.md
mv docs/AUTOMATIC_UPDATES.md        docs/development/AUTOMATIC_UPDATES.md
mv docs/build/                      docs/development/build/
```

### knowledge/

```bash
mv docs/CHANGELOG.md                docs/knowledge/CHANGELOG.md
mv docs/FIRST_PING.md               docs/knowledge/FIRST_PING.md
mv docs/COMPREHENSIVE_CODE_REVIEW.md  docs/knowledge/COMPREHENSIVE_CODE_REVIEW.md
mv docs/GHOSTWIRE_MANIFESTO.md      docs/knowledge/GHOSTWIRE_MANIFESTO.md
mv docs/ghostwire-manifesto-framework.md  docs/knowledge/MANIFESTO_FRAMEWORK.md
mv docs/GLOBAL_KNOWLEDGE_SOURCES.md  docs/knowledge/GLOBAL_KNOWLEDGE_SOURCES.md
mv docs/HARDWARE_SPEC_BOM.md        docs/knowledge/HARDWARE_SPEC_BOM.md
mv docs/IMPLEMENTATION_GUIDE.md     docs/knowledge/IMPLEMENTATION_GUIDE.md
mv docs/RECOMMENDATIONS.md          docs/knowledge/RECOMMENDATIONS.md
mv docs/REVIEW_SUMMARY.md           docs/knowledge/REVIEW_SUMMARY.md
mv docs/wiki/                       docs/knowledge/wiki/
mv docs/protocol-knowledge/         docs/knowledge/protocol-knowledge/
```

### archive/ (merged getting-started files)

```bash
mv docs/INSTALL.md                  docs/archive/INSTALL.md
mv docs/QUICK_START.md              docs/archive/QUICK_START.md
mv docs/HOW_TO_START_GHOSTWIRE.md   docs/archive/HOW_TO_START_GHOSTWIRE.md
```

---

## 3. notebookllm/ Duplicate Management

### Copied files (remove from notebookllm/, keep canonical version only)

| notebookllm/ file | Canonical location |
|------------------|-------------------|
| `API.md` | `ghostwire/docs/API.md` |
| `CHANGELOG.md` | `docs/knowledge/CHANGELOG.md` |
| `CONTRIBUTING.md` | `docs/development/CONTRIBUTING.md` |
| `GHOSTWIRE_MANUAL.md` | `ghostwire/docs/GHOSTWIRE_MANUAL.md` |
| `README.md` | `ghostwire/README.md` |
| `ROADMAP.md` | (root) `ROADMAP.md` |
| `SECURITY.md` | `docs/security/SECURITY.md` |
| `SECURITY_AUDIT_RESOLVED.md` | `docs/security/SECURITY_AUDIT.md` |

### Unique notebookllm/ files (keep — no canonical version elsewhere)

| File | Suggested destination |
|------|---------------------|
| `GhostWire_Codebase.md` | `docs/knowledge/wiki/GhostWire_Codebase.md` (7096 lines, LLM-optimized) |
| `GhostWire_Ops_Tests.md` | `docs/knowledge/GhostWire_Ops_Tests.md` |
| `Troubleshooting_Guide.md` | `docs/knowledge/Troubleshooting_Guide.md` |
| `Testing_Guide.md` | `docs/development/Testing_Guide.md` |
| `Deployment_Guide.md` | `docs/deployment/Deployment_Guide.md` |
| `Performance_Benchmarks.md` | `docs/knowledge/Performance_Benchmarks.md` |
| `Cryptography_Implementation.md` | `docs/security/Cryptography_Implementation.md` |
| `libp2p_v0.56_Migration_Guide.md` | `docs/knowledge/libp2p_Migration_Guide.md` |
| `Tauri_Mobile_App_Guide.md` | `docs/development/Tauri_Mobile_Guide.md` |
| `Competitor_Analysis_Report.md` | `docs/knowledge/Competitor_Analysis.md` |
| `GhostWire_Architecture_Audit.md` | `docs/architecture/GhostWire_Architecture_Audit.md` |
| `GhostWire_Deep_Audit_Report.md` | `docs/knowledge/GhostWire_Deep_Audit.md` |
| `GhostWire_System_Overview.md` | `docs/knowledge/GhostWire_System_Overview.md` |
| `GhostWire_Scalability_Roadmap.md` | `docs/knowledge/GhostWire_Scalability.md` |
| `GhostWire_Future_Scopes.md` | `docs/knowledge/GhostWire_Future_Scopes.md` |
| `GhostWire_Multimodal_Uniqueness.md` | `docs/knowledge/GhostWire_Multimodal.md` |
| `GhostWire_Evolution_Diagram.md` | `docs/knowledge/GhostWire_Evolution.md` |
| `GhostWire_Big_Bang_Deep_Dive.md` | `docs/knowledge/GhostWire_Big_Bang.md` |
| `Implementation_Plan.md` | `docs/knowledge/Implementation_Plan.md` |
| `Project_Implementation_Plan.md` | `docs/knowledge/Project_Implementation_Plan.md` |
| `Project_Task_List.md` | `docs/development/Project_Task_List.md` |
| `Project_Walkthrough.md` | `docs/knowledge/Project_Walkthrough.md` |
| `TASK_TRACKING.md` | `docs/development/TASK_TRACKING.md` |
| `UPDATE_SUMMARY_FEB_2026.md` | `docs/knowledge/UPDATE_SUMMARY_FEB_2026.md` |
| `February_2026_Changes.md` | `docs/knowledge/February_2026_Changes.md` |
| `DOCUMENTATION_INDEX.md` | `docs/knowledge/DOCUMENTATION_INDEX.md` |
| `CURATED_READING_ORDER.md` | `docs/knowledge/CURATED_READING_ORDER.md` |
| `AUDIO_OVERVIEW_GUIDE.md` | `docs/knowledge/AUDIO_OVERVIEW_GUIDE.md` |
| `AUDIO_OVERVIEW_SCRIPT.md` | `docs/knowledge/AUDIO_OVERVIEW_SCRIPT.md` |

### Recommended Strategy

**Option: Keep notebookllm/ as an LLM-context directory with a note.**

Since notebookllm/ serves a legitimate purpose (LLM context injection), the cleanest solution is:

1. Add a `README.md` to `notebookllm/` explaining it's an LLM context directory
2. Move only the truly unique new content into `docs/` proper structure
3. Keep the copies in notebookllm/ for AI tools but flag them as derived
4. Add CI check: `diff` between canonical docs and notebookllm copies warns on drift

---

## 4. Broken References After Move

### Files that reference moved docs (must be updated)

| Source File | Broken Link | Fix |
|------------|-------------|-----|
| `docs/README.md` | `ARCHITECTURE.md` | → `architecture/ARCHITECTURE.md` ✅ Done |
| `docs/README.md` | `INSTALL.md` | → `GETTING_STARTED.md` ✅ Done |
| `docs/README.md` | `QUICK_START.md` | → `GETTING_STARTED.md` ✅ Done |
| `docs/README.md` | `HOW_TO_START_GHOSTWIRE.md` | → `GETTING_STARTED.md` ✅ Done |
| `docs/README.md` | `build/*` | → `development/build/*` ✅ Done |
| `docs/HOW_TO_START_GHOSTWIRE.md` | `POSTMAN_API_GUIDE.md` | → `api/POSTMAN_API_GUIDE.md` (will archive this file) |
| `docs/HOW_TO_START_GHOSTWIRE.md` | `Deployment_Guide.md` | → `deployment/Deployment_Guide.md` |
| `docs/HOW_TO_START_GHOSTWIRE.md` | `Troubleshooting_Guide.md` (in notebookllm) | → `knowledge/Troubleshooting_Guide.md` |
| `docs/ARCHITECTURE.md` | Internal refs to components | Check links to sections |
| `docs/CLI.md` | Reference to `docs/TESTING.md` etc. | Update if referencing moved files |
| Root `README.md` | `docs/INSTALL.md`, `docs/QUICK_START.md` | → `docs/GETTING_STARTED.md` |
| `docs/COMPREHENSIVE_CODE_REVIEW.md` | Internal refs to other docs | Check links |
| `docs/review_summary.md` | Internal refs | Check links |

### Cross-references that will break

| Source | Broken Ref | Impact |
|--------|-----------|--------|
| `docs/ARCHITECTURE.md` → `docs/spec/CRYPTO.md` | OK — spec/ stays in place | None |
| `docs/ARCHITECTURE.md` → `docs/spec/IDENTITY.md` | OK — spec/ stays in place | None |
| `docs/DEVELOPMENT.md` → `docs/TESTING.md` | Will break → `development/TESTING.md` | Medium |
| `docs/TESTING.md` → `docs/DEVELOPMENT.md` | Will break → `development/DEVELOPMENT.md` | Medium |
| `docs/GOVERNANCE.md` → `docs/spec/*` | OK — spec/ stays | None |
| `docs/API_SECURITY.md` → `docs/SECURITY.md` | Will break → `security/SECURITY.md` | Low |
| Root `ROADMAP.md` → `docs/ARCHITECTURE.md` | Will break → `docs/architecture/ARCHITECTURE.md` | Low |
| Root `README.md` → `docs/CLI.md` | Will break → `docs/architecture/CLI.md` | Low |
| Root `README.md` → `docs/TUI.md` | Will break → `docs/architecture/TUI.md` | Low |
| `docs/presentations/*.md` → `docs/ARCHITECTURE.md` etc. | Will break for moved files | Low |
| `docs/GHOSTWIRE_MANIFESTO.md` → `docs/SECURITY.md` | Will break → `security/SECURITY.md` | Low |

### Fixing cross-references

The approach is read each moved file and update internal links. Use:

```bash
# Find all internal doc links in files being moved
grep -rn '\.\.\/docs\|\.\.\/[A-Z]' docs/architecture/ docs/security/ docs/api/ docs/development/ docs/knowledge/ | grep -v node_modules
```

---

## 5. Documentation Strategy

### Guiding Principles

1. **One source of truth.** Every piece of content lives in exactly one canonical location. No copies.
2. **Notebookllm/ is an exception** — it's an AI-context directory, not user-facing docs. Keep it, label it, but don't let it pollute the canonical structure.
3. **Flat over nested — but not too flat.** The 10-folder structure is the right depth. Avoid going deeper than 2 levels.
4. **Navigation follows audience.** New users → GETTING_STARTED. Protocol devs → spec/. App devs → development/build/. Ops → deployment/. Everyone → knowledge/.

### Maintenance Rules

| Rule | Description |
|------|-------------|
| **New doc → audit first** | Before adding a doc, check if content exists somewhere |
| **No single-file drift** | If `notebookllm/` copies drift → CI diff check |
| **Ports always 9097** | API examples MUST use 9097, never 8080 |
| **GIP specs = authoritative** | spec/ files define protocol; architecture/ describes implementation |
| **Knowledge/ is append-only** | Never delete from knowledge/ — move to archive/ instead |
| **Archive/ is the graveyard** | Merged, replaced, or stale files go here and stay forever |

### Recommended Future Investments

| Priority | Item | Rationale |
|----------|------|-----------|
| **P0** | Production deployment guide | systemd, Docker Compose, reverse proxy, monitoring |
| **P0** | Troubleshooting FAQ | FAQ exists in wiki/ but should be expanded |
| **P1** | CI pipeline for docs | Link checker + stale content detection + notebookllm sync |
| **P1** | GIP status badges | Each spec file needs Status: Draft/Review/Final |
| **P2** | Performance baseline doc | Message latency, throughput under load, memory profile |
| **P2** | CLI reference | All subcommands, flags, exit codes in one place |
| **P2** | TUI screenshots | Visual reference for TUI documentation |
| **P3** | Docusaurus/mdBook migration | Better navigation, search, versioning |
