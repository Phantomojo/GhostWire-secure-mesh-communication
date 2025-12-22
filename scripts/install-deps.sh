#!/bin/bash

# GhostWire Dependency Installer
# Automatically installs all required dependencies for GhostWire
# Supports: Ubuntu/Debian, Fedora/RHEL, Arch, openSUSE, macOS

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[✓]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[!]${NC} $1"; }
log_error() { echo -e "${RED}[✗]${NC} $1"; }

# Detect OS and package manager
detect_os() {
    if [[ "$OSTYPE" == "darwin"* ]]; then
        OS="macos"
        PKG_MGR="brew"
    elif [ -f /etc/os-release ]; then
        . /etc/os-release
        OS=$ID
        
        case $OS in
            ubuntu|debian|pop|linuxmint)
                PKG_MGR="apt"
                ;;
            fedora|rhel|centos|rocky|almalinux)
                PKG_MGR="dnf"
                ;;
            arch|manjaro|endeavouros)
                PKG_MGR="pacman"
                ;;
            opensuse*|sles)
                PKG_MGR="zypper"
                ;;
            *)
                log_warning "Unknown Linux distribution: $OS"
                PKG_MGR="unknown"
                ;;
        esac
    else
        OS="unknown"
        PKG_MGR="unknown"
    fi
    
    log_info "Detected OS: $OS (Package Manager: $PKG_MGR)"
}

# Install Rust via rustup
install_rust() {
    if command -v rustc &> /dev/null; then
        log_success "Rust already installed: $(rustc --version)"
        return
    fi
    
    log_info "Installing Rust via rustup..."
    
    # Download and run rustup
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    
    # Source cargo environment
    source "$HOME/.cargo/env"
    
    log_success "Rust installed: $(rustc --version)"
}

# Install Node.js
install_nodejs() {
    if command -v node &> /dev/null; then
        local node_version=$(node --version | cut -d'v' -f2 | cut -d'.' -f1)
        if [ "$node_version" -ge 16 ]; then
            log_success "Node.js already installed: $(node --version)"
            return
        else
            log_warning "Node.js version too old. Installing newer version..."
        fi
    fi
    
    log_info "Installing Node.js..."
    
    case $PKG_MGR in
        apt)
            # Use NodeSource repository for latest LTS
            curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
            sudo apt-get install -y nodejs
            ;;
        dnf)
            sudo dnf install -y nodejs npm
            ;;
        pacman)
            sudo pacman -S --noconfirm nodejs npm
            ;;
        zypper)
            sudo zypper install -y nodejs npm
            ;;
        brew)
            brew install node
            ;;
        *)
            log_error "Unable to install Node.js automatically"
            log_info "Please install Node.js 16+ manually from: https://nodejs.org/"
            exit 1
            ;;
    esac
    
    log_success "Node.js installed: $(node --version)"
}

# Install system dependencies
install_system_deps() {
    log_info "Installing system dependencies..."
    
    case $PKG_MGR in
        apt)
            sudo apt-get update
            sudo apt-get install -y \
                build-essential \
                pkg-config \
                libssl-dev \
                git \
                curl \
                wget
            ;;
        dnf)
            sudo dnf install -y \
                gcc \
                gcc-c++ \
                make \
                pkgconfig \
                openssl-devel \
                git \
                curl \
                wget
            ;;
        pacman)
            sudo pacman -S --noconfirm \
                base-devel \
                openssl \
                git \
                curl \
                wget
            ;;
        zypper)
            sudo zypper install -y \
                gcc \
                gcc-c++ \
                make \
                pkg-config \
                libopenssl-devel \
                git \
                curl \
                wget
            ;;
        brew)
            brew install \
                openssl \
                pkg-config \
                git \
                curl \
                wget
            ;;
        *)
            log_warning "Unable to install system dependencies automatically"
            return
            ;;
    esac
    
    log_success "System dependencies installed"
}

# Install Git (if missing)
install_git() {
    if command -v git &> /dev/null; then
        log_success "Git already installed: $(git --version)"
        return
    fi
    
    log_info "Installing Git..."
    
    case $PKG_MGR in
        apt)
            sudo apt-get install -y git
            ;;
        dnf)
            sudo dnf install -y git
            ;;
        pacman)
            sudo pacman -S --noconfirm git
            ;;
        zypper)
            sudo zypper install -y git
            ;;
        brew)
            brew install git
            ;;
        *)
            log_error "Unable to install Git automatically"
            exit 1
            ;;
    esac
    
    log_success "Git installed"
}

# Main installation
main() {
    echo -e "${GREEN}╔═══════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║  GhostWire Dependency Installer              ║${NC}"
    echo -e "${GREEN}╚═══════════════════════════════════════════════╝${NC}"
    echo ""
    
    detect_os
    
    echo ""
    log_info "Installing dependencies..."
    echo ""
    
    install_system_deps
    install_git
    install_rust
    install_nodejs
    
    echo ""
    echo -e "${GREEN}╔═══════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║  ✓ All dependencies installed successfully!  ║${NC}"
    echo -e "${GREEN}╚═══════════════════════════════════════════════╝${NC}"
    echo ""
    
    log_info "Installed versions:"
    command -v rustc &> /dev/null && echo "  Rust: $(rustc --version)"
    command -v cargo &> /dev/null && echo "  Cargo: $(cargo --version)"
    command -v node &> /dev/null && echo "  Node.js: $(node --version)"
    command -v npm &> /dev/null && echo "  npm: $(npm --version)"
    command -v git &> /dev/null && echo "  Git: $(git --version)"
    
    echo ""
    log_success "You can now run the GhostWire installer: ./install.sh"
}

main "$@"
