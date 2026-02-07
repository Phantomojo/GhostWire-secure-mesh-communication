import { useState, useEffect } from 'react';
import { api } from '../services/api';
import MovingMeshBackground from './MovingMeshBackground';
import { HiOutlinePlus, HiOutlineSwitchHorizontal, HiOutlineChevronRight, HiOutlineLockClosed } from 'react-icons/hi';

interface LoginScreenProps {
    onUnlock: () => void;
    onProfileChange?: () => void;
    status: {
        locked: boolean;
        has_identity: boolean;
        did?: string;
        username?: string;
        setup_complete?: boolean;
    };
}

type LoginMode = 'unlock' | 'select' | 'create' | 'setup';

const LoginScreen = ({ onUnlock, onProfileChange, status }: LoginScreenProps) => {
    const [mode, setMode] = useState<LoginMode>('unlock');
    const [password, setPassword] = useState('');
    const [showPassword, setShowPassword] = useState(false);
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState('');

    // Setup Phase State
    const [setupNickname, setSetupNickname] = useState('');
    const [confirmPassword, setConfirmPassword] = useState('');
    const [setupStep, setSetupStep] = useState(1);

    // Profile Management State
    const [profiles, setProfiles] = useState<string[]>([]);
    const [newProfileName, setNewProfileName] = useState('');
    const [isFetchingProfiles, setIsFetchingProfiles] = useState(false);

    useEffect(() => {
        if (!status.username && !status.did) {
            setMode('select');
            fetchProfiles();
        } else if (!status.locked && status.setup_complete === false) {
            setMode('setup');
        }
    }, [status]);

    const fetchProfiles = async () => {
        setIsFetchingProfiles(true);
        try {
            const data = await api.listProfiles();
            setProfiles(data.profiles);
        } catch (err) {
            console.error('Failed to list profiles', err);
        } finally {
            setIsFetchingProfiles(false);
        }
    };

    const handleLogin = async (e: React.FormEvent) => {
        e.preventDefault();
        setIsLoading(true);
        setError('');

        try {
            const success = await api.unlockIdentity(password);
            if (success) {
                // Peek at status to see if setup is needed
                const statusCheck = await api.getIdentityStatus();
                if (statusCheck.setup_complete === false) {
                    setMode('setup');
                } else {
                    onUnlock();
                }
            } else {
                setError('Incorrect password');
                shake();
            }
        } catch (err) {
            setError('Login failed');
        } finally {
            setIsLoading(false);
        }
    };

    const handleSelectProfile = async (name: string) => {
        setIsLoading(true);
        try {
            const success = await api.selectProfile(name);
            if (success) {
                if (onProfileChange) onProfileChange();
                setMode('unlock');
            } else {
                setError(`Failed to select profile: ${name}`);
            }
        } catch (err) {
            setError('Selection error');
        } finally {
            setIsLoading(false);
        }
    };

    const handleCreateProfile = async (e: React.FormEvent) => {
        e.preventDefault();
        if (!newProfileName.trim()) return;

        setIsLoading(true);
        try {
            const success = await api.createProfile(newProfileName);
            if (success) {
                await handleSelectProfile(newProfileName);
            } else {
                setError('Profile creation failed');
            }
        } catch (err: any) {
            setError(err.message || 'Creation error');
        } finally {
            setIsLoading(false);
        }
    };

    const handleSetup = async (e: React.FormEvent) => {
        e.preventDefault();
        if (password.length < 8) {
            setError('Passphrase must be at least 8 characters');
            shake();
            return;
        }
        if (password !== confirmPassword) {
            setError('Passphrases do not match');
            shake();
            return;
        }

        setIsLoading(true);
        setError('');
        try {
            const response = await api.post('/identity/setup', {
                nickname: setupNickname,
                password
            });

            if (response && response.success) {
                onUnlock();
            } else {
                setError(response?.error || 'Setup failed');
                shake();
            }
        } catch (err: any) {
            setError(err.message || 'Setup communication error');
            shake();
        } finally {
            setIsLoading(false);
        }
    };

    const shake = () => {
        const form = document.getElementById('login-form');
        form?.classList.add('animate-shake');
        setTimeout(() => form?.classList.remove('animate-shake'), 500);
    };

    return (
        <div className="fixed inset-0 z-[100] bg-black text-green-500 font-mono overflow-hidden flex flex-col items-center justify-center">
            {/* Immersive Background */}
            <div className="absolute inset-0 opacity-50">
                <MovingMeshBackground severity={mode === 'create' ? 0.08 : 0.04} />
            </div>
            <div className="absolute inset-0 bg-gradient-to-t from-black via-transparent to-black pointer-events-none"></div>

            <div className="relative z-10 w-full max-w-4xl px-4 flex flex-col items-center">
                {/* Large Hero Branding */}
                <div className="text-center mb-12 animate-fade-in-down">
                    <div className="relative inline-block mb-6 group">
                        <div className="absolute inset-0 bg-purple-500/20 blur-3xl rounded-full scale-150 group-hover:bg-purple-500/40 transition-all duration-700"></div>
                        <img
                            src="/logo.png"
                            alt="GhostWire"
                            className="h-32 md:h-48 w-auto relative z-10 mx-auto object-contain drop-shadow-[0_0_40px_rgba(123,44,191,0.6)] animate-pulse-slow"
                        />
                    </div>
                    <h1 className="text-5xl md:text-7xl font-bold mb-4 tracking-tighter text-transparent bg-clip-text bg-gradient-to-r from-purple-400 via-indigo-400 to-cyan-400">
                        GHOSTWIRE
                    </h1>
                    <p className="text-sm md:text-base text-purple-300/60 tracking-[0.5em] font-light uppercase">
                        Secure Mesh Communication
                    </p>
                </div>

                {/* Form / Grid Area */}
                <div id="login-form" className="w-full max-w-md p-8 glass-panel border border-purple-500/20 rounded-2xl shadow-[0_0_80px_rgba(0,0,0,0.5)] transition-all duration-500">

                    {mode === 'unlock' && (
                        <div className="space-y-6 animate-fade-in">
                            <div className="text-center mb-6">
                                <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-purple-900/30 border border-purple-500/30 text-[10px] text-purple-300 uppercase tracking-widest mb-4">
                                    <HiOutlineLockClosed className="text-xs" /> Profile Protected
                                </div>
                                <h2 className="text-2xl font-bold text-white mb-1">
                                    {status.username || 'Sovereign Node'}
                                </h2>
                                <p className="text-[10px] text-purple-400/50 truncate max-w-[250px] mx-auto font-light italic">
                                    {status.did || 'Initializing Decentralized ID...'}
                                </p>
                            </div>

                            <form onSubmit={handleLogin} className="space-y-4">
                                <div className="relative group">
                                    <input
                                        type={showPassword ? "text" : "password"}
                                        value={password}
                                        onChange={(e) => setPassword(e.target.value)}
                                        placeholder="ENTER PASSPHRASE"
                                        className="w-full bg-black/40 border border-purple-500/30 rounded-xl px-4 py-4 text-center text-lg focus:outline-none focus:border-purple-400 focus:shadow-[0_0_20px_rgba(168,85,247,0.2)] transition-all placeholder-purple-900/50 text-white"
                                        autoFocus
                                    />
                                    <button
                                        type="button"
                                        onClick={() => setShowPassword(!showPassword)}
                                        className="absolute right-4 top-1/2 -translate-y-1/2 text-purple-500/40 hover:text-purple-400 transition-colors"
                                    >
                                        {showPassword ? '👁️‍🗨️' : '👁️'}
                                    </button>
                                </div>

                                {error && (
                                    <div className="text-red-400 text-center text-xs font-bold bg-red-900/10 py-3 rounded-lg border border-red-500/20 animate-shake">
                                        ACCESS DENIED: {error}
                                    </div>
                                )}

                                <button
                                    type="submit"
                                    disabled={isLoading}
                                    className="w-full bg-gradient-to-r from-purple-600 to-indigo-600 hover:from-purple-500 hover:to-indigo-500 text-white py-4 rounded-xl font-bold transition-all duration-300 shadow-lg shadow-purple-500/20 disabled:opacity-50 flex items-center justify-center gap-2 group"
                                >
                                    {isLoading ? (
                                        <span className="flex items-center gap-2">
                                            <div className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
                                            DECRYPTING...
                                        </span>
                                    ) : (
                                        <>UNLOCK PROFILE <HiOutlineChevronRight className="group-hover:translate-x-1 transition-transform" /></>
                                    )}
                                </button>
                            </form>

                            <button
                                onClick={() => { setMode('select'); fetchProfiles(); }}
                                className="w-full py-3 text-purple-400/60 hover:text-purple-300 text-xs font-bold transition-colors flex items-center justify-center gap-2"
                            >
                                <HiOutlineSwitchHorizontal /> SWITCH PROFILE
                            </button>
                        </div>
                    )}

                    {mode === 'select' && (
                        <div className="space-y-6 animate-fade-in">
                            <div className="text-center">
                                <h2 className="text-xl font-bold text-white mb-1 uppercase tracking-wider">Select Profile</h2>
                                <p className="text-[10px] text-purple-400/50">Choose an identity to initialize core</p>
                            </div>

                            <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 max-h-[400px] overflow-y-auto pr-2 scrollbar-thin scrollbar-thumb-purple-900/40">
                                {isFetchingProfiles ? (
                                    <div className="col-span-full text-center py-12">
                                        <div className="w-12 h-12 border-2 border-purple-500/20 border-t-purple-500 rounded-full animate-spin mx-auto mb-4"></div>
                                        <div className="text-purple-500/60 animate-pulse text-xs tracking-[0.2em] uppercase">Scanning identity clusters...</div>
                                    </div>
                                ) : profiles.length === 0 ? (
                                    <div className="col-span-full text-center py-12 text-purple-400/40 italic text-sm border border-dashed border-purple-900/20 rounded-2xl">
                                        No profiles detected in local vault
                                    </div>
                                ) : (
                                    profiles.map(p => (
                                        <button
                                            key={p}
                                            onClick={() => handleSelectProfile(p)}
                                            className="group relative flex flex-col p-5 rounded-2xl bg-gradient-to-br from-purple-900/10 to-transparent border border-purple-500/10 hover:border-purple-400/40 hover:from-purple-900/20 transition-all duration-300 text-left overflow-hidden ring-1 ring-white/5 active:scale-[0.98]"
                                        >
                                            <div className="absolute -right-4 -top-4 opacity-5 group-hover:opacity-10 transition-opacity">
                                                <img src="/logo.png" alt="" className="w-24 h-24 rotate-12" />
                                            </div>
                                            <div className="w-10 h-10 rounded-lg bg-purple-500/10 flex items-center justify-center mb-4 group-hover:bg-purple-500/20 transition-colors">
                                                <span className="text-xl">👤</span>
                                            </div>
                                            <h3 className="text-white font-bold tracking-wide group-hover:text-purple-300 mb-1">{p}</h3>
                                            <div className="flex items-center gap-2">
                                                <div className="w-1 h-1 rounded-full bg-green-500 animate-pulse"></div>
                                                <span className="text-[9px] text-purple-400/40 uppercase tracking-widest font-bold">READY TO INITIALIZE</span>
                                            </div>
                                        </button>
                                    ))
                                )}
                            </div>

                            <div className="pt-4 space-y-3">
                                <button
                                    onClick={() => setMode('create')}
                                    className="w-full py-4 border border-dashed border-purple-500/30 rounded-xl text-purple-400 hover:text-purple-300 hover:border-purple-500/60 hover:bg-purple-900/10 transition-all flex items-center justify-center gap-2 text-sm"
                                >
                                    <HiOutlinePlus /> CREATE NEW PROFILE
                                </button>
                                {status.username && (
                                    <button
                                        onClick={() => setMode('unlock')}
                                        className="w-full py-2 text-purple-400/40 hover:text-purple-300 text-[10px] font-bold transition-colors uppercase tracking-[0.2em]"
                                    >
                                        Back to Login
                                    </button>
                                )}
                            </div>
                        </div>
                    )}

                    {mode === 'create' && (
                        <div className="space-y-6 animate-fade-in">
                            <div className="text-center">
                                <h2 className="text-xl font-bold text-white mb-1 uppercase tracking-wider">New Identity</h2>
                                <p className="text-[10px] text-purple-400/50">Initialize a unique cryptographic profile</p>
                            </div>

                            <form onSubmit={handleCreateProfile} className="space-y-4">
                                <div className="space-y-1">
                                    <label className="text-[10px] text-purple-500/60 uppercase tracking-widest pl-2">Profile Name</label>
                                    <input
                                        type="text"
                                        value={newProfileName}
                                        onChange={(e) => setNewProfileName(e.target.value)}
                                        placeholder="e.g. Shadow_Walker"
                                        className="w-full bg-black/60 border border-purple-500/30 rounded-xl px-4 py-4 text-center text-lg focus:outline-none focus:border-purple-400 focus:shadow-[0_0_30px_rgba(168,85,247,0.3)] transition-all placeholder-purple-900/30 text-white font-bold ring-inner ring-1 ring-white/5 shadow-inner"
                                        autoFocus
                                    />
                                    <p className="text-[9px] text-purple-500/40 px-2 italic">Use alphanumeric characters only.</p>
                                </div>

                                {error && (
                                    <div className="text-red-400 text-center text-xs font-bold bg-red-900/10 py-3 rounded-lg border border-red-500/20">
                                        {error}
                                    </div>
                                )}

                                <button
                                    type="submit"
                                    disabled={isLoading || !newProfileName.trim()}
                                    className="w-full bg-purple-600 hover:bg-purple-500 text-white py-4 rounded-xl font-bold transition-all duration-300 disabled:opacity-50 flex items-center justify-center gap-2 group"
                                >
                                    {isLoading ? 'INITIALIZING...' : <>CREATE PROFILE <HiOutlineChevronRight className="group-hover:translate-x-1 transition-transform" /></>}
                                </button>
                            </form>

                            <button
                                onClick={() => setMode('select')}
                                className="w-full py-2 text-purple-400/40 hover:text-purple-300 text-[10px] font-bold transition-colors uppercase tracking-[0.2em]"
                            >
                                Cancel
                            </button>
                        </div>
                    )}

                    {mode === 'setup' && (
                        <div className="space-y-6 animate-fade-in">
                            <div className="text-center">
                                <h2 className="text-xl font-bold text-white mb-1 uppercase tracking-wider">Establish Profile</h2>
                                <p className="text-[10px] text-purple-400/50">Sovereign Identity Phase • Step {setupStep} of 2</p>
                            </div>

                            <div className="flex justify-center gap-2 mb-4">
                                <div className={`h-1 w-10 rounded-full transition-colors ${setupStep >= 1 ? 'bg-purple-500' : 'bg-purple-900/50'}`}></div>
                                <div className={`h-1 w-10 rounded-full transition-colors ${setupStep >= 2 ? 'bg-purple-500' : 'bg-purple-900/50'}`}></div>
                            </div>

                            <form onSubmit={(e) => { e.preventDefault(); if (setupStep === 1) setSetupStep(2); else handleSetup(e); }} className="space-y-4">
                                {setupStep === 1 ? (
                                    <div className="space-y-4 animate-fade-in">
                                        <div className="space-y-1">
                                            <label className="text-[10px] text-purple-500/60 uppercase tracking-widest pl-2">Display Nickname</label>
                                            <input
                                                type="text"
                                                value={setupNickname}
                                                onChange={(e) => setSetupNickname(e.target.value)}
                                                placeholder="e.g. Ghost_Operative"
                                                className="w-full bg-black/60 border border-purple-500/30 rounded-xl px-4 py-4 text-center text-lg focus:outline-none focus:border-purple-400 transition-all text-white font-bold"
                                                autoFocus
                                            />
                                        </div>
                                        <button
                                            type="button"
                                            onClick={() => setSetupStep(2)}
                                            className="w-full bg-purple-600 hover:bg-purple-500 text-white py-4 rounded-xl font-bold transition-all flex items-center justify-center gap-2 group"
                                        >
                                            NEXT SEQUENCE <HiOutlineChevronRight className="group-hover:translate-x-1 transition-transform" />
                                        </button>
                                    </div>
                                ) : (
                                    <div className="space-y-4 animate-fade-in">
                                        <div className="space-y-4">
                                            <div className="space-y-1">
                                                <label className="text-[10px] text-purple-500/60 uppercase tracking-widest pl-2">New Secure Passphrase</label>
                                                <div className="relative">
                                                    <input
                                                        type={showPassword ? "text" : "password"}
                                                        value={password}
                                                        onChange={(e) => setPassword(e.target.value)}
                                                        placeholder="••••••••••••"
                                                        className="w-full bg-black/60 border border-purple-500/30 rounded-xl px-4 py-4 text-center text-lg focus:outline-none focus:border-purple-400 transition-all text-white"
                                                        autoFocus
                                                    />
                                                    <button
                                                        type="button"
                                                        onClick={() => setShowPassword(!showPassword)}
                                                        className="absolute right-4 top-1/2 -translate-y-1/2 text-purple-500/40 hover:text-purple-400 transition-colors"
                                                    >
                                                        {showPassword ? '👁️‍🗨️' : '👁️'}
                                                    </button>
                                                </div>
                                            </div>
                                            <div className="space-y-1">
                                                <label className="text-[10px] text-purple-500/60 uppercase tracking-widest pl-2">Confirm Passphrase</label>
                                                <input
                                                    type={showPassword ? "text" : "password"}
                                                    value={confirmPassword}
                                                    onChange={(e) => setConfirmPassword(e.target.value)}
                                                    placeholder="••••••••••••"
                                                    className="w-full bg-black/60 border border-purple-500/30 rounded-xl px-4 py-4 text-center text-lg focus:outline-none focus:border-purple-400 transition-all text-white"
                                                />
                                            </div>
                                        </div>

                                        {error && (
                                            <div className="text-red-400 text-center text-xs font-bold bg-red-900/10 py-3 rounded-lg border border-red-500/20">
                                                {error}
                                            </div>
                                        )}

                                        <div className="flex gap-3">
                                            <button
                                                type="button"
                                                onClick={() => setSetupStep(1)}
                                                className="flex-1 py-4 border border-purple-500/30 rounded-xl text-purple-400 hover:text-purple-300 transition-all text-xs font-bold"
                                            >
                                                BACK
                                            </button>
                                            <button
                                                type="submit"
                                                disabled={isLoading}
                                                className="flex-[2] bg-purple-600 hover:bg-purple-500 text-white py-4 rounded-xl font-bold transition-all disabled:opacity-50 flex items-center justify-center gap-2"
                                            >
                                                {isLoading ? 'ENCRYPTING...' : 'ESTABLISH IDENTITY'}
                                            </button>
                                        </div>
                                    </div>
                                )}
                            </form>
                        </div>
                    )}

                </div>

                <div className="mt-12 text-center text-[10px] text-purple-500/40 uppercase tracking-[0.4em] font-light">
                    GhostWire Protocol v2.0-Alpha • Phase 1 Protection Active
                </div>
            </div>

            <style>{`
                @keyframes pulse-slow {
                    0%, 100% { opacity: 0.8; transform: scale(1); filter: drop-shadow(0 0 30px rgba(123,44,191,0.4)); }
                    50% { opacity: 1; transform: scale(1.02); filter: drop-shadow(0 0 50px rgba(123,44,191,0.8)); }
                }
                .animate-pulse-slow {
                    animation: pulse-slow 4s cubic-bezier(0.4, 0, 0.6, 1) infinite;
                }
                @keyframes fade-in-down {
                    from { opacity: 0; transform: translateY(-20px); }
                    to { opacity: 1; transform: translateY(0); }
                }
                .animate-fade-in-down {
                    animation: fade-in-down 1s ease-out forwards;
                }
                @keyframes fade-in {
                    from { opacity: 0; transform: scale(0.98); }
                    to { opacity: 1; transform: scale(1); }
                }
                .animate-fade-in {
                    animation: fade-in 0.4s ease-out forwards;
                }
                .glass-panel {
                    background: rgba(15, 5, 25, 0.7);
                    backdrop-filter: blur(20px);
                }
                .animate-shake {
                    animation: shake 0.5s cubic-bezier(.36,.07,.19,.97) both;
                }
                @keyframes shake {
                    10%, 90% { transform: translate3d(-1px, 0, 0); }
                    20%, 80% { transform: translate3d(2px, 0, 0); }
                    30%, 50%, 70% { transform: translate3d(-4px, 0, 0); }
                    40%, 60% { transform: translate3d(4px, 0, 0); }
                }
            `}</style>
        </div>
    );
};

export default LoginScreen;
