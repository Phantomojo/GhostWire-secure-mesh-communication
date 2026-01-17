#!/bin/bash
set -e

# GhostWire Hostname Setup Script
# Maps ghostwire.local -> 127.0.0.1

HOST_ENTRY="127.0.0.1 ghostwire.local"
HOSTS_FILE="/etc/hosts"

echo "👻 GhostWire Hostname Setup"
echo "---------------------------"

if grep -q "ghostwire.local" "$HOSTS_FILE"; then
    echo "✅ 'ghostwire.local' is already configured!"
    echo "   You can access your node at: http://ghostwire.local:5050"
    exit 0
fi

echo "This script will add the following line to $HOSTS_FILE:"
echo "  $HOST_ENTRY"
echo ""
echo "This allows you to visit 'http://ghostwire.local:5050' instead of using an IP address."
echo "⚠️  This requires sudo privileges."
echo ""

read -p "Proceed? [y/N] " response
if [[ "$response" =~ ^[yY]$ ]]; then
    if sudo sh -c "echo '$HOST_ENTRY' >> '$HOSTS_FILE'"; then
        echo ""
        echo "✅ Success! Hostname configured."
        echo "   Access your node at: http://ghostwire.local:5050"
    else
        echo "❌ Failed to update hosts file."
        exit 1
    fi
else
    echo "Setup cancelled."
    exit 0
fi
