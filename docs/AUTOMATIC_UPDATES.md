# Automatic Updates

## Overview

GhostWire supports automatic updates with **automatic backup creation** before each update. This ensures you can always roll back if something goes wrong.

## Features

- ✅ **Automatic update checking** every 24 hours (configurable)
- ✅ **Automatic backups** created before each update
- ✅ **Desktop notifications** when updates are available
- ✅ **Configurable intervals** (hourly, daily, weekly)
- ✅ **Safe rollback** if update fails
- ✅ **Zero downtime** - updates happen in background

## Enabling Automatic Updates

### Quick Start

```bash
# Enable automatic updates
./ghostwire-manager.sh auto-update on

# This will:
# 1. Enable auto-updates in configuration
# 2. Start background daemon
# 3. Check for updates every 24 hours
# 4. Create automatic backups before updating
```

### Manual Configuration

Edit `ghostwire.conf`:

```bash
# Enable auto-updates
GHOSTWIRE_AUTO_UPDATE=true

# Check every 12 hours instead of 24
GHOSTWIRE_UPDATE_INTERVAL=43200
```

Then start the checker:

```bash
./scripts/auto-update-checker.sh --daemon &
```

## Checking for Updates Manually

```bash
# Check if updates are available
./ghostwire-manager.sh check-updates

# Output:
# ╔═══════════════════════════════════════════╗
# ║   Update Available!                       ║
# ╚═══════════════════════════════════════════╝
#
# A new version of GhostWire is available.
#
# To update now:
#   ./ghostwire-manager.sh update
```

## How Automatic Updates Work

### 1. Background Daemon

When you enable auto-updates, a background daemon starts that:
- Checks for updates at regular intervals (default: 24 hours)
- Compares local version with remote version
- Notifies you when updates are available

### 2. Automatic Backup

Before applying any update, the system:
- Creates a full backup of your installation
- Names it: `ghostwire.auto-backup.YYYYMMDD_HHMMSS`
- Stores it safely for rollback

### 3. Safe Update

The update process:
- Pulls latest changes from git
- Rebuilds backend and frontend
- Installs updated files
- **Automatically rolls back if any step fails**

### 4. Notification

Desktop notifications appear when:
- An update is available
- An update is successfully installed
- An update fails (with rollback confirmation)

## Configuration Options

### Update Interval

```bash
# In ghostwire.conf:

# Check every hour
GHOSTWIRE_UPDATE_INTERVAL=3600

# Check every 12 hours
GHOSTWIRE_UPDATE_INTERVAL=43200

#Check every 24 hours (default)
GHOSTWIRE_UPDATE_INTERVAL=86400

# Check every week
GHOSTWIRE_UPDATE_INTERVAL=604800
```

### Auto-Update Behavior

```bash
# Enable fully automatic updates (no confirmation)
GHOSTWIRE_AUTO_UPDATE=true

# Disable auto-updates (notify only)
GHOSTWIRE_AUTO_UPDATE=false
```

## Disabling Automatic Updates

```bash
# Disable auto-updates
./ghostwire-manager.sh auto-update off

# This will:
# 1. Disable auto-updates in configuration
# 2. Stop background daemon
# 3. Keep manual updates available
```

## Manual Updates

You can always update manually:

```bash
# Update with automatic backup
./ghostwire-manager.sh update

# Or directly
./update.sh
```

## Viewing Update Logs

```bash
# View auto-update checker logs
tail -f ~/.local/ghostwire/logs/auto-update.log

# View daemon logs
tail -f ~/.local/ghostwire/logs/auto-update-daemon.log
```

## Rollback

If an update fails, it automatically rolls back. Manual rollback:

```bash
# List backups
ls -la ~/.local/ghostwire.auto-backup.*

# Restore from backup
./ghostwire-manager.sh restore ~/.local/ghostwire.auto-backup.20260216_120000.tar.gz
```

## Multi-PC Deployment

### Centralized Updates

For multiple PCs, you can:

1. **Enable auto-updates on all nodes:**
```bash
# On each PC
./ghostwire-manager.sh auto-update on
```

2. **Use custom update intervals:**
```bash
# PC 1: Check every 6 hours
echo "GHOSTWIRE_UPDATE_INTERVAL=21600" >> ghostwire.conf

# PC 2: Check every 12 hours  
echo "GHOSTWIRE_UPDATE_INTERVAL=43200" >> ghostwire.conf
```

3. **Staggered updates** to avoid all nodes updating at once

## Example Workflow

### Initial Setup

```bash
# Install GhostWire
./install.sh --username alice

# Enable automatic updates
./ghostwire-manager.sh auto-update on

# Verify it's running
./ghostwire-manager.sh status
```

### Daily Operation

The system now:
1. Checks for updates every 24 hours
2. Shows desktop notification if update available
3. Creates backup automatically
4. Applies update in background
5. Notifies when complete

### If Something Goes Wrong

Updates automatically rollback on failure:
```
[INFO] Auto-updating GhostWire...
[✓] Automatic backup created: /home/alice/.local/ghostwire.auto-backup.20260216_120000
[✗] Build failed
[INFO] Rolling back to backup...
[✓] Rollback complete
```

## Best Practices

1. **Enable auto-updates** for hassle-free maintenance
2. **Monitor logs** occasionally to check update status
3. **Keep backups** for at least a week (automatic cleanup keeps last 3)
4. **Test updates** on one PC before enabling on all nodes
5. **Use longer intervals** (weekly) for production systems

## Troubleshooting

### Auto-updates not working

```bash
# Check if daemon is running
ps aux | grep auto-update-checker

# Check logs
tail -f ~/.local/ghostwire/logs/auto-update.log

# Restart daemon
./ghostwire-manager.sh auto-update off
./ghostwire-manager.sh auto-update on
```

### Update fails

Updates automatically rollback, but you can manually restore:

```bash
# List available backups
ls -la ~/.local/ghostwire.auto-backup.*

# Restore specific backup
./ghostwire-manager.sh restore /path/to/backup.tar.gz
```

## Summary

Automatic updates with GhostWire:
- ✅ **Completely automatic** - No user intervention needed
- ✅ **Safe** - Automatic backups and rollback
- ✅ **Configurable** - Set your preferred interval
- ✅ **Transparent** - Desktop notifications keep you informed
- ✅ **Easy to manage** - Simple on/off commands

Perfect for multi-PC deployments where manual updates would be time-consuming!
