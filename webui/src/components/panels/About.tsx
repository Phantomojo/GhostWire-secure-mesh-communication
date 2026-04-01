import React from 'react';

const ROADMAP = [
  { wave: 'Wave 1–6', label: 'Core protocol', done: true,
    items: ['libp2p mesh', 'Gossipsub', 'Kademlia DHT', 'WebUI', 'Identity system'] },
  { wave: 'Wave 7', label: 'Security hardening', done: true,
    items: ['Real X25519 Sphinx', 'vodozemac Double Ratchet', 'LRU replay protection', 'Binary hardening', 'Genesis ceremony'] },
  { wave: 'Wave 8', label: 'Censorship resistance', done: false,
    items: ['WebRTC transport tier', 'VP8 steganography', 'HTTPS domain fronting', 'Duress PIN', 'Dead man\'s switch'] },
  { wave: 'Wave 9', label: 'Mobile & offline', done: false,
    items: ['Android Tauri app', 'Foreground service', 'Bluetooth LE', 'LoRa bridge', '60-second onboarding'] },
  { wave: 'Wave 10', label: 'Maturity', done: false,
    items: ['Third-party security audit', 'Reproducible builds', 'Bootstrap network', 'Community governance', 'NLnet grant delivery'] },
];

const STATS = [
  { label: 'Rust source files', value: '102' },
  { label: 'Lines of Rust', value: '27,932' },
  { label: 'Genesis fingerprint', value: '848CF75A' },
  { label: 'Protocol version', value: 'v2.0' },
];

export default function AboutPanel() {
  return (
    <div className="panel-scroll animate-in">
      {/* Hero */}
      <div className="ops-panel" style={{ padding: 20, marginBottom: 14 }}>
        <div style={{ fontFamily: 'var(--mono)', fontSize: 22, fontWeight: 700, color: 'var(--sig)', letterSpacing: '0.2em', marginBottom: 6 }}>
          GHOSTWIRE
        </div>
        <div style={{ fontFamily: 'var(--mono)', fontSize: 10, color: 'var(--t3)', letterSpacing: '0.15em', marginBottom: 14 }}>
          SECURE MESH COMMUNICATION PROTOCOL
        </div>
        <div style={{ fontFamily: 'var(--mono)', fontSize: 12, color: 'var(--t2)', lineHeight: 1.7, maxWidth: 560 }}>
          Privacy infrastructure for the Global South. Built for Safaricom 3G, protest scenarios,
          metered networks, and censored environments. No servers required for local mesh operation.
          Encryption is not a feature — it is the architecture.
        </div>
        <div style={{ marginTop: 14, fontFamily: 'var(--mono)', fontSize: 11, color: 'var(--sig-dim)', fontStyle: 'italic' }}>
          "I am because we are." — Ubuntu
        </div>
      </div>

      {/* Stats */}
      <div className="metric-grid" style={{ marginBottom: 14 }}>
        {STATS.map(s => (
          <div key={s.label} className="metric-card">
            <div className="ops-label" style={{ marginBottom: 4 }}>{s.label}</div>
            <div style={{ fontFamily: 'var(--mono)', fontSize: 16, fontWeight: 700, color: 'var(--sig)' }}>{s.value}</div>
          </div>
        ))}
      </div>

      {/* Roadmap */}
      <div className="ops-panel" style={{ padding: 14, marginBottom: 14 }}>
        <div className="ops-section-header"><span className="ops-section-title">Roadmap</span></div>
        {ROADMAP.map((w, i) => (
          <div key={i} style={{ display: 'flex', gap: 14, marginBottom: i < ROADMAP.length-1 ? 16 : 0 }}>
            {/* Timeline line */}
            <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', flexShrink: 0, width: 20 }}>
              <div style={{
                width: 10, height: 10, borderRadius: '50%', flexShrink: 0,
                background: w.done ? 'var(--sig)' : 'var(--border-hi)',
                boxShadow: w.done ? 'var(--sig-glow)' : 'none',
                border: `1px solid ${w.done ? 'var(--sig)' : 'var(--border)'}`,
              }} />
              {i < ROADMAP.length - 1 && (
                <div style={{ flex: 1, width: 1, background: w.done ? 'var(--sig-dim)' : 'var(--border)', marginTop: 4 }} />
              )}
            </div>
            {/* Content */}
            <div style={{ flex: 1, paddingBottom: 4 }}>
              <div style={{ display: 'flex', gap: 8, alignItems: 'center', marginBottom: 5 }}>
                <span style={{ fontFamily: 'var(--mono)', fontSize: 11, fontWeight: 700, color: w.done ? 'var(--sig)' : 'var(--t2)' }}>
                  {w.wave}
                </span>
                <span className={`ops-tag ops-tag-${w.done ? 'sig' : 'muted'}`} style={{ fontSize: 9 }}>
                  {w.done ? '✓ DONE' : 'PLANNED'}
                </span>
                <span style={{ fontFamily: 'var(--mono)', fontSize: 10, color: 'var(--t3)' }}>{w.label}</span>
              </div>
              <div className="tag-row">
                {w.items.map(item => (
                  <span key={item} className={`ops-tag ops-tag-${w.done ? 'sig' : 'muted'}`} style={{ fontSize: 9 }}>{item}</span>
                ))}
              </div>
            </div>
          </div>
        ))}
      </div>

      {/* Technical stack */}
      <div className="ops-panel" style={{ padding: 14 }}>
        <div className="ops-section-header"><span className="ops-section-title">Technical stack</span></div>
        <div className="panel-grid-2">
          {[
            { label: 'Transport', value: 'libp2p 0.56 + Sphinx onion routing + S/Kademlia DHT' },
            { label: 'Encryption', value: 'ChaCha20-Poly1305 · vodozemac Double Ratchet · ML-KEM-768' },
            { label: 'Privacy', value: 'Sphinx mixing · cover traffic · SURB anonymous reply' },
            { label: 'Resilience', value: 'DTN Spray-and-Wait · PRoPHET routing · mDNS auto-discovery' },
            { label: 'Identity', value: 'Ed25519 + X25519 · Argon2id key derivation · Shamir recovery' },
            { label: 'Backend', value: 'Rust + Axum · ceremony-derived protocol constants' },
            { label: 'Frontend', value: 'React 19 + Vite · self-hosted fonts · WebSocket real-time' },
            { label: 'Security', value: 'mlock memory · binary integrity · obfstr string encryption' },
          ].map(item => (
            <div key={item.label} style={{ padding: '8px 0', borderBottom: '1px solid var(--border)' }}>
              <div className="ops-label" style={{ marginBottom: 3 }}>{item.label}</div>
              <div style={{ fontFamily: 'var(--mono)', fontSize: 10, color: 'var(--t2)', lineHeight: 1.5 }}>{item.value}</div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
