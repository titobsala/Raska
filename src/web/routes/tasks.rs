//! Task-related API routes

use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde_json::{json, Value};

/// Task routes
pub fn routes() -> Router {
    Router::new()
        .route("/:id", get(get_task))
        .route("/:id", put(update_task))
        .route("/:id", delete(delete_task))
}

/// Get task details
async fn get_task(Path(id): Path<u32>) -> Result<Json<Value>, StatusCode> {
    // TODO: Load task from state
    Ok(Json(json!({
        "id": id,
        "message": "Task loading not yet implemented"
    })))
}

/// Update task
async fn update_task(Path(id): Path<u32>) -> Result<Json<Value>, StatusCode> {
    // TODO: Update task in state
    Ok(Json(json!({
        "id": id,
        "message": "Task updating not yet implemented"
    })))
}

/// Delete task
async fn delete_task(Path(id): Path<u32>) -> Result<Json<Value>, StatusCode> {
    // TODO: Delete task from state
    Ok(Json(json!({
        "id": id,
        "message": "Task deletion not yet implemented"
    })))
}