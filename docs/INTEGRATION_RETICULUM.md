# Integration Plan: Reticulum into GhostWire

## Overview

Wire Reticulum as a complementary transport for LoRa hardware scenarios. Uses the existing `ProtocolAdapter` trait — no changes to the core transport stack.

**Prerequisite:** Read `docs/EVALUATION_RETICULUM_VS_LIBP2P.md` for the "why".

---

## Phase 1: ReticulumAdapter (1-2 weeks)

### Goal
Make `ReticulumManager` usable as a `ProtocolAdapter` so messages can be sent/received via Reticulum transport.

### Step 1.1: Implement ProtocolAdapter for ReticulumManager

**File:** `ghostwire/src/core/adapter_registry.rs`

```rust
// Add to existing ProtocolAdapter implementations
#[async_trait]
impl ProtocolAdapter for ReticulumManager {
    fn name(&self) -> &str { "reticulum" }
    
    fn supported_protocols(&self) -> Vec<String> {
        vec!["reticulum".to_string()]
    }
    
    async fn send_message(&self, msg: &Message) -> Result<()> {
        // Route through ReticulumManager's existing queue + onion routing
        self.send(msg).await
    }
    
    async fn receive_message(&self) -> Result<Option<Message>> {
        // Poll ReticulumManager's message queue
        self.recv().await
    }
    
    fn is_available(&self) -> bool {
        // Check if Reticulum has active paths
        self.has_active_paths()
    }
    
    async fn metrics(&self) -> Result<TransportMetrics> {
        // Return Reticulum-specific metrics
        Ok(self.stats())
    }
}
```

### Step 1.2: Fix ReticulumManager Integration Points

**File:** `ghostwire/src/core/reticulum.rs`

Current issues:
- Standalone module with its own topology (not connected to libp2p)
- No message queue polling (synchronous interface)
- No `has_active_paths()` method
- Missing `stats()` returning `TransportMetrics`

Fixes:
1. Add `async fn send(&self, msg: &Message) -> Result<()>` that queues messages
2. Add `async fn recv(&self) -> Result<Option<Message>>` that polls the queue
3. Add `fn has_active_paths(&self) -> bool` checking route table
4. Add `fn stats(&self) -> TransportMetrics` returning latency/bandwidth/peer count
5. Replace `AES-256-GCM` with GhostWire's `XChaCha20-Poly1305` for E2E encryption

### Step 1.3: Wire into Core Initialization

**File:** `ghostwire/src/core/mod.rs`

```rust
// In Core::new(), after transport_registry setup:
let reticulum_manager = if config.network.reticulum_enabled {
    Some(Arc::new(RwLock::new(ReticulumManager::new()?)))
} else {
    None
};

// Register as adapter
if let Some(ref rm) = reticulum_manager {
    adapter_registry.write().await.register(
        "reticulum".to_string(),
        Box::new(rm.clone()) as Box<dyn ProtocolAdapter>
    );
}
```

### Step 1.4: Update Configuration

**File:** `config/ghostwire.toml.example`

```toml
[network]
reticulum_enabled = false
reticulum_bind = "0.0.0.0:4242"  # Reticulum TCP port
reticulum_announce_interval = 30  # seconds
```

### Step 1.5: Add Unit Tests

**File:** `ghostwire/src/core/reticulum.rs` (add tests module)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_reticulum_adapter_send_receive() {
        let manager = ReticulumManager::new().unwrap();
        let msg = Message::test_message();
        manager.send(&msg).await.unwrap();
        let received = manager.recv().await.unwrap();
        assert_eq!(received.unwrap().id, msg.id);
    }
    
    #[tokio::test]
    async fn test_reticulum_has_active_paths() {
        let manager = ReticulumManager::new().unwrap();
        assert!(!manager.has_active_paths());
        // Add a path, verify it appears
    }
}
```

### Deliverable
- `ReticulumManager` implements `ProtocolAdapter`
- Messages can be sent/received via Reticulum transport
- Config flag enables/disables the adapter
- Unit tests pass

---

## Phase 2: LoRa Hardware Bridge (2-4 weeks)

### Goal
Connect GhostWire to actual LoRa hardware via Reticulum's RNode firmware.

### Step 2.1: Python FFI Bridge (Fast Path)

**File:** `ghostwire/src/core/transport_ext/reticulum_bridge.rs` (new)

```rust
use pyo3::prelude::*;

pub struct ReticulumBridge {
    python_module: PyObject,
}

impl ReticulumBridge {
    pub fn new() -> PyResult<Self> {
        Python::with_gil(|py| {
            let module = py.import("rns")?;
            Ok(Self { python_module: module })
        })
    }
    
    pub async fn send(&self, dest: &[u8], data: &[u8]) -> Result<()> {
        // Call RNS.Transport.sendMessage(dest, data)
        Python::with_gil(|py| {
            self.python_module
                .call_method1("send", (dest, data))?;
            Ok(())
        })
    }
}
```

**Dependency:** `pyo3` crate for Python FFI.

**Trade-off:** Adds Python runtime dependency. Acceptable for hardware validation phase.

### Step 2.2: Direct LoRa via ESP32 BLE (Long-Term)

**File:** `ghostwire/src/core/hardware/loRa.rs` (new)

```rust
pub trait LoRaTransport: Send + Sync {
    async fn send(&self, data: &[u8]) -> Result<()>;
    async fn recv(&self) -> Result<Vec<u8>>;
    fn is_connected(&self) -> bool;
}

pub struct Esp32BleLoRa {
    // BLE GATT client connecting to ESP32 running LoRa firmware
    ble_client: BleClient,
}

impl LoRaTransport for Esp32BleLoRa {
    async fn send(&self, data: &[u8]) -> Result<()> {
        // Write to GATT characteristic for LoRa transmission
        self.ble_client.writeCharacteristic(LORA_TX_CHAR, data).await
    }
}
```

**This bypasses Renticulum entirely** — GhostWire talks directly to ESP32 hardware.

### Step 2.3: Firmware Reference Implementation

**File:** `esp32-firmware-ghostwire/` (new directory)

Reference ESP32 firmware that:
- Runs LoRa radio (SX1276/SX1262)
- Exposes BLE GATT service for GhostWire communication
- Handles packet fragmentation for LoRa MTU (250 bytes)
- Implements basic mesh routing (flood or AODV)

### Deliverable
- Python FFI bridge working with RNode firmware
- OR direct BLE LoRa transport (depending on which path is faster)
- ESP32 firmware reference implementation

---

## Phase 3: DTN Integration (1-2 weeks)

### Goal
Use Reticulum's path tables as a DTN delivery backend for LoRa scenarios.

### Step 3.1: Add Reticulum as DTN Backend

**File:** `ghostwire/src/core/dtn/epidemic.rs`

```rust
pub enum DtnBackend {
    GossipSub,      // Current: libp2p pub/sub
    ReticulumPath,  // New: Reticulum announce-based paths
}

impl DtnStore {
    pub async fn deliver_bundle(&self, bundle: &DtnBundle, backend: DtnBackend) -> Result<()> {
        match backend {
            DtnBackend::GossipSub => self.deliver_via_gossipsub(bundle).await,
            DtnBackend::ReticulumPath => self.deliver_via_reticulum(bundle).await,
        }
    }
    
    async fn deliver_via_reticulum(&self, bundle: &DtnBundle) -> Result<()> {
        // Use Reticulum's path tables for delivery
        // Lower bandwidth than GossipSub, works on LoRa
        todo!()
    }
}
```

### Step 3.2: Bandwidth-Aware Backend Selection

```rust
impl DtnStore {
    pub fn select_backend(&self, metrics: &TransportMetrics) -> DtnBackend {
        if metrics.bandwidth < 1000 {  // < 1 kbps (LoRa)
            DtnBackend::ReticulumPath
        } else {
            DtnBackend::GossipSub
        }
    }
}
```

### Deliverable
- DTN can deliver bundles via Reticulum path tables
- Backend selection based on available bandwidth

---

## Phase 4: Auto-Switching (1 week)

### Goal
DisasterManager recommends Reticulum when appropriate.

### Step 4.1: Add Reticulum to TransportType

**File:** `ghostwire/src/core/transport.rs`

```rust
pub enum TransportType {
    Mesh,
    Quic,
    StealthTCP,
    Bluetooth,
    Reticulum,  // NEW
}
```

### Step 4.2: Update Disaster Policy

**File:** `ghostwire/src/core/transport_manager.rs`

```rust
impl DisasterPolicy {
    pub fn recommend_transport(&self, level: DisasterLevel, ctx: &TransportContext) -> TransportType {
        match level {
            DisasterLevel::Normal => ctx.current_transport,
            DisasterLevel::Congested => TransportType::StealthTCP,
            DisasterLevel::Isolation => {
                if ctx.has_lora_hardware {
                    TransportType::Reticulum  // LoRa fallback
                } else {
                    TransportType::Bluetooth
                }
            }
            DisasterLevel::Blackout => {
                if ctx.has_lora_hardware {
                    TransportType::Reticulum  // Last resort
                } else {
                    TransportType::Mesh  // Keep trying
                }
            }
        }
    }
}
```

### Step 4.3: Hardware Detection

```rust
pub struct TransportContext {
    pub current_transport: TransportType,
    pub has_lora_hardware: bool,  // Detected via BLE scan or config
    pub bandwidth: u64,
    pub peer_count: usize,
}
```

### Deliverable
- Auto-switcher recommends Reticulum when LoRa hardware detected
- Fallback chain: libp2p → Reticulum → Bluetooth → Mesh retry

---

## Testing Strategy

### Unit Tests
- `reticulum.rs`: ProtocolAdapter implementation, send/recv, path management
- `epidemic.rs`: DTN backend selection, Reticulum delivery
- `transport_manager.rs`: Disaster policy with Reticulum recommendation

### Integration Tests
- Two GhostWire nodes communicating via Reticulum adapter
- DTN bundle delivery over Reticulum path tables
- Auto-switching from libp2p to Reticulum when bandwidth drops

### Hardware Tests
- ESP32 running RNode firmware, communicating with GhostWire via BLE
- LoRa mesh with 3+ nodes, DTN bundle delivery
- Latency/throughput measurements on LoRa vs WiFi

---

## Dependencies

| Dependency | Purpose | License | Risk |
|------------|---------|---------|------|
| `pyo3` | Python FFI for Reticulum | MIT | Low |
| `rns` (Python) | Reticulum Network Stack | Public domain + anti-AI | **High** |
| `ble` crate | ESP32 BLE communication | MIT | Low |
| `sx1276`/`sx1262` crate | LoRa radio control | MIT/Apache-2.0 | Low |

**Critical:** The `rns` Python package has the anti-AI clause. If this is a blocker, skip Phase 2 Python FFI and go directly to Phase 2.2 (direct LoRa via BLE).

---

## Timeline

| Phase | Duration | Depends on |
|-------|----------|------------|
| Phase 1: ReticulumAdapter | 1-2 weeks | Nothing |
| Phase 2: LoRa Hardware | 2-4 weeks | Phase 1 |
| Phase 3: DTN Integration | 1-2 weeks | Phase 1 |
| Phase 4: Auto-Switching | 1 week | Phase 1 |
| **Total** | **5-9 weeks** | |

**Parallelizable:** Phases 3 and 4 can run in parallel after Phase 1.

---

## Success Criteria

1. ✅ ReticulumManager implements ProtocolAdapter
2. ✅ Messages send/receive via Reticulum transport
3. ✅ Config flag enables/disables Reticulum
4. ✅ Unit tests pass
5. ✅ LoRa hardware communicates with GhostWire (at least 2 nodes)
6. ✅ DTN bundles deliver via Reticulum paths
7. ✅ Auto-switcher recommends Reticulum when LoRa detected
8. ✅ Latency < 5s on LoRa, throughput > 100 bps

---

## Rollback Plan

If Reticulum integration fails or license becomes problematic:

1. **Phase 1 only:** Keep ReticulumManager as standalone module, don't register as adapter
2. **Phase 2:** Skip Python FFI, implement direct LoRa via BLE (independent of Reticulum)
3. **Phase 3-4:** Use GossipSub for DTN only, remove Reticulum from disaster policy

The ProtocolAdapter pattern means Reticulum can be unplugged without affecting the rest of GhostWire.
