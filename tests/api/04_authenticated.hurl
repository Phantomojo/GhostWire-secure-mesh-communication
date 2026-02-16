# ============================================================
# GhostWire API Test Suite - 04: Authenticated Endpoints
# ============================================================
# These endpoints REQUIRE a valid X-Ghost-Token header.
# First acquires a dev token, then tests all authenticated routes.
# Run: hurl --test tests/api/04_authenticated.hurl
# ============================================================

# --- Acquire Auth Token ---
GET http://localhost:3001/api/dev/token
HTTP 200
[Captures]
auth_token: jsonpath "$.data"

# --- V2 Profile ---
GET http://localhost:3001/api/v2/profile
X-Ghost-Token: {{auth_token}}
HTTP 200
[Asserts]
jsonpath "$.data" exists
jsonpath "$.error" not exists

# --- V2 Public Key ---
GET http://localhost:3001/api/v2/profile/public-key
X-Ghost-Token: {{auth_token}}
HTTP 200
[Asserts]
jsonpath "$.data" exists

# --- V2 Security Dashboard ---
GET http://localhost:3001/api/v2/security/dashboard
X-Ghost-Token: {{auth_token}}
HTTP 200
[Asserts]
jsonpath "$.data" exists
jsonpath "$.error" not exists

# --- System Stats ---
GET http://localhost:3001/api/system/stats
X-Ghost-Token: {{auth_token}}
HTTP 200
[Asserts]
jsonpath "$.data" exists

# --- Security Audit ---
GET http://localhost:3001/api/system/audit
X-Ghost-Token: {{auth_token}}
HTTP 200
[Asserts]
jsonpath "$.data" exists

# --- Get Settings ---
GET http://localhost:3001/api/settings
X-Ghost-Token: {{auth_token}}
HTTP 200
[Asserts]
jsonpath "$.data" exists

# --- Get Peers ---
GET http://localhost:3001/api/peers
X-Ghost-Token: {{auth_token}}
HTTP 200
[Asserts]
jsonpath "$.data" exists

# --- V2 Peers ---
GET http://localhost:3001/api/v2/peers
X-Ghost-Token: {{auth_token}}
HTTP 200
[Asserts]
jsonpath "$.data" exists

# --- Get Messages ---
GET http://localhost:3001/api/messages
X-Ghost-Token: {{auth_token}}
HTTP 200
[Asserts]
jsonpath "$.data" exists

# --- V2 Messages ---
GET http://localhost:3001/api/v2/messages
X-Ghost-Token: {{auth_token}}
HTTP 200
[Asserts]
jsonpath "$.data" exists

# --- Tutorial ---
GET http://localhost:3001/api/help/tutorial
X-Ghost-Token: {{auth_token}}
HTTP 200
[Asserts]
jsonpath "$.data" exists
