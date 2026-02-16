# ============================================================
# GhostWire API Test Suite - 05: Messaging & Peers
# ============================================================
# Tests messaging and peer connectivity endpoints.
# Run: hurl --test tests/api/05_messaging.hurl
# ============================================================

# --- Acquire Auth Token ---
GET http://localhost:3001/api/dev/token
HTTP 200
[Captures]
auth_token: jsonpath "$.data"

# --- Send Message (broadcast) ---
POST http://localhost:3001/api/broadcast
X-Ghost-Token: {{auth_token}}
Content-Type: application/json
{
    "content": "Hurl API test broadcast"
}
HTTP 200
[Asserts]
jsonpath "$.error" not exists

# --- Send Direct Message ---
POST http://localhost:3001/api/send_message
X-Ghost-Token: {{auth_token}}
Content-Type: application/json
{
    "recipient": "test_peer_id",
    "content": "Hurl direct message test"
}
# May fail if peer doesn't exist, but should return valid response format
[Asserts]
jsonpath "$.version" == "2.0"

# --- Ping Peer (non-existent, tests error handling) ---
POST http://localhost:3001/api/ping_peer
X-Ghost-Token: {{auth_token}}
Content-Type: application/json
{
    "peer_id": "nonexistent_peer"
}
# Should handle gracefully
[Asserts]
jsonpath "$.version" == "2.0"
