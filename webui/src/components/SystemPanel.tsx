import React from 'react';
import { GlassCard, SectionTitle, Subtitle, Badge, ActionButton } from './DesignSystem';
import {
  HiOutlineCpuChip,
  HiOutlineServerStack,
  HiOutlineShieldCheck,
  HiOutlineCommandLine,
  HiOutlineArrowUpOnSquare,
  HiOutlineArrowPath,
  HiOutlineMagnifyingGlass,
  HiOutlineCircleStack,
  HiOutlinePresentationChartLine
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
    return `${hours}h ${minutes}m`;
  };

  return (
    <div className="space-y-8 animate-fade-in">
      <div className="flex flex-col md:flex-row justify-between items-start md:items-end gap-4">
        <div>
          <SectionTitle>Node Health</SectionTitle>
          <Subtitle>Core resources and technical operations overview</Subtitle>
        </div>
        <div className="flex gap-2">
          <Badge variant="success" className="px-3 py-1">Online</Badge>
          <Badge variant="indigo" className="px-3 py-1 font-mono">{formatUptime(systemStatus.uptime)}</Badge>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
        {/* Simplified Resource Cards */}
        {[
          { label: 'CPU Usage', value: systemStatus.cpu, color: 'indigo', icon: HiOutlineCpuChip, desc: 'Core processing load' },
          { label: 'Memory', value: systemStatus.memory, color: 'sky', icon: HiOutlineCircleStack, desc: 'Storage & cache allocation' },
          { label: 'Signal', value: systemStatus.network, color: 'violet', icon: HiOutlinePresentationChartLine, desc: 'Data throughput' },
          { label: 'Security', value: systemStatus.securityScore, color: 'emerald', icon: HiOutlineShieldCheck, desc: 'Threat defense rating' }
        ].map((metric, idx) => (
          <GlassCard key={idx} className="p-6">
            <div className="flex items-center gap-3 mb-4">
              <div className={`p-2 rounded-xl bg-${metric.color}-500/10 text-${metric.color}-400`}>
                <metric.icon className="text-xl" />
              </div>
              <span className="text-[10px] font-bold text-slate-500 uppercase tracking-widest">{metric.label}</span>
            </div>
            <div className="flex items-end gap-2 mb-3">
              <span className="text-3xl font-bold text-white tracking-tight">{metric.value}%</span>
            </div>
            <div className="w-full h-1.5 bg-slate-900 rounded-full overflow-hidden">
              <div
                className={`h-full bg-${metric.color === 'emerald' ? 'emerald' : metric.color === 'sky' ? 'sky' : metric.color === 'violet' ? 'violet' : 'indigo'}-500 transition-all duration-1000`}
                style={{ width: `${metric.value}%` }}
              />
            </div>
          </GlassCard>
        ))}
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
        {/* Activity Stream */}
        <div className="lg:col-span-2">
          <GlassCard className="h-full">
            <h3 className="text-sm font-bold text-white uppercase tracking-widest mb-6 flex items-center gap-2">
              <HiOutlineCommandLine className="text-indigo-400" />
              Recent Activity
            </h3>
            <div className="space-y-4">
              {[
                { time: '14s ago', tag: 'Core', msg: 'Heartbeat stable. Primary link verified.', type: 'info' },
                { time: '2m ago', tag: 'Signal', msg: 'Minor jitter detected in packet routing at hop 3.', type: 'warn' },
                { time: '5m ago', tag: 'Mesh', msg: 'Auto-discovery complete. 12 peers synced.', type: 'info' },
                { time: '12m ago', tag: 'Sec', msg: 'End-to-end encryption key rotation successful.', type: 'sec' },
              ].map((log, idx) => (
                <div key={idx} className="flex gap-4 p-3 rounded-xl bg-white/[0.02] border border-white/5 group hover:bg-white/[0.05] transition-all">
                  <span className="text-[10px] font-mono text-slate-600 w-14 shrink-0 mt-0.5">{log.time}</span>
                  <div className="flex-1">
                    <div className="flex items-center gap-2 mb-1">
                      <span className={`text-[9px] font-bold uppercase px-1.5 py-0.5 rounded ${log.type === 'warn' ? 'bg-amber-500/10 text-amber-500' :
                        log.type === 'sec' ? 'bg-indigo-500/20 text-indigo-400 font-extrabold' : 'bg-indigo-500/10 text-indigo-400'
                        }`}>{log.tag}</span>
                      <span className="text-xs text-slate-300 group-hover:text-white transition-colors">{log.msg}</span>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </GlassCard>
        </div>

        {/* Tactical Overrides */}
        <GlassCard>
          <h3 className="text-sm font-bold text-white uppercase tracking-widest mb-6 flex items-center gap-2">
            <HiOutlineServerStack className="text-indigo-400" />
            Node Controls
          </h3>
          <div className="space-y-3">
            <ActionButton
              variant="secondary"
              onClick={() => onSystemAction('backup')}
              className="w-full justify-start py-4"
            >
              <HiOutlineArrowUpOnSquare className="text-sky-400 text-lg" />
              <div className="text-left ml-3">
                <div className="text-xs font-bold">Safe Snapshot</div>
                <div className="text-[10px] text-slate-500">Backup current session data</div>
              </div>
            </ActionButton>

            <ActionButton
              variant="secondary"
              onClick={() => onSystemAction('diagnostics')}
              className="w-full justify-start py-4"
            >
              <HiOutlineMagnifyingGlass className="text-indigo-400 text-lg" />
              <div className="text-left ml-3">
                <div className="text-xs font-bold">Deep Scan</div>
                <div className="text-[10px] text-slate-500">Analyze full stack performance</div>
              </div>
            </ActionButton>

            <div className="h-px bg-white/5 my-2" />

            <ActionButton
              variant="secondary"
              onClick={() => onSystemAction('restart')}
              className="w-full justify-start py-4 border-amber-500/20 text-amber-500 bg-amber-500/5 hover:bg-amber-500/10"
            >
              <HiOutlineArrowPath className="text-xl" />
              <div className="text-left ml-3">
                <div className="text-xs font-bold">Node Refresh</div>
                <div className="text-[10px] text-amber-500/60">Restart engine services</div>
              </div>
            </ActionButton>
          </div>
        </GlassCard>
      </div>
    </div>
  );
};

export default SystemPanel;
