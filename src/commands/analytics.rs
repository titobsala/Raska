use crate::model::{Roadmap, Task, TaskStatus, Priority, Phase};
use crate::{state, ui};
use super::CommandResult;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::Serialize;

/// Analytics data structures
#[derive(Debug, Clone, Serialize)]
pub struct ProgressAnalytics {
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub pending_tasks: usize,
    pub completion_rate: f64,
    pub velocity_tasks_per_day: f64,
    pub velocity_hours_per_day: f64,
    pub average_task_completion_time: f64,
    pub estimation_accuracy: f64,
    pub phase_analytics: Vec<PhaseAnalytics>,
    pub priority_analytics: Vec<PriorityAnalytics>,
    pub time_analytics: TimeAnalytics,
}

#[derive(Debug, Clone, Serialize)]
pub struct PhaseAnalytics {
    pub phase: Phase,
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub completion_rate: f64,
    pub estimated_hours: f64,
    pub actual_hours: f64,
    pub variance_hours: f64,
    pub ready_tasks: usize,
    pub blocked_tasks: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct PriorityAnalytics {
    pub priority: Priority,
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub completion_rate: f64,
    pub average_completion_time: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct TimeAnalytics {
    pub total_estimated_hours: f64,
    pub total_actual_hours: f64,
    pub total_variance_hours: f64,
    pub variance_percentage: f64,
    pub estimation_accuracy: f64,
    pub tasks_with_estimates: usize,
    pub tasks_with_tracked_time: usize,
    pub over_estimated_tasks: usize,
    pub under_estimated_tasks: usize,
    pub accurate_estimates: usize,
    pub total_sessions: usize,
    pub active_sessions: usize,
    pub average_session_duration: f64,
}

/// Main analytics command handler
pub fn show_analytics(
    overview: bool,
    time_focus: bool,
    phases: bool,
    priorities: bool,
    trends: bool,
    export_format: Option<String>,
) -> CommandResult {
    let roadmap = state::load_state()?;
    let analytics = calculate_analytics(&roadmap)?;
    
    if overview || (!time_focus && !phases && !priorities && !trends) {
        ui::display_analytics_overview(&analytics);
    }
    
    if time_focus {
        ui::display_time_analytics(&analytics.time_analytics);
    }
    
    if phases {
        ui::display_phase_analytics(&analytics.phase_analytics);
    }
    
    if priorities {
        ui::display_priority_analytics(&analytics.priority_analytics);
    }
    
    if trends {
        ui::display_trend_analytics(&roadmap, &analytics)?;
    }
    
    if let Some(format) = export_format {
        export_analytics_report(&analytics, &format)?;
    }
    
    Ok(())
}

/// Calculate comprehensive analytics from roadmap data
fn calculate_analytics(roadmap: &Roadmap) -> Result<ProgressAnalytics, Box<dyn std::error::Error>> {
    let total_tasks = roadmap.tasks.len();
    let completed_tasks = roadmap.tasks.iter().filter(|t| t.status == TaskStatus::Completed).count();
    let pending_tasks = total_tasks - completed_tasks;
    let completion_rate = if total_tasks > 0 { completed_tasks as f64 / total_tasks as f64 * 100.0 } else { 0.0 };
    
    // Calculate velocity (tasks completed per day)
    let velocity_tasks_per_day = calculate_task_velocity(roadmap);
    let velocity_hours_per_day = calculate_hour_velocity(roadmap);
    
    // Calculate average task completion time
    let average_task_completion_time = calculate_average_completion_time(roadmap);
    
    // Calculate estimation accuracy
    let estimation_accuracy = calculate_estimation_accuracy(roadmap);
    
    // Calculate phase analytics
    let phase_analytics = calculate_phase_analytics(roadmap);
    
    // Calculate priority analytics
    let priority_analytics = calculate_priority_analytics(roadmap);
    
    // Calculate time analytics
    let time_analytics = calculate_time_analytics(roadmap);
    
    Ok(ProgressAnalytics {
        total_tasks,
        completed_tasks,
        pending_tasks,
        completion_rate,
        velocity_tasks_per_day,
        velocity_hours_per_day,
        average_task_completion_time,
        estimation_accuracy,
        phase_analytics,
        priority_analytics,
        time_analytics,
    })
}

/// Calculate task completion velocity (tasks per day)
fn calculate_task_velocity(roadmap: &Roadmap) -> f64 {
    let completed_tasks: Vec<_> = roadmap.tasks.iter()
        .filter(|t| t.status == TaskStatus::Completed && t.completed_at.is_some())
        .collect();
    
    if completed_tasks.is_empty() {
        return 0.0;
    }
    
    // Find the date range of completed tasks
    let mut earliest_date: Option<DateTime<Utc>> = None;
    let mut latest_date: Option<DateTime<Utc>> = None;
    
    for task in &completed_tasks {
        if let Some(completed_at) = &task.completed_at {
            if let Ok(date) = DateTime::parse_from_rfc3339(completed_at) {
                let utc_date = date.with_timezone(&Utc);
                
                if earliest_date.is_none() || utc_date < earliest_date.unwrap() {
                    earliest_date = Some(utc_date);
                }
                
                if latest_date.is_none() || utc_date > latest_date.unwrap() {
                    latest_date = Some(utc_date);
                }
            }
        }
    }
    
    if let (Some(earliest), Some(latest)) = (earliest_date, latest_date) {
        let duration = latest - earliest;
        let days = duration.num_days().max(1) as f64;
        completed_tasks.len() as f64 / days
    } else {
        0.0
    }
}

/// Calculate hour completion velocity (hours per day)
fn calculate_hour_velocity(roadmap: &Roadmap) -> f64 {
    let total_hours: f64 = roadmap.tasks.iter()
        .filter(|t| t.status == TaskStatus::Completed)
        .filter_map(|t| t.actual_hours)
        .sum();
    
    let velocity_days = calculate_project_duration_days(roadmap);
    if velocity_days > 0.0 {
        total_hours / velocity_days
    } else {
        0.0
    }
}

/// Calculate average task completion time in days
fn calculate_average_completion_time(roadmap: &Roadmap) -> f64 {
    let completion_times: Vec<f64> = roadmap.tasks.iter()
        .filter(|t| t.status == TaskStatus::Completed)
        .filter_map(|t| {
            if let (Some(created), Some(completed)) = (&t.created_at, &t.completed_at) {
                if let (Ok(created_date), Ok(completed_date)) = (
                    DateTime::parse_from_rfc3339(created),
                    DateTime::parse_from_rfc3339(completed)
                ) {
                    let duration = completed_date - created_date;
                    Some(duration.num_days() as f64)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    
    if completion_times.is_empty() {
        0.0
    } else {
        completion_times.iter().sum::<f64>() / completion_times.len() as f64
    }
}

/// Calculate overall estimation accuracy percentage
fn calculate_estimation_accuracy(roadmap: &Roadmap) -> f64 {
    let tasks_with_both: Vec<_> = roadmap.tasks.iter()
        .filter(|t| t.estimated_hours.is_some() && t.actual_hours.is_some())
        .collect();
    
    if tasks_with_both.is_empty() {
        return 0.0;
    }
    
    let total_estimated: f64 = tasks_with_both.iter().filter_map(|t| t.estimated_hours).sum();
    let total_actual: f64 = tasks_with_both.iter().filter_map(|t| t.actual_hours).sum();
    
    if total_estimated > 0.0 {
        let variance = (total_actual - total_estimated).abs();
        ((total_estimated - variance) / total_estimated * 100.0).max(0.0)
    } else {
        0.0
    }
}

/// Calculate analytics for each phase
fn calculate_phase_analytics(roadmap: &Roadmap) -> Vec<PhaseAnalytics> {
    let mut phase_map: HashMap<String, Vec<&Task>> = HashMap::new();
    
    // Group tasks by phase
    for task in &roadmap.tasks {
        phase_map.entry(task.phase.name.clone()).or_insert_with(Vec::new).push(task);
    }
    
    let mut phase_analytics = Vec::new();
    let completed_task_ids = roadmap.get_completed_task_ids();
    
    for (phase_name, tasks) in phase_map {
        let phase = Phase::from_string(&phase_name);
        let total_tasks = tasks.len();
        let completed_tasks = tasks.iter().filter(|t| t.status == TaskStatus::Completed).count();
        let completion_rate = if total_tasks > 0 { completed_tasks as f64 / total_tasks as f64 * 100.0 } else { 0.0 };
        
        let estimated_hours: f64 = tasks.iter().filter_map(|t| t.estimated_hours).sum();
        let actual_hours: f64 = tasks.iter().filter_map(|t| t.actual_hours).sum();
        let variance_hours = actual_hours - estimated_hours;
        
        let ready_tasks = tasks.iter()
            .filter(|t| t.status == TaskStatus::Pending && t.can_be_started(&completed_task_ids))
            .count();
        
        let blocked_tasks = tasks.iter()
            .filter(|t| t.status == TaskStatus::Pending && !t.can_be_started(&completed_task_ids))
            .count();
        
        phase_analytics.push(PhaseAnalytics {
            phase,
            total_tasks,
            completed_tasks,
            completion_rate,
            estimated_hours,
            actual_hours,
            variance_hours,
            ready_tasks,
            blocked_tasks,
        });
    }
    
    // Sort by completion rate (highest first)
    phase_analytics.sort_by(|a, b| b.completion_rate.partial_cmp(&a.completion_rate).unwrap_or(std::cmp::Ordering::Equal));
    
    phase_analytics
}

/// Calculate analytics for each priority level
fn calculate_priority_analytics(roadmap: &Roadmap) -> Vec<PriorityAnalytics> {
    let priorities = [Priority::Critical, Priority::High, Priority::Medium, Priority::Low];
    let mut priority_analytics = Vec::new();
    
    for priority in &priorities {
        let tasks: Vec<_> = roadmap.tasks.iter().filter(|t| &t.priority == priority).collect();
        let total_tasks = tasks.len();
        let completed_tasks = tasks.iter().filter(|t| t.status == TaskStatus::Completed).count();
        let completion_rate = if total_tasks > 0 { completed_tasks as f64 / total_tasks as f64 * 100.0 } else { 0.0 };
        
        // Calculate average completion time for this priority
        let completion_times: Vec<f64> = tasks.iter()
            .filter(|t| t.status == TaskStatus::Completed)
            .filter_map(|t| {
                if let (Some(created), Some(completed)) = (&t.created_at, &t.completed_at) {
                    if let (Ok(created_date), Ok(completed_date)) = (
                        DateTime::parse_from_rfc3339(created),
                        DateTime::parse_from_rfc3339(completed)
                    ) {
                        let duration = completed_date - created_date;
                        Some(duration.num_days() as f64)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        
        let average_completion_time = if completion_times.is_empty() {
            0.0
        } else {
            completion_times.iter().sum::<f64>() / completion_times.len() as f64
        };
        
        if total_tasks > 0 {
            priority_analytics.push(PriorityAnalytics {
                priority: priority.clone(),
                total_tasks,
                completed_tasks,
                completion_rate,
                average_completion_time,
            });
        }
    }
    
    priority_analytics
}

/// Calculate comprehensive time tracking analytics
fn calculate_time_analytics(roadmap: &Roadmap) -> TimeAnalytics {
    let total_estimated_hours: f64 = roadmap.tasks.iter().filter_map(|t| t.estimated_hours).sum();
    let total_actual_hours: f64 = roadmap.tasks.iter().filter_map(|t| t.actual_hours).sum();
    let total_variance_hours = total_actual_hours - total_estimated_hours;
    
    let variance_percentage = if total_estimated_hours > 0.0 {
        (total_variance_hours / total_estimated_hours) * 100.0
    } else {
        0.0
    };
    
    let estimation_accuracy = if total_estimated_hours > 0.0 {
        ((total_estimated_hours - total_variance_hours.abs()) / total_estimated_hours * 100.0).max(0.0)
    } else {
        0.0
    };
    
    let tasks_with_estimates = roadmap.tasks.iter().filter(|t| t.estimated_hours.is_some()).count();
    let tasks_with_tracked_time = roadmap.tasks.iter().filter(|t| t.actual_hours.is_some()).count();
    let over_estimated_tasks = roadmap.tasks.iter().filter(|t| t.is_over_estimated()).count();
    let under_estimated_tasks = roadmap.tasks.iter().filter(|t| t.is_under_estimated()).count();
    
    let accurate_estimates = roadmap.tasks.iter().filter(|t| {
        if let (Some(est), Some(actual)) = (t.estimated_hours, t.actual_hours) {
            let variance_pct = (actual - est).abs() / est * 100.0;
            variance_pct <= 20.0  // Within 20% is considered accurate
        } else {
            false
        }
    }).count();
    
    let total_sessions: usize = roadmap.tasks.iter().map(|t| t.time_sessions.len()).sum();
    let active_sessions = roadmap.tasks.iter().filter(|t| t.has_active_time_session()).count();
    
    // Calculate average session duration
    let all_session_durations: Vec<f64> = roadmap.tasks.iter()
        .flat_map(|t| &t.time_sessions)
        .filter_map(|s| s.duration_hours())
        .collect();
    
    let average_session_duration = if all_session_durations.is_empty() {
        0.0
    } else {
        all_session_durations.iter().sum::<f64>() / all_session_durations.len() as f64
    };
    
    TimeAnalytics {
        total_estimated_hours,
        total_actual_hours,
        total_variance_hours,
        variance_percentage,
        estimation_accuracy,
        tasks_with_estimates,
        tasks_with_tracked_time,
        over_estimated_tasks,
        under_estimated_tasks,
        accurate_estimates,
        total_sessions,
        active_sessions,
        average_session_duration,
    }
}

/// Calculate project duration in days
fn calculate_project_duration_days(roadmap: &Roadmap) -> f64 {
    let dates: Vec<DateTime<Utc>> = roadmap.tasks.iter()
        .filter_map(|t| {
            if let Some(created_at) = &t.created_at {
                DateTime::parse_from_rfc3339(created_at).ok().map(|d| d.with_timezone(&Utc))
            } else {
                None
            }
        })
        .collect();
    
    if dates.len() < 2 {
        return 1.0; // Default to 1 day if insufficient data
    }
    
    let earliest = dates.iter().min().unwrap();
    let latest = dates.iter().max().unwrap();
    let duration = *latest - *earliest;
    duration.num_days().max(1) as f64
}

/// Export analytics report in specified format
fn export_analytics_report(analytics: &ProgressAnalytics, format: &str) -> CommandResult {
    match format.to_lowercase().as_str() {
        "json" => {
            let json_report = serde_json::to_string_pretty(&analytics)
                .map_err(|e| format!("Failed to serialize analytics: {}", e))?;
            println!("{}", json_report);
        },
        "summary" => {
            ui::display_analytics_summary(analytics);
        },
        _ => {
            return Err(format!("Unsupported export format: {}. Use 'json' or 'summary'", format).into());
        }
    }
    
    Ok(())
} 