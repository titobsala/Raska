//! Phase management commands for Rask
//! 
//! This module provides functionality for managing task phases including
//! listing phases, showing tasks by phase, setting task phases, creating custom phases,
//! and displaying phase overviews.

use crate::model::{Phase};
use crate::state;
use crate::ui;
use super::CommandResult;
use colored::Colorize;

/// List all phases with their task counts
pub fn list_phases() -> CommandResult {
    let roadmap = state::load_state()?;
    let stats = roadmap.get_statistics();
    
    ui::display_info("📊 Project Phases Overview");
    println!();
    
    if stats.tasks_by_phase.is_empty() {
        println!("  No phases found. Create tasks with phases or add custom phases.");
        println!();
        println!("  💡 Tip: Use predefined phases like 'mvp', 'beta', 'release', 'future', 'backlog'");
        println!("      or create custom phases with: rask phase create <name>");
        return Ok(());
    }
    
    for (phase, count) in &stats.tasks_by_phase {
        let emoji = phase.emoji();
        let description = phase.description();
        let phase_type = if phase.is_predefined() { "predefined" } else { "custom" };
        
        println!("  {} {} - {} tasks ({})", emoji, phase, count, phase_type);
        println!("    {}", description);
        println!();
    }
    
    Ok(())
}

/// Show all tasks in a specific phase
pub fn show_phase_tasks(phase_name: &str) -> CommandResult {
    let roadmap = state::load_state()?;
    let phase = Phase::from_string(phase_name);
    let tasks = roadmap.filter_by_phase(&phase);
    
    if tasks.is_empty() {
        ui::display_info(&format!("No tasks found in '{}' phase", phase));
        println!();
        println!("💡 Available phases:");
        let stats = roadmap.get_statistics();
        for (existing_phase, count) in &stats.tasks_by_phase {
            if *count > 0 {
                println!("  {} {} ({} tasks)", existing_phase.emoji(), existing_phase, count);
            }
        }
        return Ok(());
    }
    
    ui::display_info(&format!("{} {} Phase Tasks ({} tasks)", phase.emoji(), phase, tasks.len()));
    println!("  {}", phase.description());
    println!();
    
    ui::display_filtered_tasks(&roadmap, &tasks, false);
    
    Ok(())
}

/// Set the phase for a specific task
pub fn set_task_phase(task_id: usize, phase_name: &str) -> CommandResult {
    let mut roadmap = state::load_state()?;
    let phase = Phase::from_string(phase_name);
    
    if let Some(task) = roadmap.find_task_by_id_mut(task_id) {
        let old_phase = task.phase.clone();
        task.phase = phase.clone();
        
        state::save_state(&roadmap)?;
        
        ui::display_success(&format!(
            "Task #{} phase updated from {} {} to {} {}", 
            task_id, old_phase.emoji(), old_phase, phase.emoji(), phase
        ));
    } else {
        ui::display_error(&format!("Task #{} not found", task_id));
    }
    
    Ok(())
}

/// Create a new custom phase
pub fn create_custom_phase(name: &str, description: Option<&str>, emoji: Option<&str>) -> CommandResult {
    // Validate phase name
    if name.trim().is_empty() {
        ui::display_error("Phase name cannot be empty");
        return Ok(());
    }
    
    let phase_name = name.trim().to_string();
    
    // Check if it's a predefined phase
    let normalized = phase_name.to_lowercase();
    if matches!(normalized.as_str(), "mvp" | "beta" | "release" | "future" | "backlog") {
        ui::display_info(&format!("'{}' is a predefined phase - no need to create it", phase_name));
        return Ok(());
    }
    
    // Create the phase
    let phase = Phase::with_details(
        phase_name.clone(),
        description.map(|s| s.to_string()),
        emoji.map(|s| s.to_string()),
    );
    
    ui::display_success(&format!(
        "Custom phase created: {} {} - {}", 
        phase.emoji(), 
        phase.name,
        phase.description()
    ));
    
    println!();
    println!("💡 You can now use this phase when adding or updating tasks:");
    println!("   rask add \"My task\" --phase \"{}\"", phase_name);
    println!("   rask phase set <task_id> \"{}\"", phase_name);
    
    Ok(())
}

/// Show comprehensive phase overview with statistics and progress
pub fn show_phase_overview() -> CommandResult {
    let roadmap = state::load_state()?;
    let stats = roadmap.get_statistics();
    
    ui::display_info("🎯 Project Phase Overview");
    println!();
    
    // Overall project statistics
    println!("📈 Overall Progress:");
    println!("  Total Tasks: {}", stats.total_tasks);
    println!("  Completed: {} ({}%)", stats.completed_tasks, stats.completion_percentage);
    println!("  Pending: {}", stats.pending_tasks);
    println!();
    
    if stats.tasks_by_phase.is_empty() {
        println!("📊 No phases found.");
        println!();
        println!("💡 Get started:");
        println!("  • Add tasks with predefined phases: rask add \"My task\" --phase mvp");
        println!("  • Create custom phases: rask phase create \"Phase 1\" --description \"First iteration\"");
        return Ok(());
    }
    
    // Phase breakdown
    println!("📊 Phase Breakdown:");
    for (phase, count) in &stats.tasks_by_phase {
        if *count > 0 {
            let phase_tasks = roadmap.filter_by_phase(phase);
            let completed_in_phase = phase_tasks.iter()
                .filter(|t| t.status == crate::model::TaskStatus::Completed)
                .count();
            let completion_rate = if *count > 0 { (completed_in_phase * 100) / count } else { 0 };
            
            let phase_type = if phase.is_predefined() { "" } else { " (custom)" };
            println!("  {} {} ({} tasks, {}% complete){}", 
                phase.emoji(), phase, count, completion_rate, phase_type);
            
            // Show ready tasks in this phase
            let ready_tasks: Vec<_> = phase_tasks.iter()
                .filter(|t| t.status == crate::model::TaskStatus::Pending && 
                          t.can_be_started(&roadmap.get_completed_task_ids()))
                .collect();
            
            if !ready_tasks.is_empty() {
                println!("    ✅ {} tasks ready to start", ready_tasks.len());
            }
            
            // Show blocked tasks in this phase
            let blocked_tasks: Vec<_> = phase_tasks.iter()
                .filter(|t| t.status == crate::model::TaskStatus::Pending && 
                          !t.can_be_started(&roadmap.get_completed_task_ids()))
                .collect();
            
            if !blocked_tasks.is_empty() {
                println!("    ⏸️  {} tasks blocked by dependencies", blocked_tasks.len());
            }
            
            println!();
        }
    }
    
    // Phase recommendations
    println!("💡 Recommendations:");
    
    // Find the phase with the most ready tasks
    let mut phase_ready_counts = Vec::new();
    for (phase, _) in &stats.tasks_by_phase {
        let phase_tasks = roadmap.filter_by_phase(phase);
        let ready_count = phase_tasks.iter()
            .filter(|t| t.status == crate::model::TaskStatus::Pending && 
                      t.can_be_started(&roadmap.get_completed_task_ids()))
            .count();
        if ready_count > 0 {
            phase_ready_counts.push((phase, ready_count));
        }
    }
    
    phase_ready_counts.sort_by(|a, b| b.1.cmp(&a.1));
    
    if let Some((top_phase, count)) = phase_ready_counts.first() {
        println!("  • Focus on {} {} phase - {} tasks ready to start", top_phase.emoji(), top_phase, count);
    }
    
    // Suggest predefined phases if none are being used
    let _predefined_phases = Phase::predefined_phases();
    let used_predefined: Vec<_> = stats.tasks_by_phase.iter()
        .filter(|(phase, count)| phase.is_predefined() && *count > 0)
        .collect();
    
    if used_predefined.is_empty() && !stats.tasks_by_phase.is_empty() {
        println!("  • Consider using predefined phases for better organization:");
        println!("    🚀 MVP, 🧪 Beta, 🎯 Release, 🔮 Future, 💡 Backlog");
    }
    
    println!("  • Create custom phases: rask phase create \"<name>\" --description \"<desc>\" --emoji \"<emoji>\"");
    
    Ok(())
}

/// Fork (duplicate) tasks from a phase or specific tasks into a new phase
pub fn fork_phase_or_tasks(
    new_phase_name: &str,
    from_phase: Option<&str>,
    task_ids: Option<&str>,
    description: Option<&str>,
    emoji: Option<&str>,
    copy: bool,
) -> CommandResult {
    let mut roadmap = state::load_state()?;
    
    // Validate inputs
    if from_phase.is_none() && task_ids.is_none() {
        ui::display_error("Must specify either --from-phase or --task-ids");
        return Ok(());
    }
    
    if from_phase.is_some() && task_ids.is_some() {
        ui::display_error("Cannot specify both --from-phase and --task-ids. Choose one.");
        return Ok(());
    }
    
    if new_phase_name.trim().is_empty() {
        ui::display_error("New phase name cannot be empty");
        return Ok(());
    }
    
    // Create the new phase
    let new_phase = Phase::with_details(
        new_phase_name.trim().to_string(),
        description.map(|s| s.to_string()),
        emoji.map(|s| s.to_string()),
    );
    
    let mut tasks_to_fork = Vec::new();
    let operation = if copy { "copied" } else { "moved" };
    
    // Get tasks to fork
    if let Some(source_phase_name) = from_phase {
        // Fork entire phase
        let source_phase = Phase::from_string(source_phase_name);
        let phase_tasks = roadmap.filter_by_phase(&source_phase);
        
        if phase_tasks.is_empty() {
            ui::display_warning(&format!("No tasks found in phase '{}'", source_phase_name));
            return Ok(());
        }
        
        for task in phase_tasks {
            tasks_to_fork.push(task.id);
        }
        
        ui::display_info(&format!(
            "🍴 Forking {} tasks from {} {} to {} {}",
            tasks_to_fork.len(),
            source_phase.emoji(),
            source_phase.name,
            new_phase.emoji(),
            new_phase.name
        ));
    } else if let Some(task_ids_str) = task_ids {
        // Fork specific tasks
        let task_ids: Result<Vec<usize>, _> = task_ids_str
            .split(',')
            .map(|s| s.trim().parse::<usize>())
            .collect();
        
        let task_ids = match task_ids {
            Ok(ids) => ids,
            Err(_) => {
                ui::display_error("Invalid task IDs. Use comma-separated numbers: 1,2,3");
                return Ok(());
            }
        };
        
        // Validate all task IDs exist
        for &task_id in &task_ids {
            if roadmap.find_task_by_id(task_id).is_none() {
                ui::display_error(&format!("Task #{} not found", task_id));
                return Ok(());
            }
        }
        
        tasks_to_fork = task_ids;
        ui::display_info(&format!(
            "🍴 Forking {} specific tasks to {} {}",
            tasks_to_fork.len(),
            new_phase.emoji(),
            new_phase.name
        ));
    }
    
    // Fork the tasks
    let mut forked_count = 0;
    let next_id = roadmap.get_next_task_id();
    
    for (i, &task_id) in tasks_to_fork.iter().enumerate() {
        if let Some(original_task) = roadmap.find_task_by_id(task_id) {
            if copy {
                // Create a copy of the task with new ID and phase
                let mut new_task = original_task.clone();
                new_task.id = next_id + i;
                new_task.phase = new_phase.clone();
                
                // Reset some fields for the copy
                new_task.status = crate::model::TaskStatus::Pending;
                new_task.completed_at = None;
                new_task.actual_hours = None;
                new_task.time_sessions = Vec::new();
                new_task.created_at = Some(chrono::Utc::now().to_rfc3339());
                
                // Clear dependencies to avoid conflicts (user can re-add if needed)
                new_task.dependencies = Vec::new();
                
                roadmap.add_task(new_task);
                forked_count += 1;
            } else {
                // Move the task to the new phase
                if let Some(task) = roadmap.find_task_by_id_mut(task_id) {
                    let old_phase = task.phase.clone();
                    task.phase = new_phase.clone();
                    
                    println!("   {} Task #{} {} from {} {} to {} {}", 
                        "✅".bright_green(),
                        task_id,
                        operation,
                        old_phase.emoji(),
                        old_phase.name,
                        new_phase.emoji(),
                        new_phase.name
                    );
                    forked_count += 1;
                }
            }
        }
    }
    
    // Save the updated roadmap
    state::save_state(&roadmap)?;
    
    // Show summary
    ui::display_success(&format!(
        "🎉 Successfully {} {} tasks to {} {} phase!",
        operation,
        forked_count,
        new_phase.emoji(),
        new_phase.name
    ));
    
    if copy {
        println!();
        println!("💡 Copied tasks have:");
        println!("   • New task IDs (#{} - #{})", next_id, next_id + forked_count - 1);
        println!("   • Reset to Pending status");
        println!("   • Cleared dependencies (re-add if needed)");
        println!("   • Cleared time tracking data");
    }
    
    println!();
    println!("🔍 View the new phase: rask phase show \"{}\"", new_phase.name);
    println!("📊 Phase overview: rask phase overview");
    
    Ok(())
} 