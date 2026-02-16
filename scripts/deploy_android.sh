#!/bin/bash

# GhostWire Android Deployment Script
# This script pushes the GhostWire binary to a rooted Android device and runs it

set -e

# Configuration
BINARY_PATH="ghostwire/target/aarch64-linux-android/release/ghostwire"
DEST_PATH="/data/local/tmp/ghostwire"
CONFIG_PATH="ghostwire.toml"

# Check if binary exists
if [ ! -f "$BINARY_PATH" ]; then
    echo "Error: Binary not found at $BINARY_PATH"
    echo "Please run ./scripts/build_android.sh first."
    exit 1
fi

echo "📲 Pushing GhostWire to device..."
adb push "$BINARY_PATH" "$DEST_PATH"
adb shell chmod +x "$DEST_PATH"

# Optional: Push config if it exists
if [ -f "$CONFIG_PATH" ]; then
    echo "📄 Pushing configuration..."
    adb push "$CONFIG_PATH" "/data/local/tmp/ghostwire.toml"
fi

echo "🚀 Starting GhostWire on Android..."
echo "Press Ctrl+C to stop (binary will keep running in background if started with nohup)"

# Run with mesh flags
adb shell "GHOSTWIRE_LOG=debug GHOSTWIRE_HOME=/data/local/tmp $DEST_PATH --mesh --config /data/local/tmp/ghostwire.toml"
