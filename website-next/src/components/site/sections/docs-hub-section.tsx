interface DocLink {
  title: string;
  desc: string;
  href: string;
}

interface DocGroup {
  label: string;
  docs: DocLink[];
}

const REPO_BASE =
  "https://github.com/Phantomojo/GhostWire-secure-mesh-communication/blob/main/";

const DOC_GROUPS: DocGroup[] = [
  {
    label: "Getting Started",
    docs: [
      {
        title: "Quick Start",
        desc: "Clone, build, and run a GhostWire node in 5 minutes.",
        href: `${REPO_BASE}docs/QUICK_START.md`,
      },
      {
        title: "Install Guide",
        desc: "Full installation instructions for Linux, macOS, Windows, Docker.",
        href: `${REPO_BASE}docs/INSTALL.md`,
      },
      {
        title: "How To Start GhostWire",
        desc: "Step-by-step node startup, configuration, and first-message walkthrough.",
        href: `${REPO_BASE}docs/HOW_TO_START_GHOSTWIRE.md`,
      },
      {
        title: "API Quick Reference",
        desc: "REST + WebSocket API endpoints at /api/v2/.",
        href: `${REPO_BASE}docs/API_QUICK_REFERENCE.md`,
      },
    ],
  },
  {
    label: "Architecture & Security",
    docs: [
      {
        title: "Architecture Guide",
        desc: "System overview, core components, transport layer, data flow.",
        href: `${REPO_BASE}docs/ARCHITECTURE.md`,
      },
      {
        title: "Security Policy",
        desc: "Threat model, responsible disclosure, PGP key.",
        href: `${REPO_BASE}docs/SECURITY.md`,
      },
      {
        title: "Security Audit Report",
        desc: "February 2026 internal audit. All critical issues resolved. A+ rating.",
        href: `${REPO_BASE}docs/SECURITY_AUDIT.md`,
      },
      {
        title: "Comprehensive Code Review",
        desc: "Full codebase review and recommendations.",
        href: `${REPO_BASE}docs/COMPREHENSIVE_CODE_REVIEW.md`,
      },
    ],
  },
  {
    label: "Protocol Specification",
    docs: [
      {
        title: "Protocol Overview",
        desc: "v0.2.0-draft living specification. The authoritative 7-layer model.",
        href: `${REPO_BASE}docs/spec/PROTOCOL.md`,
      },
      {
        title: "Cryptography Spec",
        desc: "XChaCha20-Poly1305, X25519, Ed25519, ML-KEM-768, BLAKE3, Argon2.",
        href: `${REPO_BASE}docs/spec/CRYPTO.md`,
      },
      {
        title: "Pluggable Transports Spec",
        desc: "9 transports: 5 core + 4 Traffic Armor. The Transport trait.",
        href: `${REPO_BASE}docs/spec/TRANSPORT.md`,
      },
      {
        title: "Sphinx Routing Spec",
        desc: "Onion routing with SURBs. Hop counts, session key derivation.",
        href: `${REPO_BASE}docs/spec/SPHINX.md`,
      },
      {
        title: "DTN Spec",
        desc: "Delay-tolerant networking. Bundle protocol, PRoPHET, Spray-and-Wait.",
        href: `${REPO_BASE}docs/spec/DTN.md`,
      },
      {
        title: "AI Routing Spec",
        desc: "4-layer ML stack: LightGBM, Gemma 4, GNN, PRoPHET.",
        href: `${REPO_BASE}docs/spec/ROUTING.md`,
      },
      {
        title: "Identity Spec",
        desc: "Ed25519 identities, TOFU, Web of Trust, device pairing.",
        href: `${REPO_BASE}docs/spec/IDENTITY.md`,
      },
      {
        title: "Peer Discovery Spec",
        desc: "Kademlia DHT, mDNS, rendezvous.",
        href: `${REPO_BASE}docs/spec/DISCOVERY.md`,
      },
    ],
  },
  {
    label: "Community",
    docs: [
      {
        title: "Contributing Guide",
        desc: "How to contribute. Code, docs, testing, roles.",
        href: `${REPO_BASE}docs/CONTRIBUTING.md`,
      },
      {
        title: "Code of Conduct",
        desc: "Community standards. Enforced via admin@ghostwire.cc.",
        href: `${REPO_BASE}docs/CODE_OF_CONDUCT.md`,
      },
      {
        title: "Governance",
        desc: "Project governance and decision-making.",
        href: `${REPO_BASE}docs/GOVERNANCE.md`,
      },
      {
        title: "Manifesto",
        desc: "The First Root of Yggdrasil. The philosophy behind the protocol.",
        href: `${REPO_BASE}docs/GHOSTWIRE_MANIFESTO.md`,
      },
    ],
  },
];

export function DocsHubSection() {
  return (
    <section id="docs" className="py-24 border-t border-[var(--gw-border)]">
      <div className="max-w-7xl mx-auto px-6">
        <div className="gw-section-label mb-3">{"// docs"}</div>
        <h2 className="font-display text-3xl md:text-4xl font-bold text-[var(--gw-white)] mb-3">
          Documentation — read the source of truth
        </h2>
        <p className="text-[15px] text-[var(--gw-white-dim)] max-w-2xl mb-12 leading-relaxed">
          Full docs live in the repository. Here&apos;s the index — every link
          opens the canonical Markdown on GitHub.
        </p>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {DOC_GROUPS.map((group) => (
            <div key={group.label}>
              <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-1">
                {group.label}
              </h3>
              <p className="text-[12px] text-[var(--gw-white-faint)] mb-4">
                {group.docs.length} docs
              </p>
              <div className="space-y-2">
                {group.docs.map((doc) => (
                  <a
                    key={doc.title}
                    href={doc.href}
                    target="_blank"
                    rel="noopener"
                    className="block bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-4 gw-card-hover no-underline group"
                  >
                    <div className="flex items-start justify-between gap-3">
                      <div className="flex-1">
                        <div className="font-display text-[15px] font-semibold text-[var(--gw-white)] group-hover:text-[var(--gw-green)] transition-colors mb-1">
                          {doc.title}
                        </div>
                        <p className="text-[12px] text-[var(--gw-white-dim)] leading-relaxed">
                          {doc.desc}
                        </p>
                      </div>
                      <svg
                        width="16"
                        height="16"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        strokeWidth="2"
                        className="text-[var(--gw-white-faint)] group-hover:text-[var(--gw-green)] transition-colors shrink-0 mt-0.5"
                      >
                        <path d="M7 17L17 7M17 7H7M17 7v10" />
                      </svg>
                    </div>
                  </a>
                ))}
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}
