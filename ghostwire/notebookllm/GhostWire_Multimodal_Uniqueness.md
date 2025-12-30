# 📡 GhostWire: The Multimodal Edge

GhostWire's most unique feature isn't just *how* it encrypts, but *where* it can travel. Unlike standard messaging apps that die when the internet goes out, GhostWire is a **Multimodal Mesh** designed for total signal independence.

## 1. 🔘 Signal Independence (The "Mediums")
GhostWire treats the network medium as a pluggable component. It can span:
- **Radio (LoRa)**: Via `MeshtasticAdapter`. Perfect for long-range (10km+), low-power, off-grid communication.
- **Bluetooth (BLE/RFCOMM)**: For short-range, peer-to-peer communication between phones and laptops without Wi-Fi.
- **Wi-Fi (mDNS/Ad-Hoc)**: For local high-speed mesh clusters.
- **Stealth TCP**: For clandestine communication over standard internet infrastructure, disguised to bypass DPI.
- **Satellite/HF Radio**: Integrated via the `Reticulum` stack for global off-grid reach.

## 2. 🚨 Disaster Mode & Auto-Switching
The `DisasterManager` and `TransportSwitcher` components (found in `disaster.rs` and `transport_manager.rs`) provide the "brain" for this resilience:
- **Threshold Triggers**: If Wi-Fi latency exceeds 500ms or packet loss hits 20%, the system can automatically "fallback" to Bluetooth.
- **Total Dark Mode**: If all IP-based networks are detected as compromised or down, GhostWire triggers **Disaster Mode**, switching the entire node to LoRa/Radio-only operation.
- **Transport Hopping**: Like frequency hopping, GhostWire can rotate between mediums to evade traffic analysis.

## 3. 🕊️ Reticulum Integration: The "Secret Sauce"
GhostWire leverages `Reticulum` to provide:
- **Address Independence**: Your GhostWire ID remains the same whether you are on Wi-Fi or LoRa.
- **Onion Routing over Radio**: GhostWire is one of the few systems that can perform complex onion-routing even over low-bandwidth LoRa links.
- **Store-and-Forward**: If a node is offline, messages are stored in the multimodal mesh and delivered automatically once the node (or a bridging neighbor) reappears.

## 4. 🔗 The Bridge Pattern
GhostWire nodes act as **Universal Bridges**:
- A node with both Wi-Fi and a LoRa radio will automatically act as a gateway, allowing users on LoRa-only devices to communicate with the wider Wi-Fi/Internet mesh.
- This creates a "Connected Island" effect where local off-grid meshes can be linked via a single "uplink" node.

---

### 🛡️ Why it's Unique
Most "secure" apps assume a functioning internet. GhostWire assumes the **infrastructure is the enemy or is missing**. By mastering Bluetooth, Wi-Fi, and LoRa simultaneously, it ensures the "ghost" never goes silent.
