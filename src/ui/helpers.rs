use crate::model::Priority;
use colored::*;

/// Get priority indicator with appropriate color
pub fn get_priority_indicator(priority: &Priority) -> colored::ColoredString {
    match priority {
        Priority::Critical => "🔥".red(),
        Priority::High => "⬆️".bright_red(),
        Priority::Medium => "▶️".yellow(),
        Priority::Low => "⬇️".green(),
    }
}

/// Get priority color for task text based on priority level
pub fn get_priority_color(priority: &Priority) -> fn(&str) -> colored::ColoredString {
    match priority {
        Priority::Critical => |s: &str| s.bright_red().bold(),
        Priority::High => |s: &str| s.red(),
        Priority::Medium => |s: &str| s.normal(),
        Priority::Low => |s: &str| s.bright_black(),
    }
}