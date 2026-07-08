import type { LucideIcon } from "lucide-react";
import { Code2, Globe, FileCode2 } from "lucide-react";

const TECH = [
  "Rust",
  "libp2p",
  "Kademlia DHT",
  "Gossipsub",
  "Axum",
  "Tokio",
  "XChaCha20-Poly1305",
  "X25519",
  "Ed25519",
  "ML-KEM-768",
  "Sphinx (sphinx-rs)",
  "HKDF",
  "LightGBM",
  "GNN",
  "Gemma 4",
  "ONNX",
  "PRoPHET",
  "Stealth TCP",
  "WebRTC",
  "Bluetooth LE",
  "LoRa",
  "obfs4",
  "Reticulum",
  "Tauri",
  "WebAssembly",
];

interface PublishedCrate {
  name: string;
  desc: string;
  status: string;
}

const PUBLISHED_CRATES: PublishedCrate[] = [
  {
    name: "sphinx-rs",
    desc: "Sphinx onion routing with SURBs",
    status: "✅ Published on crates.io",
  },
  {
    name: "ghostwire-dtn",
    desc: "Delay-tolerant networking (PRoPHET)",
    status: "🔄 In progress",
  },
  {
    name: "hlc-rs",
    desc: "Hybrid Logical Clocks",
    status: "🔄 In progress",
  },
  {
    name: "trust-store",
    desc: "Web of Trust + TOFU key management",
    status: "🔄 In progress",
  },
];

interface BuildWay {
  title: string;
  icon: LucideIcon;
  desc: string;
}

const BUILD_WAYS: BuildWay[] = [
  {
    title: "Rust Library",
    icon: Code2,
    desc: "Use the Rust crate directly. Add `ghostwire` to your Cargo.toml and build native applications with full access to the protocol stack.",
  },
  {
    title: "REST API",
    icon: Globe,
    desc: "Start a GhostWire node and interact via HTTP/WebSocket from any language. Axum-powered REST API with JWT auth, plus WebSocket for real-time events.",
  },
  {
    title: "Implement the Protocol",
    icon: FileCode2,
    desc: "Follow the protocol specification in docs/spec/ to build your own implementation in any language. GhostWire is an open protocol — anyone can implement it.",
  },
];

interface ComparisonRow {
  feature: string;
  ghostwire: boolean;
  libp2p: boolean;
  matrix: boolean;
  nostr: boolean;
  i2p: boolean;
}

const COMPARISON: ComparisonRow[] = [
  { feature: "P2P mesh", ghostwire: true, libp2p: true, matrix: false, nostr: false, i2p: true },
  { feature: "Built-in encryption", ghostwire: true, libp2p: true, matrix: true, nostr: true, i2p: true },
  { feature: "Anonymous routing", ghostwire: true, libp2p: false, matrix: false, nostr: false, i2p: true },
  { feature: "Delay-tolerant", ghostwire: true, libp2p: false, matrix: false, nostr: false, i2p: false },
  { feature: "AI-powered routing", ghostwire: true, libp2p: false, matrix: false, nostr: false, i2p: false },
  { feature: "Post-quantum", ghostwire: true, libp2p: false, matrix: false, nostr: false, i2p: false },
  { feature: "Multi-transport", ghostwire: true, libp2p: true, matrix: false, nostr: false, i2p: true },
  { feature: "Offline messaging", ghostwire: true, libp2p: false, matrix: true, nostr: false, i2p: false },
  { feature: "Censorship resistance", ghostwire: true, libp2p: false, matrix: false, nostr: false, i2p: true },
  { feature: "Coercion resistance", ghostwire: true, libp2p: false, matrix: false, nostr: false, i2p: false },
];

const COMPARISON_COLUMNS: { key: keyof Omit<ComparisonRow, "feature">; label: string }[] = [
  { key: "ghostwire", label: "GhostWire" },
  { key: "libp2p", label: "libp2p" },
  { key: "matrix", label: "Matrix" },
  { key: "nostr", label: "Nostr" },
  { key: "i2p", label: "I2P" },
];

export function AboutSection() {
  return (
    <section id="about" className="py-24 border-t border-[var(--gw-border)]">
      <div className="max-w-[1160px] mx-auto px-6 sm:px-8">
        <div className="gw-section-label mb-3">{"// about"}</div>
        <h1 className="font-display text-4xl sm:text-5xl font-bold mb-4 text-[var(--gw-white)]">
          About GhostWire
        </h1>
        <p className="text-[var(--gw-white-dim)] mb-12 max-w-3xl text-lg leading-relaxed">
          An open protocol for resilient peer-to-peer communication — not an app.
        </p>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-12">
          {/* Left column — Mission */}
          <div className="space-y-5 text-[15px] text-[var(--gw-white-dim)] leading-relaxed">
            <p>
              GhostWire is a decentralized networking protocol. It provides the
              cryptographic, routing, and transport layers that developers build
              communication tools on top of — not a single end-user application.
            </p>
            <p>
              The reference implementation is written in Rust for memory safety,
              powered by libp2p for P2P networking, hardened with
              XChaCha20-Poly1305 encryption and post-quantum ML-KEM-768 key
              exchange, and routed by a four-layer ML stack.
            </p>
            <p>
              The project is open-sourced under AGPL-3.0 so the architecture
              itself stays in the commons. A commercial license is available for
              teams that cannot use AGPL code. It is actively developed toward
              v1.0 stable.
            </p>
          </div>

          {/* Right column — Tech stack + Published Crates */}
          <div>
            <h3 className="font-display text-xl font-semibold mb-4 text-[var(--gw-white)]">
              Tech Stack
            </h3>
            <div className="flex flex-wrap gap-2 mb-10">
              {TECH.map((tech) => (
                <span
                  key={tech}
                  className="font-mono-gw text-[12px] px-3 py-1.5 border border-[var(--gw-border-2)] rounded text-[var(--gw-white-dim)] hover:text-[var(--gw-green)] hover:border-[var(--gw-green)] transition-colors"
                >
                  {tech}
                </span>
              ))}
            </div>

            <h3 className="font-display text-xl font-semibold mb-4 text-[var(--gw-white)]">
              Published Crates
            </h3>
            <div>
              {PUBLISHED_CRATES.map((crate) => (
                <div
                  key={crate.name}
                  className="flex items-baseline gap-3 mb-2.5"
                >
                  <span className="font-mono-gw text-[13px] text-[var(--gw-green)]">
                    {crate.name}
                  </span>
                  <span className="text-[13px] text-[var(--gw-white-dim)] flex-1">
                    {crate.desc}
                  </span>
                  <span className="font-mono-gw text-[11px] text-[var(--gw-white-faint)]">
                    {crate.status}
                  </span>
                </div>
              ))}
            </div>
          </div>
        </div>

        <h3 className="font-display text-2xl font-bold mb-2 mt-14 text-[var(--gw-white)]">
          Three ways to build on GhostWire
        </h3>
        <p className="text-[var(--gw-white-dim)] mb-8 max-w-2xl text-[15px]">
          GhostWire is a protocol, not just an app. Build on it three ways.
        </p>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          {BUILD_WAYS.map((way, idx) => {
            const Icon = way.icon;
            return (
              <div
                key={way.title}
                className="bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-6 gw-card-hover"
              >
                <div className="text-[var(--gw-green)] mb-3">
                  <Icon size={24} strokeWidth={1.75} aria-hidden="true" />
                </div>
                <h4 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
                  {way.title}
                </h4>
                <p className="text-[13px] text-[var(--gw-white-dim)] leading-relaxed mb-3">
                  {way.desc}
                </p>
                {idx === 0 && (
                  <code className="font-mono-gw text-[11px] text-[var(--gw-green)] block mt-3 bg-[var(--gw-bg)] p-3 rounded border border-[var(--gw-border)]">
                    ghostwire = {"{ git = \"https://github.com/Phantomojo/GhostWire-secure-mesh-communication.git\" }"}
                  </code>
                )}
                {idx === 1 && (
                  <a
                    href="https://github.com/Phantomojo/GhostWire-secure-mesh-communication/blob/main/docs/API_QUICK_REFERENCE.md"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="font-mono-gw text-[12px] text-[var(--gw-green)] hover:text-[var(--gw-white)] transition-colors inline-block mt-2"
                  >
                    API Quick Reference →
                  </a>
                )}
                {idx === 2 && (
                  <a
                    href="https://github.com/Phantomojo/GhostWire-secure-mesh-communication/tree/main/docs/spec"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="font-mono-gw text-[12px] text-[var(--gw-green)] hover:text-[var(--gw-white)] transition-colors inline-block mt-2"
                  >
                    Protocol Spec →
                  </a>
                )}
              </div>
            );
          })}
        </div>

        <h3 className="font-display text-2xl font-bold mb-2 mt-14 text-[var(--gw-white)]">
          GhostWire vs other protocols
        </h3>
        <p className="text-[var(--gw-white-dim)] mb-8 text-[15px]">
          How GhostWire compares to libp2p, Matrix, Nostr, and I2P.
        </p>
        <div className="overflow-x-auto border border-[var(--gw-border)] rounded-lg">
          <table className="w-full border-collapse">
            <thead>
              <tr className="bg-[var(--gw-bg-3)]">
                <th className="font-mono-gw text-[12px] tracking-[0.06em] uppercase text-[var(--gw-white-faint)] py-3 px-4 border-b border-[var(--gw-border-2)] text-left">
                  Feature
                </th>
                {COMPARISON_COLUMNS.map((col) => (
                  <th
                    key={col.key}
                    className={`font-mono-gw text-[12px] tracking-[0.06em] uppercase py-3 px-4 border-b border-[var(--gw-border-2)] ${
                      col.key === "ghostwire"
                        ? "text-[var(--gw-green)]"
                        : "text-[var(--gw-white-faint)]"
                    }`}
                  >
                    {col.label}
                  </th>
                ))}
              </tr>
            </thead>
            <tbody>
              {COMPARISON.map((row) => (
                <tr
                  key={row.feature}
                  className="border-b border-[var(--gw-border)] hover:bg-[var(--gw-bg-2)] transition-colors last:border-b-0"
                >
                  <td className="text-[var(--gw-white)] font-medium text-[13px] py-3 px-4">
                    {row.feature}
                  </td>
                  {COMPARISON_COLUMNS.map((col) => (
                    <td
                      key={col.key}
                      className={`text-center text-[13px] py-3 px-4 ${
                        row[col.key]
                          ? "text-[var(--gw-green)]"
                          : "text-[var(--gw-white-faint)]"
                      }`}
                    >
                      {row[col.key] ? "✅" : "❌"}
                    </td>
                  ))}
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </section>
  );
}
