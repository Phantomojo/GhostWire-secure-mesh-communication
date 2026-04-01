#!/bin/bash
# GhostWire First Ping Test
# File: test-first-ping.sh
#
# This script runs the first ping test and validates that two nodes
# can successfully exchange a message through the GhostWire protocol.
#
# Usage:
#   ./test-first-ping.sh [--clean] [--verbose]
#
# Options:
#   --clean     Remove old containers and volumes before starting
#   --verbose   Show detailed logs
#
# Exit codes:
#   0 - First ping successful
#   1 - Test failed
#   2 - Timeout

set -e

# Configuration
COMPOSE_FILE="docker-compose.first-ping.yml"
DISCOVERY_TIMEOUT=30
PING_TIMEOUT=60
CLEAN=false
VERBOSE=false

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --clean)
            CLEAN=true
            shift
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        *)
            echo "Unknown option: $1"
            echo "Usage: ./test-first-ping.sh [--clean] [--verbose]"
            exit 1
            ;;
    esac
done

# Helper functions
log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Clean up function
cleanup() {
    log_info "Cleaning up..."
    docker-compose -f "$COMPOSE_FILE" down --remove-orphans 2>/dev/null || true
}

# Set trap for cleanup on exit
trap cleanup EXIT

# Clean if requested
if [ "$CLEAN" = true ]; then
    log_info "Cleaning old containers and volumes..."
    docker-compose -f "$COMPOSE_FILE" down -v --remove-orphans 2>/dev/null || true
fi

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    log_error "Docker is not running"
    exit 1
fi

# Check if docker-compose is available
if ! command -v docker-compose &> /dev/null; then
    log_error "docker-compose is not installed"
    exit 1
fi

log_info "Starting GhostWire First Ping Test"
echo ""

# Build images if needed
log_info "Building Docker images..."
if [ "$VERBOSE" = true ]; then
    docker-compose -f "$COMPOSE_FILE" build
else
    docker-compose -f "$COMPOSE_FILE" build > /dev/null 2>&1
fi

# Start the test topology
log_info "Starting test topology..."
docker-compose -f "$COMPOSE_FILE" up -d

# Wait for bootstrap to be healthy
log_info "Waiting for bootstrap node to be ready..."
for i in $(seq 1 $DISCOVERY_TIMEOUT); do
    if docker-compose -f "$COMPOSE_FILE" ps | grep -q "bootstrap.*healthy"; then
        log_success "Bootstrap node is ready"
        break
    fi
    
    if [ $i -eq $DISCOVERY_TIMEOUT ]; then
        log_error "Bootstrap node failed to start"
        docker-compose -f "$COMPOSE_FILE" logs bootstrap
        exit 1
    fi
    
    sleep 1
done

# Wait for peer discovery
log_info "Waiting for peer discovery (timeout: ${DISCOVERY_TIMEOUT}s)..."
DISCOVERED=false

for i in $(seq 1 $DISCOVERY_TIMEOUT); do
    # Check node-a logs for peer discovery
    if docker-compose -f "$COMPOSE_FILE" logs node-a 2>&1 | grep -q "Peer discovered\|Connected to peer"; then
        log_success "Peer discovery successful"
        DISCOVERED=true
        break
    fi
    
    # Show progress every 10 seconds
    if [ $((i % 10)) -eq 0 ]; then
        log_info "Still waiting for discovery... ($i/${DISCOVERY_TIMEOUT}s)"
    fi
    
    sleep 1
done

if [ "$DISCOVERED" = false ]; then
    log_error "Peer discovery timed out"
    log_info "Node A logs:"
    docker-compose -f "$COMPOSE_FILE" logs node-a | tail -20
    log_info "Node B logs:"
    docker-compose -f "$COMPOSE_FILE" logs node-b | tail -20
    exit 1
fi

# Wait for first ping
log_info "Waiting for first ping message (timeout: ${PING_TIMEOUT}s)..."
PING_SUCCESS=false

for i in $(seq 1 $PING_TIMEOUT); do
    # Check for first ping success in logs
    if docker-compose -f "$COMPOSE_FILE" logs node-a 2>&1 | grep -q "First ping successful\|Hello World"; then
        log_success "FIRST PING SUCCESSFUL!"
        PING_SUCCESS=true
        break
    fi
    
    # Also check node-b
    if docker-compose -f "$COMPOSE_FILE" logs node-b 2>&1 | grep -q "First ping successful\|Hello World"; then
        log_success "FIRST PING SUCCESSFUL!"
        PING_SUCCESS=true
        break
    fi
    
    # Show progress every 10 seconds
    if [ $((i % 10)) -eq 0 ]; then
        log_info "Still waiting for first ping... ($i/${PING_TIMEOUT}s)"
    fi
    
    sleep 1
done

if [ "$PING_SUCCESS" = false ]; then
    log_error "First ping timed out"
    log_info "Node A logs:"
    docker-compose -f "$COMPOSE_FILE" logs node-a | tail -30
    log_info "Node B logs:"
    docker-compose -f "$COMPOSE_FILE" logs node-b | tail -30
    exit 1
fi

# Show final logs
echo ""
log_info "Final logs:"
if [ "$VERBOSE" = true ]; then
    docker-compose -f "$COMPOSE_FILE" logs
else
    docker-compose -f "$COMPOSE_FILE" logs | grep -E "(First ping|Hello World|Peer discovered|Connected)" | tail -10
fi

echo ""
log_success "🎉 First Ping Test PASSED!"
echo ""
echo "Summary:"
echo "  ✓ Bootstrap node started"
echo "  ✓ Peer discovery successful"
echo "  ✓ First ping message exchanged"
echo "  ✓ Protocol stack validated"
echo ""

exit 0
