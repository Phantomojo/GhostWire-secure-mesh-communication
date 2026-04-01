#!/bin/bash
# Deploy GhostWire updates to Android device
# This updates the frontend files that the existing Android app uses

set -e

echo "=========================================="
echo "  GhostWire Android Deployment"
echo "=========================================="
echo ""

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Check device
echo -n "📱 Checking Android device... "
if ! adb devices | grep -q "device$"; then
    echo -e "${RED}No device found${NC}"
    exit 1
fi
echo -e "${GREEN}Connected${NC}"

# Get device architecture
ARCH=$(adb shell getprop ro.product.cpu.abi | tr -d '\r')
echo "📐 Device Architecture: $ARCH"
echo ""

# Build frontend
echo -e "${YELLOW}🔨 Building updated frontend...${NC}"
cd /home/ph/GhostWire-secure-mesh-communication/webui
npm run build > /dev/null 2>&1
echo -e "${GREEN}✅ Frontend built${NC}"
echo ""

# Deploy to Android
echo -e "${YELLOW}📤 Deploying to Android device...${NC}"

# Option 1: Push to device storage for existing app
echo "   Pushing frontend files to device..."
adb shell "mkdir -p /sdcard/ghostwire_webui" > /dev/null 2>&1
adb push dist/ /sdcard/ghostwire_webui/ > /dev/null 2>&1
echo -e "${GREEN}✅ Frontend deployed to /sdcard/ghostwire_webui${NC}"
echo ""

# Option 2: Start backend on device (if ARM binary available)
if [ -f "/home/ph/GhostWire-secure-mesh-communication/ghostwire/target/aarch64-linux-android/release/ghostwire" ]; then
    echo "   Pushing ARM backend binary..."
    adb push /home/ph/GhostWire-secure-mesh-communication/ghostwire/target/aarch64-linux-android/release/ghostwire /data/local/tmp/ghostwire
    adb shell "chmod 755 /data/local/tmp/ghostwire"
    echo -e "${GREEN}✅ ARM Backend deployed${NC}"
    echo ""
    echo "   To start backend on device:"
    echo "   adb shell /data/local/tmp/ghostwire"
else
    echo -e "${YELLOW}⚠️  ARM backend not available (x86_64 only)${NC}"
    echo "   To build ARM backend, you need Android NDK installed"
    echo ""
fi

echo ""
echo "=========================================="
echo -e "${GREEN}✅ Deployment Complete!${NC}"
echo "=========================================="
echo ""
echo "📱 On your Android device:"
echo "   1. Open the GhostWire app"
echo "   2. Or access via browser: http://<device-ip>:8080"
echo ""
echo "🔧 To start backend manually (if ARM binary deployed):"
echo "   adb shell /data/local/tmp/ghostwire"
echo ""
echo "📊 To view logs:"
echo "   adb logcat | grep -i ghostwire"
echo ""
