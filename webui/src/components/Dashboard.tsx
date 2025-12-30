import { useState } from 'react';
import {
    HiOutlineMagnifyingGlass,
    HiOutlineSignal,
    HiOutlineArrowPath,
    HiOutlineExclamationTriangle,
    HiOutlineGlobeAlt,
    HiOutlineShieldCheck,
    HiOutlineChatBubbleLeftRight,
    HiOutlineCpuChip,
    HiOutlineServerStack,
    HiOutlineClock
} from 'react-icons/hi2';

interface DashboardProps {
    systemStatus: {
        cpu: number;
        memory: number;
        network: number;
        encryption: number;
        uptime: number;
        connections: number;
        messages: number;
        securityScore: number;
    };
    peerCount: number;
    threatCount: number;
    backendConnected: boolean;
    onScanNetwork: () => void;
    onBroadcast: () => void;
    onSync: () => void;
    onEmergencyMode: () => void;
    onNavigate: (panel: 'dashboard' | 'network' | 'security' | 'system' | 'communication' | 'settings') => void;
    isEmergencyMode: boolean;
    pqcEnabled: boolean;
}

export default function Dashboard({
    systemStatus,
    peerCount,
    threatCount,
    backendConnected,
    onScanNetwork,
    onBroadcast,
    onSync,
    onEmergencyMode,
    onNavigate,
    isEmergencyMode,
    pqcEnabled,
}: DashboardProps) {
    const [isScanning, setIsScanning] = useState(false);

    const getOverallStatus = () => {
        if (!backendConnected) return { status: 'OFFLINE', color: 'danger', sub: 'Connection Lost' };
        if (isEmergencyMode) return { status: 'EMERGENCY', color: 'danger', sub: 'Protocol Active' };
        if (threatCount > 0) return { status: 'WARNING', color: 'warning', sub: 'Threats Detected' };
        return { status: 'SECURE', color: 'secure', sub: 'Mesh Operational' };
    };

    const status = getOverallStatus();

    const handleScan = async () => {
        setIsScanning(true);
        await onScanNetwork();
        setTimeout(() => setIsScanning(false), 2000);
    };

    // TACTICAL ACCENT SYSTEM - Strict Palette
    // Secure: Cyan/Teal (Cyber)
    // Warning: Amber/Orange (Alert)
    // Danger: Red/Rose (Critical)
    const statusColors = {
        secure: {
            bg: 'from-cyan-950/40 to-slate-900/60',
            border: 'border-cyan-500/20',
            text: 'text-cyan-400',
            glow: 'shadow-[0_0_30px_rgba(34,211,238,0.15)]',
            pulse: 'bg-cyan-400',
            indicator: 'bg-cyan-950 border-cyan-500/30'
        },
        warning: {
            bg: 'from-amber-950/40 to-slate-900/60',
            border: 'border-amber-500/20',
            text: 'text-amber-400',
            glow: 'shadow-[0_0_30px_rgba(251,191,36,0.15)]',
            pulse: 'bg-amber-400',
            indicator: 'bg-amber-950 border-amber-500/30'
        },
        danger: {
            bg: 'from-red-950/40 to-slate-900/60',
            border: 'border-red-500/20',
            text: 'text-red-500',
            glow: 'shadow-[0_0_30px_rgba(239,68,68,0.15)]',
            pulse: 'bg-red-500',
            indicator: 'bg-red-950 border-red-500/30'
        },
    };

    const colors = statusColors[status.color as keyof typeof statusColors];

    const getTrafficLevel = (msgs: number) => {
        if (msgs > 100) return "High";
        if (msgs > 20) return "Moderate";
        return "Low";
    };

    const QuickActionButton = ({ icon: Icon, label, sub, action, active = false, spinning = false, type = 'default' }: any) => {
        const baseStyles = "group relative overflow-hidden p-4 md:p-5 rounded-xl border transition-all duration-300 active:scale-95 flex flex-col items-center justify-center gap-2";
        const variants = {
            default: "bg-slate-900/40 border-white/5 hover:border-cyan-500/30 hover:bg-cyan-950/20",
            danger: active
                ? "bg-red-950/40 border-red-500/50 shadow-[0_0_20px_rgba(239,68,68,0.2)]"
                : "bg-slate-900/40 border-white/5 hover:border-red-500/30 hover:bg-red-950/20"
        };

        return (
            <button
                onClick={action}
                disabled={spinning}
                className={`${baseStyles} ${type === 'danger' ? variants.danger : variants.default}`}
            >
                <div className={`text-2xl transition-transform duration-300 ${active ? 'text-red-400' : 'text-slate-400 group-hover:text-cyan-400'} ${spinning ? 'animate-spin' : 'group-hover:scale-110'}`}>
                    <Icon />
                </div>
                <div className="flex flex-col items-center">
                    <span className={`text-sm font-semibold tracking-wide ${active ? 'text-red-400' : 'text-slate-200 group-hover:text-white'}`}>
                        {label}
                    </span>
                    <span className="text-[10px] uppercase tracking-wider text-slate-500 font-medium">
                        {sub}
                    </span>
                </div>

                {/* Hover Glow Effect */}
                <div className="absolute inset-0 bg-gradient-to-tr from-white/0 via-white/0 to-white/5 opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none" />
            </button>
        );
    };

    const CardBase = ({ title, icon: Icon, value, subValue, meta, onClick, accentColor = "text-cyan-400", progress = 0 }: any) => (
        <div
            onClick={onClick}
            className="group relative bg-slate-900/40 border border-white/5 rounded-2xl p-4 md:p-5 lg:p-6 cursor-pointer hover:border-white/10 hover:bg-slate-900/60 transition-all duration-300 overflow-hidden"
        >
            <div className="flex justify-between items-start mb-4 md:mb-6 gap-3 md:gap-4">
                <div className="min-w-0 flex-1">
                    <h3 className="text-slate-400 text-xs md:text-sm font-medium uppercase tracking-wider mb-1 truncate">{title}</h3>
                    <div className={`text-lg md:text-xl lg:text-2xl font-bold font-mono ${accentColor} truncate leading-tight`}>{value}</div>
                </div>
                <div className="p-2 md:p-3 rounded-xl bg-white/5 text-slate-400 group-hover:text-white group-hover:scale-110 group-hover:bg-white/10 transition-all shadow-inner shrink-0">
                    <Icon className="w-5 h-5 md:w-6 md:h-6" />
                </div>
            </div>

            <div className="space-y-2 md:space-y-3">
                <div className="w-full bg-slate-950 rounded-full h-1 md:h-1.5 overflow-hidden border border-white/5">
                    <div
                        className={`h-full rounded-full transition-all duration-500 ${accentColor.replace('text-', 'bg-')}`}
                        style={{ width: `${progress}%` }}
                    />
                </div>
                <div className="flex justify-between items-center text-[10px] md:text-xs">
                    <span className="text-slate-500 truncate mr-2">{meta}</span>
                    <span className="text-slate-300 font-mono truncate">{subValue}</span>
                </div>
            </div>
        </div>
    );

    return (
        <div className="space-y-6 animate-fade-in max-w-full pb-8">
            {/* HERO STATUS - Responsive */}
            <div className={`relative overflow-hidden rounded-2xl border ${colors.border} bg-gradient-to-br ${colors.bg} p-1 ${colors.glow}`}>
                <div className="absolute inset-0 bg-grid-pattern opacity-10" />
                <div className="relative bg-slate-950/50 backdrop-blur-sm rounded-xl p-4 md:p-8">
                    <div className="flex flex-col xl:flex-row items-center justify-between gap-6 md:gap-8">
                        <div className="flex flex-col md:flex-row items-center gap-6 text-center md:text-left">
                            {/* Animated Status Ring */}
                            <div className={`relative flex items-center justify-center w-20 h-20 md:w-24 md:h-24 rounded-full border-4 ${colors.border.replace('border', 'border-opacity-30')} shrink-0`}>
                                <div className={`absolute inset-0 rounded-full animate-ping opacity-20 ${colors.pulse}`} />
                                <div className={`w-14 h-14 md:w-16 md:h-16 rounded-full ${colors.pulse} shadow-[0_0_30px_currentColor] opacity-90`} />
                            </div>

                            <div className="min-w-0">
                                <h2 className={`text-4xl md:text-5xl font-black tracking-tight ${colors.text} font-display mb-2 drop-shadow-lg break-words`}>
                                    {status.status}
                                </h2>
                                <div className="flex items-center justify-center md:justify-start gap-2 text-slate-400 font-mono text-xs md:text-sm">
                                    <div className={`w-2 h-2 rounded-full ${backendConnected ? 'bg-emerald-500' : 'bg-red-500'} animate-pulse shrink-0`} />
                                    <span className="truncate">{status.sub}</span>
                                </div>
                            </div>
                        </div>

                        {/* Top Level Stats */}
                        <div className="flex flex-wrap justify-center xl:justify-end gap-6 md:gap-12 pb-4 xl:pb-0 border-b xl:border-b-0 xl:border-l border-white/5 xl:pl-12 w-full xl:w-auto">
                            <div className="text-center group cursor-default min-w-[80px]">
                                <div className="text-2xl md:text-3xl font-bold text-white font-mono group-hover:text-cyan-400 transition-colors">{peerCount}</div>
                                <div className="text-[10px] uppercase tracking-[0.2em] text-slate-500 font-bold mt-1">Active Peers</div>
                            </div>
                            <div className="text-center group cursor-default min-w-[80px]">
                                <div className={`text-2xl md:text-3xl font-bold font-mono transition-colors ${threatCount > 0 ? 'text-amber-500' : 'text-emerald-500'}`}>{threatCount}</div>
                                <div className="text-[10px] uppercase tracking-[0.2em] text-slate-500 font-bold mt-1">Threats</div>
                            </div>
                            <div className="text-center group cursor-default min-w-[80px]">
                                <div className="text-2xl md:text-3xl font-bold text-indigo-400 font-mono">{systemStatus.securityScore}%</div>
                                <div className="text-[10px] uppercase tracking-[0.2em] text-slate-500 font-bold mt-1">Secured</div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            {/* QUICK ACTIONS */}
            <div className="grid grid-cols-2 lg:grid-cols-4 gap-4">
                <QuickActionButton
                    icon={HiOutlineMagnifyingGlass}
                    label="Scan Sector"
                    sub="Discover Peers"
                    action={handleScan}
                    spinning={isScanning}
                />
                <QuickActionButton
                    icon={HiOutlineSignal}
                    label="Broadcast"
                    sub="Mesh Wide"
                    action={onBroadcast}
                />
                <QuickActionButton
                    icon={HiOutlineArrowPath}
                    label="Sync Data"
                    sub="Update Ledger"
                    action={onSync}
                />
                <QuickActionButton
                    icon={HiOutlineExclamationTriangle}
                    label={isEmergencyMode ? "ABORT" : "LOCKDOWN"}
                    sub={isEmergencyMode ? "Protocol Active" : "Emergency"}
                    action={onEmergencyMode}
                    active={isEmergencyMode}
                    type="danger"
                />
            </div>

            {/* STATUS CARDS */}
            <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4 md:gap-6">
                <CardBase
                    title="Mesh Network"
                    icon={HiOutlineGlobeAlt}
                    value={peerCount > 0 ? "Connected" : "Searching"}
                    subValue={`${peerCount} APs`}
                    meta={`Latency: ${systemStatus.network}ms`}
                    progress={Math.min(100, (peerCount / 50) * 100)}
                    onClick={() => onNavigate('network')}
                    accentColor="text-cyan-400"
                />
                <CardBase
                    title="Cyber Security"
                    icon={HiOutlineShieldCheck}
                    value="Encrypted"
                    subValue={pqcEnabled ? "Kyber-1024" : "AES-256"}
                    meta={`Firewall: ${threatCount > 0 ? "Alert" : "Active"}`}
                    progress={systemStatus.securityScore}
                    onClick={() => onNavigate('security')}
                    accentColor="text-emerald-400"
                />
                <CardBase
                    title="Comm Channels"
                    icon={HiOutlineChatBubbleLeftRight}
                    value={`${systemStatus.messages} Msgs`}
                    subValue="Secure"
                    meta={`Traffic: ${getTrafficLevel(systemStatus.messages)}`}
                    progress={Math.min(100, systemStatus.messages)}
                    onClick={() => onNavigate('communication')}
                    accentColor="text-indigo-400"
                />
            </div>

            {/* SYSTEM TELEMETRY */}
            <div className="bg-slate-900/30 border border-white/5 rounded-xl p-4 flex flex-wrap items-center justify-between gap-4 text-xs font-mono">
                <div className="flex items-center gap-6 text-slate-400">
                    <div className="flex items-center gap-2">
                        <HiOutlineCpuChip className="text-lg text-slate-600" />
                        <span>CPU: <span className="text-white">{systemStatus.cpu}%</span></span>
                    </div>
                    <div className="flex items-center gap-2">
                        <HiOutlineServerStack className="text-lg text-slate-600" />
                        <span>RAM: <span className="text-white">{systemStatus.memory}%</span></span>
                    </div>
                    <div className="flex items-center gap-2">
                        <HiOutlineClock className="text-lg text-slate-600" />
                        <span>UPTIME: <span className="text-white">{(systemStatus.uptime / 3600).toFixed(1)}h</span></span>
                    </div>
                </div>
                <div className="flex items-center gap-2 text-slate-500">
                    <div className={`w-1.5 h-1.5 rounded-full ${backendConnected ? 'bg-emerald-500' : 'bg-red-500'}`} />
                    <span>{backendConnected ? "SYSTEM_ONLINE" : "SYSTEM_OFFLINE"}</span>
                </div>
            </div>
        </div>
    );
}
