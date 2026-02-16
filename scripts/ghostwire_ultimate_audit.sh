#!/bin/bash

# 🌌 GhostWire Ultimate Android Audit
# Version: 3.0 (Zenith) - Deep System Mapping
# Purpose: All-level deep audit of Mesh readiness, Security, and App Integrity.

DEVICE_ID=$1
PACKAGE_NAME="org.ghostwire.messenger"

if [ -z "$DEVICE_ID" ]; then
    DEVICE_ID=$(adb devices | grep -v "List" | head -n 1 | awk '{print $1}')
fi

echo "🌌 GHOSTWIRE ULTIMATE AUDIT: $DEVICE_ID"
echo "================================================================="

# --- 1. IDENTITY & KERNEL ---
echo -e "\n[1] SYSTEM IDENTITY"
adb -s "$DEVICE_ID" shell "getprop | grep -E 'ro.product.model|ro.build.version.release|ro.build.version.sdk|ro.product.cpu.abi'" | awk -F': ' '{print "  " $1 ": " $2}'
adb -s "$DEVICE_ID" shell "uname -a" | awk '{print "  Kernel: " $0}'

# --- 2. ROOT & PRIVILEGE MAPPING ---
echo -e "\n[2] ROOT & PRIVILEGE MAPPING"
ROOT_ID=$(adb -s "$DEVICE_ID" shell "su -c 'id'" 2>/dev/null)
if [[ $ROOT_ID == *"uid=0(root)"* ]]; then
    echo "  👑 Root: VERIFIED (Full System Control)"
    # Check if we can write to /system or /data
    adb -s "$DEVICE_ID" shell "su -c 'touch /data/local/tmp/gw_test && echo SUCCESS || echo FAIL'" | grep -q "SUCCESS" && echo "  📁 Data Write: SUCCESS"
else
    echo "  ⚠️ Root: FAILED / NOT CONFIGURED"
fi

# --- 3. MESH RADIO HARDWARE CHECK ---
echo -e "\n[3] MESH RADIO HARDWARE"
echo "  📡 Wi-Fi Direct Support:"
adb -s "$DEVICE_ID" shell "pm list features" | grep -i "android.hardware.wifi.direct" > /dev/null && echo "    ✅ Hardware Feature: Supported" || echo "    ❌ Hardware Feature: Missing"

echo "  📡 Bluetooth LE / Mesh Readiness:"
adb -s "$DEVICE_ID" shell "dumpsys bluetooth_manager" | grep -iE "LE support|Advertising" | head -n 3 | awk '{print "    " $0}'

# --- 4. APP BUNDLE & SIDECAR AUDIT ---
echo -e "\n[4] APP INTEGRITY & SIDECAR"
APK_PATH=$(adb -s "$DEVICE_ID" shell "pm path $PACKAGE_NAME" | cut -d':' -f2)
echo "  📦 APK Location: $APK_PATH"

# Deep search for the elusive sidecar in the app's lib directory
echo "  🕵️ Native Libs (Device):"
adb -s "$DEVICE_ID" shell "su -c 'ls -R /data/app/*/$PACKAGE_NAME*/lib/'" 2>/dev/null | awk '{print "    " $0}'

# Check if sidecar is in the UID-specific files dir
echo "  📂 Persistent Files:"
adb -s "$DEVICE_ID" shell "su -c 'ls -la /data/data/$PACKAGE_NAME/files'" 2>/dev/null | awk '{print "    " $0}'

# --- 5. NETWORK STACK & SOCKETS ---
echo -e "\n[5] NETWORK STACK & SOCKETS"
echo "  🎧 Listening Ports (Su-Required):"
adb -s "$DEVICE_ID" shell "su -c 'netstat -tunlp'" 2>/dev/null | grep -E "ghost|app_process" | awk '{print "    " $0}' || echo "    No active GhostWire listeners."

# --- 6. THERMAL & POWER DRAIN ---
echo -e "\n[6] THERMALS & POWER"
adb -s "$DEVICE_ID" shell "dumpsys battery" | grep -E "level|status|health|temperature" | awk '{print "    " $0}'
adb -s "$DEVICE_ID" shell "cat /sys/class/thermal/thermal_zone*/temp 2>/dev/null | head -n 1" | awk '{print "    Current SoC Temp: " ($1/1000) "°C"}'

# --- 7. RECENT LOGCAT TRACE (TOP CRITICALS) ---
echo -e "\n[7] CRITICAL LOGCAT TRACE"
adb -s "$DEVICE_ID" shell "logcat -d *:E | grep -iE 'GhostWire|tauri|sidecar' | tail -n 10" | awk '{print "    " $0}'

echo "================================================================="
echo "🎯 Zenith Audit Complete."
