#!/usr/bin/env bash
set -euo pipefail

# Build the image first.
echo "Building GhostWire Docker image..."
docker compose build

# Start the bootstrap node before resolving its dynamically generated Peer ID.
echo "Starting bootstrap node..."
docker compose up -d bootstrap

echo "Waiting for bootstrap peer identity..."
BOOTSTRAP_PEER_ID=""
for _ in $(seq 1 30); do
    BOOTSTRAP_PEER_ID="$(docker compose logs --no-color bootstrap 2>&1 \
        | sed -n 's/.*Local Peer ID: \([^[:space:]]*\).*/\1/p' \
        | tail -n 1 || true)"
    if [[ -n "${BOOTSTRAP_PEER_ID}" ]]; then
        break
    fi
    sleep 2
done

if [[ -z "${BOOTSTRAP_PEER_ID}" ]]; then
    echo "Bootstrap peer identifier was not emitted within 60 seconds."
    docker compose logs --no-color bootstrap
    exit 1
fi

echo "Bootstrap Peer ID: ${BOOTSTRAP_PEER_ID}"

# Compose interpolates this value into GHOSTWIRE_BOOTSTRAP_NODES for node1/node2.
export BOOTSTRAP_PEER_ID
echo "Starting peer nodes..."
docker compose up -d node1 node2

echo "Playground is running."
echo "Bootstrap API: http://localhost:9000"
echo "Node 1 API:    http://localhost:9001"
echo "Node 2 API:    http://localhost:9002"
echo "Run 'python3 test.py' to execute the smoke test."
