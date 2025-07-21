//! Route definitions for the web API

pub mod api;
pub mod projects;
pub mod tasks;
pub mod ai;
pub mod static_files;

use axum::Router;

/// Create API routes
pub fn api_routes() -> Router {
    Router::new()
        .nest("/projects", projects::routes())
        .nest("/tasks", tasks::routes())
        .nest("/ai", ai::routes())
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