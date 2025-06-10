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