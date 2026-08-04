# constrained-mesh-patterns
- Date: 2026-08-02
- Agent: librarian (session: bg_1604bd4e)
- Status: raw

---

# Constrained Mesh Device Patterns Research

**Date:** 2026-08-02
**Agent:** librarian
**Source:** bg_1604bd4e

## Core Finding

Every constrained mesh project uses a role enum with per-role default profiles, not binary full/leaf split. The pattern is 4 dimensions: forwarding duty, power policy, storage tier, dialability.

## Project Findings

### Meshtastic (13 device roles)
- Role enum: CLIENT, CLIENT_MUTE, ROUTER, ROUTER_CLIENT, REPEATER, TRACKER, SENSOR, TAK, CLIENT_HIDDEN, LOST_AND_FOUND, TAK_TRACKER, ROUTER_LATE, CLIENT_BASE
- installRoleDefaults: single switch sets per-role profiles
- Rebroadcast modes: ALL, ALL_SKIP_DECODING, LOCAL_ONLY, KNOWN_ONLY, NONE, CORE_PORTNUMS_ONLY
- is_unmessagable flag: routers/trackers/sensors announced but cannot be DM'd
- WarmNodeStore: 4-bit role in flash, hot/warm tiers
- Role restriction: ROUTER locked to authenticated builds only

### Berty (mobile nodes drop DHT)
- Node types: peer, replication, relay, bootstrap, rdvp
- Mobile peers: disable mDNS, DHT, listeners — outbound-only
- Sacrifices: discovery, availability, addressability
- Replication: stores encrypted data without decrypting

### Briar (storage-constrained sync)
- BSP: bounded offer lists, client-controlled deletion
- Mailbox: external always-on buffer for leaf devices
- LPN/friend model from BLE Mesh

### libp2p Circuit Relay (bootstrap, not data path)
- 128 max reservations, 2-min circuit duration, 128KiB limit
- Designed for hole-punching bootstrap (dcutr), not sustained traffic
- For sustained leaf: need persistent relays or renewal

### Academic (Sensors 2025)
- BT Mesh + libp2p split: delegate discovery to radio, keep libp2p on gateway
- ~8s latency, 95-99% PDR — viable when host is gateway

## Pattern Summary

| Pattern | Source | Translation |
|---------|--------|-------------|
| Role enum with per-role defaults | Meshtastic | Leaf/Relay/Full as profiles |
| Dial-only leaf, no DHT | Berty | Outbound-only, discovery via rendezvous |
| External store/forward | Berty replication, Briar mailbox | Replicator capability |
| Relay as bootstrap | libp2p relay + dcutr | Leaf reachable via reservation |
| Bounded storage + client deletion | Briar BSP, Meshtastic WarmNodeStore | Hot/warm tiers |
| Capability = security gate | Meshtastic ROUTER restriction | Authenticated builds only |
