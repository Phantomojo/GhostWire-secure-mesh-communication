"use client";

import { useState } from "react";

const CITATION = `GhostWire Team. "GhostWire: An Open Protocol
for Resilient Peer-to-Peer Mesh
Communication." 2026.
https://www.ghostwire.cc/`;

export function PressSection() {
  const [copied, setCopied] = useState(false);

  const copyCitation = async () => {
    try {
      await navigator.clipboard.writeText(CITATION);
      setCopied(true);
      window.setTimeout(() => setCopied(false), 2000);
    } catch {
      // Clipboard may be unavailable (e.g. insecure context); no-op.
    }
  };

  return (
    <section id="press" className="py-24 border-t border-[var(--gw-border)]">
      <div className="max-w-7xl mx-auto px-6">
        <div className="gw-section-label mb-3">{"// press & cite"}</div>
        <h2 className="font-display text-3xl md:text-4xl font-bold text-[var(--gw-white)] mb-3">
          Press, cite, and share
        </h2>
        <p className="text-[15px] text-[var(--gw-white-dim)] max-w-2xl mb-12 leading-relaxed">
          Cite GhostWire in academic work, embed a badge, or share the project.
        </p>

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          {/* Part 1 — Citation block */}
          <div className="bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-6">
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-3">
              Cite GhostWire
            </h3>
            <p className="text-[13px] text-[var(--gw-white-dim)] mb-4">
              If referencing GhostWire in academic or technical work:
            </p>
            <div className="relative">
              <pre className="gw-code text-[12px]">
                <code>{CITATION}</code>
              </pre>
              <button
                onClick={copyCitation}
                className="absolute top-2 right-2 gw-btn-outline text-[11px] py-1 px-2"
              >
                {copied ? "✓ Copied" : "Copy"}
              </button>
            </div>
            <p className="text-[12px] text-[var(--gw-white-faint)] mt-3">
              BibTeX available in the repo llms.txt.
            </p>
          </div>

          {/* Part 2 — Embed badges */}
          <div className="bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-6">
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-3">
              Embed badges
            </h3>
            <p className="text-[13px] text-[var(--gw-white-dim)] mb-4">
              Show your support. Copy the Markdown:
            </p>
            <div className="space-y-3">
              {/* Badge 1: AGPL License */}
              <div>
                <img
                  src="https://img.shields.io/badge/License-AGPL--3.0-blue.svg"
                  alt="License: AGPL-3.0"
                  className="h-5"
                />
                <pre className="gw-code text-[11px] mt-1">
                  <code>{`[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL--3.0-blue.svg)](https://www.ghostwire.cc/)`}</code>
                </pre>
              </div>
              {/* Badge 2: Version */}
              <div>
                <img
                  src="https://img.shields.io/badge/version-0.2.0-blue.svg"
                  alt="Version 0.2.0"
                  className="h-5"
                />
                <pre className="gw-code text-[11px] mt-1">
                  <code>{`[![Version](https://img.shields.io/badge/version-0.2.0-blue.svg)](https://www.ghostwire.cc/)`}</code>
                </pre>
              </div>
              {/* Badge 3: Built with Rust */}
              <div>
                <img
                  src="https://img.shields.io/badge/Built%20with-Rust-orange.svg"
                  alt="Built with Rust"
                  className="h-5"
                />
                <pre className="gw-code text-[11px] mt-1">
                  <code>{`[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)](https://www.ghostwire.cc/)`}</code>
                </pre>
              </div>
            </div>
          </div>

          {/* Part 3 — Share + contact */}
          <div className="bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-6">
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-3">
              Share &amp; contact
            </h3>
            <p className="text-[13px] text-[var(--gw-white-dim)] mb-4">
              Share GhostWire or get in touch.
            </p>
            <div className="space-y-2 mb-5">
              <a
                href={`https://twitter.com/intent/tweet?text=${encodeURIComponent("GhostWire — an open protocol for resilient P2P mesh communication. Encrypted, delay-tolerant, post-quantum ready. No servers. No owners.")}&url=https://www.ghostwire.cc/`}
                target="_blank"
                rel="noopener"
                className="flex items-center gap-2 text-[13px] text-[var(--gw-white-dim)] hover:text-[var(--gw-green)] no-underline"
              >
                <svg
                  width="16"
                  height="16"
                  viewBox="0 0 24 24"
                  fill="currentColor"
                  aria-hidden="true"
                >
                  <path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z" />
                </svg>
                Share on X / Twitter
              </a>
              <a
                href={`https://news.ycombinator.com/submitlink?u=https://www.ghostwire.cc/&t=GhostWire — Resilient P2P Mesh Communication`}
                target="_blank"
                rel="noopener"
                className="flex items-center gap-2 text-[13px] text-[var(--gw-white-dim)] hover:text-[var(--gw-green)] no-underline"
              >
                <svg
                  width="16"
                  height="16"
                  viewBox="0 0 24 24"
                  fill="currentColor"
                  aria-hidden="true"
                >
                  <path d="M3 12l9-9 9 9-9 9z" />
                </svg>
                Submit to Hacker News
              </a>
              <a
                href={`https://www.reddit.com/submit?url=https://www.ghostwire.cc/&title=GhostWire — Resilient P2P Mesh Communication`}
                target="_blank"
                rel="noopener"
                className="flex items-center gap-2 text-[13px] text-[var(--gw-white-dim)] hover:text-[var(--gw-green)] no-underline"
              >
                <svg
                  width="16"
                  height="16"
                  viewBox="0 0 24 24"
                  fill="currentColor"
                  aria-hidden="true"
                >
                  <circle cx="12" cy="12" r="10" />
                </svg>
                Submit to Reddit
              </a>
              <button
                onClick={() => navigator.clipboard.writeText("https://www.ghostwire.cc/")}
                className="flex items-center gap-2 text-[13px] text-[var(--gw-white-dim)] hover:text-[var(--gw-green)] w-full text-left"
              >
                <svg
                  width="16"
                  height="16"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  strokeWidth="2"
                  aria-hidden="true"
                >
                  <path d="M10 13a5 5 0 007.54.54l3-3a5 5 0 00-7.07-7.07l-1.72 1.71M14 11a5 5 0 00-7.54-.54l-3 3a5 5 0 007.07 7.07l1.71-1.71" />
                </svg>
                Copy link
              </button>
            </div>
            <div className="pt-5 border-t border-[var(--gw-border)]">
              <div className="font-mono-gw text-[10px] tracking-[0.1em] uppercase text-[var(--gw-white-faint)] mb-2">
                Contact
              </div>
              <div className="space-y-1 text-[13px]">
                <a
                  href="mailto:admin@ghostwire.cc"
                  className="block text-[var(--gw-white-dim)] hover:text-[var(--gw-green)] no-underline"
                >
                  admin@ghostwire.cc (general)
                </a>
                <a
                  href="mailto:security@ghostwire.cc"
                  className="block text-[var(--gw-white-dim)] hover:text-[var(--gw-green)] no-underline"
                >
                  security@ghostwire.cc (disclosure)
                </a>
                <a
                  href="mailto:mirungu015@proton.me"
                  className="block text-[var(--gw-white-dim)] hover:text-[var(--gw-green)] no-underline"
                >
                  mirungu015@proton.me (PGP)
                </a>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
