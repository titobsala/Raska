use crate::model::Task;
use crate::ui::helpers::get_priority_indicator;
use colored::*;

/// Display dependency error with helpful information
pub fn display_dependency_error(task_id: usize, incomplete_deps: &[usize], roadmap: &crate::model::Roadmap) {
    println!("\n🚫 {}: Cannot complete task #{}", "Dependency Error".red().bold(), task_id);
    
    if let Some(task) = roadmap.find_task_by_id(task_id) {
        println!("   📝 Task: {}", task.description.bright_white());
    }
    
    println!("   🔗 Missing dependencies:");
    for &dep_id in incomplete_deps {
        if let Some(dep_task) = roadmap.find_task_by_id(dep_id) {
            println!("      #{} {}", 
                dep_id.to_string().bright_red(), 
                dep_task.description.dimmed()
            );
        }
    }
    
    println!("\n   💡 Complete the missing dependencies first, then try again.");
}

/// Display dependency validation errors
pub fn display_dependency_validation_errors(errors: &[crate::model::DependencyError]) {
    println!("\n🚫 {}: Found {} dependency issue(s)", 
        "Validation Failed".red().bold(), 
        errors.len().to_string().bright_white()
    );
    
    for (i, error) in errors.iter().enumerate() {
        println!("   {}. {}", (i + 1).to_string().bright_red(), error);
    }
    
    println!("\n   💡 Fix these issues before proceeding.");
}

/// Display comprehensive dependency overview
pub fn display_dependency_overview(roadmap: &crate::model::Roadmap) {
    println!("\n{}", "═".repeat(60).bright_blue());
    println!("  {}", "Dependency Analysis Overview".bold().bright_cyan());
    println!("{}", "═".repeat(60).bright_blue());
    
    let ready_tasks = roadmap.get_ready_tasks();
    let blocked_tasks = roadmap.get_blocked_tasks();
    let total_tasks = roadmap.tasks.len();
    let tasks_with_deps = roadmap.tasks.iter().filter(|t| !t.dependencies.is_empty()).count();
    
    println!("\n  📊 {}:", "Statistics".bold());
    println!("      Total tasks: {}", total_tasks.to_string().bright_white());
    println!("      Tasks with dependencies: {}", tasks_with_deps.to_string().bright_white());
    println!("      Ready to start: {}", ready_tasks.len().to_string().bright_green());
    println!("      Blocked by dependencies: {}", blocked_tasks.len().to_string().bright_red());
    
    // Show validation status
    match roadmap.validate_all_dependencies() {
        Ok(()) => {
            println!("      Validation status: {}", "✓ All dependencies valid".bright_green());
        }
        Err(errors) => {
            println!("      Validation status: {} ({} issues)", 
                "✗ Issues found".bright_red(),
                errors.len().to_string().bright_red()
            );
        }
    }
    
    println!("\n  💡 Use {} to see specific analysis", "rask dependencies --help".bright_cyan());
    println!("  💡 Use {} to validate all dependencies", "rask dependencies --validate".bright_cyan());
    println!("  💡 Use {} to see ready tasks", "rask dependencies --ready".bright_cyan());
    println!();
}

/// Display dependency tree for a specific task
pub fn display_dependency_tree(tree: &crate::model::DependencyNode, roadmap: &crate::model::Roadmap) {
    println!("\n{}", "═".repeat(60).bright_blue());
    println!("  {} #{}", "Dependency Tree for Task".bold().bright_cyan(), tree.task_id.to_string().bright_white());
    println!("{}", "═".repeat(60).bright_blue());
    
    display_dependency_node(tree, 0, true);
    
    // Show dependency chain
    let chain = roadmap.get_dependency_chain(tree.task_id);
    if !chain.is_empty() {
        println!("\n  📋 {}:", "Full Dependency Chain".bold());
        let chain_str = chain.iter()
            .map(|id| format!("#{}", id))
            .collect::<Vec<_>>()
            .join(" → ");
        println!("      {}", chain_str.bright_yellow());
    }
    
    // Show reverse dependencies (tasks that depend on this one)
    let dependents = roadmap.get_dependents(tree.task_id);
    if !dependents.is_empty() {
        println!("\n  🔄 {}:", "Tasks depending on this".bold());
        for &dep_id in &dependents {
            if let Some(task) = roadmap.find_task_by_id(dep_id) {
                println!("      #{} {}", dep_id.to_string().bright_cyan(), task.description.dimmed());
            }
        }
    }
    
    println!();
}

fn display_dependency_node(node: &crate::model::DependencyNode, depth: usize, is_last: bool) {
    let indent = "  ".repeat(depth);
    let prefix = if depth == 0 {
        "  📝"
    } else if is_last {
        "  └─"
    } else {
        "  ├─"
    };
    
    let status_icon = match node.status {
        crate::model::TaskStatus::Completed => "✓".green(),
        crate::model::TaskStatus::Pending => "□".bright_black(),
    };
    
    let task_desc = if node.is_circular {
        node.description.red().italic()
    } else {
        match node.status {
            crate::model::TaskStatus::Completed => node.description.dimmed().strikethrough(),
            crate::model::TaskStatus::Pending => node.description.normal(),
        }
    };
    
    println!("{}{} {} #{} {}", 
        indent, prefix, status_icon, 
        node.task_id.to_string().bright_white(), 
        task_desc
    );
    
    for (i, dep) in node.dependencies.iter().enumerate() {
        let is_last_dep = i == node.dependencies.len() - 1;
        display_dependency_node(dep, depth + 1, is_last_dep);
    }
}

/// Display tasks ready to be started
pub fn display_ready_tasks(ready_tasks: &[&Task]) {
    println!("\n{}", "═".repeat(60).bright_blue());
    println!("  {} ({})", 
        "Tasks Ready to Start".bold().bright_green(),
        ready_tasks.len().to_string().bright_white()
    );
    println!("{}", "═".repeat(60).bright_blue());
    
    if ready_tasks.is_empty() {
        println!("\n  🎯 No tasks are currently ready to start.");
        println!("      All pending tasks are blocked by dependencies.");
    } else {
        println!("\n  🚀 These tasks have all dependencies completed:");
        for task in ready_tasks {
            let priority_icon = get_priority_indicator(&task.priority);
            println!("      {} {} #{} {}", 
                priority_icon,
                "□".bright_green(),
                task.id.to_string().bright_white(),
                task.description
            );
        }
    }
    
    println!();
}

/// Display tasks blocked by dependencies
pub fn display_blocked_tasks(blocked_tasks: &[&Task], roadmap: &crate::model::Roadmap) {
    println!("\n{}", "═".repeat(60).bright_blue());
    println!("  {} ({})", 
        "Tasks Blocked by Dependencies".bold().bright_red(),
        blocked_tasks.len().to_string().bright_white()
    );
    println!("{}", "═".repeat(60).bright_blue());
    
    if blocked_tasks.is_empty() {
        println!("\n  ✨ No tasks are currently blocked!");
        println!("      All pending tasks are ready to start.");
    } else {
        let completed_ids = roadmap.get_completed_task_ids();
        
        println!("\n  🚫 These tasks are waiting for dependencies:");
        for task in blocked_tasks {
            let priority_icon = get_priority_indicator(&task.priority);
            let incomplete_deps: Vec<usize> = task.dependencies.iter()
                .filter(|&&dep_id| !completed_ids.contains(&dep_id))
                .copied()
                .collect();
            
            println!("      {} {} #{} {}", 
                priority_icon,
                "□".bright_red(),
                task.id.to_string().bright_white(),
                task.description
            );
            
            if !incomplete_deps.is_empty() {
                println!("        🔗 Waiting for: {}", 
                    incomplete_deps.iter()
                        .map(|id| format!("#{}", id))
                        .collect::<Vec<_>>()
                        .join(", ")
                        .bright_yellow()
                );
            }
        }
    }
    
    println!();
}