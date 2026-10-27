# GhostWire Private RNS Design (ghRNS)

## Concept

GhostWire should operate on a **private Reticulum overlay network** — not the public RNS network. Like ProtonMail's private mail infrastructure that uses standard email protocols but is isolated, GhostWire users should be on a GhostWire-specific RNS network that is invisible to public RNS nodes.

This is not about rejecting RNS — it's about using RNS's built-in privacy features (IFAC, encrypted discovery, network identity) to create a GhostWire-only network overlay.

## Why (Security/Privacy/Anonymity Lens)

### Current State (Public RNS)
- AutoInterface discovers ALL RNS nodes on the local network
- No IFAC — any RNS node can send packets to any other
- No encrypted discovery — any RNS node can see GhostWire announces
- GhostWire nodes are visible to public RNS network
- Public RNS nodes can potentially route to GhostWire nodes

### Desired State (Private RNS / ghRNS)
- Only GhostWire nodes can discover each other
- Only GhostWire nodes can send packets on the network
- Public RNS nodes cannot see GhostWire traffic
- GhostWire nodes are invisible to public RNS
- Source anonymity preserved (RNS never includes source addresses)

## RNS Mechanisms Available

### 1. Interface Access Codes (IFAC)
- Named virtual networks with passphrase authentication
- Packets carry Ed25519 signatures derived from shared secret
- Only nodes with correct network_name + passphrase can generate valid packets
- IFAC can be full 512-bit or truncated (configurable via ifac_size)
- Upon receipt, interface checks signature and drops invalid packets

### 2. Network Identity
- Shared identity used to sign/encrypt discovery announces
- Only peers with this identity can decrypt discovery
- Distributed via standard announce mechanism
- Configured in [reticulum] section

### 3. Encrypted Discovery
- discovery_encrypt = yes encrypts announce payload
- Only peers with network_identity can decode
- publish_ifac = yes includes IFAC credentials in encrypted announce
- Allows seamless onboarding for trusted peers

### 4. Interface Modes
- full: standard bidirectional
- gateway: connects networks
- access_point: accepts connections
- roaming: mobile node
- boundary: network boundary

## GhostWire RNS Design

### Network Parameters
| Parameter | Value | Source |
|---|---|---|
| Network name | "ghostwire-mesh-v1" | Protocol constant |
| Network passphrase | Derived from GhostWire master seed + protocol version | Deterministic |
| Network identity | Derived from GhostWire protocol constants | Deterministic |
| Destination name | "ghostwire.mesh" | Protocol constant |
| IFAC size | Full (512-bit) for TCP, truncated for BLE/radio | Per-interface |
| Discovery encrypt | Yes | Always |
| Publish IFAC | Yes (in encrypted announce) | Always |

### Interface Configuration
```
[reticulum]
network_identity = <derived from GhostWire protocol constants>

# GhostWire private backbone (TCP)
[[GhostWire Backbone]]
type = BackboneInterface
enabled = yes
mode = gateway
listen_on = 0.0.0.0
port = 4242
network_name = ghostwire-mesh-v1
passphrase = <derived from GhostWire master seed>
# Encrypted discovery
discoverable = yes
discovery_name = GhostWire Mesh Node
discovery_encrypt = yes
publish_ifac = yes

# GhostWire local discovery (AutoInterface with IFAC)
[[GhostWire Local]]
type = AutoInterface
enabled = yes
network_name = ghostwire-mesh-v1
passphrase = <derived from GhostWire master seed>
```

### Identity Derivation
- RNS identity: Already derived from GhostWire identity via `derive_transport_key("reticulum")`
- Network identity: Derived from GhostWire protocol constants (not user-specific)
- IFAC passphrase: Derived from GhostWire master seed + protocol version (deterministic, same for all GhostWire nodes)

### Discovery Flow
1. GhostWire node starts RNS with GhostWire IFAC on all interfaces
2. Node sends encrypted announce (only GhostWire nodes can decode)
3. Announce includes IFAC credentials (encrypted, only GhostWire nodes can read)
4. Other GhostWire nodes decode announce, extract IFAC, connect automatically
5. Public RNS nodes see encrypted announce but cannot decode → ignore
6. Public RNS nodes cannot send packets (IFAC signature invalid) → dropped

### Bridge to Public RNS (Optional)
- GhostWire nodes can optionally act as gateways between ghRNS and public RNS
- This allows GhostWire users to communicate with public RNS users if they choose
- By default, GhostWire nodes are isolated (no bridge)
- Bridge mode is explicitly enabled by operator

## Security/Privacy/Anonymity Evaluation

### Security
- IFAC prevents non-GhostWire nodes from sending packets (signature verification)
- Network identity prevents non-GhostWire nodes from decoding discovery
- GhostWire-specific ports prevent accidental connection to public RNS
- Source anonymity preserved (RNS never includes source addresses)

### Privacy
- GhostWire node presence invisible to public RNS (encrypted announces)
- GhostWire traffic invisible to public RNS (IFAC filters)
- Only GhostWire nodes can discover each other
- Topology exposure limited to GhostWire network

### Anonymity
- Source addresses never included in packets (RNS property)
- With IFAC, even destination is hidden from non-GhostWire observers
- Multi-hop routing provides additional unlinkability
- GhostWire identity (X25519) separate from RNS identity

## Implementation Plan

### Phase 1: IFAC on all interfaces
- Add network_name and passphrase to default interface config
- Derive passphrase from GhostWire protocol constants
- Set ifac_size appropriately per interface type

### Phase 2: Network identity
- Generate GhostWire network identity from protocol constants
- Configure in ReticulumManager
- Enable discovery_encrypt on all interfaces

### Phase 3: GhostWire-specific destination
- Change destination name from "mesh" to "ghostwire.mesh"
- Update announce format to include GhostWire-specific metadata

### Phase 4: Topology sync (Gap 1 from RNS audit)
- Sync ReticulumTopology → MeshTopology
- RNS-discovered peers appear in main mesh topology

### Phase 5: Announce sending (Gap 2 from RNS audit)
- Send RNS announce on startup
- Periodic re-announce
- Include GhostWire-specific metadata

### Phase 6: Bridge mode (optional)
- Allow operator to enable public RNS bridge
- Gateway mode on specific interfaces
- Explicit opt-in

## What This Gives GhostWire

1. **Invisible to public RNS** — GhostWire nodes don't appear on public RNS network
2. **Private discovery** — Only GhostWire users can find each other via RNS
3. **Authenticated transport** — IFAC ensures only GhostWire nodes can send packets
4. **Source anonymity** — RNS never includes source addresses
5. **Multi-media resilience** — RNS works over radio, serial, TCP, UDP, BLE
6. **Interoperability** — Can bridge to public RNS if needed (gateway mode)
7. **Scalability** — RNS handles routing, announce, and store-and-forward

## Comparison: GhostWire RNS vs Public RNS vs libp2p

| Feature | Public RNS | ghRNS (GhostWire) | libp2p |
|---|---|---|---|
| Discovery | Public announces | Encrypted announces | mDNS + DHT |
| Authentication | None (open) | IFAC (shared secret) | Noise (per-peer) |
| Source anonymity | Yes | Yes | No (PeerId visible) |
| Multi-media | Yes | Yes | No (IP only) |
| Offline capability | Yes (store-and-forward) | Yes | No |
| Privacy from network | Medium | High | Low |
| Interoperability | Universal RNS | GhostWire only (or bridge) | Universal libp2p |

## Implementation Priority

1. **IFAC configuration** — highest impact, lowest effort (config change)
2. **Network identity** — medium effort, high impact
3. **Topology sync** — medium effort, critical for RNS to be useful
4. **Announce sending** — low effort, critical for discovery
5. **Bridge mode** — low priority, optional feature
EOF
echo "written"