import type { LucideIcon } from "lucide-react";
import {
  Code2,
  BrainCircuit,
  LayoutDashboard,
  ShieldCheck,
  Cpu,
  BookOpen,
} from "lucide-react";

interface Role {
  title: string;
  icon: LucideIcon;
  desc: string;
  skills: string[];
  firstIssues: string;
}

const ROLES: Role[] = [
  {
    title: "Rust Core",
    icon: Code2,
    desc: "Work on the libp2p networking stack, the XChaCha20-Poly1305 crypto module, the modular transport layer, and the four-layer AI routing pipeline.",
    skills: ["Rust", "async/Tokio", "libp2p", "cryptography", "networking"],
    firstIssues: "transport-trait refactor, stealth-tcp hardening",
  },
  {
    title: "AI/ML Routing",
    icon: BrainCircuit,
    desc: "Improve the LightGBM L1 classifier, retrain the GNN L3 on new datasets, evaluate Gemma 4 L2 prompts, benchmark PRoPHET L4 fallback. Python + Rust via ONNX Runtime.",
    skills: ["Python", "PyTorch", "LightGBM", "ONNX", "GNNs"],
    firstIssues: "GNN evaluation harness, new dataset integration",
  },
  {
    title: "Frontend & Applications",
    icon: LayoutDashboard,
    desc: "Build the React web UI, the Rust TUI, and the CLI. Improve mesh topology visualization and responsive layouts. Support the Tauri desktop wrapper.",
    skills: ["React", "TypeScript", "Rust (ratatui)", "Tauri", "data viz"],
    firstIssues: "TUI dashboard polish, mobile web layout",
  },
  {
    title: "Security & Crypto",
    icon: ShieldCheck,
    desc: "Review the cryptographic stack, audit Sphinx onion routing, harden coercion resistance, implement key revocation/rotation, model new threats.",
    skills: [
      "applied cryptography",
      "threat modeling",
      "Rust",
      "protocol design",
    ],
    firstIssues: "Sphinx packet-format review, key-revocation design",
  },
  {
    title: "Embedded & LoRa",
    icon: Cpu,
    desc: "Work on ESP32 firmware adapters, prototype LoRa mesh bridges (Meshtastic T-Beam), design low-power off-grid node setups.",
    skills: ["embedded Rust", "ESP32", "LoRa", "low-power design"],
    firstIssues: "ESP32 transport stub, Meshtastic bridge PoC",
  },
  {
    title: "Docs & Community",
    icon: BookOpen,
    desc: "Write technical documentation, tutorials, and the OpenAPI 3.1 spec. Triage issues, review PRs, welcome new contributors.",
    skills: ["technical writing", "Markdown", "Git", "dev relations"],
    firstIssues: "OpenAPI spec, install guide for Windows",
  },
];

const CONTRIBUTING_URL =
  "https://github.com/Phantomojo/GhostWire-secure-mesh-communication/blob/main/ghostwire/CONTRIBUTING.md";
const GOOD_FIRST_ISSUES_URL =
  "https://github.com/Phantomojo/GhostWire-secure-mesh-communication/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22";

export function ContributorsSection() {
  return (
    <section id="contributors" className="py-24 border-t border-[var(--gw-border)]">
      <div className="max-w-[1160px] mx-auto px-6 sm:px-8">
        <div className="gw-section-label mb-3">{"// contributors"}</div>
        <h2 className="font-display text-3xl sm:text-5xl font-bold mb-6 text-[var(--gw-white)]">
          Build the mesh with us
        </h2>
        <p className="text-[16px] text-[var(--gw-white-dim)] leading-relaxed max-w-3xl mb-12">
          GhostWire is AGPL-3.0 open source. Here are the roles where help moves
          the project toward v1.0.
        </p>

        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-5">
          {ROLES.map((role) => {
            const Icon = role.icon;
            return (
              <div
                key={role.title}
                className="bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-6 gw-card-hover"
              >
                <div className="text-[var(--gw-green)] mb-4" aria-hidden="true">
                  <Icon className="w-7 h-7" strokeWidth={1.6} />
                </div>
                <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
                  {role.title}
                </h3>
                <p className="text-[13px] text-[var(--gw-white-dim)] leading-relaxed mb-4">
                  {role.desc}
                </p>
                <div className="mb-3">
                  <div className="font-mono-gw text-[10px] tracking-[0.1em] uppercase text-[var(--gw-white-faint)] mb-1.5">
                    Skills
                  </div>
                  <div className="flex flex-wrap gap-1.5">
                    {role.skills.map((s) => (
                      <span
                        key={s}
                        className="font-mono-gw text-[11px] px-2 py-0.5 border border-[var(--gw-border-2)] rounded text-[var(--gw-white-dim)]"
                      >
                        {s}
                      </span>
                    ))}
                  </div>
                </div>
                <div>
                  <div className="font-mono-gw text-[10px] tracking-[0.1em] uppercase text-[var(--gw-white-faint)] mb-1.5">
                    Good first issues
                  </div>
                  <p className="text-[12px] text-[var(--gw-green)] font-mono-gw">
                    {role.firstIssues}
                  </p>
                </div>
              </div>
            );
          })}
        </div>

        <div className="mt-10 bg-gradient-to-r from-[var(--gw-bg-2)] to-[var(--gw-bg-3)] border border-[var(--gw-border-2)] rounded-lg p-7 sm:p-9 text-center">
          <h3 className="font-display text-2xl font-bold mb-2 text-[var(--gw-white)]">
            Ready to contribute?
          </h3>
          <p className="text-[var(--gw-white-dim)] mb-5 max-w-xl mx-auto">
            Read CONTRIBUTING.md, pick a good first issue, and say hi. Code of
            conduct enforced via admin@ghostwire.cc.
          </p>
          <div className="flex flex-wrap gap-3 justify-center">
            <a
              href={CONTRIBUTING_URL}
              target="_blank"
              rel="noopener noreferrer"
              className="gw-btn-primary"
            >
              Read Contributing Guide
            </a>
            <a
              href={GOOD_FIRST_ISSUES_URL}
              target="_blank"
              rel="noopener noreferrer"
              className="gw-btn-ghost"
            >
              Browse Good First Issues
            </a>
          </div>
        </div>
      </div>
    </section>
  );
}
