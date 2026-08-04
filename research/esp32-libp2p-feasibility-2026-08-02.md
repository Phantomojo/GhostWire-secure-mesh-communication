# esp32-libp2p-feasibility
- Date: 2026-08-02
- Agent: librarian (session: bg_1f056de8)
- Status: raw

---

# ESP32 libp2p Feasibility Research

**Date:** 2026-08-02
**Agent:** librarian
**Source:** bg_1f056de8

## Core Finding

rust-libp2p cannot compile for ESP32. No `no_std` target exists, tokio only supports desktop/WASM, and a minimal swarm needs ~MB of RAM vs ~148KB free on ESP32 with WiFi+BLE active.

## Key Data Points

### ESP32 Hardware
- 520KB SRAM, ~390KB heap; WiFi+BLE costs ~205KB, leaving ~148KB free
- ESP-NOW: <0.5 Mbps, <100 nodes, <150m range, 250B v1 / 1470B v2 payload, 20 unicast peers
- BLE mesh: <1 Kbps, theoretical 32767 nodes (tested ~100), 50m range
- ESP WiFi Mesh: up to 20 Mbps, <100 nodes, 6 layers

### libp2p Platform Reality
- libp2p 0.56: ~8-46MB dependency source, ~729K SLoC deps, ~30K SLoC own
- No bare-metal target: transport crates explicitly not wasm32, no no_std/riscv/xtensa
- tokio supported platforms: Linux/Windows/macOS/BSD/Android/iOS + limited WASM — no embedded
- Nearest data point: smoldot (sans-I/O libp2p) runs in browser tab, ~MB not KB

### Existing ESP32 Projects
- esp-hal + esp-wifi 0.14/0.15: pre-1.0, requires unstable feature, production-proven (SlimeVR)
- microfips: no_std Noise_IK/XK handshakes on ESP32-D0WD — crypto works
- rns-rs: Reticulum port with no_std core, ESP32 variant tag
- minip2p: sans-I/O libp2p reimplementation for embedded (v0.1.0, very early)
- No working libp2p-on-ESP32 project exists

### Proven Architecture
Gateway-bridged: ESP32 runs bare-metal ESP-NOW/BLE mesh, bridge process terminates device mesh and speaks full libp2p.
