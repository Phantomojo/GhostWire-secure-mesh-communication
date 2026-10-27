# GhostWire IFAC Distribution Design

## Problem

IFAC (Interface Access Codes) makes Reticulum a private overlay — packets without
valid IFAC signatures are dropped at the interface level. But the current
implementation hardcodes the passphrase in source code:

```
network_name = ghostwire-mesh-v1
passphrase = ghostwire-default-ifac
```

Anyone who reads the source can join. This is security through obscurity, not a
private network. The `publish_ifac = yes` design shares credentials through
encrypted announces, but the initial bootstrap still uses the hardcoded default.

## Threat Model

| Threat | Current (hardcoded) | Desired |
|--------|-------------------|---------|
| Public RNS node accidentally joins | ❌ Blocked (IFAC works) | ✅ Blocked |
| Adversary reads source, joins overlay | ❌ Can join (passphrase known) | ✅ Cannot join without invite |
| Adversary eavesdrops on RNS traffic | ✅ Blocked (IFAC drops packets) | ✅ Blocked |
| GhostWire node discovers another GhostWire node | ✅ Works (same IFAC) | ✅ Works (via invite token) |
| Remote user onboards without physical proximity | ❌ Hardcoded (anyone can use) | ✅ Invite token shared remotely |

## Design: Signed Invite Tokens

### Concept

A GhostWire node generates an **invite token** containing the IFAC credentials.
The token is signed with the node's Ed25519 identity key and can be shared via
any channel (text message, email, QR code, clipboard). The recipient verifies
the signature, extracts the credentials, and configures their RNS interface.

This is NOT a certificate (X.509) — it's a simpler signed blob, like a JWT
but for RNS IFAC credentials. No PKI needed. Trust is established through
the social channel used to share the token (you trust the person who sent it).

### Token Structure

```json
{
  "v": 1,                          // token version
  "net": "ghostwire-mesh-v1",      // network_name
  "ifac": "<base64-encoded seed>", // IFAC passphrase/seed (32 bytes)
  "iss": "<issuer DID>",           // issuer's DID (did:key:z6Mk...)
  "exp": 1700000000,              // expiry (Unix timestamp, 0 = never)
  "uses": 1,                       // max uses (0 = unlimited)
  "sig": "<base64-encoded Ed25519 signature>"
}
```

The signature covers everything except `sig` itself:
```
sig = Ed25519.sign(issuer_private_key, canonical_json({v, net, ifac, iss, exp, uses}))
```

### Token Encoding

For sharing:
- **Text**: base64url-encoded JSON (copy-paste into any channel)
- **QR code**: same base64url string, rendered as QR
- **Short code**: optional 6-digit safety number for voice verification

Example token (base64url):
```
eyJ2IjoxLCJuZXQiOiJnaG9zdHdpcmUtbWVzaC12MSIsImlmYWMiOiJhYmNk...}
```

### Onboarding Flow

```
Node A (inviter)                    Node B (invitee)
     |                                   |
     |-- Generate invite token ---------->|
     |   (signed with A's identity key)  |
     |                                   |
     |   [Token shared via any channel]  |
     |   (text, email, QR, voice, etc.)  |
     |                                   |
     |                                   |-- Verify signature (A's pubkey)
     |                                   |-- Check expiry
     |                                   |-- Check uses remaining
     |                                   |-- Extract IFAC credentials
     |                                   |-- Configure RNS interface
     |                                   |-- Re-announce with new IFAC
     |                                   |
     |<-- RNS announce received ---------|
     |   (IFAC matches, node appears)    |
```

### Security Properties

1. **Authenticity**: Token is signed with issuer's Ed25519 key. Recipient
   verifies against issuer's DID (which is derived from the public key).

2. **Integrity**: Signature covers all fields. Tampering invalidates signature.

3. **Expiry**: Token has an `exp` field. Expired tokens are rejected.

4. **Usage limit**: Token has a `uses` field. After N uses, token is consumed.

5. **Revocation**: Issuer can revoke by publishing a revocation to the DHT
   (existing revocation system in spec/README.md §6.2).

6. **No PKI**: Trust is established through the social channel (you trust
   the person who sent you the token). No certificate authority needed.

7. **Forward secrecy**: IFAC credentials are shared once. If compromised,
   generate a new token and re-onboard all nodes.

### IFAC Passphrase Generation

Instead of a hardcoded passphrase, the invite token generates a random IFAC
seed:

```rust
// On first profile creation (or when generating an invite)
let ifac_seed: [u8; 32] = OsRng.fill_bytes();
let ifac_passphrase = base64::encode(&ifac_seed);
```

The IFAC seed is stored in the profile's encrypted keystore alongside the
identity key. It is never stored in plaintext or source code.

### Multi-Node Onboarding

For a group of N nodes:

1. **Node A** generates an invite token with `uses: N-1`
2. **Node A** shares the token with N-1 nodes (via any channel)
3. Each recipient verifies and applies the token
4. All N nodes now share the same IFAC credentials
5. RNS announces flow between all nodes

For adding a new node later:
- Any existing node generates a new invite token
- Token is shared with the new node
- New node verifies and applies

### Bridge Mode (Public RNS Interoperability)

GhostWire can optionally bridge to public RNS for interoperability:

```
GhostWire Private Overlay (IFAC-protected)
    |
    +-- Bridge Node (dual interface)
         |
         +-- Public RNS (no IFAC)
```

The bridge node has two RNS interfaces:
1. Private interface (with IFAC) — GhostWire overlay
2. Public interface (no IFAC) — public RNS

Messages crossing the bridge are re-announced. This is like ProtonMail
bridging to Gmail — GhostWire nodes can optionally reach public RNS peers.

Bridge mode is **opt-in** and **disabled by default**.

### Implementation Phases

#### Phase 1: Replace Hardcoded IFAC
- Generate random IFAC seed on first profile creation
- Store in encrypted keystore
- Use generated seed in RNS config instead of hardcoded passphrase
- **Status**: Not started

#### Phase 2: Invite Token Generation
- Implement token struct (JSON + Ed25519 signature)
- Add `generate_invite_token()` to Core
- Add `verify_invite_token()` to Core
- Encode as base64url for sharing
- **Status**: Not started

#### Phase 3: Invite Token Application
- Add `apply_invite_token()` to Core
- Configure RNS interface with new IFAC credentials
- Re-announce with new IFAC
- **Status**: Not started

#### Phase 4: QR Code Rendering
- Render invite token as QR code in web UI
- Scan QR code in web UI to apply token
- **Status**: Not started (web UI change)

#### Phase 5: Bridge Mode
- Dual-interface RNS configuration
- Bridge node discovery
- Message forwarding between overlays
- **Status**: Designed, not implemented

### Comparison with Existing Systems

| System | Mechanism | Remote OK? | Trust Model |
|--------|-----------|-----------|-------------|
| Signal | QR code scan | ❌ (proximity) | TOFU |
| WhatsApp | QR code scan | ❌ (proximity) | TOFU |
| Telegram | Phone number | ✅ (remote) | Phone number |
| Matrix | Homeserver URL | ✅ (remote) | Server trust |
| **GhostWire (proposed)** | Signed invite token | ✅ (remote) | Social channel + Ed25519 |

GhostWire's approach is unique: it combines the security of cryptographic
signatures with the flexibility of any sharing channel. No phone number, no
server, no proximity required.

### Open Questions

1. **Token expiry default**: 24 hours? 7 days? Never? (Recommend: 24h for security)
2. **Max uses default**: 1 (single-use) or unlimited? (Recommend: 1 for security)
3. **Revocation propagation**: How fast does a revocation reach all nodes?
   (Recommend: DHT-based, eventual consistency)
4. **IFAC rotation**: Should IFAC credentials rotate periodically?
   (Recommend: yes, but only when all nodes are online to receive new token)
5. **Bridge node selection**: Who decides which node is the bridge?
   (Recommend: operator configures explicitly)
