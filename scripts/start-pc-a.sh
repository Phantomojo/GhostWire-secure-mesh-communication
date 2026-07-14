#!/bin/bash
# GhostWire Live Test - Node A Start Script

echo "Starting GhostWire on Node A..."
echo "Web UI: http://localhost:8080"
echo ""

cd "$(dirname "$0")/../../ghostwire-live-test" 2>/dev/null || cd ghostwire-live-test

# Run with mesh enabled
./ghostwire \
  --port 8080 \
  --mesh-host 0.0.0.0 \
  --mesh \
  --profile default

echo ""
echo "🛑 Node stopped"
