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

# Peer Discovery Specification

**GIP:** Pending
**Version:** 0.2.0-draft
**Date:** April 12, 2026

---

## Overview

This specification defines the peer discovery mechanisms used by GhostWire to find and connect to other peers in the mesh network. GhostWire supports multiple discovery methods operating simultaneously: local network discovery via mDNS, global discovery via Kademlia DHT, manual introduction via contact codes, and direct device pairing. Each method is independent; peers discovered through any method are equally valid for communication.

---

## Discovery Methods

### 1. mDNS (Multicast DNS)

mDNS enables automatic peer discovery on the local network segment.

| Property | Value |
|----------|-------|
| **Service type** | `_ghostwire._tcp.local.` |
| **Protocol** | Multicast DNS (RFC 6762) |
| **Port** | Configurable (default: 9735) |
| **TTL** | 120 seconds |
| **Query interval** | Every 60 seconds (with jitter) |

**Service announcement:**
```
Service: _ghostwire._tcp.local.
Instance: <peer_id_base58>._ghostwire._tcp.local.
Host: <local_ip>
Port: <listening_port>
TXT Records:
  version=0.2.0
  pubkey=<ed25519_pubkey_hex>
  transports=tcp,webrtc,ble
  features=sphinx,dtn,social
```

**Discovery process:**
1. On startup, the peer broadcasts an mDNS query for `_ghostwire._tcp.local.`
2. Peers on the same network respond with their service records.
3. The discovering peer extracts the Ed25519 public key and connection details.
4. An Identify protocol exchange is initiated to verify the peer's identity.

**Network scope:** mDNS is limited to the local broadcast domain (same subnet). It does not traverse NAT or routers.

### 2. Kademlia DHT

The Kademlia Distributed Hash Table enables global peer discovery across the mesh network.

| Property | Value |
|----------|-------|
| **Protocol** | S/Kademlia-hardened Kademlia |
| **Key space** | 256-bit (BLAKE3 hash output) |
| **Bucket size (k)** | 20 entries |
| **Alpha (concurrent queries)** | 3 |
| **Bootstrap nodes** | User-configured; default: none (fully decentralized) |

**S/Kademlia hardening:**
- **Sybil resistance:** Peer IDs are cryptographically bound to Ed25519 public keys. Creating fake IDs requires generating valid keypairs, which is computationally expensive.
- **Eclipse resistance:** Each bucket maintains a mix of old and new peers. Replacement caches store alternative peers for each bucket.
- **Route table diversity:** Periodic random lookups prevent route table stagnation.
- **Secure node ID:** Node IDs are `BLAKE3(ed25519_pubkey)`, binding identity to DHT position.

**Peer storage in DHT:**
```
key = BLAKE3(peer_pubkey || "dht_peer_record")
value = DhtPeerRecord {
    pubkey: [u8; 32],
    addresses: Vec<Multiaddr>,
    transports: Vec<TransportType>,
    last_seen: u64,
    signature: [u8; 64],  // Ed25519 signature over record
}
```

**Peer lookup:**
1. Compute the target key from the desired peer's public key.
2. Perform iterative Kademlia lookup (FIND_NODE queries).
3. The closest nodes to the target key return their stored peer records.
4. Verify the Ed25519 signature on the returned record.
5. Connect to the peer using the provided addresses.

**Rendezvous points:** Peers can register at rendezvous points for discovery by topic or group:
```
rendezvous_key = BLAKE3(pubkey || "rendezvous")[:16]  // 16-byte rendezvous identifier
```
Peers sharing the same rendezvous prefix are likely to be in the same community or geographic area.

### 3. Manual Introduction (Contact Codes)

Users can manually add peers using contact codes.

| Property | Value |
|----------|-------|
| **Format** | `gw2:<base58_encoded_data>` |
| **Encoding** | Base58BTC (Bitcoin alphabet: 123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz) |
| **Content** | Ed25519 public key (32 bytes) + optional connection hints |
| **Checksum** | BLAKE3-based checksum (4 bytes) |

**Contact code structure:**
```
gw2:<base58(pubkey_32bytes || checksum_4bytes || version_1byte)>
```

**Example:** `gw2:4tK9mN2pQ7rS8vW1xY3zA5bC6dE7fG8hJ9kL0mN1pQ2rS3tU4vW5x`

**Import process:**
1. User scans QR code or pastes the contact code string.
2. The client decodes the base58 data and verifies the checksum.
3. The Ed25519 public key is extracted and added to the peer's contact list.
4. The client attempts to discover the peer via DHT or direct connection.
5. A ConnectRequest is sent (see SOCIAL.md).

### 4. Device Pairing

Device pairing enables secure connection between two devices in physical proximity.

| Property | Value |
|----------|-------|
| **Display code** | 16-character XXXX-XXXX-XXXX-XXXX format |
| **Character set** | Uppercase alphanumeric (A-Z, 2-9), excluding ambiguous characters (0, 1, O, I) |
| **Entropy** | ~68 bits (32^16) |
| **Session key derivation** | BLAKE3(display_code, output_len=32) |
| **Safety number** | 6-digit numeric (displayed for manual verification) |

**Pairing protocol:**

1. **Initiator generates code:**
   ```
   display_code = random_alphanumeric(16, charset="ABCDEFGHJKLMNPQRSTUVWXYZ23456789")
   formatted_code = XXXX-XXXX-XXXX-XXXX  // hyphen-separated groups of 4
   session_key = BLAKE3(display_code, output_len=32)
   ```

2. **Initiator displays code:** The code is shown on screen (and optionally as QR code).

3. **Responder enters code:** The user types the code into the second device.

4. **Both derive session key:** Both devices compute `session_key = BLAKE3(display_code)`.

5. **LAN discovery via mDNS:** Both devices broadcast on the local network using the session key as a rendezvous identifier.

6. **Mutual authentication:** Both devices exchange Ed25519 public keys encrypted with the session key.

7. **Safety number display:** Both devices compute and display a 6-digit safety number:
   ```
   safety_number = (BLAKE3(pubkey_a || pubkey_b, output_len=8) as u64) % 1_000_000
   ```

8. **User verification:** Users compare safety numbers (in person or via trusted channel). If they match, the pairing is confirmed.

9. **DHT rendezvous:** If mDNS fails (devices on different networks), both devices register at a DHT rendezvous point derived from the session key and retry.

**Security properties:**
- The display code has sufficient entropy (~68 bits) to resist online brute-force attacks.
- The session key is derived from the display code, ensuring only devices with the same code can communicate.
- The safety number provides a short, human-verifiable fingerprint of the key exchange.
- If safety numbers do not match, a MITM attack is in progress; the pairing is aborted.

---

## Identify Protocol

After discovering a peer through any method, the Identify protocol exchange verifies the peer's identity.

### Identify Request

```rust
pub struct IdentifyRequest {
    pub protocol_version: String,    // e.g., "0.2.0"
    pub agent_version: String,       // e.g., "ghostwire/0.2.0"
    pub public_key: [u8; 32],        // Ed25519 public key
    pub supported_protocols: Vec<String>,  // e.g., ["sphinx", "dtn", "social"]
    pub listen_addresses: Vec<Multiaddr>,
    pub signature: [u8; 64],         // Ed25519 signature over request body
}
```

### Identify Response

```rust
pub struct IdentifyResponse {
    pub protocol_version: String,
    pub agent_version: String,
    pub public_key: [u8; 32],
    pub supported_protocols: Vec<String>,
    pub listen_addresses: Vec<Multiaddr>,
    pub signature: [u8; 64],
}
```

### Verification

1. **Signature verification:** Verify the Ed25519 signature over the request/response body using the provided public key.
2. **Key consistency:** Verify that the public key in the response matches the key used for the signature.
3. **Protocol compatibility:** Check that both peers support a common set of protocols.
4. **TOFU pinning:** If this is the first contact, pin the peer's public key (see CRYPTO.md).

---

## Discovery Priority

When multiple discovery methods are available, GhostWire uses the following priority:

| Priority | Method | Use Case |
|----------|--------|----------|
| 1 | mDNS | Local network peers (lowest latency) |
| 2 | Device pairing | Physically proximate devices (highest trust) |
| 3 | Manual contact code | User-initiated connections |
| 4 | Kademlia DHT | Global mesh discovery |
| 5 | Rendezvous points | Community/group discovery |

**Parallel discovery:** All enabled methods run in parallel. The first successful discovery is used; subsequent discoveries for the same peer are merged (additional addresses are added to the peer's address list).

---

## Security Properties

| Property | Mechanism |
|----------|-----------|
| **Identity binding** | Ed25519 signatures on all discovery records |
| **Sybil resistance** | S/Kademlia-hardened DHT, cryptographic peer IDs |
| **MITM detection** | Safety number comparison during device pairing |
| **TOFU** | Public key pinning on first contact |
| **Replay resistance** | Timestamps and signatures on DHT records |
| **Eclipse resistance** | Kademlia bucket diversity, replacement caches |
| **Local network privacy** | mDNS announcements contain only public key and connection hints (no personal data) |

### Threat Model

| Adversary | Capability | Protection |
|-----------|-----------|------------|
| **Local network eavesdropper** | Monitors mDNS traffic | Only public keys and connection hints exposed |
| **DHT spammer** | Floods DHT with fake records | Signature verification rejects unsigned records; S/Kademlia limits Sybil impact |
| **MITM during pairing** | Intercepts display code exchange | Safety number mismatch detected; session key encryption prevents code interception |
| **Contact code forgery** | Creates fake contact code | Checksum validation rejects malformed codes; TOFU pins real key |

---

## Reference Implementation

See the following modules in the GhostWire reference implementation:

| Module | Path |
|--------|------|
| mDNS discovery | `src/core/discovery/mdns.rs` |
| Kademlia DHT | `src/core/discovery/dht.rs` |
| Contact codes | `src/core/discovery/contact_code.rs` |
| Device pairing | `src/core/discovery/pairing.rs` |
| Identify protocol | `src/core/discovery/identify.rs` |
| Rendezvous points | `src/core/discovery/rendezvous.rs` |
---

> **Lawful Use Only:** GhostWire is designed for lawful communication in connectivity-constrained environments. Users are responsible for complying with all applicable laws and regulations in their jurisdiction. The developers assume no liability for misuse of this software.

---
