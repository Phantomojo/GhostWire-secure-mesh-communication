#!/bin/bash
# ╔══════════════════════════════════════════════════════════╗
# ║  GhostWire Raw Message Inspector                         ║
# ║  Shows the actual encrypted blob on the wire             ║
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

# Token should be set via environment variable
TOKEN="${GHOSTWIRE_TOKEN:-}"
if [ -z "$TOKEN" ]; then
    echo -e "${RED}Error: GHOSTWIRE_TOKEN environment variable not set${NC}"
    echo "Please set your auth token: export GHOSTWIRE_TOKEN=<your-token>"
    exit 1
fi
API_A="${GHOSTWIRE_API_A:-http://127.0.0.1:9000}"
API_B="${GHOSTWIRE_API_B:-http://127.0.0.1:9001}"

echo ""
echo -e "${CYAN}╔══════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║${WHITE}  GhostWire Raw Message Inspector                         ${CYAN}║${NC}"
echo -e "${CYAN}║${WHITE}  See the encrypted blob, transmission, and decryption    ${CYAN}║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════════════════════╝${NC}"
echo ""

# Get PeerIds
# Node B peer ID should be configured via environment or local file
NODE_B_FILE="/tmp/node-b-peer-id.txt"
if [ -f "$NODE_B_FILE" ]; then
    NODE_B=$(cat "$NODE_B_FILE" 2>/dev/null)
else
    echo -e "${YELLOW}Warning: Node B peer ID not found at $NODE_B_FILE${NC}"
    echo "Set NODE_B environment variable or create $NODE_B_FILE with the peer ID"
    NODE_B=""
fi

if [ -z "$NODE_B" ]; then
    echo -e "${RED}❌ Node B not running${NC}"
    exit 1
fi

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${WHITE}PHASE 1: MESSAGE CREATION${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

MESSAGE="Secret: The password is hunter2 🔑"
echo -e "  ${DIM}Plaintext:${NC}  ${GREEN}$MESSAGE${NC}"
echo -e "  ${DIM}Length:${NC}     ${WHITE}$(echo -n "$MESSAGE" | wc -c) bytes${NC}"
echo -e "  ${DIM}Encoding:${NC}   ${WHITE}UTF-8${NC}"
echo ""

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${WHITE}PHASE 2: ENCRYPTION (Node A)${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Get message count before
BEFORE=$(curl -s "$API_A/api/v2/messages" -H "X-Ghost-Token: $TOKEN" | python3 -c "import sys,json; d=json.load(sys.stdin); print(len(d.get('data',[])))" 2>/dev/null || echo "0")

# Send
RESPONSE=$(curl -s -X POST "$API_A/api/send" \
  -H "Content-Type: application/json" \
  -H "X-Ghost-Token: $TOKEN" \
  -d "{\"recipient\": \"$NODE_B\", \"content\": \"$MESSAGE\"}" 2>&1)

sleep 2

# Get the sent message from Node A
A_MSG=$(curl -s "$API_A/api/v2/messages" -H "X-Ghost-Token: $TOKEN" 2>/dev/null)

echo -e "  ${DIM}Algorithm:${NC}    ${WHITE}AES-256-GCM${NC}"
echo -e "  ${DIM}Key Exchange:${NC} ${WHITE}X25519${NC}"
echo -e "  ${DIM}Key Size:${NC}     ${WHITE}256 bits${NC}"
echo ""

# Show the encrypted message structure
echo -e "  ${DIM}Message structure on Node A:${NC}"
echo "$A_MSG" | python3 -c "
import sys, json
d = json.load(sys.stdin)
msgs = d.get('data', [])
if msgs:
    m = msgs[-1]
    print(f'    {chr(91)}id{chr(93)}              {m.get(\"id\", \"\")[:40]}...')
    print(f'    {chr(91)}sender{chr(93)}          {m.get(\"sender\", \"\")[:40]}...')
    print(f'    {chr(91)}recipient{chr(93)}       {m.get(\"recipient\", \"\")[:50]}')
    print(f'    {chr(91)}content{chr(93)}         {m.get(\"content\", \"\")[:80]}')
    print(f'    {chr(91)}encrypted{chr(93)}       {m.get(\"encrypted\", False)}')
    print(f'    {chr(91)}encryption_status{chr(93)} {m.get(\"encryption_status\", \"none\")}')
    print(f'    {chr(91)}message_type{chr(93)}    {m.get(\"message_type\", \"text\")}')
    print(f'    {chr(91)}hops{chr(93)}            {m.get(\"hops\", 0)}')
    print(f'    {chr(91)}timestamp{chr(93)}       {m.get(\"timestamp\", 0)}')
" 2>/dev/null
echo ""

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${WHITE}PHASE 3: TRANSMISSION OVER MESH${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Check Node A logs for transmission details
A_LOGS=$(tail -20 /tmp/node-a.log 2>/dev/null | grep -i "encrypt\|publish\|direct\|dm-\|topic\|gossipsub" | tail -5)
if [ -n "$A_LOGS" ]; then
    echo -e "  ${DIM}Node A transmission log:${NC}"
    echo "$A_LOGS" | while IFS= read -r line; do
        # Extract the message part
        msg=$(echo "$line" | python3 -c "import sys,json; d=json.loads(sys.stdin); print(d.get('fields',{}).get('message',''))" 2>/dev/null || echo "$line")
        echo -e "    ${YELLOW}→${NC} $msg"
    done
else
    echo -e "  ${DIM}No transmission logs available${NC}"
fi
echo ""

echo -e "  ${DIM}Network path:${NC}"
echo -e "    ${GREEN}node-a${NC} (<NODE_A_IP>) ──TCP:4001──→ ${GREEN}node-b${NC} (<NODE_B_IP>)"
echo -e "    ${DIM}Protocol:${NC} libp2p/gossipsub"
echo -e "    ${DIM}Topic:${NC}    ghostwire-dm-{recipient_peer_id}"
echo ""

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${WHITE}PHASE 4: RECEPTION & DECRYPTION (Node B)${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

sleep 2

# Check Node B logs
NODE_B_LOG="${GHOSTWIRE_NODE_B_LOG:-/tmp/node-b.log}"
if [ -f "$NODE_B_LOG" ]; then
    B_LOGS=$(tail -20 "$NODE_B_LOG" 2>/dev/null | grep -i "encrypt\|decrypt\|direct\|message\|Received\|Processed" | tail -5)
else
    echo -e "${YELLOW}Warning: Node B log not found at $NODE_B_LOG${NC}"
    B_LOGS=""
fi
if [ -n "$B_LOGS" ]; then
    echo -e "  ${DIM}Node B reception log:${NC}"
    echo "$B_LOGS" | while IFS= read -r line; do
        msg=$(echo "$line" | python3 -c "import sys,json; d=json.loads(sys.stdin); print(d.get('fields',{}).get('message',''))" 2>/dev/null || echo "$line")
        if echo "$msg" | grep -qi "error\|fail"; then
            echo -e "    ${RED}✗${NC} $msg"
        elif echo "$msg" | grep -qi "received\|decrypt"; then
            echo -e "    ${GREEN}✓${NC} $msg"
        else
            echo -e "    ${YELLOW}→${NC} $msg"
        fi
    done
fi
echo ""

# Show the received message
TOKEN_B="${GHOSTWIRE_TOKEN_B:-}"
if [ -z "$TOKEN_B" ]; then
    echo -e "${YELLOW}Warning: GHOSTWIRE_TOKEN_B not set, cannot retrieve messages from Node B${NC}"
    exit 0
fi
B_MSG=$(curl -s "$API_B/api/v2/messages" -H "X-Ghost-Token: $TOKEN_B" 2>/dev/null)

echo -e "  ${DIM}Message structure on Node B:${NC}"
echo "$B_MSG" | python3 -c "
import sys, json
d = json.load(sys.stdin)
msgs = d.get('data', [])
if msgs:
    m = msgs[-1]
    print(f'    {chr(91)}id{chr(93)}              {m.get(\"id\", \"\")[:40]}...')
    print(f'    {chr(91)}sender{chr(93)}          {m.get(\"sender\", \"\")[:40]}...')
    print(f'    {chr(91)}recipient{chr(93)}       {m.get(\"recipient\", \"\")[:50]}')
    print(f'    {chr(91)}content{chr(93)}         {m.get(\"content\", \"\")[:80]}')
    print(f'    {chr(91)}encrypted{chr(93)}       {m.get(\"encrypted\", False)}')
    print(f'    {chr(91)}encryption_status{chr(93)} {m.get(\"encryption_status\", \"none\")}')
    print(f'    {chr(91)}message_type{chr(93)}    {m.get(\"message_type\", \"text\")}')
    print(f'    {chr(91)}hops{chr(93)}            {m.get(\"hops\", 0)}')
" 2>/dev/null
echo ""

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${WHITE}PHASE 5: VERIFICATION${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Compare original vs received
RECEIVED=$(echo "$B_MSG" | python3 -c "import sys,json; d=json.load(sys.stdin); msgs=d.get('data',[]); print(msgs[-1].get('content','') if msgs else '')" 2>/dev/null)

if [ "$RECEIVED" = "$MESSAGE" ]; then
    echo -e "  ${GREEN}✅${NC} Message integrity: ${GREEN}VERIFIED${NC}"
    echo -e "  ${GREEN}✅${NC} Original:  ${GREEN}$MESSAGE${NC}"
    echo -e "  ${GREEN}✅${NC} Received:  ${GREEN}$RECEIVED${NC}"
    echo -e "  ${GREEN}✅${NC} Match:     ${GREEN}EXACT${NC}"
else
    echo -e "  ${YELLOW}⚠️${NC} Message content differs"
    echo -e "  ${DIM}Original:${NC} $MESSAGE"
    echo -e "  ${DIM}Received:${NC} $RECEIVED"
fi
echo ""

echo -e "${CYAN}╔══════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║${WHITE}  Summary                                               ${CYAN}║${NC}"
echo -e "${CYAN}╠══════════════════════════════════════════════════════════╣${NC}"
echo -e "${CYAN}║${NC}                                                      ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}  ${GREEN}Node A${NC}                    ${GREEN}Node B${NC}                    ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}                                                      ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}  Plaintext:                                          ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}  \"$MESSAGE\"                              ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}       │                                              ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}  Encrypt (AES-256-GCM)                               ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}       │                                              ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}  Publish → gossipsub → TCP → receive                 ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}       │                                              ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}                                      Decrypt         ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}       │                                              ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}                                      Store           ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}       │                                              ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}                                      Return via API  ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}                                                      ${CYAN}║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════════════════════╝${NC}"
echo ""
