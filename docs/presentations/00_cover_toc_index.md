---

# GhostWire Bible

**The Comprehensive Guide to Secure, Modular Mesh Networking**

*Version 2.1 · September 2026*

---

<p align="center">
  <img src="../assets/ghostwire-logo-1000.png" alt="GhostWire" width="420" />
</p>

<p align="center">
  <strong>Communication for everyone, everywhere, every time.</strong>
</p>

---

> **Read this first — capability status is verified, not aspirational.**
>
> Every capability claim in this Bible is tracked in
> **[BIBLE_STATUS_SPINE.md](../BIBLE_STATUS_SPINE.md)**, which records the status,
> the date it was checked, and the file-and-line evidence behind it.
>
> Status values are a fixed set: `verified`, `implemented-unproven`, `prototype`,
> `scaffold`, `planned`, `absent`. Where a chapter and the status spine disagree,
> **the status spine is correct.**
>
> *Spine last verified 2026-09-28 against `rescue/sep-disk-tree` @ `e4028f2`.*
> *368 unit tests passing · 58,166 lines across 151 source files.*

---

## Table of Contents

### Foundations

1. [Project Overview](01_project_overview.md)
1b. [Who GhostWire Serves](01b_who_ghostwire_serves.md)
2. [Getting Started](02_getting_started.md)

### How It Works

3. [Architecture Deep Dive](03_architecture.md)
4. [Transports & Protocols](04_transports.md)
5. [Security & Privacy](05_security.md)
6. [Protocol Adapters](06_adapters.md)

### Operating GhostWire

7. [FAQ](07_faq.md)
9. [Deployment & Operations](09_deployment_operations.md)
11. [Performance & Scaling](11_performance_scaling.md)

### Depth

10. [Advanced Security & Threat Response](10_advanced_security.md)
12. [Developer Guide](12_developer_guide.md)
13. [Case Studies & Real-World Stories](13_case_studies.md)
14. [Glossary & Reference](14_glossary_reference.md)
15. [Device Hierarchy & Power-Aware Scaling](15_device_hierarchy_and_scaling.md)
16. [AI Routing & On-Device Intelligence](16_ai_routing_and_intelligence.md)
18. [Security Architecture (Technical Appendix)](security_architecture_appendix.md)

### Reference

8. [Contributing & Community](08_contributing.md)
19. [**Capability Status Spine**](../../docs/BIBLE_STATUS_SPINE.md) — the verified source of truth

---

## How to Read This Bible

**If you are new to GhostWire, or evaluating it** — chapters 1, 2, 7, 17, and 13.
What it does, how to try it, what it isn't, and who it serves. No jargon required.

**If you are deploying it** — chapters 3, 4, 9, 11. Architecture, transport
selection, operations, and how it behaves under load.

**If you are reviewing or auditing it** — chapters 5, 10, 18, and the
[status spine](../../docs/BIBLE_STATUS_SPINE.md). Security architecture, threat
response, and the evidence behind every claim in this document.

**If you are contributing code** — chapters 12 and 8, then
[BRAND_GUIDELINES.md](../BRAND_GUIDELINES.md) before you touch anything visual.

---

## What GhostWire is, in one paragraph

GhostWire is a **protocol**, not an application. It provides the cryptographic,
routing, and transport layers that developers build communication tools on top of.
There is no central server: devices connect directly, messages hop node to node,
and the mesh routes around any node that goes offline. The reference
implementation is written in Rust for memory safety, powered by libp2p for
networking, with a Sphinx onion-routing layer, delay-tolerant store-and-forward
delivery, a revocation system with anti-rollback protection, and post-quantum
key exchange on the live message path.

It is **not** a finished product. It is infrastructure under active development,
built in the open, with an honest ledger of what is verified and what is not.

---

---

<div align="center">

### Brand

**Primary** &nbsp;·&nbsp; GhostWire Green `#00DC9A` &nbsp;·&nbsp; Deep Space Blue `#0F172A`

**Accents** &nbsp;·&nbsp; Sunrise Orange `#F97316` &nbsp;·&nbsp; Trust Blue `#3B82F6` &nbsp;·&nbsp; Safeguard Yellow `#FBBF24`

Full identity specification: [BRAND_GUIDELINES.md](../BRAND_GUIDELINES.md)

</div>

---

<div align="center">

*GhostWire — a Yggdrasil layer for Indra's Net.*

*Michael Irungu Muriithi · Nairobi, Kenya*

</div>
