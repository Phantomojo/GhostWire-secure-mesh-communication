#!/bin/bash

# GhostWire Uninstaller
# Safely removes GhostWire from your system

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[✓]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[!]${NC} $1"; }
log_error() { echo -e "${RED}[✗]${NC} $1"; }

# Find installation directory
find_install_dir() {
    if [ -n "$GHOSTWIRE_INSTALL_DIR" ]; then
        INSTALL_DIR="$GHOSTWIRE_INSTALL_DIR"
    elif [ -d "$HOME/.local/ghostwire" ]; then
        INSTALL_DIR="$HOME/.local/ghostwire"
    elif [ -d "/opt/ghostwire" ]; then
        INSTALL_DIR="/opt/ghostwire"
    else
        log_error "GhostWire installation not found"
        exit 1
    fi
    
    log_info "Found installation at: $INSTALL_DIR"
}

# Stop service
stop_service() {
    log_info "Stopping GhostWire service..."
    
    # Stop process
    if pgrep -f "ghostwire" > /dev/null; then
        pkill -f "ghostwire" || true
        sleep 2
        log_success "Service stopped"
    else
        log_info "Service not running"
    fi
    
    # Disable systemd service if exists
    if systemctl list-unit-files | grep -q ghostwire.service; then
        if [ "$EUID" -eq 0 ]; then
            systemctl stop ghostwire || true
            systemctl disable ghostwire || true
            rm /etc/systemd/system/ghostwire.service || true
            systemctl daemon-reload
            log_success "Systemd service removed"
        else
            log_warning "Run with sudo to remove systemd service"
        fi
    fi
    
    # Remove launchd service (macOS)
    if [[ "$OSTYPE" == "darwin"* ]]; then
        if [ -f "$HOME/Library/LaunchAgents/io.ghostwire.plist" ]; then
            launchctl unload "$HOME/Library/LaunchAgents/io.ghostwire.plist" || true
            rm "$HOME/Library/LaunchAgents/io.ghostwire.plist"
            log_success "Launchd service removed"
        fi
    fi
}

# Remove from PATH
remove_from_path() {
    log_info "Removing from PATH..."
    
    local shell_rc=""
    
    if [ -n "$BASH_VERSION" ]; then
        shell_rc="$HOME/.bashrc"
    elif [ -n "$ZSH_VERSION" ]; then
        shell_rc="$HOME/.zshrc"
    else
        shell_rc="$HOME/.profile"
    fi
    
    if [ -f "$shell_rc" ]; then
        # Remove GhostWire PATH entries
        sed -i.bak '/# GhostWire/d' "$shell_rc" || true
        sed -i.bak '/ghostwire/d' "$shell_rc" || true
        log_success "Removed from PATH in $shell_rc"
    fi
}

# Remove installation
remove_install() {
    log_info "Removing installation..."
    
    if [ -d "$INSTALL_DIR" ]; then
        rm -rf "$INSTALL_DIR"
        log_success "Installation removed"
    else
        log_info "Installation directory not found"
    fi
}

# Remove config and data
remove_config_data() {
    echo ""
    log_warning "Do you want to remove configuration and data files?"
    log_warning "This will delete your identity, contacts, and message history."
    read -p "Remove config and data? [y/N] " -n 1 -r
    echo ""
    
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        # Remove backups
        rm -rf "$INSTALL_DIR".backup.* 2>/dev/null || true
        log_success "Configuration and data removed"
    else
        log_info "Configuration and data preserved"
    fi
}

# Main uninstall flow
main() {
    echo -e "${RED}"
    cat <<'EOF'
╔═══════════════════════════════════════════╗
║     GhostWire Uninstaller                 ║
╚═══════════════════════════════════════════╝
EOF
    echo -e "${NC}"
    
    log_warning "This will completely remove GhostWire from your system."
    echo ""
    read -p "Are you sure you want to uninstall? [y/N] " -n 1 -r
    echo ""
    
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        log_info "Uninstall cancelled"
        exit 0
    fi
    
    echo ""
    find_install_dir
    stop_service
    remove_from_path
    remove_install
    remove_config_data
    
    echo ""
    echo -e "${GREEN}╔═══════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║  ✓ GhostWire successfully uninstalled    ║${NC}"
    echo -e "${GREEN}╚═══════════════════════════════════════════╝${NC}"
    echo ""
    log_info "Thank you for using GhostWire!"
}

main "$@"
