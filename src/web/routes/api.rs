//! API route utilities and middleware

use axum::{
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde_json::{json, Value};

/// API utility routes
pub fn routes() -> Router {
    Router::new()
        .route("/version", get(get_version))
        .route("/status", get(get_status))
}

/// Get API version information
async fn get_version() -> Json<Value> {
    Json(json!({
        "version": env!("CARGO_PKG_VERSION"),
        "name": env!("CARGO_PKG_NAME"),
        "description": env!("CARGO_PKG_DESCRIPTION")
    }))
}

/// Get API status
async fn get_status() -> Json<Value> {
    Json(json!({
        "status": "operational",
        "api_version": "v1",
        "features": {
            "projects": false,
            "tasks": false,
            "ai": false,
            "websockets": false
        },
        "message": "API endpoints are being implemented"
    }))
}