use crate::model::{Priority, Roadmap, Task, TaskStatus};
use colored::*;

/// Displays the project roadmap with a beautiful formatted output
pub fn display_roadmap(roadmap: &Roadmap) {
    display_roadmap_enhanced(roadmap, false);
}

/// Enhanced roadmap display with optional detailed view
pub fn display_roadmap_enhanced(roadmap: &Roadmap, show_detailed: bool) {
    // Calculate progress statistics
    let total_tasks = roadmap.tasks.len();
    let completed_tasks = roadmap.tasks.iter().filter(|t| t.status == TaskStatus::Completed).count();
    let _progress_percentage = if total_tasks > 0 { (completed_tasks * 100) / total_tasks } else { 0 };
    
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
    println!("\n  📋 {}{}:", 
        "Tasks".bold(),
        if show_detailed { " (Detailed View)" } else { "" }
    );
    println!("  {}", "─".repeat(50).bright_black());
    
    // Print each task with enhanced formatting
    for task in &roadmap.tasks {
        display_task_line(task, show_detailed);
    }
    
    println!("  {}", "─".repeat(50).bright_black());
    
    // Print motivational message
    display_motivational_message(completed_tasks, total_tasks);
    
    // Show summary statistics if in detailed mode
    if show_detailed {
        display_project_statistics(roadmap);
    }
    
    println!();
}

/// Display project statistics summary
fn display_project_statistics(roadmap: &Roadmap) {
    let total_tasks = roadmap.tasks.len();
    let completed_tasks = roadmap.tasks.iter().filter(|t| t.status == TaskStatus::Completed).count();
    let pending_tasks = total_tasks - completed_tasks;
    
    // Priority breakdown
    let critical_tasks = roadmap.tasks.iter().filter(|t| t.priority == Priority::Critical && t.status == TaskStatus::Pending).count();
    let high_tasks = roadmap.tasks.iter().filter(|t| t.priority == Priority::High && t.status == TaskStatus::Pending).count();
    let medium_tasks = roadmap.tasks.iter().filter(|t| t.priority == Priority::Medium && t.status == TaskStatus::Pending).count();
    let low_tasks = roadmap.tasks.iter().filter(|t| t.priority == Priority::Low && t.status == TaskStatus::Pending).count();
    
    // Dependency analysis
    let ready_tasks = roadmap.tasks.iter()
        .filter(|t| t.status == TaskStatus::Pending && t.can_be_started(&roadmap.get_completed_task_ids()))
        .count();
    let blocked_tasks = pending_tasks - ready_tasks;
    
    println!("\n  📊 {}:", "Project Statistics".bold().bright_cyan());
    println!("      📈 Progress: {}/{} completed ({:.1}%)", 
        completed_tasks, total_tasks, 
        if total_tasks > 0 { (completed_tasks as f64 / total_tasks as f64) * 100.0 } else { 0.0 }
    );
    
    if pending_tasks > 0 {
        println!("      🎯 Priority Breakdown:");
        if critical_tasks > 0 { println!("         🔥 Critical: {}", critical_tasks.to_string().bright_red()); }
        if high_tasks > 0 { println!("         ⬆️  High: {}", high_tasks.to_string().red()); }
        if medium_tasks > 0 { println!("         ▶️  Medium: {}", medium_tasks.to_string().yellow()); }
        if low_tasks > 0 { println!("         ⬇️  Low: {}", low_tasks.to_string().green()); }
        
        println!("      🚀 Task Status:");
        println!("         ✅ Ready to start: {}", ready_tasks.to_string().bright_green());
        if blocked_tasks > 0 {
            println!("         🔒 Blocked by dependencies: {}", blocked_tasks.to_string().bright_red());
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

/// Get priority color for task text based on priority level
fn get_priority_color(priority: &Priority) -> fn(&str) -> colored::ColoredString {
    match priority {
        Priority::Critical => |s: &str| s.bright_red().bold(),
        Priority::High => |s: &str| s.red(),
        Priority::Medium => |s: &str| s.normal(),
        Priority::Low => |s: &str| s.bright_black(),
    }
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
    
    // Apply priority-based coloring to task description
    let priority_color_fn = get_priority_color(&task.priority);
    let description = if task.status == TaskStatus::Completed {
        priority_color_fn(&task.description).strikethrough().dimmed()
    } else {
        priority_color_fn(&task.description)
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
        // Show priority if not default
        if task.priority != Priority::Medium {
            println!("      {} Priority: {}", 
                get_priority_indicator(&task.priority),
                format!("{}", task.priority).bright_white()
            );
        }
        
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
        
        // Show creation/completion info if available
        if let Some(ref created_at) = task.created_at {
            use chrono::DateTime;
            if let Ok(datetime) = DateTime::parse_from_rfc3339(created_at) {
                println!("      📅 Created: {}", datetime.format("%Y-%m-%d %H:%M").to_string().bright_black());
            }
        }
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
        println!("      Try adjusting your search terms or filters.");
        
        // Provide helpful suggestions
        if total_tasks > 0 {
            println!("\n  💡 Suggestions:");
            println!("      • Use 'rask list' to see all tasks");
            println!("      • Use 'rask list --status all' to include completed tasks");
            println!("      • Try broader search terms with 'rask list --search <keyword>'");
            
            // Show available tags if any
            let all_tags: std::collections::HashSet<String> = roadmap.tasks.iter()
                .flat_map(|t| &t.tags)
                .cloned()
                .collect();
            if !all_tags.is_empty() {
                let tags_sample: Vec<_> = all_tags.iter().take(5).collect();
                println!("      • Available tags: {}", 
                    tags_sample.iter()
                        .map(|t| format!("#{}", t))
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
        }
        println!();
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

/// Display enhanced completion success with dependency unlocking notifications
pub fn display_completion_success_enhanced(
    task_id: usize, 
    task_description: &str, 
    newly_unblocked: &[usize],
    roadmap: &crate::model::Roadmap
) {
    println!("\n✨ {}: Task #{} completed!", 
        "Success".green().bold(), 
        task_id.to_string().bright_white()
    );
    
    println!("   📝 Task: {}", task_description.bright_white());
    println!("   🎊 Well done! Keep up the great work!");
    
    // Show dependency unlocking notifications
    if !newly_unblocked.is_empty() {
        println!("\n🔓 {} unblocked by completing this task:", 
            if newly_unblocked.len() == 1 { "Task" } else { "Tasks" }.bright_green().bold()
        );
        
        for &unblocked_id in newly_unblocked {
            if let Some(unblocked_task) = roadmap.find_task_by_id(unblocked_id) {
                let priority_indicator = get_priority_indicator(&unblocked_task.priority);
                println!("   {} {} #{} {}", 
                    "▶️".bright_green(),
                    priority_indicator,
                    unblocked_id.to_string().bright_cyan(),
                    unblocked_task.description.bright_white()
                );
            }
        }
        
        println!("   💡 {} ready to start!", 
            if newly_unblocked.len() == 1 { "This task is now" } else { "These tasks are now" }.bright_yellow()
        );
    }
    
    println!();
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
    use chrono::DateTime;
    
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

/// Display comprehensive detailed view of a specific task
/// Shows all metadata, dependencies, reverse dependencies, and contextual information
pub fn display_detailed_task_view(task: &crate::model::Task, roadmap: &crate::model::Roadmap) {
    println!("\n{}", "═".repeat(70).bright_blue());
    println!("  {} #{}", "Detailed Task View".bold().bright_cyan(), task.id.to_string().bright_white());
    println!("{}", "═".repeat(70).bright_blue());
    
    // Task status and basic info
    let status_icon = match task.status {
        crate::model::TaskStatus::Completed => "✅".to_string(),
        crate::model::TaskStatus::Pending => "⏳".to_string(),
    };
    
    let priority_icon = get_priority_indicator(&task.priority);
    
    println!("\n  📝 {}: {}", "Description".bold(), task.description.bright_white());
    println!("  📊 {}: {} {}", "Status".bold(), status_icon, 
        match task.status {
            crate::model::TaskStatus::Completed => "Completed".bright_green(),
            crate::model::TaskStatus::Pending => "Pending".bright_yellow(),
        }
    );
    println!("  {} {}: {} {}", priority_icon, "Priority".bold(), 
        format!("{}", task.priority).bright_white(),
        match task.priority {
            crate::model::Priority::Critical => "(Urgent attention required)".bright_red(),
            crate::model::Priority::High => "(Important)".bright_yellow(),
            crate::model::Priority::Medium => "(Normal priority)".normal(),
            crate::model::Priority::Low => "(Can be deferred)".bright_black(),
        }
    );
    
    // Tags
    if !task.tags.is_empty() {
        println!("  🏷️  {}: {}", "Tags".bold(), 
            task.tags.iter()
                .map(|tag| format!("#{}", tag))
                .collect::<Vec<_>>()
                .join(" ")
                .bright_cyan()
        );
    }
    
    // Notes
    if let Some(ref notes) = task.notes {
        println!("  💭 {}:", "Notes".bold());
        // Handle multi-line notes with proper indentation
        for line in notes.lines() {
            println!("      {}", line.italic().bright_black());
        }
    }
    
    // Creation date
    if let Some(ref created_at) = task.created_at {
        use chrono::DateTime;
        if let Ok(datetime) = DateTime::parse_from_rfc3339(created_at) {
            println!("  📅 {}: {}", "Created".bold(), 
                datetime.format("%Y-%m-%d at %H:%M").to_string().bright_black()
            );
        }
    }
    
    println!("\n{}", "─".repeat(70).bright_black());
    
    // Dependencies analysis
    if !task.dependencies.is_empty() {
        println!("  🔗 {} ({}):", "Dependencies".bold().bright_yellow(), task.dependencies.len());
        
        let completed_ids = roadmap.get_completed_task_ids();
        let mut completed_deps = Vec::new();
        let mut pending_deps = Vec::new();
        
        for &dep_id in &task.dependencies {
            if let Some(dep_task) = roadmap.find_task_by_id(dep_id) {
                if completed_ids.contains(&dep_id) {
                    completed_deps.push((dep_id, dep_task));
                } else {
                    pending_deps.push((dep_id, dep_task));
                }
            }
        }
        
        // Show completed dependencies
        if !completed_deps.is_empty() {
            println!("      ✅ {} completed:", "Dependencies".bright_green());
            for (dep_id, dep_task) in completed_deps {
                println!("         #{} {}", dep_id.to_string().bright_green(), dep_task.description.dimmed());
            }
        }
        
        // Show pending dependencies
        if !pending_deps.is_empty() {
            println!("      ⏳ {} pending:", "Dependencies".bright_red());
            for (dep_id, dep_task) in pending_deps {
                let dep_priority_icon = get_priority_indicator(&dep_task.priority);
                println!("         {} #{} {}", dep_priority_icon, dep_id.to_string().bright_red(), dep_task.description);
            }
        }
        
        // Show dependency chain
        let chain = roadmap.get_dependency_chain(task.id);
        if chain.len() > task.dependencies.len() {
            println!("      🔄 {}: {}", "Full dependency chain".bright_black(), 
                chain.iter()
                    .map(|id| format!("#{}", id))
                    .collect::<Vec<_>>()
                    .join(" → ")
                    .bright_black()
            );
        }
    } else {
        println!("  🔗 {}: None", "Dependencies".bold().bright_green());
    }
    
    // Reverse dependencies (tasks that depend on this one)
    let dependents = roadmap.get_dependents(task.id);
    if !dependents.is_empty() {
        println!("  🔄 {} ({}):", "Tasks depending on this".bold().bright_cyan(), dependents.len());
        for &dep_id in &dependents {
            if let Some(dep_task) = roadmap.find_task_by_id(dep_id) {
                let status_icon = match dep_task.status {
                    crate::model::TaskStatus::Completed => "✅",
                    crate::model::TaskStatus::Pending => "⏳",
                };
                let priority_icon = get_priority_indicator(&dep_task.priority);
                println!("      {} {} #{} {}", status_icon, priority_icon, dep_id.to_string().bright_cyan(), dep_task.description);
            }
        }
    } else {
        println!("  🔄 {}: None", "Tasks depending on this".bold().bright_green());
    }
    
    println!("\n{}", "─".repeat(70).bright_black());
    
    // Task readiness analysis
    let completed_ids = roadmap.get_completed_task_ids();
    if task.status == crate::model::TaskStatus::Pending {
        if task.can_be_started(&completed_ids) {
            println!("  🚀 {}: This task is ready to be started!", "Status".bold().bright_green());
            if !task.dependencies.is_empty() {
                println!("      All dependencies have been completed.");
            }
        } else {
            let incomplete_deps: Vec<usize> = task.dependencies.iter()
                .filter(|&&dep_id| !completed_ids.contains(&dep_id))
                .copied()
                .collect();
            println!("  🔒 {}: This task is blocked by {} incomplete dependencies", 
                "Status".bold().bright_red(), incomplete_deps.len());
            println!("      Complete tasks {} first", 
                incomplete_deps.iter()
                    .map(|id| format!("#{}", id))
                    .collect::<Vec<_>>()
                    .join(", ")
                    .bright_yellow()
            );
        }
    } else {
        println!("  ✅ {}: This task has been completed!", "Status".bold().bright_green());
        
        // Show what this completion unlocked
        let unlocked_tasks: Vec<usize> = roadmap.tasks.iter()
            .filter(|t| {
                t.status == crate::model::TaskStatus::Pending &&
                t.dependencies.contains(&task.id) &&
                t.can_be_started(&completed_ids)
            })
            .map(|t| t.id)
            .collect();
        
        if !unlocked_tasks.is_empty() {
            println!("      🔓 Completing this task unlocked: {}", 
                unlocked_tasks.iter()
                    .map(|id| format!("#{}", id))
                    .collect::<Vec<_>>()
                    .join(", ")
                    .bright_green()
            );
        }
    }
    
    // Validation check
    if let Err(errors) = roadmap.validate_task_dependencies(task.id) {
        println!("\n  ⚠️  {}: Found {} issue(s)", "Validation".bold().bright_red(), errors.len());
        for error in &errors {
            println!("      • {}", error.to_string().bright_red());
        }
    }
    
    println!("\n{}", "═".repeat(70).bright_blue());
    println!("  💡 Use {} to see the dependency tree", format!("rask dependencies --task-id {}", task.id).bright_cyan());
    if task.status == crate::model::TaskStatus::Pending && task.can_be_started(&completed_ids) {
        println!("  💡 Use {} to complete this task", format!("rask complete {}", task.id).bright_cyan());
    }
    println!();
} 