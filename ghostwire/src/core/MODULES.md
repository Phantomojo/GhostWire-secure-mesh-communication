# GhostWire Core Module Organization

This document describes the organization of GhostWire's core modules.

## Overview

The `core/` directory contains GhostWire's main business logic. Modules are organized into:

1. **Individual modules** - Direct implementation files
2. **Organized submodules** - Grouped related functionality

## Organized Submodules

### `crypto/`

Cryptographic modules for encryption, signing, and key management.

| Module | Description |
|--------|-------------|
| `armor` | Traffic obfuscation |
| `dht_signatures` | DHT request signing |
| `double_ratchet` | Double Ratchet KDF |
| `encryption` | Main encryption manager |
| `typed_crypto` | Type-safe cryptographic primitives |

### `network/`

Networking modules for mesh, transport, and protocols.

| Module | Description |
|--------|-------------|
| `briar` | Briar protocol implementation |
| `mesh` | Main mesh networking logic |
| `mesh_health` | Mesh health monitoring |
| `monitor` | Network monitoring |
| `presence_obfuscation` | Presence obfuscation |
| `reticulum` | Reticulum protocol |
| `transport` | Transport abstraction |
| `quic_transport` | QUIC transport |
| `stealth_tcp` | Stealth TCP transport |

### `storage/`

Storage and persistence modules.

| Module | Description |
|--------|-------------|
| `blockstore` | DHT-like content-addressed storage |
| `encrypted_store` | Encrypted key-value store |
| `mailbox` | Message mailbox |
| `persistence_manager` | Persistence coordination |
| `store` | Basic key-value store |

## Individual Modules

### Identity & Trust

| Module | Description |
|--------|-------------|
| `identity` | Identity management |
| `identity_pairing` | Device pairing |
| `recovery_shamir` | Social recovery via Shamir |
| `reputation` | Reputation system |
| `trust` | Trust scoring |

### AI/ML

| Module | Description |
|--------|-------------|
| `ai/anomaly_detector` | Isolation Forest anomaly detection |
| `ai/feature_extractor` | Mesh telemetry feature extraction |
| `ai/function_calling` | FunctionGemma tool calling |
| `ai/gemma_client` | Gemma 4 Ollama client |
| `ai/gnn_router` | GAT-based routing |
| `ai/litert_backend` | LiteRT-LM backend |
| `ai/onnx_inference` | ONNX Runtime inference |
| `ai/routing_classifier` | LightGBM routing classifier |

### Protocols & Messaging

| Module | Description |
|--------|-------------|
| `channels` | Async message channels |
| `dtn` | Delay-tolerant networking |
| `message` | Message types |
| `routing` | Routing algorithms |
| `reliability` | Reliability/ACK system |

### Security

| Module | Description |
|--------|-------------|
| `crl` | Certificate revocation lists |
| `security_v2` | Security manager |
| `traffic_armor` | Traffic obfuscation |

### Storage & Persistence

| Module | Description |
|--------|-------------|
| `blockstore` | Content-addressed storage |
| `encrypted_store` | Encrypted storage |
| `mailbox` | Message mailbox |
| `persistence_manager` | Persistence coordination |
| `store` | Key-value store |

### Network & Transport

| Module | Description |
|--------|-------------|
| `briar` | Briar protocol |
| `mesh` | Mesh networking |
| `quic_transport` | QUIC transport |
| `reticulum` | Reticulum protocol |
| `stealth_tcp` | Stealth TCP |
| `transport` | Transport abstraction |

### Utilities

| Module | Description |
|--------|-------------|
| `constants` | Protocol constants |
| `hlc` | Hybrid Logical Clocks |
| `qr` | QR code generation |
| `vector_clock` | Vector clocks |

## Feature Flags

Some modules are gated behind feature flags:

```toml
[features]
bluetooth = []      # Bluetooth support
briar = []          # Briar protocol
mesh = []           # Mesh networking
reticulum = []      # Reticulum protocol
matrix-bridge = []  # Matrix bridge
```

## Migration Guide

### Old Import Style
```rust
use crate::core::encryption::EncryptionManager;
```

### New Organized Style
```rust
use crate::core::crypto::encryption::EncryptionManager;
// or
use crate::core::encryption::EncryptionManager; // still works
```

The organized submodules provide re-exports for backward compatibility. All existing imports continue to work.

## Adding New Modules

When adding a new module to core:

1. Determine which organized submodule it belongs to
2. If it fits crypto, network, or storage → add to that submodule
3. Otherwise, add as individual module
4. Update this document with the new module

## Testing

Run tests for specific module groups:
```bash
cargo test core::crypto::*
cargo test core::network::*
cargo test core::storage::*
```
