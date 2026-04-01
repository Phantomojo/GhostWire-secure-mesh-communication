#!/bin/bash
# GhostWire Startup Script with Enhanced Logging
# This script starts GhostWire with comprehensive logging for debugging profile creation

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GHOSTWIRE_BIN="$SCRIPT_DIR/ghostwire/target/release/ghostwire"
LOG_DIR="$SCRIPT_DIR/logs"
LOG_FILE="$LOG_DIR/ghostwire_$(date +%Y%m%d_%H%M%S).log"

# Create log directory
mkdir -p "$LOG_DIR"

echo -e "${BLUE}=========================================${NC}"
echo -e "${BLUE}  GhostWire Startup with Debug Logging${NC}"
echo -e "${BLUE}=========================================${NC}"
echo ""

# Check if binary exists
if [ ! -f "$GHOSTWIRE_BIN" ]; then
    echo -e "${RED}❌ Error: GhostWire binary not found at $GHOSTWIRE_BIN${NC}"
    echo "Please build first with: cd ghostwire && cargo build --release"
    exit 1
fi

# Check if config exists
CONFIG_FILE="$SCRIPT_DIR/ghostwire.toml"
if [ ! -f "$CONFIG_FILE" ]; then
    echo -e "${YELLOW}⚠️  Config file not found, using defaults${NC}"
fi

echo -e "${GREEN}✓ Binary found:${NC} $GHOSTWIRE_BIN"
echo -e "${GREEN}✓ Log file:${NC} $LOG_FILE"
echo ""

# Set up environment
export RUST_LOG="info,profile_creation=debug,identity=debug,mesh=info,web=info"
export RUST_BACKTRACE=1

echo -e "${YELLOW}📝 Starting GhostWire with enhanced logging...${NC}"
echo -e "${YELLOW}   Press Ctrl+C to stop${NC}"
echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Start GhostWire with logging
cd "$SCRIPT_DIR/ghostwire"
"$GHOSTWIRE_BIN" 2>&1 | tee -a "$LOG_FILE"

# Capture exit code
EXIT_CODE=$?

echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

if [ $EXIT_CODE -eq 0 ]; then
    echo -e "${GREEN}✓ GhostWire stopped normally${NC}"
else
    echo -e "${RED}✗ GhostWire exited with code: $EXIT_CODE${NC}"
fi

echo ""
echo -e "${YELLOW}📄 Log file saved to: $LOG_FILE${NC}"
echo ""
echo -e "${BLUE}Quick commands:${NC}"
echo "  View errors:     grep 'ERROR\|CRITICAL' $LOG_FILE"
echo "  View profile:    grep 'profile_creation' $LOG_FILE"
echo "  View identity:   grep 'identity' $LOG_FILE"
echo "  Full log:        cat $LOG_FILE"
echo ""

exit $EXIT_CODE
