<!--
  Copyright 2025-2026 Michael Irungu and GhostWire Contributors
  SPDX-License-Identifier: Apache-2.0

  Licensed under the Apache License, Version 2.0 (the "License");
  you may not use this file except in compliance with the License.
  You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

  Unless required by applicable law or agreed to in writing, software
  distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions and
  limitations under the License.

  AI Training Restriction: This specification may not be used for training,
  fine-tuning, or otherwise developing machine learning models, large language
  models, or artificial intelligence systems without prior written permission
  from the copyright holder.
-->

# Wire Format Specification

**GIP:** Pending
**Version:** 0.2.0-draft
**Date:** April 12, 2026

---

## Overview

This specification defines the wire format for GhostWire protocol messages. All implementations MUST support this format for interoperability.

---

## Frame Structure

```
┌─────────────┬─────────┬───────┬──────────────┬──────────────┬──────────┐
│ Magic (4B)  │ Ver (1B)│ Type  │ Seq (8B)     │ Length (4B)  │ Payload  │
│ 0x47573077  │ 0x02    │ (1B)  │ Big-Endian   │ Big-Endian   │ (varies) │
└─────────────┴─────────┴───────┴──────────────┴──────────────┴──────────┘
```

### Fields

| Field | Size | Description |
|-------|------|-------------|
| **Magic** | 4 bytes | Protocol magic: `0x47 0x57 0x30 0x77` ("GW0w") |
| **Version** | 1 byte | Protocol version (current: `0x02`) |
| **Type** | 1 byte | Frame type identifier |
| **Sequence** | 8 bytes | Monotonic sequence number (big-endian, replay protection) |
| **Length** | 4 bytes | Payload length in bytes (big-endian, max 64 KB) |
| **Payload** | variable | Serialized message content |

### Frame Types

| Type ID | Name | Description |
|---------|------|-------------|
| `0x01` | `Message` | Encrypted application message |
| `0x02` | `Announce` | Peer announcement |
| `0x03` | `DhtPut` | DHT value storage request |
| `0x04` | `DhtGet` | DHT value retrieval request |
| `0x05` | `DhtValue` | DHT value response |
| `0x06` | `RouteSetup` | Sphinx route establishment |
| `0x07` | `RoutedPayload` | Onion-routed message |
| `0x08` | `MailboxDeposit` | Store message for offline recipient |
| `0x09` | `MailboxDrain` | Retrieve stored messages |
| `0x0A` | `CoverTraffic` | Dummy frame for traffic analysis resistance |
| `0x0B` | `Ping` | Liveness probe |
| `0x0C` | `Pong` | Liveness response |
| `0x0D` | `Handshake` | Initial connection handshake |

### Constraints

- **Maximum frame size:** 64 KB — prevents OOM attacks
- **Sequence numbers:** Monotonically increasing per peer; receivers MUST reject duplicates within a sliding window (default: 64 packets)
- **Version negotiation:** Handshake frames include `min_version` and `max_version`; connection proceeds at `min(local.max, remote.max)` if within range

---

## Serialization

Payloads are serialized using **bincode** unless otherwise specified.

### Message Payload (Type 0x01)

```rust
pub struct EncryptedMessage {
    pub sender_pubkey: [u8; 32],
    pub recipient_pubkey: [u8; 32],
    pub nonce: [u8; 24],
    pub ciphertext: Vec<u8>,
    pub signature: [u8; 64],
    pub pq_key: Option<Vec<u8>>,
    pub timestamp: u64,
}
```

---

## Replay Protection

- **Window size:** 64 sequence numbers (configurable)
- **Algorithm:** Bitset tracking seen sequence numbers within window
- **Rejection:** Frames with sequence numbers already in window are silently dropped

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| `0x01` | 2026-04-05 | Initial implementation (first ping) |
| `0x02` | 2026-04-12 | Added PQ key field, HLC timestamps, frame type expansion |

---

## Reference Implementation

See `src/core/wire_protocol.rs` in the GhostWire reference implementation.
---

> **Lawful Use Only:** GhostWire is designed for lawful communication in connectivity-constrained environments. Users are responsible for complying with all applicable laws and regulations in their jurisdiction. The developers assume no liability for misuse of this software.

---
