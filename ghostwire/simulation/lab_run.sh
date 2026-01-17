#!/bin/bash
# GhostWire Lab - Node Execution Script
# Requires lab_setup.sh to have been run

NUM_NODES=${1:-10}
LOG_DIR="simulation/logs"
BIN="./target/debug/ghostwire"

mkdir -p "$LOG_DIR"
rm -f "$LOG_DIR"/*.log

echo "🚀 Starting $NUM_NODES GhostWire nodes..."

# Ensure binary exists
if [ ! -f "$BIN" ]; then
    echo "🔨 Building GhostWire..."
    cargo build --features "cli web"
fi

# Start Node 0 (Bootstrap Node)
NS0="gw-node-0"
IP0="10.0.99.2"
PORT0=8080
echo "🌟 Starting Bootstrap Node 0 at $IP0:$PORT0..."
sudo ip netns exec "$NS0" env RUST_LOG=debug "$BIN" --api-host "$IP0" --mesh-host "$IP0" --port "$PORT0" --sim-addr "127.0.0.1:10000" web > "$LOG_DIR/node_0.log" 2>&1 &
PID0=$!

# Wait for Node 0 to initialize and generate its PeerID/Multiaddr
sleep 3
BOOTSTRAP_ADDR=$(grep "Listening on" "$LOG_DIR/node_0.log" | head -n 1 | awk '{print $NF}' | sed 's/"//g')

if [ -z "$BOOTSTRAP_ADDR" ]; then
    # Fallback/Guess if log parsing fails
    # Libp2p usually listens on /ip4/IP/tcp/4001 or similar. 
    # Let's try to extract it more reliably or set a fixed port if we can.
    # For now, let's wait a bit more.
    sleep 2
    BOOTSTRAP_ADDR=$(grep "Listening on" "$LOG_DIR/node_0.log" | tail -n 1 | sed 's/.*Listening on //;s/["'\'']//g')
fi

echo "🔗 Bootstrap Address: $BOOTSTRAP_ADDR"

# Start remaining nodes
for i in $(seq 1 $((NUM_NODES - 1))); do
    NS="gw-node-$i"
    IP="10.0.99.$((i + 2))"
    PORT=$((8080 + i))
    
    echo "📡 Starting Node $i at $IP:$PORT..."
    SIM_PORT=$((10000 + i))
    sudo ip netns exec "$NS" env RUST_LOG=debug "$BIN" --api-host "$IP" --mesh-host "$IP" --port "$PORT" --bootstrap "$BOOTSTRAP_ADDR" --sim-addr "127.0.0.1:$SIM_PORT" web > "$LOG_DIR/node_$i.log" 2>&1 &
done

echo "✅ All nodes started. Logs are in $LOG_DIR/"
echo "Monitor with: tail -f $LOG_DIR/node_0.log"
