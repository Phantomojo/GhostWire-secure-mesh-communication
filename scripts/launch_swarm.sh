#!/bin/bash
# GhostWire Swarm Simulator - Local Multi-Node Testing

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}🚀 GhostWire Swarm Simulator Starting...${NC}"

# Ensure we are in the root directory
if [ ! -d "ghostwire" ]; then
    echo "Error: Must run from project root"
    exit 1
fi

# Kill any existing GhostWire processes
echo "🧹 Cleaning up previous instances..."
pkill -f ghostwire 2>/dev/null

# Build the backend
echo "🔨 Building backend..."
cd ghostwire && cargo build
cd ..

# Launch Node Alpha
echo -e "${GREEN}▶ Launching Node Alpha (Web: 3001, Mesh: 8001)${NC}"
./ghostwire/target/debug/ghostwire web --port 3001 > node_alpha.log 2>&1 &
ALPHA_PID=$!

# Wait for Alpha to initialize
sleep 3

# Launch Node Beta
echo -e "${GREEN}▶ Launching Node Beta (Web: 3002, Mesh: 8002)${NC}"
# We change the port for the web server and the mesh listener
./ghostwire/target/debug/ghostwire web --port 3002 > node_beta.log 2>&1 &
BETA_PID=$!

echo -e "${BLUE}✅ Swarm Active.${NC}"
echo "Node Alpha API: http://localhost:3001"
echo "Node Beta API:  http://localhost:3002"
echo ""
echo "Monitor Alpha: tail -f node_alpha.log"
echo "Monitor Beta:  tail -f node_beta.log"
echo ""
echo "Press Ctrl+C to shutdown the swarm."

# Handle shutdown
trap "kill $ALPHA_PID $BETA_PID; echo -e '\n🛑 Swarm stopped.'; exit" INT
wait
