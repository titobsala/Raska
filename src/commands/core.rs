//! Core task management commands
//! 
//! This module contains the fundamental task operations like init, show,
//! complete, add, remove, edit, reset, list, and view.

use crate::{
    cli::CliPriority,
    model::{TaskStatus, Priority, Phase, Task}, 
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

/// Show the current project status with enhanced phase-based display options
pub fn show_project_enhanced(
    group_by_phase: bool,
    phase_filter: Option<&str>,
    detailed: bool,
    collapse_completed: bool,
) -> CommandResult {
    let roadmap = state::load_state()?;
    
    if group_by_phase {
        ui::display_roadmap_grouped_by_phase(&roadmap, detailed, collapse_completed);
    } else if let Some(phase) = phase_filter {
        ui::display_roadmap_filtered_by_phase(&roadmap, phase, detailed);
    } else {
        ui::display_roadmap_enhanced(&roadmap, detailed);
    }
    
    Ok(())
}

/// Show project timeline with phase-based horizontal layout
pub fn show_timeline(detailed: bool, active_only: bool, compact: bool, page: Option<usize>, page_size: Option<usize>) -> CommandResult {
    let roadmap = state::load_state()?;
    ui::display_project_timeline(&roadmap, detailed, active_only, compact, page, page_size);
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
    phase: &Option<String>,
    notes: &Option<String>,
    dependencies: &Option<String>,
    estimated_hours: &Option<f64>,
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

    if let Some(ref phase_str) = phase {
        let phase_model = Phase::from_string(phase_str);
        new_task = new_task.with_phase(phase_model);
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
    
    // Set estimated hours if provided
    if let Some(hours) = estimated_hours {
        if *hours <= 0.0 {
            return Err("Estimated hours must be greater than 0".into());
        }
        if *hours > 1000.0 {
            return Err("Estimated hours cannot exceed 1000 hours".into());
        }
        new_task.set_estimated_hours(*hours);
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
    phase: &Option<String>,
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

    // Apply phase filter
    if let Some(ref phase_str) = phase {
        let phase_model = Phase::from_string(phase_str);
        filtered_tasks.retain(|task| task.phase == phase_model);
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

/// Start time tracking for a task
pub fn start_time_tracking(task_id: usize, description: Option<&str>) -> CommandResult {
    let mut roadmap = state::load_state()?;
    
    // Check if any task already has an active time session
    for task in &roadmap.tasks {
        if task.has_active_time_session() {
            return Err(format!(
                "Task #{} already has an active time session. Stop it first with 'rask stop'", 
                task.id
            ).into());
        }
    }
    
    // Find the task to start tracking
    let task = roadmap.find_task_by_id_mut(task_id)
        .ok_or_else(|| format!("Task #{} not found", task_id))?;
    
    // Get task description before borrowing mutably
    let task_description = task.description.clone();
    
    // Start time tracking
    match task.start_time_session(description.map(|s| s.to_string())) {
        Ok(()) => {
            // Save the updated state
            state::save_state(&roadmap)?;
            
            ui::display_info(&format!("🕐 Started time tracking for task #{}: {}", task_id, task_description));
            if let Some(desc) = description {
                ui::display_info(&format!("📝 Session description: {}", desc));
            }
            ui::display_info("💡 Use 'rask stop' to end this session");
            Ok(())
        },
        Err(e) => Err(e.into()),
    }
}

/// Stop time tracking for the currently active task
pub fn stop_time_tracking() -> CommandResult {
    let mut roadmap = state::load_state()?;
    
    // Find the task with active time session
    let mut active_task_id = None;
    for task in &roadmap.tasks {
        if task.has_active_time_session() {
            active_task_id = Some(task.id);
            break;
        }
    }
    
    let task_id = active_task_id.ok_or("No active time tracking session found")?;
    
    // Stop time tracking
    let task = roadmap.find_task_by_id_mut(task_id).unwrap();
    let task_description = task.description.clone();
    
    match task.end_current_time_session() {
        Ok(duration_hours) => {
            let estimated_hours = task.estimated_hours;
            let total_tracked = task.get_total_tracked_hours();
            
            // Save the updated state
            state::save_state(&roadmap)?;
            
            ui::display_info(&format!("⏱️  Stopped time tracking for task #{}: {}", task_id, task_description));
            ui::display_info(&format!("⏰ Session duration: {:.2} hours", duration_hours));
            
            // Show updated totals
            if let Some(estimated) = estimated_hours {
                let variance = total_tracked - estimated;
                let percentage = (variance / estimated) * 100.0;
                
                ui::display_info(&format!("📊 Total tracked: {:.2}h | Estimated: {:.2}h | Variance: {:.2}h ({:+.1}%)", 
                    total_tracked, estimated, variance, percentage));
            } else {
                ui::display_info(&format!("📊 Total tracked time: {:.2} hours", total_tracked));
            }
            
            Ok(())
        },
        Err(e) => Err(e.into()),
    }
}

/// Show time tracking information
pub fn show_time_tracking(task_id: &Option<usize>, summary: bool, _detailed: bool) -> CommandResult {
    let roadmap = state::load_state()?;
    
    if let Some(id) = task_id {
        // Show time info for specific task
        let task = roadmap.find_task_by_id(*id)
            .ok_or_else(|| format!("Task #{} not found", id))?;
        
        // TODO: Implement proper time info display
        ui::display_info(&format!("⏰ Time tracking for task #{}: {}", id, task.description));
        
        if let Some(est) = task.estimated_hours {
            ui::display_info(&format!("📊 Estimated: {:.2} hours", est));
        }
        
        if let Some(actual) = task.actual_hours {
            ui::display_info(&format!("📊 Actual: {:.2} hours", actual));
        }
        
        if task.has_active_time_session() {
            ui::display_info("🕐 Active time session running");
        }
        
        ui::display_info(&format!("📈 Total sessions: {}", task.time_sessions.len()));
        
    } else if summary {
        // Show summary across all tasks
        let total_estimated: f64 = roadmap.tasks.iter().filter_map(|t| t.estimated_hours).sum();
        let total_actual: f64 = roadmap.tasks.iter().filter_map(|t| t.actual_hours).sum();
        let tasks_with_estimates = roadmap.tasks.iter().filter(|t| t.estimated_hours.is_some()).count();
        let tasks_with_time = roadmap.tasks.iter().filter(|t| t.actual_hours.is_some()).count();
        
        ui::display_info("📊 Time Tracking Summary");
        ui::display_info(&format!("Total estimated time: {:.2} hours ({} tasks)", total_estimated, tasks_with_estimates));
        ui::display_info(&format!("Total tracked time: {:.2} hours ({} tasks)", total_actual, tasks_with_time));
        
        if total_estimated > 0.0 {
            let variance = total_actual - total_estimated;
            let percentage = (variance / total_estimated) * 100.0;
            ui::display_info(&format!("Variance: {:.2} hours ({:+.1}%)", variance, percentage));
        }
    } else {
        // Show time info for all tasks with time data
        ui::display_info("📊 Time Tracking Overview");
        for task in &roadmap.tasks {
            if task.estimated_hours.is_some() || task.actual_hours.is_some() || !task.time_sessions.is_empty() {
                let est = task.estimated_hours.map_or("--".to_string(), |h| format!("{:.2}h", h));
                let actual = task.actual_hours.map_or("--".to_string(), |h| format!("{:.2}h", h));
                let status = if task.has_active_time_session() { "🕐" } else { "  " };
                
                ui::display_info(&format!("{} #{}: {} | Est: {} | Actual: {}", 
                    status, task.id, task.description, est, actual));
            }
        }
    }
    
    Ok(())
} 