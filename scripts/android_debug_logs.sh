#!/bin/bash
# Enhanced Android Debug Log Streaming with Save to File
# Streams GhostWire logs and saves to timestamped file

set -euo pipefail

# Create logs directory
LOGS_DIR="logs/android"
mkdir -p "$LOGS_DIR"

# Generate timestamped log file
TIMESTAMP=$(date '+%Y%m%d_%H%M%S')
LOG_FILE="$LOGS_DIR/ghostwire_android_${TIMESTAMP}.log"

echo "🔍 Starting Android Debug Log Stream..."
echo "📱 Device: $(adb devices | grep -v "List" | awk '{print $1}')"
echo "⏰ Started at: $(date '+%Y-%m-%d %H:%M:%S')"
echo "💾 Saving to: $LOG_FILE"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Clear old logs
adb logcat -c

# Stream logs with comprehensive filters
# Save to file AND display with colors
adb logcat -v threadtime \
  app_lib:V \
  RustStdoutStderr:V \
  tauri_plugin_shell:V \
  Tauri/Console:V \
  Tauri:V \
  chromium:I \
  ActivityManager:I \
  AndroidRuntime:E \
  CRASH:E \
  DEBUG:E \
  *:S | tee "$LOG_FILE" | while IFS= read -r line; do
    # Color coding
    if echo "$line" | grep -qE " E | F "; then
        echo -e "\033[0;31m$line\033[0m"  # Red for errors
    elif echo "$line" | grep -q " W "; then
        echo -e "\033[0;33m$line\033[0m"  # Yellow for warnings
    elif echo "$line" | grep -qE "GhostWire|sidecar|ghostwire"; then
        echo -e "\033[1;35m$line\033[0m"  # Magenta for GhostWire-specific
    elif echo "$line" | grep -q " I "; then
        echo -e "\033[0;32m$line\033[0m"  # Green for info
    elif echo "$line" | grep -q " D "; then
        echo -e "\033[0;36m$line\033[0m"  # Cyan for debug
    else
        echo "$line"
    fi
done

echo ""
echo "📝 Log saved to: $LOG_FILE"
