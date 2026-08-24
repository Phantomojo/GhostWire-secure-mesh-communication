# GhostWire Bootstrap Worker and Node Bootstrap Audit

Date: 2026-08-17
Canonical checkout: `/home/ph/GhostWire-secure-mesh-communication-newest`
Target: omen

## Current codebase findings

GhostWire currently has bootstrap address configuration and startup dialing, but no clearly identified dedicated bootstrap worker or first-class bootstrap-service lifecycle. `MeshTransport` contains a hard-coded `DEFAULT_BOOTSTRAPS` list, adds configured addresses during startup, and calls Kademlia bootstrap behavior. `Core` substitutes public libp2p/IPFS-style bootstrap addresses when none are configured. Dynamic bootstrap addition is explicitly unsupported in the actor-model path. Startup health reports bootstrap counts, but this is configuration/attempt state rather than verified bootstrap service health.

The current architecture therefore distinguishes configured bootstrap peers only weakly. It does not yet clearly distinguish:

- a bootstrap seed used by a joining node;
- a relay or rendezvous service;
- a normal GhostWire node that happens to accept inbound peers;
- an operator-authorized bootstrap service;
- a permanently trusted authority.

That ambiguity is dangerous. A node being reachable or useful must not imply that it is trusted, authoritative, or eligible to advertise network membership.

## Required target architecture

Bootstrap should be a capability with explicit state and policy, not a permanent identity role. A node may offer bootstrap service only when local policy enables it, the node satisfies resource and reachability requirements, its identity is stable, and the operator has authorized the capability. The capability should expose `implemented`, `configured`, `eligible`, `started`, `ready`, `verified`, and `degraded` separately.

A bootstrap worker should supervise:

1. listener and advertised-address validation;
2. bounded inbound admission and per-peer/resource quotas;
3. Kademlia routing-table refresh and bucket diversity;
4. signed bootstrap-record publication and expiry;
5. health, uptime, reachability, and load measurements;
6. bootstrap peer rotation and failure backoff;
7. abuse controls, rate limiting, connection caps, and temporary quarantine;
8. graceful shutdown and state persistence;
9. privacy-minimized telemetry;
10. separation between discovery evidence and trust/authorization.

The worker should never mint trust, approve peers, decrypt messages, decide revocation, or become a single point of authority. Bootstrap records should be signed, versioned, scoped to a network/genesis domain, include expiry and observed addresses, and be accepted as discovery hints only. A joining node still needs an authenticated GhostWire handshake and local admission policy.

## Any-node bootstrap eligibility

Any node should be able to become a bootstrap provider only through explicit capability negotiation and local authorization. Automatic promotion based solely on uptime, public reachability, peer count, or a high score is unsafe. Eligibility should be computed from measured conditions such as stable identity persistence, healthy listener, reachable advertised address, sufficient storage and bandwidth budget, acceptable clock/state health, and operator policy. Eligibility is not activation; activation requires an explicit control-plane mutation and a signed/authorized local policy.

A normal node can still help peers opportunistically through ordinary peer exchange without becoming a bootstrap service. The protocol should support tiered roles:

| Role | Meaning | Authority |
|---|---|---|
| Seed hint | Static or cached address used for first contact | None; discovery only |
| Bootstrap provider | Bounded peer/DHT entry service | No trust authority |
| Relay/rendezvous | Connectivity assistance or introduction | No message decryption or trust authority |
| Ordinary node | Participates in routing and discovery | No special authority |
| Operator-pinned seed | Locally trusted discovery input | Trust is local policy, not protocol-wide |

## Security and resilience requirements

Bootstrap must resist Sybil and eclipse pressure through diverse seeds, signed network-domain records, routing-table diversity, address and peer limits, rotation, expiry, and multiple independent bootstrap sources. A single bootstrap provider must not be able to define membership, suppress peers, or become the only path to the network. Bootstrap metadata should minimize exposure of node identity, exact location, activity timing, and peer graph; public health should be coarse and rate-limited.

Kademlia bootstrap is routing-table initialization and peer discovery, not authenticated membership. libp2p channels authenticate peer identity at the transport/protocol layer, but application authorization remains GhostWire policy. mDNS is local-link discovery and must remain untrusted evidence. Bootstrap records need an authenticated GhostWire handshake before the peer becomes usable for application traffic.

## Implementation sequence

1. Inventory current bootstrap worker/config paths and prove whether one exists.
2. Introduce a typed bootstrap capability descriptor and truthful health projection.
3. Add a supervised bootstrap worker with lifecycle, quotas, refresh, backoff, and graceful shutdown.
4. Replace hard-coded public default bootstrap assumptions with explicitly scoped configuration and safe cached peers.
5. Add signed, expiring bootstrap records bound to GhostWire genesis/network domain.
6. Add unit and integration tests for admission, expiry, diversity, rate limits, no-authority behavior, and failure recovery.
7. Only then enable Omen/EliteKali live bootstrap testing using isolated identities and test profiles.

## External references

[1] libp2p Kademlia DHT documentation: https://libp2p.io/docs/kademlia-dht/
[2] Rust libp2p discovery configuration discussion: https://discuss.libp2p.io/t/how-to-configure-discovery-when-using-rust-libp2p/1425
[3] Rust libp2p Kademlia API documentation: https://docs.rs/fluence-fork-libp2p/latest/libp2p/kad/struct.Kademlia.html
[4] IPFS Rust libp2p bootstrap-node architecture article: https://blog.ipfs.tech/2023-rust-libp2p-based-ipfs-bootstrap-node/
