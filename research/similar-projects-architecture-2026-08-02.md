# Architecture Patterns from Similar Projects — Research Report

## Key Patterns to Steal

### 1. Message Format: Signal's Envelope→Content layering
- protobuf envelope {type, flags} → oneof Content → typed messages
- ephemeral/urgent flags on envelope (mesh store-and-forward priority)
- requiredProtocolVersion for feature gating

### 2. Content Addressing: Briar's immutable messages
- message_id = hash(content) → free dedupe across redundant mesh paths
- Dependency DAG for causal ordering
- Partial-ack chunked transfer (b-tree blocks)

### 3. Multi-format Dispatch: Meshtastic's portnum
- Single 1-byte discriminator routes each payload to an app
- TEXT_MESSAGE_APP, POSITION_APP, AUDIO_APP, etc.
- One envelope, one discriminator, per-type payloads

### 4. Media Metadata: Signal's AttachmentPointer
- Pointer pattern: digest, size, MIME, dims, key, thumbnail, chunk MACs
- Never inline blob in message
- Matrix EncryptedFile: key/iv/hashes inline, blob out-of-band

### 5. Compression Tiers (proven in production)
- **Short text**: unishox2 (Meshtastic, 200bps scale)
- **Structured**: zstd + pre-trained dictionary on protobuf bytes (7.2x on CoT)
- **Images**: JPEG quality-loop to fit size budget (Briar 32KB loop)
- **Voice**: Opus ≥6kbps (BT/WiFi), Codec2 700-3200bps (LoRa)
- **Video**: AV1/VP9 SVC (Jitsi), VP8 for compatibility

### 6. Call Negotiation: Tox's capability flags
- Capability flags in invite message (audio_send/recv, video_send/recv, codec list)
- Force keyframes at start
- Keyframe re-request mid-call

### 7. Bandwidth Adaptation: Jitsi's LastN + dominant speaker
- Hard-cap concurrent high-res streams per node
- Dominant-speaker prioritization
- Per-link bitrate caps (Tox pattern)
- GCC/TWCC via str0m for adaptive layer

### 8. Liveness: MatrixRTC's deadman switch
- State-event membership with TTL
- Periodic refresh keeps alive
- Expiry = disconnect

### 9. Transport Abstraction: Briar + MatrixRTC
- Security/sync/media independent of transport plugin
- Member events describe reachability, transport is pluggable

## Rust Stack (all exist today)
- **str0m**: sans-I/O WebRTC, frame-level API, TWCC, simulcast
- **prost**: protobuf (Signal + Meshtastic both use it)
- **opus, codec2**: voice codecs
- **zstd**: compression with dictionary support
- **unishox2**: short text compression
- **lz4_flex**: fast path, no_std

## Decision: Mesh Topology Ceiling
- Full mesh breaks at ~4-6 call participants
- Design membership/transport abstraction now (MatrixRTC pattern)
- Small-call full-mesh + large-call relay mode coexist
