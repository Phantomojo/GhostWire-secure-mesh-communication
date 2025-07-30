#!/bin/bash

# 🌐 GhostWire - Complete System Launcher
# Starts backend, frontend, and TUI components with proper error handling

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
echo -e "${YELLOW}Starting backend, frontend, and TUI components...${NC}"
echo ""

# Configuration
BACKEND_PORT=${BACKEND_PORT:-3001}
FRONTEND_PORT=${FRONTEND_PORT:-5173}
BACKEND_HOST=${BACKEND_HOST:-"0.0.0.0"}
LAUNCH_MODE=${LAUNCH_MODE:-"all"} # all, backend-only, frontend-only, tui-only

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
        # Fallback: try to bind to port
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

# Function to update frontend API configuration
update_frontend_config() {
    local backend_port=$1
    local api_file="webui/src/services/api.ts"
    
    if [ -f "$api_file" ]; then
        echo -e "${BLUE}Updating frontend API configuration...${NC}"
        sed -i.bak "s|const API_BASE_URL = .*|const API_BASE_URL = 'http://localhost:$backend_port/api';|" "$api_file"
        echo -e "${GREEN}✓ Frontend API configuration updated${NC}"
    else
        echo -e "${YELLOW}Warning: Frontend API file not found at $api_file${NC}"
    fi
}

# Function to start backend
start_backend() {
    echo -e "${BLUE}Starting GhostWire backend...${NC}"
    
    # Check if Rust is installed
    if ! command_exists cargo; then
        echo -e "${RED}Error: Cargo not found. Please install Rust first.${NC}" >&2
        echo -e "${YELLOW}Run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh${NC}"
        exit 1
    fi
    
    # Check if ghostwire directory exists
    if [ ! -d "ghostwire" ]; then
        echo -e "${RED}Error: ghostwire directory not found${NC}" >&2
        exit 1
    fi
    
    # Build backend if needed
    echo -e "${BLUE}Building backend...${NC}"
    cd ghostwire
    if ! cargo build --quiet; then
        echo -e "${RED}Error: Backend build failed${NC}" >&2
        exit 1
    fi
    cd ..
    
    # Start backend
    echo -e "${BLUE}Starting backend on $BACKEND_HOST:$BACKEND_PORT...${NC}"
    cd ghostwire
    cargo run -- --host "$BACKEND_HOST" --port "$BACKEND_PORT" &
    BACKEND_PID=$!
    cd ..
    
    # Wait for backend to start
    sleep 3
    
    # Check if backend is running
    if ! kill -0 $BACKEND_PID 2>/dev/null; then
        echo -e "${RED}Error: Backend failed to start${NC}" >&2
        exit 1
    fi
    
    echo -e "${GREEN}✓ Backend started (PID: $BACKEND_PID)${NC}"
    
    # Wait for backend API to be ready
    if ! wait_for_service "http://localhost:$BACKEND_PORT/api/health" 30; then
        echo -e "${YELLOW}Warning: Backend API not responding, but process is running${NC}"
    fi
}

# Function to start frontend
start_frontend() {
    echo -e "${BLUE}Starting GhostWire frontend...${NC}"
    
    # Check if Node.js is installed
    if ! command_exists npm; then
        echo -e "${RED}Error: npm not found. Please install Node.js first.${NC}" >&2
        exit 1
    fi
    
    # Check if webui directory exists
    if [ ! -d "webui" ]; then
        echo -e "${RED}Error: webui directory not found${NC}" >&2
        exit 1
    fi
    
    # Install frontend dependencies if needed
    if [ ! -d "webui/node_modules" ]; then
        echo -e "${BLUE}Installing frontend dependencies...${NC}"
        cd webui
        npm install
        cd ..
    fi
    
    # Update frontend configuration
    update_frontend_config $BACKEND_PORT
    
    # Start frontend
    echo -e "${BLUE}Starting frontend on port $FRONTEND_PORT...${NC}"
    cd webui
    npm run dev -- --port "$FRONTEND_PORT" &
    FRONTEND_PID=$!
    cd ..
    
    # Wait for frontend to start
    sleep 5
    
    # Check if frontend is running
    if ! kill -0 $FRONTEND_PID 2>/dev/null; then
        echo -e "${RED}Error: Frontend failed to start${NC}" >&2
        exit 1
    fi
    
    echo -e "${GREEN}✓ Frontend started (PID: $FRONTEND_PID)${NC}"
    
    # Wait for frontend to be ready
    if ! wait_for_service "http://localhost:$FRONTEND_PORT" 30; then
        echo -e "${YELLOW}Warning: Frontend not responding, but process is running${NC}"
    fi
}

# Function to start TUI
start_tui() {
    echo -e "${BLUE}Starting GhostWire TUI...${NC}"
    
    # Check if backend is running
    if [ -n "$BACKEND_PID" ] && kill -0 $BACKEND_PID 2>/dev/null; then
        echo -e "${GREEN}✓ Backend is running, TUI can connect${NC}"
    else
        echo -e "${YELLOW}Warning: Backend not running, TUI may not work properly${NC}"
    fi
    
    # Start TUI in a new terminal if possible
    if command_exists gnome-terminal; then
        echo -e "${BLUE}Starting TUI in new terminal...${NC}"
        cd ghostwire
        gnome-terminal --title="GhostWire TUI" -- cargo run --bin ghostwire -- tui &
        TUI_PID=$!
        cd ..
    elif command_exists xterm; then
        echo -e "${BLUE}Starting TUI in new terminal...${NC}"
        cd ghostwire
        xterm -title "GhostWire TUI" -e "cargo run --bin ghostwire -- tui" &
        TUI_PID=$!
        cd ..
    else
        echo -e "${YELLOW}No terminal emulator found, starting TUI in background...${NC}"
        cd ghostwire
        cargo run --bin ghostwire -- tui &
        TUI_PID=$!
        cd ..
    fi
    
    echo -e "${GREEN}✓ TUI started (PID: $TUI_PID)${NC}"
}

# Function to cleanup on exit
cleanup() {
    echo -e "\n${YELLOW}Shutting down GhostWire...${NC}"
    
    if [ -n "$BACKEND_PID" ]; then
        echo -e "${BLUE}Stopping backend (PID: $BACKEND_PID)...${NC}"
        kill $BACKEND_PID 2>/dev/null || true
    fi
    
    if [ -n "$FRONTEND_PID" ]; then
        echo -e "${BLUE}Stopping frontend (PID: $FRONTEND_PID)...${NC}"
        kill $FRONTEND_PID 2>/dev/null || true
    fi
    
    if [ -n "$TUI_PID" ]; then
        echo -e "${BLUE}Stopping TUI (PID: $TUI_PID)...${NC}"
        kill $TUI_PID 2>/dev/null || true
    fi
    
    echo -e "${GREEN}✓ GhostWire shutdown complete${NC}"
    exit 0
}

# Set up signal handlers
trap cleanup SIGINT SIGTERM

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --backend-only)
            LAUNCH_MODE="backend-only"
            shift
            ;;
        --frontend-only)
            LAUNCH_MODE="frontend-only"
            shift
            ;;
        --tui-only)
            LAUNCH_MODE="tui-only"
            shift
            ;;
        --backend-port)
            BACKEND_PORT="$2"
            shift 2
            ;;
        --frontend-port)
            FRONTEND_PORT="$2"
            shift 2
            ;;
        --backend-host)
            BACKEND_HOST="$2"
            shift 2
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --backend-only     Start only the backend"
            echo "  --frontend-only    Start only the frontend"
            echo "  --tui-only         Start only the TUI"
            echo "  --backend-port N   Set backend port (default: 3001)"
            echo "  --frontend-port N  Set frontend port (default: 5173)"
            echo "  --backend-host H   Set backend host (default: 0.0.0.0)"
            echo "  -h, --help         Show this help message"
            echo ""
            echo "Environment variables:"
            echo "  BACKEND_PORT       Backend port (default: 3001)"
            echo "  FRONTEND_PORT      Frontend port (default: 5173)"
            echo "  BACKEND_HOST       Backend host (default: 0.0.0.0)"
            echo "  LAUNCH_MODE        Launch mode (default: all)"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}" >&2
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Find available ports if needed
if [ "$LAUNCH_MODE" != "frontend-only" ] && ! check_port $BACKEND_PORT; then
    echo -e "${YELLOW}Port $BACKEND_PORT is in use, finding available port...${NC}"
    BACKEND_PORT=$(find_available_port $BACKEND_PORT)
    echo -e "${BLUE}Using backend port: $BACKEND_PORT${NC}"
fi

if [ "$LAUNCH_MODE" != "backend-only" ] && ! check_port $FRONTEND_PORT; then
    echo -e "${YELLOW}Port $FRONTEND_PORT is in use, finding available port...${NC}"
    FRONTEND_PORT=$(find_available_port $FRONTEND_PORT)
    echo -e "${BLUE}Using frontend port: $FRONTEND_PORT${NC}"
fi

# Start components based on launch mode
case $LAUNCH_MODE in
    "backend-only")
        start_backend
        ;;
    "frontend-only")
        start_frontend
        ;;
    "tui-only")
        start_tui
        ;;
    "all"|*)
        start_backend
        start_frontend
        start_tui
        ;;
esac

# Print status information
echo ""
echo -e "${GREEN}==============================${NC}"
echo -e "${GREEN}✓ GhostWire started successfully!${NC}"
if [ "$LAUNCH_MODE" != "frontend-only" ] && [ "$LAUNCH_MODE" != "tui-only" ]; then
    echo -e "${CYAN}Backend API:   http://localhost:$BACKEND_PORT/api${NC}"
fi
if [ "$LAUNCH_MODE" != "backend-only" ] && [ "$LAUNCH_MODE" != "tui-only" ]; then
    echo -e "${CYAN}Frontend UI:   http://localhost:$FRONTEND_PORT${NC}"
fi
if [ "$LAUNCH_MODE" != "backend-only" ] && [ "$LAUNCH_MODE" != "frontend-only" ]; then
    echo -e "${CYAN}TUI:           Running in terminal${NC}"
fi
echo -e "${GREEN}==============================${NC}"
echo ""
echo -e "${YELLOW}Press Ctrl+C to stop all services${NC}"
echo ""

# Keep script running and monitor processes
while true; do
    # Check if any process has died
    if [ -n "$BACKEND_PID" ] && ! kill -0 $BACKEND_PID 2>/dev/null; then
        echo -e "${RED}Backend process died unexpectedly${NC}" >&2
        cleanup
    fi
    
    if [ -n "$FRONTEND_PID" ] && ! kill -0 $FRONTEND_PID 2>/dev/null; then
        echo -e "${RED}Frontend process died unexpectedly${NC}" >&2
        cleanup
    fi
    
    if [ -n "$TUI_PID" ] && ! kill -0 $TUI_PID 2>/dev/null; then
        echo -e "${YELLOW}TUI process ended${NC}"
        TUI_PID=""
    fi
    
    sleep 5
done 