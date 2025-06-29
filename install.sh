#!/bin/bash

# 🌐 GhostWire - Universal Installation Script
# This script automatically detects your OS and sets up GhostWire

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
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
echo -e "${YELLOW}Universal Installation Script${NC}"
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

# Function to create start script
create_start_script() {
    echo -e "${BLUE}Creating start script...${NC}"
    
    cat > start-ghostwire.sh << 'EOF'
#!/bin/bash

# GhostWire Start Script
set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}🌐 Starting GhostWire...${NC}"

# Check if we're in the right directory
if [ ! -f "ghostwire/Cargo.toml" ]; then
    echo -e "${YELLOW}Please run this script from the Obsidian directory${NC}"
    exit 1
fi

# Start backend in background
echo -e "${BLUE}Starting backend server...${NC}"
cd ghostwire
cargo run &
BACKEND_PID=$!
cd ..

# Wait a moment for backend to start
sleep 3

# Start frontend in background
echo -e "${BLUE}Starting frontend server...${NC}"
cd webui
npm run dev &
FRONTEND_PID=$!
cd ..

echo -e "${GREEN}✓ GhostWire started successfully!${NC}"
echo -e "${YELLOW}Backend: http://127.0.0.1:3000${NC}"
echo -e "${YELLOW}Frontend: http://localhost:5173${NC}"
echo ""
echo -e "${BLUE}Press Ctrl+C to stop both servers${NC}"

# Function to cleanup on exit
cleanup() {
    echo -e "${BLUE}Stopping GhostWire...${NC}"
    kill $BACKEND_PID 2>/dev/null || true
    kill $FRONTEND_PID 2>/dev/null || true
    echo -e "${GREEN}✓ GhostWire stopped${NC}"
    exit 0
}

# Set trap to cleanup on script exit
trap cleanup SIGINT SIGTERM

# Wait for background processes
wait
EOF

    chmod +x start-ghostwire.sh
    echo -e "${GREEN}✓ Start script created${NC}"
}

# Function to create development start script
create_dev_script() {
    echo -e "${BLUE}Creating development script...${NC}"
    
    cat > dev-start.sh << 'EOF'
#!/bin/bash

# GhostWire Development Start Script
set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}🚀 Starting GhostWire in development mode...${NC}"

# Check if we're in the right directory
if [ ! -f "ghostwire/Cargo.toml" ]; then
    echo -e "${YELLOW}Please run this script from the Obsidian directory${NC}"
    exit 1
fi

# Start backend in watch mode
echo -e "${BLUE}Starting backend in watch mode...${NC}"
cd ghostwire
cargo watch -x run &
BACKEND_PID=$!
cd ..

# Start frontend in development mode
echo -e "${BLUE}Starting frontend in development mode...${NC}"
cd webui
npm run dev &
FRONTEND_PID=$!
cd ..

echo -e "${GREEN}✓ GhostWire development servers started!${NC}"
echo -e "${YELLOW}Backend: http://127.0.0.1:3000${NC}"
echo -e "${YELLOW}Frontend: http://localhost:5173${NC}"
echo ""
echo -e "${BLUE}Press Ctrl+C to stop both servers${NC}"

# Function to cleanup on exit
cleanup() {
    echo -e "${BLUE}Stopping development servers...${NC}"
    kill $BACKEND_PID 2>/dev/null || true
    kill $FRONTEND_PID 2>/dev/null || true
    echo -e "${GREEN}✓ Development servers stopped${NC}"
    exit 0
}

# Set trap to cleanup on script exit
trap cleanup SIGINT SIGTERM

# Wait for background processes
wait
EOF

    chmod +x dev-start.sh
    echo -e "${GREEN}✓ Development script created${NC}"
}

# Main installation process
main() {
    echo -e "${BLUE}Detecting operating system...${NC}"
    OS=$(detect_os)
    echo -e "${GREEN}✓ Detected OS: $OS${NC}"
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
    create_start_script
    create_dev_script
    echo ""
    
    # Final instructions
    echo -e "${GREEN}🎉 GhostWire installation completed successfully!${NC}"
    echo ""
    echo -e "${YELLOW}Next steps:${NC}"
    echo -e "${BLUE}1. Start GhostWire:${NC} ./start-ghostwire.sh"
    echo -e "${BLUE}2. Start development mode:${NC} ./dev-start.sh"
    echo -e "${BLUE}3. Access the web interface:${NC} http://localhost:5173"
    echo -e "${BLUE}4. Access the API:${NC} http://127.0.0.1:3000"
    echo ""
    echo -e "${CYAN}🌐 Welcome to GhostWire - Secure Communication for the Digital Age${NC}"
}

# Run main function
main "$@" 