//! Phase management commands for Rask
//! 
//! This module provides functionality for managing task phases including
//! listing phases, showing tasks by phase, setting task phases, and displaying phase overviews.

use crate::cli::{CliPhase, PhaseCommands};
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
    
    for (phase, count) in &stats.tasks_by_phase {
        let emoji = phase.emoji();
        let description = phase.description();
        println!("  {} {} - {} tasks", emoji, phase, count);
        println!("    {}", description);
        println!();
    }
    
    Ok(())
}

/// Show all tasks in a specific phase
pub fn show_phase_tasks(cli_phase: &CliPhase) -> CommandResult {
    let roadmap = state::load_state()?;
    let phase: Phase = cli_phase.clone().into();
    let tasks = roadmap.filter_by_phase(&phase);
    
    if tasks.is_empty() {
        ui::display_info(&format!("No tasks found in {} phase", phase));
        return Ok(());
    }
    
    ui::display_info(&format!("{} {} Phase Tasks ({} tasks)", phase.emoji(), phase, tasks.len()));
    println!("  {}", phase.description());
    println!();
    
    ui::display_filtered_tasks(&roadmap, &tasks, false);
    
    Ok(())
}

/// Set the phase for a specific task
pub fn set_task_phase(task_id: usize, cli_phase: &CliPhase) -> CommandResult {
    let mut roadmap = state::load_state()?;
    let phase: Phase = cli_phase.clone().into();
    
    if let Some(task) = roadmap.find_task_by_id_mut(task_id) {
        let old_phase = task.phase.clone();
        task.phase = phase.clone();
        
        state::save_state(&roadmap)?;
        
        ui::display_success(&format!(
            "Task #{} phase updated from {} to {}", 
            task_id, old_phase, phase
        ));
    } else {
        ui::display_error(&format!("Task #{} not found", task_id));
    }
    
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
    
    // Phase breakdown
    println!("📊 Phase Breakdown:");
    for (phase, count) in &stats.tasks_by_phase {
        if *count > 0 {
            let phase_tasks = roadmap.filter_by_phase(phase);
            let completed_in_phase = phase_tasks.iter()
                .filter(|t| t.status == crate::model::TaskStatus::Completed)
                .count();
            let completion_rate = if *count > 0 { (completed_in_phase * 100) / count } else { 0 };
            
            println!("  {} {} ({} tasks, {}% complete)", 
                phase.emoji(), phase, count, completion_rate);
            
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
        println!("  • Focus on {} phase - {} tasks ready to start", top_phase, count);
    }
    
    // Check for phases with no tasks
    let all_phases = Phase::all();
    let empty_phases: Vec<_> = all_phases.iter()
        .filter(|phase| !stats.tasks_by_phase.iter().any(|(p, count)| p == *phase && *count > 0))
        .collect();
    
    if !empty_phases.is_empty() {
        println!("  • Consider adding tasks to: {}", 
            empty_phases.iter()
                .map(|p| format!("{} {}", p.emoji(), p))
                .collect::<Vec<_>>()
                .join(", "));
    }
    
    Ok(())
} 