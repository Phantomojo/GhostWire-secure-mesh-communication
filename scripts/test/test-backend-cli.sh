#!/bin/bash
# GhostWire Backend Communication Test - CLI Only (No UI)
# Run this on BOTH machines to verify backend communication

echo "🔧 GHOSTWIRE BACKEND COMMUNICATION TEST"
echo "========================================"
echo "Testing via CLI/API - NO UI required"
echo ""

NODE_ID=$(curl -s http://localhost:8080/api/v2/profile 2>/dev/null | python3 -c "import sys,json; print(json.load(sys.stdin)['data']['id'])" 2>/dev/null)

if [ -z "$NODE_ID" ]; then
    echo "❌ Backend NOT responding on port 8080"
    echo "   Start with: ./ghostwire --port 8080 --mesh"
    exit 1
fi

echo "✅ Backend RUNNING"
echo "   Node ID: $NODE_ID"
echo ""

echo "=== Test 1: API Endpoints ==="
echo ""

# Test profile endpoint
echo -n "1. /api/v2/profile ......... "
if curl -s http://localhost:8080/api/v2/profile | grep -q '"id"'; then
    echo "✅ OK"
else
    echo "❌ FAILED"
fi

# Test peers endpoint
echo -n "2. /api/v2/peers ........... "
PEER_COUNT=$(curl -s "http://localhost:8080/api/v2/peers?page=1&per_page=20" 2>/dev/null | python3 -c "import sys,json; print(len(json.load(sys.stdin).get('data', [])))" 2>/dev/null)
if [ -n "$PEER_COUNT" ]; then
    echo "✅ OK ($PEER_COUNT peers)"
else
    echo "❌ FAILED"
fi

# Test security dashboard
echo -n "3. /api/v2/security/dashboard "
if curl -s http://localhost:8080/api/v2/security/dashboard | grep -q '"data"'; then
    echo "✅ OK"
else
    echo "❌ FAILED (may need auth)"
fi

echo ""
echo "=== Test 2: Network Connectivity ==="
echo ""

# Check if we can reach stratsytem
echo -n "1. Ping stratsytem (100.105.58.61) "
if ping -c 1 100.105.58.61 >/dev/null 2>&1; then
    echo "✅ OK"
else
    echo "⚠️  Blocked (may be normal)"
fi

# Check local IP
LOCAL_IP=$(ip addr show | grep "inet " | grep -v "127.0.0.1" | head -1 | awk '{print $2}' | cut -d/ -f1)
echo "2. This node IP: $LOCAL_IP"

echo ""
echo "=== Test 3: Manual Peer Connection ==="
echo ""

if [ "$(hostname)" = "omen" ] || [ -n "$PC_A" ]; then
    echo "Testing connection TO stratsytem..."
    echo ""
    echo "Run this command to connect to stratsytem:"
    echo ""
    echo "curl -X POST http://localhost:8080/api/v2/peers/connect \\"
    echo "  -H 'Content-Type: application/json' \\"
    echo "  -d '{\"peer_id\": \"<STRAT_NODE_ID>\", \"address\": \"/ip4/100.105.58.61/tcp/4001\"}'"
    echo ""
elif [ "$(hostname)" = "stratsytem" ] || [ -n "$PC_B" ]; then
    echo "Testing connection TO omen..."
    echo ""
    echo "Run this command to connect to omen:"
    echo ""
    echo "curl -X POST http://localhost:8080/api/v2/peers/connect \\"
    echo "  -H 'Content-Type: application/json' \\"
    echo "  -d '{\"peer_id\": \"88ef8419-e3b2-4d27-90f7-9d22db444a6b\", \"address\": \"/ip4/100.88.234.14/tcp/4001\"}'"
    echo ""
fi

echo "=== Test 4: Check Logs ==="
echo ""
echo "Recent backend activity:"
tail -30 /tmp/ghostwire-*.log 2>/dev/null | grep -E "INFO|WARN" | grep -E "peer|mesh|discover" | tail -5 || echo "No peer-related logs yet"

echo ""
echo "========================================"
echo "✅ Backend communication test complete!"
echo ""
echo "NEXT STEPS:"
echo "-----------"
echo "1. Run this script on BOTH machines"
echo "2. Note the Node IDs from each"
echo "3. Use the 'curl' command above to manually connect"
echo "4. Wait 30 seconds"
echo "5. Run script again - peer count should increase"
echo ""
