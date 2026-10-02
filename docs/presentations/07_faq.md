# Frequently Asked Questions (FAQ)

---

## Table of Contents

1. General Questions
2. Getting Started
3. Technical & Developer Questions
4. Security & Privacy
5. Real-World Scenarios
6. Troubleshooting & Support
7. Visuals: Common Flows
8. Further Reading & Resources

---

## 1. General Questions

**Q: What is GhostWire?**
A: A modular, privacy-focused mesh networking and messaging platform supporting multiple transports and advanced security.

**Q: Who can use GhostWire?**
A: Anyone. It is designed for activists, disaster responders, rural communities, privacy enthusiasts, and developers.

**Q: Is GhostWire free and open-source?**
A: Yes. The code is on [GitHub](https://github.com/phantomojo/GhostWire-secure-mesh-communication).

**Q: What platforms are supported?**
A: Linux, Windows, and macOS. Mobile platform support is under active development.

**Q: Who is GhostWire for?**
A: GhostWire serves six main groups: disaster responders who need to coordinate when cell towers fail, privacy advocates who want no logs and no traces, rural communities where 2.6 billion people lack reliable internet, activists and journalists facing internet shutdowns, whistleblowers who need ephemeral sessions and stealth communication, and developers and researchers who want an AGPLv3 Rust codebase to study and build on.

---

## 2. Getting Started

**Q: How do I install and run GhostWire?**
A: See the Getting Started chapter. You will need Rust, Node.js, and npm for the full stack, or you can try the web demo if available.

**Q: Do I need to be a developer?**
A: No. There are easy-to-use web and mobile interfaces planned for future releases.

**Q: Can I join a mesh without the internet?**
A: Yes. Use Bluetooth, WiFi, or LoRa to connect locally.

---

## 3. Technical & Developer Questions

**Q: How do I add a new transport or adapter?**
A: See the Developer Guide. Implement the relevant trait (`Transport` or `ProtocolAdapter`) and register it in the backend.

**Q: Is there a plugin system?**
A: Yes. GhostWire is designed for modularity and extension.

**Q: How do I contribute code?**
A: Fork the repo, make your changes, and submit a pull request. See the Contributing chapter for details.

**Q: Which transports work today?**
A: **Three transports are working today: stealth TCP, WebRTC, and Reticulum.** QUIC is implemented. Bluetooth LE is a skeleton, WiFi Direct is a platform boundary awaiting an OS provider, and LoRa is designed but not implemented — see [docs/BIBLE_STATUS_SPINE.md](../BIBLE_STATUS_SPINE.md) §4 for the evidence behind each. The system also includes Traffic Armor, which camouflages traffic and auto-switches transports when one is blocked, using Thompson Sampling to pick the most energy-efficient route.

**Q: How does the device hierarchy work?**
A: GhostWire organizes devices into tiers by available RAM. A phone with 6GB or less handles 20 peers and 1GB of storage. A tablet with 8GB or less handles 30 peers. A Raspberry Pi with 4GB or less handles 15 peers. A laptop with 16GB or less handles 50 peers and 5GB of storage. A server with more than 16GB handles 100 or more peers and 50GB or more of storage. Lower-power devices can offload relay and storage duties to a nearby laptop or server, keeping the mesh stable as it grows.

**Q: What AI runs on devices and does it send my data somewhere?**
A: The *design* is for AI models to run on your device, so your data never leaves it. In the current build, however, only the PRoPHET delay-tolerant routing formula is active — no model files are present in the repository, so the LightGBM classifier, the Gemma 4 analytical layer, and the graph neural network do not load (they fail silently to a fallback). Routing decisions today are made by the mesh and PRoPHET, not by ML. No raw data is sent to any central server, and no gradients are transmitted — so federated-learning privacy properties hold by construction. See [docs/BIBLE_STATUS_SPINE.md](../BIBLE_STATUS_SPINE.md) §5 for the per-layer status.

**Q: Which protocol bridges work today versus planned?**
A: The Matrix bridge is implemented behind a feature flag and provides basic connectivity. The Meshtastic bridge is implemented with erasure coding, using 3 data and 2 parity shards so any 3 of 5 can reconstruct a message, which works around the 242-byte LoRa frame limit. The Briar bridge is planned and not yet available. The Reticulum bridge is planned and experimental behind a feature flag. Tor integration is not implemented.

**Q: Has GhostWire been audited?**
A: An external security audit is planned but has not yet been conducted. Candidate auditors include Cure53, NCC Group, and Least Authority. No audit results are available at this time.

**Q: Is my battery drained by relaying?**
A: GhostWire is designed to be battery-aware. When your battery is charging, relaying runs without restrictions. Above 50%, normal operation applies. Between 20% and 50%, cover traffic is reduced by 30%. At 20% or below, the device drops to the most minimal privacy tier and limits relaying. The system also uses energy-aware transport selection, preferring transports that drain less battery.

---

## 4. Security & Privacy

**Q: How secure is GhostWire?**
A: All messages are end-to-end encrypted. The system uses post-quantum ML-KEM-768 key encapsulation, XChaCha20-Poly1305 encryption, and perfect forward secrecy. See the Security chapter for details.

**Q: What if my device is seized?**
A: Keys are encrypted on disk. Use device encryption for extra safety. A duress PIN can protect against coercion, and a dead man's switch can wipe keys if you fail to check in.

**Q: Can I disable security features?**
A: Not recommended. Only for advanced users in test environments.

**Q: What is a privacy tier?**
A: GhostWire assigns privacy tiers based on device type and battery level. Servers operate at L3, laptops and tablets at L2, and phones and Raspberry Pis at L1. When battery drops, the device drops one tier, reducing its visibility and relaying activity.

---

## 5. Real-World Scenarios

**Q: How does GhostWire help in a disaster?**
A: It enables communication when infrastructure is down, using LoRa, WiFi, and Bluetooth.

**Q: Can GhostWire work without internet connectivity?**
A: Yes. It uses Bluetooth, WiFi, and LoRa to maintain communication during infrastructure failures.

**Q: Has GhostWire been used in real-world events?**
A: See the Case Studies chapter for detailed stories.

---

## 6. Troubleshooting & Support

| Problem                        | Solution                                      |
|-------------------------------|-----------------------------------------------|
| Can't find other nodes         | Check transport settings, try another method  |
| Messages not sending           | Ensure at least one transport is active       |
| Web UI won't load              | Check backend is running, try npm install     |
| LoRa not working               | Check hardware, drivers, and permissions      |
| Security warning               | Ensure you are using the latest version       |

- **Logs:** Check backend logs for errors.
- **Community:** Ask for help on GitHub or project chat.

---

## 7. Visuals: Common Flows
![faq](diagrams/07_faq--1.png)

---

## 8. Further Reading & Resources
- [GhostWire GitHub](https://github.com/phantomojo/GhostWire-secure-mesh-communication)
- [Mesh Networking 101](https://en.wikipedia.org/wiki/Mesh_networking)
- [Security & Privacy](05_security.md)
- [Developer Guide](12_developer_guide.md)
- [Device Hierarchy & Power-Aware Scaling](15_device_hierarchy_and_scaling.md)
- [AI Routing & On-Device Intelligence](16_ai_routing_and_intelligence.md)

---

## End of Chapter
