#!/bin/bash

# 🌐 GhostWire Complete System Launcher
# Starts both frontend and backend for cross-PC communication

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

# ASCII Art
echo -e "${CYAN}"
cat << "EOF"
██╗    ██╗ ██████╗ ███████╗██╗  ██╗███████╗██╗    ██╗██╗██████╗ ███████╗
██║    ██║██╔════╝ ██╔════╝██║  ██║██╔════╝██║    ██║██║██╔══██╗██╔════╝
██║ █╗ ██║██║  ███╗███████╗███████║█████╗  ██║ █╗ ██║██║██████╔╝█████╗  
██║███╗██║██║   ██║╚════██║██╔══██║██╔══╝  ██║███╗██║██║██╔══██╗██╔══╝  
╚███╔███╔╝╚██████╔╝███████║██║  ██║██║     ╚███╔███╔╝██║██║  ██║███████╗
 ╚══╝╚══╝  ╚═════╝ ╚══════╝╚═╝  ╚═╝╚═╝      ╚══╝╚══╝ ╚═╝╚═╝  ╚═╝╚══════╝
EOF
echo -e "${NC}"

echo -e "${GREEN}🌐 GhostWire - Complete System Launcher${NC}"
echo -e "${YELLOW}Starting frontend and backend for cross-PC communication...${NC}"
echo ""

# Configuration
BACKEND_PORT=${BACKEND_PORT:-3001}
FRONTEND_PORT=${FRONTEND_PORT:-5173}
BACKEND_HOST=${BACKEND_HOST:-"0.0.0.0"}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to check if port is available
check_port() {
    local port=$1
    if command_exists netstat; then
        netstat -an 2>/dev/null | grep -q ":$port " && return 1 || return 0
    elif command_exists ss; then
        ss -tuln | grep -q ":$port " && return 1 || return 0
    else
        timeout 1 bash -c "echo >/dev/tcp/localhost/$port" 2>/dev/null && return 1 || return 0
    fi
}

# Function to find available port
find_available_port() {
    local start_port=$1
    local port=$start_port
    
    while ! check_port $port; do
        port=$((port + 1))
        if [ $port -gt $((start_port + 100)) ]; then
            echo -e "${RED}Error: Could not find available port starting from $start_port${NC}" >&2
            exit 1
        fi
    done
    
    echo $port
}

# Function to wait for service to be ready
wait_for_service() {
    local url=$1
    local max_attempts=${2:-30}
    local attempt=1
    
    echo -e "${BLUE}Waiting for service at $url...${NC}"
    
    while [ $attempt -le $max_attempts ]; do
        if curl -s "$url" >/dev/null 2>&1; then
            echo -e "${GREEN}✓ Service ready at $url${NC}"
            return 0
        fi
        
        echo -n "."
        sleep 1
        attempt=$((attempt + 1))
    done
    
    echo -e "${RED}✗ Service failed to start at $url${NC}" >&2
    return 1
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

# Check if ports are available
echo -e "${BLUE}🔍 Checking port availability...${NC}"

if ! check_port $BACKEND_PORT; then
    echo -e "${YELLOW}⚠️  Backend port $BACKEND_PORT is in use${NC}"
    BACKEND_PORT=$(find_available_port $BACKEND_PORT)
    echo -e "${GREEN}✓ Using backend port $BACKEND_PORT${NC}"
fi

if ! check_port $FRONTEND_PORT; then
    echo -e "${YELLOW}⚠️  Frontend port $FRONTEND_PORT is in use${NC}"
    FRONTEND_PORT=$(find_available_port $FRONTEND_PORT)
    echo -e "${GREEN}✓ Using frontend port $FRONTEND_PORT${NC}"
fi

# Get local IP address
LOCAL_IP=$(hostname -I | awk '{print $1}' | head -1)
if [ -z "$LOCAL_IP" ]; then
    LOCAL_IP="127.0.0.1"
fi

echo -e "${GREEN}✓ Local IP: $LOCAL_IP${NC}"

# Start backend
echo -e "${BLUE}🚀 Starting backend server...${NC}"
cd ghostwire

# Try to start backend
if [ -f "start-backend.sh" ]; then
    echo -e "${YELLOW}Using custom backend launcher...${NC}"
    BACKEND_HOST=$BACKEND_HOST BACKEND_PORT=$BACKEND_PORT ./start-backend.sh &
    BACKEND_PID=$!
else
    echo -e "${YELLOW}Backend launcher not found. Starting mock server...${NC}"
    python3 -m http.server $BACKEND_PORT --bind $BACKEND_HOST &
    BACKEND_PID=$!
fi

cd ..

# Wait for backend to start
sleep 2

# Start frontend
echo -e "${BLUE}🚀 Starting frontend server...${NC}"
cd webui

# Update frontend configuration for cross-PC communication
echo -e "${YELLOW}Configuring frontend for cross-PC communication...${NC}"

# Create environment file for frontend
cat > .env.local << EOF
VITE_API_BASE_URL=http://$LOCAL_IP:$BACKEND_PORT/api
VITE_BACKEND_HOST=$LOCAL_IP
VITE_BACKEND_PORT=$BACKEND_PORT
EOF

echo -e "${GREEN}✓ Frontend configuration updated${NC}"

# Start frontend development server
echo -e "${YELLOW}Starting frontend on port $FRONTEND_PORT...${NC}"
npm run dev -- --host 0.0.0.0 --port $FRONTEND_PORT &
FRONTEND_PID=$!

cd ..

# Wait for services to start
echo -e "${BLUE}⏳ Waiting for services to start...${NC}"

# Wait for backend
if wait_for_service "http://localhost:$BACKEND_PORT" 30; then
    echo -e "${GREEN}✓ Backend is ready${NC}"
else
    echo -e "${YELLOW}⚠️  Backend may not be fully ready${NC}"
fi

# Wait for frontend
if wait_for_service "http://localhost:$FRONTEND_PORT" 30; then
    echo -e "${GREEN}✓ Frontend is ready${NC}"
else
    echo -e "${YELLOW}⚠️  Frontend may not be fully ready${NC}"
fi

# Display access information
echo ""
echo -e "${GREEN}🎉 GhostWire is now running!${NC}"
echo ""
echo -e "${CYAN}📱 Access Points:${NC}"
echo -e "${YELLOW}  Frontend (Local):${NC}    http://localhost:$FRONTEND_PORT"
echo -e "${YELLOW}  Frontend (Network):${NC}  http://$LOCAL_IP:$FRONTEND_PORT"
echo -e "${YELLOW}  Backend API:${NC}         http://$LOCAL_IP:$BACKEND_PORT/api"
echo ""
echo -e "${CYAN}🔧 Cross-PC Configuration:${NC}"
echo -e "${YELLOW}  For other PCs to connect:${NC}"
echo -e "${YELLOW}  1. Open the frontend URL on the other PC${NC}"
echo -e "${YELLOW}  2. Click the ⚙️ config button in the status bar${NC}"
echo -e "${YELLOW}  3. Set backend URL to: http://$LOCAL_IP:$BACKEND_PORT/api${NC}"
echo -e "${YELLOW}  4. Click Connect${NC}"
echo ""
echo -e "${CYAN}🔍 Network Scanning:${NC}"
echo -e "${YELLOW}  Click '🔍 Scan Network' in the Network panel to find other GhostWire nodes${NC}"
echo ""
echo -e "${YELLOW}Press Ctrl+C to stop all services${NC}"

# Function to cleanup on exit
cleanup() {
    echo ""
    echo -e "${YELLOW}🛑 Stopping GhostWire services...${NC}"
    
    if [ ! -z "$BACKEND_PID" ]; then
        kill $BACKEND_PID 2>/dev/null || true
        echo -e "${GREEN}✓ Backend stopped${NC}"
    fi
    
    if [ ! -z "$FRONTEND_PID" ]; then
        kill $FRONTEND_PID 2>/dev/null || true
        echo -e "${GREEN}✓ Frontend stopped${NC}"
    fi
    
    echo -e "${GREEN}🎉 All services stopped${NC}"
    exit 0
}

# Set up signal handlers
trap cleanup SIGINT SIGTERM

# Keep the script running
wait 