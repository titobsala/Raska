use crate::model::{Task};
use crate::{state};
use super::{CommandResult, utils};
use colored::*;
use std::io::{self, Write};

/// Add an implementation note to a task
pub fn add_implementation_note(
    task_id: usize,
    note: String,
) -> CommandResult {
    let mut roadmap = state::load_state()?;
    
    // Find the task
    let task = roadmap.find_task_by_id_mut(task_id)
        .ok_or_else(|| format!("Task with ID {} not found", task_id))?;
    
    // Add the implementation note
    task.add_implementation_note(note.clone());
    let note_count = task.implementation_notes.len();
    let task_description = task.description.clone();
    
    // Save the roadmap
    utils::save_and_sync(&roadmap)?;
    
    // Display success message
    println!("{}", "✅ Implementation note added successfully!".green());
    println!("📝 Task #{}: {}", task_id, task_description);
    println!("💡 Added note: {}", note.bright_blue());
    println!("📊 Total implementation notes: {}", note_count);
    
    Ok(())
}

/// List all implementation notes for a task
pub fn list_implementation_notes(
    task_id: usize,
) -> CommandResult {
    let roadmap = state::load_state()?;
    
    // Find the task
    let task = roadmap.find_task_by_id(task_id)
        .ok_or_else(|| format!("Task with ID {} not found", task_id))?;
    
    // Display task information
    println!("\n{}", "📝 Implementation Notes".bright_cyan().bold());
    println!("{}", "═".repeat(50).bright_cyan());
    println!("📋 Task #{}: {}", task_id, task.description.bright_white().bold());
    
    if task.implementation_notes.is_empty() {
        println!("\n{}", "💡 No implementation notes found for this task.".yellow());
        println!("{}", "   Use 'rask notes add <task_id> <note>' to add implementation notes.".dimmed());
        return Ok(());
    }
    
    println!("\n📊 {} implementation note(s):", task.implementation_notes.len());
    println!("{}", "─".repeat(50).bright_black());
    
    for (index, note) in task.implementation_notes.iter().enumerate() {
        println!("\n{} {}:", "📌".bright_blue(), format!("Note #{}", index).bright_white().bold());
        
        // Format multi-line notes nicely
        for line in note.lines() {
            if line.trim().is_empty() {
                println!();
            } else {
                println!("   {}", line);
            }
        }
    }
    
    println!("\n{}", "─".repeat(50).bright_black());
    println!("{}", format!("💡 Use 'rask notes edit {} <index> <new_note>' to edit a note", task_id).dimmed());
    println!("{}", format!("💡 Use 'rask notes remove {} <index>' to remove a note", task_id).dimmed());
    
    Ok(())
}

/// Remove an implementation note from a task
pub fn remove_implementation_note(
    task_id: usize,
    index: usize,
) -> CommandResult {
    let mut roadmap = state::load_state()?;
    
    // Find the task
    let task = roadmap.find_task_by_id_mut(task_id)
        .ok_or_else(|| format!("Task with ID {} not found", task_id))?;
    
    // Check if index is valid
    if index >= task.implementation_notes.len() {
        return Err(format!(
            "Invalid note index {}. Task has {} implementation notes (indices 0-{})",
            index,
            task.implementation_notes.len(),
            task.implementation_notes.len().saturating_sub(1)
        ).into());
    }
    
    // Remove the note
    let removed_note = task.remove_implementation_note(index)
        .ok_or("Failed to remove implementation note")?;
    let task_description = task.description.clone();
    let remaining_count = task.implementation_notes.len();
    
    // Save the roadmap
    utils::save_and_sync(&roadmap)?;
    
    // Display success message
    println!("{}", "✅ Implementation note removed successfully!".green());
    println!("📝 Task #{}: {}", task_id, task_description);
    println!("🗑️  Removed note #{}: {}", index, removed_note.bright_red());
    println!("📊 Remaining implementation notes: {}", remaining_count);
    
    Ok(())
}

/// Clear all implementation notes from a task
pub fn clear_implementation_notes(
    task_id: usize,
) -> CommandResult {
    let mut roadmap = state::load_state()?;
    
    // Find the task
    let task = roadmap.find_task_by_id_mut(task_id)
        .ok_or_else(|| format!("Task with ID {} not found", task_id))?;
    
    let note_count = task.implementation_notes.len();
    
    if note_count == 0 {
        println!("{}", "💡 No implementation notes to clear for this task.".yellow());
        return Ok(());
    }
    
    // Confirm before clearing
    print!("⚠️  Are you sure you want to clear all {} implementation notes from task #{}? (y/N): ", 
           note_count, task_id);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    if !input.trim().to_lowercase().starts_with('y') {
        println!("{}", "❌ Operation cancelled.".yellow());
        return Ok(());
    }
    
    // Clear all notes
    task.clear_implementation_notes();
    let task_description = task.description.clone();
    
    // Save the roadmap
    utils::save_and_sync(&roadmap)?;
    
    // Display success message
    println!("{}", "✅ All implementation notes cleared successfully!".green());
    println!("📝 Task #{}: {}", task_id, task_description);
    println!("🗑️  Cleared {} implementation notes", note_count);
    
    Ok(())
}

/// Edit an implementation note
pub fn edit_implementation_note(
    task_id: usize,
    index: usize,
    new_note: String,
) -> CommandResult {
    let mut roadmap = state::load_state()?;
    
    // Find the task
    let task = roadmap.find_task_by_id_mut(task_id)
        .ok_or_else(|| format!("Task with ID {} not found", task_id))?;
    
    // Check if index is valid
    if index >= task.implementation_notes.len() {
        return Err(format!(
            "Invalid note index {}. Task has {} implementation notes (indices 0-{})",
            index,
            task.implementation_notes.len(),
            task.implementation_notes.len().saturating_sub(1)
        ).into());
    }
    
    // Store old note for display
    let old_note = task.implementation_notes[index].clone();
    
    // Update the note
    task.implementation_notes[index] = new_note.clone();
    let task_description = task.description.clone();
    
    // Save the roadmap
    utils::save_and_sync(&roadmap)?;
    
    // Display success message
    println!("{}", "✅ Implementation note updated successfully!".green());
    println!("📝 Task #{}: {}", task_id, task_description);
    println!("📝 Note #{} updated:", index);
    println!("   {}: {}", "Old".bright_red(), old_note.bright_red());
    println!("   {}: {}", "New".bright_green(), new_note.bright_green());
    
    Ok(())
}

/// Show implementation notes in task view (helper function for other modules)
pub fn display_implementation_notes_for_task(task: &Task) {
    if !task.implementation_notes.is_empty() {
        println!("\n{} {} implementation note(s):", 
                 "🔧".bright_blue(), 
                 task.implementation_notes.len());
        
        for (index, note) in task.implementation_notes.iter().enumerate() {
            println!("   {} {}: {}", 
                     "📌".bright_blue(),
                     format!("#{}", index).bright_white().bold(),
                     note.lines().next().unwrap_or("").bright_cyan());
            
            // Show additional lines if it's a multi-line note
            let lines: Vec<&str> = note.lines().collect();
            if lines.len() > 1 {
                for line in lines.iter().skip(1).take(2) { // Show up to 2 more lines
                    println!("      {}", line.dimmed());
                }
                if lines.len() > 3 {
                    println!("      {} ({} more lines)", "...".dimmed(), lines.len() - 3);
                }
            }
        }
    }
} 