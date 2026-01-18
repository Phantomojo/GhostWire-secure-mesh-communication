import React from 'react';

const About: React.FC = () => {
    return (
        <div className="space-y-8 animate-fade-in pb-12">
            {/* Hero Section */}
            <div className="relative overflow-hidden rounded-3xl bg-slate-900/40 border border-white/5 p-8 md:p-12 text-center group">
                <div className="absolute inset-0 bg-grid-pattern opacity-5" />

                {/* Visual Accent */}
                <div className="absolute top-0 left-1/2 -translate-x-1/2 w-96 h-96 bg-purple-600/10 blur-[100px] rounded-full pointer-events-none" />

                <div className="relative z-10 flex flex-col items-center">
                    <img
                        src="/logo.png"
                        alt="GhostWire Logo"
                        className="w-48 md:w-64 h-auto object-contain drop-shadow-[0_0_30px_rgba(123,44,191,0.5)] mb-8 transition-transform duration-700 group-hover:scale-105"
                    />

                    <h1 className="text-4xl md:text-6xl font-black text-transparent bg-clip-text bg-gradient-to-r from-purple-400 via-indigo-400 to-cyan-400 mb-4 font-display italic">
                        GHOSTWIRE
                    </h1>

                    <p className="text-lg md:text-xl text-slate-400 max-w-2xl font-medium tracking-wide">
                        The Sovereign Secure Mesh Communication Protocol for the Ancient Future.
                    </p>
                </div>
            </div>

            {/* Content Sections */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
                <div className="glass-panel p-8 rounded-2xl border border-white/5 bg-slate-900/40">
                    <h2 className="text-2xl font-black text-white mb-4 flex items-center gap-3">
                        <span className="w-8 h-8 rounded-lg bg-purple-600/20 flex items-center justify-center text-purple-400 text-lg">🛡️</span>
                        The Vision
                    </h2>
                    <p className="text-slate-400 leading-relaxed font-medium">
                        GhostWire is built on the philosophy of sovereign connectivity. In an era of centralized control and surveillance, GhostWire provides a decentralized, peer-to-peer mesh network that ensures your communications remain private, resilient, and truly yours.
                    </p>
                </div>

                <div className="glass-panel p-8 rounded-2xl border border-white/5 bg-slate-900/40">
                    <h2 className="text-2xl font-black text-white mb-4 flex items-center gap-3">
                        <span className="w-8 h-8 rounded-lg bg-indigo-600/20 flex items-center justify-center text-indigo-400 text-lg">⚡</span>
                        Secure Mesh
                    </h2>
                    <p className="text-slate-400 leading-relaxed font-medium">
                        Utilizing advanced cryptographic standards and mesh networking topologies, GhostWire nodes communicate directly over local networks, Bluetooth, and long-range radio links, creating a web of trust that doesn't rely on the legacy internet.
                    </p>
                </div>
            </div>

            {/* Aesthetic Section */}
            <div className="glass-panel p-8 rounded-2xl border border-white/5 bg-slate-900/60 relative overflow-hidden">
                <div className="absolute right-0 top-0 w-64 h-64 bg-cyan-600/10 blur-[80px] rounded-full" />

                <h2 className="text-2xl font-black text-white mb-6">Ancient Future Aesthetic</h2>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                    <div className="p-4 rounded-xl bg-slate-950/50 border border-white/5">
                        <h3 className="text-purple-400 font-black text-xs uppercase tracking-widest mb-2">Vibranium Palette</h3>
                        <p className="text-xs text-slate-500">Deep purples, glowing golds, and stealth blacks inspired by Afrofuturistic excellence.</p>
                    </div>
                    <div className="p-4 rounded-xl bg-slate-950/50 border border-white/5">
                        <h3 className="text-indigo-400 font-black text-xs uppercase tracking-widest mb-2">Nsibidi Script</h3>
                        <p className="text-xs text-slate-500">Ancient symbols meeting modern cryptography to create a visual language of security.</p>
                    </div>
                    <div className="p-4 rounded-xl bg-slate-950/50 border border-white/5">
                        <h3 className="text-cyan-400 font-black text-xs uppercase tracking-widest mb-2">Sovereign Link</h3>
                        <p className="text-xs text-slate-500">The connection between ancestors and the digital pioneers of tomorrow.</p>
                    </div>
                </div>
            </div>

            {/* Version Info */}
            <div className="text-center opacity-30 select-none">
                <p className="text-[10px] uppercase font-black tracking-[0.5em] text-slate-500">
                    GHOSTWIRE NODE CLUSTER V2.0.4-BETA // NODEID: SOVEREIGN_CORE
                </p>
            </div>
        </div>
    );
};

export default About;
