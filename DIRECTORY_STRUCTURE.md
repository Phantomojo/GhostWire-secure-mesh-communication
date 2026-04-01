# 📁 GhostWire Directory Structure

**Organized:** April 1, 2026  
**Purpose:** Clean, maintainable project structure

---

## 📂 Root Directory Structure

```
GhostWire-secure-mesh-communication/
├── 📄 README.md                 # Project overview (START HERE)
├──  LICENSE                   # AGPL-3.0 license
├──  COMMERCIAL_LICENSE.md     # Commercial licensing options
├── 📄 CODE_OF_CONDUCT.md        # Community guidelines
├── 📄 .gitignore                # Git ignore rules
│
├── 📁 ghostwire/                # Rust backend source code
├── 📁 webui/                    # React frontend source code
├── 📁 src-tauri/                # Tauri mobile app
├── 📁 docs/                     # Documentation
├──  scripts/                  # Automation scripts
├──  config/                   # Configuration files
├── 📁 reports/                  # Status reports and research
├── 📁 archives/                 # Old versions and backups
│
├── 📄 docker-compose.yml        # Docker orchestration
├── 📄 Dockerfile                # Container build instructions
├── 📄 Makefile.toml             # Cargo make tasks
│
└── 📁 .github/                  # GitHub workflows and templates
```

---

## 📁 Directory Descriptions

### **Core Source Code**

| Directory | Purpose | Tech Stack |
|-----------|---------|------------|
| **ghostwire/** | Backend server, mesh networking, encryption | Rust, libp2p |
| **webui/** | Web interface, dashboard, monitoring | React, TypeScript, Vite |
| **src-tauri/** | Mobile app wrapper | Tauri, Rust |

### **Documentation**

| Directory | Contents |
|-----------|----------|
| **docs/** | All user and developer documentation |
| **docs/presentations/** | Slide decks and talks |
| **docs/wiki/** | Technical deep-dives |
| **docs/api/** | API reference documentation |

### **Scripts**

| Directory | Purpose |
|-----------|---------|
| **scripts/** | All automation scripts |
| **scripts/build/** | Build scripts (install, compile) |
| **scripts/test/** | Testing scripts |
| **scripts/deploy/** | Deployment scripts |
| **scripts/maintenance/** | Cleanup, disk management |
| **scripts/docker/** | Docker-related scripts |

### **Configuration**

| Directory | Contents |
|-----------|----------|
| **config/** | Configuration templates |
| ↳ `ghostwire.toml` | Main configuration |
| ↳ `ghostwire.toml.example` | Example configuration |
| ↳ `deny.toml` | Cargo deny rules |

### **Reports & Research**

| Directory | Contents |
|-----------|----------|
| **reports/** | Status reports, research documents |
| **reports/android/** | Android development reports |

### **Archives**

| Directory | Contents |
|-----------|----------|
| **archives/** | Old versions, backups, large zips |
| **archives/versions/** | Historical GhostWire versions |

⚠️ **Note:** Archives are gitignored - not tracked in repository

---

## 🎯 Quick Navigation

### **For New Users:**
1. Start at [`README.md`](README.md)
2. Read [`docs/INSTALL.md`](docs/INSTALL.md)
3. Check [`docs/QUICK_START.md`](docs/QUICK_START.md)

### **For Developers:**
1. Review [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)
2. See [`docs/CONTRIBUTING.md`](docs/CONTRIBUTING.md)
3. Check [`docs/TESTING_GUIDE.md`](docs/TESTING_GUIDE.md)

### **For Commercial Users:**
1. Read [`COMMERCIAL_LICENSE.md`](COMMERCIAL_LICENSE.md)
2. Review [`LICENSE_EXPLANATION.md`](LICENSE_EXPLANATION.md)
3. Contact: mirungu015@proton.me

---

## 📊 File Organization Stats

| Category | Count | Location |
|----------|-------|----------|
| **Source Code** | ~15,000 LOC | `ghostwire/`, `webui/` |
| **Documentation** | 50+ files | `docs/` |
| **Scripts** | 30+ files | `scripts/` |
| **Config Files** | 5 files | `config/` |
| **Reports** | 20+ files | `reports/` |
| **Archives** | 10+ zips | `archives/` (gitignored) |

---

## 🧹 Cleanup History

**Last Organized:** April 1, 2026

**Changes Made:**
- ✅ Moved 140+ root files into organized subdirectories
- ✅ Created `scripts/` with categorized subfolders
- ✅ Created `config/` for all configuration files
- ✅ Created `reports/` for status documents
- ✅ Created `archives/` for old versions (gitignored)
- ✅ Consolidated documentation in `docs/`

**Before:** 140 files in root  
**After:** ~30 essential files in root

---

## 📝 File Naming Conventions

### **Scripts:**
- Use kebab-case: `install-tools.sh`, `build-android.sh`
- Prefix with action: `test-`, `build-`, `deploy-`, `cleanup-`

### **Documentation:**
- Use UPPERCASE for main guides: `INSTALL.md`, `ARCHITECTURE.md`
- Use Title Case for reports: `SecurityAuditReport.md`

### **Configuration:**
- Use `.toml` extension
- Include `.example` for templates

---

## 🔍 Finding Files

### **Looking for a script?**
```bash
# List all scripts
find scripts/ -name "*.sh"

# Find build scripts
ls scripts/build/
```

### **Looking for documentation?**
```bash
# Search docs
grep -r "search term" docs/

# List all guides
ls docs/*.md
```

### **Looking for old versions?**
```bash
# Check archives
ls archives/versions/
```

---

## 🚀 Maintenance

### **Adding New Files:**

1. **Scripts** → `scripts/` (appropriate subfolder)
2. **Documentation** → `docs/`
3. **Configuration** → `config/`
4. **Reports** → `reports/`
5. **Old versions** → `archives/versions/`

### **Before Committing:**

```bash
# Check what you're adding
git status

# Verify organization
tree -L 2 -I 'target|node_modules'
```

---

## ✅ Checklist for Clean Root

- [ ] Only essential files in root (README, LICENSE, docker-compose.yml)
- [ ] All scripts in `scripts/`
- [ ] All configs in `config/`
- [ ] All docs in `docs/`
- [ ] All reports in `reports/`
- [ ] All old zips in `archives/` (gitignored)
- [ ] No duplicate files

---

**Maintained By:** Team GhostWire  
**Last Updated:** April 1, 2026
