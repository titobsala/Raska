use colored::*;

/// Display list of projects
pub fn display_projects_list(projects_config: &crate::project::ProjectsConfig, current_project: Option<&str>) {
    use chrono::DateTime;
    
    println!("\n{}", "═".repeat(60).bright_blue());
    println!("  {} ({})", 
        "Available Projects".bold().bright_cyan(),
        projects_config.projects.len().to_string().bright_white()
    );
    println!("{}", "═".repeat(60).bright_blue());
    
    // Sort projects by last accessed time (most recent first)
    let mut sorted_projects: Vec<_> = projects_config.projects.iter().collect();
    sorted_projects.sort_by(|a, b| {
        let time_a = DateTime::parse_from_rfc3339(&a.1.last_accessed)
            .unwrap_or_else(|_| DateTime::parse_from_rfc3339("1970-01-01T00:00:00Z").unwrap());
        let time_b = DateTime::parse_from_rfc3339(&b.1.last_accessed)
            .unwrap_or_else(|_| DateTime::parse_from_rfc3339("1970-01-01T00:00:00Z").unwrap());
        time_b.cmp(&time_a)
    });
    
    for (name, config) in sorted_projects {
        let is_current = current_project == Some(name);
        let is_default = projects_config.default_project.as_ref() == Some(name);
        
        // Format project name with indicators
        let mut project_name = if is_current {
            format!("👉 {}", name.bright_cyan().bold())
        } else {
            name.bright_white().to_string()
        };
        
        if is_default {
            project_name = format!("{} {}", project_name, "(default)".bright_green());
        }
        
        println!("\n  📁 {}", project_name);
        
        // Show description if available
        if let Some(ref description) = config.description {
            println!("     📝 {}", description.italic().bright_black());
        }
        
        // Show creation date
        if let Ok(created_time) = DateTime::parse_from_rfc3339(&config.created_at) {
            let created_local = created_time.with_timezone(&chrono::Local);
            println!("     📅 Created: {}", created_local.format("%Y-%m-%d %H:%M").to_string().bright_black());
        }
        
        // Show last accessed if not current
        if !is_current {
            if let Ok(accessed_time) = DateTime::parse_from_rfc3339(&config.last_accessed) {
                let accessed_local = accessed_time.with_timezone(&chrono::Local);
                println!("     🕒 Last accessed: {}", accessed_local.format("%Y-%m-%d %H:%M").to_string().bright_black());
            }
        }
        
        // Show state file path
        println!("     💾 State file: {}", config.state_file.bright_yellow());
        
        // Show source file if available
        if let Some(ref source_file) = config.source_file {
            println!("     📄 Source: {}", source_file.bright_yellow());
        }
    }
    
    println!("\n  💡 Use {} to switch projects", "rask project switch <name>".bright_cyan());
    println!("  💡 Use {} to create a new project", "rask project create <name>".bright_cyan());
    println!();
}