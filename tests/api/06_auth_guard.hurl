# ============================================================
# GhostWire API Test Suite - 06: Auth Guard Verification
# ============================================================
# Verifies that protected endpoints reject unauthenticated requests
# and that exempt endpoints allow them.
# Run: hurl --test tests/api/06_auth_guard.hurl
# ============================================================

# --- Protected endpoint WITHOUT token → should be rejected ---
GET http://localhost:3001/api/peers
HTTP 401

# --- Protected endpoint with INVALID token → should be rejected ---
GET http://localhost:3001/api/peers
X-Ghost-Token: fake_invalid_token_12345
HTTP 401

# --- Auth-exempt endpoint WITHOUT token → should work ---
GET http://localhost:3001/api/profile/list
HTTP 200
[Asserts]
jsonpath "$.data" exists

# --- Auth-exempt identity endpoint WITHOUT token → should work ---
GET http://localhost:3001/api/identity/status
HTTP 200
[Asserts]
jsonpath "$.data" exists

# --- API root WITHOUT token → should work ---
GET http://localhost:3001/api/
HTTP 200
[Asserts]
jsonpath "$.data" == "GhostWire API v2"
