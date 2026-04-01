import UserProfile from '../shared/UserProfile';
import React, { useState } from 'react';
import { useApp, type ActivePanel } from '../../context/AppContext';

const GENESIS_FINGERPRINT = '848CF75A';

const NAV: Array<{ id: ActivePanel; label: string; sigil: string }> = [
  { id: 'dashboard',     label: 'Overview',    sigil: '◈' },
  { id: 'communication', label: 'Comms',       sigil: '⟁' },
  { id: 'channels',      label: 'Channels',    sigil: '◉' },
  { id: 'network',       label: 'Network',     sigil: '⬡' },
  { id: 'ark',           label: 'Knowledge',   sigil: '⬟' },
  { id: 'security',      label: 'Security',    sigil: '⬢' },
  { id: 'system',        label: 'System',      sigil: '⊕' },
  { id: 'settings',      label: 'Config',      sigil: '⊞' },
  { id: 'about',         label: 'About',       sigil: '◌' },
];

const PROTOCOLS: Array<{ id: string; label: string; stateKey: 'isEmergencyMode' | 'isStealthMode' | 'isPanicMode'; actionType: string }> = [
  { id: 'emergency', label: 'Emergency', stateKey: 'isEmergencyMode', actionType: 'SET_EMERGENCY' },
  { id: 'stealth',   label: 'Stealth',   stateKey: 'isStealthMode',   actionType: 'SET_STEALTH'   },
  { id: 'panic',     label: 'Panic wipe',stateKey: 'isPanicMode',     actionType: 'SET_PANIC'     },
];

export default function Sidebar() {
  const { state, dispatch } = useApp();
  const [collapsed, setCollapsed] = useState(false);

  const nodeId = state.profile?.id?.slice(0, 16) ?? '...';
  const did = state.profile?.did?.replace('did:ghost:', '').slice(0, 12) ?? '...';

  return (
    <aside
      style={{
        width: collapsed ? 52 : 220,
        transition: 'width 0.2s ease',
        background: 'var(--surface)',
        borderRight: '1px solid var(--border)',
        display: 'flex',
        flexDirection: 'column',
        height: '100%',
        overflow: 'hidden',
        flexShrink: 0,
      }}
    >
      {/* Header */}
      <div style={{
        padding: collapsed ? '16px 10px' : '16px 14px',
        borderBottom: '1px solid var(--border)',
        display: 'flex',
        alignItems: 'center',
        gap: 10,
        justifyContent: collapsed ? 'center' : 'space-between',
      }}>
        {!collapsed && (
          <div>
            <div style={{
              fontFamily: 'var(--mono)',
              fontWeight: 700,
              fontSize: 14,
              letterSpacing: '0.18em',
              color: 'var(--sig)',
            }}>
              GHOSTWIRE
            </div>
            <div style={{
              fontFamily: 'var(--mono)',
              fontSize: 9,
              letterSpacing: '0.15em',
              color: 'var(--t3)',
              marginTop: 1,
            }}>
              NET·OPS·V2
            </div>
          </div>
        )}
        {collapsed && (
          <div style={{ color: 'var(--sig)', fontSize: 16, fontWeight: 700 }}>GW</div>
        )}
        <button
          onClick={() => setCollapsed(!collapsed)}
          style={{
            background: 'none',
            border: '1px solid var(--border)',
            borderRadius: 'var(--r-sm)',
            color: 'var(--t3)',
            cursor: 'pointer',
            width: 22,
            height: 22,
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            fontSize: 10,
            flexShrink: 0,
          }}
        >
          {collapsed ? '▶' : '◀'}
        </button>
      </div>

      {/* Node identity */}
      {!collapsed && (
        <div style={{
          padding: '10px 14px',
          borderBottom: '1px solid var(--border)',
        }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: 6, marginBottom: 4 }}>
            <div className={`sig-dot ${state.backendConnected ? 'sig-dot-pulse' : ''}`}
              style={{ background: state.backendConnected ? 'var(--sig)' : 'var(--crit)' }}
            />
            <span style={{ fontSize: 10, color: 'var(--t3)', letterSpacing: '0.1em', textTransform: 'uppercase' }}>
              {state.backendConnected ? 'node online' : 'link lost'}
            </span>
          </div>
          <div className="genesis-id">
            NODE <span>{nodeId}</span>
          </div>
          <div className="genesis-id" style={{ marginTop: 2 }}>
            DID  <span>{did}</span>
          </div>
        </div>
      )}

      {/* Navigation */}
      <nav style={{ flex: 1, padding: '8px 8px', overflowY: 'auto' }}>
        {!collapsed && (
          <div className="ops-label" style={{ padding: '4px 4px 8px' }}>Operations</div>
        )}
        {NAV.map(item => {
          const active = state.activePanel === item.id;
          return (
            <button
              key={item.id}
              className={`nav-item ${active ? 'active' : ''}`}
              onClick={() => dispatch({ type: 'SET_PANEL', payload: item.id })}
              title={collapsed ? item.label : undefined}
              style={{
                width: '100%',
                background: 'none',
                cursor: 'pointer',
                marginBottom: 2,
                justifyContent: collapsed ? 'center' : 'flex-start',
                paddingLeft: collapsed ? 0 : undefined,
              }}
            >
              <span style={{ fontSize: 13, width: 16, textAlign: 'center', flexShrink: 0 }}>
                {item.sigil}
              </span>
              {!collapsed && <span>{item.label}</span>}
            </button>
          );
        })}
      </nav>

      {/* Protocol toggles */}
      <div style={{
        padding: '8px',
        borderTop: '1px solid var(--border)',
      }}>
        {!collapsed && (
          <div className="ops-label" style={{ padding: '4px 4px 8px' }}>Protocols</div>
        )}
        {PROTOCOLS.map(p => {
          const active = state[p.stateKey];
          return (
            <button
              key={p.id}
              onClick={() => {
                dispatch({ type: p.actionType as any, payload: !active });
              }}
              title={collapsed ? p.label : undefined}
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: 8,
                width: '100%',
                padding: collapsed ? '6px 0' : '5px 8px',
                justifyContent: collapsed ? 'center' : 'space-between',
                background: active ? 'rgba(255,59,92,0.06)' : 'none',
                border: `1px solid ${active ? 'var(--crit-dim)' : 'transparent'}`,
                borderRadius: 'var(--r-sm)',
                cursor: 'pointer',
                marginBottom: 2,
              }}
            >
              <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
                <div style={{
                  width: 5, height: 5, borderRadius: '50%',
                  background: active ? 'var(--crit)' : 'var(--t4)',
                  flexShrink: 0,
                }} />
                {!collapsed && (
                  <span style={{
                    fontSize: 10,
                    letterSpacing: '0.1em',
                    textTransform: 'uppercase',
                    color: active ? 'var(--crit)' : 'var(--t3)',
                    fontFamily: 'var(--mono)',
                  }}>
                    {p.label}
                  </span>
                )}
              </div>
              {!collapsed && (
                <span style={{
                  fontSize: 9,
                  color: active ? 'var(--crit)' : 'var(--t4)',
                  fontFamily: 'var(--mono)',
                  letterSpacing: '0.08em',
                }}>
                  {active ? 'ON' : 'OFF'}
                </span>
              )}
            </button>
          );
        })}
      </div>

      <UserProfile />

      {/* Genesis fingerprint */}
      {!collapsed && (
        <div style={{
          padding: '8px 14px',
          borderTop: '1px solid var(--border)',
        }}>
          <div className="genesis-id">
            NET·FP <span>{GENESIS_FINGERPRINT}</span>
          </div>
          <div className="genesis-id" style={{ marginTop: 2 }}>
            NICK <span>{state.profile?.nickname?.slice(0, 12) ?? '...'}</span>
          </div>
        </div>
      )}
    </aside>
  );
}
