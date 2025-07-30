import React from 'react';

interface SystemStatus {
  cpu: number;
  memory: number;
  network: number;
  encryption: number;
  uptime: number;
  connections: number;
  messages: number;
  securityScore: number;
}

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
  systemStatus: SystemStatus;
  discoveredPeers: DiscoveredPeer[];
  onNodeAction: (nodeId: string, action: string) => void;
  onScanNetwork: () => void;
  onRefreshPeers: () => void;
  onBroadcast: () => void;
  onConnectAll: () => void;
  onDisconnectAll: () => void;
  scanning: boolean;
}

const NetworkPanel: React.FC<NetworkPanelProps> = ({
  networkNodes,
  systemStatus,
  discoveredPeers,
  onNodeAction,
  onScanNetwork,
  onRefreshPeers,
  onBroadcast,
  onConnectAll,
  onDisconnectAll,
  scanning
}) => {
  const getStatusColor = (status: string) => {
    switch (status) {
      case 'online': return '#00ff00';
      case 'warning': return '#ffaa00';
      case 'offline': return '#ff4444';
      default: return '#888888';
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'online': return '🟢';
      case 'warning': return '🟡';
      case 'offline': return '🔴';
      default: return '⚪';
    }
  };

  const formatTime = (date: Date) => {
    const now = new Date();
    const diff = now.getTime() - date.getTime();
    const minutes = Math.floor(diff / 60000);
    if (minutes < 1) return 'now';
    if (minutes < 60) return `${minutes}m ago`;
    const hours = Math.floor(minutes / 60);
    return `${hours}h ago`;
  };

  return (
    <div className="network-panel">
      {/* Network Statistics */}
      <div className="network-stats">
        <div className="stat">
          <div className="stat-value">{networkNodes.length}</div>
          <div className="stat-label">Total Nodes</div>
        </div>
        <div className="stat">
          <div className="stat-value">{networkNodes.filter(n => n.status === 'online').length}</div>
          <div className="stat-label">Online</div>
        </div>
        <div className="stat">
          <div className="stat-value">{systemStatus.connections}</div>
          <div className="stat-label">Active Connections</div>
        </div>
        <div className="stat">
          <div className="stat-value">{discoveredPeers.length}</div>
          <div className="stat-label">Discovered Peers</div>
        </div>
      </div>

      {/* Network Topology */}
      <div className="network-topology">
        <h3>🌐 Network Topology</h3>
        <div className="topology-visualization">
          <div className="local-node">
            <div className="node-circle local">LOCAL</div>
            <div className="node-info">This Node</div>
          </div>
          <div className="connections">
            {networkNodes.slice(0, 6).map((node) => (
              <div key={node.id} className="connection-line">
                <div className={`remote-node ${node.status}`}>
                  <div className="node-circle" style={{ borderColor: getStatusColor(node.status) }}>
                    {getStatusIcon(node.status)}
                  </div>
                  <div className="node-name">{node.name}</div>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Connected Nodes */}
      <div className="connected-nodes">
        <h3>🔗 Connected Nodes</h3>
        <div className="nodes-grid">
          {networkNodes.map((node) => (
            <div key={node.id} className={`node-card ${node.status}`}>
              <div className="node-header">
                <div className="node-status" style={{ color: getStatusColor(node.status) }}>
                  {getStatusIcon(node.status)}
                </div>
                <div className="node-name">{node.name}</div>
              </div>
              <div className="node-details">
                <div className="node-latency">Latency: {node.latency}ms</div>
                <div className="node-strength">Signal: {node.strength}%</div>
                <div className="node-last-seen">Last seen: {formatTime(node.lastSeen)}</div>
                {node.location && (
                  <div className="node-location">Location: {node.location}</div>
                )}
              </div>
              <div className="node-actions">
                <button onClick={() => onNodeAction(node.id, 'ping')}>Ping</button>
                <button onClick={() => onNodeAction(node.id, 'connect')}>Connect</button>
                <button onClick={() => onNodeAction(node.id, 'disconnect')}>Disconnect</button>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Discovered Peers */}
      {discoveredPeers.length > 0 && (
        <div className="discovered-peers">
          <h3>🔍 Discovered Peers</h3>
          <div className="peers-grid">
            {discoveredPeers.map((peer) => (
              <div key={peer.node_id} className="peer-card">
                <div className="peer-header">
                  <div className="peer-status" style={{ color: getStatusColor(peer.status) }}>
                    {getStatusIcon(peer.status)}
                  </div>
                  <div className="peer-name">{peer.username}</div>
                </div>
                <div className="peer-details">
                  <div className="peer-ip">IP: {peer.ip}:{peer.port}</div>
                  <div className="peer-id">ID: {peer.node_id}</div>
                  <div className="peer-last-seen">Last seen: {peer.last_seen}</div>
                </div>
                <div className="peer-actions">
                  <button onClick={() => onNodeAction(peer.node_id, 'connect')}>Connect</button>
                  <button onClick={() => onNodeAction(peer.node_id, 'ping')}>Ping</button>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Network Actions */}
      <div className="network-actions">
        <h3>⚡ Network Actions</h3>
        <div className="action-buttons">
          <button 
            className={`network-action-btn ${scanning ? 'scanning' : ''}`}
            onClick={onScanNetwork}
            disabled={scanning}
          >
            {scanning ? '🔍 Scanning...' : '🔍 Scan Network'}
          </button>
          <button 
            className="network-action-btn"
            onClick={onRefreshPeers}
          >
            🔄 Refresh Peers
          </button>
          <button 
            className="network-action-btn"
            onClick={onBroadcast}
          >
            📡 Broadcast
          </button>
          <button 
            className="network-action-btn"
            onClick={onConnectAll}
            disabled={discoveredPeers.length === 0}
          >
            🔗 Connect All ({discoveredPeers.length})
          </button>
          <button 
            className="network-action-btn"
            onClick={onDisconnectAll}
            disabled={networkNodes.length === 0}
          >
            ❌ Disconnect All ({networkNodes.length})
          </button>
        </div>
      </div>
    </div>
  );
};

export default NetworkPanel; 