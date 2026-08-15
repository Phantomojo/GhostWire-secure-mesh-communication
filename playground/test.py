#!/usr/bin/env python3
"""Smoke-test a local GhostWire Docker mesh without third-party Python packages."""

from __future__ import annotations

import json
import sys
import time
import uuid
from typing import Any
from urllib.error import HTTPError, URLError
from urllib.request import Request, urlopen

NODES = {
    "bootstrap": "http://localhost:9000",
    "node1": "http://localhost:9001",
    "node2": "http://localhost:9002",
}
REQUEST_TIMEOUT_SECONDS = 5
DELIVERY_TIMEOUT_SECONDS = 20


def request_json(url: str, method: str = "GET", payload: dict[str, Any] | None = None) -> dict[str, Any]:
    body = None
    headers = {"Accept": "application/json"}
    if payload is not None:
        body = json.dumps(payload).encode("utf-8")
        headers["Content-Type"] = "application/json"

    request = Request(url, data=body, headers=headers, method=method)
    with urlopen(request, timeout=REQUEST_TIMEOUT_SECONDS) as response:
        return json.loads(response.read().decode("utf-8"))


def get_data(response: dict[str, Any]) -> Any:
    if response.get("success") is False:
        raise RuntimeError(response.get("error", response))
    return response.get("data", response)


def wait_for_health(name: str, base_url: str) -> None:
    deadline = time.monotonic() + DELIVERY_TIMEOUT_SECONDS
    while time.monotonic() < deadline:
        try:
            data = get_data(request_json(f"{base_url}/api/v2/health"))
            print(f"{name}: healthy ({data})")
            return
        except (HTTPError, URLError, TimeoutError, ValueError, RuntimeError) as error:
            last_error = error
            time.sleep(1)
    raise RuntimeError(f"{name} did not become healthy: {last_error}")


def main() -> int:
    try:
        print("--- Checking health ---")
        for name, base_url in NODES.items():
            wait_for_health(name, base_url)

        print("\n--- Checking peer discovery ---")
        for name, base_url in NODES.items():
            peers = get_data(request_json(f"{base_url}/api/v2/peers"))
            count = len(peers) if isinstance(peers, list) else peers.get("count", 0)
            print(f"{name}: {count} known peer(s)")

        marker = f"playground-broadcast-{uuid.uuid4()}"
        print("\n--- Broadcasting from node1 ---")
        get_data(
            request_json(
                f"{NODES['node1']}/api/v2/messages/send",
                method="POST",
                payload={"recipient": "broadcast", "content": marker},
            )
        )

        print("Waiting for node2 delivery...")
        deadline = time.monotonic() + DELIVERY_TIMEOUT_SECONDS
        while time.monotonic() < deadline:
            messages = get_data(request_json(f"{NODES['node2']}/api/v2/messages?limit=100"))
            if any(message.get("content") == marker for message in messages):
                print("PASS: broadcast reached node2 through the mesh.")
                return 0
            time.sleep(1)

        raise RuntimeError("broadcast marker was not observed in node2's message history")
    except (HTTPError, URLError, TimeoutError, ValueError, RuntimeError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
