#!/bin/bash
# GhostWire Lab - Stress/Traffic Shaping Script
# Requires root/sudo

LATENCY=${1:-"100ms"}
LOSS=${2:-"1%"}

echo "🩹 Applying network stress: Latency=$LATENCY, Loss=$LOSS..."

for i in $(seq 0 50); do
    VETH="veth-$i-br"
    if ip link show "$VETH" > /dev/null 2>&1; then
        echo "Applying to $VETH..."
        sudo tc qdisc add dev "$VETH" root netem delay "$LATENCY" loss "$LOSS" 2>/dev/null || \
        sudo tc qdisc change dev "$VETH" root netem delay "$LATENCY" loss "$LOSS"
    fi
done

echo "✅ Stress applied."

echo "To reset: sudo tc qdisc del dev veth-X-br root"
