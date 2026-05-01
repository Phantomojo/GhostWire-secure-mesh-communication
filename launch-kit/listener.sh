#!/bin/bash
# GhostWire Listener — Run on Node B (stratsytem)
# Watches for incoming messages in real time

TOKEN=$(sudo cat /root/.config/ghostwire/token 2>/dev/null)
API="http://127.0.0.1:9001"
LAST_COUNT=0

echo "╔══════════════════════════════════════════╗"
echo "║  GhostWire Listener (Node B - stratsytem)║"
echo "║  Watching for messages from Node A      ║"
echo "╚══════════════════════════════════════════╝"
echo ""
echo "📡 Listening for incoming messages..."
echo "   (Press Ctrl+C to stop)"
echo ""

while true; do
    response=$(curl -s "$API/api/v2/messages" \
        -H "X-Ghost-Token: $TOKEN" 2>&1)

    # Count messages
    count=$(echo "$response" | python3 -c "import sys,json; d=json.load(sys.stdin); print(len(d.get('data',[])))" 2>/dev/null || echo "0")

    if [ "$count" != "$LAST_COUNT" ] && [ "$count" != "0" ]; then
        # New message arrived!
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo "📨 NEW MESSAGE RECEIVED!"
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo "$response" | python3 -c "
import sys, json, datetime
d = json.load(sys.stdin)
for msg in d.get('data', []):
    ts = msg.get('timestamp', 0)
    time_str = datetime.datetime.fromtimestamp(ts).strftime('%H:%M:%S') if ts else 'unknown'
    print(f'  ⏰ Time:    {time_str}')
    print(f'  👤 From:    {msg.get(\"sender\", \"unknown\")[:20]}...')
    print(f'  💬 Message: {msg.get(\"content\", \"\")}')
    print()
" 2>/dev/null
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo ""
        LAST_COUNT=$count
    fi

    sleep 1
done
