import { useState, useEffect } from 'react';
import Sidebar from './components/layout/Sidebar';
import Header from './components/layout/Header';
import LoginScreen from './components/LoginScreen';
import FirstTimeSetup from './components/FirstTimeSetup';
import Dashboard from './components/Dashboard';
import NetworkPanel from './components/NetworkPanel';
import SecurityPanel from './components/SecurityPanel';
import SystemPanel from './components/SystemPanel';
import CommunicationPanel from './components/CommunicationPanel';
import SettingsPage from './components/SettingsPage';
import MovingMeshBackground from './components/MovingMeshBackground';
import CyberToast, { type Toast, type ToastType } from './components/CyberToast';
import { TutorialOverlay } from './components/TutorialOverlay';

import { api, config } from './services/api';

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
  const [activePanel, setActivePanel] = useState<'dashboard' | 'network' | 'security' | 'system' | 'communication' | 'settings'>('dashboard');
  const [isEmergencyMode, setIsEmergencyMode] = useState(false);
  const [isStealthMode, setIsStealthMode] = useState(false);
  const [isPanicMode, setIsPanicMode] = useState(false);
  const [showConfig, setShowConfig] = useState(false);
  const [backendUrl, setBackendUrl] = useState(config.API_BASE_URL);
  const [backendConnected, setBackendConnected] = useState(false);
  const [showTutorial, setShowTutorial] = useState(false);

  const [systemStatus, setSystemStatus] = useState<SystemStatus>({
    cpu: 15,
    memory: 34,
    network: 12,
    encryption: 100,
    uptime: 3600,
    connections: 0,
    messages: 0,
    securityScore: 98
  });

  const [networkNodes, setNetworkNodes] = useState<NetworkNode[]>([]);
  const [discoveredPeers, setDiscoveredPeers] = useState<DiscoveredPeer[]>([]);
  const [securityAlerts] = useState<SecurityAlert[]>([]);
  const [messages, setMessages] = useState<Message[]>([]);
  const [scanning, setScanning] = useState(false);

  const [user, setUser] = useState({
    id: 'ghost-local',
    did: 'did:ghost:pending...',
    name: 'GhostUser',
    nickname: 'Ghost_Core',
    bio: 'Sovereign GhostWire Node',
    avatar: '👻',
    status: 'Operational',
    isOnline: true
  });
  const [pqcArmor, setPqcArmor] = useState(false);
  const [currentShard, setCurrentShard] = useState('global');

  // Feedback System
  const [toasts, setToasts] = useState<Toast[]>([]);
  const [activeRecipient, setActiveRecipient] = useState<string>('broadcast');
  const [isLocked, setIsLocked] = useState(true);
  const [profileStatus, setProfileStatus] = useState<any>(null);

  const addToast = (type: ToastType, message: string) => {
    setToasts(prev => [...prev, { id: Math.random().toString(36).substring(7), type, message }]);
  };
  const removeToast = (id: string) => {
    setToasts(prev => prev.filter(t => t.id !== id));
  };

  useEffect(() => {
    const init = async () => {
      console.log('🚀 Initializing GhostWire Core UI...');
      // Small delay to allow backend to start if needed
      await new Promise(r => setTimeout(r, 1000));

      // Check Profile Status
      try {
        const status = await api.getIdentityStatus();
        setProfileStatus(status);
        setIsLocked(status.locked);
        if (status.locked) {
          console.log('🔒 Profile is locked. Showing login screen.');
        } else {
          console.log('🔓 Profile unlocked.');
        }
      } catch (e) {
        console.error('Failed to get profile status:', e);
        // Assuming locked if failed to reach backend or error
      }

      try {
        const authed = await api.autoAuthenticate();
        if (authed) {
          console.log('✅ Auto-authentication successful');
        } else {
          console.warn('⚠️ Auto-authentication failed. Some features may be restricted.');
        }
      } catch (e) {
        console.error('❌ Auto-authentication error:', e);
      }

      await loadRealData();
      const interval = setInterval(loadRealData, 5000);
      return () => clearInterval(interval);
    };
    init();
  }, []);

  const testBackendConnection = async () => {
    try {
      await api.get('/status');
      setBackendConnected(true);
      loadRealData();
    } catch (error) {
      setBackendConnected(false);
      console.error('Backend connection check failed:', error);
    }
  };

  const loadRealData = async () => {
    try {
      const [statusRes, peersV1, profileV2, securityV2] = await Promise.all([
        api.get('/status'), // Keeping some V1 for now as fallback
        api.getPeers(), // Uses V2 with V1 fallback
        api.getProfile(),
        api.getSecurityDashboard()
      ]);

      if (profileV2) {
        setUser(prev => ({
          ...prev,
          id: profileV2.id,
          did: profileV2.did,
          username: profileV2.username,
          nickname: profileV2.nickname,
          bio: profileV2.bio,
          security_score: profileV2.security_score,
        }));
      }

      if (securityV2) {
        setSystemStatus(prev => ({
          ...prev,
          securityScore: securityV2.score,
          encryption: securityV2.encryption.active ? 100 : 0,
        }));
        setPqcArmor(securityV2.encryption.pqc_armor);
      }

      if (Array.isArray(peersV1)) {
        const mappedNodes: NetworkNode[] = peersV1.map((n: any) => ({
          id: n.id,
          name: n.username,
          status: n.status as any,
          latency: Math.floor(Math.random() * 50) + 10,
          strength: n.trust_score || 100,
          lastSeen: new Date(n.last_seen)
        }));

        setNetworkNodes(mappedNodes);
        setDiscoveredPeers(peersV1.map(n => ({
          ip: '', // V2 doesn't expose IP directly in this view for privacy
          port: 3001,
          username: n.username,
          node_id: n.id,
          public_key: n.public_key,
          last_seen: n.last_seen,
          status: n.status
        })));
        // Calculate Average Latency
        if (mappedNodes.length > 0) {
          const avg = Math.round(mappedNodes.reduce((acc, node) => acc + node.latency, 0) / mappedNodes.length);
          setSystemStatus(prev => ({ ...prev, network: avg }));
        }
      }

      if (statusRes.success) {
        setBackendConnected(true);
        const s = statusRes.data;

        setSystemStatus(prev => ({
          ...prev,
          cpu: Math.round(s.cpu_usage ?? 15),
          memory: Math.round((s.memory_used_kb / s.memory_total_kb) * 100) || 34,
          uptime: s.uptime_seconds,
          connections: s.peer_count,
        }));
      }

      // Load historical messages from the Sovereign Persistence layer
      try {
        const history = await api.getHistoricalMessages(activeRecipient === 'broadcast' ? 'all' : activeRecipient);
        if (history && history.length > 0) {
          setMessages(history);
        }
      } catch (e) {
        console.warn('Failed to load historical messages from Sovereign store:', e);
      }
    } catch (error) {
      setBackendConnected(false);
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
      await api.get('/scan_network');
      loadRealData();
    } catch (error) {
      console.error('Scan failed:', error);
    } finally {
      setScanning(false);
    }
  };

  const handleRefreshPeers = async () => loadRealData();

  const handleBroadcast = async () => {
    try {
      await api.post('/broadcast', { message: "System Broadcast Signature" });
    } catch (error) {
      console.error('Broadcast failed:', error);
    }
  };

  const handleConnectAll = async () => {
    try {
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

  const handleUpgradeEncryption = async () => {
    try {
      const newMode = !pqcArmor;
      await api.put('/v2/security/armor', { enabled: newMode });
      setPqcArmor(newMode);
    } catch (error) {
      console.error('PQC Armor toggle failed:', error);
    }
  };

  const handleUpdateMetadata = async (metadata: { nickname?: string; bio?: string }) => {
    try {
      await api.put('/v2/profile/metadata', metadata);
      loadRealData();
    } catch (error) {
      console.error('Metadata update failed:', error);
    }
  };

  const handleUpdateShard = async (shard: string) => {
    try {
      await api.put('/v2/mesh/sharding', { shard });
      setCurrentShard(shard);
    } catch (error) {
      console.error('Shard update failed:', error);
    }
  };

  const handleConfigureFirewall = async () => {
    try {
      const response = await api.post('/configure_firewall', { rules: [] });
      alert(`Firewall configured: ${JSON.stringify(response.data)}`);
    } catch (error) {
      console.error('Configure firewall failed:', error);
      alert('Firewall configuration failed');
    }
  };

  const handleTestFirewall = async () => {
    try {
      const response = await api.post('/test_firewall');
      alert(`Firewall test: ${response.data.success ? 'PASSED' : 'FAILED'}\n${JSON.stringify(response.data.data, null, 2)}`);
    } catch (error) {
      console.error('Test firewall failed:', error);
      alert('Firewall test failed');
    }
  };

  const handleShowLogs = async () => {
    try {
      const response = await api.get('/logs');
      console.log('System Logs:', response.data);
      alert(`Logs retrieved. Check browser console for details.\nLog count: ${response.data.data?.logs?.length || 0}`);
    } catch (error) {
      console.error('Get logs failed:', error);
      alert('Failed to retrieve logs');
    }
  };

  const handleSecurityScan = async () => {
    try {
      alert('Starting security scan...');
      const response = await api.post('/security_scan');
      console.log('Security Scan Results:', response.data);
      alert(`Security Scan Complete!\nScore: ${response.data.data?.security_score || 'N/A'}\nVulnerabilities: ${response.data.data?.vulnerabilities_found || 0}`);
    } catch (error) {
      console.error('Security scan failed:', error);
      alert('Security scan failed');
    }
  };

  const handleThreatHunt = async () => {
    try {
      alert('Starting threat hunt...');
      const response = await api.post('/threat_hunt');
      console.log('Threat Hunt Results:', response.data);
      alert(`Threat Hunt Complete!\nThreats Found: ${response.data.data?.threats_found || 0}\nStatus: ${response.data.data?.status || 'Unknown'}`);
    } catch (error) {
      console.error('Threat hunt failed:', error);
      alert('Threat hunt failed');
    }
  };

  const handleSecurityAudit = async () => {
    try {
      alert('Starting security audit...');
      const response = await api.post('/security_audit');
      console.log('Security Audit Results:', response.data);
      alert(`Security Audit Complete!\nFindings: ${response.data.data?.findings?.length || 0}\nRisk Level: ${response.data.data?.risk_level || 'Unknown'}`);
    } catch (error) {
      console.error('Security audit failed:', error);
      alert('Security audit failed');
    }
  };

  const handleLockdown = async () => {
    if (!confirm('⚠️ This will enable lockdown mode. All connections will be blocked. Continue?')) return;
    try {
      const response = await api.post('/security/lockdown', { enabled: true });
      alert(response.data.success ? '🔒 Lockdown enabled!' : 'Lockdown failed');
    } catch (error) {
      console.error('Lockdown failed:', error);
      alert('Lockdown failed');
    }
  };

  const handleBackupSecurity = async () => {
    try {
      const response = await api.post('/backup_security');
      console.log('Backup result:', response.data);
      alert(`Security backup created!\nBackup ID: ${response.data.data?.backup_id || 'Unknown'}`);
    } catch (error) {
      console.error('Backup security failed:', error);
      alert('Backup failed');
    }
  };

  const handleSendMessage = async (content: string) => {
    try {
      const recipient = activeRecipient;
      await api.post('/send_message', { content, recipient });
      loadRealData();
      addToast('success', `Packet sent via ${recipient === 'broadcast' ? 'OMNI-WAVE' : 'DIRECT-BEAM'}`);
    } catch (error) {
      console.error('Send message failed:', error);
      addToast('error', 'Transmission Failed: Encryption Handshake Error');
    }
  };

  const handleSetBackendUrl = async () => {
    if (backendUrl && backendUrl.trim()) {
      await config.setBackendUrl(backendUrl.trim());
      testBackendConnection();
    }
  };

  return (
    <div className={`flex h-[100dvh] w-screen overflow-hidden relative ${isEmergencyMode ? 'emergency-state' : ''}`}>
      {isLocked && profileStatus && (
        <LoginScreen
          status={profileStatus}
          onUnlock={() => {
            setIsLocked(false);
            addToast('success', 'Profile Decrypted. Access Granted.');
            // Refresh status to check setup_complete
            api.getIdentityStatus().then(setStatus => setProfileStatus(setStatus));
            loadRealData();
          }}
        />
      )}

      {!isLocked && profileStatus && !profileStatus.setup_complete && (
        <FirstTimeSetup
          onComplete={() => {
            setProfileStatus({ ...profileStatus, setup_complete: true });
            addToast('success', 'Sovereign Profile Established.');
            loadRealData();
          }}
        />
      )}

      <MovingMeshBackground />
      <CyberToast toasts={toasts} onRemove={removeToast} />

      {/* Sidebar - z-Index 20 to sit above background */}
      <div className="relative z-20 h-full">
        <Sidebar
          activePanel={activePanel}
          onPanelChange={setActivePanel}
          isEmergencyMode={isEmergencyMode}
          isStealthMode={isStealthMode}
          isPanicMode={isPanicMode}
          onEmergencyMode={handleEmergencyMode}
          onStealthMode={handleStealthMode}
          onPanicMode={handlePanicMode}
          user={user}
          onUpdateMetadata={handleUpdateMetadata}
        />
      </div>

      <div className="flex-1 flex flex-col min-w-0 relative z-10">
        <Header
          systemStatus={systemStatus}
          backendConnected={backendConnected}
          onConfigClick={() => setShowConfig(true)}
          onTutorialClick={() => setShowTutorial(true)}
        />

        {showConfig && (
          <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/70 backdrop-blur-md animate-fade-in">
            <div className="glass-card w-full max-w-md p-8 border border-white/10 shadow-2xl">
              <h3 className="text-xl font-bold text-white mb-6 text-display">Core Configuration</h3>
              <div className="space-y-4">
                <input
                  type="text"
                  value={backendUrl}
                  onChange={(e) => setBackendUrl(e.target.value)}
                  className="input-premium font-mono"
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

        <main className={`flex-1 custom-scrollbar ${activePanel === 'communication' ? 'overflow-hidden' : 'overflow-y-auto'}`} style={{ padding: activePanel === 'communication' ? 0 : 'clamp(1rem, 3vw, 2rem)' }}>
          <div className={`animate-fade-in ${activePanel === 'communication' ? 'h-full w-full' : 'max-w-6xl mx-auto'}`}>
            {activePanel === 'dashboard' && (
              <Dashboard
                systemStatus={systemStatus}
                peerCount={discoveredPeers.length}
                threatCount={0}
                backendConnected={backendConnected}
                onScanNetwork={handleScanNetwork}
                onBroadcast={handleBroadcast}
                onSync={handleRefreshPeers}
                onEmergencyMode={handleEmergencyMode}
                onNavigate={setActivePanel}
                isEmergencyMode={isEmergencyMode}
                pqcEnabled={pqcArmor}
              />
            )}

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
                currentShard={currentShard}
                onUpdateShard={handleUpdateShard}
              />
            )}

            {activePanel === 'security' && (
              <SecurityPanel
                securityAlerts={securityAlerts}
                systemStatus={systemStatus}
                onAlertAction={(alertId, action) => console.log(alertId, action)}
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
                onSystemAction={(action) => console.log(action)}
              />
            )}

            {activePanel === 'communication' && (
              <CommunicationPanel
                messages={messages}
                onSendMessage={handleSendMessage}
                onEmergency={handleEmergencyMode}
                user={user}
                peers={discoveredPeers}
                activeRecipient={activeRecipient}
                onSelectRecipient={setActiveRecipient}
              />
            )}

            {activePanel === 'settings' && (
              <SettingsPage
                backendConnected={backendConnected}
                onBack={() => setActivePanel('dashboard')}
              />
            )}
          </div>
        </main>
      </div>
      {/* Tutorial Overlay */}
      <TutorialOverlay isOpen={showTutorial} onClose={() => setShowTutorial(false)} />

      {/* Configuration Dialog */}
    </div>
  );
}

export default App;