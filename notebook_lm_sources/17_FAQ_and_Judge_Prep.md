# GhostWire Frequently Asked Questions (FAQ) & Judge Prep

## Technical Questions

**Q: How do you prevent Sybil attacks without a central authority?**
A: We use a multi-factor approach. First, we analyze signal strength (RSSI) and arrival time variance. A physical node cannot be in two places at once. If 50 nodes appear with identical signal characteristics, the "Immune System" AI flags them. Second, we use "Resource Puzzles" (Proof of Work) for new nodes joining high-trust clusters.

**Q: What happens if the internet comes back on?**
A: GhostWire seamlessly bridges. It detects the WAN connection and can route non-local messages through the internet via a "Gateway Node" while keeping local traffic on the mesh to save bandwidth.

**Q: Is the AI running on the phone?**
A: Yes. We use ONNX Runtime Web to run quantized models (MobileNet variants) directly in the browser/client. This ensures privacy; your data never leaves your device for analysis.

## Impact Questions

**Q: Why not just use Starlink?**
A: Starlink is expensive, requires a clear view of the sky, and can be jammed or geo-blocked. GhostWire runs on *trash*—old phones, laptops, anything with a battery. It is the "Cockroach" of networks: impossible to kill.

**Q: How do you monetize?**
A: We don't sell data. We sell **Resilience**.
1.  **GhostBrick Hardware:** Pre-configured robust nodes for NGOs.
2.  **Enterprise Deployment:** Tools for Red Cross/UN to deploy and manage 10,000 nodes in a disaster zone.

## "Curveball" Questions

**Q: What if a terrorist uses this?**
A: The same technology that protects a dissident protects a criminal. However, we are building "Community Trust Scores". A local mesh can vote to "shun" a node that is spamming or abusive, effectively isolating them from the local swarm without a central ban hammer.

**Q: Isn't Bluetooth range too short?**
A: Bluetooth is the "Last Mile". The "Backbone" relies on LoRa (Long Range - 5km+) and WiFi Direct. We are a *Heterogeneous* mesh; we use whatever link is strongest.

**Q: How is GhostWire different from Telemesh?**
A: Telemesh pioneered smartphone mesh + blockchain payments, which inspired our DePIN layer. However, GhostWire adds: (1) **Knowledge Ark** - 50GB offline Wikipedia/education, (2) **ONNX AI** - on-device threat detection, (3) **Heterogeneous mesh** - BLE + WiFi + LoRa simultaneously, (4) **Stealth features** - traffic morphing to evade DPI, (5) **Emergency tools** - priority routing + NGO mass deployment. Telemesh is a payment-enabled mesh; GhostWire is a complete civil defense platform.
