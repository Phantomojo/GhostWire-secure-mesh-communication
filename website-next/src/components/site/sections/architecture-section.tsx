'use client';

import { useState, type ReactNode } from "react";
import { FeatureModal } from "@/components/site/feature-modal";

interface Layer {
  number: string;
  name: string;
  components: string;
  color: string;
  tag: string;
  modalBody: ReactNode;
  differentiator?: boolean;
  ctaPrimary?: { href: string; label: string };
}

interface AIRoutingLayer {
  n: string;
  name: string;
  tech: string;
  latency: string;
  desc: string;
  tag: string;
  modalBody: ReactNode;
}

interface ActiveLayer {
  tag: string;
  title: string;
  body: ReactNode;
  ctaPrimary?: { href: string; label: string };
}

const PROTOCOL_LAYERS: Layer[] = [
  {
    number: "07",
    name: "APPLICATION",
    components: "Messaging · Knowledge Ark · Ghost Channels · Files",
    color: "var(--gw-green)",
    tag: "Layer 07 · Application",
    modalBody: (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            What lives here
          </h3>
          <ul className="space-y-1.5 list-disc list-inside marker:text-[var(--gw-green)]">
            <li>
              <strong className="text-[var(--gw-white)]">Messaging</strong> — 1:1, group,
              delay-tolerant delivery
            </li>
            <li>
              <strong className="text-[var(--gw-white)]">Knowledge Ark</strong> — code, docs,
              datasets commons
            </li>
            <li>
              <strong className="text-[var(--gw-white)]">Ghost Channels</strong> — public,
              private, ephemeral pub/sub rooms
            </li>
            <li>
              <strong className="text-[var(--gw-white)]">Files</strong> — chunked, encrypted
              file transfer
            </li>
          </ul>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            How it&apos;s exposed
          </h3>
          <p>
            Through the API layer below — REST, WebSocket, CLI, and TUI. Build any UI on top of
            the node.
          </p>
        </div>
      </>
    ),
  },
  {
    number: "06",
    name: "API",
    components: "REST (Axum) · WebSocket · CLI · TUI",
    color: "var(--gw-green)",
    tag: "Layer 06 · API",
    modalBody: (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Four interfaces
          </h3>
          <ul className="space-y-1.5 list-disc list-inside marker:text-[var(--gw-green)]">
            <li>
              <strong className="text-[var(--gw-white)]">REST API</strong> — Axum,{" "}
              <code className="font-mono-gw text-[var(--gw-green)]">/api/v2/</code>,
              JWT-authenticated
            </li>
            <li>
              <strong className="text-[var(--gw-white)]">WebSocket</strong> — real-time mesh
              events, peer joins, message receipts
            </li>
            <li>
              <strong className="text-[var(--gw-white)]">CLI</strong> —{" "}
              <code className="font-mono-gw text-[var(--gw-green)]">ghostwire</code> binary,
              fully scriptable
            </li>
            <li>
              <strong className="text-[var(--gw-white)]">TUI</strong> — terminal dashboard with
              tabs, peer list, and compose pane
            </li>
          </ul>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Start a node
          </h3>
          <pre className="font-mono-gw text-[13px] bg-[var(--gw-bg)] border border-[var(--gw-border)] rounded p-3 overflow-x-auto text-[var(--gw-white-dim)]">
            <code>{`cargo run --release
# then connect to http://127.0.0.1:8080`}</code>
          </pre>
        </div>
      </>
    ),
  },
  {
    number: "05",
    name: "PROTOCOL SERVICES",
    components:
      "BlockStore/Bitswap · Mailbox · Social Graph · Sphinx · HLC · Device Pairing · Shamir Recovery · Coercion Resistance",
    color: "var(--gw-green)",
    tag: "Layer 05 · Protocol Services",
    modalBody: (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold mb-3 text-[var(--gw-white)]">
            Eight services
          </h3>
          <ul className="space-y-2 list-disc list-inside marker:text-[var(--gw-green)]">
            <li>
              <strong className="text-[var(--gw-white)]">BlockStore / Bitswap</strong> —
              content-addressed storage, IPFS-compatible
            </li>
            <li>
              <strong className="text-[var(--gw-white)]">Mailbox</strong> — offline message
              queue, delay-tolerant delivery
            </li>
            <li>
              <strong className="text-[var(--gw-white)]">Social Graph</strong> — Web of Trust
              with signed attestations
            </li>
            <li>
              <strong className="text-[var(--gw-white)]">Sphinx</strong> — onion router for
              anonymous message routing
            </li>
            <li>
              <strong className="text-[var(--gw-white)]">HLC</strong> — Hybrid Logical Clocks
              for causal ordering
            </li>
            <li>
              <strong className="text-[var(--gw-white)]">Device Pairing</strong> — QR-based
              multi-device setup
            </li>
            <li>
              <strong className="text-[var(--gw-white)]">Shamir Recovery</strong> — T-of-N key
              recovery
            </li>
            <li>
              <strong className="text-[var(--gw-white)]">Coercion Resistance</strong> — duress
              PIN, panic wipe, dead man&apos;s switch
            </li>
          </ul>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Modular
          </h3>
          <p>
            Implementations MAY support any subset of layers while remaining protocol-compatible
            (per the spec).
          </p>
        </div>
      </>
    ),
  },
  {
    number: "04",
    name: "AI ROUTING",
    components: "LightGBM → Gemma 4 → GNN → PRoPHET",
    color: "var(--gw-orange)",
    tag: "Layer 04 · AI Routing",
    differentiator: true,
    ctaPrimary: { href: "#features", label: "See AI Routing card" },
    modalBody: (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold mb-3 text-[var(--gw-white)]">
            Four-layer routing
          </h3>
          <ul className="space-y-2 list-disc list-inside marker:text-[var(--gw-green)]">
            <li>
              <strong className="text-[var(--gw-white)]">L1 · LightGBM / ONNX</strong> —
              fast-path, &lt;1ms, AUC 1.0
            </li>
            <li>
              <strong className="text-[var(--gw-white)]">L2 · Gemma 4 via Ollama</strong> —
              analytical, 50–200ms, every 30s
            </li>
            <li>
              <strong className="text-[var(--gw-white)]">L3 · GNN</strong> — graph optimization,
              every 5 min
            </li>
            <li>
              <strong className="text-[var(--gw-white)]">L4 · PRoPHET</strong> — epidemic
              fallback, &lt;0.01ms, always available
            </li>
          </ul>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Real mesh data
          </h3>
          <p>
            The GNN layer is trained on the GuifiSants community mesh dataset — a real-world
            deployment with 63 nodes observed over 31 days.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Always-on fallback
          </h3>
          <p>
            If all ML layers fail, PRoPHET keeps the mesh working. The system degrades but never
            dies.
          </p>
        </div>
      </>
    ),
  },
  {
    number: "03",
    name: "CRYPTOGRAPHY",
    components:
      "XChaCha20-Poly1305 · X25519 · Ed25519 · ML-KEM-768 · BLAKE3 · Argon2 · Forward Secrecy Ratchet",
    color: "var(--gw-green)",
    tag: "Layer 03 · Cryptography",
    ctaPrimary: { href: "#security", label: "Security details" },
    modalBody: (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold mb-3 text-[var(--gw-white)]">
            Seven primitives
          </h3>
          <div className="border border-[var(--gw-border)] rounded overflow-hidden">
            <table className="w-full text-[13px]">
              <thead className="bg-[var(--gw-bg)]">
                <tr className="text-left font-mono-gw text-[11px] uppercase tracking-[0.08em] text-[var(--gw-white-faint)]">
                  <th className="px-3 py-2 font-normal">Primitive</th>
                  <th className="px-3 py-2 font-normal">Key</th>
                  <th className="px-3 py-2 font-normal">Standard</th>
                </tr>
              </thead>
              <tbody className="font-mono-gw text-[var(--gw-white-dim)]">
                <tr className="border-t border-[var(--gw-border)]">
                  <td className="px-3 py-2 text-[var(--gw-green)]">XChaCha20-Poly1305</td>
                  <td className="px-3 py-2">256-bit / 192b nonce</td>
                  <td className="px-3 py-2">RFC 8439 ext.</td>
                </tr>
                <tr className="border-t border-[var(--gw-border)]">
                  <td className="px-3 py-2 text-[var(--gw-green)]">X25519</td>
                  <td className="px-3 py-2">256-bit</td>
                  <td className="px-3 py-2">RFC 7748</td>
                </tr>
                <tr className="border-t border-[var(--gw-border)]">
                  <td className="px-3 py-2 text-[var(--gw-green)]">Ed25519</td>
                  <td className="px-3 py-2">256-bit</td>
                  <td className="px-3 py-2">RFC 8032</td>
                </tr>
                <tr className="border-t border-[var(--gw-border)]">
                  <td className="px-3 py-2 text-[var(--gw-green)]">ML-KEM-768</td>
                  <td className="px-3 py-2">PQ hybrid</td>
                  <td className="px-3 py-2">FIPS 203</td>
                </tr>
                <tr className="border-t border-[var(--gw-border)]">
                  <td className="px-3 py-2 text-[var(--gw-green)]">BLAKE3</td>
                  <td className="px-3 py-2">256-bit</td>
                  <td className="px-3 py-2">BLAKE3 spec</td>
                </tr>
                <tr className="border-t border-[var(--gw-border)]">
                  <td className="px-3 py-2 text-[var(--gw-green)]">Argon2id</td>
                  <td className="px-3 py-2">64 MiB / 3 iter</td>
                  <td className="px-3 py-2">RFC 9106</td>
                </tr>
                <tr className="border-t border-[var(--gw-border)]">
                  <td className="px-3 py-2 text-[var(--gw-green)]">Symmetric Ratchet</td>
                  <td className="px-3 py-2">forward secrecy</td>
                  <td className="px-3 py-2">FS ratchet</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Key conversion
          </h3>
          <p>
            Ed25519 and X25519 keys are interoperable via the{" "}
            <code className="font-mono-gw text-[var(--gw-green)]">edwards_to_montgomery</code>{" "}
            conversion — a single keypair serves both signing and key exchange.
          </p>
        </div>
      </>
    ),
  },
  {
    number: "02",
    name: "PLUGGABLE TRANSPORTS",
    components:
      "StealthTCP · WebRTC · BLE · LoRa · Reticulum · obfs4 · WS-Camo · SS-AEAD · TLS-Camo",
    color: "var(--gw-green)",
    tag: "Layer 02 · Transports",
    ctaPrimary: { href: "#how-it-works", label: "Transport details" },
    modalBody: (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold mb-3 text-[var(--gw-white)]">
            Nine transports
          </h3>
          <ul className="space-y-1.5 list-disc list-inside marker:text-[var(--gw-green)]">
            <li>
              <strong className="text-[var(--gw-white)]">StealthTCP</strong> — direct TCP with
              packet shaping
            </li>
            <li>
              <strong className="text-[var(--gw-white)]">WebRTC</strong> — browser-to-browser
              peer links
            </li>
            <li>
              <strong className="text-[var(--gw-white)]">BLE</strong> — short-range Bluetooth
              Low Energy
            </li>
            <li>
              <strong className="text-[var(--gw-white)]">LoRa</strong> — long-range
              low-bandwidth RF
            </li>
            <li>
              <strong className="text-[var(--gw-white)]">Reticulum</strong> — RNS-compatible
              overlay network
            </li>
            <li>
              <strong className="text-[var(--gw-white)]">obfs4</strong> — Tor-project pluggable
              transport
            </li>
            <li>
              <strong className="text-[var(--gw-white)]">WS-Camo</strong> — WebSocket
              Camouflage
            </li>
            <li>
              <strong className="text-[var(--gw-white)]">SS-AEAD</strong> — Shadowsocks AEAD
            </li>
            <li>
              <strong className="text-[var(--gw-white)]">TLS-Camo</strong> — TLS Camouflage
            </li>
          </ul>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Traffic Armor
          </h3>
          <p>
            Four anti-DPI transports (obfs4, WS-Camo, SS-AEAD, TLS-Camo) disguise packets as
            normal web traffic to defeat deep-packet inspection.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Build your own
          </h3>
          <p>
            Implement the{" "}
            <code className="font-mono-gw text-[var(--gw-green)]">Transport</code> trait to plug
            in any L2/L3 link.
          </p>
        </div>
      </>
    ),
  },
  {
    number: "01",
    name: "MESH NETWORKING",
    components:
      "libp2p GossipSub · Kademlia DHT · mDNS · DTN · PRoPHET Epidemic Routing",
    color: "var(--gw-green)",
    tag: "Layer 01 · Mesh Networking",
    modalBody: (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold mb-3 text-[var(--gw-white)]">
            Five mesh primitives
          </h3>
          <ul className="space-y-1.5 list-disc list-inside marker:text-[var(--gw-green)]">
            <li>
              <strong className="text-[var(--gw-white)]">libp2p GossipSub</strong> —
              topic-based publish/subscribe
            </li>
            <li>
              <strong className="text-[var(--gw-white)]">Kademlia DHT</strong> — O(log N) peer
              lookup and content addressing
            </li>
            <li>
              <strong className="text-[var(--gw-white)]">mDNS</strong> — LAN peer discovery
              without a bootstrap server
            </li>
            <li>
              <strong className="text-[var(--gw-white)]">DTN</strong> — store-carry-forward for
              disconnected nodes
            </li>
            <li>
              <strong className="text-[var(--gw-white)]">PRoPHET</strong> — epidemic routing
              with delivery predictability
            </li>
          </ul>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            No bootstrap needed
          </h3>
          <p>
            mDNS finds peers on the same LAN. Kademlia DHT finds peers across the mesh. No
            central bootstrap server required.
          </p>
        </div>
      </>
    ),
  },
];

const AIROUTING_LAYERS: AIRoutingLayer[] = [
  {
    n: "L1",
    name: "Fast Path",
    tech: "LightGBM / ONNX",
    latency: "<1ms per decision",
    desc: "Real-time anomaly detection. Classifies routing decisions at line rate. AUC 1.0 on held-out mesh data.",
    tag: "L1 · LightGBM Fast Path",
    modalBody: (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            What it does
          </h3>
          <p>
            Classifies routing decisions at line rate. The fast path runs on every packet
            decision, keeping mesh latency in the microsecond range. Trained on real mesh
            telemetry.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-3 text-[var(--gw-white)]">
            Performance
          </h3>
          <div className="border border-[var(--gw-border)] rounded overflow-hidden">
            <table className="w-full text-[13px]">
              <tbody className="font-mono-gw text-[var(--gw-white-dim)]">
                <tr className="border-b border-[var(--gw-border)]">
                  <td className="px-3 py-2 text-[var(--gw-white-faint)]">Inference</td>
                  <td className="px-3 py-2 text-[var(--gw-green)]">76.7 μs</td>
                </tr>
                <tr className="border-b border-[var(--gw-border)]">
                  <td className="px-3 py-2 text-[var(--gw-white-faint)]">AUC</td>
                  <td className="px-3 py-2 text-[var(--gw-green)]">1.0</td>
                </tr>
                <tr>
                  <td className="px-3 py-2 text-[var(--gw-white-faint)]">Model</td>
                  <td className="px-3 py-2 text-[var(--gw-green)]">ONNX export</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Fallback
          </h3>
          <p>If L1 is unavailable, L2 (Gemma 4) takes over analytical routing.</p>
        </div>
      </>
    ),
  },
  {
    n: "L2",
    name: "Analytical",
    tech: "Gemma 4 via Ollama",
    latency: "50–200ms, every 30s",
    desc: "LLM-driven reasoning over recent traffic. Adjusts routing strategy based on observed patterns.",
    tag: "L2 · Gemma 4 Analytical",
    modalBody: (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            What it does
          </h3>
          <p>
            LLM-driven reasoning over recent traffic patterns. The model analyses mesh telemetry
            and adjusts the routing strategy between L1 fast-path decisions.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Schedule
          </h3>
          <p>
            Runs every 30 seconds. Long enough for the LLM to think; short enough to react to
            mesh churn.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Model
          </h3>
          <p>
            Gemma 4 via Ollama — runs locally, fully private. No data ever leaves the node.
          </p>
        </div>
      </>
    ),
  },
  {
    n: "L3",
    name: "Graph Optimization",
    tech: "GNN on GuifiSants data",
    latency: "Every 5 min",
    desc: "Graph Neural Network trained on 7,931 real mesh samples recomputes optimal paths.",
    tag: "L3 · GNN Graph Optimization",
    modalBody: (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            What it does
          </h3>
          <p>
            Recomputes optimal paths across the graph. The GNN learns the mesh topology and
            predicts the most reliable next-hop for each destination.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Training data
          </h3>
          <p>
            Trained on the GuifiSants community mesh dataset — 7,931 samples across 63 nodes
            observed over 31 days. Real-world, not synthetic.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Crate
          </h3>
          <p>
            Lives in{" "}
            <code className="font-mono-gw text-[var(--gw-green)]">ghostwire-libs</code> as a
            PyTorch model invoked via FFI.
          </p>
        </div>
      </>
    ),
  },
  {
    n: "L4",
    name: "Fallback",
    tech: "PRoPHET epidemic routing",
    latency: "<0.01ms, always available",
    desc: "Mathematical epidemic routing formula. Runs when all other layers are unavailable. Degraded but never dead.",
    tag: "L4 · PRoPHET Fallback",
    modalBody: (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            What it does
          </h3>
          <p>
            PRoPHET — Probabilistic Routing Protocol using History of Encounters and
            Transitivity. A store-carry-forward epidemic router that needs no global view of the
            mesh.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Always-on
          </h3>
          <p>
            Runs when all other layers are unavailable. &lt;0.01ms decision time. Degraded, but
            never dead.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Bundle protocol
          </h3>
          <p>
            Uses DTN bundles with TTL, priority, and hop-count metadata so messages can survive
            multi-hop delay-tolerant delivery.
          </p>
        </div>
      </>
    ),
  },
];

const FLOW = [
  {
    n: "01",
    title: "Compose",
    text: "Sender writes message in the dashboard or via /api/v2/send. Identity keypair signs the payload.",
  },
  {
    n: "02",
    title: "Encrypt",
    text: "crypto_machine encrypts the payload with XChaCha20-Poly1305 AEAD. X25519 + ML-KEM-768 hybrid key agreement. Symmetric ratchet provides forward secrecy.",
  },
  {
    n: "03",
    title: "Route",
    text: "Four-layer routing stack picks the next hop. L1 fast-path decides in <1ms; L2/L3 refine periodically; L4 fallback always available.",
  },
  {
    n: "04",
    title: "Hop",
    text: "Packets traverse the mesh peer-to-peer. Sphinx onion routing hides source/destination from intermediate hops. Cover traffic masks timing.",
  },
  {
    n: "05",
    title: "Deliver",
    text: "Recipient decrypts with their identity key. If offline, message queues in their mailbox (delay-tolerant). Delivered when they reconnect.",
  },
];

const TECH_GROUPS = [
  {
    title: "Core",
    items: ["Rust", "libp2p", "Kademlia DHT", "Gossipsub", "Axum", "Tokio", "Tauri"],
  },
  {
    title: "Crypto",
    items: [
      "XChaCha20-Poly1305",
      "X25519",
      "Ed25519",
      "ML-KEM-768",
      "Sphinx (sphinx-rs)",
      "HKDF",
    ],
  },
  {
    title: "AI/ML",
    items: ["LightGBM", "ONNX Runtime", "GNN (PyTorch)", "Gemma 4 (Ollama)", "PRoPHET"],
  },
];

export function ArchitectureSection() {
  const [activeLayer, setActiveLayer] = useState<ActiveLayer | null>(null);

  return (
    <section id="architecture" className="py-24 border-t border-[var(--gw-border)]">
      <div className="max-w-[1160px] mx-auto px-6 sm:px-8">
        <div className="gw-section-label mb-3">{"// architecture"}</div>
        <h2 className="font-display text-3xl sm:text-5xl font-bold mb-6 text-[var(--gw-white)]">
          Architecture: Resilient Mesh Routing &amp; Messaging Flow
        </h2>
        <p className="text-[16px] text-[var(--gw-white-dim)] leading-relaxed max-w-3xl mb-12">
          GhostWire is structured as a layered Rust system: a memory-safe core built on libp2p,
          a four-layer AI routing stack, a multi-cipher crypto_machine module, and an Axum HTTP
          API surface for the dashboard and programmatic clients.
        </p>

        {/* Protocol Stack — 7 layers */}
        <div className="mb-16">
          <h3 className="font-display text-2xl font-bold mb-2 text-[var(--gw-white)]">
            Protocol stack — 7 layers
          </h3>
          <p className="text-[var(--gw-white-dim)] mb-8 max-w-2xl text-[15px]">
            From the mesh physical layer up to the application layer — every layer is modular and
            replaceable. Click any layer for deep detail.
          </p>
          <div className="space-y-2">
            {PROTOCOL_LAYERS.map((layer) => (
              <button
                key={layer.number}
                onClick={() =>
                  setActiveLayer({
                    tag: layer.tag,
                    title: `${layer.number} · ${layer.name}`,
                    body: layer.modalBody,
                    ctaPrimary: layer.ctaPrimary,
                  })
                }
                className="text-left bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-4 flex flex-col sm:flex-row sm:items-center gap-2 sm:gap-4 gw-card-hover focus:outline-none focus:border-[var(--gw-green)] w-full"
              >
                <div className="flex items-center gap-3 sm:w-48 shrink-0">
                  <span className="font-mono-gw text-[11px] text-[var(--gw-white-faint)]">
                    {layer.number}
                  </span>
                  <span
                    className="font-mono-gw text-[13px] font-semibold tracking-[0.08em] uppercase"
                    style={{ color: layer.color }}
                  >
                    {layer.name}
                  </span>
                </div>
                <div className="font-mono-gw text-[13px] text-[var(--gw-white-dim)] flex-1 flex items-center flex-wrap">
                  <span>{layer.components}</span>
                  {layer.differentiator && (
                    <span className="gw-in-progress ml-2">Differentiator</span>
                  )}
                </div>
                <span className="font-mono-gw text-[11px] text-[var(--gw-green)] ml-auto shrink-0">
                  Click for details →
                </span>
              </button>
            ))}
          </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-10 items-start mb-16">
          {/* Left column — System Layers */}
          <div>
            <div className="font-mono-gw text-[11px] tracking-[0.1em] uppercase text-[var(--gw-white-faint)] mb-4">
              System Layers
            </div>
            <div className="space-y-0">
              {AIROUTING_LAYERS.map((layer, i) => (
                <div key={layer.n}>
                  <button
                    onClick={() =>
                      setActiveLayer({
                        tag: layer.tag,
                        title: `${layer.n} · ${layer.name}`,
                        body: layer.modalBody,
                      })
                    }
                    className="text-left bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-5 gw-card-hover focus:outline-none focus:border-[var(--gw-green)] w-full flex flex-col"
                  >
                    <div className="flex items-baseline justify-between mb-2">
                      <div className="flex items-center gap-3">
                        <span className="font-mono-gw text-2xl font-bold text-[var(--gw-green)]">
                          {layer.n}
                        </span>
                        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">
                          {layer.name}
                        </h3>
                      </div>
                      <span className="font-mono-gw text-[11px] text-[var(--gw-white-faint)]">
                        {layer.latency}
                      </span>
                    </div>
                    <div className="font-mono-gw text-[12px] text-[var(--gw-green)] mb-2">
                      {layer.tech}
                    </div>
                    <p className="text-[14px] text-[var(--gw-white-dim)] leading-relaxed">
                      {layer.desc}
                    </p>
                    <span className="font-mono-gw text-[11px] text-[var(--gw-green)] mt-3 self-end shrink-0">
                      Click for details →
                    </span>
                  </button>
                  {i < AIROUTING_LAYERS.length - 1 && (
                    <div
                      className="h-3 w-px bg-[var(--gw-border-2)] mx-6"
                      aria-hidden="true"
                    />
                  )}
                </div>
              ))}
            </div>
          </div>

          {/* Right column — Message Flow */}
          <div>
            <div className="font-mono-gw text-[11px] tracking-[0.1em] uppercase text-[var(--gw-white-faint)] mb-4">
              Message Flow
            </div>
            <div className="space-y-3">
              {FLOW.map((step) => (
                <div
                  key={step.n}
                  className="flex gap-4 bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-5 gw-card-hover"
                >
                  <span className="font-mono-gw text-xl font-semibold text-[var(--gw-green)] shrink-0">
                    {step.n}
                  </span>
                  <div>
                    <h3 className="font-display text-base font-semibold mb-1 text-[var(--gw-white)]">
                      {step.title}
                    </h3>
                    <p className="text-[14px] text-[var(--gw-white-dim)] leading-relaxed">
                      {step.text}
                    </p>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>

        {/* Tech Stack Summary */}
        <div>
          <h3 className="font-display text-2xl font-bold mb-5 text-[var(--gw-white)]">
            Tech Stack Summary
          </h3>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-5">
            {TECH_GROUPS.map((group) => (
              <div
                key={group.title}
                className="bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-6 gw-card-hover"
              >
                <h4 className="font-display text-base font-semibold mb-4 text-[var(--gw-green)]">
                  {group.title}
                </h4>
                <div className="flex flex-wrap gap-2">
                  {group.items.map((item) => (
                    <span
                      key={item}
                      className="font-mono-gw text-[12px] px-3 py-1.5 border border-[var(--gw-border-2)] rounded text-[var(--gw-white-dim)]"
                    >
                      {item}
                    </span>
                  ))}
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>

      <FeatureModal
        open={activeLayer !== null}
        onClose={() => setActiveLayer(null)}
        tag={activeLayer?.tag ?? ""}
        title={activeLayer?.title ?? ""}
        ctaPrimary={activeLayer?.ctaPrimary}
      >
        {activeLayer?.body}
      </FeatureModal>
    </section>
  );
}
