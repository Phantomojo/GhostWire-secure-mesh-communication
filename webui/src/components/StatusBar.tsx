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

interface StatusBarProps {
  systemStatus: SystemStatus;
  isEmergencyMode: boolean;
  isStealthMode: boolean;
  isPanicMode: boolean;
  backendConnected: boolean;
  onConfigClick: () => void;
}

const StatusBar: React.FC<StatusBarProps> = ({
  systemStatus,
  isEmergencyMode,
  isStealthMode,
  isPanicMode,
  backendConnected,
  onConfigClick
}) => {
  const formatUptime = (seconds: number) => {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    return `${hours}h ${minutes}m`;
  };

  const getStatusColor = (value: number) => {
    if (value >= 80) return '#ff4444';
    if (value >= 60) return '#ffaa00';
    return '#44ff44';
  };

  const getStatusClass = (value: number) => {
    if (value >= 80) return 'critical';
    if (value >= 60) return 'warning';
    return 'normal';
  };

  return (
    <div className={`status-bar ${isEmergencyMode ? 'emergency' : ''} ${isStealthMode ? 'stealth' : ''} ${isPanicMode ? 'panic' : ''}`}>
      <div className="status-instruments">
        {/* System Status Indicators */}
        <div className="instrument-group">
          <div className="instrument">
            <div className="instrument-label">CPU</div>
            <div className={`gauge ${getStatusClass(systemStatus.cpu)}`}>
              <div className="gauge-fill" style={{ width: `${systemStatus.cpu}%`, backgroundColor: getStatusColor(systemStatus.cpu) }}></div>
              <div className="gauge-value">{systemStatus.cpu}%</div>
            </div>
          </div>

          <div className="instrument">
            <div className="instrument-label">MEMORY</div>
            <div className={`gauge ${getStatusClass(systemStatus.memory)}`}>
              <div className="gauge-fill" style={{ width: `${systemStatus.memory}%`, backgroundColor: getStatusColor(systemStatus.memory) }}></div>
              <div className="gauge-value">{systemStatus.memory}%</div>
            </div>
          </div>

          <div className="instrument">
            <div className="instrument-label">NETWORK</div>
            <div className={`gauge ${getStatusClass(systemStatus.network)}`}>
              <div className="gauge-fill" style={{ width: `${systemStatus.network}%`, backgroundColor: getStatusColor(systemStatus.network) }}></div>
              <div className="gauge-value">{systemStatus.network}%</div>
            </div>
          </div>

          <div className="instrument">
            <div className="instrument-label">ENCRYPTION</div>
            <div className={`gauge ${getStatusClass(systemStatus.encryption)}`}>
              <div className="gauge-fill" style={{ width: `${systemStatus.encryption}%`, backgroundColor: getStatusColor(systemStatus.encryption) }}></div>
              <div className="gauge-value">{systemStatus.encryption}%</div>
            </div>
          </div>
        </div>

        {/* System Metrics */}
        <div className="instrument-group">
          <div className="metric">
            <div className="metric-icon">⏱️</div>
            <div className="metric-value">{formatUptime(systemStatus.uptime)}</div>
            <div className="metric-label">UPTIME</div>
          </div>

          <div className="metric">
            <div className="metric-icon">🔗</div>
            <div className="metric-value">{systemStatus.connections}</div>
            <div className="metric-label">CONNECTIONS</div>
          </div>

          <div className="metric">
            <div className="metric-icon">📨</div>
            <div className="metric-value">{systemStatus.messages}</div>
            <div className="metric-label">MESSAGES</div>
          </div>

          <div className="metric">
            <div className="metric-icon">🛡️</div>
            <div className="metric-value">{systemStatus.securityScore}</div>
            <div className="metric-label">SECURITY</div>
          </div>
        </div>

        {/* Mode Indicators */}
        <div className="mode-indicators">
          <div className={`mode-indicator ${isEmergencyMode ? 'active' : ''}`}>
            <div className="mode-icon">🚨</div>
            <div className="mode-text">EMERGENCY</div>
          </div>
          <div className={`mode-indicator ${isStealthMode ? 'active' : ''}`}>
            <div className="mode-icon">👻</div>
            <div className="mode-text">STEALTH</div>
          </div>
          <div className={`mode-indicator ${isPanicMode ? 'active' : ''}`}>
            <div className="mode-icon">💀</div>
            <div className="mode-text">PANIC</div>
          </div>
        </div>

        {/* Backend Status */}
        <div className="backend-status">
          <div className={`status-indicator ${backendConnected ? 'connected' : 'disconnected'}`}>
            <div className="status-icon">{backendConnected ? '🟢' : '🔴'}</div>
            <div className="status-text">{backendConnected ? 'BACKEND' : 'OFFLINE'}</div>
          </div>
          <button className="config-button" onClick={onConfigClick}>
            ⚙️
          </button>
        </div>
      </div>
    </div>
  );
};

export default StatusBar; 