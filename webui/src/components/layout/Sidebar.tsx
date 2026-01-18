import React, { useState } from 'react';
import { HiOutlineHome, HiOutlineGlobeAlt, HiOutlineCpuChip, HiOutlineChatBubbleLeftRight, HiOutlineExclamationTriangle, HiOutlineEye, HiOutlineLockClosed, HiOutlineChevronLeft, HiOutlineChevronRight, HiOutlineShieldExclamation, HiOutlineCog6Tooth, HiOutlineInformationCircle } from 'react-icons/hi2';
import UserProfile from '../UserProfile';

interface SidebarProps {
    activePanel: string;
    onPanelChange: (panel: any) => void;
    isEmergencyMode: boolean;
    isStealthMode: boolean;
    isPanicMode: boolean;
    onEmergencyMode: () => void;
    onStealthMode: () => void;
    onPanicMode: () => void;
    user: any;
    onUpdateMetadata?: (metadata: any) => void;
}

const Sidebar: React.FC<SidebarProps> = ({
    activePanel,
    onPanelChange,
    isEmergencyMode,
    isStealthMode,
    isPanicMode,
    onEmergencyMode,
    onStealthMode,
    onPanicMode,
    user,
    onUpdateMetadata
}) => {
    const [isCollapsed, setIsCollapsed] = useState(false);

    const navItems = [
        { id: 'dashboard', label: 'Dashboard', icon: HiOutlineHome },
        { id: 'network', label: 'Network', icon: HiOutlineGlobeAlt },
        { id: 'communication', label: 'Messages', icon: HiOutlineChatBubbleLeftRight },
        { id: 'security', label: 'Security', icon: HiOutlineShieldExclamation },
        { id: 'system', label: 'System', icon: HiOutlineCpuChip },
        { id: 'settings', label: 'Settings', icon: HiOutlineCog6Tooth },
        { id: 'about', label: 'About', icon: HiOutlineInformationCircle },
    ];

    return (
        <>
            {/* Desktop Sidebar */}
            <aside
                className={`hidden md:flex flex-col h-full glass-panel border-r border-white/5 z-20 overflow-y-auto no-scrollbar transition-all duration-500 cubic-bezier(0.16, 1, 0.3, 1) ${isCollapsed ? 'w-20' : 'w-72'
                    }`}
            >
                {/* Logo Section */}
                <div className={`relative transition-all duration-500 ${isCollapsed ? 'p-4 pt-10' : 'p-8'}`}>
                    <div className="flex items-center gap-4">
                        <div className={`
                            relative flex items-center justify-center shrink-0
                            ${isCollapsed ? 'w-12 h-12' : 'w-10 h-10'}
                            transition-all duration-500
                        `}>
                            <img
                                src="/logo.png"
                                alt="GhostWire"
                                className={`
                                    object-contain
                                    drop-shadow-[0_0_15px_rgba(123,44,191,0.6)]
                                    hover:drop-shadow-[0_0_25px_rgba(123,44,191,0.9)]
                                    transition-all duration-300
                                    ${isCollapsed ? 'w-12 h-12' : 'w-10 h-10'}
                                `}
                            />
                        </div>
                        {!isCollapsed && (
                            <div className="animate-fade-in whitespace-nowrap overflow-hidden">
                                <h1 className="text-xl font-black tracking-tight text-transparent bg-clip-text bg-gradient-to-r from-purple-400 via-indigo-400 to-cyan-400">
                                    GHOSTWIRE
                                </h1>
                                <p className="text-[9px] uppercase tracking-[0.3em] font-bold text-purple-400">Node Cluster V2</p>
                            </div>
                        )}
                    </div>

                    {/* Tactical Collapse Toggle */}
                    <button
                        onClick={() => setIsCollapsed(!isCollapsed)}
                        className={`
                            absolute flex items-center justify-center
                            w-6 h-10 bg-slate-900 border border-white/10 rounded-l-xl
                            text-slate-500 hover:text-indigo-400 transition-all z-30
                            ${isCollapsed ? 'right-0 top-1/2 -translate-y-1/2' : 'right-0 top-12'}
                            shadow-2xl hover:border-indigo-500/50
                        `}
                    >
                        {isCollapsed ? <HiOutlineChevronRight size={10} /> : <HiOutlineChevronLeft size={10} />}
                    </button>
                </div>

                {/* Primary Navigation */}
                <nav className={`flex-1 flex flex-col gap-1 transition-all duration-500 ${isCollapsed ? 'px-2 items-center' : 'px-4 mt-6'}`}>
                    {!isCollapsed && (
                        <p className="px-4 text-[10px] uppercase tracking-[0.2em] text-slate-500 font-black mb-4 opacity-50">Operations</p>
                    )}

                    {navItems.map((item) => {
                        const isActive = activePanel === item.id;
                        return (
                            <button
                                key={item.id}
                                onClick={() => onPanelChange(item.id)}
                                className={`
                                    relative flex items-center transition-all duration-300 group
                                    ${isCollapsed ? 'w-12 h-12 justify-center rounded-2xl' : 'w-full px-4 py-3.5 rounded-2xl gap-4'}
                                    ${isActive
                                        ? 'bg-indigo-600/20 text-white border border-indigo-500/20 shadow-lg shadow-indigo-500/5'
                                        : 'text-slate-400 hover:bg-white/[0.03] hover:text-white border border-transparent'}
                                `}
                                title={isCollapsed ? item.label : undefined}
                            >
                                <item.icon className={`text-xl transition-transform duration-300 ${isActive ? 'scale-110 text-indigo-400' : 'group-hover:scale-110 group-hover:text-white'}`} />

                                {!isCollapsed && (
                                    <span className={`text-sm font-bold tracking-wide transition-all ${isActive ? 'translate-x-1' : ''}`}>
                                        {item.label}
                                    </span>
                                )}

                                {/* Active Indicator Bar */}
                                {isActive && (
                                    <div className={`absolute bg-indigo-500 shadow-[0_0_15px_rgba(99,102,241,0.8)] rounded-full transition-all duration-500 ${isCollapsed ? 'left-0 w-1 h-3 top-1/2 -translate-y-1/2' : 'left-0 w-1 h-6 top-1/2 -translate-y-1/2'
                                        }`} />
                                )}
                            </button>
                        );
                    })}
                </nav>

                {/* Tactical Protocols (Compact Cluster) */}
                <div className={`transition-all duration-500 ${isCollapsed ? 'p-2' : 'p-6'}`}>
                    <div className={`
                        flex flex-col gap-2 transition-all duration-500
                        bg-slate-900/40 rounded-3xl border border-white/5 shadow-inner
                        ${isCollapsed ? 'p-2 py-4' : 'p-4'}
                    `}>
                        {!isCollapsed && (
                            <p className="text-[10px] uppercase tracking-[0.2em] text-slate-500 font-extrabold mb-2 opacity-50 px-2">Secure Link</p>
                        )}

                        {[
                            { id: 'emergency', label: 'Emergency', active: isEmergencyMode, action: onEmergencyMode, icon: HiOutlineExclamationTriangle },
                            { id: 'stealth', label: 'Stealth', active: isStealthMode, action: onStealthMode, icon: HiOutlineEye },
                            { id: 'panic', label: 'Panic', active: isPanicMode, action: onPanicMode, icon: HiOutlineLockClosed }
                        ].map(mode => (
                            <button
                                key={mode.id}
                                onClick={mode.action}
                                className={`
                                    flex items-center group transition-all duration-300 rounded-xl
                                    ${isCollapsed ? 'w-10 h-10 justify-center' : 'w-full p-2.5 justify-between bg-white/[0.02] border border-white/5 hover:border-indigo-500/30'}
                                    ${mode.active ? 'bg-indigo-600/30 text-white' : 'text-slate-500'}
                                `}
                                title={isCollapsed ? mode.label : undefined}
                            >
                                <div className="flex items-center gap-3">
                                    <div className={`
                                        transition-all duration-300
                                        ${mode.active ? 'text-indigo-400' : 'group-hover:text-slate-300'}
                                    `}>
                                        <mode.icon size={isCollapsed ? 18 : 16} />
                                    </div>
                                    {!isCollapsed && (
                                        <span className="text-xs font-bold tracking-tight">{mode.label}</span>
                                    )}
                                </div>

                                {!isCollapsed && (
                                    <div className={`w-2 h-2 rounded-full transition-all duration-300 ${mode.active ? 'bg-indigo-400 shadow-[0_0_8px_rgba(99,102,241,0.8)]' : 'bg-slate-800'
                                        }`} />
                                )}
                            </button>
                        ))}
                    </div>
                </div>

                {/* User Profile Section */}
                <div className={`mt-auto transition-all duration-500 ${isCollapsed ? 'p-4' : 'p-6'}`}>
                    {!isCollapsed ? (
                        <UserProfile user={user} onUpdateMetadata={onUpdateMetadata} />
                    ) : (
                        <button
                            className="w-12 h-12 rounded-2xl bg-slate-900 border border-white/5 flex items-center justify-center hover:bg-slate-800 transition-all mx-auto shadow-2xl"
                            onClick={() => setIsCollapsed(false)}
                        >
                            <span className="text-2xl">{user.avatar}</span>
                        </button>
                    )}
                </div>
            </aside>

            {/* Mobile Bottom Bar (Instagram-style) */}
            <nav className="md:hidden fixed bottom-6 left-6 right-6 h-20 glass-panel rounded-3xl border border-white/10 z-50 flex items-center justify-around px-2 shadow-2xl overflow-hidden">
                {navItems.map((item) => {
                    const isActive = activePanel === item.id;
                    return (
                        <button
                            key={item.id}
                            onClick={() => onPanelChange(item.id)}
                            className={`flex flex-col items-center justify-center gap-1 w-16 h-16 rounded-2xl transition-all duration-500 ${isActive ? 'bg-indigo-600 text-white shadow-lg shadow-indigo-600/30 -translate-y-2' : 'text-slate-500'
                                }`}
                        >
                            <item.icon className="text-2xl" />
                            <span className="text-[9px] font-black uppercase tracking-tighter">{item.label}</span>
                        </button>
                    );
                })}
            </nav>
        </>
    );
};

export default Sidebar;
