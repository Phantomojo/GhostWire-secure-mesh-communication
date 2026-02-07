# GhostWire Technical Architecture & Engineering Specification

## 1. High-Level Architecture
GhostWire is designed as a **Heterogeneous Mesh Network** that prioritizes censorship resistance and partition tolerance over latency. The system is bifurcated into:
*   **The Core (Rust):** A headless, high-performance daemon responsible for networking, encryption, and state management.
*   **The Interface (React/TypeScript):** A responsive, accessible frontend that communicates with the Core via a strict WebSocket/HTTP API.

### Design Philosophy: "Secure by Default, Invisible by Necessity"
*   **Zero-Trust:** Every node is treated as potentially compromised until cryptographically verified.
*   **Protocol Agnostic:** The architecture supports pluggable "Transports" (Bluetooth, WiFi, LoRa) allowing the mesh to adapt to available physical layers.

---

## 2. The Core (Rust)
Located in `ghostwire/src`, the Core is built for stability and safety.

### 2.1 Identity & Cryptography
*   **Identity Generation:** Uses `Ed25519` key pairs for signing and `X25519` for key exchange (Diffie-Hellman).
*   **Rotation:** Implements **"Disposable Identities"**. A node generates a new identity for every session (or user trigger) to prevent long-term tracking.
*   **Traffic Obfuscation:** All packet headers are padded and randomized to look like generic heavy traffic (e.g., HTTPS streaming) to defeat Deep Packet Inspection (DPI).

### 2.2 Mesh Logic (The "Swarm")
*   **Peer Discovery:** Uses mDNS (local) and DHT (Distributed Hash Table) logic to find peers without a central server.
*   **Routing (OOR - Opportunistic Onion Routing):**
    1.  Messages are wrapped in multiple encryption layers (Onion routing).
    2.  Nodes rarely know the final destination, only the "next hop".
    3.  **Store-and-Forward:** If a route is broken, messages are cached encrypted (The "Mule" system) until a path opens.

### 2.3 The "Simulation Engine" (AI Module)
*   **Current State:** A deterministic rule engine that simulates AI behavior for demonstration purposes.
*   **Scenario Manager:** Can trigger localized "events" (e.g., `Scenario::SybilAttack`, `Scenario::Blackout`) to force the frontend to display countermeasures.
*   **Future State (Vision):** Will integrate generic ONNX models for real-time traffic anomaly detection.

---

## 3. The Interface (React/TypeScript)
Located in `webui/src`, the frontend is the user's command center.

### 3.1 Design System
*   **Aesthetic:** "Cyber-Physical". Dark modes, high-contrast alerts, and data-dense dashboards.
*   **Visualizers:**
    *   `MeshVisualizer.tsx`: A standard node-graph visualization of the local mesh.
    *   `MovingMeshBackground.tsx`: Ambient background to indicate connectivity status subliminally.

### 3.2 Security Dashboard
*   **Threat Visualization:** A dedicated panel (`SecurityPanel.tsx`) that translates complex cryptographic events into human-readable alerts (e.g., "Interceptor Detected").
*   **Lockdown Mode:** A single-button "Kill Switch" that severs all external connections and wipes temporary RAM keys.

---

## 4. Resilience Features
### 4.1 The Knowledge Ark
*   **Storage:** Nodes check local storage for `.zim` archives.
*   **Serving:** The Core spins up a lightweight HTTP server on a randomized local port to serve Ark content to non-GhostWire devices (standard browsers) if permitted.

### 4.2 Swarm Mode
*   **Load Balancing:** High-power nodes (desktop/server) advertise `cap:high`. Low-power nodes (phones) offload heavy validation tasks to them, trusting the result via cryptographic proof.
