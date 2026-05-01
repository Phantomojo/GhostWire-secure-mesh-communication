import React, { useState, useEffect } from 'react';
import { useApp } from '../../context/AppContext';
import { auth, profile } from '../../services/api';

export default function Login() {
  const { dispatch, toast, refreshIdentity } = useApp();
  const [mode, setMode] = useState<'unlock' | 'setup' | 'select'>('unlock');
  const [password, setPassword] = useState('');
  const [confirmPw, setConfirmPw] = useState('');
  const [nickname, setNickname] = useState('');
  const [profiles, setProfiles] = useState<string[]>([]);
  const [loading, setLoading] = useState(false);
  const [err, setErr] = useState('');

  useEffect(() => {
    profile.list().then(d => {
      setProfiles(d.profiles);
      if (d.profiles.length === 0) setMode('setup');
    }).catch(() => {});
  }, []);

  const handleUnlock = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!password) return;
    setLoading(true); setErr('');
    try {
      await auth.unlock(password);
      await refreshIdentity();
      dispatch({ type: 'SET_LOCKED', payload: false });
      toast('success', 'Identity unlocked');
    } catch {
      setErr('Incorrect passphrase');
    } finally { setLoading(false); setPassword(''); }
  };

  const handleSetup = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!nickname || !password || password !== confirmPw) {
      setErr(password !== confirmPw ? 'Passphrases do not match' : 'All fields required');
      return;
    }
    setLoading(true); setErr('');
    try {
      await auth.setup({ nickname, password });
      await refreshIdentity();
      dispatch({ type: 'SET_LOCKED', payload: false });
      toast('success', 'Identity established');
    } catch (e: any) {
      setErr(e.message ?? 'Setup failed');
    } finally { setLoading(false); }
  };

  return (
    <div style={{
      width: '100vw', height: '100vh',
      background: 'var(--void)',
      display: 'flex', alignItems: 'center', justifyContent: 'center',
      fontFamily: 'var(--mono)',
    }}>
      {/* Scan line overlay */}
      <div className="scanlines" style={{ position: 'fixed', inset: 0, pointerEvents: 'none' }} />

      {/* Corner accents */}
      <div style={{
        position: 'fixed', top: 20, left: 20,
        fontFamily: 'var(--mono)', fontSize: 10, color: 'var(--sig-dim)',
        letterSpacing: '0.2em',
      }}>
        GHOSTWIRE · NODE ACCESS
      </div>
      <div style={{
        position: 'fixed', bottom: 20, right: 20,
        fontFamily: 'var(--mono)', fontSize: 9, color: 'var(--t4)',
        letterSpacing: '0.15em',
      }}>
        NET·FP 848CF75A
      </div>

      <div style={{
        width: 380,
        padding: 32,
        background: 'var(--panel)',
        border: '1px solid var(--border-hi)',
        borderRadius: 'var(--r-md)',
        position: 'relative',
      }}>
        {/* Corner brackets */}
        <div style={{
          position: 'absolute', top: -1, left: -1,
          width: 16, height: 16,
          borderTop: '1px solid var(--sig)',
          borderLeft: '1px solid var(--sig)',
        }} />
        <div style={{
          position: 'absolute', bottom: -1, right: -1,
          width: 16, height: 16,
          borderBottom: '1px solid var(--sig)',
          borderRight: '1px solid var(--sig)',
        }} />

        {/* Wordmark */}
        <div style={{ marginBottom: 28, textAlign: 'center' }}>
          <div style={{
            fontSize: 20, fontWeight: 700, letterSpacing: '0.25em',
            color: 'var(--sig)',
          }}>
            GHOSTWIRE
          </div>
          <div style={{ fontSize: 9, letterSpacing: '0.2em', color: 'var(--t3)', marginTop: 3 }}>
            {mode === 'setup' ? 'ESTABLISH IDENTITY' : 'IDENTITY VERIFICATION'}
          </div>
        </div>

        {/* Mode tabs (only if profiles exist) */}
        {profiles.length > 0 && mode !== 'setup' && (
          <div style={{
            display: 'flex', gap: 4, marginBottom: 20,
            borderBottom: '1px solid var(--border)', paddingBottom: 12,
          }}>
            {(['unlock', 'select'] as const).map(m => (
              <button key={m}
                onClick={() => { setMode(m); setErr(''); }}
                style={{
                  flex: 1, padding: '5px 0',
                  background: mode === m ? 'var(--sig-ghost)' : 'none',
                  border: `1px solid ${mode === m ? 'var(--sig-dim)' : 'var(--border)'}`,
                  borderRadius: 'var(--r-sm)',
                  color: mode === m ? 'var(--sig)' : 'var(--t3)',
                  fontFamily: 'var(--mono)',
                  fontSize: 10, letterSpacing: '0.1em', textTransform: 'uppercase',
                  cursor: 'pointer',
                }}>
                {m === 'unlock' ? 'Unlock' : 'Profiles'}
              </button>
            ))}
          </div>
        )}

        {mode === 'unlock' && (
          <form onSubmit={handleUnlock}>
            <div style={{ marginBottom: 12 }}>
              <div className="ops-label" style={{ marginBottom: 6 }}>Passphrase</div>
              <input
                className="ops-input"
                type="password"
                placeholder="Enter passphrase..."
                value={password}
                onChange={e => setPassword(e.target.value)}
                autoFocus
                autoComplete="current-password"
              />
            </div>
            {err && (
              <div style={{
                fontFamily: 'var(--mono)', fontSize: 10, color: 'var(--crit)',
                marginBottom: 12, padding: '6px 10px',
                background: 'rgba(255,59,92,0.06)',
                border: '1px solid var(--crit-dim)',
                borderRadius: 'var(--r-sm)',
              }}>
                {err}
              </div>
            )}
            <button
              className="btn-ops btn-ops-primary"
              type="submit"
              disabled={loading || !password}
              style={{ width: '100%', justifyContent: 'center', padding: '8px' }}
            >
              {loading ? 'Verifying...' : '⬢ Unlock identity'}
            </button>
          </form>
        )}

        {mode === 'select' && (
          <div>
            <div className="ops-label" style={{ marginBottom: 8 }}>Select profile</div>
            {profiles.map(p => (
              <button key={p}
                className="btn-ops"
                onClick={() => {
                  profile.select(p).then(() => setMode('unlock'));
                }}
                style={{ width: '100%', justifyContent: 'flex-start', marginBottom: 4 }}
              >
                ◈ {p}
              </button>
            ))}
            <button
              className="btn-ops"
              style={{ width: '100%', justifyContent: 'center', marginTop: 8 }}
              onClick={() => setMode('setup')}
            >
              + New identity
            </button>
          </div>
        )}

        {mode === 'setup' && (
          <form onSubmit={handleSetup}>
            <div style={{ marginBottom: 12 }}>
              <div className="ops-label" style={{ marginBottom: 6 }}>Callsign / nickname</div>
              <input
                className="ops-input"
                placeholder="Choose a callsign..."
                value={nickname}
                onChange={e => setNickname(e.target.value)}
                autoFocus
                autoComplete="username"
              />
            </div>
            <div style={{ marginBottom: 12 }}>
              <div className="ops-label" style={{ marginBottom: 6 }}>Passphrase</div>
              <input
                className="ops-input"
                type="password"
                placeholder="Strong passphrase..."
                value={password}
                onChange={e => setPassword(e.target.value)}
                autoComplete="new-password"
              />
            </div>
            <div style={{ marginBottom: 16 }}>
              <div className="ops-label" style={{ marginBottom: 6 }}>Confirm passphrase</div>
              <input
                className="ops-input"
                type="password"
                placeholder="Repeat passphrase..."
                value={confirmPw}
                onChange={e => setConfirmPw(e.target.value)}
                autoComplete="new-password"
              />
            </div>
            {err && (
              <div style={{
                fontFamily: 'var(--mono)', fontSize: 10, color: 'var(--crit)',
                marginBottom: 12, padding: '6px 10px',
                background: 'rgba(255,59,92,0.06)',
                border: '1px solid var(--crit-dim)',
                borderRadius: 'var(--r-sm)',
              }}>
                {err}
              </div>
            )}
            <div style={{
              fontFamily: 'var(--mono)', fontSize: 9, color: 'var(--t4)',
              marginBottom: 14, lineHeight: 1.6,
            }}>
              Your identity key is derived from your passphrase using Argon2id.
              There is no reset. Store it safely.
            </div>
            <button
              className="btn-ops btn-ops-primary"
              type="submit"
              disabled={loading}
              style={{ width: '100%', justifyContent: 'center', padding: '8px' }}
            >
              {loading ? 'Generating identity...' : '◈ Establish identity'}
            </button>
            {profiles.length > 0 && (
              <button type="button" className="btn-ops"
                onClick={() => setMode('unlock')}
                style={{ width: '100%', justifyContent: 'center', marginTop: 6 }}>
                ← Back to unlock
              </button>
            )}
          </form>
        )}
      </div>
    </div>
  );
}
