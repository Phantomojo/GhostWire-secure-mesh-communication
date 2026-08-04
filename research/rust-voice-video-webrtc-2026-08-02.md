# Rust Voice/Video + WebRTC — Research Report

## Executive Summary
Everything needed for voice/video in Rust exists today. Two paths:
1. Full WebRTC (webrtc-rs v0.20.0) — most production-ready, browser-interop, gets congestion control + simulcast
2. Tox-style custom RTP over existing mesh transport — bolt on RTP/SRTP + Opus/VP8

## Key Crates
- **webrtc-rs v0.20.0** (July 31, 2026): Sans-I/O rewrite, runtime-agnostic, 5.1K stars. Supports Opus, VP8, VP9, AV1, H264, H265. Interceptors: TWCC, NACK, SR/RR, simulcast RID/MID.
- **str0m**: Sans-I/O, built for SFU/server use. Packetizers: Opus, VP8, VP9, AV1, H264, H265, H266, G711, G722.
- **opus-rs**: Pure Rust opus port. 0.1.23 (June 2026). Competitive with C libopus.
- **audiopus**: C binding (libopus 1.3). 1.2M downloads, mature.
- **rav1e**: AV1 encoder (xiph, BSD-2). 11 speed levels, real-time capable.
- **rav1d**: Safe Rust AV1 decoder (dav1d port).
- **vpx-rs**: libvpx binding (VP8/VP9). BSD-2.
- **openh264**: Cisco H264 binding.

## Architecture Patterns
- **Jitsi**: P2P for 2, SFU for groups. Simulcast (720p/360p/180p). VP8 everywhere. 10-way: 225 Mbps mesh vs 35 Mbps SFU+simulcast.
- **Tox**: Direct P2P only. Opus + VP8. MSI signaling. BWController for bitrate. No SFU.
- **Matrix**: 1:1 P2P → MatrixRTC (LiveKit SFU). State-event membership. Deadman switch.

## Bandwidth Adaptation
- **GCC** (Google Congestion Control): delay-based Kalman + loss-based, min of both
- **TWCC**: per-packet arrival times, sender-side estimator
- **Simulcast layer selection**: encode 3 layers, receiver picks fitting layer
- **Graceful fallback**: video → lower layer → audio-only → ICE restart

## Recommendation for GhostWire
- Phase 1: Opus voice via webrtc-rs v0.20.0 + opus-rs
- Phase 2: VP8 video via vpx-rs, simulcast 180p/360p/720p, TWCC adaptation
- Phase 3: AV1 via rav1e for Rust-to-Rust mesh (better compression)
- Phase 4: SFU relay via str0m or livekit for >4 participants
