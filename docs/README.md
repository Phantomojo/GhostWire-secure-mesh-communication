# GhostWire Documentation Index

> Central hub for all GhostWire documentation.

---

## 🚀 Getting Started

**New to GhostWire?** Start here:

1. **[README.md](../README.md)** - Project overview
2. **[INSTALL.md](./INSTALL.md)** - Installation guide
3. **[QUICK_START.md](./QUICK_START.md)** - Quick start guide
4. **[HOW_TO_START_GHOSTWIRE.md](./HOW_TO_START_GHOSTWIRE.md)** - Running GhostWire

---

## 📚 Documentation Categories

### 🔒 Security & API

| Document | Description | Priority |
|----------|-------------|----------|
| **[API_SECURITY.md](./API_SECURITY.md)** | Security checklist for all endpoints | 🔴 Critical |
| **[API_VERSIONING.md](./API_VERSIONING.md)** | API versioning and deprecation policy | 🟠 High |
| **[DEV_TOOLS.md](./DEV_TOOLS.md)** | Developer tools setup guide | 🔴 Critical |
| **[api/DEEP_DIVE.md](./api/DEEP_DIVE.md)** | Comprehensive API documentation | 🟠 High |
| **[SECURITY.md](./SECURITY.md)** | Security policy and disclosure | 🔴 Critical |
| **[SECURITY_AUDIT.md](./SECURITY_AUDIT.md)** | Security audit reports | 🟠 High |

### 🧪 Testing

| Document | Description | Priority |
|----------|-------------|----------|
| **[TESTING_GUIDE.md](./TESTING_GUIDE.md)** | Testing best practices | 🟠 High |
| **[API_TESTING_GUIDE.md](./API_TESTING_GUIDE.md)** | API testing with Bruno | 🟠 High |
| **[POSTMAN_API_GUIDE.md](./POSTMAN_API_GUIDE.md)** | Legacy Postman guide | 🟡 Medium |

### 🏗️ Architecture

| Document | Description | Priority |
|----------|-------------|----------|
| **[ARCHITECTURE.md](./ARCHITECTURE.md)** | System architecture | 🔴 Critical |
| **[IMPLEMENTATION_GUIDE.md](./IMPLEMENTATION_GUIDE.md)** | Implementation guidelines | 🟠 High |
| **[RECOMMENDATIONS.md](./RECOMMENDATIONS.md)** | Architecture recommendations | 🟡 Medium |

### 👥 Contributing

| Document | Description | Priority |
|----------|-------------|----------|
| **[CONTRIBUTING.md](./CONTRIBUTING.md)** | Contribution guidelines | 🔴 Critical |
| **[CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md)** | Code of conduct | 🔴 Critical |
| **[CHANGELOG.md](./CHANGELOG.md)** | Version history | 🟠 High |

---

## 🛠️ Quick Reference

### Common Commands

```bash
# Install all dev tools
./scripts/install-dev-tools.sh

# Run security audit
cargo audit

# Run tests
cargo make test

# Auto-rebuild on changes
cargo make dev

# Generate coverage report
cargo make test-coverage
```

### API Endpoints

**Base URL:** `http://localhost:8080/api/v2`

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/profile` | GET | Get current profile |
| `/profile` | PUT | Update profile |
| `/profile/unlock` | POST | Unlock profile |
| `/profile/lock` | POST | Lock profile |
| `/messages` | GET | List messages |
| `/messages` | POST | Send message |

**Full API Spec:** See [`docs/api/DEEP_DIVE.md`](./api/DEEP_DIVE.md)

### Bruno Collection

Import the Bruno collection for API testing:

1. Open Bruno
2. File → Import Collection
3. Select `bruno-collection/` folder

**Location:** [`../bruno-collection/`](../bruno-collection/)

---

## 📊 Documentation Status

| Category | Complete | In Progress | TODO |
|----------|----------|-------------|------|
| Security | ✅ 80% | 🟡 15% | 🔴 5% |
| API | ✅ 70% | 🟡 20% | 🔴 10% |
| Testing | ✅ 60% | 🟡 30% | 🔴 10% |
| Architecture | ✅ 90% | 🟡 10% | 🔴 0% |
| Contributing | ✅ 100% | 🟡 0% | 🔴 0% |

---

## 🔄 Recent Updates

### February 2026

- ✅ Added `API_SECURITY.md` - Security checklist
- ✅ Added `API_VERSIONING.md` - Versioning policy
- ✅ Added `DEV_TOOLS.md` - Dev tools guide
- ✅ Added `api/DEEP_DIVE.md` - Comprehensive API docs
- ✅ Added Bruno collection for API testing
- ✅ Added `install-dev-tools.sh` script

### January 2026

- ✅ Security audit completed
- ✅ libp2p v0.56 migration
- ✅ Profile lock/unlock feature

---

## 📝 Contributing to Docs

### Adding New Documents

1. Create markdown file in appropriate folder
2. Add to this index
3. Update "Recent Updates" section
4. Submit PR

### Updating Existing Documents

1. Update "Last Updated" timestamp
2. Add changelog entry at bottom
3. Submit PR

### Documentation Standards

- Use clear, concise language
- Include code examples
- Add tables for comparisons
- Link to related documents
- Keep "Last Updated" current

---

## 🆘 Need Help?

- **General Questions:** Check [README.md](../README.md)
- **Installation Issues:** See [INSTALL.md](./INSTALL.md)
- **Security Concerns:** Review [SECURITY.md](./SECURITY.md)
- **API Questions:** Read [api/DEEP_DIVE.md](./api/DEEP_DIVE.md)

---

## 📞 Contact

- **GitHub Issues:** [Report bugs or request features](https://github.com/Phantomojo/GhostWire-secure-mesh-communication/issues)
- **Discussions:** [Ask questions or share ideas](https://github.com/Phantomojo/GhostWire-secure-mesh-communication/discussions)

---

**Last Updated:** February 2026  
**Maintained By:** GhostWire Team
