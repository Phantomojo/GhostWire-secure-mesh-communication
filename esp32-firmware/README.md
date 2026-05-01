# GhostWire ESP32 Firmware

Secure mesh communication firmware for ESP32 nodes in the GhostWire network.

## Project Structure

```
esp32-firmware/
├── src/
│   └── main.cpp          # Main firmware source
├── include/
│   └── config.h          # Configuration headers
├── lib/                  # Custom libraries
├── test/                 # Unit tests
├── platformio.ini        # PlatformIO configuration
└── README.md            # This file
```

## Current Status

### ✅ Phase 1: Setup & Verification
- [x] PlatformIO installation
- [x] Project initialization
- [x] Blink test firmware
- [ ] Flash to ESP32
- [ ] Verify serial output

### 🔜 Phase 2: WiFi & ESP-NOW
- [ ] WiFi station mode
- [ ] WiFi AP mode
- [ ] ESP-NOW initialization
- [ ] Peer-to-peer communication
- [ ] Encrypted ESP-NOW

### 🔜 Phase 3: Mesh Networking
- [ ] Peer discovery
- [ ] Routing table
- [ ] Message forwarding
- [ ] Self-healing topology
- [ ] Multi-hop communication

### 🔜 Phase 4: GhostWire Integration
- [ ] GhostWire protocol bridge
- [ ] Encryption layer
- [ ] DTN store-and-forward
- [ ] BLE gateway
- [ ] libp2p integration

## Quick Start

### Build Firmware
```bash
cd esp32-firmware
pio run
```

### Flash to ESP32
```bash
pio run --target upload
```

### Monitor Serial Output
```bash
pio device monitor
```

### Build, Flash & Monitor (All-in-One)
```bash
pio run --target upload && pio device monitor
```

## Hardware

- **Board:** ESP32-D0WD-V3 (rev 3.1)
- **MAC:** 20:e7:c8:5a:68:18
- **Flash:** 4MB
- **USB-Serial:** CH340
- **Port:** /dev/ttyUSB0

## Troubleshooting

### Upload Failed
1. Hold BOOT button on ESP32 while uploading
2. Check USB connection: `lsusb | grep 1a86`
3. Check permissions: `ls -la /dev/ttyUSB0`
4. Try lower baud rate in platformio.ini

### Serial Monitor Shows Nothing
1. Check baud rate matches monitor_speed in platformio.ini
2. Press EN (reset) button on ESP32
3. Check USBGuard: `usbguard list-devices`

### Build Errors
1. Clean build: `pio run --target clean`
2. Rebuild: `pio run`

## Development Workflow

1. Edit `src/main.cpp`
2. Build: `pio run`
3. Flash: `pio run --target upload`
4. Monitor: `pio device monitor`
5. Repeat

## License

Part of GhostWire secure mesh communication project.
See main project LICENSE.
