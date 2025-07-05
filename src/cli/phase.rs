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
    
    /// Fork (duplicate) tasks from a phase or specific tasks into a new phase
    Fork {
        /// Name of the new phase to create for the forked tasks
        #[arg(value_name = "NEW_PHASE", help = "Name of the new phase to create")]
        new_phase: String,
        
        /// Source phase to fork tasks from (if not specified, uses task IDs)
        #[arg(long, value_name = "FROM_PHASE", help = "Source phase to fork all tasks from")]
        from_phase: Option<String>,
        
        /// Specific task IDs to fork (comma-separated)
        #[arg(long, value_name = "TASK_IDS", help = "Comma-separated list of task IDs to fork")]
        task_ids: Option<String>,
        
        /// Description for the new phase
        #[arg(long, help = "Description for the new phase")]
        description: Option<String>,
        
        /// Emoji for the new phase
        #[arg(long, help = "Emoji for the new phase")]
        emoji: Option<String>,
        
        /// Keep original tasks (copy instead of move)
        #[arg(long, help = "Keep original tasks in their current phase (copy instead of move)")]
        copy: bool,
    },
} 