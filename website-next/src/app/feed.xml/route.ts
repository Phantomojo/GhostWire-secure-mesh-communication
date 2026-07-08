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
      <title>GhostWire v0.2.0 Released</title>
      <link>https://www.ghostwire.cc</link>
      <description>Multi-cipher E2EE, post-quantum ML-KEM-768, Sphinx onion routing, 4-layer AI routing, and 9 transports.</description>
      <pubDate>${new Date("2026-07-01").toUTCString()}</pubDate>
      <guid>https://www.ghostwire.cc/#v0.2.0</guid>
    </item>
    <item>
      <title>sphinx-rs Published on crates.io</title>
      <link>https://crates.io/crates/sphinx-rs</link>
      <description>Sphinx onion routing with SURBs — published as a standalone Rust crate.</description>
      <pubDate>${new Date("2026-06-15").toUTCString()}</pubDate>
      <guid>https://www.ghostwire.cc/#sphinx-rs</guid>
    </item>
    <item>
      <title>AI Routing: LightGBM + Gemma 4 + GNN + PRoPHET</title>
      <link>https://www.ghostwire.cc/#architecture</link>
      <description>Four-layer AI routing system for optimal path selection across mesh networks.</description>
      <pubDate>${new Date("2026-06-01").toUTCString()}</pubDate>
      <guid>https://www.ghostwire.cc/#ai-routing</guid>
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
