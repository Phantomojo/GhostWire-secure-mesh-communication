#!/bin/bash
# GhostWire Background Starter

cd /home/ph/GhostWire-secure-mesh-communication/ghostwire

# Kill any existing processes
pkill -9 -f ghostwire 2>/dev/null
sleep 2

# Remove lock files
rm -rf /home/ph/.ghostwire/vault/ghost_default/LOCK 2>/dev/null

# Start in background with nohup
nohup cargo run -- web --api-host 0.0.0.0 --mesh-host 0.0.0.0 --port 8080 > /home/ph/GhostWire-secure-mesh-communication/ghostwire.log 2>&1 &

# Save PID
echo $! > /home/ph/GhostWire-secure-mesh-communication/ghostwire.pid

# Wait for startup
sleep 8

# Check if running
if ps -p $(cat /home/ph/GhostWire-secure-mesh-communication/ghostwire.pid) > /dev/null 2>&1; then
    echo "✅ GhostWire started successfully!"
    echo "PID: $(cat /home/ph/GhostWire-secure-mesh-communication/ghostwire.pid)"
    echo "Logs: tail -f /home/ph/GhostWire-secure-mesh-communication/ghostwire.log"
    echo ""
    echo "API URL: http://localhost:8080"
    echo "Web UI: http://localhost:3001"
    echo ""
    echo "To stop: kill $(cat /home/ph/GhostWire-secure-mesh-communication/ghostwire.pid)"
else
    echo "❌ Failed to start GhostWire"
    echo "Check logs: tail /home/ph/GhostWire-secure-mesh-communication/ghostwire.log"
fi
