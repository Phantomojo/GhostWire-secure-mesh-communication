# Capability-Adaptive Operating Profiles

**Status:** Deterministic policy foundation implemented and unit-tested. Runtime configuration, platform telemetry, and adapter enforcement remain in progress.
**Purpose:** Make every GhostWire installation a capable node without forcing a phone, battery-backed device, or low-rate radio gateway to behave like an unrestricted desktop/server relay.

> **A resource-constrained node may refuse new work. It may not weaken a message’s declared privacy requirement, remove relay intent, reuse an identity, or silently choose a weaker carrier.**

## Node, Role, and Carrier Are Separate Decisions

Every GhostWire installation is a node. It can receive its own messages and may relay protected envelopes when policy permits. The node’s **device class**, user-selected **role**, dynamic **runtime state**, and each carrier’s independent eligibility jointly determine what new work it may accept.

| Decision layer | Examples | What it controls |
|---|---|---|
| Device class | Desktop/server, mobile/constrained, low-rate radio gateway | Conservative base capacity and default operating posture |
| Owner role | Endpoint-only, relay opt-in, supervised relay | Whether a node has consented to forward other nodes’ protected envelopes |
| Runtime state | Battery, external power, thermal state, metering, background-execution permission | A bounded reduction of local work only |
| Carrier eligibility | User enablement, device support, OS permission, background eligibility, carrier relay opt-in | Whether a specific local interface can be selected |
| Message requirement | Delivery kind, minimum privacy, relay intent | Immutable send property; the profile cannot lower it |

This separation prevents a common failure mode: promoting a mobile device to a relay simply because it has Wi-Fi, BLE, a working mesh connection, or external power. A hardware capability is not user consent.

## Implemented Profiles

`core::operating_profile` implements deterministic limits. They are deliberately conservative defaults, not capacity claims for every device.

| Profile | Typical deployment | Relay posture | Peer / pending-operation limit | Relay queue limit | Stream / bulk limit |
|---|---|---:|---:|---:|---:|
| Desktop/server endpoint | Desktop/laptop/server that does not opt in to forwarding | Endpoint only | 128 / 4,096 | 64 MiB reserved but no new relay work | 32 streams / 8 bulk sessions |
| Desktop/server supervised relay | Explicitly managed node such as **Omen** or `cerberus-command` | Supervised relay only | 128 / 4,096 | 64 MiB | 32 streams / 8 bulk sessions, 64 MiB object ceiling |
| Mobile/constrained endpoint | Phone/tablet/battery-backed platform | Endpoint only by default | 16 / 256 | 4 MiB | 4 streams / 1 bulk session, 8 MiB object ceiling |
| Mobile relay opt-in | Mobile node whose owner has explicitly opted in and whose platform permits background execution | Conditional relay | 16 / 256 | 4 MiB | Same nominal mobile ceiling; dynamically reduced on battery/thermal pressure |
| Low-rate radio gateway | Dedicated RNode/LoRa/KISS/serial device | Supervised relay only | 4 / 64 | 512 KiB | 1 stream / **no generic bulk session** |

These limits complement—not replace—the carrier contract. A desktop profile cannot make an unauthenticated radio interface private, and a mobile profile cannot make a metadata-visible route satisfy a private delivery requirement.

## Dynamic Reductions

Runtime state may only reduce accepted work. It never changes cryptographic requirements, CRL/trust/replay/causal validation, or carrier privacy labels.

| Condition | Deterministic effect |
|---|---|
| Mobile battery below 20% while not on external power | No new relay or bulk work; lower active-peer, pending-operation, queue, stream, and bulk-session limits. Existing accepted work requires its own graceful lifecycle handling. |
| Critical thermal state | No new relay or bulk work; cap active peers, pending operations, and streams. |
| Metered mobile network | New bulk work is refused by default. |
| Background execution not permitted | A mobile relay opt-in remains an endpoint for interfaces that require background execution. |
| Low-rate radio gateway | Generic bulk work is refused permanently; a future adapter needs its own radio-native resource policy. |

Android platform guidance supports treating Wi-Fi Direct discovery/group state, BLE roles, nearby-device permission, and background behavior as distinct platform constraints; link pairing/encryption is not an application privacy substitute. [1] [2]

## Carrier Interaction

The profile applies a **restriction-only** transform to an existing `InterfaceEligibility` declaration:

```text
existing user/device/OS carrier eligibility
    ∩ operating-profile runtime allowance
    = effective carrier eligibility
```

It cannot turn `false` into `true`. In particular, it cannot enable a user-disabled carrier, invent hardware support, bypass OS permission, promote a non-relay into a relay, or allow a background carrier when the platform disallows background execution.

## Privacy and Delivery Invariants

| Invariant | Implemented test |
|---|---|
| Mobile is endpoint-only unless relay opt-in is explicit | Passing |
| Mobile relay opt-in still needs platform background permission | Passing |
| Low battery removes new relay and bulk authority | Passing |
| Critical thermal state reduces work and does not change message privacy | Passing |
| Desktop relay capacity appears only for an explicit supervised-relay role | Passing |
| Low-rate radio profile rejects generic bulk work | Passing |
| Mobile bulk object ceiling is enforced | Passing |

## Integration Work Still Required

| Work item | Acceptance condition |
|---|---|
| Runtime config | Profile/role is persisted in private configuration and defaults conservatively on upgrade. |
| Platform telemetry | Android/iOS/desktop adapters supply authenticated/local battery, thermal, metering, and background signals; unknown state never expands work. |
| Mesh enforcement | Connection manager, relay queue, bulk session, and carrier registry consume the same effective limits. |
| UI/UX | TUI/web/mobile display endpoint vs relay status, per-carrier consent, and typed resource refusal without topology/identity leakage. |
| Lifecycle | Existing work drains or reports a truthful failure when resource state revokes future work; it is not silently re-routed to a weaker carrier. |
| Live validation | Omen/EliteKali/`cerberus-command` and a mobile test device demonstrate profile limits under real Tailscale/RNS/carrier conditions. |

## References

[1]: https://developer.android.com/develop/connectivity/wifi/wifi-direct "Android Developers: Wi-Fi Direct"
[2]: https://developer.android.com/develop/connectivity/bluetooth/ble/ble-overview "Android Developers: Bluetooth Low Energy"
