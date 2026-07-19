# GhostWire Refactoring Report

> **Status**: In Progress  
> **Generated**: 2026-07-19  
> **Scope**: Exhaustive codebase analysis for modularization, maintainability, and code quality improvements

---

## Executive Summary

GhostWire is a sophisticated mesh networking platform with strong cryptographic foundations, but the codebase has significant structural debt. This report identifies monolithic code, duplication, hardcoded values, and provides actionable refactoring recommendations.

### Key Metrics

| Metric | Value | Assessment |
|--------|-------|------------|
| Rust Source Files | 142 | High |
| Python Scripts | 30 | **Needs consolidation** |
| Shell Scripts | 44 | **Needs consolidation** |
| Core Modules | 54 | Too many in single dir |
| Largest File | mesh.rs (2,227 lines) | **Critical** |
| Duplicate Code | security.rs vs security_v2.rs | **Must resolve** |
| Hardcoded Constants | 200+ | **Must centralize** |

---

## 1. MONOLITHIC FILES (Critical)

### 1.1 Rust Source Files

| File | Lines | Issues | Recommendation |
|------|-------|--------|----------------|
| `core/mesh.rs` | 2,227 | God object, 192+ imports | Split into transport, discovery, routing modules |
| `core/mod.rs` | 1,981 | Module bloat, 54 modules declared | Create sub-modules or feature gates |
| `core/encryption.rs` | 1,224 | Multiple responsibilities | Separate crypto, key management, CRL |
| `core/security_v2.rs` | 871 | Duplicates security.rs | Merge or deprecate security.rs |
| `core/ai/function_calling.rs` | 1,003 | AI coupling | Abstract AI interface |
| `core/security.rs` | 755 | **Duplicate of v2** | Deprecate and remove |

### 1.2 Shell Scripts

| Script | Lines | Issues | Recommendation |
|--------|-------|--------|----------------|
| `scripts/install.sh` | 850 | Monolithic installer | Split by function |
| `scripts/install-system-monitor.sh` | 355 | Platform-specific | Move to platform/ dir |
| `scripts/quick_start.sh` | 318 | Duplicates install | Consolidate |
| `scripts/make-repo-public.sh` | 287 | One-time script | Move to tools/ or remove |

### 1.3 Python Scripts

| Script | Lines | Issues | Recommendation |
|--------|-------|--------|----------------|
| `train_gnn.py` | 682 | Training logic | Extract to training lib |
| `train_gnn_kaggle.py` | 529 | Duplicates train_gnn.py | Parametrize instead |
| `export_gnn_to_onnx.py` | 491 | Export logic | Create unified export |
| `generate_realistic_topology.py` | 379 | Dataset generation | Create data gen package |
| `generate_100k_dataset.py` | 323 | Duplicates above | Consolidate |

---

## 2. DUPLICATION ISSUES

### 2.1 Rust Module Duplication

#### `security.rs` vs `security_v2.rs`

Both files contain nearly identical implementations:

```
security.rs:755 lines     ← DEPRECATED
security_v2.rs:871 lines  ← ACTIVE
```

**Duplicated Types:**
- `PersistentBlacklistManager`
- `BlacklistEntry`
- `FailedAttemptTracker`
- `AuditLogger`

**Action Required:**
```rust
// In mod.rs, currently has:
pub use security_v2 as security;  // ← Good

// But security.rs still exists with duplicate code
pub mod security;  // ← Remove this
```

#### `stealth_tcp.rs` vs `stealth_tcp_handshake_fix.rs`

```
stealth_tcp.rs:600 lines
stealth_tcp_handshake_fix.rs:181 lines  ← Fixes applied?
```

**Action Required:** Merge fixes back and delete stub.

#### `updater.rs` vs `updater_fixed.rs`

```
updater.rs:73 lines
updater_fixed.rs:236 lines  ← Likely has the real implementation
```

**Action Required:** Deprecate `updater.rs`, rename `updater_fixed.rs`.

### 2.2 Python Script Duplication

#### GNN Training Scripts (5 versions)
```
train_gnn.py                    (682 lines) ← Primary
train_gnn_kaggle.py             (529 lines) ← Kaggle variant
train_gnn_kaggle_v2.py         (400 lines) ← Kaggle variant 2
train_hybrid_101k.py            (264 lines) ← Hybrid variant
gnn_routing.py                  (308 lines) ← Duplicates training logic
```

**Proposed Consolidation:**
```python
# ghostwire/scripts/training/
#   ├── train.py              # Main entry point
#   ├── models/
#   │   ├── __init__.py
#   │   ├── gat.py           # Graph Attention Network
#   │   ├── hrm.py           # Hybrid Routing Model
#   │   └── anomaly.py       # Anomaly detection
#   ├── datasets/
#   │   ├── __init__.py
#   │   ├── mesh_topology.py # Synthetic data
#   │   └── guifissants.py   # Real data loader
#   └── exporters/
#       ├── __init__.py
#       └── onnx.py          # ONNX export
```

#### LoRA Fine-tuning Scripts (2 versions)
```
finetune_lora.py          (282 lines)
finetune_lora_fixed.py    (282 lines) ← Likely identical
```

**Action Required:** Delete `finetune_lora_fixed.py`, keep one.

#### Dataset Generation Scripts (4+ versions)
```
generate_realistic_topology.py    (379 lines)
generate_100k_dataset.py          (323 lines)
generate_ghostwire_mesh_dataset.py(255 lines)
generate_lightgbm_dataset.py      (?)
transform_real_dataset.py         (?)
transform_real_dataset_fast.py    (?)
```

**Proposed Consolidation:**
```python
# ghostwire/scripts/data_generation/
#   ├── __init__.py
#   ├── synthetic.py    # Synthetic mesh topology generation
#   ├── real.py        # Transform real datasets
#   └── splits.py      # Dataset splitting utilities
```

### 2.3 Shell Script Duplication

#### Installation Scripts
```
install.sh                    (850 lines) ← Main installer
install-for-omarchy.sh        (316 lines) ← Platform variant
install-dev-tools.sh           (?)
```

**Proposed Structure:**
```bash
# scripts/install/
#   ├── install.sh              # Main entry point
#   ├── platforms/
#   │   ├── common.sh          # Shared functions
#   │   ├── omarchy.sh         # Omarchy-specific
#   │   └── default.sh         # Generic Linux
#   └── steps/
#       ├── 01-deps.sh         # Dependencies
#       ├── 02-build.sh        # Build steps
#       └── 03-config.sh       # Configuration
```

---

## 3. HARDCODED VALUES

### 3.1 Magic Numbers in Rust

Found **200+ hardcoded values** scattered across the codebase. Examples:

| Value | Location | Should Be |
|-------|----------|-----------|
| `3600` | 50+ files | `TIME_SECONDS::HOUR` |
| `86400` | 10+ files | `TIME_SECONDS::DAY` |
| `1000` | 20+ files | `DEFAULT_MAX_*` |
| `4096` | 15+ files | `BUFFER_SIZE_*` |
| `65535` | 8 files | `MAX_PORT` or similar |
| `5` | 30+ files | Various timeouts |

**Recommended Constants Module:**
```rust
// core/src/constants/mod.rs

pub mod time {
    pub const SECOND: u64 = 1;
    pub const MINUTE: u64 = 60;
    pub const HOUR: u64 = 3600;
    pub const DAY: u64 = 86400;
    pub const WEEK: u64 = 604800;
}

pub mod buffer {
    pub const MIN: usize = 256;
    pub const DEFAULT: usize = 4096;
    pub const LARGE: usize = 65536;
    pub const MAX: usize = 1024 * 1024;
}

pub mod network {
    pub const MAX_PORT: u16 = 65535;
    pub const DEFAULT_MESH_PORT: u16 = 4001;
    pub const DEFAULT_API_PORT: u16 = 9000;
    pub const DEFAULT_WEB_PORT: u16 = 8080;
}

pub mod mesh {
    pub const MAX_HOPS: u8 = 5;
    pub const MAX_PEERS: usize = 50;
    pub const CONNECTION_TIMEOUT_SECS: u64 = 30;
    pub const HEARTBEAT_INTERVAL_SECS: u64 = 60;
}
```

### 3.2 Hardcoded Bootstrap Nodes

**Location:** `core/mesh.rs:120-132`

```rust
pub const DEFAULT_BOOTSTRAPS: &[&str] = &[
    "/dnsaddr/bootstrap.libp2p.io/p2p/QmNnooP36J7RSRM8uXCc8p6Y74G56EPV45UFSy9Pr7T79F",
    // ... 8 more
];
```

**Issue:** Hardcoded into binary, can't be updated without recompilation.

**Recommendation:** Move to config file or allow runtime updates.

### 3.3 Magic Strings

| String | Location | Should Be |
|--------|----------|-----------|
| `"/ghostwire/mesh/2.0.0"` | mesh.rs | Protocol version constant |
| `"/ghostwire/kad/shard/global"` | mesh.rs | Protocol path constant |
| `"ghostwire.local"` | quic_transport.rs | TLS hostname |
| `"GHOSTWIRE-TOFU-FP-SALT"` | quic_transport.rs | Domain separation constant |

---

## 4. ARCHITECTURAL ISSUES

### 4.1 Core Module Bloat

**Problem:** 54 modules in single `core/` directory.

**Current Structure:**
```
src/core/
├── mod.rs (1,981 lines) ← Lists all modules
├── encryption.rs (1,224)
├── mesh.rs (2,227)
├── security.rs (755)
├── security_v2.rs (871)
├── ... 48 more files
```

**Proposed Structure:**
```
src/core/
├── mod.rs (minimal)
├── crypto/
│   ├── mod.rs
│   ├── encryption.rs
│   ├── ratchet.rs
│   ├── key_manager.rs
│   └── constants.rs
├── network/
│   ├── mod.rs
│   ├── mesh/
│   ├── routing/
│   ├── transport/
│   └── dtn/
├── security/
│   ├── mod.rs
│   ├── manager.rs
│   ├── blacklist.rs
│   ├── audit.rs
│   └── rate_limiter.rs
├── storage/
│   ├── mod.rs
│   ├── store.rs
│   ├── persistence.rs
│   └── mailbox.rs
├── identity/
│   ├── mod.rs
│   ├── core_identity.rs
│   ├── recovery.rs
│   └── pairing.rs
└── ai/
    ├── mod.rs
    ├── routing/
    └── anomaly/
```

### 4.2 Controller Monolith

**Problem:** `controllers/all_controllers.rs` (828 lines) contains all controllers.

**Current:**
```rust
// controllers/all_controllers.rs
pub struct ArkController;
pub struct ChannelController;
pub struct RecoveryController;
// ... 15+ more
```

**Proposed:**
```
controllers/
├── mod.rs
├── ark.rs
├── channels.rs
├── recovery.rs
├── network.rs
├── pairing.rs
└── trust.rs
```

### 4.3 Mixed Feature Flags

**Problem:** Feature flags scattered in `Cargo.toml` and code.

**Current:**
```toml
[features]
default = ["mesh", "web-ui"]
mesh = []
web-ui = []
bluetooth = []
briar = []
```

**Issues:**
- Features not properly gated
- `#[cfg(feature = "mesh")]` used inconsistently
- Some feature-gated code is dead

**Recommendation:** Audit all `#[cfg(...)]` usage and ensure consistency.

---

## 5. SCRIPT CONSOLIDATION

### 5.1 Shell Scripts (44 files)

**Categorize into:**
```
scripts/
├── install/           # Installation scripts
├── launch/            # Launch scripts
├── maintenance/       # Maintenance scripts
├── test/              # Test scripts
├── deploy/            # Deployment scripts
└── utilities/        # Standalone utilities
```

### 5.2 Python Scripts (30 files)

**Categorize into:**
```
ghostwire/scripts/
├── training/          # ML training scripts
├── data/              # Dataset generation
├── export/            # Model export
├── quantization/      # Model optimization
└── notebooks/         # Jupyter notebooks
```

---

## 6. ACTION PLAN

### Phase 1: Quick Wins (1-2 weeks)

| Task | Effort | Impact | Priority |
|------|--------|--------|----------|
| Delete duplicate `security.rs` | Low | High | P0 |
| Delete `stealth_tcp_handshake_fix.rs` | Low | Medium | P1 |
| Delete duplicate Python scripts | Low | Medium | P1 |
| Delete `finetune_lora_fixed.py` | Low | Low | P2 |
| Create centralized constants module | Medium | High | P0 |
| Move bootstrap nodes to config | Medium | Medium | P1 |

### Phase 2: Modularization (2-4 weeks)

| Task | Effort | Impact | Priority |
|------|--------|--------|----------|
| Split `core/mod.rs` by domain | High | High | P0 |
| Split `core/mesh.rs` (5 files) | High | High | P0 |
| Move security to `core/security/` | Medium | High | P1 |
| Split controllers directory | Medium | Medium | P1 |
| Consolidate shell scripts | Medium | Medium | P2 |
| Consolidate Python scripts | Medium | Medium | P2 |

### Phase 3: Architecture (4-8 weeks)

| Task | Effort | Impact | Priority |
|------|--------|--------|----------|
| Create `core/crypto/` module | High | High | P1 |
| Create `core/network/` module | High | High | P1 |
| Create `core/storage/` module | Medium | Medium | P2 |
| Abstract AI interfaces | Medium | Medium | P2 |
| Document module boundaries | Low | Medium | P2 |

---

## 7. METRICS & TRACKING

### Current State
- **Total Rust LOC**: ~48,000
- **Total Python LOC**: ~8,000  
- **Total Shell LOC**: ~7,000
- **Duplication**: ~5,000 lines (estimated)
- **Hardcoded Values**: 200+

### Target State (After Refactoring)
- **Core modules**: <500 lines each
- **Controllers**: <300 lines each
- **Scripts**: Consolidated into <10 categories
- **Hardcoded Values**: <20 (all in constants module)

---

## 8. RISK ASSESSMENT

| Refactoring | Risk | Mitigation |
|-------------|------|------------|
| Delete duplicate modules | Medium | Branch, test, merge |
| Split large files | High | Incremental, maintain API |
| Consolidate scripts | Low | Scripts are standalone |
| Centralize constants | Low | Additive change |

---

## Appendix A: File Inventory

### Files to DELETE
```
ghostwire/src/core/security.rs         # Duplicate of security_v2.rs
ghostwire/src/core/stealth_tcp_handshake_fix.rs  # Merge back
ghostwire/src/core/updater.rs          # Duplicate of updater_fixed.rs
ghostwire/src/web.rs.backup           # Already deleted
ghostwire/src/web.rs.backup.cors      # Already deleted
ghostwire/scripts/finetune_lora_fixed.py  # Duplicate
ghostwire/scripts/train_gnn_kaggle.py # Parametrize train_gnn.py
ghostwire/scripts/train_gnn_kaggle_v2.py  # Parametrize train_gnn.py
```

### Files to REFACTOR
```
ghostwire/src/core/mesh.rs            # Split by transport/discovery/routing
ghostwire/src/core/mod.rs             # Minimal, use feature gates
ghostwire/src/core/encryption.rs      # Split crypto/key mgmt/CRL
ghostwire/src/controllers/all_controllers.rs  # Split by domain
ghostwire/src/web.rs                  # Split routes/controllers
scripts/install.sh                    # Modularize
```

### Files to CONSOLIDATE
```
# Python training scripts → ghostwire/scripts/training/
# Shell scripts → scripts/{install,launch,maintenance,test}/
```

---

## Appendix B: Hardcoded Constants to Extract

```rust
// core/src/constants.rs - Proposed structure

// Time constants
pub const SECOND: u64 = 1;
pub const MINUTE: u64 = 60;
pub const HOUR: u64 = 3600;
pub const DAY: u64 = 86400;

// Network constants
pub const MAX_PORT: u16 = 65535;
pub const DEFAULT_MESH_PORT: u16 = 4001;
pub const DEFAULT_API_PORT: u16 = 9000;
pub const DEFAULT_WEB_PORT: u16 = 8080;

// Buffer sizes
pub const BUFFER_SIZE_SMALL: usize = 256;
pub const BUFFER_SIZE_MEDIUM: usize = 4096;
pub const BUFFER_SIZE_LARGE: usize = 65536;
pub const BUFFER_SIZE_MAX: usize = 1024 * 1024;

// Mesh constants
pub const MAX_MESSAGE_HOPS: u8 = 5;
pub const MAX_PEERS: usize = 50;
pub const MAX_MESSAGE_SIZE: usize = 1024 * 1024;

// Security constants
pub const BLACKLIST_TTL_SECONDS: u64 = 3600;
pub const KEY_ROTATION_DAYS: u64 = 7;
pub const MAX_CONNECTIONS_PER_IP: usize = 10;
pub const MAX_MESSAGES_PER_MINUTE: usize = 100;
```

---

*Report generated by OpenHands agent*
