'use client'

import { useEffect, useRef, useState } from "react";
import { FeatureModal } from "@/components/site/feature-modal";

interface Transport {
  name: string;
  desc: string;
  tag: "Core" | "Armor";
  modalBody: React.ReactNode;
  techNote: string;
}

function SpecTable({ rows }: { rows: Array<[string, string]> }) {
  return (
    <div className="overflow-x-auto my-3">
      <table className="w-full text-left border-collapse">
        <tbody>
          {rows.map(([k, v]) => (
            <tr key={k} className="border-b border-[var(--gw-border)]">
              <td className="py-2 px-3 font-mono-gw text-[12px] text-[var(--gw-white-faint)] uppercase tracking-wide w-1/3">{k}</td>
              <td className="py-2 px-3 text-[13px] text-[var(--gw-white)]">{v}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

function TransportCard({ t, onOpen }: { t: Transport; onOpen: () => void }) {
  return (
    <button
      onClick={onOpen}
      className="text-left bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-5 gw-card-hover focus:outline-none focus:border-[var(--gw-green)] w-full"
    >
      <div className="flex items-center justify-between mb-2">
        <h4 className="font-display text-base font-semibold text-[var(--gw-white)]">{t.name}</h4>
        <span className={`font-mono-gw text-[10px] tracking-[0.06em] uppercase px-2 py-0.5 rounded border ${t.tag === "Core" ? "text-[var(--gw-green)] border-[var(--gw-border-2)] bg-[var(--gw-green-dim)]" : "text-[var(--gw-orange)] border-[rgba(249,115,22,0.3)] bg-[rgba(249,115,22,0.12)]"}`}>{t.tag}</span>
      </div>
      <p className="text-[13px] text-[var(--gw-white-dim)] leading-relaxed mb-2">{t.desc}</p>
      <span className="font-mono-gw text-[12px] text-[var(--gw-green)]">Click for spec →</span>
    </button>
  );
}

const CORE_TRANSPORTS: Transport[] = [
  {
    name: "Stealth TCP",
    desc: "Fingerprint-resistant TCP. Reshapes handshake signatures to defeat DPI and traffic classification.",
    tag: "Core",
    techNote: "Protocol: TCP / Port: 9735 (configurable) / Stealth: random high port",
    modalBody: (
      <>
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">What Is It?</h3>
        <p>Standard TCP with obfuscation to avoid simple port-based blocking. Stealth mode listens on a random high port (10000-65535) advertised via mDNS/DHT.</p>
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">Spec</h3>
        <SpecTable rows={[
          ["Protocol", "TCP"],
          ["Default port", "9735 (configurable)"],
          ["Obfuscation", "Random port selection"],
          ["Use case", "General mesh, unrestricted networks"],
        ]} />
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">Stealth mode</h3>
        <p>Avoids port-based blocking by randomizing the listening port.</p>
        <div className="font-mono-gw text-[12px] text-[var(--gw-white-faint)] mt-5 pt-4 border-t border-[var(--gw-border)]">Protocol: TCP / Port: 9735 (configurable) / Stealth: random high port</div>
      </>
    ),
  },
  {
    name: "WebRTC",
    desc: "Browser-to-browser P2P. Enables direct connections from web clients without a native binary.",
    tag: "Core",
    techNote: "Protocol: WebRTC / NAT: STUN+TURN+ICE / Encryption: DTLS 1.2",
    modalBody: (
      <>
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">What Is It?</h3>
        <p>Peer-to-peer via WebRTC data channels, enabling NAT traversal and browser compatibility.</p>
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">Spec</h3>
        <SpecTable rows={[
          ["Protocol", "WebRTC (SCTP over DTLS)"],
          ["NAT traversal", "STUN/TURN + ICE"],
          ["Encryption", "DTLS 1.2 (built-in)"],
          ["Use case", "Cross-NAT, browser"],
        ]} />
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">Signaling</h3>
        <p>WebRTC signaling via GhostWire DHT (offer/answer exchange) or direct mDNS on LAN.</p>
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">TURN fallback</h3>
        <p>If direct P2P fails, a user-configured TURN relay is used. TURN traffic is DTLS-encrypted — the relay cannot read message contents.</p>
        <div className="font-mono-gw text-[12px] text-[var(--gw-white-faint)] mt-5 pt-4 border-t border-[var(--gw-border)]">Protocol: WebRTC / NAT: STUN+TURN+ICE / Encryption: DTLS 1.2</div>
      </>
    ),
  },
  {
    name: "Bluetooth LE",
    desc: "Device-to-device mesh on phones and laptops. Short range, low power, perfect for crowds.",
    tag: "Core",
    techNote: "Protocol: BLE 4.0+ / Range: 10-100m / MTU: 512B-4096B",
    modalBody: (
      <>
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">What Is It?</h3>
        <p>Short-range BLE for opportunistic mesh networking. Perfect for crowds, disaster zones, offline areas.</p>
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">Spec</h3>
        <SpecTable rows={[
          ["Protocol", "Bluetooth 4.0+ (BLE)"],
          ["Range", "10-100m"],
          ["MTU", "512 bytes (up to 4096)"],
          ["Power", "Low (1s advert)"],
          ["Use case", "Proximity, disaster, offline"],
        ]} />
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">DTN integration</h3>
        <p>BLE sync handshake details in the DTN spec — messages bundle and forward when devices come into range.</p>
        <div className="font-mono-gw text-[12px] text-[var(--gw-white-faint)] mt-5 pt-4 border-t border-[var(--gw-border)]">Protocol: BLE 4.0+ / Range: 10-100m / MTU: 512B-4096B</div>
      </>
    ),
  },
  {
    name: "LoRa",
    desc: "Long-range radio (km-scale). Connects isolated communities and off-grid nodes across terrain.",
    tag: "Core",
    techNote: "Protocol: LoRa / Range: 2-15km / Bandwidth: 0.3-50 kbps",
    modalBody: (
      <>
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">What Is It?</h3>
        <p>Long-range, low-bandwidth radio for rural and remote areas. 2-15 km range, sub-Kbps bandwidth.</p>
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">Spec</h3>
        <SpecTable rows={[
          ["Protocol", "LoRa (CSS)"],
          ["Frequency", "868 MHz EU / 915 MHz US / 433 MHz Asia"],
          ["Range", "2-15 km LOS"],
          ["Bandwidth", "0.3-50 kbps"],
          ["Max payload", "242 bytes"],
        ]} />
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">Fragmentation</h3>
        <p>Messages exceeding the 242-byte payload fragment at the DTN layer.</p>
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">Regulatory</h3>
        <p>Complies with regional duty cycle regulations (1% in EU 868 MHz band).</p>
        <div className="font-mono-gw text-[12px] text-[var(--gw-white-faint)] mt-5 pt-4 border-t border-[var(--gw-border)]">Protocol: LoRa / Range: 2-15km / Bandwidth: 0.3-50 kbps</div>
      </>
    ),
  },
  {
    name: "Reticulum",
    desc: "Off-grid mesh networking protocol. Designed for low-power radios and intermittent connectivity.",
    tag: "Core",
    techNote: "Protocol: Reticulum / Interfaces: any / Routing: autonomous",
    modalBody: (
      <>
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">What Is It?</h3>
        <p>Reticulum Network Stack for resilient communication over ANY interface. A meta-transport that auto-selects the best underlying link.</p>
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">Spec</h3>
        <SpecTable rows={[
          ["Protocol", "Reticulum"],
          ["Interfaces", "TCP, UDP, BLE, LoRa, serial, I2P"],
          ["Routing", "Autonomous pathfinding"],
          ["Encryption", "Curve25519 + AES (built-in)"],
        ]} />
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">Meta-transport</h3>
        <p>Automatically selects the best underlying interface for communication — multi-interface resilience.</p>
        <div className="font-mono-gw text-[12px] text-[var(--gw-white-faint)] mt-5 pt-4 border-t border-[var(--gw-border)]">Protocol: Reticulum / Interfaces: any / Routing: autonomous</div>
      </>
    ),
  },
];

const ARMOR_TRANSPORTS: Transport[] = [
  {
    name: "obfs4",
    desc: "Pluggable anti-DPI transport. Scrambles packet shape to bypass censorship and deep packet inspection.",
    tag: "Armor",
    techNote: "Protocol: obfs4 / Elligator2 / Active-probe resistant",
    modalBody: (
      <>
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">What Is It?</h3>
        <p>Obfuscation protocol that makes traffic indistinguishable from random noise. From the Tor Project.</p>
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">Spec</h3>
        <SpecTable rows={[
          ["Protocol", "obfs4"],
          ["Port", "443 (configurable)"],
          ["Use case", "Highly censored networks, active DPI"],
        ]} />
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">How it works</h3>
        <p>Elligator2 mapping makes public keys indistinguishable from random. Handshake produces random-looking bytes. Active probing resisted — servers respond with random data to unauthenticated probes.</p>
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">Bridge distribution</h3>
        <ul className="list-disc list-inside space-y-1">
          <li>BridgeDB (email-based)</li>
          <li>DHT (encrypted bridge addresses)</li>
          <li>Manual (trusted contacts)</li>
        </ul>
        <div className="font-mono-gw text-[12px] text-[var(--gw-white-faint)] mt-5 pt-4 border-t border-[var(--gw-border)]">Protocol: obfs4 / Elligator2 / Active-probe resistant</div>
      </>
    ),
  },
  {
    name: "WebSocket Camouflage",
    desc: "Disguises traffic as benign WebSocket connections. Blends into normal web traffic.",
    tag: "Armor",
    techNote: "Protocol: WS over TLS / Port: 443 / DPI-evading",
    modalBody: (
      <>
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">What Is It?</h3>
        <p>WebSocket traffic disguised as standard web browsing. Indistinguishable from normal WebSocket.</p>
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">Spec</h3>
        <SpecTable rows={[
          ["Protocol", "WebSocket (RFC 6455) over TLS"],
          ["Port", "443"],
          ["Appearance", "Standard WebSocket"],
          ["Use case", "HTTPS-allowed, P2P-blocked"],
        ]} />
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">How it works</h3>
        <p>WebSocket to a user-operated bridge. Traffic wrapped in standard frames. DPI sees only normal WebSocket.</p>
        <div className="font-mono-gw text-[12px] text-[var(--gw-white-faint)] mt-5 pt-4 border-t border-[var(--gw-border)]">Protocol: WS over TLS / Port: 443 / DPI-evading</div>
      </>
    ),
  },
  {
    name: "Shadowsocks AEAD",
    desc: "AEAD-based proxy protocol. Widely used censorship circumvention, proven in the field.",
    tag: "Armor",
    techNote: "Protocol: SS AEAD / Cipher: ChaCha20-IETF-Poly1305 / Probe-resistant",
    modalBody: (
      <>
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">What Is It?</h3>
        <p>Encrypted proxy protocol with AEAD ciphers. Proven in the field for censorship circumvention.</p>
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">Spec</h3>
        <SpecTable rows={[
          ["Protocol", "Shadowsocks AEAD (SIP004)"],
          ["Cipher", "ChaCha20-IETF-Poly1305"],
          ["Port", "8388 (configurable)"],
          ["Use case", "Active-probing censors"],
        ]} />
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">How it works</h3>
        <p>SS server runs on an uncensored bridge. Client connects as proxy. AEAD authentication resists active probing.</p>
        <div className="font-mono-gw text-[12px] text-[var(--gw-white-faint)] mt-5 pt-4 border-t border-[var(--gw-border)]">Protocol: SS AEAD / Cipher: ChaCha20-IETF-Poly1305 / Probe-resistant</div>
      </>
    ),
  },
  {
    name: "TLS Camouflage",
    desc: "Wraps traffic in standard TLS. Indistinguishable from HTTPS to a passive observer.",
    tag: "Armor",
    techNote: "Protocol: TLS 1.3 + ALPN / Port: 443 / Looks like HTTPS",
    modalBody: (
      <>
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">What Is It?</h3>
        <p>TLS traffic that appears as standard HTTPS to passive observers.</p>
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">Spec</h3>
        <SpecTable rows={[
          ["Protocol", "TLS 1.3 + custom ALPN"],
          ["Port", "443"],
          ["Cert", "Self-signed or Let's Encrypt"],
          ["Use case", "SNI/ALPN-inspecting networks"],
        ]} />
        <h3 className="font-display text-lg font-semibold text-[var(--gw-white)]">How it works</h3>
        <p>TLS handshake with matching cert, custom ALPN for GhostWire. Post-handshake, GhostWire runs inside the encrypted tunnel. Passive observers see standard HTTPS.</p>
        <div className="font-mono-gw text-[12px] text-[var(--gw-white-faint)] mt-5 pt-4 border-t border-[var(--gw-border)]">Protocol: TLS 1.3 + ALPN / Port: 443 / Looks like HTTPS</div>
      </>
    ),
  },
];

const STEPS = [
  {
    n: "01",
    title: "No central server",
    text: "Devices connect directly via libp2p. Pure P2P, no single point of failure.",
  },
  {
    n: "02",
    title: "Multi-transport",
    text: "Live: 5 core transports (Stealth TCP, WebRTC, BLE, LoRa, Reticulum) + 4 Traffic Armor anti-DPI modes (obfs4, WebSocket, Shadowsocks, TLS Camo). Briar/Meshtastic/Matrix bridges experimental. Auto-failover.",
  },
  {
    n: "03",
    title: "AI-powered routing",
    text: "4-layer ML stack: LightGBM (76.7μs) → Gemma 4 → GNN → PRoPHET fallback.",
  },
  {
    n: "04",
    title: "End-to-end encrypted",
    text: "XChaCha20-Poly1305 AEAD, X25519 key exchange with forward-secrecy ratchet, post-quantum ML-KEM-768 hybrid, Sphinx onion routing.",
  },
  {
    n: "05",
    title: "Delay-tolerant delivery",
    text: "PRoPHET epidemic routing stores and carries messages when peers are offline. Delivered when the mesh reconnects.",
  },
];

export function HowItWorksSection() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [active, setActive] = useState<Transport | null>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;
    const prefersReduced = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

    let rafId = 0;
    let t = 0;

    const W = 480;
    const H = 360;
    const dpr = Math.min(window.devicePixelRatio || 1, 2);
    canvas.width = W * dpr;
    canvas.height = H * dpr;
    canvas.style.width = "100%";
    canvas.style.height = "auto";
    ctx.scale(dpr, dpr);

    // Fixed node positions for a stable diagram
    const nodes = [
      { x: 80, y: 80, hub: true },
      { x: 240, y: 60, hub: false },
      { x: 400, y: 100, hub: true },
      { x: 60, y: 220, hub: false },
      { x: 200, y: 200, hub: true },
      { x: 360, y: 240, hub: false },
      { x: 120, y: 320, hub: true },
      { x: 300, y: 310, hub: false },
      { x: 420, y: 300, hub: false },
    ];
    const edges = [
      [0, 1], [1, 2], [0, 3], [0, 4], [1, 4], [2, 5], [3, 4], [4, 5],
      [3, 6], [4, 6], [4, 7], [5, 7], [6, 7], [7, 8], [5, 8], [2, 8],
    ];

    function draw() {
      if (!ctx) return;
      ctx.clearRect(0, 0, W, H);
      t += 0.01;

      // Draw edges
      ctx.lineWidth = 0.8;
      for (const [a, b] of edges) {
        const na = nodes[a];
        const nb = nodes[b];
        const dist = Math.hypot(na.x - nb.x, na.y - nb.y);
        const alpha = Math.max(0.06, 0.25 - dist / 1200);
        ctx.strokeStyle = `rgba(0, 220, 154, ${alpha})`;
        ctx.beginPath();
        ctx.moveTo(na.x, na.y);
        ctx.lineTo(nb.x, nb.y);
        ctx.stroke();
      }

      // Draw a packet hopping along edges (loops through a path)
      const path = [0, 4, 7, 8, 5, 2];
      const segCount = path.length - 1;
      const segProgress = (t * 0.5) % segCount;
      const segIdx = Math.floor(segProgress);
      const localT = segProgress - segIdx;
      const from = nodes[path[segIdx]];
      const to = nodes[path[segIdx + 1]];
      if (from && to) {
        const px = from.x + (to.x - from.x) * localT;
        const py = from.y + (to.y - from.y) * localT;
        const g = ctx.createRadialGradient(px, py, 0, px, py, 12);
        g.addColorStop(0, "rgba(0, 220, 154, 0.85)");
        g.addColorStop(1, "rgba(0, 220, 154, 0)");
        ctx.fillStyle = g;
        ctx.beginPath();
        ctx.arc(px, py, 12, 0, Math.PI * 2);
        ctx.fill();
        ctx.fillStyle = "#F0EDE6";
        ctx.beginPath();
        ctx.arc(px, py, 2, 0, Math.PI * 2);
        ctx.fill();
      }

      // Draw nodes
      for (const n of nodes) {
        if (n.hub) {
          const g = ctx.createRadialGradient(n.x, n.y, 0, n.x, n.y, 14);
          g.addColorStop(0, "rgba(0, 220, 154, 0.35)");
          g.addColorStop(1, "rgba(0, 220, 154, 0)");
          ctx.fillStyle = g;
          ctx.beginPath();
          ctx.arc(n.x, n.y, 14, 0, Math.PI * 2);
          ctx.fill();
        }
        ctx.fillStyle = n.hub ? "#00DC9A" : "rgba(0, 220, 154, 0.6)";
        ctx.beginPath();
        ctx.arc(n.x, n.y, n.hub ? 4.5 : 3, 0, Math.PI * 2);
        ctx.fill();
      }

      if (!prefersReduced) rafId = requestAnimationFrame(draw);
    }

    draw();
    return () => cancelAnimationFrame(rafId);
  }, []);

  return (
    <section id="how-it-works" className="py-24 border-t border-[var(--gw-border)]">
      <div className="max-w-[1160px] mx-auto px-6 sm:px-8">
        <div className="gw-section-label mb-3">{"// how it works"}</div>
        <h2 className="font-display text-3xl sm:text-5xl font-bold mb-10 text-[var(--gw-white)]">
          Mesh networking for the real world
        </h2>
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-10 items-start">
          <div>
            <div className="bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-4 sm:p-6">
              <canvas
                ref={canvasRef}
                aria-label="Animated mesh network diagram showing how messages hop between nodes"
                role="img"
                className="w-full"
              />
              <p className="font-mono-gw text-[12px] text-[var(--gw-white-faint)] mt-3 text-center">
                Messages hop node-to-node. Remove any node — the mesh routes around it.
              </p>
            </div>
          </div>
          <div className="space-y-3">
            {STEPS.map((s) => (
              <div
                key={s.n}
                className="flex gap-4 bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-5 gw-card-hover"
              >
                <div className="font-mono-gw text-2xl font-semibold text-[var(--gw-green)] shrink-0">
                  {s.n}
                </div>
                <div>
                  <h3 className="font-display text-lg font-semibold mb-1 text-[var(--gw-white)]">
                    {s.title}
                  </h3>
                  <p className="text-[14px] text-[var(--gw-white-dim)] leading-relaxed">
                    {s.text}
                  </p>
                </div>
              </div>
            ))}
          </div>
        </div>

        <h3 className="font-display text-2xl font-bold mb-2 mt-14 text-[var(--gw-white)]">
          Transport layer — 9 live transports, auto-failover
        </h3>
        <p className="text-[var(--gw-white-dim)] mb-8 max-w-2xl text-[15px]">
          GhostWire auto-selects the best available link and fails over when one drops. 5 core transports move packets; 4 Traffic Armor modes disguise them. Implement the Transport trait to plug in your own. Click any card for the full spec.
        </p>
        <div className="font-mono-gw text-[11px] tracking-[0.1em] uppercase text-[var(--gw-white-faint)] mb-3">
          Core transports (5)
        </div>
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
          {CORE_TRANSPORTS.map((t) => (
            <TransportCard key={t.name} t={t} onOpen={() => setActive(t)} />
          ))}
        </div>
        <div className="font-mono-gw text-[11px] tracking-[0.1em] uppercase text-[var(--gw-white-faint)] mb-3 mt-6">
          Traffic Armor — anti-DPI (4)
        </div>
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
          {ARMOR_TRANSPORTS.map((t) => (
            <TransportCard key={t.name} t={t} onOpen={() => setActive(t)} />
          ))}
        </div>

        <FeatureModal
          open={active !== null}
          onClose={() => setActive(null)}
          tag={active?.tag ?? ""}
          title={active?.name ?? ""}
        >
          {active?.modalBody}
        </FeatureModal>
      </div>
    </section>
  );
}
