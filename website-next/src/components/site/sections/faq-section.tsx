import {
  Accordion,
  AccordionItem,
  AccordionTrigger,
  AccordionContent,
} from "@/components/ui/accordion";

interface FaqItem {
  q: string;
  a: string;
}

const FAQ_ITEMS: FaqItem[] = [
  {
    q: "What is GhostWire?",
    a: "GhostWire is an open protocol for resilient peer-to-peer communication — not an app. It provides the cryptographic, routing, and transport layers that developers build communication tools on top of. The reference implementation is written in Rust.",
  },
  {
    q: "How does P2P messaging work without a server?",
    a: "GhostWire uses libp2p GossipSub for pub/sub message propagation and Kademlia DHT for peer discovery and routing. Messages hop node-to-node across the mesh. Each device is both a client and a relay. There is no central server to seize, subpoena, or shut down.",
  },
  {
    q: "What encryption does GhostWire use?",
    a: "XChaCha20-Poly1305 authenticated encryption for all payloads, X25519 key agreement with a symmetric forward-secrecy ratchet, Ed25519 for identity signing, and ML-KEM-768 (NIST FIPS 203) hybrid post-quantum key exchange. Sphinx onion routing (via the published sphinx-rs crate) hides source and destination from intermediate hops.",
  },
  {
    q: "Does GhostWire work offline?",
    a: "Yes. Messages are delay-tolerant via PRoPHET epidemic routing — they store and carry locally when peers are offline and deliver when the mesh reconnects. GhostWire also runs over local-only transports (TCP/IP mesh, Bluetooth LE, LoRa, Reticulum) so it works during internet shutdowns and in fully off-grid environments.",
  },
  {
    q: "What is NAT traversal and does GhostWire support it?",
    a: "NAT traversal lets two devices behind home routers connect directly without a relay server. GhostWire uses libp2p's hole-punching (DCUtR) and WebRTC to establish direct P2P connections across NATs. For restrictive NATs, peers can still relay through other mesh nodes — there is no central relay.",
  },
  {
    q: "How are keys managed?",
    a: "Keys are generated on-device and stored in an encrypted local vault. There is no key escrow, no server-side key, no recovery server. Identity is an Ed25519 keypair. Session keys use X25519 with a symmetric ratchet for forward secrecy. PQ keys (ML-KEM-768) are layered in hybrid mode. Key revocation and rotation are in progress for v0.2.0.",
  },
  {
    q: "What is delay-tolerant delivery?",
    a: "If the recipient is offline, GhostWire stores the encrypted message in their mailbox on the mesh. When they reconnect, the message is delivered automatically via PRoPHET epidemic routing. This is borrowed from delay-tolerant networking (DTN) research and works even with intermittent connectivity.",
  },
  {
    q: "Is GhostWire anonymous?",
    a: "GhostWire provides strong privacy but not full anonymity against a global passive adversary. Sphinx onion routing hides source/destination from intermediate hops. Cover traffic masks timing. For stronger anonymity, obfs4, Shadowsocks, WebSocket, and TLS pluggable transports scramble packet shape; a Tor transport is in development. See the Transparency section for the full honesty disclosure.",
  },
  {
    q: "Has GhostWire been audited?",
    a: "Not yet. As of v0.2.0, GhostWire has had internal design reviews and threat modeling, but no independent third-party security audit. We use audited primitives (XChaCha20-Poly1305, libp2p, sphinx-rs, RustCrypto crates) where possible and have open-sourced everything under AGPL-3.0 for community review. We are seeking funding for a professional audit — contact admin@ghostwire.cc if you can help.",
  },
  {
    q: "How do I get started?",
    a: "Clone the repo, run `cargo run --release` inside the `ghostwire/` directory. The node starts on http://127.0.0.1:8080. For the terminal UI, run `cargo run --release -- tui`. See the Quick Start section above for full instructions.",
  },
];

export function FaqSection() {
  return (
    <section id="faq" className="py-24 border-t border-[var(--gw-border)]">
      <div className="max-w-[1160px] mx-auto px-6 sm:px-8">
        <div className="gw-section-label mb-3">{"// faq"}</div>
        <h2 className="font-display text-3xl sm:text-5xl font-bold mb-6 text-[var(--gw-white)]">
          FAQ: P2P Messaging, NAT Traversal, Key Management, Offline Behavior
        </h2>
        <p className="text-[16px] text-[var(--gw-white-dim)] leading-relaxed max-w-3xl mb-10">
          Common questions about how GhostWire works, what it protects, and how to get started.
        </p>

        <div className="max-w-3xl">
          <Accordion type="single" collapsible className="w-full">
            {FAQ_ITEMS.map((item, idx) => (
              <AccordionItem
                key={item.q}
                value={`faq-${idx}`}
                className="border-[var(--gw-border)]"
              >
                <AccordionTrigger
                  className="text-[var(--gw-white)] hover:text-[var(--gw-green)] hover:no-underline font-display text-[15px] font-semibold py-5"
                >
                  {item.q}
                </AccordionTrigger>
                <AccordionContent className="text-[14px] text-[var(--gw-white-dim)] leading-relaxed">
                  {item.a}
                </AccordionContent>
              </AccordionItem>
            ))}
          </Accordion>
        </div>
      </div>
    </section>
  );
}
