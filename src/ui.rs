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