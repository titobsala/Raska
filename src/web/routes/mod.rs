//! Route definitions for the web API

pub mod api;
pub mod projects;
pub mod tasks;
pub mod ai;
pub mod static_files;

use axum::{Router, routing::get, response::Json};
use serde_json::{json, Value};

/// Create API routes
pub fn api_routes() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .nest("/projects", projects::routes())
        .nest("/tasks", tasks::routes())
        .nest("/ai", ai::routes())
}

/// Health check endpoint for the API
async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "rask-web",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// Create WebSocket routes
pub fn websocket_routes() -> Router {
    Router::new()
        .route("/projects/:name", axum::routing::get(super::websocket::handle_websocket))
}

/// Create static file routes
pub fn static_routes() -> Router {
    static_files::routes()
}