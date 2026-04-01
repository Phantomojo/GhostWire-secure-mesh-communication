import MeshVisualizer from '../shared/MeshVisualizer';
import React from 'react';
import { useApp } from '../../context/AppContext';
import { network, peers as peersApi } from '../../services/api';

function StatCell({ label, value, sub, accent }: {
  label: string; value: string | number; sub?: string; accent?: string;
}) {
  return (
    <div className="metric-card">
      <div className="ops-label" style={{ marginBottom: 6 }}>{label}</div>
      <div style={{
        fontFamily: 'var(--mono)',
        fontSize: 22,
        fontWeight: 700,
        color: accent ?? 'var(--t1)',
        lineHeight: 1,
      }}>
        {value}
      </div>
      {sub && (
        <div style={{ fontFamily: 'var(--mono)', fontSize: 10, color: 'var(--t3)', marginTop: 4 }}>
          {sub}
        </div>
      )}

      {/* Live mesh visualizer */}
      <div className="ops-panel" style={{ marginTop:12, height:280, overflow:'hidden', padding:0 }}>
        <div style={{ padding:'8px 14px 6px', borderBottom:'1px solid var(--border)', display:'flex', alignItems:'center', gap:8 }}>
          <span className="ops-section-title">Live mesh</span>
          <div style={{ flex:1 }} />
          <div className="sig-dot sig-dot-pulse" />
        </div>
        <div style={{ height:240 }}>
          <MeshVisualizer
            nodes={state.peers.map(p => ({ id:p.id, name:p.nickname??p.username??p.node_id.slice(0,8), status:p.status, trust_score:p.trust_score, latency_ms:p.latency_ms }))}
            localNickname={state.profile?.nickname ?? 'this node'}
          />
        </div>
      </div>
    </div>
  );
}

function PeerRow({ peer }: { peer: any }) {
  const online = peer.status === 'online' || peer.is_online;
  return (
    <div style={{
      display: 'flex',
      alignItems: 'center',
      gap: 10,
      padding: '7px 0',
      borderBottom: '1px solid var(--border)',
    }}>
      <div style={{
        width: 5, height: 5, borderRadius: '50%', flexShrink: 0,
        background: online ? 'var(--sig)' : 'var(--t4)',
      }} />
      <span style={{
        fontFamily: 'var(--mono)', fontSize: 11,
        color: 'var(--t1)', flex: 1,
        overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap',
      }}>
        {peer.nickname ?? peer.username ?? peer.node_id?.slice(0, 12)}
      </span>
      <span className="ops-label">{peer.trust_level ?? 'unknown'}</span>
      {peer.latency_ms != null && (
        <span style={{ fontFamily: 'var(--mono)', fontSize: 10, color: 'var(--t3)' }}>
          {peer.latency_ms}ms
        </span>
      )}

      {/* Live mesh visualizer */}
      <div className="ops-panel" style={{ marginTop:12, height:280, overflow:'hidden', padding:0 }}>
        <div style={{ padding:'8px 14px 6px', borderBottom:'1px solid var(--border)', display:'flex', alignItems:'center', gap:8 }}>
          <span className="ops-section-title">Live mesh</span>
          <div style={{ flex:1 }} />
          <div className="sig-dot sig-dot-pulse" />
        </div>
        <div style={{ height:240 }}>
          <MeshVisualizer
            nodes={state.peers.map(p => ({ id:p.id, name:p.nickname??p.username??p.node_id.slice(0,8), status:p.status, trust_score:p.trust_score, latency_ms:p.latency_ms }))}
            localNickname={state.profile?.nickname ?? 'this node'}
          />
        </div>
      </div>
    </div>
  );
}

export default function Dashboard() {
  const { state, refreshMesh, toast } = useApp();

  const handleScan = async () => {
    try {
      await peersApi.nearby();
      await refreshMesh();
      toast('success', 'Network scan complete');
    } catch {
      toast('warn', 'Scan returned no new peers');
    }
  };

  const threatColor = {
    none: 'var(--sig)', low: 'var(--sig)',
    medium: 'var(--warn)', high: 'var(--crit)', critical: 'var(--crit)',
  }[state.threatLevel] ?? 'var(--t2)';

  return (
    <div style={{ padding: 20, height: '100%', overflowY: 'auto' }} className="animate-in">

      {/* Top stats row */}
      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(4, 1fr)', gap: 10, marginBottom: 16 }}>
        <StatCell
          label="Peer nodes"
          value={state.peerCount}
          sub={state.backendConnected ? 'mesh active' : 'link lost'}
          accent={state.peerCount > 0 ? 'var(--sig)' : 'var(--t3)'}
        />
        <StatCell
          label="Security"
          value={`${state.securityScore}%`}
          sub={state.threatLevel !== 'none' ? `⚠ ${state.threatLevel} threat` : 'nominal'}
          accent={threatColor}
        />
        <StatCell
          label="Latency"
          value={state.latencyMs > 0 ? `${state.latencyMs}ms` : '—'}
          sub={state.transport.display_name}
          accent={state.latencyMs < 100 ? 'var(--sig)' : state.latencyMs < 300 ? 'var(--warn)' : 'var(--crit)'}
        />
        <StatCell
          label="Messages"
          value={state.meshHealth?.storage.message_count ?? 0}
          sub={`${state.meshHealth?.storage.mailbox_pending ?? 0} pending DTN`}
        />
      </div>

      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 12 }}>
        {/* Left: Network state */}
        <div className="ops-panel" style={{ padding: 14 }}>
          <div className="ops-section-header">
            <span className="ops-section-title">Network state</span>
            <div style={{ flex: 1 }} />
            <button className="btn-ops btn-ops-primary" onClick={handleScan}
              style={{ padding: '3px 8px', fontSize: 10 }}>
              ⬡ Scan
            </button>
          </div>

          <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 8, marginBottom: 12 }}>
            {[
              { label: 'Internet', value: state.meshHealth?.network_state.internet_reachable ? 'REACHABLE' : 'BLOCKED', ok: state.meshHealth?.network_state.internet_reachable },
              { label: 'Transport', value: state.transport.tier.toUpperCase(), ok: state.transport.tier !== 'offline' },
              { label: 'Sphinx sessions', value: state.meshHealth?.routing.sphinx_sessions ?? 0, ok: true },
              { label: 'Mix queue', value: state.meshHealth?.routing.mixing_queue_depth ?? 0, ok: true },
            ].map(item => (
              <div key={item.label} style={{
                padding: '8px 10px',
                background: 'var(--surface)',
                borderRadius: 'var(--r-sm)',
                border: '1px solid var(--border)',
              }}>
                <div className="ops-label" style={{ marginBottom: 3 }}>{item.label}</div>
                <div style={{
                  fontFamily: 'var(--mono)', fontSize: 12, fontWeight: 700,
                  color: item.ok ? 'var(--t1)' : 'var(--warn)',
                }}>
                  {item.value}
                </div>
              </div>
            ))}
          </div>

          {/* Storage metrics */}
          <div className="ops-section-header" style={{ marginTop: 8 }}>
            <span className="ops-section-title">Storage</span>
          </div>
          {[
            { label: 'Messages', value: state.meshHealth?.storage.message_count ?? 0 },
            { label: 'Ark entries', value: state.meshHealth?.storage.ark_entry_count ?? 0 },
            { label: 'DTN bundles', value: state.meshHealth?.storage.mailbox_pending ?? 0 },
          ].map(m => (
            <div key={m.label} style={{
              display: 'flex', justifyContent: 'space-between',
              alignItems: 'center', padding: '4px 0',
              borderBottom: '1px solid var(--border)',
            }}>
              <span className="ops-label">{m.label}</span>
              <span style={{ fontFamily: 'var(--mono)', fontSize: 12, color: 'var(--t2)' }}>
                {m.value}
              </span>
            </div>
          ))}
        </div>

        {/* Right: Peer list */}
        <div className="ops-panel" style={{ padding: 14 }}>
          <div className="ops-section-header">
            <span className="ops-section-title">Known peers</span>
            <div style={{ flex: 1 }} />
            <span className="ops-label">{state.peers.length} total</span>
          </div>

          {state.peers.length === 0 ? (
            <div style={{
              padding: '24px 0', textAlign: 'center',
              fontFamily: 'var(--mono)', fontSize: 11, color: 'var(--t4)',
            }}>
              No peers in routing table<br />
              <span style={{ color: 'var(--t3)' }}>Run scan to discover nearby nodes</span>
            </div>
          ) : (
            <div style={{ maxHeight: 220, overflowY: 'auto' }}>
              {state.peers.slice(0, 20).map(p => (
                <PeerRow key={p.id} peer={p} />
              ))}
            </div>
          )}
        </div>
      </div>

      {/* Security panel quick view */}
      {state.meshHealth && (
        <div className="ops-panel" style={{ padding: 14, marginTop: 12 }}>
          <div className="ops-section-header">
            <span className="ops-section-title">Security snapshot</span>
          </div>
          <div style={{ display: 'flex', gap: 20, flexWrap: 'wrap' }}>
            {[
              { label: 'Identity', value: state.meshHealth.security.identity_locked ? 'LOCKED' : 'UNLOCKED', ok: state.meshHealth.security.identity_locked },
              { label: 'Blocked peers', value: state.meshHealth.security.blocked_peers },
              { label: 'Failed unlocks (24h)', value: state.meshHealth.security.failed_unlock_attempts_24h, warn: true },
              { label: 'PQC armor', value: state.pqcEnabled ? 'ACTIVE' : 'OFF', ok: state.pqcEnabled },
            ].map(s => (
              <div key={s.label}>
                <div className="ops-label" style={{ marginBottom: 3 }}>{s.label}</div>
                <div style={{
                  fontFamily: 'var(--mono)', fontSize: 13, fontWeight: 700,
                  color: s.ok !== undefined
                    ? (s.ok ? 'var(--sig)' : 'var(--crit)')
                    : (s.warn && Number(s.value) > 0 ? 'var(--warn)' : 'var(--t1)'),
                }}>
                  {String(s.value)}
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Live mesh visualizer */}
      <div className="ops-panel" style={{ marginTop:12, height:280, overflow:'hidden', padding:0 }}>
        <div style={{ padding:'8px 14px 6px', borderBottom:'1px solid var(--border)', display:'flex', alignItems:'center', gap:8 }}>
          <span className="ops-section-title">Live mesh</span>
          <div style={{ flex:1 }} />
          <div className="sig-dot sig-dot-pulse" />
        </div>
        <div style={{ height:240 }}>
          <MeshVisualizer
            nodes={state.peers.map(p => ({ id:p.id, name:p.nickname??p.username??p.node_id.slice(0,8), status:p.status, trust_score:p.trust_score, latency_ms:p.latency_ms }))}
            localNickname={state.profile?.nickname ?? 'this node'}
          />
        </div>
      </div>
    </div>
  );
}
