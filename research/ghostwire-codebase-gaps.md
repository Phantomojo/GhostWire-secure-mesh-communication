# GhostWire Codebase Gap Analysis

> **Source**: Background agent bg_f57540ce (explore agent)
> **Date**: 2026-07-27
> **Status**: Raw agent output — unmodified

---

## GHOSTWIRE IMPLEMENTATION GAP ANALYSIS

### Files Analyzed

- ghostwire/src/core/dtn/epidemic.rs (609 lines) — In-core DTN store: Spray-and-Wait + PRoPHET + bloom sync. Fully wired into mesh.rs and Core.
- ghostwire/src/core/dtn/bundle.rs (135 lines) — DTN bundle struct: CID, copies_remaining, TTL, hop_count. Spray-and-Wait binary split.
- ghostwire-libs/crates/ghostwire-dtn/src/lib.rs (777 lines) — Standalone ghostwire-dtn crate: duplicates epidemic.rs + bundle.rs + adds X25519 bundle encryption. NOT used by main binary.
- ghostwire/src/core/routing/sphinx.rs (809 lines) — In-tree Sphinx onion routing: create/process packet, SURBs, alpha re-randomization, MAC chain. FULLY WIRED into mesh.rs.
- ghostwire/src/core/routing/private_routing.rs (353 lines) — Veilid-style onion routing: safety+private hop segments, wrap_onion/peel_onion_layer, RouteCache. NOT wired.
- ghostwire/src/core/routing/skademlia.rs (509 lines) — S/Kademlia: NodeId PoW, IP diversity enforcement, disjoint path lookups, hardened gossipsub config. NOT wired.
- ghostwire/src/core/routing/mixing_delay.rs (308 lines) — Exponential mixing delays, MixingQueue, apply_mixing_delay(). PARTIALLY wired via mesh.rs mixing_tx.
- ghostwire/src/core/transport.rs (241 lines) — TransportKind enum + TransportRegistry (HashMap<String, TransportKind>). send_message dispatches by variant. No trait.
- ghostwire/src/core/transport_manager.rs (54 lines) — AutoTransportSwitcher: reads DisasterMetrics -> evaluates DisasterLevel -> picks TransportType.
- ghostwire/src/core/double_ratchet.rs (567 lines) — Full X3DH+Double Ratchet: RatchetState, RatchetSessionManager, encrypt/decrypt with XChaCha20Poly1305. Wired into Encryption.
- ghostwire/src/core/wire_protocol.rs (424 lines) — Binary wire format: Frame/FrameKind, WireMessage/WireAnnounce/WireHandshake, ReplayWindow, FrameParser. Frame size limit 64KB.
- ghostwire/src/core/mesh.rs (2,444 lines) — Main libp2p mesh: gossipsub + Kademlia + Sphinx routing + DTN integration + Stealth TCP. Central orchestrator.
- ghostwire/src/core/mod.rs (2,279 lines) — Core struct, Core::new() initialization, all subsystem wiring, handle_dtn_encounter(), AI components.
- ghostwire/src/core/disaster.rs (446 lines) — DisasterMetrics, DisasterLevel, DisasterManager (hysteresis), TransportType enum, DisasterPolicy.
- ghostwire-libs/crates/sphinx-rs/src/lib.rs (970 lines) — Standalone published sphinx-rs crate: configurable domains, proper Sphinx backward construction. NOT used by main binary.
- ghostwire-libs/crates/sphinx-rs/src/sphinx.rs (677 lines) — Legacy copy of in-tree sphinx.rs. OBSOLETE.
- ghostwire/src/core_wiring.rs (367 lines) — Spec document (comments only) describing what changes to make. NOT executable code.

---

### 1. DTN Code (epidemic.rs + bundle.rs + ghostwire-dtn crate)

**What exists:**
- **In-tree src/core/dtn/epidemic.rs** (609 lines): Fully working PRoPHET + Spray-and-Wait implementation with DtnStore, DeliveryTable, SyncBloom, DtnSyncHandshake. Implements on_encounter() and on_encounter_with_gnn() for bundle forwarding decisions. Uses time-decayed delivery predictability (gamma=0.98, P_init=0.75, beta=0.25), bloom filter sync, and LRU-by-delivery-probability eviction at 50MB.
- **In-tree src/core/dtn/bundle.rs** (135 lines): DtnBundle struct with spray-and-wait binary copy split (split_copies(), make_relay_copy()), TTL, CID (BLAKE3 hash).
- **Wired into Core** via Core.dtn_store: Arc<DtnStore> (mod.rs line 667). Initialized in Core::new() (mod.rs line 942-952).
- **Wired into mesh.rs** via set_dtn_store() (mesh.rs:2257-2258). Called after Core creation (mod.rs:1906). mesh.rs calls dtn_store.store(), dtn_store.purge_expired(), dtn_store.drain_local_deliveries(), dtn_store.on_encounter() (lines 784-786, 1173-1177, 1363-1393, 1925-1963).
- **AI enhancement**: Core::handle_dtn_encounter() (mod.rs:1031-1208) supports GNN-enhanced bundle forwarding with fallback to pure PRoPHET.
- **Standalone ghostwire-dtn crate** (777 lines) at ghostwire-libs/crates/ghostwire-dtn/src/lib.rs: Duplicates the in-core DTN logic but adds X25519 bundle encryption via EncryptedBundlePayload. NOT used by the main binary (Cargo.toml has it commented out for local dev: line 168-169).

**GAP:**
- **No DTN over-the-wire protocol**: DtnSyncHandshake struct exists, but mesh.rs only builds it — there is no actual DTN sync exchange happening over the network. The on_encounter() logic runs, but there is no background encounter loop or BLE encounter trigger. The spec says "Bluetooth encounter -> sync handshake -> bundle transfer" but only the data structures exist.
- **Duplicate code**: In-tree epidemic.rs vs. standalone ghostwire-dtn crate are ~80% identical. The standalone crate has bundle encryption, the in-tree one does not. One should be the canonical version.
- **No store persistence**: Bundles live in memory (HashMap), no sled/disk backing. A restart loses all bundles.
- **ghostwire-dtn crate not consumed**: Listed as ghostwire-dtn = "0.1" in Cargo.toml but commented out for local path (line 169). The binary uses the in-tree version instead.

### 2. Routing Code (routing/ directory)

**What exists:**
- **routing/mod.rs** (4 lines): Exports four modules.
- **routing/sphinx.rs** (809 lines): Full Sphinx packet implementation with real X25519 scalar multiplication for alpha re-randomization, SURBs with AEAD reply payload encryption, MAC chain verification, constant-time comparisons, small-subgroup attack defense. **FULLY WIRED** into mesh.rs (see below).
- **routing/private_routing.rs** (353 lines): Veilid-style onion routing with CompiledRoute, RouteHop, wrap_onion(), peel_onion_layer(), RouteCache. Complete but **NOT WIRED** — zero references outside this file and its mod declaration.
- **routing/skademlia.rs** (509 lines): S/Kademlia defenses — NodeId PoW (12-bit difficulty), IP diversity (/24 subnet cap at 3), disjoint path lookups (3 paths), hardened gossipsub config with peer scoring. **NOT WIRED** — hardened_gossipsub_config(), validate_peer_node_id(), disjoint_lookup() are never called. The integration comments in mesh.rs and core_wiring.rs show exactly where to add them (Change 7 and Change 8) but it was never done.
- **routing/mixing_delay.rs** (308 lines): Exponential mixing delay (default lambda=200ms), MixingQueue with background drain loop, high-anonymity/low-latency presets. **PARTIALLY WIRED**: QueuedSphinxPacket is referenced in mesh.rs (line 726, 740, 814) but apply_mixing_delay() is never called. The mixing queue exists but the actual delay is not applied before forwarding.

**GAP:**
- Sphinx routing is the only routing system actually wired. Private routing (onion), S/Kademlia defenses, and mixing delays exist as complete implementations but are disconnected from the message path.
- The spec describes a 4-layer AI routing system (LightGBM -> Gemma 4 -> GNN -> PRoPHET). The ONNX router and GNN router are loaded in Core::new() but routing_classifier.rs is never called in the message dispatch path. The AGENTS.md file explicitly flags: "ML routing classifier not wired into actual packet routing (HIGH)."
- mesh.rs select_sphinx_route() (line 831) uses a hardcoded Kademlia closest-peers heuristic — no AI routing integration.

### 3. sphinx-rs Integration

**What exists:**
- **sphinx-rs = "0.1"** (from crates.io) is listed in Cargo.toml line 172. The local path dep is commented out (line 167).
- **Standalone crate** at ghostwire-libs/crates/sphinx-rs/ has two files:
  - lib.rs (970 lines): Published crate. Configurable SphinxConfig with domain separation for all 5 key domains (rho, mu, blind, shared, kdf). Uses proper Sphinx backward header construction with fixed-size routing blocks (128 bytes each). Supports create/process with configurable RNG. Has 3-hop test IGNORED due to known truncation bug (header construction loses data with >2 hops — test says "Fix requires redesigning the header construction to avoid truncation").
  - sphinx.rs (677 lines): Legacy copy of the in-tree version. OBSOLETE.
- **In-tree routing/sphinx.rs** (the one actually used): Different implementation from sphinx-rs/lib.rs. Uses HMAC-based HopKeys derivation via genesis_constants ceremony-derived domains. Does NOT use the sphinx-rs crate at all.

**GAP:**
- **sphinx-rs is installed but NOT used**. The binary uses the in-tree routing/sphinx.rs instead. None of mesh.rs or routing code imports from the sphinx_rs crate.
- **Two competing Sphinx implementations** (in-tree routing/sphinx.rs + standalone sphinx-rs crate/lib.rs) with slightly different header construction algorithms. This is a maintenance risk.
- The standalone sphinx-rs crate has a known 3-hop failure (test marked #[ignore]). The in-tree version only tests 2-hop — 3-hop is untested.
- The in-tree version uses SPHINX_MAC_DOMAIN as a public constant for compute_mac() (single-key MAC), making gamma forgeable by any observer. Module docstring recognizes this as a known issue.

### 4. Transport Trait and Transport Manager

**What exists:**
- **transport.rs** (241 lines): TransportKind enum (not a trait) with variants: Mesh, Dummy, Quic, StealthTCP, Bluetooth, Simulated. Each variant wraps an Arc to the concrete transport type. send_message() does match-based dispatch. TransportRegistry is a HashMap<String, TransportKind>. Registered transports can be looked up by name.
- **transport_manager.rs** (54 lines): AutoTransportSwitcher backed by DisasterManager. Evaluates network metrics -> determines DisasterLevel -> picks TransportType from DisasterPolicy. The policy has priority lists per disaster level (Normal: QUIC > StealthTCP > Mesh; Congested: Mesh > WiFi; Isolation: WiFi > Bluetooth; Blackout: LoRa).
- **disaster.rs** (446 lines): Full DisasterManager with anti-thrashing hysteresis (requires N stable samples before recovery). TransportType enum (Mesh, Quic, StealthTcp, WiFi, Bluetooth, LoRa) — note this is a DIFFERENT enum from TransportKind in transport.rs.

**GAP:**
- **No Transport trait**: The code has NO trait Transport { fn send() ... }. TransportKind is an enum with hardcoded variants. Adding a new transport requires editing the enum and all match arms. The module docstring says "Transport trait removed, using TransportKind enum instead" (mod.rs:95).
- **TransportKind vs. TransportType mismatch**: transport.rs defines TransportKind with 6 variants. disaster.rs defines TransportType with 6 different variants. AutoTransportSwitcher returns TransportType, but Core.active_transport stores TransportKind. There is NO conversion function between them — the switcher's recommendation cannot be applied to the active transport register.
- **AutoTransportSwitcher is never invoked**: Core.switcher is initialized in Core::new() (mod.rs:802-805) but update_and_evaluate() is never called. No periodic metrics collection feeds the disaster manager. The entire transport switching system is architecture without execution.
- **sim_transport.rs exists** but SimTransport is only registered when sim_addr CLI arg is passed (for simulation mode). Not usable in production.

### 5. Multi-Transport Coordination Logic

**What exists:**
- The TransportRegistry stores all registered transports by name.
- Core.active_transport stores which one is currently in use as Option<TransportKind>.
- AutoTransportSwitcher is designed to coordinate switching between transports based on network conditions.
- AdapterRegistry (mod.rs:174-193) handles protocol adapters (Matrix, Meshtastic, IRC) — these are bridge protocols, not mesh transports.

**GAP:**
- **No fallback logic**: When active_transport fails, there is no code that tries the next transport in priority order. The failure modes in TransportKind::send_message() just propagate errors upward.
- **No concurrent transport usage**: All transports are registered, but only the active one is used. There is no "send on all available transports" or "broadcast across transports" mode.
- **Transport selection is not integrated with routing**: The Sphinx/mesh routing layer does not consult the TransportRegistry or AutoTransportSwitcher. Routes are selected purely from Kademlia topology, not from available transport capabilities.
- **Transport capability advertisement**: WireAnnounce has a transport_mask: u8 field (wire_protocol.rs:154) for advertising which transports a node supports, but this field is never populated or checked.

### 6. Double Ratchet (double_ratchet.rs)

**What exists:**
- **double_ratchet.rs** (567 lines): Full X3DH-based Double Ratchet implementation.
  - RatchetState: Tracks root_key, sending/receiving chain keys, DH keypairs, skipped message keys (MAX_SKIP=200), message numbers, last_ratchet_at timestamp.
  - kdf_chain(): HMAC-SHA256 based chain key derivation with domain separation (0x01 for next chain key, 0x02 for message key).
  - kdf_rk(): Root key derivation from DH output.
  - encrypt()/decrypt(): XChaCha20-Poly1305 AEAD encryption with 24-byte nonces.
  - dh_ratchet_step(): Full DH ratchet, generates new keypair per step.
  - RatchetSessionManager: Manages sessions per peer_id with serialization/export/import.
  - Test coverage: bidirectional messaging, forward secrecy, duplicate rejection, state export/import.
- **Wired into Encryption** (encryption.rs:123, 216, 248): Encryption.ratchet_sessions: Arc<RatchetSessionManager> is used for per-message ratchet encryption.
- **Re-exported via crypto/mod.rs** (line 26): pub use crate::core::double_ratchet.

**GAP:**
- **Not wired into any transport or message path**: While encryption.rs has the RatchetSessionManager, there is no code path where a message is encrypted with the double ratchet before being handed to TransportKind::send_message(). The double ratchet produces RatchetMessage objects, but the transport layer and wire protocol use Message structs with XChaCha20-Poly1305 AEAD from encryption.rs.
- **No integration with Sphinx**: The double ratchet provides per-message keys, but Sphinx packets carry a single AEAD-encrypted payload. The ratchet's chain key update and DH ratchet are not integrated with the Sphinx route construction.
- **No integration with wire_protocol**: There is no FrameKind for ratchet messages. The WireMessage struct does not carry RatchetHeader or RatchetMessage fields.
- **Separate key hierarchy**: The double ratchet uses its own DH-based key agreement (X25519) independent of the Sphinx hop keys and the encryption module's key hierarchy. Key material is not unified.

### 7. Wire Protocol (wire_protocol.rs)

**What exists:**
- **wire_protocol.rs** (424 lines): Compact binary frame format with:
  - FRAME_MAGIC (4 bytes, ceremony-derived) + kind (u16) + payload_length (u32) + payload.
  - 12 frame kinds: Message, Announce, DhtPut, DhtGet, DhtValue, RouteSetup, RoutedPayload, MailboxDeposit, MailboxDrain, CoverTraffic, Ping, Pong, Handshake.
  - MAX_FRAME_SIZE = 64KB (OOM attack protection).
  - ReplayWindow: 64-packet sliding window bitmask for seq replay protection.
  - FrameParser: Streaming parser that handles partial reads and drops oversized frames gracefully.
  - WireMessage, WireAnnounce, WireHandshake, WirePing, WirePong types.
  - cover_traffic(): Generates 256-byte random payload frames.
  - Bincode serialization (compact binary, not JSON).
- Test coverage: frame roundtrip, oversized frame rejection, replay window, partial reads, handshake version validation, binary vs. JSON size comparison (proves binary is smaller).

**GAP:**
- **Wire protocol is defined but not used for transport**: Frame::to_bytes() and FrameParser exist, but they are never called in the message send/receive path. mesh.rs uses bincode::serialize() directly on Message structs for gossipsub publishing (mesh.rs:1115-1119). The wire_protocol module is imported in core_wiring.rs Change 8 as a planned change but never applied.
- **No Sphinx integration**: There is no FrameKind::SphinxPacket. Sphinx packets are serialized separately via bincode and published on a dedicated gossipsub topic (SPHINX_TOPIC, mesh.rs:733).
- **No DTN integration**: There is no FrameKind::DtnBundle. DTN bundles are serialized directly, not wrapped in Frame.
- **CoverTraffic generation exists but the idle loop that produces it does not exist**: Frame::cover_traffic() works, but there is no background task that periodically sends cover frames.
- **WireMessage has a seq field** (for replay protection) but no caller populates it — it is always 0 in practice, making ReplayWindow ineffective.

---

## SUMMARY TABLE

| Feature | Status | Lines | Wired? | Spec Gap |
|---------|--------|-------|--------|----------|
| DTN epidemic (in-tree) | IMPLEMENTED | 744 | Partially (data structures exist, no encounter loop) | No over-wire protocol, no persistence |
| ghostwire-dtn crate | DUPLICATE | 777 | Not used | Has bundle encryption in-tree does not |
| Sphinx routing | IMPLEMENTED | 809 | Yes (mesh.rs) | 3-hop broken per known issue; MAC domain is public |
| Private routing (onion) | IMPLEMENTED | 353 | No | Zero callers |
| S/Kademlia defenses | IMPLEMENTED | 509 | No | Zero callers, mesh.rs still uses default gossipsub config |
| Mixing delays | IMPLEMENTED | 308 | Partial (queue exists, delay not applied) | apply_mixing_delay() never called |
| AI routing classifiers | LOADED | N/A | No | routing_classifier.rs, GNN router never consulted in message path |
| Transport trait | ABSENT | 0 | N/A | Enum-based dispatch, no trait, no plugin model |
| Transport switching | ARCHITECTURE ONLY | 54 | No | AutoTransportSwitcher exists but never invoked |
| TransportKind/TransportType | MISMATCHED | — | — | Two enums with same purpose, no conversion |
| Double Ratchet | IMPLEMENTED | 567 | Partial (in Encryption struct, not in message path) | Not integrated with transport or wire protocol |
| Wire Protocol | IMPLEMENTED | 424 | No | Not used by mesh.rs; FrameParser never called |
| Cover Traffic | IMPLEMENTED | ~10 lines | No | Generator exists, no idle loop |
| core_wiring.rs | SPEC ONLY | 367 | No | All comments, zero executable code |
| Reliability layer | IMPLEMENTED | 530 | Yes (in Core, hooks exist) | No periodic timeout check loop in mesh event loop |

---

## IMMEDIATE ACTION ITEMS (by impact)

1. **HIGH**: Wire S/Kademlia defenses into mesh.rs — 3 lines in the gossipsub construction. Without this, Sybil attacks on the DHT are trivially cheap.
2. **HIGH**: Wire AutoTransportSwitcher with periodic metrics polling so it actually switches transports on network degradation.
3. **HIGH**: Reconcile the two DTN implementations: fold bundle encryption from ghostwire-dtn crate into in-tree epidemic.rs, switch to the standalone crate, or delete one.
4. **MEDIUM**: Add a FrameKind::SphinxPacket and FrameKind::DtnBundle to wire protocol, then use FrameParser in mesh.rs message handler instead of raw bincode.
5. **MEDIUM**: Wire private routing (onion) into the message dispatch path as an alternative to Sphinx routing.
6. **MEDIUM**: Reconcile the two Sphinx implementations — either use the standalone sphinx-rs crate (with the 3-hop fix) or declare the in-tree version canonical and remove the crate.
7. **LOW**: Wire mixing delays into the Sphinx forward path (one await call before forwarding).
8. **LOW**: Add a background cover traffic loop using the existing Frame::cover_traffic().
9. **LOW**: Populate WireMessage.seq from a per-peer monotonic counter to make ReplayWindow effective.

---

## NEXT STEPS

The project has substantial, well-implemented core modules, but the integration layer (core_wiring.rs) is a spec document that was never applied. The critical gap is that the 4-layer AI routing system, S/Kademlia defenses, and transport switching logic exist as isolated islands with no callers. Recommend starting with reconciling the duplicate implementations (DTN, Sphinx), then wiring the defensive systems (S/Kademlia, AutoTransportSwitcher), and finally connecting the message path through the wire_protocol layer.
