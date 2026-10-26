export interface RuntimeVersionV2 {
  version: string;
  protocol_version: string;
  build_hash: string | null;
  git_revision: string | null;
  asset_manifest_hash: string | null;
  identity_id: string;
}

// api.ts — Ghostwire backend client
// SECURITY FIXES:
//   1. Auth via httpOnly cookies (server sets Set-Cookie) — not localStorage
//   2. /dev/token auto-auth removed — only in debug builds
//   3. API base URL read once at init, not from localStorage on every call
//   4. No polling — callers use WebSocket for real-time updates

const resolveBaseUrl = (): string => {
  if (import.meta.env.VITE_API_BASE_URL) return import.meta.env.VITE_API_BASE_URL;
  // Follow the page origin. This works for the Vite dev proxy and for a
  // remotely served embedded UI such as http://100.110.215.46:9918.
  return `${window.location.protocol}//${window.location.host}/api`;
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
  encryption_status?: string;
  delivery_status?: string;
  read?: boolean;
}

export interface SecurityDashboardV2 {
  score: number;
  encryption: { method: string; active: boolean; messages_encrypted: number; pqc_armor: boolean };
  sybil_defense: { method: string; difficulty: number; verified_peers: number };
  rate_limiting: { active: boolean; violations_today: number };
  blacklist: { entries: number; recent_additions: number };
  stats: {
    messages_encrypted: number;
    peers_verified: number;
    rate_limit_violations: number;
    blocked_peers: number;
  };
  threat_level: 'none' | 'low' | 'medium' | 'high' | 'critical' | string;
  last_scan: string;
}

export interface WorldDiscoveryHealthV2 {
  enabled: boolean;
  network_id: string;
  topic: string;
  state: string;
  published_total: number;
  accepted_total: number;
  rejected_total: number;
  expired_total: number;
  candidate_count: number;
  last_published_unix: number;
  last_received_unix: number;
  provider_active: boolean;
  last_error?: string | null;
}

export interface DiscoverySnapshotV2 {
  generated_at_unix: number;
  local: { peer_id: string; listener_state: string };
  methods: {
    nearby: { state: string; active_scan: boolean; count: number; note: string };
    world_gossip: {
      state: string;
      enabled: boolean;
      published: number;
      accepted: number;
      rejected: number;
      candidates: number;
      last_error?: string | null;
    };
    kademlia: { state: string; provider_active: boolean };
  };
  candidates: Array<{
    node_id: string;
    nickname?: string;
    is_online: boolean;
    last_seen: number;
    discovered_via: string;
    source: string;
  }>;
  known_peers: Array<{
    node_id: string;
    nickname?: string;
    is_online: boolean;
    last_seen: number;
    discovered_via: string;
    connection_state: string;
    source: string;
  }>;
  connected_peers: Array<{
    node_id: string;
    nickname?: string;
    is_online: boolean;
    last_seen: number;
    signal: number;
    source: string;
  }>;
  counts: { nearby: number; known: number; connected: number; world_candidates: number };
}

export interface DiscoveryScanV2 {
  scan_id: string;
  methods: {
    nearby: { state: string; active_scan: boolean };
    world_gossip: { state: string; enabled: boolean };
    kademlia: { state: string; provider_active: boolean };
  };
  world_discovery: WorldDiscoveryHealthV2;
  note: string;
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
    last_seen_secs_ago: number;
    discovery_method: string;
  }>;
  routing: { routing_table_size: number; sphinx_sessions: number; mixing_queue_depth: number };
  storage: { message_count: number; ark_entry_count: number; mailbox_pending: number };
  security: { identity_locked: boolean; blocked_peers: number; failed_unlock_attempts_24h: number };
  performance: { dedup_cache_size: number; ws_subscriber_count: number };
  interfaces?: Array<{
    id: string;
    protocol: string;
    medium: string;
    state: string;
    registered: boolean;
    started: boolean;
    ready: boolean;
    deferred: boolean;
    listen_addresses: string[];
    peer_count: number;
    last_error?: string | null;
    discovery?: {
      method: string;
      enabled: boolean;
      state: string;
      discovered_total: number;
      expired_total: number;
      last_event_unix: number;
      nearby_peer_count: number;
    } | null;
  }>;
  world_discovery?: WorldDiscoveryHealthV2;
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
  const token = sessionStorage.getItem('gw_token') ?? '';
  const res = await fetch(url, {
    method,
    headers: {
      'Content-Type': 'application/json',
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
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
  unlock: async (password: string) => {
    const result = await post<{ token?: string }>('/v2/profile/unlock', { password });
    if (result.token) sessionStorage.setItem('gw_token', result.token);
    return result;
  },

  lock: () => post<void>('/v2/profile/lock'),

  setup: async (data: { nickname: string; password: string }) => {
    const result = await post<{ token?: string }>('/identity/setup', data);
    if (result.token) sessionStorage.setItem('gw_token', result.token);
    return result;
  },

  status: () => get<IdentityStatus>('/identity/status'),
};

// ── Profile ───────────────────────────────────────────────────────────────────

export const profile = {
  get:    () => get<ProfileV2>('/v2/profile'),
  update: (data: Partial<Pick<ProfileV2, 'nickname' | 'bio'>>) =>
    put<ProfileV2>('/v2/profile/save', data),
  list:   () => get<{ profiles: string[]; current: string | null }>('/profile/list'),
  select: (name: string) => post<void>('/profile/select', { profile_name: name }),
  // Locked-screen lifecycle endpoints. They are deliberately separate from
  // the authenticated Settings operations and are guarded server-side to
  // first-run/locked state only.
  selectLocked: (name: string) => post<void>('/v2/profile/select-locked', { profile_name: name }),
  create: (data: { name: string; nickname: string; password: string }) =>
    post<void>('/profile/create', data),
  createFirst: (data: { name: string; nickname: string; password: string }) =>
    post<void>('/v2/profile/create-first', data),
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
  audit:     () => get<unknown[]>('/system/audit'),
  lockdown:  (enabled: boolean) => post<void>('/system/lockdown', { enabled }),
  // Backend registers this as PUT (web.rs system_routes), not POST —
  // POST returns 405 Method Not Allowed.
  rotateKeys: () => put<void>('/system/rotate-keys'),
};

export interface TransportRegistryDescriptor {
  name: string;
  description: string;
  registered: boolean;
  implementation: string;
  controls: string[];
}

export interface TransportHealthV2 {
  snapshot: MeshHealthV2;
  registry: TransportRegistryDescriptor[];
}

// ── Network / mesh health ─────────────────────────────────────────────────────

export const network = {
  discovery: {
    health: () => get<WorldDiscoveryHealthV2>('/v2/system/discovery'),
    snapshot: () => get<DiscoverySnapshotV2>('/v2/discovery/snapshot'),
    scan: () => post<DiscoveryScanV2>('/v2/discovery/scan'),
    publish: () => post<WorldDiscoveryHealthV2>('/v2/system/discovery/publish'),
    refresh: () => post<WorldDiscoveryHealthV2>('/v2/system/discovery/refresh'),
  },
  status:     () => get<MeshHealthV2['network_state']>('/v2/network/status'),
  version:    () => get<RuntimeVersionV2>('/v2/system/version'),
  meshHealth: () => get<MeshHealthV2>('/v2/system/mesh-health'),
  transports: () => get<TransportHealthV2>('/v2/system/transports'),
  transportAction: (transport: string, action: 'start' | 'stop') =>
    post<TransportHealthV2>(`/v2/system/transports/${encodeURIComponent(transport)}/action`, { action }),
  // SystemController::status — NOT IdentityController::get_profile.
  // `/identity/status` returns identity metadata and has none of these fields.
  systemStatus: () => get<{
    status: string;
    uptime_seconds: number;
    local_ip: string;
    peer_count: number;
    mesh_active: boolean;
    threat_level: string;
    blocked_connections: number;
  }>('/system/stats'),
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
  private wsSessionKey: CryptoKey | null = null;
  private handlers = new Set<(e: WsEvent) => void>();

  connect() {
    // A lock transition marks the previous socket dead; a later unlock starts
    // a new authenticated lifecycle with a fresh session salt.
    this.dead = false;
    const token = sessionStorage.getItem('gw_token') ?? '';
    if (!token) return;
    this.wsSessionKey = null;
    try {
      this.ws = new WebSocket(`${WS_URL}?token=${encodeURIComponent(token)}`);
      this.ws.onopen = () => { this.attempts = 0; };
      this.ws.onmessage = (e) => { void this.handleMessage(String(e.data)); };
      this.ws.onclose = () => {
        this.wsSessionKey = null;
        if (!this.dead) this.scheduleReconnect();
      };
      this.ws.onerror = () => {};
    } catch {}
  }

  private async handleMessage(data: string) {
    try {
      const parsed = JSON.parse(data) as Record<string, unknown>;
      if (parsed.type === 'session_salt' && typeof parsed.salt === 'string') {
        this.wsSessionKey = await deriveWsSessionKey(parsed.salt);
        return;
      }
      if (parsed.type === 'connected' || parsed.type === 'pong') return;

      let decoded: unknown = parsed;
      if (typeof parsed.nonce === 'string' && typeof parsed.ct === 'string') {
        if (!this.wsSessionKey) return;
        const nonce = decodeBase64(parsed.nonce);
        const ciphertext = decodeBase64(parsed.ct);
        const plaintext = await globalThis.crypto.subtle.decrypt(
          { name: 'AES-GCM', iv: nonce },
          this.wsSessionKey,
          ciphertext,
        );
        decoded = JSON.parse(new TextDecoder().decode(plaintext));
      }
      const evt = normaliseWsEvent(decoded);
      if (evt) this.handlers.forEach(h => h(evt));
    } catch {
      // Encrypted frames with a missing/rotated session key are dropped. The
      // normal HTTP refresh path remains authoritative for recovery.
    }
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
    this.wsSessionKey = null;
    this.ws?.close();
  }

  private scheduleReconnect() {
    if (this.attempts >= 8) return;
    const delay = Math.min(1000 * 2 ** this.attempts, 30_000);
    this.attempts++;
    setTimeout(() => this.connect(), delay);
  }
}

function decodeBase64(value: string): Uint8Array<ArrayBuffer> {
  const binary = atob(value);
  return Uint8Array.from(binary, char => char.charCodeAt(0));
}

function jwtSubject(token: string): string | null {
  try {
    const payload = token.split('.')[1];
    if (!payload) return null;
    const normalized = payload.replace(/-/g, '+').replace(/_/g, '/').padEnd(Math.ceil(payload.length / 4) * 4, '=');
    const data = JSON.parse(atob(normalized)) as { sub?: unknown };
    return typeof data.sub === 'string' ? data.sub : null;
  } catch {
    return null;
  }
}

async function deriveWsSessionKey(saltB64: string): Promise<CryptoKey> {
  const token = sessionStorage.getItem('gw_token') ?? '';
  const identityId = sessionStorage.getItem('gw_identity_id') ?? jwtSubject(token);
  if (!identityId) throw new Error('WebSocket identity binding unavailable');
  const base = await globalThis.crypto.subtle.importKey(
    'raw',
    new TextEncoder().encode(identityId),
    'HKDF',
    false,
    ['deriveKey'],
  );
  return globalThis.crypto.subtle.deriveKey(
    {
      name: 'HKDF',
      hash: 'SHA-256',
      salt: decodeBase64(saltB64),
      info: new TextEncoder().encode('ghostwire-ws-session-v2'),
    },
    base,
    { name: 'AES-GCM', length: 256 },
    false,
    ['decrypt'],
  );
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
    if (p.DtnBundle)        return { type: 'dtn_bundle',         payload: p.DtnBundle as { cid: string; from: string } };
    if (p.ChannelMessage)   return { type: 'channel_message',    payload: p.ChannelMessage as { channel_id: string; text: string } };
  }
  if (r.type === 'message' && r.message) {
    return { type: 'message', payload: r.message as MessageV2 };
  }
  return null;
}

// Singleton socket instance
export const socket = new GhostWireSocket();
