# GhostWire API Security Checklist

> Critical security requirements for all GhostWire API endpoints. Based on OWASP API Security Top 10 (2026).

## 🔴 Critical (Must Implement Before Production)

### 1. Rate Limiting on Authentication Endpoints

**Status:** ⚠️ **PARTIAL** - Missing on `/api/v2/profile/unlock`

**Risk:** Brute-force attacks on profile passwords

**Required Implementation:**

```rust
// Add to src/middleware/rate_limit.rs

use governor::{Quota, RateLimiter, state::keyed::DashMapStateStore};
use std::{num::NonZeroU32, time::Duration, net::IpAddr};

/// Rate limiter for sensitive endpoints (auth, unlock, etc.)
pub fn sensitive_rate_limit() -> RateLimiter<DashMapStateStore<IpAddr>> {
    let quota = Quota::with_period(Duration::from_secs(60))
        .allow_burst(NonZeroU32::new(5).unwrap());  // 5 requests per minute
    RateLimiter::dashmap(quota)
}

// Apply to unlock endpoint
pub async fn unlock_profile(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Json(req): Json<UnlockRequest>,
) -> impl IntoResponse {
    let limiter = state.sensitive_rate_limiter.clone();
    
    if limiter.check_key(&addr.ip()).is_err() {
        return Json(ApiV2Response::error(
            "RATE_LIMIT_EXCEEDED",
            "Too many requests. Please wait 60 seconds.",
        )).into_response();
    }
    
    // ... rest of unlock logic
}
```

**Verification:**
- [ ] Rate limiter installed (`cargo install cargo-nextest && cargo add governor`)
- [ ] Middleware applied to `/api/v2/profile/unlock`
- [ ] Middleware applied to `/api/v2/profile/lock`
- [ ] Middleware applied to `/api/v2/profile/save`
- [ ] Test: 6 rapid requests should return 429 Too Many Requests

---

### 2. Account Lockout After Failed Attempts

**Status:** ❌ **NOT IMPLEMENTED**

**Risk:** Credential stuffing attacks

**Required Implementation:**

```rust
// Add to src/core/security/attempt_tracker.rs

pub struct FailedAttemptTracker {
    attempts: DashMap<String, Vec<Instant>>,
    lockout_threshold: usize,      // 5 attempts
    lockout_duration: Duration,    // 15 minutes
    locked_until: DashMap<String, Instant>,
}

impl FailedAttemptTracker {
    pub fn record_failure(&self, profile_id: &str) -> bool {
        // Returns false if account is now locked
    }
    
    pub fn record_success(&self, profile_id: &str) {
        // Clear failed attempts
    }
    
    pub fn is_locked(&self, profile_id: &str) -> bool {
        // Check if account is locked
    }
}

// Usage in unlock endpoint
if !attempt_tracker.record_failure(&profile_id) {
    return Json(ApiV2Response::error(
        "ACCOUNT_LOCKED",
        "Too many failed attempts. Account locked for 15 minutes.",
    )).into_response();
}
```

**Verification:**
- [ ] FailedAttemptTracker implemented
- [ ] Integrated into unlock endpoint
- [ ] After 5 failed attempts, account locks for 15 minutes
- [ ] Successful unlock clears failed attempts

---

### 3. Audit Logging for Security Events

**Status:** ❌ **NOT IMPLEMENTED**

**Risk:** No visibility into attacks or unauthorized access

**Required Implementation:**

```rust
// Add to src/core/security/audit_logger.rs

#[derive(Debug)]
pub enum SecurityEvent {
    ProfileLocked { profile_id: String },
    ProfileUnlocked { profile_id: String },
    UnlockFailed { profile_id: String, reason: String },
    AccountLockout { profile_id: String, failed_attempts: usize },
    RateLimitExceeded { ip: String, endpoint: String },
}

pub struct AuditLogger;

impl AuditLogger {
    pub fn log(event: SecurityEvent, ip: Option<String>) {
        match event {
            SecurityEvent::UnlockFailed { .. } => {
                warn!(audit = ?event, "Unlock attempt failed");
            }
            SecurityEvent::AccountLockout { .. } => {
                error!(audit = ?event, "Account locked due to failed attempts");
            }
            _ => info!(audit = ?event, "Security event"),
        }
        // Persist to audit log file
    }
}
```

**Verification:**
- [ ] AuditLogger implemented
- [ ] All auth events logged
- [ ] Logs written to `logs/audit.log`
- [ ] Log rotation configured (daily)

---

## 🟠 High Priority (Should Implement)

### 4. Input Validation on All Endpoints

**Status:** 🟡 **PARTIAL** - Some endpoints validate, others don't

**Required:**
- [ ] All request bodies validated with `validator` crate
- [ ] String length limits enforced
- [ ] Email format validated
- [ ] Sanitize user input before storage

**Example:**
```rust
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct UpdateProfileRequest {
    #[validate(length(min = 1, max = 100))]
    pub nickname: Option<String>,
    
    #[validate(length(max = 500))]
    pub bio: Option<String>,
}
```

---

### 5. Authorization Checks on Object Access

**Status:** 🟡 **PARTIAL** - BOLA risk

**Risk:** Broken Object Level Authorization (OWASP API1)

**Required:**
- [ ] All object access checks user ownership
- [ ] Admin endpoints require admin role
- [ ] Profile endpoints check profile ownership

**Example:**
```rust
pub async fn get_profile(
    user: AuthUser,
    Path(profile_id): Path<String>,
) -> Result<Json<ProfileResponse>> {
    // Verify user owns this profile
    if user.id != profile_id {
        return Err(AppError::Forbidden);
    }
    // ... rest of logic
}
```

---

### 6. Security Headers

**Status:** ❌ **NOT IMPLEMENTED**

**Required:**
```rust
// Add to src/web.rs middleware stack

use tower_http::set_header::SetResponseHeaderLayer;

let app = Router::new()
    // ... routes
    .layer(SetResponseHeaderLayer::overriding(
        header::STRICT_TRANSPORT_SECURITY,
        "max-age=31536000; includeSubDomains",
    ))
    .layer(SetResponseHeaderLayer::overriding(
        header::X_CONTENT_TYPE_OPTIONS,
        "nosniff",
    ))
    .layer(SetResponseHeaderLayer::overriding(
        header::X_FRAME_OPTIONS,
        "DENY",
    ));
```

---

## 🟡 Medium Priority (Nice to Have)

### 7. API Versioning Policy

**Status:** ⚠️ **DOCUMENTED** - See `docs/API_VERSIONING.md`

**Required:**
- [ ] Deprecation policy documented
- [ ] Sunset headers on deprecated endpoints
- [ ] Migration guides for major versions

---

### 8. Request/Response Logging

**Status:** 🟡 **PARTIAL** - Basic tracing exists

**Required:**
- [ ] Log all requests with request ID
- [ ] Log response status and duration
- [ ] Redact sensitive fields (passwords, tokens)

---

## ✅ Security Checklist Summary

| # | Requirement | Status | Priority | ETA |
|---|-------------|--------|----------|-----|
| 1 | Rate limiting on auth endpoints | ⚠️ Partial | 🔴 Critical | 2 hours |
| 2 | Account lockout after failed attempts | ❌ Missing | 🔴 Critical | 3 hours |
| 3 | Audit logging for security events | ❌ Missing | 🔴 Critical | 4 hours |
| 4 | Input validation on all endpoints | 🟡 Partial | 🟠 High | 4 hours |
| 5 | Authorization checks on object access | 🟡 Partial | 🟠 High | 6 hours |
| 6 | Security headers | ❌ Missing | 🟠 High | 2 hours |
| 7 | API versioning policy | ✅ Documented | 🟡 Medium | N/A |
| 8 | Request/response logging | 🟡 Partial | 🟡 Medium | 4 hours |

---

## 🧪 Security Testing

### Manual Testing Checklist

**Rate Limiting:**
```bash
# Should succeed (5 requests)
for i in {1..5}; do
  curl -X POST http://localhost:8080/api/v2/profile/unlock \
    -H "Content-Type: application/json" \
    -d '{"password":"test"}'
done

# Should fail (429 Too Many Requests)
curl -X POST http://localhost:8080/api/v2/profile/unlock \
  -H "Content-Type: application/json" \
  -d '{"password":"test"}'
```

**Account Lockout:**
```bash
# 5 failed attempts
for i in {1..5}; do
  curl -X POST http://localhost:8080/api/v2/profile/unlock \
    -H "Content-Type: application/json" \
    -d '{"password":"wrong"}'
done

# 6th attempt should return ACCOUNT_LOCKED
curl -X POST http://localhost:8080/api/v2/profile/unlock \
  -H "Content-Type: application/json" \
  -d '{"password":"test"}'
```

**Audit Logging:**
```bash
# Check audit log exists
ls -la logs/audit.log

# Verify events are logged
tail -f logs/audit.log
```

---

## 📊 OWASP API Top 10 Coverage

| OWASP Risk | Coverage | Status |
|------------|----------|--------|
| API1: Broken Object Level Authorization | 🟡 70% | Needs work |
| API2: Broken Authentication | 🔴 30% | **Critical** |
| API3: Broken Object Property Level | 🟡 60% | Needs work |
| API4: Unrestricted Resource Consumption | 🔴 20% | **Critical** |
| API5: Broken Function Level Authorization | 🟢 90% | Good |
| API6: Unrestricted Business Flow Access | 🟡 50% | Needs work |
| API7: SSRF | 🟢 100% | N/A (no external fetches) |
| API8: Security Misconfiguration | 🟡 40% | Needs work |
| API9: Improper Inventory Management | 🟡 60% | Needs work |
| API10: Unsafe API Consumption | 🟢 100% | N/A (internal APIs) |

**Overall Security Score:** 🟡 **62/100** (Needs Improvement)

---

## 🎯 Next Steps

### This Week (Critical)
1. [ ] Implement rate limiting on unlock endpoint
2. [ ] Implement account lockout tracker
3. [ ] Add audit logging
4. [ ] Run `cargo audit` and fix vulnerabilities

### Next Week (High)
5. [ ] Add input validation to all endpoints
6. [ ] Add authorization checks
7. [ ] Add security headers

### This Month (Medium)
8. [ ] Implement request/response logging
9. [ ] Document API versioning policy
10. [ ] Add automated security scanning to CI

---

**Last Updated:** February 2026  
**Security Review Due:** Monthly  
**Next Review:** March 2026
