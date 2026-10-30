# GhostWire Genesis Ceremony — Design Document

## Overview

The Genesis Ceremony is GhostWire's foundational trust anchor. It serves as the
network's "constitution" — the set of rules, parameters, and identity seeds that
define what GhostWire IS and who can participate. Unlike a traditional PKI or
central authority, the Genesis Ceremony is:

- **Versioned**: `genesis-1`, `genesis-2`, etc. — each version is a distinct document
- **Signed**: Ed25519 signatures by the author(s) who authorized the change
- **Evolvable**: can be changed, but only through a ceremony that requires
  explicit authorization
- **Plausibly deniable**: the author can credibly claim they cannot reproduce or
  modify the ceremony without the proper authorization chain

## Current State (genesis-0)

The current genesis is implicit — hardcoded in the codebase:

- `src/core/genesis_constants.rs` — contains hardcoded constants
- IFAC seed generation (`get_or_create_ifac_seed()`) — per-installation
- Network name: `ghostwire-mesh-v1`
- Discovery topic: derived from network ID
- Encryption: X25519 + AES-256-GCM + ML-KEM-768

The current genesis is **static** — it works but cannot evolve. Changing any
parameter requires a code change and rebuild.

## Target State (genesis-N)

### Genesis Document Structure

```
genesis-N = {
    version: u32,                    // N
    previous_genesis_hash: [u8; 32], // hash of genesis-(N-1) document
    created_at: u64,                 // Unix timestamp
    author_pubkey: [u8; 32],         // Ed25519 public key of authorizing party
    signature: [u8; 64],             // Ed25519 signature over all fields above

    // Network parameters
    network_name: String,            // e.g. "ghostwire-mesh-v2"
    ifac_network_name: String,       // RNS IFAC network name
    discovery_topic: String,         // Gossipsub discovery topic
    broadcast_topic: String,         // Gossipsub broadcast topic
    dm_topic: String,               // Gossipsub DM topic

    // Crypto parameters
    min_encryption: String,          // minimum cipher suite
    pq_required: bool,               // whether PQ hybrid is mandatory
    min_sphinx_hops: u8,             // minimum Sphinx routing hops

    // Transport parameters
    default_transport: String,       // "hybrid", "libp2p-only", "reticulum-only"
    rns_enabled: bool,
    ble_enabled: bool,
    wifi_enabled: bool,

    // Privacy parameters
    default_privacy_tier: u8,        // L0-L3
    cover_traffic_enabled: bool,
    sealed_sender_required: bool,

    // Governance
    governance_model: String,        // "dictator", "community", "hybrid"
    successor_pubkeys: Vec<[u8; 32]>, // authorized successors
    change_threshold: u8,            // signatures needed for community changes
}
```

### Versioning Rules

1. **genesis-0** is the implicit current state (no document, hardcoded)
2. **genesis-1** is the first explicit document, capturing all current parameters
3. Each subsequent version increments N
4. A node running genesis-N can communicate with nodes running genesis-(N-1)
   for a transition period (configurable, default 30 days)
5. After the transition period, genesis-(N-1) nodes are considered legacy

### Change Process

#### Critical Changes (require dictator authorization)
- Network name changes
- Crypto parameter changes (min_encryption, pq_required)
- Governance model changes
- Successor list changes

Process:
1. Author proposes new genesis document
2. Author signs with their Ed25519 private key
3. Document is distributed via out-of-band channel (signed invite, QR code, etc.)
4. Nodes that receive and verify the document update their genesis version
5. Old genesis is phased out after transition period

#### Day-to-Day Changes (community governance)
- Transport parameter tuning
- Privacy tier defaults
- Discovery interval changes
- Cover traffic parameters

Process:
1. Community proposes change via signed PR
2. Maintainers review and merge
3. Change is included in next release
4. No genesis version bump needed (these are code-level, not protocol-level)

### Plausible Deniability

The genesis ceremony provides plausible deniability through:

1. **Commit-reveal structure**: The author commits to a hash before revealing
   the full document. This prevents the author from retroactively changing
   the ceremony.

2. **AI-assisted generation**: The genesis document can be generated with AI
   assistance. The AI conversation is ephemeral (web-based, not persisted).
   The author can credibly claim they don't remember the exact inputs.

3. **Multi-party authorization**: For critical changes, multiple signatures
   are required. No single party can unilaterally modify the genesis.

4. **Open-source transparency**: The genesis document is public. Anyone can
   verify the signature and contents. But the author's private key is needed
   to create new versions.

### Governance Model

```
┌─────────────────────────────────────────────────┐
│                 GENESIS (Constitution)           │
│  - Network identity                             │
│  - Crypto floor                                 │
│  - Governance rules                             │
│  - Successor list                               │
│                                                 │
│  Authority: DICTATOR (author)                   │
│  Change: Requires signature + ceremony           │
│  Frequency: Rare (protocol-level changes)        │
└────────────────────┬────────────────────────────┘
                     │ constrains
┌────────────────────┴────────────────────────────┐
│                 CODE (Legislation)              │
│  - Transport implementations                    │
│  - Privacy tier defaults                        │
│  - Discovery intervals                          │
│  - UI/UX                                        │
│                                                 │
│  Authority: COMMUNITY (maintainers + contributors)│
│  Change: Standard PR process                    │
│  Frequency: Regular (feature development)       │
└─────────────────────────────────────────────────┘
```

### Successor Transfer

When the author is ready to transfer control:

1. Author generates a new genesis document (genesis-N+1)
2. New document includes successor's public key in `successor_pubkeys`
3. Author signs the document
4. Successor receives the private key (out of band)
5. Successor can now create genesis-N+2
6. Old author's key is retired (but remains in history for verification)

### Implementation Plan

#### Phase 1: Genesis Document Format (current)
- Define genesis document structure (this document)
- Create `src/core/genesis.rs` with GenesisDocument struct
- Add serialization/deserialization (bincode + JSON)
- Add Ed25519 signing/verification

#### Phase 2: Genesis Storage
- Store genesis document in profile directory
- Load on startup, verify signature
- Fall back to genesis-0 (implicit) if no document found

#### Phase 3: Genesis Distribution
- Add API endpoint: GET /v2/genesis (returns current genesis document)
- Add API endpoint: POST /v2/genesis/apply (applies a new genesis document)
- Verify signature before applying
- Reject downgrades (genesis-N+1 required, not genesis-N-1)

#### Phase 4: Genesis Evolution
- Add CLI command: `ghostwire genesis propose` (creates new genesis document)
- Add CLI command: `ghostwire genesis sign` (signs with private key)
- Add CLI command: `ghostwire genesis verify` (verifies signature)
- Add transition period handling (accept both old and new genesis)

### Security Properties

- **No central PKI**: Trust is established through the genesis document, not
  a certificate authority
- **No backdoor capability**: The author cannot modify the genesis without
  their private key, which can be destroyed or transferred
- **Verifiable**: Anyone can verify the genesis signature and contents
- **Transparent**: The genesis document is public and auditable
- **Resilient**: Multiple successors can be authorized for redundancy

### Legal Considerations

The genesis ceremony provides a verifiable answer to the question
"Can you add a backdoor?":

- The genesis document is signed and public
- Modifying it requires the author's private key
- The author can credibly claim they cannot modify it without the key
- If the key is destroyed or transferred, the author has no capability
- The open-source codebase can be audited by anyone

This is not a guarantee against coercion, but it provides a strong
technical and social defense.
