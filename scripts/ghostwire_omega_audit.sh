#!/bin/bash

# 🌌 GhostWire Omega Android Audit & Injection
# Version: 4.0 (Omega) - Deep Level Mapping & Live Injection
# Purpose: Deepest system mapping + live sidecar injection for rooted verification.

DEVICE_ID=$1
PACKAGE_NAME="org.ghostwire.messenger"
SIDECAR_LOCAL="../src-tauri/binaries/ghostwire-aarch64-linux-android"
REMOTE_TMP="/data/local/tmp/ghostwire"

if [ -z "$DEVICE_ID" ]; then
    DEVICE_ID=$(adb devices | grep -v "List" | head -n 1 | awk '{print $1}')
fi

echo "🌌 GHOSTWIRE OMEGA (DEEP LEVEL): $DEVICE_ID"
echo "================================================================="

# --- 1. DEEP SYSTEM TELEMETRY ---
echo -e "\n[1] RADIO TELEMETRY"
echo "  📡 Wi-Fi State:"
adb -s "$DEVICE_ID" shell "dumpsys wifi" | grep -iE "mNetworkInfo|Wi-Fi is" | head -n 2 | awk '{print "    " $0}'
echo "  📡 BT Radio Power:"
adb -s "$DEVICE_ID" shell "dumpsys bluetooth_manager" | grep -i "state: ON" && echo "    ✅ BT Radio is Active" || echo "    ❌ BT Radio is OFF"

# --- 2. SECURITY & SOCKET AUDIT ---
echo -e "\n[2] SYSTEM SECURITY & SOCKETS"
echo "  🔒 SELinux Mode:"
adb -s "$DEVICE_ID" shell "getenforce" | awk '{print "    " $0}'
echo "  🔓 Open Sockets (Su-required):"
adb -s "$DEVICE_ID" shell "su -c 'netstat -tunlp'" 2>/dev/null | head -n 10 | awk '{print "    " $0}'

# --- 3. THE "DEEP" INJECTION TEST ---
echo -e "\n[3] LIVE SIDECAR INJECTION (ROOT)"
if [ -f "$SIDECAR_LOCAL" ]; then
    echo "  🚀 Pushing sidecar to device..."
    adb -s "$DEVICE_ID" push "$SIDECAR_LOCAL" "$REMOTE_TMP" > /dev/null
    adb -s "$DEVICE_ID" shell "chmod +x $REMOTE_TMP"
    echo "  🔥 Attempting Live Start (Headless)..."
    # Run in background via su
    adb -s "$DEVICE_ID" shell "su -c 'GHOSTWIRE_HOME=/data/data/$PACKAGE_NAME/files $REMOTE_TMP --web --port 8080 > /data/local/tmp/ghostwire.log 2>&1 &'"
    sleep 3
    
    SIDECAR_PID=$(adb -s "$DEVICE_ID" shell "ps -A | grep ghostwire" | awk '{print $2}')
    if [ ! -z "$SIDECAR_PID" ]; then
        echo "  ✅ INJECTION SUCCESSFUL! Sidecar running with PID: $SIDECAR_PID"
        echo "  🌐 Checking API responsiveness (Local):"
        adb -s "$DEVICE_ID" shell "su -c 'curl -s -X GET http://localhost:8080/api/health'" | head -n 5 | awk '{print "    API Health: " $0}'
    else
        echo "  ❌ INJECTION FAILED. Check /data/local/tmp/ghostwire.log on device."
        adb -s "$DEVICE_ID" shell "su -c 'cat /data/local/tmp/ghostwire.log'" | tail -n 5 | awk '{print "    Log: " $0}'
    fi
else
    echo "  ⚠️ Sidecar binary missing at $SIDECAR_LOCAL. Skipping injection."
fi

# --- 4. DATA ARCHITECTURE MAPPING ---
echo -e "\n[4] DATA ARCHITECTURE"
echo "  📁 Internal Vault Mapping:"
adb -s "$DEVICE_ID" shell "su -c 'ls -R /data/data/$PACKAGE_NAME/files'" 2>/dev/null | grep -v "cache" | head -n 20 | awk '{print "    " $0}'

# --- 5. POWER & DOZE STATUS ---
echo -e "\n[5] POWER MANAGEMENT (DOZE)"
adb -s "$DEVICE_ID" shell "dumpsys deviceidle" | grep -iE "mState=|mIsDeepIdle=" | awk '{print "    " $0}'
adb -s "$DEVICE_ID" shell "dumpsys battery" | grep "level" | awk '{print "    Battery: " $2 "%"}'

echo -e "\n================================================================="
echo "🎯 Omega Audit & Injection Complete."
