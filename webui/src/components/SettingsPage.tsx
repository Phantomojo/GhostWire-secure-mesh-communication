import { useState, useEffect, useCallback } from 'react';
import { api, profile, network, pairing, security, recovery, settings as settingsApi, auth, channels } from '../services/api_fixed';
import type { SettingsData, GhostChannel } from '../services/api_fixed';

interface Props {
  backendConnected: boolean;
  onBack: () => void;
}

// ── Default settings (mirrors SettingsData) ────────────────────────────────────

const DEFAULTS: SettingsData = {
  nickname: '',
  bio: '',
  avatar_url: null,
  status_message: null,
  listen_port: 4001,
  api_port: 8080,
  max_peers: 50,
  mdns_enabled: true,
  dht_enabled: true,
  transport_mode: 'stealth',
  mixing_preset: 'balanced',
  cover_traffic_pct: 20,
  pqc_enabled: false,
  auto_lock_minutes: 15,
  key_rotation_days: 30,
  failed_attempts_lockout: 5,
  presence: 'off',
  read_receipts: false,
  message_preview: false,
  strip_metadata: true,
  traffic_morphing: false,
  stealth_mode: false,
  anonymous_sender: false,
  sphinx_dms: false,
  sound_enabled: true,
  desktop_notifications: true,
  notify_connection_requests: true,
  notify_ark_sync: false,
  default_collection: 'default',
  ark_auto_sync: false,
  ark_max_mb: 500,
  debug_mode: false,
  log_level: 'info',
  msgpack_wire: true,
  auto_update: false,
};

const SECTIONS = [
  { id: 'identity',      label: 'Identity',       icon: '⬡' },
  { id: 'network',       label: 'Network',         icon: '◈' },
  { id: 'security',      label: 'Security',        icon: '◉' },
  { id: 'privacy',       label: 'Privacy',         icon: '◌' },
  { id: 'channels',      label: 'Ghost Channels',  icon: '◎' },
  { id: 'ark',           label: 'Knowledge Ark',   icon: '◫' },
  { id: 'notifications', label: 'Notifications',   icon: '◻' },
  { id: 'advanced',      label: 'Advanced',        icon: '◆' },
  { id: 'danger',        label: 'Danger Zone',     icon: '▲' },
];

// ── Sub-components ─────────────────────────────────────────────────────────────

function Toggle({ value, onChange, disabled = false }: { value: boolean; onChange: (v: boolean) => void; disabled?: boolean }) {
  return (
    <button
      type="button"
      role="switch"
      aria-checked={value}
      disabled={disabled}
      onClick={() => onChange(!value)}
      className={`relative inline-flex w-12 h-6 rounded-full transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-slate-900 ${value ? 'bg-indigo-500' : 'bg-slate-700'} ${disabled ? 'opacity-40 cursor-not-allowed' : 'cursor-pointer'}`}
    >
      <span className={`absolute top-0.5 left-0.5 w-5 h-5 bg-white rounded-full shadow transition-transform duration-200 ${value ? 'translate-x-6' : 'translate-x-0'}`} />
    </button>
  );
}

function Row({ label, hint, children }: { label: string; hint?: string; children: React.ReactNode }) {
  return (
    <div className="flex items-center justify-between py-3 border-b border-white/5 last:border-0">
      <div className="mr-4">
        <div className="text-sm font-medium text-white">{label}</div>
        {hint && <div className="text-xs text-slate-500 mt-0.5">{hint}</div>}
      </div>
      <div className="flex-shrink-0">{children}</div>
    </div>
  );
}

function Field({ label, hint, children }: { label: string; hint?: string; children: React.ReactNode }) {
  return (
    <div className="space-y-1.5">
      <label className="block text-xs font-semibold text-slate-400 uppercase tracking-wider">{label}</label>
      {children}
      {hint && <p className="text-xs text-slate-600">{hint}</p>}
    </div>
  );
}

function Input({ value, onChange, type = 'text', placeholder = '', className = '' }: {
  value: string | number; onChange: (v: string) => void; type?: string; placeholder?: string; className?: string;
}) {
  return (
    <input
      type={type}
      value={value}
      onChange={e => onChange(e.target.value)}
      placeholder={placeholder}
      className={`w-full bg-slate-800 border border-white/10 rounded-xl px-4 py-2.5 text-sm text-white placeholder-slate-600 focus:border-indigo-500 focus:outline-none transition-colors ${className}`}
    />
  );
}

function Select({ value, onChange, options }: { value: string; onChange: (v: string) => void; options: { value: string; label: string }[] }) {
  return (
    <select
      value={value}
      onChange={e => onChange(e.target.value)}
      className="w-full bg-slate-800 border border-white/10 rounded-xl px-4 py-2.5 text-sm text-white focus:border-indigo-500 focus:outline-none transition-colors"
    >
      {options.map(o => <option key={o.value} value={o.value}>{o.label}</option>)}
    </select>
  );
}

function Btn({ onClick, children, variant = 'default', disabled = false, className = '' }: {
  onClick: () => void; children: React.ReactNode; variant?: 'default' | 'danger' | 'success' | 'ghost';
  disabled?: boolean; className?: string;
}) {
  const colors = {
    default: 'bg-indigo-600 hover:bg-indigo-500 text-white',
    danger: 'bg-red-600/20 hover:bg-red-600/40 text-red-400 border border-red-500/30',
    success: 'bg-emerald-600/20 hover:bg-emerald-600/40 text-emerald-400 border border-emerald-500/30',
    ghost: 'bg-white/5 hover:bg-white/10 text-white border border-white/10',
  };
  return (
    <button
      type="button"
      onClick={onClick}
      disabled={disabled}
      className={`px-4 py-2 rounded-xl text-sm font-medium transition-all ${colors[variant]} ${disabled ? 'opacity-40 cursor-not-allowed' : ''} ${className}`}
    >
      {children}
    </button>
  );
}

function Toast({ message, type }: { message: string; type: 'success' | 'error' }) {
  return (
    <div className={`fixed bottom-6 right-6 z-50 px-5 py-3 rounded-xl text-sm font-medium shadow-lg ${type === 'success' ? 'bg-emerald-500 text-white' : 'bg-red-500 text-white'}`}>
      {message}
    </div>
  );
}

// ── Main Settings Page ─────────────────────────────────────────────────────────

export default function SettingsPage({ backendConnected, onBack }: Props) {
  const [s, setS] = useState<SettingsData>(DEFAULTS);
  const [activeSection, setActiveSection] = useState('identity');
  const [saving, setSaving] = useState(false);
  const [toast, setToast] = useState<{ message: string; type: 'success' | 'error' } | null>(null);

  // Identity state
  const [did, setDid] = useState('');
  const [pubKey, setPubKey] = useState('');
  const [profileList, setProfileList] = useState<{ name: string; has_identity: boolean; setup_complete: boolean }[]>([]);
  const [currentProfile, setCurrentProfile] = useState('default');

  // Security state
  const [auditLog, setAuditLog] = useState<unknown[]>([]);
  const [showAudit, setShowAudit] = useState(false);

  // Channels state
  const [myChannels, setMyChannels] = useState<GhostChannel[]>([]);
  const [newChannelName, setNewChannelName] = useState('');
  const [joinInvite, setJoinInvite] = useState('');

  // Rendezvous
  const [rendezvous, setRendezvous] = useState('');
  const [contactQr, setContactQr] = useState('');

  const showToast = (message: string, type: 'success' | 'error') => {
    setToast({ message, type });
    setTimeout(() => setToast(null), 3000);
  };

  const upd = useCallback(<K extends keyof SettingsData>(k: K, v: SettingsData[K]) => {
    setS(prev => ({ ...prev, [k]: v }));
  }, []);

  // ── Load ──────────────────────────────────────────────────────────────────────

  useEffect(() => {
    if (!backendConnected) return;

    settingsApi.get().then(r => { if (r.data) setS(prev => ({ ...prev, ...r.data })); }).catch(() => {});

    profile.get().then(r => {
      if (r.data) {
        setDid(r.data.did || '');
        upd('nickname', r.data.nickname || '');
        upd('bio', r.data.bio || '');
        upd('avatar_url', r.data.avatar_url || null);
      }
    }).catch(() => {});

    profile.getPublicKey().then(r => {
      if (r.data?.public_key) setPubKey(r.data.public_key);
    }).catch(() => {});

    profile.list().then(r => {
      if (r.data) {
        setProfileList(r.data.profiles || []);
        setCurrentProfile(r.data.current || 'default');
      }
    }).catch(() => {});

    pairing.getRendezvous().then(r => {
      if (r.data?.rendezvous_address) setRendezvous(r.data.rendezvous_address);
    }).catch(() => {});

    channels.list().then(r => {
      if (r.data) setMyChannels(r.data as GhostChannel[]);
    }).catch(() => {});
  }, [backendConnected]);

  // ── Save ──────────────────────────────────────────────────────────────────────

  const save = async () => {
    setSaving(true);
    try {
      await settingsApi.update(s);
      await profile.update({ nickname: s.nickname, bio: s.bio, avatar_url: s.avatar_url });
      showToast('Settings saved', 'success');
    } catch (e: unknown) {
      showToast(`Save failed: ${(e as Error).message}`, 'error');
    } finally {
      setSaving(false);
    }
  };

  // ── Copy helper ───────────────────────────────────────────────────────────────

  const copy = (text: string, label: string) => {
    navigator.clipboard.writeText(text).then(() => showToast(`${label} copied`, 'success'));
  };

  // ── Sections ──────────────────────────────────────────────────────────────────

  const renderIdentity = () => (
    <div className="space-y-6">
      <div className="grid grid-cols-2 gap-6">
        <div className="space-y-4">
          <Field label="Nickname">
            <Input value={s.nickname} onChange={v => upd('nickname', v)} placeholder="Ghost_Operative" />
          </Field>
          <Field label="Bio">
            <textarea
              value={s.bio}
              onChange={e => upd('bio', e.target.value)}
              rows={3}
              className="w-full bg-slate-800 border border-white/10 rounded-xl px-4 py-2.5 text-sm text-white placeholder-slate-600 focus:border-indigo-500 focus:outline-none resize-none"
              placeholder="Sovereign node operator..."
            />
          </Field>
          <Field label="Status message">
            <Input value={s.status_message || ''} onChange={v => upd('status_message', v || null)} placeholder="Available" />
          </Field>
        </div>

        <div className="space-y-4 bg-indigo-500/5 rounded-2xl p-5 border border-indigo-500/10">
          <p className="text-xs font-bold text-indigo-400 uppercase tracking-widest">Cryptographic identity</p>

          <Field label="Sovereign DID">
            <div className="flex gap-2">
              <div className="flex-1 bg-black/40 rounded-lg px-3 py-2 text-[11px] font-mono text-slate-400 break-all">
                {did || 'Locked — unlock profile first'}
              </div>
              <Btn onClick={() => copy(did, 'DID')} variant="ghost" className="px-3 text-xs">Copy</Btn>
            </div>
          </Field>

          <Field label="Ed25519 public key">
            <div className="flex gap-2">
              <div className="flex-1 bg-black/40 rounded-lg px-3 py-2 text-[11px] font-mono text-slate-400 break-all">
                {pubKey ? pubKey.slice(0, 64) + '...' : 'Locked'}
              </div>
              <Btn onClick={() => copy(pubKey, 'Public key')} variant="ghost" className="px-3 text-xs">Copy</Btn>
            </div>
          </Field>

          <Field label="Rendezvous address">
            <div className="flex gap-2">
              <div className="flex-1 bg-black/40 rounded-lg px-3 py-2 text-[11px] font-mono text-slate-400">
                {rendezvous || 'Generating...'}
              </div>
              <Btn onClick={() => copy(rendezvous, 'Rendezvous address')} variant="ghost" className="px-3 text-xs">Copy</Btn>
            </div>
          </Field>

          <div className="flex gap-2 pt-2">
            <Btn onClick={async () => {
              const r = await pairing.getContactCard().catch(() => null);
              if (r?.data?.qr_string) { setContactQr(r.data.qr_string); showToast('Contact card ready', 'success'); }
            }} variant="ghost" className="flex-1 text-xs">Generate QR</Btn>
            <Btn onClick={async () => {
              const r = await pairing.generateCode().catch(() => null);
              if (r?.data) copy(r.data.code, 'Pairing code');
            }} variant="ghost" className="flex-1 text-xs">Pairing code</Btn>
          </div>

          {contactQr && (
            <div className="bg-black/40 rounded-lg p-3 text-[9px] font-mono text-slate-500 break-all">{contactQr}</div>
          )}
        </div>
      </div>
    </div>
  );

  const renderNetwork = () => (
    <div className="space-y-4">
      <div className="grid grid-cols-3 gap-4">
        <Field label="Mesh port"><Input type="number" value={s.listen_port} onChange={v => upd('listen_port', parseInt(v) || 4001)} /></Field>
        <Field label="API port"><Input type="number" value={s.api_port} onChange={v => upd('api_port', parseInt(v) || 8080)} /></Field>
        <Field label="Max peers"><Input type="number" value={s.max_peers} onChange={v => upd('max_peers', parseInt(v) || 50)} /></Field>
      </div>

      <Row label="mDNS discovery" hint="Auto-find peers on local network">
        <Toggle value={s.mdns_enabled} onChange={v => upd('mdns_enabled', v)} />
      </Row>
      <Row label="DHT network" hint="Kademlia distributed peer discovery">
        <Toggle value={s.dht_enabled} onChange={v => upd('dht_enabled', v)} />
      </Row>

      <Field label="Transport mode" hint="WebSocket and Shadowsocks camouflage your traffic from DPI">
        <Select value={s.transport_mode} onChange={v => upd('transport_mode', v as SettingsData['transport_mode'])} options={[
          { value: 'stealth', label: 'Stealth TCP (HMAC handshake)' },
          { value: 'websocket', label: 'WebSocket camouflage (bypass DPI)' },
          { value: 'shadowsocks', label: 'Shadowsocks AEAD (censorship resistance)' },
          { value: 'obfs4', label: 'obfs4 (alpha — not yet functional)' },
        ]} />
      </Field>

      <Field label="Mixing delay preset" hint="Higher anonymity = more delay before forwarding messages">
        <Select value={s.mixing_preset} onChange={v => upd('mixing_preset', v as SettingsData['mixing_preset'])} options={[
          { value: 'low_latency', label: 'Low latency (50ms avg) — reduced anonymity' },
          { value: 'balanced', label: 'Balanced (200ms avg) — recommended' },
          { value: 'max_anonymity', label: 'Max anonymity (1000ms avg) — slow but strongest' },
        ]} />
      </Field>

      <Field label={`Cover traffic intensity: ${s.cover_traffic_pct}%`} hint="Dummy packets sent to obscure silence patterns">
        <input type="range" min={0} max={100} step={5} value={s.cover_traffic_pct}
          onChange={e => upd('cover_traffic_pct', parseInt(e.target.value))}
          className="w-full accent-indigo-500" />
      </Field>
    </div>
  );

  const renderSecurity = () => (
    <div className="space-y-4">
      <Row label="ML-KEM-768 (Post-quantum)" hint="Kyber hybrid key exchange — protects against future quantum computers">
        <Toggle value={s.pqc_enabled} onChange={v => upd('pqc_enabled', v)} />
      </Row>
      <Row label="Sphinx routing for DMs" hint="Onion-routes direct messages through relay nodes — sender unlinkable">
        <Toggle value={s.sphinx_dms} onChange={v => upd('sphinx_dms', v)} />
      </Row>

      <div className="grid grid-cols-3 gap-4">
        <Field label="Auto-lock (minutes)">
          <Input type="number" value={s.auto_lock_minutes} onChange={v => upd('auto_lock_minutes', parseInt(v) || 15)} />
        </Field>
        <Field label="Key rotation (days)">
          <Input type="number" value={s.key_rotation_days} onChange={v => upd('key_rotation_days', parseInt(v) || 30)} />
        </Field>
        <Field label="Lockout after N attempts">
          <Input type="number" value={s.failed_attempts_lockout} onChange={v => upd('failed_attempts_lockout', parseInt(v) || 5)} />
        </Field>
      </div>

      <div className="flex gap-3 flex-wrap">
        <Btn onClick={async () => {
          try { await security.rotateKeys(); showToast('Keys rotated successfully', 'success'); }
          catch (e) { showToast('Key rotation failed', 'error'); }
        }} variant="ghost">Rotate keys now</Btn>

        <Btn onClick={async () => {
          try {
            const r = await security.getAuditLog();
            setAuditLog((r.data as unknown[]) || []);
            setShowAudit(true);
          } catch { showToast('Could not load audit log', 'error'); }
        }} variant="ghost">View audit log</Btn>

        <Btn onClick={async () => {
          try { await security.lockdown(); showToast('Lockdown enabled — all connections blocked', 'success'); }
          catch { showToast('Lockdown failed', 'error'); }
        }} variant="danger">Lockdown</Btn>
      </div>

      {showAudit && (
        <div className="bg-black/40 rounded-xl p-4 max-h-64 overflow-y-auto">
          <div className="flex justify-between mb-3">
            <span className="text-xs font-bold text-slate-400 uppercase">Audit log</span>
            <button onClick={() => setShowAudit(false)} className="text-slate-600 hover:text-white text-xs">Close</button>
          </div>
          {auditLog.length === 0
            ? <p className="text-xs text-slate-600">No audit events recorded.</p>
            : auditLog.map((e, i) => (
                <div key={i} className="text-xs font-mono text-slate-400 py-1 border-b border-white/5">
                  {JSON.stringify(e)}
                </div>
              ))
          }
        </div>
      )}
    </div>
  );

  const renderPrivacy = () => (
    <div className="space-y-3">
      <Field label="Presence visibility" hint="Who can see when you're online">
        <Select value={s.presence} onChange={v => upd('presence', v as SettingsData['presence'])} options={[
          { value: 'off', label: 'Off — never show online status' },
          { value: 'contacts', label: 'Contacts only' },
          { value: 'public', label: 'Public — visible to all mesh peers' },
        ]} />
      </Field>

      <Row label="Read receipts" hint="Send delivery/read confirmations to senders">
        <Toggle value={s.read_receipts} onChange={v => upd('read_receipts', v)} />
      </Row>
      <Row label="Message preview in notifications" hint="Show content preview (disable for max privacy)">
        <Toggle value={s.message_preview} onChange={v => upd('message_preview', v)} />
      </Row>
      <Row label="Strip file metadata" hint="Remove EXIF/ID3 tags before sending files">
        <Toggle value={s.strip_metadata} onChange={v => upd('strip_metadata', v)} />
      </Row>
      <Row label="Traffic morphing" hint="Pad packets to fixed sizes — hides message lengths">
        <Toggle value={s.traffic_morphing} onChange={v => upd('traffic_morphing', v)} />
      </Row>
      <Row label="Stealth mode" hint="Hide from mDNS network scans">
        <Toggle value={s.stealth_mode} onChange={v => upd('stealth_mode', v)} />
      </Row>
      <Row label="Anonymous sender (Sealed Sender)" hint="Recipients cannot identify you as sender — like Signal's sealed sender">
        <Toggle value={s.anonymous_sender} onChange={v => upd('anonymous_sender', v)} />
      </Row>
    </div>
  );

  const renderChannels = () => (
    <div className="space-y-5">
      <p className="text-sm text-slate-400">Ghost Channels are anonymous broadcast rooms. No owner, no member list, no sender attribution. Share the invite code to let others join.</p>

      <div className="flex gap-3">
        <Input value={newChannelName} onChange={setNewChannelName} placeholder="Channel name (optional)" className="flex-1" />
        <Btn onClick={async () => {
          try {
            const r = await channels.create(newChannelName || undefined);
            if (r.data?.channel) {
              setMyChannels(prev => [...prev, r.data!.channel as GhostChannel]);
              setNewChannelName('');
              showToast('Channel created', 'success');
            }
          } catch { showToast('Failed to create channel', 'error'); }
        }}>Create channel</Btn>
      </div>

      <div className="flex gap-3">
        <Input value={joinInvite} onChange={setJoinInvite} placeholder="gwc1:... invite string" className="flex-1" />
        <Btn variant="ghost" onClick={async () => {
          try {
            const r = await channels.join(joinInvite);
            if (r.data) {
              setMyChannels(prev => [...prev, r.data as GhostChannel]);
              setJoinInvite('');
              showToast('Joined channel', 'success');
            }
          } catch { showToast('Invalid invite or already joined', 'error'); }
        }}>Join</Btn>
      </div>

      {myChannels.length === 0
        ? <p className="text-sm text-slate-600 italic">No channels joined yet.</p>
        : myChannels.map((ch, i) => (
            <div key={i} className="flex items-center justify-between p-4 bg-slate-800/50 rounded-xl border border-white/5">
              <div>
                <div className="text-sm font-medium text-white">{ch.local_name || 'Unnamed channel'}</div>
                <div className="text-xs text-slate-500 font-mono mt-1">
                  {ch.invite_string || Array.from(ch.topic_id.slice(0, 8)).map(b => b.toString(16).padStart(2, '0')).join('')}...
                </div>
              </div>
              <div className="flex gap-2">
                <Btn onClick={() => copy(ch.invite_string || '', 'Invite')} variant="ghost" className="text-xs">Share</Btn>
                <Btn onClick={async () => {
                  const id = Array.from(ch.topic_id).map(b => b.toString(16).padStart(2, '0')).join('');
                  await channels.leave(id);
                  setMyChannels(prev => prev.filter((_, j) => j !== i));
                }} variant="danger" className="text-xs">Leave</Btn>
              </div>
            </div>
          ))
      }
    </div>
  );

  const renderArk = () => (
    <div className="space-y-4">
      <Row label="Auto-sync public entries" hint="Automatically share public entries with connected peers">
        <Toggle value={s.ark_auto_sync} onChange={v => upd('ark_auto_sync', v)} />
      </Row>
      <Field label={`Max local Ark size: ${s.ark_max_mb} MB`}>
        <input type="range" min={100} max={5000} step={100} value={s.ark_max_mb}
          onChange={e => upd('ark_max_mb', parseInt(e.target.value))}
          className="w-full accent-indigo-500" />
      </Field>
      <Field label="Default collection for new entries">
        <Input value={s.default_collection} onChange={v => upd('default_collection', v)} placeholder="default" />
      </Field>
      <div className="flex gap-3">
        <Btn variant="ghost" onClick={async () => {
          const r = await ark_import();
          showToast(r ? 'Export started' : 'Export failed', r ? 'success' : 'error');
        }}>Export all entries</Btn>
      </div>
    </div>
  );

  const renderNotifications = () => (
    <div className="space-y-3">
      <Row label="Sound effects" hint="Play sounds for new messages and events">
        <Toggle value={s.sound_enabled} onChange={v => upd('sound_enabled', v)} />
      </Row>
      <Row label="Desktop notifications" hint="Show system notifications when app is in background">
        <Toggle value={s.desktop_notifications} onChange={v => upd('desktop_notifications', v)} />
      </Row>
      <Row label="Message preview" hint="Show message content in notifications">
        <Toggle value={s.message_preview} onChange={v => upd('message_preview', v)} />
      </Row>
      <Row label="Connection request alerts" hint="Notify when a peer wants to connect">
        <Toggle value={s.notify_connection_requests} onChange={v => upd('notify_connection_requests', v)} />
      </Row>
      <Row label="Ark sync alerts" hint="Notify when knowledge entries are synced from peers">
        <Toggle value={s.notify_ark_sync} onChange={v => upd('notify_ark_sync', v)} />
      </Row>
    </div>
  );

  const renderAdvanced = () => (
    <div className="space-y-4">
      <Row label="Debug mode" hint="Enable verbose logging — never use in production">
        <Toggle value={s.debug_mode} onChange={v => upd('debug_mode', v)} />
      </Row>
      <Field label="Log level">
        <Select value={s.log_level} onChange={v => upd('log_level', v as SettingsData['log_level'])} options={[
          { value: 'error', label: 'Error only' },
          { value: 'warn', label: 'Warnings' },
          { value: 'info', label: 'Info (recommended)' },
          { value: 'debug', label: 'Debug' },
          { value: 'trace', label: 'Trace (very verbose)' },
        ]} />
      </Field>
      <Row label="MessagePack wire format" hint="50% smaller than JSON — disable only for debugging">
        <Toggle value={s.msgpack_wire} onChange={v => upd('msgpack_wire', v)} />
      </Row>
      <Row label="Auto-update" hint="Automatically apply signed updates — requires valid release key">
        <Toggle value={s.auto_update} onChange={v => upd('auto_update', v)} />
      </Row>
    </div>
  );

  const renderDanger = () => (
    <div className="space-y-5">
      <div className="p-5 bg-red-500/5 border border-red-500/20 rounded-2xl space-y-4">
        <p className="text-xs font-bold text-red-400 uppercase tracking-widest">Profile management</p>
        <div className="space-y-2 max-h-48 overflow-y-auto">
          {profileList.map(p => (
            <div key={p.name} className="flex items-center justify-between px-4 py-2.5 bg-black/20 rounded-xl border border-white/5">
              <div className="flex items-center gap-2">
                <div className={`w-2 h-2 rounded-full ${p.name === currentProfile ? 'bg-indigo-500' : 'bg-slate-700'}`} />
                <span className={`text-sm font-mono ${p.name === currentProfile ? 'text-indigo-400 font-bold' : 'text-slate-300'}`}>
                  {p.name} {p.name === currentProfile && '(active)'}
                </span>
              </div>
              <Btn
                disabled={p.name === currentProfile}
                variant="danger"
                onClick={async () => {
                  if (!confirm(`Permanently delete profile "${p.name}"? This cannot be undone.`)) return;
                  const ok = await api.deleteProfile(p.name);
                  if (ok) { showToast('Profile deleted', 'success'); profile.list().then(r => setProfileList(r.data?.profiles || [])); }
                  else showToast('Failed to delete profile', 'error');
                }}
                className="text-xs"
              >
                Delete
              </Btn>
            </div>
          ))}
          {profileList.length === 0 && <p className="text-sm text-slate-600 italic">No profiles found</p>}
        </div>
      </div>

      <div className="p-5 bg-red-950/20 border border-red-500/30 rounded-2xl flex items-center justify-between">
        <div>
          <div className="text-sm font-bold text-red-400">Emergency wipe</div>
          <div className="text-xs text-red-300/40 mt-1">Permanently erases all profiles, messages, and keys. Shutdown follows.</div>
        </div>
        <Btn variant="danger" onClick={async () => {
          if (!confirm('⚠️ CRITICAL: Erase all Ghostwire data on this machine? This cannot be undone.')) return;
          if (!confirm('FINAL CONFIRMATION: Proceed with total wipe?')) return;
          const ok = await api.wipeSystem();
          if (ok) { showToast('Wipe complete. Shutting down...', 'success'); setTimeout(() => window.location.reload(), 2000); }
          else showToast('Wipe failed', 'error');
        }} className="font-bold">Self-destruct</Btn>
      </div>
    </div>
  );

  const CONTENT: Record<string, () => JSX.Element> = {
    identity: renderIdentity,
    network: renderNetwork,
    security: renderSecurity,
    privacy: renderPrivacy,
    channels: renderChannels,
    ark: renderArk,
    notifications: renderNotifications,
    advanced: renderAdvanced,
    danger: renderDanger,
  };

  return (
    <div className="flex flex-col h-full animate-fade-in">
      {toast && <Toast {...toast} />}

      {/* Header */}
      <div className="flex items-center justify-between pb-5 border-b border-white/10 mb-5">
        <div className="flex items-center gap-4">
          <button onClick={onBack} className="text-slate-400 hover:text-white transition-colors text-sm">← Back</button>
          <h1 className="text-xl font-bold text-white">Settings</h1>
        </div>
        <button
          onClick={save}
          disabled={saving || !backendConnected}
          className={`px-5 py-2 rounded-xl text-sm font-semibold transition-all ${saving ? 'bg-slate-700 text-slate-400' : 'bg-indigo-600 hover:bg-indigo-500 text-white'}`}
        >
          {saving ? 'Saving...' : 'Save changes'}
        </button>
      </div>

      <div className="flex gap-6 flex-1 overflow-hidden">
        {/* Sidebar */}
        <nav className="w-48 flex-shrink-0 space-y-1">
          {SECTIONS.map(sec => (
            <button
              key={sec.id}
              onClick={() => setActiveSection(sec.id)}
              className={`w-full flex items-center gap-3 px-3 py-2.5 rounded-xl text-left text-sm transition-all ${
                activeSection === sec.id
                  ? 'bg-indigo-600/20 text-white border border-indigo-500/30'
                  : 'text-slate-400 hover:bg-white/5 hover:text-white'
              } ${sec.id === 'danger' ? 'text-red-400 hover:text-red-300' : ''}`}
            >
              <span className="font-mono">{sec.icon}</span>
              <span>{sec.label}</span>
            </button>
          ))}
        </nav>

        {/* Content */}
        <div className="flex-1 overflow-y-auto bg-slate-900/50 rounded-2xl border border-white/10 p-6">
          {CONTENT[activeSection]?.()}
        </div>
      </div>
    </div>
  );
}

// placeholder until ark controller wired
const ark_import = async () => true;
