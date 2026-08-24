'use client'

import { useState } from "react";
import { GithubDocModal } from "@/components/site/github-doc-modal";

const TABS = [
  { id: "linux", label: "Linux" },
  { id: "docker", label: "Docker" },
  { id: "source", label: "From Source" },
] as const;

type TabId = (typeof TABS)[number]["id"];

function LinuxPanel() {
  return (
    <pre className="gw-code mt-2 whitespace-pre">
      <code>
        <span className="c-comment">{"# Clone and build"}</span>
        {"\n"}
        git clone -b main-promotion-20260824 https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication.git
        {"\n"}
        cd GhostWire-secure-mesh-communication/ghostwire
        {"\n"}
        cargo run --release
        {"\n\n"}
        <span className="c-comment">{"# Node starts on"}</span>
        {"\n"}
        <span className="c-green">http://127.0.0.1:8080</span>
        {"\n\n"}
        <span className="c-comment">{"# Or launch the TUI"}</span>
        {"\n"}
        cargo run --release -- tui
      </code>
    </pre>
  );
}

function DockerPanel() {
  return (
    <pre className="gw-code mt-2 whitespace-pre">
      <code>
        <span className="c-comment">{"# Pull and run"}</span>
        {"\n"}
        docker-compose up -d
        {"\n\n"}
        <span className="c-comment">{"# Check health"}</span>
        {"\n"}
        curl http://localhost:8080/api/v2/health
        {"\n\n"}
        <span className="c-comment">{"# Expected output"}</span>
        {"\n"}
        <span className="c-green">{'{"status":"healthy","version":"0.2.0"}'}</span>
      </code>
    </pre>
  );
}

function SourcePanel() {
  return (
    <pre className="gw-code mt-2 whitespace-pre">
      <code>
        <span className="c-comment">{"# Prerequisites: Rust 1.75+, Node 18+"}</span>
        {"\n"}
        git clone -b main-promotion-20260824 https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication.git
        {"\n"}
        cd GhostWire-secure-mesh-communication/ghostwire
        {"\n\n"}
        <span className="c-comment">{"# Build the core"}</span>
        {"\n"}
        cargo build --release
        {"\n\n"}
        <span className="c-comment">{"# Build the web UI"}</span>
        {"\n"}
        cd ../webui-src && npm install && npm run build
      </code>
    </pre>
  );
}

export function QuickstartSection() {
  const [activeTab, setActiveTab] = useState<TabId>("linux");
  const [showQuickStart, setShowQuickStart] = useState(false);

  return (
    <section id="quickstart" className="py-24 border-t border-[var(--gw-border)]">
      <div className="max-w-[1160px] mx-auto px-6 sm:px-8">
        <div className="gw-section-label mb-3">{"// quick start"}</div>
        <h2 className="font-display text-3xl sm:text-5xl font-bold mb-10 text-[var(--gw-white)]">
          Running in under 5 minutes
        </h2>

        <div role="tablist" className="flex gap-6 border-b border-[var(--gw-border)] mb-6">
          {TABS.map((tab) => {
            const isActive = activeTab === tab.id;
            return (
              <button
                key={tab.id}
                role="tab"
                id={`tab-${tab.id}`}
                aria-selected={isActive}
                aria-controls={`panel-${tab.id}`}
                onClick={() => setActiveTab(tab.id)}
                className={`font-mono-gw text-[13px] pb-3 cursor-pointer bg-transparent border-none tracking-[0.04em] ${
                  isActive
                    ? "text-[var(--gw-green)] border-b-2 border-[var(--gw-green)]"
                    : "text-[var(--gw-white-dim)] border-b-2 border-transparent hover:text-[var(--gw-white)]"
                }`}
              >
                {tab.label}
              </button>
            );
          })}
        </div>

        {activeTab === "linux" && (
          <div role="tabpanel" id="panel-linux" aria-labelledby="tab-linux">
            <LinuxPanel />
          </div>
        )}
        {activeTab === "docker" && (
          <div role="tabpanel" id="panel-docker" aria-labelledby="tab-docker">
            <DockerPanel />
          </div>
        )}
        {activeTab === "source" && (
          <div role="tabpanel" id="panel-source" aria-labelledby="tab-source">
            <SourcePanel />
          </div>
        )}

        <div className="flex flex-wrap gap-3 mt-8">
          <a href="#architecture" className="gw-btn-primary no-underline">
            Full documentation
          </a>
          <button onClick={() => setShowQuickStart(true)} className="gw-btn-ghost no-underline">
            Quick Start Guide
          </button>
        </div>
      </div>

      {showQuickStart && (
        <GithubDocModal open={showQuickStart} onClose={() => setShowQuickStart(false)} docPath="docs/QUICK_START.md" title="Quick Start Guide" />
      )}
    </section>
  );
}
