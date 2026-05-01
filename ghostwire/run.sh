#!/usr/bin/env bash
# GhostWire Startup Script
# Usage: ./run.sh [command] [options]
#
# Commands:
#   start       - Start the GhostWire node (default)
#   cli         - Run CLI commands
#   status      - Check node status
#   stop        - Stop running node
#   logs        - View logs
#   build       - Build from source
#
# Examples:
#   ./run.sh                          # Start with defaults
#   ./run.sh start --port 9090        # Start on port 9090
#   ./run.sh cli init                 # Initialize identity
#   ./run.sh cli send -r peer_id -m "hello"  # Send message

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINARY="$SCRIPT_DIR/target/release/ghostwire"
LOG_FILE="/tmp/ghostwire.log"
PID_FILE="/tmp/ghostwire.pid"
TOKEN_FILE="$HOME/.config/ghostwire/token"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

info() { echo -e "${BLUE}[INFO]${NC} $*"; }
ok()   { echo -e "${GREEN}[OK]${NC} $*"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $*"; }
err()  { echo -e "${RED}[ERROR]${NC} $*" >&2; }

build() {
    info "Building GhostWire (release mode)..."
    cd "$SCRIPT_DIR"
    cargo build --release 2>&1 | tail -5
    if [[ -f "$BINARY" ]]; then
        ok "Build complete: $BINARY"
    else
        err "Build failed"
        exit 1
    fi
}

start_node() {
    if is_running; then
        warn "GhostWire is already running (PID: $(cat "$PID_FILE"))"
        info "Use './run.sh stop' to stop it first"
        exit 0
    fi

    info "Starting GhostWire node..."
    
    # Default args
    local port=8080
    local mesh_port=0
    local mesh_host="127.0.0.1"
    local extra_args=()
    
    # Parse args
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --port|-p) port="$2"; shift 2 ;;
            --mesh-port) mesh_port="$2"; shift 2 ;;
            --mesh-host) mesh_host="$2"; shift 2 ;;
            --bootstrap) extra_args+=("--bootstrap" "$2"); shift 2 ;;
            --web) extra_args+=("--web"); shift ;;
            *) shift ;;
        esac
    done
    
    # Enable web by default
    if [[ ! " ${extra_args[*]:-} " =~ " --web " ]]; then
        extra_args+=("--web")
    fi
    
    info "Configuration:"
    info "  Web port: $port"
    info "  Mesh host: $mesh_host"
    info "  Mesh port: $mesh_port (0 = auto)"
    
    # Start in background
    RUST_LOG=info "$BINARY" \
        --web --port "$port" \
        --mesh-host "$mesh_host" --mesh-port "$mesh_port" \
        "${extra_args[@]}" \
        > "$LOG_FILE" 2>&1 &
    
    local pid=$!
    echo "$pid" > "$PID_FILE"
    
    info "GhostWire starting (PID: $pid)..."
    info "Waiting for server to be ready..."
    
    # Wait for server to start
    local retries=0
    while [[ $retries -lt 30 ]]; do
        if curl -s "http://127.0.0.1:$port/api/v2/health" > /dev/null 2>&1; then
            ok "GhostWire is running!"
            show_connection_info "$port"
            return 0
        fi
        sleep 1
        retries=$((retries + 1))
    done
    
    warn "Server may not have started properly. Check logs:"
    warn "  tail -f $LOG_FILE"
    return 1
}

show_connection_info() {
    local port=${1:-8080}
    echo ""
    echo -e "${GREEN}═══════════════════════════════════════════${NC}"
    echo -e "${GREEN}  GhostWire Node Started Successfully${NC}"
    echo -e "${GREEN}═══════════════════════════════════════════${NC}"
    echo ""
    echo -e "  ${BLUE}Web UI:${NC}      http://127.0.0.1:$port"
    echo -e "  ${BLUE}API:${NC}         http://127.0.0.1:$port/api/v2/"
    echo -e "  ${BLUE}Logs:${NC}        tail -f $LOG_FILE"
    echo -e "  ${BLUE}Stop:${NC}        ./run.sh stop"
    echo ""
    
    if [[ -f "$TOKEN_FILE" ]]; then
        local token
        token=$(cat "$TOKEN_FILE")
        echo -e "  ${BLUE}Auth Token:${NC}  $token"
        echo -e "  ${BLUE}API Call:${NC}    curl -H 'X-Ghost-Token: $token' http://127.0.0.1:$port/api/v2/peers"
    fi
    
    echo ""
    echo -e "${GREEN}═══════════════════════════════════════════${NC}"
}

stop_node() {
    if ! is_running; then
        warn "GhostWire is not running"
        return 0
    fi
    
    local pid
    pid=$(cat "$PID_FILE")
    info "Stopping GhostWire (PID: $pid)..."
    
    kill "$pid" 2>/dev/null || true
    sleep 2
    
    if is_running; then
        warn "Process didn't stop gracefully, forcing..."
        kill -9 "$pid" 2>/dev/null || true
    fi
    
    rm -f "$PID_FILE"
    ok "GhostWire stopped"
}

is_running() {
    if [[ -f "$PID_FILE" ]]; then
        local pid
        pid=$(cat "$PID_FILE")
        if kill -0 "$pid" 2>/dev/null; then
            return 0
        fi
    fi
    return 1
}

show_status() {
    if is_running; then
        local pid
        pid=$(cat "$PID_FILE")
        ok "GhostWire is running (PID: $pid)"
        
        # Try to get API status
        local port=8080
        if curl -s "http://127.0.0.1:$port/api/v2/health" > /dev/null 2>&1; then
            ok "API is responding on port $port"
        fi
    else
        warn "GhostWire is not running"
    fi
}

show_logs() {
    if [[ -f "$LOG_FILE" ]]; then
        tail -f "$LOG_FILE"
    else
        warn "No log file found at $LOG_FILE"
    fi
}

run_cli() {
    if [[ ! -f "$BINARY" ]]; then
        err "Binary not found. Run './run.sh build' first."
        exit 1
    fi
    
    "$BINARY" cli "$@"
}

# Main
case "${1:-start}" in
    start)
        shift || true
        start_node "$@"
        ;;
    stop)
        stop_node
        ;;
    restart)
        stop_node
        sleep 1
        shift || true
        start_node "$@"
        ;;
    status)
        show_status
        ;;
    logs)
        show_logs
        ;;
    build)
        build
        ;;
    cli)
        shift || true
        run_cli "$@"
        ;;
    --help|-h|help)
        echo "GhostWire Startup Script"
        echo ""
        echo "Usage: ./run.sh [command] [options]"
        echo ""
        echo "Commands:"
        echo "  start [--port PORT] [--mesh-port PORT] [--bootstrap ADDR]  Start node"
        echo "  stop                                                        Stop node"
        echo "  restart                                                     Restart node"
        echo "  status                                                      Show status"
        echo "  logs                                                        View logs"
        echo "  build                                                       Build from source"
        echo "  cli <subcommand> [args]                                     Run CLI command"
        echo ""
        echo "CLI Subcommands:"
        echo "  init [--username NAME]    Initialize identity"
        echo "  send -r RECIPIENT -m MSG  Send message"
        echo "  peers                     List peers"
        echo "  stats                     Show statistics"
        echo "  security                  Show security info"
        echo "  gen-key                   Generate key pair"
        echo "  connect -u USERNAME       Connect by username"
        echo ""
        ;;
    *)
        err "Unknown command: $1"
        echo "Run './run.sh --help' for usage"
        exit 1
        ;;
esac
