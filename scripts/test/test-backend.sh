#!/bin/bash
# GhostWire Backend Communication Debug Tool

echo "🔧 GHOSTWIRE BACKEND COMMUNICATION TEST"
echo "========================================"
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test local backend
test_local_backend() {
    echo -e "${YELLOW}Testing Local Backend (PC-A)...${NC}"
    
    # Check if running
    if pgrep -f "ghostwire --port 8080" > /dev/null; then
        echo -e "${GREEN}✓${NC} Backend process running"
    else
        echo -e "${RED}✗${NC} Backend NOT running"
        return 1
    fi
    
    # Test API endpoints
    echo ""
    echo "Testing API Endpoints:"
    
    # Profile
    PROFILE=$(curl -s http://127.0.0.1:8080/api/v2/profile 2>/dev/null)
    if echo "$PROFILE" | grep -q '"id"'; then
        NODE_ID=$(echo "$PROFILE" | python3 -c "import sys,json; print(json.load(sys.stdin)['data']['id'])" 2>/dev/null)
        echo -e "${GREEN}✓${NC} /api/v2/profile - Node ID: $NODE_ID"
    else
        echo -e "${RED}✗${NC} /api/v2/profile - FAILED"
    fi
    
    # Peers
    PEERS=$(curl -s "http://127.0.0.1:8080/api/v2/peers?page=1&per_page=20" 2>/dev/null)
    if echo "$PEERS" | grep -q '"data"'; then
        PEER_COUNT=$(echo "$PEERS" | python3 -c "import sys,json; print(len(json.load(sys.stdin)['data']))" 2>/dev/null)
        echo -e "${GREEN}✓${NC} /api/v2/peers - Peer count: $PEER_COUNT"
    else
        echo -e "${RED}✗${NC} /api/v2/peers - FAILED"
    fi
    
    # Mesh Health
    MESH=$(curl -s http://127.0.0.1:8080/api/v2/system/mesh-health 2>/dev/null)
    if echo "$MESH" | grep -q '"data"'; then
        echo -e "${GREEN}✓${NC} /api/v2/system/mesh-health - OK"
    else
        echo -e "${RED}✗${NC} /api/v2/system/mesh-health - FAILED (expected - needs auth)"
    fi
    
    echo ""
}

# Test stratsytem backend via SSH
test_remote_backend() {
    echo -e "${YELLOW}Testing Remote Backend (PC-B - stratsytem)...${NC}"
    
    # Test SSH connectivity first
    if ssh -o StrictHostKeyChecking=no -o ConnectTimeout=5 root@100.105.58.61 "echo 'SSH OK'" >/dev/null 2>&1; then
        echo -e "${GREEN}✓${NC} SSH connection to stratsytem"
    else
        echo -e "${RED}✗${NC} SSH connection FAILED"
        echo "   Trying Tailscale IP..."
        return 1
    fi
    
    # Check if backend running
    RUNNING=$(ssh -o StrictHostKeyChecking=no -o ConnectTimeout=5 root@100.105.58.61 "pgrep -f 'ghostwire --port 8080'" 2>/dev/null)
    if [ -n "$RUNNING" ]; then
        echo -e "${GREEN}✓${NC} Backend process running on stratsytem"
    else
        echo -e "${RED}✗${NC} Backend NOT running on stratsytem"
        echo "   To start: ssh root@100.105.58.61 'cd ~/ghostwire-live-test && ./ghostwire --port 8080 --mesh &'"
        return 1
    fi
    
    # Test API endpoints
    echo ""
    echo "Testing Remote API Endpoints:"
    
    # Profile
    PROFILE=$(ssh -o StrictHostKeyChecking=no -o ConnectTimeout=5 root@100.105.58.61 "curl -s http://localhost:8080/api/v2/profile" 2>/dev/null)
    if echo "$PROFILE" | grep -q '"id"'; then
        NODE_ID=$(echo "$PROFILE" | python3 -c "import sys,json; print(json.load(sys.stdin)['data']['id'])" 2>/dev/null)
        echo -e "${GREEN}✓${NC} /api/v2/profile - Node ID: $NODE_ID"
    else
        echo -e "${RED}✗${NC} /api/v2/profile - FAILED"
    fi
    
    # Peers
    PEERS=$(ssh -o StrictHostKeyChecking=no -o ConnectTimeout=5 root@100.105.58.61 "curl -s 'http://localhost:8080/api/v2/peers?page=1&per_page=20'" 2>/dev/null)
    if echo "$PEERS" | grep -q '"data"'; then
        PEER_COUNT=$(echo "$PEERS" | python3 -c "import sys,json; print(len(json.load(sys.stdin)['data']))" 2>/dev/null)
        echo -e "${GREEN}✓${NC} /api/v2/peers - Peer count: $PEER_COUNT"
    else
        echo -e "${RED}✗${NC} /api/v2/peers - FAILED"
    fi
    
    echo ""
}

# Test network connectivity
test_network() {
    echo -e "${YELLOW}Testing Network Connectivity...${NC}"
    
    # Ping stratsytem
    if ping -c 2 100.105.58.61 >/dev/null 2>&1; then
        echo -e "${GREEN}✓${NC} ICMP ping to stratsytem (100.105.58.61)"
    else
        echo -e "${YELLOW}!${NC} ICMP ping blocked (may be normal)"
    fi
    
    # Test HTTP connectivity via SSH tunnel
    echo -e "${YELLOW}Creating SSH tunnel for HTTP test...${NC}"
    ssh -o StrictHostKeyChecking=no -o ConnectTimeout=5 -f -N -L 8082:localhost:8080 root@100.105.58.61 2>/dev/null
    sleep 2
    
    if curl -s http://localhost:8082/api/v2/profile >/dev/null 2>&1; then
        echo -e "${GREEN}✓${NC} HTTP tunnel to stratsytem:8080"
        pkill -f "ssh.*8082" >/dev/null 2>&1
    else
        echo -e "${RED}✗${NC} HTTP tunnel FAILED"
    fi
    
    echo ""
}

# Test mDNS discovery
test_mdns() {
    echo -e "${YELLOW}Testing mDNS Discovery...${NC}"
    
    echo "Scanning for GhostWire services on local network..."
    if command -v avahi-browse >/dev/null 2>&1; then
        MDNS=$(avahi-browse -a -t 2>/dev/null | grep -i ghostwire | head -5)
        if [ -n "$MDNS" ]; then
            echo -e "${GREEN}✓${NC} mDNS services found:"
            echo "$MDNS" | sed 's/^/   /'
        else
            echo -e "${YELLOW}!${NC} No mDNS services found (may take 30-60 seconds)"
        fi
    else
        echo -e "${YELLOW}!${NC} avahi-browse not installed"
    fi
    
    echo ""
}

# Show logs
show_logs() {
    echo -e "${YELLOW}Recent Backend Logs (PC-A)...${NC}"
    tail -20 /tmp/ghostwire-omen.log 2>/dev/null | grep -E "INFO|WARN|ERROR" | tail -10 | sed 's/^/   /'
    echo ""
}

# Main execution
echo "Starting comprehensive backend test..."
echo ""

test_local_backend
test_remote_backend
test_network
test_mdns
show_logs

echo "========================================"
echo "Test Complete!"
echo ""
echo "📊 SUMMARY:"
echo "-----------"
echo "If both backends show ✓ for /api/v2/profile:"
echo "  • Both nodes are running correctly"
echo "  • Node IDs should be different"
echo "  • Wait 30-60 seconds for mDNS discovery"
echo "  • Check /api/v2/peers for peer count"
echo ""
echo "To manually connect nodes:"
echo "  curl -X POST http://127.0.0.1:8080/api/v2/peers/connect \\"
echo "    -H 'Content-Type: application/json' \\"
echo "    -d '{\"peer_id\": \"<STRAT_NODE_ID>\", \"address\": \"/ip4/100.105.58.61/tcp/4001\"}'"
