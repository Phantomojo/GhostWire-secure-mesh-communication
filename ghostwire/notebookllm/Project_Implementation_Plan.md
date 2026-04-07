# Implement Security Stats Logic

## Goal
Populate the new `SecurityStats` fields (`messages_processed`, `peers_verified`, `rate_limit_violations`) with real runtime data so the Security Dashboard reflects actual system activity.

## User Review Required
None

## Proposed Changes
### GhostWire Core
#### [MODIFY] src/core/security.rs
- Update `SecurityManager` methods (`record_message`, `verify_peer`, etc.) to increment the corresponding counters in `SecurityStats`.
- `record_message`: Increment `messages_processed`.
- `record_connection_attempt`: Increment `rate_limit_violations` if blocked due to rate limits.
- `allow_new_peer` (in SybilDefense): Hook into stats to increment `peers_verified` upon success.

#### [MODIFY] src/core/mod.rs
- Ensure `Core` propagates these events to `SecurityManager` correctly if wrappers are needed.

## Verification Plan
### Automated Tests
- `cargo check` to ensure type safety.
- `cargo test` (unit tests for security module).
- manual `curl` to triggers stats (e.g. send a message, see count go up).
