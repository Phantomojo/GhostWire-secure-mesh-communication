# GhostWire Feature Specification & Future Roadmap

## 1. Core Communication Features (The "Wire")
The fundamental layer of GhostWire is secure messaging.

*   **P2P Messaging:**
    *   Direct device-to-device messaging via WiFi Direct / Bluetooth.
    *   No internet connection required.
*   **Multi-Hop Routing:**
    *   "Friend of a Friend" routing. Message hops from User A -> Node B -> Node C -> User D.
    *   **TTL (Time To Live):** Preventing infinite loops by killing messages after N hops.
*   **Disappearing Messages:**
    *   Default "Burn on Read" options.
    *   "Panic Wipe": A specialized gesture that deletes all local logs instantly.

## 2. AI-Native Features (The "Ghost")
*Currently simulated for demonstration, planned for edge-inference implementation.*

### 2.1 The Immune System (Threat Detection)
*   **Sybil Detection:** The AI analyzes arrival times and signal strength. If 50 nodes appear instantly with identical signal strength, it flags them as a "Sybil Attack" (one attacker pretending to be many).
*   **Pattern Recognition:** Detects "Jamming Patterns" (high-frequency nonsense packets) and automatically switches channels (e.g., hops from WiFi Ch 6 to Ch 11).

### 2.2 Predictive Routing (Mule Logic)
*   **Motion Prediction:** The AI learns routine paths. "Node X travels from connectivity zone A to disconnected zone B every morning at 08:00."
*   **Pre-Caching:** The network loads messages for Zone B onto Node X *before* it leaves Zone A.

## 3. The Knowledge Ark (The "Library")
*   **Offline Mirroring:** Capable of hosting `.zim` files.
*   **Local Search Engine:** A built-in search UI that indexes the offline content.
*   **Emergency Broadcast:** A "Pinned" status that allows a node administrator to broadcast a critical alert (e.g., "Flood waters rising, move to high ground") that overrides standard traffic.

## 4. User Interface & Experience
*   **The "Netrunner" Dashboard:**
    *   A visualizer showing the active mesh topology.
    *   Red/Green status indicators for Network Health.
*   **Stealth Mode:**
    *   A "Fake UI" that looks like a calculator or a generic game. Entering a specific PIN unlocks the real Messenger.

## 5. Hardware Strategy (Future)
*   **GhostBrick:** A dedicated hardware node (based on ESP32 or Pi Zero) with a LoRa antenna and a large battery. It can be dropped on rooftops or hidden in trees to create permanent mesh infrastructure.
*   **Solar Integration:** Native support for power management to run indefinitely on small solar panels.
