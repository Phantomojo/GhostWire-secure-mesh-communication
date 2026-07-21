# GhostWire Cold Email Outreach Strategy

**Status:** Active — GhostWire is in active development (v0.2.0), NOT yet fully built
**Last updated:** 2026-07-09

---

## Core Framing (EVERY email must reflect)

GhostWire is **being built**, not already built. We're honest about:
- Protocol is in active development, not production-ready
- We have working code (Rust, GitHub, crates.io) but need contributors, funding, and partners
- Community building phase — not asking people to deploy, asking them to help build

---

## Priority 1: Funding (Do This Week)

### OTF Internet Freedom Fund
- **Amount:** $10K–$900K (ideal: $150K–$250K for 12 months)
- **Deadline:** Rolling — concept notes accepted anytime, reviewed monthly
- **Apply:** apply.opentech.fund
- **Fit:** Censorship circumvention, secure comms for at-risk users
- **Status:** DRAFT COMPLETE — see `outreach/otf-concept-note.md`
- **Next:** User review → submit at apply.opentech.fund

### NLnet NGI TALER
- **Amount:** €5K–€50K
- **Deadline:** Aug 1, 2026
- **Apply:** nlnet.nl/propose/
- **Fit:** ⚠️ ONLY fits if we propose GNU Taler payment integration on the mesh. NGI TALER is specifically about GNU Taler payments, not general privacy infrastructure.
- **Status:** RESEARCH COMPLETE — see notes below
- **Options:**
  - **Option A:** Propose "GNU Taler P2P Micropayments over GhostWire Mesh" — €30K-€50K, deadline Aug 1
  - **Option B:** Wait for NGI0 Commons Fund (reopens "after summer 2026") — better fit for core protocol
  - **Option C:** Do both — Taler integration now, core protocol later
- **European Dimension:** Hard requirement. Need European co-applicant OR argue global contribution to NGI vision.
- **Office Hour:** July 29, 2026 — "Ask us Anything" at nlnet.nl/events/20260729/

### Sovereign Tech Fund
- **Amount:** Rolling
- **Apply:** sovereign.tech
- **Fit:** Open-source infrastructure
- **Status:** NOT YET APPLIED

### Access Now Digital Security Grant
- **Amount:** $15K–$100K
- **Contact:** info@accessnow.org
- **Fit:** HRD digital security tools
- **Status:** NOT YET APPLIED

---

## Priority 2: Kenyan Tech & Civic Orgs

### Tier 1A — Direct Technical Value

| Target | Contact | Status | Email Sent |
|--------|---------|--------|------------|
| Rustaceans Kenya | rustaceanskenya.github.io | NOT CONTACTED | No |
| Daniel Bitengo (Pathweave) | LinkedIn/GitHub | NOT CONTACTED | No |
| Ushahidi | info@ushahidi.com | NOT CONTACTED | No |
| Strathmore @iLabAfrica | ilabitsecurity@strathmore.edu | NOT CONTACTED | No |

### Tier 1B — Digital Rights & Policy

| Target | Contact | Status | Email Sent |
|--------|---------|--------|------------|
| KICTANet | info@kictanet.or.ke | NOT CONTACTED | No |
| BAKE | info@bake.co.ke | NOT CONTACTED | No |
| ARTICLE 19 East Africa | +254 20 386 12 33 | NOT CONTACTED | No |
| Paradigm Initiative | judith.ogutu@paradigmhq.org | NOT CONTACTED | No |

### Tier 2 — Election Monitoring

| Target | Contact | Status | Email Sent |
|--------|---------|--------|------------|
| ELOG | elog.or.ke | NOT CONTACTED | No |
| Kura Zetu | kurazetu.readthedocs.io | NOT CONTACTED | No |
| Open Elections Kenya | open-elections-kenya.vercel.app | NOT CONTACTED | No |

### Tier 3 — Media

| Target | Contact | Status | Email Sent |
|--------|---------|--------|------------|
| Moses Kemibaro | moses@dotsavvy.com | NOT CONTACTED | No |
| Kenn Abuya (TechCabal) | LinkedIn | NOT CONTACTED | No |
| Dennis Musau (Nation) | LinkedIn | NOT CONTACTED | No |
| Faustine Ngila | LinkedIn | NOT CONTACTED | No |

---

## Priority 3: Foreign Projects & Companies

### Mesh/Privacy Projects

| Target | Contact | Status | Email Sent |
|--------|---------|--------|------------|
| Meshtastic | contact@meshtastic.com | NOT CONTACTED | No |
| Briar Project | contact@briarproject.org | NOT CONTACTED | No |
| Reticulum | Matrix/Discord community | NOT CONTACTED | No |
| Cwtch | openpriv/cwtch GitHub | NOT CONTACTED | No |
| FIPS (Jonathan Corgan) | Nostr DM | NOT CONTACTED | No |

### Podcasts

| Target | Contact | Status | Email Sent |
|--------|---------|--------|------------|
| Sovereign Engineering | sovereignengineering.io | NOT CONTACTED | No |
| Closed Network Privacy | closednetwork.io | NOT CONTACTED | No |
| Citadel Dispatch | noderunners.network | NOT CONTACTED | No |

### African Mesh Contacts

| Target | Contact | Status | Email Sent |
|--------|---------|--------|------------|
| CEKA (Nasaka-WEWE) | admin@civiceducationkenya.com | EMAILED | Yes (Jul 9) |
| Bobi Wine | X: @bobiwine | NOT CONTACTED | No |
| Conakry Hackerspace | Via coverage | NOT CONTACTED | No |

---

## Priority 4: Academic & Research

| Target | Contact | Status | Email Sent |
|--------|---------|--------|------------|
| TU Berlin TKN | info@tkn.tu-berlin.de | NOT CONTACTED | No |
| UCL InfoSec (Danezis) | g.danezis@ucl.ac.uk | NOT CONTACTED | No |
| INRIA LORELEY | claudia.ignat@inria.fr | NOT CONTACTED | No |
| CUNY (Fazio) | Google Cyber Security Grant | NOT CONTACTED | No |

---

## Priority 5: Diaspora & Long-term

| Target | Contact | Status | Email Sent |
|--------|---------|--------|------------|
| KUSAT | kusat.tech | NOT CONTACTED | No |
| Kenya Diaspora Alliance | kenyadiasporaalliance.org | NOT CONTACTED | No |
| Privacy Hub Africa | privacyhubafrica.org | NOT CONTACTED | No |
| NAFASI Initiative | rflgd.org | NOT CONTACTED | No |

---

## Email Templates

### Template 1 — Technical Peer
Subject: P2P mesh protocol built in Nairobi — want to see what we built

Hi [Name],

We're building GhostWire — a P2P mesh communication protocol in Rust. Multi-transport (BLE, WiFi Direct, LoRa, obfs4), post-quantum crypto, delay-tolerant networking, protocol bridges to Briar/Meshtastic/Matrix.

[Specific connection to their work]. Would love 15 minutes to show you what we've built.

ghostwire.cc

### Template 2 — Digital Rights Org
Subject: The technical layer for internet resilience — built in Kenya

Hi [Name],

When the internet goes dark, your advocacy needs a technical answer. GhostWire is a P2P mesh protocol that keeps people communicating without any infrastructure. No servers to shut down, no central point to compel.

We're still building — and we'd love your input on how to make this useful for the communities you serve.

ghostwire.cc

### Template 3 — Media
Subject: The protocol that survives internet shutdowns — built in Nairobi

Hi [Name],

After June 25, we built something: a P2P mesh communication protocol that works when the internet doesn't. No servers, no central authority, no single point of failure. Built in Rust, open source, from Nairobi.

Would make a strong story for [publication]. Happy to walk through the technical architecture and the real-world use cases.

ghostwire.cc

---

## Infrastructure

- **Domain:** www.ghostwire.cc
- **SendGrid:** noreply@ghostwire.cc (100/day free)
- **Email script:** `website-next/scripts/send-email.sh`
- **Email template dir:** `/tmp/` (CEKA template at `/tmp/ceka-email.html`) 
