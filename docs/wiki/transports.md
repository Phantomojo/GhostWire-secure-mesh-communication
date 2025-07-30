# Transports

GhostWire supports a modular, pluggable transport layer, enabling communication over a variety of physical and virtual networks.

---

## Supported & Planned Transports

| Transport   | Status    | Use Case / Notes                                  |
|-------------|-----------|--------------------------------------------------|
| Bluetooth   | Planned   | Short-range, mobile-to-mobile, disaster recovery  |
| WiFi        | Planned   | Local mesh, high bandwidth, urban/rural           |
| LoRa        | Planned   | Long-range, low-power, off-grid, rural/disaster   |
| WebRTC      | Planned   | Browser-to-browser, NAT traversal, P2P            |
| TCP/IP      | Supported | Standard internet, fallback, bridges to servers   |
| Stealth TCP | Supported | Obfuscated, censorship-resistant, stealth comms   |

---

## Modularity
- Each transport is implemented as a separate module/crate.
- Transports can be enabled/disabled at runtime or compile time.
- The `Transport` trait defines a common interface for all transports.
- Future transports (e.g., satellite, mesh radio) can be added easily.

---

## How It Works
- The backend manages a registry of active transports.
- Messages are routed over the best available transport.
- Transports can be prioritized, failover is automatic.
- Security and privacy features are enforced across all transports.

---

## Example Use Cases
- **Bluetooth:** Ad-hoc mesh in disaster zones, no infrastructure needed.
- **LoRa:** Rural/off-grid communication, long-range, low-power.
- **WebRTC:** Browser-based chat, NAT/firewall traversal.
- **Stealth TCP:** Circumvent censorship, operate in hostile environments. 