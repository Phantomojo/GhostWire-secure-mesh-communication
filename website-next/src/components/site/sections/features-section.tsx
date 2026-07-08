'use client'

import { useState } from "react";
import { FeatureModal } from "@/components/site/feature-modal";

interface FeatureCta {
  href: string;
  label: string;
}

interface Feature {
  tag: string;
  title: string;
  blurb: string;
  techNote: string;
  ctaPrimary?: FeatureCta;
  ctaSecondary?: FeatureCta;
  modalBody: React.ReactNode;
}

const FEATURES: Feature[] = [
  {
    tag: "Encryption",
    title: "Encryption",
    blurb:
      "Multi-cipher E2EE: XChaCha20-Poly1305 primary, vodozemac sessions, AES-256-GCM bridges. No server ever sees plaintext.",
    techNote:
      "Primary: XChaCha20-Poly1305 / Sessions: vodozemac (Olm/Megolm) / Bridges: AES-256-GCM / KEX: X25519 / Ratchet: symmetric forward-secrecy",
    ctaPrimary: { href: "#security", label: "Security Details" },
    ctaSecondary: { href: "#quickstart", label: "Get GhostWire" },
    modalBody: (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            What Is It?
          </h3>
          <p>
            GhostWire uses a defense-in-depth multi-cipher stack.
            XChaCha20-Poly1305 is the primary AEAD for all message encryption
            (256-bit key, 192-bit nonce, RFC 8439 extended). vodozemac
            (Olm/Megolm) handles Matrix-protocol-compatible session management
            in the crypto_machine module. AES-256-GCM bridges interop with
            Briar and Reticulum networks.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Why This Stack?
          </h3>
          <ul className="list-disc list-outside pl-5 space-y-1">
            <li>
              XChaCha20-Poly1305 — primary AEAD. Constant-time,
              hardware-friendly, 24-byte nonce eliminates practical nonce-reuse
              risk. Used in encryption.rs.
            </li>
            <li>
              vodozemac (Olm/Megolm) — Matrix double-ratchet sessions in
              crypto_machine.rs. Audited Matrix protocol implementation.
            </li>
            <li>
              ChaCha20Poly1305 — sealed delivery in sphinx-rs, mailbox,
              wire_protocol.
            </li>
            <li>
              AES-256-GCM — Briar and Reticulum transport bridges for network
              interop.
            </li>
          </ul>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Forward Secrecy
          </h3>
          <p>
            Every session uses a symmetric ratchet with chain key aliasing.
            Nonces derive from the ratchet for messages, CSPRNG for stored data.
            Compromising one message key does not decrypt past or future
            messages.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            What&apos;s Encrypted?
          </h3>
          <ul className="list-disc list-outside pl-5 space-y-1">
            <li>Message payloads</li>
            <li>File chunks</li>
            <li>Session state</li>
            <li>Mailbox contents</li>
            <li>Knowledge Ark entries</li>
            <li>Identity keys (encrypted local vault)</li>
            <li>Sphinx onion packets</li>
          </ul>
        </div>
        <div className="font-mono-gw text-[12px] text-[var(--gw-white-faint)] mt-5 pt-4 border-t border-[var(--gw-border)]">
          Primary: XChaCha20-Poly1305 / Sessions: vodozemac (Olm/Megolm) /
          Bridges: AES-256-GCM / KEX: X25519 / Ratchet: symmetric
          forward-secrecy
        </div>
      </>
    ),
  },
  {
    tag: "Key Exchange",
    title: "Key Exchange",
    blurb:
      "X25519 with forward-secrecy ratchet. Keys never leave your device.",
    techNote: "Signing: Ed25519 / ECDH: X25519 / KDF: HKDF-SHA256",
    modalBody: (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            What Is It?
          </h3>
          <p>
            X25519 elliptic-curve Diffie-Hellman for ephemeral key agreement.
            Ed25519 signs long-term identity keys.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Forward Secrecy
          </h3>
          <p>
            Each session derives fresh keys via a symmetric ratchet with
            chain-key aliasing. Compromising a long-term key does not decrypt
            past sessions.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Key Storage
          </h3>
          <p>
            Keys generated on-device, stored in an encrypted local vault. No key
            escrow, no server-side key, no recovery server.
          </p>
        </div>
        <div className="font-mono-gw text-[12px] text-[var(--gw-white-faint)] mt-5 pt-4 border-t border-[var(--gw-border)]">
          Signing: Ed25519 / ECDH: X25519 / KDF: HKDF-SHA256
        </div>
      </>
    ),
  },
  {
    tag: "Post-Quantum",
    title: "Post-Quantum",
    blurb:
      "ML-KEM-768 hybrid key exchange. Defends against harvest-now-decrypt-later.",
    techNote: "PQ KEM: ML-KEM-768 (FIPS 203) / Hybrid with X25519",
    modalBody: (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            What Is It?
          </h3>
          <p>
            ML-KEM-768 (formerly Kyber-768) is the NIST FIPS 203 standardized
            post-quantum key encapsulation mechanism. GhostWire layers it
            alongside X25519 in hybrid mode.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Harvest-Now-Decrypt-Later
          </h3>
          <p>
            Adversaries today record encrypted traffic for future decryption by
            quantum computers. PQ key exchange neutralizes that threat for
            long-lived secrets.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Hybrid Mode
          </h3>
          <p>
            Classical (X25519) + post-quantum (ML-KEM-768) run together. A break
            in either primitive alone does not break the session.
          </p>
        </div>
        <div className="font-mono-gw text-[12px] text-[var(--gw-white-faint)] mt-5 pt-4 border-t border-[var(--gw-border)]">
          PQ KEM: ML-KEM-768 (FIPS 203) / Hybrid with X25519
        </div>
      </>
    ),
  },
  {
    tag: "Mesh",
    title: "Networking",
    blurb:
      "libp2p GossipSub + Kademlia DHT. The same stack powering IPFS and Ethereum.",
    techNote:
      "Stack: libp2p / DHT: Kademlia / PubSub: Gossipsub / DTN: PRoPHET",
    modalBody: (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            What Is It?
          </h3>
          <p>
            libp2p is the modular P2P networking stack powering IPFS, Ethereum 2,
            and Filecoin. GhostWire uses it for peer discovery, secure channels,
            and multiplexing.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Kademlia DHT
          </h3>
          <p>
            Distributed hash table for peer lookup and content addressing —
            O(log N) lookups across the mesh.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            GossipSub
          </h3>
          <p>
            Topic-based pub/sub messaging. Efficient broadcast for mesh-wide
            events without flooding every link.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Delay-Tolerant Networking
          </h3>
          <p>
            When the mesh partitions, PRoPHET epidemic routing stores and
            carries messages forward until connectivity resumes. No message is
            lost.
          </p>
        </div>
        <div className="font-mono-gw text-[12px] text-[var(--gw-white-faint)] mt-5 pt-4 border-t border-[var(--gw-border)]">
          Stack: libp2p / DHT: Kademlia / PubSub: Gossipsub / DTN: PRoPHET
        </div>
      </>
    ),
  },
  {
    tag: "AI Routing",
    title: "AI Routing",
    blurb: "4-layer ML stack picks the optimal path. LightGBM to PRoPHET.",
    techNote: "L1: LightGBM/ONNX / L2: Gemma 4 / L3: GNN / L4: PRoPHET",
    modalBody: (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Four-Layer Routing
          </h3>
          <ul className="list-disc list-outside pl-5 space-y-1">
            <li>L1 LightGBM/ONNX fast-path (&lt;1ms)</li>
            <li>L2 Gemma 4 via Ollama analytical (50–200ms, every 30s)</li>
            <li>L3 GNN graph optimizer (every 5 min)</li>
            <li>L4 PRoPHET epidemic fallback (&lt;0.01ms, always available)</li>
          </ul>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Real Mesh Data
          </h3>
          <p>
            GNN trained on the GuifiSants community mesh dataset — 7,931
            samples, 63 nodes, 31 days of real topology.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Anomaly Detection
          </h3>
          <p>
            LightGBM classifier flags routing anomalies in 76.7μs. AUC 1.0 on
            held-out test set.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Always-On Fallback
          </h3>
          <p>
            If models are unavailable, PRoPHET epidemic routing keeps the mesh
            working — degraded but never dead.
          </p>
        </div>
        <div className="font-mono-gw text-[12px] text-[var(--gw-white-faint)] mt-5 pt-4 border-t border-[var(--gw-border)]">
          L1: LightGBM/ONNX / L2: Gemma 4 / L3: GNN / L4: PRoPHET
        </div>
      </>
    ),
  },
  {
    tag: "Privacy",
    title: "Anonymous Routing",
    blurb:
      "Sphinx onion routing with SURBs. The same protocol as Nym and Lightning.",
    techNote:
      "Onion: sphinx-rs (crates.io) / Transports: WS, SS, TLS, obfs4",
    modalBody: (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            What Is It?
          </h3>
          <p>
            Sphinx wraps each packet in layers of encryption so intermediate
            hops see only the next hop — never the source or destination. SURBs
            (Single-Use Reply Blocks) let peers reply without learning the
            sender.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Same Protocol As
          </h3>
          <p>
            Sphinx is the onion-routing protocol used by Nym (privacy network)
            and the Lightning Network (Bitcoin layer 2).
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Published Crate
          </h3>
          <p>
            Our Rust implementation{" "}
            <code className="font-mono-gw text-[var(--gw-green)]">sphinx-rs</code>{" "}
            is published on crates.io — reusable by any Rust project.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Traffic Obfuscation
          </h3>
          <p>
            4 pluggable transport modes (WebSocket, Shadowsocks, TLS, obfs4)
            scramble packet shape to defeat DPI. Cover traffic masks timing.
          </p>
        </div>
        <div className="font-mono-gw text-[12px] text-[var(--gw-white-faint)] mt-5 pt-4 border-t border-[var(--gw-border)]">
          Onion: sphinx-rs (crates.io) / Transports: WS, SS, TLS, obfs4
        </div>
      </>
    ),
  },
  {
    tag: "Transports",
    title: "Multi-Transport",
    blurb:
      "5 core transports + 4 Traffic Armor anti-DPI modes. Briar/Meshtastic/Matrix bridges experimental.",
    techNote:
      "Core: StealthTCP, WebRTC, BLE, LoRa, Reticulum / Armor: WS-Camo, SS-AEAD, TLS-Camo, obfs4 / Bridges: Briar, Meshtastic, Matrix",
    modalBody: (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            What Is It?
          </h3>
          <p>
            A modular transport layer with 9 transports in the spec. 5 core
            transports move packets. 4 &apos;Traffic Armor&apos; transports
            disguise packets to defeat DPI and censorship. Auto-failover picks
            the best available link.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Core Transports (Live)
          </h3>
          <ul className="list-disc list-outside pl-5 space-y-1">
            <li>Stealth TCP (fingerprint-resistant)</li>
            <li>WebRTC (browser P2P)</li>
            <li>Bluetooth LE (device-to-device)</li>
            <li>LoRa (km-scale radio)</li>
            <li>Reticulum (off-grid mesh)</li>
          </ul>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Traffic Armor (Anti-DPI, Live)
          </h3>
          <ul className="list-disc list-outside pl-5 space-y-1">
            <li>WebSocket Camouflage</li>
            <li>Shadowsocks AEAD</li>
            <li>TLS Camouflage</li>
            <li>obfs4</li>
          </ul>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Protocol Bridges (Experimental)
          </h3>
          <p>
            Briar, Meshtastic, and Matrix bridges let GhostWire interoperate
            with those networks. These are experimental/planned, not core
            transports.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Build Your Own
          </h3>
          <p>
            Implement the Transport trait to plug in any L2/L3 — satellite,
            acoustic, custom radio. The spec defines the interface.
          </p>
        </div>
        <div className="font-mono-gw text-[12px] text-[var(--gw-white-faint)] mt-5 pt-4 border-t border-[var(--gw-border)]">
          Core: StealthTCP, WebRTC, BLE, LoRa, Reticulum / Armor: WS-Camo,
          SS-AEAD, TLS-Camo, obfs4 / Bridges: Briar, Meshtastic, Matrix
        </div>
      </>
    ),
  },
  {
    tag: "Defense",
    title: "Coercion Resistance",
    blurb:
      "Duress PIN, panic wipe, dead man's switch. Protection against physical coercion.",
    techNote: "Duress PIN / Panic Wipe / Dead Man's Switch",
    modalBody: (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            The Threat
          </h3>
          <p>
            In coercive contexts, an adversary may force you to unlock your
            device or hand over credentials. Standard encryption does not
            protect against this.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Duress PIN
          </h3>
          <p>
            A secondary PIN that, when entered, unlocks a decoy identity with
            innocuous content — while the real identity stays hidden.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Panic Wipe
          </h3>
          <p>
            A configurable trigger that securely erases local keys, message
            stores, and identity vaults when activated.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Dead Man&apos;s Switch
          </h3>
          <p>
            If the node does not receive a check-in within a configurable
            window, it automatically locks or wipes — protecting you if you are
            detained or separated from your device.
          </p>
        </div>
        <div className="font-mono-gw text-[12px] text-[var(--gw-white-faint)] mt-5 pt-4 border-t border-[var(--gw-border)]">
          Duress PIN / Panic Wipe / Dead Man&apos;s Switch
        </div>
      </>
    ),
  },
  {
    tag: "Identity",
    title: "Web of Trust",
    blurb:
      "TOFU key pinning with signed attestations and computed trust scores.",
    techNote:
      "Model: TOFU + signed attestations / Crate: trust-store (in progress)",
    modalBody: (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            What Is It?
          </h3>
          <p>
            A decentralized trust model where peers vouch for each other&apos;s
            identity keys, building a graph of trust without any central
            certificate authority.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            TOFU (Trust On First Use)
          </h3>
          <p>
            The first time you connect to a peer, their key is pinned. Any
            change triggers a warning — defending against key-substitution
            attacks.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Signed Attestations
          </h3>
          <p>
            Peers sign each other&apos;s keys. The more independent
            attestations a key has, the higher its computed trust score.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            In Progress
          </h3>
          <p>
            The{" "}
            <code className="font-mono-gw text-[var(--gw-green)]">
              trust-store
            </code>{" "}
            crate (Web of Trust + TOFU key management) is in active development
            on the path to v1.0.{" "}
            <span className="gw-in-progress ml-2">In Progress</span>
          </p>
        </div>
        <div className="font-mono-gw text-[12px] text-[var(--gw-white-faint)] mt-5 pt-4 border-t border-[var(--gw-border)]">
          Model: TOFU + signed attestations / Crate: trust-store (in progress)
        </div>
      </>
    ),
  },
  {
    tag: "Commons",
    title: "Knowledge Ark",
    blurb:
      "A decentralized knowledge commons. Share code, docs, and datasets across the mesh.",
    techNote:
      "Storage: BlockStore / Discovery: Kademlia DHT / Replication: mesh-gossip",
    modalBody: (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            What Is It?
          </h3>
          <p>
            Knowledge Ark is a decentralized store for sharing code,
            documentation, and datasets across the mesh — accessible even when
            the internet is down.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Why It Matters
          </h3>
          <p>
            For communities with intermittent connectivity, a local cache of
            knowledge (medical guides, farming techniques, educational material)
            is more valuable than a search engine they cannot reach.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            How It Works
          </h3>
          <p>
            Content is chunked, encrypted, and distributed across mesh nodes.
            Requests resolve through the Kademlia DHT. Popular content
            replicates naturally.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Use Cases
          </h3>
          <ul className="list-disc list-outside pl-5 space-y-1">
            <li>Offline Wikipedia mirrors</li>
            <li>Disaster-response field manuals</li>
            <li>Rural education content</li>
            <li>Firmware/code distribution for air-gapped networks</li>
          </ul>
        </div>
        <div className="font-mono-gw text-[12px] text-[var(--gw-white-faint)] mt-5 pt-4 border-t border-[var(--gw-border)]">
          Storage: BlockStore / Discovery: Kademlia DHT / Replication:
          mesh-gossip
        </div>
      </>
    ),
  },
  {
    tag: "Channels",
    title: "Ghost Channels",
    blurb:
      "Channel-based group communication on the mesh. Public, private, and ephemeral.",
    techNote:
      "PubSub: Gossipsub / Group key: XChaCha20-Poly1305 / Backfill: mesh-gossip",
    modalBody: (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            What Is It?
          </h3>
          <p>
            Ghost Channels are channel-based group communication built on the
            mesh. Think of them as topic rooms that propagate peer-to-peer — no
            server, no platform, no algorithm deciding who sees what.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Channel Types
          </h3>
          <ul className="list-disc list-outside pl-5 space-y-1">
            <li>
              Public channels — anyone with the channel ID can join.
            </li>
            <li>
              Private channels — invite-only, encrypted for members.
            </li>
            <li>
              Ephemeral channels — auto-expire after a TTL, leaves no trace.
            </li>
          </ul>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            How They Work
          </h3>
          <p>
            Channels use GossipSub pub/sub. Messages encrypt for the
            channel&apos;s group key. New members get backfill from the mesh.
            Members who go offline catch up via delay-tolerant delivery.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Use Cases
          </h3>
          <ul className="list-disc list-outside pl-5 space-y-1">
            <li>Disaster-response coordination channels</li>
            <li>Community bulletin boards</li>
            <li>Activist action channels (ephemeral)</li>
            <li>Developer team channels</li>
          </ul>
        </div>
        <div className="font-mono-gw text-[12px] text-[var(--gw-white-faint)] mt-5 pt-4 border-t border-[var(--gw-border)]">
          PubSub: Gossipsub / Group key: XChaCha20-Poly1305 / Backfill:
          mesh-gossip
        </div>
      </>
    ),
  },
  {
    tag: "Recovery",
    title: "Shamir Recovery",
    blurb:
      "Split your identity key across N shares. Threshold T needed to recover.",
    techNote:
      "Scheme: Shamir's Secret Sharing (T-of-N) / Module: recovery_shamir.rs",
    modalBody: (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            What Is It?
          </h3>
          <p>
            Shamir&apos;s Secret Sharing splits your identity key into N shares
            distributed to trusted parties. Any T of them (threshold) can
            reconstruct the key — but no single party ever holds the full key.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Why It Matters
          </h3>
          <p>
            Lose your device and you lose your identity — your contacts, your
            history, your reputation. Shamir Recovery lets you reconstruct your
            identity without trusting any single party with a backup.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            How It Works
          </h3>
          <p>
            The identity Ed25519 keypair is split using Shamir&apos;s Secret
            Sharing (T-of-N). Shares distribute to trusted peers or offline
            custodians. To recover, collect T shares and reconstruct. The{" "}
            <code className="font-mono-gw text-[var(--gw-green)]">
              recovery_shamir.rs
            </code>{" "}
            module implements this.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Threat Model
          </h3>
          <p>
            Protects against device loss/destruction. Does NOT protect against
            coercion (use Duress PIN for that) or against T+1 of your custodians
            colluding — choose custodians and threshold carefully.
          </p>
        </div>
        <div className="font-mono-gw text-[12px] text-[var(--gw-white-faint)] mt-5 pt-4 border-t border-[var(--gw-border)]">
          Scheme: Shamir&apos;s Secret Sharing (T-of-N) / Module:
          recovery_shamir.rs
        </div>
      </>
    ),
  },
  {
    tag: "Consensus",
    title: "Causal Ordering",
    blurb:
      "Hybrid Logical Clocks (HLC) order messages correctly across disconnected nodes.",
    techNote:
      "Primitive: HLC (Hybrid Logical Clock) / Crate: hlc-rs (in progress)",
    modalBody: (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            What Is It?
          </h3>
          <p>
            Hybrid Logical Clocks (HLC) combine physical wall-clock time with a
            logical counter to order events correctly across distributed nodes
            — even when clocks drift or nodes disconnect.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Why Not Just Timestamps?
          </h3>
          <p>
            Wall-clock timestamps break when clocks disagree (which they always
            do). If node A&apos;s clock is 5 seconds ahead of node B&apos;s,
            A&apos;s messages look &apos;newer&apos; than B&apos;s even when
            B sent them later. HLC fixes this.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            How It Works
          </h3>
          <p>
            Each message carries an HLC timestamp. When a node receives a
            message, it merges the message&apos;s HLC with its own, incrementing
            the logical counter on tie. The result is a partial order that
            respects causality.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Crate Status
          </h3>
          <p>
            The{" "}
            <code className="font-mono-gw text-[var(--gw-green)]">hlc-rs</code>{" "}
            crate (Hybrid Logical Clocks) is in active development, published
            as part of the ghostwire-libs workspace.{" "}
            <span className="gw-in-progress ml-2">In Progress</span>
          </p>
        </div>
        <div className="font-mono-gw text-[12px] text-[var(--gw-white-faint)] mt-5 pt-4 border-t border-[var(--gw-border)]">
          Primitive: HLC (Hybrid Logical Clock) / Crate: hlc-rs (in progress)
        </div>
      </>
    ),
  },
];

export function FeaturesSection() {
  const [active, setActive] = useState<Feature | null>(null);

  return (
    <section
      id="features"
      className="py-24 border-t border-[var(--gw-border)]"
    >
      <div className="max-w-[1160px] mx-auto px-6 sm:px-8">
        <div className="gw-section-label mb-3">{"// protocol stack"}</div>
        <h2 className="font-display text-3xl sm:text-4xl font-bold mb-3 text-[var(--gw-white)]">
          Built on proven cryptographic foundations
        </h2>
        <p className="text-[var(--gw-white-dim)] text-[15px] mb-10 max-w-2xl">
          Click any card to understand what it is and why we use it.
        </p>

        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
          {FEATURES.map((feature) => (
            <button
              key={feature.tag}
              onClick={() => setActive(feature)}
              className="text-left bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-6 gw-card-hover focus:outline-none focus:border-[var(--gw-green)]"
              aria-label={`Learn about ${feature.title}`}
            >
              <div className="flex items-center gap-2 mb-3">
                <span className="font-mono-gw text-[11px] tracking-[0.08em] uppercase text-[var(--gw-green)] px-2 py-0.5 border border-[var(--gw-border-2)] rounded">
                  {feature.tag}
                </span>
              </div>
              <h3 className="font-display text-xl font-semibold mb-2 text-[var(--gw-white)]">
                {feature.title}
              </h3>
              <p className="text-[14px] text-[var(--gw-white-dim)] leading-relaxed mb-3">
                {feature.blurb}
              </p>
              <span className="font-mono-gw text-[12px] text-[var(--gw-green)]">
                Click to learn more →
              </span>
            </button>
          ))}
        </div>
      </div>

      <FeatureModal
        open={active !== null}
        onClose={() => setActive(null)}
        tag={active?.tag ?? ""}
        title={active?.title ?? ""}
        ctaPrimary={active?.ctaPrimary}
        ctaSecondary={active?.ctaSecondary}
      >
        {active?.modalBody}
      </FeatureModal>
    </section>
  );
}
