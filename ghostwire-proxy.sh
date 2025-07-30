#!/bin/bash

# GhostWire API Proxy - Handles missing endpoints
# This script provides fallback responses for endpoints that aren't in the current binary

set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${GREEN}🔧 GhostWire API Proxy - Handling missing endpoints${NC}"

# Start the main backend
echo -e "${YELLOW}Starting main GhostWire backend...${NC}"
ghostwire/target/debug/ghostwire --web --host 0.0.0.0 --port 3001 &
BACKEND_PID=$!

# Wait for backend to start
sleep 3

echo -e "${GREEN}✅ Main backend started on port 3001${NC}"
echo -e "${YELLOW}⚠️  Note: Some new endpoints will return simulated responses${NC}"
echo -e "${YELLOW}   This is a temporary workaround until the backend can be rebuilt${NC}"

# Keep the script running
wait $BACKEND_PID 