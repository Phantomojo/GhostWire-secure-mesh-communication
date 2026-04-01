import React, { useState, useEffect, useCallback } from 'react';
import { useApp } from '../../context/AppContext';
import { peers as peersApi, type PeerV2 } from '../../services/api';

const TIER_COLOR: Record<string, string> = {
  stealth:'var(--sig)', webrtc:'var(--info)', covert:'#A064FF', fronted:'var(--warn)', offline:'var(--t3)'
};

function PeerRow({ peer, onConnect, onBlock, onPing }: {
  peer: PeerV2; onConnect:(id:string)=>void; onBlock:(id:string)=>void; onPing:(id:string)=>void;
}) {
  return (
    <div className="ops-row" style={{ flexWrap:'wrap', gap:8 }}>
      <div style={{ width:6,height:6,borderRadius:'50%',background:peer.status==='online'?'var(--sig)':'var(--t4)',flexShrink:0 }} />
      <div style={{ flex:1, minWidth:120 }}>
        <div style={{ fontFamily:'var(--mono)',fontSize:11,color:'var(--t1)',fontWeight:600,overflow:'hidden',textOverflow:'ellipsis',whiteSpace:'nowrap' }}>
          {peer.nickname ?? peer.username ?? peer.node_id.slice(0,16)}
        </div>
        <div style={{ fontFamily:'var(--mono)',fontSize:9,color:'var(--t4)' }}>{peer.node_id.slice(0,24)}...</div>
      </div>
      <div className={`ops-tag ops-tag-${peer.trust_level==='verified'?'sig':peer.trust_level==='blocked'?'crit':'muted'}`} style={{ flexShrink:0 }}>
        {peer.trust_level}
      </div>
      {peer.latency_ms != null && (
        <span style={{ fontFamily:'var(--mono)',fontSize:10,color:peer.latency_ms<100?'var(--sig)':peer.latency_ms<300?'var(--warn)':'var(--crit)',flexShrink:0 }}>
          {peer.latency_ms}ms
        </span>
      )}
      <div style={{ display:'flex',gap:4,flexShrink:0 }}>
        {peer.status !== 'online' && (
          <button className="btn-ops btn-ops-primary" onClick={() => onConnect(peer.node_id)} style={{ padding:'3px 8px',fontSize:10 }}>Connect</button>
        )}
        <button className="btn-ops" onClick={() => onPing(peer.node_id)} style={{ padding:'3px 8px',fontSize:10 }}>Ping</button>
        <button className="btn-ops btn-ops-danger" onClick={() => onBlock(peer.node_id)} style={{ padding:'3px 8px',fontSize:10 }}>Block</button>
      </div>
    </div>
  );
}

export default function NetworkPanel() {
  const { state, refreshMesh, toast } = useApp();
  const [scanning, setScanning] = useState(false);
  const [nearbyPeers, setNearbyPeers] = useState<PeerV2[]>([]);
  const [manualAddr, setManualAddr] = useState('');

  const handleScan = async () => {
    setScanning(true);
    try {
      const nearby = await peersApi.nearby();
      setNearbyPeers(nearby);
      await refreshMesh();
      toast('success', `Scan complete — ${nearby.length} nearby peers`);
    } catch { toast('warn', 'Scan returned no new peers'); }
    finally { setScanning(false); }
  };

  const handleConnect = async (id: string) => {
    try { await peersApi.connect(id); await refreshMesh(); toast('success', 'Connection request sent'); }
    catch (e:any) { toast('error', e.message ?? 'Connect failed'); }
  };

  const handleBlock = async (id: string) => {
    try { await peersApi.block(id); await refreshMesh(); toast('warn', 'Peer blocked'); }
    catch (e:any) { toast('error', e.message ?? 'Block failed'); }
  };

  const handlePing = async (id: string) => {
    try {
      const r = await peersApi.ping(id);
      toast('info', `Ping: ${r.latency_ms}ms`);
    } catch { toast('warn', 'Peer unreachable'); }
  };

  const handleManualConnect = async () => {
    if (!manualAddr.trim()) return;
    try { await peersApi.connect(manualAddr.trim()); setManualAddr(''); toast('success', 'Connection request sent'); }
    catch (e:any) { toast('error', e.message ?? 'Connect failed'); }
  };

  const tier = state.transport.tier;
  const allPeers = [...state.peers, ...nearbyPeers.filter(n => !state.peers.find(p => p.id === n.id))];

  return (
    <div className="panel-scroll animate-in">
      {/* Transport status */}
      <div className="ops-panel" style={{ padding:14,marginBottom:14 }}>
        <div className="ops-section-header"><span className="ops-section-title">Transport layer</span></div>
        <div style={{ display:'flex',gap:8,flexWrap:'wrap',alignItems:'center',marginBottom:12 }}>
          {(['stealth','webrtc','covert','fronted','offline'] as const).map(t => (
            <div key={t} style={{
              display:'flex',alignItems:'center',gap:6,padding:'5px 10px',
              borderRadius:'var(--r-sm)',
              background:tier===t?`${TIER_COLOR[t]}14`:'var(--surface)',
              border:`1px solid ${tier===t?TIER_COLOR[t]:'var(--border)'}`,
              fontFamily:'var(--mono)',fontSize:10,fontWeight:700,
              color:tier===t?TIER_COLOR[t]:'var(--t4)',letterSpacing:'0.1em',textTransform:'uppercase',
            }}>
              <div style={{ width:5,height:5,borderRadius:'50%',background:tier===t?TIER_COLOR[t]:'var(--t4)' }} />
              {t}
            </div>
          ))}
        </div>
        <div style={{ display:'flex',gap:20,flexWrap:'wrap' }}>
          {[
            { label:'Active transport', value:state.transport.display_name || '—' },
            { label:'Internet', value:state.transport.internet_reachable?'REACHABLE':'BLOCKED', ok:state.transport.internet_reachable },
            { label:'Latency', value:state.latencyMs>0?`${state.latencyMs}ms`:'—' },
            { label:'Peers in mesh', value:state.peerCount },
          ].map(item => (
            <div key={item.label}>
              <div className="ops-label" style={{ marginBottom:2 }}>{item.label}</div>
              <div style={{ fontFamily:'var(--mono)',fontSize:13,fontWeight:700,
                color:'ok' in item?(item.ok?'var(--sig)':'var(--warn)'):'var(--t1)' }}>{String(item.value)}</div>
            </div>
          ))}
        </div>
      </div>

      {/* Actions row */}
      <div style={{ display:'flex',gap:8,flexWrap:'wrap',marginBottom:14 }}>
        <button className={`btn-ops ${scanning?'':'btn-ops-primary'}`} onClick={handleScan} disabled={scanning}>
          {scanning ? '⟳ Scanning...' : '⬡ Scan nearby'}
        </button>
        <div style={{ display:'flex',gap:6,flex:1,minWidth:200 }}>
          <input className="ops-input" placeholder="/ip4/x.x.x.x/tcp/4001/p2p/12D3Koo..."
            value={manualAddr} onChange={e => setManualAddr(e.target.value)}
            onKeyDown={e => e.key==='Enter' && handleManualConnect()}
            style={{ flex:1,fontSize:11 }}
          />
          <button className="btn-ops" onClick={handleManualConnect} disabled={!manualAddr.trim()}>
            Connect
          </button>
        </div>
      </div>

      {/* Peer table */}
      <div className="ops-panel" style={{ padding:14 }}>
        <div className="ops-section-header">
          <span className="ops-section-title">Known peers</span>
          <div style={{ flex:1 }} />
          <span className="ops-label">{allPeers.length} total</span>
        </div>
        {allPeers.length === 0 ? (
          <div style={{ padding:'32px 0',textAlign:'center',fontFamily:'var(--mono)',fontSize:11,color:'var(--t4)' }}>
            No peers found<br/>
            <span style={{ color:'var(--t3)',fontSize:10 }}>Run a scan or wait for mDNS discovery</span>
          </div>
        ) : allPeers.map(p => (
          <PeerRow key={p.id} peer={p} onConnect={handleConnect} onBlock={handleBlock} onPing={handlePing} />
        ))}
      </div>

      {/* Routing stats */}
      {state.meshHealth && (
        <div className="ops-panel" style={{ padding:14,marginTop:12 }}>
          <div className="ops-section-header"><span className="ops-section-title">Routing</span></div>
          <div className="metric-grid">
            {[
              { label:'Routing table', value:state.meshHealth.routing.routing_table_size },
              { label:'Sphinx sessions', value:state.meshHealth.routing.sphinx_sessions },
              { label:'Mix queue', value:state.meshHealth.routing.mixing_queue_depth },
              { label:'DTN bundles', value:state.meshHealth.storage.mailbox_pending },
            ].map(m => (
              <div key={m.label} className="metric-card">
                <div className="ops-label" style={{ marginBottom:4 }}>{m.label}</div>
                <div style={{ fontFamily:'var(--mono)',fontSize:20,fontWeight:700,color:'var(--t1)' }}>{m.value}</div>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
