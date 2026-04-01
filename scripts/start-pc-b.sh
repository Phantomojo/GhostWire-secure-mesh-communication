#!/bin/bash
# 🚀 GhostWire Live Test - PC-B (stratsytem) Start Script

echo "🔶 Starting GhostWire on PC-B (stratsytem)..."
echo "📡 Tailscale IP: 100.105.58.61"
echo "🌐 Web UI: http://localhost:8080"
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
