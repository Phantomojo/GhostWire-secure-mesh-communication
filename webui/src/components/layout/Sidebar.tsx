import React from 'react';
import { useApp } from '../../context/AppContext';

type PanelId = 'dashboard' | 'communication' | 'channels' | 'network' | 'system' | 'security' | 'settings' | 'ark' | 'about';

type NavItem = {
  id: PanelId;
  label: string;
  code: string;
  hint: string;
  count?: number;
};

type NavGroup = {
  label: string;
  items: NavItem[];
};

function statusLabel(connected: boolean, locked: boolean) {
  if (!connected) return { label: 'offline', tone: 'danger' };
  if (locked) return { label: 'locked', tone: 'warn' };
  return { label: 'nominal', tone: 'safe' };
}

export default function Sidebar() {
  const { state, dispatch } = useApp();
  const status = statusLabel(state.backendConnected, state.isLocked);
  const groups: NavGroup[] = [
    {
      label: 'Operate',
      items: [
        { id: 'dashboard', label: 'Command', code: '01', hint: 'Node overview' },
        { id: 'communication', label: 'Messages', code: '02', hint: 'Encrypted conversations', count: state.messages.length },
        { id: 'channels', label: 'Channels', code: '03', hint: 'Broadcast surfaces' },
      ],
    },
    {
      label: 'Observe',
      items: [
        { id: 'network', label: 'Network', code: '04', hint: 'Discovery and peers', count: state.peerCount },
        { id: 'system', label: 'System', code: '05', hint: 'Runtime and telemetry' },
      ],
    },
    {
      label: 'Secure',
      items: [
        { id: 'security', label: 'Security', code: '06', hint: 'Threat posture and keys' },
        { id: 'settings', label: 'Settings', code: '07', hint: 'Profiles and controls' },
      ],
    },
    {
      label: 'Archive',
      items: [
        { id: 'ark', label: 'Ark', code: '08', hint: 'Durable local records' },
        { id: 'about', label: 'About', code: '09', hint: 'Build and protocol' },
      ],
    },
  ];

  return (
    <aside className="gw-sidebar" aria-label="GhostWire primary navigation">
      <div className="gw-brand-block">
        <button className="gw-brand" type="button" onClick={() => dispatch({ type: 'SET_PANEL', payload: 'dashboard' })} aria-label="Open GhostWire command center">
          <span className="gw-brand-mark" aria-hidden="true">GW</span>
          <span>
            <strong>GhostWire</strong>
            <small>resilient mesh / local-first</small>
          </span>
        </button>
        <div className={`gw-link-state ${status.tone}`}>
          <span className="gw-state-dot" aria-hidden="true" />
          <span>API link</span>
          <b>{status.label}</b>
        </div>
      </div>

      <nav className="gw-nav" aria-label="Product surfaces">
        {groups.map(group => (
          <section className="gw-nav-group" key={group.label}>
            <div className="gw-nav-group-label">{group.label}</div>
            {group.items.map(item => {
              const active = state.activePanel === item.id;
              return (
                <button
                  className={`gw-nav-item${active ? ' active' : ''}`}
                  type="button"
                  key={item.id}
                  onClick={() => dispatch({ type: 'SET_PANEL', payload: item.id })}
                  aria-current={active ? 'page' : undefined}
                  title={item.hint}
                >
                  <span className="gw-nav-code">{item.code}</span>
                  <span className="gw-nav-label">{item.label}</span>
                  {item.count !== undefined && <span className="gw-nav-count">{item.count}</span>}
                </button>
              );
            })}
          </section>
        ))}
      </nav>

      <div className="gw-sidebar-footer">
        <div className="gw-profile-chip">
          <span className="gw-avatar" aria-hidden="true">{(state.profile?.nickname || 'GW').slice(0, 2).toUpperCase()}</span>
          <span className="gw-profile-copy">
            <strong>{state.profile?.nickname || 'Local profile'}</strong>
            <small>{state.isLocked ? 'profile locked' : 'profile unlocked'}</small>
          </span>
          <span className={`gw-lock-mark ${state.isLocked ? 'locked' : 'open'}`} aria-label={state.isLocked ? 'Locked' : 'Unlocked'}>{state.isLocked ? 'L' : 'U'}</span>
        </div>
        <div className="gw-sidebar-meta">
          <span>SECURITY</span>
          <b className={state.securityScore >= 80 ? 'safe' : 'warn'}>{state.securityScore || 0}</b>
          <span className="gw-meta-separator">/</span>
          <span>PQC</span>
          <b className={state.pqcEnabled ? 'safe' : 'muted'}>{state.pqcEnabled ? 'ON' : 'OFF'}</b>
        </div>
      </div>
    </aside>
  );
}
