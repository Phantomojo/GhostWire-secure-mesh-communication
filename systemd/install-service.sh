#!/bin/bash

# GhostWire Systemd Service Installer
# This script installs GhostWire as a system service

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${GREEN}🚀 GhostWire Systemd Service Installer${NC}"
echo ""

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo -e "${RED}❌ This script must be run as root (use sudo)${NC}"
    exit 1
fi

# Check if we're in the right directory
if [ ! -f "systemd/ghostwire.service" ]; then
    echo -e "${RED}❌ Please run this script from the GhostWire root directory${NC}"
    exit 1
fi

# Create installation directory
echo -e "${BLUE}📁 Creating installation directory...${NC}"
mkdir -p /opt/ghostwire
cp -r . /opt/ghostwire/
chown -R root:root /opt/ghostwire
chmod -R 755 /opt/ghostwire

# Install systemd service
echo -e "${BLUE}🔧 Installing systemd service...${NC}"
cp systemd/ghostwire.service /etc/systemd/system/
systemctl daemon-reload

# Enable and start service
echo -e "${BLUE}🚀 Enabling and starting service...${NC}"
systemctl enable ghostwire.service
systemctl start ghostwire.service

# Check service status
echo -e "${BLUE}🏥 Checking service status...${NC}"
sleep 5
systemctl status ghostwire.service --no-pager

echo ""
echo -e "${GREEN}🎉 GhostWire service installed successfully!${NC}"
echo ""
echo -e "${BLUE}📋 Service Management Commands:${NC}"
echo -e "  • Start: sudo systemctl start ghostwire"
echo -e "  • Stop: sudo systemctl stop ghostwire"
echo -e "  • Restart: sudo systemctl restart ghostwire"
echo -e "  • Status: sudo systemctl status ghostwire"
echo -e "  • Logs: sudo journalctl -u ghostwire -f"
echo ""
echo -e "${BLUE}🔧 Service Configuration:${NC}"
echo -e "  • Service file: /etc/systemd/system/ghostwire.service"
echo -e "  • Installation: /opt/ghostwire"
echo -e "  • Data directory: /opt/ghostwire/data"
echo -e "  • Config directory: /opt/ghostwire/config"
echo ""
echo -e "${GREEN}✅ GhostWire is now running as a system service!${NC}" 