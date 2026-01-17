#!/bin/bash
# GhostWire Lab CLI Wrapper
# Usage: ./simulation/lab_cli.sh <node_id> <command> [args...]
# Example: ./simulation/lab_cli.sh 1 status
# Example: ./simulation/lab_cli.sh 1 send <recipient_id> "Hello"

NODE_ID=$1
shift
COMMAND=$1
shift

if [ -z "$NODE_ID" ] || [ -z "$COMMAND" ]; then
    echo "Usage: $0 <node_id> <command> [args...]"
    exit 1
fi

NS_NAME="gw-node-$NODE_ID"
BIN="./target/debug/ghostwire"

if [ ! -f "$BIN" ]; then
    echo "Error: ghostwire binary not found. Build it first."
    exit 1
fi

sudo ip netns exec "$NS_NAME" "$BIN" "$COMMAND" "$@"
