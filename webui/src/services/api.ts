// API service for connecting to GhostWire backend

const API_BASE_URL = 'http://localhost:3000/api';
const WS_URL = 'ws://localhost:3000/ws';

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

// REST API functions
export const api = {
  // Send a message
  async sendMessage(recipient: string, message: string): Promise<void> {
    const response = await fetch(`${API_BASE_URL}/send_message`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ recipient, message }),
    });
    
    if (!response.ok) {
      throw new Error(`Failed to send message: ${response.statusText}`);
    }
  },

  // Get list of peers
  async getPeers(): Promise<Peer[]> {
    const response = await fetch(`${API_BASE_URL}/peers`);
    
    if (!response.ok) {
      throw new Error(`Failed to get peers: ${response.statusText}`);
    }
    
    const data = await response.json();
    return data.peers || [];
  },

  // Get settings
  async getSettings(): Promise<Settings> {
    const response = await fetch(`${API_BASE_URL}/settings`);
    
    if (!response.ok) {
      throw new Error(`Failed to get settings: ${response.statusText}`);
    }
    
    return response.json();
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
    
    return response.json();
  },
};

// WebSocket connection for real-time updates
export class WebSocketService {
  private ws: WebSocket | null = null;
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;
  private reconnectDelay = 1000;

  constructor(
    private onMessage: (message: Message) => void,
    private onPeerUpdate: (peers: Peer[]) => void,
    private onStatusUpdate: (status: any) => void,
    private onError: (error: string) => void
  ) {}

  connect() {
    try {
      this.ws = new WebSocket(WS_URL);
      
      this.ws.onopen = () => {
        console.log('WebSocket connected');
        this.reconnectAttempts = 0;
      };

      this.ws.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);
          this.handleMessage(data);
        } catch (error) {
          console.error('Failed to parse WebSocket message:', error);
        }
      };

      this.ws.onclose = () => {
        console.log('WebSocket disconnected');
        this.attemptReconnect();
      };

      this.ws.onerror = (error) => {
        console.error('WebSocket error:', error);
        this.onError('WebSocket connection error');
      };
    } catch (error) {
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