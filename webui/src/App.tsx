import { useState, useEffect } from 'react';
import axios from 'axios';
import Sidebar from './components/layout/Sidebar';
import Header from './components/layout/Header';
import NetworkPanel from './components/NetworkPanel';
import SecurityPanel from './components/SecurityPanel';
import SystemPanel from './components/SystemPanel';
import CommunicationPanel from './components/CommunicationPanel';

// Configuration
const config = {
  API_BASE_URL: 'http://localhost:3001/api',
  setBackendUrl: async (url: string) => {
    config.API_BASE_URL = url;
  }
};

const api = axios.create({
  baseURL: config.API_BASE_URL,
  timeout: 5000,
});

// Types
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
}

interface SecurityAlert {
  id: string;
  type: 'warning' | 'error' | 'info';
  message: string;
  timestamp: Date;
  severity: 'low' | 'medium' | 'high' | 'critical';
}

interface Message {
  id: string;
  content: string;
  sender: string;
  recipient: string;
  timestamp: Date;
  status: 'pending' | 'sent' | 'delivered' | 'failed';
  encrypted: boolean;
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

function App() {
  const [activePanel, setActivePanel] = useState<'network' | 'security' | 'system' | 'communication'>('network');
  const [isEmergencyMode, setIsEmergencyMode] = useState(false);
  const [isStealthMode, setIsStealthMode] = useState(false);
  const [isPanicMode, setIsPanicMode] = useState(false);
  const [showConfig, setShowConfig] = useState(false);
  const [backendUrl, setBackendUrl] = useState(config.API_BASE_URL);
  const [backendConnected, setBackendConnected] = useState(false);

  const [systemStatus, setSystemStatus] = useState<SystemStatus>({
    cpu: 15,
    memory: 34,
    network: 12,
    encryption: 100,
    uptime: 3600,
    connections: 5,
    messages: 124,
    securityScore: 98
  });

  const [networkNodes, setNetworkNodes] = useState<NetworkNode[]>([]);
  const [discoveredPeers, setDiscoveredPeers] = useState<DiscoveredPeer[]>([]);
  const [securityAlerts] = useState<SecurityAlert[]>([]);
  const [messages] = useState<Message[]>([]);
  const [scanning, setScanning] = useState(false);

  useEffect(() => {
    testBackendConnection();
    loadRealData();
    const interval = setInterval(loadRealData, 10000);
    return () => clearInterval(interval);
  }, []);

  const testBackendConnection = async () => {
    try {
      await api.get('/status');
      setBackendConnected(true);
      // Immediately load data once connected
      loadRealData();
    } catch (error) {
      setBackendConnected(false);
      console.error('Backend connection check failed:', error);
    }
  };

  const loadRealData = async () => {
    try {
      const [statusRes, peersRes, topologyRes] = await Promise.all([
        api.get('/status'),
        api.get('/peers'),
        api.get('/mesh/topology')
      ]);

      // Map status data to systemStatus
      if (statusRes.data.success && statusRes.data.data) {
        const s = statusRes.data.data;
        setSystemStatus({
          cpu: Math.round(s.cpu_usage || 15),
          memory: Math.round((s.memory_used_kb / s.memory_total_kb) * 100) || 34,
          network: s.peer_count * 2, // Arbitrary network load based on peers
          encryption: s.encryption_enabled ? 100 : 0,
          uptime: s.uptime_seconds,
          connections: s.peer_count,
          messages: 124, // Mock for now
          securityScore: 98
        });
      }

      // Map topology to networkNodes
      if (topologyRes.data.success && topologyRes.data.data) {
        const t = topologyRes.data.data;
        const mappedNodes: NetworkNode[] = t.nodes.map((n: any) => ({
          id: n.id,
          name: n.username,
          status: n.is_online ? 'online' : 'offline',
          latency: Math.floor(Math.random() * 50) + 10,
          strength: Math.floor(n.connection_quality * 100),
          lastSeen: new Date(n.last_seen * 1000)
        })).filter((n: any) => n.id !== t.local_node_id); // Don't include self in the "peers" list for the visualizer as it's separate
        setNetworkNodes(mappedNodes);
      }

      // Map peers to discoveredPeers
      if (peersRes.data.success && peersRes.data.data) {
        setDiscoveredPeers(peersRes.data.data.peers);
      }

    } catch (error) {
      console.error('Failed to load real data:', error);
    }
  };

  const handleEmergencyMode = () => setIsEmergencyMode(!isEmergencyMode);
  const handleStealthMode = () => setIsStealthMode(!isStealthMode);
  const handlePanicMode = () => {
    setIsPanicMode(!isPanicMode);
    if (!isPanicMode) {
      setIsEmergencyMode(true);
      setIsStealthMode(true);
    }
  };

  const handleNodeAction = async (nodeId: string, action: string) => {
    try {
      if (action === 'connect') {
        const peer = discoveredPeers.find(p => p.node_id === nodeId);
        if (peer) {
          await api.post('/connect_peer', {
            peer_id: peer.node_id,
            peer_name: peer.username || peer.name,
            peer_ip: peer.ip,
            peer_port: peer.port
          });
        }
      }
      loadRealData();
    } catch (error) {
      console.error('Node action failed:', error);
    }
  };

  const handleScanNetwork = async () => {
    setScanning(true);
    try {
      await api.get('/scan_network'); // In web.rs it's a GET
      loadRealData();
    } catch (error) {
      console.error('Scan failed:', error);
    } finally {
      setScanning(false);
    }
  };

  const handleRefreshPeers = async () => {
    loadRealData();
  };

  const handleBroadcast = async () => {
    try {
      await api.post('/broadcast', { message: "System Broadcast Signature" });
    } catch (error) {
      console.error('Broadcast failed:', error);
    }
  };

  const handleConnectAll = async () => {
    try {
      // Connect to all discovered peers in sequence
      for (const peer of discoveredPeers) {
        await api.post('/connect_peer', {
          peer_id: peer.node_id,
          peer_name: peer.username || peer.name,
          peer_ip: peer.ip,
          peer_port: peer.port
        });
      }
      loadRealData();
    } catch (error) {
      console.error('Connect all failed:', error);
    }
  };

  const handleRotateKeys = async () => {
    try {
      await api.post('/security/rotate-keys');
    } catch (error) {
      console.error('Rotate keys failed:', error);
    }
  };

  const handleUpgradeEncryption = () => console.log('Upgrade encryption');
  const handleConfigureFirewall = () => console.log('Configure firewall');
  const handleTestFirewall = () => console.log('Test firewall');
  const handleShowLogs = () => console.log('Show logs');
  const handleSecurityScan = () => console.log('Security scan');
  const handleThreatHunt = () => console.log('Threat hunt');
  const handleSecurityAudit = () => console.log('Security audit');
  const handleLockdown = () => console.log('Lockdown');
  const handleBackupSecurity = () => console.log('Backup security');

  const sendMessage = async (content: string, recipient: string) => {
    try {
      await api.post('/messages/send', { content, recipient });
      loadRealData();
    } catch (error) {
      console.error('Send message failed:', error);
    }
  };

  const handleCommBroadcast = () => console.log('Comm broadcast');
  const handleCommEmergency = () => console.log('Comm emergency');
  const handleCommSync = async () => console.log('Comm sync');
  const handleCommAnalyze = async () => console.log('Comm analyze');

  const handleSetBackendUrl = async () => {
    if (backendUrl && backendUrl.trim()) {
      await config.setBackendUrl(backendUrl.trim());
      testBackendConnection();
    }
  };

  return (
    <div className={`flex h-screen w-screen bg-slate-950 overflow-hidden ${isEmergencyMode ? 'emergency-state' : ''}`}>
      <Sidebar
        activePanel={activePanel}
        onPanelChange={setActivePanel}
        isEmergencyMode={isEmergencyMode}
        isStealthMode={isStealthMode}
        isPanicMode={isPanicMode}
        onEmergencyMode={handleEmergencyMode}
        onStealthMode={handleStealthMode}
        onPanicMode={handlePanicMode}
      />

      <div className="flex-1 flex flex-col min-w-0 relative">
        <Header
          systemStatus={systemStatus}
          backendConnected={backendConnected}
          onConfigClick={() => setShowConfig(true)}
        />

        {showConfig && (
          <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-950/80 backdrop-blur-sm animate-fade-in">
            <div className="glass-panel w-full max-w-md p-8 rounded-3xl border border-white/10 shadow-2xl">
              <h3 className="text-xl font-bold text-white mb-6">Core Configuration</h3>
              <div className="space-y-4">
                <input
                  type="text"
                  value={backendUrl}
                  onChange={(e) => setBackendUrl(e.target.value)}
                  className="w-full bg-slate-900 border border-white/5 rounded-xl px-4 py-3 text-sm text-white font-mono"
                  placeholder="http://node-ip:port/api"
                />
              </div>
              <div className="grid grid-cols-2 gap-3 mt-8">
                <button onClick={handleSetBackendUrl} className="btn-primary">Connect</button>
                <button onClick={() => setShowConfig(false)} className="btn-secondary">Dismiss</button>
              </div>
            </div>
          </div>
        )}

        <main className="flex-1 overflow-y-auto p-8 custom-scrollbar">
          <div className="max-w-6xl mx-auto animate-fade-in">
            {activePanel === 'network' && (
              <NetworkPanel
                networkNodes={networkNodes}
                discoveredPeers={discoveredPeers}
                onNodeAction={handleNodeAction}
                onScanNetwork={handleScanNetwork}
                onRefreshPeers={handleRefreshPeers}
                onBroadcast={handleBroadcast}
                onConnectAll={handleConnectAll}
                onNodeClick={(id: string) => handleNodeAction(id, 'connect')}
                scanning={scanning}
              />
            )}

            {activePanel === 'security' && (
              <SecurityPanel
                securityAlerts={securityAlerts}
                systemStatus={systemStatus}
                onAlertAction={(alertId: string, action: string) => console.log(alertId, action)}
                onRotateKeys={handleRotateKeys}
                onUpgradeEncryption={handleUpgradeEncryption}
                onConfigureFirewall={handleConfigureFirewall}
                onTestFirewall={handleTestFirewall}
                onViewLogs={handleShowLogs}
                onSecurityScan={handleSecurityScan}
                onThreatHunt={handleThreatHunt}
                onSecurityAudit={handleSecurityAudit}
                onLockdown={handleLockdown}
                onBackupSecurity={handleBackupSecurity}
                backendConnected={backendConnected}
              />
            )}

            {activePanel === 'system' && (
              <SystemPanel
                systemStatus={systemStatus}
                onSystemAction={(action: string) => console.log(action)}
              />
            )}

            {activePanel === 'communication' && (
              <CommunicationPanel
                messages={messages}
                onSendMessage={sendMessage}
                onBroadcast={handleCommBroadcast}
                onEmergency={handleCommEmergency}
                onSync={handleCommSync}
                onAnalyze={handleCommAnalyze}
                backendConnected={backendConnected}
              />
            )}
          </div>
        </main>
      </div>
    </div>
  );
}

export default App;