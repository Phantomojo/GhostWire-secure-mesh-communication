# Complete Android App Development Guide for GhostWire

**Date:** February 28, 2026  
**Status:** Comprehensive Research Complete

---

## 📚 **Deep Dive Research Summary**

### 1. Tauri v2 Android Development

#### Requirements:
- **Java:** JDK 17 or 21 (NOT 23+)
- **Gradle:** 8.5.2+
- **Android SDK:** API 24+ (Android 7.0+)
- **NDK:** r27+

#### Build Commands:
```bash
# Initialize (one-time)
cargo tauri android init

# Development build
cargo tauri android dev

# Production build
cargo tauri android build

# Install on device
cargo tauri android adb install
```

#### Common Issues & Fixes:

| Issue | Solution |
|-------|----------|
| `Unsupported class file major version` | Use Java 17 or 21, not 23+ |
| `JAVA_HOME not set` | `export JAVA_HOME=/usr/lib/jvm/java-17-openjdk` |
| `Android SDK not found` | `export ANDROID_HOME=$HOME/Android/Sdk` |
| `Gradle build failed` | `./gradlew clean` then rebuild |

---

### 2. Mesh Networking Apps (Briar, Bitchat, etc.)

#### Briar Messenger Architecture:

**Transport Layers:**
- **Bluetooth LE** - Short range, low power
- **Wi-Fi Direct** - Medium range, higher bandwidth  
- **Tor Network** - Long range via internet
- **Wi-Fi Aware** - Android-specific P2P

**Security:**
- End-to-end encryption (Double Ratchet protocol)
- No metadata storage
- Independent security audit (Cure53)
- Open source (FOSS)

**Android Permissions Required:**
```xml
<!-- Core -->
<uses-permission android:name="android.permission.INTERNET" />
<uses-permission android:name="android.permission.ACCESS_NETWORK_STATE" />

<!-- Bluetooth -->
<uses-permission android:name="android.permission.BLUETOOTH" />
<uses-permission android:name="android.permission.BLUETOOTH_ADMIN" />
<uses-permission android:name="android.permission.BLUETOOTH_CONNECT" />
<uses-permission android:name="android.permission.BLUETOOTH_SCAN" />
<uses-permission android:name="android.permission.BLUETOOTH_ADVERTISE" />

<!-- Wi-Fi -->
<uses-permission android:name="android.permission.CHANGE_WIFI_STATE" />
<uses-permission android:name="android.permission.ACCESS_WIFI_STATE" />
<uses-permission android:name="android.permission.NEARBY_WIFI_DEVICES" />

<!-- Location (for Bluetooth scanning) -->
<uses-permission android:name="android.permission.ACCESS_FINE_LOCATION" />
<uses-permission android:name="android.permission.ACCESS_COARSE_LOCATION" />

<!-- Foreground Service (for background messaging) -->
<uses-permission android:name="android.permission.FOREGROUND_SERVICE" />
<uses-permission android:name="android.permission.WAKE_LOCK" />

<!-- Notifications -->
<uses-permission android:name="android.permission.POST_NOTIFICATIONS" />
```

**Key Android Components:**
- `Service` - Background message relay
- `BroadcastReceiver` - Network state changes
- `JobScheduler` - Deferred message delivery
- `EncryptedSharedPreferences` - Secure storage

---

### 3. Android Security Best Practices

#### Network Security:
```xml
<!-- res/xml/network_security_config.xml -->
<network-security-config>
    <!-- Production: HTTPS only -->
    <base-config cleartextTrafficPermitted="false">
        <trust-anchors>
            <certificates src="system" />
        </trust-anchors>
    </base-config>
    
    <!-- Debug: Allow localhost -->
    <debug-overrides>
        <trust-anchors>
            <certificates src="user" />
        </trust-anchors>
        <domain-config cleartextTrafficPermitted="true">
            <domain includeSubdomains="true">localhost</domain>
            <domain includeSubdomains="true">127.0.0.1</domain>
        </domain-config>
    </debug-overrides>
</network-security-config>
```

#### Certificate Pinning:
```xml
<!-- res/raw/certificate.pem (your backend cert) -->
<network-security-config>
    <domain-config>
        <domain includeSubdomains="true">ghostwire.app</domain>
        <pin-set expiration="2027-01-01">
            <pin digest="SHA-256">BASE64_CERT_HASH</pin>
        </pin-set>
    </domain-config>
</network-security-config>
```

#### ProGuard Rules:
```proguard
# Keep Tauri classes
-keep class org.ghostwire.** { *; }
-keep class net.java.dev.jna.** { *; }

# Keep serialization
-keepclassmembers class * {
    @com.google.gson.annotations.SerializedName <fields>;
}

# Keep native methods
-keepclasseswithmembernames class * {
    native <methods>;
}
```

---

### 4. Privacy & Anonymity Enhancements

#### Data Minimization:
- Request only necessary permissions
- Clear data on logout
- No analytics/telemetry by default
- Local-first architecture

#### Anti-Fingerprinting:
- Randomize device identifiers
- Don't expose unnecessary system info
- Use Tor for metadata protection

#### Secure Storage:
```kotlin
// Android Keystore for encryption keys
val keyStore = KeyStore.getInstance("AndroidKeyStore")
keyStore.load(null)

// EncryptedSharedPreferences
val masterKey = MasterKey.Builder(context)
    .setKeyScheme(MasterKey.KeyScheme.AES256_GCM)
    .build()

val prefs = EncryptedSharedPreferences.create(
    context,
    "ghostwire_prefs",
    masterKey,
    EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
    EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM
)
```

#### Screen Security:
```kotlin
// Prevent screenshots
window.setFlags(
    WindowManager.LayoutParams.FLAG_SECURE,
    WindowManager.LayoutParams.FLAG_SECURE
)

// Prevent app switcher preview
ActivityManager.TaskDescription(
    null, // No thumbnail
    null,
    0 // No color
)
```

---

### 5. Production Deployment Checklist

#### Pre-Build:
- [ ] Set correct Java version (17 or 21)
- [ ] Configure Android SDK/NDK paths
- [ ] Update version code/name in `build.gradle`
- [ ] Configure signing keys

#### Build:
```bash
# Debug (for testing)
cargo tauri android build --debug

# Release (for production)
cargo tauri android build

# Sign release APK
apksigner sign --ks ghostwire.keystore app-release-unsigned.apk
```

#### Post-Build:
- [ ] Test on multiple Android versions (7.0 - 14)
- [ ] Verify all permissions work
- [ ] Test offline functionality
- [ ] Check battery usage
- [ ] Verify data cleanup on uninstall

#### Distribution:
- [ ] Sign with release key
- [ ] Generate release notes
- [ ] Prepare store listing
- [ ] Create privacy policy
- [ ] Submit to F-Droid / Play Store

---

### 6. GhostWire-Specific Implementations

#### Mesh Transport:
```rust
// src/core/mesh.rs
#[cfg(target_os = "android")]
use jni::JNIEnv;

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_org_ghostwire_messenger_MeshTransport_initMesh(
    env: JNIEnv,
    _class: JClass,
) {
    // Initialize mesh networking for Android
    // Use Wi-Fi Direct + Bluetooth LE
}
```

#### Android Permissions Handler:
```kotlin
// MainActivity.kt
private val requiredPermissions = arrayOf(
    Manifest.permission.INTERNET,
    Manifest.permission.BLUETOOTH,
    Manifest.permission.BLUETOOTH_CONNECT,
    Manifest.permission.CHANGE_WIFI_STATE,
)

private fun requestPermissions() {
    ActivityCompat.requestPermissions(
        this,
        requiredPermissions,
        REQUEST_PERMISSIONS
    )
}
```

#### Background Service:
```kotlin
// MeshService.kt
class MeshService : Service() {
    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        // Start mesh networking in background
        startForeground(NOTIFICATION_ID, createNotification())
        startMeshNetwork()
        return START_STICKY
    }
}
```

---

### 7. Testing Strategy

#### Unit Tests:
```kotlin
// Local unit tests (JVM)
@Test
fun testMessageEncryption() {
    val message = "Hello"
    val encrypted = encrypt(message, key)
    val decrypted = decrypt(encrypted, key)
    assertEquals(message, decrypted)
}
```

#### Integration Tests:
```kotlin
// Android instrumented tests
@RunWith(AndroidJUnit4::class)
class MeshNetworkTest {
    @Test
    fun testPeerDiscovery() {
        // Test Bluetooth/Wi-Fi peer discovery
    }
}
```

#### E2E Tests:
```bash
# Using Maestro or Appium
maestro test e2e/profile_creation.yaml
maestro test e2e/message_send.yaml
```

---

### 8. Performance Optimization

#### Battery:
- Use WorkManager for deferred tasks
- Batch network operations
- Minimize wake locks
- Use Doze mode compatibility

#### Memory:
- Limit message history cache
- Use pagination for large lists
- Clear WebView cache regularly
- Monitor with Android Profiler

#### Network:
- Compress messages before sending
- Use binary protocols where possible
- Implement message deduplication
- Cache frequently accessed data

---

## 🎯 **Next Steps for GhostWire Android**

### Immediate:
1. ✅ Install JDK 17
2. ⏳ Build Android APK
3. ⏳ Install on device
4. ⏳ Test profile creation

### Short-Term:
1. Add mesh networking permissions
2. Implement background service
3. Add Wi-Fi Direct support
4. Implement Bluetooth LE transport

### Medium-Term:
1. Certificate pinning
2. ProGuard obfuscation
3. Secure key storage
4. Root detection

### Long-Term:
1. Tor integration
2. Multi-device sync
3. Group messaging
4. Voice/video calls

---

## 📖 **References**

- [Tauri Android Docs](https://v2.tauri.app/start/prerequisites/#android)
- [Briar Source](https://code.briarproject.org/briar/briar)
- [Android Security Config](https://developer.android.com/privacy-and-security/security-config)
- [OWASP Mobile Top 10](https://owasp.org/www-project-mobile-top-10/)
- [Android Developers Guide](https://developer.android.com/guide)

---

**Status:** Ready for implementation  
**Confidence:** High - Based on proven patterns from Briar, Bitchat, etc.  
**Security:** Following Android best practices + mesh networking lessons
