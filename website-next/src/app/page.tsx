import { SiteNav } from "@/components/site/site-nav";
import { SiteFooter } from "@/components/site/site-footer";
import { HeroSection } from "@/components/site/sections/hero-section";
import { ProblemSection } from "@/components/site/sections/problem-section";
import { ForWhoSection } from "@/components/site/sections/for-who-section";
import { HowItWorksSection } from "@/components/site/sections/how-it-works-section";
import { FeaturesSection } from "@/components/site/sections/features-section";
import { ManifestoSection } from "@/components/site/sections/manifesto-section";
import { TransparencySection } from "@/components/site/sections/transparency-section";
import { QuickstartSection } from "@/components/site/sections/quickstart-section";
import { ArchitectureSection } from "@/components/site/sections/architecture-section";
import { SecuritySection } from "@/components/site/sections/security-section";
import { RoadmapSection } from "@/components/site/sections/roadmap-section";
import { ContributorsSection } from "@/components/site/sections/contributors-section";
import { FaqSection } from "@/components/site/sections/faq-section";
import { AboutSection } from "@/components/site/sections/about-section";
import { CommunitySection } from "@/components/site/sections/community-section";
import { StatsSection } from "@/components/site/sections/stats-section";
import { GlossarySection } from "@/components/site/sections/glossary-section";
import { DocsHubSection } from "@/components/site/sections/docs-hub-section";
import { PressSection } from "@/components/site/sections/press-section";
import { BlogSection } from "@/components/site/sections/blog-section";

const jsonLd = {
  "@context": "https://schema.org",
  "@type": "SoftwareApplication",
  name: "GhostWire",
  applicationCategory: "CommunicationApplication",
  operatingSystem: "Linux, macOS, Windows, Android",
  description:
    "GhostWire is an open protocol for resilient peer-to-peer mesh communication. A decentralized networking stack built in Rust — encrypted, delay-tolerant, post-quantum ready, and AI-routed. It provides the cryptographic, routing, and transport layers that developers build communication tools on top of.",
  url: "https://ghostwire.cc/",
  downloadUrl:
    "https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication",
  softwareVersion: "0.2.0",
  license: "https://www.gnu.org/licenses/agpl-3.0.html",
  programmingLanguage: "Rust",
  offers: {
    "@type": "Offer",
    price: "0",
    priceCurrency: "USD",
  },
  featureList: [
    "XChaCha20-Poly1305 E2E encryption with X25519 key exchange and forward secrecy ratchet",
    "Post-quantum ML-KEM-768 hybrid key exchange",
    "Sphinx onion routing with SURBs (anonymous routing)",
    "4-layer AI routing: LightGBM → Gemma 4 → GNN → PRoPHET",
    "Delay-tolerant networking (PRoPHET epidemic routing)",
    "Multi-transport: Stealth TCP, WebRTC, Bluetooth LE, LoRa, obfs4, Reticulum",
    "Coercion resistance: duress PIN, panic wipe, dead man's switch",
    "Web of Trust with TOFU key pinning",
    "Protocol bridges to Matrix, Meshtastic, Briar, Reticulum",
  ],
  author: {
    "@type": "Organization",
    name: "Team GhostWire",
    url: "https://ghostwire.cc/",
  },
};

const faqJsonLd = {
  "@context": "https://schema.org",
  "@type": "FAQPage",
  mainEntity: [
    {
      "@type": "Question",
      name: "What is GhostWire?",
      acceptedAnswer: {
        "@type": "Answer",
        text: "GhostWire is a decentralized, censorship-resistant mesh communication platform built in Rust. It enables secure peer-to-peer messaging and file sharing without central servers, designed for connectivity-constrained communities, disaster response, and privacy-preserving communication.",
      },
    },
    {
      "@type": "Question",
      name: "How does P2P messaging work without a server?",
      acceptedAnswer: {
        "@type": "Answer",
        text: "GhostWire uses libp2p for peer discovery and direct device-to-device connections. Messages hop node-to-node across the mesh. Each device is both a client and a relay. There is no central server to seize, subpoena, or shut down.",
      },
    },
    {
      "@type": "Question",
      name: "What encryption does GhostWire use?",
      acceptedAnswer: {
        "@type": "Answer",
        text: "A multi-cipher stack: vodozemac (Olm/Megolm) for session encryption, ChaCha20-Poly1305 for sealed delivery, AES-256-GCM for Matrix bridge compatibility. Key exchange uses X25519 with Ed25519 signatures. Post-quantum ML-KEM-768 is layered in hybrid mode.",
      },
    },
    {
      "@type": "Question",
      name: "Does GhostWire work offline?",
      acceptedAnswer: {
        "@type": "Answer",
        text: "Yes. Messages are delay-tolerant — they queue locally when peers are offline and deliver when the mesh reconnects. GhostWire also runs over local-only transports (TCP/IP mesh, Reticulum) so it works during internet shutdowns. WiFi Direct, Bluetooth LE, and LoRa transports are in development for true off-grid mesh.",
      },
    },
  ],
};

const orgJsonLd = {
  "@context": "https://schema.org",
  "@type": "Organization",
  name: "GhostWire",
  url: "https://www.ghostwire.cc/",
  logo: "https://www.ghostwire.cc/ghostwire-logo.png",
  description:
    "An open protocol for resilient peer-to-peer mesh communication. Decentralized networking stack built in Rust.",
  sameAs: [
    "https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication",
    "https://github.com/Phantomojo/GhostWire-secure-mesh-communication",
  ],
  contactPoint: [
    {
      "@type": "ContactPoint",
      contactType: "technical support",
      email: "admin@ghostwire.cc",
      url: "https://www.ghostwire.cc/",
    },
    {
      "@type": "ContactPoint",
      contactType: "security",
      email: "security@ghostwire.cc",
      url: "https://www.ghostwire.cc/#security",
    },
  ],
};

const websiteJsonLd = {
  "@context": "https://schema.org",
  "@type": "WebSite",
  name: "GhostWire",
  url: "https://www.ghostwire.cc/",
  description:
    "An open protocol for resilient peer-to-peer mesh communication.",
  publisher: { "@type": "Organization", name: "GhostWire" },
};

const softwareSourceCodeJsonLd = {
  "@context": "https://schema.org",
  "@type": "SoftwareSourceCode",
  name: "GhostWire",
  description:
    "P2P mesh communication protocol with post-quantum encryption, built in Rust",
  programmingLanguage: "Rust",
  codeRepository:
    "https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication",
  license: "https://www.gnu.org/licenses/agpl-3.0.html",
  runtimePlatform: "Linux, macOS, Windows, Android",
  softwareVersion: "0.2.0",
};

export default function Home() {
  return (
    <div className="min-h-screen flex flex-col bg-[var(--gw-bg)]">
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{ __html: JSON.stringify(jsonLd) }}
      />
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{ __html: JSON.stringify(faqJsonLd) }}
      />
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{ __html: JSON.stringify(orgJsonLd) }}
      />
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{ __html: JSON.stringify(websiteJsonLd) }}
      />
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{
          __html: JSON.stringify(softwareSourceCodeJsonLd),
        }}
      />
      <SiteNav />
      <main className="flex-1">
        <HeroSection />
        <ProblemSection />
        <ForWhoSection />
        <HowItWorksSection />
        <FeaturesSection />
        <ManifestoSection />
        <AboutSection />
        <TransparencySection />
        <QuickstartSection />
        <ArchitectureSection />
        <SecuritySection />
        <RoadmapSection />
        <ContributorsSection />
        <GlossarySection />
        <FaqSection />
        <DocsHubSection />
        <CommunitySection />
        <StatsSection />
        <BlogSection />
        <PressSection />
      </main>
      <SiteFooter />
    </div>
  );
}
