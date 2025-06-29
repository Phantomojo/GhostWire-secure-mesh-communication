// API service for connecting to GhostWire backend

const API_BASE_URL = 'http://localhost:3001/api';

export interface Message {
  id: string;
  sender: string;
  content: string;
  timestamp: string;
}

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

async function reportFrontendError(errorMsg: string) {
  try {
    const hostname = window.location.hostname;
    const userAgent = navigator.userAgent;
    const fullMsg = `Frontend error on ${hostname} [${userAgent}]: ${errorMsg}`;
    await fetch('http://192.168.100.242:3001/api/report_error', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ error: fullMsg })
    });
  } catch {}
}

// REST API functions
export const api = {
  // Send a message
  async sendMessage(recipient: string, message: string): Promise<void> {
    try {
      const response = await fetch(`${API_BASE_URL}/send_message`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
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
  async getPeers(): Promise<Peer[]> {
    const response = await fetch(`${API_BASE_URL}/peers`);
    
    if (!response.ok) {
      throw new Error(`Failed to get peers: ${response.statusText}`);
    }
    
    const data = await response.json();
    if (!data.success) {
      throw new Error(data.error || 'Failed to get peers');
    }
    
    return data.data?.peers || [];
  },

  // Get settings
  async getSettings(): Promise<Settings> {
    const response = await fetch(`${API_BASE_URL}/settings`);
    
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
    const response = await fetch(`${API_BASE_URL}/settings`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
      },
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
    const response = await fetch(`${API_BASE_URL}/scan_network`);
    
    if (!response.ok) {
      throw new Error(`Failed to scan network: ${response.statusText}`);
    }
    
    const data = await response.json();
    if (!data.success) {
      throw new Error(data.error || 'Failed to scan network');
    }
    
    return data.data;
  },

  // Set username
  async setUsername(username: string): Promise<string> {
    const response = await fetch(`${API_BASE_URL}/set_username`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
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
    const response = await fetch(`${API_BASE_URL}/get_username`);
    
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
    const response = await fetch(`${API_BASE_URL}/get_network_info`);
    
    if (!response.ok) {
      throw new Error(`Failed to get network info: ${response.statusText}`);
    }
    
    const data = await response.json();
    if (!data.success) {
      throw new Error(data.error || 'Failed to get network info');
    }
    
    return data.data;
  },
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
  ) {}

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
          const url = new URL((api as any).API_BASE_URL || 'http://localhost:3001/api');
          port = parseInt(url.port) || 3001;
        } catch {}
        this.wsUrl = `ws://${wsHost}:${port}/ws`;
      }
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
