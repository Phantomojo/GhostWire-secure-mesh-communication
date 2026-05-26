# GhostWire — Comprehensive Documentation Audit

> **Generated:** May 26, 2026
> **Scope:** 193 project-authored Markdown files across the entire monorepo
> **Purpose:** Provide a complete inventory, quality assessment, and actionable recommendations

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Inventory Breakdown](#2-inventory-breakdown)
3. [Category-by-Category Assessment](#3-category-by-category-assessment)
4. [Content Quality Assessment](#4-content-quality-assessment)
5. [Duplication Map](#5-duplication-map)
6. [Content Gaps](#6-content-gaps)
7. [Recommendations](#7-recommendations)
8. [Appendix: Full File Listing](#8-appendix-full-file-listing)

---

## 1. Executive Summary

GhostWire has **193 project-authored Markdown files** totaling approximately 40,000+ lines of documentation. The documentation ecosystem is surprisingly mature for an early-stage project — most crucial areas have at least some coverage.

### Key Findings

| Metric | Value |
|--------|-------|
| Total project .md files | 193 |
| Total lines | ~75,000+ (including very large generated/reference files) |
| Core implementation docs | ~50 files (no notebookllm duplicates) |
| Duplicate/copy files | 37 files in `ghostwire/notebookllm/` |
| Formal specs (GIP-numbered) | 10 files in `docs/spec/` |
| Presentations | 14 slide decks in `docs/presentations/` |
| Auto-generated / advisory | ~38 files (advisories, auth guide, markdown lint configs) |

### Top-Level Assessment

| Dimension | Grade | Notes |
|-----------|-------|-------|
| **Specification completeness** | A- | Formal specs with GIP identifiers, covers core protocol |
| **API documentation** | B+ | Comprehensive but uses stale port (8080 vs 9097) |
| **Architecture docs** | B | Good high-level, some internals stale |
| **Installation/setup** | B | Functional but scattered across multiple files |
| **Security documentation** | A- | Formal security policy + audit report |
| **Developer guides** | B- | Good API reference, but embedding/build guides need updates |
| **Knowledge base** | C | 8 files in `docs/wiki/`, large content-dump files |
| **Duplicate management** | D | 37 files in `notebookllm/` replicate content |

---

## 2. Inventory Breakdown

### By Directory

| Directory | File Count | Lines | Purpose |
|-----------|-----------|-------|---------|
| `root/` (monorepo root) | 14 | ~750 | README, ROADMAP, LICENSE, CONTRIBUTING, etc. |
| `docs/` | 40 | ~3,200 | Implementation guides, security, governance |
| `docs/spec/` | 10 | ~2,800 | Formal protocol specifications |
| `docs/build/` | 6 | ~2,500 | Developer build/embed guides |
| `docs/presentations/` | 15 | ~3,800 | Slide deck markdown |
| `docs/legacy/` | 6 | ~700 | Historical docs |
| `docs/wiki/` | 8 | ~13,000+ | Knowledge base / deep dives |
| `docs/protocol-knowledge/` | 1 | ~550 | libp2p curriculum |
| `docs/api/` | 1 | ~330 | API deep dive |
| `ghostwire/` | 17 | ~2,500 | Backend-specific docs |
| `ghostwire/docs/` | 7 | ~15,000 | Auto-generated API docs + big generated files |
| `ghostwire/notebookllm/` | 37 | ~23,000 | Duplicate/copy files for AI context |
| `ghostwire-libs/crates/` | 4 | ~200 | Crate-level READMEs |
| `launch-kit/` | 8 | ~800 | Deployment/launch materials |
| `social-media/` | 3 | ~400 | Social media copy |
| `grants/` | 3 | ~800 | Grant applications |
| `knowledge-base/` | 3 | ~500 | External knowledge references |
| Other | 6 | ~800 | Dotfiles, workflows, config docs |

### By Size Distribution

| Size Range | Count | Examples |
|-----------|-------|---------|
| < 100 lines | ~70 | READMEs, quick reference, config docs |
| 100–500 lines | ~80 | Most implementation guides, specs |
| 500–2000 lines | ~30 | Deeper guides, Security_Audit, testing docs |
| > 2000 lines | 3 | GhostWire_Codebase.md (7096), COMPREHENSIVE_CODE_REVIEW.md (1540), IMPLEMENTATION_GUIDE.md (1264) |

---

## 3. Category-by-Category Assessment

### 3.1 Root Docs (14 files)

| File | Lines | Quality | Issues |
|------|-------|---------|--------|
| `README.md` | 390 | A- | Recently updated; comprehensive |
| `ROADMAP.md` | 479 | B+ | Agent working doc; very detailed |
| `PROJECT_STACK.md` | 54 | B+ | Complete but brief |
| `LICENSE` | N/A | N/A | Standard AGPL-3.0 |
| `COMMERCIAL_LICENSE.md` | - | N/A | Standard commercial license |
| `LICENSE_EXPLANATION.md` | ~50 | A | Clear dual-license summary |
| `CONTRIBUTING.md` | ~50 | B | Basic but functional |
| `CODE_OF_CONDUCT.md` | ~100 | B | Standard CoC |
| `SECURITY.md` | ~100 | A | Clear disclosure policy |
| `.github/FUNDING.yml` | - | N/A | GitHub funding config |
| `.github/workflows/` docs | - | B | CI workflows documented |

**Assessment:** Root docs are the strongest category. README is comprehensive and was recently updated. ROADMAP.md doubles as an agent instruction file. PROJECT_STACK.md is concise but accurate. LICENSE_EXPLANATION.md is a nice touch for clarity on dual licensing.

**Issues:** None significant.

---

### 3.2 Implementation Guides (`docs/`, 40 files)

| File | Lines | Quality | Issues |
|------|-------|---------|--------|
| `ARCHITECTURE.md` | 125 | B+ | Good high-level diagram, needs deeper internals |
| `INSTALL.md` | ~200 | B | Functional, platform-specific detail |
| `QUICK_START.md` | ~100 | B | Concise, works |
| `HOW_TO_START_GHOSTWIRE.md` | ~550 | B | Very detailed, some redundancy with INSTALL |
| `CLI.md` | ~150 | B | CLI reference |
| `TUI.md` | ~100 | B+ | TUI keybinds, screens |
| `WEB_UI.md` | ~80 | B | Web frontend notes |
| `TESTING.md` | ~150 | B | Test patterns |
| `DEVELOPMENT.md` | ~200 | B+ | Dev workflow |
| `SECURITY.md` | ~100 | A | Security policy |
| `SECURITY_AUDIT.md` | ~500 | A | Full audit report |
| `AUDIT_SUMMARY.md` | ~200 | A | Executive summary |
| `FIRST_PING.md` | ~50 | A | Historic milestone record |
| `GOVERNANCE.md` | ~150 | A | GIP process defined |
| `CHANGELOG.md` | ~100 | B | Version history |
| `API_TESTING_GUIDE.md` | ~600 | B+ | Extensive curl examples |
| `API_SECURITY.md` | ~300 | B+ | Auth patterns |
| `BACKEND_COMMUNICATION_TEST.md` | ~200 | B+ | Multi-node test walkthrough |

**Assessment:** Solid coverage. The security audit documents stand out as thorough. HOW_TO_START_GHOSTWIRE is comprehensive but could benefit from tighter organization (552 lines covering everything from installation to troubleshooting).

**Issues:** Overlap between INSTALL.md, QUICK_START.md, and HOW_TO_START_GHOSTWIRE.md — three separate getting-started files that cover similar ground. Port references recently fixed from 8080 → 9097.

---

### 3.3 Protocol Specifications (`docs/spec/`, 10 files)

| File | Lines | Quality | GIP |
|------|-------|---------|-----|
| `PROTOCOL.md` | ~500 | A- | GIP-0001 |
| `WIRE_FORMAT.md` | ~300 | A- | GIP-0002 |
| `CRYPTO.md` | ~400 | A | GIP-0003 |
| `SPHINX.md` | ~300 | A | GIP-0004 |
| `DISCOVERY.md` | ~250 | A- | GIP-0005 |
| `ROUTING.md` | ~300 | A- | GIP-0006 |
| `DTN.md` | ~350 | A- | GIP-0007 |
| `IDENTITY.md` | ~350 | A | GIP-0008 |
| `SOCIAL.md` | ~250 | A- | GIP-0009 |
| `TRANSPORT.md` | ~200 | A- | GIP-0010 |

**Assessment:** This is the strongest documentation category. Every major subsystem has a formal spec with a GIP identifier. The spec docs are well-structured with the consistent format:
- GIP number, Status, Editor
- Motivation / Design Goals
- Detailed specification
- Security considerations
- References

**Issues:** Some specs may lag behind the actual implementation (common in fast-moving projects). DTN spec mentions GIP-0007 status without clear indication if it's Draft, Review, or Final. No version number per spec document.

---

### 3.4 Build/Developer Docs (`docs/build/`, 6 files)

| File | Lines | Quality | Issues |
|------|-------|---------|--------|
| `OVERVIEW.md` | ~150 | B+ | Build on GhostWire intro |
| `QUICKSTART.md` | ~300 | B | Rust integration walkthrough |
| `EMBEDDING.md` | ~350 | B | Daemon embedding patterns |
| `CUSTOM_TRANSPORT.md` | ~200 | B | Transport trait docs |
| `PROTOCOL_BRIDGES.md` | ~400 | B+ | Bridge implementation guide |
| `API_REFERENCE.md` | ~600 | B+ | Comprehensive endpoint listing |

**Assessment:** These are the "build on GhostWire" developer docs. API_REFERENCE.md is the most valuable — it catalogs every endpoint with curl examples and response shapes. EMBEDDING.md and PROTOCOL_BRIDGES.md are solid implementation guides.

**Issues:** CUSTOM_TRANSPORT.md could use real code examples. QUICKSTART.md is too long for a quick start. API_REFERENCE.md port references were stale (8080 vs 9097) until this session.

---

### 3.5 Backend Docs (`ghostwire/`, 17 files)

| File | Lines | Quality | Notes |
|------|-------|---------|-------|
| `README.md` | 206 | B+ | Recently updated |
| `BUILD_INSTRUCTIONS.md` | 93 | B+ | Build guide |
| `CHANGELOG.md` | ~200 | B | Release notes |
| `AGENTS.md` | ~100 | A | Agent context file |
| `GEMMA4_INTEGRATION.md` | ~300 | A | AI model setup guide |
| `TODO.md` | ~200 | B | Internal task list |
| `MASTER_TODO.md` | ~200 | B | Cross-referenced master tasks |
| `docs/API.md` | ~800 | B | Auto-generated API doc |
| `docs/GHOSTWIRE_MANUAL.md` | ~400 | B | Full manual |
| `docs/` (other) | 5 files | - | Various internal docs |
| `notebookllm/` | 37 files | D | See [§5 Duplication Map](#5-duplication-map) |

**Assessment:** The backend README and BUILD_INSTRUCTIONS were recently updated. GEMMA4_INTEGRATION.md is well-detailed. AGENTS.md is a valuable context file for AI coding tools.

**Issues:** The `docs/` subdirectory under GhostWire contains auto-generated or minimally maintained files. `notebookllm/` is a significant duplication problem.

---

### 3.6 Protocol Knowledge / Curriculum

**`docs/protocol-knowledge/30_DAY_LIBP2P_CURRICULUM.md`** (~550 lines)
- 30-day structured learning path for libp2p
- Well-structured with daily exercises
- Integrates GhostWire API calls as exercises
- **Assessment:** B+. Useful teaching resource. Some port references were stale.

**`docs/api/DEEP_DIVE.md`** (~330 lines)
- Mixed content: quick start, API reference, troubleshooting
- **Assessment:** B-. Could be better organized. Content feels like a brain dump in parts.

**`docs/wiki/`** (8 files, ~13,000 lines)
- These are large knowledge-base dumps
- **Assessment:** C. The content dump format (GhostWire_Codebase.md at 7096 lines, COMPREHENSIVE_CODE_REVIEW.md at 1540 lines) is useful for LLM context but nearly impossible for human reading.

---

### 3.7 Launch Kit & Social Media

**`launch-kit/`** (8 files, ~800 lines)
- Press release, pitch deck, roadmap presentation, comparison sheet
- Community guidelines, email templates
- **Assessment:** B. Well-organized launch materials. Roadmap presentation may be stale.

**`social-media/`** (3 files, ~400 lines)
- Twitter/Bluesky thread, dev.to post, introductory thread
- **Assessment:** B. Serviceable social media copy.

**`grants/`** (3 files, ~800 lines)
- GCD4F submission, cover letter, dev.to crosspost
- **Assessment:** B+. Grant materials are well-written.

---

## 4. Content Quality Assessment

### 4.1 By Quality Tier

**Tier A (Excellent — well-structured, comprehensive, maintained)**
- `docs/spec/` (all 10 formal specs)
- `README.md` (root), `AGENTS.md`, `PROJECT_STACK.md`
- `docs/SECURITY.md`, `docs/SECURITY_AUDIT.md`, `docs/AUDIT_SUMMARY.md`
- `ghostwire/GEMMA4_INTEGRATION.md`
- `COMMERCIAL_LICENSE.md`, `LICENSE_EXPLANATION.md`
- `grants/GCD4F_SUBMISSION.md`

**Tier B (Good — functional, maybe slightly stale or thin)**
- Most implementation guides (`docs/INSTALL.md`, `docs/CLI.md`, etc.)
- `docs/build/` developer guides
- `ghostwire/README.md`, `BUILD_INSTRUCTIONS.md`
- `ROADMAP.md`, `MASTER_TODO.md`
- `launch-kit/` materials
- `social-media/` copy

**Tier C (Fair — needs work)**
- `docs/wiki/` content-dump files (too large, unstructured)
- `docs/api/DEEP_DIVE.md` (disorganized)
- `ghostwire-notebookllm/` (duplicates)
- `knowledge-base/` files (external references, likely stale)

**Tier D (Poor — significant issues)**
- `ghostwire/notebookllm/` duplication (37 files, no clear ownership)

### 4.2 Consistency Issues

| Issue | Files Affected | Severity |
|-------|---------------|----------|
| Port `8080` vs `9097` | All API example docs (now fixed) | High |
| Repo URL inconsistency | Root vs ghostwire README (now fixed) | High |
| Double Ratchet status | README roadmap (now fixed) | High |
| `docs/` has 3 getting-started files | INSTALL.md, QUICK_START.md, HOW_TO_START_GHOSTWIRE.md | Medium |
| Protocol bridge status | Marked "Experimental" in ARCHITECTURE but "Planned" in README | Medium |
| B3 (DELETE endpoint) status | ROADMAP.md (now fixed) | Medium |

---

## 5. Duplication Map

### 5.1 The `notebookllm/` Problem

The **37 files** in `ghostwire/notebookllm/` are copies/derivatives created for LLM context injection. This creates a maintenance burden:

| Source File | notebookllm Copy | Differences |
|------------|-----------------|-------------|
| `ghostwire/README.md` | `notebookllm/ghostwire-readme.md` | Probably identical |
| `docs/SECURITY_AUDIT.md` | `notebookllm/SECURITY_AUDIT.md` | Probably identical |
| `docs/API_SECURITY.md` | `notebookllm/API_SECURITY.md` | Probably identical |
| `ghostwire/docs/API.md` | `notebookllm/API.md` | Minor differences |
| `ghostwire/docs/GHOSTWIRE_MANUAL.md` | `notebookllm/GHOSTWIRE_MANUAL.md` | Identical (byte-level) |
| Various | `notebookllm/GhostWire_Codebase.md` | 7096-line generated codebase dump |
| Various | `notebookllm/COMPREHENSIVE_CODE_REVIEW.md` | 1540-line review document |
| Various | `notebookllm/IMPLEMENTATION_GUIDE.md` | 1264-line guide |
| — | `notebookllm/HARDWARE_SPEC_BOM.md` | 409-line unique hardware spec |
| — | `notebookllm/GHOSTWIRE_VISION.md` | Unique vision document |

**Selected file-by-file comparison (for key duplicates):**

| File | Lines | Duplicate Of | Notes |
|------|-------|-------------|-------|
| `notebookllm/GHOSTWIRE_MANUAL.md` | 400 | `docs/GHOSTWIRE_MANUAL.md` | **Identical** |
| `notebookllm/API.md` | 800 | `docs/API.md` | **Minor diffs** (section ordering) |
| `notebookllm/README.md` | 180 | Unique | Short intro, port reference |
| `notebookllm/GHOSTWIRE_VISION.md` | ~100 | Unique | Vision/mission document |
| `notebookllm/HARDWARE_SPEC_BOM.md` | 409 | Unique | Hardware bill of materials |
| `notebookllm/GhostWire_Codebase.md` | 7096 | Generated | Auto-generated source dump |
| `notebookllm/COMPREHENSIVE_CODE_REVIEW.md` | 1540 | Unique | Code review document |
| `notebookllm/IMPLEMENTATION_GUIDE.md` | 1264 | Unique | Implementation log/journal |
| `notebookllm/GhostWire_Ops_Tests.md` | ~800 | Unique | LLM ops test scenarios |

### 5.2 Other Duplication

| Duplicate Group | Files | Assessment |
|----------------|-------|------------|
| Getting-started guides | INSTALL.md, QUICK_START.md, HOW_TO_START_GHOSTWIRE.md | Moderate overlap; could consolidate |
| TODO files | TODO.md, MASTER_TODO.md, ROADMAP.md (Phase sections) | ROADMAP.md references both; some duplication |
| Security audit | SECURITY.md, SECURITY_AUDIT.md, AUDIT_SUMMARY.md | Proper tiered structure — this is good |

---

## 6. Content Gaps

### 6.1 Missing or Underdeveloped Content

| Gap | Importance | Notes |
|-----|-----------|-------|
| **Production deployment guide** | High | No doc on how to run GhostWire in production (systemd, Docker compose, reverse proxy) |
| **Troubleshooting FAQ** | High | Common issues scattered across HOW_TO_START but no dedicated FAQ |
| **Performance characteristics** | Medium | No baseline: message latency, throughput, memory usage, network overhead |
| **Integration test docs** | Medium | Backend test exists but no doc on how to write/run integration tests |
| **CLI command reference** | Medium | CLI described in README but no comprehensive reference |
| **TUI screenshots** | Low | TUI described textually; visual reference would help |
| **API versioning policy** | Medium | v2 API exists but no doc on versioning strategy or deprecation policy |
| **Contributor onboarding** | Medium | CONTRIBUTING.md is basic; no "how to make your first PR" guide |
| **ESP32 docs** | Low | ESP-IDF firmware mentioned in PROJECT_STACK but no docs |
| **WebUI docs** | Medium | WEB_UI.md is thin; no component docs or state management docs |
| **ghostwire-libs crate docs** | Low | Each crate has a short README; API docs on docs.rs only |
| **Localization/i18n** | Low | No multi-language support documented |

### 6.2 Docs That Mention Non-Existent Features

| Feature Referenced | Where | Reality |
|-------------------|-------|---------|
| "React Web Interface" | ARCHITECTURE.md diagram | WebUI exists but is minimal (Tauri-based) |
| "Briar Adapter", "Meshtastic Adapter" | ARCHITECTURE.md diagram | Marked "Experimental/Planned" — not implemented |
| "AES-256-GCM" | ARCHITECTURE.md | Actual crypto is XChaCha20-Poly1305 |
| "Swagger UI at localhost:8080/swagger-ui" | DEEP_DIVE.md | uutoipa not fully wired in production build |

---

## 7. Recommendations

### 7.1 Immediate (High Impact, Low Effort)

| # | Action | Rationale |
|---|--------|-----------|
| 1 | **Consolidate getting-started files**: Merge INSTALL.md, QUICK_START.md, HOW_TO_START_GHOSTWIRE.md into a single `docs/GETTING_STARTED.md` | Three overlapping files confuse new users |
| 2 | **Create `.gitignore` for notebookllm sync**: Exclude `notebookllm/` from PR reviews or add CI check that verifies they match their sources | Prevents drift |
| 3 | **Add GIP status badges** to spec files: `Status: Draft | Review | Final | Superseded` | Currently no status indicator per spec |
| 4 | **Delete stale auto-generated files** from `ghostwire/docs/` that are clearly superseded | Reduces noise |

### 7.2 Short-Term (High Impact, Medium Effort)

| # | Action | Rationale |
|---|--------|-----------|
| 5 | **Write production deployment guide**: systemd service, Docker Compose, reverse proxy config, monitoring setup | Critical for real users |
| 6 | **Audit notebookllm/ for unique content**: Extract genuinely unique items (HARDWARE_SPEC_BOM, VISION, IMPLEMENTATION_GUIDE) to canonical locations; remove or script the rest | Eliminates 37-file maintenance burden |
| 7 | **Write API versioning policy**: When v2 → v3? How long are deprecated endpoints supported? | API consumers need this |
| 8 | **Add performance baseline doc**: Message latency, throughput under load, memory profile | Serious adopters ask for this |

### 7.3 Medium-Term

| # | Action |
|---|--------|
| 9 | Create a CLI reference doc (all subcommands, flags, exit codes) |
| 10 | Build a docs CI pipeline: links check, stale content detection, notebookllm sync verification |
| 11 | Add screenshots/asciinema to TUI documentation |
| 12 | Consider moving to a docs framework (Docusaurus, mdBook) for better navigation |

### 7.4 Recommendations for notebookllm/

This directory serves a legitimate purpose (LLM context injection) but creates a maintenance problem:

**Option A (Recommended):** Keep only the truly unique files in `notebookllm/`:
- `HARDWARE_SPEC_BOM.md` — unique content
- `GHOSTWIRE_VISION.md` — unique content  
- `IMPLEMENTATION_GUIDE.md` — unique journal
- Remove all files that are direct copies of docs elsewhere

**Option B:** Add a CI step that verifies duplicate files match their sources (e.g., diff check). Prevents drift but maintains the copies for LLM use.

**Option C:** Remove the entire `notebookllm/` directory and use `AGENTS.md` + the spec docs for LLM context instead.

---

## 8. Appendix: Full File Listing

### Root (14 files)
```
/GhostWire-secure-mesh-communication/
├── README.md                          (390 lines)
├── ROADMAP.md                         (479 lines)  [agent instruction file]
├── PROJECT_STACK.md                   (54 lines)
├── CONTRIBUTING.md                    (~50 lines)
├── CODE_OF_CONDUCT.md                 (~100 lines)
├── SECURITY.md                        (~100 lines)
├── COMMERCIAL_LICENSE.md
├── LICENSE_EXPLANATION.md             (~50 lines)
├── LICENSE
├── CHANGELOG.md                       (~100 lines)
├── rust-toolchain.toml
├── rustfmt.toml
├── .github/
│   ├── FUNDING.yml
│   └── workflows/*.yml
```

### docs/ (40 files)
```
docs/
├── README.md                          (121 lines)
├── ARCHITECTURE.md                    (125 lines)
├── INSTALL.md                         (~200 lines)
├── QUICK_START.md                     (~100 lines)
├── HOW_TO_START_GHOSTWIRE.md          (~550 lines)
├── CLI.md                             (~150 lines)
├── TUI.md                             (~100 lines)
├── WEB_UI.md                          (~80 lines)
├── TESTING.md                         (~150 lines)
├── DEVELOPMENT.md                     (~200 lines)
├── SECURITY.md                        (~100 lines)
├── SECURITY_AUDIT.md                  (~500 lines)
├── AUDIT_SUMMARY.md                   (~200 lines)
├── FIRST_PING.md                      (~50 lines)
├── GOVERNANCE.md                      (~150 lines)
├── CHANGELOG.md                       (~100 lines)
├── API_TESTING_GUIDE.md               (~600 lines)
├── API_SECURITY.md                    (~300 lines)
├── BACKEND_COMMUNICATION_TEST.md      (~200 lines)
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
├── spec/
│   ├── PROTOCOL.md                    (GIP-0001)
│   ├── WIRE_FORMAT.md                 (GIP-0002)
│   ├── CRYPTO.md                      (GIP-0003)
│   ├── SPHINX.md                      (GIP-0004)
│   ├── DISCOVERY.md                   (GIP-0005)
│   ├── ROUTING.md                     (GIP-0006)
│   ├── DTN.md                         (GIP-0007)
│   ├── IDENTITY.md                    (GIP-0008)
│   ├── SOCIAL.md                      (GIP-0009)
│   └── TRANSPORT.md                   (GIP-0010)
├── build/
│   ├── OVERVIEW.md                    (~150 lines)
│   ├── QUICKSTART.md                  (~300 lines)
│   ├── EMBEDDING.md                   (~350 lines)
│   ├── CUSTOM_TRANSPORT.md            (~200 lines)
│   ├── PROTOCOL_BRIDGES.md            (~400 lines)
│   └── API_REFERENCE.md               (~600 lines)
├── presentations/                     (14 files)
├── legacy/                            (6 files)
├── wiki/
│   ├── GhostWire_Codebase.md          (7096 lines)
│   ├── COMPREHENSIVE_CODE_REVIEW.md   (1540 lines)
│   └── (6 more knowledge files)
├── protocol-knowledge/
│   └── 30_DAY_LIBP2P_CURRICULUM.md    (~550 lines)
└── api/
    └── DEEP_DIVE.md                   (~330 lines)
```

### ghostwire/ (17 files)
```
ghostwire/
├── README.md                          (206 lines)
├── BUILD_INSTRUCTIONS.md              (93 lines)
├── CHANGELOG.md                       (~200 lines)
├── AGENTS.md                          (~100 lines)
├── GEMMA4_INTEGRATION.md              (~300 lines)
├── TODO.md                            (~200 lines)
├── MASTER_TODO.md                     (~200 lines)
├── docs/
│   ├── API.md                         (~800 lines)
│   ├── GHOSTWIRE_MANUAL.md            (~400 lines)
│   └── (5 more docs files)
└── notebookllm/                       (37 files — see §5)
```

### ghostwire-libs/ (4 files)
```
ghostwire-libs/crates/
├── sphinx-rs/README.md                (~50 lines)
├── ghostwire-dtn/README.md            (~50 lines)
├── hlc-rs/README.md                   (~50 lines)
└── trust-store/README.md              (~50 lines)
```

### Launch Kit & Media (14 files)
```
launch-kit/                            (8 files)
social-media/                          (3 files)
grants/                                (3 files)
knowledge-base/                        (3 files)
```

---

> **End of Audit — 193 files assessed, May 26, 2026**
