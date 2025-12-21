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

interface SystemPanelProps {
  systemStatus: SystemStatus;
  onSystemAction: (action: string) => void;
}

const SystemPanel: React.FC<SystemPanelProps> = ({
  systemStatus,
  onSystemAction
}) => {
  const formatUptime = (seconds: number) => {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;
    return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  };

  const getStatusLevel = (value: number) => {
    if (value >= 80) return 'critical';
    if (value >= 60) return 'warning';
    return 'optimal';
  };

  return (
    <div className="main-content-module">
      <div className="module-header">
        <h2 className="module-title">SYSTEM_CORE_DIAGNOSTICS</h2>
        <div className="module-subtitle">PRIMARY_KERNEL_STATE: ACTIVE</div>
      </div>

      <div className="module-grid">
        {/* Core Propulsion/Resource Sensors */}
        <div className="glass-module-card">
          <h3 className="card-title">RESOURCE_VITAL_SENSORS</h3>
          <div className="sensor-array-vertical">
            <div className="sensor-node">
              <div className="sensor-header">
                <span className="sensor-label">CORE_LOAD</span>
                <span className={`sensor-status-badge ${getStatusLevel(systemStatus.cpu)}`}>
                  {systemStatus.cpu}%
                </span>
              </div>
              <div className="sensor-bar-bg">
                <div className={`sensor-bar-fill ${getStatusLevel(systemStatus.cpu)}`} style={{ width: `${systemStatus.cpu}%` }}></div>
              </div>
            </div>

            <div className="sensor-node">
              <div className="sensor-header">
                <span className="sensor-label">MEMORY_POOL</span>
                <span className={`sensor-status-badge ${getStatusLevel(systemStatus.memory)}`}>
                  {systemStatus.memory}%
                </span>
              </div>
              <div className="sensor-bar-bg">
                <div className={`sensor-bar-fill ${getStatusLevel(systemStatus.memory)}`} style={{ width: `${systemStatus.memory}%` }}></div>
              </div>
            </div>

            <div className="sensor-node">
              <div className="sensor-header">
                <span className="sensor-label">NETWORK_BAND</span>
                <span className={`sensor-status-badge ${getStatusLevel(systemStatus.network)}`}>
                  {systemStatus.network}%
                </span>
              </div>
              <div className="sensor-bar-bg">
                <div className={`sensor-bar-fill ${getStatusLevel(systemStatus.network)}`} style={{ width: `${systemStatus.network}%` }}></div>
              </div>
            </div>
          </div>
        </div>

        {/* System Telemetry Module */}
        <div className="glass-module-card">
          <h3 className="card-title">PRIMARY_TELEMETRY</h3>
          <div className="telemetry-table">
            <div className="telemetry-row">
              <span className="telemetry-key">UPTIME_CLOCK</span>
              <span className="telemetry-val">{formatUptime(systemStatus.uptime)}</span>
            </div>
            <div className="telemetry-row">
              <span className="telemetry-key">PEER_NODES</span>
              <span className="telemetry-val">{systemStatus.connections}</span>
            </div>
            <div className="telemetry-row">
              <span className="telemetry-key">SEC_SCORE</span>
              <span className={`telemetry-val ${systemStatus.securityScore < 80 ? 'text-red-500' : 'text-cyan-400'}`}>
                {systemStatus.securityScore}%
              </span>
            </div>
            <div className="telemetry-row">
              <span className="telemetry-key">ENCRYPTION</span>
              <span className="telemetry-val">TLS_1.3_E2EE</span>
            </div>
          </div>
        </div>

        {/* System Management Overrides */}
        <div className="glass-module-card full-width">
          <h3 className="card-title">TACTICAL_SYSTEM_OVERRIDES</h3>
          <div className="action-grid-3">
            <button className="tactical-btn purple" onClick={() => onSystemAction('backup')}>
              <span className="btn-label">DATA_BACKUP</span>
              <span className="btn-subtext">Archive primary state</span>
            </button>
            <button className="tactical-btn cyan" onClick={() => onSystemAction('diagnostics')}>
              <span className="btn-label">RUN_DIAGNOSTICS</span>
              <span className="btn-subtext">Deep packet inspection</span>
            </button>
            <button className="tactical-btn orange" onClick={() => onSystemAction('restart')}>
              <span className="btn-label">KERNEL_RESTART</span>
              <span className="btn-subtext">Flush volatile buffers</span>
            </button>
          </div>
        </div>

        {/* Real-time System Log Buffer */}
        <div className="glass-module-card full-width">
          <h3 className="card-title">REALTIME_EVENT_BUFFER</h3>
          <div className="log-buffer">
            <div className="log-line">
              <span className="log-timestamp">[21:30:15]</span>
              <span className="log-tag tag-info">INFO</span>
              <span className="log-msg">GHOSTWIRE_CORE: Heartbeat stable.</span>
            </div>
            <div className="log-line">
              <span className="log-timestamp">[21:30:10]</span>
              <span className="log-tag tag-warn">WARN</span>
              <span className="log-msg">SIGNAL_STRENGTH: Jitter detected in packet routing.</span>
            </div>
            <div className="log-line">
              <span className="log-timestamp">[21:30:05]</span>
              <span className="log-tag tag-info">INFO</span>
              <span className="log-msg">MESH_TOPOLOGY: Node discovery phase complete.</span>
            </div>
            <div className="log-line">
              <span className="log-timestamp">[21:30:00]</span>
              <span className="log-tag tag-sec">SEC</span>
              <span className="log-msg">ENCRYPTION: Layer 2 handshakes verified.</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default SystemPanel;
