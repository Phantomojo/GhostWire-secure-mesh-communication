import React from 'react';
import { useApp, type ActivePanel } from '../../context/AppContext';

const NAV: Array<{ id: ActivePanel; sigil: string; label: string }> = [
  { id: 'dashboard',     sigil: '◈', label: 'Home'   },
  { id: 'communication', sigil: '⟁', label: 'Comms'  },
  { id: 'channels',      sigil: '◉', label: 'Chan'   },
  { id: 'network',       sigil: '⬡', label: 'Net'    },
  { id: 'security',      sigil: '⬢', label: 'Sec'    },
  { id: 'settings',      sigil: '⊞', label: 'Config' },
];

export default function MobileNav() {
  const { state, dispatch } = useApp();
  return (
    <nav style={{
      position: 'fixed', bottom: 0, left: 0, right: 0,
      height: 60,
      background: 'var(--surface)',
      borderTop: '1px solid var(--border)',
      display: 'flex', alignItems: 'center', justifyContent: 'space-around',
      zIndex: 100,
      paddingBottom: 'env(safe-area-inset-bottom)',
    }}>
      {NAV.map(item => {
        const active = state.activePanel === item.id;
        return (
          <button
            key={item.id}
            onClick={() => dispatch({ type: 'SET_PANEL', payload: item.id })}
            style={{
              display: 'flex', flexDirection: 'column', alignItems: 'center', gap: 3,
              padding: '4px 10px',
              background: 'none', border: 'none', cursor: 'pointer',
              color: active ? 'var(--sig)' : 'var(--t3)',
              transition: 'color 0.12s',
            }}
          >
            <span style={{
              fontSize: 16,
              filter: active ? 'drop-shadow(0 0 4px var(--sig))' : 'none',
            }}>{item.sigil}</span>
            <span style={{ fontFamily: 'var(--mono)', fontSize: 8, letterSpacing: '0.08em', textTransform: 'uppercase' }}>
              {item.label}
            </span>
          </button>
        );
      })}
    </nav>
  );
}
