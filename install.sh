#!/bin/bash

# GhostWire Installer - Cross-Platform Installation Script
# Supports: Linux (Ubuntu, Debian, Fedora, Arch, openSUSE) and macOS
# Version: 1.0.0

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Installation defaults
INSTALL_DIR=""
INSTALL_TYPE="user"  # user or system
INSTALL_SERVICE=false
SILENT_MODE=false
SETUP_USERNAME=""

# Banner
print_banner() {
    echo -e "${CYAN}"
    cat <<'EOF'
╔═══════════════════════════════════════════════════════════════╗
║                                                               ║
║   ██████╗ ██╗  ██╗ ██████╗ ███████╗████████╗██╗    ██╗██╗██████╗ ███████╗  ║
║  ██╔════╝ ██║  ██║██╔═══██╗██╔════╝╚══██╔══╝██║    ██║██║██╔══██╗██╔════╝  ║
║  ██║  ███╗███████║██║   ██║███████╗   ██║   ██║ █╗ ██║██║██████╔╝█████╗    ║
║  ██║   ██║██╔══██║██║   ██║╚════██║   ██║   ██║███╗██║██║██╔══██╗██╔══╝    ║
║  ╚██████╔╝██║  ██║╚██████╔╝███████║   ██║   ╚███╔███╔╝██║██║  ██║███████╗  ║
║   ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝    ╚══╝╚══╝ ╚═╝╚═╝  ╚═╝╚══════╝  ║
║                                                               ║
║              Secure Mesh Networking & Communication          ║
║                        Installer v1.0                        ║
╚═══════════════════════════════════════════════════════════════╝
EOF
    echo -e "${NC}"
}

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[✓]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[!]${NC} $1"
}

log_error() {
    echo -e "${RED}[✗]${NC} $1"
}

# Detect OS
detect_os() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        if [ -f /etc/os-release ]; then
            . /etc/os-release
            OS=$ID
            OS_VERSION=$VERSION_ID
        else
            OS="unknown"
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        OS="macos"
        OS_VERSION=$(sw_vers -productVersion)
    else
        OS="unknown"
    fi
    
    log_info "Detected OS: $OS $OS_VERSION"
}

# Check if running as root
check_root() {
    if [ "$EUID" -eq 0 ]; then
        log_warning "Running as root. This is not recommended for user installations."
        if [ "$INSTALL_TYPE" = "user" ]; then
            log_error "Please run without sudo for user installation, or use --system for system-wide installation."
            exit 1
        fi
    fi
}

# Check network connectivity
check_network() {
    log_info "Checking network connectivity..."
    
    if command -v ping &> /dev/null; then
        if ping -c 1 -W 2 8.8.8.8 &> /dev/null || ping -c 1 -W 2 1.1.1.1 &> /dev/null; then
            log_success "Network connectivity OK"
            return 0
        else
            log_warning "No network connectivity detected"
            log_warning "You may need to install dependencies manually"
            return 1
        fi
    else
        log_info "Cannot check network connectivity (ping not available)"
        return 0
    fi
}

# Check disk space
check_disk_space() {
    log_info "Checking available disk space..."
    
    local required_mb=2048  # 2GB minimum
    local available_mb=0
    
    if command -v df &> /dev/null; then
        # Get available space in MB for current directory
        available_mb=$(df -m . | awk 'NR==2 {print $4}')
        
        if [ "$available_mb" -lt "$required_mb" ]; then
            log_error "Insufficient disk space"
            log_error "Required: ${required_mb}MB, Available: ${available_mb}MB"
            log_error "Please free up disk space and try again"
            exit 1
        else
            log_success "Disk space OK (Available: ${available_mb}MB)"
        fi
    else
        log_warning "Cannot check disk space (df not available)"
    fi
}

# Check dependencies
check_dependencies() {
    log_info "Checking dependencies..."
    
    local missing_deps=()
    local has_network=$(check_network && echo "yes" || echo "no")
    
    # Check Rust
    if ! command -v rustc &> /dev/null; then
        log_warning "Rust not found"
        missing_deps+=("rust")
    else
        local rust_version=$(rustc --version | cut -d' ' -f2)
        # Check Rust version is >= 1.70
        local rust_major=$(echo $rust_version | cut -d'.' -f1)
        local rust_minor=$(echo $rust_version | cut -d'.' -f2)
        
        if [ "$rust_major" -eq 1 ] && [ "$rust_minor" -lt 70 ]; then
            log_warning "Rust version too old (need 1.70+, found $rust_version)"
            missing_deps+=("rust")
        else
            log_success "Rust found: $rust_version"
        fi
    fi
    
    # Check Cargo
    if ! command -v cargo &> /dev/null; then
        log_warning "Cargo not found"
        missing_deps+=("cargo")
    else
        local cargo_version=$(cargo --version | cut -d' ' -f2)
        log_success "Cargo found: $cargo_version"
    fi
    
    # Check Node.js
    if ! command -v node &> /dev/null; then
        log_warning "Node.js not found"
        missing_deps+=("node")
    else
        local node_version=$(node --version | cut -d'v' -f2)
        local node_major=$(echo $node_version | cut -d'.' -f1)
        
        # Check Node.js version is >= 16
        if [ "$node_major" -lt 16 ]; then
            log_warning "Node.js version too old (need 16+, found $node_version)"
            missing_deps+=("node")
        else
            log_success "Node.js found: v$node_version"
        fi
    fi
    
    # Check npm
    if ! command -v npm &> /dev/null; then
        log_warning "npm not found"
        missing_deps+=("npm")
    else
        local npm_version=$(npm --version)
        log_success "npm found: $npm_version"
    fi
    
    # Check Git
    if ! command -v git &> /dev/null; then
        log_warning "Git not found (optional but recommended for updates)"
    else
        local git_version=$(git --version | cut -d' ' -f3)
        log_success "Git found: $git_version"
    fi
    
    # Check build tools
    if ! command -v gcc &> /dev/null && ! command -v clang &> /dev/null; then
        log_warning "No C compiler found (gcc or clang)"
        missing_deps+=("build-tools")
    fi
    
    if [ ${#missing_deps[@]} -ne 0 ]; then
        log_error "Missing dependencies: ${missing_deps[*]}"
        echo ""
        
        # Show manual installation instructions if no network
        if [ "$has_network" = "no" ]; then
            log_error "No network connectivity detected. Please install dependencies manually:"
            show_manual_install_instructions
            exit 1
        fi
        
        log_info "Would you like to install missing dependencies automatically? [y/N]"
        
        if [ "$SILENT_MODE" = false ]; then
            read -r response
            if [[ "$response" =~ ^[Yy]$ ]]; then
                install_dependencies
                # Re-check after installation
                if ! verify_dependencies; then
                    log_error "Dependency installation failed or incomplete"
                    show_manual_install_instructions
                    exit 1
                fi
            else
                show_manual_install_instructions
                exit 1
            fi
        else
            log_error "Silent mode: Cannot install dependencies automatically."
            show_manual_install_instructions
            exit 1
        fi
    fi
}

# Show manual installation instructions
show_manual_install_instructions() {
    echo ""
    log_info "═══════════════════════════════════════════════════════════"
    log_info "Manual Installation Instructions:"
    log_info "═══════════════════════════════════════════════════════════"
    echo ""
    
    log_info "Option 1: Run the dependency installer script:"
    echo -e "  ${YELLOW}./scripts/install-deps.sh${NC}"
    echo ""
    
    log_info "Option 2: Install dependencies manually:"
    echo ""
    
    log_info "📦 Rust (required):"
    echo "  Download: https://rustup.rs/"
    echo "  Command:  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo ""
    
    log_info "📦 Node.js 16+ (required):"
    if [[ "$OS" == "ubuntu" ]] || [[ "$OS" == "debian" ]]; then
        echo "  Ubuntu/Debian: curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -"
        echo "                 sudo apt-get install -y nodejs"
    elif [[ "$OS" == "fedora" ]] || [[ "$OS" == "rhel" ]]; then
        echo "  Fedora/RHEL:   sudo dnf install nodejs npm"
    elif [[ "$OS" == "arch" ]]; then
        echo "  Arch:          sudo pacman -S nodejs npm"
    elif [[ "$OS" == "macos" ]]; then
        echo "  macOS:         brew install node"
    fi
    echo "  Download: https://nodejs.org/en/download/"
    echo ""
    
    log_info "📦 Build tools (required):"
    if [[ "$OS" == "ubuntu" ]] || [[ "$OS" == "debian" ]]; then
        echo "  Ubuntu/Debian: sudo apt-get install build-essential pkg-config libssl-dev"
    elif [[ "$OS" == "fedora" ]] || [[ "$OS" == "rhel" ]]; then
        echo "  Fedora/RHEL:   sudo dnf install gcc gcc-c++ make pkgconfig openssl-devel"
    elif [[ "$OS" == "arch" ]]; then
        echo "  Arch:          sudo pacman -S base-devel openssl"
    elif [[ "$OS" == "macos" ]]; then
        echo "  macOS:         xcode-select --install"
    fi
    echo ""
    
    log_info "📦 Git (optional, for updates):"
    if [[ "$OS" == "ubuntu" ]] || [[ "$OS" == "debian" ]]; then
        echo "  Ubuntu/Debian: sudo apt-get install git"
    elif [[ "$OS" == "fedora" ]] || [[ "$OS" == "rhel" ]]; then
        echo "  Fedora/RHEL:   sudo dnf install git"
    elif [[ "$OS" == "arch" ]]; then
        echo "  Arch:          sudo pacman -S git"
    elif [[ "$OS" == "macos" ]]; then
        echo "  macOS:         brew install git"
    fi
    echo "  Download: https://git-scm.com/downloads"
    echo ""
    
    log_info "After installing dependencies, run this installer again:"
    echo -e "  ${GREEN}./install.sh${NC}"
    echo ""
}

# Verify dependencies after installation attempt
verify_dependencies() {
    command -v rustc &> /dev/null && \
    command -v cargo &> /dev/null && \
    command -v node &> /dev/null && \
    command -v npm &> /dev/null
}

# Install dependencies
install_dependencies() {
    log_info "Installing dependencies..."
    
    if [ -f "./scripts/install-deps.sh" ]; then
        bash ./scripts/install-deps.sh
    else
        log_error "Dependency installer not found at ./scripts/install-deps.sh"
        exit 1
    fi
}

# Get installation directory
get_install_dir() {
    if [ -n "$INSTALL_DIR" ]; then
        return
    fi
    
    if [ "$INSTALL_TYPE" = "system" ]; then
        INSTALL_DIR="/opt/ghostwire"
    else
        INSTALL_DIR="$HOME/.local/ghostwire"
    fi
    
    log_info "Installation directory: $INSTALL_DIR"
}

# Prompt for username
prompt_username() {
    if [ -n "$SETUP_USERNAME" ]; then
        return
    fi
    
    if [ "$SILENT_MODE" = false ]; then
        echo ""
        log_info "Enter your username for GhostWire (e.g., alice, bob@home, node-alpha)"
        read -r -p "Username: " SETUP_USERNAME
        
        if [ -z "$SETUP_USERNAME" ]; then
            log_warning "No username provided. Using default: $(whoami)-node"
            SETUP_USERNAME="$(whoami)-node"
        fi
    else
        SETUP_USERNAME="$(whoami)-node"
    fi
    
    log_success "Username set to: $SETUP_USERNAME"
}

# Build backend
build_backend() {
    log_info "Building GhostWire backend (Rust)..."
    log_info "This may take several minutes on first build..."
    
    cd ghostwire
    
    # Check if Cargo.toml exists
    if [ ! -f "Cargo.toml" ]; then
        log_error "Cargo.toml not found. Are you in the correct directory?"
        cd ..
        exit 1
    fi
    
    # Create build log
    local build_log="$PWD/../build.log"
    
    # Check if we should use vendored dependencies
    if [ -d "vendor" ]; then
        log_info "Using vendored dependencies..."
        export CARGO_HOME="$(pwd)/.cargo"
        
        # Build in release mode with vendored deps
        if cargo build --release --offline > "$build_log" 2>&1; then
            log_success "Backend built successfully"
        else
            log_error "Backend build failed"
            echo ""
            log_error "Build failed. Common issues:"
            log_error "1. Missing system dependencies (OpenSSL, pkg-config)"
            log_error "2. Rust version too old (need 1.70+)"
            log_error "3. Insufficient memory (need ~2GB RAM for build)"
            echo ""
            log_info "Full build log saved to: $build_log"
            log_info "Last 20 lines of error:"
            tail -n 20 "$build_log"
            echo ""
            log_info "To install system dependencies:"
            log_info "  ./scripts/install-deps.sh"
            cd ..
            exit 1
        fi
    else
        # Check network for downloading dependencies
        if ! check_network; then
            log_error "No network connection and no vendored dependencies"
            log_error "Cannot download Rust crates without network"
            log_info "Either restore network connection or use offline installation"
            cd ..
            exit 1
        fi
        
        # Build in release mode
        if cargo build --release > "$build_log" 2>&1; then
            log_success "Backend built successfully"
        else
            log_error "Backend build failed"
            echo ""
            log_error "Build failed. Common issues:"
            log_error "1. Missing system dependencies (OpenSSL, pkg-config)"
            log_error "2. Network issues downloading crates"
            log_error "3. Rust version too old (need 1.70+)"
            log_error "4. Insufficient memory (need ~2GB RAM for build)"
            echo ""
            log_info "Full build log saved to: $build_log"
            log_info "Last 20 lines of error:"
            tail -n 20 "$build_log"
            echo ""
            log_info "Troubleshooting steps:"
            log_info "1. Install system dependencies: ./scripts/install-deps.sh"
            log_info "2. Check Rust version: rustc --version (need 1.70+)"
            log_info "3. Update Rust: rustup update"
            log_info "4. Clear cargo cache: rm -rf ~/.cargo/registry && cargo build"
            cd ..
            exit 1
        fi
    fi
    
    # Verify binary was created
    if [ ! -f "target/release/ghostwire" ]; then
        log_error "Build completed but binary not found"
        log_error "Expected: target/release/ghostwire"
        cd ..
        exit 1
    fi
    
    cd ..
}

# Setup frontend
setup_frontend() {
    log_info "Setting up GhostWire frontend (Node.js)..."
    
    cd webui
    
    # Install dependencies
    npm install --silent 2>&1 | while IFS= read -r line; do
        echo "  $line"
    done
    
    if [ $? -eq 0 ]; then
        log_success "Frontend setup complete"
    else
        log_error "Frontend setup failed"
        exit 1
    fi
    
    cd ..
}

# Create installation directories
create_directories() {
    log_info "Creating installation directories..."
    
    mkdir -p "$INSTALL_DIR/bin"
    mkdir -p "$INSTALL_DIR/config"
    mkdir -p "$INSTALL_DIR/data"
    mkdir -p "$INSTALL_DIR/logs"
    mkdir -p "$INSTALL_DIR/webui"
    
    log_success "Directories created"
}

# Copy files
copy_files() {
    log_info "Copying files to installation directory..."
    
    # Copy backend binary
    cp ghostwire/target/release/ghostwire "$INSTALL_DIR/bin/"
    chmod +x "$INSTALL_DIR/bin/ghostwire"
    
    # Copy frontend
    cp -r webui/* "$INSTALL_DIR/webui/"
    
    # Copy scripts
    mkdir -p "$INSTALL_DIR/scripts"
    cp -r scripts/* "$INSTALL_DIR/scripts/"
    chmod +x "$INSTALL_DIR/scripts"/*.sh
    
    # Copy default configs
    if [ -d "config" ]; then
        cp -r config/* "$INSTALL_DIR/config/"
    fi
    
    # Copy documentation
    mkdir -p "$INSTALL_DIR/docs"
    cp README.md "$INSTALL_DIR/docs/" 2>/dev/null || true
    cp INSTALL.md "$INSTALL_DIR/docs/" 2>/dev/null || true
    
    log_success "Files copied"
}

# Create configuration
create_config() {
    log_info "Creating configuration files..."
    
    # Create identity config
    cat > "$INSTALL_DIR/config/identity.toml" <<EOF
# GhostWire Identity Configuration
# Generated on $(date)

[identity]
username = "$SETUP_USERNAME"
display_name = "$SETUP_USERNAME"

# Note: Keypair will be auto-generated on first run
# Public key and PeerID will be saved here automatically
EOF
    
    # Create network config with NAT traversal
    cat > "$INSTALL_DIR/config/network.toml" <<EOF
# GhostWire Network Configuration

[network]
# Listen address for P2P
listen_addr = "/ip4/0.0.0.0/tcp/4001"

# Enable NAT traversal for internet connectivity
enable_relay = true
enable_autonat = true
enable_dcutr = true

# Relay servers for NAT traversal
relay_servers = []

# Bootstrap nodes for peer discovery across the internet
bootstrap_nodes = [
    "/dnsaddr/bootstrap.libp2p.io/p2p/QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN",
    "/dnsaddr/bootstrap.libp2p.io/p2p/QmQCU2EcMqAqQPR2i9bChDtGNJchTbq5TbXJJ16u19uLTa",
]

# STUN servers for NAT hole punching
stun_servers = ["stun:stun.l.google.com:19302"]

[web]
# Web UI host and port
host = "0.0.0.0"
port = 3001

[security]
# Security settings
enable_encryption = true
enable_stealth = true
EOF
    
    log_success "Configuration files created"
}

# Create launcher script
create_launcher() {
    log_info "Creating launcher script..."
    
    cat > "$INSTALL_DIR/bin/ghostwire-launch" <<'EOF'
#!/bin/bash
# GhostWire Launcher Script

INSTALL_DIR="$(cd "$(dirname "$0")/.." && pwd)"

# Set environment
export GHOSTWIRE_CONFIG_DIR="$INSTALL_DIR/config"
export GHOSTWIRE_DATA_DIR="$INSTALL_DIR/data"
export GHOSTWIRE_LOG_DIR="$INSTALL_DIR/logs"

# Launch backend
"$INSTALL_DIR/bin/ghostwire" --config "$INSTALL_DIR/config/network.toml" "$@"
EOF
    
    chmod +x "$INSTALL_DIR/bin/ghostwire-launch"
    
    log_success "Launcher script created"
}

# Add to PATH
add_to_path() {
    log_info "Adding GhostWire to PATH..."
    
    local shell_rc=""
    
    if [ -n "$BASH_VERSION" ]; then
        shell_rc="$HOME/.bashrc"
    elif [ -n "$ZSH_VERSION" ]; then
        shell_rc="$HOME/.zshrc"
    else
        shell_rc="$HOME/.profile"
    fi
    
    local path_line="export PATH=\"$INSTALL_DIR/bin:\$PATH\""
    
    if ! grep -q "$path_line" "$shell_rc" 2>/dev/null; then
        echo "" >> "$shell_rc"
        echo "# GhostWire" >> "$shell_rc"
        echo "$path_line" >> "$shell_rc"
        log_success "Added to PATH in $shell_rc"
        log_warning "Please run: source $shell_rc"
    else
        log_info "Already in PATH"
    fi
}

# Install service
install_service() {
    if [ "$INSTALL_SERVICE" = false ]; then
        return
    fi
    
    log_info "Installing system service..."
    
    if [[ "$OS" == "macos" ]]; then
        install_launchd_service
    else
        install_systemd_service
    fi
}

# Install systemd service (Linux)
install_systemd_service() {
    if [ ! -f "systemd/ghostwire.service" ]; then
        log_warning "systemd service file not found. Skipping service installation."
        return
    fi
    
    log_info "Installing systemd service..."
    
    # Check if we have sudo
    if [ "$EUID" -ne 0 ]; then
        log_error "Service installation requires sudo privileges"
        log_info "Run: sudo ./systemd/install-service.sh"
        return
    fi
    
    # Install service
    cp systemd/ghostwire.service /etc/systemd/system/
    systemctl daemon-reload
    systemctl enable ghostwire
    
    log_success "Systemd service installed"
    log_info "Start service with: sudo systemctl start ghostwire"
}

# Install launchd service (macOS)
install_launchd_service() {
    log_info "Installing launchd service..."
    
    local plist_file="$HOME/Library/LaunchAgents/io.ghostwire.plist"
    
    cat > "$plist_file" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>io.ghostwire</string>
    <key>ProgramArguments</key>
    <array>
        <string>$INSTALL_DIR/bin/ghostwire-launch</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>$INSTALL_DIR/logs/ghostwire.log</string>
    <key>StandardErrorPath</key>
    <string>$INSTALL_DIR/logs/ghostwire-error.log</string>
</dict>
</plist>
EOF
    
    launchctl load "$plist_file"
    
    log_success "Launchd service installed"
    log_info "Service will start automatically on login"
}

# Print summary
print_summary() {
    echo ""
    echo -e "${GREEN}╔═══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║                                                               ║${NC}"
    echo -e "${GREEN}║  🎉  GhostWire Installation Complete!  🎉                    ║${NC}"
    echo -e "${GREEN}║                                                               ║${NC}"
    echo -e "${GREEN}╚═══════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "${CYAN}📦 Installation Details:${NC}"
    echo -e "  Location: ${GREEN}$INSTALL_DIR${NC}"
    echo -e "  Username: ${GREEN}$SETUP_USERNAME${NC}"
    echo -e "  Type: ${GREEN}$INSTALL_TYPE${NC}"
    echo ""
    echo -e "${CYAN}🚀 Quick Start:${NC}"
    echo -e "  1. Start GhostWire:"
    echo -e "     ${YELLOW}$INSTALL_DIR/bin/ghostwire-launch${NC}"
    echo ""
    echo -e "  2. Or use the launcher script:"
    echo -e "     ${YELLOW}cd $INSTALL_DIR && ./launch-ghostwire-working.sh${NC}"
    echo ""
    echo -e "${CYAN}🌐 Access Points:${NC}"
    echo -e "  Web UI: ${GREEN}http://localhost:3001${NC}"
    echo -e "  API: ${GREEN}http://localhost:3001/api${NC}"
    echo ""
    echo -e "${CYAN}🔧 Management:${NC}"
    echo -e "  View logs: ${YELLOW}tail -f $INSTALL_DIR/logs/*.log${NC}"
    echo -e "  Edit config: ${YELLOW}$INSTALL_DIR/config/network.toml${NC}"
    echo -e "  Update: ${YELLOW}./update.sh${NC}"
    echo ""
    echo -e "${CYAN}📚 Documentation:${NC}"
    echo -e "  README: ${YELLOW}$INSTALL_DIR/docs/README.md${NC}"
    echo -e "  Install Guide: ${YELLOW}$INSTALL_DIR/docs/INSTALL.md${NC}"
    echo ""
    echo -e "${PURPLE}💡 Note: Other nodes can discover you by your username: ${GREEN}$SETUP_USERNAME${NC}"
    echo ""
}

# Parse arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --system)
                INSTALL_TYPE="system"
                shift
                ;;
            --user)
                INSTALL_TYPE="user"
                shift
                ;;
            --dir)
                INSTALL_DIR="$2"
                shift 2
                ;;
            --service)
                INSTALL_SERVICE=true
                shift
                ;;
            --no-service)
                INSTALL_SERVICE=false
                shift
                ;;
            --silent)
                SILENT_MODE=true
                shift
                ;;
            --username)
                SETUP_USERNAME="$2"
                shift 2
                ;;
            --help|-h)
                print_help
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                print_help
                exit 1
                ;;
        esac
    done
}

# Print help
print_help() {
    cat <<EOF
GhostWire Installer

Usage: ./install.sh [OPTIONS]

Options:
    --system            Install system-wide (requires sudo)
    --user              Install for current user (default)
    --dir DIR           Custom installation directory
    --service           Install as system service
    --no-service        Don't install system service (default)
    --silent            Silent mode (no prompts)
    --username NAME     Set username for identity
    --help, -h          Show this help message

Examples:
    # User installation (default)
    ./install.sh

    # System-wide installation with service
    sudo ./install.sh --system --service

    # Custom directory
    ./install.sh --dir /custom/path

    # Silent installation
    ./install.sh --silent --username alice

EOF
}

# Main installation flow
main() {
    parse_args "$@"
    
    print_banner
    
    log_info "Starting GhostWire installation..."
    echo ""
    
    detect_os
    check_root
    check_disk_space
    check_dependencies
    get_install_dir
    prompt_username
    
    echo ""
    log_info "Building GhostWire..."
    build_backend
    setup_frontend
    
    echo ""
    log_info "Installing GhostWire to $INSTALL_DIR..."
    create_directories
    copy_files
    create_config
    create_launcher
    add_to_path
    install_service
    
    print_summary
}

# Run main
main "$@"
