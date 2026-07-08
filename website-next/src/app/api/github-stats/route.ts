import { NextResponse } from "next/server";

const REPO = "Phantomojo/GhostWire-secure-mesh-communication";
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

interface GithubStats {
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

async function fetchJson(url: string, token?: string): Promise<unknown> {
  const headers: Record<string, string> = {
    Accept: "application/vnd.github+json",
    "X-GitHub-Api-Version": "2022-11-28",
    "User-Agent": "ghostwire-website",
  };
  if (token) headers.Authorization = `Bearer ${token}`;
  const res = await fetch(url, { headers });
  if (!res.ok) throw new Error(`GitHub API ${res.status} for ${url}`);
  return res.json();
}

export async function GET() {
  if (cache && Date.now() - cache.ts < CACHE_TTL * 1000) {
    return NextResponse.json(cache.data, {
      headers: { "Cache-Control": `public, s-maxage=${CACHE_TTL}, stale-while-revalidate=60` },
    });
  }

  try {
    const token = process.env.GITHUB_TOKEN || process.env.GH_TOKEN;

    const [repoData, contributorsData, commitsData] = await Promise.all([
      fetchJson(`https://api.github.com/repos/${REPO}`, token) as Promise<Record<string, unknown>>,
      fetchJson(`https://api.github.com/repos/${REPO}/contributors?per_page=100`, token) as Promise<unknown[]>,
      fetchJson(`https://api.github.com/repos/${REPO}/commits?per_page=8`, token) as Promise<Record<string, unknown>[]>,
    ]);

    const latestCommits: Commit[] = commitsData.map((c) => ({
      sha: String(c.sha).slice(0, 7),
      message: String((c.commit as Record<string, unknown>).message).split("\n")[0],
      date: String(((c.commit as Record<string, unknown>).author as Record<string, unknown>).date),
      author: String((c.author as Record<string, unknown> | null)?.login || ((c.commit as Record<string, unknown>).author as Record<string, unknown>).name || "unknown"),
      authorAvatar: (c.author as Record<string, unknown> | null)?.avatar_url as string | undefined,
      url: String(c.html_url),
    }));

    const stats: GithubStats = {
      stars: Number(repoData.stargazers_count) || 0,
      forks: Number(repoData.forks_count) || 0,
      openIssues: Number(repoData.open_issues_count) || 0,
      subscribers: Number(repoData.subscribers_count) || 0,
      watchers: Number(repoData.watchers_count) || 0,
      contributors: contributorsData.length,
      commits: contributorsData.reduce((sum, c) => sum + Number((c as Record<string, unknown>).contributions || 0), 0),
      defaultBranch: String(repoData.default_branch || "main"),
      createdAt: String(repoData.created_at),
      pushedAt: String(repoData.pushed_at),
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
    console.error("GitHub stats error:", err);
    return NextResponse.json(
      { error: "Failed to fetch GitHub stats", fetchedAt: new Date().toISOString() },
      { status: 502 }
    );
  }
}
