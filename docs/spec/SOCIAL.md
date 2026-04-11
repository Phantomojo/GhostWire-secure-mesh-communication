# Social Protocol Specification

**GIP:** Pending
**Version:** 0.2.0-draft
**Date:** April 12, 2026

---

## Overview

This specification defines the social layer of GhostWire, including the connection state machine, contact request/response protocol, discovery methods for social connections, Web of Trust system, peer filtering, and the social event bus. The social protocol enables users to discover, connect to, and manage relationships with other peers in the mesh network.

---

## Connection State Machine

### States

Each peer relationship transitions through a well-defined state machine:

```
                    ┌──────────────────────────────────────────┐
                    │                                          │
                    ▼                                          │
              ┌───────────┐                                   │
              │  Unknown   │◄──────────────────────────────────┤
              └─────┬─────┘                                   │
                    │                                          │
        ┌───────────┼───────────────┐                         │
        ▼           ▼               ▼                         │
  ┌───────────┐ ┌────────────┐ ┌──────────┐                  │
  │ Pending   │ │ Pending    │ │  Denied  │                  │
  │ Outbound  │ │ Inbound    │ │          │                  │
  └─────┬─────┘ └─────┬──────┘ └──────────┘                  │
        │              │                                      │
        ▼              ▼                                      │
  ┌───────────┐ ┌────────────┐                               │
  │ Connected  │ │  Blocked   │                               │
  └─────┬─────┘ └────────────┘                               │
        │                                                     │
        ▼                                                     │
  ┌───────────┐                                               │
  │  Removed   │──────────────────────────────────────────────┘
  └───────────┘
```

### State Transitions

| From | Event | To | Description |
|------|-------|----|-------------|
| Unknown | Send ConnectRequest | PendingOutbound | User initiated connection |
| Unknown | Receive ConnectRequest | PendingInbound | Incoming connection request |
| PendingOutbound | Accept received | Connected | Mutual connection established |
| PendingInbound | Accept sent | Connected | User accepted incoming request |
| PendingOutbound | Reject received | Removed | Request was rejected |
| PendingInbound | Reject sent | Removed | User rejected incoming request |
| PendingInbound | Ignore | PendingInbound | Request neither accepted nor rejected (stays pending) |
| Any | Block | Blocked | User blocked the peer |
| Blocked | Unblock | Removed | Block removed; relationship reset |
| Connected | Remove | Removed | User removed the connection |
| Denied | Retry | PendingOutbound | User retries a previously denied connection |

### State Persistence

Connection states are persisted to the local database and survive restarts. The state is synchronized across paired devices (see Identity spec).

---

## ConnectRequest Protocol

### ConnectRequest Format

```rust
pub struct ConnectRequest {
    pub request_id: [u8; 32],           // BLAKE3(sender_pubkey || timestamp || nonce)
    pub sender_pubkey: [u8; 32],        // Ed25519 public key
    pub display_name: String,           // User-chosen display name (max 64 chars)
    pub personal_message: Option<String>, // Optional message (max 256 chars)
    pub discovery_method: DiscoveryMethod, // How the sender discovered the recipient
    pub timestamp: u64,                 // Unix timestamp (HLC)
    pub nonce: [u8; 16],                // Random nonce for uniqueness
    pub signature: [u8; 64],            // Ed25519 signature over all above fields
}

pub enum DiscoveryMethod {
    Mdns,           // Discovered via local network mDNS
    Dht,            // Discovered via Kademlia DHT
    ManualCode,     // User entered gw2: contact code
    QrCode,         // User scanned QR code
    Introduced,     // Introduced by a mutual contact
}
```

### ConnectRequest Validation

Upon receiving a ConnectRequest, the recipient validates:

1. **Signature verification:** Verify the Ed25519 signature using `sender_pubkey`.
2. **Timestamp check:** Reject requests older than 24 hours (replay protection).
3. **Duplicate check:** Reject requests with a `request_id` already seen.
4. **Display name validation:** Reject names exceeding 64 characters or containing control characters.
5. **Personal message validation:** Reject messages exceeding 256 characters.
6. **Block check:** If the sender is blocked, silently drop the request.

### ConnectResponse Format

```rust
pub struct ConnectResponse {
    pub request_id: [u8; 32],    // Echoes the ConnectRequest's request_id
    pub recipient_pubkey: [u8; 32],
    pub accepted: bool,          // true = accepted, false = rejected
    pub display_name: String,    // Recipient's display name
    pub personal_message: Option<String>,
    pub timestamp: u64,
    pub signature: [u8; 64],
}
```

**Accepted response:** The connection transitions to `Connected`. Both peers exchange their full contact cards.

**Rejected response:** The connection transitions to `Removed`. The sender is notified of the rejection.

**No response:** If no response is received within 72 hours, the request expires and transitions to `Removed`.

---

## Discovery Methods

Social discovery uses the same mechanisms as peer discovery (see DISCOVERY.md), with social-specific semantics:

### mDNS Discovery

Peers on the local network advertise their social availability:

```
TXT Records (additional):
  social=available
  display_name=<utf8_name>
  accepts_requests=true
```

### DHT Discovery

Social profiles are stored in the DHT:

```
key = BLAKE3(pubkey || "social_profile")
value = SocialProfile {
    pubkey: [u8; 32],
    display_name: String,
    avatar_hash: Option<[u8; 32]>,
    personal_message: Option<String>,
    accepts_requests: bool,
    last_seen: u64,
    signature: [u8; 64],
}
```

### Manual Code

Users share `gw2:` contact codes (see Identity spec) for manual connection.

### QR Code

QR codes encode the contact card for easy scanning.

### Introduced by Peer

A mutual contact can introduce two peers:

```rust
pub struct Introduction {
    pub introducer_pubkey: [u8; 32],
    pub introduced_pubkey: [u8; 32],
    pub introduced_display_name: String,
    pub introduction_message: Option<String>,
    pub introducer_attestation: [u8; 64],  // Ed25519 signature
    pub timestamp: u64,
}
```

The introduced peer receives the introduction and can choose to send a ConnectRequest.

---

## Web of Trust

### Trust Levels

GhostWire implements a graduated Web of Trust:

```
Unknown → FirstContact → Trusted → Vouched → Verified
```

| Level | Description | Requirements |
|-------|-------------|--------------|
| **Unknown** | No relationship | Default state |
| **FirstContact** | Initial connection established | ConnectRequest accepted |
| **Trusted** | Direct positive interaction | Minimum 7 days connected, 10+ message exchanges |
| **Vouched** | Attested by trusted contacts | ≥2 Trusted contacts provide attestation |
| **Verified** | In-person verification | User manually verifies identity (e.g., in person, video call) |

### Attestation System

Trusted peers can issue attestations for other peers:

```rust
pub struct Attestation {
    pub attester_pubkey: [u8; 32],     // Who is making the attestation
    pub subject_pubkey: [u8; 32],      // Who is being attested
    pub attestation_type: AttestationType,
    pub confidence: f64,               // 0.0 to 1.0
    pub message: Option<String>,       // Optional comment
    pub timestamp: u64,
    pub signature: [u8; 64],           // Ed25519 signature
}

pub enum AttestationType {
    Identity,     // "I confirm this person is who they claim to be"
    Trustworthy,  // "I trust this person in the mesh network"
    Expert,       // "This person is knowledgeable in [topic]"
}
```

**Attestation validation:**
- The attester must be at `Trusted` level or higher with respect to the recipient.
- Attestations are signed and timestamped.
- Attestations older than 1 year expire.
- A peer can revoke an attestation at any time.

### Trust Score Calculation

The trust score is a numeric value used for routing and filtering decisions:

```
trust_score = base_score + attestation_bonus + interaction_bonus

Where:
  base_score:
    Unknown:      0.0
    FirstContact: 0.1
    Trusted:      0.4
    Vouched:      0.6
    Verified:     0.8

  attestation_bonus: min(0.3, Σ(attestation.confidence × 0.1))
    // Capped at 0.3; each attestation contributes up to 0.1

  interaction_bonus: min(0.1, log(message_count + 1) × 0.02)
    // Capped at 0.1; increases with message exchange frequency

  trust_score = clamp(trust_score, 0.0, 1.0)
```

**Maximum trust score:** 1.0 (Verified + max attestations + high interaction).
**Minimum trust score:** 0.0 (Unknown, no interaction).

### Key Pinning and MITM Warnings

**Key pinning:** When a connection is first established, the peer's Ed25519 public key is pinned locally.

**MITM detection:** On subsequent connections, the peer's key is verified against the pinned value.

**MITM warning:** If the key differs:
1. The connection is rejected.
2. The user is alerted with a prominent warning.
3. The event is logged with connection details.
4. The peer's trust score is temporarily reduced.

```
WARNING: Key mismatch for peer {display_name}
  Expected: {pinned_key_fingerprint}
  Received: {received_key_fingerprint}

This may indicate a man-in-the-middle attack or that the peer
has regenerated their identity keypair.

Do NOT accept this connection unless you have independently
verified the peer's new key fingerprint.
```

---

## Peer Filtering

The social protocol supports filtering peers by various criteria:

| Filter | Description |
|--------|-------------|
| **All** | All known peers (any state) |
| **Connected** | Peers in the `Connected` state |
| **Online** | Connected peers currently reachable (recent heartbeat) |
| **PendingInbound** | Peers who have sent ConnectRequests awaiting response |
| **PendingOutbound** | ConnectRequests sent by user awaiting response |
| **Nearby** | Peers discovered via mDNS (local network) |
| **Blocked** | Peers the user has blocked |

**Filter implementation:**
```rust
pub enum PeerFilter {
    All,
    Connected,
    Online,              // Connected + last_seen < 5 minutes ago
    PendingInbound,
    PendingOutbound,
    Nearby,              // Discovered via mDNS in last 10 minutes
    Blocked,
}

fn filter_peers(peers: &[Peer], filter: PeerFilter) -> Vec<Peer> {
    match filter {
        PeerFilter::All => peers.to_vec(),
        PeerFilter::Connected => peers.iter().filter(|p| p.state == Connected).cloned().collect(),
        PeerFilter::Online => peers.iter().filter(|p| p.state == Connected && p.is_online()).cloned().collect(),
        PeerFilter::PendingInbound => peers.iter().filter(|p| p.state == PendingInbound).cloned().collect(),
        PeerFilter::PendingOutbound => peers.iter().filter(|p| p.state == PendingOutbound).cloned().collect(),
        PeerFilter::Nearby => peers.iter().filter(|p| p.discovered_via_mdns() && p.recently_seen()).cloned().collect(),
        PeerFilter::Blocked => peers.iter().filter(|p| p.state == Blocked).cloned().collect(),
    }
}
```

---

## Social Event Bus

The social protocol emits events to a local event bus for UI updates, logging, and downstream processing.

### Event Types

```rust
pub enum SocialEvent {
    // Connection state changes
    ConnectRequestReceived { from: PeerId, request: ConnectRequest },
    ConnectRequestSent { to: PeerId, request_id: [u8; 32] },
    ConnectRequestAccepted { peer: PeerId },
    ConnectRequestRejected { peer: PeerId },
    ConnectRequestExpired { peer: PeerId },
    PeerConnected { peer: PeerId },
    PeerDisconnected { peer: PeerId },
    PeerBlocked { peer: PeerId },
    PeerUnblocked { peer: PeerId },
    PeerRemoved { peer: PeerId },

    // Trust changes
    TrustLevelChanged { peer: PeerId, old_level: TrustLevel, new_level: TrustLevel },
    TrustScoreUpdated { peer: PeerId, new_score: f64 },
    AttestationReceived { from: PeerId, attestation: Attestation },
    AttestationRevoked { from: PeerId, subject: PeerId },

    // Discovery
    PeerDiscovered { peer: PeerId, method: DiscoveryMethod },
    PeerIntroduced { introducer: PeerId, introduced: PeerId },

    // Key events
    KeyMismatch { peer: PeerId, expected: String, received: String },
    KeyPinned { peer: PeerId, fingerprint: String },

    // Online status
    PeerOnlineStatusChanged { peer: PeerId, online: bool },
}
```

### Event Handling

Events are processed by subscribers (UI components, logging, analytics):

```rust
// Example: UI subscriber
event_bus.subscribe(|event: SocialEvent| {
    match event {
        SocialEvent::ConnectRequestReceived { from, request } => {
            show_notification(format!("{} wants to connect", request.display_name));
            update_pending_inbound_list();
        }
        SocialEvent::PeerConnected { peer } => {
            update_online_status(peer, true);
        }
        SocialEvent::KeyMismatch { peer, expected, received } => {
            show_security_alert(format!("Key mismatch for {}!", peer));
        }
        _ => {}
    }
});
```

**Event persistence:** Social events are persisted to the local database for audit logging. Events older than 90 days are pruned (configurable).

---

## Security Properties

| Property | Mechanism |
|----------|-----------|
| **Request authenticity** | Ed25519 signatures on all ConnectRequests and responses |
| **Replay resistance** | Unique request IDs + timestamp validation (24h window) |
| **MITM detection** | Key pinning on first contact; mismatch alerts |
| **Trust score integrity** | Attestations signed and verifiable; capped bonuses prevent inflation |
| **Attestation authenticity** | Ed25519 signatures; attester must be Trusted or higher |
| **Block enforcement** | Blocked peers' messages are silently dropped |
| **Privacy** | Social profiles contain only user-provided data; no automatic data sharing |

### Threat Model

| Adversary | Capability | Protection |
|-----------|-----------|------------|
| **Fake identity** | Creates fake profile with stolen name | Key pinning; WoT verification; manual verification |
| **Attestation spam** | Creates many fake attestations | Attester must be Trusted; trust score caps prevent inflation |
| **Connection flooding** | Sends many ConnectRequests | Rate limiting per peer; automatic block after threshold |
| **Social engineering** | Tricks user into accepting malicious request | User education; trust level warnings for unknown peers |
| **Key rotation attack** | Attacker claims peer rotated keys | MITM warning; user must manually verify new key |

---

## Reference Implementation

See the following modules in the GhostWire reference implementation:

| Module | Path |
|--------|------|
| Connection state machine | `src/core/social/state_machine.rs` |
| ConnectRequest/Response | `src/core/social/connect.rs` |
| Web of Trust | `src/core/social/trust_store.rs` |
| Attestations | `src/core/social/attestation.rs` |
| Peer filtering | `src/core/social/filter.rs` |
| Social event bus | `src/core/social/events.rs` |
| Introduction protocol | `src/core/social/introduction.rs` |
