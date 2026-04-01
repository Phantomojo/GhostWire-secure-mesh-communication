#!/bin/bash
# Quick GhostWire Backend Test for Stratsytem

echo "🔧 GhostWire Backend Test - Stratsytem"
echo "======================================="
echo ""

# Check if running
if pgrep -f "ghostwire --port 8080" > /dev/null; then
    echo "✓ Backend is RUNNING"
else
    echo "✗ Backend NOT running"
    echo "Starting backend..."
    cd ~/ghostwire-live-test
    ./ghostwire --port 8080 --mesh &
    sleep 5
fi

echo ""
echo "Node Identity:"
curl -s http://localhost:8080/api/v2/profile 2>/dev/null | python3 -c "
import sys,json
try:
    d=json.load(sys.stdin)
    print('  Node ID:', d['data']['id'])
    print('  Profile:', d['data']['nickname'])
    print('  Status:', '✓ OK' if d['data']['setup_complete'] else '✗ Not setup')
except:
    print('  ✗ API not responding')
" 2>/dev/null

echo ""
echo "Connected Peers:"
curl -s "http://localhost:8080/api/v2/peers?page=1&per_page=20" 2>/dev/null | python3 -c "
import sys,json
try:
    d=json.load(sys.stdin)
    print('  Count:', len(d['data']))
    for p in d['data'][:5]:
        print('  -', p['node_id'][:20]+'...')
except:
    print('  ✗ Cannot fetch peers')
" 2>/dev/null

echo ""
echo "======================================="
echo "To test connection to PC-A (omen):"
echo ""
echo "curl -X POST http://localhost:8080/api/v2/peers/connect \\"
echo "  -H 'Content-Type: application/json' \\"
echo "  -d '{\"peer_id\": \"88ef8419-e3b2-4d27-90f7-9d22db444a6b\", \"address\": \"/ip4/100.88.234.14/tcp/4001\"}'"
