import React from 'react';
import MeshVisualizer from './MeshVisualizer';
import { GlassCard, SectionTitle, Subtitle, Badge, ActionButton } from './DesignSystem';
import {
  HiOutlineSignal,
  HiOutlineCpuChip,
  HiOutlineRadio,
  HiOutlineLink,
  HiOutlineArrowsRightLeft,
  HiOutlineMap
} from 'react-icons/hi2';

interface NetworkNode {
  id: string;
  name: string;
  status: 'online' | 'offline' | 'warning';
  latency: number;
  strength: number;
  lastSeen: Date;
  location?: string;
}

interface DiscoveredPeer {
  ip: string;
  port: number;
  username: string;
  name?: string;
  node_id: string;
  public_key: string;
  last_seen: string;
  status: string;
}

interface NetworkPanelProps {
  networkNodes: NetworkNode[];
  discoveredPeers: DiscoveredPeer[];
  onNodeAction: (nodeId: string, action: string) => void;
  onScanNetwork: () => void;
  onRefreshPeers: () => void;
  onBroadcast: () => void;
  onConnectAll: () => void;
  scanning: boolean;
  onNodeClick?: (nodeId: string) => void;
}

const NetworkPanel: React.FC<NetworkPanelProps> = ({
  networkNodes,
  discoveredPeers,
  onNodeAction,
  onScanNetwork,
  onRefreshPeers,
  onBroadcast,
  onConnectAll,
  scanning,
  onNodeClick
}) => {
  return (
    <div className="space-y-8 animate-fade-in">
      {/* Header Section */}
      <div className="flex flex-col md:flex-row justify-between items-start md:items-end gap-6 bg-slate-900/20 p-6 rounded-3xl border border-white/5">
        <div className="space-y-1">
          <SectionTitle className="text-2xl font-black">Mesh Topology</SectionTitle>
          <Subtitle className="text-slate-400 font-medium">Real-time localized peer-to-peer visualizer and discovery</Subtitle>
        </div>
        <div className="flex gap-3">
          <div className="flex flex-col items-end gap-1">
            <span className="text-[9px] font-black uppercase tracking-widest text-indigo-400/60">Active_Links</span>
            <Badge variant="indigo" className="px-4 py-1.5 rounded-xl font-black text-xs border border-indigo-500/20 shadow-lg shadow-indigo-500/10">
              {networkNodes.length} NODES
            </Badge>
          </div>
          <div className="flex flex-col items-end gap-1">
            <span className="text-[9px] font-black uppercase tracking-widest text-emerald-400/60">Detected_Ops</span>
            <Badge variant="info" className="px-4 py-1.5 rounded-xl font-black text-xs border border-emerald-500/20 shadow-lg shadow-emerald-500/10">
              {discoveredPeers.length} PEERS
            </Badge>
          </div>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-12 gap-8">
        {/* Main Visualization Area */}
        <div className="lg:col-span-8 space-y-8">
          <GlassCard className="h-[450px] md:h-[550px] relative overflow-hidden group border-indigo-500/10 hover:border-indigo-500/20 transition-all duration-700">
            {/* HUD Indicators */}
            <div className="absolute top-8 left-8 z-10 space-y-4">
              <div className="bg-slate-950/60 px-4 py-2.5 rounded-2xl border border-white/10 backdrop-blur-xl flex items-center gap-3 shadow-2xl">
                <div className="relative">
                  <div className="w-2 h-2 rounded-full bg-indigo-500 animate-pulse" />
                  <div className="absolute inset-0 w-2 h-2 rounded-full bg-indigo-400 animate-ping opacity-40" />
                </div>
                <span className="text-[10px] font-black text-indigo-100 uppercase tracking-[0.2em]">Live_Stream_Enabled</span>
              </div>

              <div className="flex flex-col gap-2 opacity-50">
                <div className="flex items-center gap-2 text-[8px] font-black text-slate-400 uppercase tracking-widest">
                  <div className="w-4 h-[1px] bg-slate-700" /> Latency_Optimized
                </div>
                <div className="flex items-center gap-2 text-[8px] font-black text-slate-400 uppercase tracking-widest">
                  <div className="w-4 h-[1px] bg-slate-700" /> Multi_Hop_Routing
                </div>
              </div>
            </div>

            <div className="w-full h-full">
              <MeshVisualizer nodes={networkNodes} onNodeClick={onNodeClick} />
            </div>

            {/* Float HUD Controls */}
            <div className="absolute bottom-8 left-8 right-8 flex justify-between items-center z-10">
              <div className="bg-slate-950/80 backdrop-blur-xl px-4 py-2.5 rounded-2xl border border-white/10 text-[9px] text-slate-400 font-black uppercase tracking-widest flex items-center gap-3 shadow-2xl">
                <span className="w-2 h-2 rounded-full bg-emerald-500/50" />
                Auto-scaling active • Dynamic routing • Layer_3
              </div>
              <div className="flex gap-2">
                <button
                  onClick={onRefreshPeers}
                  className="p-3 bg-slate-900/80 hover:bg-slate-800 text-white rounded-2xl border border-white/10 transition-all active:scale-90 hover:border-indigo-500/40 group shadow-xl"
                  title="Resync Data"
                >
                  <HiOutlineArrowsRightLeft className="text-xl group-hover:rotate-180 transition-transform duration-500" />
                </button>
                <button className="p-3 bg-slate-900/80 hover:bg-slate-800 text-white rounded-2xl border border-white/10 transition-all hover:border-indigo-500/40 shadow-xl">
                  <HiOutlineMap className="text-xl" />
                </button>
              </div>
            </div>
          </GlassCard>

          {/* Peer Quick View Grid */}
          <div className="space-y-4">
            <div className="flex items-center justify-between px-2">
              <span className="text-[10px] font-black uppercase tracking-[0.2em] text-slate-500 opacity-70">Active_Registry</span>
              <span className="text-[10px] font-medium text-slate-600 italic">Showing {Math.min(networkNodes.length, 4)} of {networkNodes.length} peers</span>
            </div>
            <div className="grid grid-cols-2 md:grid-cols-4 gap-6">
              {networkNodes.length > 0 ? (
                networkNodes.slice(0, 4).map(node => (
                  <div key={node.id} className="glass-card p-5 flex flex-col gap-4 border-indigo-500/5 hover:border-indigo-500/20 group cursor-pointer transition-all">
                    <div className="flex justify-between items-start">
                      <div className={`p-2 rounded-xl transition-all duration-300 ${node.status === 'online' ? 'bg-indigo-500/10 text-indigo-400 shadow-[0_0_12px_rgba(99,102,241,0.2)]' : 'bg-rose-500/10 text-rose-400'}`}>
                        <HiOutlineCpuChip className="text-base group-hover:scale-110 transition-transform" />
                      </div>
                      <div className="flex flex-col items-end">
                        <span className="text-[9px] font-black text-slate-600 uppercase">PING</span>
                        <span className="text-[10px] font-mono font-bold text-slate-400">{node.latency}ms</span>
                      </div>
                    </div>
                    <div>
                      <div className="text-xs font-black text-white truncate group-hover:text-indigo-400 transition-colors uppercase tracking-tight">{node.name}</div>
                      <div className="flex items-center gap-2 mt-2">
                        <div className={`w-1 h-1 rounded-full ${node.status === 'online' ? 'bg-indigo-500 anim-pulse' : 'bg-rose-500'}`} />
                        <div className="text-[8px] uppercase font-black tracking-widest text-slate-500">{node.status}</div>
                      </div>
                    </div>
                  </div>
                ))
              ) : (
                <div className="col-span-full py-12 bg-slate-900/10 rounded-3xl border border-dashed border-white/5 flex flex-col items-center justify-center opacity-40">
                  <div className="w-12 h-12 rounded-full border border-slate-700 flex items-center justify-center mb-4">
                    <HiOutlineSignal className="text-slate-500 text-xl" />
                  </div>
                  <p className="text-[10px] font-black uppercase tracking-widest text-slate-500">No active links established</p>
                </div>
              )}
            </div>
          </div>
        </div>

        {/* Sidebar Controls Area */}
        <div className="lg:col-span-4 space-y-8">
          <GlassCard className="p-8 border-indigo-500/10">
            <h3 className="text-xs font-black text-white uppercase tracking-[0.25em] mb-8 flex items-center gap-3">
              <HiOutlineRadio className="text-indigo-400 text-lg" />
              Pulse_Protocol
            </h3>
            <div className="space-y-4">
              <ActionButton
                onClick={onScanNetwork}
                disabled={scanning}
                className="w-full justify-start py-5 px-6 rounded-2xl hover:scale-[1.02] transition-all bg-indigo-600/5 active:scale-95 border-indigo-500/10"
              >
                <div className={`w-12 h-12 rounded-2xl flex items-center justify-center bg-indigo-600/10 text-indigo-400 border border-indigo-500/20 shadow-lg ${scanning ? 'animate-pulse' : 'group-hover:shadow-indigo-500/30 group-hover:border-indigo-500/50'} transition-all`}>
                  <HiOutlineSignal className="text-xl" />
                </div>
                <div className="text-left ml-5">
                  <div className="text-[11px] font-black uppercase text-white tracking-widest">{scanning ? 'Pulsing_Array...' : 'Scan Area'}</div>
                  <div className="text-[9px] text-slate-500 font-bold mt-1 uppercase tracking-tight">Discover local nodes via RF</div>
                </div>
              </ActionButton>

              <ActionButton
                variant="primary"
                onClick={onConnectAll}
                disabled={discoveredPeers.length === 0}
                className="w-full justify-start py-5 px-6 rounded-2xl hover:scale-[1.02] transition-all active:scale-95 group shadow-indigo-600/10"
              >
                <div className="w-12 h-12 rounded-2xl flex items-center justify-center bg-white/10 text-white shadow-xl group-hover:bg-white/20 transition-all border border-white/20">
                  <HiOutlineLink className="text-xl" />
                </div>
                <div className="text-left ml-5">
                  <div className="text-[11px] font-black uppercase text-white tracking-widest">Connect_Peers</div>
                  <div className="text-[9px] text-indigo-200/60 font-bold mt-1 uppercase tracking-tight">Auto-bridge discovered ops</div>
                </div>
              </ActionButton>

              <ActionButton
                onClick={onBroadcast}
                className="w-full justify-start py-5 px-6 rounded-2xl hover:scale-[1.02] transition-all bg-emerald-600/5 active:scale-95 border-emerald-500/10"
              >
                <div className="w-12 h-12 rounded-2xl flex items-center justify-center bg-emerald-600/10 text-emerald-400 border border-emerald-500/20 shadow-lg group-hover:shadow-emerald-500/30 group-hover:border-emerald-500/50 transition-all">
                  <HiOutlineRadio className="text-xl" />
                </div>
                <div className="text-left ml-5">
                  <div className="text-[11px] font-black uppercase text-white tracking-widest">Broadcast_Pulse</div>
                  <div className="text-[9px] text-slate-500 font-bold mt-1 uppercase tracking-tight">Signal all nodes in range</div>
                </div>
              </ActionButton>
            </div>
          </GlassCard>

          <GlassCard className="p-8 border-emerald-500/5">
            <div className="flex items-center justify-between mb-8">
              <h3 className="text-xs font-black text-white uppercase tracking-[0.25em] flex items-center gap-3">
                <HiOutlineArrowsRightLeft className="text-emerald-400 text-lg" />
                Log_Stream
              </h3>
              {discoveredPeers.length > 0 && (
                <div className="px-3 py-1 bg-emerald-500/10 border border-emerald-500/20 rounded-lg">
                  <span className="text-[10px] text-emerald-400 font-black font-mono tracking-tighter">{discoveredPeers.length}FOUND</span>
                </div>
              )}
            </div>

            <div className="space-y-4 max-h-[400px] overflow-y-auto pr-2 custom-scrollbar">
              {discoveredPeers.length > 0 ? (
                discoveredPeers.map(peer => (
                  <div key={peer.node_id} className="p-4 bg-slate-900/40 rounded-2xl border border-white/5 flex items-center justify-between group hover:border-indigo-500/30 transition-all hover:bg-slate-900/60 shadow-lg">
                    <div className="flex items-center gap-4 overflow-hidden">
                      <div className="w-10 h-10 rounded-xl bg-slate-950 border border-white/10 flex items-center justify-center text-indigo-400 font-black text-xs shadow-inner group-hover:border-indigo-500/20 transition-all">
                        {(peer.username || peer.name || 'U').charAt(0).toUpperCase()}
                      </div>
                      <div className="overflow-hidden">
                        <div className="text-xs font-black text-slate-200 truncate uppercase tracking-tight group-hover:text-white transition-colors">
                          {peer.username || peer.name || 'Unknown Node'}
                        </div>
                        <div className="text-[9px] text-slate-500 font-black font-mono truncate mt-1 tracking-widest opacity-60">
                          {peer.ip ? peer.ip.split('.').slice(2).join('.') : 'NODE'}•••{peer.port || '????'}
                        </div>
                      </div>
                    </div>
                    <button
                      onClick={() => onNodeAction(peer.node_id, 'connect')}
                      className="flex-shrink-0 text-[9px] font-black uppercase tracking-widest text-indigo-400 hover:text-white px-4 py-2 rounded-xl bg-indigo-500/10 border border-indigo-500/10 hover:bg-indigo-600 hover:border-indigo-500 transition-all opacity-40 group-hover:opacity-100 shadow-lg"
                    >
                      Bridge
                    </button>
                  </div>
                ))
              ) : (
                <div className="text-center py-20 border border-dashed border-white/5 rounded-[2rem] bg-slate-900/10">
                  <div className="w-2 h-2 rounded-full bg-slate-800 mx-auto mb-3 animate-pulse" />
                  <p className="text-[10px] font-black text-slate-600 uppercase tracking-[0.3em]">Discovery_Idle</p>
                </div>
              )}
            </div>
          </GlassCard>
        </div>
      </div>
    </div>
  );
};

export default NetworkPanel;
