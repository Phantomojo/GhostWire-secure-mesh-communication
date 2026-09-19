'use client'

import { useState } from "react";
import { GithubDocModal } from "@/components/site/github-doc-modal";

interface CommunityCard {
  href?: string;
  external?: boolean;
  docPath?: string;
  docTitle?: string;
  icon: React.ReactNode;
  title: string;
  text: string;
  linkLabel: string;
}

const CARDS: CommunityCard[] = [
  {
    href: "https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication",
    external: true,
    icon: (
      <svg width={28} height={28} viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
        <path d="M12 0C5.374 0 0 5.373 0 12c0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23A11.509 11.509 0 0112 5.803c1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576C20.566 21.797 24 17.3 24 12c0-6.627-5.373-12-12-12z" />
      </svg>
    ),
    title: "Source Code",
    text: "Full source on Codeberg (primary) and GitHub (mirror). Report issues, submit PRs, review the code.",
    linkLabel: "codeberg.org/PHANTOMOJO →",
  },
  {
    href: "mailto:admin@ghostwire.cc",
    icon: (
      <svg width={28} height={28} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" aria-hidden="true">
        <path strokeLinecap="round" strokeLinejoin="round" d="M21.75 6.75v10.5a2.25 2.25 0 01-2.25 2.25h-15a2.25 2.25 0 01-2.25-2.25V6.75m19.5 0A2.25 2.25 0 0019.5 4.5h-15a2.25 2.25 0 00-2.25 2.25m19.5 0v.243a2.25 2.25 0 01-1.07 1.916l-7.5 4.615a2.25 2.25 0 01-2.36 0L3.32 8.91a2.25 2.25 0 01-1.07-1.916V6.75" />
      </svg>
    ),
    title: "Contact",
    text: "Reach the team directly. Support, security reports, partnerships.",
    linkLabel: "admin@ghostwire.cc →",
  },
  {
    href: "#contributors",
    icon: (
      <svg width={28} height={28} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" aria-hidden="true">
        <path strokeLinecap="round" strokeLinejoin="round" d="M17.25 6.75L22.5 12l-5.25 5.25m-10.5 0L1.5 12l5.25-5.25m7.5-3l-4.5 16.5" />
      </svg>
    ),
    title: "Contributor Portal",
    text: "Learn about roles, active issues, bounty opportunities, and meet the mesh builders.",
    linkLabel: "contributors →",
  },
  {
    href: "#security",
    icon: (
      <svg width={28} height={28} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" aria-hidden="true">
        <path strokeLinecap="round" strokeLinejoin="round" d="M9 12.75L11.25 15 15 9.75M21 12c0 5.25-4.5 9.75-9 9.75S3 17.25 3 12V5.25l9-3 9 3V12z" />
      </svg>
    ),
    title: "Security",
    text: "Report vulnerabilities responsibly. Read our security policy and threat model.",
    linkLabel: "Security policy →",
  },
  {
    docPath: "docs/spec/PROTOCOL.md",
    docTitle: "Protocol Specification",
    icon: (
      <svg width={28} height={28} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" aria-hidden="true">
        <path strokeLinecap="round" strokeLinejoin="round" d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
        <path strokeLinecap="round" strokeLinejoin="round" d="M14 2v6h6" />
        <path strokeLinecap="round" strokeLinejoin="round" d="M16 13H8 M16 17H8 M10 9H8" />
      </svg>
    ),
    title: "Protocol Spec",
    text: "Read the protocol specification. Implement GhostWire in any language.",
    linkLabel: "Read here →",
  },
  {
    href: "https://crates.io/crates/sphinx-rs",
    external: true,
    icon: (
      <svg width={28} height={28} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" aria-hidden="true">
        <path strokeLinecap="round" strokeLinejoin="round" d="M21 8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16Z" />
        <path strokeLinecap="round" strokeLinejoin="round" d="m3.3 7 8.7 5 8.7-5" />
        <path strokeLinecap="round" strokeLinejoin="round" d="M12 22V12" />
        <path strokeLinecap="round" strokeLinejoin="round" d="m7.5 4.27 9 5.15" />
      </svg>
    ),
    title: "Published Crates",
    text: "sphinx-rs is published on crates.io. ghostwire-dtn, hlc-rs, trust-store in progress.",
    linkLabel: "crates.io/sphinx-rs →",
  },
  {
    docPath: "docs/ARCHITECTURE.md",
    docTitle: "Architecture Guide",
    icon: (
      <svg width={28} height={28} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" aria-hidden="true">
        <path strokeLinecap="round" strokeLinejoin="round" d="M12 7v14" />
        <path strokeLinecap="round" strokeLinejoin="round" d="M3 18a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h5a4 4 0 0 1 4 4 4 4 0 0 1 4-4h5a1 1 0 0 1 1 1v13a1 1 0 0 1-1 1h-6a3 3 0 0 0-3 3 3 3 0 0 0-3-3z" />
      </svg>
    ),
    title: "Documentation",
    text: "Full docs: install guides, API reference, threat model, security audit, architecture.",
    linkLabel: "Read here →",
  },
  {
    docPath: "ghostwire/CHANGELOG.md",
    docTitle: "Changelog",
    icon: (
      <svg width={28} height={28} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" aria-hidden="true">
        <circle cx="12" cy="12" r="4" />
        <path strokeLinecap="round" strokeLinejoin="round" d="M1.05 12H7 M17.01 12H22.96" />
      </svg>
    ),
    title: "Changelog",
    text: "Track releases, version bumps, and breaking changes. Currently at v0.2.0.",
    linkLabel: "Read here →",
  },
];

export function CommunitySection() {
  const [activeDoc, setActiveDoc] = useState<{ path: string; title: string } | null>(null);

  return (
    <section id="community" className="py-24 border-t border-[var(--gw-border)]">
      <div className="max-w-[1160px] mx-auto px-6 sm:px-8">
        <div className="gw-section-label mb-3">{"// community"}</div>
        <h2 className="font-display text-3xl sm:text-5xl font-bold mb-10 text-[var(--gw-white)]">
          Open infrastructure, open community
        </h2>

        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
          {CARDS.map((card) => {
            if (card.docPath) {
              return (
                <button
                  key={card.title}
                  onClick={() => setActiveDoc({ path: card.docPath!, title: card.docTitle ?? card.title })}
                  className="block w-full text-left bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-6 gw-card-hover no-underline cursor-pointer"
                >
                  <div className="text-[var(--gw-green)] mb-4">{card.icon}</div>
                  <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">{card.title}</h3>
                  <p className="text-[13px] text-[var(--gw-white-dim)] leading-relaxed mb-3">{card.text}</p>
                  <span className="font-mono-gw text-[12px] text-[var(--gw-green)]">{card.linkLabel}</span>
                </button>
              );
            }
            return (
              <a
                key={card.title}
                href={card.href}
                {...(card.external ? { target: "_blank", rel: "noopener noreferrer" } : {})}
                className="block bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-6 gw-card-hover no-underline"
              >
                <div className="text-[var(--gw-green)] mb-4">{card.icon}</div>
                <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">{card.title}</h3>
                <p className="text-[13px] text-[var(--gw-white-dim)] leading-relaxed mb-3">{card.text}</p>
                <span className="font-mono-gw text-[12px] text-[var(--gw-green)]">{card.linkLabel}</span>
              </a>
            );
          })}
        </div>
      </div>

      {activeDoc && (
        <GithubDocModal open={!!activeDoc} onClose={() => setActiveDoc(null)} docPath={activeDoc.path} title={activeDoc.title} />
      )}
    </section>
  );
}
