import type { MetadataRoute } from "next";

export default function sitemap(): MetadataRoute.Sitemap {
  const base = "https://www.ghostwire.cc";
  const now = new Date().toISOString().split("T")[0];

  // Ponytail: Single-page app — hash URLs (#section) are NOT separate pages.
  // Google/Bing ignore fragments in sitemaps. Only the root URL matters for indexing.
  // Sections are discovered via internal anchor links, not sitemap entries.
  return [
    {
      url: `${base}/`,
      lastModified: now,
      changeFrequency: "weekly",
      priority: 1.0,
    },
  ];
}
