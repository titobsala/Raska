use clap::{Parser as ClapParser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// Main CLI structure for the Rask application
#[derive(ClapParser)]
#[command(
    name = "rask",
    version = "2.3.0",
    about = "An advanced CLI project planner with tags, priorities, dependencies, phases, and templates",
    long_about = "Rask is a powerful command-line project planner that helps you track tasks defined in Markdown files. \
                  It supports tags, priorities, task dependencies, custom phases, task templates, and advanced filtering capabilities."
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
    #[command(alias = "status")]
    Show,
    
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

    /// Export roadmap to different formats
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
    },

    /// Manage task templates for quick task creation
    #[command(subcommand)]
    Template(TemplateCommands),
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
    
    /// Interactive project switcher interface
    Switcher,
    
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

/// Configuration management commands
#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Show current configuration
    Show {
        /// Show configuration for a specific section
        #[arg(value_name = "SECTION", help = "Configuration section to show (ui, behavior, export, advanced, theme)")]
        section: Option<String>,
    },
    
    /// Set a configuration value
    Set {
        /// Configuration key in format 'section.key'
        #[arg(value_name = "KEY", help = "Configuration key (e.g., ui.color_scheme, behavior.default_priority)")]
        key: String,
        
        /// Value to set
        #[arg(value_name = "VALUE", help = "Value to set for the configuration key")]
        value: String,
        
        /// Set in project config instead of user config
        #[arg(long, help = "Set in project-specific configuration")]
        project: bool,
    },
    
    /// Get a configuration value
    Get {
        /// Configuration key in format 'section.key'
        #[arg(value_name = "KEY", help = "Configuration key to get")]
        key: String,
    },
    
    /// Edit configuration in your default editor
    Edit {
        /// Edit project config instead of user config
        #[arg(long, help = "Edit project-specific configuration")]
        project: bool,
    },
    
    /// Initialize configuration files
    Init {
        /// Initialize project config
        #[arg(long, help = "Initialize project-specific configuration")]
        project: bool,
        
        /// Initialize user config
        #[arg(long, help = "Initialize user configuration")]
        user: bool,
    },
    
    /// Reset configuration to defaults
    Reset {
        /// Reset project config
        #[arg(long, help = "Reset project-specific configuration")]
        project: bool,
        
        /// Reset user config
        #[arg(long, help = "Reset user configuration")]
        user: bool,
        
        /// Force reset without confirmation
        #[arg(long, help = "Force reset without confirmation")]
        force: bool,
    },
}

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

/// Template management commands
#[derive(Subcommand, Clone)]
pub enum TemplateCommands {
    /// List all available templates
    List {
        /// Filter by category
        #[arg(long, value_name = "CATEGORY", help = "Filter templates by category")]
        category: Option<String>,
        
        /// Show detailed template information
        #[arg(long, help = "Show detailed template information")]
        detailed: bool,
    },
    
    /// Show details of a specific template
    Show {
        /// Name of the template to show
        #[arg(value_name = "NAME", help = "Name of the template to show")]
        name: String,
    },
    
    /// Create a new task from a template
    Use {
        /// Name of the template to use
        #[arg(value_name = "TEMPLATE_NAME", help = "Name of the template to use")]
        template_name: String,
        
        /// Custom description for the task (overrides template description)
        #[arg(value_name = "DESCRIPTION", help = "Custom description for the task")]
        description: Option<String>,
        
        /// Additional tags to add (comma-separated)
        #[arg(long, value_name = "TAGS", help = "Additional tags to add to the task")]
        add_tags: Option<String>,
        
        /// Override template priority
        #[arg(long, value_enum, help = "Override template priority")]
        priority: Option<CliPriority>,
        
        /// Override template phase
        #[arg(long, help = "Override template phase")]
        phase: Option<String>,
    },
    
    /// Create a new custom template
    Create {
        /// Name of the new template
        #[arg(value_name = "NAME", help = "Name of the new template")]
        name: String,
        
        /// Description for the template
        #[arg(value_name = "DESCRIPTION", help = "Description for the template")]
        description: String,
        
        /// Tags for the template (comma-separated)
        #[arg(long, value_name = "TAGS", help = "Tags for the template")]
        tags: Option<String>,
        
        /// Priority for the template
        #[arg(long, value_enum, help = "Priority for the template")]
        priority: Option<CliPriority>,
        
        /// Phase for the template
        #[arg(long, help = "Phase for the template")]
        phase: Option<String>,
        
        /// Notes for the template
        #[arg(long, help = "Notes for the template")]
        notes: Option<String>,
        
        /// Category for the template
        #[arg(long, help = "Category for the template")]
        category: Option<String>,
    },
    
    /// Delete a custom template
    Delete {
        /// Name of the template to delete
        #[arg(value_name = "NAME", help = "Name of the template to delete")]
        name: String,
        
        /// Force deletion without confirmation
        #[arg(long, help = "Force deletion without confirmation")]
        force: bool,
    },
    
    /// Export templates to a file
    Export {
        /// Output file path
        #[arg(value_name = "FILE", help = "Output file path")]
        output: PathBuf,
        
        /// Pretty print JSON output
        #[arg(long, help = "Pretty print JSON output")]
        pretty: bool,
    },
    
    /// Import templates from a file
    Import {
        /// Input file path
        #[arg(value_name = "FILE", help = "Input file path")]
        input: PathBuf,
        
        /// Merge with existing templates instead of replacing
        #[arg(long, help = "Merge with existing templates")]
        merge: bool,
    },
    
    /// Show template help and examples
    Examples,
}

/// Parse command line arguments and return the CLI structure
pub fn parse_args() -> Cli {
    Cli::parse()
} 