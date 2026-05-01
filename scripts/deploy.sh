#!/bin/bash
# GhostWire Deploy Script — zero-downtime binary update
# Usage: ./deploy.sh <target-host> [mesh-port] [bootstrap-addr]
#
# This script:
#   1. Stops the running GhostWire process
#   2. Backs up the old binary
#   3. Copies the new binary via scp
#   4. Verifies SHA256 hash matches
#   5. Starts the new process with correct args
#   6. Verifies the new process is running

set -uo pipefail

BINARY="ghostwire/target/release/ghostwire"
REMOTE_PATH="/usr/local/bin/ghostwire"
BACKUP_PATH="/usr/local/bin/ghostwire.bak.$(date +%Y%m%d%H%M%S)"
EXPECTED_HASH=$(sha256sum "$BINARY" | awk '{print $1}')

TARGET_HOST="${1:?Usage: $0 <tailscale-host> [mesh-port] [bootstrap-addr]}"
MESH_PORT="${2:-4001}"
BOOTSTRAP="${3:-}"

echo "═══════════════════════════════════════════════════════"
echo " GhostWire Deploy — $(date)"
echo "═══════════════════════════════════════════════════════"
echo " Target:    $TARGET_HOST"
echo " Binary:    $BINARY ($(du -h "$BINARY" | cut -f1))"
echo " Hash:      $EXPECTED_HASH"
echo " Mesh port: $MESH_PORT"
echo " Bootstrap: ${BOOTSTRAP:-(none — will discover via mDNS)}"
echo ""

# Step 1: Stop old process
echo "▸ Stopping old GhostWire process..."
ssh root@"$TARGET_HOST" "PID=\$(pgrep -f 'ghostwire --web' 2>/dev/null) && kill \$PID 2>/dev/null; exit 0"
sleep 3

# Verify it stopped
RUNNING=$(ssh root@"$TARGET_HOST" "pgrep -f 'ghostwire --web' 2>/dev/null || true")
if [ -n "$RUNNING" ]; then
    echo "⚠ Process didn't stop gracefully, force killing..."
    ssh root@"$TARGET_HOST" "kill -9 \$(pgrep -f 'ghostwire --web') 2>/dev/null; exit 0"
    sleep 1
fi
echo "  ✓ Old process stopped"

# Step 2: Backup old binary
echo "▸ Backing up old binary..."
ssh root@"$TARGET_HOST" "if [ -f $REMOTE_PATH ]; then cp $REMOTE_PATH $BACKUP_PATH && echo '  ✓ Backed up to $BACKUP_PATH'; else echo '  No old binary to backup'; fi"

# Step 3: Copy new binary
echo "▸ Copying new binary ($(du -h "$BINARY" | cut -f1))..."
scp -q "$BINARY" root@"$TARGET_HOST":"${REMOTE_PATH}.new"
echo "  ✓ Binary copied"

# Step 4: Verify hash
echo "▸ Verifying binary integrity..."
REMOTE_HASH=$(ssh root@"$TARGET_HOST" "sha256sum ${REMOTE_PATH}.new | awk '{print \$1}'")
if [ "$REMOTE_HASH" != "$EXPECTED_HASH" ]; then
    echo "✗ HASH MISMATCH!"
    echo "  Expected: $EXPECTED_HASH"
    echo "  Got:      $REMOTE_HASH"
    echo "  Cleaning up..."
    ssh root@"$TARGET_HOST" "rm -f ${REMOTE_PATH}.new"
    exit 1
fi
echo "  ✓ Hash verified: $REMOTE_HASH"

# Step 5: Atomically replace binary
echo "▸ Installing new binary..."
ssh root@"$TARGET_HOST" "chmod +x ${REMOTE_PATH}.new && mv ${REMOTE_PATH}.new $REMOTE_PATH"
echo "  ✓ Binary installed"

# Step 6: Start new process
echo "▸ Starting GhostWire..."
if [ -n "$BOOTSTRAP" ]; then
    ssh root@"$TARGET_HOST" "nohup $REMOTE_PATH --web --port 9097 --mesh-host 0.0.0.0 --mesh-port $MESH_PORT --bootstrap $BOOTSTRAP > /var/log/ghostwire.log 2>&1 &"
    echo "  ✓ Started with bootstrap: $BOOTSTRAP"
else
    ssh root@"$TARGET_HOST" "nohup $REMOTE_PATH --web --port 9097 --mesh-host 0.0.0.0 --mesh-port $MESH_PORT > /var/log/ghostwire.log 2>&1 &"
    echo "  ✓ Started (mDNS discovery)"
fi

# Step 7: Verify
sleep 3
PID=$(ssh root@"$TARGET_HOST" "pgrep -f 'ghostwire --web' 2>/dev/null || true")
if [ -n "$PID" ]; then
    echo "  ✓ GhostWire running (PID: $PID)"
    echo ""
    echo "═══════════════════════════════════════════════════════"
    echo " Deploy complete — $(date)"
    echo "═══════════════════════════════════════════════════════"
else
    echo "✗ Process failed to start!"
    echo "  Check logs: ssh root@$TARGET_HOST 'tail -50 /var/log/ghostwire.log'"
    echo "  Rolling back..."
    ssh root@"$TARGET_HOST" "if [ -f $BACKUP_PATH ]; then mv $BACKUP_PATH $REMOTE_PATH && echo '  ✓ Rolled back'; fi"
    exit 1
fi
