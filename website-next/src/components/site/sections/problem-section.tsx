'use client';

import { useState } from "react";
import { FeatureModal } from "@/components/site/feature-modal";

interface ProblemStat {
  id: string;
  num: string;
  label: string;
  sub: string;
  tag: string;
  title: string;
  ctaPrimary?: { href: string; label: string };
}

const STATS: ProblemStat[] = [
  {
    id: "gap",
    num: "2.6 billion",
    label: "people lack reliable internet access",
    sub: "When disasters strike or communities are remote, traditional communication breaks down.",
    tag: "context",
    title: "The connectivity gap",
  },
  {
    id: "servers",
    num: "0 servers",
    label: "in GhostWire's architecture",
    sub: "No central point to seize, subpoena, or shut down. Pure peer-to-peer.",
    tag: "architecture",
    title: "No central servers",
  },
  {
    id: "layers",
    num: "4 layers",
    label: "of AI-powered routing",
    sub: "LightGBM → Gemma 4 → GNN → PRoPHET. Optimal paths, always-on fallback.",
    tag: "routing",
    title: "Four-layer AI routing",
    ctaPrimary: { href: "#architecture", label: "Architecture details" },
  },
  {
    id: "transports",
    num: "9 transports",
    label: "transports, with auto-failover",
    sub: "Stealth TCP, WebRTC, BLE, LoRa, obfs4, Reticulum. WiFi Direct in dev.",
    tag: "transports",
    title: "Nine pluggable transports",
    ctaPrimary: { href: "#how-it-works", label: "Transport details" },
  },
];

function ModalBody({ id }: { id: string }) {
  switch (id) {
    case "gap":
      return (
        <>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              The number
            </h3>
            <p>
              2.6 billion people — roughly one-third of humanity — lack reliable
              internet access. (ITU 2023 estimate.)
            </p>
          </div>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              Who&apos;s affected
            </h3>
            <ul className="space-y-1.5 list-none p-0 m-0">
              <li>• Rural communities (last in line for telecom investment)</li>
              <li>• Disaster zones (infrastructure destroyed)</li>
              <li>• Politically isolated regions (censorship/shutdowns)</li>
              <li>• Low-income communities (affordability)</li>
              <li>• Refugees and displaced people</li>
            </ul>
          </div>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              Why it matters
            </h3>
            <p>
              Connectivity is how people coordinate aid, reach emergency
              services, access education, organize politically, and stay in
              touch with family. When it fails, the most vulnerable are hit
              hardest.
            </p>
          </div>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              What GhostWire does
            </h3>
            <p>
              GhostWire runs without any central infrastructure. Devices connect
              peer-to-peer. If you have battery power and two devices, you have
              a network.
            </p>
          </div>
        </>
      );
    case "servers":
      return (
        <>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              What this means
            </h3>
            <p>
              GhostWire has zero central servers. No login server, no message
              relay, no directory, no key server. Every node is equal.
            </p>
          </div>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              Why it matters
            </h3>
            <ul className="space-y-1.5 list-none p-0 m-0">
              <li>• No server to seize (physical)</li>
              <li>• No server to subpoena (legal)</li>
              <li>• No server to shut down (censorship)</li>
              <li>• No server to breach (security)</li>
              <li>• No server to meter access (economic)</li>
            </ul>
          </div>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              How it works
            </h3>
            <p>
              libp2p peer discovery (Kademlia DHT + mDNS), GossipSub pub/sub,
              Sphinx onion routing. Messages hop device-to-device.
            </p>
          </div>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              Trade-offs
            </h3>
            <p>
              P2P means you can&apos;t send a message to someone who&apos;s
              offline AND unreachable by any mesh path — but delay-tolerant
              delivery queues it for when they reconnect.
            </p>
          </div>
        </>
      );
    case "layers":
      return (
        <>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              The stack
            </h3>
            <p>
              Four routing layers run in parallel. The fastest layer that&apos;s
              available wins; the rest fall back.
            </p>
          </div>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              L1 — LightGBM (Fast Path)
            </h3>
            <p>
              &lt;1ms per decision. ONNX-classified. AUC 1.0 on held-out mesh
              data. Anomaly detection at line rate.
            </p>
          </div>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              L2 — Gemma 4 (Analytical)
            </h3>
            <p>
              50-200ms, runs every 30 seconds. LLM-driven reasoning over recent
              traffic via local Ollama.
            </p>
          </div>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              L3 — GNN (Graph Optimization)
            </h3>
            <p>
              Runs every 5 minutes. Graph Neural Network trained on 7,931 real
              GuifiSants mesh samples (63 nodes, 31 days).
            </p>
          </div>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              L4 — PRoPHET (Fallback)
            </h3>
            <p>
              &lt;0.01ms, always available. Epidemic store-carry-forward
              routing. Degraded but never dead.
            </p>
          </div>
        </>
      );
    case "transports":
      return (
        <>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              5 Core transports
            </h3>
            <ul className="space-y-1.5 list-none p-0 m-0">
              <li>• Stealth TCP (fingerprint-resistant)</li>
              <li>• WebRTC (browser P2P + NAT traversal)</li>
              <li>• Bluetooth LE (short-range device mesh)</li>
              <li>• LoRa (2-15 km radio)</li>
              <li>• Reticulum (meta-transport over any interface)</li>
            </ul>
          </div>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              4 Traffic Armor (anti-DPI)
            </h3>
            <ul className="space-y-1.5 list-none p-0 m-0">
              <li>• obfs4 (random-noise obfuscation)</li>
              <li>• WebSocket Camouflage (looks like web traffic)</li>
              <li>• Shadowsocks AEAD (proxy + AEAD)</li>
              <li>• TLS Camouflage (looks like HTTPS)</li>
            </ul>
          </div>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              Auto-failover
            </h3>
            <p>
              The transport layer picks the best available link and fails over
              when one drops. Implement the Transport trait to plug in your own.
            </p>
          </div>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              Protocol bridges (experimental)
            </h3>
            <p>
              Briar, Meshtastic, Matrix bridges let GhostWire interoperate with
              those networks.
            </p>
          </div>
        </>
      );
    default:
      return null;
  }
}

export function ProblemSection() {
  const [active, setActive] = useState<ProblemStat | null>(null);

  return (
    <section id="problem" className="py-24 border-t border-[var(--gw-border)]">
      <div className="max-w-[1160px] mx-auto px-6 sm:px-8">
        <div className="max-w-3xl">
          <div className="gw-section-label mb-5">{"// the problem"}</div>
          <blockquote className="font-display text-2xl sm:text-[32px] leading-[1.35] font-semibold text-[var(--gw-white)]">
            <span className="text-[var(--gw-green)]">2.6 billion people</span> lack reliable
            internet access. When disasters strike, infrastructure fails, or communities are remote
            — traditional communication breaks down precisely when coordination is most critical.
            GhostWire exists because connectivity is a human right, not a luxury.
          </blockquote>
        </div>

        <h3 className="font-display text-2xl font-bold mb-6 text-[var(--gw-white)] mt-12">
          Why this matters
        </h3>
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
          {STATS.map((s) => (
            <button
              key={s.id}
              onClick={() => setActive(s)}
              className="text-left bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-6 gw-card-hover focus:outline-none focus:border-[var(--gw-green)]"
            >
              <div className="font-mono-gw text-3xl font-bold text-[var(--gw-green)] mb-2">
                {s.num}
              </div>
              <div className="font-display text-base font-semibold text-[var(--gw-white)] mb-2">
                {s.label}
              </div>
              <p className="text-[13px] text-[var(--gw-white-dim)] leading-relaxed mb-3">
                {s.sub}
              </p>
              <span className="font-mono-gw text-[12px] text-[var(--gw-green)]">
                Click for details →
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
      >
        {active ? <ModalBody id={active.id} /> : null}
      </FeatureModal>
    </section>
  );
}
