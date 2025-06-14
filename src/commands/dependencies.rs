//! Dependency analysis and visualization commands
//! 
//! This module handles all dependency-related operations including
//! tree visualization, validation, and finding ready/blocked tasks.

use crate::{model::{Roadmap, TaskStatus}, state, ui};
use super::CommandResult;

/// Find tasks that become unblocked after completing a specific task
pub fn find_newly_unblocked_tasks(roadmap: &Roadmap, completed_task_id: usize) -> Vec<usize> {
    let mut completed_task_ids = roadmap.get_completed_task_ids();
    // Add the task we're about to complete to the list
    completed_task_ids.insert(completed_task_id);
    
    roadmap.tasks.iter()
        .filter(|task| {
            // Task must be pending
            task.status == TaskStatus::Pending && 
            // Task must depend on the completed task
            task.dependencies.contains(&completed_task_id) &&
            // All of task's dependencies must now be complete (including the one we just completed)
            task.dependencies.iter().all(|&dep_id| completed_task_ids.contains(&dep_id))
        })
        .map(|task| task.id)
        .collect()
}

/// Analyze and visualize task dependencies
pub fn analyze_dependencies(
    tree_task_id: &Option<usize>,
    validate: bool,
    show_ready: bool,
    show_blocked: bool,
) -> CommandResult {
    let roadmap = state::load_state()?;
    
    // If no specific options provided, show a summary
    if tree_task_id.is_none() && !validate && !show_ready && !show_blocked {
        ui::display_dependency_overview(&roadmap);
        return Ok(());
    }
    
    // Validate dependencies if requested
    if validate {
        match roadmap.validate_all_dependencies() {
            Ok(()) => {
                ui::display_success("All dependencies are valid!");
            }
            Err(errors) => {
                ui::display_dependency_validation_errors(&errors);
                return Err("Dependency validation failed".into());
            }
        }
    }
    
    // Show dependency tree for specific task
    if let Some(task_id) = tree_task_id {
        if let Some(tree) = roadmap.get_dependency_tree(*task_id) {
            ui::display_dependency_tree(&tree, &roadmap);
        } else {
            return Err(format!("Task {} not found", task_id).into());
        }
    }
    
    // Show ready tasks
    if show_ready {
        let ready_tasks = roadmap.get_ready_tasks();
        ui::display_ready_tasks(&ready_tasks);
    }
    
    // Show blocked tasks
    if show_blocked {
        let blocked_tasks = roadmap.get_blocked_tasks();
        ui::display_blocked_tasks(&blocked_tasks, &roadmap);
    }
    
    Ok(())
} 