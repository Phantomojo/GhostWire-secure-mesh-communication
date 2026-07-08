'use client'

import { FeatureModal } from "@/components/site/feature-modal";
import { useState } from "react";

interface Persona {
  id: string;
  title: string;
  blurb: string;
  icon: React.ReactNode;
  problem: string;
  helps: { label: string; text: string }[];
  scenario: string;
}

const PERSONAS: Persona[] = [
  {
    id: "disaster",
    title: "Disaster Responders",
    blurb: "Coordinate when cell towers fail. Works on battery and solar.",
    icon: (
      <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
        <path d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z" />
      </svg>
    ),
    problem:
      "When hurricanes, earthquakes, or floods strike, cell towers go down. First responders lose coordination. Communities are cut off from emergency services.",
    helps: [
      {
        label: "Works without internet",
        text: "TCP/IP mesh, Reticulum, and Bluetooth LE create local mesh networks between devices.",
      },
      {
        label: "Long-range with LoRa",
        text: "LoRa radio extends range to kilometers, connecting isolated communities.",
      },
      {
        label: "Battery and solar powered",
        text: "Runs on Raspberry Pi, laptops, or phones. No grid power needed.",
      },
      {
        label: "Delay-tolerant messaging",
        text: "Messages queue and forward when connectivity resumes. No message is lost.",
      },
      {
        label: "Encrypted coordination",
        text: "XChaCha20-Poly1305 E2EE ensures sensitive operational data stays private.",
      },
    ],
    scenario:
      "A flood cuts power and internet to a coastal town. Response teams deploy GhostWire nodes on battery-powered Raspberry Pis. Medics coordinate triage, logistics teams route supplies, and search teams report findings — all without any cellular or internet infrastructure.",
  },
  {
    id: "privacy",
    title: "Privacy Advocates",
    blurb: "Communicate without corporate surveillance. No logs. No traces.",
    icon: (
      <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
        <path d="M16.5 10.5V6.75a4.5 4.5 0 10-9 0v3.75m-.75 11.25h10.5a2.25 2.25 0 002.25-2.25v-6.75a2.25 2.25 0 00-2.25-2.25H6.75a2.25 2.25 0 00-2.25 2.25v6.75a2.25 2.25 0 002.25 2.25z" />
      </svg>
    ),
    problem:
      "Mainstream messaging platforms log metadata, scan content, and monetize attention. Even \"private\" apps depend on central servers that can be subpoenaed, breached, or compromised.",
    helps: [
      {
        label: "No central server",
        text: "Pure P2P architecture. No server logs, no metadata honeypot, no corporate middleman.",
      },
      {
        label: "XChaCha20-Poly1305 encryption",
        text: "Authenticated encryption for every message, file chunk, and session state.",
      },
      {
        label: "Keys never leave your device",
        text: "Ed25519/X25519 keypairs generated and stored locally. No key escrow.",
      },
      {
        label: "Post-quantum ready",
        text: "ML-KEM-768 (NIST FIPS 203) hybrid key exchange protects against future quantum adversaries.",
      },
      {
        label: "Sphinx onion routing",
        text: "Sender-hidden packets with SURBs make traffic analysis dramatically harder.",
      },
    ],
    scenario:
      "A journalist coordinates with sources without exposing them. GhostWire generates ephemeral identity profiles, encrypts every message end-to-end with XChaCha20-Poly1305, and routes through Sphinx onion paths — leaving no central metadata trail.",
  },
  {
    id: "rural",
    title: "Rural Communities",
    blurb: "Mesh nodes for entire villages. AI routing learns the best paths.",
    icon: (
      <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
        <path d="M2.25 21h19.5m-18-18v18m10.5-18v18m6-13.5V21M6.75 6.75h.75m-.75 3h.75m-.75 3h.75m3-6h.75m-.75 3h.75m-.75 3h.75M6.75 21v-3.375c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21M3 3h12m-.75 4.5H21m-3.75 3H21m-3.75 3H21" />
      </svg>
    ),
    problem:
      "Rural and remote communities are last in line for telecom investment. Spotty coverage, no redundancy, and no path to infrastructure ownership keep billions disconnected.",
    helps: [
      {
        label: "Village-scale mesh",
        text: "Each device becomes a relay. Coverage scales with the community.",
      },
      {
        label: "AI-powered routing",
        text: "4-layer ML stack (LightGBM → Gemma 4 → GNN → PRoPHET) optimizes paths continuously.",
      },
      {
        label: "Delay-tolerant delivery",
        text: "Messages queue when peers sleep or move. They deliver when the mesh reconnects.",
      },
      {
        label: "Low-cost hardware",
        text: "Runs on Raspberry Pi, old laptops, Android phones. No specialized gear required.",
      },
      {
        label: "Community-owned",
        text: "No ISP bills, no carrier lock-in. The mesh belongs to the people who run it.",
      },
    ],
    scenario:
      "A farming cooperative across 12 villages sets up GhostWire nodes on solar-powered Raspberry Pis. Market prices, weather alerts, and coordination messages hop village-to-village — even when the nearest cell tower is 40km away.",
  },
  {
    id: "activist",
    title: "Activists & Journalists",
    blurb:
      "Coordinate without infiltration. Disposable identities. Mesh during shutdowns.",
    icon: (
      <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
        <path d="M18 18.72a9.094 9.094 0 003.741-.479 3 3 0 00-4.682-2.72m.94 3.198l.001.031c0 .225-.012.447-.037.666A11.944 11.944 0 0112 21c-2.17 0-4.207-.576-5.963-1.584A6.062 6.062 0 016 18.719m12 0a5.971 5.971 0 00-.941-3.197m0 0A5.995 5.995 0 0012 12.75a5.995 5.995 0 00-5.058 2.772m0 0a3 3 0 00-4.681 2.72 8.986 8.986 0 003.74.477m.94-3.197a5.971 5.971 0 00-.94 3.197M15 6.75a3 3 0 11-6 0 3 3 0 016 0zm6 3a2.25 2.25 0 11-4.5 0 2.25 2.25 0 014.5 0zm-13.5 0a2.25 2.25 0 11-4.5 0 2.25 2.25 0 014.5 0z" />
      </svg>
    ),
    problem:
      "Activists and journalists are targeted by surveillance, infiltration, and network shutdowns. Mainstream apps leak metadata and central servers are complicit.",
    helps: [
      {
        label: "Disposable identities",
        text: "Multi-profile support. Spin up a fresh identity for each action, burn it after.",
      },
      {
        label: "Mesh during internet shutdowns",
        text: "When the internet goes dark, GhostWire keeps working via TCP/IP mesh, Bluetooth LE, and LoRa.",
      },
      {
        label: "Coercion resistance",
        text: "Duress PIN, panic wipe, and dead man's switch protect against physical coercion.",
      },
      {
        label: "Sphinx onion routing",
        text: "Packets hide source/destination from intermediate hops.",
      },
      {
        label: "No phone number required",
        text: "Identity is a keypair, not a SIM. No carrier, no KYC, no phone number leak.",
      },
    ],
    scenario:
      "During a protest, the government cuts cellular and internet. Organizers switch to GhostWire mesh over Bluetooth LE and device-to-device WiFi. Messages hop phone-to-phone across the crowd, encrypted and outside any monitored infrastructure.",
  },
  {
    id: "whistleblower",
    title: "Whistleblowers",
    blurb:
      "Send documents without revealing identity. Sphinx routing hides the source.",
    icon: (
      <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
        <path d="M8.288 15.038a5.25 5.25 0 017.424 0M5.106 11.856c3.807-3.808 9.98-3.808 13.788 0M1.924 8.674c5.565-5.565 14.587-5.565 20.152 0M12.53 18.22l-.53.53-.53-.53a.75.75 0 011.06 0z" />
      </svg>
    ),
    problem:
      "Whistleblowers face retaliation, prosecution, and physical danger. SecureDrop helps but depends on Tor + a hosted server. Document leaks need an offline-capable, server-free path.",
    helps: [
      {
        label: "Server-free document transfer",
        text: "Files chunk, encrypt, and hop peer-to-peer across the mesh. No central drop server to seize.",
      },
      {
        label: "Sphinx onion routing",
        text: "Sender-hidden packets with SURBs make traffic analysis dramatically harder.",
      },
      {
        label: "Disposable recipient identity",
        text: "Journalists publish a QR-code identity; sources message it without revealing their own.",
      },
      {
        label: "Post-quantum key exchange",
        text: "ML-KEM-768 protects leaked documents against future quantum decryption.",
      },
      {
        label: "Censorship-resistant transports",
        text: "obfs4 + Stealth TCP + Shadowsocks pluggable transports defeat DPI.",
      },
    ],
    scenario:
      "A whistleblower sends internal documents to a reporter. They scan the reporter's published GhostWire QR identity, compose an encrypted message, and the document chunks hop across the mesh — never touching a server the employer can monitor.",
  },
  {
    id: "developer",
    title: "Developers & Researchers",
    blurb: "AGPL-licensed Rust protocol. Build communication tools on top.",
    icon: (
      <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
        <path d="M17.25 6.75L22.5 12l-5.25 5.25m-10.5 0L1.5 12l5.25-5.25m7.5-3l-4.5 16.5" />
      </svg>
    ),
    problem:
      "Most mesh networking research is locked in papers or unmaintained prototypes. Researchers need a real, hackable codebase to push the field forward.",
    helps: [
      {
        label: "It's a protocol, not an app",
        text: "GhostWire provides cryptographic, routing, and transport layers. Build any communication tool on top.",
      },
      {
        label: "AGPL-3.0 source",
        text: "Full codebase on GitHub. Fork it, study it, ship it.",
      },
      {
        label: "Memory-safe Rust core",
        text: "Cargo workspace with published library crates. sphinx-rs is already on crates.io.",
      },
      {
        label: "Modular transport layer",
        text: "Implement the Transport trait to plug in new L2/L3 — LoRa, BLE, satellite, acoustic.",
      },
      {
        label: "Three ways to build",
        text: "Use the Rust library, use the REST/WebSocket API, or implement the protocol spec yourself.",
      },
    ],
    scenario:
      "A PhD student researching delay-tolerant routing clones the repo, implements a new routing strategy against the Transport trait, and benchmarks against the bundled PRoPHET fallback — without rewriting the network stack.",
  },
];

export function ForWhoSection() {
  const [active, setActive] = useState<Persona | null>(null);

  return (
    <section id="for-who" className="py-24 border-t border-[var(--gw-border)]">
      <div className="max-w-[1160px] mx-auto px-6 sm:px-8">
        <div className="gw-section-label mb-3">{"// who this is for"}</div>
        <h2 className="font-display text-3xl sm:text-5xl font-bold mb-3 text-[var(--gw-white)]">
          Built for those the internet forgot
        </h2>
        <p className="text-[var(--gw-white-dim)] mb-10 max-w-2xl">
          Click any card to learn how GhostWire helps your specific situation.
        </p>
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
          {PERSONAS.map((p) => (
            <button
              key={p.id}
              onClick={() => setActive(p)}
              className="text-left bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-6 gw-card-hover focus:outline-none focus:border-[var(--gw-green)]"
              aria-label={`Learn how GhostWire helps ${p.title}`}
            >
              <div className="text-[var(--gw-green)] mb-4">{p.icon}</div>
              <h3 className="font-display text-xl font-semibold mb-2 text-[var(--gw-white)]">
                {p.title}
              </h3>
              <p className="text-[14px] text-[var(--gw-white-dim)] leading-relaxed mb-4">
                {p.blurb}
              </p>
              <span className="font-mono-gw text-[12px] text-[var(--gw-green)]">
                Click to learn more →
              </span>
            </button>
          ))}
        </div>
      </div>

      <FeatureModal
        open={!!active}
        onClose={() => setActive(null)}
        tag="persona"
        title={active?.title ?? ""}
        ctaPrimary={{ href: "#quickstart", label: "Get GhostWire" }}
        ctaSecondary={{
          href: "https://github.com/Phantomojo/GhostWire-secure-mesh-communication",
          label: "View Source",
        }}
      >
        {active && (
          <>
            <div>
              <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
                The Problem
              </h3>
              <p>{active.problem}</p>
            </div>
            <div>
              <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-3">
                How GhostWire Helps
              </h3>
              <ul className="space-y-2.5 list-none p-0">
                {active.helps.map((h) => (
                  <li key={h.label} className="pl-5 relative">
                    <span className="absolute left-0 top-2 w-1.5 h-1.5 rounded-full bg-[var(--gw-green)]" />
                    <strong className="text-[var(--gw-white)]">{h.label}:</strong>{" "}
                    {h.text}
                  </li>
                ))}
              </ul>
            </div>
            <div>
              <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
                Real-World Scenario
              </h3>
              <p className="italic">{active.scenario}</p>
            </div>
          </>
        )}
      </FeatureModal>
    </section>
  );
}
