import { NextResponse } from "next/server";

const REPO = "PHANTOMOJO/GhostWire-secure-mesh-communication";
const CODEBERG_API = "https://codeberg.org/api/v1";
const CACHE_TTL = 60 * 15; // 15 minutes

let cache: { data: unknown; ts: number } | null = null;

interface Commit {
  sha: string;
  message: string;
  date: string;
  author: string;
  authorAvatar?: string;
  url: string;
}

interface RepoStats {
  stars: number;
  forks: number;
  openIssues: number;
  subscribers: number;
  watchers: number;
  contributors: number;
  commits: number;
  defaultBranch: string;
  createdAt: string;
  pushedAt: string;
  description: string;
  topics: string[];
  license: string | null;
  latestCommits: Commit[];
  fetchedAt: string;
}

async function fetchJson(url: string): Promise<unknown> {
  const res = await fetch(url, {
    headers: { Accept: "application/json", "User-Agent": "ghostwire-website" },
  });
  if (!res.ok) throw new Error(`Codeberg API ${res.status} for ${url}`);
  return res.json();
}

export async function GET() {
  if (cache && Date.now() - cache.ts < CACHE_TTL * 1000) {
    return NextResponse.json(cache.data, {
      headers: { "Cache-Control": `public, s-maxage=${CACHE_TTL}, stale-while-revalidate=60` },
    });
  }

  try {
    const [repoData, commitsData] = await Promise.all([
      fetchJson(`${CODEBERG_API}/repos/${REPO}`) as Promise<Record<string, unknown>>,
      fetchJson(`${CODEBERG_API}/repos/${REPO}/commits?limit=8`) as Promise<Record<string, unknown>[]>,
    ]);

    const latestCommits: Commit[] = commitsData.map((c) => ({
      sha: String(c.sha).slice(0, 7),
      message: String((c.commit as Record<string, unknown>).message).split("\n")[0],
      date: String(((c.commit as Record<string, unknown>).author as Record<string, unknown>).date),
      author: String(
        (c.committer as Record<string, unknown> | null)?.login ||
        ((c.commit as Record<string, unknown>).author as Record<string, unknown>).name ||
        "unknown"
      ),
      authorAvatar: undefined,
      url: `https://codeberg.org/${REPO}/commit/${c.sha}`,
    }));

    // Derive unique contributor count from recent commits
    const uniqueAuthors = new Set(latestCommits.map((c) => c.author));

    const stats: RepoStats = {
      stars: Number(repoData.stars_count) || 0,
      forks: Number(repoData.forks_count) || 0,
      openIssues: Number(repoData.open_issues_count) || 0,
      subscribers: 0, // Forgejo doesn't track watchers separately
      watchers: Number(repoData.watchers_count) || 0,
      contributors: Math.max(uniqueAuthors.size, 1), // At least 1 (the owner)
      commits: Number(repoData.size) || 0, // Approximate from repo size
      defaultBranch: String(repoData.default_branch || "main"),
      createdAt: String(repoData.created_at),
      pushedAt: String(repoData.updated_at),
      description: String(repoData.description || ""),
      topics: (repoData.topics as string[]) || [],
      license: (repoData.license as Record<string, unknown> | null)?.name as string || null,
      latestCommits,
      fetchedAt: new Date().toISOString(),
    };

    cache = { data: stats, ts: Date.now() };

    return NextResponse.json(stats, {
      headers: { "Cache-Control": `public, s-maxage=${CACHE_TTL}, stale-while-revalidate=60` },
    });
  } catch (err) {
    console.error("Codeberg stats error:", err);
    return NextResponse.json(
      { error: "Failed to fetch Codeberg stats", fetchedAt: new Date().toISOString() },
      { status: 502 }
    );
  }
}
