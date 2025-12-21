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
  onLockSystem: () => void;
  onBackup: () => void;
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
  onLockSystem,
  onBackup,
  onShowLogs
}) => {
  return (
    <div className="control-panel">
      {/* Navigation Module */}
      <div className="sidebar-module">
        <h3 className="module-title">NAV_CORE</h3>
        <div className="panel-nav">
          <button
            className={`nav-btn ${activePanel === 'network' ? 'active' : ''}`}
            onClick={() => onPanelChange('network')}
          >
            🛰️ NETWORK_OPS
          </button>
          <button
            className={`nav-btn ${activePanel === 'security' ? 'active' : ''}`}
            onClick={() => onPanelChange('security')}
          >
            🛡️ DEFENSE_SYS
          </button>
          <button
            className={`nav-btn ${activePanel === 'system' ? 'active' : ''}`}
            onClick={() => onPanelChange('system')}
          >
            ⚙️ KERNEL_CTL
          </button>
          <button
            className={`nav-btn ${activePanel === 'communication' ? 'active' : ''}`}
            onClick={() => onPanelChange('communication')}
          >
            💬 COMM_RELAY
          </button>
        </div>
      </div>

      {/* Critical Override Module */}
      <div className="sidebar-module">
        <h3 className="module-title">CRITICAL_OVERRIDE</h3>
        <button
          className={`quick-control-btn emergency-btn ${isEmergencyMode ? 'active' : ''}`}
          onClick={onEmergencyMode}
        >
          <span>EMERGENCY_PRO</span>
          <span>{isEmergencyMode ? 'ON' : 'OFF'}</span>
        </button>
        <button
          className={`quick-control-btn stealth-btn ${isStealthMode ? 'active' : ''}`}
          onClick={onStealthMode}
        >
          <span>STEALTH_CLOAK</span>
          <span>{isStealthMode ? 'ON' : 'OFF'}</span>
        </button>
        <button
          className={`quick-control-btn panic-btn ${isPanicMode ? 'active' : ''}`}
          onClick={onPanicMode}
        >
          <span>PANIC_VENT</span>
          <span>{isPanicMode ? 'ON' : 'OFF'}</span>
        </button>
      </div>

      {/* Tactical Actions Module */}
      <div className="sidebar-module">
        <h3 className="module-title">TACTICAL_ACTIONS</h3>
        <div className="panel-nav">
          <button
            className="nav-btn"
            onClick={onScanNetwork}
            disabled={!backendConnected || scanning}
          >
            {scanning ? 'REFRESHING...' : 'SCAN_MESH'}
          </button>
          <button className="nav-btn" onClick={onShowLogs} disabled={!backendConnected}>
            VIEW_TELEMETRY
          </button>
          <button className="nav-btn" onClick={onBackup} disabled={!backendConnected}>
            DUMP_DATABASE
          </button>
          <button className="nav-btn" onClick={onLockSystem}>
            LOCK_CONSOLE
          </button>
        </div>
      </div>

      {/* Connection Buffer */}
      <div className={`status-node ${backendConnected ? 'node-active' : 'node-failure'}`} style={{ marginTop: 'auto' }}>
        <div className="node-pulse"></div>
        <div className="node-label">{backendConnected ? 'BACKEND_STABLE' : 'SIGNAL_LOST'}</div>
      </div>
    </div>
  );
};

export default ControlPanel;
