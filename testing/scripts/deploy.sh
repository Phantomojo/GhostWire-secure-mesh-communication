#!/bin/bash
# GhostWire Fast Deployment Script
# Uses rsync instead of scp for resumable, faster transfers
# Also handles node management (start/stop/restart)

set -e

STRAT_HOST="strat"
OMEN_HOST="omen"
STRAT_IP="192.168.100.142"
OMEN_IP="192.168.100.7"
BINARY="ghostwire/target/release/ghostwire"
SSH_OPTS="-o ConnectTimeout=10 -o StrictHostKeyChecking=no"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log() { echo -e "${BLUE}[DEPLOY]${NC} $1"; }
ok() { echo -e "${GREEN}[OK]${NC} $1"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
err() { echo -e "${RED}[ERROR]${NC} $1"; }

usage() {
    echo "Usage: $0 {deploy|start|stop|restart|status|logs|clean}"
    echo ""
    echo "Commands:"
    echo "  deploy    - Build and deploy binary to both nodes (rsync)"
    echo "  start     - Start GhostWire on both nodes"
    echo "  stop      - Stop GhostWire on both nodes"
    echo "  restart   - Restart GhostWire on both nodes"
    echo "  status    - Check status on both nodes"
    echo "  logs      - Show recent logs from both nodes"
    echo "  clean     - Clean state (kill + remove data)"
    echo ""
    echo "Options:"
    echo "  --strat-only   - Only operate on strat"
    echo "  --omen-only    - Only operate on omen"
    exit 1
}

# Check if host is reachable
check_host() {
    local host=$1
    if ssh $SSH_OPTS $host "echo ok" >/dev/null 2>&1; then
        return 0
    else
        return 1
    fi
}

# Deploy binary using rsync (faster than scp, resumable)
deploy_binary() {
    local host=$1
    log "Deploying to $host via rsync..."

    if ! check_host $host; then
        err "$host is not reachable"
        return 1
    fi

    # Create remote directory
    ssh $SSH_OPTS $host "mkdir -p ~/ghostwire"

    # Rsync binary (only transfers if changed, compressed)
    rsync -avz --progress -e "ssh $SSH_OPTS" \
        $BINARY $host:~/ghostwire/ghostwire

    # Make executable
    ssh $SSH_OPTS $host "chmod +x ~/ghostwire/ghostwire"

    ok "Deployed to $host"
}

# Start GhostWire node
start_node() {
    local host=$1
    local port=$2
    local mesh_port=$3
    local bootstrap=$4

    log "Starting GhostWire on $host (port $port, mesh $mesh_port)..."

    if ! check_host $host; then
        err "$host is not reachable"
        return 1
    fi

    # Kill existing
    ssh $SSH_OPTS $host "pkill -9 ghostwire 2>/dev/null || true"
    sleep 1

    # Clean old logs
    ssh $SSH_OPTS $host "> ~/ghostwire/node.log 2>/dev/null || true"

    # Build start command
    local cmd="cd ~/ghostwire && nohup ./ghostwire --web --api-host 0.0.0.0 --mesh-host 0.0.0.0 --port $port --mesh-port $mesh_port"
    if [ -n "$bootstrap" ]; then
        cmd="$cmd --bootstrap '$bootstrap'"
    fi
    cmd="$cmd > ~/ghostwire/node.log 2>&1 &"

    ssh $SSH_OPTS $host "$cmd"

    # Wait for startup
    sleep 5

    # Check if running
    if ssh $SSH_OPTS $host "pgrep -x ghostwire >/dev/null 2>&1"; then
        ok "GhostWire started on $host"
        # Show peer ID
        ssh $SSH_OPTS $host "grep 'Peer ID:' ~/ghostwire/node.log | tail -1" || true
    else
        err "GhostWire failed to start on $host"
        ssh $SSH_OPTS $host "tail -20 ~/ghostwire/node.log"
        return 1
    fi
}

# Stop GhostWire node
stop_node() {
    local host=$1
    log "Stopping GhostWire on $host..."
    ssh $SSH_OPTS $host "pkill ghostwire 2>/dev/null || true"
    ok "Stopped on $host"
}

# Check status
status_node() {
    local host=$1
    local port=$2

    echo -e "\n${BLUE}=== $host ===${NC}"

    # Process check
    if ssh $SSH_OPTS $host "pgrep -x ghostwire >/dev/null 2>&1"; then
        echo -e "  Process: ${GREEN}RUNNING${NC}"
        ssh $SSH_OPTS $host "ps aux | grep ghostwire | grep -v grep | awk '{print \"  PID: \" \$2, \"CPU: \" \$3\"%\", \"MEM: \" \$4\"%\"}'"
    else
        echo -e "  Process: ${RED}NOT RUNNING${NC}"
    fi

    # API check
    local health
    health=$(ssh $SSH_OPTS $host "curl -s http://localhost:$port/api/v2/health 2>/dev/null" || echo "FAILED")
    if echo "$health" | grep -q "healthy"; then
        echo -e "  API:     ${GREEN}HEALTHY${NC}"
    else
        echo -e "  API:     ${RED}UNREACHABLE${NC}"
    fi

    # Recent log entries
    echo -e "  ${YELLOW}Recent logs:${NC}"
    ssh $SSH_OPTS $host "tail -5 ~/ghostwire/node.log 2>/dev/null | sed 's/^/    /'" || echo "    No logs"
}

# Show logs
show_logs() {
    local host=$1
    local lines=${2:-30}
    local grep_filter=${3:-""}

    echo -e "\n${BLUE}=== $host (last $lines lines) ===${NC}"
    if [ -n "$grep_filter" ]; then
        ssh $SSH_OPTS $host "tail -$lines ~/ghostwire/node.log 2>/dev/null | grep -i '$grep_filter'" || echo "No matching logs"
    else
        ssh $SSH_OPTS $host "tail -$lines ~/ghostwire/node.log 2>/dev/null" || echo "No logs"
    fi
}

# Clean state
clean_node() {
    local host=$1
    log "Cleaning $host..."
    ssh $SSH_OPTS $host "pkill -9 ghostwire 2>/dev/null || true"
    ssh $SSH_OPTS $host "rm -rf ~/.ghostwire && mkdir -p ~/.ghostwire"
    ok "Cleaned $host"
}

# Main
case "${1:-}" in
    deploy)
        if [ "$2" != "--omen-only" ]; then
            deploy_binary $STRAT_HOST
        fi
        if [ "$2" != "--strat-only" ]; then
            deploy_binary $OMEN_HOST
        fi
        ;;
    start)
        STRAT_PEER_ID=""
        if [ "$2" != "--omen-only" ]; then
            start_node $STRAT_HOST 8080 4001 ""
            # Get strat's peer ID for omen bootstrap
            STRAT_PEER_ID=$(ssh $SSH_OPTS $STRAT_HOST "grep 'Peer ID:' ~/ghostwire/node.log | tail -1 | awk '{print \$NF}'" 2>/dev/null || echo "")
        fi

        if [ "$2" != "--strat-only" ]; then
            if [ -n "$STRAT_PEER_ID" ]; then
                BOOTSTRAP="/ip4/$STRAT_IP/tcp/4001/p2p/$STRAT_PEER_ID"
                log "Bootstrap: $BOOTSTRAP"
            else
                BOOTSTRAP=""
                warn "Could not get strat peer ID, omen will use mDNS only"
            fi
            start_node $OMEN_HOST 8081 4001 "$BOOTSTRAP"
        fi
        ;;
    stop)
        if [ "$2" != "--omen-only" ]; then
            stop_node $STRAT_HOST
        fi
        if [ "$2" != "--strat-only" ]; then
            stop_node $OMEN_HOST
        fi
        ;;
    restart)
        $0 stop
        sleep 2
        $0 start
        ;;
    status)
        status_node $STRAT_HOST 8080
        status_node $OMEN_HOST 8081
        ;;
    logs)
        show_logs $STRAT_HOST ${2:-30} ${3:-""}
        show_logs $OMEN_HOST ${2:-30} ${3:-""}
        ;;
    clean)
        if [ "$2" != "--omen-only" ]; then
            clean_node $STRAT_HOST
        fi
        if [ "$2" != "--strat-only" ]; then
            clean_node $OMEN_HOST
        fi
        ;;
    *)
        usage
        ;;
esac
