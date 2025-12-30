import React from 'react';
import { HiOutlineSignal, HiOutlineCpuChip, HiOutlineShieldCheck, HiOutlineServerStack } from 'react-icons/hi2';

interface HeaderProps {
    systemStatus: {
        cpu: number;
        memory: number;
        connections: number;
        securityScore: number;
    };
    backendConnected: boolean;
    onConfigClick: () => void;
}

const Header: React.FC<HeaderProps> = ({ systemStatus, backendConnected }) => {
    return (
        <header className="h-16 md:h-20 glass-panel border-b border-white/5 flex items-center justify-between px-4 md:px-8 z-10 sticky top-0 bg-slate-950/40 backdrop-blur-xl">
            {/* Left Section: Node Status */}
            <div className="flex items-center gap-6">
                <div className="flex items-center gap-3 bg-slate-900/40 px-4 py-2 rounded-2xl border border-white/5 shadow-inner transition-all duration-500 hover:border-indigo-500/30 group">
                    <div className="relative">
                        <div className={`w-2.5 h-2.5 rounded-full ${backendConnected ? 'bg-indigo-500' : 'bg-rose-500'} transition-all duration-500`} />
                        <div className={`absolute inset-0 rounded-full animate-ping opacity-20 ${backendConnected ? 'bg-indigo-400' : 'bg-rose-400'}`} />
                        {backendConnected && (
                            <div className="absolute inset-0 bg-indigo-500/40 blur-md rounded-full scale-150 animate-pulse" />
                        )}
                    </div>
                    <div className="flex flex-col">
                        <span className="text-[10px] font-black text-white uppercase tracking-[0.15em] leading-none">
                            {backendConnected ? 'NODE_ACTIVE' : 'NODE_OFFLINE'}
                        </span>
                        <span className="text-[8px] font-bold text-slate-500 uppercase tracking-widest mt-0.5 group-hover:text-indigo-400 transition-colors">
                            {backendConnected ? 'Secure Link Established' : 'Connection Interrupted'}
                        </span>
                    </div>
                </div>

                <div className="h-8 w-[1px] bg-white/5 hidden lg:block" />

                {/* Tactical Metrics Group */}
                <div className="hidden lg:flex items-center gap-8">
                    <div className="flex items-center gap-3 group cursor-default">
                        <div className="p-2 bg-indigo-500/10 rounded-xl group-hover:bg-indigo-500/20 transition-colors">
                            <HiOutlineSignal className="text-indigo-400 text-lg" />
                        </div>
                        <div className="flex flex-col">
                            <span className="text-[9px] text-slate-500 uppercase font-black tracking-widest">Peers</span>
                            <span className="text-sm font-black text-white leading-tight">{systemStatus.connections}</span>
                        </div>
                    </div>

                    <div className="flex items-center gap-3 group cursor-default">
                        <div className="p-2 bg-indigo-500/10 rounded-xl group-hover:bg-indigo-500/20 transition-colors">
                            <HiOutlineShieldCheck className="text-indigo-400 text-lg" />
                        </div>
                        <div className="flex flex-col">
                            <span className="text-[9px] text-slate-500 uppercase font-black tracking-widest">Security</span>
                            <span className="text-sm font-black text-white leading-tight">{systemStatus.securityScore}%</span>
                        </div>
                    </div>
                </div>
            </div>

            {/* Right Section: Resource Monitoring & Controls */}
            <div className="flex items-center gap-4">
                {/* Resource HUD Display */}
                <div className="hidden sm:flex items-center gap-6 bg-slate-900/60 px-5 py-2.5 rounded-2xl border border-white/5 shadow-inner">
                    {/* CPU Display */}
                    <div className="flex flex-col items-start gap-1.5 min-w-[100px]">
                        <div className="flex items-center justify-between w-full px-0.5">
                            <div className="flex items-center gap-1">
                                <HiOutlineCpuChip className="text-[10px] text-slate-500" />
                                <span className="text-[9px] text-slate-500 uppercase font-black tracking-widest">CPU_LOAD</span>
                            </div>
                            <span className="text-[10px] font-black text-indigo-400">{systemStatus.cpu}%</span>
                        </div>
                        <div className="w-full h-1 bg-slate-800/50 rounded-full overflow-hidden border border-white/5">
                            <div
                                className="h-full bg-indigo-500 shadow-[0_0_10px_rgba(99,102,241,0.5)] transition-all duration-1000 cubic-bezier(0.16, 1, 0.3, 1)"
                                style={{ width: `${systemStatus.cpu}%` }}
                            />
                        </div>
                    </div>

                    <div className="h-6 w-[1px] bg-white/5" />

                    {/* Memory Display */}
                    <div className="flex flex-col items-start gap-1.5 min-w-[100px]">
                        <div className="flex items-center justify-between w-full px-0.5">
                            <div className="flex items-center gap-1">
                                <HiOutlineServerStack className="text-[10px] text-slate-500" />
                                <span className="text-[9px] text-slate-500 uppercase font-black tracking-widest">RAM_MEM</span>
                            </div>
                            <span className="text-[10px] font-black text-indigo-400">{systemStatus.memory}%</span>
                        </div>
                        <div className="w-full h-1 bg-slate-800/50 rounded-full overflow-hidden border border-white/5">
                            <div
                                className="h-full bg-indigo-400 shadow-[0_0_10px_rgba(129,140,248,0.5)] transition-all duration-1000 cubic-bezier(0.16, 1, 0.3, 1)"
                                style={{ width: `${systemStatus.memory}%` }}
                            />
                        </div>
                    </div>
                </div>

                {/* Tactical Controls - Removed per user request for single source of truth */}
                <div className="flex items-center gap-2">
                    {/* Placeholder for future specific actions if needed */}
                </div>
            </div>
        </header>
    );
};

export default Header;
