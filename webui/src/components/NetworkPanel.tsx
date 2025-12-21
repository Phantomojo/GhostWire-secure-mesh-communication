import React from 'react';
import MeshVisualizer from './MeshVisualizer';

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
}

const NetworkPanel: React.FC<NetworkPanelProps> = ({
  networkNodes,
  discoveredPeers,
  onNodeAction,
  onScanNetwork,
  onRefreshPeers,
  onBroadcast,
  onConnectAll,
  scanning
}) => {
  return (
    <div className="main-content-module">
      <div className="module-header">
        <h2 className="module-title">MESH_NETWORK_TOPOLOGY</h2>
        <div className="module-subtitle">ACTIVE_CONNECTIONS: {networkNodes.length} | PEERS_DISCOVERED: {discoveredPeers.length}</div>
      </div>

      <div className="module-grid">
        {/* Topology Visualization Module */}
        <div className="glass-module-card full-width no-padding overflow-hidden">
          <div className="topology-hud-header">
            <span className="hud-indicator">LIVE_MESH_STREAM</span>
            <div className="hud-controls">
              <span className="control-label">ZOOM:100%</span>
              <span className="control-label">ROT:0.0</span>
            </div>
          </div>
          <MeshVisualizer nodes={networkNodes} />
        </div>

        {/* Peer Management Deck */}
        <div className="glass-module-card">
          <h3 className="card-title">TACTICAL_NETWORK_DECK</h3>
          <div className="action-grid-2">
            <button className={`tactical-btn ${scanning ? 'pulse-active' : ''}`} onClick={onScanNetwork} disabled={scanning}>
              <span className="btn-label">{scanning ? 'SCANNING...' : 'SCAN_MESH'}</span>
              <span className="btn-subtext">Local P2P Discovery</span>
            </button>
            <button className="tactical-btn" onClick={onRefreshPeers}>
              <span className="btn-label">REFRESH_PEERS</span>
              <span className="btn-subtext">Update connection state</span>
            </button>
            <button className="tactical-btn purple" onClick={onBroadcast}>
              <span className="btn-label">SIGNAL_BROADCAST</span>
              <span className="btn-subtext">Announce presence</span>
            </button>
            <button className="tactical-btn cyan" onClick={onConnectAll} disabled={discoveredPeers.length === 0}>
              <span className="btn-label">AUTOCONNECT_ALL</span>
              <span className="btn-subtext">Link all valid peers</span>
            </button>
          </div>
        </div>

        {/* Connection Telemetry */}
        <div className="glass-module-card">
          <h3 className="card-title">CONNECTION_TELEMETRY</h3>
          <div className="node-list-mini">
            {networkNodes.length > 0 ? (
              networkNodes.map(node => (
                <div key={node.id} className="telemetry-node-row">
                  <div className={`node-indicator ${node.status}`}></div>
                  <div className="node-meta">
                    <span className="node-id">{node.name}</span>
                    <span className="node-latency">{node.latency}MS / {node.strength}%</span>
                  </div>
                  <div className="node-actions-mini">
                    <button className="mini-btn" onClick={() => onNodeAction(node.id, 'ping')}>PING</button>
                    <button className="mini-btn warn" onClick={() => onNodeAction(node.id, 'disconnect')}>DROP</button>
                  </div>
                </div>
              ))
            ) : (
              <div className="empty-telemetry">NO_ACTIVE_CONNECTIONS</div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

export default NetworkPanel;
