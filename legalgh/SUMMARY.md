# GhostWire Legal & Governance — Quick Reference

**Date**: July 15, 2026

---

## TL;DR

Your dual-license model (AGPL + Commercial) is correct. You're not helping anyone steal your idea — copyright protects your code automatically, and execution speed is your real moat. Stay on GitHub, add a mirror to Codeberg. Register your trademark. Join OIN + LOT (both free). The legal landscape (export controls, EARN IT, EU CRA) actually favors your architecture.

---

## Key Findings

### Patent vs Copyright
- **Copyright** protects your code (automatic, free, life + 50 years)
- **Patent** protects the idea (expensive, slow, Kenya doesn't grant pure software patents)
- **You don't need a patent.** Copyright + trademark + execution speed is enough.

### Idea Theft
- Copyright protects expression, not ideas. Anyone can independently build the same concept.
- GitHub doesn't increase theft risk — discoverability benefit outweighs theoretical risk.
- Your moat is shipping speed, not legal barriers. You're 6-12 months ahead.

### Licensing
- AGPL + Commercial is the proven model (MongoDB, MinIO, Grafana)
- AGPL perception softened in 2026 — Elastic and Redis returned to AGPL
- Add CLA before accepting community contributions
- Publish protocol spec under Apache 2.0 (separate from implementation)

### Kenya Law
- Copyright Act explicitly protects software as literary work
- Register with KECOBO: KES 1,000 ($7.70), online, 7 days
- Kenya does NOT grant pure software patents (Industrial Property Act excludes them)
- CMCA has been used to arrest developers — document "lawful use only"
- Startup Act pending — 15% corporate tax for certified startups

### Export Controls
- Open-source encryption publicly available is EXEMPT from EAR (EAR 734.7)
- Risk: adding military features (TAK, tactical mesh) could trigger ITAR
- University affiliation helps with fundamental research exception

### EARN IT Act
- Would condition Section 230 immunity on CSAM detection (de facto encryption backdoor)
- Your P2P architecture is structurally protected — no central provider to hold liable
- Still a threat, but your architecture survives it

### EU Cyber Resilience Act
- FOSS developed outside commercial activity is EXEMPT
- AGPL version stays exempt; commercial version has full obligations
- This is a SELLING POINT for the commercial license
- September 2026: vulnerability reporting obligations begin

### GitHub vs Self-Hosted
- **Stay on GitHub** for discoverability (100M+ devs)
- Mirror to Codeberg for resilience (free Forgejo hosting)
- Disable Copilot training (Settings → Copilot → uncheck)
- Phase 1: GitHub + secure. Phase 2: Codeberg mirror. Phase 3: Full migration only if triggered.

### Patent Defense (OIN + LOT)
- **Join OIN** (free, 5 min): 4,100+ members, 3M+ patents, cross-license for Linux System
- **Join LOT** (free, 5 min): 6,300+ members, 6M+ patents, conditional license against trolls
- Total cost: $0. Total time: ~10 minutes. Millions of patents covered.

### Competitor Legal Docs
- Meshtastic is the only project with a registered trademark — model to follow
- Reticulum has an AI training restriction in its license — consider adding
- Reticulum requires full copyright assignment — too aggressive, don't copy
- Most projects have no CLA/DCO — you should add one

---

## Immediate Actions

| # | Action | Cost | Time |
|---|---|---|---|
| 1 | Register GhostWire trademark | $250-350 | 1-3 months |
| 2 | Register copyright with KECOBO | $7.70 | 7 days |
| 3 | Join OIN | $0 | 5 min |
| 4 | Join LOT Network | $0 | 5 min |
| 5 | Add CLA to repo | $0 | 30 min |
| 6 | Create trademark guidelines | $0 | 1 hour |
| 7 | Add AI training restriction | $0 | 30 min |
| 8 | Disable Copilot training | $0 | 2 min |
| 9 | Document "lawful use only" | $0 | 15 min |
| 10 | Apply for NLnet NGI0 grant | $0 | 1-2 hours |

---

## Key Links

- **OIN**: https://openinventionnetwork.com/join-oin-2-0/
- **LOT**: https://lotnet.com/preview-lot-agreement/
- **KECOBO Registration**: https://nrr.copyright.go.ke
- **NLnet NGI0**: https://nlnet.nl/restack/
- **GitHub Copilot Settings**: https://github.com/settings/copilot

---

*See FULL-REPORT.md for complete analysis with sources and detailed findings.*
