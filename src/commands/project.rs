//! Project management commands
//! 
//! This module handles project creation, switching, listing, and deletion.

use crate::{model::Roadmap, state, ui};
use super::{CommandResult, utils};

/// Create a new project
pub fn create_project(name: &str, description: &Option<String>) -> CommandResult {
    use crate::project::{ProjectsConfig, set_current_project};
    
    // Load existing projects config
    let mut projects_config = ProjectsConfig::load()?;
    
    // Add the new project
    projects_config.add_project(name.to_string(), description.clone())?;
    
    // Create an empty roadmap for the new project
    let mut roadmap = Roadmap::new(format!("{} Project", name));
    roadmap.project_id = Some(name.to_string());
    roadmap.metadata.name = name.to_string();
    if let Some(desc) = description {
        roadmap.metadata.description = Some(desc.clone());
    }
    
    // Switch to the new project
    set_current_project(name)?;
    
    // Save the initial roadmap state
    state::save_state(&roadmap)?;
    
    ui::display_success(&format!("Created project '{}' and switched to it", name));
    if let Some(desc) = description {
        ui::display_info(&format!("Description: {}", desc));
    }
    
    Ok(())
}

/// Switch to a different project
pub fn switch_project(name: &str) -> CommandResult {
    use crate::project::{ProjectsConfig, set_current_project};
    
    // Load projects config
    let mut projects_config = ProjectsConfig::load()?;
    
    // Check if project exists
    if !projects_config.projects.contains_key(name) {
        return Err(format!("Project '{}' not found. Use 'rask project list' to see available projects.", name).into());
    }
    
    // Switch to the project
    set_current_project(name)?;
    
    // Update last accessed time
    projects_config.update_last_accessed(name)?;
    
    ui::display_success(&format!("Switched to project '{}'", name));
    
    // Show the project status
    super::core::show_project()?;
    
    Ok(())
}

/// List all projects
pub fn list_projects() -> CommandResult {
    use crate::project::{ProjectsConfig, get_current_project};
    
    let projects_config = ProjectsConfig::load()?;
    let current_project = get_current_project()?;
    
    if projects_config.projects.is_empty() {
        ui::display_info("No projects found. Create a project with 'rask project create <name>'");
        return Ok(());
    }
    
    ui::display_projects_list(&projects_config, current_project.as_deref());
    Ok(())
}

/// Delete a project
pub fn delete_project(name: &str, force: bool) -> CommandResult {
    use crate::project::{ProjectsConfig, get_current_project, set_current_project};
    
    // Load projects config
    let mut projects_config = ProjectsConfig::load()?;
    
    // Check if project exists
    if !projects_config.projects.contains_key(name) {
        return Err(format!("Project '{}' not found", name).into());
    }
    
    // Confirmation unless forced
    if !force {
        ui::display_warning(&format!("This will permanently delete project '{}' and all its data.", name));
        ui::display_info("Use --force to confirm deletion or use 'rask project delete <name> --force'");
        return Ok(());
    }
    
    // Check if this is the current project
    let current_project = get_current_project()?;
    let is_current = current_project.as_deref() == Some(name);
    
    // Remove the project
    projects_config.remove_project(name)?;
    
    // If this was the current project, switch to another one or clear current
    if is_current {
        if let Some(default_project) = &projects_config.default_project {
            set_current_project(default_project)?;
            ui::display_info(&format!("Switched to project '{}'", default_project));
        } else {
            // No projects left, remove current project file
            if let Err(_) = std::fs::remove_file(".rask_current_project") {
                // Ignore errors when removing current project file
            }
            ui::display_info("No projects remaining");
        }
    }
    
    ui::display_success(&format!("Deleted project '{}'", name));
    Ok(())
} 