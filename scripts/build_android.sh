#!/bin/bash

# GhostWire Android Build Script
# This script cross-compiles GhostWire for aarch64-linux-android

set -e

# Configuration
TARGET="aarch64-linux-android"
API_LEVEL=24
NDK_VERSION="26.3.11579264"
NDK_HOME="${ANDROID_NDK_HOME:-$HOME/Android/Sdk/ndk/$NDK_VERSION}"

# Check for NDK
if [ ! -d "$NDK_HOME" ]; then
    echo "Error: NDK not found at $NDK_HOME"
    echo "Please set ANDROID_NDK_HOME or ensure NDK $NDK_VERSION is installed."
    exit 1
fi

echo "🚀 Building GhostWire for $TARGET (API $API_LEVEL)..."

# Ensure local rustup is used if available
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
fi

# Add target if missing
if ! rustup target list | grep -q "$TARGET (installed)"; then
    echo "Adding rustup target $TARGET..."
    rustup target add "$TARGET"
fi

# Install cargo-ndk if missing
if ! command -v cargo-ndk &> /dev/null; then
    echo "Installing cargo-ndk..."
    cargo install cargo-ndk
fi

# Build
echo "🔨 Running cargo ndk build..."
cd ghostwire
cargo ndk -t "$TARGET" -P "$API_LEVEL" build --release

echo "✅ Build complete! Binary located at: target/$TARGET/release/ghostwire"
