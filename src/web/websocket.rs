//! WebSocket implementation for real-time updates

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path,
    },
    response::Response,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

/// WebSocket connection manager
pub struct WebSocketManager {
    // Map of project names to broadcast channels
    channels: Arc<RwLock<HashMap<String, broadcast::Sender<String>>>>,
}

impl WebSocketManager {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get or create a broadcast channel for a project
    async fn get_or_create_channel(&self, project_name: &str) -> broadcast::Sender<String> {
        let mut channels = self.channels.write().await;
        
        if let Some(sender) = channels.get(project_name) {
            sender.clone()
        } else {
            let (sender, _) = broadcast::channel(100);
            channels.insert(project_name.to_string(), sender.clone());
            sender
        }
    }

    /// Broadcast a message to all connections for a project
    pub async fn broadcast_to_project(&self, project_name: &str, message: &str) {
        let channels = self.channels.read().await;
        if let Some(sender) = channels.get(project_name) {
            let _ = sender.send(message.to_string());
        }
    }
}

// Global WebSocket manager instance
lazy_static::lazy_static! {
    static ref WS_MANAGER: WebSocketManager = WebSocketManager::new();
}

/// Handle WebSocket upgrade request
pub async fn handle_websocket(
    Path(project_name): Path<String>,
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_upgrade(|socket| handle_websocket_connection(socket, project_name))
}

/// Handle individual WebSocket connection
async fn handle_websocket_connection(socket: WebSocket, project_name: String) {
    let sender = WS_MANAGER.get_or_create_channel(&project_name).await;
    let mut receiver = sender.subscribe();

    let (mut ws_sender, mut ws_receiver) = socket.split();

    // Send welcome message
    let welcome = serde_json::json!({
        "type": "welcome",
        "project": project_name,
        "message": "Connected to Rask WebSocket"
    });
    
    if ws_sender.send(Message::Text(welcome.to_string())).await.is_err() {
        return;
    }

    // Handle incoming and outgoing messages
    let send_task = tokio::spawn(async move {
        while let Ok(msg) = receiver.recv().await {
            if ws_sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let recv_task = tokio::spawn(async move {
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    // Handle incoming messages from client
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&text) {
                        handle_websocket_message(parsed, &project_name).await;
                    }
                }
                Ok(Message::Close(_)) => break,
                _ => {}
            }
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }
}

/// Handle incoming WebSocket messages
async fn handle_websocket_message(message: serde_json::Value, _project_name: &str) {
    // TODO: Implement message handling
    // This could include:
    // - Task updates
    // - AI chat messages
    // - Real-time collaboration
    
    if let Some(msg_type) = message.get("type").and_then(|t| t.as_str()) {
        match msg_type {
            "ping" => {
                // Handle ping/pong for connection health
            }
            "task_update" => {
                // Handle task updates from client
            }
            "ai_chat" => {
                // Handle AI chat messages
            }
            _ => {
                // Unknown message type
            }
        }
    }
}

/// Broadcast project updates (to be called from CLI/state changes)
pub async fn broadcast_project_update(project_name: &str, update_type: &str, data: serde_json::Value) {
    let message = serde_json::json!({
        "type": update_type,
        "project": project_name,
        "data": data,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    WS_MANAGER.broadcast_to_project(project_name, &message.to_string()).await;
}