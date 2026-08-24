import React, { useEffect, useMemo, useState } from 'react';
import { useApp } from '../../context/AppContext';
import {
  network,
  peers as peersApi,
  type DiscoverySnapshotV2,
  type PeerV2,
  type TransportRegistryDescriptor,
  type RuntimeVersionV2,
} from '../../services/api';

const TIER_COLOR: Record<string, string> = {
  stealth: 'var(--sig)',
  webrtc: 'var(--info)',
  covert: '#A064FF',
  fronted: 'var(--warn)',
  offline: 'var(--t3)',
};

type View = 'overview' | 'nearby' | 'discovered' | 'connected' | 'diagnostics';

type Candidate = DiscoverySnapshotV2['candidates'][number];

type MethodState = {
  state: string;
  detail: string;
  tone?: 'good' | 'warn' | 'bad' | 'muted';
};

function toneForState(state: string): MethodState['tone'] {
  const normalized = state.toLowerCase();
  if (['healthy', 'ready', 'available', 'completed', 'provider_active', 'connected'].some(v => normalized.includes(v))) return 'good';
  if (['starting', 'cached', 'not_provider', 'disabled', 'unavailable', 'platform'].some(v => normalized.includes(v))) return 'muted';
  if (['rejected', 'failed', 'error', 'degraded'].some(v => normalized.includes(v))) return 'bad';
  return 'warn';
}

function stateColor(tone: MethodState['tone']): string {
  if (tone === 'good') return 'var(--sig)';
  if (tone === 'bad') return 'var(--crit)';
  if (tone === 'warn') return 'var(--warn)';
  return 'var(--t4)';
}

function formatLastSeen(value: number): string {
  if (!value) return 'never';
  const seconds = Math.max(0, Math.floor(Date.now() / 1000) - value);
  if (seconds < 5) return 'just now';
  if (seconds < 60) return `${seconds}s ago`;
  if (seconds < 3600) return `${Math.floor(seconds / 60)}m ago`;
  return `${Math.floor(seconds / 3600)}h ago`;
}

function MethodCard({ label, state, detail }: { label: string; state: string; detail: string }) {
  const tone = toneForState(state);
  return (
    <div className="metric-card" style={{ minWidth: 160, flex: '1 1 160px' }}>
      <div className="ops-label" style={{ marginBottom: 6 }}>{label}</div>
      <div style={{ display: 'flex', alignItems: 'center', gap: 7, marginBottom: 6 }}>
        <span style={{ width: 7, height: 7, borderRadius: '50%', background: stateColor(tone), boxShadow: tone === 'good' ? `0 0 8px ${stateColor(tone)}` : undefined }} />
        <span style={{ fontFamily: 'var(--mono)', fontSize: 11, color: stateColor(tone), fontWeight: 700, textTransform: 'uppercase' }}>
          {state.replaceAll('_', ' ')}
        </span>
      </div>
      <div style={{ fontFamily: 'var(--mono)', fontSize: 9, color: 'var(--t4)', lineHeight: 1.45 }}>{detail}</div>
    </div>
  );
}

function DiscoveryMeter({ label, value, max = 100, tone = 'var(--sig)' }: { label: string; value: number; max?: number; tone?: string }) {
  const width = Math.min(100, max <= 0 ? 0 : (value / max) * 100);
  return (
    <div style={{ minWidth: 130, flex: 1 }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', gap: 8, marginBottom: 4 }}>
        <span className="ops-label">{label}</span>
        <span style={{ fontFamily: 'var(--mono)', fontSize: 10, color: 'var(--t1)' }}>{value}</span>
      </div>
      <div style={{ height: 4, background: 'var(--surface)', borderRadius: 99, overflow: 'hidden' }}>
        <div style={{ width: `${width}%`, height: '100%', background: tone, borderRadius: 99, transition: 'width .3s ease' }} />
      </div>
    </div>
  );
}

function PeerRow({ peer, onConnect, onBlock, onPing }: {
  peer: PeerV2;
  onConnect: (id: string) => void;
  onBlock: (id: string) => void;
  onPing: (id: string) => void;
}) {
  const displayName = peer.nickname && !peer.nickname.startsWith('rust-libp2p/')
    ? peer.nickname
    : 'Unverified GhostWire peer';
  return (
    <div className="ops-row" style={{ flexWrap: 'wrap', gap: 8 }}>
      <div style={{ width: 6, height: 6, borderRadius: '50%', background: peer.status === 'online' ? 'var(--sig)' : 'var(--t4)', flexShrink: 0 }} />
      <div style={{ flex: 1, minWidth: 150 }}>
        <div style={{ fontFamily: 'var(--mono)', fontSize: 11, color: 'var(--t1)', fontWeight: 600, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
          {displayName}
        </div>
        <div style={{ fontFamily: 'var(--mono)', fontSize: 9, color: 'var(--t4)' }}>{peer.node_id.slice(0, 28)}...</div>
      </div>
      <div className={`ops-tag ops-tag-${peer.trust_level === 'verified' ? 'sig' : peer.trust_level === 'blocked' ? 'crit' : 'muted'}`} style={{ flexShrink: 0 }}>
        {peer.trust_level}
      </div>
      {peer.discovery_method && <span className="ops-tag ops-tag-muted" style={{ flexShrink: 0 }}>{peer.discovery_method}</span>}
      {peer.latency_ms != null && (
        <span style={{ fontFamily: 'var(--mono)', fontSize: 10, color: peer.latency_ms < 100 ? 'var(--sig)' : peer.latency_ms < 300 ? 'var(--warn)' : 'var(--crit)', flexShrink: 0 }}>
          {peer.latency_ms}ms
        </span>
      )}
      <div style={{ display: 'flex', gap: 4, flexShrink: 0 }}>
        {peer.status !== 'online' && <button className="btn-ops btn-ops-primary" onClick={() => onConnect(peer.node_id)} style={{ padding: '3px 8px', fontSize: 10 }}>Connect</button>}
        <button className="btn-ops" onClick={() => onPing(peer.node_id)} style={{ padding: '3px 8px', fontSize: 10 }}>Ping</button>
        <button className="btn-ops btn-ops-danger" onClick={() => onBlock(peer.node_id)} style={{ padding: '3px 8px', fontSize: 10 }}>Block</button>
      </div>
    </div>
  );
}

function CandidateRow({ candidate, onConnect }: { candidate: Candidate; onConnect: (id: string) => void }) {
  return (
    <div className="ops-row" style={{ alignItems: 'flex-start', gap: 10, flexWrap: 'wrap' }}>
      <div style={{ width: 7, height: 7, borderRadius: '50%', background: candidate.is_online ? 'var(--sig)' : 'var(--t4)', marginTop: 4 }} />
      <div style={{ flex: 1, minWidth: 220 }}>
        <div style={{ fontFamily: 'var(--mono)', fontSize: 11, color: 'var(--t1)', fontWeight: 700 }}>
          {candidate.nickname || candidate.node_id.slice(0, 18)}
        </div>
        <div style={{ fontFamily: 'var(--mono)', fontSize: 9, color: 'var(--t4)', marginTop: 3, wordBreak: 'break-all' }}>{candidate.node_id}</div>
        <div style={{ display: 'flex', gap: 7, flexWrap: 'wrap', marginTop: 7 }}>
          <span className="ops-tag ops-tag-muted">source: {candidate.source}</span>
          <span className="ops-tag ops-tag-muted">via: {candidate.discovered_via}</span>
          <span className="ops-tag ops-tag-muted">seen: {formatLastSeen(candidate.last_seen)}</span>
        </div>
      </div>
      <button className="btn-ops btn-ops-primary" onClick={() => onConnect(candidate.node_id)} style={{ padding: '4px 10px', fontSize: 10 }}>Connect</button>
    </div>
  );
}

export default function NetworkPanel() {
  const { state, refreshMesh, toast } = useApp();
  const [view, setView] = useState<View>('overview');
  const [scanning, setScanning] = useState(false);
  const [snapshot, setSnapshot] = useState<DiscoverySnapshotV2 | null>(null);
  const [nearbyPeers, setNearbyPeers] = useState<PeerV2[]>([]);
  const [manualAddr, setManualAddr] = useState('');
  const [discoveryBusy, setDiscoveryBusy] = useState<'publish' | 'refresh' | null>(null);
  const [lastScanId, setLastScanId] = useState<string | null>(null);
  const [registry, setRegistry] = useState<TransportRegistryDescriptor[]>([]);
  const [runtimeVersion, setRuntimeVersion] = useState<RuntimeVersionV2 | null>(null);
  const [transportBusy, setTransportBusy] = useState<string | null>(null);

  const loadSnapshot = async () => {
    try {
      const next = await network.discovery.snapshot();
      setSnapshot(next);
      return next;
    } catch {
      return null;
    }
  };

  const loadRuntimeVersion = async () => {
    try {
      setRuntimeVersion(await network.version());
    } catch {
      setRuntimeVersion(null);
    }
  };
  const loadTransportRegistry = async () => {
    try {
      const result = await network.transports();
      setRegistry(result.registry ?? []);
    } catch {
      setRegistry([]);
    }
  };
  useEffect(() => {
    void loadSnapshot();
    void loadTransportRegistry();
    void loadRuntimeVersion();
  }, []);

  const handleScan = async () => {
    setScanning(true);
    try {
      const result = await network.discovery.scan();
      setLastScanId(result.scan_id);
      await Promise.all([loadSnapshot(), loadTransportRegistry(), loadRuntimeVersion(), refreshMesh()]);
      toast('success', `Discovery cycle complete — ${result.scan_id.slice(0, 8)}`);
    } catch (e: any) {
      toast('warn', e?.message ?? 'Discovery cycle rejected');
    } finally {
      setScanning(false);
    }
  };

  const handleDiscoveryAction = async (action: 'publish' | 'refresh') => {
    setDiscoveryBusy(action);
    try {
      if (action === 'publish') await network.discovery.publish();
      else await network.discovery.refresh();
      await Promise.all([loadSnapshot(), loadTransportRegistry(), loadRuntimeVersion(), refreshMesh()]);
      toast('success', action === 'publish' ? 'Signed presence published' : 'Provider refresh requested');
    } catch (e: any) {
      toast('warn', e?.message ?? 'Discovery action was rejected');
    } finally {
      setDiscoveryBusy(null);
    }
  };

  const handleConnect = async (id: string) => {
    try {
      await peersApi.connect(id);
      await Promise.all([loadSnapshot(), loadTransportRegistry(), loadRuntimeVersion(), refreshMesh()]);
      toast('success', 'Connection request sent');
    } catch (e: any) {
      toast('error', e?.message ?? 'Connect failed');
    }
  };

  const handleBlock = async (id: string) => {
    try {
      await peersApi.block(id);
      await Promise.all([loadSnapshot(), loadTransportRegistry(), loadRuntimeVersion(), refreshMesh()]);
      toast('warn', 'Peer blocked');
    } catch (e: any) {
      toast('error', e?.message ?? 'Block failed');
    }
  };

  const handlePing = async (id: string) => {
    try {
      const result = await peersApi.ping(id);
      toast('info', `Ping: ${result.latency_ms}ms`);
    } catch {
      toast('warn', 'Peer unreachable');
    }
  };

  const handleManualConnect = async () => {
    if (!manualAddr.trim()) return;
    await handleConnect(manualAddr.trim());
    setManualAddr('');
  };

  const handleTransportAction = async (transport: string, action: 'start' | 'stop') => {
    setTransportBusy(transport);
    try {
      const result = await network.transportAction(transport, action);
      setRegistry(result.registry ?? []);
      await refreshMesh();
      toast('success', `${transport} ${action} requested`);
    } catch (e: any) {
      toast('warn', e?.message ?? `${transport} ${action} rejected`);
    } finally {
      setTransportBusy(null);
    }
  };

  const tier = state.transport.tier;
  const world = state.meshHealth?.world_discovery;
  const methods = snapshot?.methods;
  const connectedCount = snapshot?.counts.connected ?? state.peerCount;
  const knownCount = snapshot?.counts.known ?? state.peers.length;
  const allPeers = [...state.peers, ...nearbyPeers.filter(n => !state.peers.find(p => p.id === n.id))];
  const livePeers = allPeers.filter(peer => peer.status === 'online');
  const interfaces = state.meshHealth?.interfaces ?? [];

  const discoveredCandidates = useMemo(() => snapshot?.candidates ?? [], [snapshot]);
  const connectedCandidates = useMemo(() => snapshot?.connected_peers ?? [], [snapshot]);

  const tabs: Array<{ id: View; label: string; count?: number }> = [
    { id: 'overview', label: 'Overview' },
    { id: 'nearby', label: 'Nearby', count: snapshot?.counts.nearby },
    { id: 'discovered', label: 'Discovered', count: snapshot?.counts.world_candidates },
    { id: 'connected', label: 'Connected', count: connectedCount },
    { id: 'diagnostics', label: 'Diagnostics' },
  ];

  return (
    <div className="panel-scroll animate-in">
      <div className="ops-panel" style={{ padding: 14, marginBottom: 14 }}>
        <div className="ops-section-header">
          <div>
            <div className="ops-section-title">Discovery cockpit</div>
            <div style={{ fontFamily: 'var(--mono)', fontSize: 9, color: 'var(--t4)', marginTop: 4 }}>
              Evidence, candidates, connections, and trust are deliberately separate.
            </div>
          </div>
          <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
            <span className="ops-label">{knownCount} known / {connectedCount} connected</span>
            <button className="btn-ops btn-ops-primary" onClick={handleScan} disabled={scanning}>
              {scanning ? 'Scanning...' : 'Scan now'}
            </button>
          </div>
        </div>
        <div style={{ display: 'flex', gap: 6, flexWrap: 'wrap', marginTop: 14, borderBottom: '1px solid var(--border)', paddingBottom: 10 }}>
          {tabs.map(tab => (
            <button key={tab.id} className={`btn-ops ${view === tab.id ? 'btn-ops-primary' : ''}`} onClick={() => setView(tab.id)} style={{ padding: '5px 9px', fontSize: 10 }}>
              {tab.label}{tab.count != null ? ` · ${tab.count}` : ''}
            </button>
          ))}
        </div>
        <div style={{ display: 'flex', gap: 16, flexWrap: 'wrap', marginTop: 12, fontFamily: 'var(--mono)', fontSize: 9, color: 'var(--t4)' }}>
          <span>Local PeerID: {snapshot?.local.peer_id ? `${snapshot.local.peer_id.slice(0, 16)}...` : 'loading'}</span>
          <span>Listener: {snapshot?.local.listener_state ?? 'unknown'}</span>
          <span>Last scan: {lastScanId ? lastScanId.slice(0, 12) : 'not run in this session'}</span>
        </div>
      </div>

      {view === 'overview' && (
        <>
          <div className="ops-panel" style={{ padding: 14, marginBottom: 14 }}>
            <div className="ops-section-header"><span className="ops-section-title">What GhostWire is doing</span></div>
            <div style={{ display: 'flex', gap: 8, flexWrap: 'wrap' }}>
              <MethodCard label="Nearby" state={methods?.nearby.state ?? 'unknown'} detail={methods?.nearby.note ?? 'No snapshot yet'} />
              <MethodCard label="World Gossip" state={methods?.world_gossip.state ?? 'unknown'} detail={`${methods?.world_gossip.candidates ?? world?.candidate_count ?? 0} candidates · ${methods?.world_gossip.accepted ?? world?.accepted_total ?? 0} accepted`} />
              <MethodCard label="Kademlia" state={methods?.kademlia.state ?? 'unknown'} detail={methods?.kademlia.provider_active ? 'Provider authorized and active' : 'Evidence lookup/client state only'} />
              <MethodCard label="Live topology" state={snapshot?.local.listener_state ?? 'unknown'} detail={`${connectedCount} currently connected`} />
            </div>
          </div>

          <div className="ops-panel" style={{ padding: 14, marginBottom: 14 }}>
            <div className="ops-section-header">
              <div>
                <span className="ops-section-title">Transport control panel</span>
                <div style={{ fontFamily: 'var(--mono)', fontSize: 9, color: 'var(--t4)', marginTop: 4 }}>Ready, registered, and carrying are separate facts.</div>
              </div>
              <span className="ops-label">{livePeers.length} live peer sessions</span>
            </div>
            <div style={{ display: 'flex', gap: 20, flexWrap: 'wrap', margin: '12px 0' }}>
              {[
                { label: 'Preferred route', value: state.transport.display_name || 'not selected' },
                { label: 'Internet', value: state.transport.internet_reachable ? 'REACHABLE' : 'BLOCKED', ok: state.transport.internet_reachable },
                { label: 'Observed latency', value: state.latencyMs > 0 ? `${state.latencyMs}ms` : '—' },
                { label: 'Connected peers', value: connectedCount },
              ].map(item => (
                <div key={item.label}><div className="ops-label" style={{ marginBottom: 2 }}>{item.label}</div><div style={{ fontFamily: 'var(--mono)', fontSize: 13, fontWeight: 700, color: 'ok' in item ? (item.ok ? 'var(--sig)' : 'var(--warn)') : 'var(--t1)' }}>{String(item.value)}</div></div>
              ))}
            </div>
            {interfaces.length === 0 ? (
              <div style={{ padding: '14px 0', color: 'var(--t4)', fontFamily: 'var(--mono)', fontSize: 10 }}>No per-interface health returned.</div>
            ) : (
              <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(220px, 1fr))', gap: 8 }}>
                {interfaces.map(iface => (
                  <div key={iface.id} className="metric-card" style={{ minHeight: 122 }}>
                    <div style={{ display: 'flex', alignItems: 'center', gap: 7 }}>
                      <span className="sig-dot" style={{ background: iface.ready ? 'var(--sig)' : iface.started ? 'var(--warn)' : 'var(--t4)' }} />
                      <span style={{ fontFamily: 'var(--mono)', fontSize: 11, fontWeight: 700, color: 'var(--t1)' }}>{iface.id}</span>
                      <span className="ops-tag ops-tag-muted" style={{ marginLeft: 'auto', fontSize: 8 }}>{iface.state}</span>
                    </div>
                    <div style={{ marginTop: 8, display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 5, fontFamily: 'var(--mono)', fontSize: 9, color: 'var(--t4)' }}>
                      <span>protocol: <b style={{ color: 'var(--t2)' }}>{iface.protocol}</b></span>
                      <span>medium: <b style={{ color: 'var(--t2)' }}>{iface.medium}</b></span>
                      <span>registered: {iface.registered ? 'yes' : 'no'}</span>
                      <span>started: {iface.started ? 'yes' : 'no'}</span>
                      <span>ready: {iface.ready ? 'yes' : 'no'}</span>
                      <span>peers: {iface.peer_count}</span>
                    </div>
                    <div style={{ marginTop: 8, fontFamily: 'var(--mono)', fontSize: 9, color: iface.last_error ? 'var(--crit)' : 'var(--t4)', overflowWrap: 'anywhere' }}>
                      {iface.last_error || (iface.listen_addresses.length ? iface.listen_addresses.join(', ') : 'No listener address reported')}
                    </div>
                    <div style={{ display: 'flex', alignItems: 'center', gap: 7, marginTop: 10 }}>
                      {iface.id === 'reticulum' ? (
                        <button className={`btn-ops ${iface.started ? 'btn-ops-danger' : 'btn-ops-primary'}`} disabled={transportBusy !== null} onClick={() => void handleTransportAction('reticulum', iface.started ? 'stop' : 'start')} style={{ padding: '4px 9px', fontSize: 9 }}>
                          {transportBusy === 'reticulum' ? 'Working...' : iface.started ? 'Stop RNS' : 'Start RNS'}
                        </button>
                      ) : (
                        <span className="ops-label">observe-only: lifecycle action not wired</span>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>

          <div className="ops-panel" style={{ padding: 14, marginBottom: 14 }}>
            <div className="ops-section-header">
              <span className="ops-section-title">Transport registry</span>
              <span className="ops-label">registration is not readiness</span>
            </div>
            {registry.length === 0 ? (
              <div style={{ padding: '18px 0', color: 'var(--t4)', fontFamily: 'var(--mono)', fontSize: 10 }}>
                No registry descriptors returned. Runtime health remains authoritative.
              </div>
            ) : (
              <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(190px, 1fr))', gap: 8, marginTop: 10 }}>
                {registry.map(descriptor => (
                  <div key={descriptor.name} className="metric-card" style={{ minHeight: 70 }}>
                    <div style={{ display: 'flex', alignItems: 'center', gap: 7 }}>
                      <span className="sig-dot" style={{ background: descriptor.registered ? 'var(--sig)' : 'var(--t4)' }} />
                      <span style={{ fontFamily: 'var(--mono)', fontSize: 11, fontWeight: 700, color: 'var(--t1)' }}>{descriptor.name}</span>
                      <span className="ops-tag ops-tag-muted" style={{ marginLeft: 'auto', fontSize: 8 }}>{descriptor.registered ? 'registered' : 'unavailable'}</span>
                    </div>
                    <div style={{ marginTop: 7, color: 'var(--t4)', fontFamily: 'var(--mono)', fontSize: 9, lineHeight: 1.45 }}>{descriptor.description}</div>
                  </div>
                ))}
              </div>
            )}
          </div>
          <div className="ops-panel" style={{ padding: 14 }}>
            <div className="ops-section-header"><span className="ops-section-title">Discovery counters</span></div>
            <div style={{ display: 'flex', gap: 12, flexWrap: 'wrap' }}>
              <DiscoveryMeter label="Published" value={world?.published_total ?? methods?.world_gossip.published ?? 0} max={Math.max(world?.published_total ?? 0, 1)} />
              <DiscoveryMeter label="Accepted" value={world?.accepted_total ?? methods?.world_gossip.accepted ?? 0} max={Math.max((world?.accepted_total ?? 0) + (world?.rejected_total ?? 0), 1)} />
              <DiscoveryMeter label="Rejected" value={world?.rejected_total ?? methods?.world_gossip.rejected ?? 0} max={Math.max((world?.accepted_total ?? 0) + (world?.rejected_total ?? 0), 1)} tone="var(--warn)" />
              <DiscoveryMeter label="Expired" value={world?.expired_total ?? 0} max={Math.max(world?.expired_total ?? 0, 1)} tone="var(--t4)" />
            </div>
          </div>
        </>
      )}

      {view === 'nearby' && (
        <div className="ops-panel" style={{ padding: 14 }}>
          <div className="ops-section-header"><span className="ops-section-title">Nearby evidence</span><span className="ops-label">cached social-graph results</span></div>
          <div style={{ padding: '8px 0 14px', fontFamily: 'var(--mono)', fontSize: 10, color: 'var(--t4)' }}>
            Scan now runs the supported discovery refresh cycle. Nearby results are shown with provenance; an empty list does not prove that the LAN is empty.
          </div>
          {discoveredCandidates.length === 0 ? <div style={{ padding: '32px 0', textAlign: 'center', fontFamily: 'var(--mono)', fontSize: 11, color: 'var(--t4)' }}>No nearby evidence recorded</div> : discoveredCandidates.map(candidate => <CandidateRow key={candidate.node_id} candidate={candidate} onConnect={handleConnect} />)}
        </div>
      )}

      {view === 'discovered' && (
        <div className="ops-panel" style={{ padding: 14 }}>
          <div className="ops-section-header"><span className="ops-section-title">World-discovered candidates</span><span className="ops-label">evidence is not trust</span></div>
          <div style={{ display: 'flex', gap: 8, margin: '10px 0 14px', flexWrap: 'wrap' }}>
            <button className="btn-ops btn-ops-primary" onClick={() => handleDiscoveryAction('publish')} disabled={discoveryBusy !== null || !world?.enabled}>{discoveryBusy === 'publish' ? 'Publishing...' : 'Publish signed presence'}</button>
            <button className="btn-ops" onClick={() => handleDiscoveryAction('refresh')} disabled={discoveryBusy !== null || !world?.enabled}>{discoveryBusy === 'refresh' ? 'Refreshing...' : 'Refresh providers'}</button>
          </div>
          <div style={{ padding: '12px 0', fontFamily: 'var(--mono)', fontSize: 10, color: 'var(--t4)' }}>World candidates: {snapshot?.counts.world_candidates ?? world?.candidate_count ?? 0}. Every candidate must still be verified, connected, and trusted separately.</div>
          {snapshot?.counts.world_candidates === 0 && <div style={{ padding: '28px 0', textAlign: 'center', fontFamily: 'var(--mono)', fontSize: 11, color: 'var(--t4)' }}>No world-discovery candidates recorded</div>}
        </div>
      )}

      {view === 'connected' && (
        <div className="ops-panel" style={{ padding: 14 }}>
          <div className="ops-section-header"><span className="ops-section-title">Connected peers</span><span className="ops-label">live topology</span></div>
          {livePeers.length === 0 && connectedCandidates.length === 0 ? <div style={{ padding: '32px 0', textAlign: 'center', fontFamily: 'var(--mono)', fontSize: 11, color: 'var(--t4)' }}>No live connections</div> : livePeers.length > 0 ? (
            livePeers.map(peer => <PeerRow key={peer.id} peer={peer} onConnect={handleConnect} onBlock={handleBlock} onPing={handlePing} />)
          ) : (
            connectedCandidates.map(peer => <div key={peer.node_id} className="ops-row" style={{ flexWrap: 'wrap', gap: 8 }}><span style={{ width: 7, height: 7, borderRadius: '50%', background: 'var(--sig)' }} /><div style={{ flex: 1, minWidth: 180 }}><div style={{ fontFamily: 'var(--mono)', fontSize: 11, color: 'var(--t1)' }}>{peer.nickname || 'Unverified GhostWire peer'}</div><div style={{ fontFamily: 'var(--mono)', fontSize: 9, color: 'var(--t4)' }}>{peer.node_id}</div></div><span className="ops-tag ops-tag-sig">connected</span><span className="ops-tag ops-tag-muted">{peer.source}</span><span className="ops-label">seen {formatLastSeen(peer.last_seen)}</span></div>)
          )}
        </div>
      )}

      {view === 'diagnostics' && (
        <div className="ops-panel" style={{ padding: 14 }}>
          <div className="ops-section-header"><span className="ops-section-title">Discovery diagnostics</span><span className="ops-label">why was or was not a node found?</span></div>
          <div style={{ display: 'flex', gap: 8, flexWrap: 'wrap', marginBottom: 14 }}>
            <MethodCard label="Nearby query" state={methods?.nearby.state ?? 'unknown'} detail={methods?.nearby.note ?? 'Waiting for snapshot'} />
            <MethodCard label="GossipSub" state={methods?.world_gossip.state ?? 'unknown'} detail={methods?.world_gossip.last_error || 'No error reported'} />
            <MethodCard label="Kademlia" state={methods?.kademlia.state ?? 'unknown'} detail={methods?.kademlia.provider_active ? 'Provider active' : 'Provider not active'} />
          </div>
          <div style={{ display: 'flex', gap: 8, flexWrap: 'wrap', alignItems: 'center', marginBottom: 14 }}>
            <button className="btn-ops btn-ops-primary" onClick={handleScan} disabled={scanning}>{scanning ? 'Running cycle...' : 'Run discovery cycle'}</button>
            <button className="btn-ops" onClick={() => void loadSnapshot()}>Reload evidence</button>
          </div>
          <div style={{ fontFamily: 'var(--mono)', fontSize: 10, color: 'var(--t4)', lineHeight: 1.7 }}>
            <div>Listener: {snapshot?.local.listener_state ?? 'unknown'}</div>
            <div>Network scope: {world?.network_id ?? methods?.world_gossip.state ?? 'unknown'}</div>
            <div>Last error: {world?.last_error || methods?.world_gossip.last_error || 'none reported'}</div>
            <div>Scan semantics: the current backend refreshes signed world presence/provider state and reports cached nearby evidence. Transport-specific radio scans remain explicit capabilities.</div>
          </div>
        </div>
      )}

      <div className="ops-panel" style={{ padding: 14, marginTop: 12 }}>
        <div className="ops-section-header"><span className="ops-section-title">Advanced connect</span><span className="ops-label">manual fallback</span></div>
        <div style={{ display: 'flex', gap: 6, flexWrap: 'wrap' }}>
          <input className="ops-input" placeholder="/ip4/x.x.x.x/tcp/4821/p2p/12D3Koo..." value={manualAddr} onChange={e => setManualAddr(e.target.value)} onKeyDown={e => e.key === 'Enter' && void handleManualConnect()} style={{ flex: 1, minWidth: 240, fontSize: 11 }} />
          <button className="btn-ops" onClick={() => void handleManualConnect()} disabled={!manualAddr.trim()}>Connect</button>
        </div>
      </div>

      {state.meshHealth && (
        <div className="ops-panel" style={{ padding: 14, marginTop: 12 }}>
          <div className="ops-section-header"><span className="ops-section-title">Routing and storage</span></div>
          <div className="metric-grid">
            {[
              { label: 'Routing table', value: state.meshHealth.routing.routing_table_size },
              { label: 'Sphinx sessions', value: state.meshHealth.routing.sphinx_sessions },
              { label: 'Mix queue', value: state.meshHealth.routing.mixing_queue_depth },
              { label: 'DTN bundles', value: state.meshHealth.storage.mailbox_pending },
            ].map(metric => <div key={metric.label} className="metric-card"><div className="ops-label" style={{ marginBottom: 4 }}>{metric.label}</div><div style={{ fontFamily: 'var(--mono)', fontSize: 20, fontWeight: 700, color: 'var(--t1)' }}>{metric.value}</div></div>)}
          </div>
        </div>
      )}
    </div>
  );
}
