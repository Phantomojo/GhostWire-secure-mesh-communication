import { NextResponse } from "next/server";

const RSS_XML = `<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom" xmlns:content="http://purl.org/rss/1.0/modules/content/">
  <channel>
    <title>GhostWire</title>
    <link>https://www.ghostwire.cc</link>
    <description>P2P mesh communication protocol with post-quantum encryption. Open-source, built in Rust.</description>
    <language>en-us</language>
    <lastBuildDate>${new Date().toUTCString()}</lastBuildDate>
    <atom:link href="https://www.ghostwire.cc/feed.xml" rel="self" type="application/rss+xml"/>
    <item>
      <title>XChaCha20-Poly1305 in a Mesh Network: A Practical Deep Dive</title>
      <link>https://dev.to/phantomojo/xchacha20-poly1305-in-a-mesh-network-a-practical-deep-dive-44gj</link>
      <description>GhostWire uses XChaCha20-Poly1305 as its primary cipher. Here's why, and how it works in practice.</description>
      <pubDate>${new Date("2026-07-21").toUTCString()}</pubDate>
      <guid>https://dev.to/phantomojo/xchacha20-poly1305-in-a-mesh-network-a-practical-deep-dive-44gj</guid>
    </item>
    <item>
      <title>Why We Built GhostWire: Infrastructure for Human Dignity</title>
      <link>https://dev.to/ghostwire_ke/why-we-built-ghostwire-infrastructure-for-human-dignity-49h5</link>
      <description>A decentralized, censorship-resistant communication mesh built in Rust. No servers. No owners. Built for when the internet goes dark.</description>
      <pubDate>${new Date("2026-07-08").toUTCString()}</pubDate>
      <guid>https://dev.to/ghostwire_ke/why-we-built-ghostwire-infrastructure-for-human-dignity-49h5</guid>
    </item>
    <item>
      <title>GhostWire v0.2.0 Released</title>
      <link>https://www.ghostwire.cc</link>
      <description>Multi-cipher E2EE, post-quantum ML-KEM-768, Sphinx onion routing, 4-layer AI routing, and 9 transports.</description>
      <pubDate>${new Date("2026-07-01").toUTCString()}</pubDate>
      <guid>https://www.ghostwire.cc/#v0.2.0</guid>
    </item>
    <item>
      <title>I Published 4 Rust Crates from My Mesh Network Project</title>
      <link>https://dev.to/phantomojo/i-published-4-rust-crates-from-my-mesh-network-project-5bc2</link>
      <description>Split GhostWire into 4 reusable crates: sphinx-rs, hlc-rs, trust-store, ghostwire-dtn.</description>
      <pubDate>${new Date("2026-04-11").toUTCString()}</pubDate>
      <guid>https://dev.to/phantomojo/i-published-4-rust-crates-from-my-mesh-network-project-5bc2</guid>
    </item>
    <item>
      <title>Building a Decentralized Mesh Network in Rust — Lessons from the Global South</title>
      <link>https://dev.to/phantomojo/building-a-decentralized-mesh-network-in-rust-lessons-from-the-global-south-k44</link>
      <description>How we built GhostWire — a decentralized, censorship-resistant mesh communication platform from Nairobi, Kenya.</description>
      <pubDate>${new Date("2026-04-05").toUTCString()}</pubDate>
      <guid>https://dev.to/phantomojo/building-a-decentralized-mesh-network-in-rust-lessons-from-the-global-south-k44</guid>
    </item>
  </channel>
</rss>`;

export async function GET() {
  return new NextResponse(RSS_XML, {
    headers: {
      "Content-Type": "application/rss+xml; charset=utf-8",
      "Cache-Control": "public, max-age=3600, s-maxage=3600",
    },
  });
}
