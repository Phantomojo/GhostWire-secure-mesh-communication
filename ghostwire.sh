#!/bin/bash

# GhostWire Unified Launcher
# Starts both backend and frontend with a single command

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

# ASCII Art
echo -e "${CYAN}"
cat << "EOF"
 ██████╗ ██╗  ██╗ ██████╗ ███████╗████████╗██╗    ██╗██╗██████╗ ███████╗
██╔════╝ ██║  ██║██╔═══██╗██╔════╝╚══██╔══╝██║    ██║██║██╔══██╗██╔════╝
██║  ███╗███████║██║   ██║███████╗   ██║   ██║ █╗ ██║██║██████╔╝█████╗  
██║   ██║██╔══██║██║   ██║╚════██║   ██║   ██║███╗██║██║██╔══██╗██╔══╝  
╚██████╔╝██║  ██║╚██████╔╝███████║   ██║   ╚███╔███╔╝██║██║  ██║███████╗
 ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝    ╚══╝╚══╝ ╚═╝╚═╝  ╚═╝╚══════╝
EOF
echo -e "${NC}"

echo -e "${GREEN}🚀 GhostWire Unified Launcher${NC}"
echo -e "${YELLOW}Secure Mesh Communication Platform${NC}"
echo ""

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BACKEND_DIR="$SCRIPT_DIR/ghostwire"
FRONTEND_DIR="$SCRIPT_DIR/webui"

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to get local IP
get_local_ip() {
    if command_exists ip; then
        ip route get 1.1.1.1 | grep -oP 'src \K\S+' 2>/dev/null || echo "127.0.0.1"
    elif command_exists ifconfig; then
        ifconfig | grep "inet " | grep -v 127.0.0.1 | awk '{print $2}' | head -n1 || echo "127.0.0.1"
    else
        echo "127.0.0.1"
    fi
}

# Check dependencies
echo -e "${BLUE}🔍 Checking dependencies...${NC}"

if ! command_exists node; then
    echo -e "${RED}❌ Node.js not found. Please install Node.js first.${NC}"
    exit 1
fi

if ! command_exists npm; then
    echo -e "${RED}❌ npm not found. Please install npm first.${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Node.js and npm found${NC}"

# Check if we're in the right directory
if [ ! -d "$BACKEND_DIR" ] || [ ! -d "$FRONTEND_DIR" ]; then
    echo -e "${RED}❌ Please run this script from the GhostWire root directory${NC}"
    exit 1
fi

# Get local IP
LOCAL_IP=$(get_local_ip)
echo -e "${GREEN}✓ Local IP: ${LOCAL_IP}${NC}"

# Ports
BACKEND_PORT=3001
FRONTEND_PORT=5173

# Check if ports are in use
if lsof -Pi :$BACKEND_PORT -sTCP:LISTEN -t >/dev/null 2>&1; then
    echo -e "${YELLOW}⚠️  Backend port $BACKEND_PORT is in use, using 3002${NC}"
    BACKEND_PORT=3002
fi

if lsof -Pi :$FRONTEND_PORT -sTCP:LISTEN -t >/dev/null 2>&1; then
    echo -e "${YELLOW}⚠️  Frontend port $FRONTEND_PORT is in use, using 5175${NC}"
    FRONTEND_PORT=5175
fi

# Find backend binary
echo -e "${BLUE}🔍 Looking for backend binary...${NC}"
BACKEND_BIN=""
if [ -f "$BACKEND_DIR/target/release/ghostwire" ]; then
    BACKEND_BIN="$BACKEND_DIR/target/release/ghostwire"
    echo -e "${GREEN}✓ Found release binary${NC}"
elif [ -f "$BACKEND_DIR/target/debug/ghostwire" ]; then
    BACKEND_BIN="$BACKEND_DIR/target/debug/ghostwire"
    echo -e "${YELLOW}⚠️  Using debug binary (for performance, run: cargo build --release)${NC}"
else
    echo -e "${YELLOW}⚠️  No binary found, building...${NC}"
    cd "$BACKEND_DIR"
    cargo build --release
    BACKEND_BIN="$BACKEND_DIR/target/release/ghostwire"
    cd "$SCRIPT_DIR"
fi

# Start backend
echo -e "${BLUE}🚀 Starting backend on port $BACKEND_PORT...${NC}"
$BACKEND_BIN --web --host 0.0.0.0 --port $BACKEND_PORT &
BACKEND_PID=$!
sleep 2

if ! kill -0 $BACKEND_PID 2>/dev/null; then
    echo -e "${RED}❌ Backend failed to start${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Backend started (PID: $BACKEND_PID)${NC}"

# Configure and start frontend
echo -e "${BLUE}🚀 Starting frontend on port $FRONTEND_PORT...${NC}"
cd "$FRONTEND_DIR"

# Install dependencies if needed
if [ ! -d "node_modules" ]; then
    echo -e "${YELLOW}Installing frontend dependencies...${NC}"
    npm install --silent
fi

# Create environment file
cat > .env.local << EOF
VITE_API_BASE_URL=http://${LOCAL_IP}:${BACKEND_PORT}/api
VITE_BACKEND_URL=http://${LOCAL_IP}:${BACKEND_PORT}/api
EOF

npm run dev -- --host 0.0.0.0 --port $FRONTEND_PORT &
FRONTEND_PID=$!
cd "$SCRIPT_DIR"

# Wait for services
echo -e "${BLUE}⏳ Waiting for services to start...${NC}"

echo -n "Backend"
for i in {1..30}; do
    if curl -s http://localhost:$BACKEND_PORT/api/status >/dev/null 2>&1; then
        echo -e " ${GREEN}✓${NC}"
        break
    fi
    echo -n "."
    sleep 1
    if [ $i -eq 30 ]; then
        echo -e " ${YELLOW}(may not be ready)${NC}"
    fi
done

echo -n "Frontend"
for i in {1..30}; do
    if curl -s http://localhost:$FRONTEND_PORT >/dev/null 2>&1; then
        echo -e " ${GREEN}✓${NC}"
        break
    fi
    echo -n "."
    sleep 1
    if [ $i -eq 30 ]; then
        echo -e " ${YELLOW}(may not be ready)${NC}"
    fi
done

# Success message
echo ""
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}  🎉 GhostWire is running!${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${CYAN}📱 Access Points:${NC}"
echo -e "   Frontend (Local):   ${GREEN}http://localhost:$FRONTEND_PORT${NC}"
echo -e "   Frontend (Network): ${GREEN}http://${LOCAL_IP}:$FRONTEND_PORT${NC}"
echo -e "   Backend API:        ${GREEN}http://${LOCAL_IP}:$BACKEND_PORT/api${NC}"
echo ""
echo -e "${CYAN}🔧 For other devices:${NC}"
echo -e "   Open ${GREEN}http://${LOCAL_IP}:$FRONTEND_PORT${NC} in a browser"
echo ""
echo -e "${YELLOW}Press Ctrl+C to stop all services${NC}"
echo ""

# Cleanup function
cleanup() {
    echo ""
    echo -e "${BLUE}🛑 Stopping GhostWire...${NC}"
    
    if kill -0 $BACKEND_PID 2>/dev/null; then
        kill $BACKEND_PID
        echo -e "${GREEN}✓ Backend stopped${NC}"
    fi
    
    if kill -0 $FRONTEND_PID 2>/dev/null; then
        kill $FRONTEND_PID
        echo -e "${GREEN}✓ Frontend stopped${NC}"
    fi
    
    echo -e "${GREEN}🎉 All services stopped${NC}"
    exit 0
}

trap cleanup SIGINT SIGTERM

# Keep running
wait
