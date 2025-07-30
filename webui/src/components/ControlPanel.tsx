import React from 'react';

interface ControlPanelProps {
  activePanel: 'network' | 'security' | 'system' | 'communication';
  onPanelChange: (panel: 'network' | 'security' | 'system' | 'communication') => void;
  onEmergencyMode: () => void;
  onStealthMode: () => void;
  onPanicMode: () => void;
  isEmergencyMode: boolean;
  isStealthMode: boolean;
  isPanicMode: boolean;
  onScanNetwork: () => void;
  scanning: boolean;
  backendConnected: boolean;
  onRefresh: () => void;
  onLockSystem: () => void;
  onBackup: () => void;
  onRestart: () => void;
  onShowStats: () => void;
  onShowConfig: () => void;
  onShowLogs: () => void;
}

const ControlPanel: React.FC<ControlPanelProps> = ({
  activePanel,
  onPanelChange,
  onEmergencyMode,
  onStealthMode,
  onPanicMode,
  isEmergencyMode,
  isStealthMode,
  isPanicMode,
  onScanNetwork,
  scanning,
  backendConnected,
  onRefresh,
  onLockSystem,
  onBackup,
  onRestart,
  onShowStats,
  onShowConfig,
  onShowLogs
}) => {
  return (
    <div className="control-panel">
      {/* Panel Navigation */}
      <div className="panel-nav">
        <button
          className={`nav-btn ${activePanel === 'network' ? 'active' : ''}`}
          onClick={() => onPanelChange('network')}
        >
          🌐 Network
        </button>
        <button
          className={`nav-btn ${activePanel === 'security' ? 'active' : ''}`}
          onClick={() => onPanelChange('security')}
        >
          🛡️ Security
        </button>
        <button
          className={`nav-btn ${activePanel === 'system' ? 'active' : ''}`}
          onClick={() => onPanelChange('system')}
        >
          ⚙️ System
        </button>
        <button
          className={`nav-btn ${activePanel === 'communication' ? 'active' : ''}`}
          onClick={() => onPanelChange('communication')}
        >
          💬 Communication
        </button>
      </div>

      {/* Emergency Controls */}
      <div className="emergency-controls">
        <h4>🚨 Emergency Controls</h4>
        <button
          className={`emergency-btn ${isEmergencyMode ? 'active' : ''}`}
          onClick={onEmergencyMode}
        >
          {isEmergencyMode ? '🟢' : '🔴'} Emergency Mode
        </button>
        <button
          className={`stealth-btn ${isStealthMode ? 'active' : ''}`}
          onClick={onStealthMode}
        >
          {isStealthMode ? '🟢' : '🔴'} Stealth Mode
        </button>
        <button
          className={`panic-btn ${isPanicMode ? 'active' : ''}`}
          onClick={onPanicMode}
        >
          {isPanicMode ? '🟢' : '🔴'} Panic Mode
        </button>
      </div>

      {/* Quick Actions */}
      <div className="quick-actions">
        <h4>⚡ Quick Actions</h4>
        <button
          className={`network-action-btn ${scanning ? 'scanning' : ''}`}
          onClick={onScanNetwork}
          disabled={!backendConnected || scanning}
        >
          {scanning ? '🔍 Scanning...' : '🔍 Scan Network'}
        </button>
        <button 
          className="refresh-btn"
          onClick={onRefresh}
          disabled={!backendConnected}
        >
          🔄 Refresh
        </button>
        <button 
          className="lock-btn"
          onClick={onLockSystem}
        >
          🔒 Lock System
        </button>
        <button 
          className="backup-btn"
          onClick={onBackup}
          disabled={!backendConnected}
        >
          💾 Backup
        </button>
        <button 
          className="restart-btn"
          onClick={onRestart}
        >
          🔄 Restart
        </button>
        <button 
          className="stats-btn"
          onClick={onShowStats}
          disabled={!backendConnected}
        >
          📊 Stats
        </button>
        <button 
          className="config-btn"
          onClick={onShowConfig}
        >
          ⚙️ Config
        </button>
        <button 
          className="logs-btn"
          onClick={onShowLogs}
          disabled={!backendConnected}
        >
          📋 Logs
        </button>
      </div>

      {/* System Controls */}
      <div className="system-controls">
        <h4>🎛️ System Controls</h4>
        <div className="control-item">
          <label>Auto-backup</label>
          <input type="checkbox" defaultChecked />
        </div>
        <div className="control-item">
          <label>Threat Detection</label>
          <input type="checkbox" defaultChecked />
        </div>
        <div className="control-item">
          <label>Performance Monitoring</label>
          <input type="checkbox" defaultChecked />
        </div>
        <div className="control-item">
          <label>Network Optimization</label>
          <input type="checkbox" defaultChecked />
        </div>
        <div className="control-item">
          <label>Encryption Rotation</label>
          <input type="checkbox" defaultChecked />
        </div>
        <div className="control-item">
          <label>Peer Verification</label>
          <input type="checkbox" defaultChecked />
        </div>
      </div>

      {/* Connection Status */}
      <div className="connection-status">
        <h4>🔗 Connection Status</h4>
        <div className={`status-indicator ${backendConnected ? 'connected' : 'disconnected'}`}>
          <div className="status-dot"></div>
          <div className="status-text">
            {backendConnected ? 'Backend Connected' : 'Backend Disconnected'}
          </div>
        </div>
        {!backendConnected && (
          <div className="connection-warning">
            ⚠️ Configure backend URL to enable network features
          </div>
        )}
      </div>
    </div>
  );
};

export default ControlPanel; 