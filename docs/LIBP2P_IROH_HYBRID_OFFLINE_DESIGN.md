# GhostWire libp2p + Iroh Hybrid Data-Plane Design

> **Status:** Offline architecture proposal only. No Iroh dependency, network behavior, or interoperability claim has been added to the codebase. The exact crate APIs, relay behavior, discovery semantics, and security properties must be validated against current upstream documentation before implementation begins.

## Purpose

GhostWire already has two distinct networking layers. The libp2p mesh owns peer discovery, gossip, routing, relay-related behavior, and decentralized control traffic. A separate Quinn-based direct transport can carry a serialized `Message` through QUIC once GhostWire has a socket address and has established trust-on-first-use state. The proposed hybrid must strengthen large-object and high-throughput delivery **without creating an alternate trust system, weakening anonymous-routing policy, or leaking recipient metadata through an uncontrolled direct path**.

The design therefore assigns exactly one role to each plane. **libp2p remains the control plane and policy authority.** A future Iroh integration becomes an optional, capability-gated data plane for encrypted payload bytes or content-addressed objects. It must never become a second social graph, a replacement identity registry, or an automatic bypass around GhostWire's onion, delay, DTN, revocation, and transport-selection policy.

| Plane | Existing GhostWire responsibility | Proposed responsibility | Must not do |
|---|---|---|---|
| libp2p mesh | Peer discovery, GossipSub, Kademlia, bootstrap, CRL gossip, message routing | Authenticate and distribute short-lived data-plane offers; decide whether direct transfer is policy-allowed | Carry arbitrary attachment bytes by default or disclose data-plane endpoint data to unapproved observers |
| Iroh data plane | Not yet present | Efficient transfer of application-ciphertext blobs after a recipient accepts a signed, expiring offer | Establish GhostWire identity trust, publish social metadata, or override transport privacy policy |
| Existing Quinn transport | Direct `Message` delivery with per-peer TOFU certificate fingerprints | Transitional fallback while a dedicated blob data plane is developed | Remain a separate, unbounded identity and framing silo |
| GhostWire crypto layer | Identity, hybrid encryption, CRL, message encryption and signatures | Bind a data-plane endpoint key to the GhostWire identity and encrypt every data-plane object | Delegate authorization to a transport implementation |

## Current-code baseline

The current design provides clear insertion points, but it also exposes constraints that the integration must respect. `Core::new` registers the Quinn provider in `src/core/mod.rs`; `TransportKind` in `src/core/transport.rs` dispatches directly to mesh, Quinn, Stealth TCP, Bluetooth, and simulated transports; and `QuicTransportProvider` in `src/core/quic_transport.rs` owns a `quinn::Endpoint`, a peer-to-socket-address map, active connections, and TOFU fingerprints. The QUIC listener accepts an unbounded length field and then allocates a `Vec<u8>` of that size, which is a separate hardening issue to resolve before borrowing its design for an Iroh adapter.

The libp2p mesh already has the appropriate governance hooks. It receives gossip events, rejects revoked peers before connection/message handling, gossips CRL entries, and manages causal ordering through vector clocks. The hybrid should reuse those hooks to govern data-plane offer issuance, acceptance, cancellation, and audit events. It should not add a public data-plane discovery channel that bypasses CRL verification or trust policy.

## Trust and identity binding

A transport endpoint key is not automatically a GhostWire identity. Before a remote endpoint can transfer bytes, the recipient must receive a **signed DataPlaneOffer** through a GhostWire-authorized control path and bind it to the known sender identity.

```text
DataPlaneOffer v1
  protocol_version: u16
  transfer_id: random 256-bit identifier
  issuer_ghostwire_key_id: canonical GhostWire key identifier
  issuer_libp2p_peer_id: current control-plane peer identifier
  data_plane_public_key: Iroh endpoint public key or equivalent
  encrypted_locator: recipient-encrypted endpoint/ticket material
  object_manifest_hash: BLAKE3 hash of encrypted object manifest
  max_bytes: u64
  expires_at: bounded timestamp
  privacy_class: {direct_allowed | relay_preferred | anonymous_only}
  recipient_binding: hash of intended recipient identity/key
  signature: GhostWire hybrid signature over every preceding field
```

The offer must be encrypted to the intended recipient wherever it contains an endpoint locator, relay routing material, or ticket-like bearer capability. Observers of a broadcast gossip channel must not learn that a particular recipient is about to retrieve a specific object. The receiver verifies the GhostWire signature, expected recipient binding, expiry, CRL state of the issuer, object-size policy, and explicit privacy class before it initializes a data-plane transfer.

| Verification | Required result | Failure handling |
|---|---|---|
| GhostWire signing key is recognized and not revoked | The offer is authenticated by the current trust policy | Drop offer; emit a local security event without revealing the locator |
| Recipient binding matches a local authorized identity | The offer cannot be replayed to another participant | Drop offer and mark transfer ID consumed only locally |
| Offer timestamp and byte bound are within policy | Resource use remains bounded | Reject before any data-plane connection attempt |
| Data-plane endpoint key is signed by the issuer | Endpoint cannot be silently substituted after control-plane authorization | Treat as identity-binding failure; do not TOFU a replacement |
| Privacy class permits direct exchange on this route | Direct path is an explicit opt-in | Fall back to Sphinx, DTN, Reticulum, or defer delivery |

## Privacy policy

A direct data plane can reveal network reachability and timing to the two transfer participants. That may be acceptable for a contact-approved bulk transfer but is incompatible with anonymous-only delivery. The transport decision must be made by GhostWire policy before an offer is generated, not by an Iroh connection fallback.

The proposed policy has three explicit classes. `direct_allowed` permits a direct encrypted transfer when both parties are trusted and policy enables it. `relay_preferred` allows a direct transfer only if the selected Iroh configuration uses the approved relay/privacy route; otherwise GhostWire defers or falls back. `anonymous_only` prohibits direct endpoint disclosure and permits only existing onion, mix-delay, DTN, or Reticulum-compatible paths. A message's privacy class must be immutable once its manifest is signed.

The control plane should receive coarse transfer lifecycle events only: offer issued, accepted, completed, cancelled, expired, and failed with a local reason code. It must not publish plaintext filenames, object hashes, byte counts, recipient endpoint identifiers, progress timestamps, or raw Iroh tickets on public gossip topics.

## Data model and actor boundaries

The integration should avoid repeating the present `RwLock<MeshManager>` pattern. A bounded command actor gives one owner to mutable data-plane state and makes cancellation and pressure behavior testable.

```text
Core / application
        |
        | DataPlaneCommand over bounded mpsc
        v
HybridDataPlane actor ------------------> Iroh runtime / endpoint
        |                                      |
        | TransferEvent over bounded broadcast  | encrypted bytes only
        v                                      v
NetworkStack policy handle <--------- local encrypted object store
        ^
        | signed control messages via libp2p mesh
libp2p control plane
```

The actor should expose an opaque `HybridDataPlaneHandle`, rather than an endpoint or connection map. Its initial command surface should remain deliberately small.

| Command | Inputs | Reply | Security boundary |
|---|---|---|---|
| `PrepareOffer` | recipient policy, encrypted manifest, maximum bytes | signed encrypted offer or policy denial | Validates CRL, recipient trust, privacy class, and local quotas |
| `AcceptOffer` | authenticated offer | transfer handle or rejection | Revalidates every signed field and records one-time transfer state |
| `CancelTransfer` | transfer ID and reason | acknowledgement | Removes ephemeral capability and closes local state |
| `QueryTransfer` | local transfer ID | coarse local status | Never exposes endpoint locators through the web/TUI API by default |
| `Shutdown` | none | acknowledgement | Cancels active work with a bounded grace period and zeroizes ephemeral material |

`TransferEvent` must use a bounded channel with an explicit drop/backpressure policy. It should carry identifiers that are local to the UI session rather than remote endpoint identifiers. Sensitive tokens, endpoint key material, recipient locators, and transfer encryption keys must live in zeroizing/secrecy wrappers and must not implement `Debug` or `Serialize`.

## Object and message flow

1. The sender encrypts the attachment or bulk payload using existing GhostWire envelope semantics before data-plane transfer. The data plane receives ciphertext only.
2. The sender stores or streams the ciphertext under a manifest containing a version, ciphertext size, chunk commitments, expiry, transfer identifier, and integrity hash.
3. The sender checks recipient trust, CRL status, recipient privacy policy, local quota, and transport policy. If the check fails, it uses an existing non-direct delivery path or returns a policy denial.
4. The sender creates and signs an encrypted `DataPlaneOffer`, then sends it through the authorized GhostWire control path.
5. The recipient verifies the offer, stores a one-time local acceptance record, and instructs the hybrid actor to obtain the ciphertext.
6. The recipient verifies the manifest and every chunk commitment before releasing the ciphertext to GhostWire decryption. A completed transfer is only a transport fact; it is not proof that the application message is authentic until normal GhostWire verification succeeds.
7. Both sides erase ephemeral capability material at completion, cancellation, expiry, or shutdown. The mesh receives only a privacy-safe lifecycle acknowledgement when policy permits it.

## Proposed module layout

The following layout keeps optional Iroh code out of default builds until it has been reviewed and tested.

```text
src/core/data_plane/
  mod.rs                 # public opaque handle, feature gates, shared types
  actor.rs               # bounded command loop and cancellation ownership
  offer.rs               # signed/encrypted DataPlaneOffer wire type
  policy.rs              # privacy class, quotas, CRL and trust checks
  manifest.rs            # ciphertext-only manifest and chunk integrity checks
  iroh_adapter.rs        # thin Iroh-specific endpoint implementation
  test_support.rs        # deterministic in-memory fake behind a test feature
```

`Cargo.toml` should receive an **optional** `iroh-data-plane` feature rather than enabling Iroh in the default feature set. Default production builds must not acquire a new endpoint, relay behavior, or discovery path until the feature has passed review. The adapter should be selected through an internal trait such as `DataPlaneBackend`; it should not be added directly as another broad `TransportKind` variant until the actor boundary has replaced direct mutable transport ownership.

## Incremental implementation plan

| Increment | Deliverable | Exit criterion |
|---|---|---|
| 0 — API validation | Verify current Iroh crate APIs, feature flags, relay controls, endpoint identifiers, ticket serialization, and licensing against upstream sources | Written compatibility note; no code merged |
| 1 — Control contract | Implement versioned `DataPlaneOffer`, manifest, signature/encryption binding, and negative tests without an Iroh dependency | Offers reject altered signature, wrong recipient, expiry, revocation, oversize, and replay |
| 2 — Actor shell | Add bounded `HybridDataPlaneHandle` and a deterministic in-memory backend | Cancellation, saturation, shutdown, and no-log-secret tests pass |
| 3 — Optional adapter | Implement the Iroh adapter behind `iroh-data-plane` and bind its endpoint key to an accepted offer | Two-node encrypted ciphertext transfer works in a private test topology |
| 4 — Privacy routing | Enforce direct/relay/anonymous policy selection and existing fallback behavior | Anonymous-only tests show no direct adapter invocation |
| 5 — Operations | Add local observability counters, storage quotas, expiry cleanup, and Docker CI scenario | Logs and UI expose only coarse, non-sensitive lifecycle status |

## Acceptance tests

The integration should not be accepted on a successful file transfer alone. The minimum suite must cover control-plane tampering, recipient substitution, issuer revocation before acceptance, issuer revocation during transfer, offer expiry, replayed offer, object-size denial, chunk-integrity failure, actor queue saturation, cancellation, process shutdown, direct-path prohibition for anonymous-only messages, and fallback to an existing transport.

The existing Docker playground can later provide a three-node topology. Node 1 issues an encrypted offer to node 2 through the bootstrap mesh; node 3 observes control-plane traffic but cannot decode the locator or retrieve the object. That scenario must be run in an isolated CI environment with captured logs checked for ticket, endpoint, plaintext, and recipient-metadata leakage.

## Open questions requiring online validation

This offline document deliberately leaves the following items unresolved until external documentation and source review are available: the exact supported Iroh version; its current relay and NAT traversal configuration; stable endpoint-key and ticket APIs; how to disable or constrain discovery; operational bandwidth and connection quotas; current cryptographic defaults; compatibility with the project's Tokio and Rust versions; and license compatibility. These are release-blocking validation items, not implementation details to infer from memory.
