# GhostWire Troubleshooting

## 1. Build Issues

### Rust version too old

```
error[E0670]: `edition2024` is required
```

The `time` crate (v0.3.47) requires Rust edition 2024 support. Ubuntu's apt package is 1.75, which is too old.

**Fix**: Install via rustup, not apt.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
rustc --version   # must be >= 1.80
```

### Missing system libraries

```
error: failed to run custom build script for `libbluer-sys`
```

`bluer` (Bluetooth) needs BlueZ development headers.

```bash
# Debian/Ubuntu
sudo apt install libbluetooth-dev pkg-config

# Arch
sudo pacman -S bluez
```

If you don't need Bluetooth, build without it:

```bash
cargo build --no-default-features --features "web,cli,mesh,security,stealth"
```

### Feature flag build errors

```
error[E0432]: unresolved import `serialport`
```

`meshtastic` and `matrix-bridge` features pull in optional dependencies. Don't enable them unless you have the hardware/infrastructure.

```bash
# Safe build (no optional extras)
cargo build --release

# With Matrix bridge (needs matrix-sdk)
cargo build --release --features "matrix-bridge"

# With LoRa serial (needs USB adapter)
cargo build --release --features "meshtastic"
```

### `rand` version conflict

```
error[E0432]: unresolved import `rand_core::OsRng`
```

`rand` is pinned to 0.8.x. Version 0.9 breaks compatibility with `argon2`, `x25519-dalek`, and `libp2p-webrtc`. If you see this, check that `Cargo.lock` hasn't been tampered with.

```bash
git checkout Cargo.lock
cargo build
```

### Slow initial build

First build downloads ~2 GB of dependencies and compiles everything. Expect 5-8 minutes on a modern machine.

Speed it up:

```bash
# Use release profile for speed (already optimized in Cargo.toml)
cargo build --release
```

The release profile has `opt-level = 3` for critical crates (ring, libp2p, tokio) and `lto = false` for faster linking.

---

## 2. Common Runtime Errors

### Port already in use

```
Error: Address already in use (os error 98)
```

The daemon API (default 9097) or web server (default 8080) port is taken.

```bash
# Find what's using the port
ss -tlnp | grep :9097
# or
lsof -i :9097

# Kill the existing process or use a different port
ghostwire start --port 9100
```

The boot sequence checks port 8080 and warns if it's in use.

### Permission denied on data directory

```
Error: Permission denied (os error 13)
```

The daemon writes to `~/.ghostwire/`. If running as a different user (e.g., via systemd), ensure the user owns the directory.

```bash
# Fix ownership
sudo chown -R ghostwire:ghostwire /home/ghostwire/.ghostwire/
```

### Stale PID file

```
Error: Daemon is already running (PID <old_pid>)
```

The daemon crashed without cleaning up its PID file.

```bash
# Remove stale PID
rm ~/.ghostwire/daemon.pid

# Then start
ghostwire start
```

### Configuration load failure

```
Error: Configuration load failed
```

Custom config file is malformed.

```bash
# Run with default config
ghostwire start

# Or fix the TOML syntax in your config file
```

---

## 3. Network Connectivity

### mDNS not discovering peers

mDNS requires the same broadcast domain (flat LAN). It won't work across:

- VLANs
- VPN tunnels
- Different subnets
- Docker bridge networks

**Fix**: Use explicit bootstrap addresses.

```bash
# On the node that knows the peer
ghostwire start --bootstrap /ip4/192.168.1.100/tcp/4001/p2p/<PEER_ID>
```

### Firewall blocking mesh traffic

The mesh transport uses both TCP and UDP on the configured `--mesh-port`.

```bash
# Check if port is open
nc -zv <peer-ip> 4001

# Open the port (nftables)
nft add rule inet filter input tcp dport 4001 accept
nft add rule inet filter input udp dport 4001 accept
```

### NAT traversal issues

GhostWire uses libp2p's `autonat` and `relay` protocols for NAT traversal. If nodes behind NAT can't connect:

1. Check if the relay server is reachable.
2. Try `--bootstrap` with a node that has a public IP.
3. Use Tailscale as an overlay network (`TAILSCALE=1`).

### Tailscale bootstrap not working

Format: `--tailscale-bootstrap <tailscale-ip>/<peer-id>`

```bash
# Wrong
ghostwire start --tailscale-bootstrap 100.64.0.1

# Correct
ghostwire start --tailscale-bootstrap 100.64.0.1/12D3KooW...
```

The peer ID is the full libp2p peer ID (starts with `12D3KooW`).

---

## 4. Known Test Failures

These are pre-existing and tracked in AGENTS.md. They do not block normal operation.

### 2 pre-existing test failures

| Test | Status | Impact |
|---|---|---|
| Identity storage tests | FAIL | Identity save/load has edge cases. Daemon still works for normal use. |
| Social graph tests | FAIL | Social graph feature not fully wired. Does not affect core mesh or messaging. |

### 2 network-hang skips

| Test | Status | Impact |
|---|---|---|
| Network transport startup (1) | SKIP | Hangs on transport init. Skipped in CI. |
| Network transport startup (2) | SKIP | Same root cause as above. |

These skips don't indicate a runtime problem. They're test harness timeouts during transport initialization in the test environment.

### Running tests yourself

```bash
# All tests
cargo test 2>&1 | tail -30

# Skip the hanging tests
cargo test -- --skip "test_network_transport"

# Specific module
cargo test ai::gemma_client::tests
cargo test sphinx -- --nocapture
```

---

## 5. Bluetooth Issues

### BlueZ not running

```
Error: bluetooth daemon not available
```

The `bluer` crate communicates with BlueZ via D-Bus. BlueZ must be running.

```bash
# Check if running
systemctl status bluetooth

# Start it
sudo systemctl start bluetooth
sudo systemctl enable bluetooth
```

### Permission denied on Bluetooth adapter

```
Error: Permission denied (os error 13) on /dev/hci0
```

Your user needs to be in the `bluetooth` group.

```bash
sudo usermod -aG bluetooth $USER
# Log out and back in for group change to take effect
```

### Bluetooth feature not compiled

The `bluetooth` feature is disabled by default because `ble_controller` isn't implemented yet.

```bash
# Build with Bluetooth support
cargo build --features "bluetooth"

# Runtime: must also pass the feature at compile time
./target/release/ghostwire --help
```

### No Bluetooth adapter detected

```bash
# Check for adapters
bluetoothctl list

# If empty, the adapter isn't recognized by BlueZ.
# Try a USB Bluetooth dongle.
```

---

## 6. AI Module Issues

### ONNX inference broken

```
error[E0432]: unresolved import `ort::Value`
```

`onnx_inference.rs` is currently commented out due to an `ort` crate import error. The ONNX runtime engine (`ort` v2.0.0-rc.4) is a release candidate and has breaking API changes.

**Status**: HIGH severity, tracked in AGENTS.md. ONNX inference does not work. The system falls back to PRoPHET (Layer 4) routing.

### Gemma client not wired

`initialize_gemma_client()` exists in `src/core/ai/gemma_client.rs` but never assigns to `self.gemma_client`. The Gemma 4 Ollama client is functional as a standalone HTTP client but isn't called in routing decisions.

**Status**: CRITICAL severity. Routing uses only PRoPHET (Layer 4), not the AI layers.

### Anomaly detector duplicates

`anomaly_detector.rs` duplicates the trained `models/anomaly_detector.joblib`. The Isolation Forest model should be loaded from the joblib file, not reimplemented.

### What actually works

| Layer | Status |
|---|---|
| Layer 4 (PRoPHET) | Working. In `src/core/dtn/epidemic.rs`. |
| Layer 1 (ONNX) | Broken. Import error. |
| Layer 2 (Gemma) | Client exists. Not wired to routing. |
| Layer 3 (GNN) | Offline training only. |

Current routing accuracy: 2.69% (target: 85-95%).

---

## 7. Performance Tips

### Use release builds

Debug builds are 254 MB and 10-50x slower.

```bash
cargo build --release
```

Release profile settings in `Cargo.toml`:

- `opt-level = 2` (main binary)
- `opt-level = 3` (ring, libp2p, tokio)
- `panic = "abort"` (smaller binary)
- `strip = true` (debug symbols removed)
- `lto = false` (faster build, slight size tradeoff)

### Reduce compile time

```bash
# Check only (no linking)
cargo check

# Faster lint
cargo clippy -- -D warnings
```

### Reduce binary size

The release profile already strips debug symbols and uses `panic = "abort"`. For further reduction:

```bash
# Strip manually (already in profile, but explicit)
strip target/release/ghostwire
```

### Memory considerations

- Sled database grows with message history. Monitor `~/.ghostwire/db/` size.
- The mesh transport maintains connection state per peer. `large` mesh mode uses more memory.
- AI model loading (ONNX, Gemma) adds memory overhead when enabled.

### Network performance

- Use `--mesh-size small` for 2-5 peer deployments (less gossipsub overhead).
- Use `--mesh-size large` only when you actually have 20+ peers.
- The mesh transport binds to `0.0.0.0` by default. Restrict to a specific interface if on a multi-homed machine:

```bash
ghostwire start --mesh-host 192.168.1.50 --mesh-port 4001
```
