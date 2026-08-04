# Architecture Decisions — Protocol, Compression, Calls

**Date**: 2026-08-02
**Status**: Implemented

## 1. Message Struct Extension

**Decision**: Extended `Message` with content_type, content_bytes, content_cid, metadata, compressed, compression_algo.

**Rationale**: GhostWire needs to carry media, files, and binary data — not just text. The existing `content: String` field is insufficient for images, voice notes, files, and media clips referenced by CID.

**Pattern learned from**: Matrix (mxc:// URIs for media), Briar (file attachments with metadata), Meshtastic (protobuf with typed payloads).

**Trade-offs**:
- `content_bytes: Option<Vec<u8>>` for inline binary (max 64KB) — avoids BlockStore round-trip for small payloads
- `content_cid: Option<String>` for large files — delegates to existing BlockStore/BLAKE3 system
- `compressed: bool` + `compression_algo: Option<String>` — tracks compression state for transparent decompress

## 2. Compression Layer

**Decision**: zstd (primary) + lz4 (fast path), skip-compress for <256B payloads.

**Rationale**: Meshtastic proved zstd + dictionaries works for mesh: 7.2x on protobuf for LoRa 237B MTU. GhostWire follows the same pattern.

**Key insight**: General-purpose compression of tiny packets (<256B) usually loses — headers outweigh savings. Only payloads >1KB reliably pay off.

**Implementation**:
- `Codec` enum: Zstd (0x01), Lz4 (0x02), None (0xFF)
- `compress(data, codec)` / `decompress(data, codec)` with skip-compress for small payloads
- `choose_codec(len, prefer_speed)` for adaptive selection
- Wire format: 1-byte flags (codec ID + dict ID) + payload; 0xFF = raw (skip-compress)

**Skipped**: brotli (too slow for constrained), gzip (5x slower decomp than zstd), custom adaptive framework (1-byte header is enough).

## 3. Call Signaling

**Decision**: Minimal call state machine with ICE candidate exchange, SDP offer/answer.

**Rationale**: Voice/video calls need direct internet transports (QUIC, TCP hole-punching). The signaling layer must work over GhostWire's existing mesh for call setup, then hand off to direct transport for media.

**Pattern learned from**: Tox (AV call signaling over DHT), Jitsi (SFU with ICE), str0m (pure Rust SFU).

**Implementation**:
- `ActiveCall` state machine: Idle → Inviting → Ringing → Connected → Disconnected
- `CallSignal` enum: Invite, Accept, Reject, End, IceCandidate
- SDP generation with ICE ufrag/pwd
- MediaType: Audio, Video, AudioVideo

**Skipped**: Full WebRTC stack (webrtc-rs is complex, wire later), SRTP key exchange (do after SDP), DTLS (use existing X25519), SFU (str0m when needed).

## 4. Research Findings Applied

| Area | Finding | Applied |
|------|---------|---------|
| Compression | zstd + dict = 7.2x on protobuf (Meshtastic) | zstd as primary codec |
| Compression | lz4 = fastest, no_std, 8KB memory | lz4 as fast path |
| Compression | <256B payloads: skip-compress | MIN_COMPRESS_SIZE = 256 |
| Calls | Tox: AV over DHT, ICE via relay | Call signaling over mesh |
| Calls | str0m: pure Rust SFU | Future SFU option |
| Calls | webrtc-rs v0.20: full WebRTC | Future full WebRTC |
| Similar projects | Matrix: mxc:// media refs | content_cid pattern |
| Similar projects | Briar: file attachments | content_bytes inline |
| Similar projects | Signal: envelope encryption | Existing encryption layer |

## 5. Files Changed

- `src/core/message.rs` — Extended Message struct, added compress/decompress
- `src/core/compression.rs` — New module: Codec, compress/decompress, skip-compress
- `src/core/calls.rs` — New module: Call signaling, state machine, SDP
- `src/core/mod.rs` — Registered compression and calls modules
- `ghostwire/Cargo.toml` — Added zstd, lz4_flex dependencies
- `research/rust-compression-libraries-2026-08-02.md` — Compression research
- `research/similar-projects-architecture-2026-08-02.md` — Similar projects research
- `research/rust-voice-video-webrtc-2026-08-02.md` — Voice/video research
- `research/protocol-compression-calls-decisions-2026-08-02.md` — This document

## 6. Test Results

```
message::tests: 11 passed (including compress_roundtrip, compress_binary)
calls::tests: 4 passed (call_lifecycle, reject, sdp_offer, media_types)
compression::tests: 6 passed (roundtrip zstd, roundtrip lz4, skip small, byte roundtrip, choose codec, incompressible)
```

## 7. Next Steps

1. Wire compression into actual message send/receive path (mesh_transport.rs)
2. Add dictionary training for zstd (Meshtastic pattern)
3. Add SRTP key exchange for call media encryption
4. Wire webrtc-rs or str0m for actual media handling
5. Add TURN relay support for NAT traversal
