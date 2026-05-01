# GhostWire Hardware Specification & BOM Document
**Project:** GhostWire Secure Mesh Communication  
**Competition:** GCD4F 2026 (Global Competition on Design for Futures)  
**Document Version:** 0.1.0  
**Date:** 2026-04-10  
**Author:** [Your Name/Team Member]  
**Status:** Draft

---

## 📋 Table of Contents
1. [Hardware Tier Overview](#hardware-tier-overview)
2. [Tier 0: Lifeboat Node](#tier-0-lifeboat-node)
3. [Tier 1: Router Node](#tier-1-router-node)
4. [Tier 2: Gateway Node](#tier-2-gateway-node)
5. [Tier 3: Concierge/Ark Node](#tier-3-conciergeark-node)
6. [Bill of Materials (BOM)](#bill-of-materials-bom)
7. [Power Requirements](#power-requirements)
8. [Environmental Specifications](#environmental-specifications)
9. [Certification & Compliance](#certification--compliance)
10. [Assembly Notes](#assembly-notes)

---

## Hardware Tier Overview

GhostWire supports **4 hardware tiers** to enable deployment from emergency scenarios to full-scale infrastructure:

| Tier | Name | Primary Use | Target BOM | Key Transports |
|------|------|-------------|------------|----------------|
| **Tier 0** | Lifeboat | Emergency/Minimal | <$20 | LoRa only |
| **Tier 1** | Router | Community mesh | $50 | WiFi + BLE |
| **Tier 2** | Gateway | Mesh↔Internet bridge | $100 | All + Ethernet |
| **Tier 3** | Concierge/Ark | Full mesh + archive | $200+ | Server-class |

---

## Tier 0: Lifeboat Node

### Purpose
Emergency communication, minimal viable mesh node, extreme low-power deployment

### Core Specifications
- **MCU:** ESP32-S3 (dual-core Xtensa LX7, 240MHz)
- **RAM:** 512KB SRAM + 8MB PSRAM (minimum 64MB usable)
- **Storage:** 4MB Flash (firmware + config)
- **Radio:** Semtech SX1262 LoRa (868/915 MHz, 22dBm TX)
- **Power:** Coin cell CR2032 or single 18650 Li-ion
- **Operating Voltage:** 3.3V
- **Current Draw:** 
  - Sleep: <10μA
  - TX (22dBm): ~120mA
  - RX: ~15mA
- **Antenna:** 868/915 MHz LoRa antenna (2.15dBi whip)
- **Enclosure:** IP67 waterproof project box (100x60x40mm)
- **Operating Temp:** -20°C to +60°C

### Firmware Requirements
- GhostWire Core (Rust, compiled for ESP32)
- PRoPHET routing (L4 fallback only)
- LoRa packet forwarding
- Basic encryption (AES-128)

### Limitations
- No AI routing (L1-L3 unavailable)
- PRoPHET mathematical routing only
- Store-and-forward messaging (no real-time)
- Max payload: 240 bytes per packet

---

## Tier 1: Router Node

### Purpose
Community mesh backbone, neighborhood-level coverage, old smartphone repurposing

### Core Specifications
- **SBC:** Raspberry Pi Zero 2W (or equivalent)
- **SoC:** Broadcom BCM2710A1 (ARM Cortex-A53 quad-core, 1GHz)
- **RAM:** 512MB LPDDR2
- **Storage:** 16GB microSD (minimum), 32GB recommended
- **Wireless:** 
  - WiFi 802.11b/g/n (2.4GHz, onboard)
  - Bluetooth 4.2 + BLE (onboard)
- **USB:** 1x USB 2.0 OTG (for external adapters)
- **Power:** 5V/1.2A via micro-USB
- **Current Draw:** ~150-200mA typical
- **Antenna:** Onboard ceramic antenna (external u.FL connector available)
- **Enclosure:** Weatherproof case with external antenna mounts
- **Operating Temp:** -10°C to +50°C

### Optional Add-ons
- **LoRa HAT:** Dragino LoRa HAT for SX1276 (extends to Tier 0 compatibility)
- **External WiFi Antenna:** 5dBi omnidirectional (extends range to ~500m)
- **Battery Backup:** 5000mAh power bank (8+ hours runtime)

### AI Capabilities
- **L1 LightGBM:** ONNX runtime (sub-ms routing decisions)
- **L1b Anomaly Detection:** Isolation Forest (mesh-domain security)
- **L2 Gemma 4:** Via Ollama (requires external server or quantized e2b model)
- **L3 GNN:** Not supported (insufficient RAM)
- **L4 PRoPHET:** Always available as fallback

### Performance Targets
- Mesh hops: 5-10 nodes
- Latency: <50ms per hop
- Throughput: 5-15 Mbps (WiFi direct)
- Coverage: 100-200m radius (urban), 500m (line-of-sight)

---

## Tier 2: Gateway Node

### Purpose
Internet bridge, mesh backbone, neighborhood hub

### Core Specifications
- **SBC:** Raspberry Pi 4 Model B (or equivalent)
- **SoC:** Broadcom BCM2711 (ARM Cortex-A72 quad-core, 1.5GHz)
- **RAM:** 2GB LPDDR4 (4GB recommended)
- **Storage:** 64GB microSD or 128GB USB 3.0 SSD
- **Ethernet:** Gigabit Ethernet (realtek RTL8153 via USB 3.0)
- **Wireless:** 
  - WiFi 802.11b/g/n/ac (2.4GHz + 5GHz, dual-band)
  - Bluetooth 5.0 + BLE
- **USB:** 2x USB 3.0, 2x USB 2.0
- **GPIO:** 40-pin header (for LoRa HAT, sensors)
- **Power:** 5V/3A USB-C
- **Current Draw:** ~600-800mA typical (under load)
- **Enclosure:** Aluminum case with heatsinks + fan (optional)
- **Operating Temp:** 0°C to +50°C (with active cooling)

### Required Peripherals
- **LoRa HAT:** Dragino LoRa HAT v1.4 (SX1276, 868/915 MHz)
- **External WiFi Antenna:** Dual-band 5dBi (2.4GHz + 5GHz)
- **UPS Module:** Waveshare UPS HAT (battery backup + graceful shutdown)
- **Enclosure:** IP65 outdoor-rated with PoE injector

### AI Capabilities
- **L1 LightGBM:** Full ONNX runtime (production-ready)
- **L1b Anomaly Detection:** Isolation Forest + custom models
- **L2 Gemma 4:** Ollama serving (gemma4:e2b, 2B params)
- **L3 GNN:** Graph Neural Network (PyTorch Geometric)
- **L4 PRoPHET:** Always available as fallback
- **Mesh Memory:** 5-minute rolling window (RSSI + latency trends)
- **Sybil Detection:** Signal strength variance analysis

### Performance Targets
- Mesh hops: 15-20 nodes
- Latency: <20ms per hop
- Throughput: 50-100 Mbps (WiFi AC), 1000 Mbps (Ethernet)
- Coverage: 500m-1km radius (with external antennas)
- Connected devices: 50-100 concurrent clients

---

## Tier 3: Concierge/Ark Node

### Purpose
Full mesh infrastructure, archive server, community hub

### Core Specifications
- **Server:** Custom build or enterprise-grade (e.g., Dell R740, HP DL380)
- **CPU:** Intel Xeon Silver 4210R (10-core, 2.4GHz base) or AMD EPYC 7302P (16-core)
- **RAM:** 32GB DDR4 ECC (64GB+ recommended for large meshes)
- **Storage:** 
  - OS: 512GB NVMe SSD
  - Archive: 4TB+ HDD (Wikipedia, offline content)
  - Backup: RAID 1 or RAID 5 configuration
- **Network:** 
  - 2x Gigabit Ethernet (LAN + WAN)
  - 1x 10GbE SFP+ (optional, for backbone links)
  - WiFi 6E AP (for local mesh access)
- **GPU:** NVIDIA RTX 4060 Ti 16GB (for AI training/inference)
- **Power:** 500W 80+ Gold PSU + UPS
- **Enclosure:** 2U rackmount or tower server
- **Operating Temp:** 15°C to +35°C (climate-controlled room)

### AI Capabilities
- **All L1-L4:** Full stack operational
- **L2 Gemma 4:** Full model (not quantized), 7B+ params
- **L3 GNN:** Training + inference (PyTorch + DGL)
- **L4 PRoPHET:** Optimized for large-scale topologies
- **Mesh Memory:** Extended window (30+ minutes)
- **Anomaly Detection:** Real-time, multi-modal analysis
- **Model Training:** On-device fine-tuning with local data

### Archive Services (Ark)
- **Wikipedia Offline:** 50GB+ compressed dump
- **OpenStreetMap:** Regional maps (vector + raster)
- **Medical Reference:** Offline medical databases
- **Educational Content:** Khan Academy, Coursera offline
- **Emergency Docs:** First aid, survival guides

### Performance Targets
- Mesh hops: 50+ nodes
- Latency: <10ms per hop
- Throughput: 1000+ Mbps (10GbE backbone)
- Connected devices: 500-1000 concurrent clients
- Archive queries: <100ms response time

---

## Bill of Materials (BOM)

### Tier 0: Lifeboat (<$20)

| Component | Part Number | Supplier | Qty | Unit Price | Total |
|-----------|-------------|----------|-----|------------|-------|
| ESP32-S3 DevKit | ESP32-S3-WROOM-1-N8R8 | Mouser/DigiKey | 1 | $6.50 | $6.50 |
| LoRa Module | Semtech SX1262MBK1CAS | Mouser | 1 | $8.00 | $8.00 |
| Antenna | 868/915 MHz 2.15dBi | AliExpress | 1 | $1.50 | $1.50 |
| Enclosure | IP67 Project Box 100x60x40 | AliExpress | 1 | $2.00 | $2.00 |
| Battery | 18650 Li-ion 2600mAh | Local | 1 | $2.00 | $2.00 |
| **Total** | | | | | **$20.00** |

### Tier 1: Router (~$50)

| Component | Part Number | Supplier | Qty | Unit Price | Total |
|-----------|-------------|----------|-----|------------|-------|
| Raspberry Pi Zero 2W | SC0190 | Raspberry Pi | 1 | $15.00 | $15.00 |
| MicroSD Card | SanDisk 32GB A1 | Amazon | 1 | $6.00 | $6.00 |
| LoRa HAT | Dragino LoRa HAT v1.4 | Dragino | 1 | $12.00 | $12.00 |
| WiFi Antenna | 5dBi 2.4GHz + pigtail | AliExpress | 1 | $4.00 | $4.00 |
| Enclosure | Weatherproof case | AliExpress | 1 | $5.00 | $5.00 |
| Power Supply | 5V 2A micro-USB | Local | 1 | $3.00 | $3.00 |
| Cables/Connectors | Assorted | Local | 1 | $5.00 | $5.00 |
| **Total** | | | | | **$50.00** |

### Tier 2: Gateway (~$100)

| Component | Part Number | Supplier | Qty | Unit Price | Total |
|-----------|-------------|----------|-----|------------|-------|
| Raspberry Pi 4B 2GB | SC0230 | Raspberry Pi | 1 | $35.00 | $35.00 |
| MicroSD/SSD | SanDisk 64GB A2 / 128GB USB SSD | Amazon | 1 | $15.00 | $15.00 |
| LoRa HAT | Dragino LoRa HAT v1.4 | Dragino | 1 | $12.00 | $12.00 |
| WiFi Antenna | Dual-band 5dBi + pigtails | AliExpress | 1 | $8.00 | $8.00 |
| Enclosure | IP65 outdoor + PoE | AliExpress | 1 | $15.00 | $15.00 |
| Power Supply | 5V 3A USB-C + UPS HAT | Amazon/AliExpress | 1 | $10.00 | $10.00 |
| Ethernet Cable | Cat6 10m | Local | 1 | $5.00 | $5.00 |
| **Total** | | | | | **$100.00** |

### Tier 3: Concierge/Ark (~$200+)

| Component | Part Number | Supplier | Qty | Unit Price | Total |
|-----------|-------------|----------|-----|------------|-------|
| Server Chassis | 2U Rackmount or Tower | Local/Refurbished | 1 | $100.00 | $100.00 |
| CPU | Intel Xeon Silver 4210R | eBay/Refurbished | 1 | $80.00 | $80.00 |
| Motherboard | C621 chipset, dual socket | eBay/Refurbished | 1 | $60.00 | $60.00 |
| RAM | 32GB DDR4 ECC REG | eBay/Refurbished | 1 | $40.00 | $40.00 |
| Storage (OS) | 512GB NVMe SSD | Amazon | 1 | $40.00 | $40.00 |
| Storage (Archive) | 4TB HDD 7200RPM | Amazon | 1 | $80.00 | $80.00 |
| GPU | NVIDIA RTX 4060 Ti 16GB | Amazon | 1 | $450.00 | $450.00 |
| PSU | 500W 80+ Gold | Local | 1 | $50.00 | $50.00 |
| Network Card | 10GbE SFP+ (Mellanox ConnectX-3) | eBay | 1 | $30.00 | $30.00 |
| WiFi 6E AP | TP-Link Archer AXE75 | Amazon | 1 | $100.00 | $100.00 |
| UPS | APC Back-UPS 1500VA | Amazon | 1 | $150.00 | $150.00 |
| **Total** | | | | | **$1,180.00** |

*Note: Tier 3 pricing assumes refurbished/used enterprise hardware. New hardware will cost 2-3x more. For GCD4F, we recommend starting with repurposed desktop/server hardware.*

---

## Power Requirements

### Power Budget Summary

| Tier | Typical Draw | Peak Draw | Battery Runtime (2600mAh) | Solar Panel Recommendation |
|------|--------------|-----------|---------------------------|---------------------------|
| **Tier 0** | 15mA (RX) / 120mA (TX) | 120mA | 20-170 hours | 5V 1W panel |
| **Tier 1** | 150-200mA | 400mA | 2.5-3.5 hours | 5V 5W panel + 10000mAh battery |
| **Tier 2** | 600-800mA | 1.5A | 45-60 minutes | 12V 20W panel + 20000mAh battery |
| **Tier 3** | 150-300W | 500W | 15-30 minutes (UPS) | Grid-tied + solar backup (1kW+) |

### Power Management Features
- **Tier 0-1:** Deep sleep modes, duty cycling, wake-on-packet
- **Tier 2:** Dynamic frequency scaling, PoE passthrough
- **Tier 3:** ACPI power states, GPU passthrough control, scheduled AI inference

---

## Environmental Specifications

### Operating Conditions

| Tier | Temperature | Humidity | Ingress Protection | Vibration |
|------|-------------|----------|-------------------|-----------|
| **Tier 0** | -20°C to +60°C | 0-95% non-condensing | IP67 | MIL-STD-810G (field deployable) |
| **Tier 1** | -10°C to +50°C | 0-90% non-condensing | IP65 (with case) | Basic shock resistance |
| **Tier 2** | 0°C to +50°C | 0-85% non-condensing | IP65 (outdoor case) | Indoor/light outdoor |
| **Tier 3** | 15°C to +35°C | 20-80% non-condensing | N/A (indoor rackmount) | Climate-controlled room |

### Deployment Scenarios
- **Tier 0:** Emergency response, disaster zones, wildlife tracking, maritime
- **Tier 1:** Urban neighborhoods, rural villages, campus networks
- **Tier 2:** Community centers, schools, clinics, cell towers
- **Tier 3:** Data centers, government facilities, universities

---

## Certification & Compliance

### Required Certifications (by region)

| Certification | Region | Applicability | Status |
|---------------|--------|---------------|--------|
| **Kenya CA** | Kenya | All radio equipment | ⚠️ Pending |
| **FCC Part 15** | USA/Canada | WiFi, BLE, LoRa (915MHz) | ⚠️ Pending |
| **EU RED** | European Union | All radio equipment | ⚠️ Pending |
| **CE Marking** | EU/EEA | Electrical safety + EMC | ⚠️ Pending |
| **RoHS** | EU/Global | Hazardous substances | ✅ Compliant (component selection) |
| **ICASA** | South Africa | Radio spectrum | ⚠️ Pending |

### Spectrum Allocation (LoRa)

| Region | Frequency | Max TX Power | Duty Cycle |
|--------|-----------|--------------|------------|
| **EU868** | 868.1-868.5 MHz | 14dBm | 1% (sub-band) |
| **US915** | 902-928 MHz | 30dBm | No limit (FHSS) |
| **AS923** | 923-925 MHz | 16dBm | 1% (region-dependent) |
| **AU915** | 915-928 MHz | 30dBm | No limit |

---

## Assembly Notes

### Tier 0: Lifeboat Assembly
1. Solder LoRa module to ESP32-S3 via SPI (GPIO pins: MOSI, MISO, SCK, NSS, DIO0, DIO1)
2. Attach LoRa antenna to u.FL connector (do NOT power on without antenna)
3. Flash firmware via USB-serial adapter (UART: TX, RX, GND, 3.3V)
4. Configure via serial console: set frequency, TX power, node ID
5. Seal in IP67 enclosure with antenna cable gland
6. Test range: 1-5km (line-of-sight), 500m-1km (urban)

### Tier 1: Router Assembly
1. Flash Raspberry Pi OS Lite to microSD (64-bit, headless)
2. Install LoRa HAT on GPIO header (align pins carefully)
3. Attach external WiFi antenna to u.FL connector (if applicable)
4. Install GhostWire Core via `cargo install ghostwire` or pre-built binary
5. Configure `/etc/ghostwire/config.toml`:
   ```toml
   [node]
   tier = 1
   transports = ["wifi", "ble", "lora"]
   
   [ai]
   l1_enabled = true
   l2_enabled = false  # Requires external server
   l4_fallback = true
   ```
6. Enable systemd service: `systemctl enable --now ghostwire`
7. Test mesh connectivity: `ghostwire-cli mesh status`

### Tier 2: Gateway Assembly
1. Follow Tier 1 steps, plus:
2. Install UPS HAT and configure graceful shutdown:
   ```bash
   sudo apt install apcupsd
   sudo systemctl enable apcupsd
   ```
3. Configure Ethernet bridge in `/etc/ghostwire/config.toml`:
   ```toml
   [gateway]
   ethernet_interface = "eth0"
   internet_uplink = true
   nat_enabled = true
   ```
4. Install Ollama for L2 AI: `curl -fsSL https://ollama.com/install.sh | sh`
5. Pull Gemma model: `ollama pull gemma4:e2b`
6. Enable GNN training (optional): `ghostwire-cli gnn train --dataset mesh-topology`

### Tier 3: Concierge/Ark Assembly
1. Install OS (Ubuntu Server 22.04 LTS or Debian 12)
2. Install NVIDIA drivers + CUDA toolkit:
   ```bash
   sudo apt install nvidia-driver-535 nvidia-cuda-toolkit
   ```
3. Install GhostWire + AI dependencies:
   ```bash
   pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu118
   pip install dgl -f https://data.dgl.ai/wheels/cu118/repo.html
   cargo install ghostwire --features full-ai
   ```
4. Configure archive services (Wikipedia, OSM, etc.)
5. Set up monitoring: Prometheus + Grafana dashboard
6. Configure mesh backbone: `ghostwire-cli mesh backbone --mode concierge`

---

## 📝 Revision History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 0.1.0 | 2026-04-10 | [Your Name] | Initial draft for GCD4F |

---

## ✅ Approval Sign-off

| Role | Name | Signature | Date |
|------|------|-----------|------|
| Hardware Lead | | | |
| Software Lead | | | |
| Project Manager | | | |
| GCD4F Team Rep | | | |

---

*This document is part of the GhostWire GCD4F submission. For questions, contact the hardware team or open an issue on GitHub.*
