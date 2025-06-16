use clap::{Parser as ClapParser, Subcommand};
use std::path::PathBuf;

// Import all the modularized CLI components
pub mod types;
pub mod project;
pub mod phase;
pub mod config;
pub mod notes;
pub mod bulk;
pub mod template;

// Re-export the types for easier access
pub use types::{CliPriority, ExportFormat};
pub use project::ProjectCommands;
pub use phase::PhaseCommands;
pub use config::ConfigCommands;
pub use notes::NotesCommands;
pub use bulk::BulkCommands;
pub use template::TemplateCommands;

/// Main CLI structure for the Rask application
#[derive(ClapParser)]
#[command(
    name = "rask",
    version = "2.6.0",
    about = "An advanced CLI project planner with tags, priorities, dependencies, phases, and templates",
    long_about = "Rask is a powerful command-line project planner that helps you track tasks defined in Markdown files. \
                  It supports tags, priorities, task dependencies, custom phases, task templates, and advanced filtering capabilities."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
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
    #[command(alias = "status")]
    Show {
        /// Group tasks by phase for better organization
        #[arg(long, help = "Group tasks by phase (MVP, Beta, Release, etc.)")]
        group_by_phase: bool,
        
        /// Show only tasks from a specific phase
        #[arg(long, value_name = "PHASE", help = "Show only tasks from this phase")]
        phase: Option<String>,
        
        /// Show detailed information including notes and dependencies
        #[arg(long, help = "Show detailed task information including notes and dependencies")]
        detailed: bool,
        
        /// Collapse completed phases to focus on active work
        #[arg(long, help = "Collapse completed phases to reduce visual clutter")]
        collapse_completed: bool,
    },
    
    /// Mark a task as completed
    #[command(alias = "done")]
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
        
        /// Phase for the task
        #[arg(long, value_name = "PHASE", help = "Phase name (e.g., mvp, beta, release, future, backlog, or custom name)")]
        phase: Option<String>,
        
        /// Additional notes for the task
        #[arg(long, value_name = "NOTES", help = "Detailed notes or description for the task")]
        note: Option<String>,
        
        /// Task IDs this task depends on (comma-separated)
        #[arg(long = "depends-on", value_name = "TASK_IDS", help = "Comma-separated task IDs this task depends on")]
        dependencies: Option<String>,
        
        /// Estimated time to complete the task in hours
        #[arg(long, value_name = "HOURS", help = "Estimated time to complete the task in hours (e.g., 2.5)")]
        estimated_hours: Option<f64>,
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
    #[command(alias = "ls")]
    List {
        /// Filter by tags (comma-separated)
        #[arg(long, value_name = "TAGS", help = "Show only tasks with these tags (comma-separated)")]
        tag: Option<String>,
        
        /// Filter by priority level
        #[arg(long, value_enum, value_name = "PRIORITY", help = "Show only tasks with this priority")]
        priority: Option<CliPriority>,
        
        /// Filter by phase
        #[arg(long, value_name = "PHASE", help = "Show only tasks in this phase")]
        phase: Option<String>,
        
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

    /// Analyze and visualize task dependencies
    Dependencies {
        /// Show dependency tree for a specific task
        #[arg(long, value_name = "TASK_ID", help = "Show dependency tree for a specific task")]
        task_id: Option<usize>,
        
        /// Validate all dependencies for issues
        #[arg(long, help = "Validate all dependencies and show any issues")]
        validate: bool,
        
        /// Show tasks ready to be started
        #[arg(long, help = "Show tasks that are ready to be started")]
        show_ready: bool,
        
        /// Show tasks blocked by dependencies
        #[arg(long, help = "Show tasks blocked by incomplete dependencies")]
        show_blocked: bool,
    },

    /// Manage and view project phases
    #[command(subcommand)]
    Phase(PhaseCommands),

    /// Manage configuration settings
    #[command(subcommand)]
    Config(ConfigCommands),

    /// View detailed information about a specific task
    View {
        /// ID of the task to view in detail
        #[arg(value_name = "TASK_ID", help = "The ID number of the task to view")]
        id: usize,
    },

    /// Perform bulk operations on multiple tasks
    #[command(subcommand)]
    Bulk(BulkCommands),

    /// Manage implementation notes for tasks
    #[command(subcommand)]
    Notes(NotesCommands),

    /// Export roadmap to different formats with advanced time-based filtering
    Export {
        /// Output format
        #[arg(value_enum, help = "Export format: json, csv, or html")]
        format: ExportFormat,
        
        /// Output file path (optional, defaults to stdout)
        #[arg(short, long, value_name = "FILE", help = "Output file path")]
        output: Option<PathBuf>,
        
        /// Include completed tasks
        #[arg(long, help = "Include completed tasks in export")]
        include_completed: bool,
        
        /// Include only specific tags (comma-separated)
        #[arg(long, value_name = "TAGS", help = "Export only tasks with these tags")]
        tags: Option<String>,
        
        /// Include only specific priority
        #[arg(long, value_enum, help = "Export only tasks with this priority")]
        priority: Option<CliPriority>,
        
        /// Include only specific phase
        #[arg(long, help = "Export only tasks in this phase")]
        phase: Option<String>,
        
        /// Pretty print JSON output
        #[arg(long, help = "Pretty print JSON output")]
        pretty: bool,
        
        // NEW: Time-based filtering options for Phase 3 enhancement
        /// Filter tasks created after this date (YYYY-MM-DD format)
        #[arg(long, value_name = "DATE", help = "Include only tasks created after this date (YYYY-MM-DD)")]
        created_after: Option<String>,
        
        /// Filter tasks created before this date (YYYY-MM-DD format)
        #[arg(long, value_name = "DATE", help = "Include only tasks created before this date (YYYY-MM-DD)")]
        created_before: Option<String>,
        
        /// Filter tasks with estimated hours greater than threshold
        #[arg(long, value_name = "HOURS", help = "Include only tasks with estimated hours greater than this value")]
        min_estimated_hours: Option<f64>,
        
        /// Filter tasks with estimated hours less than threshold
        #[arg(long, value_name = "HOURS", help = "Include only tasks with estimated hours less than this value")]
        max_estimated_hours: Option<f64>,
        
        /// Filter tasks with actual hours greater than threshold
        #[arg(long, value_name = "HOURS", help = "Include only tasks with actual hours greater than this value")]
        min_actual_hours: Option<f64>,
        
        /// Filter tasks with actual hours less than threshold
        #[arg(long, value_name = "HOURS", help = "Include only tasks with actual hours less than this value")]
        max_actual_hours: Option<f64>,
        
        /// Include only tasks with time tracking data
        #[arg(long, help = "Include only tasks that have time tracking data (estimates or actual time)")]
        with_time_data: bool,
        
        /// Include only tasks with active time sessions
        #[arg(long, help = "Include only tasks with currently active time tracking sessions")]
        active_sessions_only: bool,
        
        /// Include only over-estimated tasks
        #[arg(long, help = "Include only tasks that took longer than estimated")]
        over_estimated_only: bool,
        
        /// Include only under-estimated tasks
        #[arg(long, help = "Include only tasks that took less time than estimated")]
        under_estimated_only: bool,
    },

    /// Manage task templates for quick task creation
    #[command(subcommand)]
    Template(TemplateCommands),

    /// Start time tracking for a task
    Start {
        /// ID of the task to start tracking time for
        #[arg(value_name = "TASK_ID", help = "The ID number of the task to start time tracking")]
        id: usize,
        
        /// Optional description of what will be worked on
        #[arg(long, value_name = "DESCRIPTION", help = "Description of what will be worked on during this session")]
        description: Option<String>,
    },

    /// Stop time tracking for the currently active task
    Stop,

    /// View time tracking information for tasks
    Time {
        /// Show time information for a specific task
        #[arg(value_name = "TASK_ID", help = "Show time information for a specific task")]
        task_id: Option<usize>,
        
        /// Show summary of time tracking across all tasks
        #[arg(long, help = "Show time tracking summary for all tasks")]
        summary: bool,
        
        /// Show detailed time session history
        #[arg(long, help = "Show detailed time session history")]
        detailed: bool,
    },

    /// View comprehensive project analytics and progress reports
    #[command(alias = "stats")]
    Analytics {
        /// Show overview analytics (default)
        #[arg(long, help = "Show comprehensive analytics overview")]
        overview: bool,
        
        /// Show detailed time tracking analytics
        #[arg(long, help = "Show detailed time tracking analytics")]
        time: bool,
        
        /// Show phase-based analytics
        #[arg(long, help = "Show analytics broken down by phases")]
        phases: bool,
        
        /// Show priority-based analytics
        #[arg(long, help = "Show analytics broken down by priorities")]
        priorities: bool,
        
        /// Show trend analytics and velocity metrics
        #[arg(long, help = "Show trend analytics and project velocity")]
        trends: bool,
        
        /// Export analytics to file
        #[arg(long, value_name = "FILE", help = "Export analytics summary to file")]
        export: Option<PathBuf>,
        
        /// Show all analytics sections
        #[arg(long, help = "Show all available analytics sections")]
        all: bool,
    },

    /// Show project timeline with phase-based horizontal layout
    Timeline {
        /// Show detailed task information in timeline
        #[arg(long, help = "Show detailed task information in timeline view")]
        detailed: bool,
        
        /// Show only active phases (hide empty phases)
        #[arg(long, help = "Show only phases that contain tasks")]
        active_only: bool,
        
        /// Compact view with fewer details per task
        #[arg(long, help = "Use compact view to fit more information")]
        compact: bool,
    },
}

/// Parse command line arguments and return the CLI structure
pub fn parse_args() -> Cli {
    Cli::parse()
} 