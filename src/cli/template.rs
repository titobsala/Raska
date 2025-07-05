use clap::Subcommand;
use std::path::PathBuf;
use super::types::CliPriority;

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
    
    /// Generate templates using AI based on project context
    Generate {
        /// Description of the templates to generate
        #[arg(value_name = "DESCRIPTION", help = "Description of what kind of templates to generate")]
        description: String,
        
        /// Number of templates to generate
        #[arg(long, default_value = "3", help = "Number of templates to generate")]
        count: usize,
        
        /// Category for generated templates
        #[arg(long, help = "Category for generated templates")]
        category: Option<String>,
        
        /// Phase for generated templates
        #[arg(long, help = "Phase for generated templates")]
        phase: Option<String>,
        
        /// Apply generated templates to the project
        #[arg(long, help = "Automatically save generated templates")]
        apply: bool,
    },
    
    /// Get AI suggestions for relevant templates based on current project
    Suggest {
        /// Limit number of suggestions
        #[arg(long, default_value = "5", help = "Number of template suggestions to show")]
        limit: usize,
        
        /// Category filter for suggestions
        #[arg(long, help = "Filter suggestions by category")]
        category: Option<String>,
        
        /// Show detailed suggestions
        #[arg(long, help = "Show detailed suggestion reasoning")]
        detailed: bool,
    },
    
    /// Use AI to enhance an existing template with better details
    Enhance {
        /// Name of the template to enhance
        #[arg(value_name = "NAME", help = "Name of the template to enhance")]
        name: String,
        
        /// Apply the enhanced template (replace original)
        #[arg(long, help = "Replace the original template with the enhanced version")]
        apply: bool,
    },

    /// Generate a new project roadmap from a template
    Roadmap {
        /// Name of the roadmap template to use
        #[arg(value_name = "TEMPLATE_NAME", help = "Name of the roadmap template to use")]
        template_name: String,

        /// Name of the new project
        #[arg(value_name = "PROJECT_NAME", help = "Name of the new project")]
        project_name: String,
    },
} 