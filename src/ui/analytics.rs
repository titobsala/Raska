use crate::commands::analytics::{ProgressAnalytics, PhaseAnalytics, PriorityAnalytics, TimeAnalytics};
use crate::model::{Roadmap, Priority};
use colored::*;

/// Display comprehensive analytics overview
pub fn display_analytics_overview(analytics: &ProgressAnalytics) {
    println!("\n{}", "═".repeat(70).bright_blue());
    println!("  {}", "📊 Project Analytics Overview".bold().bright_cyan());
    println!("{}", "═".repeat(70).bright_blue());
    
    // Overall progress section
    display_progress_section(analytics);
    
    // Velocity and performance section
    display_velocity_section(analytics);
    
    // Time tracking section
    display_time_overview(&analytics.time_analytics);
    
    // Quick phase summary
    display_phase_summary(&analytics.phase_analytics);
    
    println!("\n💡 Use {} for detailed analytics", "rask analytics --help".bright_cyan());
    println!();
}

/// Display detailed time analytics
pub fn display_time_analytics(time_analytics: &TimeAnalytics) {
    println!("\n{}", "═".repeat(70).bright_blue());
    println!("  {}", "⏱️ Time Tracking Analytics".bold().bright_cyan());
    println!("{}", "═".repeat(70).bright_blue());
    
    // Time overview
    println!("\n  📈 {}:", "Time Overview".bold());
    println!("      Total estimated: {:.1} hours", time_analytics.total_estimated_hours.to_string().bright_white());
    println!("      Total tracked: {:.1} hours", time_analytics.total_actual_hours.to_string().bright_green());
    
    let variance_color = if time_analytics.total_variance_hours > 0.0 {
        "bright_red"
    } else if time_analytics.total_variance_hours < 0.0 {
        "bright_green"
    } else {
        "bright_white"
    };
    
    println!("      Variance: {:.1} hours ({:+.1}%)", 
        time_analytics.total_variance_hours.to_string().color(variance_color),
        time_analytics.variance_percentage.to_string().color(variance_color)
    );
    
    // Estimation accuracy
    let accuracy_color = if time_analytics.estimation_accuracy >= 80.0 {
        "bright_green"
    } else if time_analytics.estimation_accuracy >= 60.0 {
        "bright_yellow"
    } else {
        "bright_red"
    };
    
    println!("      Estimation accuracy: {}%", 
        format!("{:.1}", time_analytics.estimation_accuracy).color(accuracy_color)
    );
    
    // Task breakdown
    println!("\n  📊 {}:", "Task Breakdown".bold());
    println!("      Tasks with estimates: {}", time_analytics.tasks_with_estimates.to_string().bright_white());
    println!("      Tasks with tracked time: {}", time_analytics.tasks_with_tracked_time.to_string().bright_white());
    println!("      Over-estimated tasks: {}", time_analytics.over_estimated_tasks.to_string().bright_red());
    println!("      Under-estimated tasks: {}", time_analytics.under_estimated_tasks.to_string().bright_yellow());
    println!("      Accurate estimates: {}", time_analytics.accurate_estimates.to_string().bright_green());
    
    // Session analytics
    println!("\n  🔄 {}:", "Session Analytics".bold());
    println!("      Total sessions: {}", time_analytics.total_sessions.to_string().bright_white());
    println!("      Active sessions: {}", time_analytics.active_sessions.to_string().bright_cyan());
    println!("      Average session: {:.1} hours", time_analytics.average_session_duration.to_string().bright_white());
    
    println!();
}

/// Display phase analytics
pub fn display_phase_analytics(phase_analytics: &[PhaseAnalytics]) {
    println!("\n{}", "═".repeat(70).bright_blue());
    println!("  {}", "🎯 Phase Analytics".bold().bright_cyan());
    println!("{}", "═".repeat(70).bright_blue());
    
    if phase_analytics.is_empty() {
        println!("\n  📊 No phase data available.");
        println!("  💡 Add tasks with phases: {}", "rask add \"Task\" --phase mvp".bright_cyan());
        return;
    }
    
    for phase in phase_analytics {
        println!("\n  {} {} {}:", 
            phase.phase.emoji(), 
            phase.phase.name.bold().bright_white(),
            if phase.phase.is_predefined() { "" } else { "(custom)" }.dimmed()
        );
        
        // Progress bar for phase
        let progress_bar = create_progress_bar(phase.completed_tasks, phase.total_tasks, 30);
        println!("      Progress: {} {:.1}% ({}/{})", 
            progress_bar,
            phase.completion_rate,
            phase.completed_tasks.to_string().bright_green(),
            phase.total_tasks.to_string().bright_white()
        );
        
        // Time data if available
        if phase.estimated_hours > 0.0 || phase.actual_hours > 0.0 {
            println!("      Time: Est {:.1}h | Actual {:.1}h | Variance {:+.1}h", 
                phase.estimated_hours,
                phase.actual_hours,
                phase.variance_hours
            );
        }
        
        // Task status
        if phase.ready_tasks > 0 {
            println!("      Ready to start: {}", phase.ready_tasks.to_string().bright_green());
        }
        if phase.blocked_tasks > 0 {
            println!("      Blocked: {}", phase.blocked_tasks.to_string().bright_red());
        }
    }
    
    println!();
}

/// Display priority analytics
pub fn display_priority_analytics(priority_analytics: &[PriorityAnalytics]) {
    println!("\n{}", "═".repeat(70).bright_blue());
    println!("  {}", "⚡ Priority Analytics".bold().bright_cyan());
    println!("{}", "═".repeat(70).bright_blue());
    
    if priority_analytics.is_empty() {
        println!("\n  📊 No priority data available.");
        return;
    }
    
    for priority in priority_analytics {
        let (icon, color) = match priority.priority {
            Priority::Critical => ("🔥", "bright_red"),
            Priority::High => ("⬆️", "bright_yellow"),
            Priority::Medium => ("▶️", "bright_blue"),
            Priority::Low => ("⬇️", "bright_black"),
        };
        
        println!("\n  {} {} Priority:", icon, format!("{:?}", priority.priority).color(color).bold());
        
        let progress_bar = create_progress_bar(priority.completed_tasks, priority.total_tasks, 25);
        println!("      Tasks: {} {:.1}% ({}/{})", 
            progress_bar,
            priority.completion_rate,
            priority.completed_tasks.to_string().bright_green(),
            priority.total_tasks.to_string().bright_white()
        );
        
        if priority.average_completion_time > 0.0 {
            println!("      Avg completion: {:.1} days", priority.average_completion_time);
        }
    }
    
    println!();
}

/// Display trend analytics (placeholder for future implementation)
pub fn display_trend_analytics(roadmap: &Roadmap, analytics: &ProgressAnalytics) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "═".repeat(70).bright_blue());
    println!("  {}", "📈 Trend Analytics".bold().bright_cyan());
    println!("{}", "═".repeat(70).bright_blue());
    
    // For now, show basic trend information
    println!("\n  🚀 {}:", "Project Velocity".bold());
    println!("      Tasks per day: {:.2}", analytics.velocity_tasks_per_day);
    println!("      Hours per day: {:.2}", analytics.velocity_hours_per_day);
    
    if analytics.average_task_completion_time > 0.0 {
        println!("      Avg task completion: {:.1} days", analytics.average_task_completion_time);
    }
    
    // Show project timeline if we have date data
    let tasks_with_dates = roadmap.tasks.iter()
        .filter(|t| t.created_at.is_some())
        .count();
    
    if tasks_with_dates > 0 {
        println!("\n  📅 {}:", "Timeline".bold());
        println!("      Tasks with timestamps: {}/{}", tasks_with_dates, roadmap.tasks.len());
        
        // Find project start date
        if let Some(earliest_task) = roadmap.tasks.iter()
            .filter_map(|t| t.created_at.as_ref())
            .min() {
            if let Ok(start_date) = chrono::DateTime::parse_from_rfc3339(earliest_task) {
                let days_active = (chrono::Utc::now() - start_date.with_timezone(&chrono::Utc)).num_days();
                println!("      Project active: {} days", days_active);
            }
        }
    }
    
    println!("\n💡 More detailed trend analysis coming in future updates!");
    println!();
    
    Ok(())
}

/// Display analytics summary (for export)
pub fn display_analytics_summary(analytics: &ProgressAnalytics) {
    println!("📊 Analytics Summary");
    println!("==================");
    println!("Total Tasks: {}", analytics.total_tasks);
    println!("Completed: {} ({:.1}%)", analytics.completed_tasks, analytics.completion_rate);
    println!("Pending: {}", analytics.pending_tasks);
    println!("Velocity: {:.2} tasks/day, {:.2} hours/day", 
        analytics.velocity_tasks_per_day, 
        analytics.velocity_hours_per_day
    );
    println!("Estimation Accuracy: {:.1}%", analytics.estimation_accuracy);
    println!("Time Variance: {:+.1}%", analytics.time_analytics.variance_percentage);
}

/// Helper function to display progress section
fn display_progress_section(analytics: &ProgressAnalytics) {
    println!("\n  📈 {}:", "Progress Overview".bold());
    
    let progress_bar = create_progress_bar(analytics.completed_tasks, analytics.total_tasks, 40);
    println!("      {} {:.1}% ({}/{})", 
        progress_bar,
        analytics.completion_rate,
        analytics.completed_tasks.to_string().bright_green(),
        analytics.total_tasks.to_string().bright_white()
    );
    
    let remaining = analytics.pending_tasks;
    println!("      Remaining tasks: {}", remaining.to_string().bright_yellow());
}

/// Helper function to display velocity section
fn display_velocity_section(analytics: &ProgressAnalytics) {
    println!("\n  🚀 {}:", "Velocity & Performance".bold());
    
    if analytics.velocity_tasks_per_day > 0.0 {
        println!("      Task velocity: {:.2} tasks/day", analytics.velocity_tasks_per_day.to_string().bright_green());
    }
    
    if analytics.velocity_hours_per_day > 0.0 {
        println!("      Hour velocity: {:.2} hours/day", analytics.velocity_hours_per_day.to_string().bright_green());
    }
    
    if analytics.average_task_completion_time > 0.0 {
        println!("      Avg completion time: {:.1} days", analytics.average_task_completion_time.to_string().bright_white());
    }
    
    if analytics.estimation_accuracy > 0.0 {
        let accuracy_color = if analytics.estimation_accuracy >= 80.0 {
            "bright_green"
        } else if analytics.estimation_accuracy >= 60.0 {
            "bright_yellow"
        } else {
            "bright_red"
        };
        
        println!("      Estimation accuracy: {}%", 
            format!("{:.1}", analytics.estimation_accuracy).color(accuracy_color)
        );
    }
}

/// Helper function to display time overview
fn display_time_overview(time_analytics: &TimeAnalytics) {
    if time_analytics.total_estimated_hours > 0.0 || time_analytics.total_actual_hours > 0.0 {
        println!("\n  ⏱️ {}:", "Time Tracking Summary".bold());
        
        if time_analytics.total_estimated_hours > 0.0 {
            println!("      Estimated: {:.1}h | Tracked: {:.1}h", 
                time_analytics.total_estimated_hours,
                time_analytics.total_actual_hours
            );
            
            let variance_color = if time_analytics.total_variance_hours > 0.0 {
                "bright_red"
            } else if time_analytics.total_variance_hours < 0.0 {
                "bright_green"
            } else {
                "bright_white"
            };
            
            println!("      Variance: {} ({:+.1}%)", 
                format!("{:.1}h", time_analytics.total_variance_hours).color(variance_color),
                format!("{:.1}", time_analytics.variance_percentage).color(variance_color)
            );
        }
        
        if time_analytics.total_sessions > 0 {
            println!("      Sessions: {} | Active: {}", 
                time_analytics.total_sessions,
                time_analytics.active_sessions
            );
        }
    }
}

/// Helper function to display phase summary
fn display_phase_summary(phase_analytics: &[PhaseAnalytics]) {
    if !phase_analytics.is_empty() {
        println!("\n  🎯 {}:", "Phase Summary".bold());
        
        for phase in phase_analytics.iter().take(3) { // Show top 3 phases
            let progress_indicator = if phase.completion_rate >= 80.0 {
                "🟢"
            } else if phase.completion_rate >= 50.0 {
                "🟡"
            } else {
                "🔴"
            };
            
            println!("      {} {} {}: {:.0}% ({}/{})", 
                progress_indicator,
                phase.phase.emoji(),
                phase.phase.name,
                phase.completion_rate,
                phase.completed_tasks,
                phase.total_tasks
            );
        }
        
        if phase_analytics.len() > 3 {
            println!("      ... and {} more phases", phase_analytics.len() - 3);
        }
    }
}

/// Create a visual progress bar
fn create_progress_bar(completed: usize, total: usize, width: usize) -> String {
    if total == 0 {
        return "▱".repeat(width).bright_black().to_string();
    }
    
    let percentage = completed as f64 / total as f64;
    let filled_width = (percentage * width as f64).round() as usize;
    let empty_width = width - filled_width;
    
    let filled = "█".repeat(filled_width);
    let empty = "░".repeat(empty_width);
    
    match percentage {
        p if p >= 0.8 => format!("{}{}", filled.bright_green(), empty.bright_black()),
        p if p >= 0.6 => format!("{}{}", filled.bright_yellow(), empty.bright_black()),
        p if p >= 0.4 => format!("{}{}", filled.bright_blue(), empty.bright_black()),
        _ => format!("{}{}", filled.bright_red(), empty.bright_black()),
    }
} 