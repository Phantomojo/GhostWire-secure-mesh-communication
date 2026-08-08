# The GhostWire Proving Ground — Addendum A
### Testing the Real Disaster/Transport-Selection Architecture and the Hybrid RNS Integration
**Supersedes nothing in the base document — extends it with concrete targets now that the real internals are known.**

---

## 0. What changed

The base Proving Ground document (§5.5, "the blackout scenario") proposed testing a generic disaster-detection concept. It turns out GhostWire already *has* this, as real (or precisely spec'd) code:

- `core/disaster.rs` — a `DisasterManager` with four levels (`Normal`, `Congested`, `Isolation`, `Blackout`), driven by `calculate_instant_level()` and gated by a two-rule hysteresis (`evaluate()`): worsening conditions apply **immediately**, improving conditions require `required_stable_samples` consecutive good readings before downgrading the alert level.
- `core/transport_selector.rs` — a Thompson Sampling multi-armed bandit that scores each transport (`signal`, `throughput`, `loss`, `latency`, `energy`, `congestion`, `reliability`), blends 70% weighted score with 30% bandit exploration, and learns **per-context** (a context is derived from current metrics — e.g. "high-loss context" vs "low-latency context" learn independently).
- `core/disaster.rs` `DisasterPolicy` — a ranked transport priority list *per disaster level* (Normal→QUIC>StealthTCP>Mesh; Congested→Mesh>WiFi; Isolation→WiFi>Bluetooth; Blackout→LoRa only).
- A parallel effort (`GHOSTWIRE_RNS_ARCHITECTURE.md`, `SYMBIOTIC_P2P_ARCHITECTURE.md`) to bring Reticulum (RNS) in as a second full transport stack, with a `UnifiedPeerTable` tracking every peer's addresses across both stacks, cross-pollinate discovery (a peer found via one transport gets registered for the other), and updated `DisasterPolicy` priority lists that include Reticulum specifically for Isolation/Blackout.

This means the testing ground's most valuable job right now is narrower and sharper than "test resilience generically": **does the DisasterManager correctly classify real degraded conditions, does the hysteresis actually prevent thrashing, and does Thompson Sampling actually converge on the transport the DisasterPolicy says it should use — under conditions that come from real `tc netem` faults, not just `simulate_transport_failure()` calls?**

---

## 1. The two testing philosophies, and why you want both

| | **In-process (sketched in `SYMBIOTIC_P2P_ARCHITECTURE.md` §6)** | **Black-box Docker chaos (this project's `chaos.sh`)** |
|---|---|---|
| Mechanism | Rust `#[tokio::test]`, calls `node.simulate_transport_failure(...)` directly | Real `tc netem` loss/delay/blackout applied to real container network interfaces |
| Speed | Fast (seconds), fully deterministic | Slower (minutes), subject to real scheduling/timing noise |
| What it catches | Logic bugs in `DisasterManager`, `TransportSelector`, `UnifiedPeerTable` — race conditions, off-by-one thresholds, GC correctness | Integration bugs — the gap between "the code believes X" and "X is actually what real conditions produce." This is exactly how the send-queue backpressure bug was found — that would **not** have been visible from a `simulate_transport_failure()` call, because nothing in that call path exercises the real GossipSub outbound queue under real congestion. |
| What it *can't* catch | Whether `simulate_transport_failure()` is actually a faithful model of what real network degradation looks like to the rest of the stack | Fast, cheap, fine-grained assertions on internal state — Docker-level tests can only observe what's in the logs/API, not internal Rust struct state |

**Recommendation: build both, and use one to validate the other.** Specifically — once the in-process `TestHarness.simulate_transport_failure()` exists, run the *same logical scenario* (e.g. "libp2p degrades to Congested-level loss") through both paths and diff the outcomes. If the in-process test says Thompson Sampling should shift to Mesh within N iterations, but the real Docker chaos test shows it taking 3x longer or not shifting cleanly, that gap **is** the finding — it tells you the mocked failure model in the Rust tests is unrealistic and needs correcting, which is valuable independent of whether GhostWire itself has a bug.

---

## 2. Flagship scenario, made precise: `disaster-ladder`

Replace the generic "blackout scenario" from the base doc with this precise version, since you now know the exact state machine being tested.

```yaml
name: disaster-ladder
seed: 7
description: >
  Drive a node through all four DisasterManager levels in sequence using
  real network conditions, and verify both the classification and the
  hysteresis behave as core/disaster.rs specifies.
steps:
  - action: netem
    target: gw-node4
    profile: nairobi-4g           # baseline: should read Normal
    duration: 30s
  - action: netem
    target: gw-node4
    profile: matatu-wifi           # latency/loss should cross Congested thresholds
    duration: 45s
  - action: netem
    target: gw-node4
    profile: rural-edge            # should NOT yet cross into Isolation (internet still reachable)
    duration: 45s
  - action: netem
    target: gw-node4
    profile: blackout              # internet unreachable -> Isolation, then Blackout if peer count drops
    duration: 120s
  - action: netem
    target: gw-node4
    profile: rural-edge            # recovery begins — hysteresis should HOLD at current level briefly
    duration: 60s
  - action: netem
    target: gw-node4
    profile: nairobi-4g            # sustained good conditions — should eventually downgrade to Normal
    duration: 90s
invariants:
  - disaster_level_sequence_matches: [Normal, Congested, Isolation, Blackout, Isolation, Congested, Normal]
  - no_downgrade_without_stability_window     # tests Rule 2 (slow recovery) directly
  - immediate_upgrade_on_worsening            # tests Rule 1 (fast adaptation) directly
  - active_transport_matches_disaster_policy  # e.g. at Blackout, active transport should be LoRa/RNS, not QUIC
  - no_stuck_send_queue                       # carried over from the base doc's invariant #1
```

**Why this specific sequence matters beyond "does it survive":** it directly exercises both hysteresis rules independently. The worsening leg (steps 1→4) should track the instantaneous level with effectively no delay. The recovery leg (steps 5→6) should *lag* — sitting at the worse level for `required_stable_samples` worth of good readings before downgrading. If your invariant checker sees the recovery leg downgrade immediately (no lag), that's a real bug in `evaluate()`, or a config value (`required_stable_samples`) set to something too low to matter in practice — either way, exactly the kind of thing worth catching once instead of debugging live in production.

**New invariant to add to `mesh-health.sh` / the invariant catalog:** `active_transport_matches_disaster_policy` — parse whichever log line indicates the currently active transport, and cross-check it against the `DisasterPolicy` priority list for whatever disaster level was just classified. This turns "did the mesh survive" into "did it survive *using the transport the architecture says it should have used*" — a meaningfully stronger claim.

---

## 3. Testing Thompson Sampling as a learning system, not just a switch

Because the transport selector is a contextual bandit, not a static rule, the interesting test isn't binary ("did it fail over") — it's **convergence behavior**. Two scenarios worth adding:

**`bandit-convergence`** — repeatedly cycle a node through the same degraded profile (e.g. `rural-edge`) many times in a row, and track which transport gets selected on each cycle. Early cycles should show exploration (occasional non-optimal transport choices — that's the 30% Thompson-sampled component doing its job). Later cycles should converge toward whatever transport actually performs best under that profile, and stay there. If it never converges (keeps flip-flopping) or converges instantly with no exploration at all (suggesting the bandit isn't actually being exercised), both are worth flagging — the first is a stability bug, the second suggests the exploration_rate config or the context-key derivation isn't working as designed.

**`bandit-context-isolation`** — verify contexts are actually independent, per the "learns different transport preferences for different conditions" design. Run `rural-edge` until convergence, note the preferred transport, then switch to `nairobi-4g` (a different context) and confirm the selector doesn't carry over the rural-edge preference inappropriately — then switch back to `rural-edge` and confirm the earlier learning persisted (context-keyed state should survive a detour into a different context). This is a real correctness property of contextual bandits that's easy to get subtly wrong (e.g. accidentally sharing one global Beta distribution instead of one per context).

---

## 4. Updating the Docker playground for the hybrid RNS reality

The current 6-node fleet is homogeneous — every node runs identical libp2p-only config. Once RNS integration lands (scoped at 2-4 hours per `GHOSTWIRE_RNS_ARCHITECTURE.md` §3), the playground should mirror the actual test topology already sketched in `SYMBIOTIC_P2P_ARCHITECTURE.md` §6.2.1, not stay homogeneous:

| Node persona | libp2p | RNS | Models |
|---|---|---|---|
| `gateway` | ✅ | ✅ (gateway mode) | A node bridging both networks — most real deployments probably look like this |
| `libp2p-only` | ✅ | ❌ | A node that only ever sees the internet-connected side |
| `rns-only` | ❌ | ✅ (access-point mode) | A pure off-grid/radio node — the whole reason RNS exists in this architecture |

Add this as compose profile labels (`role=gateway`, `role=libp2p-only`, `role=rns-only`) per the base doc's §4.1 recommendation, but now with real, specific roles instead of placeholders. The single most valuable new chaos scenario this unlocks: **`cross-pollinate-discovery`** — start an `rns-only` node with zero prior libp2p knowledge, confirm it becomes reachable from a `libp2p-only` node purely via a `gateway` node's cross-pollination, matching `test_cross_pollinate_discovery` from the Rust test spec — but verified against the real Docker network instead of an in-process mock.

**A second flagship scenario once RNS lands:** `rns-fallback-blackout` — take the `disaster-ladder` scenario above, but run it on a `gateway` node, and add the specific check that at Blackout level, messages actually continue flowing via the `rns-only` node's radio-equivalent path (simulated here as the RNS transport over the Docker network, since you won't have real LoRa hardware in the lab) while libp2p is fully dead. This is the single test that most directly proves or disproves GhostWire's actual thesis — a message sent while the "internet" (libp2p path) is fully blacked out still arrives, because RNS carried it.

---

## 5. One more layer worth a light touch: AI routing sanity checks

You've got a 4-layer ML routing stack (LightGBM fast-path, Gemma 4 analytical, GNN topology optimization, PRoPHET epidemic fallback for fully disconnected delivery). Full testing of this is its own project, but one cheap, high-value check fits naturally into the scenarios above: **during `disaster-ladder`, log whether Layer 1 (LightGBM, <1ms, runs on every decision) agrees with the Thompson Sampling selector's actual choice.** If they diverge often, that's worth knowing — it'd mean the fast-path model isn't well-calibrated to what the bandit has actually learned, which matters because the fast path is what runs on every single message, not just periodically like Gemma 4's 30-second analytical pass.

Not worth building dedicated infrastructure for this yet — just add a grep for both signals to `mesh-health.sh`'s output once the RNS/disaster scenarios are running, and see if anything looks off.

---

## 6. Updated priority order (supersedes base doc §7 roadmap for near-term work)

1. **Now:** formalize `no_stuck_send_queue` and `active_transport_matches_disaster_policy` as real invariant checkers (small, standalone scripts per the base doc's pattern).
2. **Now:** build `disaster-ladder` as a named `chaos.sh` scenario using the network profile table already defined in the base doc — no new tooling required, just a new scenario file.
3. **Once RNS Phase 1 lands (2-4 hours per the RNS doc's own estimate):** add the three node personas to the compose file, build `cross-pollinate-discovery` and `rns-fallback-blackout`.
4. **After that:** `bandit-convergence` and `bandit-context-isolation` — these need the disaster-ladder infrastructure working first since they reuse the same profile-cycling mechanism.
5. **Whenever the in-process Rust `TestHarness` gets implemented (currently a spec, not code):** cross-validate one scenario (recommend `disaster-ladder`, since it's the most state-machine-precise) through both paths and diff the outcomes, per §1 above.

---

## 7. What stays unchanged from the base document

Everything in the base Proving Ground doc's §5 (churn realism, degraded-but-alive modeling, asymmetric partition, handshake storms) and §6 (byzantine node simulation) still applies exactly as written — none of it conflicts with what the new architecture docs reveal, it's just now got a sharper target to aim at. The byzantine node category (§6) is arguably *more* relevant now, not less: `byzantine-dht-poison` and `byzantine-sybil` both become more interesting once there's a `UnifiedPeerTable` and cross-pollination mechanism that a malicious node could try to poison across *both* transport stacks simultaneously — worth keeping in mind for when Phase 5 comes around.
