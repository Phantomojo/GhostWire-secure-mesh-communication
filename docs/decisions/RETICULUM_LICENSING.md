# Decision: Reticulum Licensing & Integration Path

**Date**: July 2026
**Status**: Accepted
**Supersedes**: Section 5 of `docs/EVALUATION_RETICULUM_VS_LIBP2P.md`

---

## Context

GhostWire needs LoRa mesh networking for OTF hardware validation (ESP32, LoRa, smartphones). Reticulum is the only mature LoRa mesh stack, but its license has restrictions. This decision documents the full license analysis, all Rust implementations, and the chosen integration path.

## Decision

**Use Beechat/Reticulum-rs (MIT) as the Rust Reticulum integration.** Accept that LoRa interfaces (Serial/KISS/RNode) are not yet implemented — contribute them or fork when needed.

## Rationale

### Python Reticulum License

The Reticulum License is modified MIT with two extra conditions:
1. No use in systems that harm humans
2. No use in AI/ML training datasets

**What it restricts**: Training models on the source code. That's it.

**What it does NOT restrict** (per maintainer markqvist, GitHub #790 and #1062):
- Using Reticulum as a library/dependency
- Sending any data over the network
- Using AI coding tools
- Academic research

**The real blocker is packaging**: Debian classifies it `non-free` (DFSG #6 violation), F-Droid won't accept it, NixOS may drop pre-built binaries.

### Rust Implementation Landscape

| Project | License | Anti-AI? | LoRa | Status |
|---|---|---|---|---|
| **Beechat/Reticulum-rs** | **MIT** | **No** | ❌ | v0.1.0, 278 stars, active |
| lelloman/rns-rs | Reticulum License | Yes | ✅ Full | 900+ tests, most complete |
| ratspeak/rsReticulum | AGPL-3.0 | No (copyleft) | ✅ Full | v1.0.1, active |
| FreeTAKTeam/LXMF-rs | EPL-2.0 | No | Partial | v0.9.5 |
| Python Reticulum | Reticulum License | Yes | ✅ Full | Reference impl |

**Beechat's reticulum-rs is plain MIT** — clean-room reimplementation, not a Python fork. The Reticulum protocol itself is public domain.

### Why Not the Others

- **lelloman/rns-rs**: Anti-AI clause conflicts with GhostWire's open-source goals
- **ratspeak/rsReticulum**: AGPL forces GhostWire to open-source everything
- **Python FFI**: Dependency bloat, same license issues for packaging
- **Direct LoRa**: Must build mesh stack from scratch (months of work)

### Why Beechat

- MIT license — no restrictions, no packaging issues
- Wire-compatible with Python Reticulum
- TCP/UDP transport works now (v0.1.0)
- LoRa interfaces (Serial/KISS/RNode) can be added
- Protocol is public domain — we can reimplement if Beechat stalls

## Consequences

1. **LoRa not available immediately** — Beechat lacks Serial/KISS/RNode interfaces
2. **Contribution path** — contribute RNode interface to Beechat's repo, or fork and add it
3. **License clean** — MIT means no packaging restrictions, no anti-AI clause, no copyleft
4. **Fallback** — if Beechat stalls, reimplement the protocol (public domain) under MIT

## References

- [Beechat/Reticulum-rs](https://github.com/BeechatNetworkSystemsLtd/Reticulum-rs) — MIT, v0.1.0
- [lelloman/rns-rs](https://github.com/lelloman/rns-rs) — Reticulum License, most complete
- [ratspeak/rsReticulum](https://github.com/ratspeak/rsReticulum) — AGPL-3.0, v1.0.1
- [Reticulum Discussion #790](https://github.com/markqvist/Reticulum/discussions/790) — License change discussion
- [Reticulum Discussion #1062](https://github.com/markqvist/Reticulum/discussions/1062) — Maintainer clarifications
- [Reticulum License](https://github.com/markqvist/Reticulum/blob/master/LICENSE) — Full license text
- [docs/EVALUATION_RETICULUM_VS_LIBP2P.md](../EVALUATION_RETICULUM_VS_LIBP2P.md) — Full protocol comparison
- [docs/INTEGRATION_RETICULUM.md](../INTEGRATION_RETICULUM.md) — 4-phase integration plan
