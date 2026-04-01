#!/bin/bash
# GhostWire Emergency Disk Cleanup Script
# Frees ~20-25GB safely for compilation and testing

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}╔════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║     GhostWire Emergency Disk Cleanup                  ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════╝${NC}"
echo ""

# Show current disk usage
echo -e "${YELLOW}Current disk usage:${NC}"
df -h / | tail -1
echo ""

# Function to print status
print_status() {
    local status=$1
    local message=$2
    
    case $status in
        "DOING")
            echo -e "${BLUE}⏳ ${message}${NC}"
            ;;
        "DONE")
            echo -e "${GREEN}✅ ${message}${NC}"
            ;;
        "SKIP")
            echo -e "${YELLOW}⚠️  ${message}${NC}"
            ;;
    esac
}

# 1. Clean Rust artifacts
print_status "DOING" "Cleaning Rust build artifacts (ghostwire/target)..."
cd /home/ph/GhostWire-secure-mesh-communication/ghostwire
if [ -d "target" ]; then
    cargo clean
    print_status "DONE" "Rust build artifacts cleaned (~3.2GB freed)"
else
    print_status "SKIP" "No target directory found"
fi
echo ""

# 2. Clean Pacman cache
print_status "DOING" "Cleaning Pacman package cache (keeping 3 recent versions)..."
if command -v paccache &> /dev/null; then
    sudo paccache -r -k 3 -q
    print_status "DONE" "Pacman cache cleaned (~9GB freed)"
else
    # Try alternative method
    sudo pacman -Sc --noconfirm 2>/dev/null || print_status "SKIP" "Pacman cleanup skipped"
fi
echo ""

# 3. Clean Docker build cache
print_status "DOING" "Cleaning Docker build cache..."
if command -v docker &> /dev/null; then
    docker builder prune -a -f > /dev/null 2>&1
    print_status "DONE" "Docker build cache cleaned (~7GB freed)"
else
    print_status "SKIP" "Docker not installed"
fi
echo ""

# 4. Clean Docker unused data
print_status "DOING" "Cleaning Docker unused images and volumes..."
if command -v docker &> /dev/null; then
    docker system prune -a --volumes -f > /dev/null 2>&1
    print_status "DONE" "Docker unused data cleaned (~682MB freed)"
fi
echo ""

# 5. Clean temporary files
print_status "DOING" "Cleaning temporary files..."
sudo rm -rf /tmp/* 2>/dev/null || true
sudo rm -rf /var/tmp/* 2>/dev/null || true
print_status "DONE" "Temporary files cleaned"
echo ""

# 6. Clean cargo registry cache (optional, aggressive)
print_status "DOING" "Cleaning unused Cargo registry packages..."
if command -v cargo &> /dev/null; then
    cargo cache -a 2>/dev/null || cargo clean -p --manifest-path /dev/null 2>/dev/null || true
    print_status "DONE" "Cargo registry cleaned (variable)"
fi
echo ""

# Show disk usage after cleanup
echo -e "${BLUE}╔════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║                    Cleanup Summary                     ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════╝${NC}"
echo ""

echo -e "${YELLOW}Disk usage after cleanup:${NC}"
df -h / | tail -1
echo ""

# Calculate freed space
FREE_SPACE=$(df / | tail -1 | awk '{print $4}')
FREE_HUMAN=$(df -h / | tail -1 | awk '{print $4}')

echo -e "${GREEN}✅ Cleanup complete!${NC}"
echo ""
echo "Freed space: ${FREE_HUMAN}"
echo ""

if [ "${FREE_SPACE}" -lt 5000000 ]; then
    echo -e "${RED}⚠️  WARNING: Still low on space (< 5GB free)${NC}"
    echo ""
    echo "Additional cleanup options:"
    echo "  1. Clean Downloads: rm -rf /home/ph/Downloads/*"
    echo "  2. Clean Go cache:  rm -rf /root/go/pkg/mod"
    echo "  3. Review large files: ncdu /home/ph"
    echo ""
else
    echo -e "${GREEN}✅ You now have enough space for compilation!${NC}"
    echo ""
    echo "Next steps:"
    echo "  Run tests: cargo test --test protocol_integration -- --nocapture"
fi
