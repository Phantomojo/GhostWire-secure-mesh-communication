import type { Message } from '../types';

const WS_URL = 'ws://localhost:3001/ws';

export function connectToMessageSocket(onMessage: (msg: Message) => void) {
  let ws: WebSocket | null = null;
  let reconnectTimeout: NodeJS.Timeout | null = null;

  function connect() {
    ws = new WebSocket(WS_URL);
    ws.onopen = () => {
      // Optionally authenticate or subscribe here
    };
    ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        if (data && data.type === 'message' && data.message) {
          onMessage(data.message as Message);
        }
      } catch (e) {
        // Ignore parse errors
      }
    };
    ws.onclose = () => {
      // Try to reconnect after 2 seconds
      reconnectTimeout = setTimeout(connect, 2000);
    };
    ws.onerror = () => {
      ws?.close();
    };
  }

  connect();

  return () => {
    if (ws) ws.close();
    if (reconnectTimeout) clearTimeout(reconnectTimeout);
  };
} 