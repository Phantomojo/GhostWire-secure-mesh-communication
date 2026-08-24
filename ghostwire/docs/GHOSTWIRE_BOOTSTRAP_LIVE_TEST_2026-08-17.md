# GhostWire BootstrapWorker Capability Test — Final Verdict

**Date:** 2026-08-17 UTC
**Author:** Manus AI
**Machines:** Omen (`100.88.234.14`) and EliteKali (`100.110.215.46`)
**Scope:** Separate GhostWire-One bootstrap-provider capability test, intentionally isolated from the earlier Tailscale-only libp2p TCP path test.

## Executive verdict

> **PASS — GhostWire-One BootstrapWorker lifecycle and explicit bootstrap transport path are operational.**

The test proved that an Omen node can be explicitly configured, locally authorized, started, and marked ready from actual listener evidence. A fresh EliteKali node then joined through an operator-supplied GhostWire bootstrap multiaddress over the Omen–EliteKali Tailscale path, established a libp2p connection, completed the GhostWire handshake, and exchanged encryption key material. The test used fresh identities and isolated homes on both machines. Production Omen remained running and unchanged throughout.

This is **not** a claim that the public API currently exercises signed BootstrapRecord publication and admission end to end. That surface is not exposed as a live API endpoint yet. The existing worker unit tests cover scoped signatures, expiry, network binding, and bounded admission; the live test proved the provider capability lifecycle and the explicit bootstrap join path.

## Test configuration and artifacts

| Item | Omen | EliteKali |
|---|---|---|
| Isolated home | `/tmp/ghostwire-bootstrap-omen` | `/tmp/ghostwire-bootstrap-elite` |
| API | `127.0.0.1:9907` | `127.0.0.1:9908` |
| libp2p TCP mesh | `4811` | `4812` |
| QUIC | `8891` | `8892` |
| mDNS | Disabled at behaviour construction | Disabled at behaviour construction |
| Binary build | Omen native release, Reticulum feature | EliteKali native release, Reticulum feature |
| Binary SHA-256 | `ee64313a93c5f0ba28fb505feb769760edfee3b930199d819bb1d4c3a0efb4a0` | `621da2e27aca7605c604b17c2a5dceb799d246f07ef888242efb1bb11ab65c1a` |

EliteKali’s first readiness-wired compile attempt exposed source drift in the previous partial test checkout: hardened persistence and mailbox methods present on Omen were absent on EliteKali. The source-transfer procedure was corrected by replacing the partial checkout with a complete source snapshot using the existing secret/runtime exclusions. No identity, key, mailbox, PID, daemon-argument, log, `.ghostwire`, or target state was transferred. EliteKali then passed formatting, Reticulum-feature all-target compilation, and the native release build.

## Bootstrap provider lifecycle

Omen was started with a fresh profile and authenticated locally through the persisted test token. The API sequence was executed in the required order. Configuration returned `eligible=true`; explicit authorization returned `authorized=true`; and explicit start returned `started=true, ready=true`. The final authenticated BootstrapWorker state was:

```json
{
  "implemented": true,
  "configured": true,
  "eligible": true,
  "authorized": true,
  "started": true,
  "ready": true,
  "verified": false,
  "degraded": false
}
```

The Omen test PeerID was `12D3KooWB6YMScvha52Ed6XHJFa7Sp2NsSRgtmBRj9ZECPCVGrCS`. The `verified=false` value is expected in this run because the current public API does not expose signed BootstrapRecord publication. It does not mean that listener readiness or the explicit bootstrap handshake failed.

## Readiness-contract correction

The first run exposed a real contract defect. The worker transitioned to `started=true` but stayed at `ready=false` after the mesh listener had bound and the peer handshake had completed. That state was technically conservative but operationally incomplete because the API and cockpit could not reflect actual runtime readiness.

`Core::start_bootstrap()` now performs a runtime confirmation after explicit worker start. It checks for an actual mesh listener or an initialized Reticulum interface and calls `mark_ready(true)` only when one exists. When no listener exists, it continues to report started-but-not-ready. This removes the manual-only readiness gap without claiming readiness merely because the capability compiled or was configured.

## Explicit bootstrap join evidence

EliteKali was started with the exact Omen Tailscale multiaddress:

```text
/ip4/100.88.234.14/tcp/4811/p2p/12D3KooWB6YMScvha52Ed6XHJFa7Sp2NsSRgtmBRj9ZECPCVGrCS
```

EliteKali’s log recorded the following sequence:

```text
Adding bootstrap node: 12D3KooWB6YMScvha52Ed6XHJFa7Sp2NsSRgtmBRj9ZECPCVGrCS (/ip4/100.88.234.14/tcp/4811/p2p/12D3KooWB6YMScvha52Ed6XHJFa7Sp2NsSRgtmBRj9ZECPCVGrCS)
Dialing configured GhostWire bootstrap
Connection established with 12D3KooWB6YMScvha52Ed6XHJFa7Sp2NsSRgtmBRj9ZECPCVGrCS
Received Handshake from 12D3KooWB6YMScvha52Ed6XHJFa7Sp2NsSRgtmBRj9ZECPCVGrCS
Stored encryption key for peer 12D3KooWB6YMScvha52Ed6XHJFa7Sp2NsSRgtmBRj9ZECPCVGrCS
```

EliteKali’s fresh PeerID was `12D3KooWGa6PPtd8T8u2TbKtfm3CPHY2v73iFbJG8WM75ZVxWcrR`. Its authenticated transport snapshot reported one connected mesh peer. Omen’s corresponding snapshot reported EliteKali as connected with discovery method `mesh` and one mesh peer.

The socket proof showed Omen’s `100.88.234.14:4811` established to EliteKali’s `100.110.215.46` endpoint. The reciprocal EliteKali socket showed the same Tailscale address pair. No `192.168.100.x` data connection appeared in the captured endpoint set, and mDNS was disabled on both nodes. The bootstrap connection was therefore isolated from the shared-Wi-Fi discovery path for this test.

## Production preservation and cleanup

The production Omen daemon was not stopped, restarted, upgraded, or reconfigured. At cleanup it remained:

```text
PID 932431 ./target/release/ghostwire --daemon --port 9097 --mesh-port 4001
TCP 127.0.0.1:9097
TCP 0.0.0.0:4001
UDP 0.0.0.0:8081
```

Both isolated daemons were stopped through their isolated `GHOSTWIRE_HOME` profiles. Omen test listeners `9907`, `4811`, and `8891` were closed. EliteKali test listeners `9908`, `4812`, and `8892` were closed. No production listener changed.

## What remains unproven

The following items remain deliberately outside this PASS. The signed BootstrapRecord publication/admission API is not yet exposed for an end-to-end live call, so `verified=true` was not expected. The earlier Tailscale test proved the authenticated libp2p TCP path; it did not independently prove a GhostWire QUIC application exchange. Same-Wi-Fi TCP/QUIC and mDNS discovery remain separate phases and must be run with LAN endpoint proof. RNS over an explicit GhostWire-controlled peer path remains conditional. Wi-Fi Direct, Wi-Fi Aware/NAN, BLE, and stealth TCP are not production-ready routes; their interfaces must remain platform-gated rather than falsely reported as available.

The final serial release suite should be rerun after the readiness patch and explicit bootstrap-dial patch before a stable-release tag. The public record-publication route, admission integration, and provider record rotation should then be added and tested independently. These are follow-up hardening items, not failures of the capability lifecycle test reported here.

## Source evidence

The detailed executable test plan, including the prior containment finding and the post-fix BootstrapWorker section, is maintained in [`ghostwire_live_test_plan.md`][1]. The longitudinal audit log is maintained in [`ghostwire_audit_progress.md`][2].

[1]: ./ghostwire_live_test_plan.md "GhostWire two-machine live-test plan"
[2]: ./ghostwire_audit_progress.md "GhostWire audit progress log"
