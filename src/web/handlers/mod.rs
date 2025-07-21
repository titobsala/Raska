//! Request handlers for the web API

// This module will contain shared handler utilities and middleware
// as the API grows more complex

use axum::{
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};

/// Standard error response format
pub fn error_response(status: StatusCode, message: &str) -> (StatusCode, Json<Value>) {
    (
        status,
        Json(json!({
            "error": true,
            "message": message,
            "status": status.as_u16()
        }))
    )
}

/// Standard success response format
pub fn success_response(data: Value) -> Json<Value> {
    Json(json!({
        "success": true,
        "data": data
    }))
}