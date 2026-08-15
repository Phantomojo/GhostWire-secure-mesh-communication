# GhostWire Docker Playground

This directory provides a local, three-node integration environment for validating GhostWire mesh bootstrap and broadcast delivery before deployment. It is a development simulation; do not expose its host ports on an untrusted network.

## Prerequisites

Install Docker Engine with the Compose plugin, then run all commands from this directory. The launcher uses only Docker and Bash; the smoke test uses Python 3's standard library and requires no additional Python packages.

## Features
- **Multi-node Setup**: 1 Bootstrap node + 2 regular nodes.
- **Automated Wiring**: `start.sh` automatically extracts the bootstrap Peer ID and configures the mesh.
- **Health Monitoring**: Integrated health checks for each node.
- **Automated Testing**: `test.py` checks API health, peer discovery visibility, and end-to-end broadcast delivery to a second mesh node.
- **Runtime Isolation**: Each node has a distinct persistent volume path and API/mesh port.

## How to use

1. **Start the playground**:
   ```bash
   ./start.sh
   ```

2. **Run automated tests**:
   ```bash
   python3 test.py
   ```

3. **View logs**:
   ```bash
   docker compose logs -f
   ```

4. **Stop the playground**:
   ```bash
   docker compose down -v
   ```

## Architecture
The playground uses a dedicated Docker network `gw-mesh` (172.20.0.0/16). The launcher first starts the bootstrap node, extracts its runtime-generated libp2p Peer ID from logs, and then supplies that peer's Multiaddr to the remaining nodes through `GHOSTWIRE_BOOTSTRAP_NODES`.

The smoke test verifies broadcast propagation. It does **not** prove anonymity, cryptographic security, direct-message routing, revocation propagation, or causal ordering under packet reordering; those require dedicated integration and adversarial tests.
