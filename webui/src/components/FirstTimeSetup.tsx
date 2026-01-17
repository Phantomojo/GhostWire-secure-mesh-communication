import { useState } from 'react';
import { api } from '../services/api';

interface FirstTimeSetupProps {
    onComplete: () => void;
}

const FirstTimeSetup = ({ onComplete }: FirstTimeSetupProps) => {
    const [step, setStep] = useState(1);
    const [nickname, setNickname] = useState('');
    const [password, setPassword] = useState('');
    const [confirmPassword, setConfirmPassword] = useState('');
    const [showPassword, setShowPassword] = useState(false);
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState('');

    const handleSetup = async (e: React.FormEvent) => {
        e.preventDefault();
        setError('');

        if (password !== confirmPassword) {
            setError('Passwords do not match');
            return;
        }

        if (password.length < 8) {
            setError('Password must be at least 8 characters');
            return;
        }

        setIsLoading(true);

        try {
            const response = await api.post('/identity/setup', {
                nickname,
                password
            });

            if (response && response.success) {
                onComplete();
            } else {
                setError(response?.error || 'Setup failed');
            }
        } catch (err: any) {
            // Handle error object properly
            const errorMessage = err?.message || 'Setup failed due to network or server error';
            setError(errorMessage);
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/95 backdrop-blur-xl text-green-500 font-mono">
            <div className="absolute inset-0 bg-[url('/mesh.svg')] opacity-10 animate-pulse"></div>

            <div className="relative z-10 w-full max-w-lg p-8 glass-card border border-green-500/30 rounded-lg shadow-[0_0_50px_rgba(0,255,0,0.1)]">
                <div className="text-center mb-8">
                    <h1 className="text-3xl font-bold mb-2 text-white">PROFILE ESTABLISHMENT</h1>
                    <p className="text-sm opacity-70">SOVEREIGN IDENTITY PHASE</p>
                </div>

                <div className="flex justify-center mb-8 space-x-2">
                    <div className={`h-2 w-12 rounded ${step >= 1 ? 'bg-green-500' : 'bg-green-900'}`}></div>
                    <div className={`h-2 w-12 rounded ${step >= 2 ? 'bg-green-500' : 'bg-green-900'}`}></div>
                </div>

                <form onSubmit={handleSetup} className="space-y-6">
                    {step === 1 && (
                        <div className="space-y-4 animate-fade-in">
                            <p className="text-center text-green-400 mb-4">Establish Profile Identity</p>
                            <div>
                                <label className="block text-xs opacity-70 mb-1">NICKNAME (PUBLIC)</label>
                                <input
                                    type="text"
                                    value={nickname}
                                    onChange={(e) => setNickname(e.target.value)}
                                    placeholder="Ghost_Operative"
                                    className="w-full bg-black/50 border border-green-500/50 rounded px-4 py-3 focus:outline-none focus:border-green-400 font-bold"
                                    autoFocus
                                />
                            </div>
                            <button
                                onClick={() => setStep(2)}
                                className="w-full bg-green-900/20 hover:bg-green-500/20 text-green-400 border border-green-500/50 hover:border-green-400 py-3 rounded font-bold transition-all mt-4"
                            >
                                NEXT SEQUENCE →
                            </button>
                        </div>
                    )}
                    {error && <p className="text-red-500 text-sm text-center">{error}</p>}

                    {step === 2 && (
                        <div className="space-y-4 animate-fade-in">
                            <p className="text-center text-green-400 mb-4">Secure Neural Link</p>
                            <div className="relative">
                                <label className="block text-xs opacity-70 mb-1">DATA ENCRYPTION PASSPHRASE</label>
                                <input
                                    type={showPassword ? "text" : "password"}
                                    value={password}
                                    onChange={(e) => setPassword(e.target.value)}
                                    placeholder="••••••••••••"
                                    className="w-full bg-black/50 border border-green-500/50 rounded px-4 py-3 focus:outline-none focus:border-green-400"
                                    autoFocus
                                />
                                <button
                                    type="button"
                                    onClick={() => setShowPassword(!showPassword)}
                                    className="absolute right-3 top-[34px] text-green-500/50 hover:text-green-500 transition-colors"
                                    title={showPassword ? "Hide Passphrase" : "Show Passphrase"}
                                >
                                    {showPassword ? '👁️‍🗨️' : '👁️'}
                                </button>
                            </div>
                            <div className="relative">
                                <label className="block text-xs opacity-70 mb-1">CONFIRM PASSPHRASE</label>
                                <input
                                    type={showPassword ? "text" : "password"}
                                    value={confirmPassword}
                                    onChange={(e) => setConfirmPassword(e.target.value)}
                                    placeholder="••••••••••••"
                                    className="w-full bg-black/50 border border-green-500/50 rounded px-4 py-3 focus:outline-none focus:border-green-400"
                                />
                            </div>

                            {error && (
                                <div className="text-red-500 text-center text-sm font-bold bg-red-900/20 py-2 rounded border border-red-500/30">
                                    ⚠️ {error}
                                </div>
                            )}

                            <div className="flex space-x-3 mt-4">
                                <button
                                    type="button"
                                    onClick={() => setStep(1)}
                                    className="flex-1 bg-transparent hover:bg-green-500/10 text-green-400 border border-green-500/30 py-3 rounded font-bold transition-all"
                                >
                                    ← BACK
                                </button>
                                <button
                                    type="submit"
                                    disabled={isLoading}
                                    className="flex-[2] bg-green-500 hover:bg-green-400 text-black border border-green-500 py-3 rounded font-bold transition-all disabled:opacity-50"
                                >
                                    {isLoading ? 'ENCRYPTING...' : 'ESTABLISH PROFILE'}
                                </button>
                            </div>
                        </div>
                    )}
                </form>
            </div>
        </div>
    );
};

export default FirstTimeSetup;
