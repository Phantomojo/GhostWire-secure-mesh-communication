# GhostWire Privacy/Anonymity Audit (2026-10-26)

Scope: metadata leakage, traffic analysis, Sphinx routing, sealed sender, peer discovery (mDNS), logging, PII.

## Summary
- Sphinx mixnet exists but hop identifiers are real PeerIds, so every relay learns the next hop's (and ultimately recipient's) PeerId.
- Sealed sender is not implemented: `Message.sender`/`recipient` remain plaintext in frames even when `content` is encrypted.
- mDNS is correctly disabled by default (`GHOSTWIRE_ENABLE_MDNS`), but opt-in still broadcasts PeerId on LAN.
- Several `tracing` calls log PeerIds/message IDs/recipients, which leak metadata if logs are collected.
- Anonymous invites optionally carry a `nickname` (PII) and precise `created_at`.

## Findings

### F1 — Hop identifiers are real PeerIds (Sphinx)
- `src/core/routing/sphinx.rs` `create_sphinx_packet` takes `route_ids: &[String]` and embeds them as the next-hop identifier in `beta` (lines ~399-408, ~574-582). `src/core/mesh.rs` `select_sphinx_route` pushes `recipient.to_string()` (PeerId) into `route_ids` (≈1886). Relays therefore learn the recipient's PeerId. Use opaque per-route hop tokens instead.

### F2 — Minimum hop count below production guidance
- `src/core/routing/sphinx.rs:40-41` `SPHINX_MIN_HOPS = 2` (comments say ≥3 for real anonymity). 2-hop routes provide weak anonymity.

### F3 — Sealed sender absent
- `src/core/message.rs:187-188` `sender`/`recipient` fields. `src/core/mesh.rs:3381` sets `sender_id: msg.sender.clone()` into `EncryptedMessage`; frames carry `msg.sender`/`msg.recipient` in clear (`mesh.rs:2254`, `3314`). Network observers and relays can map social graph. Implement sealed sender (hide sender from relays; recipient learns sender only after decrypt).

### F4 — Precise timestamps / message IDs
- `src/core/message.rs:194` `timestamp: u64` (second precision). `mesh.rs:2150` vector clock. No jitter applied before send (comment at message.rs:228 notes "add jitter"). Traffic timing is correlatable.

### F5 — Logging leaks metadata
- `src/core/mesh.rs:2214,2241,2257,2270` log `recipient`.
- `src/core/mesh.rs:3316-3322` logs `forwarded.recipient` on relay.
- `src/core/mesh.rs:3414` logs `msg.sender` on decrypt failure.
- `src/core/mesh.rs:3514` logs `msg.sender` on Sphinx delivery.
- `src/core/mesh.rs:3591-3592` logs `peer_id` + `msg.sender` on handshake.
- `src/core/mesh.rs:3721` logs `msg_id` + `recipient`.
- `src/startup.rs:338` logs partial `node_id`.
- `src/web.rs:366,430` logs client IP/origin.
These should be redacted or gated behind a debug flag in production.

### F6 — mDNS opt-in still leaks PeerId
- `src/core/mesh.rs:567-577`: when `GHOSTWIRE_ENABLE_MDNS=1`, `Mdns::new(..., local_peer_id)` broadcasts PeerId on LAN. Expected but should be documented as identity-revealing; consider hashed/rotating identifiers.

### F7 — World discovery exposes stable identifiers on public backbone
- `src/core/discovery_bridge.rs:193-208` `DiscoveryAnnounce` carries `peer_id` and `did` in phase 1, which is broadcast to every backbone hop. Rendezvous keys rotate daily (`windowed_rendezvous_key`) but PeerId/DID do not, enabling long-term tracking.

### F8 — Anonymous invite PII/timestamps
- `src/core/identity/anonymous_intro.rs:44-61` `AnonymousInvite.nickname: Option<String>` (optional PII in QR), `created_at: u64` precise, `surb_bytes` exposes relay PeerIds via `first_hop`/route_ids. Recommend nickname=None by default and coarsened timestamps.

### F9 — Cover/traffic padding only on Sphinx path
- `src/core/mesh.rs:2170-2177` pads content to 512 bytes before encryption, but direct DM path (`!sent_via_sphinx`) publishes raw frame (≈2323-2330) without mixing delay; size/timing side channels remain.

### F10 — SURB reply_key handling
- `src/core/routing/sphinx.rs:188-193` `Surb.reply_key` is serialized into the invite (`anonymous_intro.rs:138-139`); anyone with the QR (Bob) can decrypt replies, and relay first_hop is public. Acceptable for intro flow but means invite confidentiality matters.

## Recommendations
1. Implement sealed sender and opaque hop tokens.
2. Raise default hop count to ≥3; make 2-hop dev-only.
3. Redact PeerId/recipient/sender from production logs; gate behind `GHOSTWIRE_DEBUG`.
4. Jitter message timestamps and add delay to direct DMs.
5. Default invites to no nickname; coarsen `created_at`.
6. Document mDNS as identity-revealing; consider rotating discovery identifiers.