import React, { useState, useEffect } from 'react';

const STARTUP_LOGS = [
    "INITIALIZING GHOSTWIRE CORE V2.0.4...",
    "LOADING CRYPTOGRAPHIC MODULES [OK]",
    "MOUNTING SOVEREIGN IDENTITY ENGINE...",
    "did:ghost:7b2cbf...8f2a1 [VERIFIED]",
    "SCANNING PHYSICAL TRANSPORTS [MESH, BT, LORA]...",
    "ESTABLISHING SECURE MEMORY SEGMENTS...",
    "BOOTSTRAPPING PQC ARMOR LAYER [KYBER-1024]...",
    "CONNECTING TO ANCESTRAL NODES...",
    "SYNCHRONIZING SECURE LEDGER...",
    "MESH TOPOLOGY MAPPING COMPLETE.",
    "GHOSTWIRE NODE ONLINE."
];

const LoadingScreen: React.FC<{ onComplete: () => void }> = ({ onComplete }) => {
    const [currentLogs, setCurrentLogs] = useState<string[]>([]);
    const [progress, setProgress] = useState(0);

    useEffect(() => {
        let logIndex = 0;
        const logInterval = setInterval(() => {
            if (logIndex < STARTUP_LOGS.length) {
                setCurrentLogs(prev => [...prev, STARTUP_LOGS[logIndex]]);
                logIndex++;
                setProgress((logIndex / STARTUP_LOGS.length) * 100);
            } else {
                clearInterval(logInterval);
                // Give user a moment to see "NODE ONLINE"
                setTimeout(onComplete, 800);
            }
        }, 350);

        return () => clearInterval(logInterval);
    }, [onComplete]);

    return (
        <div className="fixed inset-0 z-[100] flex flex-col items-center justify-center bg-slate-950 text-white font-mono overflow-hidden">
            {/* Background Mesh Effect */}
            <div className="absolute inset-0 bg-[url('/mesh.svg')] opacity-5 animate-pulse" />

            {/* Logo Center */}
            <div className="relative mb-12">
                <div className="absolute inset-0 bg-purple-600/20 blur-[60px] rounded-full animate-pulse" />
                <img
                    src="/logo.png"
                    alt="GhostWire"
                    className="w-32 h-32 md:w-48 md:h-48 object-contain drop-shadow-[0_0_20px_rgba(123,44,191,0.6)] relative z-10"
                />
            </div>

            {/* Terminal Logs */}
            <div className="w-full max-w-lg px-6">
                <div className="h-64 bg-slate-900/60 border border-white/5 rounded-xl p-6 backdrop-blur-md flex flex-col gap-2 overflow-hidden shadow-2xl relative">
                    <div className="absolute top-0 left-0 w-full h-1 bg-white/5">
                        <div
                            className="h-full bg-indigo-500 transition-all duration-300 ease-out shadow-[0_0_10px_rgba(99,102,241,0.5)]"
                            style={{ width: `${progress}%` }}
                        />
                    </div>

                    <div className="flex-1 overflow-y-auto no-scrollbar pt-2 space-y-1">
                        {currentLogs.map((log, idx) => (
                            <div key={idx} className="flex gap-3 text-[10px] md:text-xs">
                                <span className="text-slate-600 font-bold">[{new Date().toLocaleTimeString()}]</span>
                                <span className={log.includes('OK') || log.includes('VERIFIED') || log.includes('ONLINE') ? 'text-emerald-400 font-black' : 'text-slate-300'}>
                                    {log}
                                </span>
                            </div>
                        ))}
                        <div className="w-2 h-4 bg-emerald-500/50 animate-pulse inline-block align-middle ml-1" />
                    </div>
                </div>

                <div className="mt-4 flex justify-between items-center text-[10px] uppercase tracking-[0.3em] font-black text-slate-600">
                    <span>SYSTEM_BOOT_SEQUENCE</span>
                    <span className="text-indigo-400">{Math.floor(progress)}%</span>
                </div>
            </div>

            {/* Footer */}
            <div className="fixed bottom-12 text-center">
                <p className="text-[10px] uppercase font-bold tracking-[0.5em] text-slate-700 animate-pulse">
                    Secure Mesh Connection Initializing
                </p>
            </div>
        </div>
    );
};

export default LoadingScreen;
