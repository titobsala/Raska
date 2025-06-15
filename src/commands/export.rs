//! Export functionality commands
//! 
//! This module handles exporting roadmaps to different formats including
//! JSON, CSV, and HTML with filtering and formatting options.

use crate::{
    cli::CliPriority,
    model::{TaskStatus, Priority, Phase, Task, Roadmap},
    state,
    ui
};
use super::{CommandResult, utils, ExportFormat};
use std::fs;
use std::path::Path;

/// Export roadmap to different formats (JSON, CSV, HTML)
pub fn export_roadmap(
    format: &ExportFormat,
    output_path: Option<&Path>,
    include_completed: bool,
    tags_filter: Option<&str>,
    priority_filter: Option<&CliPriority>,
    phase_filter: Option<&String>,
    pretty: bool,
) -> CommandResult {
    let roadmap = state::load_state()?;
    
    // Apply filters to get the tasks to export
    let mut tasks_to_export: Vec<&Task> = roadmap.tasks.iter().collect();
    
    // Filter by completion status
    if !include_completed {
        tasks_to_export.retain(|task| task.status != TaskStatus::Completed);
    }
    
    // Filter by tags if specified
    if let Some(tags_str) = tags_filter {
        let filter_tags: Vec<String> = tags_str.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        
        if !filter_tags.is_empty() {
            tasks_to_export.retain(|task| {
                filter_tags.iter().any(|tag| task.tags.contains(tag))
            });
        }
    }
    
    // Filter by priority if specified
    if let Some(priority_filter) = priority_filter {
        let target_priority: Priority = priority_filter.clone().into();
        tasks_to_export.retain(|task| task.priority == target_priority);
    }

    // Filter by phase if specified
    if let Some(phase_str) = phase_filter {
        let target_phase = Phase::from_string(phase_str);
        tasks_to_export.retain(|task| task.phase == target_phase);
    }

    // Sort tasks by ID for consistent output
    tasks_to_export.sort_by_key(|task| task.id);
    
    // Generate export content based on format
    let export_content = match format {
        ExportFormat::Json => export_to_json(&roadmap, &tasks_to_export, pretty)?,
        ExportFormat::Csv => export_to_csv(&roadmap, &tasks_to_export)?,
        ExportFormat::Html => export_to_html(&roadmap, &tasks_to_export)?,
    };
    
    // Output to file or stdout
    match output_path {
        Some(path) => {
            fs::write(path, export_content)?;
            ui::display_success(&format!("✅ Exported {} tasks to {}", 
                tasks_to_export.len(), 
                path.display()));
        },
        None => {
            println!("{}", export_content);
        }
    }
    
    Ok(())
}

/// Export roadmap to JSON format
fn export_to_json(roadmap: &Roadmap, tasks: &[&Task], pretty: bool) -> Result<String, Box<dyn std::error::Error>> {
    use serde_json;
    
    // Create export structure
    let export_data = serde_json::json!({
        "roadmap": {
            "title": roadmap.title,
            "description": roadmap.metadata.description,
            "project_id": roadmap.project_id,
            "exported_at": chrono::Utc::now().to_rfc3339(),
            "total_tasks": roadmap.tasks.len(),
            "exported_tasks": tasks.len(),
            "progress": {
                "completed": roadmap.tasks.iter().filter(|t| t.status == TaskStatus::Completed).count(),
                "total": roadmap.tasks.len(),
                "percentage": (roadmap.tasks.iter().filter(|t| t.status == TaskStatus::Completed).count() as f64 / roadmap.tasks.len() as f64 * 100.0).round()
            }
        },
        "tasks": tasks.iter().map(|task| {
            serde_json::json!({
                "id": task.id,
                "description": task.description,
                "status": match task.status {
                    TaskStatus::Pending => "pending",
                    TaskStatus::Completed => "completed"
                },
                "priority": match task.priority {
                    Priority::Low => "low",
                    Priority::Medium => "medium", 
                    Priority::High => "high",
                    Priority::Critical => "critical"
                },
                "phase": {
                    "name": task.phase.name,
                    "description": task.phase.description(),
                    "emoji": task.phase.emoji(),
                    "is_predefined": task.phase.is_predefined()
                },
                "tags": task.tags.iter().collect::<Vec<_>>(),
                "notes": task.notes,
                "implementation_notes": task.implementation_notes,
                "dependencies": task.dependencies,
                "created_at": task.created_at,
                "completed_at": task.completed_at
            })
        }).collect::<Vec<_>>()
    });
    
    if pretty {
        Ok(serde_json::to_string_pretty(&export_data)?)
    } else {
        Ok(serde_json::to_string(&export_data)?)
    }
}

/// Export roadmap to CSV format
fn export_to_csv(_roadmap: &Roadmap, tasks: &[&Task]) -> Result<String, Box<dyn std::error::Error>> {
    let mut csv_content = String::new();
    
    // Add header
    csv_content.push_str("ID,Description,Status,Priority,Phase,Phase Type,Tags,Notes,Implementation Notes,Dependencies,Created At,Completed At\n");
    
    // Add tasks
    for task in tasks {
        let tags_str = task.tags.iter().cloned().collect::<Vec<_>>().join(";");
        let deps_str = task.dependencies.iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(";");
        let notes_escaped = task.notes.as_deref().unwrap_or("").replace("\"", "\"\"");
        let impl_notes_str = task.implementation_notes.join(" | ");
        let impl_notes_escaped = impl_notes_str.replace("\"", "\"\"");
        let desc_escaped = task.description.replace("\"", "\"\"");
        let phase_type = if task.phase.is_predefined() { "predefined" } else { "custom" };
        
        csv_content.push_str(&format!(
            "{},\"{}\",{},{},\"{}\",{},\"{}\",\"{}\",\"{}\",\"{}\",{},{}\n",
            task.id,
            desc_escaped,
            match task.status {
                TaskStatus::Pending => "pending",
                TaskStatus::Completed => "completed"
            },
            match task.priority {
                Priority::Low => "low",
                Priority::Medium => "medium",
                Priority::High => "high", 
                Priority::Critical => "critical"
            },
            task.phase.name,
            phase_type,
            tags_str,
            notes_escaped,
            impl_notes_escaped,
            deps_str,
            task.created_at.as_deref().unwrap_or(""),
            task.completed_at.as_deref().unwrap_or("")
        ));
    }
    
    Ok(csv_content)
}

/// Export roadmap to HTML format
fn export_to_html(roadmap: &Roadmap, tasks: &[&Task]) -> Result<String, Box<dyn std::error::Error>> {
    let completed_count = roadmap.tasks.iter().filter(|t| t.status == TaskStatus::Completed).count();
    let progress_percentage = (completed_count as f64 / roadmap.tasks.len() as f64 * 100.0).round();
    
    let mut html = String::new();
    
    // HTML header with embedded CSS
    html.push_str(&format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; margin: 40px; background: #f8f9fa; }}
        .container {{ max-width: 1200px; margin: 0 auto; background: white; padding: 40px; border-radius: 12px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }}
        h1 {{ color: #2c3e50; border-bottom: 3px solid #3498db; padding-bottom: 10px; }}
        .progress {{ background: #ecf0f1; border-radius: 20px; height: 20px; margin: 20px 0; }}
        .progress-bar {{ background: linear-gradient(90deg, #3498db, #2ecc71); height: 100%; border-radius: 20px; transition: width 0.3s; }}
        .stats {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px; margin: 30px 0; }}
        .stat-card {{ background: #f8f9fa; padding: 20px; border-radius: 8px; text-align: center; border-left: 4px solid #3498db; }}
        .stat-number {{ font-size: 2em; font-weight: bold; color: #2c3e50; }}
        .stat-label {{ color: #7f8c8d; margin-top: 5px; }}
        table {{ width: 100%; border-collapse: collapse; margin-top: 30px; }}
        th, td {{ padding: 12px; text-align: left; border-bottom: 1px solid #ddd; }}
        th {{ background: #34495e; color: white; font-weight: 600; }}
        tr:hover {{ background: #f5f5f5; }}
        .status-completed {{ color: #27ae60; font-weight: bold; }}
        .status-pending {{ color: #e67e22; font-weight: bold; }}
        .priority-critical {{ color: #e74c3c; font-weight: bold; }}
        .priority-high {{ color: #f39c12; font-weight: bold; }}
        .priority-medium {{ color: #3498db; }}
        .priority-low {{ color: #95a5a6; }}
        .tags {{ display: flex; flex-wrap: wrap; gap: 5px; }}
        .tag {{ background: #3498db; color: white; padding: 2px 8px; border-radius: 12px; font-size: 0.8em; }}
        .dependencies {{ color: #7f8c8d; font-style: italic; }}
        .export-info {{ background: #e8f4fd; padding: 15px; border-radius: 8px; margin-bottom: 30px; border-left: 4px solid #3498db; }}
    </style>
</head>
<body>
    <div class="container">
        <h1>{}</h1>
        
        <div class="export-info">
            <strong>📊 Export Information:</strong><br>
            Exported on: {}<br>
            Total tasks in roadmap: {} | Tasks in this export: {}
        </div>
        
        <div class="progress">
            <div class="progress-bar" style="width: {}%"></div>
        </div>
        
        <div class="stats">
            <div class="stat-card">
                <div class="stat-number">{}</div>
                <div class="stat-label">Total Tasks</div>
            </div>
            <div class="stat-card">
                <div class="stat-number">{}</div>
                <div class="stat-label">Completed</div>
            </div>
            <div class="stat-card">
                <div class="stat-number">{}%</div>
                <div class="stat-label">Progress</div>
            </div>
            <div class="stat-card">
                <div class="stat-number">{}</div>
                <div class="stat-label">In Export</div>
            </div>
        </div>
"#, 
        roadmap.title,
        roadmap.title,
        chrono::Utc::now().format("%Y-%m-%d %H:%M UTC"),
        roadmap.tasks.len(),
        tasks.len(),
        progress_percentage,
        roadmap.tasks.len(),
        completed_count,
        progress_percentage,
        tasks.len()
    ));
    
    // Tasks table
    html.push_str(r#"
        <table>
            <thead>
                <tr>
                    <th>ID</th>
                    <th>Description</th>
                    <th>Status</th>
                    <th>Priority</th>
                    <th>Tags</th>
                    <th>Dependencies</th>
                    <th>Created</th>
                </tr>
            </thead>
            <tbody>
"#);
    
    for task in tasks {
        let status_class = match task.status {
            TaskStatus::Completed => "status-completed",
            TaskStatus::Pending => "status-pending",
        };
        
        let priority_class = match task.priority {
            Priority::Critical => "priority-critical",
            Priority::High => "priority-high",
            Priority::Medium => "priority-medium",
            Priority::Low => "priority-low",
        };
        
        let tags_html = if task.tags.is_empty() {
            String::new()
        } else {
            format!("<div class=\"tags\">{}</div>", 
                task.tags.iter()
                    .map(|tag| format!("<span class=\"tag\">{}</span>", tag))
                    .collect::<Vec<_>>()
                    .join(""))
        };
        
        let deps_html = if task.dependencies.is_empty() {
            String::new()
        } else {
            format!("<span class=\"dependencies\">Depends on: {}</span>", 
                task.dependencies.iter()
                    .map(|id| format!("#{}", id))
                    .collect::<Vec<_>>()
                    .join(", "))
        };
        
        html.push_str(&format!(r#"
                <tr>
                    <td>#{}</td>
                    <td>{}</td>
                    <td class="{}">{}</td>
                    <td class="{}">{}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}</td>
                </tr>
"#,
            task.id,
            utils::html_escape(&task.description),
            status_class,
            match task.status {
                TaskStatus::Completed => "✅ Completed",
                TaskStatus::Pending => "⏳ Pending",
            },
            priority_class,
            match task.priority {
                Priority::Critical => "🔥 Critical",
                Priority::High => "⬆️ High",
                Priority::Medium => "▶️ Medium",
                Priority::Low => "⬇️ Low",
            },
            tags_html,
            deps_html,
            task.created_at.as_deref().unwrap_or("").split('T').next().unwrap_or("")
        ));
    }
    
    // Close HTML
    html.push_str(r#"
            </tbody>
        </table>
    </div>
</body>
</html>
"#);
    
    Ok(html)
} 