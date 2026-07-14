#!/bin/bash
# GhostWire Background Starter

REPO_ROOT="$(dirname "$0")/.."
cd "$REPO_ROOT/ghostwire"

# Kill any existing processes
pkill -9 -f ghostwire 2>/dev/null
sleep 2

# Remove lock files
rm -rf ~/.ghostwire/vault/ghost_default/LOCK 2>/dev/null

# Start in background with nohup
nohup cargo run -- web --api-host 0.0.0.0 --mesh-host 0.0.0.0 --port 8080 > "$REPO_ROOT/ghostwire.log" 2>&1 &

# Save PID
echo $! > "$REPO_ROOT/ghostwire.pid"

# Wait for startup
sleep 8

# Check if running
if ps -p $(cat "$REPO_ROOT/ghostwire.pid") > /dev/null 2>&1; then
    echo "✅ GhostWire started successfully!"
    echo "PID: $(cat "$REPO_ROOT/ghostwire.pid")"
    echo "Logs: tail -f $REPO_ROOT/ghostwire.log"
    echo ""
    echo "API URL: http://localhost:8080"
    echo "Web UI: http://localhost:3001"
    echo ""
    echo "To stop: kill $(cat "$REPO_ROOT/ghostwire.pid")"
else
    echo "❌ Failed to start GhostWire"
    echo "Check logs: tail $REPO_ROOT/ghostwire.log"
fi
