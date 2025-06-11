use clap::{Parser as ClapParser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// Main CLI structure for the Rask application
#[derive(ClapParser)]
#[command(
    name = "rask",
    version = "2.1.0",
    about = "An advanced CLI project planner with tags, priorities, and dependencies",
    long_about = "Rask is a powerful command-line project planner that helps you track tasks defined in Markdown files. \
                  It supports tags, priorities, task dependencies, and advanced filtering capabilities."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

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

/// Available commands for the Rask CLI
#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new project from a Markdown file
    Init { 
        /// Path to the Markdown file containing your project plan
        #[arg(value_name = "FILE", help = "The markdown file to parse")]
        filepath: PathBuf 
    },
    
    /// Show the current project status and task list
    Show,
    
    /// Mark a task as completed
    Complete { 
        /// ID of the task to mark as complete
        #[arg(value_name = "TASK_ID", help = "The ID number of the task to complete")]
        id: usize 
    },

    /// Add a new task to the project with optional metadata
    Add {
        /// Description of the new task to add
        #[arg(value_name = "DESCRIPTION", help = "The description of the new task")]
        description: String,
        
        /// Tags to categorize the task (comma-separated)
        #[arg(long, value_name = "TAGS", help = "Comma-separated tags (e.g., backend,urgent)")]
        tag: Option<String>,
        
        /// Priority level for the task
        #[arg(long, value_enum, value_name = "PRIORITY", help = "Priority level: low, medium, high, critical")]
        priority: Option<CliPriority>,
        
        /// Additional notes for the task
        #[arg(long, value_name = "NOTES", help = "Detailed notes or description for the task")]
        note: Option<String>,
        
        /// Task IDs this task depends on (comma-separated)
        #[arg(long = "depends-on", value_name = "TASK_IDS", help = "Comma-separated task IDs this task depends on")]
        dependencies: Option<String>,
    },

    /// Remove a task from the project
    Remove {
        /// ID of the task to remove
        #[arg(value_name = "TASK_ID", help = "The ID number of the task to remove")]
        id: usize
    },

    /// Edit the description of an existing task
    Edit {
        /// ID of the task to edit
        #[arg(value_name = "TASK_ID", help = "The ID number of the task to edit")]
        id: usize,
        /// New description for the task
        #[arg(value_name = "DESCRIPTION", help = "The new description for the task")]
        description: String
    },

    /// Reset task(s) to pending status
    Reset {
        /// ID of the task to reset (if not provided, resets all tasks)
        #[arg(value_name = "TASK_ID", help = "The ID number of the task to reset (optional - resets all if not provided)")]
        id: Option<usize>
    },

    /// List and filter tasks with advanced options
    List {
        /// Filter by tags (comma-separated)
        #[arg(long, value_name = "TAGS", help = "Show only tasks with these tags (comma-separated)")]
        tag: Option<String>,
        
        /// Filter by priority level
        #[arg(long, value_enum, value_name = "PRIORITY", help = "Show only tasks with this priority")]
        priority: Option<CliPriority>,
        
        /// Filter by status
        #[arg(long, value_name = "STATUS", help = "Filter by status: pending, completed, all")]
        status: Option<String>,
        
        /// Search in task descriptions and notes
        #[arg(long, value_name = "QUERY", help = "Search for text in task descriptions and notes")]
        search: Option<String>,
        
        /// Show detailed information including notes
        #[arg(long, help = "Show detailed task information including notes and dependencies")]
        detailed: bool,
    },

    /// Manage projects (multi-project support)
    #[command(subcommand)]
    Project(ProjectCommands),
}

/// Project management commands
#[derive(Subcommand)]
pub enum ProjectCommands {
    /// Create a new project
    Create {
        /// Name of the project
        #[arg(value_name = "NAME", help = "Name of the new project")]
        name: String,
        
        /// Description of the project
        #[arg(long, value_name = "DESCRIPTION", help = "Description of the project")]
        description: Option<String>,
    },
    
    /// Switch to a different project
    Switch {
        /// Name of the project to switch to
        #[arg(value_name = "NAME", help = "Name of the project to switch to")]
        name: String,
    },
    
    /// List all projects
    List,
    
    /// Delete a project
    Delete {
        /// Name of the project to delete
        #[arg(value_name = "NAME", help = "Name of the project to delete")]
        name: String,
        
        /// Force deletion without confirmation
        #[arg(long, help = "Force deletion without confirmation")]
        force: bool,
    },
}

/// Parse command line arguments and return the CLI structure
pub fn parse_args() -> Cli {
    Cli::parse()
} 