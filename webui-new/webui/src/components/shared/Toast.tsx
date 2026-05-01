import React, { useEffect } from 'react';
import { useApp, type Toast } from '../../context/AppContext';

const ICONS: Record<Toast['type'], string> = {
  success: '✓',
  error:   '✕',
  warn:    '⚠',
  info:    '◈',
};

function ToastItem({ toast }: { toast: Toast }) {
  const { dispatch } = useApp();
  return (
    <div
      className={`ops-toast ops-toast-${toast.type}`}
      style={{ cursor: 'pointer', userSelect: 'none' }}
      onClick={() => dispatch({ type: 'TOAST_REMOVE', payload: toast.id })}
    >
      <span style={{
        fontWeight: 700,
        color: toast.type === 'success' ? 'var(--sig)'
             : toast.type === 'error'   ? 'var(--crit)'
             : toast.type === 'warn'    ? 'var(--warn)'
             : 'var(--info)',
        flexShrink: 0,
      }}>
        {ICONS[toast.type]}
      </span>
      <span style={{ color: 'var(--t2)' }}>{toast.message}</span>
    </div>
  );
}

export default function ToastContainer() {
  const { state } = useApp();
  if (state.toasts.length === 0) return null;

  return (
    <div style={{
      position: 'fixed',
      top: 60,
      right: 16,
      zIndex: 9000,
      display: 'flex',
      flexDirection: 'column',
      gap: 8,
      pointerEvents: 'none',
    }}>
      {state.toasts.map(t => (
        <div key={t.id} style={{ pointerEvents: 'all' }}>
          <ToastItem toast={t} />
        </div>
      ))}
    </div>
  );
}
