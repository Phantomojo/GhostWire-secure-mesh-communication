// API service for connecting to GhostWire backend

import type { User, Chat, Message, Contact, Group } from '../types';

// Dynamic API base URL - supports cross-PC communication
const getApiBaseUrl = () => {
  // Check for localStorage configuration (highest priority for manual overrides)
  const savedBackendUrl = localStorage.getItem('ghostwire_backend_url');
  if (savedBackendUrl) {
    if (savedBackendUrl.startsWith('http')) return savedBackendUrl;
    return `http://${savedBackendUrl}/api`;
  }

  // Check for environment variable
  if (import.meta.env.VITE_API_BASE_URL) {
    return import.meta.env.VITE_API_BASE_URL;
  }

  // Default to localhost, but use window.location.hostname for cross-PC
  const hostname = window.location.hostname;
  const port = '8080'; // Default backend port for GhostWire

  // If we're not on localhost, assume we're accessing from another PC
  if (hostname !== 'localhost' && hostname !== '127.0.0.1') {
    return `http://${hostname}:${port}/api`;
  }

  // Default local development
  return 'http://localhost:8080/api';
};

const API_BASE_URL = getApiBaseUrl();

// Auth token for X-Ghost-Token header
const getAuthToken = (): string | null => {
  // Check multiple possible keys used by different versions
  return localStorage.getItem('ghostwire_auth_token') ||
    localStorage.getItem('ghostwire_token') ||
    localStorage.getItem('token');
};

// Get auth headers for API requests
const getAuthHeaders = (): Record<string, string> => {
  const token = getAuthToken();
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
  };
  if (token) {
    headers['X-Ghost-Token'] = token;
  }
  return headers;
};

export interface Peer {
  id: string;
  name: string;
  status: 'online' | 'offline';
  lastSeen: string;
}

export interface Settings {
  stealth_mode: boolean;
}

export interface SendMessageRequest {
  recipient: string;
  message: string;
}

export interface DiscoveredPeer {
  ip: string;
  port: number;
  username: string;
  node_id: string;
  public_key: string;
  last_seen: string;
  status: string;
}

export interface NetworkScanResult {
  discovered_peers: DiscoveredPeer[];
  scan_time: string;
}

export interface NetworkInfo {
  local_ip: string;
  timestamp: string;
}

// --- API V2 Interfaces ---

export interface V2Response<T> {
  version: string;
  timestamp: string;
  request_id: string;
  data?: T;
  error?: V2Error;
  pagination?: PaginationMeta;
  links?: HateoasLinks;
  rate_limit?: RateLimitInfo;
}

export interface V2Error {
  code: string;
  message: string;
  details?: any;
  help_url?: string;
  trace_id?: string;
}

export interface PaginationMeta {
  total: number;
  page: number;
  per_page: number;
  pages: number;
}

export interface HateoasLinks {
  self: string;
  next?: string;
  prev?: string;
  related?: Record<string, string>;
}

export interface RateLimitInfo {
  limit: number;
  remaining: number;
  reset: string;
}

export interface ProfileV2 {
  id: string;
  did: string;
  username: string;
  nickname: string;
  bio: string;
  public_key: string;
  key_id: string;
  created_at: string;
  dht_published: boolean;
  security_score: number;
}

export interface SecurityDashboardV2 {
  score: number;
  encryption: {
    method: string;
    active: boolean;
    pqc_armor: boolean;
  };
  stats: {
    messages_encrypted: number;
    peers_verified: number;
    rate_limit_violations: number;
    blacklisted_peers: number;
  };
  threat_level: string;
}

export interface PeerV2 {
  id: string;
  username: string;
  status: string;
  last_seen: string;
  trust_score: number;
  public_key: string;
}

export interface MessageV2 {
  id: string; // Now a string thanks to our backend update
  sender: string;
  recipient: string;
  content: string;
  timestamp: number;
  encrypted: boolean;
  message_type: string;
  encryption_status: string;
}

// Configuration management
export const config = {
  setBackendUrl: (url: string) => {
    localStorage.setItem('ghostwire_backend_url', url);
    window.location.reload(); // Reload to use new URL
  },

  getBackendUrl: () => {
    return localStorage.getItem('ghostwire_backend_url') || 'http://localhost:3001/api';
  },

  resetBackendUrl: () => {
    localStorage.removeItem('ghostwire_backend_url');
    window.location.reload();
  },

  // Auth token management
  setAuthToken: (token: string) => {
    localStorage.setItem('ghostwire_auth_token', token);
    console.log('🔐 Auth token set. API requests will now include X-Ghost-Token header.');
  },

  getAuthToken: (): string | null => {
    return getAuthToken();
  },

  clearAuthToken: () => {
    localStorage.removeItem('ghostwire_auth_token');
    console.log('🔓 Auth token cleared.');
  },

  isAuthenticated: (): boolean => {
    return !!getAuthToken();
  },

  API_BASE_URL: API_BASE_URL
};

async function reportFrontendError(errorMsg: string) {
  try {
    const hostname = window.location.hostname;
    const userAgent = navigator.userAgent;
    const fullMsg = `Frontend error on ${hostname} [${userAgent}]: ${errorMsg}`;

    // Try to report to the current backend
    const backendUrl = getApiBaseUrl().replace('/api', '');
    await fetch(`${backendUrl}/api/report_error`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ error: fullMsg })
    });
  } catch {
    // Silently fail if error reporting fails
  }
}

// REST API functions
export const api = {
  // Generic HTTP methods
  async get(endpoint: string): Promise<any> {
    const url = `${getApiBaseUrl()}${endpoint}`;
    console.log(`API GET: ${url}`);

    let response = await fetch(url, {
      headers: getAuthHeaders(),
    });

    if (response.status === 401) {
      console.warn('🔒 Unauthorized, attempting auto-discovery...');
      if (await this.autoAuthenticate()) {
        response = await fetch(url, {
          headers: getAuthHeaders(),
        });
      }
    }

    if (!response.ok) {
      throw new Error(`GET ${endpoint} failed: ${response.statusText}`);
    }
    return response.json();
  },

  async post(endpoint: string, data?: any): Promise<any> {
    const url = `${getApiBaseUrl()}${endpoint}`;
    console.log(`API POST: ${url}`, data);

    let response = await fetch(url, {
      method: 'POST',
      headers: getAuthHeaders(),
      body: data ? JSON.stringify(data) : undefined,
    });

    if (response.status === 401) {
      console.warn('🔒 Unauthorized, attempting auto-discovery...');
      if (await this.autoAuthenticate()) {
        response = await fetch(url, {
          method: 'POST',
          headers: getAuthHeaders(),
          body: data ? JSON.stringify(data) : undefined,
        });
      }
    }

    if (!response.ok) {
      throw new Error(`POST ${endpoint} failed: ${response.statusText}`);
    }
    return response.json();
  },

  async put(endpoint: string, data?: any): Promise<any> {
    const url = `${getApiBaseUrl()}${endpoint}`;
    console.log(`API PUT: ${url}`, data);

    let response = await fetch(url, {
      method: 'PUT',
      headers: getAuthHeaders(),
      body: data ? JSON.stringify(data) : undefined,
    });

    if (response.status === 401) {
      console.warn('🔒 Unauthorized, attempting auto-discovery...');
      if (await this.autoAuthenticate()) {
        response = await fetch(url, {
          method: 'PUT',
          headers: getAuthHeaders(),
          body: data ? JSON.stringify(data) : undefined,
        });
      }
    }

    if (!response.ok) {
      throw new Error(`PUT ${endpoint} failed: ${response.statusText}`);
    }
    return response.json();
  },

  async delete(endpoint: string): Promise<any> {
    const url = `${getApiBaseUrl()}${endpoint}`;
    console.log(`API DELETE: ${url}`);

    const response = await fetch(url, {
      method: 'DELETE',
      headers: getAuthHeaders(),
    });
    if (!response.ok) {
      throw new Error(`DELETE ${endpoint} failed: ${response.statusText}`);
    }
    return response.json();
  },

  // --- API V2 Methods ---

  async v2Get<T>(endpoint: string): Promise<T> {
    const url = `${getApiBaseUrl()}/v2${endpoint}`;
    console.log(`🚀 API V2 GET: ${url}`);

    const response = await fetch(url, {
      headers: getAuthHeaders(),
    });

    const envelope: V2Response<T> = await response.json();

    if (!response.ok || envelope.error) {
      const error = envelope.error || { code: 'UNKNOWN', message: response.statusText };
      console.error(`❌ API V2 Error (${error.code}): ${error.message}`);
      throw new Error(error.message);
    }

    return envelope.data!;
  },

  async v2Post<T>(endpoint: string, data?: any): Promise<T> {
    const url = `${getApiBaseUrl()}/v2${endpoint}`;
    console.log(`🚀 API V2 POST: ${url}`, data);

    const response = await fetch(url, {
      method: 'POST',
      headers: getAuthHeaders(),
      body: data ? JSON.stringify(data) : undefined,
    });

    const envelope: V2Response<T> = await response.json();

    if (!response.ok || envelope.error) {
      const error = envelope.error || { code: 'UNKNOWN', message: response.statusText };
      console.error(`❌ API V2 Error (${error.code}): ${error.message}`);
      throw new Error(error.message);
    }

    return envelope.data!;
  },

  // Send a message
  async sendMessage(recipient: string, message: string): Promise<void> {
    try {
      const response = await fetch(`${getApiBaseUrl()}/send_message`, {
        method: 'POST',
        headers: getAuthHeaders(),
        body: JSON.stringify({ recipient, message }),
      });
      if (!response.ok) {
        await reportFrontendError(`sendMessage failed: ${response.statusText}`);
        throw new Error(`Failed to send message: ${response.statusText}`);
      }
      const data = await response.json();
      if (!data.success) {
        await reportFrontendError(`sendMessage failed: ${data.error}`);
        throw new Error(data.error || 'Failed to send message');
      }
    } catch (e: any) {
      await reportFrontendError(`sendMessage exception: ${e}`);
      throw e;
    }
  },

  // Get list of peers
  async getPeers(page = 1, perPage = 20): Promise<PeerV2[]> {
    try {
      return await this.v2Get<PeerV2[]>(`/peers?page=${page}&per_page=${perPage}`);
    } catch (e) {
      console.warn('Falling back to V1 peers...');
      const response = await fetch(`${getApiBaseUrl()}/peers`, {
        headers: getAuthHeaders(),
      });
      const data = await response.json();
      return (data.data?.peers || []).map((p: any) => ({
        id: p.id,
        username: p.name,
        status: p.status,
        last_seen: p.lastSeen,
        trust_score: 100,
        public_key: '',
      }));
    }
  },

  // Get Profile V2
  async getProfile(): Promise<ProfileV2> {
    return this.v2Get<ProfileV2>('/profile');
  },

  // Get Security Dashboard V2
  async getSecurityDashboard(): Promise<SecurityDashboardV2> {
    return this.v2Get<SecurityDashboardV2>('/security/dashboard');
  },

  // Get historical messages from the Sovereign Persistence layer
  async getHistoricalMessages(peerId: string = 'all'): Promise<any[]> {
    try {
      const messages = await this.v2Get<MessageV2[]>(`/messages?peer_id=${peerId}`);
      return messages.map(m => ({
        id: m.id,
        content: m.content,
        sender: m.sender,
        recipient: m.recipient,
        timestamp: new Date(m.timestamp * 1000),
        status: 'sent',
        encrypted: m.encrypted
      }));
    } catch (e) {
      console.warn('🚀 Persistence Fetch Failed, falling back to empty stream.', e);
      return [];
    }
  },

  // Get settings
  async getSettings(): Promise<Settings> {
    const response = await fetch(`${getApiBaseUrl()}/settings`, {
      headers: getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error(`Failed to get settings: ${response.statusText}`);
    }

    const data = await response.json();
    if (!data.success) {
      throw new Error(data.error || 'Failed to get settings');
    }

    return data.data || { stealth_mode: false };
  },

  // Update settings
  async updateSettings(settings: Partial<Settings>): Promise<Settings> {
    const response = await fetch(`${getApiBaseUrl()}/settings`, {
      method: 'PUT',
      headers: getAuthHeaders(),
      body: JSON.stringify(settings),
    });

    if (!response.ok) {
      throw new Error(`Failed to update settings: ${response.statusText}`);
    }

    const data = await response.json();
    if (!data.success) {
      throw new Error(data.error || 'Failed to update settings');
    }

    return data.data || settings;
  },

  // Register this peer with another node for discovery
  async registerWithPeer(peerAddress: string, peerId: string, peerName: string, publicKey: string): Promise<void> {
    // Get our own network info to provide the correct IP
    const networkInfo = await this.getNetworkInfo();
    const myAddress = `${networkInfo.local_ip}:3001`;

    const response = await fetch(`http://${peerAddress}/api/register_peer`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        peer_id: peerId,
        peer_name: peerName,
        public_key: publicKey,
        address: myAddress
      }),
    });

    if (!response.ok) {
      throw new Error(`Failed to register with peer: ${response.statusText}`);
    }

    const data = await response.json();
    if (!data.success) {
      throw new Error(data.error || 'Failed to register with peer');
    }
  },

  // Scan network for other GhostWire nodes
  async scanNetwork(): Promise<NetworkScanResult> {
    console.log('Scanning network for GhostWire nodes...');
    const response = await fetch(`${getApiBaseUrl()}/scan_network`, {
      headers: getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error(`Failed to scan network: ${response.statusText}`);
    }

    const data = await response.json();
    if (!data.success) {
      throw new Error(data.error || 'Failed to scan network');
    }

    console.log('Network scan completed:', data.data);
    return data.data;
  },

  // Set username
  async setUsername(username: string): Promise<string> {
    const response = await fetch(`${getApiBaseUrl()}/set_username`, {
      method: 'POST',
      headers: getAuthHeaders(),
      body: JSON.stringify({ username }),
    });

    if (!response.ok) {
      throw new Error(`Failed to set username: ${response.statusText}`);
    }

    const data = await response.json();
    if (!data.success) {
      throw new Error(data.error || 'Failed to set username');
    }

    return data.data;
  },

  // Get current username
  async getUsername(): Promise<string> {
    const response = await fetch(`${getApiBaseUrl()}/get_username`, {
      headers: getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error(`Failed to get username: ${response.statusText}`);
    }

    const data = await response.json();
    if (!data.success) {
      throw new Error(data.error || 'Failed to get username');
    }

    return data.data;
  },

  // Get network information (local IP)
  async getNetworkInfo(): Promise<NetworkInfo> {
    const response = await fetch(`${getApiBaseUrl()}/get_network_info`, {
      headers: getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error(`Failed to get network info: ${response.statusText}`);
    }

    const data = await response.json();
    if (!data.success) {
      throw new Error(data.error || 'Failed to get network info');
    }

    return data.data;
  },

  // --- Auto-Authentication ---
  async autoAuthenticate(): Promise<boolean> {
    const existingToken = getAuthToken();
    if (existingToken && existingToken.trim() !== '') return true;

    try {
      // Direct fetch to the discovery endpoint on the backend
      const response = await fetch(`${getApiBaseUrl()}/dev/token`);
      const data = await response.json();
      if (data.success && data.data) {
        console.log('✅ Auto-authenticated with local dev token');
        config.setAuthToken(data.data);
        return true;
      }
    } catch (e) {
      // Quietly fail discovery
    }
    return false;
  },

  // Test backend connectivity
  async testConnection(): Promise<boolean> {
    try {
      // Try to auto-authenticate first if needed
      if (!getAuthToken()) {
        await this.autoAuthenticate();
      }

      const response = await fetch(`${getApiBaseUrl()}/status`, {
        headers: getAuthHeaders(),
      });

      // If unauthorized, try discovery once more (case where token was expired/invalid)
      if (response.status === 401) {
        if (await this.autoAuthenticate()) {
          // Retry the status check
          const retryResponse = await fetch(`${getApiBaseUrl()}/status`, {
            headers: getAuthHeaders(),
          });
          return retryResponse.ok;
        }
      }

      return response.ok;
    } catch {
      return false;
    }
  }
};

// WebSocket connection for real-time updates
export class WebSocketService {
  private ws: WebSocket | null = null;
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;
  private reconnectDelay = 1000;
  private wsUrl: string | null = null;

  constructor(
    private onMessage: (message: Message) => void,
    private onPeerUpdate: (peers: Peer[]) => void,
    private onStatusUpdate: (status: any) => void,
    private onError: (error: string) => void
  ) { }

  async connect() {
    try {
      if (!this.wsUrl) {
        // Dynamically fetch backend IP and port
        const info = await api.getNetworkInfo();
        // Use window.location.hostname for frontend IP, but backend IP for LAN
        const wsHost = info.local_ip || window.location.hostname;
        // Try to get the port from the API base URL
        let port = 3001;
        try {
          const url = new URL(getApiBaseUrl());
          port = parseInt(url.port) || 3001;
        } catch { }
        this.wsUrl = `ws://${wsHost}:${port}/ws`;
      }

      console.log(`Connecting to WebSocket: ${this.wsUrl}`);
      this.ws = new WebSocket(this.wsUrl);

      this.ws.onopen = () => {
        console.log('WebSocket connected');
        this.reconnectAttempts = 0;
      };

      this.ws.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);
          this.handleMessage(data);
        } catch (error) {
          reportFrontendError(`WebSocket message parse error: ${error}`);
          console.error('Failed to parse WebSocket message:', error);
        }
      };

      this.ws.onclose = () => {
        console.log('WebSocket disconnected');
        this.attemptReconnect();
      };

      this.ws.onerror = (error) => {
        reportFrontendError(`WebSocket error: ${error}`);
        console.error('WebSocket error:', error);
        this.onError('WebSocket connection error');
      };
    } catch (error) {
      reportFrontendError(`WebSocket connect exception: ${error}`);
      console.error('Failed to create WebSocket:', error);
      this.onError('Failed to connect to server');
    }
  }

  private handleMessage(data: any) {
    switch (data.type) {
      case 'message':
        this.onMessage(data.message);
        break;
      case 'peers_update':
        this.onPeerUpdate(data.peers);
        break;
      case 'status_update':
        this.onStatusUpdate(data.status);
        break;
      default:
        console.log('Unknown message type:', data.type);
    }
  }

  private attemptReconnect() {
    if (this.reconnectAttempts < this.maxReconnectAttempts) {
      this.reconnectAttempts++;
      console.log(`Attempting to reconnect (${this.reconnectAttempts}/${this.maxReconnectAttempts})...`);

      setTimeout(() => {
        this.connect();
      }, this.reconnectDelay * this.reconnectAttempts);
    } else {
      this.onError('Failed to reconnect to server');
    }
  }

  disconnect() {
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
  }

  send(data: any) {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(data));
    } else {
      console.error('WebSocket is not connected');
    }
  }
}

export async function getUser(): Promise<User> {
  const res = await fetch(`${API_BASE_URL}/user`);
  if (!res.ok) throw new Error('Failed to fetch user');
  return res.json();
}

export async function getChats(): Promise<Chat[]> {
  const res = await fetch(`${API_BASE_URL}/chats`);
  if (!res.ok) throw new Error('Failed to fetch chats');
  return res.json();
}

export async function getMessages(chatId: string): Promise<Message[]> {
  const res = await fetch(`${API_BASE_URL}/chats/${chatId}/messages`);
  if (!res.ok) throw new Error('Failed to fetch messages');
  return res.json();
}

export async function getContacts(): Promise<Contact[]> {
  const res = await fetch(`${API_BASE_URL}/contacts`);
  if (!res.ok) throw new Error('Failed to fetch contacts');
  return res.json();
}

export async function getGroups(): Promise<Group[]> {
  const res = await fetch(`${API_BASE_URL}/groups`);
  if (!res.ok) throw new Error('Failed to fetch groups');
  return res.json();
}

export async function sendMessage(chatId: string, text: string): Promise<Message> {
  const res = await fetch(`${API_BASE_URL}/chats/${chatId}/messages`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ text }),
  });
  if (!res.ok) throw new Error('Failed to send message');
  return res.json();
} 
