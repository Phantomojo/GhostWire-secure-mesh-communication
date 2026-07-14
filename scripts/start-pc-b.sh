#!/bin/bash
# GhostWire Live Test - Node B Start Script

echo "Starting GhostWire on Node B..."
echo "Web UI: http://localhost:8080"
echo ""

cd ~/ghostwire-live-test

# Run with mesh enabled
./ghostwire \
  --port 8080 \
  --mesh-host 0.0.0.0 \
  --mesh \
  --profile default

echo ""
echo "🛑 Node stopped"
