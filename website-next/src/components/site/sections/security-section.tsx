'use client';

import { useState } from "react";
import { FeatureModal } from "@/components/site/feature-modal";

type ThreatLevel = "LOW" | "MEDIUM" | "HIGH";

interface Threat {
  id: string;
  title: string;
  level: ThreatLevel;
  text: string;
  tag: string;
}

const THREATS: Threat[] = [
  {
    id: "surveillance",
    title: "Network Surveillance",
    level: "LOW",
    text: "Passive eavesdroppers cannot read content (XChaCha20-Poly1305 AEAD) or trace source (Sphinx onion routing). Cover traffic masks timing.",
    tag: "threat",
  },
  {
    id: "mitm",
    title: "Active MitM",
    level: "LOW",
    text: "X25519 + Ed25519 signatures authenticate peers. ML-KEM-768 hybrid protects against future quantum adversaries.",
    tag: "threat",
  },
  {
    id: "sybil",
    title: "Sybil Attacks",
    level: "MEDIUM",
    text: "Proof-of-work node admission + TrustStore reputation make large-scale Sybil attacks economically infeasible on commodity hardware.",
    tag: "threat",
  },
  {
    id: "infrastructure",
    title: "Infrastructure Failure",
    level: "MEDIUM",
    text: "Cell tower outages, ISP failures, power grid collapse. GhostWire operates independently via TCP/IP mesh, Stealth TCP, Bluetooth LE, LoRa, and Reticulum. Messages queue and deliver when connectivity resumes.",
    tag: "threat",
  },
  {
    id: "censorship",
    title: "Censorship & DPI",
    level: "MEDIUM",
    text: "Stealth TCP reshapes handshake fingerprints. obfs4, Shadowsocks, WebSocket, and TLS pluggable transports scramble packet shape. Tor transport in development.",
    tag: "threat",
  },
  {
    id: "seizure",
    title: "Physical Seizure",
    level: "HIGH",
    text: "If your device is seized while unlocked, local data is at risk. Encrypted-at-rest stores, identity auto-lock, and disposable identities are the mitigations. See the Transparency section.",
    tag: "threat",
  },
];

function levelClass(level: ThreatLevel): string {
  switch (level) {
    case "LOW":
      return "text-[var(--gw-green)] border-[var(--gw-border-2)] bg-[var(--gw-green-dim)]";
    case "MEDIUM":
      return "text-[var(--gw-orange)] border-[rgba(249,115,22,0.3)] bg-[rgba(249,115,22,0.12)]";
    case "HIGH":
      return "text-red-400 border-red-500/30 bg-red-500/10";
  }
}

function ModalBody({ id }: { id: string }) {
  switch (id) {
    case "surveillance":
      return (
        <>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              The threat
            </h3>
            <p>
              Passive eavesdroppers monitor network traffic, hoping to read
              content or trace who&apos;s talking to whom.
            </p>
          </div>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              How GhostWire mitigates
            </h3>
            <ul className="space-y-1.5 list-none p-0 m-0">
              <li>• XChaCha20-Poly1305 AEAD encrypts all content (no plaintext visible)</li>
              <li>• Sphinx onion routing hides source/destination from intermediate hops</li>
              <li>• Cover traffic masks timing patterns</li>
              <li>• BLAKE3 content addressing prevents correlation by payload hash</li>
            </ul>
          </div>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              Residual risk
            </h3>
            <p>
              A global passive adversary with visibility into many links may
              still correlate traffic volume and timing. Full anonymity against
              such an adversary is a harder problem.
            </p>
          </div>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              Strengthen with
            </h3>
            <p>
              Enable cover traffic, use Sphinx routing for sensitive messages,
              consider the Tor transport (in development) for stronger anonymity.
            </p>
          </div>
        </>
      );
    case "mitm":
      return (
        <>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              The threat
            </h3>
            <p>
              An active adversary intercepts and relays traffic, attempting to
              substitute keys, modify messages, or impersonate peers.
            </p>
          </div>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              How GhostWire mitigates
            </h3>
            <ul className="space-y-1.5 list-none p-0 m-0">
              <li>• X25519 + Ed25519 authenticate peers (TOFU key pinning detects substitution)</li>
              <li>• ML-KEM-768 hybrid PQ defends against future quantum adversaries</li>
              <li>• AEAD authentication detects any modification</li>
              <li>• Web of Trust signed attestations verify identity</li>
            </ul>
          </div>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              Residual risk
            </h3>
            <p>
              A first-connection MitM before TOFU pinning succeeds could
              substitute keys. Verify keys out-of-band (QR pairing) for
              high-trust contexts.
            </p>
          </div>
        </>
      );
    case "sybil":
      return (
        <>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              The threat
            </h3>
            <p>
              An adversary spins up thousands of fake identities to outvote,
              isolate, or de-anonymize honest nodes.
            </p>
          </div>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              How GhostWire mitigates
            </h3>
            <ul className="space-y-1.5 list-none p-0 m-0">
              <li>• Proof-of-work node admission (cheap for one node, expensive for a botnet)</li>
              <li>• TrustStore reputation system (bad actors get isolated)</li>
              <li>• Web of Trust signed attestations (verifiable identity graph)</li>
              <li>• Peer scoring (planned in Phase 3)</li>
            </ul>
          </div>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              Residual risk
            </h3>
            <p>
              An adversary willing to burn compute on PoW can still create some
              Sybils. Reputation + Web of Trust limit their influence. Full peer
              scoring lands in Phase 3.
            </p>
          </div>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              Status
            </h3>
            <p>
              Peer scoring is <span className="gw-in-progress">In Progress</span>{" "}
              (Phase 3).
            </p>
          </div>
        </>
      );
    case "infrastructure":
      return (
        <>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              The threat
            </h3>
            <p>
              Cell tower outages, ISP failures, power grid collapse, natural
              disasters. Centralized comms break precisely when needed most.
            </p>
          </div>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              How GhostWire mitigates
            </h3>
            <ul className="space-y-1.5 list-none p-0 m-0">
              <li>• TCP/IP mesh + Stealth TCP keep working without internet</li>
              <li>• Reticulum meta-transport auto-selects any available interface</li>
              <li>• Bluetooth LE + LoRa for device-to-device/off-grid</li>
              <li>• Delay-tolerant delivery (PRoPHET) queues messages until connectivity resumes</li>
            </ul>
          </div>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              Residual risk
            </h3>
            <p>
              If ALL transports are down (no WiFi, no BLE, no LoRa, no wired),
              messages queue locally and deliver when any transport returns.
              Physical device loss is a separate threat (see Physical Seizure).
            </p>
          </div>
        </>
      );
    case "censorship":
      return (
        <>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              The threat
            </h3>
            <p>
              National firewalls and ISPs use Deep Packet Inspection to block,
              throttle, or surveil targeted protocols.
            </p>
          </div>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              How GhostWire mitigates
            </h3>
            <ul className="space-y-1.5 list-none p-0 m-0">
              <li>• 4 Traffic Armor transports disguise traffic (obfs4 = random noise, WebSocket Camo = web traffic, Shadowsocks AEAD = proxy, TLS Camo = HTTPS)</li>
              <li>• Stealth TCP randomizes ports</li>
              <li>• Pluggable transport architecture (implement the Transport trait)</li>
            </ul>
          </div>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              Bridge distribution
            </h3>
            <p>
              obfs4 bridges distributed via BridgeDB (email), DHT (encrypted
              addresses), manual (trusted contacts).
            </p>
          </div>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              Residual risk
            </h3>
            <p>
              A censor willing to block ALL HTTPS/WebSocket can still deny
              service. The arms race is ongoing; the pluggable transport model
              lets us add new transports as old ones get blocked.
            </p>
          </div>
        </>
      );
    case "seizure":
      return (
        <>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              The threat
            </h3>
            <p>
              An adversary with physical access to your unlocked device can read
              local stores, identity keys, and contact lists — even with
              encryption at rest.
            </p>
          </div>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              How GhostWire mitigates
            </h3>
            <ul className="space-y-1.5 list-none p-0 m-0">
              <li>• Encrypted-at-rest local stores (Argon2id KDF, 64 MiB memory cost)</li>
              <li>• Identity vault auto-lock after inactivity</li>
              <li>• Duress PIN unlocks a decoy identity</li>
              <li>• Panic wipe securely erases keys/stores</li>
              <li>• Dead man&apos;s switch auto-locks/wipes if no check-in</li>
              <li>• Shamir Recovery (T-of-N) so a seized device doesn&apos;t mean a lost identity</li>
            </ul>
          </div>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              Residual risk
            </h3>
            <p>
              If the device is seized WHILE UNLOCKED, an adversary can read the
              live session. Layered endpoint hygiene (disk encryption,
              dedicated device/VM) is non-negotiable for high-risk users.
            </p>
          </div>
          <div>
            <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2">
              Status
            </h3>
            <p>
              Shamir Recovery is live (<code className="font-mono-gw text-[var(--gw-green)]">recovery_shamir.rs</code>).
              Coercion resistance (duress/panic/dead-man) is live.
            </p>
          </div>
        </>
      );
    default:
      return null;
  }
}

const CRYPTO_ROWS = [
  {
    purpose: "Session encryption",
    primitive: "XChaCha20-Poly1305",
    notes: "Extended-nonce AEAD, 24-byte nonce",
  },
  {
    purpose: "Key agreement",
    primitive: "X25519",
    notes: "ECDH, forward-secrecy ratchet",
  },
  {
    purpose: "Signing",
    primitive: "Ed25519",
    notes: "Identity keys",
  },
  {
    purpose: "Post-quantum KEM",
    primitive: "ML-KEM-768 (FIPS 203)",
    notes: "Hybrid with X25519",
  },
  {
    purpose: "Forward secrecy",
    primitive: "Symmetric ratchet",
    notes: "Chain key aliasing",
  },
  {
    purpose: "Onion routing",
    primitive: "Sphinx (sphinx-rs)",
    notes: "SURBs for sender-hidden replies",
  },
  {
    purpose: "Key derivation",
    primitive: "HKDF-SHA256",
    notes: "Key stretching",
  },
  {
    purpose: "Pluggable transports",
    primitive: "obfs4, Shadowsocks, WebSocket, TLS",
    notes: "Anti-DPI",
  },
  {
    purpose: "Delay-tolerant routing",
    primitive: "PRoPHET",
    notes: "Epidemic store-carry-forward",
  },
];

const DISCLOSURE_META = [
  { label: "Report to", value: "security@ghostwire.cc", href: "mailto:security@ghostwire.cc" },
  { label: "PGP (encrypted)", value: "mirungu015@proton.me", href: "mailto:mirungu015@proton.me" },
  { label: "Response time", value: "Within 48 hours" },
  { label: "Credit", value: "We publicly credit responsible reporters" },
  { label: "Scope", value: "All GhostWire code, dependencies, and infrastructure" },
  { label: "PGP preferred", value: "For sensitive reports, encrypt with our PGP key" },
];

interface AuditStat {
  stat: string;
  label: string;
  sub: string;
  color: string;
  highlight?: boolean;
}

const AUDIT_STATS: AuditStat[] = [
  {
    stat: "0",
    label: "Critical issues",
    sub: "3 found and fixed in the January 2026 audit cycle.",
    color: "var(--gw-green)",
  },
  {
    stat: "0",
    label: "High-priority issues",
    sub: "5 found and fixed. All CVE patches applied.",
    color: "var(--gw-green)",
  },
  {
    stat: "A+",
    label: "Internal audit rating",
    sub: "Production-ready per the February 2026 internal review.",
    color: "var(--gw-green)",
  },
  {
    stat: "Pending",
    label: "Independent audit",
    sub: "Seeking funding for a third-party security audit. Contact admin@ghostwire.cc.",
    color: "var(--gw-orange)",
    highlight: true,
  },
];

export function SecuritySection() {
  const [active, setActive] = useState<Threat | null>(null);

  return (
    <section id="security" className="py-24 border-t border-[var(--gw-border)]">
      <div className="max-w-[1160px] mx-auto px-6 sm:px-8">
        <div className="gw-section-label mb-3">{"// security"}</div>
        <h2 className="font-display text-3xl sm:text-5xl font-bold mb-6 text-[var(--gw-white)]">
          Security: Threat Model and End-to-End Encryption Details
        </h2>
        <p className="text-[16px] text-[var(--gw-white-dim)] leading-relaxed max-w-3xl mb-14">
          GhostWire&apos;s security model is built on layered defense: XChaCha20-Poly1305
          end-to-end encryption, Sybil-resistant peer admission, traffic obfuscation,
          and an honest threat model that documents what we do and do not protect
          against.
        </p>

        {/* A. Threat Model */}
        <div id="threat-model" className="mb-16 scroll-mt-24">
          <h3 className="font-display text-2xl font-bold mb-6 text-[var(--gw-white)]">
            Threat Model
          </h3>
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
            {THREATS.map((t) => (
              <button
                key={t.id}
                onClick={() => setActive(t)}
                className="text-left bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-5 w-full gw-card-hover focus:outline-none focus:border-[var(--gw-green)]"
              >
                <div className="flex items-center justify-between mb-2">
                  <h4 className="font-display text-base font-semibold text-[var(--gw-white)]">
                    {t.title}
                  </h4>
                  <span
                    className={`font-mono-gw text-[10px] tracking-[0.08em] uppercase px-2 py-0.5 rounded border ${levelClass(
                      t.level
                    )}`}
                  >
                    {t.level}
                  </span>
                </div>
                <p className="text-[13px] text-[var(--gw-white-dim)] leading-relaxed mb-3">
                  {t.text}
                </p>
                <span className="font-mono-gw text-[12px] text-[var(--gw-green)]">
                  Click for mitigation →
                </span>
              </button>
            ))}
          </div>
        </div>

        {/* Audit Status */}
        <div className="mb-16">
          <h3 className="font-display text-2xl font-bold mb-2 text-[var(--gw-white)] mt-16">
            Security audit status
          </h3>
          <p className="text-[var(--gw-white-dim)] mb-8 text-[15px]">
            Internal audit complete (February 2026). Independent third-party audit pending.
          </p>
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
            {AUDIT_STATS.map((s) => (
              <div
                key={s.label}
                className={`bg-[var(--gw-bg-2)] border rounded-lg p-6 gw-card-hover ${
                  s.highlight
                    ? "border-[rgba(249,115,22,0.3)]"
                    : "border-[var(--gw-border)]"
                }`}
              >
                <div
                  className="font-mono-gw text-3xl font-bold mb-2"
                  style={{ color: s.color }}
                >
                  {s.stat}
                </div>
                <div className="font-display text-base font-semibold text-[var(--gw-white)] mb-2">
                  {s.label}
                </div>
                <p className="text-[13px] text-[var(--gw-white-dim)] leading-relaxed">
                  {s.sub}
                </p>
              </div>
            ))}
          </div>
        </div>

        {/* B. Cryptographic Stack */}
        <div id="cryptography" className="mb-16 scroll-mt-24">
          <h3 className="font-display text-2xl font-bold mb-6 text-[var(--gw-white)]">
            Cryptographic Stack
          </h3>
          <div className="overflow-x-auto bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg">
            <table className="w-full text-left border-collapse">
              <thead>
                <tr>
                  <th className="font-mono-gw text-[11px] tracking-[0.08em] uppercase text-[var(--gw-white-faint)] border-b border-[var(--gw-border-2)] py-3 px-4">
                    Purpose
                  </th>
                  <th className="font-mono-gw text-[11px] tracking-[0.08em] uppercase text-[var(--gw-white-faint)] border-b border-[var(--gw-border-2)] py-3 px-4">
                    Primitive
                  </th>
                  <th className="font-mono-gw text-[11px] tracking-[0.08em] uppercase text-[var(--gw-white-faint)] border-b border-[var(--gw-border-2)] py-3 px-4">
                    Notes
                  </th>
                </tr>
              </thead>
              <tbody>
                {CRYPTO_ROWS.map((row) => (
                  <tr
                    key={row.purpose}
                    className="border-b border-[var(--gw-border)] hover:bg-[var(--gw-bg-3)] transition-colors"
                  >
                    <td className="py-3 px-4 text-[13px] text-[var(--gw-white)] font-medium">
                      {row.purpose}
                    </td>
                    <td className="py-3 px-4 text-[13px] text-[var(--gw-white-dim)] font-mono-gw">
                      {row.primitive}
                    </td>
                    <td className="py-3 px-4 text-[13px] text-[var(--gw-white-dim)]">
                      {row.notes}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>

        {/* Encrypted vs Not Encrypted */}
        <div className="mb-16">
          <h3 className="font-display text-2xl font-bold mb-2 text-[var(--gw-white)] mt-16">
            What&apos;s encrypted — and what isn&apos;t
          </h3>
          <p className="text-[var(--gw-white-dim)] mb-8 text-[15px]">
            Honest disclosure: here&apos;s exactly what GhostWire protects, and what it doesn&apos;t.
          </p>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="bg-[var(--gw-bg-2)] border border-[var(--gw-border-2)] rounded-lg p-6">
              <h4 className="font-display text-lg font-semibold mb-4 text-[var(--gw-green)]">
                ✓ Encrypted
              </h4>
              <ul className="space-y-2.5 list-none p-0">
                <li className="text-[13px] text-[var(--gw-white-dim)] leading-relaxed pl-5 relative">
                  <span className="absolute left-0 text-[var(--gw-green)]">✓</span>
                  Message payloads (XChaCha20-Poly1305 AEAD)
                </li>
                <li className="text-[13px] text-[var(--gw-white-dim)] leading-relaxed pl-5 relative">
                  <span className="absolute left-0 text-[var(--gw-green)]">✓</span>
                  File chunks (XChaCha20-Poly1305)
                </li>
                <li className="text-[13px] text-[var(--gw-white-dim)] leading-relaxed pl-5 relative">
                  <span className="absolute left-0 text-[var(--gw-green)]">✓</span>
                  Session state (encrypted vault)
                </li>
                <li className="text-[13px] text-[var(--gw-white-dim)] leading-relaxed pl-5 relative">
                  <span className="absolute left-0 text-[var(--gw-green)]">✓</span>
                  Mailbox contents (encrypted at rest)
                </li>
                <li className="text-[13px] text-[var(--gw-white-dim)] leading-relaxed pl-5 relative">
                  <span className="absolute left-0 text-[var(--gw-green)]">✓</span>
                  Knowledge Ark entries (encrypted)
                </li>
                <li className="text-[13px] text-[var(--gw-white-dim)] leading-relaxed pl-5 relative">
                  <span className="absolute left-0 text-[var(--gw-green)]">✓</span>
                  Identity keys (encrypted local vault)
                </li>
                <li className="text-[13px] text-[var(--gw-white-dim)] leading-relaxed pl-5 relative">
                  <span className="absolute left-0 text-[var(--gw-green)]">✓</span>
                  Onion-routed packets (Sphinx layers)
                </li>
              </ul>
            </div>
            <div className="bg-[var(--gw-bg-2)] border border-[rgba(249,115,22,0.3)] rounded-lg p-6">
              <h4 className="font-display text-lg font-semibold mb-4 text-[var(--gw-orange)]">
                ⚠ Not encrypted (metadata)
              </h4>
              <ul className="space-y-2.5 list-none p-0">
                <li className="text-[13px] text-[var(--gw-white-dim)] leading-relaxed pl-5 relative">
                  <span className="absolute left-0 text-[var(--gw-orange)]">⚠</span>
                  Packet timing{" "}
                  <span className="text-[var(--gw-white-faint)] text-[12px]">
                    — mitigated by cover traffic
                  </span>
                </li>
                <li className="text-[13px] text-[var(--gw-white-dim)] leading-relaxed pl-5 relative">
                  <span className="absolute left-0 text-[var(--gw-orange)]">⚠</span>
                  Packet size patterns{" "}
                  <span className="text-[var(--gw-white-faint)] text-[12px]">
                    — mitigated by padding
                  </span>
                </li>
                <li className="text-[13px] text-[var(--gw-white-dim)] leading-relaxed pl-5 relative">
                  <span className="absolute left-0 text-[var(--gw-orange)]">⚠</span>
                  Network topology{" "}
                  <span className="text-[var(--gw-white-faint)] text-[12px]">
                    — visible to mesh participants
                  </span>
                </li>
                <li className="text-[13px] text-[var(--gw-white-dim)] leading-relaxed pl-5 relative">
                  <span className="absolute left-0 text-[var(--gw-orange)]">⚠</span>
                  Peer existence{" "}
                  <span className="text-[var(--gw-white-faint)] text-[12px]">
                    — visible to direct neighbors
                  </span>
                </li>
                <li className="text-[13px] text-[var(--gw-white-dim)] leading-relaxed pl-5 relative">
                  <span className="absolute left-0 text-[var(--gw-orange)]">⚠</span>
                  Traffic volume{" "}
                  <span className="text-[var(--gw-white-faint)] text-[12px]">
                    — mitigated by cover traffic
                  </span>
                </li>
              </ul>
              <p className="text-[12px] text-[var(--gw-white-faint)] mt-4 pt-4 border-t border-[var(--gw-border)]">
                Full anonymity against a global passive adversary is a harder problem. Sphinx + cover traffic raise the cost but cannot eliminate it.
              </p>
            </div>
          </div>
        </div>

        {/* C. Vulnerability Disclosure */}
        <div id="disclosure" className="scroll-mt-24">
          <h3 className="font-display text-2xl font-bold mb-6 text-[var(--gw-white)]">
            Responsible Disclosure
          </h3>
          <div className="bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-6 sm:p-7">
            <p className="text-[15px] text-[var(--gw-white-dim)] leading-relaxed mb-5">
              We take security reports seriously. If you believe you have found a vulnerability,
              please report it responsibly.
            </p>
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
              {DISCLOSURE_META.map((item) => (
                <div
                  key={item.label}
                  className="border border-[var(--gw-border)] rounded-md p-4 bg-[var(--gw-bg)]"
                >
                  <div className="font-mono-gw text-[10px] tracking-[0.1em] uppercase text-[var(--gw-white-faint)] mb-1.5">
                    {item.label}
                  </div>
                  {item.href ? (
                    <a
                      href={item.href}
                      className="text-[14px] text-[var(--gw-green)] hover:underline font-mono-gw"
                    >
                      {item.value}
                    </a>
                  ) : (
                    <div className="text-[14px] text-[var(--gw-white)]">{item.value}</div>
                  )}
                </div>
              ))}
            </div>
            <div className="mt-4 p-4 border border-[var(--gw-orange)]/30 bg-[rgba(249,115,22,0.06)] rounded text-[13px] text-[var(--gw-white-dim)]">
              Do not publicly disclose vulnerabilities before we have had a chance to investigate
              and patch. Coordinated disclosure protects users.
            </div>
          </div>
        </div>
      </div>

      <FeatureModal
        open={active !== null}
        onClose={() => setActive(null)}
        tag={active?.tag ?? ""}
        title={active?.title ?? ""}
      >
        {active ? <ModalBody id={active.id} /> : null}
      </FeatureModal>
    </section>
  );
}
