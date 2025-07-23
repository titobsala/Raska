//! Project-related API routes

use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde_json::{json, Value};
use crate::state;
use std::collections::HashMap;

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
    // For now, we check if current directory has a .rask project
    if state::has_local_workspace() {
        match state::load_state() {
            Ok(roadmap) => {
                let project_info = json!({
                    "name": "current", // Use 'current' as canonical name for single-project mode
                    "display_name": roadmap.title,
                    "task_count": roadmap.tasks.len(),
                    "completed_tasks": roadmap.tasks.iter().filter(|t| matches!(t.status, crate::model::TaskStatus::Completed)).count(),
                    "phases": roadmap.get_all_phases().iter().map(|p| json!({
                        "name": p.name,
                        "description": p.description,
                        "emoji": p.emoji
                    })).collect::<Vec<_>>(),
                });
                Ok(Json(json!({
                    "projects": [project_info]
                })))
            }
            Err(e) => {
                eprintln!("Error loading project state: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    } else {
        Ok(Json(json!({
            "projects": [],
            "message": "No .rask project found in current directory"
        })))
    }
}

/// Get project details
async fn get_project(Path(_name): Path<String>) -> Result<Json<Value>, StatusCode> {
    // Load project from .rask/state.json
    match state::load_state() {
        Ok(roadmap) => {
            // Create the API response with phases included and normalized task statuses
            let mut response = serde_json::to_value(&roadmap).unwrap();
            
            // Normalize task statuses for frontend compatibility
            if let Value::Object(ref mut map) = response {
                if let Some(Value::Array(ref mut tasks)) = map.get_mut("tasks") {
                    for task in tasks.iter_mut() {
                        if let Value::Object(ref mut task_map) = task {
                            if let Some(Value::String(ref mut status)) = task_map.get_mut("status") {
                                *status = match status.as_str() {
                                    "Completed" => "completed".to_string(),
                                    "Pending" => "todo".to_string(),
                                    _ => status.to_lowercase(),
                                };
                            }
                        }
                    }
                }
            }
            
            // Extract unique phases from tasks
            let phases = roadmap.get_all_phases();
            let phases_json: Vec<Value> = phases.iter().map(|phase| {
                json!({
                    "name": phase.name,
                    "description": phase.description,
                    "emoji": phase.emoji,
                    "custom": false
                })
            }).collect();
            
            // Add phases to the response
            if let Value::Object(ref mut map) = response {
                map.insert("phases".to_string(), Value::Array(phases_json));
            }
            
            Ok(Json(response))
        }
        Err(e) => {
            eprintln!("Error loading project: {}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

/// Get project tasks
async fn get_project_tasks(Path(_name): Path<String>) -> Result<Json<Value>, StatusCode> {
    // Load and return project tasks with normalized statuses
    match state::load_state() {
        Ok(roadmap) => {
            let mut tasks_json = serde_json::to_value(&roadmap.tasks).unwrap();
            
            // Normalize task statuses for frontend compatibility
            if let Value::Array(ref mut tasks) = tasks_json {
                for task in tasks.iter_mut() {
                    if let Value::Object(ref mut task_map) = task {
                        if let Some(Value::String(ref mut status)) = task_map.get_mut("status") {
                            *status = match status.as_str() {
                                "Completed" => "completed".to_string(),
                                "Pending" => "todo".to_string(),
                                _ => status.to_lowercase(),
                            };
                        }
                    }
                }
            }
            
            Ok(Json(tasks_json))
        }
        Err(e) => {
            eprintln!("Error loading tasks: {}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

/// Get project dependencies
async fn get_project_dependencies(Path(_name): Path<String>) -> Result<Json<Value>, StatusCode> {
    // Generate dependency graph data
    match state::load_state() {
        Ok(roadmap) => {
            let mut nodes = Vec::new();
            let mut links = Vec::new();

            // Create nodes from tasks
            for task in &roadmap.tasks {
                nodes.push(json!({
                    "id": task.id,
                    "name": task.description,
                    "status": match task.status {
                        crate::model::TaskStatus::Pending => "Pending",
                        crate::model::TaskStatus::Completed => "Completed",
                    },
                    "priority": task.priority.to_string(),
                    "phase": task.phase.name
                }));
            }

            // Create links from dependencies
            for task in &roadmap.tasks {
                for dep_id in &task.dependencies {
                    links.push(json!({
                        "source": dep_id,
                        "target": task.id
                    }));
                }
            }

            Ok(Json(json!({
                "nodes": nodes,
                "links": links
            })))
        }
        Err(e) => {
            eprintln!("Error loading dependencies: {}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

/// Get project analytics
async fn get_project_analytics(Path(_name): Path<String>) -> Result<Json<Value>, StatusCode> {
    // Generate analytics data
    match state::load_state() {
        Ok(roadmap) => {
            let total_tasks = roadmap.tasks.len();
            let completed_tasks = roadmap.tasks.iter().filter(|t| matches!(t.status, crate::model::TaskStatus::Completed)).count();
            let pending_tasks = roadmap.tasks.iter().filter(|t| matches!(t.status, crate::model::TaskStatus::Pending)).count();
            let in_progress_tasks = 0; // Not supported in current TaskStatus enum

            // Phase breakdown
            let mut phase_stats = HashMap::new();
            for task in &roadmap.tasks {
                let phase_name = &task.phase.name;
                let entry = phase_stats.entry(phase_name.clone()).or_insert(json!({
                    "total": 0,
                    "completed": 0,
                    "in_progress": 0,
                    "pending": 0
                }));
                entry["total"] = (entry["total"].as_u64().unwrap() + 1).into();
                match &task.status {
                    crate::model::TaskStatus::Completed => entry["completed"] = (entry["completed"].as_u64().unwrap() + 1).into(),
                    crate::model::TaskStatus::Pending => entry["pending"] = (entry["pending"].as_u64().unwrap() + 1).into(),
                }
            }

            let completion_rate = if total_tasks > 0 {
                (completed_tasks as f64 / total_tasks as f64 * 100.0).round()
            } else {
                0.0
            };

            Ok(Json(json!({
                "total_tasks": total_tasks,
                "completed_tasks": completed_tasks,
                "in_progress_tasks": in_progress_tasks,
                "pending_tasks": pending_tasks,
                "completion_rate": completion_rate,
                "phase_breakdown": phase_stats
            })))
        }
        Err(e) => {
            eprintln!("Error generating analytics: {}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}