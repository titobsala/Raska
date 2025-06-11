use crate::{cli::CliPriority, markdown_writer, model::{TaskStatus, Priority, Task, Roadmap}, parser, state, ui};
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
    
    // Find and update the task
    let task = roadmap.tasks.iter_mut().find(|t| t.id == task_id);
    
    match task {
        Some(task) => {
            task.mark_completed();
            
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



/// Add a new task with enhanced metadata support
pub fn add_task_enhanced(
    description: &str,
    tags: &Option<String>,
    priority: &Option<CliPriority>,
    notes: &Option<String>,
    dependencies: &Option<String>,
) -> CommandResult {
    // Load current state
    let mut roadmap = state::load_state()?;
    
    // Parse tags
    let parsed_tags: Vec<String> = if let Some(tag_str) = tags {
        tag_str.split(',').map(|s| s.trim().to_string()).collect()
    } else {
        Vec::new()
    };
    
    // Parse dependencies
    let parsed_deps: Vec<usize> = if let Some(dep_str) = dependencies {
        dep_str.split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect()
    } else {
        Vec::new()
    };
    
    // Validate dependencies exist
    for &dep_id in &parsed_deps {
        if roadmap.find_task_by_id(dep_id).is_none() {
            return Err(format!("Dependency task {} does not exist.", dep_id).into());
        }
    }
    
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
        new_task = new_task.with_notes(note_text.clone());
    }
    
    if !parsed_deps.is_empty() {
        new_task = new_task.with_dependencies(parsed_deps);
    }
    
    // Add task to roadmap
    roadmap.add_task(new_task.clone());
    
    // Save to both JSON state and original markdown file
    state::save_state(&roadmap)?;
    markdown_writer::sync_to_source_file(&roadmap)?;
    
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
        state::save_state(&roadmap)?;
        markdown_writer::sync_to_source_file(&roadmap)?;
        
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
                        task.mark_pending();
                        
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
                    task.mark_pending();
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
    show_project()?;
    
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