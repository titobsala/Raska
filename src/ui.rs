use crate::model::{Priority, Roadmap, Task, TaskStatus};
use colored::*;
use std::collections::HashSet;

/// Displays the project roadmap with a beautiful formatted output
pub fn display_roadmap(roadmap: &Roadmap) {
    // Calculate progress statistics
    let total_tasks = roadmap.tasks.len();
    let completed_tasks = roadmap.tasks.iter().filter(|t| t.status == TaskStatus::Completed).count();
    let progress_percentage = if total_tasks > 0 { (completed_tasks * 100) / total_tasks } else { 0 };
    
    // Print header with project title
    println!("\n{}", "═".repeat(60).bright_blue());
    println!("  {}", roadmap.title.bold().bright_cyan());
    
    // Show current project information if available
    if let Ok(Some(project_info)) = crate::project::get_current_project_info() {
        println!("  📁 Project: {} {}", 
            project_info.name.bright_yellow(),
            if let Some(ref desc) = project_info.description {
                format!("({})", desc).italic().bright_black().to_string()
            } else {
                String::new()
            }
        );
    }
    
    println!("{}", "═".repeat(60).bright_blue());
    
    // Print progress bar
    display_progress_bar(completed_tasks, total_tasks);
    
    // Print task list header
    println!("\n  📋 {}:", "Tasks".bold());
    println!("  {}", "─".repeat(50).bright_black());
    
    // Print each task with enhanced formatting
    for task in &roadmap.tasks {
        display_task_line(task, false);
    }
    
    println!("  {}", "─".repeat(50).bright_black());
    
    // Print motivational message
    display_motivational_message(completed_tasks, total_tasks);
    
    println!();
}

/// Display a single task line with enhanced formatting
fn display_task_line(task: &Task, detailed: bool) {
    let status_icon = if task.status == TaskStatus::Completed { "✓" } else { "□" };
    let status_color = if task.status == TaskStatus::Completed { 
        status_icon.green() 
    } else { 
        status_icon.bright_black() 
    };
    
    // Priority indicator with color
    let priority_indicator = get_priority_indicator(&task.priority);
    
    // Task description with strikethrough if completed
    let description = if task.status == TaskStatus::Completed {
        task.description.strikethrough().dimmed()
    } else {
        task.description.normal()
    };
    
    // Format the main task line
    print!("  {} {}{} #{} {}", 
        status_color, 
        priority_indicator,
        if !task.tags.is_empty() { " " } else { "" },
        format!("{:2}", task.id).bright_white(),
        description
    );
    
    // Add tags if present
    if !task.tags.is_empty() {
        let tags_str = task.tags.iter()
            .map(|tag| format!("#{}", tag).bright_magenta().to_string())
            .collect::<Vec<_>>()
            .join(" ");
        print!(" {}", tags_str);
    }
    
    println!();
    
    // Show detailed info if requested
    if detailed {
        if let Some(ref notes) = task.notes {
            println!("      💭 {}", notes.italic().bright_black());
        }
        
        if !task.dependencies.is_empty() {
            let deps_str = task.dependencies.iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            println!("      🔗 Depends on: {}", deps_str.bright_yellow());
        }
    }
}

/// Get priority indicator with appropriate color
fn get_priority_indicator(priority: &Priority) -> colored::ColoredString {
    match priority {
        Priority::Critical => "🔥".red(),
        Priority::High => "⬆️".bright_red(),
        Priority::Medium => "▶️".yellow(),
        Priority::Low => "⬇️".green(),
    }
}

/// Display filtered tasks with optional detailed view
pub fn display_filtered_tasks(roadmap: &Roadmap, filtered_tasks: &[&Task], detailed: bool) {
    let total_tasks = roadmap.tasks.len();
    let filtered_count = filtered_tasks.len();
    
    // Print header
    println!("\n{}", "═".repeat(60).bright_blue());
    println!("  {} (Showing {} of {} tasks)", 
        roadmap.title.bold().bright_cyan(), 
        filtered_count.to_string().bright_white(),
        total_tasks.to_string().bright_white()
    );
    println!("{}", "═".repeat(60).bright_blue());
    
    if filtered_tasks.is_empty() {
        println!("\n  🔍 No tasks match your filter criteria.");
        println!("      Try adjusting your search terms or filters.\n");
        return;
    }
    
    // Print task list header
    println!("\n  📋 {}:", "Filtered Tasks".bold());
    println!("  {}", "─".repeat(50).bright_black());
    
    // Print each filtered task
    for task in filtered_tasks {
        display_task_line(task, detailed);
    }
    
    println!("  {}", "─".repeat(50).bright_black());
    
    // Print filter summary
    if filtered_count < total_tasks {
        println!("  📊 Showing {} of {} total tasks", 
            filtered_count.to_string().bright_white(),
            total_tasks.to_string().bright_white()
        );
    }
    
    println!();
}

/// Display enhanced add success message
pub fn display_add_success_enhanced(task: &Task) {
    println!("\n➕ {}: Task #{} added successfully!", 
        "Success".green().bold(), 
        task.id.to_string().bright_white()
    );
    
    println!("   📝 Task: {}", task.description.bright_white());
    println!("   🆔 Assigned ID: {}", task.id.to_string().bright_cyan());
    
    // Show priority if not default
    if task.priority != Priority::Medium {
        println!("   {} Priority: {}", 
            get_priority_indicator(&task.priority),
            format!("{}", task.priority).bright_white()
        );
    }
    
    // Show tags if present
    if !task.tags.is_empty() {
        let tags_str = task.tags.iter()
            .map(|tag| format!("#{}", tag).bright_magenta().to_string())
            .collect::<Vec<_>>()
            .join(" ");
        println!("   🏷️  Tags: {}", tags_str);
    }
    
    // Show notes if present
    if let Some(ref notes) = task.notes {
        println!("   💭 Notes: {}", notes.italic().bright_black());
    }
    
    // Show dependencies if present
    if !task.dependencies.is_empty() {
        let deps_str = task.dependencies.iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        println!("   🔗 Dependencies: {}", deps_str.bright_yellow());
    }
    
    println!("   💡 Task added to both state and markdown file!\n");
}

/// Displays a simple progress bar
fn display_progress_bar(completed: usize, total: usize) {
    let percentage = if total > 0 { (completed * 100) / total } else { 0 };
    let bar_width = 40;
    let filled = (percentage * bar_width) / 100;
    let empty = bar_width - filled;
    
    let filled_bar = "█".repeat(filled).bright_green();
    let empty_bar = "░".repeat(empty).bright_black();
    
    println!("  Progress: [{}{}] {}% ({}/{})", 
        filled_bar, empty_bar, percentage, completed, total);
}

/// Display motivational messages based on progress
fn display_motivational_message(completed: usize, total: usize) {
    if total == 0 {
        println!("  🌟 Ready to start your project!");
        return;
    }
    
    let percentage = (completed * 100) / total;
    let remaining = total - completed;
    
    match percentage {
        0 => println!("  🚀 Ready to start? Complete your first task!"),
        1..=25 => println!("  💪 Keep going! {} tasks remaining.", remaining),
        26..=50 => println!("  🎯 Great progress! You're {} tasks away from halfway.", total/2 - completed),
        51..=75 => println!("  🔥 Over halfway there! {} more to go!", remaining),
        76..=99 => println!("  🏁 Almost done! Just {} tasks left!", remaining),
        100 => println!("  🎉 Congratulations! All tasks completed! 🎊"),
        _ => println!("  📈 Keep up the great work!"),
    }
}

/// Display success message for project initialization
pub fn display_init_success(roadmap: &Roadmap) {
    println!("\n🎯 {}: Project initialized successfully!", "Success".green().bold());
    println!("   📝 Project: {}", roadmap.title.bright_cyan());
    println!("   📊 Total tasks: {}", roadmap.tasks.len().to_string().bright_white());
    println!("   💾 State saved to: {}", ".rask_state.json".bright_yellow());
    println!("\n   💡 Use {} to view your tasks!", "rask show".bright_cyan());
}

/// Display success message for task completion
pub fn display_completion_success(task_id: usize) {
    println!("\n✨ {}: Task #{} completed!", "Success".green().bold(), task_id.to_string().bright_white());
    println!("   🎊 Well done! Keep up the great work!");
}

/// Display success message for task addition (backward compatibility)
pub fn display_add_success(task_id: usize, description: &str) {
    println!("\n➕ {}: Task #{} added successfully!", "Success".green().bold(), task_id.to_string().bright_white());
    println!("   📝 Task: {}", description.bright_white());
    println!("   🆔 Assigned ID: {}", task_id.to_string().bright_cyan());
    println!("   💡 Task added to both state and markdown file!");
}

/// Display success message for task removal
pub fn display_remove_success(description: &str) {
    println!("\n🗑️  {}: Task removed successfully!", "Success".green().bold());
    println!("   📝 Removed: {}", description.strikethrough().bright_black());
    println!("   💡 Task removed from both state and markdown file!");
}

/// Display success message for task editing
pub fn display_edit_success(task_id: usize, old_description: &str, new_description: &str) {
    println!("\n✏️  {}: Task #{} updated successfully!", "Success".green().bold(), task_id.to_string().bright_white());
    println!("   📝 Old: {}", old_description.strikethrough().bright_black());
    println!("   📝 New: {}", new_description.bright_white());
    println!("   💡 Changes synced to both state and markdown file!");
}

/// Display success message for task reset
pub fn display_reset_success(task_id: Option<usize>) {
    match task_id {
        Some(id) => {
            println!("\n🔄 {}: Task #{} reset to pending!", "Success".green().bold(), id.to_string().bright_white());
            println!("   💡 Task status updated in both state and markdown file!");
        },
        None => {
            println!("\n🔄 {}: All tasks reset to pending!", "Success".green().bold());
            println!("   💡 All task statuses updated in both state and markdown file!");
        }
    }
}

/// Display informational messages
pub fn display_info(message: &str) {
    println!("\n💡 {}: {}", "Info".blue().bold(), message);
}

/// Display error messages
pub fn display_error(message: &str) {
    eprintln!("\n❌ {}: {}", "Error".red().bold(), message);
}

/// Display success messages
pub fn display_success(message: &str) {
    println!("\n✅ {}: {}", "Success".green().bold(), message);
}

/// Display warning messages
pub fn display_warning(message: &str) {
    println!("\n⚠️  {}: {}", "Warning".yellow().bold(), message);
}

/// Display list of projects
pub fn display_projects_list(projects_config: &crate::project::ProjectsConfig, current_project: Option<&str>) {
    use chrono::{DateTime, Utc};
    
    println!("\n{}", "═".repeat(60).bright_blue());
    println!("  {} ({})", 
        "Available Projects".bold().bright_cyan(),
        projects_config.projects.len().to_string().bright_white()
    );
    println!("{}", "═".repeat(60).bright_blue());
    
    // Sort projects by last accessed time (most recent first)
    let mut sorted_projects: Vec<_> = projects_config.projects.iter().collect();
    sorted_projects.sort_by(|a, b| {
        let time_a = DateTime::parse_from_rfc3339(&a.1.last_accessed)
            .unwrap_or_else(|_| DateTime::parse_from_rfc3339("1970-01-01T00:00:00Z").unwrap());
        let time_b = DateTime::parse_from_rfc3339(&b.1.last_accessed)
            .unwrap_or_else(|_| DateTime::parse_from_rfc3339("1970-01-01T00:00:00Z").unwrap());
        time_b.cmp(&time_a)
    });
    
    for (name, config) in sorted_projects {
        let is_current = current_project == Some(name);
        let is_default = projects_config.default_project.as_ref() == Some(name);
        
        // Format project name with indicators
        let mut project_name = if is_current {
            format!("👉 {}", name.bright_cyan().bold())
        } else {
            name.bright_white().to_string()
        };
        
        if is_default {
            project_name = format!("{} {}", project_name, "(default)".bright_green());
        }
        
        println!("\n  📁 {}", project_name);
        
        // Show description if available
        if let Some(ref description) = config.description {
            println!("     📝 {}", description.italic().bright_black());
        }
        
        // Show creation date
        if let Ok(created_time) = DateTime::parse_from_rfc3339(&config.created_at) {
            let created_local = created_time.with_timezone(&chrono::Local);
            println!("     📅 Created: {}", created_local.format("%Y-%m-%d %H:%M").to_string().bright_black());
        }
        
        // Show last accessed if not current
        if !is_current {
            if let Ok(accessed_time) = DateTime::parse_from_rfc3339(&config.last_accessed) {
                let accessed_local = accessed_time.with_timezone(&chrono::Local);
                println!("     🕒 Last accessed: {}", accessed_local.format("%Y-%m-%d %H:%M").to_string().bright_black());
            }
        }
        
        // Show state file path
        println!("     💾 State file: {}", config.state_file.bright_yellow());
        
        // Show source file if available
        if let Some(ref source_file) = config.source_file {
            println!("     📄 Source: {}", source_file.bright_yellow());
        }
    }
    
    println!("\n  💡 Use {} to switch projects", "rask project switch <name>".bright_cyan());
    println!("  💡 Use {} to create a new project", "rask project create <name>".bright_cyan());
    println!();
}

/// Display dependency error with helpful information
pub fn display_dependency_error(task_id: usize, incomplete_deps: &[usize], roadmap: &crate::model::Roadmap) {
    println!("\n🚫 {}: Cannot complete task #{}", "Dependency Error".red().bold(), task_id);
    
    if let Some(task) = roadmap.find_task_by_id(task_id) {
        println!("   📝 Task: {}", task.description.bright_white());
    }
    
    println!("   🔗 Missing dependencies:");
    for &dep_id in incomplete_deps {
        if let Some(dep_task) = roadmap.find_task_by_id(dep_id) {
            println!("      #{} {}", 
                dep_id.to_string().bright_red(), 
                dep_task.description.dimmed()
            );
        }
    }
    
    println!("\n   💡 Complete the missing dependencies first, then try again.");
}

/// Display dependency validation errors
pub fn display_dependency_validation_errors(errors: &[crate::model::DependencyError]) {
    println!("\n🚫 {}: Found {} dependency issue(s)", 
        "Validation Failed".red().bold(), 
        errors.len().to_string().bright_white()
    );
    
    for (i, error) in errors.iter().enumerate() {
        println!("   {}. {}", (i + 1).to_string().bright_red(), error);
    }
    
    println!("\n   💡 Fix these issues before proceeding.");
}

/// Display comprehensive dependency overview
pub fn display_dependency_overview(roadmap: &crate::model::Roadmap) {
    println!("\n{}", "═".repeat(60).bright_blue());
    println!("  {}", "Dependency Analysis Overview".bold().bright_cyan());
    println!("{}", "═".repeat(60).bright_blue());
    
    let ready_tasks = roadmap.get_ready_tasks();
    let blocked_tasks = roadmap.get_blocked_tasks();
    let total_tasks = roadmap.tasks.len();
    let tasks_with_deps = roadmap.tasks.iter().filter(|t| !t.dependencies.is_empty()).count();
    
    println!("\n  📊 {}:", "Statistics".bold());
    println!("      Total tasks: {}", total_tasks.to_string().bright_white());
    println!("      Tasks with dependencies: {}", tasks_with_deps.to_string().bright_white());
    println!("      Ready to start: {}", ready_tasks.len().to_string().bright_green());
    println!("      Blocked by dependencies: {}", blocked_tasks.len().to_string().bright_red());
    
    // Show validation status
    match roadmap.validate_all_dependencies() {
        Ok(()) => {
            println!("      Validation status: {}", "✓ All dependencies valid".bright_green());
        }
        Err(errors) => {
            println!("      Validation status: {} ({} issues)", 
                "✗ Issues found".bright_red(),
                errors.len().to_string().bright_red()
            );
        }
    }
    
    println!("\n  💡 Use {} to see specific analysis", "rask dependencies --help".bright_cyan());
    println!("  💡 Use {} to validate all dependencies", "rask dependencies --validate".bright_cyan());
    println!("  💡 Use {} to see ready tasks", "rask dependencies --ready".bright_cyan());
    println!();
}

/// Display dependency tree for a specific task
pub fn display_dependency_tree(tree: &crate::model::DependencyNode, roadmap: &crate::model::Roadmap) {
    println!("\n{}", "═".repeat(60).bright_blue());
    println!("  {} #{}", "Dependency Tree for Task".bold().bright_cyan(), tree.task_id.to_string().bright_white());
    println!("{}", "═".repeat(60).bright_blue());
    
    display_dependency_node(tree, 0, true);
    
    // Show dependency chain
    let chain = roadmap.get_dependency_chain(tree.task_id);
    if !chain.is_empty() {
        println!("\n  📋 {}:", "Full Dependency Chain".bold());
        let chain_str = chain.iter()
            .map(|id| format!("#{}", id))
            .collect::<Vec<_>>()
            .join(" → ");
        println!("      {}", chain_str.bright_yellow());
    }
    
    // Show reverse dependencies (tasks that depend on this one)
    let dependents = roadmap.get_dependents(tree.task_id);
    if !dependents.is_empty() {
        println!("\n  🔄 {}:", "Tasks depending on this".bold());
        for &dep_id in &dependents {
            if let Some(task) = roadmap.find_task_by_id(dep_id) {
                println!("      #{} {}", dep_id.to_string().bright_cyan(), task.description.dimmed());
            }
        }
    }
    
    println!();
}

fn display_dependency_node(node: &crate::model::DependencyNode, depth: usize, is_last: bool) {
    let indent = "  ".repeat(depth);
    let prefix = if depth == 0 {
        "  📝"
    } else if is_last {
        "  └─"
    } else {
        "  ├─"
    };
    
    let status_icon = match node.status {
        crate::model::TaskStatus::Completed => "✓".green(),
        crate::model::TaskStatus::Pending => "□".bright_black(),
    };
    
    let task_desc = if node.is_circular {
        node.description.red().italic()
    } else {
        match node.status {
            crate::model::TaskStatus::Completed => node.description.dimmed().strikethrough(),
            crate::model::TaskStatus::Pending => node.description.normal(),
        }
    };
    
    println!("{}{} {} #{} {}", 
        indent, prefix, status_icon, 
        node.task_id.to_string().bright_white(), 
        task_desc
    );
    
    for (i, dep) in node.dependencies.iter().enumerate() {
        let is_last_dep = i == node.dependencies.len() - 1;
        display_dependency_node(dep, depth + 1, is_last_dep);
    }
}

/// Display tasks ready to be started
pub fn display_ready_tasks(ready_tasks: &[&crate::model::Task]) {
    println!("\n{}", "═".repeat(60).bright_blue());
    println!("  {} ({})", 
        "Tasks Ready to Start".bold().bright_green(),
        ready_tasks.len().to_string().bright_white()
    );
    println!("{}", "═".repeat(60).bright_blue());
    
    if ready_tasks.is_empty() {
        println!("\n  🎯 No tasks are currently ready to start.");
        println!("      All pending tasks are blocked by dependencies.");
    } else {
        println!("\n  🚀 These tasks have all dependencies completed:");
        for task in ready_tasks {
            let priority_icon = get_priority_indicator(&task.priority);
            println!("      {} {} #{} {}", 
                priority_icon,
                "□".bright_green(),
                task.id.to_string().bright_white(),
                task.description
            );
        }
    }
    
    println!();
}

/// Display tasks blocked by dependencies
pub fn display_blocked_tasks(blocked_tasks: &[&crate::model::Task], roadmap: &crate::model::Roadmap) {
    println!("\n{}", "═".repeat(60).bright_blue());
    println!("  {} ({})", 
        "Tasks Blocked by Dependencies".bold().bright_red(),
        blocked_tasks.len().to_string().bright_white()
    );
    println!("{}", "═".repeat(60).bright_blue());
    
    if blocked_tasks.is_empty() {
        println!("\n  ✨ No tasks are currently blocked!");
        println!("      All pending tasks are ready to start.");
    } else {
        let completed_ids = roadmap.get_completed_task_ids();
        
        println!("\n  🚫 These tasks are waiting for dependencies:");
        for task in blocked_tasks {
            let priority_icon = get_priority_indicator(&task.priority);
            let incomplete_deps: Vec<usize> = task.dependencies.iter()
                .filter(|&&dep_id| !completed_ids.contains(&dep_id))
                .copied()
                .collect();
            
            println!("      {} {} #{} {}", 
                priority_icon,
                "□".bright_red(),
                task.id.to_string().bright_white(),
                task.description
            );
            
            if !incomplete_deps.is_empty() {
                println!("        🔗 Waiting for: {}", 
                    incomplete_deps.iter()
                        .map(|id| format!("#{}", id))
                        .collect::<Vec<_>>()
                        .join(", ")
                        .bright_yellow()
                );
            }
        }
    }
    
    println!();
} 