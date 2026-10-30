# Math & Physics Audit — GhostWire

**Date:** 2026-10-28
**Session:** `local-65b7204d944e980fa9ced0b5e32e95b2dd78672e353a64793a653ae27243dc20` (Voice and speech setup options → GhostWire deep work)
**Canonical home base:** `/home/phantomojo/rescue/gw-push/ghostwire` (live line of record, branch `rescue/sep-disk-tree`)
**Companion ledger:** `~/GHOSTWIRE_LEDGER.md`
**Standing rule (agreed 2026-10-28):** every idea, research thread, and design discussion that touches GhostWire goes into the ledger (or is referenced from it) so work is not lost across sessions. This doc is the math/physics counterpart to `RESILIENCE_SPEED_AUDIT.md` and `PRINCIPLES_AUDIT.md`.

---

## 0. Purpose

GhostWire already applies a lot of math (lattice crypto, Bayesian transport selection, ML routing cascade, AIMD, XOR metric spaces, onion routing). This audit asks: **which mathematical and physical tools are relevant to a resilient, anonymous, anti-censorship mesh that GhostWire is NOT using yet**, ranked by payoff, with physics (radio/EM) included because GhostWire's roadmap now spans Reticulum, BLE, WiFi, and LoRa-adjacent work.

Not a literature review. Actionable gaps only.

---

## 1. What GhostWire already applies (do not redo)

| Area | Where | Form |
|---|---|---|
| Lattice / PQC crypto | `encryption.rs`, `pq_key` | ML-KEM-768 (FIPS 203) |
| ECC / hashing | core | X25519, Ed25519, HKDF (label-separated per Briar audit) |
| KDF / password hardening | identity path | Argon2id |
| Bayesian optimization | `transport_selector.rs` | Thompson Sampling (DORS taxonomy) |
| ML routing cascade | `ROUTING.md` L1–L4 | LightGBM → Gemma → GNN → PRoPHET |
| Congestion control | retransmission | AIMD (1s→30s, RTT smoothing) |
| Metric space routing | `skademlia` | XOR distance |
| Onion routing | `sphinx.rs` (2-hop min, noted weak) | Sphinx |
| Anonymous reply | core | SURB |
| State machine | `TRANSPORT.md` | Disaster Levels Normal→Blackout |
| DTN policy | `DTN.md` | BundlePriority, TTLs |

---

## 2. Physics — radio / EM / propagation (the area Michael hasn't explored due to no hardware)

GhostWire's roadmap now includes Reticulum, BLE, WiFi, and (via research) LoRa-adjacent transports. The physics below governs all of them; none of it is modeled anywhere in the codebase today.

### 2.1 Link budget
`P_RX = P_TX + G_sys − L_sys − L_channel − M` (fading margin). A link is **link-limited** when channel loss drops received power below the SNR needed for correct demodulation, and **interference-limited** when noise floor / other transmitters dominate. GhostWire's transport selector treats transports as abstract "available/unavailable" — it has no notion of *why* a link degraded or whether more TX power would help. Relevant for `transport_selector.rs` and the RNS interface work.

### 2.2 Path loss models
- **Free-space:** `32.45 + 20 log d[km] + 20 log f[MHz]` (inverse-square).
- **2-ray / ground reflection:** `120 − 20 log(h_TX h_RX) + 40 log d` beyond critical distance — fourth-power loss. This is why low antennas kill range.
- **Log-normal shadowing:** `N(0, σ)` dB, σ≈3–8 dB depending on environment. LoRa opportunistic routing (Roesler et al.) uses a LoS model `P_pl = 56 + 20 log f + 20 log d + shadowing_TX + shadowing_RX` with 10 m grid shadow maps.
- **Okumura-Hata / COST231 / Walfish-Ikegami:** empirical urban models (800–2000 MHz, 20 m–5 km, LOS/NLOS).

**Gap:** GhostWire's RNS/BLE/WiFi design docs describe framing and MTU but never a propagation model. Any "range / meshing" claim is ungrounded without one.

### 2.3 Thermal noise & Shannon capacity
`N_0 = NF · k · T` (noise floor), reception sensitivity `S_r = SNR · N_0 · BW`, link budget `L_budget = P_TX / S_r`. Shannon: `C = B log₂(1 + SNR)`. LoRa trades spreading factor (SF) for sensitivity: higher SF = lower data rate but better SNR margin. **Dynamic coding rate** (MeshCore roadmap item) is exactly this tradeoff automated. GhostWire has no link-rate adaptation; `transport_selector` picks transports but not coding rates.

### 2.4 Interference, fading, Doppler
Multipath, Doppler, and interference are absorbed into a fading margin `M`. For mobile nodes (phones, Elitebook on Tailscale, Android Tauri client) Doppler and multipath matter. No margin is modeled anywhere.

### 2.5 What to do with this (low cost, high clarity)
- Add a **link budget / propagation model section** to `RNS_BLE_WIFI_INTERFACE_DESIGN.md` and the transport docs, with the formulas above and a stated assumption set (antenna height, environment, frequency). This turns "meshing" claims into checkable numbers.
- Treat **dynamic coding rate** as a transport-selector output (not just transport choice), mirroring MeshCore's roadmap. This is the math behind "resilience vs speed" on radio links.

---

## 3. Math — not applied, ranked by payoff

### P0 — high value, concrete, fits existing gaps

**3.1 Erasure / fountain coding (Reed-Solomon, LT, Raptor) for DTN bundles.**
GhostWire's ledger headline: *"WORKING CRYPTO CORE, NON-FUNCTIONAL MESSAGE PATH — not one payload has ever been delivered."* AIMD retransmits whole bundles. Fountain codes (rateless: spray coded packets, recover from any k of n) eliminate per-packet ACKs and tolerate arbitrary loss — directly addressing delivery reliability on lossy radio/DTN paths. LT: degree drawn from robust soliton distribution, XOR encoding, peeling decoder. Raptor: LT + outer code for linear-time decode. **Caveat:** fountain codes need large block sizes for near-optimal rates; GhostWire's small messages may favor Reed-Solomon or simple XOR (COPE-style) network coding instead. Research needed on block-size fit.

**3.2 Formal verification (TLA+) of the message path.**
GhostWire has a "truth ledger" and "cockpit-truth" but no machine-checked invariants. A small TLA+ spec of the handshake + DHT + message delivery would catch the "no message delivered" class of bug before code. Pattern: signal-fish-server verified session state machine with TLC (8,814 states), reconnect exposure bound `reconnectExposure ≤ QCAP + PCAP + BURST + RATE·WINDOW`. Tamarin/ProVerif for the crypto handshake (Dolev-Yao adversary). **Cost:** spec-first, low; payoff: high for a protocol making security claims.

**3.3 Shamir threshold secret sharing for genesis / key distribution.**
GhostWire's IFAC seed (`~/.ghostwire/ifac_seed`) and "genesis ceremony" / successor-transfer discussion (Oct 27) map directly to (k,n) threshold schemes: split root secret into n shares, any k reconstruct, k−1 reveal nothing (information-theoretic). Mesh literature (Shamir 1979, wireless mesh key management) uses this for master-key distribution without a single trusted authority. Pairs with your "black box genesis, but not unchangeable forever" requirement: rotate via threshold re-sharing.

### P1 — medium value, design-time

**3.4 Hyperbolic greedy routing (Kleinberg).**
GhostWire uses skademlia (flat XOR metric). Hyperbolic embedding gives 97%+ greedy success with ~6–10% path stretch on scale-free graphs, using only local neighbor coordinates — no global routing table. Relevant to the L1→L4 cascade's fallback and to scaling beyond current peer counts. Cost: embedding computation; benefit: locality + scalability. Probably not needed until peer counts grow; note as future.

**3.5 Spectral graph theory for mesh health.**
Eigenvalues of adjacency/Laplacian: algebraic connectivity (Fiedler value) = robustness to partition; spectral gap = gossip mixing time. Formalizes "is this mesh well-connected or fragmenting" for the cockpit-truth view instead of heuristics. Low implementation cost if you already compute neighbor sets.

**3.6 Differential privacy for telemetry/metadata.**
GhostWire's lenses claim privacy but there's no formal ε-bound on aggregate reporting (peer counts, transport stats, health). Local DP (perturb before transmission) + a privacy budget ledger (ε ∈ [0.5,1.0] starting point) would make "privacy" auditable rather than asserted. Especially relevant if GhostWire ever reports fleet stats or runs a directory. MIC-DP for correlated features.

**3.7 Control theory beyond AIMD (BBR-style).**
AIMD is 1s→30s exponential backoff. BBR models bottleneck bandwidth × min RTT (BDP) and paces to fill the pipe without persistent queue buildup. Smoother, less timing-leaky (relevant to `ReliabilityMode::Anonymous` — fixed intervals to avoid timing leakage). Direct upgrade candidate for `retransmission`/AIMD code.

**3.8 Queueing theory (Little's law).**
DTN TTLs are constants (`DTN.md`). Little's law `L = λW` gives principled backlog bounds: derive TTL/buffer limits from arrival rate × service time instead of guessing. Needed before high-load testing.

### P2 — lower priority / situational

**3.9 Network coding (COPE-style XOR at forwarding layer).**
Relay nodes combine packets (`c1 ⊕ c2`) to reduce transmissions. Gains 5–70% depending on traffic pattern, but adversarial settings risk pollution attacks (detection probability must exceed ~0.85 to beat routing). GhostWire's anonymity/threat model makes pollution detection hard. **Recommend: skip unless a specific throughput bottleneck appears; if adopted, needs homomorphic-ish verification or per-packet MACs.**

**3.10 Zero-knowledge proofs.**
Prove "valid GhostWire peer" / "valid IFAC holder" without revealing identity. Natural fit for anonymous intro and IFAC gating, but heavy (SNARK/STARK). Track; not now.

**3.11 Steganographic timing / covert channels.**
Anti-censorship: hide traffic in benign timing (mimic HTTPS cadence). Your obfuscation research lists padding/timing-chaff but no formal information-hiding math. Medium-term; pairs with the "speed optimizations are metadata leaks" insight from `RESILIENCE_SPEED_AUDIT.md`.

**3.12 Game theory / mechanism design for relaying.**
GhostWire assumes altruistic forwarding. At scale, a light accounting/reputation scheme (not blockchain) could matter. Low priority until network grows.

---

## 4. What NOT to apply (skip)

- **Random matrix theory, compressive sensing, chaos/dynamical systems** — no clear protocol payoff at GhostWire's scale.
- **Homomorphic encryption** — too costly; threat model doesn't need computation on ciphertexts.
- **More ML beyond the routing cascade** — risks the "1859 research vs 291 build" imbalance you've flagged; the cascade already exists.
- **Network coding in adversarial settings** (see 3.9) — pollution risk outweighs gain for an anonymity-focused mesh.

---

## 5. Cross-reference: MeshCore lessons (no hardware needed)

From the MeshCore analysis (Oct 28): role-based repeating discipline (companions don't repeat; dedicated repeaters do; bounded opt-in repeating), transport codes for zoning/filtering, bridge-mode standardization, remote management over LoRa, KISS serial bridge, V2 protocol spec discussion. These are **policy/protocol** lessons, not math, but they interact with 2.5 (dynamic coding rate) and 3.1 (erasure coding) and should be considered alongside.

---

## 6. Recommended next steps (ordered)

1. **P0-a:** Add link budget + propagation model section to `RNS_BLE_WIFI_INTERFACE_DESIGN.md` (physics, §2).
2. **P0-b:** Research erasure/fountain coding fit for GhostWire's bundle sizes; write a short design note (replacing AIMD retry with k-of-n shards on DTN path). Decide RS vs LT/Raptor vs COPE-XOR by block size.
3. **P0-c:** Draft a TLA+ spec for the message path (handshake → DHT → delivery) with invariants; run TLC on a bounded model. This directly targets "no message delivered."
4. **P0-d:** Design (k,n) threshold sharing for the IFAC seed / genesis secret; document rotation via re-sharing.
5. **P1:** BBR-style congestion control for the transport layer; spectral health metric; Little's law for DTN buffers.
6. **Track (not now):** hyperbolic routing, DP telemetry, ZK, steganographic timing, game-theoretic relaying.

---

## 7. References (primary sources consulted)

- Erasure/fountain codes: arXiv 1305.0918; MacKay 2005 fountain codes; Luby 2002 LT; Shokrollahi 2006 Raptor; ACM SIGCOMM 1998 digital fountain.
- Network coding: Ahlswede et al. 2000; COPE (Katti et al., MIT); Purdue empirical study (Infocom 2009); arXiv 1801.02119 (queueing analysis); adversarial analysis (ScienceDirect 2014).
- Hyperbolic routing: Boguñá et al. 2010; Kleinberg; HPI (Friedrich) embeddings; Nature Sci. Rep. 2023 greedy optimization.
- Formal verification: TLA+ / TLC (signal-fish-server formal-verification.md); Tamarin/ProVerif for crypto protocols; ADR-0003 rationale.
- Secret sharing: Shamir 1979; ITU-T Y.3808; wireless mesh key management (IntechOpen, Sabancı, IFIP).
- Differential privacy: (ε,δ)-DP; local DP; MIC-DP; DAP (Cloudflare); MDPI 6G privacy.
- BBR: Cardwell et al.; IETF draft-ietf-ccwg-bbr; Stanford CS244; Wi-Fi 6 BBRv3 study (arXiv 2512.18259).
- Radio physics: Semtech AN1200.22 link budget; LoRa opportunistic routing (Roesler 2023); Campbell Sci link budget; Okumura-Hata / COST231; Shannon capacity.
- MeshCore: github.com/meshcore-dev/MeshCore (README, roadmap, examples).

---

*Standing rule: future ideas, research threads, and design discussions touching GhostWire are appended to `~/GHOSTWIRE_LEDGER.md` (and referenced here) so work is not lost across sessions.*