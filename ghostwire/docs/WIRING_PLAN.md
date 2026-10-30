# GhostWire Wiring Plan — Unwired Modules

## Status: Designed but not running

The compensating controls audit found that GhostWire's compensating chain is
"designed but not running." Several modules exist but have 0 or minimal callers.
This document plans the wiring for each.

## Priority 1: RNS Address Exchange (BLOCKING)

**Status**: Fix in progress. Root cause: `set_rns_local_address()` writes to
`MeshManager.transport.rns_local_address` (shared Arc), but the MeshDriver
reads from its own cloned Arc. The Arc IS shared (cloned at driver creation),
so the issue is timing: `init_reticulum()` runs after the handshake is sent.

**Fix**: Move `init_reticulum()` before `start_mesh()` in the unlock handler.
This ensures `rns_local_address` is populated before the MeshDriver is created.

**Files**: `src/controllers/identity.rs`, `src/core/mod.rs`, `src/core/mesh.rs`
**Test**: Node-to-node: verify peer list shows non-null rns_address after handshake.

## Priority 2: Sealed Sender Wiring

**Status**: `sealed_sender.rs` exists (155 lines). `Message` struct has no
`sealed_sender` field. 0 callers.

**Wiring plan**:
1. Add `sealed_sender: bool` field to `Message` struct
2. In `send_direct_message()`: if privacy tier >= L2, call `seal_message()`
   before sending, set `message.sealed_sender = true`
3. In receive path: if `message.sealed_sender`, call `unseal_message()`
4. Add AAD binding (include recipient hint in MAC)
5. Add replay protection (track delivery tokens)
6. Add expiry (check message timestamp)

**Files**: `src/core/message.rs`, `src/core/mod.rs`, `src/core/sealed_sender.rs`
**Test**: Node-to-node: verify sealed messages are unreadable by relay nodes.

## Priority 3: RNS Transport Carrying Traffic

**Status**: RNS initialized, 0 peers discovered. No actual RNS message delivery
in production (receive pump exists but untested).

**Wiring plan**:
1. Ensure RNS announces are sent (currently only on startup)
2. Implement RNS announce re-broadcast (periodic, jittered)
3. Wire RNS receive pump to message store (already done, untested)
4. Implement RNS path finding (currently only direct links)
5. Wire RNS into transport selector (currently selector runs but send path
   doesn't consume output)

**Files**: `src/core/reticulum.rs`, `src/core/transport_selector.rs`, `src/core/mod.rs`
**Test**: Node-to-node: send message via RNS when libp2p is blocked.

## Priority 4: vodozemac Wiring

**Status**: Module exists, unwired. Custom X25519+ChaCha AEAD used instead.

**Wiring plan**:
1. Add vodozemac as dependency (already in Cargo.toml?)
2. Create adapter layer: vodozemac Olm sessions → GhostWire Message encryption
3. Replace custom encryption in `send_direct_message()` with vodozemac
4. Add double ratchet for forward secrecy
5. Add post-compromise security (key rotation)

**Files**: `src/core/encryption.rs`, `src/core/vodozemac.rs` (new)
**Test**: Node-to-node: verify forward secrecy (old keys can't decrypt new messages).

## Priority 5: Message Delivery Receipts

**Status**: `delivery_status` field exists on Message. UI shows icons. Backend
status updates not wired.

**Wiring plan**:
1. On message send: set `delivery_status = "sent"`
2. On message receive: send ACK back to sender
3. On ACK receive: set `delivery_status = "delivered"`
4. On read (UI): send read receipt, set `delivery_status = "read"`
5. Store delivery status in message store

**Files**: `src/controllers/messaging.rs`, `src/core/mod.rs`, `src/core/message.rs`
**Test**: Node-to-node: verify delivery status updates on both sides.

## Priority 6: Store-and-Forward Completion

**Status**: Mailbox purge wired (12h). `mailbox_pending_count()` API added.
Reconnect drain wired. `store_incoming` not called.

**Wiring plan**:
1. Call `store_incoming()` when recipient is offline
2. On reconnect, drain mailbox and deliver pending messages
3. Add TTL for stored messages (expire after N days)
4. Add size limit for mailbox (prevent DoS)

**Files**: `src/core/mailbox.rs`, `src/core/mod.rs`
**Test**: Node-to-node: send message while recipient offline, verify delivery on reconnect.

## Priority 7: Power Management Schedules

**Status**: PowerMode enum created. PowerManager wired into Core. Schedules not
implemented.

**Wiring plan**:
1. Implement power mode transitions (Active → Idle → Sleep → DeepSleep)
2. Add battery monitoring (Linux: /sys/class/power_supply/)
3. Add power-aware transport selection (reduce RNS announce frequency on battery)
4. Add wake-on-message (wake from sleep when message arrives)

**Files**: `src/core/power.rs`, `src/core/mod.rs`
**Test**: Unit tests for power mode transitions.

## Priority 8: Privacy Tiers (L0-L3)

**Status**: Designed in docs/PRIVACY_TIERS.md. Not implemented.

**Wiring plan**:
1. Add `privacy_tier: u8` to profile/config
2. Create privacy tier selector (maps tier to transport/encryption settings)
3. Wire into message send path (select encryption based on tier)
4. Add UI toggle for privacy tier
5. Add auto-escalation based on environmental conditions

**Files**: `src/core/privacy_tier.rs` (new), `src/core/mod.rs`, `src/webui/`
**Test**: Node-to-node: verify different privacy tiers produce different metadata exposure.

## Implementation Order

1. RNS address exchange (blocking — needed for cross-transport discovery)
2. Sealed sender wiring (privacy — compensating control for DM metadata)
3. RNS transport carrying traffic (resilience — primary transport)
4. Message delivery receipts (UX — user-facing feature)
5. Store-and-forward completion (resilience — offline messaging)
6. Power management schedules (UX — battery life)
7. Privacy tiers (UX — user control)
8. vodozemac wiring (security — forward secrecy)

## Testing Strategy

Every wiring change requires:
1. Unit tests for the new code
2. Node-to-node integration test (two daemon instances)
3. Verification that removing the feature breaks the test
4. Update to truth ledger

No exceptions.
EOF
echo "created"