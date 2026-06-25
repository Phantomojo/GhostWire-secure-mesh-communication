# Ghostwire — Build Instructions (Wave 7 Fixes)

## Requirements

- **Rust >= 1.80** (the apt package on Ubuntu 24 is 1.75 which is too old due to
  `time v0.3.47` requiring `edition2024`. Use rustup to get a current toolchain.)
- Linux or macOS (Windows needs minor path adjustments)
- ~2 GB disk space for dependencies

## Quick start

```bash
# 1. Install current Rust (if not already)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 2. Verify version
rustc --version   # should be >= 1.80

# 3. Build
cd ghostwire
cargo build --release 2>&1 | tee build.log

# 4. Run
./target/release/ghostwire --help
./target/release/ghostwire serve --port 8080
```

## Optional: inject release signing key at build time

```bash
# Generate a keypair (do this once, keep private key offline)
# Using ed25519-keygen or openssl or ssh-keygen -t ed25519

# Set the public key as hex (64 chars = 32 bytes) before building
export GHOSTWIRE_RELEASE_SIGNING_KEY_HEX="<your 64-char hex pubkey here>"
cargo build --release
```

## Feature flags

```bash
# Minimal build (no Bluetooth, no QR)
cargo build --release --no-default-features --features "web,cli,mesh,security,stealth"

# With Matrix bridge
cargo build --release --features "matrix-bridge"

# With LoRa serial bridge (Wave 7)
cargo build --release --features "meshtastic"
```

## Known build notes

- `rand` crate pinned to 0.8.x — 0.9 breaks rand_core 0.6 compat with argon2/x25519-dalek/libp2p-webrtc
- `obfs4` crate is commented out — the 0.1.0-alpha.1 is non-functional. Wire in when stable.
- `btleplug` compiles but Bluetooth is not yet connected to the mesh. Feature-gated: `--features bluetooth`.
- `libp2p-webrtc` is alpha — if it causes issues, remove from Cargo.toml (not used in main path).

## Running tests

```bash
cargo test 2>&1 | tee test.log

# Run only the new crypto tests
cargo test sphinx -- --nocapture
cargo test wire_protocol -- --nocapture  
cargo test ghost_channels -- --nocapture
cargo test crypto_machine -- --nocapture
```

## Environment variables at runtime

| Variable | Purpose |
|----------|---------|
| `GHOSTWIRE_RELEASE_SIGNING_KEY` | Ed25519 pubkey hex for update verification |
| `GHOSTWIRE_PORT` | Override default port (8080) |
| `GHOSTWIRE_PROFILE` | Identity profile name to load |
| `GHOSTWIRE_LOG` | Tracing filter, e.g. `ghostwire=debug,libp2p=warn` |

