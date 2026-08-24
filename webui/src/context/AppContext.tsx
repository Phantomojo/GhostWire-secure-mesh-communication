import React, { createContext, useContext, useEffect, useReducer, useRef } from 'react';
import {
  auth, profile, peers, messages, network, security,
  socket, GhostWireSocket,
  type IdentityStatus, type ProfileV2, type PeerV2,
  type MessageV2, type MeshHealthV2, type TransportStatus,
} from '../services/api';

// ── State shape ──────────────────────────────────────────────────────────────

export type ActivePanel =
  | 'dashboard' | 'network' | 'communication' | 'channels'
  | 'ark' | 'security' | 'system' | 'settings' | 'about';

export type TransportTier = 'stealth' | 'webrtc' | 'covert' | 'fronted' | 'offline';

export interface Toast {
  id: string;
  type: 'success' | 'error' | 'warn' | 'info';
  message: string;
}

interface AppState {
  // Auth / identity
  identityStatus: IdentityStatus | null;
  profile: ProfileV2 | null;
  isLocked: boolean;
  isInitializing: boolean;

  // Network
  peers: PeerV2[];
  peerCount: number;
  latencyMs: number;
  transport: TransportStatus;
  meshHealth: MeshHealthV2 | null;
  backendConnected: boolean;

  // Messages
  messages: MessageV2[];
  activeRecipient: string;

  // Security
  securityScore: number;
  pqcEnabled: boolean;
  threatLevel: string;
  isEmergencyMode: boolean;
  isStealthMode: boolean;
  isPanicMode: boolean;

  // UI
  activePanel: ActivePanel;
  toasts: Toast[];

  // System metrics
  cpuPct: number;
  memPct: number;
  uptimeSecs: number;
}

type Action =
  | { type: 'SET_INIT_DONE' }
  | { type: 'SET_IDENTITY_STATUS'; payload: IdentityStatus }
  | { type: 'SET_PROFILE'; payload: ProfileV2 }
  | { type: 'SET_LOCKED'; payload: boolean }
  | { type: 'SET_PEERS'; payload: PeerV2[] }
  | { type: 'SET_MESH_HEALTH'; payload: MeshHealthV2 }
  | { type: 'SET_TRANSPORT'; payload: TransportStatus }
  | { type: 'SET_BACKEND_CONNECTED'; payload: boolean }
  | { type: 'SET_MESSAGES'; payload: MessageV2[] }
  | { type: 'ADD_MESSAGE'; payload: MessageV2 }
  | { type: 'SET_ACTIVE_RECIPIENT'; payload: string }
  | { type: 'SET_SECURITY'; payload: { score: number; pqc: boolean; threat: string } }
  | { type: 'SET_SYSTEM'; payload: { cpu: number; mem: number; uptime: number; peers: number } }
  | { type: 'SET_EMERGENCY'; payload: boolean }
  | { type: 'SET_STEALTH'; payload: boolean }
  | { type: 'SET_PANIC'; payload: boolean }
  | { type: 'SET_PANEL'; payload: ActivePanel }
  | { type: 'TOAST_ADD'; payload: Toast }
  | { type: 'TOAST_REMOVE'; payload: string };

const initial: AppState = {
  identityStatus: null,
  profile: null,
  isLocked: true,
  isInitializing: true,
  peers: [],
  peerCount: 0,
  latencyMs: 0,
  transport: { tier: 'offline', display_name: 'offline', latency_ms: 0, internet_reachable: false },
  meshHealth: null,
  backendConnected: false,
  messages: [],
  activeRecipient: 'broadcast',
  securityScore: 0,
  pqcEnabled: false,
  threatLevel: 'none',
  isEmergencyMode: false,
  isStealthMode: false,
  isPanicMode: false,
  activePanel: 'dashboard',
  toasts: [],
  cpuPct: 0,
  memPct: 0,
  uptimeSecs: 0,
};

function reducer(state: AppState, action: Action): AppState {
  switch (action.type) {
    case 'SET_INIT_DONE':        return { ...state, isInitializing: false };
    case 'SET_IDENTITY_STATUS':  return { ...state, identityStatus: action.payload, isLocked: action.payload.locked };
    case 'SET_PROFILE':          return { ...state, profile: action.payload };
    case 'SET_LOCKED':           return { ...state, isLocked: action.payload };
    case 'SET_PEERS':            return { ...state, peers: action.payload, peerCount: action.payload.length };
    case 'SET_MESH_HEALTH':      return {
      ...state,
      meshHealth: action.payload,
      peerCount: action.payload.peers.length,
      latencyMs: action.payload.network_state.latency_ms,
      backendConnected: true,
    };
    case 'SET_TRANSPORT':        return { ...state, transport: action.payload };
    case 'SET_BACKEND_CONNECTED':return { ...state, backendConnected: action.payload };
    case 'SET_MESSAGES':         return { ...state, messages: action.payload };
    case 'ADD_MESSAGE':          return { ...state, messages: [...state.messages, action.payload] };
    case 'SET_ACTIVE_RECIPIENT': return { ...state, activeRecipient: action.payload };
    case 'SET_SECURITY':         return {
      ...state,
      securityScore: action.payload.score,
      pqcEnabled: action.payload.pqc,
      threatLevel: action.payload.threat,
    };
    case 'SET_SYSTEM':           return {
      ...state,
      cpuPct: action.payload.cpu,
      memPct: action.payload.mem,
      uptimeSecs: action.payload.uptime,
      peerCount: action.payload.peers,
    };
    case 'SET_EMERGENCY':        return { ...state, isEmergencyMode: action.payload };
    case 'SET_STEALTH':          return { ...state, isStealthMode: action.payload };
    case 'SET_PANIC':            return {
      ...state,
      isPanicMode: action.payload,
      isEmergencyMode: action.payload || state.isEmergencyMode,
      isStealthMode: action.payload || state.isStealthMode,
    };
    case 'SET_PANEL':            return { ...state, activePanel: action.payload };
    case 'TOAST_ADD':            return { ...state, toasts: [...state.toasts, action.payload] };
    case 'TOAST_REMOVE':         return { ...state, toasts: state.toasts.filter(t => t.id !== action.payload) };
    default:                     return state;
  }
}

// ── Context ───────────────────────────────────────────────────────────────────

interface AppCtx {
  state: AppState;
  dispatch: React.Dispatch<Action>;
  toast: (type: Toast['type'], message: string) => void;
  refreshIdentity: () => Promise<void>;
  refreshMesh: () => Promise<void>;
  sendMessage: (content: string, recipient: string) => Promise<void>;
}

const Ctx = createContext<AppCtx | undefined>(undefined);

export function AppProvider({ children }: { children: React.ReactNode }) {
  const [state, dispatch] = useReducer(reducer, initial);

  const toast = (type: Toast['type'], message: string) => {
    const id = Math.random().toString(36).slice(2);
    dispatch({ type: 'TOAST_ADD', payload: { id, type, message } });
    setTimeout(() => dispatch({ type: 'TOAST_REMOVE', payload: id }), 4000);
  };

  const refreshIdentity = async () => {
    try {
      const status = await auth.status();
      dispatch({ type: 'SET_IDENTITY_STATUS', payload: status });
      if (!status.locked && status.has_identity) {
        const p = await profile.get().catch(() => null);
        if (p) {
          sessionStorage.setItem('gw_identity_id', p.id);
          dispatch({ type: 'SET_PROFILE', payload: p });
        } else {
          sessionStorage.removeItem('gw_identity_id');
        }
      }
    } catch {
      dispatch({ type: 'SET_BACKEND_CONNECTED', payload: false });
    }
  };

  const refreshMesh = async () => {
    try {
      const [health, sec, sys] = await Promise.allSettled([
        network.meshHealth(),
        security.dashboard(),
        network.systemStatus(),
      ]);

      if (health.status === 'fulfilled') {
        dispatch({ type: 'SET_MESH_HEALTH', payload: health.value });
        dispatch({ type: 'SET_TRANSPORT', payload: {
          tier: mapTier(health.value.network_state.active_transport),
          display_name: health.value.network_state.active_transport,
          latency_ms: health.value.network_state.latency_ms,
          internet_reachable: health.value.network_state.internet_reachable,
        }});
        dispatch({ type: 'SET_PEERS', payload: health.value.peers.map(p => ({
          id: p.node_id, node_id: p.node_id,
          username: p.nickname ?? p.node_id.slice(0, 8),
          nickname: p.nickname,
          status: p.is_online ? 'online' : 'offline',
          last_seen: p.last_seen_secs_ago === 0 ? 'just now' : `${p.last_seen_secs_ago}s ago`, trust_level: p.trust_level,
          trust_score: p.signal_strength * 100,
          public_key: '', discovery_method: p.discovery_method,
          latency_ms: p.latency_ms,
        })) });
        dispatch({ type: 'SET_BACKEND_CONNECTED', payload: true });
      } else {
        dispatch({ type: 'SET_BACKEND_CONNECTED', payload: false });
      }

      if (sec.status === 'fulfilled') {
        dispatch({ type: 'SET_SECURITY', payload: {
          score: sec.value.score,
          pqc: sec.value.encryption.pqc_armor,
          threat: sec.value.threat_level,
        }});
      }

      if (sys.status === 'fulfilled') {
        const s = sys.value;
        dispatch({ type: 'SET_SYSTEM', payload: {
          cpu: Math.round(s.cpu_usage ?? 0),
          mem: s.memory_total_kb > 0
            ? Math.round((s.memory_used_kb / s.memory_total_kb) * 100) : 0,
          uptime: s.uptime_seconds,
          peers: s.peer_count,
        }});
      }
    } catch {}
  };

  const sendMessage = async (content: string, recipient: string) => {
    try {
      if (recipient === 'broadcast') {
        await messages.broadcast(content);
      } else {
        await messages.send(recipient, content);
      }
      toast('success', recipient === 'broadcast'
        ? 'Broadcast accepted by daemon; receipts pending'
        : 'Message accepted by daemon; receipt pending');
      // Load updated messages
      const latest = await messages.list(recipient === 'broadcast' ? 'all' : recipient)
        .catch(() => [] as MessageV2[]);
      dispatch({ type: 'SET_MESSAGES', payload: latest });
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : 'Transmission failed';
      toast('error', msg);
      throw e;
    }
  };

  // Boot sequence
  useEffect(() => {
    (async () => {
      await refreshIdentity();
      await refreshMesh();
      dispatch({ type: 'SET_INIT_DONE' });
    })();
  }, []);

  // WebSocket — real-time events, no polling
  useEffect(() => {
    if (state.isLocked || !sessionStorage.getItem('gw_token')) return;
    socket.connect();
    const off = socket.on((evt) => {
      switch (evt.type) {
        case 'message':
          dispatch({ type: 'ADD_MESSAGE', payload: evt.payload });
          break;
        case 'network_update':
          dispatch({ type: 'SET_SYSTEM', payload: {
            cpu: state.cpuPct, mem: state.memPct,
            uptime: state.uptimeSecs,
            peers: evt.payload.active_peers,
          }});
          break;
        case 'peer_connected':
        case 'peer_disconnected':
          refreshMesh();
          break;
        case 'security_event':
          toast(evt.payload.level === 'critical' ? 'error' : 'warn', evt.payload.message);
          break;
      }
    });

    // Reconnect mesh health every 60s (light, not every 5s)
    const interval = setInterval(refreshMesh, 60_000);

    return () => {
      off();
      clearInterval(interval);
      socket.disconnect();
    };
  }, [state.isLocked]);

  return (
    <Ctx.Provider value={{ state, dispatch, toast, refreshIdentity, refreshMesh, sendMessage }}>
      {children}
    </Ctx.Provider>
  );
}

export function useApp() {
  const ctx = useContext(Ctx);
  if (!ctx) throw new Error('useApp must be inside AppProvider');
  return ctx;
}

function mapTier(s: string): TransportTier {
  if (s.includes('stego')) return 'covert';
  if (s.includes('webrtc')) return 'webrtc';
  if (s.includes('front')) return 'fronted';
  if (s.includes('stealth') || s.includes('tcp')) return 'stealth';
  return 'offline';
}
