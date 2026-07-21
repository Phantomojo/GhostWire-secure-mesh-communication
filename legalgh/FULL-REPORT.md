# GhostWire Legal & Governance Intelligence Report

**Date**: July 15, 2026
**Project**: GhostWire Secure Mesh Communication
**Author**: Michael Irungu Muriithi, Open University of Kenya

---

## Table of Contents

1. [Tech Landscape 2026](#1-tech-landscape-2026)
2. [Open Source Licensing Analysis](#2-open-source-licensing-analysis)
3. [Law & Governance — How It Affects GhostWire](#3-law--governance)
4. [IP Protection & Idea Theft](#4-ip-protection--idea-theft)
5. [GitHub vs Self-Hosted](#5-github-vs-self-hosted)
6. [Kenya Copyright & Software Law](#6-kenya-copyright--software-law)
7. [OIN & LOT Network — Patent Defense](#7-oin--lot-network)
8. [Competitor Legal Docs Analysis](#8-competitor-legal-docs-analysis)
9. [Priority Actions](#9-priority-actions)

---

## 1. Tech Landscape 2026

### Competitive Position

GhostWire occupies a **unique position** in the 2026 P2P mesh landscape. No single project combines all five differentiators:

| Capability | GhostWire | libp2p | Reticulum | Meshtastic | I2P | Nym |
|---|---|---|---|---|---|---|
| AI routing (4-layer ML) | **Only one** | ❌ | ❌ | ❌ | ❌ | ❌ |
| Sphinx onion + PQ crypto | ✅ | ❌ | ❌ | ❌ | ✅ (garlic) | ✅ |
| DTN (PRoPHET) | **Only one** | ❌ | ❌ | ❌ | ❌ | ❌ |
| Multi-transport failover | ✅ | ✅ | ✅ | ❌ | ❌ | ❌ |
| Coercion resistance | **Only one** | ❌ | ❌ | ❌ | ❌ | ❌ |

### Key Competitors

- **Meshtastic** (~4M nodes, LoRa only, hitting scalability limits at ~20 active nodes)
- **MeshCore** (3.2K stars, MIT license, directed routing, eating Meshtastic's lunch for larger meshes)
- **Reticulum** (6K stars, closest transport-layer competitor, but Python-only, single-maintainer, no AI)

### Commoditized Features (no longer differentiation)

- Encryption by default — every project has this now
- Kademlia DHT — libp2p standard
- Multi-hop — all meshes have this
- Open source — everyone is

### Market Trends

1. **Censorship Resistance** (Highest Growth Vector): 45% YoY increase in government-ordered internet blackouts (APNIC, Feb 2026). Mesh networking is the only countermeasure that works at the physical layer.

2. **Disaster Response & Emergency Comms**: FEMA, Red Cross, UN OCHA actively procuring mesh networking kits. Meshtastic is the default field tool due to low cost ($30/node).

3. **Military/Tactical**: US Army NGC2 / Project Convergence — Anduril's Lattice Mesh deployed across 4th Infantry Division (April-May 2026). 2,500+ soldier devices as mesh nodes. Commercial MANET hardware sells for $5K-50K/node.

4. **Post-Quantum Cryptography — NOW production**: AWS turned on ML-KEM hybrid TLS by default (April 2026). I2P shipped PQ crypto in 2.11.0. GhostWire already uses hybrid X25519+ML-KEM-768 by default.

5. **AI/ML Routing — Academic to Production**: GhostWire is the only project with a production 4-layer ML pipeline. Academic papers (GraphSAGE-MAPPO, GROF) are still simulation-only.

6. **DTN Revival**: Driven by UAV/FANET research, 6G standardization, disaster comms procurement. PRoPHET is the most widely studied protocol.

### Threats

| Threat | Timeline | Severity |
|--------|----------|----------|
| libp2p adds PQ + DTN natively | 1-2 years | HIGH |
| Reticulum gets a Rust port | 2-3 years | MEDIUM |
| Gemma/LLM cost drops to zero | 1 year | MEDIUM |
| 6G native mesh/multicast | 3-5 years | HIGH |
| Single-maintainer dependency | Ongoing | HIGH |

### Funding Landscape

- **NLnet NGI0 Commons Fund**: Largest funding source for open P2P infrastructure. Restack program (€7M, 2026-2030) is an exact fit for GhostWire.
- **GhostWire would be an excellent NLnet candidate** — maps perfectly to "protocols and interoperability" and "local-first infrastructure."

---

## 2. Open Source Licensing Analysis

### Your Current Model: AGPL-3.0 + Commercial

This is **exactly right** and follows the proven playbook (MongoDB, MinIO, Grafana, Elastic — who returned to AGPL after trying BSL).

### Key Findings

- **AGPL perception softened** in 2026. BSL/SSPL fatigue making AGPL look reasonable. It's now the strongest copyleft that OSI approves.
- **Protocol vs Implementation split**: Protocol specs should be Apache 2.0/MIT for max adoption. Reference implementation stays AGPL. You're already doing this with sphinx-rs (MIT + Apache 2.0).
- **CLA is mandatory** if you accept external contributions — without it, contributors retain copyright and can relicense.
- **Enforcement is real**: Bambu Lab AGPL violation (May 2026, SFC), Entr'Ouvert v. Orange (€500K damages), Neo4j v. PureThink (pending Ninth Circuit).

### Licensing by Competitor

| Project | License | CLA/DCO |
|---|---|---|
| libp2p | MIT | DCO (sign-off only) |
| Meshtastic | GPL-3.0 | CLA via cla-assistant.io |
| Reticulum | Custom (MIT + anti-AI + anti-weapons) | Full copyright assignment |
| I2P | Public Domain (core) | None |
| Nym | Apache-2.0 / GPL-3.0 / MIT (mixed) | None |
| MeshCore | MIT | None |

### Dual Licensing in Kenya

- AGPL and commercial licenses are legally valid under Kenya Copyright Act
- University IP policy must be checked (Open University of Kenya)
- EU CRA applies to products sold in EU regardless of origin

---

## 3. Law & Governance

### Export Controls (EAR/ITAR)

**Good news**: Open-source encryption publicly available is **exempt from EAR** (EAR 734.7, 734.13). Your GitHub public repo qualifies.

**Risk zone**: If you add military-specific features (TAK integration, tactical mesh for DoD), you could cross from EAR99 into ITAR territory. Your university affiliation (Open University of Kenya) helps with the fundamental research exception.

**Recommendation**: Keep the open-source version general-purpose. The commercial license variant for defense customers should include export compliance documentation.

### EARN IT Act

The EARN IT Act would condition Section 230 immunity on "best practices" for CSAM detection — widely criticized as a de facto encryption backdoor mandate.

**Your structural protection**: Decentralized P2P means no central provider to hold liable. There's no server to subpoena. This is exactly the architecture that survives EARN IT.

**Status**: Passed Senate Judiciary Committee but never reached full floor vote. Still a threat, but your architecture is resilient.

### EU Cyber Resilience Act (CRA)

**In force December 2024, full compliance by December 2027.**

- FOSS developed outside commercial activity is **EXEMPT**
- Your AGPL version stays in the exempt zone
- The commercial license version has full manufacturer obligations (SBOM, vulnerability reporting, CE marking)
- **This is a selling point** for your commercial license — customers get compliance support built in
- **September 2026 deadline**: Vulnerability reporting obligations begin

---

## 4. IP Protection & Idea Theft

### Copyright vs Patent

| | Copyright | Patent |
|---|---|---|
| **What it protects** | The specific code you wrote | The underlying algorithm or method |
| **How you get it** | Automatic the moment you write code | Must file, examine, grant (2-5 years) |
| **Cost** | $0 (free) | $5K-50K+ (filing + attorney) |
| **Duration** | Life + 50 years | 20 years from filing |
| **What others can't do** | Copy your code | Use your invention (even if independently invented) |
| **What others CAN do** | Independently write the same idea | Prior art can invalidate your patent |

### Are You Helping Someone Steal Your Idea?

**No.** Here's why:

1. **Copyright protects expression, not ideas.** The protocol *concept* cannot be copyrighted. But your *implementation* — the actual Rust code — is automatically copyrighted the moment you write it.

2. **GitHub doesn't increase theft risk.** Research shows GitHub presence doesn't materially increase IP theft. The discoverability benefit (100M+ devs) vastly outweighs theoretical risk.

3. **Execution speed is your real protection.** Ideas are cheap; shipping is hard. You're at v0.2.0 with a published crate (sphinx-rs). A copycat starting from scratch is 6-12 months behind.

4. **Network effects win.** The first mesh to get users wins. Focus on adoption, not protection.

### What Actually Protects You

1. **Trademark** (~$250-350 to register). Forks must use different names.
2. **Execution speed.** You're 6-12 months ahead.
3. **Network effects.** First mesh to get users wins.
4. **Published protocol spec as prior art.** Blocks future patent trolls.

---

## 5. GitHub vs Self-Hosted

### Recommendation: Stay on GitHub. Add Codeberg as a mirror.

| Factor | GitHub | Self-Hosted (Forgejo/Codeberg) |
|---|---|---|
| Discoverability | 100M+ devs | Near zero |
| Community | 87% of devs use it | Every extra account step loses 20-30% contributors |
| CI/Security | Free CodeQL, Dependabot, secret scanning | Needs external tooling |
| Cost | $0 | $9-18/month + maintenance |
| Copilot training | Opt-out needed | Not applicable |

### Phased Approach

- **Phase 1** (now): Stay GitHub, disable Copilot training, add license enforcement
- **Phase 2** (3-6 months): Mirror to Codeberg
- **Phase 3** (only if triggered): Full migration if GitHub TOS changes or commercial customers demand it

---

## 6. Kenya Copyright & Software Law

### Copyright Protection

The **Copyright Act, 2001 (Cap. 130)** explicitly includes software as a protected category under "literary work."

- **Automatic protection**: Copyright vests automatically upon creation — no registration required
- **Duration**: Life + 50 years
- **Berne Convention**: Kenya is a member — automatic protection in 180+ countries

### Registration (KECOBO)

- **Cost**: KES 1,000 (~$7.70 USD)
- **Process**: Online at [nrr.copyright.go.ke](https://nrr.copyright.go.ke)
- **Timeline**: 5-7 working days
- **Certificate**: *Prima facie* evidence of ownership in court
- **Recommended**: Upload source code excerpts as deposit

### Software Patents — Copyright Only

Kenya does **NOT** grant patents for pure software. The **Industrial Property Act** explicitly excludes "computer programs per se."

**If GhostWire has novel hardware integration**: File a utility model with KIPI (~KES 3,000-5,000, 6-12 months).

### Kenya-Specific Risks

1. **Computer Misuse and Cybercrimes Act (CMCA)**: Has been used to arrest developers. Rose Njeri (June 2025) was prosecuted for creating a tool. High Court partially suspended 2024 amendment.

2. **Government surveillance**: NIS conducts targeted interception. CIVICUS downgraded Kenya for digital rights.

3. **Data localization**: Data Protection Act 2019 allows Cabinet Secretary to mandate data localization.

4. **Encryption**: KICA gives Communications Authority power over encryption. No explicit ban on strong encryption, but surveillance pressure growing.

### Startup Incentives

- **Startup Act** (pending): 15% corporate tax for certified startups
- **NIFC**: 15% tax for first 3 years, 20% for next 4 (vs standard 30%)
- **KeNIA**: Innovation grants and commercialization support
- **30% Kenyan ownership** required for foreign ICT companies

---

## 7. OIN & LOT Network

### OIN (Open Invention Network)

**What it does**: 4,100+ members cross-license patents covering 5,181 open-source packages. If any member tries to sue you for using Linux/OSS, they lose their license to everyone else's patents.

**Cost**: Free (under $10M revenue)
**Time**: 5 minutes to sign
**Scope**: Linux System — covers your Rust toolchain, dependencies, any Linux-adjacent code

**AGPL explicitly preserved**: Section 5.4 says nothing in OIN forces you to violate GPL.

**Case studies**: Defended GNOME against Rothschild Patent Imaging (2019), helped TomTom vs Microsoft (2009), weakened WSOU/Dell case (2023).

### LOT Network (License on Transfer)

**What it does**: 6,300+ members grant a conditional license that activates ONLY when a patent is sold to a patent troll. The license "runs with the patent" — even if a troll buys it, they can't sue you.

**Cost**: Free (under $25M revenue)
**Time**: 5 minutes to sign
**Scope**: Any product, any technology — not limited to OSS

**Members CAN still sue each other directly** — LOT only protects against PAE transfer.

### Join Both

| | OIN | LOT |
|---|---|---|
| Protects against | Other OIN members (non-aggression) | PAE/troll attacks |
| Cost | $0 | $0 |
| Members | 4,100+ | 6,300+ |
| Patents covered | 3M+ | 6M+ |

**Total cost: $0. Total time: ~10 minutes. Total protection: millions of patents from 10,000+ organizations.**

---

## 8. Competitor Legal Docs Analysis

### Trademark — The Real Legal Battleground

| Project | Trademark Status | Policy |
|---|---|---|
| libp2p | Informal (Protocol Labs) | No formal policy |
| **Meshtastic** | **Registered (USPTO #6243557)** | Detailed licensing page, Trademark Grant process |
| Reticulum | None found | No policy |
| I2P | None (community) | No policy |
| Nym | Implied (Nym Technologies SA) | ToS prohibits removing trademarks |
| MeshCore | None found | No policy |

**Meshtastic is the only one that takes trademark seriously.** This is the model to follow.

### AI Training Restrictions

| Project | Has AI restriction? |
|---|---|
| libp2p | Disclosure required in PRs (not license-level) |
| Meshtastic | No |
| **Reticulum** | **Yes** — license explicitly prohibits AI/ML training data use |
| I2P | No |
| Nym | No |
| MeshCore | No |

**Reticulum is the only project with an enforceable AI training ban.**

### Governance Maturity

| Project | Structure | Legal Entity |
|---|---|---|
| libp2p | Transitioning to independent foundation | Protocol Labs |
| Meshtastic | BDFL-like via LLC | Meshtastic LLC (New Mexico) |
| Reticulum | BDFL (Mark Qvist) | Individual |
| I2P | Decentralized, informal | None |
| Nym | Corporate | Nym Technologies SA (Swiss corp, $43M+ funded) |
| MeshCore | BDFL (Scott Powell) | Individual |

### What to Copy from Competitors

1. **Meshtastic's trademark model**: Register trademark, create formal Licensing & Trademark page, designate Trademark Supervisor
2. **Meshtastic's CLA**: Use cla-assistant.io — free, automated, blocking on PRs
3. **libp2p's DCO approach**: Lighter than CLA if you prefer contributors keeping copyright
4. **Reticulum's AI training restriction**: Consider adding this to prevent AI scraping

### What NOT to Copy

1. **Reticulum's copyright assignment**: Too aggressive, will deter contributors
2. **Nym's corporate complexity**: Overkill for a solo maintainer at v0.2.0
3. **I2P's informal governance**: Works for them after 20+ years, but risky for a new project

---

## 9. Priority Actions

| Priority | Action | Cost | Time |
|---|---|---|---|
| 1 | Register GhostWire trademark (Kenya or US) | $250-350 | 1-3 months |
| 2 | Register copyright with KECOBO | KES 1,000 ($7.70) | 7 days |
| 3 | Join OIN | $0 | 5 min |
| 4 | Join LOT Network | $0 | 5 min |
| 5 | Add CLA to repo (cla-assistant.io) | $0 | 30 min |
| 6 | Create TRADEMARK_USAGE_GUIDELINES.md | $0 | 1 hour |
| 7 | Add AI training restriction to license | $0 | 30 min |
| 8 | Disable GitHub Copilot training | $0 | 2 min |
| 9 | Document "lawful use only" in README | $0 | 15 min |
| 10 | Apply for NLnet NGI0 grant | $0 | 1-2 hours |

**Total cost: ~$260-360. Total time: ~3 hours of work + 7 days for KECOBO + months for trademark.**

---

*Report compiled from 4 parallel librarian research agents, 5+ web searches, and direct repo analysis of 6 competing projects.*
