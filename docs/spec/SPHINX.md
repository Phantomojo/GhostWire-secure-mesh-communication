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

# Sphinx Onion Routing Specification

**GIP:** Pending
**Version:** 0.2.0-draft
**Date:** April 12, 2026

---

## Overview

This specification defines the Sphinx onion routing protocol used by GhostWire for anonymous, unlinkable message delivery. Sphinx provides bitwise unlinkability, layered encryption, and support for Single-Use Reply Blocks (SURBs) — enabling anonymous responses without revealing the sender's identity.

The GhostWire Sphinx implementation follows the same protocol design as Nym and Lightning Network BOLT #4, with adaptations for mesh networking constraints (variable hop counts, cover traffic, and DTN integration).

---

## Protocol Design

### Architecture

Sphinx onion routing constructs a layered encryption "onion" that is peeled one layer at each hop. Each intermediate node can only decrypt the layer addressed to it, revealing the next hop's address and the remaining encrypted onion. No single hop knows both the sender and the recipient.

```
Sender → Hop 1 → Hop 2 → ... → Hop N → Recipient
         (peels   (peels              (final
          layer 1) layer 2)            delivery)
```

### Hop Count

| Parameter | Value |
|-----------|-------|
| **Minimum hops** | 2 (enforced; single-hop routes are rejected) |
| **Maximum hops** | 5 |
| **Default hops** | 3 |
| **Hop selection** | AI routing layer (L1-L4 fallback, see ROUTING.md) |

**Rationale for minimum 2 hops:** A single-hop route provides no anonymity — the sole intermediate node knows both sender and recipient. Two hops ensure that no single node has complete path knowledge.

---

## Cryptographic Construction

### Session Key Derivation

For a route with N hops, the sender generates a single ephemeral session key and derives per-hop keys using a blinding factor:

```
session_key = random_bytes(32)  // CSPRNG
session_pubkey = X25519_basepoint * session_key

blinding_factor_1 = BLAKE3(session_pubkey, output_len=32)
blinding_factor_i = BLAKE3(blinding_factor_{i-1}, output_len=32)
```

### Per-Hop Key Derivation

Each hop's shared secret and ephemeral key are derived via Curve25519 scalar multiplication:

```
ephemeral_key_1 = session_key
ephemeral_pubkey_1 = session_pubkey

ephemeral_key_i = ephemeral_key_{i-1} * blinding_factor_{i-1}
ephemeral_pubkey_i = ephemeral_pubkey_{i-1} * blinding_factor_{i-1}

shared_secret_i = X25519(ephemeral_key_i, hop_pubkey_i)
```

**Bitwise unlinkability:** The blinding factor ensures that `ephemeral_pubkey_i` is unlinkable to `ephemeral_pubkey_{i-1}` under the Decisional Diffie-Hellman (DDH) assumption on Curve25519. An observer cannot determine that two consecutive Sphinx packets share the same session key.

### Layered Encryption

Each hop's layer is encrypted using ChaCha20 with a key derived from the shared secret:

```
rho_i = HKDF-BLAKE3(
    salt = shared_secret_i,
    ikm = shared_secret_i,
    info = "ghostwire/sphinx/rho/v2",
    length = 32
)

delta_i = HKDF-BLAKE3(
    salt = shared_secret_i,
    ikm = shared_secret_i,
    info = "ghostwire/sphinx/delta/v2",
    length = 32
)
```

- `rho_i` is the stream cipher key for encrypting the payload at hop i.
- `delta_i` is the MAC key for authenticating the header at hop i.

### Header Construction

Each hop header contains:

```rust
pub struct SphinxHopHeader {
    pub ephemeral_pubkey: [u8; 32],    // X25519 ephemeral public key
    pub next_hop: [u8; 32],            // BLAKE3 hash of next hop's pubkey (or recipient)
    pub mac: [u8; 32],                 // BLAKE3 MAC for integrity
    pub padding: [u8; 64],             // Fixed-size padding for constant header size
}
```

**Constant size:** All headers are fixed-size (128 bytes) to prevent traffic analysis based on header length.

### Payload Encryption

The payload at each hop is encrypted with the `rho` key:

```
encrypted_payload_i = ChaCha20(rho_i, plaintext_i)
mac_i = BLAKE3_keyed(delta_i, encrypted_payload_i || header_i, output_len=32)
```

The MAC covers both the encrypted payload and the header, ensuring that any modification is detected.

---

## Onion Construction

### Building the Onion

The sender constructs the onion from the last hop to the first (inside-out):

```
// Start with the final payload (message to recipient)
payload_N = message_to_recipient

// For each hop from N down to 1:
for i in N..=1:
    header_i = SphinxHopHeader {
        ephemeral_pubkey: ephemeral_pubkey_i,
        next_hop: if i == N { recipient_id } else { hop_pubkey_{i+1} },
        mac: compute_mac(delta_i, payload_i, header_i),
        padding: random_bytes(64),
    }
    encrypted_header = ChaCha20(rho_i, header_i)
    payload_{i-1} = encrypted_header || payload_i  // prepend and pad to constant size
```

**Constant-size payload:** The total onion packet is padded to a constant size to prevent traffic analysis. The padding fills the space that would be occupied by additional hops if the route were at maximum length.

### Processing at Each Hop

When a hop receives a Sphinx packet:

1. **Compute shared secret:** `shared_secret = X25519(local_privkey, ephemeral_pubkey)`
2. **Derive keys:** Compute `rho` and `delta` from the shared secret.
3. **Decrypt header:** `header = ChaCha20(rho, encrypted_header)`
4. **Verify MAC:** `verify_mac(delta, decrypted_payload, header)` — reject if invalid.
5. **Extract next hop:** Read `next_hop` from the decrypted header.
6. **Strip layer:** Remove the current hop's header from the payload.
7. **Forward:** Send the remaining payload to `next_hop`.

If any step fails (MAC verification, decryption error, unknown next hop), the packet is silently dropped. No error is returned to the sender.

---

## Single-Use Reply Blocks (SURBs)

SURBs enable anonymous responses without revealing the sender's identity.

### SURB Construction

A SURB is a pre-constructed reverse onion that the recipient uses to send a reply:

```rust
pub struct Surb {
    pub reply_id: [u8; 32],           // Unique identifier for this SURB
    pub onion: Vec<u8>,               // Pre-built reverse onion
    pub session_key: [u8; 32],        // Ephemeral key for decrypting reply
    pub expiry: u64,                  // Unix timestamp when SURB expires
}
```

### SURB Usage

1. **Sender generates SURB:** The sender creates a SURB with a reverse route (recipient → hop N → ... → hop 1 → sender).
2. **Sender includes SURB ID:** The sender includes the `reply_id` in the forward message.
3. **Recipient builds reply:** The recipient encrypts their reply with the SURB's `session_key` and wraps it in the SURB's `onion`.
4. **SURB is consumed:** After use, the SURB is destroyed. SURBs are strictly single-use.

**SURB expiry:** SURBs have a configurable TTL (default: 1 hour). Expired SURBs are rejected. This prevents replay attacks and limits the window for traffic correlation.

**Multiple SURBs:** A sender may include multiple SURBs in a single message to enable multi-round anonymous conversation. Each SURB is independent and single-use.

---

## Cover Traffic

GhostWire generates cover traffic to resist traffic analysis and timing correlation attacks.

### Frame Type

Cover traffic uses `FrameType::CoverTraffic` (type ID `0x0A` in the wire format, see WIRE_FORMAT.md).

### Cover Traffic Construction

```rust
pub struct CoverTrafficFrame {
    pub dummy_onion: Vec<u8>,     // Valid Sphinx onion to a random destination
    pub padding: Vec<u8>,         // Random padding to match typical message size
}
```

### Generation Strategy

| Parameter | Value |
|-----------|-------|
| **Interval** | Exponential distribution, mean = 30 seconds (configurable) |
| **Route length** | Same distribution as real traffic (2-5 hops) |
| **Payload size** | Matches real message size distribution |
| **Destination** | Random peer ID (message discarded at final hop) |

**Security property:** Cover traffic is indistinguishable from real traffic at the network layer. An observer cannot determine whether a given Sphinx packet carries a real message or cover traffic.

**Resource cost:** Cover traffic consumes bandwidth and CPU. Users configure the cover traffic rate based on their threat model and available resources. In bandwidth-constrained environments (BLE, LoRa), cover traffic is disabled by default.

---

## Route Selection

### AI-Guided Hop Selection

The AI routing layer (see ROUTING.md) selects hops based on:
- **Latency:** Minimize end-to-end delay.
- **Reliability:** Prefer hops with high uptime and low packet loss.
- **Geographic diversity:** Prefer hops in different physical locations to resist localized surveillance.
- **Trust score:** Prefer hops with high Web of Trust scores (see SOCIAL.md).
- **Load balancing:** Avoid overloading any single hop.

### Hop Discovery

Hops are discovered via:
- **Kademlia DHT:** Peers advertise their willingness to route Sphinx packets.
- **mDNS:** Local network peers announce routing capability.
- **Manual configuration:** Users can designate trusted peers as preferred hops.

### Route Validation

Before using a route, the sender validates:
1. No hop appears twice in the route (loop prevention).
2. The sender is not the recipient (trivial route rejection).
3. All hops are currently online and reachable.
4. The route meets the minimum 2-hop requirement.

---

## Security Properties

| Property | Mechanism |
|----------|-----------|
| **Bitwise unlinkability** | X25519 scalar blinding (Curve25519 DDH assumption) |
| **Sender anonymity** | No hop knows both sender and recipient (min 2 hops) |
| **Recipient anonymity** | Final hop only knows the previous hop, not the sender |
| **Payload confidentiality** | ChaCha20 stream cipher with per-hop keys |
| **Payload integrity** | BLAKE3 MAC (32-byte tag) over header + payload |
| **Replay resistance** | SURBs are single-use; forward messages use sequence numbers |
| **Traffic analysis resistance** | Cover traffic (FrameType::CoverTraffic), constant-size packets |
| **Route confidentiality** | Each hop only knows its predecessor and successor |
| **Forward anonymity** | Compromising a hop does not reveal past routes (ephemeral keys) |

### Threat Model

| Adversary | Capability | Protection |
|-----------|-----------|------------|
| **Passive network observer** | Monitors all traffic on a link | Cannot link sender to recipient (unlinkability) |
| **Compromised hop** | Controls one intermediate node | Cannot determine full route or read payload |
| **Global passive adversary** | Monitors all network links | Mitigated by cover traffic and timing chaff |
| **Active adversary** | Injects, modifies, or replays packets | Detected by BLAKE3 MAC; replayed packets dropped |
| **Colluding hops** | Controls multiple hops in a route | Anonymity preserved if at least one honest hop remains |

### Limitations

- **Global passive adversary:** An adversary monitoring all network links with precise timing can correlate traffic patterns. Cover traffic and timing chaff reduce but do not eliminate this risk.
- **Endpoint compromise:** If the sender's or recipient's device is compromised, Sphinx provides no protection.
- **Small networks:** In networks with few peers, anonymity sets are small and statistical de-anonymization is easier.

---

## Reference Implementation

See the following modules in the GhostWire reference implementation:

| Module | Path |
|--------|------|
| Sphinx packet construction | `src/core/sphinx/packet.rs` |
| Route building | `src/core/sphinx/route.rs` |
| SURB management | `src/core/sphinx/surb.rs` |
| Cover traffic | `src/core/sphinx/cover_traffic.rs` |
| Hop selection | `src/core/sphinx/hop_selector.rs` |
| Sphinx tests | `tests/sphinx_protocol.rs` |
---

> **Lawful Use Only:** GhostWire is designed for lawful communication in connectivity-constrained environments. Users are responsible for complying with all applicable laws and regulations in their jurisdiction. The developers assume no liability for misuse of this software.

---
