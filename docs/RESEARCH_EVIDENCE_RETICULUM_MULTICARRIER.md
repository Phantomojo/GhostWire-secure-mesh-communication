# Research Evidence: Reticulum, libp2p, and Multi-Interface Carrier Design

**Prepared:** 2026-08-12 UTC
**Status:** Design evidence for the GhostWire libp2p–RNS hybrid, carrier-policy work, and device-capability profiles. This file records external facts; implementation claims are tracked separately in the architecture limits register and test matrix.

## Reticulum Runtime and Packet Semantics

The official Reticulum reference implementation is distributed as a Python runtime. An application creates an `RNS.Reticulum` instance, which opens interfaces from a configuration directory; an already-running shared instance may be reused locally. A daemon is recommended for systems with multiple applications using RNS. [1] [2]

| Finding | Consequence for GhostWire |
|---|---|
| RNS traffic is addressed to **destinations**, not IP addresses or libp2p peer IDs. A destination is a 128-bit hash and `SINGLE` destinations are bound to a Reticulum identity. [3] | GhostWire must maintain a separate, explicit mapping between a GhostWire contact and its RNS destination. A libp2p peer ID, a GhostWire account identifier, and an RNS destination must not be merged into one identifier. |
| RNS `SINGLE` packet mode automatically encrypts a packet with a newly derived ephemeral key. Links offer forward secrecy; destination ratchets can protect packet-mode traffic after a ratchet-bearing announce. [2] [4] | GhostWire hands RNS already authenticated opaque application payloads. It must not recreate RNS encryption, receipt, announce, path, or routing behavior. The bridge’s own identity remains separate from GhostWire’s identity. |
| The default RNS MTU is 500 bytes; the maximum encrypted single-packet payload is 383 bytes. Changing RNS MTU breaks compatibility. [2] [5] | The present bridge accepts at most 383 opaque bytes. Larger GhostWire envelopes need a designed and tested RNS `Link`/`Resource` profile; ad-hoc fragmentation is forbidden. |
| RNS may combine AutoInterface, TCP, Backbone, UDP, I2P, serial, KISS, RNode/LoRa, and custom interfaces in one runtime. [6] | RNS configuration is a carrier configuration boundary, not a single generic “Reticulum address.” GhostWire uses policy to decide whether an RNS carrier is eligible but does not dynamically fabricate an RNS interface from a string. |
| RNS AutoInterface uses local IPv6 multicast discovery and UDP transport. Backbone/TCP support intermittent connectivity; RNode supports radio parameters and airtime limits. [6] | RNS carrier declarations must include discovery, reachability, transport-operator status, MTU, latency, rate, energy/airtime, and observable metadata. “RNS” alone cannot receive one static privacy or reliability label. |

> **Reference implementation warning:** Reticulum’s license includes use restrictions, including an AI/ML-training-dataset prohibition. It is not an OSI-approved open-source license and its own project discussion describes GPL compatibility as uncertain. GhostWire therefore treats RNS as a separately installed optional runtime; it must not vendor or statically link RNS without a separate legal compatibility review. [7] [8]

## libp2p Semantics

libp2p distinguishes raw transports from a secured, multiplexed peer connection. Security and stream multiplexing are negotiated after raw connection establishment; the recommended baseline security protocol is Noise. libp2p also explicitly recommends bounded connection management, dynamic protocol eligibility by physical connection, and lifecycle events. [9]

| Finding | Consequence for GhostWire |
|---|---|
| A libp2p connection authenticates a libp2p peer and secures streams after negotiation; it does not establish anonymity or hide the peer relationship by itself. [9] | libp2p direct streams are not automatically eligible for GhostWire private-direct requirements. Their policy label remains at most `ContentConfidential` unless a higher-layer privacy route changes the threat model. |
| Supported protocols may vary by physical connection and may be revoked dynamically based on link properties. [9] | Carrier capability declarations must be dynamic but bounded. A metered, relayed, mobile, or low-rate interface can refuse bulk, broadcast, routing, or high-chatter protocol classes without being considered faulty. |
| Persistent connections require upper bounds and prioritization to avoid resource exhaustion. [9] | Desktop/server profiles can retain more peers; mobile/constrained profiles must enforce lower connection/stream/queue limits. The policy must be deterministic rather than an AI-only selector. |

## Wi-Fi Direct and BLE Semantics

Android’s Wi-Fi Direct creates nearby peer-to-peer groups without an Internet access point and provides WPA2 link encryption. It uses discoverable device/group state, selects a group owner based partly on device capabilities, and requires location/nearby-device permissions for discovery on current Android versions. [10]

Android BLE is designed for short nearby transfers and reduced power use. It distinguishes central/peripheral and GATT client/server roles; its documentation explicitly warns that BLE pairing does not replace application-layer protection because other apps on a device can access paired-device data. [11]

| Carrier | Native world-model | GhostWire policy boundary |
|---|---|---|
| Wi-Fi Direct | Device discovery, P2P group formation, group owner/client roles, local IP sockets, platform permissions. | Treat group metadata and peer discovery as observable. Use GhostWire end-to-end authentication/encryption after local-IP connection; never treat WPA2 as message privacy. Constrained profiles must not automatically become group owners or relays. |
| BLE/GATT | Advertisements, central/peripheral roles, GATT characteristics, small transfers, radio/energy constraints, platform permissions. | Use short-lived rotating rendezvous material, strict MTU-aware framing, application-layer encryption, and an explicit duty-cycle/queue budget. BLE does not qualify for an anonymity claim merely because a link is encrypted. |
| RNode/LoRa and KISS/serial | Very low-rate interfaces, radio frequency and airtime constraints, interface-specific framing and hardware. [6] | No generic retry/broadcast behavior. Enforce carrier-specific fragment/resource profiles, bounded announcements, airtime budget, and local regulatory configuration. |

## Architecture Decisions Derived from the Sources

GhostWire’s unifying abstraction must be a **carrier contract**, not a universal packet implementation. The shared layer may carry a versioned opaque GhostWire envelope and enforce identity association, CRL/trust/replay/causal checks, bounded queues, policy eligibility, and receipt interpretation. It must not erase native carrier semantics.

Every registered carrier should state, at minimum, its native identity namespace; peer-discovery mode; whether route/peer relationship metadata is observable; maximum authenticated payload; fragmentation/resource model; receipt semantics; minimum/expected latency; rate and airtime/energy cost; operating-system permission/background constraints; relay eligibility; and device-profile eligibility. Privacy policy must choose only among candidates that meet the operation’s requirements; a carrier failure must not silently trigger a lower-privacy alternative.

## References

[1]: https://reticulum.network/manual/using.html "Reticulum: Using Reticulum on Your System"
[2]: https://reticulum.network/manual/reference.html "Reticulum API Reference"
[3]: https://reticulum.network/manual/understanding.html "Reticulum: Understanding Reticulum"
[4]: https://reticulum.network/manual/examples.html "Reticulum Code Examples"
[5]: https://github.com/markqvist/Reticulum/blob/master/RNS/Packet.py "Reticulum Reference Packet Implementation"
[6]: https://reticulum.network/manual/interfaces.html "Reticulum: Configuring Interfaces"
[7]: https://reticulum.network/manual/license.html "Reticulum License"
[8]: https://github.com/markqvist/Reticulum/discussions/790 "Reticulum License Change Discussion"
[9]: https://github.com/libp2p/specs/blob/master/connections/README.md "libp2p Connection Establishment Specification"
[10]: https://developer.android.com/develop/connectivity/wifi/wifi-direct "Android Developers: Wi-Fi Direct"
[11]: https://developer.android.com/develop/connectivity/bluetooth/ble/ble-overview "Android Developers: Bluetooth Low Energy"
