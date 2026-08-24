import React from 'react';
import { useApp } from '../../context/AppContext';

const TITLES: Record<string, { title: string; subtitle: string }> = {
  dashboard: { title: 'Command center', subtitle: 'One calm view of this node' },
  communication: { title: 'Messages', subtitle: 'Encrypted conversations and delivery evidence' },
  channels: { title: 'Channels', subtitle: 'Broadcast surfaces and local membership' },
  network: { title: 'Network cockpit', subtitle: 'Discovery evidence, peers, and transport state' },
  system: { title: 'System', subtitle: 'Runtime health, counters, and diagnostics' },
  security: { title: 'Security', subtitle: 'Identity, cryptography, and threat posture' },
  settings: { title: 'Settings', subtitle: 'Profiles, controls, and operating policy' },
  ark: { title: 'Ark', subtitle: 'Durable local records and recovery material' },
  about: { title: 'About', subtitle: 'Build, protocol, and runtime provenance' },
};

function StateChip({ label, value, tone = 'neutral' }: { label: string; value: string; tone?: string }) {
  return (
    <div className={`gw-header-chip ${tone}`}>
      <span>{label}</span>
      <b>{value}</b>
    </div>
  );
}

export default function Header() {
  const { state, refreshMesh, refreshIdentity, dispatch } = useApp();
  const page = TITLES[state.activePanel] || TITLES.dashboard;
  const transport = state.transport?.display_name || 'No active transport';
  const transportTone = state.transport?.display_name ? 'safe' : 'warn';
  const identity = state.isLocked ? 'Locked' : (state.profile?.nickname || 'Unlocked');

  return (
    <header className="gw-header">
      <div className="gw-header-title">
        <div className="gw-breadcrumb"><span>GHOSTWIRE</span><i>/</i><b>{page.title.toUpperCase()}</b></div>
        <h1>{page.title}</h1>
        <p>{page.subtitle}</p>
      </div>
      <div className="gw-header-actions">
        <StateChip label="API" value={state.backendConnected ? 'READY' : 'OFFLINE'} tone={state.backendConnected ? 'safe' : 'danger'} />
        <StateChip label="IDENTITY" value={identity} tone={state.isLocked ? 'warn' : 'safe'} />
        <StateChip label="TRANSPORT" value={transport} tone={transportTone} />
        {state.transport?.latency_ms !== undefined && <span className="gw-latency">{Math.round(state.transport.latency_ms)} ms</span>}
        <button className="gw-icon-action" type="button" onClick={() => { void refreshIdentity(); void refreshMesh(); }} title="Refresh live state" aria-label="Refresh live state">↻</button>
        <button className="gw-command-action" type="button" onClick={() => dispatch({ type: 'SET_PANEL', payload: 'network' })} title="Open network cockpit">
          <span className="gw-command-key">N</span><span>inspect</span>
        </button>
      </div>
    </header>
  );
}
