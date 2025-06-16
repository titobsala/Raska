use clap::Subcommand;

/// Implementation notes management commands
#[derive(Subcommand)]
pub enum NotesCommands {
    /// Add an implementation note to a task
    Add {
        /// Task ID to add note to
        #[arg(value_name = "TASK_ID", help = "ID of the task to add implementation note to")]
        task_id: usize,
        
        /// Implementation note content
        #[arg(value_name = "NOTE", help = "Implementation note content (code snippets, technical details, etc.)")]
        note: String,
    },
    
    /// List all implementation notes for a task
    List {
        /// Task ID to show notes for
        #[arg(value_name = "TASK_ID", help = "ID of the task to show implementation notes for")]
        task_id: usize,
    },
    
    /// Remove an implementation note from a task
    Remove {
        /// Task ID to remove note from
        #[arg(value_name = "TASK_ID", help = "ID of the task to remove implementation note from")]
        task_id: usize,
        
        /// Index of the note to remove (0-based)
        #[arg(value_name = "INDEX", help = "Index of the implementation note to remove (0-based)")]
        index: usize,
    },
    
    /// Clear all implementation notes from a task
    Clear {
        /// Task ID to clear notes from
        #[arg(value_name = "TASK_ID", help = "ID of the task to clear all implementation notes from")]
        task_id: usize,
    },
    
    /// Edit an implementation note
    Edit {
        /// Task ID containing the note
        #[arg(value_name = "TASK_ID", help = "ID of the task containing the implementation note")]
        task_id: usize,
        
        /// Index of the note to edit (0-based)
        #[arg(value_name = "INDEX", help = "Index of the implementation note to edit (0-based)")]
        index: usize,
        
        /// New content for the note
        #[arg(value_name = "NOTE", help = "New content for the implementation note")]
        note: String,
    },
} 