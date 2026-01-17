#!/bin/bash
# GhostWire Lab - Multi-log tail

LOG_DIR="simulation/logs"

if [ ! -d "$LOG_DIR" ]; then
    echo "❌ Log directory not found."
    exit 1
fi

echo "📖 Tailing all node logs. Press Ctrl+C to stop."
tail -f "$LOG_DIR"/*.log | awk '
    /==> .* <==/ { 
        split($2, a, "/"); 
        node=a[length(a)]; 
        gsub(".log", "", node); 
        color=(31 + (i++ % 6)); # cycle colors
        next 
    } 
    { print "\033[0;" color "m[" node "]\033[0m " $0 }
'
