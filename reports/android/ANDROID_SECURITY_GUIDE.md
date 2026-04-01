# GhostWire Android Development & Security Guide

## 🔍 Critical Issue Identified

### Problem: Android WebView Blocking HTTP Traffic to Localhost

**Error from logs:**
```
Access to fetch at 'http://tauri.localhost:8080/api/v2/profile' from origin 
'http://tauri.localhost' has been blocked by CORS policy: Request header field 
content-type is not allowed by Access-Control-AllowHeaders in preflight response.
```

**Root Cause:** Android 9+ (API 28+) blocks cleartext HTTP traffic by default, including to localhost/127.0.0.1.

---

## 📋 Android-Specific Considerations

### 1. Network Security Configuration

Android requires explicit configuration to allow cleartext (HTTP) traffic to localhost:

#### Create `res/xml/network_security_config.xml`:

```xml
<?xml version="1.0" encoding="utf-8"?>
<network-security-config>
    <!-- Allow cleartext traffic for development -->
    <debug-overrides>
        <trust-anchors>
            <certificates src="system" />
        </trust-anchors>
    </debug-overrides>
    
    <!-- Allow localhost for backend communication -->
    <domain-config cleartextTrafficPermitted="true">
        <domain includeSubdomains="true">localhost</domain>
        <domain includeSubdomains="true">127.0.0.1</domain>
        <domain includeSubdomains="true">tauri.localhost</domain>
    </domain-config>
</network-security-config>
```

#### Update `AndroidManifest.xml`:

```xml
<manifest ...>
    <application
        android:networkSecurityConfig="@xml/network_security_config"
        android:usesCleartextTraffic="true"
        ...>
        ...
    </application>
</manifest>
```

⚠️ **Security Note:** Only enable `usesCleartextTraffic="true"` for debug builds. For production, use HTTPS.

---

### 2. CORS Configuration for Android

The backend must allow Tauri Android origins:

#### Update `ghostwire/src/web.rs`:

```rust
let cors = CorsLayer::new()
    .allow_origin([
        "http://localhost:5173".parse::<HeaderValue>().unwrap(),
        "http://127.0.0.1:5173".parse::<HeaderValue>().unwrap(),
        "tauri://localhost".parse::<HeaderValue>().unwrap(),
        "http://tauri.localhost".parse::<HeaderValue>().unwrap(),
        "http://tauri.localhost:8080".parse::<HeaderValue>().unwrap(), // Android needs this
        "http://localhost:8080".parse::<HeaderValue>().unwrap(),       // Android fallback
    ])
    .allow_methods([
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::DELETE,
        Method::OPTIONS, // Important for preflight
    ])
    .allow_credentials(true)
    .allow_headers([
        header::CONTENT_TYPE,  // Critical for JSON requests
        header::AUTHORIZATION,
        header::ACCEPT,
        header::ORIGIN,
        HeaderName::from_static("x-ghost-token"),
    ]);
```

---

### 3. Android WebView Security Best Practices

#### Enable JavaScript (Required for Tauri):
```kotlin
webView.settings.javaScriptEnabled = true
webView.settings.domStorageEnabled = true
```

#### Security Settings:
```kotlin
// Disable file access
webView.settings.allowFileAccess = false
webView.settings.allowContentAccess = false

// Disable geolocation unless needed
webView.settings.geolocationEnabled = false

// Set user agent if needed for API compatibility
webView.settings.userAgentString = "GhostWire-Android/1.0"
```

#### Clear Sensitive Data on Logout:
```kotlin
// Clear cookies
CookieManager.getInstance().removeAllCookies(null)
CookieManager.getInstance().flush()

// Clear WebView cache
webView.clearCache(true)
webView.clearHistory()

// Clear localStorage (via JavaScript)
webView.evaluateJavascript("localStorage.clear()", null)
```

---

### 4. Android Permissions

#### Minimum Required Permissions (`AndroidManifest.xml`):

```xml
<!-- Internet access for backend communication -->
<uses-permission android:name="android.permission.INTERNET" />

<!-- Network state (optional but recommended) -->
<uses-permission android:name="android.permission.ACCESS_NETWORK_STATE" />

<!-- Local network access (Android 13+) -->
<uses-permission android:name="android.permission.ACCESS_WIFI_STATE" />
```

#### Optional Permissions (Feature-Dependent):

```xml
<!-- For mesh networking -->
<uses-permission android:name="android.permission.CHANGE_WIFI_STATE" />
<uses-permission android:name="android.permission.NEARBY_WIFI_DEVICES" />

<!-- For Bluetooth mesh -->
<uses-permission android:name="android.permission.BLUETOOTH" />
<uses-permission android:name="android.permission.BLUETOOTH_ADMIN" />
<uses-permission android:name="android.permission.BLUETOOTH_CONNECT" />

<!-- For notifications -->
<uses-permission android:name="android.permission.POST_NOTIFICATIONS" />
```

---

### 5. Android-Specific Security Enhancements

#### A. Certificate Pinning (Production)

Create `res/raw/certificate.pem` with your backend's certificate:

```xml
<!-- network_security_config.xml -->
<network-security-config>
    <domain-config>
        <domain includeSubdomains="true">your-backend.com</domain>
        <pin-set expiration="2026-12-31">
            <pin digest="SHA-256">BASE64_CERT_HASH</pin>
        </pin-set>
    </domain-config>
</network-security-config>
```

#### B. ProGuard/R8 Obfuscation

Enable in `build.gradle`:

```gradle
android {
    buildTypes {
        release {
            minifyEnabled true
            shrinkResources true
            proguardFiles getDefaultProguardFile('proguard-android-optimize.txt'), 'proguard-rules.pro'
        }
    }
}
```

Add to `proguard-rules.pro`:

```proguard
# Keep Tauri classes
-keep class org.ghostwire.messenger.** { *; }
-keep class net.java.dev.jna.** { *; }

# Keep serialization classes
-keepclassmembers class * {
    @com.google.gson.annotations.SerializedName <fields>;
}
```

#### C. Secure Storage

Use Android Keystore for sensitive data:

```kotlin
// Store auth token securely
val preferences = EncryptedSharedPreferences.create(
    context,
    "ghostwire_prefs",
    masterKey,
    EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
    EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM
)
```

---

### 6. Android Build Configuration

#### `build.gradle` (App):

```gradle
android {
    compileSdk 34
    
    defaultConfig {
        applicationId "org.ghostwire.messenger"
        minSdk 24  // Android 7.0+
        targetSdk 34
        versionCode 1
        versionName "0.1.0"
        
        // Enable multidex for large apps
        multiDexEnabled true
    }
    
    buildTypes {
        debug {
            applicationIdSuffix ".debug"
            debuggable true
        }
        release {
            minifyEnabled true
            shrinkResources true
            proguardFiles getDefaultProguardFile('proguard-android-optimize.txt'), 'proguard-rules.pro'
        }
    }
    
    // Enable Jetifier for compatibility
    jetifier true
}

dependencies {
    // Tauri dependencies
    implementation project(':tauri-android')
    
    // Security
    implementation 'androidx.security:security-crypto:1.1.0-alpha06'
    
    // Multidex
    implementation 'androidx.multidex:multidex:2.0.1'
}
```

---

### 7. Privacy Considerations

#### A. Data Minimization

Only request necessary permissions. Document why each permission is needed:

```xml
<!-- Example privacy manifest entry -->
<permission
    android:name="android.permission.INTERNET"
    android:usage="Required for mesh network communication" />
```

#### B. User Consent

Implement runtime permission requests with clear explanations:

```kotlin
if (ContextCompat.checkSelfPermission(this, Manifest.permission.POST_NOTIFICATIONS)
    != PackageManager.PERMISSION_GRANTED) {
    
    ActivityCompat.requestPermissions(this,
        arrayOf(Manifest.permission.POST_NOTIFICATIONS),
        REQUEST_NOTIFICATION_PERMISSION)
}
```

#### C. Data Retention

Implement automatic data cleanup:

```kotlin
// Clear old messages after 30 days
fun cleanupOldData() {
    val thirtyDaysAgo = System.currentTimeMillis() - (30 * 24 * 60 * 60 * 1000)
    messageStore.deleteMessagesOlderThan(thirtyDaysAgo)
}
```

---

### 8. Testing on Android

#### A. USB Debugging Setup

1. Enable Developer Options (tap Build Number 7 times)
2. Enable USB Debugging
3. Connect device via USB
4. Verify connection: `adb devices`

#### B. Log Collection

```bash
# Real-time logs
adb logcat -s GhostWire:* RUST:* tauri:*

# Save logs to file
adb logcat -d > ghostwire_logs.txt

# Filter by error level
adb logcat *:E | grep ghostwire
```

#### C. Network Inspection

```bash
# Port forwarding (if backend runs on PC)
adb reverse tcp:8080 tcp:8080

# Test backend connectivity
adb shell curl http://localhost:8080/api/status
```

---

### 9. Common Android Issues & Solutions

| Issue | Solution |
|-------|----------|
| **CORS errors** | Add `tauri.localhost:8080` to CORS origins |
| **Cleartext blocked** | Add `usesCleartextTraffic="true"` to manifest |
| **WebView blank** | Enable JavaScript: `settings.javaScriptEnabled = true` |
| **API calls fail** | Check network security config allows localhost |
| **App crashes on startup** | Check logcat for missing permissions or resources |
| **Backend not reachable** | Use `adb reverse tcp:8080 tcp:8080` |

---

### 10. Production Checklist

- [ ] Disable cleartext traffic (use HTTPS)
- [ ] Enable ProGuard/R8 obfuscation
- [ ] Implement certificate pinning
- [ ] Remove debug logging
- [ ] Sign with release key (not debug key)
- [ ] Test on multiple Android versions (7.0 - 14)
- [ ] Verify all permissions are necessary
- [ ] Implement secure token storage
- [ ] Add crash reporting
- [ ] Test offline functionality
- [ ] Verify data cleanup works

---

## 🔐 Security Architecture

### Android Threat Model

| Threat | Mitigation |
|--------|-----------|
| **Man-in-the-Middle** | Certificate pinning, HTTPS |
| **Data Leakage** | Encrypted storage, permission controls |
| **Reverse Engineering** | ProGuard obfuscation, native code |
| **Root Access** | Root detection, SafetyNet attestation |
| **Screen Recording** | FLAG_SECURE for sensitive screens |
| **Clipboard Leakage** | Disable clipboard for sensitive data |

---

## 📱 Next Steps

1. **Immediate Fix:** Update CORS and network security config
2. **Short-term:** Implement secure storage for tokens
3. **Medium-term:** Add certificate pinning for production
4. **Long-term:** Implement root detection and anti-tampering

---

**References:**
- [Android Network Security Config](https://developer.android.com/privacy-and-security/security-config)
- [Tauri Android Development](https://v2.tauri.app/develop/develop/android/)
- [Android WebView Security](https://www.securing.pl/en/webview-security-issues-in-android-applications/)
- [OWASP Mobile Security](https://owasp.org/www-project-mobile-security/)
