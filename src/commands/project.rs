//! Project management commands
//! 
//! This module handles project creation, switching, listing, and deletion.

use crate::{model::Roadmap, state, ui};
use super::CommandResult;
use colored::Colorize;
use crate::project::{get_current_project};

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
    
    // Show the project status (only for CLI, not TUI)
    super::core::show_project()?;
    
    Ok(())
}

/// Switch to a different project without showing status (TUI-safe)
pub fn switch_project_tui_safe(name: &str) -> CommandResult {
    use crate::project::{ProjectsConfig, set_current_project};
    
    // Load projects config
    let mut projects_config = ProjectsConfig::load()?;
    
    // Check if project exists
    if !projects_config.projects.contains_key(name) {
        return Err(format!("Project '{}' not found", name).into());
    }
    
    // Switch to the project
    set_current_project(name)?;
    
    // Update last accessed time
    projects_config.update_last_accessed(name)?;
    
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

/// Interactive project switcher interface
pub fn project_switcher() -> CommandResult {
    use crate::project::{ProjectsConfig};
    
    let projects_config = ProjectsConfig::load()?;
    let current_project = get_current_project()?;
    
    if projects_config.projects.is_empty() {
        ui::display_info("No projects found. Create a project with 'rask project create <name>'");
        return Ok(());
    }
    
    // Display the interactive switcher interface
    display_project_switcher(&projects_config, current_project.as_deref())?;
    
    Ok(())
}

/// Display the interactive project switcher interface
fn display_project_switcher(
    projects_config: &crate::project::ProjectsConfig,
    current_project: Option<&str>,
) -> CommandResult {
    use std::io::{self, Write};
    
    println!("\n{}", "🚀 Project Switcher".bright_cyan().bold());
    println!("{}", "═".repeat(60).bright_cyan());
    
    // Get projects sorted by last accessed (most recent first)
    let mut projects: Vec<_> = projects_config.projects.iter().collect();
    projects.sort_by(|a, b| b.1.last_accessed.cmp(&a.1.last_accessed));
    
    // Display current project info
    if let Some(current) = current_project {
        if let Some(project) = projects_config.get_project(current) {
            println!("\n📍 {}: {}", "Current Project".bright_green().bold(), current.bright_white().bold());
            if let Some(desc) = &project.description {
                println!("   📝 {}", desc.dimmed());
            }
            
            // Try to load and display project stats
            if let Ok(roadmap) = crate::state::load_state() {
                let stats = roadmap.get_statistics();
                println!("   📊 Progress: {}/{} tasks ({:.0}%)", 
                    stats.completed_tasks, 
                    stats.total_tasks,
                    stats.completion_percentage
                );
            }
        }
    } else {
        println!("\n⚠️  {}", "No current project selected".bright_yellow());
    }
    
    println!("\n📋 {}: ({} total)", "Available Projects".bright_blue().bold(), projects.len());
    println!("{}", "─".repeat(60).bright_black());
    
    // Display projects with numbers for selection
    for (index, (name, project)) in projects.iter().enumerate() {
        let number = format!("{}", index + 1).bright_white().bold();
        let is_current = current_project == Some(name);
        
        let project_name = if is_current {
            format!("{} {}", name, "(current)".bright_green())
        } else {
            name.bright_white().to_string()
        };
        
        println!("  {} {}", number, project_name);
        
        if let Some(desc) = &project.description {
            println!("     📝 {}", desc.dimmed());
        }
        
        // Show creation date
        if let Ok(datetime) = chrono::DateTime::parse_from_rfc3339(&project.created_at) {
            println!("     📅 Created: {}", 
                datetime.format("%Y-%m-%d").to_string().bright_black()
            );
        }
        
        // Try to load project stats
        let state_file = std::path::Path::new(&project.state_file);
        if state_file.exists() {
            if let Ok(content) = std::fs::read_to_string(state_file) {
                if let Ok(roadmap) = serde_json::from_str::<crate::model::Roadmap>(&content) {
                    let stats = roadmap.get_statistics();
                    let progress_bar = create_mini_progress_bar(stats.completed_tasks, stats.total_tasks);
                    println!("     📊 {} {}/{} tasks ({:.0}%)", 
                        progress_bar,
                        stats.completed_tasks, 
                        stats.total_tasks,
                        stats.completion_percentage
                    );
                }
            }
        } else {
            println!("     📊 {} (no tasks yet)", "Empty project".bright_black());
        }
        
        println!(); // Add spacing between projects
    }
    
    println!("{}", "─".repeat(60).bright_black());
    println!("💡 {}", "Actions:".bright_blue().bold());
    println!("   {}: Switch to project (enter project number)", format!("1-{}", projects.len()).bright_cyan());
    println!("   {} Create new project", "n:".bright_cyan());
    println!("   {} Refresh project list", "r:".bright_cyan());
    println!("   {} Exit switcher", "q:".bright_cyan());
    
    // Get user input
    print!("\n🎯 Choose an action: ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();
    
    match input.as_str() {
        "q" | "quit" | "exit" => {
            println!("👋 Goodbye!");
            return Ok(());
        },
        "n" | "new" => {
            return create_project_interactive();
        },
        "r" | "refresh" => {
            // Reload and redisplay
            let updated_config = crate::project::ProjectsConfig::load()?;
            let updated_current = get_current_project()?;
            return display_project_switcher(&updated_config, updated_current.as_deref());
        },
        _ => {
            // Try to parse as project number
            if let Ok(project_num) = input.parse::<usize>() {
                if project_num > 0 && project_num <= projects.len() {
                    let (project_name, _) = &projects[project_num - 1];
                    return switch_project(project_name);
                } else {
                    ui::display_error(&format!("Invalid project number. Please enter 1-{}", projects.len()));
                    return display_project_switcher(projects_config, current_project);
                }
            } else {
                ui::display_error("Invalid input. Please enter a project number, 'n' for new, 'r' for refresh, or 'q' to quit.");
                return display_project_switcher(projects_config, current_project);
            }
        }
    }
}

/// Create a mini progress bar for project display
fn create_mini_progress_bar(completed: usize, total: usize) -> String {
    if total == 0 {
        return "▱▱▱▱▱".bright_black().to_string();
    }
    
    let percentage = (completed as f64 / total as f64) * 100.0;
    let filled_blocks = ((percentage / 20.0).round() as usize).min(5);
    let empty_blocks = 5 - filled_blocks;
    
    let filled = "▰".repeat(filled_blocks);
    let empty = "▱".repeat(empty_blocks);
    
    match percentage {
        p if p >= 80.0 => format!("{}{}", filled.bright_green(), empty.bright_black()),
        p if p >= 60.0 => format!("{}{}", filled.bright_yellow(), empty.bright_black()),
        p if p >= 40.0 => format!("{}{}", filled.bright_blue(), empty.bright_black()),
        _ => format!("{}{}", filled.bright_red(), empty.bright_black()),
    }
}

/// Interactive project creation
fn create_project_interactive() -> CommandResult {
    use std::io::{self, Write};
    
    println!("\n{}", "📝 Create New Project".bright_cyan().bold());
    println!("{}", "═".repeat(40).bright_cyan());
    
    // Get project name
    print!("📋 Project name: ");
    io::stdout().flush()?;
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let name = name.trim();
    
    if name.is_empty() {
        ui::display_error("Project name cannot be empty");
        return Ok(());
    }
    
    // Get project description (optional)
    print!("📝 Project description (optional): ");
    io::stdout().flush()?;
    let mut description = String::new();
    io::stdin().read_line(&mut description)?;
    let description = description.trim();
    
    let description = if description.is_empty() {
        None
    } else {
        Some(description.to_string())
    };
    
    // Create the project
    create_project(name, &description)?;
    
    println!("\n✨ Project '{}' created successfully!", name.bright_green());
    println!("🚀 You are now working in the '{}' project", name.bright_white().bold());
    
    Ok(())
} 