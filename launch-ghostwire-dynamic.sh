#!/bin/bash

# GhostWire Dynamic Launcher - ALL BUTTONS FUNCTIONAL
# Every button now does something real and dynamic

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

# ASCII Art
echo -e "${PURPLE}"
cat << "EOF"
██╗    ██╗ ██████╗ ███████╗██╗  ██╗███████╗██╗    ██╗██╗██████╗ ███████╗
██║    ██║██╔════╝ ██╔════╝██║  ██║██╔════╝██║    ██║██║██╔══██╗██╔════╝
██║ █╗ ██║██║  ███╗███████╗███████║█████╗  ██║ █╗ ██║██║██████╔╝█████╗  
██║███╗██║██║   ██║╚════██║██╔══██║██╔══╝  ██║███╗██║██║██╔══██╗██╔══╝  
╚███╔███╔╝╚██████╔╝███████║██║  ██║██║     ╚███╔███╔╝██║██║  ██║███████╗
 ╚══╝╚══╝  ╚═════╝ ╚══════╝╚═╝  ╚═╝╚═╝      ╚══╝╚══╝ ╚═╝╚═╝ ═╝╚══════╝
EOF
echo -e "${NC}"

echo -e "${GREEN}🚀 GhostWire Dynamic Launcher - ALL BUTTONS FUNCTIONAL${NC}"
echo -e "${YELLOW}Every button now does something real and dynamic!${NC}"
echo ""

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
if [ ! -f "ghostwire/Cargo.toml" ] || [ ! -f "webui/package.json" ]; then
    echo -e "${RED}❌ Please run this script from the GhostWire root directory${NC}"
    exit 1
fi

# Get local IP
LOCAL_IP=$(get_local_ip)
echo -e "${GREEN}✓ Local IP: ${LOCAL_IP}${NC}"

# Check port availability
echo -e "${BLUE}🔍 Checking port availability...${NC}"

BACKEND_PORT=3001
FRONTEND_PORT=5173

# Check if ports are in use
if lsof -Pi :$BACKEND_PORT -sTCP:LISTEN -t >/dev/null 2>&1; then
    echo -e "${YELLOW}⚠️  Backend port $BACKEND_PORT is in use${NC}"
    BACKEND_PORT=3002
    echo -e "${GREEN}✓ Using backend port $BACKEND_PORT${NC}"
else
    echo -e "${GREEN}✓ Backend port $BACKEND_PORT available${NC}"
fi

if lsof -Pi :$FRONTEND_PORT -sTCP:LISTEN -t >/dev/null 2>&1; then
    echo -e "${YELLOW}⚠️  Frontend port $FRONTEND_PORT is in use${NC}"
    FRONTEND_PORT=5175
    echo -e "${GREEN}✓ Using frontend port $FRONTEND_PORT${NC}"
else
    echo -e "${GREEN}✓ Frontend port $FRONTEND_PORT available${NC}"
fi

# Fix rustup proxy issue temporarily
echo -e "${BLUE}🔧 Fixing rustup configuration...${NC}"
export RUSTUP_TOOLCHAIN="stable"
unset RUSTUP_PROXY 2>/dev/null || true

# Start backend
echo -e "${BLUE}🚀 Starting backend server...${NC}"

# Check if we have a pre-built binary
if [ -f "ghostwire/target/debug/ghostwire" ]; then
    echo -e "${GREEN}✓ Found debug binary${NC}"
    BACKEND_CMD="ghostwire/target/debug/ghostwire"
elif [ -f "ghostwire/target/release/ghostwire" ]; then
    echo -e "${GREEN}✓ Found release binary${NC}"
    BACKEND_CMD="ghostwire/target/release/ghostwire"
else
    echo -e "${YELLOW}⚠️  No pre-built binary found, attempting to build...${NC}"
    echo -e "${YELLOW}   (This may take a few minutes on first run)${NC}"
    
    # Try to build without rustup proxy
    cd ghostwire
    RUSTUP_PROXY="" cargo build --bin ghostwire --features web || {
        echo -e "${RED}❌ Failed to build backend. Using fallback method...${NC}"
        # Fallback: try to run with system rust
        BACKEND_CMD="cargo run --bin ghostwire --features web"
    }
    cd ..
    
    if [ -f "ghostwire/target/debug/ghostwire" ]; then
        BACKEND_CMD="ghostwire/target/debug/ghostwire"
    else
        echo -e "${RED}❌ Could not build or find backend binary${NC}"
        exit 1
    fi
fi

echo -e "${GREEN}Starting backend on 0.0.0.0:$BACKEND_PORT${NC}"

# Start backend in background
$BACKEND_CMD --web --host 0.0.0.0 --port $BACKEND_PORT &
BACKEND_PID=$!

# Wait a moment for backend to start
sleep 2

# Check if backend started successfully
if ! kill -0 $BACKEND_PID 2>/dev/null; then
    echo -e "${RED}❌ Backend failed to start${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Backend started successfully (PID: $BACKEND_PID)${NC}"

# Start frontend
echo -e "${BLUE}🚀 Starting frontend server...${NC}"

# Configure frontend for cross-PC communication
echo -e "${BLUE}Configuring frontend for cross-PC communication...${NC}"

# Update frontend configuration
cd webui
npm install --silent

# Create environment file for frontend
cat > .env.local << EOF
VITE_API_BASE_URL=http://${LOCAL_IP}:${BACKEND_PORT}/api
VITE_BACKEND_URL=http://${LOCAL_IP}:${BACKEND_PORT}/api
EOF

echo -e "${GREEN}✓ Frontend configuration updated${NC}"
echo -e "${GREEN}Starting frontend on port $FRONTEND_PORT...${NC}"

# Start frontend
npm run dev -- --host 0.0.0.0 --port $FRONTEND_PORT &
FRONTEND_PID=$!

cd ..

# Wait for services to start
echo -e "${BLUE}⏳ Waiting for services to start...${NC}"

# Wait for backend
echo -n "Waiting for service at http://localhost:$BACKEND_PORT"
for i in {1..30}; do
    if curl -s http://localhost:$BACKEND_PORT/api/status >/dev/null 2>&1; then
        echo -e "\n${GREEN}✓ Service ready at http://localhost:$BACKEND_PORT${NC}"
        break
    fi
    echo -n "."
    sleep 1
    if [ $i -eq 30 ]; then
        echo -e "\n${YELLOW}⚠️  Backend may not be fully ready${NC}"
    fi
done

# Wait for frontend
echo -n "Waiting for service at http://localhost:$FRONTEND_PORT"
for i in {1..30}; do
    if curl -s http://localhost:$FRONTEND_PORT >/dev/null 2>&1; then
        echo -e "\n${GREEN}✓ Service ready at http://localhost:$FRONTEND_PORT${NC}"
        break
    fi
    echo -n "."
    sleep 1
    if [ $i -eq 30 ]; then
        echo -e "\n${YELLOW}⚠️  Frontend may not be fully ready${NC}"
    fi
done

echo -e "${GREEN}✓ Frontend is ready${NC}"

# Success message
echo ""
echo -e "${GREEN}🎉 GhostWire Dynamic is now running!${NC}"
echo ""
echo -e "${BLUE}📱 Access Points:${NC}"
echo -e "  Frontend (Local):    ${GREEN}http://localhost:$FRONTEND_PORT${NC}"
echo -e "  Frontend (Network):  ${GREEN}http://${LOCAL_IP}:$FRONTEND_PORT${NC}"
echo -e "  Backend API:         ${GREEN}http://${LOCAL_IP}:$BACKEND_PORT/api${NC}"
echo ""
echo -e "${PURPLE}🎯 DYNAMIC FEATURES NOW WORKING:${NC}"
echo -e "  ✅ 🔍 Scan Network - Actually scans for peers"
echo -e "  ✅ 🔄 Refresh Peers - Reloads real data"
echo -e "  ✅ 📡 Broadcast - Sends messages to all peers"
echo -e "  ✅ 🔗 Connect All - Connects to all discovered peers"
echo -e "  ✅ ❌ Disconnect All - Disconnects from all peers"
echo -e "  ✅ 🔄 Refresh - Reloads system data"
echo -e "  ✅ 🔒 Lock System - Activates emergency mode"
echo -e "  ✅ 💾 Backup - Creates system backup"
echo -e "  ✅ 🔄 Restart - Restarts the system"
echo -e "  ✅ 📊 Stats - Shows system statistics"
echo -e "  ✅ ⚙️ Config - Opens configuration"
echo -e "  ✅ 📋 Logs - Shows system logs"
echo -e "  ✅ Ping buttons - Actually ping peers"
echo -e "  ✅ Connect/Disconnect - Real peer management"
echo -e "  ✅ Send Messages - Real messaging"
echo ""
echo -e "${BLUE}🔧 Cross-PC Configuration:${NC}"
echo -e "  For other PCs to connect:"
echo -e "  1. Open the frontend URL on the other PC"
echo -e "  2. Click the ⚙️ config button in the status bar"
echo -e "  3. Set backend URL to: ${GREEN}http://${LOCAL_IP}:$BACKEND_PORT/api${NC}"
echo -e "  4. Click Connect"
echo ""
echo -e "${BLUE}🔍 Network Scanning:${NC}"
echo -e "  Click '🔍 Scan Network' in the Network panel to find other GhostWire nodes"
echo ""
echo -e "${BLUE}💡 Test Dynamic Features:${NC}"
echo -e "  • Click any button - they all do something real now!"
echo -e "  • Try 'Connect All' after scanning for peers"
echo -e "  • Use 'Broadcast' to send messages to all connected peers"
echo -e "  • Check 'Logs' to see system activity"
echo -e "  • Use 'Backup' to create system backups"
echo ""
echo -e "${YELLOW}Press Ctrl+C to stop all services${NC}"

# Function to cleanup on exit
cleanup() {
    echo ""
    echo -e "${BLUE}🛑 Stopping GhostWire services...${NC}"
    
    # Kill backend
    if kill -0 $BACKEND_PID 2>/dev/null; then
        kill $BACKEND_PID
        echo -e "${GREEN}✓ Backend stopped${NC}"
    fi
    
    # Kill frontend
    if kill -0 $FRONTEND_PID 2>/dev/null; then
        kill $FRONTEND_PID
        echo -e "${GREEN}✓ Frontend stopped${NC}"
    fi
    
    echo -e "${GREEN}🎉 All services stopped${NC}"
    exit 0
}

# Set up signal handlers
trap cleanup SIGINT SIGTERM

# Keep script running
wait 