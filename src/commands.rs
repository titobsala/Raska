use crate::{cli::CliPriority, markdown_writer, model::{TaskStatus, Priority, Task, Roadmap}, parser, state, ui};
use crate::cli::ConfigCommands;
use crate::config::RaskConfig;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

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

/// Show the current project status with enhanced display
pub fn show_project() -> CommandResult {
    let roadmap = state::load_state()?;
    ui::display_roadmap_enhanced(&roadmap, true); // Show detailed view with tags, priorities, and notes
    Ok(())
}

/// Find tasks that become unblocked after completing a specific task
fn find_newly_unblocked_tasks(roadmap: &Roadmap, completed_task_id: usize) -> Vec<usize> {
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

/// Enhanced input validation for task descriptions
fn validate_task_description(description: &str) -> Result<(), String> {
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
    let newly_unblocked = find_newly_unblocked_tasks(&roadmap, task_id);
    
    // Find and update the task
    let task = roadmap.tasks.iter_mut().find(|t| t.id == task_id);
    
    match task {
        Some(task) => {
            let task_description = task.description.clone();
            task.mark_completed();
            
            // Save to both JSON state and original markdown file
            state::save_state(&roadmap)?;
            markdown_writer::sync_to_source_file(&roadmap)?;
            
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
    if let Err(validation_error) = validate_task_description(description) {
        ui::display_error(&format!("Invalid task description: {}", validation_error));
        ui::display_info("💡 Try providing a more descriptive task name");
        return Err(validation_error.into());
    }
    
    // Load current state
    let mut roadmap = state::load_state()?;
    
    // Parse tags with validation
    let parsed_tags: Vec<String> = if let Some(tag_str) = tags {
        let tags: Vec<String> = tag_str.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        
        // Validate tag format
        for tag in &tags {
            if tag.len() > 50 {
                return Err(format!("Tag '{}' is too long (max 50 characters)", tag).into());
            }
            if !tag.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
                return Err(format!("Tag '{}' contains invalid characters. Use only letters, numbers, hyphens, and underscores", tag).into());
            }
        }
        tags
    } else {
        Vec::new()
    };
    
    // Parse dependencies with enhanced validation
    let parsed_deps: Vec<usize> = if let Some(dep_str) = dependencies {
        let deps: Vec<usize> = dep_str.split(',')
            .filter_map(|s| {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    match trimmed.parse() {
                        Ok(id) => Some(id),
                        Err(_) => {
                            ui::display_warning(&format!("Invalid dependency ID '{}' - must be a number", trimmed));
                            None
                        }
                    }
                }
            })
            .collect();
        
        // Validate dependencies exist
        for &dep_id in &deps {
            if roadmap.find_task_by_id(dep_id).is_none() {
                return Err(format!("Dependency task {} does not exist. Use 'rask list' to see available tasks.", dep_id).into());
            }
        }
        deps
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

/// Handle configuration-related commands
/// This provides a comprehensive configuration management interface
pub fn handle_config_command(config_command: &ConfigCommands) -> CommandResult {
    match config_command {
        ConfigCommands::Show { section } => show_config(section.as_deref()),
        ConfigCommands::Set { key, value, project } => set_config(key, value, *project),
        ConfigCommands::Get { key } => get_config(key),
        ConfigCommands::Edit { project } => edit_config(*project),
        ConfigCommands::Init { project, user } => init_config(*project, *user),
        ConfigCommands::Reset { project, user, force } => reset_config(*project, *user, *force),
    }
}

/// Show current configuration or a specific section
fn show_config(section: Option<&str>) -> CommandResult {
    let config = RaskConfig::load()?;
    
    match section {
        Some("ui") => {
            ui::display_info("🎨 UI Configuration:");
            println!("  Color scheme: {:?}", config.ui.color_scheme);
            println!("  Show completed: {}", config.ui.show_completed);
            println!("  Default sort: {}", config.ui.default_sort);
            println!("  Compact view: {}", config.ui.compact_view);
            println!("  Show task IDs: {}", config.ui.show_task_ids);
            println!("  Max width: {} (0 = auto)", config.ui.max_width);
        },
        Some("behavior") => {
            ui::display_info("⚙️  Behavior Configuration:");
            println!("  Default project: {:?}", config.behavior.default_project);
            println!("  Default priority: {}", config.behavior.default_priority);
            println!("  Default tags: {:?}", config.behavior.default_tags);
            println!("  Auto archive days: {} (0 = never)", config.behavior.auto_archive_days);
            println!("  Warn on circular: {}", config.behavior.warn_on_circular);
            println!("  Confirm destructive: {}", config.behavior.confirm_destructive);
            println!("  Auto sync markdown: {}", config.behavior.auto_sync_markdown);
        },
        Some("export") => {
            ui::display_info("📤 Export Configuration:");
            println!("  Default format: {}", config.export.default_format);
            println!("  Default path: {:?}", config.export.default_path);
            println!("  Include completed: {}", config.export.include_completed);
            println!("  Include metadata: {}", config.export.include_metadata);
        },
        Some("advanced") => {
            ui::display_info("🔧 Advanced Configuration:");
            println!("  Aliases: {:?}", config.advanced.aliases);
            println!("  Editor: {:?}", config.advanced.editor);
            println!("  Templates: {:?}", config.advanced.templates);
            println!("  Debug: {}", config.advanced.debug);
        },
        Some("theme") => {
            ui::display_info("🎭 Theme Configuration:");
            println!("  Name: {}", config.theme.name);
            println!("  Priority colors: {:?}", config.theme.priority_colors);
            println!("  Status colors: {:?}", config.theme.status_colors);
            println!("  Symbols: {:?}", config.theme.symbols);
        },
        Some(unknown) => {
            return Err(format!("Unknown configuration section: {}. Available sections: ui, behavior, export, advanced, theme", unknown).into());
        },
        None => {
            // Show all configuration
            ui::display_info("📋 Complete Rask Configuration:");
            show_config(Some("ui"))?;
            println!();
            show_config(Some("behavior"))?;
            println!();
            show_config(Some("export"))?;
            println!();
            show_config(Some("advanced"))?;
            println!();
            show_config(Some("theme"))?;
            
            // Show config file locations
            println!();
            ui::display_info("📁 Configuration Files:");
            if let Ok(user_config_dir) = crate::config::get_rask_config_dir() {
                println!("  User config: {}", user_config_dir.join("config.toml").display());
            }
            println!("  Project config: .rask/config.toml");
        }
    }
    
    Ok(())
}

/// Set a configuration value
fn set_config(key: &str, value: &str, project_config: bool) -> CommandResult {
    let mut config = RaskConfig::load()?;
    
    // Set the configuration value
    config.set(key, value)?;
    
    // Save to the appropriate config file
    if project_config {
        config.save_project_config()?;
        ui::display_success(&format!("Set {} = {} in project configuration", key, value));
    } else {
        config.save_user_config()?;
        ui::display_success(&format!("Set {} = {} in user configuration", key, value));
    }
    
    Ok(())
}

/// Get a configuration value
fn get_config(key: &str) -> CommandResult {
    let config = RaskConfig::load()?;
    
    if let Some(value) = config.get(key) {
        println!("{}", value);
    } else {
        return Err(format!("Configuration key '{}' not found", key).into());
    }
    
    Ok(())
}

/// Edit configuration in the user's preferred editor
fn edit_config(project_config: bool) -> CommandResult {
    let config = RaskConfig::load()?;
    
    // Determine the editor to use
    let editor_env = std::env::var("EDITOR").ok();
    let editor = config.advanced.editor
        .as_ref()
        .or_else(|| editor_env.as_ref())
        .ok_or("No editor configured. Set EDITOR environment variable or use 'rask config set advanced.editor <editor>'")?;
    
    // Determine the config file path
    let config_path = if project_config {
        // Ensure local .rask directory exists
        crate::project::init_local_rask_directory()?;
        PathBuf::from(".rask/config.toml")
    } else {
        let config_dir = crate::config::get_rask_config_dir()?;
        config_dir.join("config.toml")
    };
    
    // Create the config file if it doesn't exist
    if !config_path.exists() {
        if project_config {
            RaskConfig::init_project_config()?;
        } else {
            RaskConfig::init_user_config()?;
        }
    }
    
    // Launch the editor
    let status = Command::new(editor)
        .arg(&config_path)
        .status()?;
    
    if status.success() {
        ui::display_success(&format!("Configuration file {} edited successfully", config_path.display()));
    } else {
        return Err("Editor exited with error".into());
    }
    
    Ok(())
}

/// Initialize configuration files
fn init_config(project_config: bool, user_config: bool) -> CommandResult {
    if !project_config && !user_config {
        return Err("Specify --project or --user to initialize configuration".into());
    }
    
    if project_config {
        crate::project::init_local_rask_directory()?;
        RaskConfig::init_project_config()?;
        ui::display_success("Initialized project configuration at .rask/config.toml");
    }
    
    if user_config {
        RaskConfig::init_user_config()?;
        let config_dir = crate::config::get_rask_config_dir()?;
        ui::display_success(&format!("Initialized user configuration at {}", config_dir.join("config.toml").display()));
    }
    
    Ok(())
}

/// Reset configuration to defaults
fn reset_config(project_config: bool, user_config: bool, force: bool) -> CommandResult {
    if !project_config && !user_config {
        return Err("Specify --project or --user to reset configuration".into());
    }
    
    if !force {
        ui::display_warning("This will reset configuration to defaults and cannot be undone.");
        ui::display_info("Use --force to confirm the reset operation");
        return Ok(());
    }
    
    if project_config {
        RaskConfig::init_project_config()?;
        ui::display_success("Reset project configuration to defaults");
    }
    
    if user_config {
        RaskConfig::init_user_config()?;
        ui::display_success("Reset user configuration to defaults");
    }
    
    Ok(())
} 