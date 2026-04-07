# Getting Started with GhostWire

Welcome to GhostWire! This guide will help you set up, build, and run the project for development or evaluation.

---

## 🚀 What is GhostWire?
GhostWire is a modular, privacy-focused mesh networking and messaging platform. It supports multiple transports (Bluetooth, WiFi, LoRa, WebRTC, TCP/IP), protocol adapters (Briar, Meshtastic, Matrix), and advanced security features.

---

## 1. Prerequisites
- **Rust** (latest stable, [install](https://rustup.rs/))
- **Node.js** (v18+ recommended, [download](https://nodejs.org/))
- **npm** (comes with Node.js)
- **Git**

---

## 2. Clone the Repository
```bash
git clone https://github.com/phantomojo/GhostWire-secure-mesh-communication.git
cd GhostWire-secure-mesh-communication
```

---

## 3. Build & Run the Backend (Rust)
```bash
cd ghostwire
cargo build --release
# To run:
cargo run --release
```

---

## 4. Build & Run the Frontend (React/Tailwind)
```bash
cd webui
npm install
npm run dev
```
- The web UI will be available at [http://localhost:3000](http://localhost:3000)

---

## 5. Build & Preview the Astro Site (Project Homepage)
```bash
cd astro-site
npm install
npm run dev
```
- The Astro site will be available at [http://localhost:4321](http://localhost:4321)

---

## 6. Next Steps
- [Architecture Overview](architecture.md)
- [Transports](transports.md)
- [Security & Privacy](security.md)
- [Protocol Adapters](adapters.md)
- [FAQ](faq.md)
- [Contributing](contributing.md)

---

For any issues, open an [issue on GitHub](https://github.com/phantomojo/GhostWire-secure-mesh-communication/issues) or join the community (Discord coming soon). 