#!/bin/bash
# GhostWire Lab - Network Setup Script
# Requires root/sudo

NUM_NODES=${1:-10}
BRIDGE="gw-br0"
BRIDGE_IP="10.0.99.1"

echo "🚀 Setting up GhostWire Lab with $NUM_NODES nodes..."

# Create bridge if it doesn't exist
if ! ip link show "$BRIDGE" > /dev/null 2>&1; then
    sudo ip link add name "$BRIDGE" type bridge
    sudo ip addr add "$BRIDGE_IP/24" dev "$BRIDGE"
    sudo ip link set dev "$BRIDGE" up
fi

for i in $(seq 0 $((NUM_NODES - 1))); do
    NS="gw-node-$i"
    VETH_NS="veth-$i-ns"
    VETH_BR="veth-$i-br"
    IP="10.0.99.$((i + 2))"

    echo "🌐 Setting up node $i ($NS) with IP $IP..."

    # Create namespace
    sudo ip netns add "$NS"

    # Create veth pair
    sudo ip link add "$VETH_NS" type veth peer name "$VETH_BR"

    # Move one end to namespace
    sudo ip link set "$VETH_NS" netns "$NS"

    # Attach other end to bridge
    sudo ip link set "$VETH_BR" master "$BRIDGE"

    # Bring interfaces up
    sudo ip link set "$VETH_BR" up
    sudo ip netns exec "$NS" ip addr add "$IP/24" dev "$VETH_NS"
    sudo ip netns exec "$NS" ip link set dev "$VETH_NS" up
    sudo ip netns exec "$NS" ip link set dev lo up
done

echo "✅ Network setup complete."
