#!/bin/bash

# GhostWire Optimized Build Script
# This script builds different versions of GhostWire with varying feature sets

set -e

echo "🔧 GhostWire Optimized Build Script"
echo "=================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to get binary size
get_binary_size() {
    local binary_path="$1"
    if [ -f "$binary_path" ]; then
        du -h "$binary_path" | cut -f1
    else
        echo "N/A"
    fi
}

# Clean previous builds
print_status "Cleaning previous builds..."
cargo clean

# Build minimal version (core functionality only)
print_status "Building minimal version..."
cargo build --release --no-default-features --features minimal
MINIMAL_SIZE=$(get_binary_size "target/release/ghostwire")
print_success "Minimal build complete: ${MINIMAL_SIZE}"

# Build security-focused version
print_status "Building security-focused version..."
cargo build --release --no-default-features --features "minimal,security"
SECURITY_SIZE=$(get_binary_size "target/release/ghostwire")
print_success "Security build complete: ${SECURITY_SIZE}"

# Build CLI version
print_status "Building CLI version..."
cargo build --release --no-default-features --features "minimal,security,cli"
CLI_SIZE=$(get_binary_size "target/release/ghostwire")
print_success "CLI build complete: ${CLI_SIZE}"

# Build web version
print_status "Building web version..."
cargo build --release --no-default-features --features "minimal,security,web"
WEB_SIZE=$(get_binary_size "target/release/ghostwire")
print_success "Web build complete: ${WEB_SIZE}"

# Build mesh version
print_status "Building mesh version..."
cargo build --release --no-default-features --features "minimal,security,cli,mesh"
MESH_SIZE=$(get_binary_size "target/release/ghostwire")
print_success "Mesh build complete: ${MESH_SIZE}"

# Build full version
print_status "Building full version..."
cargo build --release
FULL_SIZE=$(get_binary_size "target/release/ghostwire")
print_success "Full build complete: ${FULL_SIZE}"

# Create optimized builds with different names
print_status "Creating optimized builds..."

# Minimal binary
cp target/release/ghostwire target/release/ghostwire-minimal

# CLI binary
cargo build --release --no-default-features --features "minimal,security,cli"
cp target/release/ghostwire target/release/ghostwire-cli

# Web binary
cargo build --release --no-default-features --features "minimal,security,web"
cp target/release/ghostwire target/release/ghostwire-web

# Mesh binary
cargo build --release --no-default-features --features "minimal,security,cli,mesh"
cp target/release/ghostwire target/release/ghostwire-mesh

# Full binary
cargo build --release
cp target/release/ghostwire target/release/ghostwire-full

# Print summary
echo ""
echo "📊 Build Summary"
echo "================"
echo "Minimal (core only):     ${MINIMAL_SIZE}"
echo "Security-focused:        ${SECURITY_SIZE}"
echo "CLI version:             ${CLI_SIZE}"
echo "Web version:             ${WEB_SIZE}"
echo "Mesh version:            ${MESH_SIZE}"
echo "Full version:            ${FULL_SIZE}"
echo ""

# List all binaries
print_status "Available binaries:"
ls -lh target/release/ghostwire-*

print_success "Build process complete!"
print_warning "Remember to test each binary before deployment" 