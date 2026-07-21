#!/bin/bash
# Setup Tailscale connectivity for GhostWire testing
# Configure remote node via environment variables:
#   REMOTE_NODE_NAME - hostname of remote node in Tailscale (default: prompt)
#   REMOTE_NODE_USER - SSH username for remote node (default: current user)

set -e

echo "🌐 GhostWire Tailscale Setup"
echo "============================"
echo ""

# Remote node configuration (from environment)
REMOTE_NODE_NAME="${REMOTE_NODE_NAME:-}"
REMOTE_NODE_USER="${REMOTE_NODE_USER:-$USER}"

# Check if Tailscale is installed
if ! command -v tailscale &> /dev/null; then
    echo "❌ Tailscale not installed!"
    echo ""
    echo "Install Tailscale:"
    echo "  Ubuntu/Debian:"
    echo "    curl -fsSL https://tailscale.com/install.sh | sh"
    echo ""
    echo "  Fedora/RHEL:"
    echo "    dnf install tailscale"
    echo ""
    echo "  Arch:"
    echo "    pacman -S tailscale"
    echo ""
    echo "  macOS:"
    echo "    brew install --cask tailscale"
    exit 1
fi

# Check Tailscale status
echo "=== Tailscale Status ==="
tailscale status

# Check if connected
if ! tailscale status | grep -q "online"; then
    echo ""
    echo "❌ Tailscale not connected!"
    echo ""
    echo "Connect with:"
    echo "  sudo tailscale up"
    echo ""
    echo "Or authenticate with:"
    echo "  sudo tailscale up --login-server https://login.tailscale.com"
    exit 1
fi

echo ""
echo "✅ Tailscale is running and connected"

# Get this node's IP
THIS_IP=$(tailscale ip -4 2>/dev/null || echo "unknown")
THIS_HOSTNAME=$(hostname)

echo ""
echo "=== This Node ==="
echo "Hostname: $THIS_HOSTNAME"
echo "Tailscale IP: $THIS_IP"

# Prompt for remote node name if not set
if [ -z "$REMOTE_NODE_NAME" ]; then
    echo ""
    echo "=== Remote Node Configuration ==="
    echo "Enter the hostname of your remote node (as shown in 'tailscale status'):"
    echo -n "Remote node hostname: "
    read -r REMOTE_NODE_NAME
fi

if [ -z "$REMOTE_NODE_NAME" ]; then
    echo ""
    echo "⚠️  No remote node specified. Skipping remote node tests."
else
    # Check for remote node
    echo ""
    echo "=== Looking for Remote Node: $REMOTE_NODE_NAME ==="
    REMOTE_STATUS=$(tailscale status | grep -i "$REMOTE_NODE_NAME" || echo "")
    
    if [ -z "$REMOTE_STATUS" ]; then
        echo "⚠️  $REMOTE_NODE_NAME not found in Tailscale network"
        echo ""
        echo "To add $REMOTE_NODE_NAME:"
        echo "1. Install Tailscale on $REMOTE_NODE_NAME"
        echo "2. Connect to same Tailscale account"
        echo "3. Run: sudo tailscale up"
    else
        echo "✅ $REMOTE_NODE_NAME found!"
        echo "$REMOTE_STATUS"
        
        REMOTE_IP=$(tailscale status | grep -i "$REMOTE_NODE_NAME" | awk '{print $3}' | cut -d'/' -f1)
        echo ""
        echo "$REMOTE_NODE_NAME Tailscale IP: $REMOTE_IP"
        
        # Test connectivity
        echo ""
        echo "=== Testing Connectivity ==="
        if ping -c 3 "$REMOTE_IP" > /dev/null 2>&1; then
            echo "✅ Ping successful"
        else
            echo "⚠️  Ping failed (may be firewall)"
            echo "Testing TCP port 8080..."
            if command -v nc &> /dev/null; then
                if timeout 5 nc -zv "$REMOTE_IP" 8080 2>/dev/null; then
                    echo "✅ Port 8080 reachable"
                else
                    echo "⚠️  Port 8080 not reachable"
                    echo "Ensure GhostWire is running on $REMOTE_NODE_NAME"
                fi
            fi
        fi
    fi
fi

# Create helper script
echo ""
echo "=== Creating Helper Script ==="
cat > .qwen/tailscale-helper.sh << 'EOF'
#!/bin/bash
# Tailscale helper for GhostWire testing
# Configure via environment variables:
#   REMOTE_NODE_NAME - hostname of remote node in Tailscale

REMOTE_NODE_NAME="${REMOTE_NODE_NAME:-}"
REMOTE_NODE_USER="${REMOTE_NODE_USER:-$USER}"

case "$1" in
    status)
        tailscale status
        ;;
    ip)
        tailscale ip -4
        ;;
    list)
        echo "Available nodes on Tailscale network:"
        tailscale status | grep -E "^[a-zA-Z0-9]" | head -20
        ;;
    ping)
        if [ -z "$REMOTE_NODE_NAME" ]; then
            echo "Set REMOTE_NODE_NAME environment variable"
            exit 1
        fi
        REMOTE_IP=$(tailscale status | grep -i "$REMOTE_NODE_NAME" | awk '{print $3}' | cut -d'/' -f1)
        if [ -z "$REMOTE_IP" ]; then
            echo "Node $REMOTE_NODE_NAME not found"
            exit 1
        fi
        ping -c 3 "$REMOTE_IP"
        ;;
    deploy)
        if [ -z "$REMOTE_NODE_NAME" ]; then
            echo "Set REMOTE_NODE_NAME environment variable"
            exit 1
        fi
        REMOTE_IP=$(tailscale status | grep -i "$REMOTE_NODE_NAME" | awk '{print $3}' | cut -d'/' -f1)
        if [ -z "$REMOTE_IP" ]; then
            echo "Node $REMOTE_NODE_NAME not found"
            exit 1
        fi
        echo "Deploying to $REMOTE_IP..."
        # Use SSH config for authentication
        scp ghostwire/target/release/ghostwire "${REMOTE_NODE_USER}@${REMOTE_IP}:~/ghostwire-live/"
        echo "✅ Deployed"
        ;;
    *)
        echo "Usage: $0 {status|ip|list|ping|deploy}"
        echo ""
        echo "Configure via environment:"
        echo "  export REMOTE_NODE_NAME=<node-hostname>"
        echo "  export REMOTE_NODE_USER=<ssh-username>"
        exit 1
        ;;
esac
EOF

chmod +x .qwen/tailscale-helper.sh
echo "✅ Helper script created: .qwen/tailscale-helper.sh"

echo ""
echo "=========================================="
echo "✅ Tailscale Setup Complete!"
echo "=========================================="
echo ""
echo "Quick commands:"
echo "  ./scripts/setup-tailscale.sh  - Rerun this script"
echo "  .qwen/tailscale-helper.sh status  - Check status"
echo "  .qwen/tailscale-helper.sh list    - List available nodes"
echo "  .qwen/tailscale-helper.sh ping    - Ping remote node"
echo "  .qwen/tailscale-helper.sh deploy  - Deploy to remote node"
echo ""
echo "To configure a remote node:"
echo "  export REMOTE_NODE_NAME=<node-hostname>"
echo "  export REMOTE_NODE_USER=<ssh-username>"
