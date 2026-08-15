# GhostWire Error Index and Public-Fault Contract

**Status:** Proposed stable contract for implementation.
**Audience:** Core, transport, API, WebSocket, TUI, web UI, test, and operations maintainers.
**Privacy rule:** A public fault is an explanation of what the caller may safely learn. It is not an exception dump.

## Purpose

GhostWire currently has both a small numeric `ErrorCode` catalogue and broad internal error wrappers, neither of which is consistently emitted at API, event, or client boundaries. This document establishes one forward-compatible, privacy-safe contract. It is intentionally more specific than the current implementation so frontend behavior, retry policy, metrics, and regression tests can converge on the same vocabulary.

A fault has two representations. The **internal fault** preserves a typed source chain and protected diagnostic context for a local operator. The **public fault** exposes only a stable numeric code, category, retry directive, safe message, and opaque diagnostic reference. The public representation must never serialize raw `anyhow` strings, backtraces, peer IDs, multiaddrs, IP addresses, contact/conversation IDs, routing paths, payment identifiers, message ciphertext/plaintext, keys, tokens, or filesystem paths.

```rust
#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub struct PublicFault {
    pub code: ErrorCode,
    pub category: FaultCategory,
    pub retry: RetryDirective,
    pub public_message: &'static str,
    pub diagnostic_ref: DiagnosticRef,
}
```

## Code ranges

| Range | Category | Intended owner | Default external detail |
|---|---|---|---|
| `0x0000–0x00FF` | Connectivity and carrier transport | Network stack | Generic outcome only. |
| `0x0100–0x01FF` | Local session, HTTP, and authorization | API/security stack | Generic outcome only. |
| `0x1000–0x10FF` | Identity, trust, and cryptography | Identity/security stack | Generic security outcome. |
| `0x2000–0x20FF` | Protocol and message validation | Shared ingress protocol | Generic rejection reason. |
| `0x3000–0x30FF` | Privacy and delivery policy | Transport coordinator | Privacy class/status but no route/peer detail. |
| `0x4000–0x40FF` | Resource limits and overload | All bounded queues/codecs | Generic limit state. |
| `0x5000–0x50FF` | Storage and lifecycle durability | Storage/message service | Generic local persistence outcome. |
| `0x6000–0x60FF` | Configuration and feature availability | Core/bootstrap | Generic configuration state. |
| `0x7000–0x70FF` | Optional external adapter | Iroh/Reticulum/bridge modules | Generic adapter status. |
| `0x7F00–0x7FFF` | Internal invariant or bug | Any | “Internal operation failed” plus diagnostic ref. |

## Retry directives

Retryability belongs to a policy, not to an exception string. The client must follow the directive and a server-provided delay if present; it must not repeatedly retry a security, privacy, or validation fault.

| Directive | Meaning | Example |
|---|---|---|
| `Never` | Retrying unchanged input is not safe or useful. | Invalid signature, malformed request, revoked identity. |
| `After(Duration)` | Retry the same intent after the stated bounded delay. | Temporary transport timeout. |
| `QueueUntil(Expiry)` | Preserve only ciphertext/intent and retry when a policy-compliant route is available. | Anonymity-required mailbox delivery has no eligible route. |
| `UserAction` | User must authenticate, unlock, approve a privacy downgrade, or correct input. | Local daemon session expired. |
| `NewOperation` | A new request/operation is required; do not replay the current identifier. | Expired payment request or closed session. |

## Initial index

The initial code set should be deliberately small. Reserve codes when a public client behavior, telemetry aggregation, or operator workflow needs them; do not encode every library error.

| Code | Symbol | Public message | Retry | Owner | Test requirement |
|---:|---|---|---|---|---|
| `0x0001` | `TRANSPORT_UNREACHABLE` | “No eligible transport could reach the recipient.” | `After` / policy candidate | Coordinator | Deterministic adapter failure/failover. |
| `0x0002` | `TRANSPORT_TIMEOUT` | “The transport attempt timed out.” | `After` | Coordinator | Fake-clock deadline test. |
| `0x0003` | `TRANSPORT_CAPABILITY_UNAVAILABLE` | “No transport satisfies this delivery requirement.” | `QueueUntil` or `Never` | Coordinator | Privacy/capability selection test. |
| `0x0101` | `SESSION_UNAUTHENTICATED` | “Authentication is required.” | `UserAction` | HTTP/WebSocket | Unauthenticated mutation/upgrade test. |
| `0x0102` | `COMMAND_FORBIDDEN` | “This command is not permitted.” | `Never` | Command service | Capability/role test. |
| `0x0103` | `ORIGIN_REJECTED` | “This client origin is not permitted.” | `Never` | WebSocket/API | Origin policy test. |
| `0x1001` | `IDENTITY_REVOKED` | “The identity is no longer trusted.” | `Never` | CRL/trust | CRL enforcement integration test. |
| `0x1002` | `IDENTITY_BINDING_MISMATCH` | “The remote identity did not match the expected identity.” | `Never` | Handshake/offer | Wrong binding test. |
| `0x1003` | `AUTHENTICATION_FAILED` | “Message authentication failed.” | `Never` | Ratchet/AEAD | Failed decrypt leaves state unchanged. |
| `0x1004` | `SIGNATURE_INVALID` | “Message signature verification failed.” | `Never` | Ingress | Tampered envelope test. |
| `0x2001` | `FRAME_MALFORMED` | “The message format is invalid.” | `Never` | Codec | Fuzz/negative corpus. |
| `0x2002` | `FRAME_EXPIRED` | “The message has expired.” | `NewOperation` | Ingress | Fake-clock expiry test. |
| `0x2003` | `FRAME_REPLAYED` | “This message was already processed.” | `Never` | Dedup | Replay test. |
| `0x2004` | `CAUSAL_DEPENDENCY_MISSING` | “The message is waiting for earlier messages.” | `QueueUntil` | Causal buffer | Gap and expiry/cap tests. |
| `0x2005` | `PROTOCOL_VERSION_UNSUPPORTED` | “This protocol version is not supported.” | `Never` | Codec | Version-negotiation test. |
| `0x3001` | `PRIVACY_ROUTE_REQUIRED` | “No route meets the required privacy level.” | `QueueUntil` / `UserAction` | Coordinator | Prove no implicit downgrade. |
| `0x3002` | `PRIVACY_DOWNGRADE_REQUIRES_APPROVAL` | “A less private route requires your approval.” | `UserAction` | Client policy | UI approval/decline test. |
| `0x3003` | `PRIVATE_DELIVERY_UNAVAILABLE` | “Private delivery is temporarily unavailable.” | `QueueUntil` | Direct/mailbox | Three-node privacy test. |
| `0x4001` | `FRAME_SIZE_LIMIT` | “The message exceeds the permitted size.” | `Never` | Codec/QUIC | Bounded allocation test. |
| `0x4002` | `QUEUE_CAPACITY_LIMIT` | “The delivery queue is at capacity.” | `After` / `Never` | Mailbox/causal | Per-peer/global quota test. |
| `0x4003` | `RATE_LIMITED` | “The operation is temporarily rate limited.” | `After` | API/network | Token bucket test. |
| `0x5001` | `ENCRYPTED_PERSIST_FAILED` | “The message could not be stored securely.” | `After` / `Never` | Storage | Prove no plaintext fallback. |
| `0x5002` | `MESSAGE_LIFECYCLE_CONFLICT` | “The message state could not be updated.” | `After` | Lifecycle store | Idempotency/concurrency test. |
| `0x6001` | `FEATURE_UNAVAILABLE` | “This feature is not enabled.” | `Never` | Bootstrap | Feature-gate API test. |
| `0x6002` | `CONFIGURATION_INVALID` | “The local configuration is invalid.” | `UserAction` | Builder | Validation unit test. |
| `0x7001` | `DATAPLANE_OFFER_INVALID` | “The data-transfer offer was rejected.” | `Never` | Iroh adapter | Offer binding/expiry/replay tests. |
| `0x7002` | `RETICULUM_BRIDGE_UNAVAILABLE` | “The Reticulum bridge is unavailable.” | `After` / `Never` | Bridge | Adapter smoke test. |
| `0x7F00` | `INTERNAL_INVARIANT` | “An internal operation failed.” | `Never` | Any | No sensitive source serialization test. |

## Privacy-safe context policy

| Field type | Public event/API | Local protected diagnostic log | Telemetry |
|---|---|---|---|
| Peer ID / contact ID | Never raw; optional opaque correlation ID. | Allowed only under explicit local operator policy. | Count only. |
| IP / multiaddr / route / relay | Never. | Allowed only redacted and access-controlled. | Never. |
| Message ID | Opaque operation ID allowed. | Allowed. | Aggregated only. |
| Conversation ID / payment identifier | Never. | Redacted/hardened reference only. | Never. |
| Key material / tokens / plaintext/ciphertext | Never. | Never unless an isolated cryptographic forensic process explicitly defines it. | Never. |
| Error source chain | Never. | Allowed, sanitized and access-controlled. | Categorical code only. |

## Migration plan

1. Retain `ErrorCode` as the stable registry, but add the proposed categories/ranges and `#[non_exhaustive]` where public compatibility is relevant.
2. Replace `GhostWireError` direct peer/multiaddr fields with an internal-only diagnostic context. Make `to_public()` explicit and audited.
3. Keep `GhostError` and typed subsystem errors as internal causes. Add `From` mappings at message command, ingress, storage, and API/WebSocket boundaries first.
4. Update HTTP and WebSocket schemas to return `PublicFault`, `operation_id`, and lifecycle state. Do not return `anyhow` text.
5. Add a redaction snapshot test for every boundary and a code-uniqueness/metadata completeness test for the registry.

## Registry invariants

The test suite must prove that each registered code is unique; categories/ranges match the code; every code has a public-safe message and retry directive; `PublicFault` serialization contains no disallowed field names/values; and every public API fault maps to a registry code. Unknown underlying errors may map to `INTERNAL_INVARIANT`, but their internal source chain must remain local.
