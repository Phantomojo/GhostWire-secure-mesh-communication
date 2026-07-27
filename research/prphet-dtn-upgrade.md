# PRoPHET DTN Algorithm Upgrade Research

> **Source**: Background agent bg_bc38c7dc (librarian agent)
> **Date**: 2026-07-27
> **Status**: Raw agent output — unmodified

---

# PRoPHET for DTN Mesh Networks — Complete Reference

---

## 1. PRoPHET v1 Core Algorithm (2003, Lindgren & Doria)

The protocol hinges on **Delivery Predictability** `P(a,b) ∈ [0,1]` — a scalar at every node for every known destination.

### Three operations on every encounter

**Update on direct encounter** (Eq 1):
```
P(a,b) = P(a,b)_old + (1 - P(a,b)_old) × P_encounter
```
Where `P_encounter` is a configurable constant (typically 0.75 on first meet).

**Aging** (Eq 2) — decay over time:
```
P(a,b) = P(a,b)_old × γ^k
```
Where `γ ∈ [0,1]` is the aging constant and `k` is time units since last aging.

**Transitivity** (Eq 3):
```
P(a,c) = P(a,c)_old + (1 - P(a,c)_old) × P(a,b) × P(b,c) × β
```
Where `β` controls how fast transitive knowledge spreads.

**Forwarding rule**: Forward bundle for destination `D` from node A to node B iff `P(B,D) > P(A,D)`.

### State-of-the-art improvements (2024–2026)

| Improvement | Concept | Gain | Source |
|---|---|---|---|
| **R-PRoPHET** | Replaces fixed `I_typ` (typical time between encounters) with dynamic **estimated encounter time** from a weighted moving average of past inter-encounter intervals. Also adds a forwarding strategy that forwards when `P(B,C) >= P(A,C)` (not just `>`), which starts routing earlier. | +20% delivery ratio, -16% latency | [Arteaga-Arce, Mitton 2024](https://hal.science/hal-04684901v1) |
| **TES-PRoPHET** | Triple exponential smoothing over DP time series to compute adaptive delivery predictability (ADP). Handles non-linear trends in encounter patterns. | Significantly higher delivery rate, lower avg delay vs PRoPHETv2 and AR-PRoPHET | [Peng et al., 2024](https://doi.org/10.1109/ucom62433.2024.10695840) |
| **LH-PRoPHET** | Incorporates trajectory prediction from position history. Transfers messages to nodes moving in divergent directions for maximum coverage. | Higher delivery ratio, lower latency with less resource consumption | [Matsubara, Hayashibara 2025](https://doi.org/10.1109/cyberscitech68397.2025.00036) |
| **Energy/Buffer-optimized PRoPHET** | Adds ACK-based buffer management + energy-aware forwarding for smart city IoT | Better delivery under energy constraints | [Spaho et al., 2025](https://doi.org/10.15157/IJITIS.2025.8.3.687-706) |
| **Epidemic-hybrid PRoPHET** | Uses epidemic flooding for the first `N` forwarding hops, then switches to PRoPHET. Controlled by threshold counters. | Higher delivery, lower delay than pure PRoPHET | [Han & Chung, improved PRoPHET](https://pmc.ncbi.nlm.nih.gov/articles/PMC4306380/) |
| **Scheduling-PRoPHET** | Adds message priority scheduling with hop-count and traffic-load tiebreakers in buffer management | Lower drop rate, better overhead | [Scheduling-PROPHET, Sensors 2019](https://pmc.ncbi.nlm.nih.gov/articles/PMC6358925/) |

---

## 2. PRoPHET v2 — Changes & What Improved

Defined in [Grasic, Davies, Lindgren, Doria 2011](https://doi.org/10.1145/2030652.2030661). Protocol version `0x02` per [RFC 6693 IANA registry](https://www.iana.org/assignments/prophet/prophet.xhtml).

### The "Parking Lot Problem"

**v1 issue**: When many nodes cluster (e.g., a parking lot, or flapping wireless links), every encounter triggers transitive updates. `P(a,c)` for *every* known node rises monotonically even if none of those nodes were actually met. This creates false high DPs for distant nodes, destroying the routing gradient.

### v2 fixes — two equation changes

**New direct-encounter update** (Eq 4 in v2):
```
P_encounter = P_encounter_max × (IntvlB / I_typ)  for IntvlB < I_typ
P_encounter = P_encounter_max                       for IntvlB ≥ I_typ
```
Where `IntvlB` is time since last encounter with node B. If two encounters happen faster than `I_typ` (the expected interval), the DP boost is proportionally reduced. This stops repeated handshakes from over-inflating DPs.

**New transitive update** (Eq 5 in v2):
```
P(a,i) = max(P(a,i)_old,  P(a,b) × P(b,i) × β)
```
Changed from *additive* (`old + increment`) to *max-based*. This means transitive updates can only increase DPs up to what the intermediary actually knows — no unbounded creep from repeated meetings.

### Other v2 changes

- **`P_encounter_first` set to 0.5** (was 0.75 in early v1). Reduces the impact of single random encounters.
- **`P_encounter_max` caps** the maximum DP increment per encounter.
- **Explicit `δ` parameter**: maximum DP is `1 - δ`, preventing DPs from reaching exactly 1.0 (`P(a,b) = P(a,b)_old + (1 - δ - P(a,b)_old) × P_encounter`).
- **Clarified behavior for long-duration encounters** — when to re-exchange DPs.
- **Multiple simultaneous contact handling** formally specified.
- **Bundle offer/response TLVs** added offset/length fields for fragments and source EID.
- **Submessage mechanism** with sequence numbers for unreliable transports (UDP).
- **`β=0.9` recommended** default (was 0.25 in some earlier v1 specs).

### Performance

v2 performs **similar to or better than MaxProp and Spray-and-Wait** in heterogeneous mobility scenarios, while maintaining a full RFC specification for interoperable implementations.

---

## 3. Known Weaknesses & How to Address Them

### Weakness 1: Random Waypoint mobility kills PRoPHET
**Problem**: PRoPHET assumes mobility has patterns. Under Random Waypoint (memoryless movement), DPs never converge to meaningful values. Epidemic or Spray-and-Wait outperforms it.

**Fix**: Detect mobility pattern entropy. If encounters look random (high entropy), fall back to Spray-and-Wait or binary spray. The [PRoPHETv2 paper](https://www.diva-portal.org/smash/get/diva2:1043060/FULLTEXT01.pdf) documents this exact finding.

### Weakness 2: Slow dissemination speed
**Problem**: PRoPHET only forwards when `P(neighbor) > P(self)`, so in early network life when no node has any DP for a destination, no forwarding happens.

**Fix**: Use **epidemic boost** for the first N copies or H hops ([Han & Chung](https://pmc.ncbi.nlm.nih.gov/articles/PMC4306380/) — +delivery, -delay). Or use R-PRoPHET's `>=` forwarding strategy.

### Weakness 3: No buffer management awareness
**Problem**: PRoPHET forwards based solely on DP — doesn't consider remaining buffer space, message priority, or TTL urgency.

**Fix**: Adopt **Scheduling-PRoPHET**'s multi-factor forwarding: DP > hop count > traffic load tiebreakers. Implement **message priority queuing** (TTL-based preemption). The [Forward/Drop policy paper](https://doi.org/10.1109/msn.2016.045) adds optimal stopping rules.

### Weakness 4: Single metric fragility
**Problem**: DP alone doesn't capture energy, bandwidth, or congestion.

**Fix**: **PRoPHET+** ([Huang et al., 2010](https://doi.org/10.1109/aina.2010.162)) computes a weighted "deliverability" score from DP + buffer size + power + location + popularity. **Distance-based PRoPHET** ([Sok et al., 2013](https://doi.org/10.1109/hpcc.and.euc.2013.175)) adds cross-layer distance.

### Weakness 5: Parameter sensitivity
**Problem**: Alpha, beta, gamma, I_typ are static and scenario-dependent. Wrong values kill performance.

**Fix**: Use ANN to predict optimal α, β, γ from network conditions ([Kumar et al., 2023](https://doi.org/10.14569/ijacsa.2023.0140522) — 99% accuracy predicting optimal params). Or use R-PRoPHET's dynamic I_typ estimation.

### Weakness 6: No security model
**Problem**: Malicious nodes can advertise fake DPs, intercept bundles, or flood false transitive info.

**Fix**: Sign DP exchanges (Ed25519 as done in [GhostWire](https://github.com/Phantomojo/GhostWire-secure-mesh-communication/blob/main/docs/spec/DTN.md)). Use reputation scoring for peers. Wise-PRoPHET ([Basu et al., 2018](https://doi.org/10.1016/j.jnca.2018.02.012)) adds watchdog supervision.

---

## 4. Adding ML/AI to PRoPHET — Practical Strategy

### What data to collect at each node

Store per-encounter events as a time-series dataset:

| Field | Type | Why |
|---|---|---|
| `timestamp` | u64 | Time of encounter |
| `peer_id` | [u8; 32] | Node identity encountered |
| `encounter_duration_s` | f32 | How long contact lasted |
| `throughput_bps` | f32 | Historical throughput with this peer |
| `buffer_occupancy` | f32 | % buffer full at encounter |
| `battery_level` | f32 | 0.0–1.0 remaining |
| `dp_after` | f32 | DP value after update |
| `inter_encounter_interval_s` | f32 | Time since last encounter with this peer |
| `num_hops` | u8 | Hops of bundles exchanged |
| `delivery_success` | bool | (Label) Did a bundle destined through this peer reach its destination? |
| `transports_active` | u8 | Bitmask of active transports (BLE=1, WiFi=2, LoRa=4, TCP=8) |

### Feature engineering — what matters

Research ([MLProph, 2016](https://doi.org/10.1109/jsyst.2016.2630923); [Prophet_TD, 2020](https://www.techscience.com/cmc/v64n3/39462)) identifies these most predictive features:

1. **DP value history** (last N values per node) — most important single feature
2. **Inter-encounter interval variance** — high variance = unreliable peer
3. **Encounter duration** — longer contacts transfer more bundles
4. **Node popularity** — how many distinct peers a node has met
5. **Transitivity depth** — how many hops of transitive knowledge separate you from destination
6. **Throughput** — actual data transfer rate during encounters
7. **Battery trend** — draining fast → may die soon
8. **Time-of-day / day-of-week** — human mobility follows circadian patterns

### Model architectures that work on-device

| Approach | Architecture | Input | Output | Computational Cost | Target Hardware |
|---|---|---|---|---|---|
| **ANN parameter predictor** | 4-layer feedforward (64→32→16→3) | Throughput, delay, buffer | α, β, γ | Very low (~100 params) | ESP32, Cortex-M | [Kumar et al., 2023](https://doi.org/10.14569/ijacsa.2023.0140522) |
| **MLProph** | Decision tree + shallow NN | DP, popularity, battery, speed, location | Binary: forward/don't | Low | Mobile CPU | [Sharma et al., 2016](https://doi.org/10.1109/jsyst.2016.2630923) |
| **Random Forest (AIPropRF)** | Ensemble of 50 trees | DP + node features | Optimal next-hop probability | Medium | RPi-class | [Chourasia et al., 2024](https://doi.org/10.1109/cybercom63683.2024.10803134) |
| **XGBoost (AIPropXG)** | Gradient boosted trees | DP + node features | Optimal next-hop probability | Medium | RPi-class | [Chourasia et al., 2024](https://doi.org/10.1109/cybercom63683.2024.10803134) |
| **NeuraLunaDTNet** | Feedforward NN (4 hidden layers) | src, dst, creation_time, epoch | Next-hop node ID | Low (~70K train steps) | Space-grade | [Patel, Radenkovic 2024](https://arxiv.org/pdf/2403.20199) |
| **GNN routing** | Graph Neural Network | Entire DP table as adjacency matrix | Per-edge forwarding probability | Medium-High | Gateway/server | [GhostWire gnn-routing](https://crates.io/crates/ghostwire-dtn) |
| **LSTM + MC Dropout** | RNN-LSTM with uncertainty estimation | Trajectory history | Node trajectory + routing uncertainty | Medium | RPi-class | [Sammou, 2026](https://doi.org/10.1109/access.2026.3655507) |

### Implementation for edge devices

```python
# Minimal on-device model for PRoPHET parameter prediction
# Input: [throughput, delay, buffer_occupancy, encounter_rate, battery]
# Output: [alpha, beta, gamma] (the 3 PRoPHET params)
import torch
import torch.nn as nn

class ProphetParamPredictor(nn.Module):
    def __init__(self):
        super().__init__()
        self.net = nn.Sequential(
            nn.Linear(5, 32), nn.ReLU(),
            nn.Linear(32, 16), nn.ReLU(),
            nn.Linear(16, 3), nn.Sigmoid()  # alpha, beta, gamma all in [0,1]
        )
    def forward(self, x):
        return self.net(x)

# At inference: takes ~50μs on Cortex-M4, runs at each encounter
```

### Using a bandit for routing (lighter than supervised ML)

[Murmuration](https://github.com/borisgraudt/murmuration) shows that **UCB1 bandit** routing (score = mean_reward + sqrt(2·ln(N)/n_i)) can match PRoPHET without any training data — it learns delivery outcomes in real-time. Their key finding: bandits hit a structural ceiling because rewards are destination-agnostic. **Q-routing** (value bootstrapping per destination) breaks that ceiling and roughly doubles delivery.

---

## 5. Comparison with Other DTN Routing Algorithms

| Protocol | Type | Learning | Copies | Delivery | Latency | Overhead | Best For |
|---|---|---|---|---|---|---|---|
| **Epidemic** | Flooding | None | Unlimited | Highest | Lowest | Highest | Small nets, low buffer |
| **PRoPHET v1** | Probabilistic | Encounter history | Unlimited (pruned) | Medium | Medium | Medium-Low | Predictable mobility |
| **PRoPHET v2** | Probabilistic | Encounter history | Unlimited (pruned) | Medium-High | Medium | Medium | Heterogeneous mobility |
| **Spray-and-Wait** | Quota-based | None | Fixed (L copies) | Medium-High | Medium | **Lowest** | Resource-constrained |
| **MaxProp** | Priority-based | Encounter history | Unlimited (prioritized) | High | Medium | Medium | Buffer-aware routing |
| **RAPID** | Utility-based | Per-packet utility | Unlimited (utility-pruned) | High | Low-Medium | Medium-High | Delay-optimization |
| **Spray-and-Focus** | Hybrid | Utility + quota | Fixed + utility | **Highest** | **Lowest** | Low | General purpose |
| **Direct Delivery** | Single-copy | None | 1 | Lowest | Highest | None | Baseline |

### Empirical findings from [2026 comparison study](https://doi.org/10.1109/icpcsn68523.2026.11543715)

Using ONE simulator with Helsinki mobility model:
- **Epidemic**: highest delivery ratio but highest overhead — flooding dominates when buffers allow
- **PRoPHET + MaxProp**: best balance of delivery vs overhead — the probabilistic pruning works
- **Spray-and-Wait**: lowest overhead through controlled replication — best for energy-constrained
- **PRoPHETv2** outperforms v1 by 8-15% in delivery in heterogeneous scenarios

### Decision matrix

```
┌──────────────────────────────────────────────────────────────┐
│  Need minimal overhead?           → Spray-and-Wait           │
│  Need max delivery at any cost?   → Epidemic                 │
│  Predictable human mobility?      → PRoPHET v2               │
│  Unpredictable / random mobility? → Spray-and-Focus          │
│  Need delay optimization?         → RAPID                    │
│  Large buffers available?         → MaxProp                  │
│  Hybrid + learning?               → PRoPHET + ML (AIPropRF)  │
└──────────────────────────────────────────────────────────────┘
```

---

## 6. Real-World Open Source Implementations

### Rust implementations

| Project | Status | Features | Link |
|---|---|---|---|
| **ghostwire-dtn** | Active (v0.1.0, 2026) | PRoPHET + Spray-and-Wait, Bloom filter sync, XChaCha20 encryption, **GNN-enhanced routing feature**, bounded buffer | [crates.io](https://crates.io/crates/ghostwire-dtn) / [GitHub](https://github.com/Phantomojo/GhostWire-secure-mesh-communication) |
| **dtn7-rs** | Active (v0.21.0) | BPv7 (RFC 9171), TCP/MTCP/HTTP/UDP CLs, epidemic/sprayandwait/static routing, IPND discovery. **External routing API** — can plug in PRoPHET as external process | [GitHub](https://github.com/dtn7/dtn7-rs) |
| **hardy** | Active (multi-crate) | Full BPv7 + BPSec, TCPCLv4+TLS, SQLite/S3 backends, gRPC, OTel, `no_std` cores, **Time-Variant Routing**, filter pipeline | [GitHub](https://github.com/ricktaylor/hardy) |
| **spacearth-dtn** | Early dev | BPv7 bundle storage, TCP CLA, modular CLA trait designed for BLE/LoRa extension | [crates.io](https://crates.io/crates/spacearth-dtn) |
| **murmuration-routing** | Active (v0.2.0) | DTN routing evaluation toolkit with UCB1 bandit, Q-routing, contact traces, foremost-journey oracle | [crates.io](https://crates.io/crates/murmuration-routing) |
| **TAIGA** | Experimental | BLE + WiFi Direct multi-transport DTN with onion routing, SOCKS5, DTN buffer with `redb` | [GitHub](https://github.com/RedBaron1914/TAIGA) |
| **dtn-mesh** (Kotlin) | Active (Android) | PRoPHET routing over WiFi Direct + BLE + LoRa (USB OTG), E2E encryption, PTT audio, fragmentation | [GitHub](https://github.com/jeremiaspala/dtn-mesh) |

### C/C++ implementations

| Project | Status | Features | Link |
|---|---|---|---|
| **IBR-DTN** | Mature (C++) | Modular routing: PRoPHET (GRTR + GTMX strategies), epidemic, flooding, static. SQLite/memory storage. Embedded-friendly (RouterBoard, Ubiquiti). **Most complete C++ PRoPHET reference.** | [GitHub](https://github.com/ibrdtn/ibrdtn) / [doxygen](https://www.ibr.cs.tu-bs.de/projects/ibr-dtn/doxygen/1.0.0/) |
| **DTN7** (Go) | Active | BPv7, modular CLA architecture, epidemic routing (routing interface can be extended to PRoPHET), MTCP, IPND | [GitHub](https://github.com/dtn7/dtn7) |
| **uD3TN** (C) | Active | µD3TN for constrained devices, LoRa CLA integration | [GitLab](https://gitlab.com/d3tn/ud3tn) |
| **NASA ION-DTN** (C) | Mature | Full BP implementation, STCP convergence layer, used in space | [GitHub](https://github.com/nasa-jpl/ION-DTN) |
| **NASA HDTN** (C++) | Active | High-rate DTN for space, TCPCLv4 | [GitHub](https://github.com/nasa/HDTN) |

### Key implementation details from IBR-DTN (C++ reference)

From the [IBR-DTN source](https://www.ibr.cs.tu-bs.de/projects/ibr-dtn/doxygen/1.0.0/ProphetRoutingExtension_8cpp_source.html):

- **Two forwarding strategies**: `GRTR` (forward if neighbor DP > local DP) and `GTMX` (same but max NF_max times per bundle)
- **Thread-safe DP map**: `DeliveryPredictabilityMap` with mutex locking
- **Aging on access**: Each `getDeliveryPredictabilityMap()` call triggers aging
- **Handshake protocol**: requestHandshake → responseHandshake → processHandshake — exchanges DP maps + acknowledgement sets + Bloom filter summary vectors
- **Configurable** via `ibrdtnd.conf`:
  ```
  prophet_p_encounter_max = 0.7
  prophet_p_encounter_first = 0.5
  prophet_p_first_threshold = 0.1
  prophet_beta = 0.9
  prophet_gamma = 0.999
  prophet_delta = 0.01
  prophet_time_unit = 1
  prophet_i_typ = 300  # 5 min expected interval
  prophet_forwarding_strategy = GRTR
  prophet_gtmx_nf_max = 30
  ```

---

## 7. Multi-Transport Operation (BLE + TCP + LoRa + Nostr)

### The core problem

PRoPHET's RFC 6693 was designed assuming TCP underneath. To run across BLE, LoRa, and Nostr simultaneously, you need a **Convergence Layer Adapter (CLA) abstraction** that decouples routing from transport.

### Architecture

```
┌──────────────────────────────────────────────────┐
│                   Application                     │
├──────────────────────────────────────────────────┤
│              Bundle Protocol Agent                 │
│         (creation, storage, dispatch)              │
├──────────────────────────────────────────────────┤
│              PRoPHET Router                        │
│   (per-transport DP tables + aggregated DP table)  │
├──────────────────────────────────────────────────┤
│         Transport Arbiter / Routing Table           │
│   (selects best CLA per destination + per bundle)  │
├─────────────┬──────────┬──────────┬───────────────┤
│   BLE CLA   │ TCP CLA  │ LoRa CLA │  Nostr CLA    │
│  (GATT/L2CAP)│(TCPCLv4) │ (serial) │ (NIP-01 relay)│
└─────────────┴──────────┴──────────┴───────────────┘
```

### Key design patterns from existing implementations

#### 1. Modular CLA Trait (spacearth-dtn, DTN7, hardy)

```rust
trait Cla: Send + Sync {
    async fn send(&self, bundle: &Bundle) -> Result<()>;
    async fn receive(&self) -> Result<Bundle>;
    fn mtu(&self) -> usize;
    fn cost(&self) -> u32;          // lower = preferred
    fn transport_type(&self) -> TransportType;
    fn is_reliable(&self) -> bool;  // TCP=true, BLE=false, LoRa=false
    fn max_bundle_size(&self) -> usize;
}
```

#### 2. Per-transport DP tables

Run **separate PRoPHET DP tables per transport interface**. A node `N` maintains:
- `P_wifi[N][dest]` — based on TCP/WiFi encounters
- `P_ble[N][dest]` — based on BLE encounters
- `P_lora[N][dest]` — based on LoRa encounters

The forwarding decision becomes: forward via transport `t` to neighbor `B` iff `P_t(B, dest) > P_t(N, dest)` **and** bundle fits the transport's MTU/budget.

This is what [dtn-mesh](https://github.com/jeremiaspala/dtn-mesh) does — it runs a single PRoPHET router but maintains separate contact histories per transport.

#### 3. Transport cost-aware routing

```rust
fn select_transport(bundle: &Bundle, destinations: &[Peer]) -> Option<TransportType> {
    // Cost function combining PRoPHET DP + transport cost
    destinations
        .iter()
        .flat_map(|peer| peer.transports.iter())
        .map(|t| (t, score_transport(t, bundle)))
        .max_by_key(|(_, score)| *score)
        .map(|(t, _)| t.transport_type())
}

fn score_transport(t: &Transport, bundle: &Bundle) -> f32 {
    let dp = t.delivery_predictability(bundle.destination());
    let cost = match t.transport_type() {
        TransportType::WifiTcp => 1.0,  // fast, cheap
        TransportType::Ble => 3.0,      // slow, battery
        TransportType::Lora => 5.0,     // very slow, limited payload
        TransportType::Nostr => 2.0,    // internet-mediated
    };
    let size_penalty = if bundle.size() > t.mtu() { 0.5 } else { 1.0 };
    dp / cost * size_penalty
}
```

#### 4. GhostWire's approach (production reference)

From [GhostWire's spec](https://github.com/Phantomojo/GhostWire-secure-mesh-communication/blob/main/docs/spec/TRANSPORT.md):

- **Transport registry** with ordered fallback: `StealthTCP → WebRTC → BLE → LoRa → Reticulum`
- **BLE sync**: Bloom filter exchange for set reconciliation (30s timeout, deferred bundles >4KB)
- **PRoPHET state exchange** is signed with Ed25519 for security
- **Predictive spray**: combines Spray-and-Wait binary spray with PRoPHET DP — if PRoPHET state is unavailable, falls back to pure binary spray

#### 5. LoRa specifics

From [DTN7's LoRa CLA](https://github.com/dtn7/dtn7-gold) and [dtn-mesh](https://github.com/jeremiaspala/dtn-mesh):

- **MTU**: LoRa packets are typically 51-241 bytes. Bundle fragmentation is mandatory.
- **Throughput**: 0.3–50 kbps. Bundle size limits: 32 KB fragments per [dtn-mesh](https://github.com/jeremiaspala/dtn-mesh)
- **Half-duplex**: LoRa is usually half-duplex. Encounters are brief and uni-directional unless you have two modules.
- **PRoPHET over LoRa**: Exchange only DP values for the top 10 destinations (not full table). Keep the RIB dictionary compact.

#### 6. Nostr integration

From [hyfmesh](https://github.com/hyfmesh/hyf):

- Nostr relays act as **store-and-forward buffers in the cloud**
- PRoPHET DP exchanges can happen as NIP-01 events on relays
- A node advertises its bundle list via Nostr events; peers respond with bundle requests
- **Key design**: Nostr is a DTN transport where the "encounter" is asynchronous — you don't need simultaneous connectivity

#### 7. Multi-transport encounter detection

```rust
// Abstract transport encounter — the heart of multi-transport PRoPHET
struct TransportEncounter {
    peer_id: [u8; 32],
    transport: TransportType,
    timestamp: Instant,
    duration: Duration,
    approx_distance: Option<f32>,
    throughput: Option<f32>,
}

// When any transport detects a neighbor, feed into PRoPHET
async fn on_transport_encounter(encounter: TransportEncounter, prophet: &mut ProphetRouter) {
    // Per-transport DP update
    prophet.update_dp_for_transport(
        encounter.peer_id,
        encounter.transport,
        encounter.timestamp,
    );
    
    // Cross-transport DP inference:
    // If we meet peer B via WiFi, increase P_lora(N, B) slightly too
    // (shared mobility signal across transports)
    for t in all_transports() {
        if t != encounter.transport {
            prophet.infer_dp_cross_transport(
                encounter.peer_id,
                t,
                0.3, // cross-transport confidence factor
            );
        }
    }
}
```

### Practical multi-transport PRoPHET configuration

```toml
# Example per-node config
[prophet]
# Per-transport typical intervals vary by 10-100x
i_typ_wifi = 300       # 5 min between WiFi contacts
i_typ_ble = 120        # 2 min between BLE contacts  
i_typ_lora = 3600      # 1 hour between LoRa contacts
i_typ_nostr = 600      # 10 min on Nostr relays

p_encounter_first = 0.5
beta = 0.9
gamma = 0.999
delta = 0.01

[prophet.forwarding]
# Forward to a better node even on a slower transport if the wait exceeds threshold
cross_transport_forward_threshold_s = 300

[prophet.aggregation]
# Merge DP tables when encounters are infrequent per transport
# Weight: WiFi=1.0, BLE=0.7, LoRa=0.4, Nostr=0.8
merge_weight_wifi = 1.0
merge_weight_ble = 0.7
merge_weight_lora = 0.4
merge_weight_nostr = 0.8
```

### The hard unsolved problems

1. **Multi-transport transitive updates**: If A meets B via BLE and B meets C via LoRa, should A's `P_ble(A, C)` or `P_lora(A, C)` change? Current implementations dodge this by keeping DP tables per-transport and only using cross-transport inference heuristically.

2. **Nostr as a transitive accelerator**: Nostr relays can act as a "meeting place" for PRoPHET DP exchange even when nodes are never in physical proximity — effectively giving every node global transitive knowledge. This breaks PRoPHET's assumption that transitive knowledge propagates through physical encounters.

3. **LoRa's "encounter" problem**: LoRa range (2-15km) means encounters last minutes to hours. The definition of "encounter duration" and "inter-encounter interval" needs rethinking for long-range radios.

---

## Quick Reference — Implementation Checklist

```rust
// Minimal PRoPHET core for a new implementation
pub struct ProphetRouter {
    dp: HashMap<[u8; 32], HashMap<[u8; 32], f64>>,  // peer -> {dest -> P}
    last_encounter: HashMap<(TransportType, [u8; 32]), Instant>,
    last_aged: Instant,
    // Config
    p_encounter_first: f64,  // 0.5
    beta: f64,               // 0.9
    gamma: f64,              // 0.999
    delta: f64,              // 0.01
    i_typ: Duration,         // transport-specific
}

impl ProphetRouter {
    fn on_encounter(&mut self, peer: &[u8; 32], dest_dps: &HashMap<[u8; 32], f64>) { /* v2 update + transitive max */ }
    fn age(&mut self) { /* P *= gamma^(dt/time_unit) */ }
    fn should_forward(&self, peer: &[u8; 32], dest: &[u8; 32]) -> bool { /* P_peer(dest) > P_self(dest) */ }
    fn handle_encounter(&mut self, peer: &[u8; 32], neighbor_dps: &HashMap<[u8; 32], f64>) {
        self.age();
        self.on_encounter(peer, neighbor_dps);
        // Also update transitive DPs using max-based v2 logic
    }
}
```

**Key papers to cite**:
- [RFC 6693](https://datatracker.ietf.org/doc/html/rfc6693) — PRoPHET specification
- [PRoPHETv2 evolution](https://doi.org/10.1145/2030652.2030661) — Grasic, Davies, Lindgren, Doria 2011
- [R-PRoPHET](https://hal.science/hal-04684901v1) — dynamic I_typ estimation, +20% delivery (2024)
- [PF-DTN LSTM routing](https://doi.org/10.1109/access.2026.3655507) — RNN-LSTM + MC Dropout for PRoPHET (2026)
- [GhostWire DTN spec](https://github.com/Phantomojo/GhostWire-secure-mesh-communication/blob/main/docs/spec/DTN.md) — Rust reference with GNN routing feature
