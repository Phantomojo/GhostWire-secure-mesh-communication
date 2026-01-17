#!/bin/bash
# GhostWire Lab - Cleanup Script
# Requires root/sudo

BRIDGE="gw-br0"

echo "🧹 Cleaning up GhostWire Lab..."

# Delete namespaces
for ns in $(ip netns list | awk '{print $1}' | grep "^gw-node-"); do
    echo "Deletting namespace $ns..."
    sudo ip netns delete "$ns"
done

# Delete bridge
if ip link show "$BRIDGE" > /dev/null 2>&1; then
    echo "Deleting bridge $BRIDGE..."
    sudo ip link delete "$BRIDGE" type bridge
fi

echo "✅ Cleanup complete."
