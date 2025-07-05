//! AI-related CLI commands and types

use clap::Subcommand;

/// AI-related commands for intelligent task management
#[derive(Subcommand, Clone)]
pub enum AiCommands {
    /// Start an interactive chat session with the AI assistant
    Chat {
        /// Optional initial message to send to the AI
        #[arg(value_name = "MESSAGE", help = "Initial message to send to the AI assistant")]
        message: Option<String>,
        
        /// Include current project context in the conversation
        #[arg(long, help = "Include current project context in the conversation")]
        with_context: bool,
    },
    
    /// Get AI analysis and suggestions for current tasks
    Analyze {
        /// Limit the number of suggestions returned
        #[arg(long, short, value_name = "COUNT", default_value = "5", help = "Maximum number of task suggestions to show")]
        limit: usize,
        
        /// Export analysis to file instead of displaying
        #[arg(long, short, value_name = "FILE", help = "Export analysis to JSON file")]
        output: Option<String>,
        
        /// Include only tasks from specific phase
        #[arg(long, value_name = "PHASE", help = "Analyze only tasks from this phase")]
        phase: Option<String>,
    },
    
    /// Generate task breakdown from a high-level description
    Breakdown {
        /// High-level task description to break down
        #[arg(value_name = "DESCRIPTION", help = "High-level task description to break down into subtasks")]
        description: String,
        
        /// Apply the generated tasks immediately to the project
        #[arg(long, help = "Apply the generated task breakdown immediately to the project")]
        apply: bool,
        
        /// Default phase for generated tasks
        #[arg(long, value_name = "PHASE", help = "Default phase to assign to generated tasks")]
        phase: Option<String>,
    },
    
    /// Get project insights and recommendations
    Insights {
        /// Show detailed performance analysis if time tracking data is available
        #[arg(long, help = "Include detailed performance analysis using time tracking data")]
        detailed: bool,
        
        /// Export insights to file
        #[arg(long, short, value_name = "FILE", help = "Export insights to JSON file")]
        output: Option<String>,
    },
    
    /// Configure AI settings and API keys
    Configure {
        /// Set AI provider (gemini, openai, claude - future)
        #[arg(long, value_name = "PROVIDER", help = "Set AI provider: gemini")]
        provider: Option<String>,
        
        /// Set API key for the current provider
        #[arg(long, value_name = "API_KEY", help = "Set API key for the current provider")]
        api_key: Option<String>,
        
        /// Set default model to use
        #[arg(long, value_name = "MODEL", help = "Set default model to use")]
        model: Option<String>,
        
        /// Enable or disable AI features
        #[arg(long, value_name = "ENABLED", help = "Enable or disable AI features (true/false)")]
        enabled: Option<bool>,
        
        /// Set AI response temperature (0.0-1.0)
        #[arg(long, value_name = "TEMP", help = "Set AI response temperature (0.0-1.0, lower = more focused)")]
        temperature: Option<f32>,
        
        /// Show current AI configuration
        #[arg(long, help = "Show current AI configuration settings")]
        show: bool,
    },
    
    /// Get AI-powered project status summary
    Summary {
        /// Include specific recommendations for next actions
        #[arg(long, help = "Include specific recommendations for next actions")]
        with_recommendations: bool,
        
        /// Focus on specific area (tasks, risks, performance, resources)
        #[arg(long, value_name = "AREA", help = "Focus on specific area: tasks, risks, performance, resources")]
        focus: Option<String>,
    },
    
    /// Suggest next tasks based on current project state
    Suggest {
        /// Number of task suggestions to generate
        #[arg(long, short, value_name = "COUNT", default_value = "3", help = "Number of task suggestions to generate")]
        count: usize,
        
        /// Apply suggested tasks immediately to the project
        #[arg(long, help = "Apply suggested tasks immediately to the project")]
        apply: bool,
        
        /// Priority level for suggested tasks
        #[arg(long, value_name = "PRIORITY", help = "Priority level for suggested tasks: low, medium, high, critical")]
        priority: Option<String>,
        
        /// Phase for suggested tasks
        #[arg(long, value_name = "PHASE", help = "Phase to assign to suggested tasks")]
        phase: Option<String>,
    },
    
    /// Analyze roadmap file and suggest improvements or create a plan
    Roadmap {
        /// Roadmap file to analyze (defaults to current project's roadmap)
        #[arg(value_name = "FILE", help = "Roadmap markdown file to analyze")]
        file: Option<String>,
        
        /// Apply suggested improvements/tasks to the project
        #[arg(long, help = "Apply AI suggestions directly to the project")]
        apply: bool,
        
        /// Focus area for analysis (structure, priorities, phases, timeline, dependencies)
        #[arg(long, value_name = "FOCUS", help = "Focus analysis on: structure, priorities, phases, timeline, dependencies")]
        focus: Option<String>,
        
        /// Export detailed analysis to file
        #[arg(long, short, value_name = "FILE", help = "Export detailed analysis to file")]
        output: Option<String>,
        
        /// Generate a new project plan instead of analyzing existing roadmap
        #[arg(long, help = "Generate a new project plan based on requirements")]
        generate_plan: bool,
    },
}