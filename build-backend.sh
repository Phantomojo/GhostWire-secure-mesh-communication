#!/bin/bash

# Build GhostWire backend bypassing rustup proxy issues

set -e

echo "🔧 Building GhostWire backend..."

# Temporarily unset rustup proxy
unset RUSTUP_PROXY 2>/dev/null || true

# Try to use system rust directly
export PATH="/home/phantomojo/.cargo/bin:$PATH"

# Remove the problematic rustup configuration temporarily
if [ -f ~/.rustup/settings.toml ]; then
    echo "Backing up rustup settings..."
    cp ~/.rustup/settings.toml ~/.rustup/settings.toml.backup
    # Remove the proxy configuration
    sed -i '/proxy/d' ~/.rustup/settings.toml 2>/dev/null || true
fi

# Try to build
echo "Building with cargo..."
cd ghostwire
cargo build --bin ghostwire --features web

echo "✅ Backend built successfully!"

# Restore rustup settings if we backed them up
if [ -f ~/.rustup/settings.toml.backup ]; then
    echo "Restoring rustup settings..."
    mv ~/.rustup/settings.toml.backup ~/.rustup/settings.toml
fi

cd ..
echo "🎉 Backend build complete!" 