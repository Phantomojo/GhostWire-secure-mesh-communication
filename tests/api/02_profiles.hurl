# ============================================================
# GhostWire API Test Suite - 02: Profile Management
# ============================================================
# Profile routes are AUTH-EXEMPT (bootstrapping flow).
# Tests the full CRUD lifecycle: list → create → select → delete.
# Run: hurl --test tests/api/02_profiles.hurl
# ============================================================

# --- List Profiles (initial state) ---
GET http://localhost:3001/api/profile/list
HTTP 200
Content-Type: application/json
[Asserts]
jsonpath "$.data" exists
jsonpath "$.data.profiles" isCollection
jsonpath "$.data.current" exists
jsonpath "$.error" not exists

# --- Create Profile (valid name) ---
POST http://localhost:3001/api/profile/create
Content-Type: application/json
{
    "name": "hurl_test_profile"
}
HTTP 200
[Asserts]
jsonpath "$.data" exists
jsonpath "$.data" contains "hurl_test_profile"
jsonpath "$.error" not exists

# --- Create Profile (duplicate - should fail) ---
POST http://localhost:3001/api/profile/create
Content-Type: application/json
{
    "name": "hurl_test_profile"
}
HTTP 409
[Asserts]
jsonpath "$.error" exists
jsonpath "$.error.code" == "INVALID_INPUT"

# --- Create Profile (invalid characters - should fail) ---
POST http://localhost:3001/api/profile/create
Content-Type: application/json
{
    "name": "bad name!@#"
}
HTTP 400
[Asserts]
jsonpath "$.error" exists
jsonpath "$.error.code" == "INVALID_INPUT"

# --- Verify new profile appears in list ---
GET http://localhost:3001/api/profile/list
HTTP 200
[Asserts]
jsonpath "$.data.profiles" includes "hurl_test_profile"

# --- Select Profile ---
POST http://localhost:3001/api/profile/select
Content-Type: application/json
{
    "profile_name": "hurl_test_profile"
}
HTTP 200
[Asserts]
jsonpath "$.data" exists
jsonpath "$.data.status" == "profile_switched"

# --- Switch back to default before deleting ---
POST http://localhost:3001/api/profile/select
Content-Type: application/json
{
    "profile_name": "default"
}
HTTP 200
[Asserts]
jsonpath "$.data.status" == "profile_switched"

# --- Delete Test Profile (cleanup) ---
DELETE http://localhost:3001/api/profile/delete/hurl_test_profile
HTTP 200
[Asserts]
jsonpath "$.data" contains "purged"
jsonpath "$.error" not exists

# --- Verify deletion ---
GET http://localhost:3001/api/profile/list
HTTP 200
[Asserts]
jsonpath "$.data.profiles" not includes "hurl_test_profile"
