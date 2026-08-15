# Remote GhostWire Interoperability Testing Runbook

**Status:** Test-environment runbook. It is deliberately separate from production deployment.
**Primary path:** Private **Tailscale** connectivity.
**Fallback:** Cloudflare Access Tunnel for SSH or a protected web console only; it is **not** a substitute for QUIC/UDP/libp2p/RNS transport validation.

## Security Model

A real-machine test node should be reachable only across a dedicated private test network. Do **not** expose SSH, GhostWire, RNS, QUIC, libp2p, or radio-control ports directly to the Internet. Do **not** reuse a personal Tailscale login, a production server tag, a long-lived reusable key, a Tailscale owner token, or an SSH private key for this work.

Tailscale access controls are deny-by-default when explicitly configured, directional, and locally enforced. Auth keys can be one-off, tagged, and ephemeral; tagged keys are appropriate for non-user nodes. [1] [2] Tailscale SSH can apply network policy and SSH policy separately and can restrict a connection to an existing non-root host account. [3]

| Component | Required access | Explicitly prohibited |
|---|---|---|
| Disposable sandbox test node | Tailscale → target TCP/22, target RNS TCP test port, target GhostWire test ports only | Internet exposure, subnet routes, exit-node use, LAN access, root SSH |
| Omen / selected target machine | Receive narrowly scoped test traffic in a dedicated non-root account | Production secrets, personal home directory, `sudo`, host-network administration |
| GhostWire test processes | Dedicated temporary profile/config/state directories | Default profile, real contact database, payment material, production RNS identity |
| Cloudflare, if used | Access-protected SSH or HTTPS dashboard | UDP/QUIC/libp2p/RNS mesh-semantic testing |

> **Critical:** A Tailscale connection is only the management and controlled-IP test substrate. It must not be mistaken for BLE, Wi-Fi Direct, LoRa, RNS AutoInterface, or Internet-NAT interoperability. We separately test the carrier semantics that can actually be represented over the private test path: for example, RNS TCP/Backbone and libp2p QUIC over Tailscale IPs.

## Recommended Tailscale Setup

### 1. Create a non-root test account on the target machine

On **Omen** or another machine intended for testing, create an account that owns only a disposable checkout and test state:

```bash
sudo useradd --create-home --shell /bin/bash ghostwire-test
sudo install -d -o ghostwire-test -g ghostwire-test -m 0700 /srv/ghostwire-test
sudo -u ghostwire-test git clone https://github.com/Phantomojo/GhostWire-secure-mesh-communication /srv/ghostwire-test/GhostWire
```

Do **not** add `ghostwire-test` to `wheel`, `sudo`, Docker, serial-device, Bluetooth, or other privileged groups until a specific hardware test requires an explicit, reviewed exception. For an RNode, BLE radio, or serial device test, create a separate hardware-specific test host or grant only the required device permission for the duration of that test.

### 2. Install Tailscale and join the target normally

On Arch Linux:

```bash
sudo pacman -S --needed tailscale
sudo systemctl enable --now tailscaled
sudo tailscale up --accept-dns=false
sudo tailscale set --ssh
```

Use the interactive login in your browser for the target machine. Name and tag it in the Tailscale admin console as `ghostwire-test-host`; do **not** tag it as a production node. `tailscale set --ssh` manages SSH only on the Tailscale IP and does not rewrite normal `sshd_config` or `authorized_keys`. [3]

### 3. Create two dedicated tags and a least-privilege policy

In **Tailscale Admin Console → Access controls**, add tags and rules equivalent to the following. Replace the port placeholders with the exact ports used for one test run.

```jsonc
{
  "tagOwners": {
    "tag:ghostwire-test-host": ["autogroup:admin"],
    "tag:ghostwire-sandbox": ["autogroup:admin"]
  },

  "acls": [
    // Sandbox → dedicated test host only. No broad tailnet access.
    {
      "action": "accept",
      "src": ["tag:ghostwire-sandbox"],
      "dst": [
        "tag:ghostwire-test-host:22",       // management SSH
        "tag:ghostwire-test-host:4242",     // example RNS TCP / Backbone test listener
        "tag:ghostwire-test-host:8081",     // example GhostWire QUIC UDP must be a separate UDP rule if supported by policy
        "tag:ghostwire-test-host:8080"      // optional test-only local API; omit unless required
      ]
    }
  ],

  "ssh": [
    {
      "action": "accept",
      "src": ["tag:ghostwire-sandbox"],
      "dst": ["tag:ghostwire-test-host"],
      "users": ["ghostwire-test"]
    }
  ]
}
```

Tailscale ACLs control IP/port reachability. Tailscale SSH also needs its own policy rule, and it maps access to an existing local user rather than creating one. [1] [3] If your policy syntax supports protocol-specific grants, permit **only** the exact TCP and UDP ports needed for a declared test. Keep SSH to TCP/22 and keep RNS TCP/Backbone and libp2p QUIC test ports distinct.

**Do not use** `"src": ["*"]`, `autogroup:member`, `"dst": ["*:*"]`, an allow-all default ACL, `root` in the SSH users list, or a broad subnet route.

### 4. Provision a short-lived sandbox node

Create an auth key in **Tailscale Admin Console → Settings → Keys** with all of the following properties:

| Property | Required value |
|---|---|
| Description | `ghostwire-sandbox-one-test-YYYYMMDD` |
| Reusable | **No** — one-off only |
| Expiry | The shortest available period; revoke immediately after the test |
| Ephemeral | **Yes** |
| Tag | `tag:ghostwire-sandbox` |
| Pre-approved | Only if your tailnet requires device approval and you understand the consequence |

A one-off ephemeral tagged key limits the sandbox node to the dedicated tag and removes it when it goes offline. Revoke the key after use; deleting the enrolled sandbox device also removes its tailnet authorization. [2]

**Never send a Tailscale owner API token, a reusable key, a personal device key, SSH private key, password, or any production secret.** If you choose to authorize the sandbox node, share only this one-off temporary test key through the task message, then revoke it after the session. Treat it as a temporary enrollment secret.

### 5. Isolate GhostWire and RNS state on the target

Run every remote test under the unprivileged account and with disposable directories:

```bash
sudo -iu ghostwire-test
cd /srv/ghostwire-test/GhostWire/ghostwire

export GHOSTWIRE_HOME=/srv/ghostwire-test/state/ghostwire
export GHOSTWIRE_RNS_CONFIG_DIR=/srv/ghostwire-test/state/rns
export GHOSTWIRE_RNS_BRIDGE_PATH="$PWD/scripts/reticulum_bridge.py"
mkdir -p "$GHOSTWIRE_HOME" "$GHOSTWIRE_RNS_CONFIG_DIR"
chmod 700 /srv/ghostwire-test/state /srv/ghostwire-test/state/ghostwire /srv/ghostwire-test/state/rns
```

For initial RNS tests, configure a **TCPClientInterface** or **BackboneInterface** to the Tailscale address of a designated test peer. Do not use `AutoInterface` as proof of an Internet/Tailscale test: it is local-network multicast discovery and has different topology/visibility semantics. RNS interface configuration remains in its dedicated test config directory. [4]

### 6. Validate in narrow stages

First prove management access; then prove only the selected carrier. Do not enable all transports at once.

```bash
# On target: confirm its private tailnet address and tag/host identity.
tailscale status
tailscale ip -4

# On target: confirm the dedicated test account and test state.
id ghostwire-test
ls -ld /srv/ghostwire-test/state/*

# Inside GhostWire test checkout: run real RNS bridge test locally first.
python3 scripts/test_reticulum_bridge.py

# Then run a declared two-machine test with exact addresses/ports documented in the test record.
```

The first cross-machine sequence should be: one RNS TCP/Backbone link; then one libp2p QUIC link; then a deliberately controlled cross-carrier handoff test. BLE, Wi-Fi Direct, and LoRa require physical/platform-specific test plans and are not proven merely because both machines can reach one another over Tailscale.

### 7. Tear down immediately after a test window

```bash
# Target
sudo tailscale set --ssh=false

# Tailscale Admin Console
# 1. Revoke the one-off auth key.
# 2. Delete/expire the ghostwire-sandbox node.
# 3. Remove the narrow ACL and SSH rules if no longer needed.

# Target, if the test state should not be preserved
sudo rm -rf /srv/ghostwire-test/state
```

## Cloudflare Fallback

Use Cloudflare only if Tailscale is unavailable and the goal is **human/management access** to a protected SSH service or a web test dashboard. Cloudflare Tunnel avoids opening inbound ports, while Cloudflare Access can enforce identity policy; Cloudflare documents client-side `cloudflared` and Access as a native-terminal SSH option. [5]

It is not suitable as the primary substrate for GhostWire UDP/QUIC/NAT/RNS mesh testing. A Cloudflare SSH tunnel proves that Cloudflare can proxy SSH; it does not prove libp2p direct reachability, UDP behavior, local radio discovery, RNS AutoInterface behavior, or a real multi-carrier handoff.

## Current Acceptance Boundary

| Claim | What this runbook can prove |
|---|---|
| Real-RNS bridge | Official RNS TCP/Backbone delivery and receipts between isolated test hosts |
| libp2p QUIC | Controlled QUIC reachability over private Tailscale addresses |
| Secure maintenance access | Tailscale SSH constrained to `ghostwire-test` |
| Cross-carrier switching | Only after a defined encrypted-envelope handoff test is implemented |
| Wi-Fi Direct / BLE / RNode/LoRa | Requires separate physical-device tests; not established by the Internet test path |

## References

[1]: https://tailscale.com/docs/features/access-control/acls "Tailscale: Manage Permissions Using ACLs"
[2]: https://tailscale.com/docs/features/access-control/auth-keys "Tailscale: Auth Keys"
[3]: https://tailscale.com/docs/features/tailscale-ssh "Tailscale SSH"
[4]: https://reticulum.network/manual/interfaces.html "Reticulum: Configuring Interfaces"
[5]: https://developers.cloudflare.com/cloudflare-one/networks/connectors/cloudflare-tunnel/use-cases/ssh/ "Cloudflare One: SSH"
