import React, { useState } from 'react';
import { useApp } from '../../context/AppContext';
import { profile as profileApi } from '../../services/api';

export default function UserProfile() {
  const { state, dispatch, toast, refreshIdentity } = useApp();
  const [editing, setEditing] = useState(false);
  const [nick, setNick] = useState(state.profile?.nickname ?? '');
  const [saving, setSaving] = useState(false);

  const p = state.profile;

  const save = async () => {
    setSaving(true);
    try {
      await profileApi.update({ nickname: nick });
      await refreshIdentity();
      setEditing(false);
      toast('success', 'Callsign updated');
    } catch { toast('error', 'Save failed'); }
    finally { setSaving(false); }
  };

  const score = state.securityScore;
  const scoreCol = score >= 80 ? 'var(--sig)' : score >= 60 ? 'var(--warn)' : 'var(--crit)';

  return (
    <div style={{
      padding:'10px 12px',
      borderTop:'1px solid var(--border)',
      background:'var(--surface)',
    }}>
      {/* Identity line */}
      <div style={{ display:'flex', alignItems:'center', gap:8, marginBottom:6 }}>
        <div style={{
          width:28, height:28, borderRadius:'var(--r-sm)',
          background:'var(--sig-ghost)', border:'1px solid var(--sig-dim)',
          display:'flex', alignItems:'center', justifyContent:'center',
          fontFamily:'var(--mono)', fontSize:12, fontWeight:700, color:'var(--sig)',
          flexShrink:0,
        }}>
          {(p?.nickname ?? p?.username ?? 'GW')[0].toUpperCase()}
        </div>

        {editing ? (
          <input
            className="ops-input"
            value={nick}
            onChange={e => setNick(e.target.value)}
            onKeyDown={e => { if(e.key==='Enter') save(); if(e.key==='Escape') setEditing(false); }}
            style={{ flex:1, fontSize:11, padding:'3px 7px' }}
            autoFocus
          />
        ) : (
          <div style={{ flex:1, minWidth:0 }}>
            <div style={{
              fontFamily:'var(--mono)', fontSize:11, fontWeight:700,
              color:'var(--t1)', overflow:'hidden', textOverflow:'ellipsis', whiteSpace:'nowrap',
            }}>
              {p?.nickname ?? p?.username ?? 'GHOST'}
            </div>
            <div style={{ fontFamily:'var(--mono)', fontSize:9, color:'var(--t3)' }}>
              <span style={{ color:scoreCol }}>●</span> SEC {score}
            </div>
          </div>
        )}

        <button
          onClick={() => editing ? save() : setEditing(true)}
          disabled={saving}
          style={{
            background:'none', border:'1px solid var(--border)',
            borderRadius:'var(--r-sm)', color:'var(--t3)',
            cursor:'pointer', width:22, height:22,
            display:'flex', alignItems:'center', justifyContent:'center',
            fontSize:10, flexShrink:0, transition:'all 0.12s',
          }}
          title={editing ? 'Save' : 'Edit callsign'}
        >
          {saving ? '⟳' : editing ? '✓' : '✎'}
        </button>
      </div>

      {/* DID */}
      {p?.did && (
        <div style={{
          fontFamily:'var(--mono)', fontSize:8, color:'var(--t4)',
          overflow:'hidden', textOverflow:'ellipsis', whiteSpace:'nowrap',
          cursor:'pointer',
        }}
          title={p.did}
          onClick={() => { navigator.clipboard?.writeText(p.did); toast('success','DID copied'); }}
        >
          {p.did.replace('did:ghost:','did:ghost:').slice(0,32)}…
        </div>
      )}
    </div>
  );
}
