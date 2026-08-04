# GhostWire libp2p Feature Gap Analysis

**Date**: July 28, 2026  
**libp2p version**: 0.56 (current), 0.57 (latest)

---

## Current Usage vs Full Feature Set

### ✅ Already Using
- TCP, Noise, Yamux, Gossipsub, Kademlia, mDNS, Identify, AutoNAT, Relay (client), DCUtR, Request-Response, CBOR, DNS (implicit)

### 🔴 Reinventing (Replace with libp2p)
| Custom Code | Replace With | Priority |
|-------------|--------------|----------|
| Custom relay/proxy logic | `libp2p::relay::server::Behaviour` | **HIGH** |
| Custom keep-alive/heartbeat | `libp2p_ping::Behaviour` | HIGH |
| Manual connection counting | `libp2p::connection_limits::Behaviour` | HIGH |
| Custom PeerStore | libp2p Identify + Kademlia routing table | MEDIUM |

### 🟢 Missing (Add These)
| Feature | What It Does | Priority |
|---------|-------------|----------|
| **Relay server** | Act as relay for other NAT'd peers | **HIGH** |
| **TCP fallback** | QUIC-only fails on restrictive networks | **HIGH** |
| **WebSocket** | Browser node support | **HIGH** |
| **Rendezvous** | Simpler mesh discovery than DHT | MEDIUM |
| **Allow/Block lists** | IP CIDR + PeerId filtering | MEDIUM |
| **Metrics** | Prometheus/OpenMetrics observability | MEDIUM |
| **UPnP** | Auto port mapping on consumer routers | LOW |
| **Pnet (PSK)** | Private network deployments | LOW |

---

## What Big Projects Use That We're Missing

| Project | Key libp2p Features We Don't Use |
|---------|----------------------------------|
| **IPFS** | Circuit Relay server, UPnP, Bitswap (req-resp), 85K+ DCUtR measurements |
| **Ethereum 2.0** | Heavily customized gossipsub scoring, Discv5 discovery |
| **Polkadot** | Custom PeerStore with reputation, Notification protocols, litep2p native reimplementation |

---

## Recommended Cargo.toml Additions

```toml
libp2p = { version = "0.56", features = [
    # Already have
    "tokio", "macros", "tcp", "dns", "noise", "yamux",
    "kad", "gossipsub", "mdns", "identify", "autonat",
    "relay", "dcutr", "request-response", "cbor",
    # ADD THESE
    "ping",              # Replace custom heartbeat
    "quic",              # Native libp2p QUIC (replace standalone quinn)
    "websocket",         # Browser support
    "tls",               # Alternative security transport
    "rendezvous",        # Structured mesh discovery
    "connection-limits", # Enforce connection caps
    "allow-block-list",  # IP/PeerId filtering
    "metrics",           # Prometheus instrumentation
    "upnp",              # Auto port mapping
] }
```

---

## Summary: Stop Reinventing

1. **Custom heartbeat → libp2p ping** — free, spec-compliant, battle-tested
2. **Custom connection counting → connection-limits** — built-in, handles edge cases
3. **No relay server → add relay server** — help other peers, earn reputation
4. **QUIC-only → add TCP fallback** — enterprise firewalls block UDP
5. **No browser path → add WebSocket** — WASM builds need it
6. **Peer scoring code exists but unwired → wire it** — skademlia.rs has the params, just not connected
