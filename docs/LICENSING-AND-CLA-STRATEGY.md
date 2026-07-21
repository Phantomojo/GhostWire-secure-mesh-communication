# GhostWire Protocol Publishing & Contributor License Agreement Strategy

**Status**: Research Complete — Awaiting Decision
**Date**: July 15, 2026
**Author**: Sisyphus (automated research synthesis)
**Lawful Use Only**: This document and all GhostWire software, protocols, and documentation are provided for LAWFUL USE ONLY. Users are responsible for compliance with all applicable local, national, and international laws.

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Current License Landscape](#2-current-license-landscape)
3. [Protocol Publishing Strategy](#3-protocol-publishing-strategy)
4. [CLA Strategy](#4-contributor-license-agreement-strategy)
5. [Implementation Plan](#5-implementation-plan)
6. [Templates & References](#6-templates--references)

---

## 1. Executive Summary

GhostWire needs two licensing decisions:

| Decision | Recommendation | Why |
|----------|---------------|-----|
| **Protocol spec license** | Apache 2.0 | Patent grant, max adoption, no conflict with AGPL impl |
| **Contributor CLA** | Yes, CLA (not DCO) | Required for dual-licensing model to work legally |

**Bottom line**: Publish the protocol specification under **Apache 2.0** (separate from the AGPL-3.0 implementation). Use a **Contributor License Agreement** based on the Apache ICLA template, adapted for dual-licensing. This is the same pattern used by Signal, libp2p, OpenAPI, and every dual-licensed open source project.

**Lawful Use Only**: All GhostWire software, protocols, and documentation are provided for LAWFUL USE ONLY. Users must comply with all applicable laws. GhostWire is designed for privacy-respecting, legitimate communication. The developers assume no liability for misuse.

---

## 2. Current License Landscape

### What Exists Today

| Component | License | Copyright | Status |
|-----------|---------|-----------|--------|
| `ghostwire/` (main binary) | AGPL-3.0-or-later | Michael Irungu (2025-2026) | ✅ Active |
| Root LICENSE | AGPL-3.0 + Commercial dual | Michael Irungu | ✅ Active |
| COMMERCIAL_LICENSE.md | Commercial terms | Michael Irungu | ✅ Active |
| `ghostwire-libs/` (sphinx-rs, etc.) | MIT OR Apache-2.0 | Workspace level | ⚠️ No LICENSE files in crate dirs |
| `docs/spec/` (10 protocol docs) | **NONE** | **NONE** | ❌ No copyright or SPDX headers |
| `docs/CLA.md` | Draft only ("Coming Soon") | — | ❌ Not activated |
| `src-tauri/` | Empty (`license = ""`) | — | ❌ Missing |
| `ghostwire/LICENSE` (sub-crate) | MIT (old, superseded) | — | ⚠️ Confusing leftover |
| `graphify/` | MIT | Safi Shamsi | ✅ Separate project |
| `esp-idf/` | Apache 2.0 | Espressif | ✅ Third-party |

### Gaps to Fix

1. **Protocol specs have no license** — 10 files under `docs/spec/` (PROTOCOL.md, WIRE_FORMAT.md, CRYPTO.md, SPHINX.md, DISCOVERY.md, ROUTING.md, DTN.md, IDENTITY.md, SOCIAL.md, TRANSPORT.md) lack copyright headers or license identifiers
2. **CLA is draft** — `docs/CLA.md` exists but is marked "Coming Soon" with no signing mechanism
3. **Library crates missing LICENSE files** — sphinx-rs, ghostwire-dtn, hlc-rs, trust-store have no LICENSE-MIT or LICENSE-APACHE files in their directories despite declaring MIT OR Apache-2.0 in Cargo.toml
4. **Stale ghostwire/LICENSE** — MIT license file in the ghostwire sub-crate conflicts with AGPL root license
5. **src-tauri has no license** — empty license field in Cargo.toml

---

## 3. Protocol Publishing Strategy

### The Question

Should GhostWire publish its protocol specification separately from the implementation, and under what license?

### Answer: Yes, Apache 2.0 for the Spec

**Apache 2.0 is the correct license for the protocol specification.** Here's why.

### How Major Protocols Handle This

| Project | Spec License | Implementation License | Relationship |
|---------|-------------|----------------------|--------------|
| **Noise Protocol** | Public domain | MIT, Apache 2.0, BSD-3 | Completely independent |
| **libp2p** | No explicit license | MIT / Apache-2.0 dual | Separate repos |
| **Signal Protocol** | Unlicensed whitepapers | AGPL-3.0 (reference) | Spec docs ≠ code |
| **WireGuard** | Whitepaper (no license) | GPLv2 / MIT / BSD / Apache | Independent per impl |
| **Tor** | Unlicensed spec docs | BSD / ISC | Separate |
| **MLS (RFC 9420)** | IETF: Revised BSD (code) | Various BSD/MIT/Apache | RFCs are the spec |
| **QUIC (RFC 9000)** | IETF: Revised BSD (code) | Various | RFCs are the spec |
| **OpenAPI** | Apache 2.0 | Various | Spec has its own license |
| **Galileo Protocol** | Apache 2.0 | Apache 2.0 | Same license for both |
| **ACP (Agent Comm.)** | Apache 2.0 | Apache 2.0 | Same license for both |

**Consistent pattern**: Specs and implementations nearly always have **different** licenses, or the spec has no license at all. They are treated as distinct works.

### Why Apache 2.0 Specifically

#### 1. Patent Grant (Strongest Argument)

Apache 2.0 Section 3 grants every implementer:
- A **perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable** patent license
- Applies to **any patent claims necessarily infringed** by a contribution
- **Retaliation**: If you sue for patent infringement, your patent licenses terminate

For a protocol spec, this means:
- Anyone who contributes to the spec grants patent rights to every implementer
- If GhostWire has patentable innovations (novel mesh routing, crypto constructs), putting the spec under Apache 2.0 means contributors cannot later sue implementers
- Companies can safely implement the spec without fear of submarine patents

**Comparison**:
- MIT/BSD: No patent grant (implicit only, legally uncertain)
- CC-BY-4.0: No patent provisions (not designed for code/specs)
- Public domain: No patent provisions
- IETF RFC BSD: No patent grant (IETF has separate IPR disclosure process)
- **Apache 2.0: Explicit, enforceable patent grant**

#### 2. No Copyleft — Maximum Adoption

Implementations can be proprietary, open source under any license (including AGPL), or anything else. This maximizes adoption. If the spec were AGPL, anyone implementing it would arguably need to release their entire application under AGPL — this kills adoption.

#### 3. No Legal Conflict with AGPL Implementation

The key distinction: a **protocol specification** is a descriptive document (text, diagrams, wire format descriptions). An **implementation** is code. They are separate works.

- The Apache 2.0 license on the spec governs use/copying/modification of the **spec document itself**
- The AGPL-3.0 license on the implementation governs distribution of the **code**

An AGPL-3.0 implementation that implements an Apache-2.0 spec is fine because:
1. Specs describe interfaces, not code. Copyright doesn't protect protocols or algorithms, only specific expression.
2. No derivative work issue — writing code that conforms to a spec is not creating a derivative work of the spec document.
3. **Precedent**: Every IETF RFC is effectively "all rights reserved + Revised BSD for code components." Yet GPL, MIT, Apache, and proprietary implementations of TCP, TLS, HTTP all coexist.

#### 4. Industry Accepted

Apache 2.0 is understood and accepted by corporate legal teams. Android, Kubernetes, TensorFlow all use it.

#### 5. Trademark Protection

Apache 2.0 explicitly does NOT grant trademark rights (Section 6), protecting the GhostWire brand.

### What Needs to Change in the Codebase

1. **Add license headers to `docs/spec/*.md`** — SPDX identifier: `SPDX-License-Identifier: Apache-2.0`
2. **Add copyright header**: `Copyright 2025-2026 Michael Irungu. Licensed under the Apache License, Version 2.0.`
3. **Create a `docs/spec/LICENSE`** file with the full Apache 2.0 text
4. **Update root LICENSE** to note the dual-license zones:
   - `ghostwire/` source code: AGPL-3.0-or-later (+ Commercial)
   - `docs/spec/`: Apache 2.0
   - `ghostwire-libs/`: MIT OR Apache-2.0
5. **Optionally create a separate `ghostwire-protocol` repo** for the spec (like libp2p/specs) — but this can wait until the spec stabilizes

### Spec Versioning

Adopt a spec lifecycle:
```
draft → stable → deprecated
```
- `docs/spec/PROTOCOL.md` already says "Version 0.2.0-draft"
- Use RFC 2119 language (`MUST`, `SHOULD`, `MAY`) for interoperability
- Include test vectors in the spec (these can be Apache-2.0 code components)

---

## 4. Contributor License Agreement Strategy

### The Question

Does GhostWire need a CLA, a DCO, or neither?

### Answer: CLA is Required. DCO Will Not Work.

#### Why Not DCO?

DCO (Developer Certificate of Origin) uses an "inbound=outbound" model — the project gets the same rights over a contribution as the project's license grants to the world.

**The dealbreaker**: The AGPL grants broad rights to *users*, but it does **not** grant the maintainer the right to sublicense contributions under *commercial/proprietary terms*. Since GhostWire offers a commercial license, **every single contribution not covered by a CLA becomes a blocker** — the maintainer cannot legally include it in the commercial version without tracking down that contributor and getting separate permission.

This is exactly why MongoDB, Grafana Labs, Mattermost, GitLab, and every other dual-licensed project uses a CLA.

#### Why CLA Is Necessary for GhostWire

| Capability | CLA | DCO | GhostWire Needs? |
|-----------|-----|-----|-----------------|
| Relicensing / dual licensing | ✅ Explicitly permitted | ❌ Inbound=outbound only | **✅ CRITICAL** |
| Patent grant | ✅ Explicit | ❌ None | ✅ Helpful |
| Contributor friction | Higher | Minimal | Trade-off accepted |
| Legal weight | Contractual | Conduct-based | ✅ Needed for commercial |
| Employer authorization | ✅ Handled via CCLA | ❌ Not covered | ✅ Needed |

**From ownCloud** (who switched from CLA to DCO): *"The old CLA is what makes our license change legally possible. Because ownCloud GmbH owns the copyright on CLA-covered contributions, we can change licenses."*

**Consensus in the field**: *"DCO for community, CLA for company."* GhostWire has a company-style commercial licensing tier → CLA is the right choice.

### What the CLA Must Include

#### Essential Clauses

1. **Definitions** — "You" (contributor), "Contribution" (original work), "Work" (GhostWire), "Submit" (PRs, emails, issues)

2. **Copyright License** (the critical one) — A perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable copyright license that includes the right to **sublicense**. Must be broader than AGPL — must explicitly permit the maintainer to distribute under **both** AGPL-3.0 and commercial/proprietary licenses.

3. **Patent License** — Grant covering claims necessarily infringed by the contribution. Defensive termination for patent litigation.

4. **Dual-Licensing Language** — Explicit section stating:
   - Contribution may be distributed under AGPL-3.0-or-later for open source use
   - Contribution may be distributed under a commercial/proprietary license
   - Contributor retains copyright and can use their contribution elsewhere

5. **Representations & Warranties** — Legally entitled to grant, employer permission, original work, third-party disclosure

6. **AS IS / No Support** — Standard: contributions provided without warranty

### ICLA + CCLA

**Recommended: Create both from day one.**

- **ICLA (Individual CLA)**: Signed by each contributor personally. Covers own-time contributions. One-time signature, all future contributions.
- **CCLA (Corporate CLA)**: Signed by authorized signer at a company. Lists authorized employees. Covers work-for-hire contributions. Only needed once per company.

**Why both**: Under "work made for hire" laws, employers own copyright — not the individual. If GhostWire accepts a contribution from an employee without a CCLA from their employer, the employer could later assert ownership and demand removal from the commercial version.

**Practical approach**: Start with ICLA only (most contributors are individuals). Add CCLA when a company wants to contribute.

### CLA Automation

**Recommended: CLA Assistant GitHub Action**

| Detail | Info |
|--------|------|
| **Action** | `badideasforsale/cla-github-action@v3` (active fork) |
| **Setup** | 2-minute GitHub Action YAML |
| **Storage** | Signatures stored in-repo as JSON |
| **Flow** | Contributor comments "I have read the CLA Document and I hereby sign the CLA" on PR |
| **Cost** | Free |

**Minimal config:**
```yaml
jobs:
  CLAAssistant:
    runs-on: ubuntu-latest
    if: >
      (github.event.comment.body == 'recheck' ||
       github.event.comment.body == 'I have read the CLA Document and I hereby sign the CLA') ||
      github.event_name == 'pull_request_target'
    steps:
      - name: CLA Assistant
        uses: badideasforsale/cla-github-action@v3
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          path-to-signatures: 'signatures/version1/cla.json'
          path-to-document: 'https://github.com/Phantomojo/GhostWire-secure-mesh-communication/blob/main/CLA.md'
          branch: 'main'
          allowlist: 'dependabot[bot],github-actions[bot]'
```

### Library Crates: Use DCO Instead

Since sphinx-rs and ghostwire-dtn use MIT + Apache-2.0 (no dual-licensing), a DCO is sufficient for those. Use `Signed-off-by` commits for the permissive crates and CLA for the AGPL+Commercial core.

---

## 5. Implementation Plan

### Phase 1: Protocol Spec License (1-2 hours)

| Step | Action | Files |
|------|--------|-------|
| 1 | Add Apache-2.0 license header to all `docs/spec/*.md` | 10 files |
| 2 | Create `docs/spec/LICENSE` with full Apache-2.0 text | 1 new file |
| 3 | Update root LICENSE to note dual-license zones | LICENSE |
| 4 | Remove stale `ghostwire/LICENSE` (MIT, superseded) | 1 file delete |
| 5 | Update `LICENSE_EXPLANATION.md` to document the 3 zones | LICENSE_EXPLANATION.md |

### Phase 2: CLA Activation (2-3 hours)

| Step | Action | Files |
|------|--------|-------|
| 1 | Draft CLA.md from Apache ICLA template + dual-license modification | CLA.md |
| 2 | Draft CLA-CORPORATE.md for corporate contributors | CLA-CORPORATE.md |
| 3 | Add CLA Assistant GitHub Action workflow | `.github/workflows/cla.yml` |
| 4 | Update CONTRIBUTING.md to reference CLA requirement | CONTRIBUTING.md |
| 5 | Test: open a PR, verify CLA check runs | Manual test |

### Phase 3: Library Crate Cleanup (30 min)

| Step | Action | Files |
|------|--------|-------|
| 1 | Add LICENSE-MIT and LICENSE-APACHE to sphinx-rs | 2 new files |
| 2 | Add LICENSE-MIT and LICENSE-APACHE to ghostwire-dtn | 2 new files |
| 3 | Add LICENSE-MIT and LICENSE-APACHE to hlc-rs | 2 new files |
| 4 | Add LICENSE-MIT and LICENSE-APACHE to trust-store | 2 new files |
| 5 | Fix src-tauri Cargo.toml license field | 1 file edit |
| 6 | Add copyright/SPDX headers to ghostwire-libs source files | ~20 files |

### Phase 4: Documentation (1 hour)

| Step | Action | Files |
|------|--------|-------|
| 1 | Add "Lawful Use Only" notice to README.md | README.md |
| 2 | Add "Lawful Use Only" notice to all docs/spec/*.md | 10 files |
| 3 | Add "Lawful Use Only" notice to THREAT_MODEL.md | 1 file |
| 4 | Add "Lawful Use Only" to website pages | 5+ files |

---

## 6. Templates & References

### CLA Templates to Adapt

| Template | URL | Why |
|----------|-----|-----|
| **Apache ICLA v2.2** (best base) | https://www.apache.org/licenses/icla.pdf | Most widely-adopted, explicitly licensed for reuse |
| **Apache CCLA** | https://www.apache.org/licenses/cla-corporate.pdf | Corporate counterpart |
| **Stemmagraph CLA** | https://github.com/ltmoerdani/stemmagraph/blob/main/CLA.md | Closest existing AGPL+Commercial CLA |
| **Harmony ICLA** | https://www.harmonyagreements.org/docs/ha-cla-i-v1.pdf | Modular options, Option 5 for dual-licensing |
| **Google CLA** | https://cla.developers.google.com/about/google-individual | Forked from Apache, with process docs |

### License Compatibility Reference

| Combination | Compatible? | Notes |
|-------------|-------------|-------|
| Apache 2.0 spec + AGPL-3.0 impl | ✅ Yes | Spec ≠ code, separate works |
| Apache 2.0 code in AGPL-3.0 project | ✅ Yes | One-way: Apache → AGPL works |
| AGPL-3.0 code in Apache 2.0 project | ❌ No | Copyleft incompatible |
| CLA + AGPL-3.0 | ✅ Yes | CLA is broader grant, no conflict |
| CLA + Commercial | ✅ Yes | CLA enables commercial licensing |

### Key Quotes for the CLA

From the Apache ICLA FAQ (reuse permitted):
> "Yes, you may re-use and modify them. If you adapt these agreements for your purposes, make sure that the phrase 'Apache Software Foundation' does not appear in your version."

From ownCloud on why they needed CLA:
> "The old CLA is what makes our license change legally possible."

From the ACP spec on Apache 2.0 for protocols:
> "Apache 2.0 + royalty-free patent grant. Build products, sell services. You cannot own the standard."

---

## Appendix A: "Lawful Use Only" Notice Template

Use this notice in all GhostWire documentation:

```
LAWFUL USE ONLY

This software, protocol specification, and associated documentation
(collectively, "GhostWire") are provided for LAWFUL USE ONLY.

GhostWire is designed for privacy-respecting, legitimate communication.
Users are solely responsible for ensuring that their use of GhostWire
complies with all applicable local, national, and international laws
and regulations.

The developers and contributors of GhostWire assume no liability for
use of this software for any unlawful purpose. By using GhostWire,
you agree to use it only for lawful purposes and in compliance with
all applicable laws.
```

---

## Appendix B: Decision Record

| Decision | Date | Rationale |
|----------|------|-----------|
| Protocol spec: Apache 2.0 | Jul 2026 | Patent grant, max adoption, no conflict with AGPL impl |
| CLA (not DCO) | Jul 2026 | Required for dual-licensing; DCO prevents commercial licensing |
| Apache ICLA as template | Jul 2026 | Most battle-tested, explicitly licensed for reuse |
| CLA Assistant for automation | Jul 2026 | Free, 2-min setup, in-repo signatures |
| DCO for library crates | Jul 2026 | MIT/Apache-2.0 crates don't need CLA |

---

*Document generated by automated research synthesis. Consult legal counsel before finalizing any legal agreements.*
