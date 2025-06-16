use clap::Subcommand;

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