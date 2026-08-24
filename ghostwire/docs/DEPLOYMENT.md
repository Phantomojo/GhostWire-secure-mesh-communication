# GhostWire Deployment Guide

## 1. Single-Node Deployment (Direct Binary)

Fastest way to run GhostWire. No container overhead.

### Prerequisites

- Rust >= 1.80 (use `rustup`, not apt)
- ~2 GB disk for dependencies
- Linux or macOS (Windows needs path adjustments)

### Build and run

```bash
cd /path/to/ghostwire
cargo build --release
./target/release/ghostwire --help

# Start as interactive process (foreground)
./target/release/ghostwire --port 8080 --mesh-port 4001

# Start as daemon (background)
./target/release/ghostwire start --mesh-port 4001
```

### Daemon management

```bash
# Start daemon
ghostwire start --mesh-port 4001 --bootstrap /ip4/192.168.1.100/tcp/4001/p2p/<PEER_ID>

# Check status
ghostwire status

# Stop
ghostwire stop

# Launch TUI (auto-starts daemon if needed)
ghostwire tui
```

The daemon binds the API to `127.0.0.1` by default. Mesh traffic binds to `0.0.0.0`.

---

## 2. Two-Node Mesh

Reference setup using two development machines.

### Node 1 (primary)

```bash
ghostwire start \
  --mesh-port 4001 \
  --mesh-size small
```

Note the peer ID from the logs (printed at boot). It looks like `12D3KooW...`.

### Node 2 (secondary)

```bash
ghostwire start \
  --mesh-port 4001 \
  --bootstrap /ip4/<NODE_1_LAN_IP>/tcp/4001/p2p/<NODE_1_PEER_ID> \
  --mesh-size small
```

### Verify connectivity

```bash
# On either node
ghostwire peers
# or
curl http://127.0.0.1:9097/api/v2/peers
```

Nodes discover each other via mDNS on the same LAN automatically. The `--bootstrap` flag is a fallback for cross-subnet or first-run scenarios.

---

## 3. Multi-Node Mesh Considerations

| Network Size | Flag | Gossipsub Config |
|---|---|---|
| 2–5 peers | `--mesh-size small` | Default |
| 6–20 peers | `--mesh-size medium` | Larger mesh, more fanout |
| 20+ peers | `--mesh-size large` | Full mesh, higher bandwidth |

### Guidelines

- **Bootstrap nodes**: At least one node with a stable address. Other nodes point to it via `--bootstrap`.
- **mDNS**: Works on flat LANs (same broadcast domain). Cross-VLAN or cross-VPN requires explicit bootstrap.
- **Tailscale integration**: Set `TAILSCALE=1` to auto-scan the `100.64.0.0/10` Tailscale subnet for GhostWire peers. Use `--tailscale-bootstrap <tailscale-ip>/<peer-id>` for specific nodes.
- **Bandwidth**: Each peer maintains connections to its gossipsub mesh. `large` mode uses more connections per peer. Monitor with `ghostwire peers`.
- **Storage**: Each node runs its own sled database. No shared storage.

---

## 4. Docker Deployment

### Dockerfile

```dockerfile
FROM rust:1.82-slim AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libbluetooth-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libbluetooth3 \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -r -s /bin/false ghostwire
RUN mkdir -p /home/ghostwire/.ghostwire && chown ghostwire:ghostwire /home/ghostwire/.ghostwire

COPY --from=builder /app/target/release/ghostwire /usr/local/bin/ghostwire

USER ghostwire
ENV GHOSTWIRE_HOME=/home/ghostwire/.ghostwire

EXPOSE 8080
EXPOSE 4001/udp
EXPOSE 4001/tcp

ENTRYPOINT ["ghostwire"]
CMD ["start", "--port", "8080", "--mesh-port", "4001", "--mesh-host", "0.0.0.0"]
```

### docker-compose.yml

```yaml
version: "3.8"

services:
  ghostwire:
    build: .
    ports:
      - "8080:8080"
      - "4001:4001/tcp"
      - "4001:4001/udp"
    volumes:
      - ghostwire-data:/home/ghostwire/.ghostwire
    environment:
      - GHOSTWIRE_HOME=/home/ghostwire/.ghostwire
      - GHOSTWIRE_LOG=ghostwire=info
    restart: unless-stopped

volumes:
  ghostwire-data:
```

### Docker notes

- The `bluetooth` feature is disabled by default. If you need it, build with `--features bluetooth` and add `--privileged` or device mappings.
- `libp2p-webrtc` is alpha. If it causes build issues in Docker, remove it from `Cargo.toml` (it's not in the main path).
- Mesh port 4001 needs both TCP and UDP for full connectivity.

---

## 5. Health Checks and Monitoring

### Boot sequence

GhostWire runs a 6-phase boot sequence (`startup.rs`):

| Phase | Name | What it checks |
|---|---|---|
| 0 | Pre-flight | Data directory, disk space, port conflicts |
| 1 | Identity | Load/create identity, verify keys |
| 2 | Storage | Sled database opens correctly |
| 3 | Network | Internet, LAN IP, listen port, mDNS |
| 4 | Knowledge | Load ark collections, sync index |
| 5 | Ready | Announce presence, start API |

Each phase emits `BootEvent` structs with pass/warn/fail status.

### Health check endpoint

```bash
curl http://127.0.0.1:9097/api/v2/status
```

### Monitoring the daemon

```bash
# Check if running
ghostwire status

# Watch logs
tail -f ~/.ghostwire/daemon.log

# List connected peers
ghostwire peers
curl http://127.0.0.1:9097/api/v2/peers
```

### Environment variables for logging

| Variable | Example | Effect |
|---|---|---|
| `GHOSTWIRE_LOG` | `ghostwire=debug,libp2p=warn` | Per-module log levels |
| `--verbose` | CLI flag | `ghostwire=debug` |
| `--quiet` | CLI flag | `ghostwire=warn` |

Production builds output structured JSON logs. Debug builds use pretty-printed format.

---

## 6. Persistent Storage

### Data directory

Default location: `~/.ghostwire/`

Override with `GHOSTWIRE_HOME` environment variable.

```
~/.ghostwire/
├── db/                  # Sled database (message store, peer state)
├── identity.json        # Encrypted identity (nickname, keys, DID)
├── daemon.pid           # PID of running daemon
├── daemon.log           # Daemon log output
└── daemon-args.json     # Saved start arguments (for TUI auto-start)
```

### Backup strategy

The sled database (`~/.ghostwire/db/`) is the critical data. To back up:

```bash
# Stop the daemon first to avoid corruption
ghostwire stop

# Backup
tar czf ghostwire-backup-$(date +%Y%m%d).tar.gz -C ~ .ghostwire/

# Restart
ghostwire start
```

Sled is append-only and crash-safe. The identity file is encrypted at rest (Argon2 + AES-GCM). Keep backups encrypted.

---

## 7. Network Configuration

### Ports

| Port | Protocol | Purpose | Default |
|---|---|---|---|
| 8080 | TCP | Web API (web server mode) | `--port` flag |
| 9097 | TCP | Daemon API (daemon mode) | `DEFAULT_DAEMON_PORT` |
| 4001 | TCP+UDP | libp2p mesh transport | `--mesh-port` flag |

The daemon API binds to `127.0.0.0` (localhost only). The mesh transport binds to `0.0.0.0` (all interfaces).

### API host binding

```bash
# Localhost only (default, safe for single machine)
ghostwire start --port 9097

# LAN-accessible (for TUI on another machine)
ghostwire start --port 9097 --api-host 0.0.0.0
```

### Mesh transport

The mesh host defaults to `0.0.0.0` and listens on the port specified by `--mesh-port`. If `--mesh-port 0`, the OS picks a random port.

```bash
# Fixed mesh port
ghostwire start --mesh-port 4001

# Random port (ephemeral)
ghostwire start --mesh-port 0
```

---

## 8. Security Hardening in Production

### Firewall rules (nftables/iptables)

```bash
# Allow mesh port only from trusted subnet
iptables -A INPUT -p tcp --dport 4001 -s 10.0.0.0/8 -j ACCEPT
iptables -A INPUT -p udp --dport 4001 -s 10.0.0.0/8 -j ACCEPT
iptables -A INPUT -p tcp --dport 4001 -j DROP

# Allow daemon API only from localhost (already bound to 127.0.0.0)
# No additional rules needed if --api-host is left at default
```

### TLS

The web API does not terminate TLS by default. For production:

- **Option 1**: Run behind a reverse proxy (nginx, Caddy) that handles TLS.
- **Option 2**: Use `--sim-addr` or network-level encryption (WireGuard, Tailscale).

The mesh transport itself uses libp2p Noise encryption (end-to-end, authenticated). Traffic between peers is always encrypted regardless of the API layer.

### Identity and key management

- Identity is encrypted at rest with Argon2 + AES-GCM.
- Ed25519 keypairs for peer authentication.
- X25519 for key exchange.
- PQCrypto ML-KEM (post-quantum) is available for key encapsulation.
- Release verification: set `GHOSTWIRE_RELEASE_VERIFYING_KEY_HEX` at build time.

### Additional hardening

- Run as a non-root user (the Dockerfile does this).
- Enable `panic = "abort"` (already set in release profile).
- Use `GHOSTWIRE_HOME` to isolate data from other services.
- Monitor `cargo audit` output for dependency CVEs.

---

## 9. Systemd Service File

```ini
[Unit]
Description=GhostWire Secure Mesh Network Daemon
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=ghostwire
Group=ghostwire
Environment=GHOSTWIRE_HOME=/home/ghostwire/.ghostwire
Environment=GHOSTWIRE_LOG=ghostwire=info
ExecStart=/usr/local/bin/ghostwire start --port 9097 --mesh-port 4001 --mesh-host 0.0.0.0
ExecStop=/usr/local/bin/ghostwire stop
Restart=on-failure
RestartSec=5
TimeoutStartSec=30
StandardOutput=journal
StandardError=journal
SyslogIdentifier=ghostwire

# Security hardening
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=read-only
ReadWritePaths=/home/ghostwire/.ghostwire
PrivateTmp=true

[Install]
WantedBy=multi-user.target
```

### Install and manage

```bash
# Create service user
sudo useradd -r -s /bin/false ghostwire
sudo mkdir -p /home/ghostwire/.ghostwire
sudo chown ghostwire:ghostwire /home/ghostwire/.ghostwire

# Copy binary
sudo cp target/release/ghostwire /usr/local/bin/

# Install service
sudo cp ghostwire.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable ghostwire
sudo systemctl start ghostwire

# Check status
sudo systemctl status ghostwire
sudo journalctl -u ghostwire -f
```
