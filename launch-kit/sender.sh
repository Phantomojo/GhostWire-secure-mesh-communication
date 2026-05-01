#!/bin/bash
# GhostWire Sender — Run on Node A (omen)
# Sends messages to Node B (stratsytem) in real time

TOKEN="eOAL2kTExt70taE71HmYh1wWZ5eD8T3W"
API="http://127.0.0.1:9000"

echo "╔══════════════════════════════════════════╗"
echo "║  GhostWire Sender (Node A - omen)       ║"
echo "║  Messages go to Node B (stratsytem)     ║"
echo "╚══════════════════════════════════════════╝"
echo ""
echo "Type a message and press Enter to send."
echo "Type 'quit' to exit."
echo ""

while true; do
    read -p "📝 Message: " msg
    if [ "$msg" = "quit" ] || [ "$msg" = "exit" ]; then
        echo "👋 Goodbye!"
        break
    fi
    if [ -z "$msg" ]; then
        continue
    fi

    echo "📡 Sending..."
    response=$(curl -s -X POST "$API/api/send" \
        -H "Content-Type: application/json" \
        -H "X-Ghost-Token: $TOKEN" \
        -d "{\"recipient\": \"broadcast\", \"content\": \"$msg\"}" 2>&1)

    echo "✅ $response"
    echo ""
done
