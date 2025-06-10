use crate::model::{Roadmap, TaskStatus};
use colored::*;

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
    display_progress_bar(completed_tasks, total_tasks, progress_percentage);
    
    // Print tasks section
    display_tasks_section(&roadmap.tasks);
    
    // Print footer with motivational message
    display_footer(completed_tasks, total_tasks);
}

/// Displays a progress bar with completion statistics
fn display_progress_bar(completed: usize, total: usize, percentage: usize) {
    let bar_width = 40;
    let filled = (completed * bar_width) / total.max(1);
    let empty = bar_width - filled;
    
    print!("  Progress: [");
    print!("{}", "█".repeat(filled).green());
    print!("{}", "░".repeat(empty).dimmed());
    println!("] {}% ({}/{})", percentage, completed, total);
}

/// Displays the tasks section with proper formatting
fn display_tasks_section(tasks: &[crate::model::Task]) {
    println!("\n  {} Tasks:", "📋".to_string().bold());
    println!("  {}", "─".repeat(50).dimmed());
    
    for task in tasks {
        match task.status {
            TaskStatus::Pending => {
                println!("  {} {} {}", 
                    "□".bright_white(), 
                    format!("#{:2}", task.id).dimmed(), 
                    task.description.white()
                );
            }
            TaskStatus::Completed => {
                println!("  {} {} {}", 
                    "✓".bright_green().bold(), 
                    format!("#{:2}", task.id).strikethrough().dimmed(), 
                    task.description.strikethrough().dimmed()
                );
            }
        }
    }
}

/// Displays footer with motivational messages
fn display_footer(completed: usize, total: usize) {
    println!("  {}", "─".repeat(50).dimmed());
    if completed == total && total > 0 {
        println!("  {} All tasks completed! Great job! {}", "🎉".to_string(), "🎉".to_string());
    } else if completed > 0 {
        println!("  {} Keep going! {} tasks remaining.", "💪".to_string(), total - completed);
    } else {
        println!("  {} Ready to start? Complete your first task!", "🚀".to_string());
    }
    println!();
}

/// Displays success message for project initialization
pub fn display_init_success(roadmap: &Roadmap) {
    println!("\n{} {} Project initialized successfully!", "🎯".to_string(), "Success:".green().bold());
    println!("   📝 Project: {}", roadmap.title.bright_cyan());
    println!("   📊 Total tasks: {}", roadmap.tasks.len().to_string().bright_yellow());
    println!("   💾 State saved to: {}", ".rask_state.json".dimmed());
    println!("\n   💡 Use {} to view your tasks!", "rask show".bright_green());
}

/// Displays success message for task completion
pub fn display_completion_success(task_id: usize) {
    println!("\n{} {} Task #{} completed!", "✨".to_string(), "Success:".green().bold(), task_id);
    println!("   🎊 Well done! Keep up the great work!\n");
}

/// Displays error messages with consistent formatting
pub fn display_error(message: &str) {
    eprintln!("{} {}", "Error:".red().bold(), message);
} 