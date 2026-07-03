# Installation

GhostWire v0.2.0. AGPL-3.0-or-later.

## Prerequisites

| Requirement | Minimum | Notes |
|---|---|---|
| Rust | >= 1.80 | `time v0.3.47` needs `edition2024`. Ubuntu 24's apt package (1.75) is too old. Use rustup. |
| OS | Linux or macOS | Windows works with minor path adjustments. |
| Disk | ~2 GB | For compiled dependencies. |
| BlueZ | Optional | Required for Bluetooth mesh. Any adapter BlueZ supports (built-in, USB dongle). |

### Install Rust (if needed)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
rustc --version  # should be >= 1.80
```

## From Source

```bash
git clone https://github.com/ghostwire/ghostwire.git
cd ghostwire
cargo build --release
```

The binary lands at `target/release/ghostwire`.

### System-wide install

```bash
cargo install --path .
```

This puts `ghostwire` in `~/.cargo/bin/` which should be on your PATH.

### Inject release signing key (optional)

Set the hex-encoded ed25519 public key (64 chars = 32 bytes) before building. This key verifies automatic updates at runtime.

```bash
export GHOSTWIRE_RELEASE_SIGNING_KEY_HEX="<64-char-hex-pubkey>"
cargo build --release
```

Generate the keypair once (with `ed25519-keygen`, `ssh-keygen -t ed25519`, or similar). Keep the private key offline.

## Binary Download

Pre-built binaries for supported platforms are published on GitHub Releases. Check:

```
https://github.com/ghostwire/ghostwire/releases
```

## Feature Flags

Default features: `web`, `cli`, `mesh`, `security`, `stealth`, `qr`.

| Feature | What it does | Extra deps |
|---|---|---|
| `web` | Web UI and HTTP API server | None |
| `cli` | CLI subcommands (start, stop, tui, send, etc.) | None |
| `mesh` | libp2p mesh networking | None |
| `security` | Encryption, key management, threat detection | None |
| `stealth` | Obfuscated TCP transport (DPI evasion) | None |
| `qr` | QR code generation for peer sharing | `qrcode-generator` |
| `matrix` | Matrix protocol support (base) | None |
| `matrix-bridge` | Full Matrix bridge | `matrix-sdk` |
| `meshtastic` | LoRa serial bridge for Meshtastic devices | `serialport` |
| `reticulum` | Reticulum network transport | None |
| `briar` | Briar peer-to-peer protocol | None |
| `bluetooth` | Bluetooth mesh via BlueZ | `bluer` (needs BlueZ installed) |
| `minimal` | Stripped-down build, no frills | None |

### Build examples

```bash
# Default build
cargo build --release

# Minimal build (no QR, no Bluetooth)
cargo build --release --no-default-features --features "web,cli,mesh,security,stealth"

# With Matrix bridge
cargo build --release --features "matrix-bridge"

# With Meshtastic LoRa
cargo build --release --features "meshtastic"

# With Bluetooth mesh
cargo build --release --features "bluetooth"
```

Note: `rand` is pinned to 0.8.x. Version 0.9 breaks compatibility with `argon2`, `x25519-dalek`, and `libp2p-webrtc`.

## Docker

No Dockerfile ships with the repo yet. A basic one for reference:

```dockerfile
FROM rust:1.80-slim AS builder
WORKDIR /build
COPY . .
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /build/target/release/ghostwire /usr/local/bin/
ENTRYPOINT ["ghostwire"]
```

## Post-Install Verification

```bash
# Check version
ghostwire --version

# Start the TUI (auto-starts daemon if needed)
ghostwire tui

# Or start the daemon explicitly, then check status
ghostwire start
ghostwire status
```

The daemon listens on port **9097** by default. The API is at `http://127.0.0.1:9097`.

### Run tests

```bash
cargo test
```

203 tests pass. A few pre-existing failures (identity storage, social graph) and 2 network-hang skips exist upstream. Known issue, not a build problem.

## Troubleshooting

**Build fails with `time` crate errors**: Rust version too old. Install >= 1.80 via rustup, not the system package.

**Bluetooth features don't compile**: Make sure BlueZ development headers are installed (`libbluetooth-dev` on Debian/Ubuntu, `bluez-libs-devel` on Fedora).

**libp2p-webrtc build issues**: This crate is alpha. If it causes problems, remove it from Cargo.toml. It's not used in the main code path.

**`rand` version conflicts**: The `rand` crate must stay at 0.8.x. Do not upgrade to 0.9 without checking argon2, x25519-dalek, and libp2p-webrtc compatibility.
