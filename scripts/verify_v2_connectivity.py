"""Verify GhostWire v2 APIs against explicitly configured local test nodes.

This utility intentionally never stores node credentials in source control. Set
`GHOSTWIRE_NODE_ALPHA_TOKEN` and `GHOSTWIRE_NODE_BETA_TOKEN` in the invoking
environment before running it.
"""

import os
import time

import requests

REQUEST_TIMEOUT_SECONDS = 10


def required_env(name: str) -> str:
    """Return a required environment value without printing its contents."""
    value = os.environ.get(name)
    if not value:
        raise RuntimeError(f"{name} must be set before running this connectivity check")
    return value


def configured_nodes() -> dict[str, dict[str, str]]:
    """Build local-node configuration from non-persisted environment inputs."""
    return {
        "node_alpha": {
            "name": "Alpha",
            "url": os.environ.get("GHOSTWIRE_NODE_ALPHA_URL", "http://localhost:3001"),
            "token": required_env("GHOSTWIRE_NODE_ALPHA_TOKEN"),
        },
        "node_beta": {
            "name": "Beta",
            "url": os.environ.get("GHOSTWIRE_NODE_BETA_URL", "http://localhost:3002"),
            "token": required_env("GHOSTWIRE_NODE_BETA_TOKEN"),
        },
    }


def get_json(url: str, headers: dict[str, str]) -> requests.Response:
    """Issue a bounded GET request so a local node outage cannot hang the script."""
    return requests.get(url, headers=headers, timeout=REQUEST_TIMEOUT_SECONDS)


def test_v2_connectivity() -> None:
    """Exercise the v2 profile, security-dashboard, and peer-discovery routes."""
    nodes = configured_nodes()
    print("Waiting for nodes to initialize...")
    time.sleep(2)

    for node in nodes.values():
        print(f"\n--- Testing Node {node['name']} ({node['url']}) ---")
        headers = {"X-Ghost-Token": node["token"]}

        checks = (
            ("Profile V2", "/api/v2/profile"),
            ("Security Dashboard V2", "/api/v2/security/dashboard"),
            ("Peers V2", "/api/v2/peers"),
        )
        for label, route in checks:
            url = f"{node['url']}{route}"
            print(f"Requesting {label}: {url}")
            try:
                response = get_json(url, headers)
                if response.status_code != 200:
                    print(f"{label} failed with status {response.status_code}")
                    continue
                payload = response.json()
                print(f"{label} succeeded")
                if label == "Profile V2" and "links" in payload:
                    print(f"HATEOAS: {payload['links']['self']}")
                if label == "Peers V2":
                    print(f"Found {len(payload['data'])} peers")
            except requests.RequestException as error:
                print(f"{label} request failed: {error}")


if __name__ == "__main__":
    test_v2_connectivity()
