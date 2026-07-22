#!/bin/bash
# ╔══════════════════════════════════════════════════════════╗
# ║  GhostWire Multi-Hop Relay Demo                          ║
# ║  Shows messages hopping through intermediate nodes       ║
# ╚══════════════════════════════════════════════════════════╝

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
WHITE='\033[1;37m'
DIM='\033[0;90m'
NC='\033[0m'

echo ""
echo -e "${CYAN}╔══════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║${WHITE}  GhostWire Multi-Hop Relay Demo                          ${CYAN}║${NC}"
echo -e "${CYAN}║${WHITE}  Messages hop through intermediate nodes (max 5)         ${CYAN}║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════════════════════╝${NC}"
echo ""

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${WHITE}HOW MULTI-HOP WORKS${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo -e "  ${DIM}When a node receives a message NOT intended for itself:${NC}"
echo ""
echo -e "  1. Check: Is this message for me?"
echo -e "     ${GREEN}No${NC} → Continue to step 2"
echo -e "     ${RED}Yes${NC} → Decrypt, store, done"
echo ""
echo -e "  2. Check: Has this message exceeded max hops?"
echo -e "     ${GREEN}No${NC} (hops < 5) → Continue to step 3"
echo -e "     ${RED}Yes${NC} (hops ≥ 5) → Drop message"
echo ""
echo -e "  3. Check: Have I seen this message ID before?"
echo -e "     ${GREEN}No${NC} → Continue to step 4"
echo -e "     ${RED}Yes${NC} → Drop (duplicate)"
echo ""
echo -e "  4. Increment hops counter and republish"
echo -e "     hops = hops + 1"
echo -e "     Publish to recipient's DM topic"
echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${WHITE}VISUALIZATION${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo -e "  ${GREEN}Node A${NC}          ${YELLOW}Node B${NC}          ${GREEN}Node C${NC}"
echo -e "  (omen)          (relay)         (stratsytem)"
echo -e "     │                │                │"
echo -e "     │  1. Create     │                │"
echo -e "     │  msg (hops=0)  │                │"
echo -e "     │                │                │"
echo -e "     ├──── publish ──→│                │"
echo -e "     │  topic: dm-C   │                │"
echo -e "     │                │                │"
echo -e "     │                │  2. Receive    │"
echo -e "     │                │  Not for me    │"
echo -e "     │                │  hops=0 < 5    │"
echo -e "     │                │  Not seen      │"
echo -e "     │                │                │"
echo -e "     │                ├──── publish ──→│"
echo -e "     │                │  topic: dm-C   │"
echo -e "     │                │  hops=1        │"
echo -e "     │                │                │"
echo -e "     │                │                │  3. Receive"
echo -e "     │                │                │  For me!"
echo -e "     │                │                │  Decrypt"
echo -e "     │                │                │  Store"
echo -e "     │                │                │  Done ✓"
echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${WHITE}CURRENT NETWORK${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Get PeerIds
NODE_A=$(grep "local_peer_id" /tmp/node-a.log 2>/dev/null | tail -1 | python3 -c "import sys,json; [print(json.loads(l)['fields']['local_peer_id']) for l in sys.stdin]" 2>/dev/null)
NODE_B=$(ssh "${GHOSTWIRE_NODE_B_USER:-root}@${GHOSTWIRE_NODE_B_HOST:?Set GHOSTWIRE_NODE_B_HOST}" "grep 'local_peer_id' /tmp/node-b.log 2>/dev/null | tail -1" | python3 -c "import sys,json; [print(json.loads(l)['fields']['local_peer_id']) for l in sys.stdin]" 2>/dev/null)

echo -e "  ${GREEN}Node A:${NC}      ${YELLOW}${NODE_A:-not running}${NC}"
echo -e "  ${GREEN}Node B:${NC}      ${YELLOW}${NODE_B:-not running}${NC}"
echo ""

if [ -n "$NODE_A" ] && [ -n "$NODE_B" ]; then
    echo -e "  ${DIM}Topology:${NC}"
    echo -e "    ${GREEN}A${NC} ←─── direct connection ───→ ${GREEN}B${NC}"
    echo -e "    ${DIM}(Currently 2 nodes — multi-hop activates with 3+ nodes)${NC}"
    echo ""
    echo -e "  ${DIM}With 2 nodes, messages go directly. With 3+ nodes,${NC}"
    echo -e "  ${DIM}intermediate nodes relay messages they didn't originate.${NC}"
else
    echo -e "  ${RED}⚠️${NC} One or both nodes not running"
fi
echo ""

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${WHITE}MESSAGE WITH HOP COUNTER${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Send a message and check hops
# ponytail: token moved to env var — was hardcoded, now via GHOSTWIRE_TOKEN
TOKEN="${GHOSTWIRE_TOKEN:?Set GHOSTWIRE_TOKEN env var}"
MESSAGE="Multi-hop test message 🔄"

echo -e "  ${DIM}Sending:${NC} $MESSAGE"
echo -e "  ${DIM}From:${NC}    Node A"
echo -e "  ${DIM}To:${NC}      Node B"
echo ""

curl -s -X POST "http://127.0.0.1:9000/api/send" \
  -H "Content-Type: application/json" \
  -H "X-Ghost-Token: $TOKEN" \
  -d "{\"recipient\": \"$NODE_B\", \"content\": \"$MESSAGE\"}" 2>&1 | python3 -m json.tool 2>/dev/null

sleep 3

# Check received message hops
TOKEN_B="${GHOSTWIRE_TOKEN_B:-}"
NODE_B_HOST="${GHOSTWIRE_NODE_B_HOST:?Set GHOSTWIRE_NODE_B_HOST}"
NODE_B_USER="${GHOSTWIRE_NODE_B_USER:-root}"
B_MSG=$(ssh "${NODE_B_USER}@${NODE_B_HOST}" "curl -s http://127.0.0.1:9001/api/v2/messages -H 'X-Ghost-Token: $TOKEN_B'" 2>/dev/null)

echo -e "\n  ${DIM}Received on Node B:${NC}"
echo "$B_MSG" | python3 -c "
import sys, json
d = json.load(sys.stdin)
msgs = d.get('data', [])
if msgs:
    m = msgs[-1]
    hops = m.get('hops', 0)
    print(f'    Content:  {m.get(\"content\", \"\")}')
    print(f'    Hops:     {hops}')
    if hops == 0:
        print(f'    Path:     Direct (no relay needed)')
    elif hops == 1:
        print(f'    Path:     A → B (1 hop)')
    else:
        print(f'    Path:     A → ... → B ({hops} hops)')
" 2>/dev/null
echo ""

echo -e "${CYAN}╔══════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║${WHITE}  Multi-Hop Configuration                                 ${CYAN}║${NC}"
echo -e "${CYAN}╠══════════════════════════════════════════════════════════╣${NC}"
echo -e "${CYAN}║${NC}                                                      ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}  Max hops:        5                                  ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}  Deduplication:   Yes (by message UUID)              ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}  Forwarding:      Automatic for non-local DMs        ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}  Broadcast:       No hop counting (flood)            ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}                                                      ${CYAN}║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════════════════════╝${NC}"
echo ""
