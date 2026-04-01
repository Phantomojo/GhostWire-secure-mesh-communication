# GhostWire Developer Tools Setup Guide

> Complete guide to setting up and using developer tools for GhostWire development.

## Quick Start

```bash
# Install all dev tools (takes 5-10 minutes)
./scripts/install-dev-tools.sh

# Initialize configuration files
cargo deny init
cargo make init

# Install Bruno GUI (required for API testing)
# Download from: https://www.usebruno.com/downloads
```

---

## 📦 Tool Categories

### Security Tools (🔴 Critical)

| Tool | Purpose | Install | Usage |
|------|---------|---------|-------|
| **cargo-audit** | Check dependencies for vulnerabilities | `cargo install cargo-audit` | `cargo audit` |
| **cargo-deny** | Enforce security/license policies | `cargo install cargo-deny` | `cargo deny check` |
| **cargo-geiger** | Detect unsafe code in dependencies | `cargo install cargo-geiger` | `cargo geiger` |

**When to use:**
- `cargo audit` - Run in CI/CD, before releases, weekly
- `cargo deny check` - Run on every PR
- `cargo geiger` - Monthly security review

**Example output:**
```
$ cargo audit
    Fetching advisory database from RustSec...
    Loaded 587 security advisories
    Checking ghostwire v0.1.0
    ✅ All dependencies are secure
```

---

### Testing Tools (🔴 Critical)

| Tool | Purpose | Install | Usage |
|------|---------|---------|-------|
| **cargo-nextest** | Fast test runner (3x speed) | `cargo install --locked cargo-nextest` | `cargo nextest run` |
| **cargo-llvm-cov** | Code coverage reports | `cargo install cargo-llvm-cov` | `cargo llvm-cov --html` |

**When to use:**
- `cargo nextest run` - Every commit, replaces `cargo test`
- `cargo llvm-cov --html` - Weekly coverage review, before releases

**Example:**
```bash
# Run all tests with nice output
cargo nextest run

# Generate HTML coverage report
cargo llvm-cov --html
open target/llvm-cov/html/index.html
```

---

### Documentation Tools (🟠 High)

| Tool | Purpose | Install | Usage |
|------|---------|---------|-------|
| **utoipa** | Auto-generate OpenAPI spec | Add to Cargo.toml | See below |
| **cargo-doc** | Generate Rust docs | Built-in | `cargo doc --open` |

**Setup utoipa:**

1. Add to `ghostwire/Cargo.toml`:
```toml
[dependencies]
utoipa = { version = "4", features = ["axum_extras"] }
utoipa-swagger-ui = { version = "6", features = ["axum"] }
```

2. Add to `src/lib.rs`:
```rust
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api_v2_endpoints::get_profile,
        crate::api_v2_endpoints::update_profile,
    ),
    components(
        schemas(
            crate::api_v2_types::ProfileResponse,
            crate::api_v2_types::UpdateProfileRequest,
        )
    )
)]
pub struct ApiDoc;
```

3. Add Swagger UI to `src/web.rs`:
```rust
use utoipa_swagger_ui::SwaggerUi;

let app = Router::new()
    .merge(SwaggerUi::new("/swagger-ui")
        .url("/api-docs/openapi.json", ApiDoc::openapi()))
    // ... rest of routes
```

4. Access at: `http://localhost:8080/swagger-ui`

---

### Development Tools (🟡 Medium)

| Tool | Purpose | Install | Usage |
|------|---------|---------|-------|
| **cargo-watch** | Auto-rebuild on changes | `cargo install cargo-watch` | `cargo watch -x check` |
| **cargo-outdated** | Check outdated deps | `cargo install cargo-outdated` | `cargo outdated` |
| **cargo-expand** | Debug macro expansion | `cargo install cargo-expand` | `cargo expand` |
| **cargo-make** | Task automation | `cargo install cargo-make` | `cargo make TASK` |

---

### API Testing Tools (🔴 Critical)

#### Bruno API Client (Recommended)

**Why Bruno over Postman:**
- ✅ Stores collections as files in your project (Git-friendly)
- ✅ Works completely offline - no account needed
- ✅ No cloud dependency - all data stays local
- ✅ Unlimited collection runs (Postman limits free tier)
- ✅ Open-source and free

**Install:**
```bash
# Option 1: Download GUI (recommended)
# https://www.usebruno.com/downloads

# Option 2: CLI (for CI/CD)
npm install -g @usebruno/cli
```

**Usage:**
1. Open Bruno
2. Click "Import Collection"
3. Navigate to `ghostwire/bruno-collection/` (create this folder)
4. Start testing endpoints

**Example Bruno Collection:**
```
ghostwire/
└── bruno-collection/
    ├── bruno.json          # Collection config
    ├── profile/
    │   ├── get-profile.bru
    │   └── update-profile.bru
    └── auth/
        └── unlock.bru
```

#### Alternative Tools

| Tool | Best For | Install |
|------|----------|---------|
| **Hoppscotch** | Browser-based testing | https://hoppscotch.io |
| **HTTPie** | CLI + Desktop combo | `cargo install httpie` |
| **Insomnia** | GraphQL support | https://insomnia.rest/download |
| **Thunder Client** | VS Code integration | VS Code Extension |

---

## 🚀 Task Automation with cargo-make

Create `Makefile.toml` in project root:

```toml
[tasks.dev]
description = "Auto-rebuild on file changes"
command = "cargo"
args = ["watch", "-x", "check"]

[tasks.test]
description = "Run all tests"
command = "cargo"
args = ["nextest", "run"]

[tasks.test-coverage]
description = "Generate coverage report"
command = "cargo"
args = ["llvm-cov", "--html"]

[tasks.security-audit]
description = "Run security audit"
command = "cargo"
args = ["audit"]

[tasks.docs]
description = "Build and open documentation"
command = "cargo"
args = ["doc", "--open"]

[tasks.check-all]
description = "Run all checks"
dependencies = ["security-audit", "test", "test-coverage"]

[tasks.build-release]
description = "Build optimized release binary"
command = "cargo"
args = ["build", "--release"]
```

**Usage:**
```bash
cargo make dev              # Auto-rebuild on changes
cargo make test             # Run all tests
cargo make test-coverage    # Generate coverage report
cargo make security-audit   # Run security audit
cargo make docs             # Build documentation
cargo make check-all        # Run everything
cargo make build-release    # Build release binary
```

---

## 📋 Security Policies

### cargo-deny Configuration

Create `deny.toml` in project root:

```toml
[security]
vulnerability = "deny"      # Fail on any vulnerability
unmaintained = "warn"       # Warn for unmaintained crates
yanked = "warn"             # Warn for yanked crates
notice = "warn"             # Warn for notices

[licenses]
unlicensed = "deny"         # Deny unlicensed crates
allow = [
    "MIT",
    "Apache-2.0",
    "AGPL-3.0-or-later",
    "ISC",
    "BSD-3-Clause",
    "Unicode-DFS-2016",
]
deny = [
    "GPL-3.0",              # Copyleft license
]
copyleft = "warn"           # Warn for copyleft licenses

[bans]
multiple-versions = "warn"  # Warn for duplicate crates
deny = [
    # Deny specific crates if needed
]
```

**Usage:**
```bash
# Check all policies
cargo deny check

# Check only security
cargo deny check security

# Check only licenses
cargo deny check licenses
```

---

## 🔄 CI/CD Integration

### GitHub Actions Workflow

Create `.github/workflows/dev-tools.yml`:

```yaml
name: Developer Tools

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always

jobs:
  security:
    name: Security Audit
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Install cargo-audit
        run: cargo install cargo-audit
      
      - name: Run security audit
        run: cargo audit

  deny:
    name: Cargo Deny
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install cargo-deny
        run: cargo install cargo-deny
      
      - name: Run cargo deny
        run: cargo deny check

  test:
    name: Tests
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Install cargo-nextest
        run: cargo install --locked cargo-nextest
      
      - name: Run tests
        run: cargo nextest run --all-features

  coverage:
    name: Code Coverage
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Install cargo-llvm-cov
        run: cargo install cargo-llvm-cov
      
      - name: Generate coverage
        run: cargo llvm-cov --lcov --output-path lcov.info
      
      - name: Upload to Codecov
        uses: codecov/codecov-action@v4
```

---

## 📊 Tool Status Dashboard

Run this command to check all tools:

```bash
echo "=== GhostWire Dev Tools Status ==="
echo ""
echo "Security Tools:"
cargo audit --version 2>/dev/null && echo "  ✓ cargo-audit" || echo "  ✗ cargo-audit"
cargo deny --version 2>/dev/null && echo "  ✓ cargo-deny" || echo "  ✗ cargo-deny"
cargo geiger --version 2>/dev/null && echo "  ✓ cargo-geiger" || echo "  ✗ cargo-geiger"
echo ""
echo "Testing Tools:"
cargo nextest --version 2>/dev/null && echo "  ✓ cargo-nextest" || echo "  ✗ cargo-nextest"
cargo llvm-cov --version 2>/dev/null && echo "  ✓ cargo-llvm-cov" || echo "  ✗ cargo-llvm-cov"
echo ""
echo "Development Tools:"
cargo watch --version 2>/dev/null && echo "  ✓ cargo-watch" || echo "  ✗ cargo-watch"
cargo outdated --version 2>/dev/null && echo "  ✓ cargo-outdated" || echo "  ✗ cargo-outdated"
cargo expand --version 2>/dev/null && echo "  ✓ cargo-expand" || echo "  ✗ cargo-expand"
cargo make --version 2>/dev/null && echo "  ✓ cargo-make" || echo "  ✗ cargo-make"
echo ""
echo "API Testing:"
bruno --version 2>/dev/null && echo "  ✓ Bruno CLI" || echo "  ✗ Bruno (install GUI from usebruno.com)"
```

---

## 🎯 Quick Reference

### Daily Development
```bash
cargo make dev              # Auto-rebuild on changes
cargo make test             # Run tests before commit
```

### Before Commit
```bash
cargo audit                 # Security check
cargo deny check            # Policy check
cargo nextest run           # Run tests
```

### Weekly Maintenance
```bash
cargo outdated              # Check for outdated deps
cargo geiger                # Check for unsafe code
cargo llvm-cov --html       # Review coverage
```

### Before Release
```bash
cargo make check-all        # Run everything
cargo make build-release    # Build release binary
```

---

## 🆘 Troubleshooting

### cargo-nextest installation fails
```bash
# Try specific version
cargo install --locked cargo-nextest --version 0.9.9
```

### cargo-audit database fetch fails
```bash
# Update rustsec-advisory-db
cargo audit --fix
```

### Bruno collections not loading
```bash
# Ensure bruno.json exists in collection folder
# Check file permissions
```

### cargo-make tasks not found
```bash
# Ensure Makefile.toml is in project root
cargo make --list-all-steps
```

---

## 📚 Additional Resources

- [cargo-audit documentation](https://docs.rs/cargo-audit)
- [cargo-deny documentation](https://embarkstudios.github.io/cargo-deny/)
- [cargo-nextest documentation](https://nexte.st/docs/)
- [Bruno documentation](https://docs.usebruno.com/)
- [utoipa documentation](https://docs.rs/utoipa)
- [cargo-make documentation](https://sagiegurari.github.io/cargo-make/)

---

**Last Updated:** February 2026  
**Maintained By:** GhostWire Team
