# Getting Started with GhostWire

---

## 1. Welcome & Audience

GhostWire is for community organizers, first responders, privacy advocates, rural
users, and developers. This chapter walks through your first steps regardless of
background.

> **Before you start — what GhostWire is today.** GhostWire is a **protocol with a
> reference implementation**, not a packaged consumer product. There is no app
> store download and no hosted service. You run the node yourself, from source.
> Three transports are working (stealth TCP, WebRTC, Reticulum); others are
> designed but not implemented. Per-capability status with evidence is in
> [docs/BIBLE_STATUS_SPINE.md](../BIBLE_STATUS_SPINE.md) — read it before making
> deployment decisions.

---

## 2. What You Can Do with GhostWire

- **Send messages** to peers on the same mesh, including when the internet is
  unreachable — via delay-tolerant store-and-forward.
- **Join a local mesh** at an event, in a neighbourhood, or across a campus.
- **Encrypt traffic** with a hybrid classical + post-quantum handshake, and route
  it over an onion-routed path.
- **Build on the protocol.** This is the intended use: implement the wire format
  in another language or application and interoperate.

**Not available today:** a mobile app, a public demo instance, a hosted relay
service, or bridges to Briar/Meshtastic/Matrix. Those are on the roadmap, not in
the build.

---

## 3. Prerequisites

| Requirement | Why | Notes |
|---|---|---|
| Rust (stable toolchain) | Builds the node | <https://rustup.rs> |
| Node.js + npm | Builds the web UI | <https://nodejs.org> |
| ~2 GB disk | Build artifacts | Release build of the node is large |
| LoRa / RISC-V hardware | Optional, long-range | **Transport not implemented** — see ch04 |

No GPU is required. The AI routing layers are inactive in the current build
(ch16), so no model files or Ollama instance are needed.

---

## 4. Build and Run

```sh
# 1. Clone
git clone https://github.com/phantomojo/GhostWire-secure-mesh-communication.git
cd GhostWire-secure-mesh-communication/ghostwire

# 2. Create an identity (required before first run)
cargo run --release -- init -u yourname

# 3. Start the node
cargo run --release -- start --host 127.0.0.1 --port 8080 --web
```

The node serves its HTTP API on `127.0.0.1:8080` by default.

### Web UI

The web UI is a separate build. It is **not required** — the node's HTTP API and
the terminal UI both work on their own.

```sh
cd ../webui
npm install
npm run dev
```

> **Known gap:** the `webui` directory in the current repository does not contain
> a `package.json`, so the commands above will not run as written. The served web
> assets are pre-built under `webui/dist`. Until the source front-end is restored,
> use the TUI or the HTTP API. Tracked as a documentation defect — not a
> workaround you need to solve.

### Terminal UI

```sh
cargo run --release -- tui
```

---

## 5. CLI Reference

The binary is `ghostwire`. The subcommands below are taken from
`src/cli/mod.rs`; this is the real interface, not an illustration.

| Command | Purpose |
|---|---|
| `ghostwire init -u <name>` | Create an identity (do this first) |
| `ghostwire start [--host H] [--port P] [--web] [--mode M]` | Run the node. `--mode` is `hybrid` (default), `libp2p-only`, or `reticulum-only` |
| `ghostwire send -r <peer> -m <text>` | Send a message to a peer |
| `ghostwire peers` | List connected peers |
| `ghostwire stats` | Network statistics |
| `ghostwire security` | Security information |
| `ghostwire genkey` | Generate a key pair |
| `ghostwire encrypt -e <text>` / `decrypt -d <ciphertext>` | Crypto operations |
| `ghostwire connect -u <username> [--anonymous]` | Connect to a peer; `--anonymous` uses SURB-based introduction |
| `ghostwire manage <start\|stop\|restart\|status\|logs\|backup\|restore>` | Process and data management |
| `ghostwire monitor [-f]` | Network and security monitor |
| `ghostwire tui` | Interactive terminal interface |
| `ghostwire setup` | Set up dependencies and environment |
| `ghostwire deploy -t <target> -p <platform>` | Deploy payloads |

Examples:

```sh
ghostwire start --web --port 8080
ghostwire connect -u alice
ghostwire connect -u alice --anonymous     # SURB-based, no direct peer ID needed
ghostwire send -r alice -m "Hello, Bob!"
ghostwire manage logs -f
```

---

## 6. Device Roles

Your hardware determines what role you can play. Peers relay for each other, so
a device class that never relays reduces the mesh's reach.

| Device | Memory | Max Peers | Storage | Typical Role |
|---|---|---|---|---|
| Phone | ≤6 GB | 20 | 1 GB | Leaf — messaging only |
| Tablet | ≤8 GB | 30 | 2 GB | Leaf or light relay |
| Raspberry Pi | ≤4 GB | 15 | 500 MB | Relay or sensor |
| Laptop | ≤16 GB | 50 | 5 GB | Good relay |
| Server | >16 GB | 100+ | 50 GB+ | Relay and replicator |

**Energy-aware behaviour.** The selector weights transports by energy cost
(BLE 0.30, LoRa 0.35, Reticulum 0.25, WiFi 0.05, TCP/QUIC 0) and a
`DeviceTier` (High / Medium / Low) applies cover-traffic intervals of
approximately 5–15 seconds depending on tier.

> **Two corrections to earlier versions of this chapter.** Battery level does
> **not** currently drive privacy behaviour: the `L0`–`L3` privacy tiers described
> in earlier revisions are documentation-only — `PrivacyParams` and
> `PrivacyLevel` have no implementation in `src/`. The device also **cannot read
> battery state** today. QR pairing is likewise not implemented. What does exist
> is the `DeviceTier` enum and the energy weights above. See the status spine §5.

---

## 7. Joining and Creating a Mesh

- **Discovery:** peers find each other over the active transports (libp2p
  identify / gossipsub, plus Reticulum when enabled). Discovery over WiFi Direct
  and LoRa is designed but **not implemented**.
- **Joining a known peer:** `ghostwire connect -u <username>`, or
  `--anonymous` for SURB-based introduction.
- **Group messaging:** the protocol defines a group/social layer; the UI for
  creating and joining groups is not complete in the current build.

---

## 8. Troubleshooting

| Problem | Likely cause | Action |
|---|---|---|
| No peers discovered | No working transport enabled | `ghostwire stats` to see active transports; confirm WebRTC or Reticulum |
| `init` not run | Missing identity | `ghostwire init -u <name>` |
| Web UI does not load | `webui` has no `package.json` in this tree | Use `ghostwire tui` or the HTTP API on `:8080` |
| LoRa or WiFi Direct unavailable | Not implemented | Use stealth TCP, WebRTC, or Reticulum |
| Update or integrity check disabled | Build-time keys unset | Expected in local builds; set `GHOSTWIRE_RELEASE_VERIFYING_KEY_HEX` and `GHOSTWIRE_BINARY_HASH` for release builds |
| Test failures mention `GHOSTWIRE_HOME` | Tests share on-disk state | Serialise; see status spine §10 |

---

## 9. Deployment Scenarios

> These are **scenarios the design targets**, not deployments that have been
> exercised. Treat them as illustrations of intent.

**Community event.** Organisers run nodes on phones and laptops, form a mesh, and
share peer identifiers. Viable today over WebRTC or a local network.

**Blackout response.** Responders run nodes with no internet; delay-tolerant
store-and-forward carries messages until a path reappears. This is the
best-supported scenario in the current build — the DTN layer is tested
(9 tests in `dtn/epidemic.rs`).

**Rural deployment.** Homes relay for each other over a mix of long-range and
local links. **Blocked today**: the long-range transports (LoRa, WiFi Direct) are
not implemented, so this scenario currently depends on Reticulum infrastructure
or conventional WiFi/cellular.

---

## 10. Best Practices

- Run `ghostwire init` before anything else.
- Use the latest build — the revocation and anti-rollback work is recent.
- Treat mesh access credentials as secrets; there is no default shared password.
- Read the status spine before relying on a capability. It is the honest source.
- Back up with `ghostwire manage backup` before major changes.

---

## 11. Further Reading

- [Capability Status Spine](../BIBLE_STATUS_SPINE.md) — what is actually built
- [Security & Privacy](05_security.md)
- [Transports & Protocols](04_transports.md)
- [Developer Guide](12_developer_guide.md)
- [Mesh networking overview](https://en.wikipedia.org/wiki/Mesh_networking)
- [GhostWire repository](https://github.com/phantomojo/GhostWire-secure-mesh-communication)

---

## End of Chapter
