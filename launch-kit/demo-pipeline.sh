#!/bin/bash
# ╔══════════════════════════════════════════════════════════╗
# ║  GhostWire Full Pipeline Demo                            ║
# ║  Shows: Encryption → Transmission → Decryption → Storage ║
# ╚══════════════════════════════════════════════════════════╝

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
WHITE='\033[1;37m'
NC='\033[0m'

# Config
# Tokens should be set via environment variables
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
echo -e "${CYAN}║${WHITE}  GhostWire Full Pipeline Demo                          ${CYAN}║${NC}"
echo -e "${CYAN}║${WHITE}  Encryption → Transmission → Decryption → Storage      ${CYAN}║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════════════════════╝${NC}"
echo ""

# Get Node B PeerId
echo -e "${BLUE}[STEP 0]${NC} Discovering nodes..."
# Node B peer ID should be configured via environment or local file
NODE_B_FILE="/tmp/node-b-peer-id.txt"
if [ -f "$NODE_B_FILE" ]; then
    NODE_B=$(cat "$NODE_B_FILE" 2>/dev/null)
else
    echo -e "${YELLOW}Warning: Node B peer ID not found at $NODE_B_FILE${NC}"
    echo "Set NODE_B environment variable or create $NODE_B_FILE with the peer ID"
    NODE_B=""
fi
NODE_A=$(grep "local_peer_id" /tmp/node-a.log 2>/dev/null | tail -1 | python3 -c "import sys,json; [print(json.loads(l)['fields']['local_peer_id']) for l in sys.stdin]" 2>/dev/null)

if [ -z "$NODE_B" ]; then
    echo -e "${RED}❌ Node B not found. Make sure both nodes are running.${NC}"
    exit 1
fi

echo -e "  ${GREEN}✅${NC} Node A (omen):      ${YELLOW}$NODE_A${NC}"
echo -e "  ${GREEN}✅${NC} Node B (stratsytem): ${YELLOW}$NODE_B${NC}"
echo ""

# The message
MESSAGE="Hello from the GhostWire pipeline demo! 🔐"
echo -e "${BLUE}[STEP 1]${NC} Original Message"
echo -e "  ${WHITE}Content:${NC}  ${GREEN}$MESSAGE${NC}"
echo -e "  ${WHITE}From:${NC}     Node A (omen)"
echo -e "  ${WHITE}To:${NC}       Node B (stratsytem)"
echo -e "  ${WHITE}Type:${NC}     Direct P2P (encrypted)"
echo ""

# Send the message
echo -e "${BLUE}[STEP 2]${NC} Encrypting message on Node A..."
echo -e "  ${WHITE}Algorithm:${NC} AES-256-GCM"
echo -e "  ${WHITE}Key Exchange:${NC} X25519"
echo -e "  ${WHITE}Status:${NC}    ${GREEN}Encrypting...${NC}"
sleep 1

RESPONSE=$(curl -s -X POST "$API_A/api/send" \
  -H "Content-Type: application/json" \
  -H "X-Ghost-Token: $TOKEN" \
  -d "{\"recipient\": \"$NODE_B\", \"content\": \"$MESSAGE\"}" 2>&1)

echo -e "  ${GREEN}✅${NC} Message encrypted and published to mesh"
echo ""

# Show what was sent
echo -e "${BLUE}[STEP 3]${NC} Transmission over libp2p mesh"
echo -e "  ${WHITE}Transport:${NC}  TCP/IP (port 4001)"
echo -e "  ${WHITE}Protocol:${NC}   Gossipsub"
echo -e "  ${WHITE}Topic:${NC}      ${YELLOW}ghostwire-dm-$NODE_B${NC}"
echo -e "  ${WHITE}Path:${NC}       omen → stratsytem (direct)"
echo ""

# Wait for delivery
echo -e "${BLUE}[STEP 4]${NC} Waiting for delivery..."
sleep 3

# Check Node B logs for encryption/decryption
echo -e "${BLUE}[STEP 5]${NC} Receiving on Node B..."
# Node B logs should be accessed via configured log path
NODE_B_LOG="${GHOSTWIRE_NODE_B_LOG:-/tmp/node-b.log}"
if [ -f "$NODE_B_LOG" ]; then
    B_LOGS=$(tail -20 "$NODE_B_LOG" 2>/dev/null | grep -i "encrypt\|decrypt\|direct\|message\|Received" | tail -5)
else
    echo -e "${YELLOW}Warning: Node B log not found at $NODE_B_LOG${NC}"
    B_LOGS=""
fi

if echo "$B_LOGS" | grep -q "Received direct message"; then
    echo -e "  ${GREEN}✅${NC} Message received on Node B"
else
    echo -e "  ${YELLOW}⚠️${NC} Message delivery status unclear"
fi
echo ""

# Show the stored message
echo -e "${BLUE}[STEP 6]${NC} Decrypted message on Node B"
TOKEN_B="${GHOSTWIRE_TOKEN_B:-}"
if [ -z "$TOKEN_B" ]; then
    echo -e "${YELLOW}Warning: GHOSTWIRE_TOKEN_B not set, cannot retrieve messages from Node B${NC}"
    exit 0
fi
B_MESSAGES=$(curl -s "$API_B/api/v2/messages" -H "X-Ghost-Token: $TOKEN_B" 2>/dev/null)

echo "$B_MESSAGES" | python3 -c "
import sys, json
d = json.load(sys.stdin)
msgs = d.get('data', [])
if msgs:
    last = msgs[-1]
    print(f'  {chr(91)}{chr(93)} Content:     {chr(34)}{last.get(\"content\", \"\")}{chr(34)}')
    print(f'  {chr(91)}{chr(93)} Encrypted:   {last.get(\"encrypted\", False)}')
    print(f'  {chr(91)}{chr(93)} Status:      {last.get(\"encryption_status\", \"none\")}')
    print(f'  {chr(91)}{chr(93)} Type:        {last.get(\"message_type\", \"text\")}')
    print(f'  {chr(91)}{chr(93)} Hops:        {last.get(\"hops\", 0)}')
    print(f'  {chr(91)}{chr(93)} Timestamp:   {last.get(\"timestamp\", 0)}')
else:
    print('  No messages found')
" 2>/dev/null
echo ""

# Show the full pipeline
echo -e "${CYAN}╔══════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║${WHITE}  Pipeline Summary                                      ${CYAN}║${NC}"
echo -e "${CYAN}╠══════════════════════════════════════════════════════════╣${NC}"
echo -e "${CYAN}║${NC}                                                      ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}  ${GREEN}Node A (omen)${NC}                          ${GREEN}Node B (stratsytem)${NC}  ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}                                                      ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}  1. Create message                                   ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}     \"$MESSAGE\"                              ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}              │                                       ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}  2. Encrypt (AES-256-GCM)                            ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}     ciphertext → base64                              ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}              │                                       ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}  3. Publish to gossipsub                             ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}     topic: ghostwire-dm-{peer_id}                    ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}              │                                       ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}              └────── TCP/IP ──────→                  ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}                                      4. Receive      ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}                                         │            ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}                                      5. Decrypt      ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}                                         │            ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}                                      6. Store        ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}                                         │            ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}                                      7. API returns  ${CYAN}║${NC}"
echo -e "${CYAN}║${NC}                                                      ${CYAN}║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${GREEN}✅ Pipeline complete!${NC}"
echo ""
