"use client";

import { useEffect, useState } from "react";
import { createPortal } from "react-dom";

interface DevToArticle {
  id: number;
  title: string;
  description: string;
  published_at: string;
  cover_image: string | null;
  reading_time_minutes: number;
  tag_list: string[];
  url: string;
  slug: string;
  user: { name: string; username: string };
}

const USERNAME = "phantomojo";

function formatDate(iso: string) {
  return new Date(iso).toLocaleDateString("en-US", {
    year: "numeric",
    month: "short",
    day: "numeric",
  });
}

function ArticleModal({
  article,
  onClose,
}: {
  article: DevToArticle;
  onClose: () => void;
}) {
  const [html, setHtml] = useState("");

  useEffect(() => {
    fetch(`https://dev.to/api/articles/${USERNAME}/${article.slug}`)
      .then((r) => r.json())
      .then((d) => setHtml(d.body_html || ""))
      .catch(() => setHtml("<p>Failed to load article.</p>"));
  }, [article.slug]);

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") onClose();
    };
    document.addEventListener("keydown", onKey);
    document.body.style.overflow = "hidden";
    return () => {
      document.removeEventListener("keydown", onKey);
      document.body.style.overflow = "";
    };
  }, [onClose]);

  if (typeof document === "undefined") return null;

  return createPortal(
    <div
      className="fixed inset-0 z-[100] flex items-center justify-center p-4 sm:p-6"
      role="dialog"
      aria-modal="true"
    >
      <div
        className="absolute inset-0 bg-black/70 backdrop-blur-sm"
        onClick={onClose}
      />
      <div className="relative bg-[var(--gw-bg-2)] border border-[var(--gw-border-2)] rounded-lg max-w-3xl w-full max-h-[88vh] overflow-y-auto no-scrollbar shadow-2xl">
        <button
          onClick={onClose}
          aria-label="Close"
          className="sticky top-4 float-right mr-4 w-8 h-8 flex items-center justify-center text-[var(--gw-white-dim)] hover:text-[var(--gw-green)] text-2xl bg-[var(--gw-bg-2)] border border-[var(--gw-border-2)] rounded cursor-pointer z-10"
        >
          ×
        </button>
        {article.cover_image && (
          <img
            src={article.cover_image}
            alt=""
            className="w-full h-48 sm:h-64 object-cover rounded-t-lg"
          />
        )}
        <div className="p-7 sm:p-9">
          <span className="font-mono-gw text-[11px] tracking-[0.1em] uppercase text-[var(--gw-green)] inline-block px-2 py-1 border border-[var(--gw-border-2)] rounded mb-4">
            {article.tag_list[0] || "blog"}
          </span>
          <h2 className="font-display text-2xl sm:text-3xl font-bold text-[var(--gw-white)] mb-3">
            {article.title}
          </h2>
          <div className="flex items-center gap-3 text-[13px] text-[var(--gw-white-dim)] mb-6">
            <span>{formatDate(article.published_at)}</span>
            <span>·</span>
            <span>{article.reading_time_minutes} min read</span>
          </div>
          {html ? (
            <div
              className="prose prose-invert prose-headings:text-[var(--gw-white)] prose-p:text-[var(--gw-white-dim)] prose-a:text-[var(--gw-green)] prose-code:text-[var(--gw-green)] prose-pre:bg-[var(--gw-bg)] prose-pre:border prose-pre:border-[var(--gw-border)] max-w-none text-[15px] leading-relaxed"
              dangerouslySetInnerHTML={{ __html: html }}
            />
          ) : (
            <div className="space-y-3">
              <div className="h-4 bg-[var(--gw-bg)] rounded w-3/4 animate-pulse" />
              <div className="h-4 bg-[var(--gw-bg)] rounded w-1/2 animate-pulse" />
              <div className="h-4 bg-[var(--gw-bg)] rounded w-2/3 animate-pulse" />
            </div>
          )}
          <div className="mt-8 pt-6 border-t border-[var(--gw-border)]">
            <a
              href={article.url}
              target="_blank"
              rel="noopener noreferrer"
              className="gw-btn-primary text-sm"
            >
              Read on dev.to →
            </a>
          </div>
        </div>
      </div>
    </div>,
    document.body
  );
}

export function BlogSection() {
  const [articles, setArticles] = useState<DevToArticle[]>([]);
  const [selected, setSelected] = useState<DevToArticle | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetch(`https://dev.to/api/articles?username=${USERNAME}&per_page=6`)
      .then((r) => r.json())
      .then((data: DevToArticle[]) => {
        setArticles(data);
        setLoading(false);
      })
      .catch(() => setLoading(false));
  }, []);

  return (
    <section id="blog" className="py-24 border-t border-[var(--gw-border)]">
      <div className="max-w-7xl mx-auto px-6">
        <div className="gw-section-label mb-3">{"// blog"}</div>
        <h2 className="font-display text-3xl md:text-4xl font-bold text-[var(--gw-white)] mb-3">
          From the blog
        </h2>
        <p className="text-[15px] text-[var(--gw-white-dim)] max-w-2xl mb-12 leading-relaxed">
          Technical deep-dives, security research, and build logs from the
          GhostWire team.
        </p>

        {loading ? (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {[1, 2, 3].map((i) => (
              <div
                key={i}
                className="bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-6 animate-pulse"
              >
                <div className="h-4 bg-[var(--gw-bg)] rounded w-1/3 mb-4" />
                <div className="h-6 bg-[var(--gw-bg)] rounded w-3/4 mb-3" />
                <div className="h-4 bg-[var(--gw-bg)] rounded w-full mb-2" />
                <div className="h-4 bg-[var(--gw-bg)] rounded w-2/3" />
              </div>
            ))}
          </div>
        ) : articles.length === 0 ? (
          <div className="bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-8 text-center">
            <p className="text-[var(--gw-white-dim)]">
              No articles yet. Check back soon.
            </p>
            <a
              href={`https://dev.to/${USERNAME}`}
              target="_blank"
              rel="noopener noreferrer"
              className="gw-btn-outline mt-4 inline-block text-sm"
            >
              Follow on dev.to →
            </a>
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {articles.map((a) => (
              <button
                key={a.id}
                onClick={() => setSelected(a)}
                className="bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg overflow-hidden text-left hover:border-[var(--gw-green)] transition-colors cursor-pointer group"
              >
                {a.cover_image && (
                  <img
                    src={a.cover_image}
                    alt=""
                    className="w-full h-40 object-cover"
                  />
                )}
                <div className="p-5">
                  <div className="flex items-center gap-2 mb-3">
                    {a.tag_list.slice(0, 2).map((t) => (
                      <span
                        key={t}
                        className="font-mono-gw text-[10px] tracking-[0.1em] uppercase text-[var(--gw-green)] px-2 py-0.5 border border-[var(--gw-border-2)] rounded"
                      >
                        {t}
                      </span>
                    ))}
                  </div>
                  <h3 className="font-display text-lg font-semibold text-[var(--gw-white)] mb-2 group-hover:text-[var(--gw-green)] transition-colors">
                    {a.title}
                  </h3>
                  <p className="text-[13px] text-[var(--gw-white-dim)] line-clamp-2 mb-3">
                    {a.description}
                  </p>
                  <div className="flex items-center gap-2 text-[12px] text-[var(--gw-white-dim)]">
                    <span>{formatDate(a.published_at)}</span>
                    <span>·</span>
                    <span>{a.reading_time_minutes} min</span>
                  </div>
                </div>
              </button>
            ))}
          </div>
        )}

        <div className="mt-8 text-center">
          <a
            href={`https://dev.to/${USERNAME}`}
            target="_blank"
            rel="noopener noreferrer"
            className="gw-btn-ghost text-sm"
          >
            View all on dev.to →
          </a>
        </div>
      </div>

      {selected && (
        <ArticleModal article={selected} onClose={() => setSelected(null)} />
      )}
    </section>
  );
}
