# Compensating Controls & Armory Model — Security Engineering Audit

- **Status:** Draft (research audit; not a code review)
- **Date:** 2026-10-27
- **Scope:** Two GhostWire architectural principles, as recorded in
  `docs/GHOSTWIRE_ARCHITECTURE_LESSONS.md` and `docs/PRIVACY_TIERS.md`:
  1. **Compensating controls** — no single feature is strong in all of
     security / privacy / anonymity; the system becomes strong by combining
     features whose weaknesses are covered by other features' strengths
     ("each layer compensates for the layer below. Defense in depth.").
  2. **The armory, not the weapon** — default to L1, escalate to L3 on demand,
     degrade gracefully; the L0–L3 privacy tier and the Normal–Blackout
     disaster level are orthogonal axes.
- **Method:** Grounded in GhostWire's own docs + ledger, cross-checked against
  published security-engineering standards (NIST) and deployed systems (Tor,
  Signal, Briar). Web sources cited inline and collected at the end.

---

## Executive summary

- **Q1 — Is the pattern well-founded?** Yes, but the label is doing two
  different jobs. What GhostWire calls "compensating controls" is, in standard
  terminology, **defense-in-depth / layered resilience** (NIST SP 800-82),
  *plus* an adjacent use of **compensating controls** in the NIST SP 800-53
  sense (a substitute control that provides equivalent protection when the
  preferred one is infeasible). Both are legitimate, established ideas. The
  audit recommends naming them separately, because they fail differently.
- **Q2 — Do others do dynamic escalation?** Yes, and GhostWire's armory maps
  cleanly onto three deployed precedents: Tor's Security Slider (discrete,
  reversible, usability-vs-security), Signal's sealed-sender toggle (a privacy
  feature with an explicit abuse trade-off, defaulting to convenience), and
  Briar's transport selection (environment-driven default + manual override +
  degrade-to-local). GhostWire's design is consistent with the state of the
  art.
- **Q3 — Do compensating controls create emergent vulnerabilities?** Yes, four
  classes: (a) **downgrade attacks** — "degrade gracefully" is itself an
  attack surface (FREAK/Logjam/POODLE); (b) **correlated collapse** — the
  compensation argument assumes independent weaknesses, but shared
  dependencies (one identity, one unwired crypto module) break several
  "independent" layers at once; (c) **false confidence / unmaintained layers**
  — more layers stops adding security once they can't all be verified; (d)
  **silent capability loss** — negotiation hides which mode was actually
  selected. **GhostWire-specific:** the covering layers in the documented
  chain (sealed sender, RNS transport, vodozemac) are currently unwired, so the
  compensation is *designed but not running* — a compensating control that
  never executes provides zero compensation.
- **Q4 — Discrete tiers or continuous?** A **hybrid** is correct, and
  GhostWire already gestures at it. Keep a *small set of discrete,
  individually-provable privacy profiles* (guarantee-bearing), but drive
  *selection continuously* from context (threat, censorship likelihood,
  battery, link quality) and allow continuous parameter scaling *within* a
  profile. Do not expose a raw continuous dial to users — it manufactures a
  continuous fingerprint and choice overload. The real split is: **guarantee
  definition = discrete; selection mechanism = continuous.**

**Bottom line:** Both principles are sound and well-aligned with deployed
privacy systems. The risk is not in the ideas but in their *current
realization*: the compensating chain is largely unwired, "degrade gracefully"
has no documented security floor, and the tiers are not yet tied to provable
anonymity guarantees. Fix the realization, keep the ideas.

---

## Q1 — Is the compensating-controls pattern well-founded in security engineering?

**Yes.** The core intuition — no single control is sufficient; combine
overlapping controls so the failure of one is covered by another — is one of
the oldest and best-established principles in the field. It goes by several
names, and GhostWire's usage actually conflates two of them, which is worth
disentangling.

### 1a. Defense in depth / layered resilience (what GhostWire mostly means)

NIST frames this as layering so that "the impact of a failure in any one
mechanism is minimized," explicitly because "a single security product,
technology or solution cannot adequately protect [a system] by itself"
(NIST SP 800-82). GhostWire's insight #4 — "Each layer compensates for the
layer below. Defense in depth." — is precisely this.

The formal preconditions for layering to actually help are the important part:
- **Independent failure modes.** Layers only add safety if they can fail
  independently. Two layers from the same dependency, sharing one key, or
  trusting one identity do *not* provide depth — they provide a single point
  of failure wearing a costume.
- **No unsecured gaps between layers.** Overlap and gaps both occur "if
  enabling techniques and methods, such as security APIs, are not standardized"
  (ScienceDirect, defense-in-depth overview).
- **Each layer must be understood and maintained.** See Q3.

### 1b. Compensating controls in the NIST SP 800-53 sense (a related but distinct idea)

NIST SP 800-53 defines a *compensating control* narrowly:

> "a management, operational, or technical control … employed by an
> organization **in lieu of** a recommended security control … that provides
> **equivalent or comparable protection** … only under the following
> conditions: (i) the compensating control is selected from the catalog (or
> adopted if none fits); (ii) the organization provides a complete and
> convincing rationale for how it provides equivalent protection and why the
> baseline control could not be employed; (iii) the organization **assesses and
> formally accepts the risk**." (NIST SP 800-53 R1–R4)

This is *substitution under constraint* (the preferred control is infeasible,
so you provide something equivalent), not *stacking for redundancy*. It is a
governance discipline with a hard requirement: **supporting rationale + formal
risk acceptance, documented in the security plan and approved by an
authorizing official.**

### Why the distinction matters for GhostWire

GhostWire uses "compensating controls" to mean **both**:
- "mDNS (weak anonymity) → covered by RNS encrypted announces" — that's
  *layering/defense-in-depth* (two controls jointly cover a threat).
- The armory's "L1 covers what L3 doesn't need" — that's closer to *selection*.

Neither is wrong, but the audit recommends GhostWire adopt precise language,
because the two have different failure modes and different proof obligations:

| | Defense in depth (stacking) | Compensating control (substitution) |
|---|---|---|
| Goal | Redundancy; one failure covered by another | Equivalent protection when preferred control is infeasible |
| Proof needed | Independence of failure modes | Equivalence + documented risk acceptance |
| Typical failure | Correlated collapse (Q3a) | Unjustified equivalence claim; unmaintained substitute |

**Verdict (Q1):** The pattern is well-founded and standard. GhostWire should
keep it, but (1) call the stacking "defense in depth," reserve "compensating
control" for genuine substitutions, and (2) for every compensating layer,
write down the NIST-style rationale: *which threat it covers, why the primary
control can't, and that the risk is accepted.* GhostWire's per-feature
strength/weakness ledger (insight #9) is already the right seed of this — it
just needs the equivalence rationale attached.

---

## Q2 — How do other systems implement dynamic privacy escalation?

Three deployed systems cover the three designs GhostWire is choosing between.
All three confirm the armory instinct (default middle, escalate on demand,
degrade gracefully); each also models a different failure of it.

### Tor — discrete, reversible, usability-centric slider

- **Model:** 3 discrete levels today — **Standard** (all features on, the
  default), **Safer** (JS disabled on non-HTTPS, WebGL/media click-to-play),
  **Safest** (JS disabled everywhere, images/fonts/math symbols restricted).
  The slider trades *usability for security*, ranked by vulnerability count and
  web prevalence. It is fully **reversible** ("undo at any time").
- **Origin:** Proposed by Roger Dingledine / iSEC Partners as a way to disable
  features "ranked by both vulnerability count and web prevalence/usability
  impact." iSEC originally sketched four bands (Low / Medium-Low /
  Medium-High / High); it shipped as three.
- **Lesson for GhostWire:** Discrete + reversible + *default-to-convenient* is
  the proven UX. Note that Tor's slider is mostly about **attack-surface
  reduction and fingerprinting**, not about the anonymity set per se — a
  reminder that "security level" and "privacy/anonymity level" are different
  axes and GhostWire should keep them distinguishable.

### Signal — one privacy feature, an explicit abuse trade-off, default = convenience

- **Model:** Content is *always* end-to-end encrypted (no tier there). The
  tiered axis is **sealed sender** (who-sees-who metadata). Introduced 2018;
  uses a 96-bit **delivery token** derived from the profile key, so
  sealed-sender messages only arrive from established contacts who know your
  token — this is the abuse protection. In Settings → Privacy → Advanced there
  is a **"Sealed Sender: Everyone / Nobody"** toggle: enabling "Everyone" lets
  non-contacts send sealed-sender messages (more private metadata-wise) **at
  increased spam/abuse risk**. Separately, contact *discovery* defaults to
  "Everyone" for convenience and can be set to "Nobody."
- **Lesson for GhostWire:** Signal shows the cleanest version of the armory
  claim "don't force maximum privacy." The maximum-privacy option exists, is
  one toggle away, and is *explicitly labeled with its cost*. GhostWire's
  sealed-sender design should copy this pattern: make the strong option
  one-click, and make its trade-off (spam, discovery exposure) legible rather
  than hidden. (Note: GhostWire's own ledger records sealed sender as
  compiling with **zero callers**, no AAD, no replay/expiry — see Q3d.)

### Briar — environment-driven transport selection, degrade-to-local

- **Model:** Transport is the tier. **Tor** when the internet is up (hides who
  talks to whom); **Bluetooth / Wi-Fi** (~10 m) when it's down (blackout);
  **Mailbox** store-and-forward for asynchronous contacts. Connection mode:
  **Automatic based on location / Tor without bridges / Tor with bridges /
  Don't connect**, plus "only on Wi-Fi" and "only while charging."
- **Lesson for GhostWire:** Briar is the closest analog to the armory +
  disaster-level orthogonality. Its "Automatic based on location" is the
  context-driven *default*; the manual modes are the *escalation*; Bluetooth/
  Wi-Fi are the *graceful degradation*. GhostWire's "RNS primary when IP is
  down, libp2p bootstrap when up" (insights #1–#2) is the same shape. Caveat:
  Briar is now in **maintenance mode**, and its local transports trade long
  range for proximity disclosure — the same proximity leak GhostWire's own
  ledger flags for BLE/Wi-Fi discovery.

### Synthesis

| System | Tier axis | Default | Escalation | Degradation | Reversible |
|---|---|---|---|---|---|
| Tor | Attack surface / fingerprint | Standard | Slider up | Slider down | Yes |
| Signal | Metadata (sealed sender) | Convenience | Toggle "Everyone" | Toggle "Nobody" | Yes |
| Briar | Transport / connectivity | Auto by location | Bridges / Tor | Bluetooth / Wi-Fi | Yes |
| **GhostWire (designed)** | **Privacy params + disaster level** | **L1** | **→ L3 on demand** | **battery/network down-shift** | **Yes (by design)** |

GhostWire's design is **consistent with the state of the art.** The gaps are
realization, not concept (see Q3d, Q4).

---

## Q3 — Do compensating controls create emergent vulnerabilities?

Yes. Layering/compensation is not free. Four emergent failure classes, then the
GhostWire-specific realization gap.

### 3a. Downgrade attacks — "degrade gracefully" is an attack surface

Any system that negotiates "the best mode both sides support and otherwise
fall back" hands an active adversary a lever: **force the fallback.** This is
the textbook downgrade attack behind **POODLE, FREAK, Logjam, BEAST** —
"algorithm agility opens up opportunities for downgrade attacks" (Microsoft
Research, *Downgrade Resilience in Key-Exchange Protocols*). The fix is not to
remove fallback but to make it **security-relevant**: a hard floor below which
the connection *refuses* rather than degrades, downgrade-protection sentinels
(TLS 1.3), `TLS_FALLBACK_SCSV`, and **logging the negotiated mode** so silent
downgrades surface. The abstractopedia mechanism note is blunt: "Its central
weakness is the fallback that makes it robust … permissive fallback also hides
*silent* capability loss."

> **GhostWire implication:** "Degrade gracefully when resources are constrained"
> is exactly the negotiable-fallback pattern that gets attacked. An adversary
> who can make RNS / Sphinx / sealed-sender *appear* unavailable (jam the
> channel, drop QUIC, spoof "no bridge") can push a target to L0/L1 and strip
> anonymity while content encryption survives — a targeted metadata harvest.
> **Recommendation:** define a **non-negotiable security floor** (e.g.,
> content E2E-E and a minimum anonymity guarantee that battery/network
> adaptation may *raise* but never *lower*); bind tier negotiation to an
> authenticated, signed channel so a peer can't forge "capability absent";
> log and alert on tier changes; and distinguish *resource-driven* downgrade
> (allowed) from *adversary-forced* downgrade (detected, not silent).

### 3b. Correlated collapse — the independence assumption is the hidden risk

The compensation argument ("weakness of A covered by strength of B") is only
valid if A's and B's weaknesses are **independent**. Real systems share hidden
dependencies, so multiple "independent" layers fall together:
- Shared **identity** (one DID/PeerId across transports) — GhostWire's ledger
  already flags DID–PeerId correlation as a known defect and cross-transport
  discovery as a link between RNS address and PeerId.
- Shared **crypto module / key material** — if sealed sender, vodozemac, and
  the custom X25519 path all draw on one key store, one compromise defeats
  every "layer."
- Shared **vendor/dependency** — NIST and practitioners both note correlated
  failure from a shared dependency ("single-vendor collapse").

When layers share a root, "defense in depth" silently becomes "one defense,
counted N times." **Recommendation:** for each compensating pair, explicitly
state the **independence assumption** and what would violate it. Treat a single
identity or single key store as a *correlation point to be minimized*, not a
convenience.

### 3c. False confidence and unmaintained layers — complexity is the enemy

The practitioner literature is consistent and warning-level: "at some point,
'more layers' stops meaning 'more secure' and starts meaning slower response,
higher operational risk … you have entropy." Blind spots appear where "you
think you mitigated that risk, but you haven't maintained a solid presence
there." "Clever attackers might use [interactions between layers] to render
some of an organization's layered security ineffective." The failure mode is
**confidence without verification**: the more layers claim to cover a threat,
the easier it is for *no one* to own it.

> **Recommendation:** every compensating layer needs an **integration test
> that fails if the layer is removed** (GhostWire's own "The Rule" already
> mandates this — enforce it as the gate for claiming compensation) and a
> named owner. Map each threat → the specific layer(s) covering it → the test
> that proves the coverage. If a threat's only coverage is an unwired module,
> the threat is **uncovered**, full stop.

### 3d. Silent capability loss + the GhostWire realization gap

The single most important finding for GhostWire is not theoretical. The
documented compensating chain is:

> "RNS (weak anonymity) → carries **sealed sender** envelopes → sealed sender
> hides sender; Sphinx (weak anonymity at 2 hops) → adds multi-hop
> unlinkability."

GhostWire's own truth ledger and security-claims audit record that, as of the
current code:
- **Sealed sender compiles with zero callers**, no AAD binding, no
  replay/expiry, `pq_key` always `None` → **DM metadata still leaks.**
- **RNS is initialized but does not carry traffic**; libp2p carries 100% of
  messages. Cross-transport discovery was only recently fixed to exchange a
  real RNS address, and RNS-discovered peers still don't sync into the main
  topology.
- **vodozemac Olm/Megolm exists but is not in the core module graph**; at-rest
  protection unverified; active path uses custom X25519.
- **Sphinx min-hops = 2** (weak); hop identifiers are real PeerIds (a privacy
  leak).

**A compensating control that never executes provides zero compensation.** The
chain above is *designed* but several of its links are *unwired*, so the
system currently gets the *complexity cost* of layering (Q3c) without the
*resilience benefit*. This is the canonical emergent vulnerability: the
architecture doc and README imply depth that the module graph does not yet
deliver. **Recommendation:** until a layer is proven end-to-end by the node-to-
node test, mark it **"designed, not compensating"** in the ledger; do not let
the compensating-controls narrative outrun the wiring.

---

## Q4 — Is the L0–L3 tier model sufficient, or should it be continuous?

**A hybrid is correct — and it is the direction the field is moving.** The
tension dissolves once you separate two questions that "discrete vs continuous"
conflates:

1. **How are guarantees *defined*?** → Keep **discrete**.
2. **How is a level *selected* and *tuned*?** → Make it **continuous / context-
   driven**.

### Why guarantees should stay discrete

- **Provability.** An anonymity claim is only as good as the proof behind it.
  It is far easier to *prove* "at L3, cover-traffic ratio ≥ 50%, min Sphinx
  hops = N, sealed sender on ⇒ this anonymity set holds" than to prove a
  property over a continuous parameter space. Tor deliberately uses a few
  discrete levels rather than a dial, in part for **fingerprinting resistance**:
  a continuous setting is itself a continuous fingerprint.
- **Testability & auditability.** Discrete profiles are each individually
  testable; the combinatorial space of a continuous dial is not.
- **Predictability for the user.** Four named tiers ("Daily / Sensitive /
  High-Risk") are decisionable; a slider of 1,000 positions is choice overload.

### Why *selection* should be continuous

- **The world is continuous.** Battery %, link latency, censorship likelihood,
  and neighbor density are analog. GhostWire already models this — battery
  "reduce cover traffic by 30%," "downgrade one level below 20%," "satellite →
  halve mixing delay." That is continuous *tuning layered on top of* discrete
  levels, which is exactly the right shape.
- **The adaptive-security literature points this way.** Risk-based / adaptive
  access control (RBAC/ABAC → adaptive) scores context in real time and
  **steps controls up or down mid-session**, revoking or tightening when the
  risk score crosses a threshold. The consensus is "continuous monitoring,
  dynamic re-evaluation," not a one-time discrete pick.

### The synthesis: discrete profiles, continuous selection, bounded tuning

- Define a **small set of privacy profiles** (L0–L3), each with a **documented,
  provable guarantee** and the anonymity set it buys.
- Drive **selection continuously** from context (threat/censorship signal,
  battery, link quality) — this is GhostWire's device/battery/network
  adaptation, promoted from ad-hoc tweaks to the primary selection engine.
- Allow **continuous parameter scaling *within* a profile** (cover-traffic
  ratio, mixing delay) but **never below that profile's security floor** (ties
  back to Q3a).
- **Do not expose a raw continuous dial to end users.** Surface the profile
  name and let the engine tune within it. This preserves fingerprinting
  resistance and avoids choice overload while still getting the benefit of
  fine-grained adaptation.
- Keep the **two-axis model** (privacy tier × disaster level) that insight #5
  already proposes — it is a good decomposition, because censorship state
  (disaster level) and device/resource state (which drives the tier) genuinely
  vary independently.

**Verdict (Q4):** The L0–L3 model is **sufficient as the guarantee layer but
insufficient as the whole story.** Add a continuous, context-driven *selection
and tuning* layer on top, bound it by per-profile floors, and keep the user
facing discrete names. That is both the theoretically sound answer and the one
GhostWire is already half-building.

---

## Consolidated recommendations

1. **Separate the two "compensating" meanings.** Call stacking "defense in
   depth"; reserve "compensating control" for NIST-style substitutions, and
   attach the NIST rationale (threat covered + why primary is infeasible +
   accepted risk) to each.
2. **Make degradation security-relevant.** Define a non-negotiable floor
   (content E2E-E + minimum anonymity guarantee) that resource-driven
   adaptation may raise but never lower; authenticate tier negotiation so
   adversary-forced downgrades are *detected and logged*, not silent.
3. **State independence assumptions.** For each compensating pair, name the
   correlation point (shared identity, shared key store) that would collapse
   it, and treat single identity / single key store as risks to minimize.
4. **Wire or withdraw the claims.** Until sealed sender, RNS transport, and
   vodozemac are proven by the node-to-node test, label them "designed, not
   compensating." A compensating control that never runs is a liability, not
   depth.
5. **Keep discrete guarantees, make selection continuous.** Discrete,
   provable profiles; continuous context-driven selection and in-profile
   tuning; bounded by per-profile floors; user sees names, not a dial.

---

## Sources

- NIST SP 800-53 R1–R4 — Compensating controls definition and conditions
  (equivalent protection; rationale + risk acceptance; security-plan
  documentation). https://nvlpubs.nist.gov/nistpubs/specialpublications/nist.sp.800-53r4.pdf
- NIST SP 800-82 — Defense-in-depth: layering so a single mechanism's failure
  is minimized; no single product suffices.
  https://nvlpubs.nist.gov/nistpubs/specialpublications/nist.sp.800-82r1.pdf
- Tor Browser — Security Levels (Standard / Safer / Safest), reversible,
  usability-vs-security. https://support.torproject.org/tor-browser/features/security-levels/
- Tor Browser — Security Slider design origin (iSEC Partners hardening study;
  features ranked by vulnerability count and prevalence; four-band proposal).
  https://blog.torproject.org/isec-partners-conducts-tor-browser-hardening-study/
- Signal — Sealed sender technology preview (delivery token, contacts-only by
  default, "everyone" option at abuse risk). https://signal.org/blog/sealed-sender/
- Signal — User guide (Sealed Sender Everyone/Nobody toggle; discovery
  defaults). https://aboutsignal.com/signal-messenger-user-guide/
- Briar — How it works / manual (Tor / Bluetooth / Wi-Fi / Mailbox; connection
  modes incl. automatic-by-location; now in maintenance mode).
  https://briarproject.org/how-it-works/ ; https://briarproject.org/manual/
- Downgrade attacks — definitions, FREAK/Logjam/POODLE, TLS 1.3 downgrade
  protection, TLS_FALLBACK_SCSV. https://cyberdefers.org/cybersecurity-glossary/downgrade-attack/ (cf.
  Microsoft Research, *Downgrade Resilience in Key-Exchange Protocols*,
  https://www.microsoft.com/en-us/research/wp-content/uploads/2016/07/Downgrade-Resilience-in-Key-Exchange-Protocols.pdf)
- Protocol negotiation — "fallback is the weakness that makes it robust,"
  silent capability loss, hard floor + negotiation logging.
  https://abstractopedia.org/mechanisms/protocol_negotiation/
- Layered security dark side — complexity, correlated/shared-dependency
  failure, unmaintained layers, blind spots.
  https://www.csoonline.com/article/553599/the-dark-side-of-layered-security.html
- Defense-in-depth overview — overlap/gap risks, standardization of security
  APIs. https://www.sciencedirect.com/topics/computer-science/layered-security
- Adaptive / risk-based access control — continuous context-driven selection,
  step-up/down, mid-session revocation (PrivGuardAI; MDPI fog-IoT; Ping
  Identity; PD-ZTA).
  https://link.springer.com/article/10.1007/s10791-026-10472-8 ;
  https://www.pingidentity.com/en/resources/identity-fundamentals/authorization/authorization-methods.html

## GhostWire primary sources

- `docs/GHOSTWIRE_ARCHITECTURE_LESSONS.md` — standing constraint; insights #4
  (compensating controls), #5 (armory), #9 (per-feature strength/weakness).
- `docs/PRIVACY_TIERS.md` — L0–L3 definitions, params, device/battery/network
  adaptation, implementation status.
- `docs/DEVICE_TIERS.md` — device classes → privacy mapping.
- `GHOSTWIRE_LEDGER.md` and the `ghostwire-security-claims-and-advisories`
  audit — current wiring status (sealed sender zero callers, RNS not carrying
  traffic, vodozemac unwired, Sphinx 2-hop, DID–PeerId correlation).
