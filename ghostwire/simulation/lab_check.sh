#!/bin/bash
# GhostWire Lab - Health Check Script

NUM_NODES=${1:-10}
echo "🧐 Checking health of $NUM_NODES GhostWire nodes..."

for i in $(seq 0 $((NUM_NODES - 1))); do
    NS="gw-node-$i"
    IP="10.0.99.$((i + 2))"
    PORT=$((8080 + i))
    
    # Get token
    TOKEN=$(curl -s "http://$IP:$PORT/api/dev/token" | jq -r '.data')
    
    if [ -z "$TOKEN" ] || [ "$TOKEN" == "null" ]; then
        echo "❌ Node $i ($IP:$PORT): OFFLINE or UNREACHABLE"
        continue
    fi

    # Get status
    STATUS_JSON=$(curl -s -H "X-Ghost-Token: $TOKEN" "http://$IP:$PORT/api/status")
    PEER_COUNT=$(echo "$STATUS_JSON" | jq -r '.data.peer_count')
    THREAT_LEVEL=$(echo "$STATUS_JSON" | jq -r '.data.threat_level')

    # Get peers
    PEERS_JSON=$(curl -s -H "X-Ghost-Token: $TOKEN" "http://$IP:$PORT/api/peers")
    KNOWN_PEERS=$(echo "$PEERS_JSON" | jq -r '.data.peers | length')

    echo "✅ Node $i ($IP:$PORT): Peers=$PEER_COUNT (Known=$KNOWN_PEERS) Threat=$THREAT_LEVEL"
done
