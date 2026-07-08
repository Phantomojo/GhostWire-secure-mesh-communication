'use client'

import { useEffect, useState } from "react";

interface Commit {
  sha: string;
  message: string;
  date: string;
  author: string;
  authorAvatar?: string;
  url: string;
}

interface Stats {
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

const REPO_URL =
  "https://github.com/Phantomojo/GhostWire-secure-mesh-communication";

/**
 * Returns a short "Xh ago" / "Xd ago" / "Xw ago" style relative string.
 * Handles invalid and future dates gracefully.
 */
function timeAgo(iso: string): string {
  if (!iso) return "—";
  const then = new Date(iso).getTime();
  if (Number.isNaN(then)) return "—";
  const now = Date.now();
  const diffMs = now - then;
  // Future date — just show "just now" instead of negative.
  if (diffMs < 0) return "just now";
  const sec = Math.floor(diffMs / 1000);
  if (sec < 60) return "just now";
  const min = Math.floor(sec / 60);
  if (min < 60) return `${min}m ago`;
  const hr = Math.floor(min / 60);
  if (hr < 24) return `${hr}h ago`;
  const day = Math.floor(hr / 24);
  if (day < 7) return `${day}d ago`;
  const wk = Math.floor(day / 7);
  if (wk < 5) return `${wk}w ago`;
  const mo = Math.floor(day / 30);
  if (mo < 12) return `${mo}mo ago`;
  const yr = Math.floor(day / 365);
  return `${yr}y ago`;
}

function StatCard({
  value,
  label,
  sub,
}: {
  value: string | number;
  label: string;
  sub: string;
}) {
  return (
    <div className="bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-6 gw-card-hover">
      <div className="font-mono-gw text-3xl font-bold text-[var(--gw-green)] mb-2">
        {value}
      </div>
      <div className="font-display text-base font-semibold text-[var(--gw-white)] mb-1">
        {label}
      </div>
      <p className="text-[13px] text-[var(--gw-white-dim)]">{sub}</p>
    </div>
  );
}

function SkeletonCard() {
  return (
    <div className="bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-6">
      <div className="h-9 w-20 animate-pulse bg-[var(--gw-bg)] rounded mb-3" />
      <div className="h-4 w-28 animate-pulse bg-[var(--gw-bg)] rounded mb-2" />
      <div className="h-3 w-40 animate-pulse bg-[var(--gw-bg)] rounded" />
    </div>
  );
}

export function StatsSection() {
  const [loading, setLoading] = useState(true);
  const [stats, setStats] = useState<Stats | null>(null);
  const [error, setError] = useState(false);

  useEffect(() => {
    let cancelled = false;
    fetch("/api/github-stats")
      .then(async (res) => {
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return (await res.json()) as Stats;
      })
      .then((data) => {
        if (cancelled) return;
        setStats(data);
        setError(false);
      })
      .catch(() => {
        if (cancelled) return;
        setError(true);
      })
      .finally(() => {
        if (!cancelled) setLoading(false);
      });
    return () => {
      cancelled = true;
    };
  }, []);

  return (
    <section
      id="stats"
      className="py-24 border-t border-[var(--gw-border)]"
    >
      <div className="max-w-[1160px] mx-auto px-6 sm:px-8">
        <div className="gw-section-label mb-3">{"// live stats"}</div>
        <h2 className="font-display text-3xl sm:text-4xl font-bold text-[var(--gw-white)] mb-3">
          Live from GitHub
        </h2>
        <p className="text-[15px] text-[var(--gw-white-dim)] max-w-2xl mb-10">
          Real-time project health, fetched from the GhostWire repository.
          Auto-updates every 15 minutes.
        </p>

        {loading && (
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
            <SkeletonCard />
            <SkeletonCard />
            <SkeletonCard />
            <SkeletonCard />
          </div>
        )}

        {error && (
          <div className="bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-6">
            <div className="font-display text-base font-semibold text-[var(--gw-white)] mb-1">
              Stats temporarily unavailable
            </div>
            <p className="text-[13px] text-[var(--gw-white-dim)] mb-4">
              We couldn&apos;t reach the GitHub API right now. You can still view
              the repository directly.
            </p>
            <a
              href={REPO_URL}
              target="_blank"
              rel="noopener noreferrer"
              className="gw-btn-outline"
            >
              View on GitHub →
            </a>
          </div>
        )}

        {!loading && !error && stats && (
          <>
            {/* Part A — Stat grid */}
            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
              <StatCard
                value={stats.stars}
                label="GitHub stars"
                sub="Star the repo to support the project."
              />
              <StatCard
                value={stats.commits}
                label="commits on main"
                sub={`Last push ${timeAgo(stats.pushedAt)}`}
              />
              <StatCard
                value={stats.contributors}
                label="contributors"
                sub="Building the protocol in the open."
              />
              <StatCard
                value="v0.2.0"
                label="current version"
                sub="Active development toward v1.0 stable."
              />
            </div>

            {/* Part B — Latest activity + repo badges */}
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 mt-10">
              {/* Left column: latest commits */}
              <div className="lg:col-span-2">
                <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-4">
                  Latest activity
                </h3>
                <div className="space-y-2 max-h-[400px] overflow-y-auto no-scrollbar">
                  {stats.latestCommits.slice(0, 8).map((commit) => (
                    <a
                      key={commit.sha}
                      href={commit.url}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="flex items-start gap-3 p-3 border border-[var(--gw-border)] rounded hover:bg-[var(--gw-bg-2)] transition-colors no-underline group"
                    >
                      {commit.authorAvatar && (
                        <img
                          src={commit.authorAvatar}
                          alt={commit.author}
                          width={28}
                          height={28}
                          className="rounded-full shrink-0"
                        />
                      )}
                      <div className="flex-1 min-w-0">
                        <div className="flex items-center gap-2 mb-0.5">
                          <span className="font-mono-gw text-[11px] text-[var(--gw-green)]">
                            {commit.sha}
                          </span>
                          <span className="text-[11px] text-[var(--gw-white-faint)]">
                            {timeAgo(commit.date)}
                          </span>
                        </div>
                        <p className="text-[13px] text-[var(--gw-white)] truncate group-hover:text-[var(--gw-green)] transition-colors">
                          {commit.message}
                        </p>
                        <span className="text-[11px] text-[var(--gw-white-faint)]">
                          by {commit.author}
                        </span>
                      </div>
                    </a>
                  ))}
                </div>
              </div>

              {/* Right column: repo badges */}
              <div className="bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-6">
                <h4 className="font-display text-base font-semibold text-[var(--gw-white)] mb-4">
                  Repository
                </h4>
                <dl className="space-y-2.5 text-[13px]">
                  <div className="flex justify-between">
                    <dt className="text-[var(--gw-white-faint)]">License</dt>
                    <dd className="text-[var(--gw-white)]">
                      {stats.license || "AGPL-3.0"}
                    </dd>
                  </div>
                  <div className="flex justify-between">
                    <dt className="text-[var(--gw-white-faint)]">
                      Default branch
                    </dt>
                    <dd className="font-mono-gw text-[var(--gw-green)]">
                      {stats.defaultBranch}
                    </dd>
                  </div>
                  <div className="flex justify-between">
                    <dt className="text-[var(--gw-white-faint)]">Created</dt>
                    <dd className="text-[var(--gw-white)]">
                      {new Date(stats.createdAt).toLocaleDateString("en", {
                        year: "numeric",
                        month: "short",
                      })}
                    </dd>
                  </div>
                  <div className="flex justify-between">
                    <dt className="text-[var(--gw-white-faint)]">Forks</dt>
                    <dd className="text-[var(--gw-white)]">{stats.forks}</dd>
                  </div>
                  <div className="flex justify-between">
                    <dt className="text-[var(--gw-white-faint)]">
                      Open issues
                    </dt>
                    <dd className="text-[var(--gw-white)]">
                      {stats.openIssues}
                    </dd>
                  </div>
                  <div className="flex justify-between">
                    <dt className="text-[var(--gw-white-faint)]">Watchers</dt>
                    <dd className="text-[var(--gw-white)]">{stats.watchers}</dd>
                  </div>
                </dl>

                <div className="mt-5 pt-5 border-t border-[var(--gw-border)]">
                  <div className="font-mono-gw text-[10px] tracking-[0.1em] uppercase text-[var(--gw-white-faint)] mb-2">
                    Topics
                  </div>
                  <div className="flex flex-wrap gap-1.5">
                    {stats.topics.map((t) => (
                      <span
                        key={t}
                        className="font-mono-gw text-[11px] px-2 py-0.5 border border-[var(--gw-border-2)] rounded text-[var(--gw-white-dim)]"
                      >
                        {t}
                      </span>
                    ))}
                  </div>
                </div>

                <a
                  href={REPO_URL}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="gw-btn-outline mt-5 w-full justify-center"
                >
                  View on GitHub →
                </a>
              </div>
            </div>

            {/* Last updated */}
            <div className="mt-8 font-mono-gw text-[11px] text-[var(--gw-white-faint)]">
              Last updated: {timeAgo(stats.fetchedAt)}
            </div>
          </>
        )}
      </div>
    </section>
  );
}
