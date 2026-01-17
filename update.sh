#!/bin/bash

# GhostWire Update Script
# Updates existing GhostWire installation to the latest version

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

# Parse arguments
AUTO_CONFIRM=false
while [[ $# -gt 0 ]]; do
    case $1 in
        -y|--yes)
            AUTO_CONFIRM=true
            shift
            ;;
        *)
            shift
            ;;
    esac
done

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
        log_info "Checked: $HOME/.local/ghostwire and /opt/ghostwire"
        exit 1
    fi
    
    log_info "Found installation at: $INSTALL_DIR"
}

# Get current version
get_current_version() {
    if [ -f "$INSTALL_DIR/bin/ghostwire" ]; then
        CURRENT_VERSION=$("$INSTALL_DIR/bin/ghostwire" --version 2>/dev/null | cut -d' ' -f2 || echo "unknown")
    else
        CURRENT_VERSION="unknown"
    fi
    log_info "Current version: $CURRENT_VERSION"
}

# Create backup
create_backup() {
    log_info "Creating backup..."
    
    BACKUP_DIR="$INSTALL_DIR.backup.$(date +%Y%m%d_%H%M%S)"
    cp -r "$INSTALL_DIR" "$BACKUP_DIR"
    
    log_success "Backup created at: $BACKUP_DIR"
    echo "$BACKUP_DIR" > /tmp/ghostwire_last_backup
}

# Pull latest changes
pull_updates() {
    log_info "Pulling latest changes..."
    
    if [ -d ".git" ]; then
        git pull origin main || git pull origin master
        log_success "Repository updated"
    else
        log_warning "Not a git repository. Skipping git pull."
        log_info "If you want to update via git, clone the repository first."
    fi
}

# Rebuild backend
rebuild_backend() {
    log_info "Rebuilding backend..."
    
    cd ghostwire
    
    if [ -d "vendor" ]; then
        cargo build --release --offline
    else
        cargo build --release
    fi
    
    log_success "Backend rebuilt"
    cd ..
}

# Update frontend
update_frontend() {
    log_info "Updating frontend..."
    
    cd webui
    npm install --silent
    log_success "Frontend updated"
    cd ..
}

# Install updated files
install_updates() {
    log_info "Installing updates..."
    
    # Stop service if running
    if pgrep -f "ghostwire" > /dev/null; then
        log_warning "Stopping GhostWire..."
        pkill -f "ghostwire" || true
        sleep 2
    fi
    
    # Copy new binaries
    cp ghostwire/target/release/ghostwire "$INSTALL_DIR/bin/"
    chmod +x "$INSTALL_DIR/bin/ghostwire"
    
    # Update frontend
    cp -r webui/* "$INSTALL_DIR/webui/"
    
    # Update scripts (preserve config)
    cp -r scripts/* "$INSTALL_DIR/scripts/"
    chmod +x "$INSTALL_DIR/scripts"/*.sh
    
    log_success "Updates installed"
}

# Verify installation
verify_installation() {
    log_info "Verifying installation..."
    
    if [ -f "$INSTALL_DIR/bin/ghostwire" ]; then
        NEW_VERSION=$("$INSTALL_DIR/bin/ghostwire" --version 2>/dev/null | cut -d' ' -f2 || echo "unknown")
        log_success "New version: $NEW_VERSION"
    else
        log_error "Verification failed: Binary not found"
        rollback
        exit 1
    fi
}

# Rollback on failure
rollback() {
    log_error "Update failed. Rolling back..."
    
    if [ -f /tmp/ghostwire_last_backup ]; then
        BACKUP_DIR=$(cat /tmp/ghostwire_last_backup)
        if [ -d "$BACKUP_DIR" ]; then
            rm -rf "$INSTALL_DIR"
            mv "$BACKUP_DIR" "$INSTALL_DIR"
            log_success "Rollback complete"
        fi
    else
        log_error "No backup found. Cannot rollback."
    fi
}

# Cleanup old backups
cleanup_backups() {
    log_info "Cleaning up old backups..."
    
    # Keep only the 3 most recent backups
    ls -dt "$INSTALL_DIR".backup.* 2>/dev/null | tail -n +4 | xargs rm -rf 2>/dev/null || true
    
    log_success "Cleanup complete"
}

# Main update flow
main() {
    echo -e "${CYAN}"
    cat <<'EOF'
╔═══════════════════════════════════════════╗
║     GhostWire Update Manager              ║
╚═══════════════════════════════════════════╝
EOF
    echo -e "${NC}"
    
    log_info "Starting GhostWire update..."
    echo ""
    
    find_install_dir
    get_current_version
    
    echo ""
    log_warning "This will update GhostWire to the latest version."
    log_warning "A backup will be created automatically."
    if [ "$AUTO_CONFIRM" = "true" ]; then
        log_info "Auto-confirm enabled, skipping confirmation prompt."
    else
        read -p "Continue? [Y/n] " -n 1 -r
        echo ""
        
        if [[ ! $REPLY =~ ^[Yy]$ ]] && [[ -n $REPLY ]]; then
            log_info "Update cancelled"
            exit 0
        fi
    fi
    
    echo ""
    create_backup
    
    # Try to update, rollback on failure
    {
        pull_updates
        rebuild_backend
        update_frontend
        install_updates
        verify_installation
    } || {
        rollback
        exit 1
    }
    
    cleanup_backups
    
    echo ""
    echo -e "${GREEN}╔═══════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║  ✓ Update completed successfully!        ║${NC}"
    echo -e "${GREEN}╚═══════════════════════════════════════════╝${NC}"
    echo ""
    log_info "Previous version: $CURRENT_VERSION"
    log_info "Current version: $NEW_VERSION"
    echo ""
    log_info "Start GhostWire with: $INSTALL_DIR/bin/ghostwire-launch"
}

main "$@"
