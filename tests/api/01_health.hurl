# ============================================================
# GhostWire API Test Suite - 01: Health & Bootstrap
# ============================================================
# These endpoints require NO authentication.
# Run: hurl --test tests/api/01_health.hurl
# ============================================================

# --- API Root ---
GET http://localhost:3001/api/
HTTP 200
Content-Type: application/json
[Asserts]
jsonpath "$.data" == "GhostWire API v2"
jsonpath "$.version" == "2.0"
jsonpath "$.request_id" exists
jsonpath "$.error" not exists

# --- Dev Token (auto-auth bootstrap) ---
GET http://localhost:3001/api/dev/token
HTTP 200
[Asserts]
jsonpath "$.data" exists

# Capture the token for authenticated requests
[Captures]
auth_token: jsonpath "$.data"

# --- Tutorial endpoint ---
GET http://localhost:3001/api/help/tutorial
HTTP 200
[Asserts]
jsonpath "$.data" exists
jsonpath "$.error" not exists
