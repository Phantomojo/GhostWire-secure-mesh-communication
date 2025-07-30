#!/bin/bash

# 🌐 GhostWire - Universal Multi-PC Installation Script
# This script automatically detects your OS and sets up GhostWire for multi-PC communication

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

echo -e "${GREEN}🌐 GhostWire - Secure Messaging Network${NC}"
echo -e "${YELLOW}Universal Multi-PC Installation Script${NC}"
echo ""

# Function to detect OS
detect_os() {
    case "$(uname -s)" in
        Linux*)     
            if grep -q Microsoft /proc/version; then
                echo "WSL"
            else
                echo "Linux"
            fi
            ;;
        Darwin*)    echo "macOS";;
        CYGWIN*)    echo "Windows";;
        MINGW*)     echo "Windows";;
        MSYS*)      echo "Windows";;
        *)          echo "Unknown";;
    esac
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to get local IP address
get_local_ip() {
    if [[ "$OS" == "Windows" ]]; then
        # Windows
        ipconfig | grep -A 5 "Ethernet adapter" | grep "IPv4" | head -1 | awk '{print $NF}'
    elif [[ "$OS" == "macOS" ]]; then
        # macOS
        ifconfig | grep "inet " | grep -v 127.0.0.1 | awk '{print $2}' | head -1
    else
        # Linux/WSL
        hostname -I | awk '{print $1}'
    fi
}

# Function to find available port
find_available_port() {
    local start_port=$1
    local port=$start_port
    
    while netstat -an 2>/dev/null | grep -q ":$port "; do
        port=$((port + 1))
    done
    
    echo $port
}

# Function to install Rust
install_rust() {
    echo -e "${BLUE}Installing Rust...${NC}"
    if ! command_exists rustc; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
        echo -e "${GREEN}✓ Rust installed successfully${NC}"
    else
        echo -e "${GREEN}✓ Rust already installed${NC}"
    fi
}

# Function to install Node.js
install_nodejs() {
    echo -e "${BLUE}Installing Node.js...${NC}"
    if ! command_exists node; then
        if [[ "$OS" == "Linux" || "$OS" == "WSL" ]]; then
            curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
            sudo apt-get install -y nodejs
        elif [[ "$OS" == "macOS" ]]; then
            if command_exists brew; then
                brew install node
            else
                echo -e "${RED}Please install Homebrew first: https://brew.sh/${NC}"
                exit 1
            fi
        elif [[ "$OS" == "Windows" ]]; then
            echo -e "${YELLOW}Please install Node.js from: https://nodejs.org/${NC}"
            exit 1
        fi
        echo -e "${GREEN}✓ Node.js installed successfully${NC}"
    else
        echo -e "${GREEN}✓ Node.js already installed${NC}"
    fi
}

# Function to install Git
install_git() {
    echo -e "${BLUE}Installing Git...${NC}"
    if ! command_exists git; then
        if [[ "$OS" == "Linux" || "$OS" == "WSL" ]]; then
            sudo apt-get update && sudo apt-get install -y git
        elif [[ "$OS" == "macOS" ]]; then
            if command_exists brew; then
                brew install git
            else
                echo -e "${RED}Please install Homebrew first: https://brew.sh/${NC}"
                exit 1
            fi
        elif [[ "$OS" == "Windows" ]]; then
            echo -e "${YELLOW}Please install Git from: https://git-scm.com/${NC}"
            exit 1
        fi
        echo -e "${GREEN}✓ Git installed successfully${NC}"
    else
        echo -e "${GREEN}✓ Git already installed${NC}"
    fi
}

# Function to clone and setup GhostWire
setup_ghostwire() {
    echo -e "${BLUE}Setting up GhostWire...${NC}"
    
    # Clone repository if not already present
    if [ ! -d "Obsidian" ]; then
        git clone https://github.com/Phantomojo/Obsidian.git
        cd Obsidian
    else
        echo -e "${YELLOW}Obsidian directory already exists, updating...${NC}"
        cd Obsidian
        git pull origin main
    fi
    
    # Build backend
    echo -e "${BLUE}Building GhostWire backend...${NC}"
    cd ghostwire
    cargo build --release
    echo -e "${GREEN}✓ Backend built successfully${NC}"
    
    # Setup frontend
    echo -e "${BLUE}Setting up GhostWire frontend...${NC}"
    cd ../webui
    npm install
    echo -e "${GREEN}✓ Frontend dependencies installed${NC}"
    
    cd ..
}

# Function to create multi-PC start script
create_multi_pc_script() {
    echo -e "${BLUE}Creating multi-PC start script...${NC}"
    
    local_ip=$(get_local_ip)
    backend_port=$(find_available_port 3001)
    frontend_port=$(find_available_port 5173)
    
    cat > start-multi-pc.sh << EOF
#!/bin/bash

# GhostWire Multi-PC Start Script
set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
PURPLE='\033[0;35m'
NC='\033[0m'

# Configuration
LOCAL_IP="${local_ip}"
BACKEND_PORT=${backend_port}
FRONTEND_PORT=${frontend_port}

echo -e "${CYAN}🌐 GhostWire Multi-PC Setup${NC}"
echo -e "${BLUE}Local IP: ${LOCAL_IP}${NC}"
echo -e "${BLUE}Backend Port: ${BACKEND_PORT}${NC}"
echo -e "${BLUE}Frontend Port: ${FRONTEND_PORT}${NC}"
echo ""

# Check if we're in the right directory
if [ ! -f "ghostwire/Cargo.toml" ]; then
    echo -e "${YELLOW}Please run this script from the Obsidian directory${NC}"
    exit 1
fi

# Update frontend configuration for this PC
echo -e "${BLUE}Configuring frontend for port ${BACKEND_PORT}...${NC}"
cd webui
sed -i.bak "s/localhost:3001/localhost:${BACKEND_PORT}/g" src/services/api.ts
sed -i.bak "s/localhost:3001/localhost:${BACKEND_PORT}/g" src/App.tsx
cd ..

# Start backend in background
echo -e "${BLUE}Starting backend server on port ${BACKEND_PORT}...${NC}"
cd ghostwire
cargo run -- --host 0.0.0.0 --port ${BACKEND_PORT} &
BACKEND_PID=\$!
cd ..

# Wait a moment for backend to start
sleep 3

# Start frontend in background
echo -e "${BLUE}Starting frontend server on port ${FRONTEND_PORT}...${NC}"
cd webui
npm run dev -- --port ${FRONTEND_PORT} &
FRONTEND_PID=\$!
cd ..

echo -e "${GREEN}✓ GhostWire started successfully!${NC}"
echo ""
echo -e "${PURPLE}🌐 Network Information:${NC}"
echo -e "${YELLOW}Backend API:${NC} http://${LOCAL_IP}:${BACKEND_PORT}"
echo -e "${YELLOW}Frontend UI:${NC} http://${LOCAL_IP}:${FRONTEND_PORT}"
echo -e "${YELLOW}Local Backend:${NC} http://localhost:${BACKEND_PORT}"
echo -e "${YELLOW}Local Frontend:${NC} http://localhost:${FRONTEND_PORT}"
echo ""
echo -e "${CYAN}🔍 Multi-PC Instructions:${NC}"
echo -e "${BLUE}1. Other PCs can connect to:${NC} http://${LOCAL_IP}:${FRONTEND_PORT}"
echo -e "${BLUE}2. Use the Peers tab to scan for other GhostWire nodes${NC}"
echo -e "${BLUE}3. Set your username in the Peers tab${NC}"
echo ""
echo -e "${BLUE}Press Ctrl+C to stop both servers${NC}"

# Function to cleanup on exit
cleanup() {
    echo -e "${BLUE}Stopping GhostWire...${NC}"
    kill \$BACKEND_PID 2>/dev/null || true
    kill \$FRONTEND_PID 2>/dev/null || true
    echo -e "${GREEN}✓ GhostWire stopped${NC}"
    exit 0
}

# Set trap to cleanup on script exit
trap cleanup SIGINT SIGTERM

# Wait for background processes
wait
EOF

    chmod +x start-multi-pc.sh
    echo -e "${GREEN}✓ Multi-PC start script created${NC}"
}

# Function to create network discovery script
create_network_script() {
    echo -e "${BLUE}Creating network discovery script...${NC}"
    
    cat > discover-peers.sh << 'EOF'
#!/bin/bash

# GhostWire Network Discovery Script
set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}🔍 GhostWire Network Discovery${NC}"
echo ""

# Get local network information
LOCAL_IP=$(hostname -I | awk '{print $1}')
echo -e "${BLUE}Your IP: ${LOCAL_IP}${NC}"

# Scan for other GhostWire nodes
echo -e "${BLUE}Scanning network for other GhostWire nodes...${NC}"

# Common ports for GhostWire
PORTS=(3001 3002 3003 3004 3005)

for port in "${PORTS[@]}"; do
    echo -e "${YELLOW}Scanning port ${port}...${NC}"
    
    # Scan local network range
    for i in {1..254}; do
        ip="192.168.1.${i}"
        if [ "$ip" != "$LOCAL_IP" ]; then
            if curl -s --connect-timeout 1 "http://${ip}:${port}/api/status" > /dev/null 2>&1; then
                echo -e "${GREEN}✓ Found GhostWire node at ${ip}:${port}${NC}"
                
                # Try to get node info
                if username=$(curl -s --connect-timeout 2 "http://${ip}:${port}/api/get_username" 2>/dev/null | grep -o '"data":"[^"]*"' | cut -d'"' -f4); then
                    echo -e "${CYAN}  Username: ${username}${NC}"
                fi
            fi
        fi
    done
done

echo ""
echo -e "${GREEN}✓ Network scan completed${NC}"
echo -e "${BLUE}Use the web interface to connect to discovered peers${NC}"
EOF

    chmod +x discover-peers.sh
    echo -e "${GREEN}✓ Network discovery script created${NC}"
}

# Function to create Windows batch file
create_windows_script() {
    echo -e "${BLUE}Creating Windows batch file...${NC}"
    
    local_ip=$(get_local_ip)
    backend_port=$(find_available_port 3001)
    frontend_port=$(find_available_port 5173)
    
    cat > start-ghostwire.bat << EOF
@echo off
chcp 65001 >nul
echo 🌐 GhostWire Multi-PC Setup
echo.
echo Local IP: ${local_ip}
echo Backend Port: ${backend_port}
echo Frontend Port: ${frontend_port}
echo.

REM Check if we're in the right directory
if not exist "ghostwire\\Cargo.toml" (
    echo Please run this script from the Obsidian directory
    pause
    exit /b 1
)

REM Update frontend configuration
echo Configuring frontend for port ${backend_port}...
cd webui
powershell -Command "(Get-Content src/services/api.ts) -replace 'localhost:3001', 'localhost:${backend_port}' | Set-Content src/services/api.ts"
powershell -Command "(Get-Content src/App.tsx) -replace 'localhost:3001', 'localhost:${backend_port}' | Set-Content src/App.tsx"
cd ..

REM Start backend
echo Starting backend server on port ${backend_port}...
start "GhostWire Backend" cmd /k "cd ghostwire && cargo run -- --host 0.0.0.0 --port ${backend_port}"

REM Wait for backend to start
timeout /t 3 /nobreak >nul

REM Start frontend
echo Starting frontend server on port ${frontend_port}...
start "GhostWire Frontend" cmd /k "cd webui && npm run dev -- --port ${frontend_port}"

echo.
echo ✓ GhostWire started successfully!
echo.
echo 🌐 Network Information:
echo Backend API: http://${local_ip}:${backend_port}
echo Frontend UI: http://${local_ip}:${frontend_port}
echo Local Backend: http://localhost:${backend_port}
echo Local Frontend: http://localhost:${frontend_port}
echo.
echo 🔍 Multi-PC Instructions:
echo 1. Other PCs can connect to: http://${local_ip}:${frontend_port}
echo 2. Use the Peers tab to scan for other GhostWire nodes
echo 3. Set your username in the Peers tab
echo.
pause
EOF

    echo -e "${GREEN}✓ Windows batch file created${NC}"
}

# Main installation process
main() {
    echo -e "${BLUE}Detecting operating system...${NC}"
    OS=$(detect_os)
    echo -e "${GREEN}✓ Detected OS: $OS${NC}"
    echo ""
    
    # Get network information
    echo -e "${BLUE}Detecting network configuration...${NC}"
    LOCAL_IP=$(get_local_ip)
    BACKEND_PORT=$(find_available_port 3001)
    FRONTEND_PORT=$(find_available_port 5173)
    echo -e "${GREEN}✓ Local IP: $LOCAL_IP${NC}"
    echo -e "${GREEN}✓ Backend Port: $BACKEND_PORT${NC}"
    echo -e "${GREEN}✓ Frontend Port: $FRONTEND_PORT${NC}"
    echo ""
    
    # Install dependencies
    install_git
    install_rust
    install_nodejs
    echo ""
    
    # Setup GhostWire
    setup_ghostwire
    echo ""
    
    # Create convenience scripts
    create_multi_pc_script
    create_network_script
    
    if [[ "$OS" == "Windows" ]]; then
        create_windows_script
    fi
    echo ""
    
    # Final instructions
    echo -e "${GREEN}🎉 GhostWire multi-PC installation completed successfully!${NC}"
    echo ""
    echo -e "${PURPLE}🌐 Network Configuration:${NC}"
    echo -e "${BLUE}Your IP Address:${NC} $LOCAL_IP"
    echo -e "${BLUE}Backend Port:${NC} $BACKEND_PORT"
    echo -e "${BLUE}Frontend Port:${NC} $FRONTEND_PORT"
    echo ""
    echo -e "${YELLOW}Next steps:${NC}"
    if [[ "$OS" == "Windows" ]]; then
        echo -e "${BLUE}1. Start GhostWire:${NC} start-ghostwire.bat"
    else
        echo -e "${BLUE}1. Start GhostWire:${NC} ./start-multi-pc.sh"
    fi
    echo -e "${BLUE}2. Scan for peers:${NC} ./discover-peers.sh"
    echo -e "${BLUE}3. Access the web interface:${NC} http://$LOCAL_IP:$FRONTEND_PORT"
    echo -e "${BLUE}4. Access the API:${NC} http://$LOCAL_IP:$BACKEND_PORT"
    echo ""
    echo -e "${CYAN}🔍 Multi-PC Setup:${NC}"
    echo -e "${BLUE}• Run this installer on each PC${NC}"
    echo -e "${BLUE}• Each PC will get a unique port${NC}"
    echo -e "${BLUE}• Use the Peers tab to discover other nodes${NC}"
    echo -e "${BLUE}• Set unique usernames on each PC${NC}"
    echo ""
    echo -e "${CYAN}🌐 Welcome to GhostWire - Secure Multi-PC Communication${NC}"
}

# Run main function
main "$@" 