# ============================================================
# GhostWire API Test Suite - 03: Identity Management
# ============================================================
# Identity routes are AUTH-EXEMPT.
# Run: hurl --test tests/api/03_identity.hurl
# ============================================================

# --- Get Identity Status ---
GET http://localhost:3001/api/identity/status
HTTP 200
Content-Type: application/json
[Asserts]
jsonpath "$.data" exists
jsonpath "$.error" not exists

# --- Get Identity (root) ---
GET http://localhost:3001/api/identity/
HTTP 200
Content-Type: application/json
[Asserts]
jsonpath "$.data" exists

# --- Get Public Key ---
GET http://localhost:3001/api/identity/public-key
HTTP 200
[Asserts]
jsonpath "$.data" exists
jsonpath "$.error" not exists
