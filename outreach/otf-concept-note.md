# OTF Internet Freedom Fund — Concept Note Draft

**Application URL:** https://apply.opentech.fund/
**Status:** DRAFT — Ready for review
**Date:** 2026-07-09

---

## 1. Project Title

**GhostWire: Resilient P2P Mesh Communication Protocol with Cross-Ecosystem Bridging**

---

## 2. Describe your project in 1-3 sentences (100 words max)

GhostWire is an open-source, multi-transport P2P mesh communication protocol built in Rust. It provides end-to-end encrypted messaging across six transport layers (BLE, WiFi Direct, LoRa, stealth TCP, WebRTC, Reticulum) with post-quantum cryptography (ML-KEM-768), anonymous routing, coercion resistance, and delay-tolerant delivery — ensuring communication survives when centralized infrastructure is shut down or destroyed. A key feature is cross-protocol bridging: GhostWire connects existing mesh ecosystems (Briar, Meshtastic, Matrix, Reticulum) so that users on different platforms can communicate seamlessly through a single mesh. Built from Nairobi, Kenya, where internet shutdowns are lived experience, GhostWire is protocol infrastructure that any developer can build on.

---

## 3. What problem will your project address? (500 words max)

Internet shutdowns are accelerating across Africa and the Global South. In 2025 alone, governments in Sudan, Ethiopia, Senegal, Uganda, and Kenya disrupted connectivity during protests, elections, and civil unrest. On June 25, 2025, Kenya experienced its most severe internet disruption — Telegram was cut to 1% reachability by 3PM, ISPs throttled traffic, and transmission stations were physically raided. Citizens received farewell messages from family members. Ambulances were teargassed. Hospitals received 90 casualties with no way to coordinate incoming patients.

Existing encrypted communication tools fail during these scenarios. Signal and WhatsApp require centralized servers that governments can block. Briar works over Bluetooth and WiFi but supports only two transports. Meshtastic operates on LoRa but lacks end-to-end encryption designed for protest environments. No single tool combines multiple transport layers with post-quantum security, protocol bridges to existing ecosystems, and coercion resistance.

The gap is not just technical — it is existential. When the internet goes dark, the people who need to communicate most — journalists documenting abuses, activists coordinating responses, medical workers treating casualties — lose their only lifeline. Current tools force users to choose between security and reliability. GhostWire eliminates that tradeoff by building a protocol layer that adapts to whatever network infrastructure survives.

The problem compounds across borders. A journalist in Nairobi may need to communicate with a source in Addis Ababa during simultaneous shutdowns. A human rights defender in Khartoum may need to relay information through a mesh of nodes spanning Sudan, South Sudan, and Uganda. No existing protocol handles multi-transport, multi-hop, cross-border communication with post-quantum security and delay-tolerant delivery.

GhostWire addresses this by building a protocol — not an application — that any developer can integrate. The protocol layer means GhostWire can power messaging apps, election monitoring tools, emergency coordination systems, and sensor networks. It is infrastructure, not a product.

The urgency is immediate. Kenya's Digital Rights Coalition, Civic Education Kenya (CEKA), and election monitoring organizations like ELOG have expressed need for communication tools that survive shutdowns. CEKA has already built Nasaka-WEWE, a Swahili-first mesh messenger forked from Briar, and is seeking protocol-level improvements. GhostWire's Briar bridge adapter makes integration possible without requiring CEKA to abandon their existing work.

---

## 4. Project Form

**Technology Development**

---

## 5. Activities Overview (500 words max)

**Objective 1: Complete Core Protocol Transport Layer (Months 1–6)**
- Finalize BLE mesh transport using libp2p with Noise protocol authentication
- Implement WiFi Direct transport for higher-bandwidth local communication
- Integrate LoRa transport via Meshtastic adapter for long-range, low-bandwidth messaging
- Implement stealth TCP transport with traffic morphing to resist deep packet inspection
- Add WebRTC transport for internet-connected scenarios
- Deliverable: Working multi-transport protocol with automatic transport selection based on network conditions

**Objective 2: Post-Quantum Cryptography & Anonymous Routing (Months 3–9)**
- Integrate ML-KEM-768 (CRYSTALS-Kyber) for key encapsulation
- Implement XChaCha20-Poly1305 as primary symmetric cipher
- Build Sphinx onion routing layer for anonymous message delivery
- Implement coercion resistance via duress PIN system with decoy profiles and panic wipe
- Deliverable: Protocol with post-quantum key exchange, anonymous routing, and coercion resistance

**Objective 3: Protocol Bridge Ecosystem (Months 4–12)**
- Complete Briar bridge adapter for cross-protocol messaging
- Complete Meshtastic adapter for LoRa hardware integration
- Complete Matrix bridge for federation with existing encrypted chat infrastructure
- Complete Reticulum adapter for integration with community mesh networks
- Deliverable: GhostWire nodes can communicate with Briar, Meshtastic, Matrix, and Reticulum users

**Objective 4: Delay-Tolerant Networking (Months 6–14)**
- Implement spray-and-wait routing for intermittent connectivity
- Build mailbox system for store-and-forward messaging
- Implement knowledge ark for distributed data storage during extended outages
- Deliverable: Messages survive multi-day network partitions and deliver when connectivity returns

**Objective 5: AI-Assisted Routing & Network Intelligence (Months 8–16)**
- Implement graph neural network (GNN) routing for optimal multi-hop message delivery
- Build anomaly detection for network health monitoring
- Add presence obfuscation to resist traffic analysis
- Deliverable: Intelligent routing that adapts to network topology changes in real-time

**Objective 6: Security Hardening & Audit (Months 12–18)**
- Commission independent security audit of cryptographic implementations
- Implement panic wipe system for device seizure scenarios
- Build dead man's switch for automatic data destruction
- Publish threat model and security analysis
- Deliverable: Audit report, hardened protocol, published threat model

**Objective 7: Community, Documentation & Deployment (Months 1–24)**
- Publish protocol specification (RFC-style)
- Create developer documentation and integration guides
- Conduct user research with journalists and activists in Kenya, Uganda, and Ethiopia
- Host 4 community workshops in Nairobi
- Build reference applications demonstrating GhostWire integration
- Deliverable: Published spec, developer docs, user research report, workshop materials, reference apps

---

## 6. Similar Projects (250 words max)

**qaul** (OTF-funded, $303K, 2026): P2P mesh messenger over BLE and LAN. GhostWire differs by adding WiFi Direct, LoRa, stealth TCP, WebRTC, and Reticulum transports, plus post-quantum cryptography and protocol bridges.

**Briar** (OTF-funded): Decentralized encrypted messaging over Bluetooth/WiFi/Tor. GhostWire builds a bridge adapter to Briar, enabling cross-protocol communication rather than competing. Briar is application-level; GhostWire is protocol-level infrastructure.

**Amigo** (OTF-funded, $172K, 2026): Research on secure group messaging over mesh during shutdowns. GhostWire complements by providing the production protocol layer that research projects like Amigo can build upon.

**NewNode** (OTF-funded, $503K, 2023): Decentralized P2P content delivery. GhostWire differs by focusing on bidirectional mesh communication with anonymity, not content distribution.

**Meshtastic** (open-source): LoRa mesh networking. GhostWire provides the encrypted protocol layer that Meshtastic lacks, with bridges enabling Meshtastic hardware to participate in GhostWire's multi-transport mesh.

**Reticulum** (open-source): Modular mesh networking stack. GhostWire builds a Reticulum adapter, treating it as a transport layer rather than a competing protocol.

**VPNalyzer** (OTF-funded, $161K/24 months): Crowdsourced VPN research tool — comparable in timeline and budget scope.

GhostWire's unique position: it is the only project combining multi-transport mesh, post-quantum cryptography, cross-protocol bridging (Briar, Meshtastic, Matrix, Reticulum), anonymous routing, coercion resistance, delay-tolerant networking, and AI-assisted routing — built from Nairobi by a team that experiences the problem firsthand.

---

## 7. Timeline

24 months (2 years), milestone-based with Phase 1 (Months 1–12: Core Protocol) and Phase 2 (Months 13–24: Hardening, Audit & Deployment)

---

## 8. Funding Estimate

$250,000 – $300,000 (over 24 months)

**Budget breakdown:**
| Category | 2-Year Total | Notes |
|----------|-------------|-------|
| Lead developer (Nairobi) | $100,000–$130,000 | Full-time protocol development |
| Security audit | $30,000–$60,000 | Independent audit + OTF Security Lab (free) |
| Infrastructure | $16,000–$24,000 | Test nodes, CI, domains, mirrors |
| Community grants | $14,000–$20,000 | Bug bounties, code reviews, translations |
| Travel/conferences | $10,000–$15,000 | OTF convenings, relevant events |
| Equipment | $5,000–$10,000 | Laptop, security keys, comms setup |

**Comparable OTF projects:** VPNalyzer ($161K/24mo), Círculo ($259K/21mo), Polymorphic ($337K/12mo)

---

## 9. Who Benefits? (500 words max)

**Everyone who needs to communicate when networks fail.**

Internet shutdowns don't just affect activists and journalists — they affect anyone with a phone. When Kenya's internet was cut on June 25, 2025, it wasn't just reporters who lost contact. Parents couldn't reach their children. Businesses lost payment systems. Hospitals couldn't coordinate incoming patients. Boda boda riders became impromptu ambulances because no one could call for help. The person selling vegetables at the market lost M-Pesa. The student couldn't submit an assignment. The grandmother in the village couldn't video call her family in Nairobi.

**Primary beneficiaries:**

1. **Ordinary citizens during network failures** — Anyone with a phone benefits when mesh networking provides an alternative to failed cellular and internet infrastructure. During natural disasters, power outages, or infrastructure collapse, GhostWire's multi-transport mesh means phones can still communicate directly without cell towers or internet. Parents reach children. Families coordinate. Communities self-organize.

2. **Human rights defenders and journalists** (Kenya, Uganda, Ethiopia, Sudan — estimated 5,000+ at-risk): Document abuses, coordinate responses, and maintain contact with sources during shutdowns. GhostWire's coercion resistance and anonymous routing provide protection that no centralized app can offer.

3. **Emergency responders and medical workers** — During Kenya's June 25 shutdown, hospitals received 90 casualties with no way to coordinate incoming patients. GhostWire's mesh networking enables ambulance-to-hospital communication, triage coordination, and medical supply logistics when infrastructure is down.

4. **Election monitors and civic organizations** (ELOG, Kura Zetu, CEKA, KICTANet — ~10,000 observers): Transmit results and incident reports during election-night shutdowns. GhostWire provides encrypted, delay-tolerant transport that cannot be centrally blocked.

5. **Rural and underserved communities** — Over 4 billion people live in areas with unreliable connectivity. GhostWire's mesh networking enables local communication without requiring cellular or internet infrastructure. Agricultural cooperatives, community health workers, and local governance benefit from communication that works offline.

6. **Open-source developers and the broader P2P ecosystem** — GhostWire's protocol-level architecture and bridge ecosystem (Briar, Meshtastic, Matrix, Reticulum) provide infrastructure that any developer can build on. Hardware manufacturers (Heltec, LilyGo, RAK) whose LoRa devices gain encrypted protocol support. Other OTF-funded projects (qaul, Amigo, Briar) can integrate GhostWire's protocol as infrastructure.

7. **Global diaspora communities** (Kenya, Sudan, Ethiopia — millions): Maintain secure communication with family during crises. GhostWire's delay-tolerant networking enables asynchronous messaging across borders during shutdowns.

8. **Privacy-conscious users globally** — GhostWire's post-quantum cryptography (ML-KEM-768), anonymous routing (Sphinx), and coercion resistance provide security guarantees that no centralized messaging platform can match — regardless of whether the internet is shut down.

**The core insight:** GhostWire isn't a niche tool for a specific group. It's infrastructure that becomes critical when centralized systems fail. Just as everyone benefits from roads regardless of whether they drive a specific car, everyone benefits from resilient mesh networking regardless of what app they use. The protocol layer means GhostWire powers the communication underneath — whether that's a messaging app, an election monitoring tool, a hospital coordination system, or a community network.

---

## 10. Geographic Location

**Primary:** Eastern Africa (Kenya, Uganda, Ethiopia, Sudan)
**Secondary:** Western Africa (Nigeria, Senegal), Global

---

## 11. Why You/Your Team (500 words max)

We are building GhostWire from Nairobi, Kenya — not because it is convenient, but because we live the problem. The June 25, 2025 internet shutdown was not a news story for us. It was the day we could not reach family members, the day ambulances were teargassed, the day we decided to build what did not exist.

**Cybersecurity expertise:**

The lead developer's background is in cybersecurity and network infrastructure, not just protocol development. Previous work includes:

- **Cerberus Honeypot** — Bio-adaptive IoT honeypot system built in C for cybersecurity research, demonstrating deep understanding of network protocols, attack vectors, and defensive architecture. Published on GitHub (Phantomojo/cerberus-honeypot).

- **Honeypot Guardian** — Multi-protocol honeypot system with AWS integration and threat intelligence collection, showing ability to build production-grade security infrastructure.

- **Privacy Stack** — Enterprise-grade privacy system with DNS filtering, proxy management, and comprehensive documentation. Understanding of the censorship and surveillance stack that GhostWire must defeat.

- **WISPGate deployment** — Technical implementation of ISP billing, CRM, and provisioning systems for Elegant Telekom. Hands-on experience with MikroTik RouterOS, TCP/IP, VLANs, PPP, DHCP, and ISP infrastructure — the same network layers GhostWire's stealth TCP transport must operate over.

- **Claude Code Proxy** — API translation layer integrating multiple LLM providers with streaming, tool use, and classifier bypass. Demonstrates ability to build complex protocol translation systems — directly relevant to GhostWire's cross-protocol bridging.

**Competition recognition:**

GhostWire was a participant in the **Global Competition on Design for Futures (2026)** — themed "AI for Society" with participants from across Asia and Africa. The project demonstrated strong technical depth and real-world relevance, but completing production-quality development requires funding to address infrastructure, security auditing, and dedicated development time that academic constraints did not permit. This OTF application directly addresses those gaps. Team GhostWire comprises five members from different Kenyan universities (Open University of Kenya, KCA University, Zetech University, University of Nairobi), demonstrating collaborative capacity and interdisciplinary approach.

**Technical depth (current codebase):**

The GhostWire protocol is 42,000+ lines of Rust across 128 source files, implementing multi-transport mesh networking (libp2p), post-quantum cryptography (ML-KEM-768), Sphinx onion routing, coercion resistance, protocol bridges, delay-tolerant networking, and AI-assisted routing via graph neural networks. Published on crates.io and GitHub under AGPL-3.0.

**Community connections:**

We are embedded in Kenya's digital rights ecosystem with direct relationships to:
- CEKA (Civic Education Kenya) — already in contact, exploring integration
- KICTANet — Kenya's leading ICT policy think tank
- BAKE (Bloggers Association of Kenya) — grassroots creator community
- Internet Society Kenya Chapter — 2,270+ members, community network deployments

We are not outsiders proposing to solve a problem we read about. We are builders who experienced the shutdown, documented its impact, and are constructing the technical answer — combining cybersecurity expertise, network infrastructure experience, and the lived understanding that only comes from being in the place where the problem is real.

---

## 12. Acknowledgments

- [ ] I am at least 18 years of age
- [ ] I have read and understand OTF's Terms and Privacy policy
- [ ] I understand that all intellectual property created with support must be openly licensed
- [ ] Yes, I agree to undergo a code security audit facilitated by OTF
