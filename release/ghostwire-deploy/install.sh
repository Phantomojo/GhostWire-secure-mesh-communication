#!/bin/bash
# GhostWire Deployment Script for Arch Linux
# Run this script on the target machine after extracting the package

set -e

GHOSTWIRE_VERSION="0.1.0"
INSTALL_DIR="/opt/ghostwire"
CONFIG_DIR="/etc/ghostwire"
DATA_DIR="/var/lib/ghostwire"
BIN_DIR="/usr/local/bin"

echo "========================================"
echo "  GhostWire v${GHOSTWIRE_VERSION} Installer"
echo "========================================"
echo ""

# Check if running as root
if [ "$EUID" -ne 0 ]; then 
    echo "Please run as root (sudo ./install.sh)"
    exit 1
fi

# Create directories
echo "[1/5] Creating directories..."
mkdir -p "$INSTALL_DIR"
mkdir -p "$CONFIG_DIR"
mkdir -p "$DATA_DIR"
mkdir -p "$BIN_DIR"

# Install binary
echo "[2/5] Installing binary..."
cp "$(dirname "$0")/bin/ghostwire" "$BIN_DIR/ghostwire"
chmod 755 "$BIN_DIR/ghostwire"

# Install config
echo "[3/5] Installing configuration..."
if [ -f "$(dirname "$0")/config/ghostwire.toml.example" ]; then
    cp "$(dirname "$0")/config/ghostwire.toml.example" "$CONFIG_DIR/ghostwire.toml"
elif [ -f "$(dirname "$0")/config/ghostwire.conf" ]; then
    cp "$(dirname "$0")/config/ghostwire.conf" "$CONFIG_DIR/ghostwire.conf"
fi
chmod 644 "$CONFIG_DIR"/*

# Install systemd service
echo "[4/5] Installing systemd service..."
if [ -f "$(dirname "$0")/systemd/ghostwire.service" ]; then
    cp "$(dirname "$0")/systemd/ghostwire.service" /etc/systemd/system/
    systemctl daemon-reload
    echo "    Service installed. Enable with: systemctl enable ghostwire"
    echo "    Start with: systemctl start ghostwire"
fi

# Set permissions
echo "[5/5] Setting permissions..."
chown -R root:root "$INSTALL_DIR"
chown -R root:root "$CONFIG_DIR"
chown -R root:root "$DATA_DIR"

echo ""
echo "========================================"
echo "  Installation Complete!"
echo "========================================"
echo ""
echo "Configuration: $CONFIG_DIR"
echo "Data directory: $DATA_DIR"
echo "Binary: $BIN_DIR/ghostwire"
echo ""
echo "To run GhostWire:"
echo "  ghostwire --config $CONFIG_DIR/ghostwire.toml"
echo ""
echo "Or as a service:"
echo "  systemctl enable --now ghostwire"
echo ""
