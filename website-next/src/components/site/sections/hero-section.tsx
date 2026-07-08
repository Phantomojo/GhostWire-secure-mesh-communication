'use client'

import { MeshCanvas } from "@/components/site/mesh-canvas";

const STATS = [
  { num: "P2P", label: "No Central Server" },
  { num: "E2EE", label: "Multi-Cipher Encrypted" },
  { num: "Rust", label: "Memory-Safe Core" },
  { num: "AI", label: "Learned Routing" },
  { num: "DTN", label: "Delay-Tolerant" },
];

export function HeroSection() {
  return (
    <section
      id="top"
      className="relative min-h-screen flex items-center pt-16 overflow-hidden"
    >
      <div
        className="absolute inset-0 pointer-events-none"
        style={{
          background:
            "radial-gradient(ellipse at 30% 40%, rgba(0,220,154,0.08) 0%, var(--gw-bg) 60%)",
        }}
      />
      <MeshCanvas />
      <div className="relative z-10 max-w-[1160px] mx-auto px-6 sm:px-8 pt-20 pb-24 w-full">
        <div className="flex items-center gap-2.5 mb-8">
          <span className="gw-pulse-dot" />
          <span className="font-mono-gw text-[12px] text-[var(--gw-green)] tracking-[0.06em]">
            v0.2.0 — Active Development
          </span>
        </div>
        <h1 className="font-display font-extrabold text-[clamp(2.5rem,8vw,5.5rem)] leading-[1.02] tracking-tight mb-7">
          Infrastructure
          <br />
          <span className="text-[var(--gw-green)] gw-glow">for Human Dignity</span>
        </h1>
        <p className="text-[17px] sm:text-[19px] text-[var(--gw-white-dim)] max-w-2xl leading-relaxed mb-9">
          A decentralized, censorship-resistant communication mesh.
          <br />
          No servers. No owners. Built for when the internet goes dark.
        </p>
        <div className="flex flex-wrap gap-3 mb-14">
          <a href="#quickstart" className="gw-btn-primary">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" aria-hidden="true">
              <path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4M7 10l5 5 5-5M12 15V3" />
            </svg>
            Get GhostWire
          </a>
          <a href="#architecture" className="gw-btn-ghost">
            Read the Docs
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" aria-hidden="true">
              <path d="M7 17L17 7M17 7H7M17 7v10" />
            </svg>
          </a>
        </div>

        <div className="flex flex-wrap items-center gap-x-6 gap-y-4">
          {STATS.map((s, i) => (
            <div key={s.num} className="flex items-center gap-6">
              {i > 0 && <div className="h-9 w-px bg-[var(--gw-border-2)] hidden sm:block" />}
              <div>
                <div className="font-mono-gw text-xl sm:text-2xl font-semibold text-[var(--gw-white)]">
                  {s.num}
                </div>
                <div className="text-[11px] sm:text-[12px] text-[var(--gw-white-faint)] tracking-wide">
                  {s.label}
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>

      <div className="absolute bottom-8 left-1/2 -translate-x-1/2 flex flex-col items-center gap-2 z-10" aria-hidden="true">
        <span className="font-mono-gw text-[10px] tracking-[0.2em] text-[var(--gw-white-faint)]">SCROLL</span>
        <div className="w-px h-10 bg-gradient-to-b from-[var(--gw-green)] to-transparent" />
      </div>
    </section>
  );
}
