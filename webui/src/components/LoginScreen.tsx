import { useState } from 'react';
import { api } from '../services/api';

interface LoginScreenProps {
    onUnlock: () => void;
    status: {
        locked: boolean;
        has_identity: boolean;
        did?: string;
        username?: string;
    };
}

const LoginScreen = ({ onUnlock, status }: LoginScreenProps) => {
    const [password, setPassword] = useState('');
    const [showPassword, setShowPassword] = useState(false);
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState('');

    const handleLogin = async (e: React.FormEvent) => {
        e.preventDefault();
        setIsLoading(true);
        setError('');

        try {
            const success = await api.unlockIdentity(password);
            if (success) {
                onUnlock();
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

    const shake = () => {
        const form = document.getElementById('login-form');
        form?.classList.add('animate-shake');
        setTimeout(() => form?.classList.remove('animate-shake'), 500);
    };

    return (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/95 backdrop-blur-xl text-green-500 font-mono">
            <div className="absolute inset-0 bg-[url('/mesh.svg')] opacity-10 animate-pulse"></div>

            <div id="login-form" className="relative z-10 w-full max-w-md p-8 glass-card border border-green-500/30 rounded-lg shadow-[0_0_50px_rgba(0,255,0,0.1)]">
                <div className="text-center mb-8">
                    <h1 className="text-4xl font-bold mb-2 glitch-text" data-text="GHOSTWIRE">GHOSTWIRE</h1>
                    <p className="text-sm opacity-70">SECURE MESH COMMUNICATION NODE</p>
                </div>

                <div className="mb-8 text-center">
                    <div className="w-24 h-24 mx-auto mb-4 rounded-full border-2 border-green-500 flex items-center justify-center relative shadow-[0_0_15px_rgba(0,255,0,0.5)]">
                        <span className="text-4xl">👻</span>
                        {status.locked && (
                            <div className="absolute -bottom-2 -right-2 bg-black border border-green-500 rounded-full p-1">
                                🔒
                            </div>
                        )}
                    </div>
                    {status.username && (
                        <h2 className="text-xl text-white">{status.username}</h2>
                    )}
                    <p className="text-xs text-green-500/60 truncate max-w-[200px] mx-auto">{status.did || 'Starting up...'}</p>
                </div>

                <form onSubmit={handleLogin} className="space-y-6">
                    <div className="relative">
                        <input
                            type={showPassword ? "text" : "password"}
                            value={password}
                            onChange={(e) => setPassword(e.target.value)}
                            placeholder="ENTER PASSPHRASE"
                            className="w-full bg-black/50 border border-green-500/50 rounded px-4 py-3 text-center text-lg focus:outline-none focus:border-green-400 focus:shadow-[0_0_15px_rgba(0,255,0,0.3)] transition-all placeholder-green-800"
                            autoFocus
                        />
                        <button
                            type="button"
                            onClick={() => setShowPassword(!showPassword)}
                            className="absolute right-4 top-1/2 -translate-y-1/2 text-green-500/50 hover:text-green-500 transition-colors text-xl"
                            title={showPassword ? "Hide Passphrase" : "Show Passphrase"}
                        >
                            {showPassword ? '👁️‍🗨️' : '👁️'}
                        </button>
                    </div>

                    {error && (
                        <div className="text-red-500 text-center text-sm font-bold bg-red-900/20 py-2 rounded border border-red-500/30">
                            ⚠️ {error}
                        </div>
                    )}

                    <button
                        type="submit"
                        disabled={isLoading}
                        className="w-full bg-green-900/20 hover:bg-green-500/20 text-green-400 border border-green-500/50 hover:border-green-400 py-3 rounded font-bold transition-all duration-200 transform hover:scale-[1.02] disabled:opacity-50 disabled:cursor-not-allowed group relative overflow-hidden"
                    >
                        {isLoading ? (
                            <span className="animate-pulse">DECRYPTING IDENTITY...</span>
                        ) : (
                            <span className="relative z-10 flex items-center justify-center gap-2">
                                UNLOCK PROFILE <span className="group-hover:translate-x-1 transition-transform">→</span>
                            </span>
                        )}
                        <div className="absolute inset-0 bg-green-500/10 translate-y-full group-hover:translate-y-0 transition-transform duration-300"></div>
                    </button>
                </form>

                <div className="mt-6 text-center text-xs opacity-50">
                    <p>PHASE 1: PROFILE PROTECTION ACTIVE</p>
                    <p className="mt-2 text-[10px]">V2.0.0-ALPHA-SECURE</p>
                </div>
            </div>

            <style>{`
        .glitch-text {
          position: relative;
        }
        .glitch-text::before,
        .glitch-text::after {
          content: attr(data-text);
          position: absolute;
          top: 0;
          left: 0;
          width: 100%;
          height: 100%;
        }
        .glitch-text::before {
          left: 2px;
          text-shadow: -1px 0 red;
          clip: rect(24px, 550px, 90px, 0);
          animation: glitch-anim-2 3s infinite linear alternate-reverse;
        }
        .glitch-text::after {
          left: -2px;
          text-shadow: -1px 0 blue;
          clip: rect(85px, 550px, 140px, 0);
          animation: glitch-anim-1 2.5s infinite linear alternate-reverse;
        }
        @keyframes glitch-anim-1 {
          0% { clip: rect(20px, 9999px, 15px, 0); }
          20% { clip: rect(70px, 9999px, 95px, 0); }
          40% { clip: rect(20px, 9999px, 5px, 0); }
          60% { clip: rect(80px, 9999px, 5px, 0); }
          80% { clip: rect(5px, 9999px, 75px, 0); }
          100% { clip: rect(65px, 9999px, 30px, 0); }
        }
        @keyframes glitch-anim-2 {
          0% { clip: rect(65px, 9999px, 100px, 0); }
          20% { clip: rect(15px, 9999px, 85px, 0); }
          40% { clip: rect(50px, 9999px, 5px, 0); }
          60% { clip: rect(5px, 9999px, 55px, 0); }
          80% { clip: rect(95px, 9999px, 35px, 0); }
          100% { clip: rect(35px, 9999px, 80px, 0); }
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
