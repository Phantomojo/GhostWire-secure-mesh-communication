'use client'

import { useEffect, useState } from "react";
import { createPortal } from "react-dom";
import { GithubDocModal } from "./github-doc-modal";

interface FeatureModalProps {
  open: boolean;
  onClose: () => void;
  tag: string;
  title: string;
  children: React.ReactNode;
  ctaPrimary?: { href: string; label: string; docPath?: string; docTitle?: string };
  ctaSecondary?: { href: string; label: string };
}

export function FeatureModal({
  open,
  onClose,
  tag,
  title,
  children,
  ctaPrimary,
  ctaSecondary,
}: FeatureModalProps) {
  const [showDoc, setShowDoc] = useState(false);

  useEffect(() => {
    if (!open) return;
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") onClose();
    };
    document.addEventListener("keydown", onKey);
    document.body.style.overflow = "hidden";
    return () => {
      document.removeEventListener("keydown", onKey);
      document.body.style.overflow = "";
    };
  }, [open, onClose]);

  if (!open || typeof document === "undefined") return null;

  return createPortal(
    <div
      className="fixed inset-0 z-[100] flex items-center justify-center p-4 sm:p-6"
      role="dialog"
      aria-modal="true"
      aria-labelledby="feature-modal-title"
    >
      <div
        className="absolute inset-0 bg-black/70 backdrop-blur-sm"
        onClick={onClose}
        aria-hidden="true"
      />
      <div className="relative bg-[var(--gw-bg-2)] border border-[var(--gw-border-2)] rounded-lg max-w-2xl w-full max-h-[88vh] overflow-y-auto no-scrollbar shadow-2xl gw-reveal">
        <button
          onClick={onClose}
          aria-label="Close modal"
          className="absolute top-4 right-4 w-8 h-8 flex items-center justify-center text-[var(--gw-white-dim)] hover:text-[var(--gw-green)] text-2xl bg-transparent border-none cursor-pointer"
        >
          ×
        </button>
        <div className="p-7 sm:p-9">
          <span className="font-mono-gw text-[11px] tracking-[0.1em] uppercase text-[var(--gw-green)] inline-block px-2 py-1 border border-[var(--gw-border-2)] rounded mb-4">
            {tag}
          </span>
          <h2
            id="feature-modal-title"
            className="font-display text-3xl sm:text-4xl font-bold mb-5 text-[var(--gw-white)]"
          >
            {title}
          </h2>
          <div className="space-y-5 text-[var(--gw-white-dim)] text-[15px] leading-relaxed">
            {children}
          </div>
          {(ctaPrimary || ctaSecondary) && (
            <div className="mt-7 flex flex-wrap gap-3">
              {ctaPrimary && ctaPrimary.docPath && (
                <button onClick={() => setShowDoc(true)} className="gw-btn-primary">
                  {ctaPrimary.label}
                </button>
              )}
              {ctaPrimary && !ctaPrimary.docPath && (
                <a href={ctaPrimary.href} className="gw-btn-primary">
                  {ctaPrimary.label}
                </a>
              )}
              {ctaSecondary && (
                <a href={ctaSecondary.href} className="gw-btn-ghost">
                  {ctaSecondary.label}
                </a>
              )}
            </div>
          )}
        </div>
      </div>

      {showDoc && ctaPrimary?.docPath && (
        <GithubDocModal open={showDoc} onClose={() => setShowDoc(false)} docPath={ctaPrimary.docPath} title={ctaPrimary.docTitle || ctaPrimary.label} />
      )}
    </div>,
    document.body
  );
}
