# ESP32 Setup & Capabilities Guide

## Device Information

**Chip:** ESP32-D0WD-V3 (revision v3.1)  
**MAC Address:** 20:e7:c8:5a:68:18  
**Flash:** 4MB (Manufacturer: 68, Device: 4016)  
**Crystal:** 40MHz  
**Features:** 
- ✅ Wi-Fi (2.4 GHz)
- ✅ Bluetooth Classic + BLE
- ✅ Dual Core + Low Power Core
- ✅ 240MHz CPU
- ✅ Vref calibration in eFuse

**USB-Serial:** CH340 (Vendor: 1a86, Product: 7523)  
**Port:** `/dev/ttyUSB0`  
**USBGuard Status:** Allowed (Device ID 24)

---

## USBGuard Issue (RESOLVED)

**Problem:** Device initially blocked by USBGuard
```
[  393.254111] usb 3-2: Device is not authorized for usage
```

**Solution:** USBGuard rule already exists (device 24):
```
allow id 1a86:7523 serial "" name "USB Serial" hash "Y4crZ93bH5ULGhbzur95S8NvmqO0Ir3Altz0ZLZ1dO4="
```

**If blocked again:**
```bash
# Check device status
usbguard list-devices | grep 1a86

# Allow device (replace ID with actual)
usbguard allow-device <ID>

# Or create permanent rule
usbguard allow-device --permanent <ID>
```

---

## ESP32-D0WD-V3 Capabilities

### Processing Power
- **Dual-core Xtensa LX6** @ 240MHz
- **Low-power coprocessor** (ULP) for deep sleep tasks
- **520KB SRAM** (internal)
- **4MB External Flash** (storage for code/data)
- **448KB ROM** (bootloader, basic functions)

### Wireless Communication
1. **Wi-Fi 802.11 b/g/n (2.4 GHz)**
   - Station mode (connect to AP)
   - AP mode (create hotspot)
   - AP+STA mode (simultaneous)
   - Wi-Fi Direct (P2P)
   - Promiscuous mode (packet monitoring)

2. **Bluetooth v4.2 BR/EDR + BLE**
   - Classic Bluetooth (SPP, A2DP, etc.)
   - Bluetooth Low Energy (BLE)
   - Dual-mode operation

3. **ESP-NOW** (Critical for GhostWire)
   - Connectionless protocol
   - <10ms latency
   - Up to 250 bytes payload
   - Encrypted (AES-CCMP)
   - Up to 20 encrypted peers
   - No WiFi infrastructure needed

### Hardware Interfaces
- **GPIO:** 34 pins (configurable)
- **ADC:** 18 channels (12-bit SAR ADC)
- **DAC:** 2 channels (8-bit)
- **PWM:** 16 channels
- **I2C:** 2 interfaces
- **SPI:** 4 interfaces
- **UART:** 3 interfaces
- **I2S:** 2 interfaces
- **CAN:** 1 interface
- **Ethernet MAC:** 1 interface
- **Touch Sensors:** 10 capacitive
- **Hall Effect Sensor:** Built-in

### Security Features
- **Hardware Acceleration:**
  - AES-128/256
  - RSA
  - ECC
  - SHA-2 (SHA-224/256/384/512)
  - HMAC
- **Secure Boot:** v2 (RSA-3072)
- **Flash Encryption:** AES-128 XTS
- **Digital Signature Peripheral**
- **World Controller** (isolated execution)
- **Random Number Generator:** Hardware TRNG

### Power Management
- **Active Mode:** ~240mA @ 240MHz
- **Modem Sleep:** ~20mA
- **Light Sleep:** ~0.8mA
- **Deep Sleep:** ~10μA (ULP running)
- **Hibernation:** ~5μA

---

## GhostWire Integration Opportunities

### 1. ESP-NOW Mesh Network (Priority 1)
**Use Case:** Decentralized message relay without internet/WiFi infrastructure

**Architecture:**
```
[ESP32 Node 1] ←ESP-NOW→ [ESP32 Node 2] ←ESP-NOW→ [ESP32 Node 3]
      ↓                        ↓                        ↓
  Sensor Data            Message Relay            Gateway to Internet
```

**Benefits:**
- No router needed
- Self-healing topology
- <10ms per hop latency
- Encrypted peer-to-peer
- Range: ~100-300m (line of sight)

**Implementation:**
- PlatformIO + Arduino ESP-NOW API
- Custom routing protocol (AODV/OLSR)
- Integrate with GhostWire libp2p layer

### 2. BLE Proximity Network
**Use Case:** Device discovery, presence detection, secure pairing

**Applications:**
- Physical device authentication
- Proximity-based message delivery
- BLE beacon mesh for location tracking

### 3. Sensor Network Layer
**Use Case:** Physical environment monitoring for GhostWire nodes

**Sensors to Integrate:**
- Temperature/humidity (DHT22, BME280)
- Air quality (MQ-135, PMS5003)
- Motion (PIR, accelerometer)
- RF spectrum analysis (promiscuous mode)

### 4. DTN (Delay-Tolerant Networking)
**Use Case:** Store-and-forward messaging in disconnected areas

**ESP32 Role:**
- Store encrypted GhostWire messages in flash
- Forward when peer detected (ESP-NOW/BLE)
- Opportunistic sync when internet available

### 5. Physical Security Module
**Use Case:** Hardware security for GhostWire key storage

**Features:**
- Secure boot + flash encryption
- Hardware AES/RSA acceleration
- Tamper detection (touch sensors, hall effect)
- Physical unclonable functions (PUF) via ADC noise

---

## Development Setup

### Option A: PlatformIO (Recommended)
```bash
# Install PlatformIO
pip install platformio --break-system-packages

# Create project
mkdir esp32-ghostwire && cd esp32-ghostwire
pio init --board esp32dev

# Project structure
esp32-ghostwire/
├── src/
│   └── main.cpp          # Main firmware
├── include/
│   └── config.h          # Configuration
├── lib/                  # Custom libraries
├── platformio.ini        # Build config
└── test/                 # Unit tests
```

**platformio.ini:**
```ini
[env:esp32dev]
platform = espressif32
board = esp32dev
framework = arduino
monitor_speed = 115200
lib_deps = 
    ESPNow
    ArduinoJson
    Crypto
build_flags = 
    -DCORE_DEBUG_LEVEL=3
    -DCONFIG_ESPNOW_ENABLE_LONG_RANGE=y
```

### Option B: esp-rs (Rust Native)
```bash
# Install esp-rs toolchain
rustup target add xtensa-esp32-none-elf
cargo install cargo-espflash
cargo install cargo-espmonitor

# Generate project
cargo generate esp-rs/esp-generate

# Build and flash
cargo espflash flash --monitor --port /dev/ttyUSB0
```

**Cargo.toml:**
```toml
[package]
name = "ghostwire-esp32"
version = "0.1.0"
edition = "2021"

[dependencies]
esp32 = "0.1.0"
esp-idf-sys = "0.34"
esp-now = "0.1.0"
crypto = "0.5.0"
```

### Option C: ESP-IDF (Espressif Official)
```bash
# Install ESP-IDF
git clone --recursive https://github.com/espressif/esp-idf.git
cd esp-idf
./install.sh esp32
. ./export.sh

# Create project
idf.py create-project ghostwire-node
cd ghostwire-node
idf.py set-target esp32
idf.py build flash monitor
```

---

## Quick Start Projects

### Project 1: Blink Test (Verify Setup)
**Goal:** Confirm toolchain and board work

```cpp
// src/main.cpp
void setup() {
  pinMode(LED_BUILTIN, OUTPUT);
}

void loop() {
  digitalWrite(LED_BUILTIN, HIGH);
  delay(1000);
  digitalWrite(LED_BUILTIN, LOW);
  delay(1000);
}
```

**Build & Flash:**
```bash
pio run --target upload
pio device monitor
```

### Project 2: ESP-NOW Peer Communication
**Goal:** Two ESP32s talk directly (no WiFi)

**Sender:**
```cpp
#include <esp_now.h>
#include <WiFi.h>

uint8_t peerMAC[] = {0xXX, 0xXX, 0xXX, 0xXX, 0xXX, 0xXX}; // Receiver MAC

void setup() {
  WiFi.mode(WIFI_STA);
  esp_now_init();
  
  esp_now_peer_info_t peerInfo = {};
  memcpy(peerInfo.peer_addr, peerMAC, 6);
  esp_now_add_peer(&peerInfo);
}

void loop() {
  const char* msg = "GhostWire Node Active";
  esp_now_send(peerMAC, (uint8_t*)msg, strlen(msg));
  delay(2000);
}
```

### Project 3: ESP-NOW Mesh Router
**Goal:** Multi-hop message forwarding

**Features:**
- Discover peers via promiscuous mode
- Maintain routing table
- Forward messages to destination
- TTL to prevent loops

### Project 4: BLE + ESP-NOW Bridge
**Goal:** BLE devices communicate via ESP-NOW mesh

**Architecture:**
```
[Phone BLE] → [ESP32 Gateway] ←ESP-NOW→ [Mesh Network]
```

---

## Security Hardening for GhostWire

### 1. Flash Encryption
```bash
# Generate encryption key
espsecure.py generate_flash_encryption_key key.bin

# Burn key to eFuse (ONE-TIME, IRREVERSIBLE)
espefuse.py burn_key flash_encryption key.bin

# Enable flash encryption
espefuse.py burn_efuse FLASH_CRYPT_CONFIG 0xF
```

### 2. Secure Boot v2
```bash
# Generate signing key
espsecure.py generate_signing_key secure_boot_v2.key

# Sign bootloader
espsecure.py sign_data --keyfile secure_boot_v2.key --version 2 bootloader.bin

# Burn secure boot key (ONE-TIME)
espefuse.py burn_key secure_boot secure_boot_v2.key
```

### 3. ESP-NOW Encryption
```cpp
// Set encryption key (must match on all nodes)
uint8_t encrypt_key[16] = { /* 128-bit key */ };
esp_now_set_pmk(encrypt_key);

// Add encrypted peer
esp_now_peer_info_t peer = {};
peer.encrypt = true;
memcpy(peer.lmk, encrypt_key, 16);
esp_now_add_peer(&peer);
```

---

## Testing Checklist

- [ ] **Blink test** - Verify toolchain
- [ ] **Serial monitor** - Debug output works
- [ ] **WiFi scan** - Detect networks
- [ ] **ESP-NOW send/receive** - Peer communication
- [ ] **BLE advertise/scan** - BLE functionality
- [ ] **Deep sleep wakeup** - Power management
- [ ] **Flash read/write** - Persistent storage
- [ ] **ADC sensor reading** - Analog input
- [ ] **GPIO interrupt** - Event handling

---

## Resources

### Official Documentation
- [ESP32 Technical Reference Manual](https://www.espressif.com/sites/default/files/documentation/esp32_technical_reference_manual_en.pdf)
- [ESP-IDF Programming Guide](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/)
- [ESP-NOW API Guide](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/network/esp_now.html)

### Rust ESP
- [esp-rs Book](https://esp-rs.github.io/book/)
- [esp-idf-sys](https://github.com/esp-rs/esp-idf-sys)
- [esp32 crate](https://crates.io/crates/esp32)

### Mesh Networking
- [PainlessMesh Library](https://github.com/gmag11/painlessMesh)
- [ESP-MESH (Official)](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/network/esp_mesh.html)
- [ESP-NOW Mesh Examples](https://github.com/esp-now)

### Security
- [ESP32 Security Features](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/security/index.html)
- [Flash Encryption Guide](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/security/flash-encryption.html)
- [Secure Boot Guide](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/security/secure-boot-v2.html)

---

## Next Steps for GhostWire Integration

1. **Install PlatformIO** and run blink test
2. **Get second ESP32** for ESP-NOW testing
3. **Design mesh routing protocol** (AODV/OLSR adaptation)
4. **Integrate with GhostWire libp2p** (bridge ESP-NOW to libp2p)
5. **Implement encryption layer** (AES-CCMP + GhostWire crypto)
6. **Build sensor node prototype** (environmental monitoring)
7. **Create BLE gateway** (phone-to-mesh bridge)
8. **Test range and reliability** (urban vs open field)

---

## Troubleshooting

### Device Not Detected
```bash
# Check USB connection
lsusb | grep 1a86

# Check kernel messages
dmesg | tail -20

# Check USBGuard
usbguard list-devices | grep 1a86
usbguard allow-device <ID>
```

### Permission Denied
```bash
# Add user to uucp group
usermod -aG uucp $USER
# Logout and login again

# Or use sudo (temporary)
sudo esptool --port /dev/ttyUSB0 chip-id
```

### Upload Failed
- Hold BOOT button while uploading
- Check wiring (GPIO0 must be LOW during boot)
- Try lower baud rate: `upload_speed = 115200`

### ESP-NOW Not Working
- Both devices must be on same WiFi channel
- Initialize WiFi before ESP-NOW
- Check peer MAC address format (little-endian)

---

**Last Updated:** 2026-04-12  
**Device MAC:** 20:e7:c8:5a:68:18  
**Status:** ✅ Connected and Ready for Development
