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
}

/// Parse command line arguments and return the CLI structure
pub fn parse_args() -> Cli {
    Cli::parse()
} 