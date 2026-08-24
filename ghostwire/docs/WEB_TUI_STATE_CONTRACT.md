# GhostWire Shared Web UI / Nomad-Net Interface Contract

## Purpose

The Web UI and Nomad-Net TUI are different interaction surfaces over the same daemon. They must not invent separate meanings for readiness, discovery, trust, transport, or delivery. The daemon remains authoritative; both clients render typed state and submit explicit commands.

## Canonical state vocabulary

| State | Meaning | User-facing consequence |
| --- | --- | --- |
| `locked` | The selected local profile is locked. | Show unlock action; authenticated operations are unavailable. |
| `configured` | A transport or feature has configuration but has not started. | Show configuration and start capability, never green readiness. |
| `registered` | A transport exists in the registry but does not report active lifecycle state. | Show as present/available, not started or ready. |
| `starting` | Startup has been requested and has not reached a terminal state. | Disable duplicate start; show progress and last transition. |
| `started` | The driver/process is active but not necessarily usable for traffic. | Show active but not ready. |
| `ready` | The feature can perform its advertised operation. | Permit operation; show readiness reason and timestamp when possible. |
| `waiting_for_peers` | Discovery is functioning but no peer evidence is currently available. | Show scan/publish controls; do not show failure. |
| `connected` | A transport or peer connection exists. | Show connectivity only; do not infer trust or delivery. |
| `trusted` | Local policy accepts the peer identity/key. | Permit trust-dependent actions and show verification source. |
| `sending` | The local node accepted an outgoing message and is attempting delivery. | Show queued/in-flight state and transport evidence. |
| `delivered` | Receiver-side evidence confirms message receipt. | Show delivery evidence and timestamp. |
| `degraded` | Active but limited, stale, partially configured, or reporting an error. | Explain limitation and offer remediation. |
| `failed` | The requested operation or subsystem failed. | Show reason and retry/diagnostic action. |
| `platform_gated` | The capability is intentionally unavailable on this platform/runtime. | Explain platform gate; do not offer a fake toggle. |

## Shared domain objects

### Identity

The clients consume identity status, selected profile, lock state, setup completion, nickname/callsign, DID, and security posture. Profile creation and unlock are workflows with validation and explicit failure text. The passphrase rule is a minimum of 12 bytes across every interface.

### Transport

Every registered transport is rendered with `name`, `medium`, `state`, `registered`, `started`, `ready`, `deferred`, `reason`, `listen addresses`, `discovery state`, counters, and last error where available. Registry membership is not readiness. Wi-Fi Direct/Aware and BLE remain platform-gated unless a concrete driver reports lifecycle state.

### Discovery evidence

Discovery snapshots contain method-level state and detailed evidence. `waiting_for_peers` is a valid no-peer state. RNS announcements, mDNS observations, world-gossip advertisements, bootstrap records, and manually entered addresses are evidence sources; none is trust authorization by itself. The UI must expose discovery method, first/last observed time, address count, and validation/rejection reason where returned.

### Peer trust

Peer rows separate identity, connection state, trust level, discovery method, latency, last seen, and message-delivery history. A discovered candidate can be inspected or connected but is not automatically trusted. Trust changes require an explicit user action and confirmation.

### Message delivery

Each outgoing message has local lifecycle state: `draft`, `queued`, `sending`, `accepted`, `delivered`, `failed`, or `unknown`. Transport connection must not be displayed as delivery. The Web UI can show richer timelines; the TUI uses compact markers and an inspection key.

## Commands exposed to both clients

Both clients should offer equivalent operations where the backend supports them: refresh health, scan discovery, publish/refresh world discovery, inspect a candidate, connect to a peer, ping, block/unblock, send direct message, send broadcast, view delivery evidence, select profile, unlock profile, lock profile, and open diagnostics. Controls must be disabled or marked unavailable from the canonical state rather than failing only after submission.

## Presentation split

The Web UI is a progressive-disclosure cockpit. It favors grouped navigation, cards, drawers, filters, timelines, and responsive layouts. The TUI remains keyboard-first. `Tab` cycles major views; `d` scans discovery; `p` publishes discovery; `r` refreshes providers; `Enter` selects; `c` connects; `x` blocks; `i` inspects; `m` opens messaging; `s` opens security/system status; and `q` exits. The TUI should display the same semantic labels and use compact help bars rather than duplicating Web UI layout.

## Error policy

Errors must include the affected domain, operation, reason, and whether retry is safe. A transport rejected advertisement should not be silently converted to “offline.” A no-peer scan should not be shown as failed. A missing platform driver should not be shown as a disconnected live transport.
