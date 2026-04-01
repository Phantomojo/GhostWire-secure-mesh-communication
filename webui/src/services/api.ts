// api.ts — Ghostwire backend client
// SECURITY FIXES:
//   1. Auth via httpOnly cookies (server sets Set-Cookie) — not localStorage
//   2. /dev/token auto-auth removed — only in debug builds
//   3. API base URL read once at init, not from localStorage on every call
//   4. No polling — callers use WebSocket for real-time updates

const resolveBaseUrl = (): string => {
  if (import.meta.env.VITE_API_BASE_URL) return import.meta.env.VITE_API_BASE_URL;
  const h = window.location.hostname;
  const port = '8080';
  if (h !== 'localhost' && h !== '127.0.0.1') return `http://${h}:${port}/api`;
  return 'http://localhost:8080/api';
};

// Read once at module init — never from localStorage
export const API_BASE = resolveBaseUrl();
export const WS_URL = API_BASE.replace(/^http/, 'ws').replace('/api', '') + '/api/ws';

// ── Types ────────────────────────────────────────────────────────────────────

export interface ApiEnvelope<T> {
  version: string;
  timestamp: string;
  request_id: string;
  data?: T;
  error?: { code: string; message: string; details?: unknown };
}

export interface IdentityStatus {
  locked: boolean;
  has_identity: boolean;
  did?: string;
  username?: string;
  nickname?: string;
  setup_complete?: boolean;
}

export interface ProfileV2 {
  id: string;
  did: string;
  username: string;
  nickname: string;
  bio: string;
  public_key: string;
  created_at: string;
  security_score: number;
}

export interface PeerV2 {
  id: string;
  node_id: string;
  username: string;
  nickname?: string;
  status: 'online' | 'offline' | 'connecting';
  last_seen: string;
  trust_level: string;
  trust_score: number;
  public_key: string;
  discovery_method: string;
  latency_ms?: number;
}

export interface MessageV2 {
  id: string;
  sender: string;
  recipient: string;
  content: string;
  timestamp: number;
  encrypted: boolean;
  message_type: string;
}

export interface SecurityDashboardV2 {
  score: number;
  encryption: { method: string; active: boolean; pqc_armor: boolean };
  stats: {
    messages_encrypted: number;
    peers_verified: number;
    rate_limit_violations: number;
    blocked_peers: number;
  };
  threat_level: 'none' | 'low' | 'medium' | 'high' | 'critical';
}

export interface MeshHealthV2 {
  timestamp: number;
  network_state: {
    state: string;
    internet_reachable: boolean;
    active_transport: string;
    latency_ms: number;
    packet_loss_pct: number;
  };
  peers: Array<{
    node_id: string;
    nickname?: string;
    is_online: boolean;
    latency_ms?: number;
    trust_level: string;
    signal_strength: number;
  }>;
  routing: { routing_table_size: number; sphinx_sessions: number; mixing_queue_depth: number };
  storage: { message_count: number; ark_entry_count: number; mailbox_pending: number };
  security: { identity_locked: boolean; blocked_peers: number; failed_unlock_attempts_24h: number };
  performance: { dedup_cache_size: number; ws_subscriber_count: number };
}

export interface GhostChannel {
  topic_id: string;
  local_name?: string;
  description?: string;
  created_at: number;
  invite_string: string;
}

export interface ArkEntry {
  cid: string;
  title: string;
  body: string;
  collection: string;
  tags: string[];
  kind: string;
  access: string;
  author: string;
  created_at: number;
}

export interface ArkStats {
  total_entries: number;
  total_collections: number;
  total_tags: number;
}

export interface TransportStatus {
  tier: 'stealth' | 'webrtc' | 'covert' | 'fronted' | 'offline';
  display_name: string;
  latency_ms: number;
  internet_reachable: boolean;
}

// ── Core fetch wrapper ───────────────────────────────────────────────────────

async function request<T>(
  method: string,
  path: string,
  body?: unknown
): Promise<T> {
  const url = `${API_BASE}${path}`;
  const res = await fetch(url, {
    method,
    credentials: 'include',              // httpOnly cookie auth — no localStorage
    headers: { 'Content-Type': 'application/json' },
    body: body !== undefined ? JSON.stringify(body) : undefined,
  });

  const ct = res.headers.get('content-type') ?? '';
  if (!ct.includes('application/json')) {
    throw new Error(`Non-JSON response from ${path}: ${res.status}`);
  }

  const envelope: ApiEnvelope<T> = await res.json();

  if (!res.ok || envelope.error) {
    const e = envelope.error ?? { code: 'HTTP_ERROR', message: res.statusText };
    throw Object.assign(new Error(e.message), { code: e.code, status: res.status });
  }

  return envelope.data as T;
}

const get  = <T>(path: string)              => request<T>('GET',    path);
const post = <T>(path: string, b?: unknown) => request<T>('POST',   path, b);
const put  = <T>(path: string, b?: unknown) => request<T>('PUT',    path, b);
const del  = <T>(path: string)              => request<T>('DELETE', path);

// ── Auth ─────────────────────────────────────────────────────────────────────

export const auth = {
  unlock: (password: string) =>
    post<void>('/identity/unlock', { password }),

  lock: () => post<void>('/identity/lock'),

  setup: (data: { nickname: string; password: string }) =>
    post<string>('/identity/setup', data),

  status: () => get<IdentityStatus>('/identity/status'),

  // NOTE: /dev/token endpoint removed from production builds.
  // Dev auto-auth was a security hole. In Rust: #[cfg(debug_assertions)] only.
};

// ── Profile ───────────────────────────────────────────────────────────────────

export const profile = {
  get:    () => get<ProfileV2>('/v2/profile'),
  update: (data: Partial<Pick<ProfileV2, 'nickname' | 'bio'>>) =>
    put<ProfileV2>('/v2/profile/save', data),
  list:   () => get<{ profiles: string[]; current: string | null }>('/profile/list'),
  select: (name: string) => post<void>('/profile/select', { profile_name: name }),
  create: (name: string) => post<void>('/profile/create', { name }),
  delete: (name: string) => del<void>(`/profile/delete/${name}`),
};

// ── Peers ─────────────────────────────────────────────────────────────────────

export const peers = {
  list:      (page = 1) => get<PeerV2[]>(`/v2/peers?page=${page}`),
  nearby:    ()         => get<PeerV2[]>('/v2/peers/nearby'),
  connect:   (node_id: string, message?: string) =>
    post<void>('/v2/peers/connect', { node_id, message }),
  block:     (id: string) => post<void>(`/v2/peers/${id}/block`),
  ping:      (node_id: string) => post<{ latency_ms: number }>('/v2/peers/ping', { node_id }),
};

// ── Messages ──────────────────────────────────────────────────────────────────

export const messages = {
  list:   (peer_id = 'all') => get<MessageV2[]>(`/v2/messages?peer_id=${peer_id}`),
  send:   (recipient: string, content: string) =>
    post<void>('/v2/messages/send', { recipient, content }),
  broadcast: (content: string) =>
    post<void>('/broadcast', { message: content }),
};

// ── Security ──────────────────────────────────────────────────────────────────

export const security = {
  dashboard: () => get<SecurityDashboardV2>('/v2/security/dashboard'),
  audit:     () => get<unknown[]>('/v2/security/audit'),
  lockdown:  (enabled: boolean) => post<void>('/security/lockdown', { enabled }),
  rotateKeys: () => post<void>('/security/rotate-keys'),
};

// ── Network / mesh health ─────────────────────────────────────────────────────

export const network = {
  status:     () => get<MeshHealthV2['network_state']>('/v2/network/status'),
  meshHealth: () => get<MeshHealthV2>('/v2/system/mesh-health'),
  systemStatus: () => get<{
    cpu_usage: number;
    memory_used_kb: number;
    memory_total_kb: number;
    uptime_seconds: number;
    peer_count: number;
  }>('/status'),
  transport: (): Promise<TransportStatus> =>
    network.meshHealth().then(h => ({
      tier: mapTransportTier(h.network_state.active_transport),
      display_name: h.network_state.active_transport,
      latency_ms: h.network_state.latency_ms,
      internet_reachable: h.network_state.internet_reachable,
    })).catch(() => ({
      tier: 'offline' as const,
      display_name: 'offline',
      latency_ms: 0,
      internet_reachable: false,
    })),
};

function mapTransportTier(s: string): TransportStatus['tier'] {
  if (s.includes('webrtc') && s.includes('stego')) return 'covert';
  if (s.includes('webrtc')) return 'webrtc';
  if (s.includes('front')) return 'fronted';
  if (s.includes('stealth') || s.includes('tcp')) return 'stealth';
  return 'offline';
}

// ── Ghost Channels ────────────────────────────────────────────────────────────

export const channels = {
  list:     () => get<GhostChannel[]>('/v2/channels'),
  create:   (name?: string) => post<GhostChannel>('/v2/channels', { name }),
  join:     (invite: string) => post<GhostChannel>('/v2/channels', { invite }),
  leave:    (id: string) => post<void>(`/v2/channels/${id}/leave`),
  publish:  (id: string, text: string) => post<void>(`/v2/channels/${id}/publish`, { text }),
  messages: (id: string) => get<string[]>(`/v2/channels/${id}/messages`),
};

// ── Knowledge Ark ─────────────────────────────────────────────────────────────

export const ark = {
  stats:       () => get<ArkStats>('/v2/ark/stats'),
  collections: () => get<string[]>('/v2/ark/collections'),
  search:      (q: string) => get<Array<{ entry: ArkEntry; score: number }>>(`/v2/ark/search?q=${encodeURIComponent(q)}`),
  byTag:       (tag: string) => get<ArkEntry[]>(`/v2/ark/tags/${encodeURIComponent(tag)}`),
  get:         (cid: string) => get<ArkEntry>(`/v2/ark/entries/${cid}`),
  create:      (data: Partial<ArkEntry>) => post<{ cid: string }>('/v2/ark/entries', data),
  delete:      (cid: string) => del<void>(`/v2/ark/entries/${cid}`),
};

// ── Trust ─────────────────────────────────────────────────────────────────────

export const trust = {
  get:    (peer_id: string) => get<unknown>(`/v2/trust/${peer_id}`),
  verify: (peer_id: string, public_key: string) =>
    post<void>(`/v2/trust/${peer_id}/verify`, { public_key }),
  block:  (peer_id: string) => post<void>(`/v2/trust/${peer_id}/block`),
  list:   () => get<unknown[]>('/v2/trust'),
};

// ── Recovery ──────────────────────────────────────────────────────────────────

export const recovery = {
  setup: (data: {
    threshold: number;
    total: number;
    guardians: Array<{ peer_id: string; name: string; x25519_pub_hex: string }>;
  }) => post<unknown>('/v2/recovery/setup', data),
  myShares: () => get<unknown[]>('/v2/recovery/my-shares'),
};

// ── WebSocket client ──────────────────────────────────────────────────────────

export type WsEvent =
  | { type: 'message'; payload: MessageV2 }
  | { type: 'peer_connected'; payload: { peer_id: string } }
  | { type: 'peer_disconnected'; payload: { peer_id: string } }
  | { type: 'network_update'; payload: { active_peers: number; latency_ms: number } }
  | { type: 'security_event'; payload: { level: string; message: string } }
  | { type: 'dtn_bundle'; payload: { cid: string; from: string } }
  | { type: 'channel_message'; payload: { channel_id: string; text: string } };

export class GhostWireSocket {
  private ws: WebSocket | null = null;
  private attempts = 0;
  private dead = false;
  private handlers = new Set<(e: WsEvent) => void>();

  connect() {
    if (this.dead) return;
    try {
      this.ws = new WebSocket(WS_URL);
      this.ws.onopen    = () => { this.attempts = 0; };
      this.ws.onmessage = (e) => {
        try {
          const parsed = JSON.parse(e.data);
          const evt = normaliseWsEvent(parsed);
          if (evt) this.handlers.forEach(h => h(evt));
        } catch {}
      };
      this.ws.onclose = () => { if (!this.dead) this.scheduleReconnect(); };
      this.ws.onerror = () => {};
    } catch {}
  }

  on(handler: (e: WsEvent) => void) {
    this.handlers.add(handler);
    return () => this.handlers.delete(handler);
  }

  send(data: unknown) {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(data));
    }
  }

  disconnect() {
    this.dead = true;
    this.ws?.close();
  }

  private scheduleReconnect() {
    if (this.attempts >= 8) return;
    const delay = Math.min(1000 * 2 ** this.attempts, 30_000);
    this.attempts++;
    setTimeout(() => this.connect(), delay);
  }
}

function normaliseWsEvent(raw: unknown): WsEvent | null {
  if (!raw || typeof raw !== 'object') return null;
  const r = raw as Record<string, unknown>;

  // Backend sends: { type: "core_event", payload: { "NewMessage": {...} } }
  // or:            { type: "message", message: {...} }
  if (r.type === 'core_event' && r.payload && typeof r.payload === 'object') {
    const p = r.payload as Record<string, unknown>;
    if (p.NewMessage)       return { type: 'message',            payload: p.NewMessage as MessageV2 };
    if (p.PeerConnected)    return { type: 'peer_connected',     payload: { peer_id: p.PeerConnected as string } };
    if (p.PeerDisconnected) return { type: 'peer_disconnected',  payload: { peer_id: p.PeerDisconnected as string } };
    if (p.NetworkUpdate)    return { type: 'network_update',     payload: p.NetworkUpdate as { active_peers: number; latency_ms: number } };
  }
  if (r.type === 'message' && r.message) {
    return { type: 'message', payload: r.message as MessageV2 };
  }
  return null;
}

// Singleton socket instance
export const socket = new GhostWireSocket();
