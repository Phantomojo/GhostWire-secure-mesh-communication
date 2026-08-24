# GhostWire cockpit control contract

## Design principle

GhostWire’s interfaces are control panels, not decorative dashboards. Every displayed control must map to an authenticated backend action, and every green state must be backed by an observable runtime fact. The protocol-level PeerID remains canonical; a profile name is a verified human-facing label resolved from signed identity metadata, never a replacement for the PeerID.

## Transport state model

Each transport/interface is represented by one record:

| Field | Meaning |
|---|---|
| `id` | Stable GhostWire identifier such as `mesh`, `quic`, `reticulum`, `bluetooth`, `wifi_direct`, or `wifi_aware`. |
| `protocol` | Actual protocol implementation, such as `libp2p`, `quic`, or `rns`. |
| `medium` | Physical/logical medium, such as `lan_or_ip`, `ip`, `wifi`, `bluetooth`, or `multi_medium`. |
| `implementation` | `compiled`, `registered`, `platform_gated`, `not_built`, or `placeholder`. |
| `desired_enabled` | Persisted operator intent. |
| `state` | `disabled`, `deferred`, `starting`, `ready`, `degraded`, `failed`, or `unavailable`. |
| `registered` / `started` / `ready` | Separate lifecycle facts; none implies the others. |
| `carrying_connections` | Number of live connections attributed to this transport. |
| `bytes_tx` / `bytes_rx` | Counters from the transport itself, not guessed from global traffic. |
| `packets_tx` / `packets_rx` | Transport-level packet counters where available. |
| `last_activity_unix` | Last observed traffic or lifecycle event. |
| `listen_addresses` | Actual bound addresses, redacted when necessary. |
| `discovery` | Discovery method, scope, state, candidates, and last error. |
| `controls` | Whether enable, disable, configure, scan, or connect actions are actually supported. |
| `last_error` | Operator-readable failure, with a stable error code in the machine schema. |

`active_transport` is retained only for backward compatibility. New UI should use `carrying_connections` and `connection.transport` to identify the real path. A preferred route is not the same thing as the transport carrying current traffic.

## Identity model

| Concept | Definition |
|---|---|
| Local profile | User-selected local vault/profile, such as `default` or `elitekali`. |
| GhostWire identity | The cryptographic identity created locally for that profile. |
| libp2p PeerID | The network identity derived from the libp2p key; canonical for peer routing. |
| RNS identity/address | A separate Reticulum identity/address derived through the RNS adapter; it must not be silently equated with the libp2p PeerID. |
| Display profile name | Human-facing metadata resolved from a verified identity record and shown alongside, never instead of, the canonical PeerID. |
| API token/JWT | Local control-plane authentication credential; not a network identity and never displayed as peer identity. |

## Discovery truth model

Discovery evidence, candidates, connected peers, authenticated identities, trusted peers, GossipSub topic membership, and message receipts are separate states. The UI must never promote one state into another automatically.

## Message truth model

The send path reports `sending`, `accepted`, `receipt_pending`, `delivered`, or `failed`. `connected` means a network session exists; it does not mean the recipient accepted or decrypted a message.

## Initial implementation priorities

First, initialize and register the existing RNS manager after profile unlock, expose RNS interface metrics, and keep unsupported BLE/Wi‑Fi Direct/Aware states explicitly gated. Second, replace the hard-coded/ambiguous active-transport display with per-interface cards. Third, remove duplicate rendering of `connectedCandidates` and `allPeers`, and map `discovery_method` and `last_seen` from backend health instead of discarding them. Fourth, add authenticated controls only for lifecycle operations the backend can actually perform; unsupported controls remain disabled with a reason.

## Progressive-disclosure interface

The default view exposes five questions: Is my local identity ready? Is the mesh running? How many peers are connected? Can I discover another node? Can I send and receive a message? It uses plain labels such as `ready`, `waiting`, `connected`, `needs attention`, and `unavailable`, with one primary action per blocked state.

Advanced mode exposes the same records with protocol names, lifecycle flags, listen addresses, PeerIDs, RNS addresses, discovery sources, GossipSub/Kademlia/mDNS evidence, transport counters, route selection, errors, and authenticated operator actions. Advanced mode must not invent controls: an unavailable control is shown disabled with its reason, and a registered transport without a lifecycle action is shown as observe-only.

Identity screens use a simple profile card by default and an advanced identity drawer for DID, libp2p PeerID, RNS address, key algorithms, creation time, and verification status. A human name is displayed only when it is bound to a verified signed identity record; otherwise the UI uses `Unverified GhostWire peer` plus the canonical PeerID.

Transport toggles are not generic booleans. They represent desired administrative state and report the restart/reconfiguration boundary. A toggle that requires a daemon restart must say so before it changes intent. A runtime switch is permitted only when the backend can stop/start the specific transport safely and report the resulting state.
