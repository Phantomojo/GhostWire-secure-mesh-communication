# Security Audit — Remaining Items

## Fixed (11 of 25)

| # | Finding | Fix Location |
|---|---------|-------------|
| 1 | master.key single point of failure | Key rotation wired (encryption.rs, mesh.rs) |
| 2 | mesh.rs:1878 logs plaintext msg.content | Log redaction (content redacted) |
| 3 | tui.log initial password in plaintext | Already fixed in current code |
| 4 | traffic_armor.wrap() not wired | Wired into send path (mesh.rs) |
| 5 | .identity.key uses SHA256 | Argon2id v3 format (core_identity.rs) |
| 6 | Identity encryption weak Argon2 params | Upgraded to 64 MiB (identity/encryption.rs) |
| 7 | File permissions 0o644 on encrypted blobs | 0o600 (encryption.rs, core_identity.rs) |
| 8 | /api/v2/profile public | Auth required (web.rs) |
| 9 | /api/status public | Auth required (web.rs) |
| 10 | GossipSub topic names identifiable | BLAKE3 hash topics (constants.rs) |
| 11 | Auth token non-constant-time comparison | subtle::ConstantTimeEq (web.rs) |

## Remaining HIGH (3)

| # | Finding | Effort | Notes |
|---|---------|--------|-------|
| 12 | presence_obfuscation not wired | Medium | Module exists but zero callers. Wire into connection handler. |
| 13 | Log redaction incomplete | Small | Only mesh.rs:1878 fixed. Check all log sites for IP/PeerID leakage. |
| 14 | master.key no key-wrapping | Medium | Raw 32-byte key on disk. Wrap with machine-derived key or HSM. |

## Remaining MEDIUM (7)

| # | Finding | Effort | Notes |
|---|---------|--------|-------|
| 15 | mDNS broadcasts identity on LAN | Small | Disable or obfuscate mDNS service name. |
| 16 | Static HKDF salt for ECDH sessions | Small | Generate random salt per session. |
| 17 | clear() misses messages_plain tree | Trivial | Add `drop_tree("messages_plain")` to store.rs clear(). |
| 18 | AUTH_TOKEN/JWT_SECRET not zeroized | Small | Use Zeroize trait on auth state. |
| 19 | CLI send exposes content in process list | Small | Read content from stdin/pipe instead of argv. |
| 20 | ValueMeta.created_at plaintext | Small | Encrypt or remove timestamp from sled metadata. |
| 21 | Mailbox metadata plaintext on disk | Small | Encrypt sled values in MailboxStore. |

## Remaining LOW (4)

| # | Finding | Effort | Notes |
|---|---------|--------|-------|
| 22 | .initial-password plaintext on disk | Trivial | Delete after migration to .identity.key. |
| 23 | Leftover test identity files | Trivial | Delete `~/.ghostwire/identity_*.json` test files. |
| 24 | Zeroization gaps in auth state | Small | Zeroize AUTH_TOKEN/JWT_SECRET on drop. |
| 25 | Vector clocks leak peer relationships | Medium | Consider rate-limiting or batching vector clock updates. |
