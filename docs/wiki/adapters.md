# Protocol Adapters

Protocol adapters allow GhostWire to bridge and interoperate with other mesh and secure messaging protocols.

---

## What is a Protocol Adapter?
- A software module that translates messages and events between GhostWire and another protocol (e.g., Briar, Meshtastic, Matrix).
- Enables cross-network messaging, group chat, and file sharing.

---

## Supported & Planned Adapters

| Adapter     | Status    | Notes / Features                                 |
|-------------|-----------|--------------------------------------------------|
| Briar       | Planned   | Contact-based messaging, offline queuing, groups  |
| Meshtastic  | Planned   | LoRa radio, store-and-forward, mesh relay         |
| Matrix      | Planned   | Federation, bridges to IRC/XMPP, group chat       |
| Bitchat     | Planned   | Simple mesh chat, proof-of-concept                |

---

## How Adapters Work
- Each adapter implements a common interface.
- Adapters can be enabled/disabled at runtime.
- Messages are translated, deduplicated, and relayed as needed.
- Adapters can bridge group chats, files, and metadata.

---

## Architecture Notes
- Adapters run as part of the backend, in their own modules.
- Adapter logic is isolated for security and maintainability.
- Future adapters (e.g., Signal, Tox) can be added easily.

---

## Future Plans
- Full-featured Matrix bridge for federation.
- Briar/Meshtastic integration for offline and long-range comms.
- Adapter SDK for third-party protocol support. 