# Reticulum Licensing Quick Reference

> **TL;DR**: Python Reticulum has an anti-AI license clause. Beechat's Rust port (reticulum-rs) is plain MIT. Use Beechat for GhostWire integration.

## The Anti-AI Clause (Python Reticulum)

```text
The Software shall not be used, directly or indirectly, in the creation of
an artificial intelligence, machine learning or language model training dataset,
including but not limited to any use that contributes to the training or
development of such a model or algorithm.
```

**What it restricts**: Training models on source code. That's it.
**What it does NOT restrict**: Using the library, sending data, AI coding tools.
**Real blocker**: Debian/F-Droid packaging (DFSG #6 violation).

## Rust Implementations

| Project | License | Anti-AI? | LoRa |
|---|---|---|---|
| Beechat/Reticulum-rs | MIT | No | ❌ |
| lelloman/rns-rs | Reticulum License | Yes | ✅ |
| ratspeak/rsReticulum | AGPL-3.0 | No | ✅ |
| Python Reticulum | Reticulum License | Yes | ✅ |

## Decision

Use **Beechat/Reticulum-rs (MIT)**. Protocol is public domain — can reimplement if needed.

## Key Links

- Decision record: `docs/decisions/RETICULUM_LICENSING.md`
- Full evaluation: `docs/EVALUATION_RETICULUM_VS_LIBP2P.md`
- Integration plan: `docs/INTEGRATION_RETICULUM.md`
