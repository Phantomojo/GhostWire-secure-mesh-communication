'use client'

import { useState } from "react";
import { FeatureModal } from "@/components/site/feature-modal";

interface Term {
  term: string;
  category: string;
  short: string;
  definition: string;
}

const TERMS: Term[] = [
  // Mesh
  {
    term: "P2P (Peer-to-Peer)",
    category: "Mesh",
    short: "Direct device-to-device networking. No central server.",
    definition:
      "A peer-to-peer network is one where every node is equal — each device acts as both a client and a server. There is no central authority, no login server, no relay. GhostWire uses libp2p as its P2P networking stack, the same library powering IPFS, Ethereum 2, and Filecoin. Messages hop directly from device to device across the mesh.",
  },
  {
    term: "Mesh Network",
    category: "Mesh",
    short: "A network where every node relays for every other node.",
    definition:
      "In a mesh network, each node relays traffic for its neighbors. There's no single path that can fail — if one node drops, traffic routes around it. Mesh networks are inherently resilient: they survive infrastructure failure, censorship, and partition. GhostWire forms ad-hoc meshes using libp2p GossipSub for messaging and Kademlia DHT for peer discovery.",
  },
  {
    term: "GossipSub",
    category: "Mesh",
    short: "libp2p's topic-based pub/sub messaging protocol.",
    definition:
      "Gossipsub is a pub/sub protocol used by libp2p. Nodes subscribe to topics and receive messages published to those topics. It uses a mesh of connected peers who gossip about which messages they've seen, enabling efficient broadcast without flooding every link. GhostWire uses GossipSub for mesh-wide events and Ghost Channels.",
  },
  {
    term: "Kademlia DHT",
    category: "Mesh",
    short: "Distributed hash table for O(log N) peer lookup.",
    definition:
      "Kademlia is a distributed hash table (DHT) algorithm. It lets nodes find each other and find content across a massive network in O(log N) hops — for a million nodes, that's ~20 hops. GhostWire uses Kademlia for peer discovery and content addressing (BlockStore/Bitswap). It's the same DHT used by IPFS and Mainline BitTorrent.",
  },

  // Crypto
  {
    term: "XChaCha20-Poly1305",
    category: "Crypto",
    short: "Primary AEAD cipher. 256-bit key, 192-bit nonce.",
    definition:
      "XChaCha20-Poly1305 is an authenticated encryption with associated data (AEAD) construction. It's the extended-nonce variant of ChaCha20-Poly1305 (RFC 8439). The 24-byte (192-bit) nonce makes random nonce generation safe even at high message volumes. It's constant-time (no timing side channels), hardware-friendly, and used in WireGuard, age, and Tailscale. GhostWire uses it as the primary AEAD for all message encryption.",
  },
  {
    term: "X25519",
    category: "Crypto",
    short: "Elliptic-curve Diffie-Hellman key exchange (RFC 7748).",
    definition:
      "X25519 is a Diffie-Hellman key exchange using Curve25519 in Montgomery form. Two parties can derive a shared secret over an insecure channel without ever transmitting the secret itself. GhostWire uses X25519 for ephemeral session key agreement and for Sphinx hop keys. X25519 keys are interoperable with Ed25519 signing keys via the edwards_to_montgomery conversion.",
  },
  {
    term: "ML-KEM-768",
    category: "Crypto",
    short: "NIST FIPS 203 post-quantum key encapsulation.",
    definition:
      "ML-KEM-768 (formerly Kyber-768) is the NIST-standardized post-quantum key encapsulation mechanism (FIPS 203). It resists attacks by future quantum computers. GhostWire layers ML-KEM-768 alongside X25519 in hybrid mode — a break in either primitive alone does not break the session. This defends against harvest-now-decrypt-later attacks where adversaries record encrypted traffic today for future quantum decryption.",
  },
  {
    term: "Sphinx",
    category: "Crypto",
    short: "Onion routing protocol with SURBs. Same as Nym and Lightning.",
    definition:
      "Sphinx is an onion-routing protocol that wraps each packet in layers of encryption. Each intermediate hop peels one layer and sees only the next hop — never both sender and recipient. SURBs (Single-Use Reply Blocks) let peers reply without learning the sender's identity. Sphinx is the same protocol used by Nym (privacy network) and the Lightning Network (Bitcoin L2). GhostWire's implementation, sphinx-rs, is published on crates.io.",
  },

  // Routing
  {
    term: "DTN (Delay-Tolerant Networking)",
    category: "Routing",
    short: "Networking that works when peers are intermittently offline.",
    definition:
      "Delay-Tolerant Networking (DTN) is a networking architecture designed for environments where end-to-end connectivity is not continuously available. Messages are stored locally and forwarded when a connection opportunity arises — store, carry, forward. GhostWire implements DTN with PRoPHET epidemic routing and Spray-and-Wait, so messages queue when peers are offline and deliver when the mesh reconnects.",
  },
  {
    term: "PRoPHET",
    category: "Routing",
    short: "Probabilistic routing using encounter history. The L4 fallback.",
    definition:
      "PRoPHET (Probabilistic Routing Protocol using History of Encounters and Transitivity) is a DTN routing algorithm. It estimates the probability that a node will encounter a destination based on past encounters and transitive relationships, then forwards bundles accordingly. GhostWire uses PRoPHET as the L4 always-on fallback in its 4-layer AI routing stack — if all ML layers are unavailable, PRoPHET keeps the mesh working (degraded but never dead).",
  },
  {
    term: "LightGBM",
    category: "Routing",
    short: "Gradient-boosted decision tree model. The L1 fast-path router.",
    definition:
      "LightGBM is a gradient-boosted decision tree (GBDT) machine learning framework. GhostWire trains a LightGBM classifier on mesh routing telemetry, exports it to ONNX format, and runs it in the L1 fast-path layer. It classifies routing decisions in 76.7 microseconds with AUC 1.0 on held-out test data — fast enough to run on every routing decision at line rate.",
  },
  {
    term: "GNN (Graph Neural Network)",
    category: "Routing",
    short: "Neural network over graph-structured data. The L3 optimizer.",
    definition:
      "A Graph Neural Network (GNN) is a neural network architecture that operates on graph-structured data. GhostWire trains a GNN on the GuifiSants community mesh dataset (7,931 samples, 63 nodes, 31 days of real topology) to learn optimal routing paths. The GNN runs every 5 minutes as the L3 graph optimization layer, recomputing paths across the mesh topology graph.",
  },

  // Identity
  {
    term: "TOFU (Trust On First Use)",
    category: "Identity",
    short: "Pin a peer's key on first connection. Detect substitution.",
    definition:
      "TOFU (Trust On First Use) is a key-verification model. The first time you connect to a peer, their public key is recorded (pinned). Any future change to that key triggers a warning — defending against man-in-the-middle key-substitution attacks. It's the same model SSH uses for host keys. GhostWire uses TOFU as the foundation of its Web of Trust, augmented by signed attestations from other peers.",
  },
  {
    term: "SURB (Single-Use Reply Block)",
    category: "Identity",
    short: "Anonymous reply token. Lets peers reply without learning the sender.",
    definition:
      "A SURB (Single-Use Reply Block) is a cryptographic token included in a Sphinx onion-routed message. It lets the recipient send a reply back to the original sender without learning the sender's identity — the reply routes back through the onion path. SURBs are what make Sphinx sender-anonymous. Each SURB can be used exactly once (replay protection).",
  },
  {
    term: "Shamir's Secret Sharing",
    category: "Identity",
    short: "Split a key into N shares. Threshold T needed to reconstruct.",
    definition:
      "Shamir's Secret Sharing is a cryptographic scheme that splits a secret (like an identity key) into N shares distributed to trusted parties. Any T of them (the threshold) can reconstruct the secret, but no fewer. GhostWire uses it for identity recovery: split your Ed25519 key across N trusted custodians, and recover your identity even if you lose your device — without any single custodian ever holding the full key.",
  },
  {
    term: "Proof-of-Work (PoW)",
    category: "Identity",
    short: "Computational puzzle for Sybil-resistant node admission.",
    definition:
      "Proof-of-Work is a computational puzzle that requires resources to solve. GhostWire requires new nodes to solve a tunable PoW puzzle to join the mesh. This is cheap for a single honest node but expensive for an adversary trying to spin up thousands of fake identities (a Sybil attack). Combined with the TrustStore reputation system, PoW makes sustained Sybil attacks economically infeasible on commodity hardware.",
  },
];

const CATEGORIES = ["All", "Mesh", "Crypto", "Routing", "Identity"] as const;

export function GlossarySection() {
  const [active, setActive] = useState<Term | null>(null);
  const [filter, setFilter] = useState<(typeof CATEGORIES)[number]>("All");

  const visibleTerms =
    filter === "All"
      ? TERMS
      : TERMS.filter((t) => t.category === filter);

  return (
    <section
      id="glossary"
      className="py-24 border-t border-[var(--gw-border)]"
    >
      <div className="max-w-[1160px] mx-auto px-6 sm:px-8">
        <div className="gw-section-label mb-3">{"// glossary"}</div>
        <h2 className="font-display text-3xl sm:text-4xl font-bold text-[var(--gw-white)] mb-3">
          Glossary — P2P, mesh, and cryptography terms
        </h2>
        <p className="text-[15px] text-[var(--gw-white-dim)] max-w-2xl mb-8">
          Key terms used throughout GhostWire. Click any card for a deeper
          definition.
        </p>

        {/* Category filter */}
        <div
          className="flex flex-wrap gap-2 mb-8"
          role="tablist"
          aria-label="Filter glossary by category"
        >
          {CATEGORIES.map((cat) => {
            const isActive = filter === cat;
            return (
              <button
                key={cat}
                type="button"
                role="tab"
                aria-selected={isActive}
                onClick={() => setFilter(cat)}
                className={
                  "font-mono-gw text-[12px] tracking-[0.04em] uppercase px-3 py-1.5 rounded-full border transition-colors cursor-pointer " +
                  (isActive
                    ? "bg-[var(--gw-green)] text-[var(--gw-bg)] border-[var(--gw-green)]"
                    : "bg-transparent text-[var(--gw-white-dim)] border-[var(--gw-border-2)] hover:text-[var(--gw-white)] hover:border-[var(--gw-white-dim)]")
                }
              >
                {cat}
              </button>
            );
          })}
        </div>

        {/* Term cards */}
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
          {visibleTerms.map((term) => (
            <button
              key={term.term}
              type="button"
              onClick={() => setActive(term)}
              className="text-left bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-5 gw-card-hover focus:outline-none focus:border-[var(--gw-green)]"
            >
              <div className="flex items-center justify-between mb-2 gap-2">
                <h3 className="font-display text-base font-semibold text-[var(--gw-white)]">
                  {term.term}
                </h3>
                <span className="font-mono-gw text-[10px] tracking-[0.06em] uppercase text-[var(--gw-green)] px-2 py-0.5 border border-[var(--gw-border-2)] rounded shrink-0">
                  {term.category}
                </span>
              </div>
              <p className="text-[13px] text-[var(--gw-white-dim)] leading-relaxed mb-3">
                {term.short}
              </p>
              <span className="font-mono-gw text-[12px] text-[var(--gw-green)]">
                Click for definition →
              </span>
            </button>
          ))}
        </div>
      </div>

      <FeatureModal
        open={active !== null}
        onClose={() => setActive(null)}
        tag={active?.category ?? ""}
        title={active?.term ?? ""}
      >
        <p className="text-[15px] text-[var(--gw-white-dim)] leading-relaxed">
          {active?.definition}
        </p>
      </FeatureModal>
    </section>
  );
}
