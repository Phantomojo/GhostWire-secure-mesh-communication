'use client'

import { useEffect, useState } from "react";

const NAV_LINKS = [
  { href: "#problem", label: "Problem" },
  { href: "#how-it-works", label: "How It Works" },
  { href: "#features", label: "Features" },
  { href: "#architecture", label: "Architecture" },
  { href: "#security", label: "Security" },
  { href: "#docs", label: "Docs" },
  { href: "#glossary", label: "Glossary" },
  { href: "#roadmap", label: "Roadmap" },
  { href: "#faq", label: "FAQ" },
] as const;

export function SiteNav() {
  const [scrolled, setScrolled] = useState(false);
  const [mobileOpen, setMobileOpen] = useState(false);
  const [activeSection, setActiveSection] = useState<string>("");

  useEffect(() => {
    const onScroll = () => setScrolled(window.scrollY > 24);
    onScroll();
    window.addEventListener("scroll", onScroll, { passive: true });
    return () => window.removeEventListener("scroll", onScroll);
  }, []);

  useEffect(() => {
    const observer = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          if (entry.isIntersecting) {
            setActiveSection(`#${entry.target.id}`);
          }
        }
      },
      { rootMargin: "-40% 0px -55% 0px", threshold: 0 }
    );
    NAV_LINKS.forEach((l) => {
      const el = document.querySelector(l.href);
      if (el) observer.observe(el);
    });
    return () => observer.disconnect();
  }, []);

  return (
    <nav
      className={`fixed top-0 left-0 right-0 z-50 h-16 border-b backdrop-blur-xl transition-colors ${
        scrolled
          ? "bg-[rgba(7,10,14,0.97)] border-[var(--gw-border-2)]"
          : "bg-[rgba(7,10,14,0.85)] border-[var(--gw-border)]"
      }`}
    >
      <div className="max-w-[1160px] mx-auto px-6 sm:px-8 h-full flex items-center justify-between">
        <a href="#top" className="flex items-center gap-2.5 no-underline text-[var(--gw-white)]">
          <img
            src="/ghostwire-logo.png"
            alt="GhostWire logo"
            width={28}
            height={28}
            className="rounded-sm"
          />
          <span className="font-mono-gw text-[13px] font-semibold tracking-[0.12em]">
            GHOSTWIRE
          </span>
        </a>

        <div className="hidden lg:flex items-center gap-7">
          {NAV_LINKS.map((link) => (
            <a
              key={link.href}
              href={link.href}
              className={`text-[13px] tracking-[0.02em] no-underline transition-colors ${
                activeSection === link.href
                  ? "text-[var(--gw-green)]"
                  : "text-[var(--gw-white-dim)] hover:text-[var(--gw-white)]"
              }`}
            >
              {link.label}
            </a>
          ))}
          <a
            href="https://github.com/Phantomojo/GhostWire-secure-mesh-communication"
            target="_blank"
            rel="noopener"
            className="flex items-center gap-1.5 text-[13px] text-[var(--gw-green)] no-underline px-3.5 py-1.5 border border-[var(--gw-border-2)] rounded hover:bg-[var(--gw-green-dim)] hover:border-[var(--gw-green)] transition-colors"
          >
            <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
              <path d="M12 0C5.374 0 0 5.373 0 12c0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23A11.509 11.509 0 0112 5.803c1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576C20.566 21.797 24 17.3 24 12c0-6.627-5.373-12-12-12z" />
            </svg>
            GitHub
          </a>
        </div>

        <button
          className="lg:hidden flex flex-col gap-1.5 bg-transparent border-none cursor-pointer p-1"
          aria-label="Toggle navigation menu"
          aria-expanded={mobileOpen}
          onClick={() => setMobileOpen((v) => !v)}
        >
          <span
            className={`block w-[22px] h-[1.5px] bg-[var(--gw-white-dim)] transition-transform ${
              mobileOpen ? "translate-y-[7px] rotate-45" : ""
            }`}
          />
          <span
            className={`block w-[22px] h-[1.5px] bg-[var(--gw-white-dim)] transition-opacity ${
              mobileOpen ? "opacity-0" : ""
            }`}
          />
          <span
            className={`block w-[22px] h-[1.5px] bg-[var(--gw-white-dim)] transition-transform ${
              mobileOpen ? "-translate-y-[7px] -rotate-45" : ""
            }`}
          />
        </button>
      </div>

      {mobileOpen && (
        <div className="lg:hidden flex flex-col bg-[var(--gw-bg-2)] border-t border-[var(--gw-border)] px-6 py-4 max-h-[calc(100vh-4rem)] overflow-y-auto">
          {NAV_LINKS.map((link) => (
            <a
              key={link.href}
              href={link.href}
              onClick={() => setMobileOpen(false)}
              className="py-3 text-[15px] text-[var(--gw-white-dim)] no-underline border-b border-[var(--gw-border)] last:border-none"
            >
              {link.label}
            </a>
          ))}
          <a
            href="https://github.com/Phantomojo/GhostWire-secure-mesh-communication"
            target="_blank"
            rel="noopener"
            onClick={() => setMobileOpen(false)}
            className="py-3 text-[15px] text-[var(--gw-green)] no-underline"
          >
            GitHub →
          </a>
        </div>
      )}
    </nav>
  );
}
