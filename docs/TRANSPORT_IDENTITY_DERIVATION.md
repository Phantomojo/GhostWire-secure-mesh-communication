# Transport and Interface Identity Derivation

**Status:** Implemented foundation. The API is tested, but adapters do not yet consume every derived key.
**Decision:** GhostWire is an inter-transport security switchboard. It must not reuse its root identity as a carrier, radio, or external-network identity.

## Executive Decision

GhostWire previously had a recovery API named `Identity::from_seed()`. Its only current callers restore the **root GhostWire identity** after recovery; it is not a complete transport/interface hierarchy. Treating that recovery material as a general seed for libp2p, QUIC, BLE, Wi-Fi Direct, RNode/LoRa, or RNS identities would make a public identifier from one carrier correlate the node across every other carrier.

The hardened model therefore separates three layers.

| Layer | Key/identifier owner | Derivation and persistence rule | Must not be used as |
|---|---|---|---|
| Root GhostWire identity | GhostWire identity vault | Randomly generated or restored only from a **32-byte recovery secret** | A libp2p peer key, BLE/radio identity, RNS identity, or device fingerprint |
| GhostWire carrier-binding key | GhostWire | HKDF-SHA-256 from an **unlocked** root identity, explicit carrier domain, opaque local interface-instance ID, and persisted rotation epoch | A cross-carrier stable identifier or native external-stack identity |
| Native carrier identity | The carrier/runtime | Created, persisted, announced, and rotated according to the carrier’s own rules | Input to GhostWire root derivation or a replacement for end-to-end GhostWire identity |

> **Key principle:** GhostWire performs authenticated handoff of protected GhostWire envelopes. It does **not** force Wi-Fi, BLE, RNS, LoRa, libp2p, or Internet paths to share one identity scheme.

## Implemented Construction

`identity::transport_derivation` implements a fixed HKDF-SHA-256 construction. HKDF’s extract-and-expand design is specifically intended to derive one or more keys from input keying material while binding a distinct `info` context to each output [1]. GhostWire uses a fixed protocol-versioned salt and a fixed-size context:

```text
HKDF-SHA-256(
  salt = "GhostWire/transport-key/salt/v1",
  IKM  = serialized unlocked root identity keypair,
  info = "GhostWire/transport-key/v1"
         || carrier-domain-u8
         || rotation-epoch-u64-be
         || opaque-interface-instance-id-16
)
→ 32-byte Ed25519 child secret
```

The resulting child key authenticates GhostWire’s local interaction with that carrier. It is **not** exported as an external carrier identity unless a future adapter has an explicit, reviewed specification that permits it.

| Domain | Intended use | Explicit boundary |
|---|---|---|
| `Libp2pMesh` | GhostWire-side libp2p mesh binding | Must not reuse the root public key as a peer identity |
| `DirectQuic` | GhostWire direct-QUIC binding | QUIC’s TLS/session identity and GhostWire binding remain distinct concerns |
| `BluetoothLowEnergy` | GhostWire application binding around BLE | Does not replace BLE privacy addresses or platform pairing identities |
| `WifiDirect` | GhostWire application binding around Wi-Fi Direct | Does not use SSID, BSSID, MAC address, or device name as derivation input |
| `RNodeOrLoRa` | GhostWire application binding for radio adapter policy | Does not make a radio identifier into a root or cross-network identity |
| `StoreAndForward` | Encrypted mailbox/DTN handoff binding | Does not disclose provider/availability records publicly |
| `RnsBridgeBinding` | Local GhostWire ↔ official-RNS bridge control boundary | **Not an RNS identity seed**; RNS owns its independent identity and routing cryptography |

## Interface Instance IDs

Every configured local interface receives a random 16-byte `InterfaceInstanceId`. It is deliberately opaque. The following are forbidden derivation inputs because they are discoverable, stable, or otherwise create a correlation handle:

| Forbidden input | Why it is unsafe |
|---|---|
| MAC / Bluetooth address | Stable or link-layer-observable device correlation |
| SSID / BSSID | Exposes local network membership and location context |
| Interface name / hardware serial | Host-local and often user-identifying fingerprint |
| IP address / multiaddr | Network-location correlation and topology leakage |
| RNS destination hash / libp2p peer ID | Collapses independent network identities into one linked graph |

The interface instance ID must be saved only in private local configuration alongside the adapter configuration. Changing it intentionally rotates the derived binding key; restarting does not. An all-zero identifier is rejected to prevent a shared default identity.

## Recovery Boundary

`Identity::from_recovery_seed()` now checks for the exact 32-byte Ed25519 recovery secret and documents that it is for recovery/import only. `from_seed()` remains only as a compatibility alias. No transport code may call either method. New transport code must call `Identity::derive_transport_keypair(TransportKeyScope)` while the root identity is unlocked.

The derivation API refuses to operate when the root identity is locked. This avoids presenting a stale cached or public-only value as a private transport key.

## Acceptance Tests

The implementation has deterministic unit coverage for the following invariants.

| Invariant | Test result |
|---|---|
| Same root + same carrier + same interface + same epoch produces the same public child key | Passing |
| Changing the carrier domain produces a different public child key | Passing |
| Changing interface instance or rotation epoch produces a different public child key | Passing |
| All-zero local interface instance IDs are rejected | Passing |
| RNS bridge-binding key differs from the GhostWire root public key | Passing |
| A locked root identity cannot derive a carrier-binding key | Passing |
| Recovery imports reject a non-32-byte root seed | Passing |

## Adapter Adoption Order

The derivation primitive is a safety foundation, not proof that every carrier has been wired. Adapters adopt it only after their native identity, discovery, and security model has been reviewed. The adoption order is deliberately conservative.

| Adapter | Current decision | Required before enabling user traffic |
|---|---|---|
| Official RNS bridge | Keep RNS identity independent; use `RnsBridgeBinding` only to authenticate the local bridge protocol once it is extended | Shared bounded ingress, authenticated bridge control protocol, RNS `Link`/`Resource` path for non-packet payloads, real multi-node tests |
| libp2p | Plan a distinct `Libp2pMesh` binding | Migration away from root-key reuse, peer identity rotation/mapping policy, mixed-route tests |
| QUIC | Plan a distinct `DirectQuic` binding | TLS/TOFU binding review and direct-stream handoff integration |
| BLE / Wi-Fi Direct | No claim of implementation | Platform-specific privacy-address, permission, background-execution, and MTU/fragmentation reviews |
| RNode/LoRa | No claim of implementation | Airtime/duty-cycle budget, native radio identity analysis, low-rate store-and-forward tests |

## References

[1]: https://datatracker.ietf.org/doc/html/rfc5869 "RFC 5869: HMAC-based Extract-and-Expand Key Derivation Function (HKDF)"
