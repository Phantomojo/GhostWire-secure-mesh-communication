#!/bin/bash

# GhostWire Automatic Update Checker
# Runs in background and checks for updates periodically
# Creates automatic backups before updating

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $1" >> "$LOG_FILE"; }
log_success() { echo -e "${GREEN}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $1" >> "$LOG_FILE"; }

# Find installation directory
find_install_dir() {
    if [ -n "$GHOSTWIRE_INSTALL_DIR" ]; then
        INSTALL_DIR="$GHOSTWIRE_INSTALL_DIR"
    elif [ -d "$HOME/.local/ghostwire" ]; then
        INSTALL_DIR="$HOME/.local/ghostwire"
    elif [ -d "/opt/ghostwire" ]; then
        INSTALL_DIR="/opt/ghostwire"
    else
        echo "GhostWire installation not found"
        exit 1
    fi
}

find_install_dir

# Configuration
CHECK_INTERVAL=${GHOSTWIRE_UPDATE_INTERVAL:-86400}  # 24 hours default
LOG_FILE="$INSTALL_DIR/logs/auto-update.log"
LAST_CHECK_FILE="$INSTALL_DIR/data/.last_update_check"
UPDATE_AVAILABLE_FILE="$INSTALL_DIR/data/.update_available"

# Create log directory if needed
mkdir -p "$INSTALL_DIR/logs"
mkdir -p "$INSTALL_DIR/data"

log_info "Auto-update checker started (interval: ${CHECK_INTERVAL}s)"

# Check if we're in a git repository
check_git_repo() {
    if [ ! -d "$(dirname "$INSTALL_DIR")/.git" ] && [ ! -d "$PWD/.git" ]; then
        log_info "Not in a git repository, cannot check for updates"
        return 1
    fi
    return 0
}

# Check for updates
check_for_updates() {
    log_info "Checking for updates..."
    
    # Record check time
    date +%s > "$LAST_CHECK_FILE"
    
    # Try to find git repository
    local git_dir=""
    if [ -d "$PWD/.git" ]; then
        git_dir="$PWD"
    elif [ -d "$(dirname "$INSTALL_DIR")/.git" ]; then
        git_dir="$(dirname "$INSTALL_DIR")"
    else
        log_info "No git repository found, skipping update check"
        return 0
    fi
    
    cd "$git_dir"
    
    # Fetch latest changes
    if ! git fetch origin &>/dev/null; then
        log_info "Failed to fetch updates (network issue?)"
        return 1
    fi
    
    # Check if we're behind
    local local_commit=$(git rev-parse HEAD)
    local remote_commit=$(git rev-parse @{u} 2>/dev/null || echo "$local_commit")
    
    if [ "$local_commit" != "$remote_commit" ]; then
        log_info "Update available! Local: $local_commit, Remote: $remote_commit"
        echo "$remote_commit" > "$UPDATE_AVAILABLE_FILE"
        
        # Send notification if available
        if command -v notify-send &> /dev/null; then
            notify-send "GhostWire Update Available" "A new version of GhostWire is available. Run 'ghostwire-manager.sh update' to install."
        fi
        
        return 0
    else
        log_info "Already up to date"
        rm -f "$UPDATE_AVAILABLE_FILE"
        return 1
    fi
}

# Auto-update if enabled
auto_update() {
    if [ ! -f "$UPDATE_AVAILABLE_FILE" ]; then
        return 0
    fi
    
    log_info "Auto-updating GhostWire..."
    
    # Create automatic backup
    local backup_dir="$INSTALL_DIR.auto-backup.$(date +%Y%m%d_%H%M%S)"
    cp -r "$INSTALL_DIR" "$backup_dir"
    log_success "Automatic backup created: $backup_dir"
    
    # Run update script
    if [ -f "$(dirname "$INSTALL_DIR")/update.sh" ]; then
        bash "$(dirname "$INSTALL_DIR")/update.sh" >> "$LOG_FILE" 2>&1
        
        if [ $? -eq 0 ]; then
            log_success "Auto-update completed successfully"
            rm -f "$UPDATE_AVAILABLE_FILE"
            
            # Notify user
            if command -v notify-send &> /dev/null; then
                notify-send "GhostWire Updated" "GhostWire has been automatically updated to the latest version."
            fi
        else
            log_info "Auto-update failed, check logs"
        fi
    fi
}

# Main loop
if [ "$1" = "--daemon" ]; then
    log_info "Running in daemon mode"
    
    while true; do
        # Check if enough time has passed
        if [ -f "$LAST_CHECK_FILE" ]; then
            last_check=$(cat "$LAST_CHECK_FILE")
            current_time=$(date +%s)
            time_diff=$((current_time - last_check))
            
            if [ $time_diff -lt $CHECK_INTERVAL ]; then
                sleep $((CHECK_INTERVAL - time_diff))
                continue
            fi
        fi
        
        # Check for updates
        if check_for_updates; then
            # If auto-update is enabled, update now
            if [ "$GHOSTWIRE_AUTO_UPDATE" = "true" ]; then
                auto_update
            fi
        fi
        
        sleep $CHECK_INTERVAL
    done
else
    # One-time check
    check_for_updates
    
    if [ -f "$UPDATE_AVAILABLE_FILE" ]; then
        echo ""
        echo -e "${GREEN}╔═══════════════════════════════════════════╗${NC}"
        echo -e "${GREEN}║   Update Available!                       ║${NC}"
        echo -e "${GREEN}╚═══════════════════════════════════════════╝${NC}"
        echo ""
        echo "A new version of GhostWire is available."
        echo ""
        echo "To update now:"
        echo "  ./ghostwire-manager.sh update"
        echo ""
        echo "Or enable automatic updates:"
        echo "  export GHOSTWIRE_AUTO_UPDATE=true"
        echo ""
    else
        echo "GhostWire is up to date."
    fi
fi
