# GhostWire API Versioning Policy

> Guidelines for evolving GhostWire APIs while maintaining backward compatibility.

---

## 📋 Version Format

GhostWire APIs use **path-based versioning**:

```
/api/v{major}/{resource}
```

**Examples:**
```
GET  /api/v2/profile
POST /api/v2/messages
GET  /api/v3/profile  (future version)
```

---

## 🔄 Versioning Strategy

### Hybrid Approach: Evolution + Explicit Versioning

#### Non-Breaking Changes (Evolution)

Make these changes **without** incrementing the version:

- Adding new endpoints
- Adding optional query parameters
- Adding optional request fields
- Adding response fields
- Improving performance
- Fixing bugs

**Example:**
```rust
// v2 endpoint - adding optional field (backward compatible)
GET /api/v2/profile?fields=bio,nickname  // New query param
```

#### Breaking Changes (Explicit Versioning)

Increment the **major version** for:

- Removing an endpoint
- Removing required fields
- Changing field types
- Changing authentication requirements
- Changing error response format
- Changing pagination behavior

**Example:**
```rust
// v3 endpoint - breaking change
GET /api/v3/profile  // New version with different response structure
```

---

## 📅 Deprecation Timeline

```
┌─────────────┬──────────────┬────────────┐
│ Announcement│   Migration  │   Sunset   │
│   (Day 0)   │   (6 months) │  (9 months)│
└─────────────┴──────────────┴────────────┘
```

### Phase 1: Announcement (Month 0)

- Add `Deprecation` header to endpoint
- Update documentation
- Notify users via changelog
- Add migration guide

**Example:**
```http
HTTP/1.1 200 OK
Deprecation: true
Sunset: 2026-11-18
Link: </api/v3/profile>; rel="successor-version"
```

### Phase 2: Active Migration (Months 1-6)

- Log usage of deprecated endpoints
- Send reminders to active users
- Provide migration support
- Monitor adoption of new version

### Phase 3: Sunset (Month 9)

- Remove deprecated endpoint
- Return 410 Gone with migration info
- Keep logs for 30 days

**Example:**
```http
HTTP/1.1 410 Gone
Content-Type: application/json

{
  "error": "ENDPOINT_DEPRECATED",
  "message": "This endpoint was deprecated on 2026-02-18",
  "migration": {
    "successor": "/api/v3/profile",
    "guide": "https://docs.ghostwire.io/migration/v2-to-v3"
  }
}
```

---

## 🚨 Breaking Change Definition

### What Counts as Breaking?

| Change Type | Breaking? | Example |
|-------------|-----------|---------|
| Remove endpoint | ✅ Yes | `DELETE /api/v2/profile` |
| Remove required field | ✅ Yes | Remove `nickname` from request |
| Change field type | ✅ Yes | `id: string` → `id: integer` |
| Change auth requirements | ✅ Yes | Require token where none was needed |
| Change error format | ✅ Yes | `{error: string}` → `{errors: []}` |
| Change pagination | ✅ Yes | Offset → Cursor-based |
| Add new endpoint | ❌ No | `POST /api/v2/profile/avatar` |
| Add optional field | ❌ No | Add `bio` to request |
| Add response field | ❌ No | Add `created_at` to response |
| Add query param | ❌ No | `?include=metadata` |
| Performance improvement | ❌ No | Faster response time |

---

## 📝 Documentation Requirements

### For Each Version

1. **OpenAPI Specification**
   - Maintain separate spec per major version
   - Auto-generate with utoipa
   - Host at `/api-docs/v{version}/openapi.json`

2. **Changelog**
   - Document all changes
   - Mark breaking changes clearly
   - Include migration notes

3. **Migration Guides**
   - One guide per major version transition
   - Before/after examples
   - Code snippets for common use cases

**Example Structure:**
```
docs/
├── api/
│   ├── v2/
│   │   ├── openapi.json
│   │   └── reference.md
│   ├── v3/
│   │   ├── openapi.json
│   │   └── reference.md
│   └── migration/
│       └── v2-to-v3.md
```

---

## 🔧 Implementation Guidelines

### Adding Deprecation Header

```rust
use axum::{
    response::{AppendHeaders, IntoResponse},
    Json,
};
use http::header::{Deprecation, Sunset, Link};

pub async fn get_profile_deprecated() -> impl IntoResponse {
    (
        AppendHeaders([
            (Deprecation::name(), "true"),
            (Sunset::name(), "2026-11-18"),
            (Link::name(), "</api/v3/profile>; rel=\"successor-version\""),
        ]),
        Json(profile),
    )
}
```

### Version Routing

```rust
let app = Router::new()
    // v2 routes
    .nest("/api/v2", v2_routes())
    // v3 routes
    .nest("/api/v3", v3_routes())
    // Default to latest version
    .route("/api/profile", get(latest_version_handler));
```

### Version Detection

```rust
pub async fn check_deprecated_usage(
    request: Request,
    next: Next,
) -> impl IntoResponse {
    let path = request.uri().path();
    
    // Log usage of deprecated endpoints
    if path.contains("/api/v2/") {
        tracing::warn!(deprecated_endpoint = %path, "Deprecated API version in use");
    }
    
    next.run(request).await
}
```

---

## 📊 Version Support Matrix

| Version | Released | Deprecated | Sunset | Status |
|---------|----------|------------|--------|--------|
| v1 | 2025-01-01 | 2025-06-01 | 2025-09-01 | ❌ Removed |
| v2 | 2025-06-01 | - | - | ✅ Current |
| v3 | - | - | - | 🚧 In Development |

---

## 🎯 Decision Framework

### When to Create a New Version

**Create v3 when:**
- You need to remove a field or endpoint
- You need to change a field type
- You need to change authentication
- The API has fundamental design issues

**Don't create v3 for:**
- Adding new features
- Adding optional fields
- Performance improvements
- Bug fixes

### When to Deprecate

**Deprecate v2 when:**
- v3 is stable and tested
- Migration guide is complete
- You've given 3 months notice
- Less than 10% of users are on v2

---

## ✅ Checklist for New Versions

### Before Releasing v3

- [ ] OpenAPI specification complete
- [ ] All endpoints tested
- [ ] Migration guide written
- [ ] Documentation updated
- [ ] Deprecation headers ready for v2
- [ ] Changelog entry created
- [ ] Team trained on new version

### During Migration Period

- [ ] Monitor v2 usage weekly
- [ ] Send monthly reminders to v2 users
- [ ] Provide migration support
- [ ] Track migration progress

### Before Sunsetting v2

- [ ] Confirm <1% usage
- [ ] Send final 30-day notice
- [ ] Prepare 410 response handler
- [ ] Update documentation
- [ ] Monitor for errors after sunset

---

## 📚 Examples

### Good: Backward-Compatible Evolution

```rust
// v2 - Original endpoint
GET /api/v2/profile
Response: { id, nickname }

// v2 - Add optional field (still v2)
GET /api/v2/profile?include=bio
Response: { id, nickname, bio }  // bio is optional
```

### Bad: Breaking Change Without Version

```rust
// v2 - Original
GET /api/v2/profile
Response: { id: string, nickname: string }

// v2 - Breaking change! (Don't do this)
GET /api/v2/profile
Response: { id: integer, name: string }  // Changed type and field name!
```

### Good: Breaking Change with New Version

```rust
// v2 - Still supported
GET /api/v2/profile
Response: { id: string, nickname: string }

// v3 - Breaking changes clearly versioned
GET /api/v3/profile
Response: { id: integer, name: string }  // New version, different contract
```

---

## 🆘 FAQ

**Q: How long do you support old versions?**  
A: 9 months from deprecation announcement (3 months notice + 6 months migration).

**Q: Can I use multiple versions simultaneously?**  
A: Yes! Clients can use different versions concurrently.

**Q: What if there's a security vulnerability in an old version?**  
A: We patch all supported versions. Deprecated versions may not receive patches.

**Q: How do I know which version to use?**  
A: Always use the latest stable version unless you have a specific reason not to.

---

**Last Updated:** February 2026  
**Current Version:** v2  
**Next Version:** v3 (planned)
