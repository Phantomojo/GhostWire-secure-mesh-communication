#!/bin/bash
# 🚀 GhostWire Live Test - PC-A (omen) Start Script

echo "🔷 Starting GhostWire on PC-A (omen)..."
echo "📡 Tailscale IP: 100.88.234.14"
echo "🌐 Web UI: http://localhost:8080"
echo ""

cd /home/ph/ghostwire-live-test

# Run with mesh enabled
./ghostwire \
  --port 8080 \
  --mesh-host 0.0.0.0 \
  --mesh \
  --profile default

echo ""
echo "🛑 Node stopped"
