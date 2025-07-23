//! Axum web server implementation

use axum::{
    http::StatusCode,
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use anyhow::Result;

use crate::ui;
use super::{routes, watcher};

/// Main web server entry point
pub async fn run_server(port: u16, host: &str) -> Result<()> {
    // Start file watcher for current directory
    if crate::state::has_local_workspace() {
        match watcher::create_project_watcher(".").await {
            Ok(_watcher) => {
                ui::display_info("📁 File watcher started for project changes");
                // TODO: Integrate watcher with WebSocket broadcasts
            }
            Err(e) => {
                ui::display_warning(&format!("Could not start file watcher: {}", e));
            }
        }
    } else {
        ui::display_warning("No .rask directory found - file watching disabled");
    }

    let app = create_app().await?;
    
    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .map_err(|e| anyhow::anyhow!("Invalid address {}:{}: {}", host, port, e))?;

    ui::display_info(&format!("🌐 Starting Rask web server at http://{}", addr));
    ui::display_info("Press Ctrl+C to stop the server");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    ui::display_success("Web server stopped gracefully");
    Ok(())
}

/// Create the main Axum application with all routes
async fn create_app() -> Result<Router> {
    let app = Router::new()
        // API routes
        .nest("/api", routes::api_routes())
        // WebSocket routes
        .nest("/ws", routes::websocket_routes())
        // Static file serving (for React frontend)
        .nest("/", routes::static_routes())
        // Fallback for SPA routing
        .fallback(spa_fallback)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods(Any)
                        .allow_headers(Any),
                ),
        );

    Ok(app)
}


/// Fallback handler for SPA routing - serves index.html for unknown routes
async fn spa_fallback() -> Result<Html<String>, StatusCode> {
    // For now, return a simple HTML page
    // This will be replaced with the embedded React app later
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Rask Web Interface</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            margin: 0;
            padding: 2rem;
            background: #f5f5f5;
        }
        .container {
            max-width: 800px;
            margin: 0 auto;
            background: white;
            padding: 2rem;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }
        .header {
            text-align: center;
            margin-bottom: 2rem;
        }
        .status {
            background: #e8f5e8;
            border: 1px solid #4caf50;
            padding: 1rem;
            border-radius: 4px;
            margin: 1rem 0;
        }
        .api-info {
            background: #f0f8ff;
            border: 1px solid #2196f3;
            padding: 1rem;
            border-radius: 4px;
            margin: 1rem 0;
        }
        code {
            background: #f4f4f4;
            padding: 2px 6px;
            border-radius: 3px;
            font-family: 'Monaco', 'Consolas', monospace;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🚀 Rask Web Interface</h1>
            <p>Your intelligent project planning companion</p>
        </div>
        
        <div class="status">
            <h3>✅ Server Status</h3>
            <p>Rask web server is running successfully!</p>
            <p><strong>Version:</strong> {version}</p>
        </div>
        
        <div class="api-info">
            <h3>🔧 Development Mode</h3>
            <p>The React frontend is not yet built. You can:</p>
            <ul>
                <li>Test API endpoints at <code>/api/*</code></li>
                <li>Check server health at <code>/health</code></li>
                <li>View API documentation (coming soon)</li>
            </ul>
        </div>
        
        <div class="api-info">
            <h3>📚 API Endpoints</h3>
            <ul>
                <li><code>GET /health</code> - Server health check</li>
                <li><code>GET /api/projects</code> - List projects (coming soon)</li>
                <li><code>GET /api/projects/{name}/tasks</code> - Get project tasks (coming soon)</li>
                <li><code>WS /ws/projects/{name}</code> - WebSocket connection (coming soon)</li>
            </ul>
        </div>
    </div>
</body>
</html>
    "#.replace("{version}", env!("CARGO_PKG_VERSION"));

    Ok(Html(html))
}

/// Graceful shutdown signal handler
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    ui::display_info("Shutdown signal received, stopping server...");
}