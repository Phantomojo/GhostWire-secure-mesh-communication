# GhostWire Implementation Roadmap

> Created 2026-10-26. This is the canonical order for making scaffolding real.
> If work starts out of order, flag it.

## The Rule

No new feature starts until the previous one has a **node-to-node integration test** that:
- Starts two daemon instances
- Performs the actual operation
- Verifies the result on the receiving node
- Fails if the feature is removed

No feature is "done" without that test. No exceptions.

## Dependency Chain (bottom to top)

### Layer 1: Transport (foundation)
- [ ] RNS carries actual traffic (not just initialized)
- [ ] BLE transport implementation (design doc exists)
- [ ] WiFi transport implementation (design doc exists)
- [ ] Cross-transport discovery actually switches transports

### Layer 2: Routing
- [ ] Sphinx multi-hop tested end-to-end (2+ hops)
- [ ] DTN store-and-forward: store_incoming called, reconnect drain verified
- [ ] Reticulum announce feeds topology map with real addresses

### Layer 3: Privacy
- [ ] Sealed sender wired into send/receive flow
- [ ] Metadata (sender/recipient/timestamp) hidden or claim downgraded
- [ ] PQC verified hybrid ML-KEM exchange or claim removed

### Layer 4: UX
- [ ] Unread badges per conversation
- [ ] Typing indicators
- [ ] Delivery/read receipts (sent → delivered → read)
- [ ] Message search
- [ ] Attachments (file/image)
- [ ] Login UX (auto-unlock, password recovery, session persistence)

### Layer 5: Quality
- [ ] Replace unwraps with proper error handling (mesh.rs, encryption.rs first)
- [ ] Bound all unbounded collections
- [ ] Fix broken tests (transport_router_rns references non-existent Quic variant)
- [ ] Clippy -D warnings passing
- [ ] All integration tests green

## Suggested Next Session
Store-and-forward (Layer 2) — closest to working. Mailbox purge wired, reconnect drain exists, only store_incoming missing. Wire it, test it, then move on.

## Anti-patterns to avoid
- Creating modules without integration tests
- Starting new features before wiring the previous one
- Claiming features in README/truth ledger that aren't tested end-to-end
- Research subagent → design doc → module → move on (produces scaffolding, not software)

## Standing Constraint (every stage)

At EVERY stage — design, research, wiring, testing — evaluate:
- **Security**: Does this change introduce a new attack surface? Does it weaken existing guarantees? Are inputs validated? Are keys handled safely?
- **Privacy**: Does this change leak metadata (sender, recipient, timestamp, size)? Does it expose peer relationships? Does it create correlation opportunities?
- **Anonymity**: Does this change link a DID to a PeerId? Does it expose network topology? Does it enable traffic analysis?

No feature is "done" without passing this evaluation. This is not a phase — it is a lens applied to every change, at every layer.

### Specific concerns per layer
- **Transport**: BLE/WiFi discovery leaks proximity. RNS announces leak network topology. mDNS leaks presence on LAN.
- **Routing**: Sphinx hop IDs are real PeerIds (privacy leak). Cross-transport discovery links RNS address to PeerId. DTN bundles expose destination hashes.
- **Privacy**: Sealed sender must hide sender identity. Metadata (sender/recipient/timestamp/size) visible on shared DM topic. DID-PeerId correlation is a known defect.
- **UX**: Message read receipts leak activity patterns. Typing indicators leak presence. Unread counts leak communication frequency.
- **Quality**: Unwrap/panic can crash daemon and expose state. Unbounded collections enable DoS. Error messages leak internal details.
