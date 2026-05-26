# GhostWire API Comprehensive Deep Dive

> Complete API architecture, security, testing, and development guide.  
> **Last Updated:** February 2026

---

## 📊 Table of Contents

1. [Executive Summary](#executive-summary)
2. [Current State Analysis](#current-state-analysis)
3. [Security Deep Dive](#security-deep-dive)
4. [Architecture Deep Dive](#architecture-deep-dive)
5. [Testing Strategy](#testing-strategy)
6. [Development Workflow](#development-workflow)
7. [API Reference](#api-reference)
8. [Migration Guides](#migration-guides)

---

## Executive Summary

### GhostWire API Overview

**Total Endpoints:** 34  
**API Version:** v2  
**Security Score:** 62/100 (Needs Improvement)  
**Test Coverage:** TBD  

### Critical Action Items

| Priority | Issue | Status | ETA |
|----------|-------|--------|-----|
| 🔴 | Rate limiting on unlock endpoint | ⚠️ Partial | 2 hours |
| 🔴 | Account lockout after failed attempts | ❌ Missing | 3 hours |
| 🔴 | Audit logging for security events | ❌ Missing | 4 hours |

### Quick Links

- **Security Checklist:** [`docs/API_SECURITY.md`](./API_SECURITY.md)
- **Versioning Policy:** [`docs/API_VERSIONING.md`](./API_VERSIONING.md)
- **Dev Tools Setup:** [`docs/DEV_TOOLS.md`](./DEV_TOOLS.md)
- **Bruno Collection:** [`bruno-collection/`](../bruno-collection/)

---

## Current State Analysis

### API Inventory

| Module | Endpoints | Status | Security |
|--------|-----------|--------|----------|
| **Identity** | 12 | ✅ Active | 🟡 Medium |
| **Messaging** | 6 | ✅ Active | 🟢 High |
| **Peers** | 4 | ✅ Active | 🟢 High |
| **System** | 3 | ✅ Active | 🟢 High |
| **Profiles** | 8 | ✅ Active | 🔴 Low (NEW) |
| **TOTAL** | **34** | | **62/100** |

### Architecture Diagram

```
┌─────────────────────────────────────────────────────────┐
│                    Frontend (React)                      │
│                         WebUI                            │
└─────────────────────────────────────────────────────────┘
                          ↓ WebSocket / HTTP
┌─────────────────────────────────────────────────────────┐
│                   Axum Web Server                        │
│  ┌─────────────────────────────────────────────────┐    │
│  │              API v2 Endpoints                    │    │
│  │  /api/v2/identity/*  /api/v2/messages/*         │    │
│  │  /api/v2/profile/*   /api/v2/peers/*            │    │
│  └─────────────────────────────────────────────────┘    │
│                          ↓                               │
│  ┌─────────────────────────────────────────────────┐    │
│  │              Controllers Layer                   │    │
│  │  IdentityController, MessagingController, etc.   │    │
│  └─────────────────────────────────────────────────┘    │
│                          ↓                               │
│  ┌─────────────────────────────────────────────────┐    │
│  │                 Core Module                      │    │
│  │  Core, MeshManager, Encryption, SecurityManager  │    │
│  └─────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────┘
```

### OWASP API Top 10 Assessment

| Risk | Coverage | Priority |
|------|----------|----------|
| API1: Broken Object Level Authorization | 70% | 🟠 High |
| API2: Broken Authentication | 30% | 🔴 **Critical** |
| API3: Broken Object Property Level | 60% | 🟠 High |
| API4: Unrestricted Resource Consumption | 20% | 🔴 **Critical** |
| API5: Broken Function Level Authorization | 90% | 🟢 Low |
| API6: Unrestricted Business Flow Access | 50% | 🟠 High |
| API7: SSRF | 100% | 🟢 Low (N/A) |
| API8: Security Misconfiguration | 40% | 🟠 High |
| API9: Improper Inventory Management | 60% | 🟡 Medium |
| API10: Unsafe API Consumption | 100% | 🟢 Low (N/A) |

---

## Security Deep Dive

### Critical Security Gaps

#### 1. No Rate Limiting on Sensitive Endpoints

**Affected Endpoints:**
- `POST /api/v2/profile/unlock`
- `POST /api/v2/profile/lock`
- `POST /api/v2/profile/save`

**Risk:** Brute-force attacks on profile passwords

**Solution:**
```rust
use governor::{Quota, RateLimiter, state::keyed::DashMapStateStore};
use std::{num::NonZeroU32, time::Duration, net::IpAddr};

pub fn sensitive_rate_limit() -> RateLimiter<DashMapStateStore<IpAddr>> {
    let quota = Quota::with_period(Duration::from_secs(60))
        .allow_burst(NonZeroU32::new(5).unwrap());
    RateLimiter::dashmap(quota)
}
```

**Implementation Guide:** See [`docs/API_SECURITY.md`](./API_SECURITY.md#1-rate-limiting-on-authentication-endpoints)

---

#### 2. No Account Lockout

**Risk:** Credential stuffing attacks

**Solution:** Implement `FailedAttemptTracker` with:
- 5 failed attempts threshold
- 15-minute lockout duration
- Automatic unlock after timeout

**Implementation Guide:** See [`docs/API_SECURITY.md`](./API_SECURITY.md#2-account-lockout-after-failed-attempts)

---

#### 3. No Audit Logging

**Risk:** No visibility into attacks

**Solution:** Implement `AuditLogger` for:
- Profile lock/unlock events
- Failed authentication attempts
- Rate limit violations
- Account lockouts

**Implementation Guide:** See [`docs/API_SECURITY.md`](./API_SECURITY.md#3-audit-logging-for-security-events)

---

### Security Headers

**Required Headers:**
```rust
Strict-Transport-Security: max-age=31536000; includeSubDomains
X-Content-Type-Options: nosniff
X-Frame-Options: DENY
X-XSS-Protection: 1; mode=block
```

---

## Architecture Deep Dive

### Proposed Service Layer

**Current:** Controllers → Core (direct)  
**Problem:** Core mixes concerns  
**Solution:** Add service layer

```
Controllers → Services → Core → Repositories
```

### Service Example

```rust
pub struct IdentityService {
    core: Arc<RwLock<Core>>,
    audit_logger: AuditLogger,
    attempt_tracker: Arc<FailedAttemptTracker>,
}

impl IdentityService {
    pub async fn unlock_profile(
        &self,
        profile_id: &str,
        password: &str,
        ip: &str,
    ) -> Result<(), IdentityError> {
        // 1. Check lockout
        // 2. Attempt unlock
        // 3. Record success/failure
        // 4. Audit log
    }
}
```

---

## Testing Strategy

### Test Pyramid

```
           /\
          /  \       E2E Tests (10%)
         /----\      
        /      \    
       /--------\   Integration Tests (30%)
      /          \  
     /------------\ 
    /              \  Unit Tests (60%)
   /----------------\
```

### Integration Test Example

```rust
// tests/profile_tests.rs
mod common;

#[tokio::test]
async fn test_unlock_profile_rate_limiting() {
    let ctx = TestContext::new().await;
    
    // 5 successful attempts
    for i in 0..5 {
        let response = unlock(&ctx).await;
        assert_eq!(response.status(), StatusCode::OK);
    }
    
    // 6th should be rate limited
    let response = unlock(&ctx).await;
    assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
}
```

**Testing Guide:** See [`docs/TESTING.md`](./TESTING.md) (TODO: Create)

---

## Development Workflow

### Daily Development

```bash
# Auto-rebuild on changes
cargo make dev

# Run tests before commit
cargo make test
```

### Before Commit

```bash
# Security check
cargo audit

# Policy check
cargo deny check

# Run tests
cargo nextest run
```

### Weekly Maintenance

```bash
# Check outdated deps
cargo outdated

# Check unsafe code
cargo geiger

# Review coverage
cargo llvm-cov --html
```

**Dev Tools Guide:** See [`docs/DEV_TOOLS.md`](./DEV_TOOLS.md)

---

## API Reference

### Identity Endpoints

| Method | Endpoint | Description | Auth |
|--------|----------|-------------|------|
| GET | `/api/v2/identity` | Get current identity | No |
| POST | `/api/v2/identity/setup` | Initial setup | No |
| PUT | `/api/v2/identity` | Update identity | Yes |

### Profile Endpoints

| Method | Endpoint | Description | Auth | Rate Limit |
|--------|----------|-------------|------|------------|
| GET | `/api/v2/profile` | Get profile | No | Standard |
| PUT | `/api/v2/profile` | Update profile | Yes | Standard |
| POST | `/api/v2/profile/lock` | Lock profile | Yes | **Sensitive** |
| POST | `/api/v2/profile/unlock` | Unlock profile | No | **Sensitive** |
| POST | `/api/v2/profile/save` | Save profile | Yes | **Sensitive** |
| GET | `/api/v2/profile/status` | Get lock status | No | Standard |
| GET | `/api/v2/profile/public-key` | Get public key | No | Standard |
| DELETE | `/api/v2/profile` | Delete profile | Yes | Standard |

### Messaging Endpoints

| Method | Endpoint | Description | Auth |
|--------|----------|-------------|------|
| GET | `/api/v2/messages` | List messages | Yes |
| POST | `/api/v2/messages` | Send message | Yes |
| GET | `/api/v2/messages/:id` | Get message | Yes |
| DELETE | `/api/v2/messages/:id` | Delete message | Yes |

**Full API Spec:** Access Swagger UI at `http://localhost:9097/swagger-ui` (after utoipa setup)

---

## Migration Guides

### v2 to v3 (Planned)

**Status:** 🚧 In Development  
**Estimated Release:** Q3 2026

**Breaking Changes:**
- Profile ID format: string → integer
- Authentication: Required on all endpoints
- Response format: Standardized error structure

**Migration Steps:**
1. Update client to handle new ID format
2. Add authentication to all requests
3. Update error handling

**Migration Guide:** See `docs/migration/v2-to-v3.md` (TODO: Create)

---

## Tools & Resources

### Required Tools

| Tool | Purpose | Install |
|------|---------|---------|
| cargo-audit | Security audit | `cargo install cargo-audit` |
| cargo-nextest | Test runner | `cargo install --locked cargo-nextest` |
| Bruno | API testing | https://usebruno.com |
| cargo-make | Task automation | `cargo install cargo-make` |

**Full Tool List:** See [`docs/DEV_TOOLS.md`](./DEV_TOOLS.md)

### Bruno Collection

Import the Bruno collection for API testing:

```bash
# Open Bruno
# File → Import Collection
# Navigate to bruno-collection/
```

**Collection Location:** [`bruno-collection/`](../bruno-collection/)

---

## Implementation Roadmap

### Phase 1: Critical Security (Week 1-2)

| Task | Effort | Priority |
|------|--------|----------|
| Rate limiting on unlock | 2 hours | 🔴 Critical |
| Account lockout tracker | 3 hours | 🔴 Critical |
| Audit logging | 4 hours | 🔴 Critical |
| Security headers | 2 hours | 🟠 High |

### Phase 2: Architecture (Week 3-4)

| Task | Effort | Priority |
|------|--------|----------|
| Service layer implementation | 8 hours | 🟠 High |
| Repository pattern | 8 hours | 🟡 Medium |
| Controller refactoring | 12 hours | 🟡 Medium |

### Phase 3: Testing (Week 5-6)

| Task | Effort | Priority |
|------|--------|----------|
| Integration test framework | 6 hours | 🟠 High |
| Profile endpoint tests | 8 hours | 🟠 High |
| Auth endpoint tests | 6 hours | 🟡 Medium |
| CI pipeline | 4 hours | 🟡 Medium |

---

## Monitoring & Observability

### Key Metrics

| Metric | Alert Threshold | Action |
|--------|-----------------|--------|
| Auth failures > 100/min | 🔴 Critical | Investigate brute force |
| Rate limit hits > 50/min | 🟠 Warning | Review limits |
| P99 latency > 500ms | 🟠 Warning | Performance investigation |
| Error rate > 5% | 🔴 Critical | Rollback or hotfix |

### Logging

**Log Levels:**
- ERROR: Security events, failures
- WARN: Rate limits, validation failures
- INFO: Successful operations
- DEBUG: Request/response details

**Log Location:** `logs/ghostwire.log`  
**Audit Log:** `logs/audit.log` (TODO: Implement)

---

## FAQ

### Q: How do I test the unlock endpoint?

**A:** Use Bruno:
1. Open Bruno
2. Import `bruno-collection/`
3. Navigate to `auth/unlock-profile.bru`
4. Replace password with your test password
5. Click Send

### Q: Where are API logs?

**A:** Check `logs/ghostwire.log` for request logs. Audit logs coming soon.

### Q: How do I add rate limiting to a new endpoint?

**A:** See [`docs/API_SECURITY.md`](./API_SECURITY.md#1-rate-limiting-on-authentication-endpoints)

### Q: When is v3 releasing?

**A:** Planned for Q3 2026. See [`docs/API_VERSIONING.md`](./API_VERSIONING.md)

---

## Contributing

### Adding New Endpoints

1. Create handler in `src/controllers/`
2. Add route in `src/web.rs`
3. Add to Bruno collection
4. Write integration tests
5. Update this documentation

### Security Review Process

1. Run `cargo audit`
2. Run `cargo deny check`
3. Manual security review against [`docs/API_SECURITY.md`](./API_SECURITY.md)
4. Test with Bruno collection
5. PR review

---

**Document Maintainer:** GhostWire Team  
**Last Reviewed:** February 2026  
**Next Review:** March 2026
