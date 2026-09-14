import React, { useState, useEffect } from 'react';
import { useApp } from '../../context/AppContext';
import { auth, profile } from '../../services/api';

const MIN_PASSPHRASE_BYTES = 12;
const passphraseBytes = (value: string) => new TextEncoder().encode(value).length;
const PROFILE_NAME_RE = /^[A-Za-z0-9_-]{1,48}$/;

type LoginMode = 'unlock' | 'setup' | 'select' | 'create';

export default function Login() {
  const { dispatch, toast, refreshIdentity } = useApp();
  const [mode, setMode] = useState<LoginMode>('unlock');
  const [password, setPassword] = useState('');
  const [confirmPw, setConfirmPw] = useState('');
  const [nickname, setNickname] = useState('');
  const [profileName, setProfileName] = useState('');
  const [profiles, setProfiles] = useState<string[]>([]);
  const [loading, setLoading] = useState(false);
  const [err, setErr] = useState('');
  const [daemonLocked, setDaemonLocked] = useState<boolean | null>(null);

  useEffect(() => {
    Promise.all([profile.list(), auth.status()]).then(([profilesResult, identity]) => {
      setProfiles(profilesResult.profiles);
      setDaemonLocked(identity.locked);
      // A fresh daemon creates an unsaved default identity. The status flag,
      // not the profile list, is authoritative for choosing setup mode.
      if (identity.setup_complete === false) setMode('setup');
    }).catch(() => {
      setErr('Daemon unavailable or unable to read profile state');
    });
  }, []);

  const switchMode = (next: LoginMode) => {
    setMode(next);
    setErr('');
    setPassword('');
    setConfirmPw('');
  };

  const finishUnlock = async (passphrase: string) => {
    await auth.unlock(passphrase);
    await refreshIdentity();
    dispatch({ type: 'SET_LOCKED', payload: false });
    toast('success', 'Identity unlocked');
  };

  const handleUnlock = async (e: React.FormEvent) => {
    e.preventDefault();
    if (passphraseBytes(password) < MIN_PASSPHRASE_BYTES) {
      setErr(`Passphrase must be at least ${MIN_PASSPHRASE_BYTES} bytes`);
      return;
    }
    setLoading(true); setErr('');
    try {
      await finishUnlock(password);
    } catch {
      setErr('Incorrect passphrase or profile could not be unlocked');
      sessionStorage.removeItem('gw_token');
    } finally { setLoading(false); setPassword(''); }
  };

  const handleSelect = async (name: string) => {
    setLoading(true); setErr('');
    try {
      // Selection reconstructs Core and revokes old sessions. It must be a
      // backend transition, never a sessionStorage-only cosmetic change.
      // The locked-screen endpoint requires a locked daemon: lock first when
      // the cockpit is still unlocked, otherwise selection dead-ends.
      if (daemonLocked === false) {
        sessionStorage.removeItem('gw_token');
        await auth.lock();
        setDaemonLocked(true);
        dispatch({ type: 'SET_LOCKED', payload: true });
      }
      sessionStorage.removeItem('gw_token');
      await profile.selectLocked(name);
      setMode('unlock');
      toast('info', `${name} selected; enter its passphrase`);
    } catch (e: any) {
      setErr(e?.message ?? 'Profile selection failed');
    } finally { setLoading(false); }
  };

  const handleLockForSelect = async () => {
    setLoading(true); setErr('');
    try {
      sessionStorage.removeItem('gw_token');
      await auth.lock();
      setDaemonLocked(true);
      dispatch({ type: 'SET_LOCKED', payload: true });
      toast('info', 'Active profile locked; choose a profile');
    } catch (e: any) {
      setErr(e?.message ?? 'Lock failed');
    } finally { setLoading(false); }
  };

  const handleCreate = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!PROFILE_NAME_RE.test(profileName)) {
      setErr('Profile name must use only letters, numbers, _ or - (1–48 characters)');
      return;
    }
    if (!nickname.trim()) {
      setErr('Callsign / nickname is required');
      return;
    }
    if (passphraseBytes(password) < MIN_PASSPHRASE_BYTES) {
      setErr(`Passphrase must be at least ${MIN_PASSPHRASE_BYTES} bytes`);
      return;
    }
    if (password !== confirmPw) {
      setErr('Passphrases do not match');
      return;
    }

    setLoading(true); setErr('');
    try {
      await profile.createFirst({
        name: profileName,
        nickname: nickname.trim(),
        password,
      });
      sessionStorage.removeItem('gw_token');
      await profile.selectLocked(profileName);
      await finishUnlock(password);
      toast('success', `Profile ${profileName} established`);
    } catch (e: any) {
      setErr(e?.message ?? 'Profile creation failed');
      sessionStorage.removeItem('gw_token');
    } finally {
      setLoading(false);
      setPassword('');
      setConfirmPw('');
    }
  };

  const handleSetup = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!nickname.trim()) {
      setErr('Callsign / nickname is required');
      return;
    }
    if (passphraseBytes(password) < MIN_PASSPHRASE_BYTES) {
      setErr(`Passphrase must be at least ${MIN_PASSPHRASE_BYTES} bytes`);
      return;
    }
    if (password !== confirmPw) {
      setErr('Passphrases do not match');
      return;
    }
    setLoading(true); setErr('');
    try {
      await auth.setup({ nickname: nickname.trim(), password });
      await refreshIdentity();
      dispatch({ type: 'SET_LOCKED', payload: false });
      toast('success', 'Identity established');
    } catch (e: any) {
      setErr(e?.message ?? 'Setup failed; create a named profile if this identity is locked');
    } finally { setLoading(false); setPassword(''); setConfirmPw(''); }
  };

  const renderError = err && (
    <div style={{
      fontFamily: 'var(--mono)', fontSize: 10, color: 'var(--crit)',
      marginBottom: 12, padding: '6px 10px',
      background: 'rgba(255,59,92,0.06)',
      border: '1px solid var(--crit-dim)',
      borderRadius: 'var(--r-sm)',
    }}>
      {err}
    </div>
  );

  const renderPassphraseFields = (labels = true) => (
    <>
      <div style={{ marginBottom: 12 }}>
        {labels && <div className="ops-label" style={{ marginBottom: 6 }}>Passphrase</div>}
        <input
          className="ops-input"
          type="password"
          placeholder="Strong passphrase (12+ bytes)..."
          minLength={MIN_PASSPHRASE_BYTES}
          value={password}
          onChange={e => setPassword(e.target.value)}
          autoComplete="new-password"
        />
      </div>
      <div style={{ marginBottom: 16 }}>
        {labels && <div className="ops-label" style={{ marginBottom: 6 }}>Confirm passphrase</div>}
        <input
          className="ops-input"
          type="password"
          placeholder="Repeat passphrase..."
          value={confirmPw}
          onChange={e => setConfirmPw(e.target.value)}
          autoComplete="new-password"
        />
      </div>
    </>
  );

  return (
    <div style={{
      width: '100vw', height: '100vh',
      background: 'var(--void)',
      display: 'flex', alignItems: 'center', justifyContent: 'center',
      fontFamily: 'var(--mono)',
    }}>
      <div className="scanlines" style={{ position: 'fixed', inset: 0, pointerEvents: 'none' }} />
      <div style={{ position: 'fixed', top: 20, left: 20, fontFamily: 'var(--mono)', fontSize: 10, color: 'var(--sig-dim)', letterSpacing: '0.2em' }}>
        GHOSTWIRE · NODE ACCESS
      </div>
      <div style={{ position: 'fixed', bottom: 20, right: 20, fontFamily: 'var(--mono)', fontSize: 9, color: 'var(--t4)', letterSpacing: '0.15em' }}>
        NET·FP 848CF75A
      </div>

      <div style={{ width: 410, maxWidth: 'calc(100vw - 28px)', padding: 32, background: 'var(--panel)', border: '1px solid var(--border-hi)', borderRadius: 'var(--r-md)', position: 'relative' }}>
        <div style={{ position: 'absolute', top: -1, left: -1, width: 16, height: 16, borderTop: '1px solid var(--sig)', borderLeft: '1px solid var(--sig)' }} />
        <div style={{ position: 'absolute', bottom: -1, right: -1, width: 16, height: 16, borderBottom: '1px solid var(--sig)', borderRight: '1px solid var(--sig)' }} />

        <div style={{ marginBottom: 22, textAlign: 'center' }}>
          <div style={{ fontSize: 20, fontWeight: 700, letterSpacing: '0.25em', color: 'var(--sig)' }}>GHOSTWIRE</div>
          <div style={{ fontSize: 9, letterSpacing: '0.2em', color: 'var(--t3)', marginTop: 3 }}>
            {mode === 'create' ? 'CREATE PROFILE' : mode === 'setup' ? 'ESTABLISH IDENTITY' : mode === 'select' ? 'SELECT PROFILE' : 'IDENTITY VERIFICATION'}
          </div>
        </div>

        <div style={{ display: 'flex', gap: 4, marginBottom: 20, borderBottom: '1px solid var(--border)', paddingBottom: 12, flexWrap: 'wrap' }}>
          {(['unlock', 'select', 'create'] as const).map(m => (
            <button key={m} onClick={() => switchMode(m)} style={{
              flex: '1 1 30%', padding: '6px 4px', background: mode === m ? 'var(--sig-ghost)' : 'none',
              border: `1px solid ${mode === m ? 'var(--sig-dim)' : 'var(--border)'}`,
              borderRadius: 'var(--r-sm)', color: mode === m ? 'var(--sig)' : 'var(--t3)',
              fontFamily: 'var(--mono)', fontSize: 9, letterSpacing: '0.06em', textTransform: 'uppercase', cursor: 'pointer',
            }}>
              {m === 'unlock' ? 'Unlock' : m === 'select' ? 'Profiles' : 'Create profile'}
            </button>
          ))}
        </div>

        {mode === 'unlock' && (
          <form onSubmit={handleUnlock}>
            <div style={{ marginBottom: 12 }}>
              <div className="ops-label" style={{ marginBottom: 6 }}>Passphrase</div>
              <input className="ops-input" type="password" placeholder="Enter passphrase (12+ bytes)..." minLength={MIN_PASSPHRASE_BYTES} value={password} onChange={e => setPassword(e.target.value)} autoFocus autoComplete="current-password" />
            </div>
            {renderError}
            <button className="btn-ops btn-ops-primary" type="submit" disabled={loading || passphraseBytes(password) < MIN_PASSPHRASE_BYTES} style={{ width: '100%', justifyContent: 'center', padding: '8px' }}>
              {loading ? 'Verifying...' : '⬢ Unlock identity'}
            </button>
            <div style={{ marginTop: 12, fontFamily: 'var(--mono)', fontSize: 9, color: 'var(--t4)', lineHeight: 1.55 }}>
              The selected profile is local to this node. A profile name is not a PeerID; the node identity key cryptographically binds the profile to its network identity.
            </div>
          </form>
        )}

        {mode === 'select' && (
          <div>
            <div className="ops-label" style={{ marginBottom: 8 }}>Select local profile</div>
            {daemonLocked === false && (
              <button className="btn-ops btn-ops-primary" disabled={loading} onClick={() => void handleLockForSelect()} style={{ width: '100%', justifyContent: 'center', marginBottom: 8 }}>
                {loading ? 'Locking...' : '⬢ Lock active profile first'}
              </button>
            )}
            <div style={{ maxHeight: 260, overflowY: 'auto' }}>
              {profiles.length === 0 ? (
                <div style={{ padding: 12, color: 'var(--t4)', fontSize: 10 }}>No persisted profiles found.</div>
              ) : profiles.map(p => (
                <button key={p} className="btn-ops" disabled={loading} onClick={() => void handleSelect(p)} style={{ width: '100%', justifyContent: 'flex-start', marginBottom: 4 }}>
                  ◈ {p}
                </button>
              ))}
            </div>
            {renderError}
            <div style={{ marginTop: 10, padding: '8px 10px', border: '1px solid var(--border)', borderRadius: 'var(--r-sm)', color: 'var(--t4)', fontFamily: 'var(--mono)', fontSize: 9, lineHeight: 1.5 }}>
              Selecting a profile reconstructs the backend core and revokes the previous session. Unlock the selected profile before using the cockpit.
            </div>
          </div>
        )}

        {mode === 'create' && (
          <form onSubmit={handleCreate}>
            <div style={{ marginBottom: 12 }}>
              <div className="ops-label" style={{ marginBottom: 6 }}>Profile name</div>
              <input className="ops-input" placeholder="e.g. field-operator" value={profileName} onChange={e => setProfileName(e.target.value)} autoFocus autoComplete="off" />
            </div>
            <div style={{ marginBottom: 12 }}>
              <div className="ops-label" style={{ marginBottom: 6 }}>Callsign / nickname</div>
              <input className="ops-input" placeholder="Choose a callsign..." value={nickname} onChange={e => setNickname(e.target.value)} autoComplete="nickname" />
            </div>
            {renderPassphraseFields()}
            {renderError}
            <div style={{ fontFamily: 'var(--mono)', fontSize: 9, color: 'var(--t4)', marginBottom: 14, lineHeight: 1.6 }}>
              This node generates its own identity locally. The passphrase protects the new profile and cannot be recovered by the network.
            </div>
            <button className="btn-ops btn-ops-primary" type="submit" disabled={loading} style={{ width: '100%', justifyContent: 'center', padding: '8px' }}>
              {loading ? 'Creating and unlocking...' : '◈ Create profile'}
            </button>
          </form>
        )}

        {mode === 'setup' && (
          <form onSubmit={handleSetup}>
            <div style={{ marginBottom: 12 }}>
              <div className="ops-label" style={{ marginBottom: 6 }}>Callsign / nickname</div>
              <input className="ops-input" placeholder="Choose a callsign..." value={nickname} onChange={e => setNickname(e.target.value)} autoFocus autoComplete="nickname" />
            </div>
            {renderPassphraseFields()}
            {renderError}
            <div style={{ fontFamily: 'var(--mono)', fontSize: 9, color: 'var(--t4)', marginBottom: 14, lineHeight: 1.6 }}>
              Use at least 12 bytes. If the active default identity is already locked, create a named profile instead of attempting legacy setup.
            </div>
            <button className="btn-ops btn-ops-primary" type="submit" disabled={loading} style={{ width: '100%', justifyContent: 'center', padding: '8px' }}>
              {loading ? 'Generating identity...' : '◈ Establish identity'}
            </button>
          </form>
        )}
      </div>
    </div>
  );
}
