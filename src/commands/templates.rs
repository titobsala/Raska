use crate::{
    cli::{TemplateCommands, CliPriority},
    model::{TaskTemplate, TemplateCollection, TemplateCategory, Priority, Phase},
    state,
};
use std::path::Path;
use std::fs;
use colored::*;

/// Handle template commands
pub fn handle_template_command(cmd: TemplateCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        TemplateCommands::List { category, detailed } => {
            list_templates(category.as_deref(), detailed)
        }
        TemplateCommands::Show { name } => {
            show_template(&name)
        }
        TemplateCommands::Use { template_name, description, add_tags, priority, phase } => {
            use_template(&template_name, description, add_tags, priority, phase)
        }
        TemplateCommands::Create { name, description, tags, priority, phase, notes, category } => {
            create_template(name, description, tags, priority, phase, notes, category)
        }
        TemplateCommands::Delete { name, force } => {
            delete_template(&name, force)
        }
        TemplateCommands::Export { output, pretty } => {
            export_templates(&output, pretty)
        }
        TemplateCommands::Import { input, merge } => {
            import_templates(&input, merge)
        }
        TemplateCommands::Examples => {
            show_template_help()
        }
    }
}

/// List all available templates
fn list_templates(category_filter: Option<&str>, detailed: bool) -> Result<(), Box<dyn std::error::Error>> {
    let templates = load_templates()?;
    
    println!("{}", "═".repeat(80).bright_cyan());
    println!("  📋 {} Available Task Templates", "Rask".bright_cyan().bold());
    println!("{}", "═".repeat(80).bright_cyan());
    
    // Filter by category if specified
    let filtered_templates: Vec<&TaskTemplate> = if let Some(cat_filter) = category_filter {
        templates.templates.iter()
            .filter(|t| t.category.to_string().to_lowercase().contains(&cat_filter.to_lowercase()))
            .collect()
    } else {
        templates.templates.iter().collect()
    };
    
    if filtered_templates.is_empty() {
        println!("  {} No templates found", "ℹ️".bright_blue());
        if let Some(cat) = category_filter {
            println!("     Try without the category filter '{}'", cat.yellow());
        }
        return Ok(());
    }
    
    // Group by category
    let mut categories: std::collections::HashMap<String, Vec<&TaskTemplate>> = std::collections::HashMap::new();
    for template in &filtered_templates {
        categories.entry(template.category.to_string())
            .or_insert_with(Vec::new)
            .push(template);
    }
    
    for (category, templates_in_cat) in categories {
        println!("\n  📁 {} Category", category.bright_yellow().bold());
        println!("  {}", "─".repeat(50).dimmed());
        
        for template in templates_in_cat {
            let priority_icon = match template.priority {
                Priority::Critical => "🔴",
                Priority::High => "⬆️",
                Priority::Medium => "▶️",
                Priority::Low => "⬇️",
            };
            
            println!("  {} {} {}", 
                priority_icon,
                template.name.bright_white().bold(),
                format!("({})", template.phase.name).dimmed()
            );
            
            if detailed {
                println!("     📝 {}", template.description.dimmed());
                if !template.tags.is_empty() {
                    let tags: Vec<String> = template.tags.iter()
                        .map(|t| format!("#{}", t))
                        .collect();
                    println!("     🏷️  {}", tags.join(" ").bright_blue());
                }
                if let Some(notes) = &template.notes {
                    let first_line = notes.lines().next().unwrap_or("");
                    println!("     💡 {}", first_line.dimmed());
                }
                println!();
            }
        }
    }
    
    println!("\n  💡 {} Use 'rask template use <name>' to create a task from a template", "Tip:".bright_green().bold());
    println!("     Use 'rask template show <name>' to see full template details");
    
    Ok(())
}

/// Show detailed information about a specific template
fn show_template(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let templates = load_templates()?;
    
    if let Some(template) = templates.find_template(name) {
        println!("{}", "═".repeat(80).bright_cyan());
        println!("  📋 Template: {}", template.name.bright_white().bold());
        println!("{}", "═".repeat(80).bright_cyan());
        
        let priority_icon = match template.priority {
            Priority::Critical => "🔴",
            Priority::High => "⬆️", 
            Priority::Medium => "▶️",
            Priority::Low => "⬇️",
        };
        
        println!("  📝 Description: {}", template.description);
        println!("  📁 Category:    {}", template.category.to_string().bright_yellow());
        println!("  {} Priority:     {}", priority_icon, template.priority.to_string().bright_white());
        println!("  🚀 Phase:       {} {}", 
            template.phase.emoji(),
            template.phase.name.bright_cyan()
        );
        
        if !template.tags.is_empty() {
            let tags: Vec<String> = template.tags.iter()
                .map(|t| format!("#{}", t))
                .collect();
            println!("  🏷️  Tags:        {}", tags.join(" ").bright_blue());
        }
        
        if let Some(notes) = &template.notes {
            println!("\n  💡 Notes:");
            for line in notes.lines() {
                println!("     {}", line.dimmed());
            }
        }
        
        if !template.implementation_notes.is_empty() {
            println!("\n  🔧 Implementation Notes:");
            for (i, note) in template.implementation_notes.iter().enumerate() {
                println!("     {}. {}", i + 1, note.dimmed());
            }
        }
        
        println!("\n  📅 Created: {}", template.created_at.dimmed());
        
        println!("\n  💡 {} To use this template:", "Usage:".bright_green().bold());
        println!("     rask template use \"{}\"", template.name);
        println!("     rask template use \"{}\" \"Custom task description\"", template.name);
        
    } else {
        println!("  {} Template '{}' not found", "❌".bright_red(), name.bright_white());
        println!("  Use 'rask template list' to see available templates");
    }
    
    Ok(())
}

/// Create a new task from a template
fn use_template(
    template_name: &str, 
    custom_description: Option<String>,
    add_tags: Option<String>,
    priority_override: Option<CliPriority>,
    phase_override: Option<String>
) -> Result<(), Box<dyn std::error::Error>> {
    let templates = load_templates()?;
    let mut roadmap = state::load_state()?;
    
    if let Some(template) = templates.find_template(template_name) {
        let task_id = roadmap.get_next_task_id();
        let mut task = template.create_task(task_id, custom_description);
        
        // Apply overrides
        if let Some(priority) = priority_override {
            task.priority = priority.into();
        }
        
        if let Some(phase_str) = phase_override {
            task.phase = Phase::from_string(&phase_str);
        }
        
        // Add additional tags
        if let Some(tags_str) = add_tags {
            let additional_tags: Vec<String> = tags_str
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            
            for tag in additional_tags {
                task.tags.insert(tag);
            }
        }
        
        roadmap.add_task(task.clone());
        state::save_state(&roadmap)?;
        
        println!("  {} Task created from template '{}'", "✅".bright_green(), template_name.bright_white());
        println!("     ID: {}", task.id.to_string().bright_cyan().bold());
        println!("     Description: {}", task.description);
        
        let priority_icon = match task.priority {
            Priority::Critical => "🔴",
            Priority::High => "⬆️",
            Priority::Medium => "▶️", 
            Priority::Low => "⬇️",
        };
        println!("     {} Priority: {}", priority_icon, task.priority.to_string());
        println!("     🚀 Phase: {} {}", task.phase.emoji(), task.phase.name);
        
        if !task.tags.is_empty() {
            let tags: Vec<String> = task.tags.iter()
                .map(|t| format!("#{}", t))
                .collect();
            println!("     🏷️  Tags: {}", tags.join(" ").bright_blue());
        }
        
    } else {
        println!("  {} Template '{}' not found", "❌".bright_red(), template_name.bright_white());
        println!("  Use 'rask template list' to see available templates");
        return Err("Template not found".into());
    }
    
    Ok(())
}

/// Create a new custom template
fn create_template(
    name: String,
    description: String,
    tags: Option<String>,
    priority: Option<CliPriority>,
    phase: Option<String>,
    notes: Option<String>,
    category: Option<String>
) -> Result<(), Box<dyn std::error::Error>> {
    let mut templates = load_templates()?;
    
    // Check if template already exists
    if templates.find_template(&name).is_some() {
        println!("  {} Template '{}' already exists", "❌".bright_red(), name.bright_white());
        println!("  Use 'rask template delete \"{}\"' to remove it first", name);
        return Err("Template already exists".into());
    }
    
    let mut template = TaskTemplate::new(name.clone(), description);
    
    // Set tags
    if let Some(tags_str) = tags {
        let tag_list: Vec<String> = tags_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        template.tags = tag_list.into_iter().collect();
    }
    
    // Set priority
    if let Some(prio) = priority {
        template.priority = prio.into();
    }
    
    // Set phase
    if let Some(phase_str) = phase {
        template.phase = Phase::from_string(&phase_str);
    }
    
    // Set notes
    template.notes = notes;
    
    // Set category
    if let Some(cat_str) = category {
        template.category = match cat_str.to_lowercase().as_str() {
            "development" => TemplateCategory::Development,
            "testing" => TemplateCategory::Testing,
            "documentation" => TemplateCategory::Documentation,
            "devops" => TemplateCategory::DevOps,
            "design" => TemplateCategory::Design,
            "research" => TemplateCategory::Research,
            "meeting" => TemplateCategory::Meeting,
            "bug" => TemplateCategory::Bug,
            "feature" => TemplateCategory::Feature,
            _ => TemplateCategory::Custom(cat_str),
        };
    }
    
    templates.add_template(template);
    save_templates(&templates)?;
    
    println!("  {} Template '{}' created successfully", "✅".bright_green(), name.bright_white());
    println!("  Use 'rask template use \"{}\"' to create tasks from this template", name);
    
    Ok(())
}

/// Delete a custom template
fn delete_template(name: &str, force: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut templates = load_templates()?;
    
    if let Some(template) = templates.find_template(name) {
        // Check if it's a predefined template
        let predefined_names: Vec<String> = TaskTemplate::predefined_templates()
            .iter()
            .map(|t| t.name.clone())
            .collect();
        
        if predefined_names.contains(&template.name) {
            println!("  {} Cannot delete predefined template '{}'", "❌".bright_red(), name.bright_white());
            return Err("Cannot delete predefined template".into());
        }
        
        if !force {
            println!("  {} Are you sure you want to delete template '{}'? (y/N)", "⚠️".bright_yellow(), name.bright_white());
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            if !input.trim().to_lowercase().starts_with('y') {
                println!("  Template deletion cancelled");
                return Ok(());
            }
        }
        
        templates.remove_template(name);
        save_templates(&templates)?;
        
        println!("  {} Template '{}' deleted successfully", "✅".bright_green(), name.bright_white());
        
    } else {
        println!("  {} Template '{}' not found", "❌".bright_red(), name.bright_white());
    }
    
    Ok(())
}

/// Export templates to a file
fn export_templates(output: &Path, pretty: bool) -> Result<(), Box<dyn std::error::Error>> {
    let templates = load_templates()?;
    
    let json_content = if pretty {
        serde_json::to_string_pretty(&templates)?
    } else {
        serde_json::to_string(&templates)?
    };
    
    fs::write(output, json_content)?;
    
    println!("  {} Templates exported to '{}'", "✅".bright_green(), output.display().to_string().bright_white());
    println!("     {} templates exported", templates.templates.len());
    
    Ok(())
}

/// Import templates from a file
fn import_templates(input: &Path, merge: bool) -> Result<(), Box<dyn std::error::Error>> {
    if !input.exists() {
        println!("  {} File '{}' not found", "❌".bright_red(), input.display().to_string().bright_white());
        return Err("Input file not found".into());
    }
    
    let content = fs::read_to_string(input)?;
    let imported_templates: TemplateCollection = serde_json::from_str(&content)?;
    
    let mut current_templates = if merge {
        load_templates()?
    } else {
        TemplateCollection::new()
    };
    
    let mut imported_count = 0;
    let mut skipped_count = 0;
    
    for template in imported_templates.templates {
        if current_templates.find_template(&template.name).is_some() {
            if merge {
                println!("  {} Skipping existing template '{}'", "⚠️".bright_yellow(), template.name);
                skipped_count += 1;
                continue;
            }
        }
        
        current_templates.add_template(template);
        imported_count += 1;
    }
    
    save_templates(&current_templates)?;
    
    println!("  {} Templates imported from '{}'", "✅".bright_green(), input.display().to_string().bright_white());
    println!("     {} templates imported", imported_count);
    if skipped_count > 0 {
        println!("     {} templates skipped (already exist)", skipped_count);
    }
    
    Ok(())
}

/// Show template help and examples
fn show_template_help() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "═".repeat(80).bright_cyan());
    println!("  📋 {} Task Templates Help & Examples", "Rask".bright_cyan().bold());
    println!("{}", "═".repeat(80).bright_cyan());
    
    println!("\n  {} What are Task Templates?", "🤔".bright_blue().bold());
    println!("     Task templates are pre-configured task patterns that help you quickly");
    println!("     create consistent tasks with predefined tags, priorities, phases, and notes.");
    
    println!("\n  {} Common Commands:", "🚀".bright_green().bold());
    println!("     rask template list                    # List all templates");
    println!("     rask template list --detailed         # List with full details");
    println!("     rask template show \"Bug Fix\"          # Show template details");
    println!("     rask template use \"Feature Implementation\" \"Add user login\"");
    println!("     rask template create \"My Template\" \"Custom task description\"");
    
    println!("\n  {} Template Categories:", "📁".bright_yellow().bold());
    println!("     • Development  - General development tasks");
    println!("     • Testing      - Testing and QA tasks");
    println!("     • Documentation- Documentation tasks");
    println!("     • DevOps       - Infrastructure and deployment");
    println!("     • Design       - UI/UX and design tasks");
    println!("     • Research     - Research and analysis");
    println!("     • Bug          - Bug fixes and issues");
    println!("     • Feature      - New feature development");
    
    println!("\n  {} Creating Custom Templates:", "✨".bright_magenta().bold());
    println!("     rask template create \"Code Review\" \\");
    println!("       \"Review pull request for [PR_NAME]\" \\");
    println!("       --tags \"review,quality\" \\");
    println!("       --priority high \\");
    println!("       --phase mvp \\");
    println!("       --category development \\");
    println!("       --notes \"Check: code style, tests, documentation\"");
    
    println!("\n  {} AI Integration Example:", "🤖".bright_purple().bold());
    println!("     You can ask AI assistants to generate roadmaps using this template:");
    println!();
    println!("     {}", "\"Create a development roadmap for a web application with the following".dimmed());
    println!("     {}", "structure. Each task should include appropriate tags, priorities, phases,".dimmed());
    println!("     {}", "and implementation notes:".dimmed());
    println!();
    println!("     {}", "# Project: [PROJECT_NAME]".dimmed());
    println!();
    println!("     {}", "## MVP Phase".dimmed());
    println!("     {}", "- [ ] Task description #tag1 #tag2 (Priority: High)".dimmed());
    println!("     {}", "  Notes: Implementation details and considerations".dimmed());
    println!();
    println!("     {}", "## Beta Phase".dimmed());
    println!("     {}", "- [ ] Another task #testing (Priority: Medium)".dimmed());
    println!("     {}", "  Notes: Testing requirements and acceptance criteria".dimmed());
    println!();
    println!("     {}", "IMPORTANT: Avoid using list formatting in Notes sections.".dimmed());
    println!("     {}", "Use plain text descriptions instead of bullet points or".dimmed());
    println!("     {}", "numbered lists to prevent parsing issues.".dimmed());
    println!();
    println!("     {}", "Please format it as a markdown file that I can use with 'rask init'\"".dimmed());
    
    println!("\n  {} Pro Tips:", "💡".bright_green().bold());
    println!("     • Use placeholders like [FEATURE_NAME] in template descriptions");
    println!("     • Templates inherit all properties but can be overridden when used");
    println!("     • Export/import templates to share with your team");
    println!("     • Use detailed implementation notes for complex templates");
    
    Ok(())
}

/// Load templates from file or create default collection
fn load_templates() -> Result<TemplateCollection, Box<dyn std::error::Error>> {
    let templates_path = get_templates_path()?;
    
    if templates_path.exists() {
        let content = fs::read_to_string(&templates_path)?;
        let templates: TemplateCollection = serde_json::from_str(&content)?;
        Ok(templates)
    } else {
        // Create default templates file
        let templates = TemplateCollection::default();
        save_templates(&templates)?;
        Ok(templates)
    }
}

/// Save templates to file
fn save_templates(templates: &TemplateCollection) -> Result<(), Box<dyn std::error::Error>> {
    let templates_path = get_templates_path()?;
    
    // Ensure directory exists
    if let Some(parent) = templates_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    let content = serde_json::to_string_pretty(templates)?;
    fs::write(&templates_path, content)?;
    
    Ok(())
}

/// Get the path to the templates file
fn get_templates_path() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir()
        .ok_or("Could not find config directory")?;
    
    let rask_dir = config_dir.join("rask");
    Ok(rask_dir.join("templates.json"))
} 