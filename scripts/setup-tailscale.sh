#!/bin/bash
# Setup Tailscale connectivity for two-environment GhostWire testing

set -e

echo "🌐 GhostWire Tailscale Setup"
echo "============================"
echo ""

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

# Check for stratsytem
echo ""
echo "=== Looking for Stratsytem ==="
STRAT_STATUS=$(tailscale status | grep -i stratsytem || echo "")

if [ -z "$STRAT_STATUS" ]; then
    echo "⚠️  Stratsytem not found in Tailscale network"
    echo ""
    echo "To add stratsytem:"
    echo "1. Install Tailscale on stratsytem"
    echo "2. Connect to same Tailscale account"
    echo "3. Run: sudo tailscale up"
    echo ""
    echo "Alternative: Use any remote node with Tailscale"
else
    echo "✅ Stratsytem found!"
    echo "$STRAT_STATUS"
    
    STRAT_IP=$(tailscale status | grep -i stratsytem | awk '{print $3}' | cut -d'/' -f1)
    echo ""
    echo "Stratsytem Tailscale IP: $STRAT_IP"
    
    # Test connectivity
    echo ""
    echo "=== Testing Connectivity ==="
    if ping -c 3 "$STRAT_IP" > /dev/null 2>&1; then
        echo "✅ Ping successful"
    else
        echo "⚠️  Ping failed (may be firewall)"
        echo "Testing TCP port 8080..."
        if command -v nc &> /dev/null; then
            if timeout 5 nc -zv "$STRAT_IP" 8080 2>/dev/null; then
                echo "✅ Port 8080 reachable"
            else
                echo "⚠️  Port 8080 not reachable"
                echo "Ensure GhostWire is running on stratsytem"
            fi
        fi
    fi
fi

# Create helper script
echo ""
echo "=== Creating Helper Script ==="
cat > .qwen/tailscale-helper.sh << 'EOF'
#!/bin/bash
# Tailscale helper for GhostWire two-env testing

case "$1" in
    status)
        tailscale status
        ;;
    ip)
        tailscale ip -4
        ;;
    stratsytem)
        tailscale status | grep -i stratsytem
        ;;
    ping)
        STRAT_IP=$(tailscale status | grep -i stratsytem | awk '{print $3}' | cut -d'/' -f1)
        ping -c 3 "$STRAT_IP"
        ;;
    deploy)
        STRAT_IP=$(tailscale status | grep -i stratsytem | awk '{print $3}' | cut -d'/' -f1)
        echo "Deploying to $STRAT_IP..."
        scp ghostwire/target/release/ghostwire "ph@$STRAT_IP:~/ghostwire-live/"
        echo "✅ Deployed"
        ;;
    *)
        echo "Usage: $0 {status|ip|stratsytem|ping|deploy}"
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
echo "  .qwen/tailscale-helper.sh stratsytem  - Find stratsytem"
echo "  .qwen/tailscale-helper.sh deploy  - Deploy to stratsytem"
echo ""
echo "Next steps:"
echo "1. Ensure stratsytem is on same Tailscale network"
echo "2. Run: ./scripts/deploy-to-stratsytem.sh"
echo "3. Test: .qwen/tailscale-helper.sh ping"
