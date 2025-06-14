//! Shared utilities for command operations
//! 
//! This module contains common validation functions and utilities
//! used across multiple command modules.

use crate::{model::{Roadmap}, state, markdown_writer};
use super::CommandResult;

/// Enhanced input validation for task descriptions
pub fn validate_task_description(description: &str) -> Result<(), String> {
    let trimmed = description.trim();
    
    if trimmed.is_empty() {
        return Err("Task description cannot be empty".to_string());
    }
    
    if trimmed.len() < 3 {
        return Err("Task description must be at least 3 characters long".to_string());
    }
    
    if trimmed.len() > 500 {
        return Err("Task description cannot exceed 500 characters".to_string());
    }
    
    // Check for suspicious patterns
    if trimmed.chars().all(|c| c.is_whitespace() || c == '.' || c == '-') {
        return Err("Task description must contain meaningful content".to_string());
    }
    
    Ok(())
}

/// Validate and parse tags from a comma-separated string
pub fn validate_and_parse_tags(tags_str: &str) -> Result<Vec<String>, String> {
    let tags: Vec<String> = tags_str.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    
    // Validate tag format
    for tag in &tags {
        if tag.len() > 50 {
            return Err(format!("Tag '{}' is too long (max 50 characters)", tag));
        }
        if !tag.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(format!("Tag '{}' contains invalid characters. Use only letters, numbers, hyphens, and underscores", tag));
        }
    }
    
    Ok(tags)
}

/// Validate and parse dependencies from a comma-separated string
pub fn validate_and_parse_dependencies(deps_str: &str, roadmap: &Roadmap) -> Result<Vec<usize>, String> {
    let deps: Vec<usize> = deps_str.split(',')
        .filter_map(|s| {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                None
            } else {
                match trimmed.parse() {
                    Ok(id) => Some(id),
                    Err(_) => {
                        crate::ui::display_warning(&format!("Invalid dependency ID '{}' - must be a number", trimmed));
                        None
                    }
                }
            }
        })
        .collect();
    
    // Validate dependencies exist
    for &dep_id in &deps {
        if roadmap.find_task_by_id(dep_id).is_none() {
            return Err(format!("Dependency task {} does not exist. Use 'rask list' to see available tasks.", dep_id));
        }
    }
    
    Ok(deps)
}

/// Parse comma-separated task IDs and validate they exist
pub fn parse_and_validate_task_ids(ids_str: &str, roadmap: &Roadmap) -> Result<Vec<usize>, String> {
    let task_ids: Result<Vec<usize>, _> = ids_str
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>())
        .collect();
    
    let task_ids = task_ids.map_err(|_| "Invalid task ID format. Use comma-separated numbers (e.g., 1,2,3)".to_string())?;
    
    if task_ids.is_empty() {
        return Err("No task IDs provided".to_string());
    }
    
    // Validate all task IDs exist
    let mut missing_ids = Vec::new();
    for &task_id in &task_ids {
        if roadmap.find_task_by_id(task_id).is_none() {
            missing_ids.push(task_id);
        }
    }
    
    if !missing_ids.is_empty() {
        return Err(format!("Tasks not found: {}", 
            missing_ids.iter()
                .map(|id| format!("#{}", id))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    
    Ok(task_ids)
}

/// Common pattern for saving state and syncing to markdown
pub fn save_and_sync(roadmap: &Roadmap) -> CommandResult {
    state::save_state(roadmap)?;
    markdown_writer::sync_to_source_file(roadmap)?;
    Ok(())
}

/// Escape HTML special characters for export functionality
pub fn html_escape(text: &str) -> String {
    text.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#x27;")
} 