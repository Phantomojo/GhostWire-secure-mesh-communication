# GhostWire Android Deployment Guide

## Quick Deployment (Without Full NDK Build)

Since we're debugging the profile creation issue, we can deploy the backend to Android and run it directly:

### Option 1: Deploy Backend Binary to Android

```bash
#!/bin/bash
# deploy-to-android.sh

# 1. Build for Android architecture (if cross-compilation is set up)
# OR use the x86_64 binary if running on Android x86 emulator

# 2. Push to device
adb push /home/ph/GhostWire-secure-mesh-communication/ghostwire/target/release/ghostwire /data/local/tmp/

# 3. Make executable
adb shell "chmod 755 /data/local/tmp/ghostwire"

# 4. Run with logging
adb shell "RUST_LOG=info,profile_creation=debug,identity=debug /data/local/tmp/ghostwire"
```

### Option 2: Use Existing Tauri App with Updated Frontend

The Tauri app already has the backend integrated. We just need to:

1. **Update the frontend code** (already done - LoginScreen.tsx has enhanced logging)
2. **Build the Tauri Android app**
3. **Deploy via ADB**

## Building Tauri Android App

### Prerequisites

```bash
# Install Rust Android targets
rustup target add aarch64-linux-android
rustup target add x86_64-linux-android
rustup target add armv7-linux-androideabi

# Install Android NDK (if not present)
# Download from: https://developer.android.com/ndk/downloads
# Or use sdkmanager: sdkmanager "ndk;27.0.12077973"
```

### Build Commands

```bash
cd /home/ph/GhostWire-secure-mesh-communication/webui

# Install Tauri CLI
npm install -D @tauri-apps/cli

# Build for Android
npx tauri android build --release

# Or build for specific target
npx tauri android build --target aarch64
```

### Deploy to Device

```bash
# List connected devices
adb devices

# Install APK
adb install -r /home/ph/GhostWire-secure-mesh-communication/webui/src-tauri/target/aarch64-linux-android/release/app.apk

# Or use tauri command
npx tauri android adb install
```

## Testing Profile Creation

### On PC

```bash
# Start with enhanced logging
cd /home/ph/GhostWire-secure-mesh-communication
./start-ghostwire-debug.sh

# In another terminal, watch logs
tail -f logs/ghostwire_*.log | grep -E "profile_creation|identity|ERROR"

# Test via browser
curl -X POST http://localhost:8080/api/v2/profile \
  -H "Content-Type: application/json" \
  -d '{"name": "Test_Profile"}'
```

### On Android

```bash
# Start logcat
adb logcat -s GhostWire:* RUST:* > android_logs.txt

# Open the app and try to create a profile
# Then check logs
cat android_logs.txt | grep -E "profile|identity|error|failed"
```

## Troubleshooting

### NDK Not Found

```bash
# Set NDK path
export ANDROID_NDK_HOME=/opt/android-ndk-r27c
export NDK_HOME=/opt/android-ndk-r27c

# Or install via sdkmanager
sdkmanager "ndk;27.0.12077973"
```

### Build Errors

```bash
# Clean and rebuild
cd webui
rm -rf src-tauri/target
npx tauri android build --release
```

### Runtime Errors

```bash
# Check app logs
adb logcat | grep -i ghostwire

# Check backend logs (if running separately)
adb shell logcat | grep RUST
```

## Current Status

✅ **PC Backend**: Built with enhanced logging  
✅ **Frontend**: Updated with better error messages  
✅ **Debug Scripts**: Created for easy testing  
⏳ **Android APK**: Needs NDK for full build  

## Next Steps

1. **Test PC version first** to identify the exact error
2. **Install NDK** if Android-specific testing is needed
3. **Share logs** from both platforms for analysis

---

**Quick Test Command (PC):**
```bash
cd /home/ph/GhostWire-secure-mesh-communication
./start-ghostwire-debug.sh
# Then try to create a profile via WebUI
```

**Quick Deploy Command (Android):**
```bash
adb push ghostwire/target/release/ghostwire /data/local/tmp/
adb shell "chmod 755 /data/local/tmp/ghostwire && RUST_LOG=info,profile_creation=debug /data/local/tmp/ghostwire"
```
