'use client';

import { useState } from "react";
import { FeatureModal } from "@/components/site/feature-modal";

interface RoadmapColumn {
  label: "NOW" | "NEXT" | "LATER";
  subtitle: string;
  color: string;
  bulletColor: string;
  badge?: string;
  items: string[];
  tag: string;
  title: string;
  ctaPrimary?: { href: string; label: string };
}

const COLUMNS: RoadmapColumn[] = [
  {
    label: "NOW",
    subtitle: "Working in v0.2.0",
    color: "var(--gw-green)",
    bulletColor: "var(--gw-green)",
    items: [
      "Transport layer — libp2p, Iroh, Bluetooth LE, obfs4 (8/10)",
      "Encryption — XChaCha20-Poly1305, X25519, ML-KEM-768 (8/10)",
      "Security — SecurityManager, audit, rate limiting, CVE patches (8/10)",
      "Storage — MessageStore, Mailbox, BlockStore (7/10)",
      "API — Axum REST, WebSocket, JWT (7/10)",
      "Applications — TUI, CLI, Web UI (7/10)",
      "Health monitoring — Ping/Pong, RTT, liveness (7/10)",
      "Forward secrecy — symmetric ratchet (8/10)",
      "Messaging — ACK, ordering, retry, dedup, TTL (7/10)",
    ],
    tag: "v0.2.0",
    title: "Working now — v0.2.0",
    ctaPrimary: { href: "#quickstart", label: "Quick Start" },
  },
  {
    label: "NEXT",
    subtitle: "In progress",
    color: "var(--gw-orange)",
    bulletColor: "var(--gw-orange)",
    badge: "In Progress",
    items: [
      "Identity — key revocation & rotation (6/10)",
      "Privacy — adaptive cover traffic (6/10)",
      "Network — DHT integration, peer scoring (6/10)",
      "Social — groups & reputation (5/10)",
      "AI/ML — routing accuracy (5/10)",
      "Message chunking — large message support",
      "Key revocation — gossip-based CRL propagation",
      "Key rotation — automatic on schedule",
    ],
    tag: "in progress",
    title: "In active development",
    ctaPrimary: { href: "#contributors", label: "Contribute" },
  },
  {
    label: "LATER",
    subtitle: "Planned for v1.0",
    color: "var(--gw-white-faint)",
    bulletColor: "var(--gw-white-faint)",
    items: [
      "Phase 3: Peer scoring & bandwidth estimation (2-3 weeks)",
      "Phase 3: DHT integration for peer discovery (2-3 weeks)",
      "Phase 4: Fuzzing setup with cargo-fuzz (2-3 weeks)",
      "Phase 4: OpenAPI 3.1 spec (2-3 weeks)",
      "Phase 4: Property-based tests with proptest (2-3 weeks)",
      "Phase 5: Contact verification & trust (1-2 weeks)",
      "Phase 5: Message compression (1-2 weeks)",
      "Phase 5: Storage backup/restore (1-2 weeks)",
      "Phase 5: Performance benchmarks (1-2 weeks)",
    ],
    tag: "v1.0 plan",
    title: "Planned for v1.0 stable",
    ctaPrimary: {
      href: "https://github.com/Phantomojo/GhostWire-secure-mesh-communication/blob/main/ghostwire/ROADMAP.md",
      label: "Full ROADMAP.md",
    },
  },
];

function ModalBody({ label }: { label: "NOW" | "NEXT" | "LATER" }) {
  if (label === "NOW") {
    const rows: Array<{ item: string; score: string }> = [
      { item: "Transport (libp2p, Iroh, BLE, obfs4)", score: "8/10" },
      { item: "Encryption (XChaCha20-Poly1305, X25519, ML-KEM-768)", score: "8/10" },
      { item: "Security (SecurityManager, audit, rate limit, CVE patches)", score: "8/10" },
      { item: "Storage (MessageStore, Mailbox, BlockStore)", score: "7/10" },
      { item: "API (Axum REST, WebSocket, JWT)", score: "7/10" },
      { item: "Applications (TUI, CLI, Web UI)", score: "7/10" },
      { item: "Health monitoring (Ping/Pong, RTT, liveness)", score: "7/10" },
      { item: "Forward secrecy (symmetric ratchet)", score: "8/10" },
      { item: "Messaging (ACK, ordering, retry, dedup, TTL)", score: "7/10" },
    ];
    return (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
            Current state
          </h3>
          <p>
            GhostWire v0.2.0 is live and actively developed. Overall score 7/10
            (B-). These systems are working and tested.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
            What works
          </h3>
          <div className="overflow-x-auto border border-[var(--gw-border)] rounded-md">
            <table className="w-full text-left border-collapse">
              <thead>
                <tr>
                  <th className="font-mono-gw text-[10px] tracking-[0.08em] uppercase text-[var(--gw-white-faint)] border-b border-[var(--gw-border-2)] py-2 px-3">
                    System
                  </th>
                  <th className="font-mono-gw text-[10px] tracking-[0.08em] uppercase text-[var(--gw-white-faint)] border-b border-[var(--gw-border-2)] py-2 px-3">
                    Score
                  </th>
                </tr>
              </thead>
              <tbody>
                {rows.map((r) => (
                  <tr key={r.item} className="border-b border-[var(--gw-border)] last:border-b-0">
                    <td className="py-2 px-3 text-[13px] text-[var(--gw-white)]">{r.item}</td>
                    <td className="py-2 px-3 text-[13px] text-[var(--gw-green)] font-mono-gw">
                      {r.score}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
            Try it
          </h3>
          <pre className="font-mono-gw text-[13px] text-[var(--gw-green)] bg-[var(--gw-bg)] border border-[var(--gw-border)] rounded-md p-3 overflow-x-auto">
{`cargo run --release  →  http://127.0.0.1:8080`}
          </pre>
        </div>
      </>
    );
  }
  if (label === "NEXT") {
    return (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
            What&apos;s being built
          </h3>
          <p>
            These systems are partially implemented. Expect rapid progress
            toward v0.3.0 and v0.4.0.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
            In-progress items
          </h3>
          <ul className="space-y-1.5 list-none p-0 m-0">
            <li>• Identity — key revocation &amp; rotation (6/10)</li>
            <li>• Privacy — adaptive cover traffic (6/10)</li>
            <li>• Network — DHT integration, peer scoring (6/10)</li>
            <li>• Social — groups &amp; reputation (5/10)</li>
            <li>• AI/ML — routing accuracy (5/10)</li>
            <li>• Message chunking — large message support</li>
            <li>• Key revocation — gossip-based CRL propagation</li>
            <li>• Key rotation — automatic on schedule</li>
          </ul>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
            How to help
          </h3>
          <p>
            These are exactly the areas where contributors move the project
            forward. See the Contributors section.
          </p>
        </div>
      </>
    );
  }
  // LATER
  return (
    <>
      <div>
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
          The path to v1.0
        </h3>
        <p>
          Five phases remain to reach v1.0 stable — a reliable, secure,
          production-ready messaging platform.
        </p>
      </div>
      <div>
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
          Phase 3 — Network Hardening (2-3 weeks)
        </h3>
        <ul className="space-y-1.5 list-none p-0 m-0">
          <li>• Peer scoring</li>
          <li>• Bandwidth estimation</li>
          <li>• PrivacyParams</li>
          <li>• DHT integration</li>
        </ul>
      </div>
      <div>
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
          Phase 4 — Testing &amp; Documentation (2-3 weeks)
        </h3>
        <ul className="space-y-1.5 list-none p-0 m-0">
          <li>• cargo-fuzz</li>
          <li>• OpenAPI 3.1 spec</li>
          <li>• SECURITY.md</li>
          <li>• Property-based tests (proptest)</li>
          <li>• Integration tests</li>
        </ul>
      </div>
      <div>
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
          Phase 5 — Polish (1-2 weeks)
        </h3>
        <ul className="space-y-1.5 list-none p-0 m-0">
          <li>• Contact verification</li>
          <li>• Message compression</li>
          <li>• Storage backup/restore</li>
          <li>• Performance benchmarks</li>
        </ul>
      </div>
      <div>
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
          Release schedule
        </h3>
        <p>
          v0.2.0 (current) → v0.3.0 (Phase 1) → v0.4.0 (Phase 2) →
          v1.0.0-alpha → v1.0.0-beta → v1.0.0 (Phase 5).
        </p>
      </div>
    </>
  );
}

export function RoadmapSection() {
  const [active, setActive] = useState<RoadmapColumn | null>(null);

  return (
    <section id="roadmap" className="py-24 border-t border-[var(--gw-border)]">
      <div className="max-w-[1160px] mx-auto px-6 sm:px-8">
        <div className="gw-section-label mb-3">{"// roadmap"}</div>
        <h2 className="font-display text-3xl sm:text-5xl font-bold mb-6 text-[var(--gw-white)]">
          Roadmap to v1.0 stable
        </h2>
        <p className="text-[16px] text-[var(--gw-white-dim)] leading-relaxed max-w-3xl mb-12">
          Current version v0.2.0 — overall score 7/10 (B-). Here&apos;s what works
          today, what&apos;s in progress, and what&apos;s planned for v1.0.
        </p>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          {COLUMNS.map((col) => (
            <button
              key={col.label}
              onClick={() => setActive(col)}
              className="text-left bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-6 w-full gw-card-hover focus:outline-none focus:border-[var(--gw-green)] h-full flex flex-col"
            >
              <div className="flex items-center gap-2.5 mb-1">
                <span
                  className="w-2 h-2 rounded-full"
                  style={{ background: col.color }}
                  aria-hidden="true"
                />
                <span className="font-mono-gw text-sm font-semibold tracking-[0.12em] text-[var(--gw-white)]">
                  {col.label}
                </span>
              </div>
              <p className="text-[12px] text-[var(--gw-white-faint)] mb-5">
                {col.subtitle}
              </p>
              <ul className="space-y-2.5 list-none p-0 m-0 flex-1">
                {col.items.map((it) => (
                  <li
                    key={it}
                    className="flex items-start gap-2 text-[13px] text-[var(--gw-white-dim)] leading-relaxed"
                  >
                    <span
                      className="font-mono-gw mt-0.5"
                      style={{ color: col.bulletColor }}
                      aria-hidden="true"
                    >
                      ▸
                    </span>
                    <span>
                      {it}
                      {col.badge && (
                        <span className="gw-in-progress ml-2">{col.badge}</span>
                      )}
                    </span>
                  </li>
                ))}
              </ul>
              <div className="mt-4 pt-4 border-t border-[var(--gw-border)]">
                <span className="font-mono-gw text-[12px] text-[var(--gw-green)]">
                  Click for phase details →
                </span>
              </div>
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
        {active ? <ModalBody label={active.label} /> : null}
      </FeatureModal>
    </section>
  );
}
