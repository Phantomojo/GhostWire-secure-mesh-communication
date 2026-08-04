# Rust Compression Libraries for GhostWire — Research Report

## Summary
Meshtastic proved zstd + dictionaries works for mesh: 7.2x on protobuf, skip-compress for tiny packets. GhostWire should follow the same pattern.

## Comparison Table

| Crate | Ratio (text/JSON/binary) | Compress speed | Decompress speed | no_std | WASM | Production mesh? |
|---|---|---|---|---|---|---|
| **zstd** | 2.5-3.9x text, 3.3-8x JSON, 7.2x protobuf w/ dict | L1: 155 MB/s, L3: 95 MB/s | 400-1500+ MB/s | via ruzstd/zstd_nostd | zstd-wasm | YES — Meshtastic TAKPacket |
| **lz4_flex** | 4.4x JSON, 1.57x text (weak) | 1272-1615 MiB/s | 4540-5973 MiB/s | YES (block format) | lz4-wasm | No (DB use) |
| **brotli** | Best: 5-12x text/JSON | Slow (q11 extremely slow) | ~600 MB/s | YES (no-stdlib) | Pure Rust | No (web/CDN) |
| **flate2** | 5-8x JSON | Moderate | ~250 MB/s | YES (miniz_oxide) | wasm-flate | Yes (Meshtastic legacy) |

## Key Insight
General-purpose compression of tiny packets (<256B) usually LOSES — headers outweigh savings. Only payloads >1KB reliably pay off.

## Recommendation for GhostWire
1. **Primary**: zstd + pre-trained dictionary (Meshtastic proven pattern)
2. **Fast path**: lz4_flex block format (no_std, zero unsafe, 8-16KB memory)
3. **Skip both brotli and gzip for mesh path**
4. **Wire format**: 1-byte flags (codec ID + dict ID) + payload; 0xFF = raw (skip-compress)
5. **Layered**: Protobuf/CBOR for structure → delta/varint for telemetry → zstd+dict for >256B → skip-compress <128-256B
6. **Adaptive level**: f(payload size, link quality) — 1-byte header, not a framework
