import React from 'react';
import { GlassCard, SectionTitle, Subtitle, Badge, ActionButton } from './DesignSystem';
import {
  HiOutlineShieldCheck,
  HiOutlineLockClosed,
  HiOutlineShieldExclamation,
  HiOutlineDocumentMagnifyingGlass,
  HiOutlineKey,
  HiOutlineArrowPath,
  HiOutlineFingerPrint,
  HiOutlineFire,
  HiOutlineArchiveBox
} from 'react-icons/hi2';

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
  onSecurityScan: () => void;
  onThreatHunt: () => void;
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
  onSecurityScan,
  onThreatHunt,
  onSecurityAudit,
  onLockdown,
  onBackupSecurity,
  backendConnected
}) => {
  const getSeverityVariant = (severity: string): any => {
    switch (severity) {
      case 'critical': return 'danger';
      case 'high': return 'warning';
      case 'medium': return 'info';
      default: return 'indigo';
    }
  };

  const getSecurityStatus = () => {
    if (systemStatus.securityScore >= 90) return { label: 'Optimal', variant: 'success' as const };
    if (systemStatus.securityScore >= 70) return { label: 'Warning', variant: 'warning' as const };
    return { label: 'Compromised', variant: 'danger' as const };
  };

  const status = getSecurityStatus();

  return (
    <div className="space-y-8">
      <div className="flex justify-between items-end">
        <div>
          <SectionTitle>Security Ops</SectionTitle>
          <Subtitle>Active monitoring, encryption control, and threat mitigation</Subtitle>
        </div>
        <div className="flex gap-3 mb-6">
          <Badge variant={status.variant}>Status: {status.label}</Badge>
          <Badge variant="indigo">Score: {systemStatus.securityScore}</Badge>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
        {/* Alerts & Monitoring */}
        <div className="lg:col-span-2 space-y-8">
          <GlassCard className="min-h-[300px]">
            <div className="flex justify-between items-center mb-6">
              <h3 className="text-sm font-bold text-white uppercase tracking-widest flex items-center gap-2">
                <HiOutlineShieldExclamation className="text-indigo-400" />
                Security Alerts
              </h3>
              {securityAlerts.length > 0 && (
                <button className="text-[10px] font-bold text-slate-500 hover:text-white transition-colors uppercase tracking-widest">
                  Clear All
                </button>
              )}
            </div>

            <div className="space-y-4">
              {securityAlerts.length > 0 ? (
                securityAlerts.map(alert => (
                  <div key={alert.id} className="bg-white/5 border border-white/5 rounded-2xl p-4 flex flex-col sm:flex-row sm:items-center justify-between gap-4 hover:border-white/10 transition-all">
                    <div className="flex items-start gap-4">
                      <div className={`mt-1 p-2 rounded-lg ${alert.severity === 'critical' ? 'bg-rose-500/20 text-rose-400' :
                          alert.severity === 'high' ? 'bg-amber-500/20 text-amber-400' : 'bg-sky-500/20 text-sky-400'
                        }`}>
                        <HiOutlineShieldExclamation />
                      </div>
                      <div>
                        <div className="flex items-center gap-3 mb-1">
                          <span className="text-xs font-bold text-white">{alert.message}</span>
                          <Badge variant={getSeverityVariant(alert.severity)} className="scale-75 origin-left">
                            {alert.severity}
                          </Badge>
                        </div>
                        <span className="text-[10px] text-slate-500 font-mono">
                          {alert.timestamp.toLocaleTimeString()} • {alert.type.toUpperCase()}
                        </span>
                      </div>
                    </div>
                    <div className="flex gap-2">
                      <button
                        onClick={() => onAlertAction(alert.id, 'resolve')}
                        className="text-[10px] font-bold text-emerald-400 bg-emerald-500/10 hover:bg-emerald-500/20 px-3 py-1.5 rounded-lg transition-all"
                      >
                        Resolve
                      </button>
                      <button
                        onClick={() => onAlertAction(alert.id, 'investigate')}
                        className="text-[10px] font-bold text-slate-400 bg-white/5 hover:bg-white/10 px-3 py-1.5 rounded-lg transition-all"
                      >
                        Audit
                      </button>
                    </div>
                  </div>
                ))
              ) : (
                <div className="text-center py-12 border border-dashed border-white/5 rounded-3xl">
                  <div className="w-12 h-12 bg-emerald-500/10 text-emerald-500 rounded-full flex items-center justify-center mx-auto mb-4">
                    <HiOutlineShieldCheck className="text-2xl" />
                  </div>
                  <p className="text-xs font-bold text-slate-500 uppercase tracking-widest">No active threats detected</p>
                </div>
              )}
            </div>
          </GlassCard>

          <div className="grid grid-cols-1 sm:grid-cols-2 gap-8">
            <GlassCard>
              <h3 className="text-xs font-bold text-white uppercase tracking-widest mb-4 flex items-center gap-2">
                <HiOutlineKey className="text-indigo-400" />
                Encryption
              </h3>
              <div className="mb-6">
                <div className="flex justify-between items-end mb-2">
                  <span className="text-[10px] font-bold text-slate-500 uppercase">Entropy Level</span>
                  <span className="text-xs font-bold text-white">{systemStatus.encryption}%</span>
                </div>
                <div className="w-full h-1.5 bg-slate-900 rounded-full overflow-hidden">
                  <div
                    className="h-full bg-gradient-to-r from-indigo-500 to-violet-500 transition-all duration-1000"
                    style={{ width: `${systemStatus.encryption}%` }}
                  />
                </div>
              </div>
              <div className="grid grid-cols-2 gap-3">
                <ActionButton variant="secondary" onClick={onRotateKeys} disabled={!backendConnected} className="text-[10px] py-2">
                  Rotate Keys
                </ActionButton>
                <ActionButton variant="secondary" onClick={onUpgradeEncryption} disabled={!backendConnected} className="text-[10px] py-2">
                  Upgrade
                </ActionButton>
              </div>
            </GlassCard>

            <GlassCard>
              <h3 className="text-xs font-bold text-white uppercase tracking-widest mb-4 flex items-center gap-2">
                <HiOutlineFire className="text-rose-500" />
                Firewall
              </h3>
              <div className="flex items-center gap-3 mb-6 bg-emerald-500/10 p-3 rounded-xl border border-emerald-500/20">
                <div className="w-2 h-2 rounded-full bg-emerald-500 animate-pulse" />
                <span className="text-[10px] font-bold text-emerald-400 uppercase tracking-widest">Protection Active</span>
              </div>
              <div className="grid grid-cols-2 gap-3">
                <ActionButton variant="secondary" onClick={onConfigureFirewall} disabled={!backendConnected} className="text-[10px] py-2">
                  Configure
                </ActionButton>
                <ActionButton variant="secondary" onClick={onTestFirewall} disabled={!backendConnected} className="text-[10px] py-2">
                  Test Link
                </ActionButton>
              </div>
            </GlassCard>
          </div>
        </div>

        {/* Security Actions */}
        <div className="space-y-8">
          <GlassCard>
            <h3 className="text-sm font-bold text-white uppercase tracking-widest mb-6 flex items-center gap-2">
              <HiOutlineFingerPrint className="text-indigo-400" />
              Defense Actions
            </h3>
            <div className="space-y-3">
              <ActionButton
                onClick={onSecurityScan}
                className="w-full justify-start py-3"
              >
                <div className="flex items-center gap-3">
                  <HiOutlineDocumentMagnifyingGlass className="text-lg" />
                  <div className="text-left">
                    <div className="text-xs font-bold">Deep Core Scan</div>
                    <div className="text-[10px] text-indigo-300/60">Search for vulnerabilities</div>
                  </div>
                </div>
              </ActionButton>

              <ActionButton
                variant="secondary"
                onClick={onThreatHunt}
                className="w-full justify-start py-3"
              >
                <div className="flex items-center gap-3">
                  <HiOutlineShieldExclamation className="text-amber-500" />
                  <div className="text-left">
                    <div className="text-xs font-bold">Threat Hunting</div>
                    <div className="text-[10px] text-slate-500">Active pattern analysis</div>
                  </div>
                </div>
              </ActionButton>

              <ActionButton
                variant="secondary"
                onClick={onSecurityAudit}
                className="w-full justify-start py-3"
              >
                <div className="flex items-center gap-3">
                  <HiOutlineArrowPath className="text-sky-500" />
                  <div className="text-left">
                    <div className="text-xs font-bold">Security Audit</div>
                    <div className="text-[10px] text-slate-500">Compliance & peer check</div>
                  </div>
                </div>
              </ActionButton>

              <div className="h-px bg-white/5 my-4" />

              <ActionButton
                variant="danger"
                onClick={onLockdown}
                className="w-full justify-center py-4"
              >
                <div className="flex items-center gap-3">
                  <HiOutlineLockClosed className="text-lg" />
                  <span className="font-bold tracking-widest uppercase text-xs">Full Lockdown</span>
                </div>
              </ActionButton>
            </div>
          </GlassCard>

          <GlassCard>
            <h3 className="text-xs font-bold text-white uppercase tracking-widest mb-4 flex items-center gap-2">
              <HiOutlineArchiveBox className="text-slate-400" />
              Data Persistence
            </h3>
            <div className="grid grid-cols-1 gap-3">
              <ActionButton variant="secondary" onClick={onViewLogs} className="py-2.5 text-xs">
                Access Audit Logs
              </ActionButton>
              <ActionButton variant="secondary" onClick={onBackupSecurity} className="py-2.5 text-xs">
                Snapshot Config
              </ActionButton>
            </div>
          </GlassCard>
        </div>
      </div>
    </div>
  );
};

export default SecurityPanel;