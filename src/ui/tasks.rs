use crate::model::{Priority, Task, TaskStatus};
use crate::ui::helpers::{get_priority_indicator, get_priority_color};
use colored::*;

/// Display a single task line with enhanced formatting
pub fn display_task_line(task: &Task, detailed: bool) {
    let status_icon = if task.status == TaskStatus::Completed { "✓" } else { "□" };
    let status_color = if task.status == TaskStatus::Completed { 
        status_icon.green() 
    } else { 
        status_icon.bright_black() 
    };
    
    // AI task indicator - show special icon for AI-generated tasks
    let ai_indicator = if task.is_ai_generated() {
        "🤖".bright_cyan()
    } else {
        "  ".normal()
    };
    
    // Apply priority-based coloring to task description
    let priority_color_fn = get_priority_color(&task.priority);
    let mut description = if task.status == TaskStatus::Completed {
        priority_color_fn(&task.description).strikethrough().dimmed()
    } else {
        priority_color_fn(&task.description)
    };
    
    // Special coloring for AI-generated tasks (cyan tint when not completed)
    if task.is_ai_generated() && task.status != TaskStatus::Completed {
        description = description.bright_cyan();
    }
    
    // Format the main task line with consistent spacing
    // In detailed mode, we don't show priority icon here since it's shown in details below
    // In non-detailed mode, we show the priority icon for quick reference
    if detailed {
        // Detailed view: no priority icon in main line (shown in details below)
        print!("  {} {} #{:2} {}", 
            status_color,       // Status checkbox (✓ or □)
            ai_indicator,       // AI indicator (🤖 or spaces)
            task.id,           // Task ID with consistent 2-digit padding
            description        // Task description with priority/AI coloring
        );
    } else {
        // List view: show priority icon for quick scanning
        let priority_indicator = get_priority_indicator(&task.priority);
        print!("  {} {} {} #{:2} {}", 
            status_color,           // Status checkbox (✓ or □)
            ai_indicator,           // AI indicator (🤖 or spaces)
            priority_indicator,     // Priority emoji (🔥, ⬆️, ▶️, ⬇️)
            task.id,               // Task ID with consistent 2-digit padding
            description            // Task description with priority/AI coloring
        );
    }
    
    // Add tags if present, with consistent spacing
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
        // Always show priority in detailed view since we removed it from the main line
        println!("       {} Priority: {}", 
            get_priority_indicator(&task.priority),
            format!("{}", task.priority).bright_white()
        );
        
        // Show AI information if available
        if task.is_ai_generated() {
            if let Some(operation) = task.get_ai_operation() {
                println!("       🤖 AI Generated: {} operation", operation.bright_cyan());
            }
            if let Some(reasoning) = task.get_ai_reasoning() {
                println!("       💡 AI Suggestion: {}", reasoning.bright_blue().italic());
            }
        }
        
        if let Some(ref notes) = task.notes {
            println!("       💭 {}", notes.italic().bright_black());
        }
        
        if !task.dependencies.is_empty() {
            let deps_str = task.dependencies.iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            println!("       🔗 Depends on: {}", deps_str.bright_yellow());
        }
        
        // Show creation/completion info if available
        if let Some(ref created_at) = task.created_at {
            use chrono::DateTime;
            if let Ok(datetime) = DateTime::parse_from_rfc3339(created_at) {
                println!("       📅 Created: {}", datetime.format("%Y-%m-%d %H:%M").to_string().bright_black());
            }
        }
    }
}

/// Display filtered tasks with optional detailed view
pub fn display_filtered_tasks(roadmap: &crate::model::Roadmap, filtered_tasks: &[&Task], detailed: bool) {
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
    
    println!("    📝 Task: {}", task.description.bright_white());
    println!("    🆔 Assigned ID: {}", task.id.to_string().bright_cyan());
    
    // Show priority if not default
    if task.priority != Priority::Medium {
        println!("    {} Priority: {}", 
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
        println!("    🏷️  Tags: {}", tags_str);
    }
    
    // Show notes if present
    if let Some(ref notes) = task.notes {
        println!("    💭 Notes: {}", notes.italic().bright_black());
    }
    
    // Show dependencies if present
    if !task.dependencies.is_empty() {
        let deps_str = task.dependencies.iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        println!("    🔗 Dependencies: {}", deps_str.bright_yellow());
    }
    
    println!("    💡 Task added to both state and markdown file!\n");
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
    
    // AI Information - prominently displayed for AI-generated tasks
    if task.is_ai_generated() {
        println!("\n{}", "─".repeat(40).bright_cyan());
        println!("  🤖 {} {}", "AI Generated Task".bold().bright_cyan(), "🤖".bright_cyan());
        println!("{}", "─".repeat(40).bright_cyan());
        
        if let Some(operation) = task.get_ai_operation() {
            println!("  🔧 {}: {} operation", "AI Source".bold(), operation.bright_cyan());
        }
        
        if let Some(reasoning) = task.get_ai_reasoning() {
            println!("  💡 {}:", "AI Analysis & Suggestions".bold().bright_blue());
            // Handle multi-line AI reasoning with proper indentation
            for line in reasoning.lines() {
                if line.trim().is_empty() {
                    println!();
                } else {
                    println!("      {}", line.bright_blue().italic());
                }
            }
        }
        
        if let Some(ai_timestamp) = &task.ai_info.ai_timestamp {
            use chrono::DateTime;
            if let Ok(datetime) = DateTime::parse_from_rfc3339(ai_timestamp) {
                println!("  🕒 {}: {}", "AI Generated".bold(), 
                    datetime.format("%Y-%m-%d at %H:%M").to_string().bright_black()
                );
            }
        }
        
        if let Some(model) = &task.ai_info.ai_model {
            println!("  🧠 {}: {}", "AI Model".bold(), model.bright_magenta());
        }
        
        println!("{}", "─".repeat(40).bright_cyan());
    }
    
        // Notes
    if let Some(ref notes) = task.notes {
        println!("  💭 {}:", "Notes".bold());
        // Handle multi-line notes with proper indentation
        for line in notes.lines() {
            println!("      {}", line.italic().bright_black());
        }
    }

    // Implementation Notes
    if !task.implementation_notes.is_empty() {
        println!("  🔧 {} ({}):", "Implementation Notes".bold().bright_blue(), task.implementation_notes.len());
        for (index, note) in task.implementation_notes.iter().enumerate() {
            println!("      {} {}:", format!("#{}", index).bright_white().bold(), "Note".bright_blue());
            // Handle multi-line implementation notes with proper indentation
            for line in note.lines() {
                if line.trim().is_empty() {
                    println!();
                } else {
                    println!("        {}", line.bright_cyan());
                }
            }
            if index < task.implementation_notes.len() - 1 {
                println!(); // Add spacing between notes
            }
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