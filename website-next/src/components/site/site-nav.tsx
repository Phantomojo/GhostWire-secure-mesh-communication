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
  { href: "#blog", label: "Blog" },
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
            href="https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication"
            target="_blank"
            rel="noopener"
            className="flex items-center gap-1.5 text-[13px] text-[var(--gw-green)] no-underline px-3.5 py-1.5 border border-[var(--gw-border-2)] rounded hover:bg-[var(--gw-green-dim)] hover:border-[var(--gw-green)] transition-colors"
          >
            <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
              <path d="M11.999.747A11.974 11.974 0 0 0 0 12.75c0 2.254.635 4.465 1.833 6.376L11.837 6.19c.072-.092.251-.092.323 0l4.178 5.402h-2.992l.065.239h3.113l.882 1.138h-3.674l.103.374h3.86l.777 1.003h-4.358l.135.483h4.593l.695.894h-5.038l.165.589h5.326l.609.785h-5.717l.182.65h6.038l.562.727h-6.397l.183.65h6.717A12.003 12.003 0 0 0 24 12.75 11.977 11.977 0 0 0 11.999.747zm3.654 19.104.182.65h5.326c.173-.204.353-.433.513-.65zm.385 1.377.18.65h3.563c.233-.198.485-.428.712-.65zm.383 1.377.182.648h1.203c.356-.204.685-.412 1.042-.648zz" />
            </svg>
            Codeberg
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
            href="https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication"
            target="_blank"
            rel="noopener"
            onClick={() => setMobileOpen(false)}
            className="py-3 text-[15px] text-[var(--gw-green)] no-underline"
          >
            Codeberg →
          </a>
        </div>
      )}
    </nav>
  );
}
