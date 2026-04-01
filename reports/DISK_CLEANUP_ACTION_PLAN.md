# 🚀 GhostWire Disk Cleanup - Complete Action Plan

**Date:** March 30, 2026  
**Goal:** Free 200GB+ safely without losing work data  
**Status:** Ready to Execute

---

## 📊 Current Situation

### Disk Usage (1TB NVMe)
```
Used:  944GB (100%) ❌ CRITICAL
Free:  477MB
```

### Second Drive (326GB NVMe - Windows)
```
Used:  249GB (77%)
Free:  77GB
Mount: /mnt/winspace
```

---

## 🎯 Three-Phase Cleanup Plan

### Phase 1: SAFE Automated Cleanup (5 minutes, frees 30-40GB)
**What it does:**
- ✅ Cleans build artifacts (Rust, Docker)
- ✅ Cleans package caches (pacman, AUR, npm, pip)
- ✅ Deletes old ISOs (StratOS)
- ✅ Cleans temp files
- ✅ **Does NOT touch:** Downloads, work data, Steam, personal files

**How to run:**
```bash
cd /home/ph/GhostWire-secure-mesh-communication
./cleanup-safe.sh
```

**Expected results:**
- Frees: **30-40GB**
- Risk: **ZERO** (all safe cleanup)
- Time: **5 minutes**

---

### Phase 2: Manual Review (30 minutes, frees 50GB+)
**What to review:**

#### 1. Old Firmware Files (12GB)
```
Location: /home/ph/Samsung/firmware/
Files:
  - AP_A145FXXSCDZA2_QB105604778_REV00_user_low_ship_MULTI_CERT_meta_OS15.tar.md5 (6.4GB)
  - AP_A145FXXSCDZA2.tar (6.4GB)
  - A145FXXSCDZA2_AFR.zip (5.7GB)
  - magisk_patched-30600_wq83i.tar (5.0GB)

Action: Review if still needed for Samsung device flashing
Delete if: Devices already flashed or firmware outdated
```

#### 2. Old Disk Images (6.1GB)
```
Location: /home/ph/Downloads/Persepolis/disk.7z
Size: 6.1GB

Action: Review if still needed
Delete if: Old backup or no longer needed
```

#### 3. Charity Backup (9.7GB)
```
Location: /root/charity-backup/
Size: 9.7GB

Action: Review contents
Delete if: Old project no longer needed
Command: ls -lh /root/charity-backup/
```

#### 4. Large Archive Files (5GB+)
```
Locations:
  - /home/ph/data/urbanpredict-ke/data/pp-complete.zip (1.5GB)
  - /home/ph/tak-server/takserver-docker-5.6-RELEASE-2.zip (562MB)
  - /home/ph/CTFs/*.ova, *.7z files

Action: Review each archive
Delete if: Old projects, completed CTFs, or backed up elsewhere
```

**Expected results:**
- Frees: **50GB+** (depending on what you keep)
- Risk: **LOW** (review before deleting)
- Time: **30 minutes**

---

### Phase 3: Move Steam Games (2 hours, frees 170GB)
**What it does:**
- Moves Steam games from `/home/ph/.local/share/Steam` to `/mnt/winspace/ph-archive/games/`
- Creates symlinks so Steam still works
- Keeps Steam itself on Linux drive (config, screenshots)

**How to run:**
```bash
cd /home/ph/GhostWire-secure-mesh-communication
./move-steam-to-windows.sh
```

**What gets moved:**
- Game files (~170GB)
- Shader cache (~10GB)
- Proton compatdata (~20GB)

**What stays:**
- Steam client
- Steam config
- Screenshots
- Save games (usually in Documents anyway)

**Expected results:**
- Frees: **170GB**
- Risk: **LOW** (symlinks preserve functionality)
- Time: **30 minutes** (plus download time if Steam re-downloads)

---

## 📋 Step-by-Step Instructions

### Step 1: Run Phase 1 Cleanup (NOW - 5 minutes)
```bash
cd /home/ph/GhostWire-secure-mesh-communication
./cleanup-safe.sh
```

**Watch for:**
- Green checkmarks ✅ = Success
- Yellow warnings ⚠️ = Skipped (safe to ignore)
- Red errors ❌ = Problem (report these)

**After completion:**
```bash
df -h /
```
Should show **40GB+ free**

---

### Step 2: Verify Compilation Works (2 minutes)
```bash
cd /home/ph/GhostWire-secure-mesh-communication/ghostwire
cargo clean
cargo build --release
```

**Expected:** Builds successfully without "No space left on device" error

---

### Step 3: Review Large Files (15 minutes)
```bash
# Find files > 500MB
find /home/ph -type f -size +500M -exec ls -lh {} \; | sort -k5 -hr

# Interactive disk usage
ncdu /home/ph
```

**Decide what to keep/delete:**
- ✅ Keep: Work projects, current CTFs, active development
- ⚠️ Review: Old firmware, old ISOs, old backups
- ❌ Delete: Duplicates, completed projects, outdated files

---

### Step 4: Delete Reviewed Files (10 minutes)
```bash
# Example deletions (REVIEW FIRST!)
# rm /home/ph/Samsung/firmware/old-file.tar
# rm /root/charity-backup/old-backup.tar.gz
# rm /home/ph/Downloads/old-iso.iso
```

**After deletions:**
```bash
df -h /
```
Should show **60-90GB free**

---

### Step 5: Move Steam Games (OPTIONAL - 30 minutes + download time)
**Only if you need more space after Steps 1-4**

```bash
cd /home/ph/GhostWire-secure-mesh-communication
./move-steam-to-windows.sh
```

**Follow the prompts:**
1. Script will show games to move
2. Confirm with 'y'
3. Wait for rsync to complete (10-30 minutes)
4. Script creates symlinks automatically
5. Test Steam to verify games work

**After completion:**
```bash
df -h /
```
Should show **200GB+ free**

---

## 🎯 Expected Final Results

### After Phase 1 Only:
```
Used:  904GB (95%)
Free:  40GB    ✅ SAFE for compilation
```

### After Phase 1 + 2:
```
Used:  860GB (90%)
Free:  84GB    ✅ COMFORTABLE
```

### After All Phases (with Steam move):
```
Used:  690GB (72%)
Free:  254GB   ✅ PLENTY of space
```

---

## ⚠️ What NOT to Delete

### Work Data (Keep):
- ❌ `/home/ph/Cybesecurity/` (97GB)
- ❌ `/home/ph/Samsung/` (83GB) - except firmware reviews
- ❌ `/home/ph/data/` (28GB) - except old archives
- ❌ `/home/ph/tak-server/` (6.7GB)
- ❌ `/home/ph/CTFs/` (2.9GB) - unless completed

### Project Source (Keep):
- ❌ `/home/ph/GhostWire-secure-mesh-communication/` (source code only)
- ✅ OK to clean: `ghostwire/target/` (build artifacts)

### Personal Files (Keep):
- ❌ `/home/ph/Documents/`
- ❌ `/home/ph/Pictures/`
- ❌ `/home/ph/.local/share/lutris/` (if you use Lutris)

---

## 🛠️ Troubleshooting

### "No space left on device" during cleanup:
```bash
# Emergency free 1GB
rm -rf ~/.cache/thumbnails/*
rm -rf /tmp/*
```

### Steam games don't work after move:
```bash
# Check symlinks
ls -la /home/ph/.local/share/Steam/steamapps/common

# Should show: common -> /mnt/winspace/ph-archive/games/steamgames

# If broken, restore from backup
rm /home/ph/.local/share/Steam/steamapps/common
mv /home/ph/.local/share/Steam/steamapps/common.backup \
   /home/ph/.local/share/Steam/steamapps/common
```

### Can't mount Windows drive:
```bash
# Check if drive exists
lsblk

# Try manual mount
sudo mount /dev/nvme1n1p4 /mnt/winspace
```

---

## 📊 Monitoring Disk Usage

### Check current usage:
```bash
df -h /
```

### Find what's using space:
```bash
# Top 20 largest directories
du -sh /home/ph/* 2>/dev/null | sort -hr | head -20

# Interactive
ncdu /home/ph
```

### Setup monitoring (optional):
```bash
# Add to ~/.bashrc
alias disk='df -h / | tail -1 && echo "" && du -sh /home/ph/* 2>/dev/null | sort -hr | head -10'
```

---

## 🎯 Success Criteria

### Minimum Success (Phase 1 only):
- ✅ 30GB+ freed
- ✅ Can compile Rust projects
- ✅ No "No space left" errors

### Good Success (Phase 1 + 2):
- ✅ 80GB+ freed
- ✅ Comfortable margin
- ✅ Room for growth

### Excellent Success (All phases):
- ✅ 200GB+ freed
- ✅ Steam on second drive
- ✅ Organized file structure
- ✅ Long-term solution

---

## 📞 Need Help?

### If unsure about deleting something:
1. Check file age: `ls -lh /path/to/file`
2. Check file type: `file /path/to/file`
3. Search for references: `grep -r "filename" ~/projects`
4. Ask: "Would I need this in the next month?"

### Before deleting work data:
1. Check if backed up
2. Check if in version control (git)
3. Check if project still active
4. Ask team members (if applicable)

---

## 🎉 Post-Cleanup Tasks

### After successful cleanup:

1. **Verify GhostWire tests work:**
   ```bash
   cd ghostwire
   cargo test --test protocol_integration
   ```

2. **Setup automatic cleanup (optional):**
   ```bash
   # Add to crontab (monthly cleanup)
   crontab -e
   
   # Add line:
   0 3 1 * * /home/ph/GhostWire-secure-mesh-communication/cleanup-safe.sh
   ```

3. **Monitor disk usage:**
   ```bash
   # Install ncdu
   sudo pacman -S ncdu
   
   # Check weekly
   ncdu /home/ph
   ```

---

*Action Plan created: March 30, 2026*  
*Execute Phase 1 immediately, Phase 2-3 as needed*
