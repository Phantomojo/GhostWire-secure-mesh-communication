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

  const getPerformanceStatus = (value: number) => {
    if (value >= 80) return { status: 'CRITICAL', color: '#ff4444', icon: '🚨' };
    if (value >= 60) return { status: 'WARNING', color: '#ffaa00', icon: '⚠️' };
    return { status: 'NORMAL', color: '#44ff44', icon: '✅' };
  };

  const cpuStatus = getPerformanceStatus(systemStatus.cpu);
  const memoryStatus = getPerformanceStatus(systemStatus.memory);
  const networkStatus = getPerformanceStatus(systemStatus.network);

  return (
    <div className="system-panel">
      <div className="panel-header">
        <h2>⚙️ SYSTEM MONITORING</h2>
        <div className="system-overview">
          <div className="system-uptime">
            <div className="uptime-label">SYSTEM UPTIME</div>
            <div className="uptime-value">{formatUptime(systemStatus.uptime)}</div>
          </div>
          <div className="system-health">
            <div className="health-label">SYSTEM HEALTH</div>
            <div className="health-value">GOOD</div>
          </div>
        </div>
      </div>

      <div className="panel-content">
        {/* Performance Metrics */}
        <div className="performance-metrics">
          <div className="section-title">PERFORMANCE METRICS</div>
          <div className="metrics-grid">
            <div className="metric-card">
              <div className="metric-header">
                <div className="metric-icon">🖥️</div>
                <div className="metric-title">CPU USAGE</div>
              </div>
              <div className="metric-value">
                <div className="circular-progress">
                  <div className="progress-ring">
                    <svg width="120" height="120">
                      <circle
                        cx="60"
                        cy="60"
                        r="50"
                        fill="none"
                        stroke="#2a2a2a"
                        strokeWidth="8"
                      />
                      <circle
                        cx="60"
                        cy="60"
                        r="50"
                        fill="none"
                        stroke={cpuStatus.color}
                        strokeWidth="8"
                        strokeDasharray={`${(systemStatus.cpu / 100) * 314} 314`}
                        strokeDashoffset="0"
                        transform="rotate(-90 60 60)"
                      />
                    </svg>
                    <div className="progress-text">{systemStatus.cpu}%</div>
                  </div>
                </div>
              </div>
              <div className="metric-status" style={{ color: cpuStatus.color }}>
                <span className="status-icon">{cpuStatus.icon}</span>
                <span className="status-text">{cpuStatus.status}</span>
              </div>
            </div>

            <div className="metric-card">
              <div className="metric-header">
                <div className="metric-icon">💾</div>
                <div className="metric-title">MEMORY USAGE</div>
              </div>
              <div className="metric-value">
                <div className="circular-progress">
                  <div className="progress-ring">
                    <svg width="120" height="120">
                      <circle
                        cx="60"
                        cy="60"
                        r="50"
                        fill="none"
                        stroke="#2a2a2a"
                        strokeWidth="8"
                      />
                      <circle
                        cx="60"
                        cy="60"
                        r="50"
                        fill="none"
                        stroke={memoryStatus.color}
                        strokeWidth="8"
                        strokeDasharray={`${(systemStatus.memory / 100) * 314} 314`}
                        strokeDashoffset="0"
                        transform="rotate(-90 60 60)"
                      />
                    </svg>
                    <div className="progress-text">{systemStatus.memory}%</div>
                  </div>
                </div>
              </div>
              <div className="metric-status" style={{ color: memoryStatus.color }}>
                <span className="status-icon">{memoryStatus.icon}</span>
                <span className="status-text">{memoryStatus.status}</span>
              </div>
            </div>

            <div className="metric-card">
              <div className="metric-header">
                <div className="metric-icon">🌐</div>
                <div className="metric-title">NETWORK LOAD</div>
              </div>
              <div className="metric-value">
                <div className="circular-progress">
                  <div className="progress-ring">
                    <svg width="120" height="120">
                      <circle
                        cx="60"
                        cy="60"
                        r="50"
                        fill="none"
                        stroke="#2a2a2a"
                        strokeWidth="8"
                      />
                      <circle
                        cx="60"
                        cy="60"
                        r="50"
                        fill="none"
                        stroke={networkStatus.color}
                        strokeWidth="8"
                        strokeDasharray={`${(systemStatus.network / 100) * 314} 314`}
                        strokeDashoffset="0"
                        transform="rotate(-90 60 60)"
                      />
                    </svg>
                    <div className="progress-text">{systemStatus.network}%</div>
                  </div>
                </div>
              </div>
              <div className="metric-status" style={{ color: networkStatus.color }}>
                <span className="status-icon">{networkStatus.icon}</span>
                <span className="status-text">{networkStatus.status}</span>
              </div>
            </div>
          </div>
        </div>

        {/* System Information */}
        <div className="system-info">
          <div className="section-title">SYSTEM INFORMATION</div>
          <div className="info-grid">
            <div className="info-card">
              <div className="info-header">
                <div className="info-icon">🔗</div>
                <div className="info-title">CONNECTIONS</div>
              </div>
              <div className="info-value">{systemStatus.connections}</div>
              <div className="info-label">Active Connections</div>
            </div>

            <div className="info-card">
              <div className="info-header">
                <div className="info-icon">📨</div>
                <div className="info-title">MESSAGES</div>
              </div>
              <div className="info-value">{systemStatus.messages}</div>
              <div className="info-label">Total Messages</div>
            </div>

            <div className="info-card">
              <div className="info-header">
                <div className="info-icon">🔒</div>
                <div className="info-title">ENCRYPTION</div>
              </div>
              <div className="info-value">{systemStatus.encryption}%</div>
              <div className="info-label">Encryption Level</div>
            </div>

            <div className="info-card">
              <div className="info-header">
                <div className="info-icon">🛡️</div>
                <div className="info-title">SECURITY</div>
              </div>
              <div className="info-value">{systemStatus.securityScore}/100</div>
              <div className="info-label">Security Score</div>
            </div>
          </div>
        </div>

        {/* System Controls */}
        <div className="system-controls">
          <div className="section-title">SYSTEM CONTROLS</div>
          <div className="controls-grid">
            <div className="control-group">
              <div className="group-title">MAINTENANCE</div>
              <div className="control-buttons">
                <button 
                  className="system-control-btn"
                  onClick={() => onSystemAction('backup')}
                >
                  <div className="btn-icon">💾</div>
                  <div className="btn-label">BACKUP SYSTEM</div>
                </button>
                
                <button 
                  className="system-control-btn"
                  onClick={() => onSystemAction('restore')}
                >
                  <div className="btn-icon">🔄</div>
                  <div className="btn-label">RESTORE SYSTEM</div>
                </button>
                
                <button 
                  className="system-control-btn"
                  onClick={() => onSystemAction('update')}
                >
                  <div className="btn-icon">⬆️</div>
                  <div className="btn-label">UPDATE FIRMWARE</div>
                </button>
              </div>
            </div>

            <div className="control-group">
              <div className="group-title">DIAGNOSTICS</div>
              <div className="control-buttons">
                <button 
                  className="system-control-btn"
                  onClick={() => onSystemAction('diagnostics')}
                >
                  <div className="btn-icon">🔍</div>
                  <div className="btn-label">RUN DIAGNOSTICS</div>
                </button>
                
                <button 
                  className="system-control-btn"
                  onClick={() => onSystemAction('performance-test')}
                >
                  <div className="btn-icon">📊</div>
                  <div className="btn-label">PERFORMANCE TEST</div>
                </button>
                
                <button 
                  className="system-control-btn"
                  onClick={() => onSystemAction('network-test')}
                >
                  <div className="btn-icon">🌐</div>
                  <div className="btn-label">NETWORK TEST</div>
                </button>
              </div>
            </div>

            <div className="control-group">
              <div className="group-title">SYSTEM</div>
              <div className="control-buttons">
                <button 
                  className="system-control-btn"
                  onClick={() => onSystemAction('restart')}
                >
                  <div className="btn-icon">🔄</div>
                  <div className="btn-label">RESTART SYSTEM</div>
                </button>
                
                <button 
                  className="system-control-btn"
                  onClick={() => onSystemAction('shutdown')}
                >
                  <div className="btn-icon">⏹️</div>
                  <div className="btn-label">SHUTDOWN</div>
                </button>
                
                <button 
                  className="system-control-btn"
                  onClick={() => onSystemAction('factory-reset')}
                >
                  <div className="btn-icon">🏭</div>
                  <div className="btn-label">FACTORY RESET</div>
                </button>
              </div>
            </div>
          </div>
        </div>

        {/* System Logs */}
        <div className="system-logs">
          <div className="section-title">SYSTEM LOGS</div>
          <div className="logs-container">
            <div className="log-entry">
              <div className="log-time">21:27:15</div>
              <div className="log-level info">INFO</div>
              <div className="log-message">System performance monitoring active</div>
            </div>
            <div className="log-entry">
              <div className="log-time">21:27:10</div>
              <div className="log-level warning">WARN</div>
              <div className="log-message">Memory usage approaching threshold</div>
            </div>
            <div className="log-entry">
              <div className="log-time">21:27:05</div>
              <div className="log-level info">INFO</div>
              <div className="log-message">Network optimization completed</div>
            </div>
            <div className="log-entry">
              <div className="log-time">21:27:00</div>
              <div className="log-level info">INFO</div>
              <div className="log-message">Security scan completed - no threats found</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default SystemPanel; 