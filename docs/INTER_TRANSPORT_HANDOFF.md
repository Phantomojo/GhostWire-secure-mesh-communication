# Inter-Transport Protected-Envelope Handoff

**Status:** Deterministic planning contract implemented and tested. Runtime adapter dispatch and shared ingress integration remain open.
**Claim boundary:** GhostWire can now model a safe carrier-changing handoff. It must **not** yet claim that every Wi-Fi, BLE, RNS, LoRa, and libp2p adapter is wired into that runtime path.

## The Airport Model, Precisely

Every GhostWire participant is a node. A node can be an endpoint, an optional relay, or a carrier-changing handoff point. The handoff point is not a plaintext gateway. It accepts a **validated, still-protected GhostWire envelope** on one carrier and emits the same protected object through another carrier’s native delivery mechanism.

```text
Alice’s GhostWire node                 Relay GhostWire node                 Bob’s GhostWire node

protected envelope ── carrier A ──> validate + policy + plan ── carrier B ──> protected envelope
                                     no user plaintext here
```

The relay changes the **per-leg carrier context**, which can include a libp2p stream, RNS destination/link/resource, Wi-Fi Direct socket, BLE GATT exchange, radio resource, or encrypted store-and-forward queue. It does not translate Alice’s root identity into Bob’s carrier identity, and it does not decrypt user plaintext simply to forward the message.

## Preconditions Before Handoff

`transport_handoff::plan_handoff()` accepts only a `HandoffRequest` representing a protected envelope that has already passed the shared ingress checks. The future runtime pipeline must establish all of these before any carrier conversion:

| Required precondition | Why it cannot be deferred to the destination carrier |
|---|---|
| Message authentication | A relay must not amplify unauthenticated input onto another network. |
| CRL/revocation and trust policy | A revoked/suspicious sender must not regain reachability by switching carrier. |
| Expiry and replay protection | Carrier retries and store-and-forward paths otherwise become replay multipliers. |
| Causal-order eligibility | A handoff must not bypass a dependency buffer merely because carrier B arrived first. |
| Privacy/delivery intent | A destination adapter cannot silently weaken a requirement selected by the sender. |
| Bounded queue/memory state | A low-rate/partitioned carrier cannot become an unbounded reassembly or relay queue. |

This is why a real RNS bridge alone is not full inter-transport support: bridge ingress must still enter the shared validator before any application delivery or second-leg forwarding.

## Handoff Decision Contract

The planner maintains one `ProtectedEnvelopeId` across every carrier leg. Carrier receipt identifiers are local to their leg and cannot overwrite the end-to-end message operation ID.

| Decision input | Enforced behavior |
|---|---|
| Source carrier | Must be currently selectable: available, owner-enabled, device-supported, OS-permitted, background-eligible, and internally truthful about its native identity namespace. |
| Delivery requirement | Must explicitly set `requires_relay = true`; an ordinary direct send cannot be mislabelled as a cross-carrier handoff. |
| Destination candidates | Must meet the declared delivery kind and privacy class in explicit product-policy order. Metadata-visible candidates cannot satisfy a relationship-metadata-protected requirement. |
| Relay eligibility | The destination interface must have relay opt-in. A phone/device remains an endpoint and optional relay, never an automatic radio forwarder. |
| Carrier change | The next carrier must differ from the source. A same-carrier retry is not recorded as a handoff. |
| Object length | Uses protected-object length after GhostWire encryption/authentication; plaintext length is not a reliable transport decision input. |

If no candidate satisfies all constraints, planning returns a typed failure. The caller may place the protected envelope in an encrypted, bounded store-and-forward queue only if that was separately permitted by the original delivery operation. It may not publish to a shared topic, relax privacy, or make an arbitrary direct connection.

## Fragmentation Is Per-Leg, Not Universal

The selected carrier’s `CarrierFragmentation` contract determines the only permitted result:

| Next-leg contract | Planner result | What GhostWire must not do |
|---|---|---|
| Bounded stream | Direct logical frame below the carrier’s declared frame ceiling; otherwise private block exchange | Split ciphertext into generic application fragments and call that “stream support” |
| Native resource/link | Direct packet below the native packet limit; otherwise the carrier’s documented resource facility | Reimplement or fake the native resource/reliability protocol |
| Strict single packet | Direct packet only when within limit | Truncate, silently drop tail bytes, or invent unauthenticated fragments |
| Private block exchange | Direct block below block size; otherwise a separately authorized verified block exchange | Announce public provider/availability data or send ambient wantlists |

The current external-RNS bridge is explicitly a strict single-packet path capped at 383 encrypted payload bytes. Reticulum documents a default 500-byte MTU and a maximum 383-byte encrypted single-packet payload; applications requiring larger transfers must use appropriate native link/resource behavior instead of changing that limit. [1] [2]

Borrowed BitTorrent/IPFS mechanics are limited to integrity and resumability: immutable content-addressed blocks, verified manifests/Merkle proofs, bounded requests, cancellation, and backpressure. Bitswap’s wantlists and availability signals are useful mechanics but can become content-interest metadata; a GhostWire private bulk session must therefore scope them to an already authorized private peer/path rather than public routing. [3] [4]

## Receipt and Lifecycle Semantics

Handoff requires three separate state classes. Collapsing them is how a first-hop write gets falsely shown as recipient delivery.

| State | Meaning | Example evidence |
|---|---|---|
| `LegAccepted` | The next carrier accepted the protected envelope for processing. | RNS bridge accepted bounded JSON command; local QUIC stream write accepted. |
| `CarrierReceipt` | Carrier B supplied its native acknowledgement/receipt. | RNS packet receipt, native link/resource confirmation, a verified radio layer acknowledgement. |
| `EndToEndAcknowledged` | Bob’s GhostWire node verified and acknowledged the protected GhostWire envelope. | Authenticated GhostWire acknowledgement bound to `ProtectedEnvelopeId`. |

Only the third state is recipient delivery. Current work proves bounded RNS packet receipts, not an end-to-end RNS/BLE/Wi-Fi hybrid delivery acknowledgement.

## Current Deterministic Tests

| Property | Result |
|---|---|
| A Wi-Fi-Direct-class source → RNS destination preserves one protected envelope ID | Passing |
| The RNS 383-byte ceiling is accepted; 384-byte protected objects fail without generic fragmentation | Passing |
| A metadata-visible shared-gossip destination is rejected for a protected relayed direct request | Passing |
| A BLE destination without relay opt-in is rejected | Passing |
| Oversized stream transfer selects private block exchange rather than an implicit chunker | Passing |
| A same-carrier attempt is rejected as a false handoff | Passing |
| A non-relay direct operation cannot be repurposed as a handoff | Passing |

## Remaining Runtime Work

| Work item | Required acceptance condition |
|---|---|
| Shared ingress validator | The same malformed/revoked/replayed/causally blocked corpus yields the same result for gossip, direct stream, RNS, mailbox, and future BLE/radio paths. |
| Adapter registry | Concrete libp2p, QUIC, RNS, BLE, Wi-Fi Direct, radio, and mailbox adapters provide live carrier declarations to the coordinator. |
| Durable lifecycle | Operation record preserves one end-to-end ID plus bounded per-leg attempt/receipt detail without exposing topology publicly. |
| Private bulk session | Encrypted manifest, recipient/path authorization, bounded verified blocks, cancel/expiry, no public provider records, and no relay plaintext access. |
| RNS Link/Resource | Large protected-object flow uses official RNS Link/Resource semantics and a real-node test; packet cap remains immutable. |
| Three-node observer regression | An observing carrier participant sees neither protected plaintext nor an implicit shared private-direct topic/availability record. |

## References

[1]: https://reticulum.network/manual/reference.html "Reticulum API Reference"
[2]: https://github.com/markqvist/Reticulum/blob/master/RNS/Packet.py "Reticulum Reference Packet Implementation"
[3]: https://www.bittorrent.org/beps/bep_0052.html "BEP 52: BitTorrent Protocol Specification v2"
[4]: https://github.com/ipfs/specs/blob/main/src/bitswap-protocol.md "IPFS Bitswap Protocol"
