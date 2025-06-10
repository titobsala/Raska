use crate::{markdown_writer, model::TaskStatus, parser, state, ui};
use std::fs;
use std::path::PathBuf;

/// Result type for command operations
pub type CommandResult = Result<(), Box<dyn std::error::Error>>;

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

/// Show the current project status
pub fn show_project() -> CommandResult {
    let roadmap = state::load_state()?;
    ui::display_roadmap(&roadmap);
    Ok(())
}

/// Mark a task as completed
pub fn complete_task(task_id: usize) -> CommandResult {
    // Load current state
    let mut roadmap = state::load_state()?;
    
    // Find and update the task
    let task = roadmap.tasks.iter_mut().find(|t| t.id == task_id);
    
    match task {
        Some(task) => {
            task.status = TaskStatus::Completed;
            
            // Save to both JSON state and original markdown file
            state::save_state(&roadmap)?;
            markdown_writer::sync_to_source_file(&roadmap)?;
            
            // Display success and updated roadmap
            ui::display_completion_success(task_id);
            ui::display_roadmap(&roadmap);
            
            Ok(())
        }
        None => Err(format!("Task with ID {} not found.", task_id).into()),
    }
}

/// Add a new task to the project
pub fn add_task(description: &str) -> CommandResult {
    // Load current state
    let mut roadmap = state::load_state()?;
    
    // Generate new task ID (highest existing ID + 1)
    let new_id = roadmap.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    
    // Create new task
    let new_task = crate::model::Task {
        id: new_id,
        description: description.to_string(),
        status: TaskStatus::Pending,
    };
    
    // Add task to roadmap
    roadmap.tasks.push(new_task);
    
    // Save to both JSON state and original markdown file
    state::save_state(&roadmap)?;
    markdown_writer::sync_to_source_file(&roadmap)?;
    
    // Display success and updated roadmap
    ui::display_add_success(new_id, description);
    ui::display_roadmap(&roadmap);
    
    Ok(())
}

/// Remove a task from the project
pub fn remove_task(task_id: usize) -> CommandResult {
    // Load current state
    let mut roadmap = state::load_state()?;
    
    // Find task index
    let task_index = roadmap.tasks.iter().position(|t| t.id == task_id);
    
    match task_index {
        Some(index) => {
            let removed_task = roadmap.tasks.remove(index);
            
            // Renumber remaining tasks to maintain sequential IDs
            for (i, task) in roadmap.tasks.iter_mut().enumerate() {
                task.id = i + 1;
            }
            
            // Save to both JSON state and original markdown file
            state::save_state(&roadmap)?;
            markdown_writer::sync_to_source_file(&roadmap)?;
            
            // Display success and updated roadmap
            ui::display_remove_success(&removed_task.description);
            ui::display_roadmap(&roadmap);
            
            Ok(())
        }
        None => Err(format!("Task with ID {} not found.", task_id).into()),
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
            state::save_state(&roadmap)?;
            markdown_writer::sync_to_source_file(&roadmap)?;
            
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
                        task.status = TaskStatus::Pending;
                        
                        // Save to both JSON state and original markdown file
                        state::save_state(&roadmap)?;
                        markdown_writer::sync_to_source_file(&roadmap)?;
                        
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
                    task.status = TaskStatus::Pending;
                }
                
                // Save to both JSON state and original markdown file
                state::save_state(&roadmap)?;
                markdown_writer::sync_to_source_file(&roadmap)?;
                
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