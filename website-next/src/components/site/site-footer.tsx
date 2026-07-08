'use client'

import { useState } from "react";

export function SiteFooter() {
  const [email, setEmail] = useState("");
  const [status, setStatus] = useState<"idle" | "loading" | "success" | "error">("idle");
  const [message, setMessage] = useState("");

  async function handleSubscribe(e: React.FormEvent) {
    e.preventDefault();
    if (!email) return;
    setStatus("loading");
    setMessage("");
    try {
      const res = await fetch("/api/newsletter", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ email }),
      });
      const data = await res.json();
      if (!res.ok) throw new Error(data.error || "Subscription failed");
      setStatus("success");
      setMessage(data.message || "Subscribed. Welcome to the mesh.");
      setEmail("");
    } catch (err) {
      setStatus("error");
      setMessage(err instanceof Error ? err.message : "Subscription failed");
    }
  }

  return (
    <footer className="border-t border-[var(--gw-border)] bg-[var(--gw-bg-2)] mt-auto">
      <div className="max-w-[1160px] mx-auto px-6 sm:px-8 py-14">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-10">
          {/* Brand + Newsletter */}
          <div className="lg:col-span-2">
            <div className="flex items-center gap-2.5 mb-3">
              <img
                src="/ghostwire-logo.png"
                alt="GhostWire"
                width={22}
                height={22}
                className="rounded-sm"
              />
              <span className="font-mono-gw text-[13px] font-semibold tracking-[0.12em]">
                GHOSTWIRE
              </span>
            </div>
            <p className="text-sm text-[var(--gw-white-dim)] leading-relaxed mb-2">
              An open protocol for resilient peer-to-peer communication.
              <br />
              Build communication tools that work regardless of the odds.
            </p>
            <p className="font-mono-gw text-[11px] text-[var(--gw-white-faint)] mb-1">
              AGPL-3.0 / Commercial License available
            </p>

            <div className="mt-7">
              <h4 className="font-mono-gw text-[12px] tracking-[0.08em] text-[var(--gw-green)] mb-2">
                {"// newsletter"}
              </h4>
              <p className="text-[13px] text-[var(--gw-white-dim)] mb-3">
                Security advisories, release announcements, and protocol updates.
              </p>
              <form onSubmit={handleSubscribe} className="flex flex-col sm:flex-row gap-2">
                <input
                  type="email"
                  required
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                  placeholder="email@address.com"
                  aria-label="Email address"
                  className="flex-1 bg-[var(--gw-bg)] border border-[var(--gw-border-2)] rounded px-3 py-2.5 text-[13px] font-mono-gw text-[var(--gw-white)] placeholder:text-[var(--gw-white-faint)] focus:outline-none focus:border-[var(--gw-green)]"
                />
                <button
                  type="submit"
                  disabled={status === "loading"}
                  className="gw-btn-primary justify-center disabled:opacity-60"
                >
                  {status === "loading" ? "Subscribing..." : "Subscribe"}
                </button>
              </form>
              {message && (
                <p
                  className={`mt-2 text-[12px] font-mono-gw ${
                    status === "success" ? "text-[var(--gw-green)]" : "text-[var(--gw-orange)]"
                  }`}
                  role="status"
                  aria-live="polite"
                >
                  {status === "success" ? "✓ " : "⚠ "}
                  {message}
                </p>
              )}
            </div>
          </div>

          {/* Project column */}
          <div>
            <h4 className="font-mono-gw text-[11px] tracking-[0.12em] text-[var(--gw-white-faint)] uppercase mb-4">
              Project
            </h4>
            <ul className="space-y-2.5 list-none p-0">
              <FooterLink href="#about" label="About" />
              <FooterLink href="#manifesto" label="Manifesto" />
              <FooterLink href="#roadmap" label="Roadmap" />
              <FooterLink href="#contributors" label="Contributors" />
              <FooterLink
                href="https://github.com/Phantomojo/GhostWire-secure-mesh-communication/blob/main/ghostwire/CHANGELOG.md"
                label="Changelog"
                external
              />
            </ul>
          </div>

          {/* Get Started column */}
          <div>
            <h4 className="font-mono-gw text-[11px] tracking-[0.12em] text-[var(--gw-white-faint)] uppercase mb-4">
              Get Started
            </h4>
            <ul className="space-y-2.5 list-none p-0">
              <FooterLink href="#quickstart" label="Quick Start" />
              <FooterLink href="#architecture" label="Architecture" />
              <FooterLink href="#security" label="Security" />
              <FooterLink href="#faq" label="FAQ" />
              <FooterLink
                href="https://github.com/Phantomojo/GhostWire-secure-mesh-communication/blob/main/docs/API_QUICK_REFERENCE.md"
                label="API Reference"
                external
              />
            </ul>
          </div>

          {/* Community column */}
          <div>
            <h4 className="font-mono-gw text-[11px] tracking-[0.12em] text-[var(--gw-white-faint)] uppercase mb-4">
              Community
            </h4>
            <ul className="space-y-2.5 list-none p-0">
              <FooterLink
                href="https://github.com/Phantomojo/GhostWire-secure-mesh-communication"
                label="GitHub"
                external
              />
              <FooterLink
                href="https://github.com/Phantomojo/GhostWire-secure-mesh-communication/blob/main/ghostwire/CONTRIBUTING.md"
                label="Contribute"
                external
              />
              <FooterLink
                href="https://github.com/Phantomojo/GhostWire-secure-mesh-communication/blob/main/ghostwire/CODE_OF_CONDUCT.md"
                label="Code of Conduct"
                external
              />
              <FooterLink href="mailto:admin@ghostwire.cc" label="Support" />
              <FooterLink href="mailto:security@ghostwire.cc" label="Security" />
            </ul>
          </div>
        </div>

        <div className="mt-12 pt-6 border-t border-[var(--gw-border)] flex flex-col sm:flex-row justify-between items-start sm:items-center gap-3">
          <span className="font-mono-gw text-[11px] text-[var(--gw-white-faint)]">
            © 2026 GhostWire — open protocol, AGPL-3.0
          </span>
          <span className="font-mono-gw text-[11px] text-[var(--gw-white-faint)]">
            ghostwire.cc ·{" "}
            <a
              href="mailto:admin@ghostwire.cc"
              className="text-[var(--gw-green-dim)] hover:text-[var(--gw-green)] no-underline"
            >
              admin@ghostwire.cc
            </a>
          </span>
        </div>
      </div>
    </footer>
  );
}

function FooterLink({
  href,
  label,
  external,
}: {
  href: string;
  label: string;
  external?: boolean;
}) {
  return (
    <li>
      <a
        href={href}
        {...(external ? { target: "_blank", rel: "noopener" } : {})}
        className="text-[13px] text-[var(--gw-white-dim)] no-underline hover:text-[var(--gw-green)] transition-colors"
      >
        {label}
      </a>
    </li>
  );
}
