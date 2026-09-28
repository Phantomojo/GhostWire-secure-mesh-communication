import React from 'react';
import { useApp } from '../../context/AppContext';

function Bar({ value, warn=70, crit=90 }: { value:number; warn?:number; crit?:number }) {
  const cls = value >= crit ? 'crit' : value >= warn ? 'warn' : '';
  return (
    <div className="data-bar" style={{ marginTop:6 }}>
      <div className={`data-bar-fill ${cls}`} style={{ width:`${Math.min(value,100)}%` }} />
    </div>
  );
}

function Stat({ label, value, accent }: { label:string; value:string|number; accent?:string }) {
  return (
    <div style={{ display:'flex',justifyContent:'space-between',alignItems:'center',padding:'6px 0',borderBottom:'1px solid var(--border)' }}>
      <span className="ops-label">{label}</span>
      <span style={{ fontFamily:'var(--mono)',fontSize:12,fontWeight:600,color:accent??'var(--t1)' }}>{value}</span>
    </div>
  );
}

function fmt(s: number) {
  const h = Math.floor(s/3600), m = Math.floor((s%3600)/60), sec = s%60;
  return `${h.toString().padStart(2,'0')}:${m.toString().padStart(2,'0')}:${sec.toString().padStart(2,'0')}`;
}

export default function SystemPanel() {
  const { state, refreshMesh } = useApp();
  const h = state.meshHealth;

  return (
    <div className="panel-scroll animate-in">
      <div style={{ display:'flex',justifyContent:'space-between',alignItems:'center',marginBottom:14 }}>
        <div className="ops-label">System telemetry</div>
        <button className="btn-ops btn-ops-primary" onClick={refreshMesh} style={{ padding:'4px 10px',fontSize:10 }}>
          ⟳ Refresh
        </button>
      </div>

      <div className="panel-grid-2" style={{ marginBottom:12 }}>
        {/* CPU — the daemon's /system/stats does not report CPU. Showing
            "0%" would look like a real reading of an idle machine, so render
            "n/a" when the value is absent. */}
        <div className="ops-panel" style={{ padding:14 }}>
          <div className="ops-label" style={{ marginBottom:4 }}>CPU</div>
          <div style={{ fontFamily:'var(--mono)',fontSize:28,fontWeight:700,
            color:state.cpuPct==null?'var(--t4)'
              :state.cpuPct>=90?'var(--crit)':state.cpuPct>=70?'var(--warn)':'var(--t1)' }}>
            {state.cpuPct==null ? 'n/a' : `${state.cpuPct}%`}
          </div>
          {state.cpuPct==null
            ? <div style={{ fontFamily:'var(--mono)',fontSize:9,color:'var(--t4)' }}>
                not reported by daemon
              </div>
            : <Bar value={state.cpuPct} />}
        </div>
        {/* Memory — same: /system/stats reports no memory figures. */}
        <div className="ops-panel" style={{ padding:14 }}>
          <div className="ops-label" style={{ marginBottom:4 }}>Memory</div>
          <div style={{ fontFamily:'var(--mono)',fontSize:28,fontWeight:700,
            color:state.memPct==null?'var(--t4)'
              :state.memPct>=90?'var(--crit)':state.memPct>=70?'var(--warn)':'var(--t1)' }}>
            {state.memPct==null ? 'n/a' : `${state.memPct}%`}
          </div>
          {state.memPct==null
            ? <div style={{ fontFamily:'var(--mono)',fontSize:9,color:'var(--t4)' }}>
                not reported by daemon
              </div>
            : <Bar value={state.memPct} />}
        </div>
      </div>

      {/* System stats */}
      <div className="ops-panel" style={{ padding:14,marginBottom:12 }}>
        <div className="ops-section-header"><span className="ops-section-title">Runtime</span></div>
        <Stat label="Uptime" value={fmt(state.uptimeSecs)} />
        <Stat label="Backend" value={state.backendConnected?'CONNECTED':'DISCONNECTED'} accent={state.backendConnected?'var(--sig)':'var(--crit)'} />
        <Stat label="Transport" value={state.transport.tier.toUpperCase()} />
        <Stat label="Internet" value={state.transport.internet_reachable?'REACHABLE':'BLOCKED'} accent={state.transport.internet_reachable?'var(--sig)':'var(--warn)'} />
        <Stat label="Latency" value={state.latencyMs>0?`${state.latencyMs}ms`:'—'} />
      </div>

      {/* Mesh stats */}
      {h && (
        <div className="ops-panel" style={{ padding:14,marginBottom:12 }}>
          <div className="ops-section-header"><span className="ops-section-title">Mesh</span></div>
          <Stat label="Peers" value={h.peers.length} />
          <Stat label="Sphinx sessions" value={h.routing.sphinx_sessions} />
          <Stat label="Mix queue depth" value={h.routing.mixing_queue_depth} />
          <Stat label="Routing table" value={h.routing.routing_table_size} />
          <Stat label="WS subscribers" value={h.performance.ws_subscriber_count} />
          <Stat label="Dedup cache" value={h.performance.dedup_cache_size} />
        </div>
      )}

      {/* Storage stats */}
      {h && (
        <div className="ops-panel" style={{ padding:14 }}>
          <div className="ops-section-header"><span className="ops-section-title">Storage</span></div>
          <Stat label="Messages" value={h.storage.message_count} />
          <Stat label="Ark entries" value={h.storage.ark_entry_count} />
          <Stat label="DTN bundles pending" value={h.storage.mailbox_pending} accent={h.storage.mailbox_pending>0?'var(--warn)':undefined} />
          <Stat label="Identity locked" value={h.security.identity_locked?'YES':'NO'} accent={h.security.identity_locked?'var(--warn)':'var(--sig)'} />
          <Stat label="Blocked peers" value={h.security.blocked_peers} />
          <Stat label="Failed unlocks 24h" value={h.security.failed_unlock_attempts_24h} accent={h.security.failed_unlock_attempts_24h>0?'var(--crit)':undefined} />
        </div>
      )}
    </div>
  );
}
