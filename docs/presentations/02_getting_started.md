# Getting Started with GhostWire

---

## Table of Contents
1. Welcome & Audience
2. What You Can Do with GhostWire
3. Quick Start (No Coding)
4. Full Setup (Technical)
5. Joining & Creating a Mesh
6. Using the Web UI & CLI
7. Troubleshooting & Support
8. Real-World Onboarding Scenarios
9. Best Practices for New Users
10. Further Reading & Resources

---

## 1. Welcome & Audience

GhostWire is for everyone—community organizers, first responders, privacy advocates, rural users, and developers. This guide walks you through your first steps, no matter your background.

---

## 2. What You Can Do with GhostWire
- **Send messages** to friends, family, or colleagues—even if the internet is down.
- **Join a local mesh** at an event, protest, or in your neighborhood.
- **Bridge to other networks** (like Briar or Meshtastic) for wider reach.
- **Contribute to open-source** and help build the future of secure, decentralized communication.

---

## 3. Quick Start (No Coding)

### Option 1: Web Demo
1. Visit the GhostWire demo site (if available).
2. Follow the on-screen instructions to join a mesh and send your first message.

### Option 2: Mobile/Desktop App
1. Download the GhostWire app for your platform (links on the project website).
2. Install and open the app.
3. Choose your preferred transport (Bluetooth, WiFi, LoRa).
4. Join or create a local mesh.
5. Start messaging!

---

## 4. Full Setup (Technical)

### Prerequisites
- **Rust** (for backend/CLI): [Install Rust](https://rustup.rs/)
- **Node.js & npm** (for web UI): [Install Node.js](https://nodejs.org/)
- **LoRa hardware** (optional, for long-range)

### Step-by-Step
1. **Clone the repo:**
   ```sh
   git clone https://github.com/phantomojo/GhostWire-secure-mesh-communication.git
   cd GhostWire-secure-mesh-communication/ghostwire
   ```
2. **Build the backend:**
   ```sh
   cargo build --release
   ```
3. **Run the backend:**
   ```sh
   cargo run --release
   ```
4. **Start the web UI:**
   ```sh
   cd ../../webui
   npm install
   npm run dev
   ```
5. **Access the UI:**
   Open your browser to [http://localhost:3000](http://localhost:3000)

---

## 5. Joining & Creating a Mesh

- **Auto-Discovery:** GhostWire finds nearby nodes using Bluetooth, WiFi, or LoRa.
- **Manual Join:** Enter a mesh ID or scan a QR code to join a specific group.
- **Creating a Mesh:** Click “Create Mesh” in the UI, set a name and (optional) password, and invite others.

---

## 6. Using the Web UI & CLI

### Web UI
- **Dashboard:** See connected nodes, active transports, and recent messages.
- **Chat:** Send/receive messages, share files, create groups.
- **Settings:** Choose transports, manage keys, set quotas, enable/disable adapters.
- **Visuals:**
  ```mermaid
  graph TD;
    User["You"] -->|Web UI| GhostWire["GhostWire Node"]
    GhostWire -->|Bluetooth/WiFi/LoRa| Mesh["Mesh Network"]
  ```

### CLI (for Power Users)
- **Start a node:**
  ```sh
  ghostwire-cli start --transport wifi --mesh mymesh
  ```
- **Send a message:**
  ```sh
  ghostwire-cli send --to bob --message "Hello, Bob!"
  ```
- **List nodes:**
  ```sh
  ghostwire-cli nodes
  ```

---

## 7. Troubleshooting & Support

| Problem                        | Solution                                      |
|-------------------------------|-----------------------------------------------|
| Can’t find other nodes         | Check transport settings, try another method  |
| Messages not sending           | Ensure at least one transport is active       |
| Web UI won’t load              | Check backend is running, try npm install     |
| LoRa not working               | Check hardware, drivers, and permissions      |
| Security warning               | Ensure you’re using the latest version        |

- **Logs:** Check backend logs for errors.
- **Community:** Ask for help on GitHub or project chat.

---

## 8. Real-World Onboarding Scenarios

### Community Event
- **Goal:** Set up a mesh for a festival or protest.
- **Steps:**
  1. Organizers install GhostWire on phones/laptops.
  2. Create a mesh and share the QR code.
  3. Attendees join and start messaging.

### Disaster Response
- **Goal:** Connect first responders in a blackout.
- **Steps:**
  1. Deploy LoRa nodes at key locations.
  2. Responders join the mesh via mobile or CLI.
  3. Use store-and-forward to relay messages.

### Rural Village
- **Goal:** Connect homes with no internet.
- **Steps:**
  1. Install GhostWire on home computers.
  2. Use WiFi or LoRa to form a mesh.
  3. Share news, alerts, and messages.

---

## 9. Best Practices for New Users
- Always use the latest version for security.
- Try multiple transports for best coverage.
- Use strong passwords for mesh access.
- Learn about privacy features in the Security chapter.
- Join the community for support and updates.

---

## 10. Further Reading & Resources
- [GhostWire GitHub](https://github.com/phantomojo/GhostWire-secure-mesh-communication)
- [Mesh Networking 101](https://en.wikipedia.org/wiki/Mesh_networking)
- [LoRa Setup Guide](https://lora-alliance.org/about-lorawan/)
- [CLI Reference](12_developer_guide.pdf)
- [Security & Privacy](05_security.pdf)

---

## End of Chapter 