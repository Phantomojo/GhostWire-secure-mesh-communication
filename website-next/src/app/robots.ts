import type { MetadataRoute } from "next";

export default function robots(): MetadataRoute.Robots {
  return {
    rules: [
      // Standard search crawlers — allow
      {
        userAgent: ["Googlebot", "Bingbot", "DuckDuckBot", "Slurp", "BraveBot", "PerplexityBot", "Timpibot", "GoogleOther"],
        allow: "/",
      },
      // AI training crawlers — BLOCKED (no training on this content)
      {
        userAgent: ["GPTBot", "ChatGPT-User", "CCBot", "anthropic-ai", "Claude-Web", "ClaudeBot", "Google-Extended", "Bytespider", "Amazonbot", "Meta-ExternalAgent", "FacebookBot", "Omgilibot", "YouBot", "Diffbot", "DataForSeoBot", "Applebot-Extended"],
        disallow: "/",
      },
      // Default — allow all other crawlers
      {
        userAgent: "*",
        allow: "/",
      },
    ],
    sitemap: "https://www.ghostwire.cc/sitemap.xml",
    // ponytail: Removed non-standard `host` directive — Google ignores it.
    // Use <meta name="host"> or Google Search Console if multi-domain needed.
  };
}
