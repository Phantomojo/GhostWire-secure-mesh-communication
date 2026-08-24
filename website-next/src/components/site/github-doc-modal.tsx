'use client'

import { useEffect, useState, useCallback } from "react";
import { createPortal } from "react-dom";

const REPO_RAW =
  "https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication/raw/branch/main-promotion-20260824/";

interface GithubDocModalProps {
  open: boolean;
  onClose: () => void;
  /** Path relative to repo root, e.g. "docs/QUICK_START.md" */
  docPath: string;
  title: string;
}

/** Minimal markdown → HTML (headings, code blocks, bold, italic, links, lists, paragraphs) */
function renderMd(src: string): string {
  let html = src
    // code blocks (```...```)
    .replace(/```(\w*)\n([\s\S]*?)```/g, '<pre class="gw-code my-4 overflow-x-auto text-[13px]"><code>$2</code></pre>')
    // inline code
    .replace(/`([^`]+)`/g, '<code class="font-mono-gw text-[var(--gw-green)] bg-[var(--gw-bg)] px-1.5 py-0.5 rounded text-[13px]">$1</code>')
    // headings
    .replace(/^#### (.+)$/gm, '<h4 class="font-display text-lg font-semibold text-[var(--gw-white)] mt-6 mb-2">$1</h4>')
    .replace(/^### (.+)$/gm, '<h3 class="font-display text-xl font-bold text-[var(--gw-white)] mt-8 mb-3">$1</h3>')
    .replace(/^## (.+)$/gm, '<h2 class="font-display text-2xl font-bold text-[var(--gw-white)] mt-10 mb-4">$1</h2>')
    .replace(/^# (.+)$/gm, '<h1 class="font-display text-3xl font-bold text-[var(--gw-white)] mt-10 mb-4">$1</h1>')
    // bold + italic
    .replace(/\*\*\*(.+?)\*\*\*/g, '<strong><em>$1</em></strong>')
    .replace(/\*\*(.+?)\*\*/g, '<strong class="text-[var(--gw-white)]">$1</strong>')
    .replace(/\*(.+?)\*/g, '<em>$1</em>')
    // links
    .replace(/\[([^\]]+)\]\(([^)]+)\)/g, '<a href="$2" target="_blank" rel="noopener" class="text-[var(--gw-green)] hover:underline">$1</a>')
    // horizontal rules
    .replace(/^---+$/gm, '<hr class="border-[var(--gw-border)] my-6" />')
    // unordered lists
    .replace(/^[*-] (.+)$/gm, '<li class="ml-4 mb-1 text-[var(--gw-white-dim)]">$1</li>')
    // ordered lists
    .replace(/^\d+\. (.+)$/gm, '<li class="ml-4 mb-1 list-decimal text-[var(--gw-white-dim)]">$1</li>');

  // wrap consecutive <li> in <ul>
  html = html.replace(/((?:<li[^>]*>.*?<\/li>\n?)+)/g, (m) => {
    const isOrdered = m.includes('list-decimal');
    return `<${isOrdered ? 'ol' : 'ul'} class="my-3 space-y-1">${m}</${isOrdered ? 'ol' : 'ul'}>`;
  });

  // paragraphs: wrap remaining loose lines
  html = html
    .split('\n\n')
    .map(block => {
      const trimmed = block.trim();
      if (!trimmed) return '';
      if (trimmed.startsWith('<h') || trimmed.startsWith('<pre') || trimmed.startsWith('<ul') || trimmed.startsWith('<ol') || trimmed.startsWith('<hr') || trimmed.startsWith('<table')) return trimmed;
      return `<p class="mb-3">${trimmed}</p>`;
    })
    .join('\n');

  return html;
}

export function GithubDocModal({ open, onClose, docPath, title }: GithubDocModalProps) {
  const [html, setHtml] = useState("");
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  const fetchDoc = useCallback(async () => {
    if (!open) return;
    setLoading(true);
    setError("");
    try {
      const res = await fetch(REPO_RAW + docPath);
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const md = await res.text();
      setHtml(renderMd(md));
    } catch (e) {
      setError(e instanceof Error ? e.message : "Failed to load");
    } finally {
      setLoading(false);
    }
  }, [open, docPath]);

  useEffect(() => {
    if (!open) return;
    const task = window.setTimeout(() => { void fetchDoc(); }, 0);
    return () => window.clearTimeout(task);
  }, [open, fetchDoc]);

  useEffect(() => {
    if (!open) return;
    const onKey = (e: KeyboardEvent) => { if (e.key === "Escape") onClose(); };
    document.addEventListener("keydown", onKey);
    document.body.style.overflow = "hidden";
    return () => {
      document.removeEventListener("keydown", onKey);
      document.body.style.overflow = "";
    };
  }, [open, onClose]);

  if (!open || typeof document === "undefined") return null;

  return createPortal(
    <div className="fixed inset-0 z-[100] flex items-center justify-center p-4 sm:p-6" role="dialog" aria-modal="true">
      <div className="absolute inset-0 bg-black/70 backdrop-blur-sm" onClick={onClose} aria-hidden="true" />
      <div className="relative bg-[var(--gw-bg-2)] border border-[var(--gw-border-2)] rounded-lg max-w-3xl w-full max-h-[88vh] overflow-y-auto no-scrollbar shadow-2xl gw-reveal">
        <button
          onClick={onClose}
          aria-label="Close"
          className="absolute top-4 right-4 w-8 h-8 flex items-center justify-center text-[var(--gw-white-dim)] hover:text-[var(--gw-green)] text-2xl bg-transparent border-none cursor-pointer z-10"
        >
          ×
        </button>
        <div className="p-7 sm:p-9">
          <span className="font-mono-gw text-[11px] tracking-[0.1em] uppercase text-[var(--gw-green)] inline-block px-2 py-1 border border-[var(--gw-border-2)] rounded mb-4">
            docs
          </span>
          <h2 className="font-display text-2xl sm:text-3xl font-bold mb-5 text-[var(--gw-white)]">
            {title}
          </h2>

          {loading && (
            <div className="flex items-center gap-3 text-[var(--gw-white-dim)] text-[14px] py-8">
              <div className="w-5 h-5 border-2 border-[var(--gw-green)] border-t-transparent rounded-full animate-spin" />
              Loading from repository...
            </div>
          )}

          {error && (
            <div className="text-[var(--gw-orange)] text-[14px] py-8">
              <p className="mb-2">Failed to load document.</p>
              <a
                href={`https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication/blob/main-promotion-20260824/${docPath}`}
                target="_blank"
                rel="noopener"
                className="text-[var(--gw-green)] hover:underline text-[13px]"
              >
                Open on Codeberg instead →
              </a>
            </div>
          )}

          {!loading && !error && (
            <div
              className="prose-gw text-[var(--gw-white-dim)] text-[15px] leading-relaxed"
              dangerouslySetInnerHTML={{ __html: html }}
            />
          )}

          <div className="mt-6 pt-4 border-t border-[var(--gw-border)] flex items-center gap-4">
            <a
              href={`https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication/blob/main/${docPath}`}
              target="_blank"
              rel="noopener"
              className="font-mono-gw text-[12px] text-[var(--gw-green)] hover:text-[var(--gw-white)] transition-colors"
            >
              View on Codeberg →
            </a>
            <span className="text-[var(--gw-white-faint)] text-[11px]">·</span>
            <a
              href={`https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication/edit/main-promotion-20260824/${docPath}`}
              target="_blank"
              rel="noopener"
              className="font-mono-gw text-[12px] text-[var(--gw-white-faint)] hover:text-[var(--gw-green)] transition-colors"
            >
              Edit this page →
            </a>
          </div>
        </div>
      </div>
    </div>,
    document.body
  );
}
