# Nostr Protocol for Mesh Fallback Research

> **Source**: Background agent bg_2c58cc93 (librarian agent)
> **Date**: 2026-07-27
> **Status**: Raw agent output — unmodified

---

## Nostr Protocol Research for GhostWire Mesh Fallback

### 1. Nostr Basics — Relays, Events, Subscriptions, NIPs

Nostr (Notes and Other Stuff Transmitted by Relays) has two components: **clients** and **relays**. Clients publish/sign events; relays store and forward them over WebSocket.

**Event Structure** (NIP-01):
```json
{
  "id": "<SHA-256 hash>",
  "pubkey": "<schnorr secp256k1 pubkey>",
  "created_at": <unix timestamp>,
  "kind": <integer>,
  "tags": [["e", ...], ["p", ...], ["g", ...]],
  "content": "<string>",
  "sig": "<64-byte schnorr sig>"
}
```

**Client-Relay Messages** (WebSocket JSON arrays):
- Client to Relay: `EVENT`, `REQ` (subscribe), `CLOSE`
- Relay to Client: `EVENT` (delivery), `OK` (ack), `EOSE` (end of stored), `NOTICE`, `CLOSED`

**Event Kind Ranges** (critical for GhostWire):
| Range | Behavior | Use |
|-------|----------|-----|
| 0-9999 | Regular (stored) | Text notes, DMs |
| 10000-19999 | Replaceable | Only latest per pubkey kept |
| **20000-29999** | **Ephemeral** | **Not stored — forwarded to live subscribers only** |
| 30000-39999 | Addressable | Latest per pubkey+kind+d tag |

**Filters** support `ids`, `authors`, `kinds`, `#<single-letter>` tag filters, `since`/`until`, `limit`. Multiple filters in one REQ = OR; fields within a filter = AND.

---

### 2. Key NIPs for GhostWire

#### NIP-04 — Encrypted Direct Messages (kind 4)
- Legacy. AES-256-CBC with base64. **Flawed** — CBC malleability, no padding oracle protection, leaks metadata.
- **Do not use this.** NIP-44 + NIP-59 replace it.

#### NIP-44 v2 — Versioned Encrypted Payloads (CURRENT STANDARD)
The crypto stack for GhostWire:
```
1. ECDH(priv_A, pub_B) -> shared_x (unhashed 32-byte x coordinate)
2. HKDF-extract(shared_x, salt="nip44-v2") -> conversation_key
3. 32-byte CSPRNG nonce -> HKDF-expand(conversation_key, nonce, L=76)
   -> chacha_key(32) + chacha_nonce(12) + hmac_key(20)
4. pad(plaintext) -> ChaCha20 -> HMAC-SHA256(nonce || ciphertext)
5. base64(version(1) || nonce(32) || ciphertext || mac(32))
```

- Uses ChaCha20 + HMAC-SHA256 (not AES — faster, multi-key resistant)
- Custom padding scheme (better length leakage protection than padme)
- Maximum plaintext: 64KB (v2) / ~4GB (v3 — in draft)

**NIP-44 v3** (draft as of 2026, PR #1838) adds context binding, binary encoding, and decouples encryption keys from signing keys.

#### NIP-59 — Gift Wrap (kind 1059 / kind 21059)
Three-layer encapsulation for **sender anonymity**:
```
Layer 1: Rumor — unsigned event with real content (deniability)
Layer 2: Seal (kind 13) — rumor encrypted via NIP-44 to recipient, signed by sender, empty tags
Layer 3: Gift Wrap (kind 1059) — seal encrypted via NIP-44 with EPHEMERAL key, signed by ephemeral key, [p] tag to recipient
```

- **Relays see only**: ephemeral pubkey + recipient p-tag. Cannot identify the real sender.
- **Ephemeral variant**: kind:21059 (merged May 2026) — for real-time apps that do not need persistence.
- Relays SHOULD delete kind:1059 events via NIP-09/NIP-62 on request.

For GhostWire: use kind:21059 for live mesh bridge traffic, kind:1059 for store-and-forward offline messages.

#### NIP-65 — Relay List Metadata (kind 10002)
The "outbox model" — users advertise which relays they read from / write to:
```json
{
  "kind": 10002,
  "tags": [
    ["r", "wss://relay.damus.io", "read"],
    ["r", "wss://my-relay.example.com", "write"]
  ],
  "content": ""
}
```
- **Write relays** = where you publish -> others query these to find your content
- **Read relays** = where you read mentions -> others publish to these to reach you
- Keep lists small (2-4 per category)
- Publish your kind:10002 to well-known indexer relays

---

### 3. BitChat's Nostr Usage — The Model for GhostWire

BitChat is the closest existing project to GhostWire. It is a **dual-transport** messenger: BLE mesh for local + Nostr for global. Key design:

**Ephemeral Geohash Events (kind=20000)**:
- Location-based chat using geohash-precision channels
- Tagged with `["g", geohash]` — clients subscribe by `#g` filter
- Ephemeral (kind 20000-29999), so relays do not store them long-term
- Default fetch limit: 1000 events
- Content is message text + optional `["n", nickname]` tag

**Presence Heartbeats (kind=20001)**:
- Empty content. No nickname. Just `["g", geohash]` tag.
- Broadcast every 40-80s (randomized to prevent correlation)
- Privacy: **only broadcast at precision 5** (region/province/city). Never at neighborhood/block level.
- Per-broadcast random delays (2-5s) between different geohash levels prevent temporal correlation
- "Online" = any event (20000 or 20001) seen within 5 minutes

**Geohash-Derived Identities**:
- Each geohash channel gets an **ephemeral pubkey** derived from the user's master key + geohash string
- Fresh identity per location, preventing cross-location correlation

**Dual Transport Fallback Chain**:
```
1. BLE mesh (direct Noise session) — fastest, most private
2. Nostr fallback (NIP-17 gift-wrapped DM via global relays)
3. Smart queuing — messages stored until transport available
```

**Relay Stats** (from permissionlesstech/georelays):
- 5,735 known relays globally, ~1,341 reachable (~23.4%)
- 290+ BitChat-compatible (support kind 20000)
- Geographically distributed relay discovery via DNS/SRV

---

### 4. Nostr vs Tor — Can Nostr Provide Anonymity?

**Short answer: No. Nostr is NOT a Tor alternative. It provides censorship resistance, not anonymity.**

| Property | Nostr | Tor |
|----------|-------|-----|
| IP hidden from relays? | No — relay sees your IP | Yes — onion routing |
| Sender identity hidden? | Only with NIP-59 gift wrap | Yes |
| Metadata protected? | Partial (NIP-17/59) | Yes |
| Censorship resistant? | Yes (write to any relay) | Yes |
| Store-and-forward? | Yes (relays persist events) | No (real-time tunnels) |
| Bandwidth overhead | Low (direct WebSocket) | High (3-hop encryption) |
| Message latency | Low (~100-500ms) | High (~1-5s) |

**Nostr attack vectors** (per 2026 research):
- **IP leaks**: Every relay you connect to logs your IP. Mitigation: VPN or Tor before connecting.
- **Metadata leakage**: Even with NIP-17 gift wrap, relays see timing, volume, and recipient. Event kind fingerprints the protocol.
- **Filter correlation**: Subscription filters (REQ messages) reveal what you are looking for.
- **NIP-04 is broken**: CBC malleability allows plaintext extraction via link previews.

**For GhostWire**: Nostr is for **reachability**, not anonymity. GhostWire's mesh layer (Noise Protocol) provides the actual privacy. Nostr is the internet bridge when mesh is unavailable. Use Tor/VPN at the OS level if you need IP anonymity on the Nostr transport.

---

### 5. Relay Infrastructure — Count, Reliability, Censorship Resistance

**Current relay network (2026 data)**:
- Total known: ~5,735 (from rust-nostr/nostr-stats)
- Reachable/online: ~1,341 (23.4%)
- Public/listed: ~471 public, ~191 restricted
- Geographic spread: 50 countries, 151 autonomous systems
- North America: 470, Europe: 295, Asia: 63
- **No single country or AS controls >25%** of relays

**Relay types**:
- **Public relays**: Anyone can read/write. Examples: wss://relay.damus.io, wss://nos.lol
- **Restricted relays**: Require NIP-42 auth, payment, or PoW
- **Personal relays**: You run your own (nostr-rs-relay, strfry, n0str, Stonr)
- **BitChat-compatible**: 290+ relays supporting kind 20000

**Censorship resistance properties**:
- Events are signed -> relays cannot forge or alter content
- Write to any relay -> no single point of control
- You can run your own relay -> your data survives even if all public relays ban you
- Relays are permissionless to operate -> anyone can spin one up
- Weakness: relays can selectively drop events, refuse subscriptions, or go offline
- Mitigation: publish to multiple relays (3-5), prefer NIP-65 outbox model

**For GhostWire**: You do not need to discover all relays — you need a curated set that supports ephemeral events (kinds 20000+) and has good uptime. Run at least one personal relay as anchor, and fall back to public relay pool.

---

### 6. Nostr as Store-and-Forward Layer

**Yes, this is exactly what Nostr is designed for.** It is the killer feature for mesh:

**Core mechanism**: Events are persisted on relays. When a peer comes online:
1. Client opens WebSocket to relay
2. Sends REQ with filters (since: <last_online_timestamp>, kinds: [your-mesh-kind], #p: [your-pubkey])
3. Relay returns all matching events from storage
4. Client processes backlog

**Production implementations**:

**BitChat Prekey System** (Noise X pattern + Nostr kind 1401):
```go
// From peerclaw-agent — Nostr mailbox for offline delivery
mailbox := NewMailbox(MailboxConfig{
    InboxRelays:  []string{"wss://relay.example.com"},
    Ed25519Seed:  seed,  // 32-byte seed -> Nostr keypair
    TTL:          7 * 24 * time.Hour,
    SyncInterval: 5 * time.Minute,
})

// Encrypt via NIP-44, publish as kind 20007 to recipient's inbox relays
mailbox.SendToInbox(ctx, envelope, destRelays, destNostrPubKey)

// Periodic SyncInbox polls for new messages since last sync
mailbox.SyncInbox(ctx)  // Called every SyncInterval
```

**Key design**:
- **Ephemeral one-time prekeys** (Curve25519) distributed via mesh gossip + Nostr
- **Rotating recipient tags**: 16-byte HMAC tag derived from recipient's static key + UTC day — recipient scans for mail without revealing static identity to relay
- **Delivery receipts** (kind 20008) sent back to sender's inbox
- **Exponential backoff** on outbox: 5s base, 5min max, 10 retries
- **Deduplication** via ExpiringIDSet to prevent amplification storms

**For GhostWire**: Use a custom event kind (e.g., kind=20007 for mesh messages, kind=20008 for delivery receipts). When BLE cannot reach destination, publish NIP-44-encrypted message blob to the peer's inbox relays (discovered via NIP-65 or mesh gossip). Peer syncs on reconnect.

---

### 7. Nostr + BLE Mesh Hybrid — How It Works in Practice

**NIP-BE (official Nostr BLE spec)**, merged 2025, defines exactly this:

**Architecture**:
```
                     +------------------+
                     |   Nostr Relay     |
                     |  (Internet)       |
                     +--------+---------+
                              | WebSocket
                    +---------+---------+
                    |   Nostr Client     |
                    |   (Your Device)    |
                    +--------+---------+
                             | NIP-BE (BLE GATT)
                    +--------+--------+
                    |  BLE Peer       |
                    |  (Mesh Node)    |
                    +-----------------+
```

**NIP-BE Protocol**:
- GATT server/client roles assigned by UUID comparison (higher UUID = server/relay)
- All messages follow NIP-01 structure over BLE instead of WebSocket
- DEFLATE compression on messages
- Chunking for BLE MTU: 20-23 bytes (BLE 4.2) or 256 bytes (BLE 5+)
- Half-duplex sync using NIP-77 Negentropy
- Max message size: 64KB

**BitChat's dual transport implementation**:
```
// Pseudocode for GhostWire's transport selector
func Send(message, destination) {
    if bleMesh.IsReachable(destination) {
        // 1. BLE mesh — direct Noise session
        bleMesh.Send(message, destination)
    } else if nostr.HasInternet() {
        // 2. Nostr fallback — gift-wrapped + encrypted
        envelope := noise.Seal(message, destination.Prekey)
        nostr.PublishGiftWrap(envelope, destination.NostrPubKey)
    } else {
        // 3. Queued — BLE mesh broadcast for courier delivery
        bleMesh.Broadcast(message, destination)
    }
}
```

**Practical hybrid flow**:
1. Device A and B are in BLE range -> Noise session, direct messages, no internet needed
2. Device A moves out of range -> messages queue locally
3. Device A's Nostr sync fires (every 5 min) -> publishes encrypted messages to relay(s)
4. Device B comes online -> syncs inbox from relay -> decrypts
5. If Device A is internet-disconnected but BLE-connected to Device C:
   - C acts as **courier**: receives A's outbound messages via BLE
   - C publishes them to Nostr relay (when C has internet)
   - This is BitChat's BridgeCourierService pattern

---

### 8. NIP-13 — Proof of Work for Spam Resistance

**Mechanism**: Mine the event id to have N leading zero bits:
```
nonce tag format: ["nonce", "<counter>", "<target_difficulty>"]
```
- Increment nonce, recalculate SHA-256 id, check leading zero bits
- Difficulty = number of leading zero bits in hash
- `000000000e9d...` = 36 bits difficulty

**Relay enforcement**: Relays advertise required PoW in NIP-11:
```json
{
  "limitation": {
    "pow_difficulty": 16,
    "pow_required": true
  }
}
```

**Recommended difficulties**:
| Event Type | Bits | ~Hashes | Time |
|-----------|------|---------|------|
| Reactions | 8 | 256 | instant |
| Encrypted DMs | 12 | 4,096 | instant |
| Text notes | 16 | 65,536 | ~0.1s |
| Contact lists | 20 | 1,048,576 | ~1.5s |
| Deletions | 20 | 1,048,576 | ~1.5s |

**Delegated PoW**: Since the event id does not commit to the signature, you can outsource PoW to a provider. Useful for battery-constrained mesh nodes.

**For GhostWire**: Use PoW 8-12 for mesh-transit events (low work, mobile-friendly), 16-20 for direct relay submissions. The nostr-sdk crate has built-in pow(20) on EventBuilder and a pow-multi-thread feature flag.

---

### 9. Rust Nostr Libraries

**nostr-sdk** (v0.44.1 stable, v0.45.0-alpha.4) — high-level client SDK:
```toml
[dependencies]
nostr-sdk = { version = "0.44", features = ["nip44", "nip59", "tor"] }
```

Key API surface for GhostWire:
```rust
use nostr_sdk::prelude::*;

// Keys
let keys = Keys::generate();

// Client with Tor proxy support
let client = Client::builder()
    .signer(keys.clone())
    .opts(ClientOptions::new()
        .connection(Connection::new()
            .proxy("127.0.0.1:9050")
            .target(ConnectionTarget::Onion)))
    .build();

client.add_relay("wss://relay.damus.io").await?;
client.connect().await;

// Publish with PoW
let builder = EventBuilder::new(Kind::Custom(20007), "encrypted-payload")
    .pow(16);
client.send_event_builder(builder).await?;

// Gift wrap (NIP-59)
client.gift_wrap(&recipient_pubkey, unsigned_event, []).await?;

// Subscribe with filters
let filter = Filter::new()
    .kind(Kind::Custom(20007))
    .pubkey(recipient_pubkey)
    .since(Timestamp::now() - Duration::hours(24));

client.subscribe(filter, None).await?;
```

**nostr** crate (core protocol types — used by nostr-sdk internally):
```rust
use nostr::{Event, EventBuilder, Keys, Kind, Tag, Tags};
```

**nostr-relay-pool**: Lower-level relay connection management.

**nwc**: Nostr Wallet Connect (NIP-47).

**Feature flags to enable for GhostWire**:
| Feature | Why |
|---------|-----|
| nip44 | Encrypted payloads (mandatory) |
| nip59 | Gift wrap (sender anonymity) |
| tor | Embedded Tor client for .onion relays |
| pow-multi-thread | Fast PoW mining |
| nip65 | Relay list metadata (outbox model) |

---

### Architecture Synthesis: GhostWire + Nostr

```
+---------------------------------------------------+
|                  GhostWire                         |
+---------------------------------------------------+
|  Transport Layer (transport_selector)              |
|  +---------------+  +---------------------------+  |
|  | BLE Mesh     |  | Nostr (internet bridge)   |  |
|  | (NIP-BE)     |  |                           |  |
|  | * Noise P2P  |  | * Kind 20007 (encrypted)  |  |
|  | * 7-hop mesh |  | * Kind 20008 (receipts)   |  |
|  | * Courier    |  | * Kind 21059 (giftwrap)   |  |
|  +---------------+  +---------------------------+  |
+---------------------------------------------------+
|  Encryption Layer                                  |
|  +---------------+  +---------------------------+  |
|  | Noise Proto  |  | NIP-44 + NIP-59           |  |
|  | (mesh)       |  | (internet)                 |  |
|  +---------------+  +---------------------------+  |
+---------------------------------------------------+
|  Identity Layer (derived per-geohash keys)        |
+---------------------------------------------------+
```

**Key design decisions**:
1. **Use kind 20000-29999 range** for mesh-transit events (ephemeral — relays do not need to store mesh chatter long-term)
2. **Separate inbox kind** (e.g., kind 20007) for store-and-forward with explicit retention
3. **NIP-44 + NIP-59** for all internet-transit messages — the relay never sees plaintext or real sender
4. **Geohash tags** (`["g", "dr5rs"]`) with geohash-derived pubkeys for location channels
5. **NIP-65 relay lists** for peer inbox discovery: each node publishes its preferred inbox relays
6. **Hybrid courier**: BLE nodes carry messages for disconnected peers and deposit to Nostr when internet available
7. **Rotating HMAC tags** (BitChat pattern) for scanning inbox without revealing static pubkey
