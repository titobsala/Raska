// WebSocket client for real-time updates

import type { WebSocketMessage } from '@/types';

type WebSocketEventHandler = (message: WebSocketMessage) => void;

export class WebSocketClient {
  private ws: WebSocket | null = null;
  private handlers: Map<string, Set<WebSocketEventHandler>> = new Map();
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;
  private reconnectInterval = 1000; // Start with 1 second
  private isConnected = false;
  private projectName: string;

  constructor(projectName: string) {
    this.projectName = projectName;
  }

  connect(): Promise<void> {
    return new Promise((resolve, reject) => {
      try {
        const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        const wsUrl = `${protocol}//${window.location.host}/ws/projects/${encodeURIComponent(this.projectName)}`;
        
        this.ws = new WebSocket(wsUrl);

        this.ws.onopen = () => {
          console.log('WebSocket connected to project:', this.projectName);
          this.isConnected = true;
          this.reconnectAttempts = 0;
          this.reconnectInterval = 1000;
          resolve();
        };

        this.ws.onmessage = (event) => {
          try {
            const message: WebSocketMessage = JSON.parse(event.data);
            this.handleMessage(message);
          } catch (error) {
            console.error('Failed to parse WebSocket message:', error);
          }
        };

        this.ws.onclose = (event) => {
          console.log('WebSocket disconnected:', event.code, event.reason);
          this.isConnected = false;
          
          // Attempt to reconnect if it wasn't a normal closure
          if (event.code !== 1000 && this.reconnectAttempts < this.maxReconnectAttempts) {
            this.scheduleReconnect();
          }
        };

        this.ws.onerror = (error) => {
          console.error('WebSocket error:', error);
          reject(error);
        };

      } catch (error) {
        reject(error);
      }
    });
  }

  private scheduleReconnect(): void {
    if (this.reconnectAttempts >= this.maxReconnectAttempts) {
      console.error('Max reconnection attempts reached');
      return;
    }

    setTimeout(() => {
      console.log(`Attempting to reconnect (${this.reconnectAttempts + 1}/${this.maxReconnectAttempts})...`);
      this.reconnectAttempts++;
      this.reconnectInterval = Math.min(this.reconnectInterval * 2, 30000); // Max 30 seconds
      this.connect().catch(() => {
        // Reconnection failed, will try again
      });
    }, this.reconnectInterval);
  }

  private handleMessage(message: WebSocketMessage): void {
    // Handle welcome messages
    if (message.type === 'welcome') {
      console.log('WebSocket welcome:', message);
      return;
    }

    // Emit to specific type handlers
    const typeHandlers = this.handlers.get(message.type);
    if (typeHandlers) {
      typeHandlers.forEach(handler => handler(message));
    }

    // Emit to general message handlers
    const generalHandlers = this.handlers.get('*');
    if (generalHandlers) {
      generalHandlers.forEach(handler => handler(message));
    }
  }

  // Subscribe to specific message types
  on(type: string, handler: WebSocketEventHandler): () => void {
    if (!this.handlers.has(type)) {
      this.handlers.set(type, new Set());
    }
    
    this.handlers.get(type)!.add(handler);

    // Return unsubscribe function
    return () => {
      const handlers = this.handlers.get(type);
      if (handlers) {
        handlers.delete(handler);
        if (handlers.size === 0) {
          this.handlers.delete(type);
        }
      }
    };
  }

  // Send message to server
  send(message: WebSocketMessage): void {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(message));
    } else {
      console.warn('WebSocket is not connected. Message not sent:', message);
    }
  }

  // Send ping to keep connection alive
  ping(): void {
    this.send({ type: 'ping' });
  }

  // Check connection status
  isConnectedToServer(): boolean {
    return this.isConnected && this.ws?.readyState === WebSocket.OPEN;
  }

  // Close connection
  disconnect(): void {
    if (this.ws) {
      this.ws.close(1000, 'Client disconnect');
      this.ws = null;
      this.isConnected = false;
    }
  }
}

// Singleton WebSocket manager for the current project
let currentWsClient: WebSocketClient | null = null;

export function getWebSocketClient(projectName: string): WebSocketClient {
  // If we have a client for a different project, disconnect it
  if (currentWsClient && currentWsClient.isConnectedToServer()) {
    currentWsClient.disconnect();
  }

  // Create new client for the project
  currentWsClient = new WebSocketClient(projectName);
  return currentWsClient;
}

// Hook for React components to use WebSocket
export function useWebSocket(projectName: string) {
  const client = getWebSocketClient(projectName);

  return {
    client,
    isConnected: client.isConnectedToServer(),
    connect: () => client.connect(),
    disconnect: () => client.disconnect(),
    on: (type: string, handler: WebSocketEventHandler) => client.on(type, handler),
    send: (message: WebSocketMessage) => client.send(message),
  };
}