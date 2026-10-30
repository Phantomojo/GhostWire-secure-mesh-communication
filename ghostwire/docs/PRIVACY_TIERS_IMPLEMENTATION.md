# GhostWire Privacy Tiers — Implementation Plan

## Overview

Privacy tiers (L0-L3) provide graduated privacy guarantees. Users choose their
level based on their threat model and resource constraints. The system auto-
escalates when conditions change (censorship detected, battery low, etc.).

## Tier Definitions

### L0 — Minimal (No privacy)
- **Use case**: Public broadcasts, announcements, non-sensitive data
- **Encryption**: None (plaintext)
- **Metadata**: Fully visible (sender, recipient, timestamp, size)
- **Transport**: Fastest available (libp2p direct)
- **Overhead**: Minimal (no crypto, no padding, no cover traffic)
- **Performance**: Maximum speed, minimum battery

### L1 — Standard (Encrypted content) [DEFAULT]
- **Use case**: Normal messaging, most conversations
- **Encryption**: X25519 + AES-256-GCM (E2E)
- **Metadata**: Sender/recipient/timestamp/size visible to peers on shared topic
- **Transport**: libp2p primary, RNS fallback
- **Overhead**: Moderate (E2E encryption, no padding)
- **Performance**: Good speed, low battery

### L2 — Enhanced (Sealed sender)
- **Use case**: Sensitive conversations, metadata protection needed
- **Encryption**: X25519 + AES-256-GCM + sealed sender envelope
- **Metadata**: Sender hidden (recipient hint only), timestamp/size still visible
- **Transport**: libp2p primary, RNS fallback, cover traffic optional
- **Overhead**: Higher (sealed envelope, cover traffic)
- **Performance**: Moderate speed, moderate battery

### L3 — Maximum (Full anonymity)
- **Use case**: High-threat scenarios, journalist/activist communications
- **Encryption**: X25519 + AES-256-GCM + sealed sender + Sphinx routing
- **Metadata**: Minimal (recipient hint only, padded to fixed size, random delay)
- **Transport**: RNS primary, Sphinx multi-hop, cover traffic mandatory
- **Overhead**: High (sealed + Sphinx + padding + cover + random delay)
- **Performance**: Slow, high battery

## Implementation Plan

### Phase 1: Tier Selector (config + API)
1. Add `privacy_tier: u8` to profile config (default: 1)
2. Add `GET /v2/privacy/tier` and `POST /v2/privacy/tier` endpoints
3. Add privacy tier to message metadata
4. Wire tier into send path (select encryption based on tier)

### Phase 2: Tier Enforcement
1. L0: send plaintext (no encryption)
2. L1: encrypt with X25519 + AES-256-GCM
3. L2: encrypt + seal sender envelope
4. L3: encrypt + seal + Sphinx route + padding + cover traffic

### Phase 3: Auto-Escalation
1. Monitor environmental conditions (censorship detected, battery low, etc.)
2. Auto-escalate privacy tier when conditions worsen
3. Auto-degrade when resources constrained (never below floor)
4. User notification when tier changes

### Phase 4: UI Integration
1. Privacy tier slider in settings (L0-L3)
2. Color-coded indicator in chat view
3. Auto-escalation notification
4. Power user: manual override

## Security Floor

Per compensating controls audit: "degrade gracefully" must have a non-
negotiable floor. The floor is L1 (encrypted content). Adaptation may
raise but never lower below L1. Authenticate tier negotiation so
adversary-forced downgrades are detected.

## Orthogonal Axes

Privacy tier (L0-L3) and disaster level (Normal-Blackout) are orthogonal:
- User picks privacy level
- System picks transports based on disaster level
- L1 default (fast, encrypted content, not anonymous)
- L3 on demand (sealed + cover + RNS + Sphinx)
- Auto-escalate when conditions change
- Degrade gracefully (reduce privacy overhead to maintain connectivity)
- Never force maximum (kills usability) or minimum (kills security)

## Files to Modify

- `src/core/privacy_tier.rs` (new) — tier selector + enforcement
- `src/core/mod.rs` — wire tier into send path
- `src/controllers/privacy.rs` (new) — API endpoints
- `src/web.rs` — route wiring
- `webui/src/components/panels/Settings.tsx` — tier slider
- `webui/src/components/panels/Communication.tsx` — tier indicator
EOF
echo "created"