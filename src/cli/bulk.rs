use clap::Subcommand;
use super::types::CliPriority;

/// Bulk operations on multiple tasks
#[derive(Subcommand)]
pub enum BulkCommands {
    /// Complete multiple tasks at once
    Complete {
        /// Comma-separated list of task IDs to complete
        #[arg(value_name = "IDS", help = "Task IDs separated by commas (e.g., 1,2,3)")]
        ids: String,
    },
    
    /// Add tags to multiple tasks
    AddTags {
        /// Comma-separated list of task IDs
        #[arg(value_name = "IDS", help = "Task IDs separated by commas")]
        ids: String,
        
        /// Comma-separated list of tags to add
        #[arg(value_name = "TAGS", help = "Tags separated by commas")]
        tags: String,
    },
    
    /// Remove tags from multiple tasks
    RemoveTags {
        /// Comma-separated list of task IDs
        #[arg(value_name = "IDS", help = "Task IDs separated by commas")]
        ids: String,
        
        /// Comma-separated list of tags to remove
        #[arg(value_name = "TAGS", help = "Tags separated by commas")]
        tags: String,
    },
    
    /// Set priority for multiple tasks
    SetPriority {
        /// Comma-separated list of task IDs
        #[arg(value_name = "IDS", help = "Task IDs separated by commas")]
        ids: String,
        
        /// Priority level to set
        #[arg(value_enum, help = "Priority level")]
        priority: CliPriority,
    },
    
    /// Set phase for multiple tasks
    SetPhase {
        /// Comma-separated list of task IDs
        #[arg(value_name = "IDS", help = "Task IDs separated by commas")]
        ids: String,
        
        /// Phase to set
        #[arg(help = "Phase name")]
        phase: String,
    },
    
    /// Reset multiple tasks to pending status
    Reset {
        /// Comma-separated list of task IDs to reset
        #[arg(value_name = "IDS", help = "Task IDs separated by commas")]
        ids: String,
    },
    
    /// Remove multiple tasks (with dependency validation)
    Remove {
        /// Comma-separated list of task IDs to remove
        #[arg(value_name = "IDS", help = "Task IDs separated by commas")]
        ids: String,
        
        /// Force removal even if other tasks depend on these
        #[arg(long, help = "Force removal even with dependencies")]
        force: bool,
    },
} 