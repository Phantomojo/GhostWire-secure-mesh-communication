# Configuration

GhostWire loads settings from config files, environment variables, and CLI arguments. Later sources override earlier ones.

## Precedence

```
CLI arguments  >  config file  >  environment variables  >  built-in defaults
```

For settings controlled by both CLI and config (host, port, web toggle), the CLI wins. For settings only in config (security limits, encryption params), the config file wins over env vars.

## File Locations

### Config files (checked in order)

GhostWire tries these paths and stops at the first one that exists:

1. `ghostwire.toml` (current working directory)
2. `config/ghostwire.toml` (current working directory)
3. `~/.config/ghostwire/config.toml`

Pass `--config /path/to/file.toml` to use a specific file, skipping the search.

### Data directory

All persistent data lives under `$GHOSTWIRE_HOME` or `~/.ghostwire/`:

| File | Purpose |
|---|---|
| `identity.json` | Encrypted identity (nickname, DID, keys) |
| `.initial-password` | Temporary file with initial identity password (deleted after setup) |
| `daemon.pid` | PID of the running daemon |
| `daemon.log` | Daemon stdout/stderr log |
| `daemon-args.json` | Saved daemon startup arguments (bootstrap nodes, mesh port) |
| `vault/` | Persistent encrypted storage |

Set `GHOSTWIRE_HOME` to move all of this elsewhere (useful for testing or running multiple instances).

## default.toml Reference

The full config file format. Every field shown with its default value.

```toml
[server]
host = "127.0.0.1"
port = 8080
web_enabled = true
cli_enabled = true
auto_update = true

[security]
max_connections_per_ip = 10
max_messages_per_minute = 100
blacklist_enabled = true
threat_detection_enabled = true
allowed_ips = []
blocked_ips = []
key_rotation_interval_days = 7

[network]
mesh_enabled = true
reticulum_enabled = false
briar_enabled = false
stealth_tcp_enabled = false
max_peers = 50
connection_timeout_secs = 30
obfs4_enabled = true
# stealth_handshake_secret and obfs4_secret_key are excluded from file output
# Set via GHOSTWIRE_STEALTH_SECRET and GHOSTWIRE_OBFS4_SECRET env vars instead

[encryption]
algorithm = "AES-256-GCM"
key_size = 32
session_timeout_hours = 1

[features]
# Bridge protocols
matrix_bridge = false
meshtastic_bridge = false

# UI
web_ui = true
cli = true
api = true

# AI/ML routing
gemma_2b = true        # Gemma 4 E2B (2.3B params, edge-optimized)
gemma_4b = false       # Gemma 4 E4B (4B params)
gemma_26b = false      # Gemma 4 MoE (26B total, 4B active)
gemma_31b = false      # Gemma 4 Dense (31B)
ai_routing = true
ai_threat_detection = true

# Gemma 4 configuration
gemma_version = "4"              # "3", "3n", or "4"
inference_backend = "ollama"     # "ollama", "litert", or "local"
gemma_model_variant = ""         # "e2b", "e4b", "26b", "31b" (auto-detect if empty)
ollama_url = "http://localhost:11434"
litert_model_path = "models/gemma4_e2b_litert.tflite"
litert_num_threads = 4
litert_use_gpu = false

# FunctionGemma (tool calling)
function_gemma_enabled = true
function_gemma_model = "functiongemma:270m"
function_gemma_url = "http://localhost:11434"

# Edge device detection
auto_detect_device = true
device_class = ""          # auto-detect: "phone", "tablet", "raspberry_pi", "laptop", "server"
available_ram_mb = 0        # auto-detect
```

### Save a config file

```bash
# Generate and save default config
ghostwire config --save ghostwire.toml
```

Or write the TOML manually and pass it at startup:

```bash
ghostwire start --config ghostwire.toml
```

## Identity Configuration

Identity lives at `~/.ghostwire/identity.json`. It's encrypted with Argon2-derived keys and password-protected.

Run the interactive setup to create or update your identity:

```bash
ghostwire setup
```

This walks you through nickname, bio, and password. Your DID (decentralised identifier) is generated automatically from your ed25519 keypair.

The identity file is locked by default. Unlock it via the API before making changes:

```bash
curl -X POST http://127.0.0.1:9097/api/v2/profile/unlock \
  -d '{"password": "your-password"}'
```

## CLI Arguments

### Global flags

| Flag | Short | Default | Description |
|---|---|---|---|
| `--verbose` | `-v` | off | Debug-level logging (also enables `libp2p_gossipsub=debug`) |
| `--quiet` | `-q` | off | Warn-level logging only |
| `--config` | | none | Path to config file |
| `--port` | | `9097` | Daemon API port |
| `--api-host` | | `127.0.0.1` | API listen address |
| `--mesh-host` | | `0.0.0.0` | Mesh listen address |
| `--mesh-port` | | `0` (random) | Mesh TCP port |
| `--bootstrap` | | none | Bootstrap multiaddrs (repeatable) |
| `--tailscale-bootstrap` | | none | Tailscale nodes as `<ip>/<peer-id>` (repeatable) |
| `--mesh-size` | | `small` | Network size hint: `small` (2-5), `medium` (6-20), `large` (20+) |
| `--web` | | off | Start web server (interactive mode) |
| `--daemon` | | off | Run as background daemon (used internally) |
| `--profile` | | none | Identity profile name |
| `--sim-addr` | | none | Simulation address |

### Subcommands

```bash
# Daemon lifecycle
ghostwire start [--mesh-port 4001] [--bootstrap /ip4/1.2.3.4/tcp/4001/p2p/...]
ghostwire stop
ghostwire status

# TUI (auto-starts daemon if needed)
ghostwire tui

# Identity setup
ghostwire setup

# Quick peer list
ghostwire peers

# Send a message
ghostwire send <recipient-peer-id> "hello world"

# Start web server directly (not via daemon)
ghostwire web --port 8080 --bootstrap /ip4/1.2.3.4/tcp/4001/p2p/...

# Run with arbitrary CLI args
ghostwire cli [args...]
```

### Web subcommand defaults

The `web` subcommand uses slightly different defaults than the daemon:

| Flag | `web` subcommand | Daemon/default |
|---|---|---|
| `--api-host` | `127.0.0.1` | `127.0.0.1` |
| `--mesh-host` | `0.0.0.0` | `0.0.0.0` |
| `--port` | `8080` | `9097` |

## Environment Variables

### Core

| Variable | Default | Description |
|---|---|---|
| `GHOSTWIRE_HOME` | `~/.ghostwire` | Data directory for identity, daemon files, vault |
| `GHOSTWIRE_HOST` | `127.0.0.1` | API listen host |
| `GHOSTWIRE_PORT` | `9097` | API listen port |
| `GHOSTWIRE_LOG` | `ghostwire=info` | Tracing filter (e.g. `ghostwire=debug,libp2p=warn`) |
| `GHOSTWIRE_PROFILE` | none | Identity profile name to load |
| `GHOSTWIRE_AUTO_UPDATE` | `true` | Enable automatic update checks |
| `GHOSTWIRE_RELEASE_VERIFYING_KEY` | none | Ed25519 pubkey hex for update verification |

### Security

| Variable | Default | Description |
|---|---|---|
| `GHOSTWIRE_MAX_CONNECTIONS_PER_IP` | `10` | Max simultaneous connections from one IP |
| `GHOSTWIRE_STEALTH_SECRET` | none | Stealth handshake secret key |
| `GHOSTWIRE_OBFS4_ENABLED` | `true` | **NO EFFECT.** Obfs4 is not implemented; this flag is read only by a startup warning. |
| `GHOSTWIRE_OBFS4_SECRET` | none | **NO EFFECT** today. Unused until obfs4 exists. |

### Bridges

| Variable | Default | Description |
|---|---|---|
| `GHOSTWIRE_MATRIX_BRIDGE` | off | Enable Matrix bridge (presence of any value enables it) |
| `GHOSTWIRE_MESHTASTIC_BRIDGE` | off | Enable Meshtastic bridge |
| `MATRIX_HOMESERVER` | `https://matrix.org` | Matrix homeserver URL |
| `MATRIX_USER` | `@ghostwire:matrix.org` | Matrix user ID |
| `MATRIX_ACCESS_TOKEN` | empty | Matrix access token |
| `MESHTASTIC_DEVICE_PATH` | `/dev/ttyUSB0` | Serial port for Meshtastic device |
| `MESHTASTIC_CHANNEL` | `main` | Meshtastic channel name |

### Networking

| Variable | Default | Description |
|---|---|---|
| `TAILSCALE` | off | Set to `1` to scan `100.64.0.0/10` for GhostWire peers on Tailscale |
| `GHOSTWIRE_MESH_SIZE` | `small` | Network size hint (set automatically from `--mesh-size`) |

## Feature Flag Configuration

Feature flags in the config file control runtime behavior. These are separate from Cargo feature flags (which control what gets compiled).

### Bridge toggles

```toml
[features]
matrix_bridge = true      # Requires "matrix-bridge" Cargo feature
meshtastic_bridge = true   # Requires "meshtastic" Cargo feature
```

### AI/ML model selection

GhostWire auto-detects the best model for your device. Override manually if needed:

```toml
[features]
gemma_model_variant = "e2b"      # Force E2B for low-RAM devices
inference_backend = "litert"      # Use LiteRT instead of Ollama
auto_detect_device = false       # Disable auto-detection
device_class = "laptop"          # Manual device class
```

Available device classes: `phone`, `tablet`, `raspberry_pi`, `laptop`, `server`.

Available model variants: `e2b` (2.3B), `e4b` (4B), `26b` (26B MoE), `31b` (31B Dense).

### Stealth and obfuscation

Stealth secrets and obfs4 keys are intentionally excluded from TOML file serialization (marked `#[serde(skip)]`). Set them via environment variables:

```bash
export GHOSTWIRE_STEALTH_SECRET="your-handshake-secret"
export GHOSTWIRE_OBFS4_SECRET="hex-encoded-obfs4-key"
```

> **Currently inert.** Obfs4 is not implemented, so this variable has no effect on
> traffic. It is retained so the config surface and the startup warning keep
> working. Do not rely on it for obfuscation.

## Config Precedence in Practice

Example: setting the API port.

```bash
# 1. Default: 9097
ghostwire start

# 2. Config file overrides default
# ghostwire.toml: server.port = 8080
ghostwire start --config ghostwire.toml

# 3. Env var overrides config file
GHOSTWIRE_PORT=3000 ghostwire start --config ghostwire.toml

# 4. CLI flag overrides everything
ghostwire start --port 4000
```

The actual port in scenario 4 is **4000**. The `--port` CLI arg takes precedence over `GHOSTWIRE_PORT` which takes precedence over `server.port` in the config file which takes precedence over the hardcoded default of `9097`.
