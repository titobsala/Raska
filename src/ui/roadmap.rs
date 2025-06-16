use crate::model::{Priority, Roadmap, TaskStatus, Phase};
use crate::ui::progress::{display_progress_bar, display_motivational_message};
use crate::ui::tasks::display_task_line;
use colored::*;
use std::collections::HashMap;

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

/// Display roadmap grouped by phases for better organization
pub fn display_roadmap_grouped_by_phase(roadmap: &Roadmap, detailed: bool, collapse_completed: bool) {
    let total_tasks = roadmap.tasks.len();
    let completed_tasks = roadmap.tasks.iter().filter(|t| t.status == TaskStatus::Completed).count();
    
    // Print header
    println!("\n{}", "═".repeat(80).bright_blue());
    println!("  {} - {} tasks across phases", roadmap.title.bold().bright_cyan(), total_tasks);
    
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
    
    println!("{}", "═".repeat(80).bright_blue());
    
    // Overall progress bar
    display_progress_bar(completed_tasks, total_tasks);
    
    // Group tasks by phase
    let mut phase_groups: HashMap<String, Vec<&crate::model::Task>> = HashMap::new();
    for task in &roadmap.tasks {
        let phase_name = task.phase.name.clone();
        phase_groups.entry(phase_name).or_insert_with(Vec::new).push(task);
    }
    
    // Get all phases from roadmap in proper order (predefined first, then custom alphabetically)
    let all_phases = roadmap.get_all_phases();
    
    // Display phases in order
    for phase in &all_phases {
        if let Some(tasks) = phase_groups.get(&phase.name) {
            display_phase_section(&phase.name, &phase.emoji(), tasks, detailed, collapse_completed);
        }
    }
    
    println!("\n  💡 {} Use 'rask show --phase <name>' to focus on a specific phase", "Tip:".bright_green().bold());
    println!("     Use 'rask timeline' for a horizontal phase view");
    println!();
}

/// Display roadmap filtered by a specific phase
pub fn display_roadmap_filtered_by_phase(roadmap: &Roadmap, phase_filter: &str, detailed: bool) {
    let filtered_tasks: Vec<&crate::model::Task> = roadmap.tasks.iter()
        .filter(|t| t.phase.name.to_lowercase() == phase_filter.to_lowercase())
        .collect();
    
    if filtered_tasks.is_empty() {
        println!("\n  {} No tasks found in phase '{}'", "ℹ️".bright_blue(), phase_filter.bright_yellow());
        println!("  Use 'rask phase list' to see available phases");
        return;
    }
    
    let completed_tasks = filtered_tasks.iter().filter(|t| t.status == TaskStatus::Completed).count();
    let total_tasks = filtered_tasks.len();
    
    // Print header
    println!("\n{}", "═".repeat(80).bright_blue());
    println!("  {} - {} Phase", roadmap.title.bold().bright_cyan(), phase_filter.bright_yellow().bold());
    println!("  📊 {} tasks in this phase", total_tasks);
    println!("{}", "═".repeat(80).bright_blue());
    
    // Phase-specific progress bar
    display_progress_bar(completed_tasks, total_tasks);
    
    // Find the phase emoji from actual roadmap phases
    let phase_emoji = if let Some(phase) = roadmap.get_all_phases().iter().find(|p| p.name.to_lowercase() == phase_filter.to_lowercase()) {
        phase.emoji()
    } else {
        "📋".to_string()
    };
    
    println!("\n  {} {} Phase Tasks:", phase_emoji, phase_filter.bright_yellow().bold());
    println!("  {}", "─".repeat(50).bright_black());
    
    // Display tasks
    for task in &filtered_tasks {
        display_task_line(task, detailed);
    }
    
    println!("  {}", "─".repeat(50).bright_black());
    
    // Phase-specific statistics
    let ready_tasks = filtered_tasks.iter()
        .filter(|t| t.status == TaskStatus::Pending && t.can_be_started(&roadmap.get_completed_task_ids()))
        .count();
    let blocked_tasks = total_tasks - completed_tasks - ready_tasks;
    
    println!("\n  📊 {} Phase Statistics:", phase_filter.bright_yellow().bold());
    println!("     ✅ Completed: {}", completed_tasks.to_string().bright_green());
    println!("     🚀 Ready to start: {}", ready_tasks.to_string().bright_cyan());
    if blocked_tasks > 0 {
        println!("     🔒 Blocked: {}", blocked_tasks.to_string().bright_red());
    }
    
    println!("\n  💡 {} Use 'rask show --group-by-phase' to see all phases", "Tip:".bright_green().bold());
    println!();
}

/// Display project timeline with horizontal phase layout
pub fn display_project_timeline(roadmap: &Roadmap, detailed: bool, active_only: bool, compact: bool) {
    let total_tasks = roadmap.tasks.len();
    let completed_tasks = roadmap.tasks.iter().filter(|t| t.status == TaskStatus::Completed).count();
    
    // Print header
    println!("\n{}", "═".repeat(100).bright_blue());
    println!("  📅 {} Project Timeline", roadmap.title.bold().bright_cyan());
    println!("{}", "═".repeat(100).bright_blue());
    
    // Overall progress
    println!("  📈 Overall Progress: [{}] {}% ({}/{})", 
        create_progress_bar(completed_tasks, total_tasks, 30),
        if total_tasks > 0 { (completed_tasks * 100) / total_tasks } else { 0 },
        completed_tasks,
        total_tasks
    );
    
    // Group tasks by phase
    let mut phase_groups: HashMap<String, Vec<&crate::model::Task>> = HashMap::new();
    for task in &roadmap.tasks {
        let phase_name = task.phase.name.clone();
        phase_groups.entry(phase_name).or_insert_with(Vec::new).push(task);
    }
    
    // Get actual phases from roadmap instead of hardcoded predefined phases
    let all_phases = roadmap.get_all_phases();
    let phases_to_show: Vec<&Phase> = if active_only {
        all_phases.iter().filter(|p| phase_groups.contains_key(&p.name)).collect()
    } else {
        all_phases.iter().collect()
    };
    
    if phases_to_show.is_empty() {
        println!("\n  {} No active phases found", "ℹ️".bright_blue());
        return;
    }
    
    println!("\n");
    
    // Display phase headers
    for (i, phase) in phases_to_show.iter().enumerate() {
        let empty_vec = vec![];
        let tasks = phase_groups.get(&phase.name).unwrap_or(&empty_vec);
        let phase_completed = tasks.iter().filter(|t| t.status == TaskStatus::Completed).count();
        let phase_total = tasks.len();
        let percentage = if phase_total > 0 { (phase_completed * 100) / phase_total } else { 0 };
        
        print!("  {} {} ", phase.emoji(), phase.name.bright_yellow().bold());
        if compact {
            print!("({})", phase_total);
        } else {
            print!("({} tasks)", phase_total);
        }
        
        if i < phases_to_show.len() - 1 {
            print!("  →  ");
        }
    }
    println!("\n");
    
    // Display phase progress bars
    for (i, phase) in phases_to_show.iter().enumerate() {
        let empty_vec = vec![];
        let tasks = phase_groups.get(&phase.name).unwrap_or(&empty_vec);
        let phase_completed = tasks.iter().filter(|t| t.status == TaskStatus::Completed).count();
        let phase_total = tasks.len();
        let percentage = if phase_total > 0 { (phase_completed * 100) / phase_total } else { 0 };
        
        print!("  {}% [{}]", 
            format!("{:3}", percentage).bright_white(),
            create_progress_bar(phase_completed, phase_total, 12)
        );
        
        if i < phases_to_show.len() - 1 {
            print!("     ");
        }
    }
    println!("\n");
    
    // Display task boxes
    let max_tasks_to_show = if compact { 3 } else { 5 };
    
    for row in 0..max_tasks_to_show {
        for (i, phase) in phases_to_show.iter().enumerate() {
            let empty_vec = vec![];
            let tasks = phase_groups.get(&phase.name).unwrap_or(&empty_vec);
            
            if row < tasks.len() {
                let task = tasks[row];
                let status_icon = if task.status == TaskStatus::Completed { "✓" } else { "□" };
                let priority_icon = match task.priority {
                    Priority::Critical => "🔴",
                    Priority::High => "⬆️",
                    Priority::Medium => "▶️",
                    Priority::Low => "⬇️",
                };
                
                if compact {
                    print!("  {} {} #{}", status_icon, priority_icon, task.id);
                } else {
                    let desc = if task.description.len() > 12 {
                        format!("{}...", &task.description[..9])
                    } else {
                        task.description.clone()
                    };
                    print!("  {} {} #{} {}", status_icon, priority_icon, task.id, desc);
                }
            } else if row == max_tasks_to_show - 1 && tasks.len() > max_tasks_to_show {
                let remaining = tasks.len() - max_tasks_to_show + 1;
                print!("  ... {} more", remaining);
            } else {
                print!("  {}", " ".repeat(if compact { 8 } else { 20 }));
            }
            
            if i < phases_to_show.len() - 1 {
                print!("  │  ");
            }
        }
        println!();
    }
    
    // Dependencies flow - show actual phases
    if phases_to_show.len() > 1 {
        print!("\n  🔗 Dependencies: ");
        for (i, phase) in phases_to_show.iter().enumerate() {
            let color = match i % 4 {
                0 => phase.name.bright_cyan(),
                1 => phase.name.bright_yellow(),
                2 => phase.name.bright_green(),
                _ => phase.name.bright_magenta(),
            };
            print!("{}", color);
            if i < phases_to_show.len() - 1 {
                print!(" → ");
            }
        }
        println!();
    }
    
    // Ready tasks summary
    let ready_tasks = roadmap.tasks.iter()
        .filter(|t| t.status == TaskStatus::Pending && t.can_be_started(&roadmap.get_completed_task_ids()))
        .count();
    
    println!("  🚀 Ready to start: {} tasks", ready_tasks.to_string().bright_green().bold());
    
    println!("\n  💡 {} Use 'rask show --group-by-phase' for detailed phase view", "Tip:".bright_green().bold());
    println!("     Use 'rask show --phase <name>' to focus on specific phase");
    println!();
}

/// Helper function to display a phase section
fn display_phase_section(phase_name: &str, emoji: &str, tasks: &[&crate::model::Task], detailed: bool, collapse_completed: bool) {
    let completed_tasks = tasks.iter().filter(|t| t.status == TaskStatus::Completed).count();
    let total_tasks = tasks.len();
    let percentage = if total_tasks > 0 { (completed_tasks * 100) / total_tasks } else { 0 };
    
    // Check if phase is completed and should be collapsed
    let is_completed = percentage == 100;
    let should_collapse = collapse_completed && is_completed;
    
    println!("\n  {} {} Phase - {} ({} tasks, {}% complete)", 
        emoji, 
        phase_name.bright_yellow().bold(),
        if is_completed { "Complete".bright_green() } else { "In Progress".bright_cyan() },
        total_tasks,
        percentage
    );
    
    // Phase progress bar
    print!("  Progress: [{}] {}%", create_progress_bar(completed_tasks, total_tasks, 20), percentage);
    if should_collapse {
        println!(" {}", "(collapsed - all tasks completed)".dimmed());
        return;
    }
    println!();
    
    println!("  {}", "─".repeat(60).bright_black());
    
    // Show tasks (limit to first few if not detailed)
    let tasks_to_show = if detailed { tasks.len() } else { std::cmp::min(tasks.len(), 5) };
    
    for (i, task) in tasks.iter().take(tasks_to_show).enumerate() {
        display_task_line(task, detailed);
    }
    
    // Show "and X more" if there are more tasks
    if tasks.len() > tasks_to_show {
        let remaining = tasks.len() - tasks_to_show;
        println!("  {} ... and {} more tasks", "  ".dimmed(), remaining.to_string().bright_blue());
    }
    
    // Phase statistics
    let ready_tasks = tasks.iter()
        .filter(|t| t.status == TaskStatus::Pending && {
            // We need access to roadmap for dependency checking, but for now just show pending
            t.dependencies.is_empty() || t.status == TaskStatus::Completed
        })
        .count();
    
    if !is_completed {
        println!("  📊 {} ready to start", ready_tasks.to_string().bright_green());
    }
}

/// Helper function to create a progress bar string
fn create_progress_bar(completed: usize, total: usize, width: usize) -> String {
    if total == 0 {
        return "░".repeat(width).dimmed().to_string();
    }
    
    let filled = (completed * width) / total;
    let empty = width - filled;
    
    format!("{}{}", 
        "█".repeat(filled).bright_green(),
        "░".repeat(empty).dimmed()
    )
}