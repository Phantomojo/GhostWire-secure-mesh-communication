import React from 'react';
import { useApp } from '../../context/AppContext';

const TIER_CLASS: Record<string, string> = {
  stealth: 'tier-stealth',
  webrtc:  'tier-webrtc',
  covert:  'tier-covert',
  fronted: 'tier-fronted',
  offline: 'tier-offline',
};

const TIER_LABEL: Record<string, string> = {
  stealth: '⬤ STEALTH',
  webrtc:  '⬤ WEBRTC',
  covert:  '⬤ COVERT',
  fronted: '⬤ FRONTED',
  offline: '○ OFFLINE',
};

function MetricBar({ label, value, warn = 80, crit = 95 }: {
  label: string; value: number; warn?: number; crit?: number;
}) {
  const fillClass = value >= crit ? 'crit' : value >= warn ? 'warn' : '';
  return (
    <div style={{ display: 'flex', alignItems: 'center', gap: 8, minWidth: 120 }}>
      <span className="ops-label" style={{ minWidth: 32 }}>{label}</span>
      <div className="data-bar" style={{ flex: 1 }}>
        <div className={`data-bar-fill ${fillClass}`} style={{ width: `${value}%` }} />
      </div>
      <span style={{
        fontFamily: 'var(--mono)', fontSize: 11,
        color: value >= crit ? 'var(--crit)' : value >= warn ? 'var(--warn)' : 'var(--t2)',
        minWidth: 34, textAlign: 'right',
      }}>
        {value}%
      </span>
    </div>
  );
}

function formatUptime(s: number): string {
  const h = Math.floor(s / 3600);
  const m = Math.floor((s % 3600) / 60);
  if (h > 0) return `${h}h${m.toString().padStart(2,'0')}m`;
  return `${m}m${(s % 60).toString().padStart(2,'0')}s`;
}

export default function Header() {
  const { state, dispatch } = useApp();
  const tier = state.transport.tier;
  const tierClass = TIER_CLASS[tier] ?? 'tier-offline';
  const tierLabel = TIER_LABEL[tier] ?? '○ OFFLINE';

  return (
    <header style={{
      height: 48,
      borderBottom: '1px solid var(--border)',
      background: 'var(--surface)',
      display: 'flex',
      alignItems: 'center',
      padding: '0 16px',
      gap: 20,
      flexShrink: 0,
    }}>
      {/* Transport tier — most important status indicator */}
      <div className={`tier-badge ${tierClass}`}>
        {tierLabel}
      </div>

      {/* Latency */}
      {state.transport.internet_reachable && (
        <span style={{ fontFamily: 'var(--mono)', fontSize: 11, color: 'var(--t3)' }}>
          <span style={{ color: 'var(--t2)' }}>{state.latencyMs}</span>ms
        </span>
      )}

      <div style={{ flex: 1 }} />

      {/* System metrics */}
      <div style={{ display: 'flex', gap: 16, alignItems: 'center' }}>
        <MetricBar label="CPU" value={state.cpuPct} />
        <MetricBar label="MEM" value={state.memPct} />
      </div>

      <div style={{ width: 1, height: 20, background: 'var(--border)' }} />

      {/* Peers */}
      <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
        <div className={`sig-dot ${state.peerCount > 0 ? 'sig-dot-pulse' : ''}`}
          style={{ background: state.peerCount > 0 ? 'var(--sig)' : 'var(--t4)' }}
        />
        <span style={{ fontFamily: 'var(--mono)', fontSize: 11, color: 'var(--t2)' }}>
          {state.peerCount} peers
        </span>
      </div>

      {/* Uptime */}
      <span style={{ fontFamily: 'var(--mono)', fontSize: 11, color: 'var(--t3)' }}>
        UP {formatUptime(state.uptimeSecs)}
      </span>

      {/* Security score */}
      <div style={{
        display: 'flex', alignItems: 'center', gap: 5,
        padding: '3px 8px',
        borderRadius: 'var(--r-sm)',
        background: state.securityScore >= 80
          ? 'rgba(46,255,122,0.06)' : 'rgba(255,59,92,0.06)',
        border: `1px solid ${state.securityScore >= 80 ? 'var(--sig-dim)' : 'var(--crit-dim)'}`,
      }}>
        <span style={{
          fontFamily: 'var(--mono)', fontSize: 11,
          color: state.securityScore >= 80 ? 'var(--sig)' : 'var(--crit)',
          fontWeight: 700,
        }}>
          {state.securityScore}
        </span>
        <span className="ops-label">SEC</span>
      </div>

      {/* Config button */}
      <button
        className="btn-ops"
        onClick={() => dispatch({ type: 'SET_PANEL', payload: 'settings' })}
        style={{ padding: '4px 8px' }}
      >
        ⊞
      </button>
    </header>
  );
}
