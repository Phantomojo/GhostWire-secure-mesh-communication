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

interface SecurityAlert {
  id: string;
  type: 'warning' | 'error' | 'info';
  message: string;
  timestamp: Date;
  severity: 'low' | 'medium' | 'high' | 'critical';
}

interface SecurityPanelProps {
  securityAlerts: SecurityAlert[];
  systemStatus: SystemStatus;
  onAlertAction: (alertId: string, action: string) => void;
  onRotateKeys: () => void;
  onUpgradeEncryption: () => void;
  onConfigureFirewall: () => void;
  onTestFirewall: () => void;
  onViewLogs: () => void;
  onSecuritySettings: () => void;
  onManageAuth: () => void;
  onAuditAuth: () => void;
  onSecurityScan: () => void;
  onThreatHunt: () => void;
  onKeyRotation: () => void;
  onSecurityAudit: () => void;
  onLockdown: () => void;
  onBackupSecurity: () => void;
  backendConnected: boolean;
}

const SecurityPanel: React.FC<SecurityPanelProps> = ({
  securityAlerts,
  systemStatus,
  onAlertAction,
  onRotateKeys,
  onUpgradeEncryption,
  onConfigureFirewall,
  onTestFirewall,
  onViewLogs,
  onSecuritySettings,
  onManageAuth,
  onAuditAuth,
  onSecurityScan,
  onThreatHunt,
  onKeyRotation,
  onSecurityAudit,
  onLockdown,
  onBackupSecurity,
  backendConnected
}) => {
  const getSeverityIcon = (severity: string) => {
    switch (severity) {
      case 'critical': return '🚨';
      case 'high': return '⚠️';
      case 'medium': return '⚡';
      case 'low': return 'ℹ️';
      default: return '📋';
    }
  };

  const getTypeIcon = (type: string) => {
    switch (type) {
      case 'error': return '❌';
      case 'warning': return '⚠️';
      case 'info': return 'ℹ️';
      default: return '📋';
    }
  };

  const formatTime = (date: Date) => {
    return date.toLocaleTimeString();
  };

  const getSecurityStatus = () => {
    if (systemStatus.securityScore >= 90) return { status: 'SECURE', color: '#44ff44', icon: '🛡️' };
    if (systemStatus.securityScore >= 70) return { status: 'WARNING', color: '#ffaa00', icon: '⚠️' };
    return { status: 'CRITICAL', color: '#ff4444', icon: '🚨' };
  };

  const securityStatus = getSecurityStatus();

  return (
    <div className="security-panel">
      <div className="panel-header">
        <h2>🛡️ SECURITY OPERATIONS CENTER</h2>
        <div className="security-overview">
          <div className="security-status">
            <div className="status-indicator" style={{ color: securityStatus.color }}>
              <span className="status-icon">{securityStatus.icon}</span>
              <span className="status-text">{securityStatus.status}</span>
            </div>
          </div>
          <div className="security-metrics">
            <div className="metric">
              <span className="metric-label">Security Score:</span>
              <span className="metric-value">{systemStatus.securityScore}/100</span>
            </div>
            <div className="metric">
              <span className="metric-label">Active Threats:</span>
              <span className="metric-value">{securityAlerts.filter(a => a.severity === 'high' || a.severity === 'critical').length}</span>
            </div>
            <div className="metric">
              <span className="metric-label">Encryption:</span>
              <span className="metric-value">{systemStatus.encryption}%</span>
            </div>
          </div>
        </div>
      </div>

      <div className="panel-content">
        {/* Security Alerts */}
        <div className="security-alerts">
          <div className="section-title">SECURITY ALERTS</div>
          <div className="alerts-container">
            {securityAlerts.length === 0 ? (
              <div className="no-alerts">
                <div className="no-alerts-icon">✅</div>
                <div className="no-alerts-text">No active security alerts</div>
              </div>
            ) : (
              securityAlerts.map((alert) => (
                <div key={alert.id} className={`alert-card ${alert.severity}`}>
                  <div className="alert-header">
                    <div className="alert-severity">
                      <span className="severity-icon">{getSeverityIcon(alert.severity)}</span>
                      <span className="severity-text">{alert.severity.toUpperCase()}</span>
                    </div>
                    <div className="alert-time">{formatTime(alert.timestamp)}</div>
                  </div>
                  
                  <div className="alert-content">
                    <div className="alert-type">
                      <span className="type-icon">{getTypeIcon(alert.type)}</span>
                      <span className="type-text">{alert.type.toUpperCase()}</span>
                    </div>
                    <div className="alert-message">{alert.message}</div>
                  </div>
                  
                  <div className="alert-actions">
                    <button 
                      className="alert-action-btn"
                      onClick={() => onAlertAction(alert.id, 'acknowledge')}
                    >
                      ✓ ACKNOWLEDGE
                    </button>
                    <button 
                      className="alert-action-btn"
                      onClick={() => onAlertAction(alert.id, 'investigate')}
                    >
                      🔍 INVESTIGATE
                    </button>
                    <button 
                      className="alert-action-btn"
                      onClick={() => onAlertAction(alert.id, 'resolve')}
                    >
                      ✅ RESOLVE
                    </button>
                  </div>
                </div>
              ))
            )}
          </div>
        </div>

        {/* Security Controls */}
        <div className="security-controls">
          <div className="section-title">SECURITY CONTROLS</div>
          <div className="controls-grid">
            <div className="control-card">
              <div className="control-header">
                <div className="control-icon">🔒</div>
                <div className="control-title">ENCRYPTION</div>
              </div>
              <div className="control-status">
                <div className="status-bar">
                  <div 
                    className="status-fill" 
                    style={{ width: `${systemStatus.encryption}%` }}
                  ></div>
                </div>
                <div className="status-text">{systemStatus.encryption}% Active</div>
              </div>
              <div className="control-actions">
                <button 
                  className="control-btn"
                  onClick={onRotateKeys}
                  disabled={!backendConnected}
                >
                  ROTATE KEYS
                </button>
                <button 
                  className="control-btn"
                  onClick={onUpgradeEncryption}
                  disabled={!backendConnected}
                >
                  UPGRADE
                </button>
              </div>
            </div>

            <div className="control-card">
              <div className="control-header">
                <div className="control-icon">🛡️</div>
                <div className="control-title">FIREWALL</div>
              </div>
              <div className="control-status">
                <div className="status-indicator active">
                  <span className="status-dot"></span>
                  <span className="status-text">ACTIVE</span>
                </div>
              </div>
              <div className="control-actions">
                <button 
                  className="control-btn"
                  onClick={onConfigureFirewall}
                  disabled={!backendConnected}
                >
                  CONFIGURE
                </button>
                <button 
                  className="control-btn"
                  onClick={onTestFirewall}
                  disabled={!backendConnected}
                >
                  TEST
                </button>
              </div>
            </div>

            <div className="control-card">
              <div className="control-header">
                <div className="control-icon">👁️</div>
                <div className="control-title">MONITORING</div>
              </div>
              <div className="control-status">
                <div className="status-indicator active">
                  <span className="status-dot"></span>
                  <span className="status-text">ACTIVE</span>
                </div>
              </div>
              <div className="control-actions">
                <button 
                  className="control-btn"
                  onClick={onViewLogs}
                  disabled={!backendConnected}
                >
                  VIEW LOGS
                </button>
                <button 
                  className="control-btn"
                  onClick={onSecuritySettings}
                  disabled={!backendConnected}
                >
                  SETTINGS
                </button>
              </div>
            </div>

            <div className="control-card">
              <div className="control-header">
                <div className="control-icon">🔐</div>
                <div className="control-title">AUTHENTICATION</div>
              </div>
              <div className="control-status">
                <div className="status-indicator active">
                  <span className="status-dot"></span>
                  <span className="status-text">ACTIVE</span>
                </div>
              </div>
              <div className="control-actions">
                <button 
                  className="control-btn"
                  onClick={onManageAuth}
                  disabled={!backendConnected}
                >
                  MANAGE
                </button>
                <button 
                  className="control-btn"
                  onClick={onAuditAuth}
                  disabled={!backendConnected}
                >
                  AUDIT
                </button>
              </div>
            </div>
          </div>
        </div>

        {/* Security Actions */}
        <div className="security-actions">
          <div className="section-title">SECURITY ACTIONS</div>
          <div className="actions-grid">
            <button 
              className="security-action-btn"
              onClick={onSecurityScan}
              disabled={!backendConnected}
            >
              <div className="action-icon">🔍</div>
              <div className="action-label">SECURITY SCAN</div>
            </button>
            
            <button 
              className="security-action-btn"
              onClick={onThreatHunt}
              disabled={!backendConnected}
            >
              <div className="action-icon">🛡️</div>
              <div className="action-label">THREAT HUNT</div>
            </button>
            
            <button 
              className="security-action-btn"
              onClick={onKeyRotation}
              disabled={!backendConnected}
            >
              <div className="action-icon">🔐</div>
              <div className="action-label">KEY ROTATION</div>
            </button>
            
            <button 
              className="security-action-btn"
              onClick={onSecurityAudit}
              disabled={!backendConnected}
            >
              <div className="action-icon">📊</div>
              <div className="action-label">SECURITY AUDIT</div>
            </button>
            
            <button 
              className="security-action-btn"
              onClick={onLockdown}
              disabled={!backendConnected}
            >
              <div className="action-icon">🚨</div>
              <div className="action-label">LOCKDOWN</div>
            </button>
            
            <button 
              className="security-action-btn"
              onClick={onBackupSecurity}
              disabled={!backendConnected}
            >
              <div className="action-icon">💾</div>
              <div className="action-label">BACKUP SECURITY</div>
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default SecurityPanel; 