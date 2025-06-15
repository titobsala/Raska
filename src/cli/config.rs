use clap::Subcommand;

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