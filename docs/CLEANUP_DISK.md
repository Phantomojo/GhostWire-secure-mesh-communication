# 🗑️ GhostWire Disk Space Cleanup Guide

**Date:** March 30, 2026  
**Problem:** Disk 100% full (944GB used of 952GB)  
**Critical:** Only 477MB free - Rust compilation failing!

---

## 📊 Disk Usage Analysis

### Current Situation
```
Filesystem      Size  Used   Avail  Use%
/dev/mapper/root 952G  944G   477M   100%  ❌ CRITICAL
```

### Largest Space Consumers

| Location | Size | Can Clean? | Priority |
|----------|------|------------|----------|
| `/home/ph/Cybesecurity` | 97GB | ⚠️ User data | Low |
| `/home/ph/Samsung` | 83GB | ⚠️ User data | Low |
| `/mnt/winspace` | 248GB | ⚠️ External mount | N/A |
| `/home/ph/data` | 28GB | ⚠️ User data | Low |
| `/home/ph/Downloads` | 14GB | ✅ **YES** | **HIGH** |
| **`/home/ph/GhostWire/ghostwire/target`** | **3.2GB** | ✅ **YES** | **HIGH** |
| `/home/ph/GhostWire/src-tauri` | 5.9GB | ⚠️ Keep src | Medium |
| `/root/go` | 1.5GB | ✅ **YES** | Medium |
| **`/var/cache/pacman`** | **9.2GB** | ✅ **YES** | **HIGH** |
| **Docker Build Cache** | **7.2GB** | ✅ **YES** | **HIGH** |

---

## 🚀 Quick Cleanup (Safe - Frees ~20GB+)

### 1. Clean Rust Build Artifacts (3.2GB)
```bash
# Clean GhostWire target directory
cd /home/ph/GhostWire-secure-mesh-communication/ghostwire
cargo clean

# Expected savings: ~3.2GB
```

### 2. Clean Pacman Cache (9.2GB)
```bash
# Remove old package versions (keep current)
sudo paccache -r

# OR remove all cached packages
sudo pacman -Scc

# Expected savings: ~9GB
```

### 3. Clean Docker Build Cache (7.2GB)
```bash
# Remove all build cache
docker builder prune -a -f

# Expected savings: ~7.2GB
```

### 4. Clean Docker Unused Data (466MB)
```bash
# Remove unused images, volumes, networks
docker system prune -a --volumes -f

# Expected savings: ~682MB
```

### 5. Clean Downloads (Estimate 5-10GB)
```bash
# Review downloads folder
ls -lh /home/ph/Downloads

# Delete old downloads
rm -rf /home/ph/Downloads/*

# Expected savings: 5-10GB
```

---

## ⚠️ Medium Cleanup (Use Caution)

### 6. Clean Go Cache (1.5GB)
```bash
# Only if you don't need Go module cache
rm -rf /root/go/pkg/mod

# Expected savings: ~1.5GB
```

### 7. Clean Old ISOs (2.2GB)
```bash
# Check if you need this ISO
ls -lh /root/StratOS-Hyprland-*.iso

# Delete if not needed
rm /root/StratOS-Hyprland-2026.02.28-x86_64.iso

# Expected savings: 2.2GB
```

### 8. Clean Charity Backup (9.7GB)
```bash
# Review backup
ls -lh /root/charity-backup

# Archive or delete if not needed
# Expected savings: 9.7GB
```

---

## 🎯 Recommended Cleanup Script

Save as `cleanup-disk.sh`:

```bash
#!/bin/bash
# GhostWire Emergency Disk Cleanup
# Run this to free ~20-25GB safely

set -e

echo "🧹 Starting GhostWire Disk Cleanup..."
echo ""

# 1. Clean Rust artifacts
echo "📦 Cleaning Rust build artifacts..."
cd /home/ph/GhostWire-secure-mesh-communication/ghostwire
cargo clean
echo "  ✅ Rust clean complete"

# 2. Clean Pacman cache (keep 3 recent versions)
echo "📦 Cleaning Pacman package cache..."
sudo paccache -r -k 3
echo "  ✅ Pacman clean complete"

# 3. Clean Docker build cache
echo "🐳 Cleaning Docker build cache..."
docker builder prune -a -f
echo "  ✅ Docker build cache clean complete"

# 4. Clean Docker unused data
echo "🐳 Cleaning Docker unused data..."
docker system prune -a --volumes -f
echo "  ✅ Docker system prune complete"

# 5. Clean temporary files
echo "🗑️  Cleaning temporary files..."
rm -rf /tmp/*
rm -rf /var/tmp/*
echo "  ✅ Temp files clean complete"

# 6. Show disk usage after cleanup
echo ""
echo "📊 Disk usage after cleanup:"
df -h / | tail -1

echo ""
echo "✅ Cleanup complete!"
echo "You should now have 20-25GB free space."
```

Make executable and run:
```bash
chmod +x cleanup-disk.sh
./cleanup-disk.sh
```

---

## 📈 Expected Results

### Before Cleanup
```
Used:  944GB (100%)
Free:  477MB  ❌ CRITICAL
```

### After Quick Cleanup
```
Used:  924GB (97%)
Free:  20GB    ✅ SAFE for compilation
```

### After Full Cleanup
```
Used:  910GB (95%)
Free:  34GB    ✅ COMFORTABLE
```

---

## 🔧 Prevent Future Issues

### 1. Configure Cargo to Limit Build Artifacts
Add to `~/.cargo/config.toml`:
```toml
[build]
# Keep only last build
incremental = false
```

### 2. Configure Pacman to Auto-Clean
Add to `/etc/pacman.conf`:
```
# Keep only 3 recent versions
CleanMethod = KeepCurrent,Keep3
```

### 3. Setup Docker Auto-Prune
Create systemd timer or cron job:
```bash
# Weekly Docker cleanup
0 3 * * 0 docker system prune -a -f
```

### 4. Monitor Disk Usage
Install and run `ncdu`:
```bash
sudo pacman -S ncdu
ncdu /home/ph/GhostWire-secure-mesh-communication
```

---

## ⚡ Emergency Quick Fix (5 minutes)

If you need space NOW for compilation:

```bash
# Quick 10GB in 5 minutes
cd /home/ph/GhostWire-secure-mesh-communication/ghostwire && cargo clean
docker builder prune -a -f
sudo paccache -r -k 1

# Check free space
df -h /
```

---

## 🎯 Priority Actions for GhostWire Testing

To run tests immediately:

1. **Clean Rust target** (3.2GB) - 2 minutes
2. **Clean Docker cache** (7.2GB) - 1 minute  
3. **Clean Pacman** (9.2GB) - 1 minute

**Total time:** 4 minutes  
**Total freed:** ~20GB  
**Result:** Can compile and run tests ✅

---

*Generated: March 30, 2026*
