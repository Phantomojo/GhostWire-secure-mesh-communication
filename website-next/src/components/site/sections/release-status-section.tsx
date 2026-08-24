export function ReleaseStatusSection() {
  return (
    <section id="release-status" className="border-y border-[var(--gw-border)] bg-[var(--gw-bg-soft)] py-16">
      <div className="mx-auto max-w-6xl px-6">
        <p className="font-mono-gw text-[11px] tracking-[0.12em] text-[var(--gw-green)] uppercase">{"// release status"}</p>
        <div className="mt-4 grid gap-8 lg:grid-cols-[1.35fr_1fr] lg:items-start">
          <div>
            <h2 className="max-w-3xl text-3xl font-semibold tracking-tight text-[var(--gw-white)] sm:text-4xl">Source is available. Stable binaries are not.</h2>
            <p className="mt-4 max-w-3xl text-[15px] leading-7 text-[var(--gw-white-dim)]">
              GhostWire is an actively developed protocol and Rust reference implementation. The current Codeberg promotion branch is a review snapshot of the authoritative Omen tree, not a stable release and not a security audit.
            </p>
            <div className="mt-6 flex flex-wrap gap-3">
              <a href="https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication/src/branch/main-promotion-20260824" target="_blank" rel="noopener" className="gw-btn-primary">Review Codeberg snapshot</a>
              <a href="https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication/src/branch/audit/verified-20260824" target="_blank" rel="noopener" className="gw-btn-secondary">View audit branch</a>
            </div>
          </div>
          <div className="border-l border-[var(--gw-border)] pl-6">
            <p className="font-mono-gw text-[11px] tracking-[0.12em] text-[var(--gw-green)] uppercase">verified in this review</p>
            <ul className="mt-4 space-y-3 text-[13px] leading-6 text-[var(--gw-white-dim)]">
              <li>• Publication tree matches the authoritative Omen snapshot.</li>
              <li>• Web UI production build and Rust cargo check pass in isolation.</li>
              <li>• Current publication secret-pattern checks are clean.</li>
              <li>• Full tests, dependency audit, clippy, and full Docker build remain release gates.</li>
            </ul>
          </div>
        </div>
        <div className="mt-10 border-t border-[var(--gw-border)] pt-6">
          <p className="text-[13px] leading-6 text-[var(--gw-white-dim)]">
            <strong className="text-[var(--gw-orange)]">No official binary download is currently provided.</strong> Do not install an unversioned or unsigned binary. A future release will publish an immutable artefact, SHA-256 manifest, signed release manifest, source commit, build metadata, and SBOM together.
          </p>
        </div>
      </div>
    </section>
  );
}
