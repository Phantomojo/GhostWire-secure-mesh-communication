import React, { useEffect, useState } from 'react';

const MIN_PASSPHRASE_BYTES = 12;
const passphraseBytes = (value: string) => new TextEncoder().encode(value).length;
import { useApp } from '../../context/AppContext';
import { profile as profileApi, auth, API_BASE } from '../../services/api';

function Section({ title, children }: { title:string; children:React.ReactNode }) {
  return (
    <div className="ops-panel" style={{ padding:16,marginBottom:12 }}>
      <div className="ops-section-header"><span className="ops-section-title">{title}</span></div>
      {children}
    </div>
  );
}

function Field({ label, children }: { label:string; children:React.ReactNode }) {
  return (
    <div style={{ marginBottom:12 }}>
      <div className="ops-label" style={{ marginBottom:5 }}>{label}</div>
      {children}
    </div>
  );
}

export default function SettingsPanel() {
  const { state, dispatch, toast, refreshIdentity } = useApp();
  const [nickname, setNickname] = useState(state.profile?.nickname ?? '');
  const [bio, setBio] = useState(state.profile?.bio ?? '');
  const [saving, setSaving] = useState(false);
  const [wipeVal, setWipeVal] = useState('');
  const [showWipe, setShowWipe] = useState(false);
  const [locking, setLocking] = useState(false);
  const [profileList, setProfileList] = useState<string[]>([]);
  const [currentProfile, setCurrentProfile] = useState<string | null>(null);
  const [newProfileName, setNewProfileName] = useState('');
  const [newProfileNickname, setNewProfileNickname] = useState('');
  const [newProfilePassword, setNewProfilePassword] = useState('');
  const [profileBusy, setProfileBusy] = useState(false);

  const refreshProfiles = async () => {
    const result = await profileApi.list();
    setProfileList(result.profiles);
    setCurrentProfile(result.current);
  };
  useEffect(() => { refreshProfiles().catch(() => {}); }, []);
  const handleCreateProfile = async () => {
    if (passphraseBytes(newProfilePassword) < MIN_PASSPHRASE_BYTES) {
      toast('warn', `Passphrase must be at least ${MIN_PASSPHRASE_BYTES} bytes`);
      return;
    }
    if (!/^[A-Za-z0-9_-]{1,48}$/.test(newProfileName)) {
      toast('error', 'Profile name must use only letters, numbers, _ or -');
      return;
    }
    if (!newProfileNickname.trim()) {
      toast('error', 'Nickname is required and passphrase must be at least 12 bytes');
      return;
    }
    setProfileBusy(true);
    try {
      await profileApi.create({
        name: newProfileName,
        nickname: newProfileNickname.trim(),
        password: newProfilePassword,
      });
      await refreshProfiles();
      setNewProfileName(''); setNewProfileNickname(''); setNewProfilePassword('');
      toast('success', `Identity ${newProfileName} created`);
    } catch (e:any) { toast('error', e.message ?? 'Profile creation failed'); }
    finally { setProfileBusy(false); }
  };
  const handleSelectProfile = async (name: string) => {
    if (name === currentProfile) return;
    setProfileBusy(true);
    try {
      await profileApi.select(name);
      sessionStorage.removeItem('gw_token');
      dispatch({ type:'SET_LOCKED', payload:true });
      toast('warn', `Switched to ${name}; unlock it to continue`);
      window.location.reload();
    } catch (e:any) { toast('error', e.message ?? 'Profile switch failed'); }
    finally { setProfileBusy(false); }
  };
  const handleSaveProfile = async () => {
    setSaving(true);
    try {
      await profileApi.update({ nickname, bio });
      await refreshIdentity();
      toast('success', 'Profile updated');
    } catch (e:any) { toast('error', e.message ?? 'Save failed'); }
    finally { setSaving(false); }
  };

  const handleLock = async () => {
    setLocking(true);
    try {
      await auth.lock();
      dispatch({ type:'SET_LOCKED', payload:true });
      toast('warn', 'Identity locked');
    } catch (e:any) { toast('error', e.message ?? 'Lock failed'); }
    finally { setLocking(false); }
  };

  const handleWipe = async () => {
    if (wipeVal !== 'WIPE') return;
    try {
      await fetch(`${API_BASE}/system/wipe`, { method:'POST', credentials:'include', headers:{'Content-Type':'application/json'} });
      toast('warn', 'System wiped — reload the page');
      setShowWipe(false);
    } catch (e:any) { toast('error', e.message ?? 'Wipe failed'); }
  };

  const sessionToken = sessionStorage.getItem('gw_token');

  return (
    <div className="panel-scroll animate-in">
      {/* Connection */}
      <Section title="Connection">
        <Field label="Backend URL">
          <div style={{ fontFamily:'var(--mono)',fontSize:11,color:'var(--sig)',padding:'6px 0' }}>{API_BASE}</div>
          <div style={{ fontFamily:'var(--mono)',fontSize:9,color:'var(--t4)' }}>
            Status: {state.backendConnected ? '● Connected' : '○ Disconnected'}
          </div>
        </Field>
        <Field label="Session token">
          <div style={{ display:'flex',gap:8,alignItems:'center' }}>
            <div style={{ fontFamily:'var(--mono)',fontSize:11,color:'var(--t3)',flex:1 }}>
              {sessionToken ? `${sessionToken.slice(0,8)}...${sessionToken.slice(-4)}` : 'Not set — paste token from terminal below'}
            </div>
            {sessionToken && (
              <button className="btn-ops btn-ops-danger"
                onClick={() => { sessionStorage.removeItem('gw_token'); dispatch({ type:'SET_LOCKED',payload:true }); toast('warn','Session cleared'); }}
                style={{ padding:'3px 8px',fontSize:10 }}>
                Clear
              </button>
            )}
          </div>
          {!sessionToken && (
            <div style={{ display:'flex',gap:6,marginTop:6 }}>
              <input className="ops-input" placeholder="Paste X-Ghost-Token from terminal..."
                id="token-input" style={{ flex:1,fontSize:11 }} />
              <button className="btn-ops btn-ops-primary"
                onClick={() => {
                  const el = document.getElementById('token-input') as HTMLInputElement;
                  if (el.value.trim()) { sessionStorage.setItem('gw_token',el.value.trim()); window.location.reload(); }
                }}
                style={{ padding:'4px 10px',fontSize:11 }}>
                Save
              </button>
            </div>
          )}
        </Field>
      </Section>

      {/* Local identities */}
      <Section title="Local identities">
        <Field label="Active profile">
          <div style={{ fontFamily:'var(--mono)',fontSize:11,color:'var(--sig)' }}>{currentProfile ?? '—'}</div>
        </Field>
        <Field label="Switch identity">
          <div style={{ display:'flex',gap:6,flexWrap:'wrap' }}>
            {profileList.map(name => (
              <button key={name} className="btn-ops"
                disabled={profileBusy || name === currentProfile}
                onClick={() => handleSelectProfile(name)}>
                {name === currentProfile ? '● ' : '◈ '}{name}
              </button>
            ))}
          </div>
          <div style={{ marginTop:6,fontFamily:'var(--mono)',fontSize:9,color:'var(--t4)' }}>
            Switching changes the cryptographic persona and requires the selected profile passphrase.
          </div>
        </Field>
        <Field label="Create identity">
          <input className="ops-input" value={newProfileName} onChange={e=>setNewProfileName(e.target.value)} placeholder="Profile name, e.g. field-operator" style={{ width:'100%',marginBottom:6 }} />
          <input className="ops-input" value={newProfileNickname} onChange={e=>setNewProfileNickname(e.target.value)} placeholder="Callsign / nickname" style={{ width:'100%',marginBottom:6 }} />
          <div style={{ display:'flex',gap:6 }}>
            <input className="ops-input" type="password" value={newProfilePassword} onChange={e=>setNewProfilePassword(e.target.value)} placeholder="Passphrase (12+ bytes)" minLength={MIN_PASSPHRASE_BYTES} style={{ flex:1 }} autoComplete="new-password" />
            <button className="btn-ops btn-ops-primary" onClick={handleCreateProfile} disabled={profileBusy || passphraseBytes(newProfilePassword) < MIN_PASSPHRASE_BYTES}>Create</button>
          </div>
        </Field>
      </Section>
      {/* Identity */}
      <Section title="Identity">
        <Field label="DID">
          <div style={{ fontFamily:'var(--mono)',fontSize:10,color:'var(--t3)',wordBreak:'break-all' }}>
            {state.profile?.did ?? '—'}
          </div>
        </Field>
        <Field label="Callsign / Nickname">
          <div style={{ display:'flex',gap:6 }}>
            <input className="ops-input" value={nickname} onChange={e => setNickname(e.target.value)} placeholder="Your callsign" style={{ flex:1 }} />
            <button className="btn-ops btn-ops-primary" onClick={handleSaveProfile} disabled={saving} style={{ padding:'6px 12px' }}>
              {saving?'Saving...':'Save'}
            </button>
          </div>
        </Field>
        <Field label="Bio">
          <div style={{ display:'flex',gap:6 }}>
            <input className="ops-input" value={bio} onChange={e => setBio(e.target.value)} placeholder="Short bio" style={{ flex:1 }} />
            <button className="btn-ops btn-ops-primary" onClick={handleSaveProfile} disabled={saving} style={{ padding:'6px 12px' }}>
              {saving?'...':'Save'}
            </button>
          </div>
        </Field>
      </Section>

      {/* Security protocols */}
      <Section title="Security protocols">
        <div style={{ fontFamily:'var(--mono)',fontSize:11,color:'var(--t3)',marginBottom:12,lineHeight:1.6 }}>
          Advanced security features. Configure before arming — they cannot be easily undone during an incident.
        </div>
        {[
          { label:'Duress PIN', desc:'Set a decoy passphrase that unlocks a clean decoy profile under coercion.', tag:'Wave 8' },
          { label:'Dead man\'s switch', desc:'Automatically wipe and alert if you miss a check-in within N hours.', tag:'Wave 8' },
          { label:'Panic wipe', desc:'Immediately wipe all keys and send alerts. Use the sidebar toggle.', tag:'Active' },
        ].map(item => (
          <div key={item.label} className="ops-row" style={{ flexWrap:'wrap',gap:8 }}>
            <div style={{ flex:1,minWidth:180 }}>
              <div style={{ fontFamily:'var(--mono)',fontSize:11,color:'var(--t1)',fontWeight:600 }}>{item.label}</div>
              <div style={{ fontFamily:'var(--mono)',fontSize:10,color:'var(--t4)',marginTop:2 }}>{item.desc}</div>
            </div>
            <div className={`ops-tag ${item.tag==='Active'?'ops-tag-sig':'ops-tag-muted'}`} style={{ flexShrink:0 }}>{item.tag}</div>
          </div>
        ))}
      </Section>

      {/* Danger zone */}
      <Section title="Danger zone">
        <div style={{ display:'flex',gap:8,flexWrap:'wrap' }}>
          <button className="btn-ops btn-ops-warn" onClick={handleLock} disabled={locking}>
            {locking ? 'Locking...' : '⬢ Lock identity'}
          </button>
          <button className="btn-ops btn-ops-danger" onClick={() => setShowWipe(!showWipe)}>
            ✕ Wipe all data
          </button>
        </div>
        {showWipe && (
          <div className="inline-confirm" style={{ marginTop:10 }}>
            <div style={{ fontFamily:'var(--mono)',fontSize:11,color:'var(--crit)',marginBottom:8 }}>
              This will permanently destroy all keys, messages, and identity. There is no recovery.
            </div>
            <div style={{ fontFamily:'var(--mono)',fontSize:10,color:'var(--t3)',marginBottom:6 }}>Type WIPE to confirm:</div>
            <div style={{ display:'flex',gap:6 }}>
              <input className="ops-input" value={wipeVal} onChange={e=>setWipeVal(e.target.value)}
                placeholder="WIPE" style={{ flex:1,fontSize:11 }} autoFocus />
              <button className="btn-ops btn-ops-danger" onClick={handleWipe} disabled={wipeVal!=='WIPE'} style={{ padding:'4px 10px' }}>Execute</button>
              <button className="btn-ops" onClick={() => { setShowWipe(false); setWipeVal(''); }} style={{ padding:'4px 10px' }}>Cancel</button>
            </div>
          </div>
        )}
      </Section>
    </div>
  );
}
