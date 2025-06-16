use clap::ValueEnum;

/// Priority levels for tasks
#[derive(Clone, Debug, ValueEnum)]
pub enum CliPriority {
    Low,
    Medium,
    High,
    Critical,
}

impl From<CliPriority> for crate::model::Priority {
    fn from(cli_priority: CliPriority) -> Self {
        match cli_priority {
            CliPriority::Low => crate::model::Priority::Low,
            CliPriority::Medium => crate::model::Priority::Medium,
            CliPriority::High => crate::model::Priority::High,
            CliPriority::Critical => crate::model::Priority::Critical,
        }
    }
}

/// Export format options
#[derive(ValueEnum, Clone)]
pub enum ExportFormat {
    /// JSON format
    Json,
    /// CSV format  
    Csv,
    /// HTML format
    Html,
} 