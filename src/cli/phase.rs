use clap::Subcommand;

/// Phase management commands
#[derive(Subcommand)]
pub enum PhaseCommands {
    /// List all phases and their task counts
    List,
    
    /// Show tasks in a specific phase
    Show {
        /// Phase to show tasks for
        #[arg(help = "Phase name to display")]
        phase: String,
    },
    
    /// Set phase for a task
    Set {
        /// Task ID to update
        #[arg(value_name = "TASK_ID", help = "ID of the task to update")]
        task_id: usize,
        
        /// New phase for the task
        #[arg(help = "Phase name to set")]
        phase: String,
    },
    
    /// Show phase overview with statistics
    Overview,
    
    /// Create a new custom phase
    Create {
        /// Name of the new phase
        #[arg(help = "Name of the new phase")]
        name: String,
        
        /// Description of the phase
        #[arg(long, help = "Description of the phase")]
        description: Option<String>,
        
        /// Emoji for the phase
        #[arg(long, help = "Emoji for the phase")]
        emoji: Option<String>,
    },
} 