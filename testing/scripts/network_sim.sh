#!/bin/bash
# GhostWire Network Simulation Script
# Uses tc/netem to simulate various network conditions
#
# Usage:
#   ./network_sim.sh apply <node> <condition>
#   ./network_sim.sh clear <node>
#   ./network_sim.sh status <node>
#
# Conditions:
#   latency-50     - 50ms latency ± 10ms jitter
#   latency-100    - 100ms latency ± 20ms jitter
#   latency-200    - 200ms latency ± 50ms jitter
#   loss-1         - 1% packet loss
#   loss-5         - 5% packet loss
#   loss-10        - 10% packet loss
#   loss-20        - 20% packet loss
#   bw-1mbps       - 1 Mbps bandwidth limit
#   bw-5mbps       - 5 Mbps bandwidth limit
#   bw-10mbps      - 10 Mbps bandwidth limit
#   corrupt-1      - 1% packet corruption
#   duplicate-5    - 5% packet duplication
#   reorder-10     - 10% packet reordering
#   worst-case     - 200ms + 10% loss + 1Mbps (terrible conditions)

SSH_OPTS="-o ConnectTimeout=10 -o StrictHostKeyChecking=no"

STRAT_HOST="strat"
OMEN_HOST="omen"

usage() {
    echo "Usage: $0 {apply|clear|status} <strat|omen|both> <condition>"
    echo ""
    echo "Conditions:"
    echo "  latency-50/100/200  - Add latency (ms)"
    echo "  loss-1/5/10/20      - Packet loss (%)"
    echo "  bw-1/5/10mbps       - Bandwidth limit"
    echo "  corrupt-1           - Packet corruption"
    echo "  duplicate-5         - Packet duplication"
    echo "  reorder-10          - Packet reordering"
    echo "  worst-case          - All bad things at once"
    exit 1
}

apply_condition() {
    local host=$1
    local condition=$2

    echo "Applying '$condition' to $host..."

    case $condition in
        latency-50)
            ssh $SSH_OPTS $host "tc qdisc add dev eth0 root netem delay 50ms 10ms distribution normal 2>/dev/null || tc qdisc change dev eth0 root netem delay 50ms 10ms"
            ;;
        latency-100)
            ssh $SSH_OPTS $host "tc qdisc add dev eth0 root netem delay 100ms 20ms distribution normal 2>/dev/null || tc qdisc change dev eth0 root netem delay 100ms 20ms"
            ;;
        latency-200)
            ssh $SSH_OPTS $host "tc qdisc add dev eth0 root netem delay 200ms 50ms distribution normal 2>/dev/null || tc qdisc change dev eth0 root netem delay 200ms 50ms"
            ;;
        loss-1)
            ssh $SSH_OPTS $host "tc qdisc add dev eth0 root netem loss 1% 2>/dev/null || tc qdisc change dev eth0 root netem loss 1%"
            ;;
        loss-5)
            ssh $SSH_OPTS $host "tc qdisc add dev eth0 root netem loss 5% 2>/dev/null || tc qdisc change dev eth0 root netem loss 5%"
            ;;
        loss-10)
            ssh $SSH_OPTS $host "tc qdisc add dev eth0 root netem loss 10% 2>/dev/null || tc qdisc change dev eth0 root netem loss 10%"
            ;;
        loss-20)
            ssh $SSH_OPTS $host "tc qdisc add dev eth0 root netem loss 20% 2>/dev/null || tc qdisc change dev eth0 root netem loss 20%"
            ;;
        bw-1mbps)
            ssh $SSH_OPTS $host "tc qdisc add dev eth0 root tbf rate 1mbit burst 32kbit latency 400ms 2>/dev/null || tc qdisc change dev eth0 root tbf rate 1mbit burst 32kbit latency 400ms"
            ;;
        bw-5mbps)
            ssh $SSH_OPTS $host "tc qdisc add dev eth0 root tbf rate 5mbit burst 32kbit latency 400ms 2>/dev/null || tc qdisc change dev eth0 root tbf rate 5mbit burst 32kbit latency 400ms"
            ;;
        bw-10mbps)
            ssh $SSH_OPTS $host "tc qdisc add dev eth0 root tbf rate 10mbit burst 32kbit latency 400ms 2>/dev/null || tc qdisc change dev eth0 root tbf rate 10mbit burst 32kbit latency 400ms"
            ;;
        corrupt-1)
            ssh $SSH_OPTS $host "tc qdisc add dev eth0 root netem corrupt 1% 2>/dev/null || tc qdisc change dev eth0 root netem corrupt 1%"
            ;;
        duplicate-5)
            ssh $SSH_OPTS $host "tc qdisc add dev eth0 root netem duplicate 5% 2>/dev/null || tc qdisc change dev eth0 root netem duplicate 5%"
            ;;
        reorder-10)
            ssh $SSH_OPTS $host "tc qdisc add dev eth0 root netem reorder 10% 25% 2>/dev/null || tc qdisc change dev eth0 root netem reorder 10% 25%"
            ;;
        worst-case)
            ssh $SSH_OPTS $host "tc qdisc add dev eth0 root netem delay 200ms 50ms loss 10% corrupt 1% duplicate 5% 2>/dev/null || tc qdisc change dev eth0 root netem delay 200ms 50ms loss 10% corrupt 1% duplicate 5%"
            ;;
        *)
            echo "Unknown condition: $condition"
            usage
            ;;
    esac
}

clear_rules() {
    local host=$1
    echo "Clearing tc rules on $host..."
    ssh $SSH_OPTS $host "tc qdisc del dev eth0 root 2>/dev/null || true"
}

show_status() {
    local host=$1
    echo -e "\n=== $host ==="
    ssh $SSH_OPTS $host "tc qdisc show dev eth0 2>/dev/null || echo 'No tc rules'"
}

# Main
ACTION=${1:-}
TARGET=${2:-}
CONDITION=${3:-}

case $ACTION in
    apply)
        if [ -z "$CONDITION" ]; then
            echo "Error: condition required"
            usage
        fi
        if [ "$TARGET" = "strat" ] || [ "$TARGET" = "both" ]; then
            apply_condition $STRAT_HOST $CONDITION
        fi
        if [ "$TARGET" = "omen" ] || [ "$TARGET" = "both" ]; then
            apply_condition $OMEN_HOST $CONDITION
        fi
        ;;
    clear)
        if [ "$TARGET" = "strat" ] || [ "$TARGET" = "both" ]; then
            clear_rules $STRAT_HOST
        fi
        if [ "$TARGET" = "omen" ] || [ "$TARGET" = "both" ]; then
            clear_rules $OMEN_HOST
        fi
        ;;
    status)
        if [ "$TARGET" = "strat" ] || [ "$TARGET" = "both" ]; then
            show_status $STRAT_HOST
        fi
        if [ "$TARGET" = "omen" ] || [ "$TARGET" = "both" ]; then
            show_status $OMEN_HOST
        fi
        ;;
    *)
        usage
        ;;
esac
