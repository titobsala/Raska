//! Phase management commands for Rask
//! 
//! This module provides functionality for managing task phases including
//! listing phases, showing tasks by phase, setting task phases, creating custom phases,
//! and displaying phase overviews.

use crate::model::{Phase, Roadmap};
use crate::state;
use crate::ui;
use super::CommandResult;

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
    let predefined_phases = Phase::predefined_phases();
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