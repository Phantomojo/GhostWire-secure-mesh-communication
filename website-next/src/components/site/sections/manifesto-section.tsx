export function ManifestoSection() {
  const principles: { roman: string; text: string }[] = [
    {
      roman: "I.",
      text: "Connection cannot be owned. Information wants to be free not because someone decided this, but because this is its nature.",
    },
    {
      roman: "II.",
      text: "Power distributed is power that cannot be seized. The architecture itself is the philosophy.",
    },
    {
      roman: "III.",
      text: "Ubuntu — I am because we are. The stranger is a node that has not yet connected to the net.",
    },
    {
      roman: "IV.",
      text: "GhostWire begins here: not as a product, but as a restoration. The first root of Yggdrasil.",
    },
  ];

  return (
    <section
      id="manifesto"
      className="py-24 border-t border-[var(--gw-border)]"
    >
      <div className="max-w-[1160px] mx-auto px-6 sm:px-8">
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-12 items-start">
          <div>
            <div className="gw-section-label mb-3">{"// manifesto"}</div>
            <h2 className="font-display text-3xl sm:text-4xl font-bold mb-6 text-[var(--gw-white)] leading-tight">
              The internet was built to be ungovernable.
            </h2>
            <div className="space-y-4 text-[var(--gw-white-dim)] text-[15px] leading-relaxed">
              <p>
                In Hindu and Buddhist cosmology,{" "}
                <strong className="text-[var(--gw-white)]">
                  Indra&apos;s Net
                </strong>{" "}
                is an infinite web. At each intersection hangs a jewel. Each
                reflected in all others. No jewel is more important. The net has
                no center. The net has no edge.
              </p>
              <p>
                The original internet architects independently rediscovered what
                African philosophy had encoded for millennia: systems built on
                mutual relationship rather than central authority are more
                resilient, more equitable, and more aligned with existence
                itself.
              </p>
              <p>
                <em className="text-[var(--gw-green)]">Ubuntu</em> — I am
                because we are. The stranger is a node that has not yet
                connected to the net.
              </p>
              <p className="text-[var(--gw-white-faint)] text-[13px] italic">
                — From <em>GhostWire: The First Root of Yggdrasil</em>
              </p>
            </div>
            <a href="#about" className="gw-btn-outline mt-7">
              Read the full manifesto
            </a>
          </div>
          <div className="space-y-4">
            {principles.map((p) => (
              <div
                key={p.roman}
                className="bg-[var(--gw-bg-2)] border border-[var(--gw-border)] rounded-lg p-6 gw-card-hover"
              >
                <div className="font-mono-gw text-2xl font-semibold text-[var(--gw-green)] mb-2">
                  {p.roman}
                </div>
                <p className="text-[var(--gw-white-dim)] text-[15px] leading-relaxed">
                  {p.text}
                </p>
              </div>
            ))}
          </div>
        </div>
      </div>
    </section>
  );
}
