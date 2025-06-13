//! Bulk operations commands
//! 
//! This module handles batch operations on multiple tasks including
//! completion, tag management, priority setting, and removal.

use crate::{
    cli::{CliPriority, CliPhase},
    model::{TaskStatus, Priority, Phase},
    ui
};
use super::{CommandResult, utils, dependencies, BulkCommands};

/// Handle bulk operations on multiple tasks
pub fn handle_bulk_command(bulk_command: &BulkCommands) -> CommandResult {
    match bulk_command {
        BulkCommands::Complete { ids } => bulk_complete_tasks(ids),
        BulkCommands::AddTags { ids, tags } => bulk_add_tags(ids, tags),
        BulkCommands::RemoveTags { ids, tags } => bulk_remove_tags(ids, tags),
        BulkCommands::SetPriority { ids, priority } => bulk_set_priority(ids, priority),
        BulkCommands::SetPhase { ids, phase } => bulk_set_phase(ids, phase),
        BulkCommands::Reset { ids } => bulk_reset_tasks(ids),
        BulkCommands::Remove { ids, force } => bulk_remove_tasks(ids, *force),
    }
}

/// Complete multiple tasks at once
pub fn bulk_complete_tasks(ids_str: &str) -> CommandResult {
    let mut roadmap = crate::state::load_state()?;
    let task_ids = utils::parse_and_validate_task_ids(ids_str, &roadmap)?;
    
    ui::display_info(&format!("🚀 Attempting to complete {} tasks...", task_ids.len()));
    
    let mut completed_count = 0;
    let mut failed_tasks = Vec::new();
    let mut newly_unblocked = Vec::new();
    
    for &task_id in &task_ids {
        // Check if task is already completed
        if let Some(task) = roadmap.find_task_by_id(task_id) {
            if task.status == TaskStatus::Completed {
                ui::display_warning(&format!("Task #{} is already completed", task_id));
                continue;
            }
        }
        
        // Validate dependencies
        {
            if let Err(errors) = roadmap.validate_task_dependencies(task_id) {
                failed_tasks.push((task_id, format!("Dependency validation failed: {}", 
                    errors.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", "))));
                continue;
            }
            
            // Check if task can be started
            if let Some(task) = roadmap.find_task_by_id(task_id) {
                let completed_ids = roadmap.get_completed_task_ids();
                if !task.can_be_started(&completed_ids) {
                    let incomplete_deps: Vec<usize> = task.dependencies.iter()
                        .filter(|&&dep_id| !completed_ids.contains(&dep_id))
                        .copied()
                        .collect();
                    failed_tasks.push((task_id, format!("Blocked by dependencies: {}", 
                        incomplete_deps.iter()
                            .map(|id| format!("#{}", id))
                            .collect::<Vec<_>>()
                            .join(", "))));
                    continue;
                }
            }
        }
        
        // Find newly unblocked tasks before completing this one
        let unblocked = dependencies::find_newly_unblocked_tasks(&roadmap, task_id);
        newly_unblocked.extend(unblocked);
        
        // Complete the task
        if let Some(task) = roadmap.tasks.iter_mut().find(|t| t.id == task_id) {
            task.mark_completed();
            completed_count += 1;
            ui::display_success(&format!("✅ Completed task #{}: {}", task_id, task.description));
        }
    }
    
    // Save state if any tasks were completed
    if completed_count > 0 {
        utils::save_and_sync(&roadmap)?;
        
        ui::display_success(&format!("🎉 Successfully completed {} out of {} tasks!", 
            completed_count, task_ids.len()));
        
        // Show newly unblocked tasks
        if !newly_unblocked.is_empty() {
            newly_unblocked.sort();
            newly_unblocked.dedup();
            ui::display_info(&format!("🔓 Unlocked tasks: {}", 
                newly_unblocked.iter()
                    .map(|id| format!("#{}", id))
                    .collect::<Vec<_>>()
                    .join(", ")));
        }
    }
    
    // Report failed tasks
    if !failed_tasks.is_empty() {
        ui::display_warning(&format!("⚠️  Failed to complete {} tasks:", failed_tasks.len()));
        for (task_id, reason) in failed_tasks {
            ui::display_error(&format!("  #{}: {}", task_id, reason));
        }
        ui::display_info("💡 Dependencies must be completed before tasks can be marked as done");
    }
    
    Ok(())
}

/// Add tags to multiple tasks
pub fn bulk_add_tags(ids_str: &str, tags_str: &str) -> CommandResult {
    let mut roadmap = crate::state::load_state()?;
    let task_ids = utils::parse_and_validate_task_ids(ids_str, &roadmap)?;
    
    // Parse and validate tags
    let tags = utils::validate_and_parse_tags(tags_str)?;
    
    if tags.is_empty() {
        return Err("No tags provided".into());
    }
    
    ui::display_info(&format!("🏷️  Adding tags {} to {} tasks...", 
        tags.iter().map(|t| format!("#{}", t)).collect::<Vec<_>>().join(" "),
        task_ids.len()));
    
    let mut modified_count = 0;
    
    for &task_id in &task_ids {
        if let Some(task) = roadmap.tasks.iter_mut().find(|t| t.id == task_id) {
            let mut added_tags = Vec::new();
            
            for tag in &tags {
                if !task.tags.contains(tag) {
                    task.tags.insert(tag.clone());
                    added_tags.push(tag);
                }
            }
            
            if !added_tags.is_empty() {
                modified_count += 1;
                ui::display_success(&format!("✅ Added tags {} to task #{}: {}", 
                    added_tags.iter().map(|t| format!("#{}", t)).collect::<Vec<_>>().join(" "),
                    task_id, task.description));
            } else {
                ui::display_info(&format!("ℹ️  Task #{} already has all specified tags", task_id));
            }
        }
    }
    
    if modified_count > 0 {
        utils::save_and_sync(&roadmap)?;
        ui::display_success(&format!("🎉 Successfully modified {} tasks!", modified_count));
    }
    
    Ok(())
}

/// Remove tags from multiple tasks
pub fn bulk_remove_tags(ids_str: &str, tags_str: &str) -> CommandResult {
    let mut roadmap = crate::state::load_state()?;
    let task_ids = utils::parse_and_validate_task_ids(ids_str, &roadmap)?;
    
    let tags: Vec<String> = tags_str.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    
    if tags.is_empty() {
        return Err("No tags provided".into());
    }
    
    ui::display_info(&format!("🗑️  Removing tags {} from {} tasks...", 
        tags.iter().map(|t| format!("#{}", t)).collect::<Vec<_>>().join(" "),
        task_ids.len()));
    
    let mut modified_count = 0;
    
    for &task_id in &task_ids {
        if let Some(task) = roadmap.tasks.iter_mut().find(|t| t.id == task_id) {
            let mut removed_tags = Vec::new();
            
            for tag in &tags {
                if task.tags.remove(tag) {
                    removed_tags.push(tag);
                }
            }
            
            if !removed_tags.is_empty() {
                modified_count += 1;
                ui::display_success(&format!("✅ Removed tags {} from task #{}: {}", 
                    removed_tags.iter().map(|t| format!("#{}", t)).collect::<Vec<_>>().join(" "),
                    task_id, task.description));
            } else {
                ui::display_info(&format!("ℹ️  Task #{} doesn't have any of the specified tags", task_id));
            }
        }
    }
    
    if modified_count > 0 {
        utils::save_and_sync(&roadmap)?;
        ui::display_success(&format!("🎉 Successfully modified {} tasks!", modified_count));
    }
    
    Ok(())
}

/// Set priority for multiple tasks
pub fn bulk_set_priority(ids_str: &str, priority: &CliPriority) -> CommandResult {
    let mut roadmap = crate::state::load_state()?;
    let task_ids = utils::parse_and_validate_task_ids(ids_str, &roadmap)?;
    let new_priority: Priority = priority.clone().into();
    
    ui::display_info(&format!("⚡ Setting priority to {} for {} tasks...", 
        new_priority, task_ids.len()));
    
    let mut modified_count = 0;
    
    for &task_id in &task_ids {
        if let Some(task) = roadmap.tasks.iter_mut().find(|t| t.id == task_id) {
            if task.priority != new_priority {
                let old_priority = task.priority.clone();
                task.priority = new_priority.clone();
                modified_count += 1;
                ui::display_success(&format!("✅ Changed priority of task #{} from {} to {}: {}", 
                    task_id, old_priority, new_priority, task.description));
            } else {
                ui::display_info(&format!("ℹ️  Task #{} already has {} priority", task_id, new_priority));
            }
        }
    }
    
    if modified_count > 0 {
        utils::save_and_sync(&roadmap)?;
        ui::display_success(&format!("🎉 Successfully modified {} tasks!", modified_count));
    }
    
    Ok(())
}

/// Set phase for multiple tasks
pub fn bulk_set_phase(ids_str: &str, phase: &CliPhase) -> CommandResult {
    let mut roadmap = crate::state::load_state()?;
    let task_ids = utils::parse_and_validate_task_ids(ids_str, &roadmap)?;
    let new_phase: Phase = phase.clone().into();
    
    ui::display_info(&format!("{} Setting phase to {} for {} tasks...", 
        new_phase.emoji(), new_phase, task_ids.len()));
    
    let mut modified_count = 0;
    
    for &task_id in &task_ids {
        if let Some(task) = roadmap.tasks.iter_mut().find(|t| t.id == task_id) {
            if task.phase != new_phase {
                let old_phase = task.phase.clone();
                task.phase = new_phase.clone();
                modified_count += 1;
                ui::display_success(&format!("✅ Changed phase of task #{} from {} {} to {} {}: {}", 
                    task_id, old_phase.emoji(), old_phase, new_phase.emoji(), new_phase, task.description));
            } else {
                ui::display_info(&format!("ℹ️  Task #{} is already in {} phase", task_id, new_phase));
            }
        }
    }
    
    if modified_count > 0 {
        utils::save_and_sync(&roadmap)?;
        ui::display_success(&format!("🎉 Successfully modified {} tasks!", modified_count));
    }
    
    Ok(())
}

/// Reset multiple tasks to pending status
pub fn bulk_reset_tasks(ids_str: &str) -> CommandResult {
    let mut roadmap = crate::state::load_state()?;
    let task_ids = utils::parse_and_validate_task_ids(ids_str, &roadmap)?;
    
    ui::display_info(&format!("🔄 Resetting {} tasks to pending status...", task_ids.len()));
    
    let mut reset_count = 0;
    
    for &task_id in &task_ids {
        if let Some(task) = roadmap.tasks.iter_mut().find(|t| t.id == task_id) {
            if task.status == TaskStatus::Completed {
                task.status = TaskStatus::Pending;
                reset_count += 1;
                ui::display_success(&format!("✅ Reset task #{}: {}", task_id, task.description));
            } else {
                ui::display_info(&format!("ℹ️  Task #{} is already pending", task_id));
            }
        }
    }
    
    if reset_count > 0 {
        utils::save_and_sync(&roadmap)?;
        ui::display_success(&format!("🎉 Successfully reset {} tasks!", reset_count));
    }
    
    Ok(())
}

/// Remove multiple tasks
pub fn bulk_remove_tasks(ids_str: &str, force: bool) -> CommandResult {
    let mut roadmap = crate::state::load_state()?;
    let task_ids = utils::parse_and_validate_task_ids(ids_str, &roadmap)?;
    
    // Check for tasks that depend on the ones being removed
    let mut blocking_dependencies = Vec::new();
    for &task_id in &task_ids {
        let dependents = roadmap.get_dependents(task_id);
        if !dependents.is_empty() {
            // Filter out dependents that are also being removed
            let external_dependents: Vec<usize> = dependents.iter()
                .filter(|&&dep_id| !task_ids.contains(&dep_id))
                .copied()
                .collect();
            
            if !external_dependents.is_empty() {
                blocking_dependencies.push((task_id, external_dependents));
            }
        }
    }
    
    // Show warning about dependencies if not forced
    if !blocking_dependencies.is_empty() && !force {
        ui::display_warning("⚠️  The following tasks have dependencies that would be broken:");
        for (task_id, dependents) in &blocking_dependencies {
            if let Some(task) = roadmap.find_task_by_id(*task_id) {
                ui::display_error(&format!("  #{}: {} (depended on by: {})", 
                    task_id, task.description,
                    dependents.iter().map(|id| format!("#{}", id)).collect::<Vec<_>>().join(", ")));
            }
        }
        ui::display_info("💡 Use --force to remove tasks anyway (this will break dependencies)");
        return Err("Cannot remove tasks with dependencies. Use --force to override.".into());
    }
    
    ui::display_info(&format!("🗑️  Removing {} tasks...", task_ids.len()));
    
    let mut removed_count = 0;
    let mut task_descriptions = Vec::new();
    
    // Collect task descriptions before removal
    for &task_id in &task_ids {
        if let Some(task) = roadmap.find_task_by_id(task_id) {
            task_descriptions.push((task_id, task.description.clone()));
        }
    }
    
    // Remove tasks (in reverse order to maintain indices)
    let mut sorted_ids = task_ids.clone();
    sorted_ids.sort_by(|a, b| b.cmp(a)); // Sort in descending order
    
    for &task_id in &sorted_ids {
        if let Some(pos) = roadmap.tasks.iter().position(|t| t.id == task_id) {
            roadmap.tasks.remove(pos);
            removed_count += 1;
        }
    }
    
    // Show removed tasks
    for (task_id, description) in task_descriptions {
        ui::display_success(&format!("✅ Removed task #{}: {}", task_id, description));
    }
    
    if removed_count > 0 {
        utils::save_and_sync(&roadmap)?;
        ui::display_success(&format!("🎉 Successfully removed {} tasks!", removed_count));
        
        if !blocking_dependencies.is_empty() {
            ui::display_warning("⚠️  Some task dependencies were broken by this removal");
        }
    }
    
    Ok(())
} 