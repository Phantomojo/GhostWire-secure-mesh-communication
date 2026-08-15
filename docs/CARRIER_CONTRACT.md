# GhostWire Carrier Contract

**Status:** Policy model implemented; concrete adapter declarations and shared ingress wiring remain in progress.
**Purpose:** Make GhostWire an **inter-transport security switchboard** without pretending Wi-Fi, BLE, RNS, LoRa, and libp2p are the same network.

## What GhostWire Switches

A GhostWire node can be an endpoint, relay, and inter-transport handoff point. The node receives a still-protected GhostWire envelope from one carrier, validates it, then sends the same protected envelope over another approved carrier. It changes **carrier framing and per-leg session behavior**, not user plaintext.

> **A successful carrier receipt is not automatically end-to-end delivery.** A libp2p write, RNS packet receipt, BLE acknowledgement, or radio transmit event describes one carrier leg. Final delivery requires the GhostWire end-to-end acknowledgement state machine.

A handoff boundary must enforce CRL/revocation, trust policy, message authentication, replay protection, causal ordering, expiry, bounded queueing, and privacy requirements before forwarding. It must fail closed if the next carrier cannot meet those requirements.

## Carrier Contract Schema

Every concrete adapter must publish a `TransportCapability` with a `CarrierSemantics` declaration. The policy coordinator rejects a declaration if its carrier family and native identity namespace contradict one another, or if the local interface is not eligible.

| Required declaration | Why GhostWire needs it |
|---|---|
| Carrier family | Prevents labels such as “RNS” or “Bluetooth” from concealing fundamentally different network rules. |
| Native identity namespace | Keeps libp2p peer IDs, RNS destinations, Wi-Fi Direct group/device state, BLE address/GATT role, radio identifiers, and mailbox handles separate. |
| Discovery mode | Prevents discovery encryption from being mistaken for metadata privacy. |
| Privacy class | Expresses observed network metadata properties, not merely payload encryption. |
| Fragmentation/resource model | Stops generic message chunking from violating MTU, resource, and integrity constraints. |
| Operation support | Separately declares broadcast, direct, store-and-forward, and bulk-ciphertext capability. |
| Interface eligibility | Requires owner enablement, hardware/platform support, OS permission, and background-execution permission. |
| Relay eligibility | Forces an explicit user/profile relay decision; phones do not relay simply because a radio is available. |

## Native Carrier Worlds

| Carrier family | Native identity / routing world | Discovery and metadata reality | Current GhostWire policy posture |
|---|---|---|---|
| libp2p | Peer IDs, multiaddrs, negotiated secure/multiplexed streams | A direct connection reveals a peer relationship; Noise/stream encryption does not imply anonymity. [1] | Direct carrier defaults to at most `ContentConfidential` unless a higher privacy route proves stronger protection. |
| Reticulum | RNS destinations, announces, paths, interfaces, packet/link/resource semantics | Each RNS interface has different reachability and observability; RNS is not one universal privacy label. [2] [3] | Real external-RNS bridge currently supports bounded single-packet opaque payloads only; larger delivery waits for a verified Link/Resource path. |
| Wi-Fi Direct | Device/group discovery, group-owner/client roles, local-IP sockets | Group formation and nearby discovery reveal local-proximity metadata; WPA2 link encryption is not GhostWire E2EE. [4] | No implementation claim. Owner/profile must explicitly permit group owner or relay behavior. |
| BLE/GATT | Advertisement, central/peripheral role, GATT service/characteristic, small MTU-aware exchange | Pairing does not replace application-layer protection; scanning/advertisements leak local presence. [5] | No implementation claim. Requires rotating rendezvous, strict MTU plan, OS permission, and energy/queue budget. |
| RNode/LoRa/KISS | Radio/serial interface configuration, airtime and rate constraints | Extremely low rate and observable radio behavior make generic retries, flood broadcast, and arbitrary chunks unsafe. [3] | No implementation claim. Requires native airtime/duty-cycle policy and dedicated resource/fragment strategy. |
| Store-and-forward | Private mailbox/postbox handles | Queues can protect availability but become metadata infrastructure if provider/availability data is public. | Must retain encrypted storage and private route policy; no public provider advertisements. |

## Local Eligibility and Relay Posture

A carrier must be both **available** and **eligible**. Availability comes from adapter health. Eligibility is a local device-policy statement.

| Eligibility factor | Example that must block selection |
|---|---|
| Owner enabled | The user turned off Bluetooth or disallowed GhostWire relay. |
| Device supported | The mobile device lacks the required radio or driver. |
| OS permission | Android/iOS nearby-device, Bluetooth, location, or background permission is absent. |
| Background execution allowed | A mobile profile cannot keep a carrier alive in the background. |
| Relay opt-in | A user is a GhostWire node but has not consented to forward other nodes’ traffic on this interface. |

`RelayEligibility::OwnerOptIn` is the normal mobile/constrained default. `RelayEligibility::SupervisedNode` is appropriate only for explicitly configured desktop/server infrastructure, such as a maintained node on **Omen** or `cerberus-command`. Neither setting authorizes decryption of application plaintext.

## Fragmentation Rule

GhostWire does not have a universal fragmenter. Carrier policy selects one of three outcomes **after message-level protection**:

| Carrier contract | Allowed result |
|---|---|
| Native stream | A bounded direct logical frame or a separately authorized private block-exchange session. |
| Native resource/link facility | A direct packet below the carrier MDU, otherwise the carrier’s documented resource facility. |
| Strict single-packet path | Send only when within the authenticated packet limit; reject oversize input. |

The current RNS bridge is a strict 383-byte encrypted-packet path. It rejects larger data rather than silently truncating or inventing fragments. Reticulum specifies a default 500-byte MTU and a maximum 383-byte encrypted single-packet payload; changing that MTU breaks interoperability. [2] [6]

## Security Invariants Now Tested

The current policy tests prove all of the following deterministic outcomes.

| Invariant | Result |
|---|---|
| A carrier below the declared direct-message privacy level is rejected rather than downgraded | Passing |
| An unavailable carrier is rejected even when it has a higher privacy declaration | Passing |
| A user-disabled or OS-ineligible local interface cannot be selected | Passing |
| Cross-carrier relay requires explicit relay eligibility and owner opt-in | Passing |
| A carrier with a contradictory native identity declaration cannot be selected | Passing |
| RNS packet mode rejects data above 383 bytes rather than generic-splitting it | Passing |

## Explicit Limits

This model is the policy contract; it has **not** yet automatically wired every concrete adapter. The following remain blocked until tested implementations exist:

| Limit | Correct next action |
|---|---|
| Concrete libp2p, QUIC, RNS, BLE, Wi-Fi Direct, and RNode adapters do not yet emit dynamic carrier declarations into one registry | Wire adapters only after their measured MTU, OS/radio, discovery, and receipt behavior is implemented. |
| Shared ingress validation is not unified across every carrier | Route all carrier ingress through one bounded CRL/trust/auth/replay/causal pipeline before application delivery. |
| Cross-carrier handoff state machine is not yet connected to outbound dispatch | Implement protected-envelope handoff with per-leg receipts and explicit final-delivery states. |
| Mobile/device profiles are not yet integrated into runtime configuration | Add bounded deterministic profile policy before enabling mobile relay behavior. |

## References

[1]: https://github.com/libp2p/specs/blob/master/connections/README.md "libp2p Connection Establishment Specification"
[2]: https://reticulum.network/manual/reference.html "Reticulum API Reference"
[3]: https://reticulum.network/manual/interfaces.html "Reticulum: Configuring Interfaces"
[4]: https://developer.android.com/develop/connectivity/wifi/wifi-direct "Android Developers: Wi-Fi Direct"
[5]: https://developer.android.com/develop/connectivity/bluetooth/ble/ble-overview "Android Developers: Bluetooth Low Energy"
[6]: https://github.com/markqvist/Reticulum/blob/master/RNS/Packet.py "Reticulum Reference Packet Implementation"
