#!/bin/bash

# GhostWire Manager - Unified management script
# Commands: start, stop, restart, status, logs, update, backup, restore, uninstall

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
PURPLE='\033[0;35m'
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
}

# Start GhostWire
cmd_start() {
    log_info "Starting GhostWire..."
    
    if pgrep -f "ghostwire" > /dev/null; then
        log_warning "GhostWire is already running"
        cmd_status
        exit 0
    fi
    
    # Start in background
    nohup "$INSTALL_DIR/bin/ghostwire-launch" > "$INSTALL_DIR/logs/ghostwire.log" 2>&1 &
    
    sleep 2
    
    if pgrep -f "ghostwire" > /dev/null; then
        log_success "GhostWire started successfully"
        log_info "Web UI: http://localhost:3001"
        log_info "View logs: $0 logs"
    else
        log_error "Failed to start GhostWire"
        log_info "Check logs: $INSTALL_DIR/logs/ghostwire.log"
        exit 1
    fi
}

# Stop GhostWire
cmd_stop() {
    log_info "Stopping GhostWire..."
    
    if ! pgrep -f "ghostwire" > /dev/null; then
        log_info "GhostWire is not running"
        exit 0
    fi
    
    pkill -f "ghostwire"
    sleep 2
    
    if pgrep -f "ghostwire" > /dev/null; then
        log_warning "Process still running, forcing stop..."
        pkill -9 -f "ghostwire"
    fi
    
    log_success "GhostWire stopped"
}

# Restart GhostWire
cmd_restart() {
    log_info "Restarting GhostWire..."
    cmd_stop
    sleep 1
    cmd_start
}

# Show status
cmd_status() {
    echo -e "${CYAN}╔═══════════════════════════════════════════╗${NC}"
    echo -e "${CYAN}║        GhostWire Status                   ║${NC}"
    echo -e "${CYAN}╚═══════════════════════════════════════════╝${NC}"
    echo ""
    
    # Process status
    if pgrep -f "ghostwire" > /dev/null; then
        echo -e "Status: ${GREEN}● Running${NC}"
        PID=$(pgrep -f "ghostwire")
        echo -e "PID: ${GREEN}$PID${NC}"
        
        # Memory usage
        if command -v ps &> /dev/null; then
            MEM=$(ps -p $PID -o rss= | awk '{print $1/1024 " MB"}')
            echo -e "Memory: ${GREEN}$MEM${NC}"
        fi
    else
        echo -e "Status: ${RED}● Stopped${NC}"
    fi
    
    # Version
    if [ -f "$INSTALL_DIR/bin/ghostwire" ]; then
        VERSION=$("$INSTALL_DIR/bin/ghostwire" --version 2>/dev/null | cut -d' ' -f2 || echo "unknown")
        echo -e "Version: ${GREEN}$VERSION${NC}"
    fi
    
    # Installation directory
    echo -e "Install Dir: ${BLUE}$INSTALL_DIR${NC}"
    
    # Web UI
    if pgrep -f "ghostwire" > /dev/null; then
        echo ""
        echo -e "${CYAN}Access Points:${NC}"
        echo -e "  Web UI: ${GREEN}http://localhost:3001${NC}"
        echo -e "  API: ${GREEN}http://localhost:3001/api${NC}"
    fi
}

# View logs
cmd_logs() {
    local follow=false
    local lines=50
    
    if [ "$1" = "-f" ] || [ "$1" = "--follow" ]; then
        follow=true
    fi
    
    if [ "$1" = "-n" ]; then
        lines=$2
    fi
    
    log_info "Viewing logs from: $INSTALL_DIR/logs/ghostwire.log"
    echo ""
    
    if [ "$follow" = true ]; then
        tail -f "$INSTALL_DIR/logs/ghostwire.log"
    else
        tail -n $lines "$INSTALL_DIR/logs/ghostwire.log"
    fi
}

# Update GhostWire
cmd_update() {
    if [ -f "./update.sh" ]; then
        bash ./update.sh
    else
        log_error "Update script not found"
        log_info "Run from GhostWire repository directory"
        exit 1
    fi
}

# Check for updates
cmd_check_updates() {
    if [ -f "./scripts/auto-update-checker.sh" ]; then
        bash ./scripts/auto-update-checker.sh
    else
        log_error "Update checker script not found"
        exit 1
    fi
}

# Show identity information
cmd_show_identity() {
    log_info "Retrieving GhostWire identity..."
    
    local identity_file="$HOME/.ghostwire/identity.json"
    
    if [ ! -f "$identity_file" ]; then
        log_warning "No identity file found. GhostWire hasn't been run yet."
        log_info "Start GhostWire to create an identity: $0 start"
        exit 0
    fi
    
    echo -e "${CYAN}╔═══════════════════════════════════════════╗${NC}"
    echo -e "${CYAN}║        GhostWire Identity                 ║${NC}"
    echo -e "${CYAN}╚═══════════════════════════════════════════╝${NC}"
    echo ""
    
    # Extract identity information using jq if available
    if command -v jq &> /dev/null; then
        local username=$(jq -r '.metadata.nickname' "$identity_file" 2>/dev/null || echo "Unknown")
        local did=$(jq -r '.did' "$identity_file" 2>/dev/null || echo "Unknown")
        local id=$(jq -r '.id' "$identity_file" 2>/dev/null || echo "Unknown")
        
        echo -e "Username: ${GREEN}$username${NC}"
        echo -e "DID: ${BLUE}$did${NC}"
        echo -e "Node ID: ${BLUE}$id${NC}"
    else
        cat "$identity_file"
    fi
    
    echo ""
}

# Set username
cmd_set_username() {
    local new_username=$1
    
    if [ -z "$new_username" ]; then
        log_error "Please specify a username"
        log_info "Usage: $0 set-username <your-username>"
        exit 1
    fi
    
    local identity_file="$HOME/.ghostwire/identity.json"
    
    if [ ! -f "$identity_file" ]; then
        log_warning "No identity file found. Creating new identity with username: $new_username"
        log_info "Start GhostWire to create the identity: $0 start"
        exit 0
    fi
    
    log_info "Updating username to: $new_username"
    
    # Update using jq if available
    if command -v jq &> /dev/null; then
        local temp_file="${identity_file}.tmp"
        jq ".metadata.nickname = \"$new_username\" | .metadata.last_updated = $(date +%s)" "$identity_file" > "$temp_file"
        mv "$temp_file" "$identity_file"
        log_success "Username updated to: $new_username"
        log_info "Restart GhostWire for changes to take effect: $0 restart"
    else
        log_error "jq is required for username updates. Please install jq."
        log_info "Alternative: Use the WebUI at http://localhost:3001"
        exit 1
    fi
}

# Enable automatic updates
cmd_enable_auto_update() {
    log_info "Configuring automatic updates..."
    
    # Create/update config file
    local config_file="$INSTALL_DIR/ghostwire.conf"
    
    if [ -f "ghostwire.conf" ]; then
        cp ghostwire.conf "$config_file"
    fi
    
    # Enable auto-update in config
    if [ -f "$config_file" ]; then
        sed -i 's/GHOSTWIRE_AUTO_UPDATE=false/GHOSTWIRE_AUTO_UPDATE=true/' "$config_file"
        log_success "Automatic updates enabled"
    else
        echo "GHOSTWIRE_AUTO_UPDATE=true" > "$config_file"
        log_success "Configuration created with auto-updates enabled"
    fi
    
    # Start auto-update checker in background
    if [ -f "./scripts/auto-update-checker.sh" ]; then
        nohup bash ./scripts/auto-update-checker.sh --daemon > "$INSTALL_DIR/logs/auto-update-daemon.log" 2>&1 &
        log_success "Auto-update daemon started"
        log_info "Updates will be checked every 24 hours"
        log_info "Automatic backups will be created before each update"
    fi
}

# Disable automatic updates
cmd_disable_auto_update() {
    log_info "Disabling automatic updates..."
    
    # Update config file
    local config_file="$INSTALL_DIR/ghostwire.conf"
    
    if [ -f "$config_file" ]; then
        sed -i 's/GHOSTWIRE_AUTO_UPDATE=true/GHOSTWIRE_AUTO_UPDATE=false/' "$config_file"
        log_success "Automatic updates disabled"
    fi
    
    # Stop auto-update daemon
    pkill -f "auto-update-checker.sh --daemon" || true
    log_success "Auto-update daemon stopped"
}

# Create backup
cmd_backup() {
    log_info "Creating backup..."
    
    BACKUP_DIR="$HOME/ghostwire-backup-$(date +%Y%m%d_%H%M%S)"
    mkdir -p "$BACKUP_DIR"
    
    # Backup config
    cp -r "$INSTALL_DIR/config" "$BACKUP_DIR/"
    
    # Backup data
    if [ -d "$INSTALL_DIR/data" ]; then
        cp -r "$INSTALL_DIR/data" "$BACKUP_DIR/"
    fi
    
    log_success "Backup created at: $BACKUP_DIR"
    
    # Create archive
    if command -v tar &> /dev/null; then
        tar -czf "$BACKUP_DIR.tar.gz" -C "$(dirname $BACKUP_DIR)" "$(basename $BACKUP_DIR)"
        rm -rf "$BACKUP_DIR"
        log_success "Archive created: $BACKUP_DIR.tar.gz"
    fi
}

# Restore from backup
cmd_restore() {
    local backup_file=$1
    
    if [ -z "$backup_file" ]; then
        log_error "Please specify backup file"
        log_info "Usage: $0 restore <backup-file.tar.gz>"
        exit 1
    fi
    
    if [ ! -f "$backup_file" ]; then
        log_error "Backup file not found: $backup_file"
        exit 1
    fi
    
    log_info "Restoring from backup..."
    log_warning "This will overwrite current configuration and data"
    read -p "Continue? [y/N] " -n 1 -r
    echo ""
    
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        log_info "Restore cancelled"
        exit 0
    fi
    
    # Extract backup
    TEMP_DIR=$(mktemp -d)
    tar -xzf "$backup_file" -C "$TEMP_DIR"
    
    # Stop service
    cmd_stop || true
    
    # Restore files
    if [ -d "$TEMP_DIR/config" ]; then
        cp -r "$TEMP_DIR"/*/config/* "$INSTALL_DIR/config/" 2>/dev/null || cp -r "$TEMP_DIR/config"/* "$INSTALL_DIR/config/"
    fi
    
    if [ -d "$TEMP_DIR/data" ] || [ -d "$TEMP_DIR"/*/data ]; then
        cp -r "$TEMP_DIR"/*/data/* "$INSTALL_DIR/data/" 2>/dev/null || cp -r "$TEMP_DIR/data"/* "$INSTALL_DIR/data/" 2>/dev/null || true
    fi
    
    rm -rf "$TEMP_DIR"
    
    log_success "Restore complete"
    log_info "Start GhostWire: $0 start"
}

# Uninstall GhostWire
cmd_uninstall() {
    if [ -f "./uninstall.sh" ]; then
        bash ./uninstall.sh
    else
        log_error "Uninstall script not found"
        exit 1
    fi
}

# Show help
cmd_help() {
    cat <<EOF
${CYAN}╔═══════════════════════════════════════════════════╗${NC}
${CYAN}║         GhostWire Manager - Help                  ║${NC}
${CYAN}╚═══════════════════════════════════════════════════╝${NC}

${PURPLE}Usage:${NC} $0 <command> [options]

${PURPLE}Commands:${NC}
  ${GREEN}start${NC}              Start GhostWire
  ${GREEN}stop${NC}               Stop GhostWire
  ${GREEN}restart${NC}            Restart GhostWire
  ${GREEN}status${NC}             Show service status
  ${GREEN}logs${NC}               View logs (last 50 lines)
  ${GREEN}logs -f${NC}            Follow logs (real-time)
  ${GREEN}logs -n <N>${NC}        View last N lines
  ${GREEN}update${NC}             Update to latest version
  ${GREEN}check-updates${NC}      Check if updates are available
  ${GREEN}auto-update on${NC}     Enable automatic updates (with backups)
  ${GREEN}auto-update off${NC}    Disable automatic updates
  ${GREEN}backup${NC}             Create backup of config and data
  ${GREEN}restore <file>${NC}    Restore from backup
  ${GREEN}show-identity${NC}      Show your GhostWire username and DID
  ${GREEN}set-username <name>${NC} Set your username
  ${GREEN}uninstall${NC}          Uninstall GhostWire
  ${GREEN}help${NC}               Show this help

${PURPLE}Examples:${NC}
  $0 start                  # Start GhostWire
  $0 status                 # Check if running
  $0 logs -f                # View logs in real-time
  $0 backup                 # Create backup
  $0 update                 # Update to latest version
  $0 restart                # Restart service

${PURPLE}Installation Directory:${NC} $INSTALL_DIR

EOF
}

# Main
main() {
    find_install_dir
    
    case "${1:-help}" in
        start)
            cmd_start
            ;;
        stop)
            cmd_stop
            ;;
        restart)
            cmd_restart
            ;;
        status)
            cmd_status
            ;;
        logs)
            shift
            cmd_logs "$@"
            ;;
        update)
            cmd_update
            ;;
        check-updates)
            cmd_check_updates
            ;;
        auto-update)
            case "${2:-}" in
                on|enable)
                    cmd_enable_auto_update
                    ;;
                off|disable)
                    cmd_disable_auto_update
                    ;;
                *)
                    log_error "Usage: $0 auto-update [on|off]"
                    exit 1
                    ;;
            esac
            ;;
        backup)
            cmd_backup
            ;;
        restore)
            shift
            cmd_restore "$@"
            ;;
        show-identity)
            cmd_show_identity
            ;;
        set-username)
            shift
            cmd_set_username "$@"
            ;;
        uninstall)
            cmd_uninstall
            ;;
        help|--help|-h)
            cmd_help
            ;;
        *)
            log_error "Unknown command: $1"
            echo ""
            cmd_help
            exit 1
            ;;
    esac
}

main "$@"
