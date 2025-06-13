//! Core task management commands
//! 
//! This module contains the fundamental task operations like init, show,
//! complete, add, remove, edit, reset, list, and view.

use crate::{
    cli::CliPriority,
    markdown_writer, 
    model::{TaskStatus, Priority, Task, Roadmap}, 
    parser, 
    state, 
    ui
};
use super::{CommandResult, utils, dependencies};
use std::fs;
use std::path::PathBuf;

/// Initialize a new project from a Markdown file
pub fn init_project(filepath: &PathBuf) -> CommandResult {
    // Read and parse the markdown file
    let markdown_content = fs::read_to_string(filepath)?;
    let roadmap = parser::parse_markdown_to_roadmap(&markdown_content, Some(filepath))?;
    
    // Save the state
    state::save_state(&roadmap)?;
    
    // Display success message
    ui::display_init_success(&roadmap);
    
    Ok(())
}

/// Show the current project status with enhanced display
pub fn show_project() -> CommandResult {
    let roadmap = state::load_state()?;
    ui::display_roadmap_enhanced(&roadmap, true); // Show detailed view with tags, priorities, and notes
    Ok(())
}

/// Mark a task as completed
pub fn complete_task(task_id: usize) -> CommandResult {
    // Load current state
    let mut roadmap = state::load_state()?;
    
    // Validate dependencies first
    if let Err(errors) = roadmap.validate_task_dependencies(task_id) {
        for error in &errors {
            ui::display_error(&format!("Dependency validation failed: {}", error));
        }
        return Err("Cannot complete task due to dependency issues".into());
    }
    
    // Check dependencies before completing
    if let Some(task) = roadmap.find_task_by_id(task_id) {
        let completed_task_ids = roadmap.get_completed_task_ids();
        if !task.can_be_started(&completed_task_ids) {
            let incomplete_deps: Vec<usize> = task.dependencies.iter()
                .filter(|&&dep_id| !completed_task_ids.contains(&dep_id))
                .copied()
                .collect();
            
            // Show detailed dependency information
            ui::display_dependency_error(task_id, &incomplete_deps, &roadmap);
            return Err(format!(
                "Cannot complete task {}. Missing dependencies: {:?}", 
                task_id, incomplete_deps
            ).into());
        }
    }
    
    // Find tasks that will be unblocked (before completing this task)
    let newly_unblocked = dependencies::find_newly_unblocked_tasks(&roadmap, task_id);
    
    // Find and update the task
    let task = roadmap.tasks.iter_mut().find(|t| t.id == task_id);
    
    match task {
        Some(task) => {
            let task_description = task.description.clone();
            task.mark_completed();
            
            // Save to both JSON state and original markdown file
            utils::save_and_sync(&roadmap)?;
            
            // Display enhanced completion success with dependency unlocking
            ui::display_completion_success_enhanced(task_id, &task_description, &newly_unblocked, &roadmap);
            ui::display_roadmap(&roadmap);
            
            Ok(())
        }
        None => Err(format!("Task with ID {} not found.", task_id).into()),
    }
}

/// Add a new task with enhanced metadata support
pub fn add_task_enhanced(
    description: &str,
    tags: &Option<String>,
    priority: &Option<CliPriority>,
    notes: &Option<String>,
    dependencies: &Option<String>,
) -> CommandResult {
    // Enhanced input validation
    if let Err(validation_error) = utils::validate_task_description(description) {
        ui::display_error(&format!("Invalid task description: {}", validation_error));
        ui::display_info("💡 Try providing a more descriptive task name");
        return Err(validation_error.into());
    }
    
    // Load current state
    let mut roadmap = state::load_state()?;
    
    // Parse tags with validation
    let parsed_tags: Vec<String> = if let Some(tag_str) = tags {
        utils::validate_and_parse_tags(tag_str)?
    } else {
        Vec::new()
    };
    
    // Parse dependencies with enhanced validation
    let parsed_deps: Vec<usize> = if let Some(dep_str) = dependencies {
        utils::validate_and_parse_dependencies(dep_str, &roadmap)?
    } else {
        Vec::new()
    };
    
    // Create a temporary task to check for circular dependencies
    if !parsed_deps.is_empty() {
        let temp_task = Task::new(roadmap.get_next_task_id(), description.to_string())
            .with_dependencies(parsed_deps.clone());
        let mut temp_roadmap = roadmap.clone();
        temp_roadmap.tasks.push(temp_task);
        
        // Check for circular dependencies
        if let Err(errors) = temp_roadmap.validate_task_dependencies(temp_roadmap.get_next_task_id() - 1) {
            for error in &errors {
                ui::display_error(&format!("Dependency validation failed: {}", error));
            }
            return Err("Cannot add task due to dependency conflicts".into());
        }
    }
    
    // Create new task with enhanced features
    let mut new_task = Task::new(roadmap.get_next_task_id(), description.to_string());
    
    if !parsed_tags.is_empty() {
        new_task = new_task.with_tags(parsed_tags);
    }
    
    if let Some(ref priority_cli) = priority {
        let priority_model: Priority = priority_cli.clone().into();
        new_task = new_task.with_priority(priority_model);
    }
    
    if let Some(ref note_text) = notes {
        if note_text.trim().is_empty() {
            ui::display_warning("Empty note provided - skipping");
        } else if note_text.len() > 1000 {
            return Err("Note cannot exceed 1000 characters".into());
        } else {
            new_task = new_task.with_notes(note_text.clone());
        }
    }
    
    if !parsed_deps.is_empty() {
        new_task = new_task.with_dependencies(parsed_deps);
    }
    
    // Add task to roadmap
    roadmap.add_task(new_task.clone());
    
    // Save to both JSON state and original markdown file
    utils::save_and_sync(&roadmap)?;
    
    // Display success and updated roadmap
    ui::display_add_success_enhanced(&new_task);
    ui::display_roadmap(&roadmap);
    
    Ok(())
}

/// Remove a task from the project
pub fn remove_task(task_id: usize) -> CommandResult {
    // Load current state
    let mut roadmap = state::load_state()?;
    
    // Check if any other tasks depend on this one
    let dependents: Vec<usize> = roadmap.tasks.iter()
        .filter(|t| t.dependencies.contains(&task_id))
        .map(|t| t.id)
        .collect();
    
    if !dependents.is_empty() {
        return Err(format!(
            "Cannot remove task {}. Other tasks depend on it: {:?}", 
            task_id, dependents
        ).into());
    }
    
    // Remove the task
    if let Some(removed_task) = roadmap.remove_task(task_id) {
        // Save to both JSON state and original markdown file
        utils::save_and_sync(&roadmap)?;
        
        // Display success and updated roadmap
        ui::display_remove_success(&removed_task.description);
        ui::display_roadmap(&roadmap);
        
        Ok(())
    } else {
        Err(format!("Task with ID {} not found.", task_id).into())
    }
}

/// Edit the description of an existing task
pub fn edit_task(task_id: usize, new_description: &str) -> CommandResult {
    // Load current state
    let mut roadmap = state::load_state()?;
    
    // Find and update the task
    let task = roadmap.tasks.iter_mut().find(|t| t.id == task_id);
    
    match task {
        Some(task) => {
            let old_description = task.description.clone();
            task.description = new_description.to_string();
            
            // Save to both JSON state and original markdown file
            utils::save_and_sync(&roadmap)?;
            
            // Display success and updated roadmap
            ui::display_edit_success(task_id, &old_description, new_description);
            ui::display_roadmap(&roadmap);
            
            Ok(())
        }
        None => Err(format!("Task with ID {} not found.", task_id).into()),
    }
}

/// Reset task(s) to pending status
pub fn reset_tasks(task_id: Option<usize>) -> CommandResult {
    // Load current state
    let mut roadmap = state::load_state()?;
    
    match task_id {
        Some(id) => {
            // Reset specific task
            let task = roadmap.tasks.iter_mut().find(|t| t.id == id);
            
            match task {
                Some(task) => {
                    if task.status == TaskStatus::Completed {
                        task.mark_pending();
                        
                        // Save to both JSON state and original markdown file
                        utils::save_and_sync(&roadmap)?;
                        
                        // Display success and updated roadmap
                        ui::display_reset_success(Some(id));
                        ui::display_roadmap(&roadmap);
                    } else {
                        ui::display_info(&format!("Task {} is already pending.", id));
                    }
                    
                    Ok(())
                }
                None => Err(format!("Task with ID {} not found.", id).into()),
            }
        }
        None => {
            // Reset all tasks
            let completed_count = roadmap.tasks.iter()
                .filter(|t| t.status == TaskStatus::Completed)
                .count();
            
            if completed_count > 0 {
                for task in &mut roadmap.tasks {
                    task.mark_pending();
                }
                
                // Save to both JSON state and original markdown file
                utils::save_and_sync(&roadmap)?;
                
                // Display success and updated roadmap
                ui::display_reset_success(None);
                ui::display_roadmap(&roadmap);
            } else {
                ui::display_info("All tasks are already pending.");
            }
            
            Ok(())
        }
    }
}

/// List and filter tasks with advanced options
pub fn list_tasks(
    tags: &Option<String>,
    priority: &Option<CliPriority>,
    status: &Option<String>,
    search: &Option<String>,
    detailed: bool,
) -> CommandResult {
    let roadmap = state::load_state()?;
    
    // Start with all tasks
    let mut filtered_tasks: Vec<&Task> = roadmap.tasks.iter().collect();
    
    // Apply tag filter
    if let Some(tag_str) = tags {
        let filter_tags: Vec<String> = tag_str.split(',').map(|s| s.trim().to_string()).collect();
        filtered_tasks.retain(|task| {
            filter_tags.iter().any(|tag| task.has_tag(tag))
        });
    }
    
    // Apply priority filter
    if let Some(ref priority_cli) = priority {
        let priority_model: Priority = priority_cli.clone().into();
        filtered_tasks.retain(|task| task.priority == priority_model);
    }
    
    // Apply status filter
    if let Some(ref status_str) = status {
        match status_str.to_lowercase().as_str() {
            "pending" => filtered_tasks.retain(|task| task.status == TaskStatus::Pending),
            "completed" => filtered_tasks.retain(|task| task.status == TaskStatus::Completed),
            "all" => {}, // Keep all tasks
            _ => return Err(format!("Invalid status filter: {}. Use 'pending', 'completed', or 'all'.", status_str).into()),
        }
    }
    
    // Apply search filter
    if let Some(ref query) = search {
        let search_results = roadmap.search_tasks(query);
        let search_ids: std::collections::HashSet<usize> = search_results.iter().map(|t| t.id).collect();
        filtered_tasks.retain(|task| search_ids.contains(&task.id));
    }
    
    // Display filtered results
    ui::display_filtered_tasks(&roadmap, &filtered_tasks, detailed);
    
    Ok(())
}

/// View detailed information about a specific task
pub fn view_task(task_id: usize) -> CommandResult {
    let roadmap = state::load_state()?;
    
    // Find the task
    let task = roadmap.find_task_by_id(task_id)
        .ok_or_else(|| format!("Task #{} not found", task_id))?;
    
    // Display detailed task information
    ui::display_detailed_task_view(task, &roadmap);
    
    Ok(())
} 