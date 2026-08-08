# The GhostWire Proving Ground
### A design document for a mesh-native testing and observability instrument
**Prepared for: implementation by coding agent** | **Author: Claude, for RVC** | **August 2026**

---

## 0. Read this first (for the agent implementing this)

This document specifies a *testing instrument*, not a feature. Its job is to answer one question with evidence, repeatedly, cheaply: **"Under conditions this mesh is actually meant to survive, does it survive — and if not, why not?"**

Everything below is organized so you can implement it in phases without needing the whole vision at once. Phase 1 is buildable today on top of the existing `raw-logs.sh` / `chaos.sh` / `mesh-health.sh` / `preflight.sh` scripts already in the repo. Later phases are genuine upgrades, not replacements — nothing here asks you to throw away working code.

Where a design choice was informed by external research rather than invented, that's cited inline so you can go verify it rather than take it on faith.

---

## 1. Why this is designed the way it is

Three lineages of practice inform this design, and GhostWire's specific mission bends all three toward something more particular than generic "chaos engineering."

**Lineage 1 — Chaos engineering (Netflix Chaos Monkey and descendants).** The original insight: you don't know your system is resilient until you've actually broken it, on purpose, and watched it recover. This is the philosophy behind the `chaos.sh` scenarios already built (churn, partition, netem). Its limit: it's *random* and *reactive* — it tells you a system survived what you happened to throw at it, not what it's guaranteed to survive.

**Lineage 2 — Distributed systems correctness testing (Jepsen, and its philosophical descendant, Deterministic Simulation Testing).** Kyle Kingsbury's Jepsen framework introduced the idea of a **"nemesis"** — a process that deliberately attacks a cluster (partitions, clock skew, process kills) while a **checker** verifies that observed behavior never violates a stated invariant (e.g., linearizability). The key move Jepsen made that Chaos Monkey didn't: *you write down what "correct" means first*, then you attack until you find a violation or run out of budget.

The more advanced descendant of this is **Deterministic Simulation Testing (DST)**, pioneered by FoundationDB and refined by companies like TigerBeetle and Antithesis. The core idea: run the *entire distributed system* — all nodes, network, disk, clock — inside a single-threaded, seeded, deterministic simulation. Every source of randomness (message ordering, delays, failures) is controlled by a seed. When a bug is found, you get the exact seed back, and can replay the *exact* failure deterministically, forever, in milliseconds instead of hours. FoundationDB's team has stated this is *why* their database earned a reputation as one of the most battle-tested distributed systems ever built — Jepsen's own author declined to test it because their internal simulator already exceeded what Jepsen could do. TigerBeetle now runs the equivalent of **millennia of simulated node-years per night** on commodity hardware.

Full DST is a large engineering investment (it requires GhostWire's Rust core to have injectable/pluggable time, randomness, network, and disk — a substantial refactor). This document does **not** ask you to build that now. But it borrows DST's most transferable idea even without the full architecture: **every chaos run should be seeded and reproducible**, and **every scenario should check a stated invariant, not just "did it crash."**

**Lineage 3 — libp2p's own testing evolution.** This is the most directly relevant prior art, because GhostWire is *built on* libp2p (GossipSub, Kademlia DHT). The libp2p project ran a platform called **Testground** for years — a heavyweight system supporting 2 to 10,000-node simulated networks with built-in Grafana metrics and `tc`/`netem`-equivalent network shaping. In **January 2026**, the libp2p team published a retrospective explaining why they moved *away* from Testground toward a much leaner bash + Docker framework: Testground's operational complexity (a full orchestration platform, Kubernetes runners, custom builders) cost more in maintenance than it returned in test velocity for their actual use case. Their replacement optimizes for **fast feedback, reproducibility, and near-zero dependency maintenance** — running natively via plain Docker Compose on Linux/macOS/Windows.

**The lesson for GhostWire, directly:** don't build a Testground. Build the libp2p team's *2026* answer — lean, Docker-native, scriptable, seeded where it matters — sized for a solo operator's lab, not an enterprise SRE team. Everything in this document is scoped accordingly.

---

## 2. What makes GhostWire's testing ground different from a generic chaos lab

A generic chaos lab throws generic faults (kill -9, iptables DROP, sleep) at a generic service. GhostWire is not a generic service — it's explicitly designed to survive **sub-Saharan African low-connectivity conditions and hostile network environments**. The testing ground should be a *working model of that specific world*, not a stock chaos-monkey deployment. This is where the design gets to be genuinely inventive rather than a checklist copy of Netflix's playbook.

Concretely, that means building fault profiles modeled on real, documented conditions rather than abstract "30% packet loss":

- **The matatu wifi problem** — many peers joining and leaving a local mesh in bursts, on cheap shared hotspots, for minutes at a time. This is `churn` but with a *realistic arrival/departure distribution*, not a single stop/start. (Grounded in P2P churn-modeling research — Stutzbach & Rejaie's peer-churn studies characterize real P2P networks as having heavy-tailed session lengths, not uniform on/off cycles — worth modeling GhostWire's churn scenarios the same way instead of one clean stop/start.)
- **The rural EDGE/2G tower problem** — sustained high latency (500ms-2s), low bandwidth (~50-250kbit/s), and periodic total blackouts as a phone roams between towers. Standard "Slow 3G" browser dev-tool profiles (500Kb/s down, 200Kb/s up, 200-400ms latency) are a reasonable starting baseline, but GhostWire's target is often *worse* than that — this should go further into 2G/EDGE territory.
- **The internet-shutdown problem** — a full, sustained partition of part of the mesh from the rest, potentially for hours, not seconds. Kenya and neighboring countries have documented precedent for regional or national connectivity shutdowns during periods of unrest. This is GhostWire's actual reason for existing, so it deserves to be a first-class, named scenario, not a footnote — see §5.5.
- **The hostile-peer problem** — not every node in a real deployment will run honest code. A mesh built for resilience against surveillance/censorship should assume some fraction of peers may be malicious: spamming GossipSub, attempting DHT poisoning, replaying stale Double Ratchet messages, or forging CRL entries. This is a different test category from network chaos entirely — see §6.

This reframing is the "inspiration" layer: the testing ground's fault catalog should read like a field guide to how GhostWire will actually die in production, not a generic chaos-engineering checklist.

---

## 3. Architecture: six layers

```
┌─────────────────────────────────────────────────────────────┐
│ L5  REPLAY & REGRESSION                                      │
│     Seeded scenarios. A failure becomes a permanent test.    │
├─────────────────────────────────────────────────────────────┤
│ L4  INVARIANT / VERDICT LAYER                                │
│     "Did the mesh actually stay correct?" — not just "alive" │
├─────────────────────────────────────────────────────────────┤
│ L3  SCENARIO ORCHESTRATION                                   │
│     Named, composable fault campaigns (YAML-defined)         │
├─────────────────────────────────────────────────────────────┤
│ L2  OBSERVABILITY                                             │
│     Aggregated structured logs + live topology + metrics     │
├─────────────────────────────────────────────────────────────┤
│ L1  FAULT INJECTION                                           │
│     Network shaping, churn, partition, adversarial peers      │
├─────────────────────────────────────────────────────────────┤
│ L0  NODE FLEET                                                 │
│     The docker-compose mesh itself (currently 6 nodes)        │
└─────────────────────────────────────────────────────────────┘
```

Layers 0-1 exist today (docker-compose + `chaos.sh`). This document's contribution is hardening L1, and building L2 through L5 on top.

---

## 4. Layer by layer: what to build

### 4.1 L0 — Node fleet upgrades

- **Scale knob.** Parameterize `docker-compose.yml` (or move to a generator script) so the fleet can be spun up at N=6, N=12, N=25 nodes without hand-editing YAML. Even without full Testground-scale (thousands of nodes), going from 6→25 nodes will surface convergence-time and gossipsub-fanout behaviors invisible at 6.
- **Resource realism.** Current 512MB/node limit is reasonable for a laptop-class edge device. Consider adding a **low-resource tier** (128-256MB, 0.25 vCPU) to model an actual cheap Android phone or Raspberry Pi-class deployment target, since that's closer to GhostWire's real hardware profile than a dev laptop container.
- **Node roles/personas.** Tag containers by role in compose labels: `role=honest`, `role=byzantine`, `role=bootstrap`, `role=low-power`. This lets later scenario tooling target "all byzantine nodes" or "all low-power nodes" declaratively instead of hardcoding container names.

### 4.2 L1 — Fault injection: tool selection, with rationale

You already have **pumba** wired up (wraps `tc`/`netem` inside target containers). Keep it — it's the right tool for *container/interface-level* network shaping (delay, loss, bandwidth, corruption, duplication) because it operates below the application, requiring zero code changes to GhostWire itself.

Add **Toxiproxy** (Shopify) as a *complementary*, not competing, tool for a specific gap pumba can't fill well: **programmatic, fine-grained, protocol-aware faults on a specific connection** — e.g., TCP RESET injection, slow-close, "slicer" (chopping messages into tiny fragments to stress your framing/parsing code), or bandwidth caps that persist across reconnects without needing root/NET_ADMIN in the target container. Toxiproxy sits as a proxy *in front of* a node's listen port; GhostWire nodes would dial through it. This is more invasive to set up (requires routing peer connections through the proxy) but gives you fault types pumba's `tc netem` wrapper fundamentally cannot: connection resets, TCP-level tampering, byte-level corruption of specific streams.

**Recommendation:** pumba for network *conditions* (latency, loss, bandwidth — modeling bad towers), Toxiproxy for connection *pathology* (resets, malformed streams — modeling adversarial or buggy peers) once you're ready for that level of depth. Don't build the Toxiproxy integration in Phase 1 — it's real engineering effort (requires re-routing libp2p dial addresses through proxy ports) and pumba covers the highest-value scenarios first.

**Realistic network profiles to encode as named presets** (build these into `chaos.sh` as `--profile` shortcuts rather than requiring the user to remember raw `tc` numbers each time):

| Profile name | Latency | Loss | Bandwidth | Models |
|---|---|---|---|---|
| `nairobi-4g` | 40-80ms | 0-1% | unrestricted | baseline urban control |
| `matatu-wifi` | 100-300ms | 2-5% | 1-3 Mbit/s | shared hotspot, many peers |
| `rural-3g` | 200-500ms | 3-8% | 250-750 kbit/s | standard "Slow 3G" territory |
| `rural-edge` | 500-2000ms | 5-15% | 50-150 kbit/s | 2G/EDGE fallback, GhostWire's harder target |
| `dying-battery` | normal → periodic 100% loss bursts (device sleep/power-save cycling) | | | intermittent solar/battery-powered node |
| `blackout` | 100% loss, sustained (minutes-hours, not seconds) | | | regional shutdown — see §5.5 |

### 4.3 L2 — Observability: from manual grep to a real instrument

This is the highest-leverage upgrade available right now, because every diagnostic session so far has been: capture logs → tarball → upload → manual grepping across 6 files. That doesn't scale, and worse, **it's not live** — you only find out about a stuck send-queue after the fact, in a forensic log dive.

**Minimum viable upgrade (do this first):** add a log-shipping sidecar to `docker-compose.yml` so all 6 nodes' structured JSON logs land in one queryable place instead of six separate files.

- **Option A (simplest): Dozzle** — a single lightweight container, zero config, gives you a live web UI tailing all container logs side-by-side with search. No query language, but near-zero setup cost. Good enough for solo real-time debugging during a chaos run.
- **Option B (real upgrade): Vector → Loki + Grafana.** Vector ships as a tiny sidecar per node (or one central collector reading the Docker socket), parses your structured JSON logs, and forwards to Loki. Grafana then gives you `logql` queries like `{container=~"gw-node.*"} |= "Send Queue full"` across the whole mesh instantly, plus the ability to build the metrics dashboard described next. This is the CERBERUS Phoenix HUD instinct, redirected at GhostWire.

**Metrics worth graphing over time** (whether via Grafana or a simpler polling script that just writes CSV — start simple):
- Neighbor count per node (mesh completeness over time — would have shown the churn test's dip-and-recovery as a graph instead of a grep)
- GossipSub send-queue depth / `Send Queue full` event rate per node (would have caught the stuck-queue bug in real time instead of a forensic replay)
- Handshake completion latency (time from `Received Handshake` to `Stored encryption key`)
- Reconnect attempts per node per minute
- DTN sync backlog size (once that subsystem is instrumented — check if it already emits a queue-depth metric; if not, that's a small, high-value addition to the Rust core)

**A live topology view.** Once you confirm the actual v2 API routes, a small polling script (or a tiny HTML/JS page) that queries each node's peer list every few seconds and renders a force-directed graph would make partition/reconvergence tests *visually obvious* — you'd see the mesh split into two visual clusters and watch them re-merge, instead of inferring it from neighbor-count numbers.

### 4.4 L3 — Scenario orchestration: from scripts to a scenario language

`chaos.sh` today is a good, functional set of positional-argument scenarios. The upgrade: move scenario *definitions* into small YAML files that `chaos.sh` (or a thin Python/Rust runner) interprets, so scenarios become declarative, composable, and reviewable in a diff — not buried in shell logic.

Example scenario file (`scenarios/rural-blackout-recovery.yaml`):
```yaml
name: rural-blackout-recovery
seed: 42                       # for reproducibility (see L5)
description: >
  Simulate a rural node losing tower connectivity entirely for 3 minutes,
  then recovering under degraded EDGE conditions before returning to normal.
steps:
  - action: netem
    target: gw-node5
    profile: rural-edge
    duration: 60s
  - action: netem
    target: gw-node5
    profile: blackout
    duration: 180s
  - action: netem
    target: gw-node5
    profile: rural-edge
    duration: 60s
  - action: settle
    duration: 45s
  - action: clear_faults
    target: gw-node5
invariants:
  - mesh_reconverges_within: 90s
  - no_stuck_send_queue
  - no_duplicate_message_delivery
  - all_nodes_report_full_neighbor_count
```

This also makes **campaigns** — sequences of scenarios run back-to-back to model a realistic bad day — a natural extension:

- **The Nairobi Blackout Campaign** — normal operation → regional shutdown (all-to-all partition) → partial restoration (some nodes back, some not) → full restoration. Tests the exact censorship-resistance thesis GhostWire is built on.
- **The Rural Handset Campaign** — a node cycling through `nairobi-4g` → `rural-3g` → `rural-edge` → `dying-battery` → `blackout` → recovery, over an extended run. Tests graceful degradation, not just binary up/down.
- **The Malicious Insider Campaign** — see §6, an honest mesh with one node flipped to byzantine behavior mid-run.

### 4.5 L4 — Invariants: stop measuring "alive," start measuring "correct"

This is the single most valuable conceptual upgrade in this document, borrowed directly from Jepsen's nemesis/checker split. Right now, `mesh-health.sh` checks things like "did neighbor count return to N-1" — good, but that's an *aliveness* check, not a *correctness* check. A mesh can be "alive" (all nodes connected) while silently violating properties that actually matter for a secure messaging system.

**Candidate invariants worth writing checkers for, roughly in priority order:**

1. **No unbounded queue growth** — directly motivated by the send-queue finding from the last test run. A `Send Queue full` count that never stops climbing across a scenario, or that's still climbing N seconds after the fault clears, is a FAIL, not a WARN. (Already partially in `mesh-health.sh` — formalize it as a first-class invariant with a hard time bound, e.g. "queue depth must return to baseline within 60s of fault clearing.")
2. **Mesh reconvergence bound** — after any partition heals or node restarts, all healthy nodes must reach full neighbor count within a stated time budget (start with a generous bound, e.g. 90s, and tighten it as you learn real numbers).
3. **No message loss across a transient partition** — messages sent to a node during a partition should arrive via DTN sync once it heals, not be silently dropped. This is GhostWire's actual value proposition and deserves a dedicated test: publish N tagged messages to a partitioned sub-mesh, heal the partition, assert all N are eventually delivered mesh-wide.
4. **No duplicate decryption / replay acceptance** — the Double Ratchet protocol should reject replayed ciphertext. A chaos scenario that intentionally re-delivers a captured message (easy to do via a byzantine node, see §6) should result in the message being rejected, not re-processed.
5. **Fresh session after reconnection** — when a node drops and rejoins, confirm it re-handshakes with a *new* key (already partially visible in your logs as fresh `Stored encryption key` events) rather than silently reusing a stale Double Ratchet state, which would be a forward-secrecy regression.
6. **CRL (cert revocation list) propagation completeness** — if a cert is revoked during a partition, does every node eventually learn about it once healed? This is a correctness property specific to GhostWire's `ghostwire/kad/shard/global` and CRL gossip topic — worth a dedicated scenario once the CRL subsystem has clear success signals in its logs.
7. **Sphinx onion-routing anonymity set maintained under partial node loss** — harder to verify mechanically, but worth a manual/semi-automated check: does losing 1-2 nodes collapse the anonymity set for onion-routed messages below a safe threshold? This one may start as a design-review question rather than an automated check.

Each invariant should be a small, independent script (bash is fine to start) that takes a `mesh-health.sh`-style log bundle and returns pass/fail plus evidence — composable the same way the existing scripts already are.

### 4.6 L5 — Seeds and replay

Full DST is out of scope for now (see §1), but you can borrow its cheapest, highest-value idea without rebuilding the Rust core: **every scenario run gets a seed, and the seed plus the scenario YAML together fully determine what was injected and when.**

Practically: when `chaos.sh` runs a scenario, log the seed (even if it's just used to derive fault timing jitter, or which node in a group gets targeted for partial-partition scenarios) into the scenario marker file. If a run finds a bug, the scenario file + seed becomes a **permanent regression test** — re-run it after a fix to confirm the fix actually holds, and keep it in a `scenarios/regressions/` folder that grows over time. This is the practical, low-investment version of "if you can't reproduce it, you can't fix it" without needing a full deterministic hypervisor.

---

## 5. The realistic-conditions fault catalog (detailed)

This expands §2 into concrete, implementable specs.

### 5.1 Churn, modeled realistically
Real P2P churn isn't clean stop/start — session lengths are heavy-tailed (many short sessions, a few long ones). Rather than `chaos.sh churn <node>` on a fixed timer, add a `chaos.sh churn-realistic <node-group> <duration>` mode that picks downtime intervals from a distribution (even a simple bounded Pareto/log-normal approximation is far more realistic than uniform) and cycles multiple nodes independently within the window — modeling a shared local mesh where people's phones genuinely come and go.

### 5.2 Degraded-but-alive (not just binary up/down)
Most of today's testing has been binary (node up / node down). Real rural connectivity is rarely binary — it degrades gradually. The `rural-edge` → `dying-battery` → `blackout` → recovery sequence in §4.4's example scenario models this. This is a meaningfully different test than churn: does GhostWire detect and adapt to *degrading* quality (e.g., should the AI routing layer, once Ollama is reachable, be deprioritizing a degrading peer proactively rather than waiting for a hard failure?), or does it only react to hard failures?

### 5.3 Asymmetric partition
Real network failures are often asymmetric (A can send to B, B can't send to A) — congested uplinks are a classic real-world cause. `pumba netem loss` applied to only one direction (achievable via `tc` direction-specific rules, or by applying the fault to only one side of a pair) is worth a dedicated scenario, since GossipSub's mesh-formation logic may behave very differently under asymmetric vs symmetric loss.

### 5.4 The handshake storm, scaled up
Already have `chaos.sh storm` (simultaneous restart of all 6). Worth extending to a **cold-start storm** — tear down all 6 containers *and volumes* and bring the whole mesh up from nothing simultaneously, to test bootstrap-node discovery and initial DHT population under maximum contention, not just reconnection of already-known peers.

### 5.5 The blackout scenario (GhostWire's actual thesis, tested directly)
This deserves to be the flagship scenario, not a footnote. A sustained (minutes, not seconds) 100%-loss partition applied to a subset of nodes, modeling a regional internet shutdown, followed by healing. The invariant that matters here isn't just reconvergence — it's **did any message get permanently lost**, and **did the DTN sync subsystem actually do its job** once the partition healed. If you only build one invariant checker first, build #3 from §4.5 and pair it with this scenario — it's the most direct test of the reason GhostWire exists.

---

## 6. Adversarial / byzantine node simulation

This is a genuinely different test category from network chaos — it tests whether GhostWire is *defensible*, not just *resilient*. Given the security/forensics background behind this project, this is likely the most interesting category to build out, and it's underexplored by generic chaos tooling entirely (Chaos Monkey and pumba have no concept of a node that's technically "up" but behaving maliciously).

**Approach:** build a small number of "byzantine node" container images — deliberately-misbehaving GhostWire instances (or lightweight libp2p stub nodes, if forking the full app is too heavy) — each modeling one specific attack:

- **`byzantine-spam`** — floods GossipSub with valid-looking but garbage messages, testing rate-limiting and the send-queue backpressure behavior under adversarial (not just organic) load. Directly relevant given the last test's stuck-queue finding — is that bug *worse* under deliberate flooding?
- **`byzantine-replay`** — captures and re-broadcasts old ciphertext, testing Double Ratchet replay rejection (invariant #4 above).
- **`byzantine-dht-poison`** — announces false routing records into the Kademlia DHT, testing whether honest nodes validate DHT entries or trust them blindly.
- **`byzantine-crl-forge`** — attempts to inject a forged certificate-revocation entry, testing CRL validation.
- **`byzantine-sybil`** — spins up many low-cost identities simultaneously to test Sybil resistance in peer selection/routing.

Each gets its own scenario file and, critically, its own invariant: "the honest mesh continues to function correctly *despite* this node's presence," not "the mesh doesn't crash." This is a natural fit for the **Malicious Insider Campaign** named in §4.4.

---

## 7. Implementation roadmap (phased, for the agent)

**Phase 1 — Harden what exists (today, low effort).**
- Ship `preflight.sh`, `chaos.sh`, `mesh-health.sh`, `raw-logs.sh` as-is (already done).
- Add named network profiles (§4.2 table) to `chaos.sh netem-loss`/`netem-delay` as a `--profile` flag.
- Add the `no_stuck_send_queue` hard-bound check to `mesh-health.sh` as a first invariant (formalize what's already informally checked).

**Phase 2 — Observability (highest leverage, moderate effort).**
- Add Dozzle to `docker-compose.yml` for immediate live multi-node log viewing (near-zero effort, do this first).
- If/when time allows: Vector → Loki → Grafana for queryable logs + the metrics list in §4.3.

**Phase 3 — Scenario language + invariants (moderate effort).**
- Move scenario definitions to YAML (§4.4 format).
- Build 2-3 invariant checkers (§4.5, items 1-3 first) as standalone scripts callable from `mesh-health.sh`.
- Build the blackout scenario (§5.5) as the flagship test.

**Phase 4 — Scale (moderate effort, mostly compose-file work).**
- Parameterize node count; test at 12 and 25 nodes to see if convergence-time or queue-depth behaviors change nonlinearly.

**Phase 5 — Adversarial nodes (higher effort, most novel/interesting).**
- Build the byzantine node images (§6), starting with `byzantine-spam` since it directly follows up on the real finding from the last test.

**Phase 6 — Seeds + regression suite (ongoing, low effort per-instance).**
- Every bug found via Phase 3-5 scenarios becomes a permanent seeded regression test, re-run before any significant merge.

**Not in scope for now, but worth knowing exists:** full Deterministic Simulation Testing (§1) would require GhostWire's Rust core to abstract time, randomness, network I/O, and disk I/O behind injectable traits — a genuine architecture investment, most valuable *if* GhostWire's core protocol logic (Double Ratchet state machine, GossipSub mesh maintenance, DHT routing) reaches a stability point where correctness bugs become rare and expensive to find by black-box means alone. Worth revisiting after Phase 5, not before.

---

## 8. Closing note on scope

Every recommendation here is sized to be buildable by one operator directing an AI coding agent, on a single machine, without adopting a platform (Kubernetes, a hosted chaos-engineering SaaS, a full DST rewrite). The libp2p team's own 2026 retrospective on abandoning Testground is the load-bearing argument for that restraint: more platform is not automatically more signal. The goal is a lab that tells you the truth about GhostWire fast, cheaply, and repeatedly — not an impressive-looking piece of infrastructure that's expensive to keep alive.
