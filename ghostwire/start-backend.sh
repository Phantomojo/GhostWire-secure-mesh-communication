#!/bin/bash

# GhostWire Backend Launcher
# This script starts the backend server for cross-PC communication

set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}🚀 Starting GhostWire Backend Server${NC}"

# Configuration
BACKEND_HOST=${BACKEND_HOST:-"0.0.0.0"}
BACKEND_PORT=${BACKEND_PORT:-3001}

# Check if we have a pre-built binary
if [ -f "target/release/ghostwire" ]; then
    echo -e "${GREEN}✓ Found pre-built binary${NC}"
    echo -e "${YELLOW}Starting backend on ${BACKEND_HOST}:${BACKEND_PORT}${NC}"
    ./target/release/ghostwire web --host $BACKEND_HOST --port $BACKEND_PORT
elif [ -f "target/debug/ghostwire" ]; then
    echo -e "${GREEN}✓ Found debug binary${NC}"
    echo -e "${YELLOW}Starting backend on ${BACKEND_HOST}:${BACKEND_PORT}${NC}"
    ./target/debug/ghostwire web --host $BACKEND_HOST --port $BACKEND_PORT
else
    echo -e "${RED}❌ No binary found. Attempting to build...${NC}"
    
    # Try to fix rustup issue
    export RUSTUP_TOOLCHAIN="stable"
    unset CARGO_PROXY
    unset RUSTUP_PROXY
    
    # Try building with system rustc
    if command -v rustc >/dev/null 2>&1; then
        echo -e "${YELLOW}Using system rustc...${NC}"
        RUSTC=$(which rustc) cargo build --release
    else
        echo -e "${RED}❌ Rust compiler not found${NC}"
        echo -e "${YELLOW}Please install Rust: https://rustup.rs/${NC}"
        exit 1
    fi
    
    if [ -f "target/release/ghostwire" ]; then
        echo -e "${GREEN}✓ Build successful${NC}"
        echo -e "${YELLOW}Starting backend on ${BACKEND_HOST}:${BACKEND_PORT}${NC}"
        ./target/release/ghostwire web --host $BACKEND_HOST --port $BACKEND_PORT
    else
        echo -e "${RED}❌ Build failed${NC}"
        echo -e "${YELLOW}Starting mock backend server...${NC}"
        
        # Start a simple mock server for testing
        python3 -m http.server $BACKEND_PORT --bind $BACKEND_HOST &
        MOCK_PID=$!
        echo -e "${GREEN}✓ Mock server started on ${BACKEND_HOST}:${BACKEND_PORT}${NC}"
        echo -e "${YELLOW}Press Ctrl+C to stop${NC}"
        trap "kill $MOCK_PID" EXIT
        wait $MOCK_PID
    fi
fi 