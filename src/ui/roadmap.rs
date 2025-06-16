use crate::model::{Priority, Roadmap, TaskStatus};
use crate::ui::progress::{display_progress_bar, display_motivational_message};
use crate::ui::tasks::display_task_line;
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
    println!("       📈 Progress: {}/{} completed ({:.1}%)", 
        completed_tasks, total_tasks, 
        if total_tasks > 0 { (completed_tasks as f64 / total_tasks as f64) * 100.0 } else { 0.0 }
    );
    
    if pending_tasks > 0 {
        println!("       🎯 Priority Breakdown:");
        if critical_tasks > 0 { println!("          🔥 Critical: {}", critical_tasks.to_string().bright_red()); }
        if high_tasks > 0 { println!("          ⬆️  High: {}", high_tasks.to_string().red()); }
        if medium_tasks > 0 { println!("          ▶️  Medium: {}", medium_tasks.to_string().yellow()); }
        if low_tasks > 0 { println!("          ⬇️  Low: {}", low_tasks.to_string().green()); }
        
        println!("       🚀 Task Status:");
        println!("          ✅ Ready to start: {}", ready_tasks.to_string().bright_green());
        if blocked_tasks > 0 {
            println!("          🔒 Blocked by dependencies: {}", blocked_tasks.to_string().bright_red());
        }
    }
}