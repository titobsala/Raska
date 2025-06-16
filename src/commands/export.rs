//! Export functionality commands
//! 
//! This module handles exporting roadmaps to different formats including
//! JSON, CSV, and HTML with filtering and formatting options.
//! 
//! **Phase 3 Enhancement**: Enhanced Export Capabilities 📤
//! - Added comprehensive time tracking data integration
//! - Enhanced filtering capabilities for time-based exports
//! - Report templates with time analysis
//! - Interactive visualizations and productivity metrics

use crate::{
    cli::CliPriority,
    model::{TaskStatus, Priority, Phase, Task, Roadmap},
    state,
    ui
};
use super::{CommandResult, utils, ExportFormat};
use std::fs;
use std::path::Path;

/// Export roadmap to different formats with enhanced time-based filtering (Phase 3)
pub fn export_roadmap_enhanced(
    format: &ExportFormat,
    output_path: Option<&Path>,
    include_completed: bool,
    tags_filter: Option<&str>,
    priority_filter: Option<&CliPriority>,
    phase_filter: Option<&String>,
    pretty: bool,
    created_after: Option<&str>,
    created_before: Option<&str>,
    min_estimated_hours: Option<f64>,
    max_estimated_hours: Option<f64>,
    min_actual_hours: Option<f64>,
    max_actual_hours: Option<f64>,
    with_time_data: bool,
    active_sessions_only: bool,
    over_estimated_only: bool,
    under_estimated_only: bool,
) -> CommandResult {
    let roadmap = state::load_state()?;
    
    // Apply all filters to get the tasks to export
    let mut tasks_to_export: Vec<&Task> = roadmap.tasks.iter().collect();
    
    // Apply existing filters
    if !include_completed {
        tasks_to_export.retain(|task| task.status != TaskStatus::Completed);
    }
    
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
    
    if let Some(priority_filter) = priority_filter {
        let target_priority: Priority = priority_filter.clone().into();
        tasks_to_export.retain(|task| task.priority == target_priority);
    }

    if let Some(phase_str) = phase_filter {
        let target_phase = Phase::from_string(phase_str);
        tasks_to_export.retain(|task| task.phase == target_phase);
    }

    // NEW: Apply time-based filters (Phase 3 enhancement)
    
    // Date range filtering
    if let Some(after_date) = created_after {
        tasks_to_export.retain(|task| {
            if let Some(created_at) = &task.created_at {
                created_at.as_str() >= after_date
            } else {
                false
            }
        });
    }
    
    if let Some(before_date) = created_before {
        tasks_to_export.retain(|task| {
            if let Some(created_at) = &task.created_at {
                created_at.as_str() <= before_date
            } else {
                false
            }
        });
    }
    
    // Time estimation filtering
    if let Some(min_est) = min_estimated_hours {
        tasks_to_export.retain(|task| {
            task.estimated_hours.map_or(false, |est| est >= min_est)
        });
    }
    
    if let Some(max_est) = max_estimated_hours {
        tasks_to_export.retain(|task| {
            task.estimated_hours.map_or(false, |est| est <= max_est)
        });
    }
    
    // Actual time filtering
    if let Some(min_actual) = min_actual_hours {
        tasks_to_export.retain(|task| {
            task.actual_hours.map_or(false, |actual| actual >= min_actual)
        });
    }
    
    if let Some(max_actual) = max_actual_hours {
        tasks_to_export.retain(|task| {
            task.actual_hours.map_or(false, |actual| actual <= max_actual)
        });
    }
    
    // Time tracking data filtering
    if with_time_data {
        tasks_to_export.retain(|task| {
            task.estimated_hours.is_some() || task.actual_hours.is_some() || !task.time_sessions.is_empty()
        });
    }
    
    if active_sessions_only {
        tasks_to_export.retain(|task| task.has_active_time_session());
    }
    
    if over_estimated_only {
        tasks_to_export.retain(|task| task.is_over_estimated());
    }
    
    if under_estimated_only {
        tasks_to_export.retain(|task| task.is_under_estimated());
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

/// Legacy export function for backward compatibility
pub fn export_roadmap(
    format: &ExportFormat,
    output_path: Option<&Path>,
    include_completed: bool,
    tags_filter: Option<&str>,
    priority_filter: Option<&CliPriority>,
    phase_filter: Option<&String>,
    pretty: bool,
) -> CommandResult {
    // Call the enhanced version with default values for new parameters
    export_roadmap_enhanced(
        format, output_path, include_completed, tags_filter, priority_filter, phase_filter, pretty,
        None, None, None, None, None, None, false, false, false, false
    )
}

/// Export roadmap to JSON format with comprehensive time tracking data
fn export_to_json(roadmap: &Roadmap, tasks: &[&Task], pretty: bool) -> Result<String, Box<dyn std::error::Error>> {
    use serde_json;
    
    // Calculate time tracking metrics for the entire export
    let total_estimated: f64 = tasks.iter().filter_map(|t| t.estimated_hours).sum();
    let total_actual: f64 = tasks.iter().filter_map(|t| t.actual_hours).sum();
    let tasks_with_estimates = tasks.iter().filter(|t| t.estimated_hours.is_some()).count();
    let tasks_with_time = tasks.iter().filter(|t| t.actual_hours.is_some()).count();
    let total_sessions: usize = tasks.iter().map(|t| t.time_sessions.len()).sum();
    
    // Calculate overall variance
    let overall_variance = if total_estimated > 0.0 && total_actual > 0.0 {
        total_actual - total_estimated
    } else {
        0.0
    };
    
    let overall_variance_percentage = if total_estimated > 0.0 {
        (overall_variance / total_estimated * 100.0).round()
    } else {
        0.0
    };
    
    // Create export structure with enhanced time tracking data
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
            },
            // NEW: Comprehensive time tracking metrics
            "time_tracking": {
                "total_estimated_hours": total_estimated,
                "total_actual_hours": total_actual,
                "total_variance_hours": overall_variance,
                "variance_percentage": overall_variance_percentage,
                "estimation_accuracy": if total_estimated > 0.0 { 
                    (100.0 - (overall_variance.abs() / total_estimated * 100.0)).max(0.0).round() 
                } else { 
                    0.0 
                },
                "tasks_with_estimates": tasks_with_estimates,
                "tasks_with_tracked_time": tasks_with_time,
                "total_time_sessions": total_sessions,
                "active_sessions": tasks.iter().filter(|t| t.has_active_time_session()).count(),
                "productivity_metrics": {
                    "average_estimated_hours": if tasks_with_estimates > 0 { 
                        total_estimated / tasks_with_estimates as f64 
                    } else { 
                        0.0 
                    },
                    "average_actual_hours": if tasks_with_time > 0 { 
                        total_actual / tasks_with_time as f64 
                    } else { 
                        0.0 
                    },
                    "over_estimated_tasks": tasks.iter().filter(|t| t.is_over_estimated()).count(),
                    "under_estimated_tasks": tasks.iter().filter(|t| t.is_under_estimated()).count(),
                    "accurate_estimates_count": tasks.iter().filter(|t| {
                        if let (Some(est), Some(actual)) = (t.estimated_hours, t.actual_hours) {
                            let variance_pct = ((actual - est).abs() / est * 100.0);
                            variance_pct <= 20.0  // Within 20% is considered accurate
                        } else {
                            false
                        }
                    }).count()
                }
            }
        },
        "tasks": tasks.iter().map(|task| {
            // Calculate task-specific time metrics
            let variance = task.get_time_variance().unwrap_or(0.0);
            let variance_percentage = task.get_time_variance_percentage().unwrap_or(0.0);
            
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
                "completed_at": task.completed_at,
                // NEW: Comprehensive time tracking data for each task
                "time_tracking": {
                    "estimated_hours": task.estimated_hours,
                    "actual_hours": task.actual_hours,
                    "variance_hours": if variance != 0.0 { Some(variance) } else { None },
                    "variance_percentage": if variance_percentage != 0.0 { Some(variance_percentage) } else { None },
                    "is_over_estimated": task.is_over_estimated(),
                    "is_under_estimated": task.is_under_estimated(),
                    "has_active_session": task.has_active_time_session(),
                    "total_sessions": task.time_sessions.len(),
                    "sessions": task.time_sessions.iter().map(|session| {
                        serde_json::json!({
                            "start_time": session.start_time,
                            "end_time": session.end_time,
                            "duration_minutes": session.duration_minutes,
                            "duration_hours": session.duration_hours(),
                            "description": session.description,
                            "is_active": session.is_active(),
                            "date": session.start_time.split('T').next().unwrap_or("unknown")
                        })
                    }).collect::<Vec<_>>()
                }
            })
        }).collect::<Vec<_>>()
    });
    
    if pretty {
        Ok(serde_json::to_string_pretty(&export_data)?)
    } else {
        Ok(serde_json::to_string(&export_data)?)
    }
}

/// Export roadmap to CSV format with comprehensive time tracking columns
fn export_to_csv(_roadmap: &Roadmap, tasks: &[&Task]) -> Result<String, Box<dyn std::error::Error>> {
    let mut csv_content = String::new();
    
    // Add enhanced header with time tracking columns
    csv_content.push_str("ID,Description,Status,Priority,Phase,Phase Type,Tags,Notes,Implementation Notes,Dependencies,Created At,Completed At,Estimated Hours,Actual Hours,Variance Hours,Variance %,Total Sessions,Active Session,Is Over Estimated,Is Under Estimated,Session Details\n");
    
    // Add tasks with comprehensive time tracking data
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
        
        // Time tracking data
        let estimated_hours = task.estimated_hours.map_or("".to_string(), |h| format!("{:.2}", h));
        let actual_hours = task.actual_hours.map_or("".to_string(), |h| format!("{:.2}", h));
        let variance_hours = task.get_time_variance().map_or("".to_string(), |v| format!("{:.2}", v));
        let variance_percentage = task.get_time_variance_percentage().map_or("".to_string(), |v| format!("{:.1}", v));
        let total_sessions = task.time_sessions.len().to_string();
        let has_active_session = if task.has_active_time_session() { "Yes" } else { "No" };
        let is_over_estimated = if task.is_over_estimated() { "Yes" } else { "No" };
        let is_under_estimated = if task.is_under_estimated() { "Yes" } else { "No" };
        
        // Session details as a summary string
        let session_details = if task.time_sessions.is_empty() {
            "".to_string()
        } else {
            task.time_sessions.iter()
                .map(|session| {
                    let duration = session.duration_hours()
                        .map_or("active".to_string(), |h| format!("{:.2}h", h));
                    let desc = session.description.as_deref().unwrap_or("No description");
                    format!("[{}:{}]", duration, desc)
                })
                .collect::<Vec<_>>()
                .join(";")
        };
        let session_details_escaped = session_details.replace("\"", "\"\"");
        
        csv_content.push_str(&format!(
            "{},\"{}\",{},{},\"{}\",{},\"{}\",\"{}\",\"{}\",\"{}\",{},{},{},{},{},{},{},{},{},{},\"{}\"\n",
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
            task.completed_at.as_deref().unwrap_or(""),
            estimated_hours,
            actual_hours,
            variance_hours,
            variance_percentage,
            total_sessions,
            has_active_session,
            is_over_estimated,
            is_under_estimated,
            session_details_escaped
        ));
    }
    
    Ok(csv_content)
}

/// Export roadmap to HTML format with interactive time tracking visualizations
fn export_to_html(roadmap: &Roadmap, tasks: &[&Task]) -> Result<String, Box<dyn std::error::Error>> {
    let completed_count = roadmap.tasks.iter().filter(|t| t.status == TaskStatus::Completed).count();
    let progress_percentage = (completed_count as f64 / roadmap.tasks.len() as f64 * 100.0).round();
    
    // Calculate comprehensive time tracking metrics for HTML display
    let total_estimated: f64 = tasks.iter().filter_map(|t| t.estimated_hours).sum();
    let total_actual: f64 = tasks.iter().filter_map(|t| t.actual_hours).sum();
    let tasks_with_estimates = tasks.iter().filter(|t| t.estimated_hours.is_some()).count();
    let tasks_with_time = tasks.iter().filter(|t| t.actual_hours.is_some()).count();
    let total_sessions: usize = tasks.iter().map(|t| t.time_sessions.len()).sum();
    let active_sessions = tasks.iter().filter(|t| t.has_active_time_session()).count();
    let over_estimated_count = tasks.iter().filter(|t| t.is_over_estimated()).count();
    let under_estimated_count = tasks.iter().filter(|t| t.is_under_estimated()).count();
    
    let overall_variance = if total_estimated > 0.0 && total_actual > 0.0 {
        total_actual - total_estimated
    } else {
        0.0
    };
    
    let estimation_accuracy = if total_estimated > 0.0 { 
        (100.0 - (overall_variance.abs() / total_estimated * 100.0)).max(0.0).round() 
    } else { 
        0.0 
    };
    
    let mut html = String::new();
    
    // HTML header with embedded CSS
    html.push_str(&format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - Time Tracking Report</title>
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; margin: 20px; background: #f8f9fa; }}
        .container {{ max-width: 1400px; margin: 0 auto; background: white; padding: 40px; border-radius: 12px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }}
        h1 {{ color: #2c3e50; border-bottom: 3px solid #3498db; padding-bottom: 10px; }}
        h2 {{ color: #34495e; border-bottom: 2px solid #e8f4fd; padding-bottom: 8px; margin-top: 40px; }}
        
        /* Progress Bars */
        .progress {{ background: #ecf0f1; border-radius: 20px; height: 20px; margin: 20px 0; position: relative; }}
        .progress-bar {{ background: linear-gradient(90deg, #3498db, #2ecc71); height: 100%; border-radius: 20px; transition: width 0.3s; }}
        .progress-text {{ position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); font-weight: bold; font-size: 0.9em; }}
        
        /* Statistics Grid */
        .stats {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); gap: 20px; margin: 30px 0; }}
        .stat-card {{ background: #f8f9fa; padding: 20px; border-radius: 8px; text-align: center; border-left: 4px solid #3498db; transition: transform 0.2s; }}
        .stat-card:hover {{ transform: translateY(-2px); }}
        .stat-number {{ font-size: 2em; font-weight: bold; color: #2c3e50; }}
        .stat-label {{ color: #7f8c8d; margin-top: 5px; font-size: 0.9em; }}
        
        /* Time Tracking Specific Stats */
        .time-stats {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px; margin: 30px 0; }}
        .time-card {{ background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 25px; border-radius: 12px; text-align: center; }}
        .time-card.variance {{ background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%); }}
        .time-card.accuracy {{ background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%); }}
        .time-card.sessions {{ background: linear-gradient(135deg, #43e97b 0%, #38f9d7 100%); }}
        
        /* Variance Indicators */
        .variance-good {{ color: #27ae60; font-weight: bold; }}
        .variance-bad {{ color: #e74c3c; font-weight: bold; }}
        .variance-neutral {{ color: #7f8c8d; }}
        
        /* Table Styles */
        table {{ width: 100%; border-collapse: collapse; margin-top: 30px; }}
        th, td {{ padding: 12px; text-align: left; border-bottom: 1px solid #ddd; }}
        th {{ background: #34495e; color: white; font-weight: 600; }}
        tr:hover {{ background: #f5f5f5; }}
        
        /* Status and Priority Colors */
        .status-completed {{ color: #27ae60; font-weight: bold; }}
        .status-pending {{ color: #e67e22; font-weight: bold; }}
        .priority-critical {{ color: #e74c3c; font-weight: bold; }}
        .priority-high {{ color: #f39c12; font-weight: bold; }}
        .priority-medium {{ color: #3498db; }}
        .priority-low {{ color: #95a5a6; }}
        
        /* Tags and Dependencies */
        .tags {{ display: flex; flex-wrap: wrap; gap: 5px; }}
        .tag {{ background: #3498db; color: white; padding: 2px 8px; border-radius: 12px; font-size: 0.8em; }}
        .dependencies {{ color: #7f8c8d; font-style: italic; }}
        
        /* Time Tracking Columns */
        .time-estimate {{ color: #3498db; font-weight: bold; }}
        .time-actual {{ color: #27ae60; font-weight: bold; }}
        .time-variance {{ font-weight: bold; }}
        .time-sessions-count {{ background: #e8f4fd; padding: 4px 8px; border-radius: 12px; font-size: 0.9em; }}
        
        /* Info Boxes */
        .export-info {{ background: #e8f4fd; padding: 15px; border-radius: 8px; margin-bottom: 30px; border-left: 4px solid #3498db; }}
        .time-summary {{ background: #f0f8ff; padding: 20px; border-radius: 8px; margin: 20px 0; border-left: 4px solid #667eea; }}
        
        /* Session Details */
        .session-details {{ font-size: 0.85em; color: #7f8c8d; }}
        .session-badge {{ background: #ecf0f1; padding: 2px 6px; border-radius: 8px; margin: 1px; display: inline-block; }}
        .active-session {{ background: #e74c3c; color: white; animation: pulse 2s infinite; }}
        
        @keyframes pulse {{
            0% {{ opacity: 1; }}
            50% {{ opacity: 0.5; }}
            100% {{ opacity: 1; }}
        }}
        
        /* Responsive Design */
        @media (max-width: 768px) {{
            .container {{ padding: 20px; margin: 10px; }}
            .stats, .time-stats {{ grid-template-columns: 1fr; }}
            table {{ font-size: 0.9em; }}
        }}
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
            <div class="progress-text">{}% Complete</div>
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
        
        <h2>⏱️ Time Tracking Overview</h2>
        
        <div class="time-summary">
            <strong>📈 Time Tracking Summary:</strong><br>
            Tasks with estimates: {} | Tasks with tracked time: {} | Total sessions: {} | Active sessions: {}
        </div>
        
        <div class="time-stats">
            <div class="time-card">
                <div class="stat-number">{:.1}h</div>
                <div class="stat-label">📋 Total Estimated</div>
            </div>
            <div class="time-card">
                <div class="stat-number">{:.1}h</div>
                <div class="stat-label">⏰ Total Actual</div>
            </div>
            <div class="time-card variance">
                <div class="stat-number">{:+.1}h</div>
                <div class="stat-label">📊 Variance</div>
            </div>
            <div class="time-card accuracy">
                <div class="stat-number">{:.1}%</div>
                <div class="stat-label">🎯 Accuracy</div>
            </div>
            <div class="time-card sessions">
                <div class="stat-number">{}</div>
                <div class="stat-label">🔄 Total Sessions</div>
            </div>
            <div class="time-card">
                <div class="stat-number">{}</div>
                <div class="stat-label">📈 Over Estimated</div>
            </div>
            <div class="time-card">
                <div class="stat-number">{}</div>
                <div class="stat-label">📉 Under Estimated</div>
            </div>
            <div class="time-card">
                <div class="stat-number">{}</div>
                <div class="stat-label">🔴 Active Now</div>
            </div>
        </div>
"#, 
        roadmap.title,
        roadmap.title,
        chrono::Utc::now().format("%Y-%m-%d %H:%M UTC"),
        roadmap.tasks.len(),
        tasks.len(),
        progress_percentage,
        progress_percentage,
        roadmap.tasks.len(),
        completed_count,
        progress_percentage,
        tasks.len(),
        tasks_with_estimates,
        tasks_with_time,
        total_sessions,
        active_sessions,
        total_estimated,
        total_actual,
        overall_variance,
        estimation_accuracy,
        total_sessions,
        over_estimated_count,
        under_estimated_count,
        active_sessions
    ));
    
    // Enhanced Tasks table with time tracking columns
    html.push_str(r#"
        <h2>📋 Task Details</h2>
        <table>
            <thead>
                <tr>
                    <th>ID</th>
                    <th>Description</th>
                    <th>Status</th>
                    <th>Priority</th>
                    <th>Phase</th>
                    <th>⏱️ Est.</th>
                    <th>⏰ Actual</th>
                    <th>📊 Variance</th>
                    <th>🔄 Sessions</th>
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
        
        // Generate time tracking data for the row
        let estimated_display = task.estimated_hours
            .map_or("--".to_string(), |h| format!("{:.1}h", h));
        let actual_display = task.actual_hours
            .map_or("--".to_string(), |h| format!("{:.1}h", h));
        
        let (variance_display, variance_class) = if let Some(variance) = task.get_time_variance() {
            let variance_str = format!("{:+.1}h", variance);
            let class = if variance > 1.0 {
                "variance-bad"
            } else if variance < -1.0 {
                "variance-good"
            } else {
                "variance-neutral"
            };
            (variance_str, class)
        } else {
            ("--".to_string(), "variance-neutral")
        };
        
        let sessions_display = if task.time_sessions.is_empty() {
            "--".to_string()
        } else {
            let active_indicator = if task.has_active_time_session() {
                " 🔴"
            } else {
                ""
            };
            format!("<span class=\"time-sessions-count\">{}{}</span>", task.time_sessions.len(), active_indicator)
        };
        
        html.push_str(&format!(r#"
                <tr>
                    <td>#{}</td>
                    <td>{}</td>
                    <td class="{}">{}</td>
                    <td class="{}">{}</td>
                    <td>{} {}</td>
                    <td class="time-estimate">{}</td>
                    <td class="time-actual">{}</td>
                    <td class="time-variance {}">{}</td>
                    <td>{}</td>
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
            task.phase.emoji(),
            utils::html_escape(&task.phase.name),
            estimated_display,
            actual_display,
            variance_class,
            variance_display,
            sessions_display,
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