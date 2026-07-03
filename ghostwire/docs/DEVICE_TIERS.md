# GhostWire Device Tiers Guide

> **Version**: 1.0 | **Status**: Draft | **Last Updated**: 2026-07-01
> **Related**: [Privacy Tiers](./PRIVACY_TIERS.md)

---

## 1. Overview

GhostWire detects device capabilities and adapts behavior accordingly. This affects privacy parameters, cover traffic, and resource usage.

---

## 2. Device Classes

| Class | RAM Threshold | Example Devices |
|-------|--------------|-----------------|
| **Phone** | ≤6,144 MB | Budget smartphones, older phones |
| **Tablet** | ≤8,192 MB | iPads, Android tablets |
| **Raspberry Pi** | ≤4,096 MB | RPi 3, RPi 4, ESP32-class |
| **Laptop** | ≤16,384 MB | Standard laptops, ultrabooks |
| **Server** | >16,384 MB | Desktops, servers, cloud instances |

---

## 3. Detection Method

### 3.1 Auto-Detection

```rust
pub fn detect_device_class() -> DeviceClass {
    let total_ram_mb = get_total_ram_mb();

    if is_raspberry_pi() {
        return DeviceClass::RaspberryPi;
    }

    match total_ram_mb {
        0..=6144 => DeviceClass::Phone,
        6145..=8192 => DeviceClass::Tablet,
        8193..=16384 => DeviceClass::Laptop,
        _ => DeviceClass::Server,
    }
}
```

### 3.2 Raspberry Pi Detection

```rust
fn is_raspberry_pi() -> bool {
    // Check /proc/cpuinfo for Raspberry Pi
    if let Ok(cpuinfo) = std::fs::read_to_string("/proc/cpuinfo") {
        if cpuinfo.contains("Raspberry Pi") || cpuinfo.contains("BCM2") {
            return true;
        }
    }

    // Check device tree
    if let Ok(model) = std::fs::read_to_string("/proc/device-tree/model") {
        if model.contains("Raspberry Pi") {
            return true;
        }
    }

    // Check ARM architecture
    if let Ok(cpuinfo) = std::fs::read_to_string("/proc/cpuinfo") {
        if cpuinfo.contains("ARMv7") || cpuinfo.contains("aarch64") {
            // Additional check for Raspberry Pi specifics
            if let Ok(gpu_mem) = std::fs::read_to_string(
                "/proc/device-tree/soc/gpu/memory"
            ) {
                return true;
            }
        }
    }

    false
}
```

### 3.3 GPU Detection

```rust
fn has_dedicated_gpu() -> bool {
    // Check for NVIDIA GPU
    if std::path::Path::new("/proc/driver/nvidia/version").exists() {
        return true;
    }

    // Check for AMD GPU
    if let Ok(rom) = std::fs::read("/sys/class/drm/card0/device/vendor") {
        if rom == b"0x1002" {  // AMD vendor ID
            return true;
        }
    }

    false
}
```

---

## 4. Capabilities by Class

| Capability | Phone | Tablet | RPi | Laptop | Server |
|-----------|-------|--------|-----|--------|--------|
| **Max RAM** | 6GB | 8GB | 4GB | 16GB | Unlimited |
| **CPU Cores** | 4-8 | 4-8 | 4 | 4-16 | 16+ |
| **GPU** | Integrated | Integrated | None | Integrated/Dedicated | Varies |
| **Battery** | Yes | Yes | No | Yes | No |
| **Network** | WiFi/LTE | WiFi/LTE | WiFi/Ethernet | WiFi/Ethernet | Ethernet |
| **Storage** | 32-256GB | 32-256GB | 8-64GB | 256GB-2TB | 1TB+ |

---

## 5. Resource Limits by Class

| Resource | Phone | Tablet | RPi | Laptop | Server |
|----------|-------|--------|-----|--------|--------|
| **Max Peers** | 20 | 30 | 15 | 50 | 100+ |
| **Max Connections** | 10 | 15 | 8 | 25 | 50+ |
| **Message Queue** | 100 | 150 | 50 | 300 | 500+ |
| **Cover Traffic** | Low | Medium | Low | Medium | High |
| **Storage Limit** | 1GB | 2GB | 500MB | 5GB | 50GB+ |

---

## 6. Privacy Mapping

| Class | Default Privacy | Cover Traffic | Mixing Delay |
|-------|----------------|---------------|--------------|
| **Phone** | L1 (Low) | 10% | 50ms |
| **Tablet** | L2 (Medium) | 30% | 150ms |
| **RPi** | L1 (Low) | 10% | 50ms |
| **Laptop** | L2 (Medium) | 30% | 150ms |
| **Server** | L3 (High) | 50% | 500ms |

See [Privacy Tiers Guide](./PRIVACY_TIERS.md) for full parameter details.

---

## 7. Battery Awareness

### 7.1 Battery States

| State | Condition | Action |
|-------|-----------|--------|
| **Charging** | `battery_status == Charging` | No restrictions |
| **On Battery** | `battery_level > 50%` | Normal operation |
| **Low Battery** | `20% < battery_level <= 50%` | Reduce cover traffic by 30% |
| **Critical** | `battery_level <= 20%` | Reduce to L0/L1, minimal cover |

### 7.2 OEM-Specific Handling

GhostWire supports battery reporting for major OEMs:

| OEM | Detection Method |
|-----|-----------------|
| Samsung | `/sys/class/power_supply/battery/` |
| Xiaomi | `/sys/class/power_supply/battery/` + MIUI specific |
| Huawei | `/sys/class/power_supply/battery/` + HiSilicon |
| OPPO | `/sys/class/power_supply/battery/` + VOOC |
| Vivo | `/sys/class/power_supply/battery/` + FlashCharge |
| Transsion | `/sys/class/power_supply/battery/` (basic) |

### 7.3 Battery Severity

| Severity | Condition | Cover Traffic | Mixing |
|----------|-----------|---------------|--------|
| **Standard** | >50% battery | Normal | Normal |
| **Moderate** | 20-50% battery | -30% | -20% |
| **Aggressive** | <20% battery | Minimal | Minimal |

---

## 8. Configuration

### 8.1 Manual Override

```toml
# config.toml
[device]
auto_detect = true
# Override detected class:
# device_class = "Phone"

[privacy]
# Override default privacy level:
# level = "Medium"
```

### 8.2 Environment Variables

```bash
GHOSTWIRE_DEVICE_CLASS=Phone
GHOSTWIRE_PRIVACY_LEVEL=Low
GHOSTWIRE_AUTO_DETECT=false
```

---

## 9. References

- [GhostWire Source: ai/function_calling.rs](../src/core/ai/function_calling.rs#L800-L859)
- [GhostWire Source: oem_battery_guide.rs](../src/core/oem_battery_guide.rs)
- [Privacy Tiers Guide](./PRIVACY_TIERS.md)
