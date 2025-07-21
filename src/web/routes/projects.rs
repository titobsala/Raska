//! Project-related API routes

use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde_json::{json, Value};

/// Project routes
pub fn routes() -> Router {
    Router::new()
        .route("/", get(list_projects))
        .route("/:name", get(get_project))
        .route("/:name/tasks", get(get_project_tasks))
        .route("/:name/dependencies", get(get_project_dependencies))
        .route("/:name/analytics", get(get_project_analytics))
}

/// List all available projects
async fn list_projects() -> Result<Json<Value>, StatusCode> {
    // TODO: Implement project discovery
    // For now, return a placeholder response
    Ok(Json(json!({
        "projects": [],
        "message": "Project discovery not yet implemented"
    })))
}

/// Get project details
async fn get_project(Path(name): Path<String>) -> Result<Json<Value>, StatusCode> {
    // TODO: Load project from .rask/state.json
    Ok(Json(json!({
        "name": name,
        "message": "Project loading not yet implemented"
    })))
}

/// Get project tasks
async fn get_project_tasks(Path(name): Path<String>) -> Result<Json<Value>, StatusCode> {
    // TODO: Load and return project tasks
    Ok(Json(json!({
        "project": name,
        "tasks": [],
        "message": "Task loading not yet implemented"
    })))
}

/// Get project dependencies
async fn get_project_dependencies(Path(name): Path<String>) -> Result<Json<Value>, StatusCode> {
    // TODO: Generate dependency graph data
    Ok(Json(json!({
        "project": name,
        "dependencies": [],
        "message": "Dependency analysis not yet implemented"
    })))
}

/// Get project analytics
async fn get_project_analytics(Path(name): Path<String>) -> Result<Json<Value>, StatusCode> {
    // TODO: Generate analytics data
    Ok(Json(json!({
        "project": name,
        "analytics": {},
        "message": "Analytics not yet implemented"
    })))
}