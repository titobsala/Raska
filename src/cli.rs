use clap::{Parser as ClapParser, Subcommand};
use std::path::PathBuf;

/// Main CLI structure for the Rask application
#[derive(ClapParser)]
#[command(
    name = "rask",
    version = "0.1.0",
    about = "A CLI to manage project tasks from a Markdown file",
    long_about = "Rask is a command-line project planner that helps you track tasks defined in Markdown files. \
                  It provides a beautiful, colorful interface to manage your project progress."
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
    Show,
    
    /// Mark a task as completed
    Complete { 
        /// ID of the task to mark as complete
        #[arg(value_name = "TASK_ID", help = "The ID number of the task to complete")]
        id: usize 
    },

    /// Add a new task to the project
    Add {
        /// Description of the new task to add
        #[arg(value_name = "DESCRIPTION", help = "The description of the new task")]
        description: String
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
}

/// Parse command line arguments and return the CLI structure
pub fn parse_args() -> Cli {
    Cli::parse()
} 