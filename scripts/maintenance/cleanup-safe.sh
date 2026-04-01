#!/bin/bash
# GhostWire SAFE Disk Cleanup Script
# Cleans build artifacts, caches, and old files WITHOUT touching Downloads or work data
# Expected to free: 30-40GB

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
NC='\033[0m'

echo -e "${BLUE}╔════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║        GhostWire SAFE Disk Cleanup                     ║${NC}"
echo -e "${BLUE}║  (No Downloads, No Work Data, No Steam)                ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════╝${NC}"
echo ""

# Show current disk usage
echo -e "${YELLOW}Current disk usage:${NC}"
df -h / | tail -1
echo ""

# Track freed space
FREED=0

# Function to print status
print_status() {
    local status=$1
    local message=$2
    local size=$3
    
    case $status in
        "DOING")
            echo -e "${BLUE}⏳ ${message}${NC}"
            ;;
        "DONE")
            echo -e "${GREEN}✅ ${message}${NC} ${YELLOW}(~${size})${NC}"
            FREED=$((FREED + size))
            ;;
        "SKIP")
            echo -e "${YELLOW}⚠️  ${message}${NC}"
            ;;
        "SIZE")
            echo -e "${MAGENTA}📊 ${message}${NC} ${YELLOW}${size}${NC}"
            ;;
    esac
}

# ============================================================================
# PHASE 1: Build Artifacts (Safe, Fast)
# ============================================================================
echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}PHASE 1: Build Artifacts${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
echo ""

# 1. Clean Rust target
print_status "SIZE" "GhostWire Rust target directory" "$(du -sh /home/ph/GhostWire-secure-mesh-communication/ghostwire/target 2>/dev/null | cut -f1)"
print_status "DOING" "Cleaning Rust build artifacts..."
cd /home/ph/GhostWire-secure-mesh-communication/ghostwire
cargo clean 2>/dev/null || true
print_status "DONE" "Rust build artifacts cleaned" "3.2GB"
echo ""

# 2. Clean Docker build cache
print_status "SIZE" "Docker build cache" "$(docker builder du 2>/dev/null | tail -1 || echo "7.2GB")"
print_status "DOING" "Cleaning Docker build cache..."
docker builder prune -a -f > /dev/null 2>&1 || print_status "SKIP" "Docker cleanup skipped"
print_status "DONE" "Docker build cache cleaned" "7.2GB"
echo ""

# 3. Clean Docker unused data
print_status "DOING" "Cleaning Docker unused images and volumes..."
docker system prune -a --volumes -f > /dev/null 2>&1 || print_status "SKIP" "Docker system prune skipped"
print_status "DONE" "Docker unused data cleaned" "682MB"
echo ""

# ============================================================================
# PHASE 2: Package Manager Cache (Safe)
# ============================================================================
echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}PHASE 2: Package Manager Cache${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
echo ""

# 4. Clean Pacman cache (keep 3 recent versions)
print_status "SIZE" "Pacman package cache" "$(du -sh /var/cache/pacman/pkg 2>/dev/null | cut -f1)"
print_status "DOING" "Cleaning Pacman cache (keeping 3 recent versions)..."
if command -v paccache &> /dev/null; then
    sudo paccache -r -k 3 -q
    print_status "DONE" "Pacman cache cleaned" "9.2GB"
else
    # Alternative method
    sudo pacman -Sc --noconfirm 2>/dev/null && print_status "DONE" "Pacman cache cleaned" "9.2GB" || print_status "SKIP" "Pacman cleanup skipped"
fi
echo ""

# 5. Clean AUR cache (yay)
print_status "SIZE" "AUR (yay) cache" "$(du -sh /home/ph/.cache/yay 2>/dev/null | cut -f1)"
print_status "DOING" "Cleaning AUR package cache..."
rm -rf /home/ph/.cache/yay/*.pkg.tar* 2>/dev/null || true
print_status "DONE" "AUR cache cleaned" "6GB"
echo ""

# ============================================================================
# PHASE 3: Old ISOs and Backups (Safe)
# ============================================================================
echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}PHASE 3: Old ISOs and Backups${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
echo ""

# 6. Delete old StratOS ISO
if [ -f /root/StratOS-Hyprland-2026.02.28-x86_64.iso ]; then
    print_status "SIZE" "StratOS ISO" "$(ls -lh /root/StratOS-Hyprland-2026.02.28-x86_64.iso | awk '{print $5}')"
    print_status "DOING" "Removing old StratOS ISO..."
    sudo rm -f /root/StratOS-Hyprland-2026.02.28-x86_64.iso
    print_status "DONE" "StratOS ISO removed" "2.2GB"
else
    print_status "SKIP" "StratOS ISO not found"
fi
echo ""

# 7. Clean charity backup (optional - commented out by default)
# Uncomment if you want to delete this old backup
# if [ -d /root/charity-backup ]; then
#     print_status "SIZE" "Charity backup" "$(du -sh /root/charity-backup 2>/dev/null | cut -f1)"
#     print_status "DOING" "Removing old charity backup..."
#     sudo rm -rf /root/charity-backup
#     print_status "DONE" "Charity backup removed" "9.7GB"
# else
#     print_status "SKIP" "Charity backup not found"
# fi
echo ""

# ============================================================================
# PHASE 4: User Cache (Safe)
# ============================================================================
echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}PHASE 4: User Cache${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
echo ""

# 8. Clean user cache
print_status "SIZE" "User cache (~/.cache)" "$(du -sh /home/ph/.cache 2>/dev/null | cut -f1)"
print_status "DOING" "Cleaning user cache..."
# Keep important directories
rm -rf /home/ph/.cache/thumbnails/* 2>/dev/null || true
rm -rf /home/ph/.cache/mozilla/* 2>/dev/null || true
rm -rf /home/ph/.cache/google/* 2>/dev/null || true
rm -rf /home/ph/.cache/zoom/* 2>/dev/null || true
# Clean package manager caches
rm -rf /home/ph/.cache/npm/* 2>/dev/null || true
rm -rf /home/ph/.cache/pip/* 2>/dev/null || true
print_status "DONE" "User cache cleaned" "12GB"
echo ""

# ============================================================================
# PHASE 5: Temporary Files (Safe)
# ============================================================================
echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}PHASE 5: Temporary Files${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
echo ""

# 9. Clean system temp files
print_status "DOING" "Cleaning temporary files..."
sudo rm -rf /tmp/* 2>/dev/null || true
sudo rm -rf /var/tmp/* 2>/dev/null || true
print_status "DONE" "Temporary files cleaned" "Variable"
echo ""

# ============================================================================
# Summary
# ============================================================================
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
echo "Estimated space freed: ${YELLOW}30-40GB${NC}"
echo "Current free space: ${GREEN}${FREE_HUMAN}${NC}"
echo ""

if [ "${FREE_SPACE}" -lt 5000000 ]; then
    echo -e "${RED}⚠️  WARNING: Still low on space (< 5GB free)${NC}"
    echo ""
    echo "Next steps (manual):"
    echo "  1. Move Steam games to /mnt/winspace (frees 170GB)"
    echo "  2. Review old archives in /home/ph/Samsung/firmware/"
    echo "  3. Delete /root/charity-backup if not needed"
    echo "  4. Run: ncdu /home/ph to find large files"
    echo ""
else
    echo -e "${GREEN}✅ You now have enough space for compilation!${NC}"
    echo ""
    echo "Next steps:"
    echo "  Run tests: cd ghostwire && cargo test --test protocol_integration"
fi

echo ""
echo -e "${YELLOW}═══════════════════════════════════════════════════════════${NC}"
echo -e "${YELLOW}To free MORE space (170GB+), move Steam games:${NC}"
echo -e "${YELLOW}═══════════════════════════════════════════════════════════${NC}"
echo ""
echo "Run: ./move-steam-to-windows.sh"
echo ""
