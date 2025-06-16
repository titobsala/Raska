use colored::*;

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

/// Display success message for project initialization
pub fn display_init_success(roadmap: &crate::model::Roadmap) {
    println!("\n🎯 {}: Project initialized successfully!", "Success".green().bold());
    println!("   📝 Project: {}", roadmap.title.bright_cyan());
    println!("   📊 Total tasks: {}", roadmap.tasks.len().to_string().bright_white());
    println!("   💾 State saved to: {}", ".rask_state.json".bright_yellow());
    println!("\n   💡 Use {} to view your tasks!", "rask show".bright_cyan());
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