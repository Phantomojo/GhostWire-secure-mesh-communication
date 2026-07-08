'use client'

import { useState } from "react";
import { FeatureModal } from "@/components/site/feature-modal";

interface Limitation {
  id: string;
  icon: string;
  title: string;
  blurb: string;
  modalBody: React.ReactNode;
}

const LIMITATIONS: Limitation[] = [
  {
    id: "physical",
    icon: "⚠",
    title: "Physical device seizure",
    blurb: "If your device is seized, local data may be at risk.",
    modalBody: (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            The Risk
          </h3>
          <p>
            If your device is seized while unlocked, an adversary with physical
            access can read local message stores, identity keys, and contact
            lists — even with encryption at rest.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Mitigation
          </h3>
          <ul className="list-disc list-outside pl-5 space-y-1">
            <li>
              Enable full-disk encryption with a strong passphrase (LUKS on
              Linux, FileVault on macOS, BitLocker on Windows).
            </li>
            <li>
              Use GhostWire&apos;s identity lock feature — encrypted identity
              vault auto-locks after inactivity.
            </li>
            <li>
              For high-risk contexts, use disposable identities and burn them
              after each action.
            </li>
            <li>
              Consider Tails or Whonix on read-only media for ephemeral
              sessions.
            </li>
          </ul>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            What GhostWire Does
          </h3>
          <p>
            Local stores are encrypted at rest. Identity vaults auto-lock.
            Message history is per-identity, so burning an identity also burns
            its history.
          </p>
        </div>
      </>
    ),
  },
  {
    id: "anonymity",
    icon: "⚠",
    title: "Full anonymity",
    blurb:
      "Strong privacy, not complete anonymity. Traffic analysis remains a risk.",
    modalBody: (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Privacy vs Anonymity
          </h3>
          <p>
            Privacy means your messages are confidential. Anonymity means an
            adversary cannot link you to the act of communicating. GhostWire
            provides strong privacy; full anonymity against a global passive
            adversary is a harder problem.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Traffic Analysis Risk
          </h3>
          <p>
            An observer who can monitor timing and volume of packets across many
            network links may correlate send/receive events. Cover traffic and
            timing randomization raise the cost but cannot eliminate the risk.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Mitigation
          </h3>
          <ul className="list-disc list-outside pl-5 space-y-1">
            <li>
              Use Sphinx onion routing (built-in) to hide source/destination
              from intermediate hops.
            </li>
            <li>Enable cover traffic to mask message timing.</li>
            <li>
              For stronger anonymity, route over Tor (transport in development).
            </li>
            <li>
              Avoid publishing a long-lived username identity if anonymity is
              critical.
            </li>
          </ul>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Tor Transport
          </h3>
          <p>
            Full Tor onion-service transport is on the roadmap. obfs4 pluggable
            transport is already live.
          </p>
        </div>
      </>
    ),
  },
  {
    id: "endpoint",
    icon: "⚠",
    title: "Endpoint compromise",
    blurb:
      "If either device has malware, encryption can't protect content at that endpoint.",
    modalBody: (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            The Reality
          </h3>
          <p>
            End-to-end encryption protects messages in transit. If your device
            (or your recipient&apos;s device) is compromised by malware, the
            attacker can read messages from the screen, log keystrokes, or
            exfiltrate the decrypted store.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Mitigation
          </h3>
          <ul className="list-disc list-outside pl-5 space-y-1">
            <li>Keep your OS and browser updated.</li>
            <li>
              Run GhostWire on a dedicated device or VM for high-risk contexts.
            </li>
            <li>
              Use hardware security keys (FIDO2) for device unlock where
              supported.
            </li>
            <li>Audit installed software; avoid installing untrusted binaries.</li>
            <li>On Android, use a separate work profile for GhostWire.</li>
          </ul>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            What GhostWire Cannot Do
          </h3>
          <p>
            No messaging app can defend against a keylogger on the endpoint.
            Layered endpoint hygiene is non-negotiable for high-risk users.
          </p>
        </div>
      </>
    ),
  },
  {
    id: "audit",
    icon: "⚠",
    title: "Not yet independently audited",
    blurb:
      "No third-party security audit yet. Internal audits available in docs/.",
    modalBody: (
      <>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Audit Status
          </h3>
          <p>
            As of v0.2.0, GhostWire has not undergone an independent third-party
            security audit. Internal design reviews and threat modeling are
            documented in the repository, but they are not a substitute for a
            professional audit.
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            What We&apos;ve Done
          </h3>
          <ul className="list-disc list-outside pl-5 space-y-1">
            <li>Published a full threat model on the Security page.</li>
            <li>Documented the cryptographic stack with citations.</li>
            <li>
              Used audited primitives (libp2p, sphinx-rs, RustCrypto crates)
              where possible.
            </li>
            <li>
              Open-sourced the entire codebase under AGPL-3.0 for community
              review.
            </li>
          </ul>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            What&apos;s Next
          </h3>
          <p>
            We are actively seeking funding for a professional audit. If you can
            sponsor or perform one, contact{" "}
            <a
              href="mailto:admin@ghostwire.cc"
              className="text-[var(--gw-green)] hover:underline"
            >
              admin@ghostwire.cc
            </a>
            .
          </p>
        </div>
        <div>
          <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
            Responsible Disclosure
          </h3>
          <p>
            Report vulnerabilities to{" "}
            <a
              href="mailto:admin@ghostwire.cc"
              className="text-[var(--gw-green)] hover:underline"
            >
              admin@ghostwire.cc
            </a>{" "}
            (PGP preferred). We respond within 72 hours and credit responsible
            reporters.
          </p>
        </div>
      </>
    ),
  },
];

export function TransparencySection() {
  const [active, setActive] = useState<Limitation | null>(null);

  return (
    <section
      id="transparency"
      className="py-24 border-t border-[var(--gw-border)]"
    >
      <div className="max-w-[1160px] mx-auto px-6 sm:px-8">
        <div className="gw-section-label mb-3">{"// transparency"}</div>
        <h2 className="font-display text-3xl sm:text-4xl font-bold mb-3 text-[var(--gw-white)]">
          What GhostWire does NOT protect against
        </h2>
        <p className="text-[var(--gw-white-dim)] text-[15px] mb-10 max-w-2xl">
          Honest security requires honest limitations. Click each card for
          details and mitigation steps.
        </p>

        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
          {LIMITATIONS.map((l) => (
            <button
              key={l.id}
              onClick={() => setActive(l)}
              className="text-left bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-6 gw-card-hover focus:outline-none focus:border-[var(--gw-orange)]"
              aria-label={`Learn about ${l.title}`}
            >
              <div className="text-2xl text-[var(--gw-orange)] mb-3">
                {l.icon}
              </div>
              <h3 className="font-display text-lg font-semibold mb-2 text-[var(--gw-white)]">
                {l.title}
              </h3>
              <p className="text-[13px] text-[var(--gw-white-dim)] leading-relaxed mb-3">
                {l.blurb}
              </p>
              <span className="font-mono-gw text-[12px] text-[var(--gw-orange)]">
                Click for mitigation steps →
              </span>
            </button>
          ))}
        </div>
      </div>

      <FeatureModal
        open={active !== null}
        onClose={() => setActive(null)}
        tag="limitation"
        title={active?.title ?? ""}
      >
        {active?.modalBody}
      </FeatureModal>
    </section>
  );
}
