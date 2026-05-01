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
    return `${hours}H ${minutes}M`;
  };

  const getStatusClass = (value: number) => {
    if (value >= 80) return 'status-critical';
    if (value >= 60) return 'status-warning';
    return 'status-optimal';
  };

  return (
    <div className={`cockpit-hud-top ${isEmergencyMode ? 'emergency-state' : ''} ${isStealthMode ? 'stealth-state' : ''} ${isPanicMode ? 'panic-state' : ''}`}>
      <div className="hud-frame-left"></div>
      <div className="hud-frame-right"></div>

      <div className="hud-instruments">
        {/* Core Vital Sensors */}
        <div className="sensor-array">
          <div className="sensor-item">
            <span className="sensor-label">CPU_CORE</span>
            <div className={`sensor-meter ${getStatusClass(systemStatus.cpu)}`}>
              <div className="meter-fill" style={{ width: `${systemStatus.cpu}%` }}></div>
              <div className="meter-glitch"></div>
            </div>
            <span className="sensor-value">{systemStatus.cpu}%</span>
          </div>

          <div className="sensor-item">
            <span className="sensor-label">MEM_BANK</span>
            <div className={`sensor-meter ${getStatusClass(systemStatus.memory)}`}>
              <div className="meter-fill" style={{ width: `${systemStatus.memory}%` }}></div>
            </div>
            <span className="sensor-value">{systemStatus.memory}%</span>
          </div>

          <div className="sensor-item">
            <span className="sensor-label">NET_LINK</span>
            <div className={`sensor-meter ${getStatusClass(systemStatus.network)}`}>
              <div className="meter-fill" style={{ width: `${systemStatus.network}%` }}></div>
            </div>
            <span className="sensor-value">{systemStatus.network}%</span>
          </div>
        </div>

        {/* Tactical Telemetry */}
        <div className="telemetry-deck">
          <div className="telemetry-item">
            <div className="telemetry-icon">⚡</div>
            <div className="telemetry-content">
              <span className="telemetry-value">{formatUptime(systemStatus.uptime)}</span>
              <span className="telemetry-label">UPTIME_SYS</span>
            </div>
          </div>

          <div className="telemetry-divider"></div>

          <div className="telemetry-item">
            <div className="telemetry-icon">🛰️</div>
            <div className="telemetry-content">
              <span className="telemetry-value">{systemStatus.connections}</span>
              <span className="telemetry-label">PEER_NODES</span>
            </div>
          </div>

          <div className="telemetry-divider"></div>

          <div className="telemetry-item">
            <div className="telemetry-icon">🛡️</div>
            <div className="telemetry-content">
              <span className={`telemetry-value ${systemStatus.securityScore < 80 ? 'text-red-500' : 'text-cyan-400'}`}>
                {systemStatus.securityScore}%
              </span>
              <span className="telemetry-label">SEC_INTEGRITY</span>
            </div>
          </div>
        </div>

        {/* Propulsion/Status Indicators */}
        <div className="hud-status-node">
          <div className={`status-node ${backendConnected ? 'node-active' : 'node-failure'}`}>
            <div className="node-pulse"></div>
            <div className="node-label">{backendConnected ? 'LINK_ESTABLISHED' : 'LINK_LOST'}</div>
          </div>
          <button className="hud-config-btn" onClick={onConfigClick} title="System Configuration">
            <span className="btn-glimmer"></span>
            ⚙️
          </button>
        </div>
      </div>
    </div>
  );
};

export default StatusBar;
