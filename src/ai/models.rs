//! AI data models and types

use serde::{Deserialize, Serialize};
use crate::model::{Priority, Phase};

/// AI analysis of tasks with suggestions and insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiTaskAnalysis {
    /// Overall project health score (0-100)
    pub health_score: u8,
    
    /// General insights about the project
    pub insights: Vec<String>,
    
    /// Specific task suggestions
    pub task_suggestions: Vec<AiTaskSuggestion>,
    
    /// Workflow recommendations
    pub workflow_recommendations: Vec<String>,
    
    /// Potential issues or blockers identified
    pub potential_issues: Vec<String>,
}

/// AI-generated task suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiTaskSuggestion {
    /// Suggested task description
    pub description: String,
    
    /// Suggested priority
    pub priority: Priority,
    
    /// Suggested phase
    pub phase: Phase,
    
    /// Suggested tags
    pub tags: Vec<String>,
    
    /// Estimated effort in hours
    pub estimated_hours: Option<f64>,
    
    /// Dependencies (described, not IDs since these are new tasks)
    pub dependencies: Vec<String>,
    
    /// Implementation notes
    pub notes: Option<String>,
    
    /// Reasoning for this suggestion
    pub reasoning: String,
}

/// Project-level insights from AI analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiProjectInsights {
    /// Project completion assessment
    pub completion_assessment: String,
    
    /// Critical path analysis
    pub critical_path: Vec<String>,
    
    /// Resource allocation suggestions
    pub resource_suggestions: Vec<String>,
    
    /// Risk assessment
    pub risks: Vec<AiRisk>,
    
    /// Next recommended actions
    pub next_actions: Vec<String>,
    
    /// Performance metrics and trends
    pub performance_insights: Option<AiPerformanceInsights>,
}

/// AI-identified risk in the project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiRisk {
    /// Risk description
    pub description: String,
    
    /// Severity level (Low, Medium, High, Critical)
    pub severity: String,
    
    /// Suggested mitigation strategies
    pub mitigation: Vec<String>,
    
    /// Affected tasks or areas
    pub affected_areas: Vec<String>,
}

/// Performance insights from AI analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiPerformanceInsights {
    /// Time estimation accuracy
    pub estimation_accuracy: Option<f64>,
    
    /// Most efficient phases or areas
    pub efficient_areas: Vec<String>,
    
    /// Areas needing improvement
    pub improvement_areas: Vec<String>,
    
    /// Productivity trends
    pub productivity_trends: String,
}

/// Chat message for AI conversations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiChatMessage {
    /// Unique message ID
    pub id: String,
    
    /// Message content
    pub content: String,
    
    /// Whether this is from the user or AI
    pub is_user: bool,
    
    /// Timestamp
    pub timestamp: String,
    
    /// Optional metadata (e.g., model used, tokens, etc.)
    pub metadata: Option<AiMessageMetadata>,
}

/// Metadata for AI messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiMessageMetadata {
    /// Model used for generation
    pub model: String,
    
    /// Number of tokens in request
    pub input_tokens: Option<u32>,
    
    /// Number of tokens in response
    pub output_tokens: Option<u32>,
    
    /// Processing time in milliseconds
    pub processing_time: Option<u64>,
}

/// Chat conversation context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiChatContext {
    /// Session ID
    pub session_id: String,
    
    /// Message history
    pub messages: Vec<AiChatMessage>,
    
    /// Current project context
    pub project_context: Option<String>,
    
    /// Relevant tasks for context
    pub relevant_tasks: Vec<String>,
    
    /// Created timestamp
    pub created_at: String,
    
    /// Last updated timestamp
    pub updated_at: String,
}

impl AiChatContext {
    /// Create a new chat context
    pub fn new() -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            messages: Vec::new(),
            project_context: None,
            relevant_tasks: Vec::new(),
            created_at: now.clone(),
            updated_at: now,
        }
    }
    
    /// Add a user message
    pub fn add_user_message(&mut self, content: String) {
        let message = AiChatMessage {
            id: uuid::Uuid::new_v4().to_string(),
            content,
            is_user: true,
            timestamp: chrono::Utc::now().to_rfc3339(),
            metadata: None,
        };
        self.messages.push(message);
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }
    
    /// Add an AI response
    pub fn add_ai_response(&mut self, content: String, metadata: Option<AiMessageMetadata>) {
        let message = AiChatMessage {
            id: uuid::Uuid::new_v4().to_string(),
            content,
            is_user: false,
            timestamp: chrono::Utc::now().to_rfc3339(),
            metadata,
        };
        self.messages.push(message);
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }
    
    /// Get recent conversation context for API calls (last N messages)
    pub fn get_conversation_context(&self, max_messages: usize) -> Vec<&AiChatMessage> {
        self.messages
            .iter()
            .rev()
            .take(max_messages)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect()
    }
    
    /// Set project context
    pub fn set_project_context(&mut self, context: String) {
        self.project_context = Some(context);
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }
}

/// AI-generated template structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiTemplateGeneration {
    /// Template name
    pub name: String,
    
    /// Template description (may include placeholders)
    pub description: String,
    
    /// Suggested tags
    pub tags: Vec<String>,
    
    /// Suggested priority
    pub priority: String,
    
    /// Suggested phase
    pub phase: String,
    
    /// Template category
    pub category: String,
    
    /// Implementation guidance
    pub implementation_notes: Vec<String>,
    
    /// Usage examples
    pub usage_examples: Vec<String>,
    
    /// AI reasoning for this template
    pub reasoning: String,
}

/// AI template suggestion for existing projects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiTemplateSuggestion {
    /// Suggested template name
    pub name: String,
    
    /// Description of what this template would do
    pub description: String,
    
    /// Template category
    pub category: String,
    
    /// Suggested priority level
    pub priority: String,
    
    /// Reasoning for this suggestion
    pub reasoning: String,
    
    /// Usefulness score (0-100)
    pub usefulness_score: u8,
}

/// AI template enhancement results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiTemplateEnhancement {
    /// Enhanced description
    pub enhanced_description: String,
    
    /// Additional tags to add
    pub additional_tags: Vec<String>,
    
    /// Enhanced implementation notes
    pub enhanced_implementation_notes: Vec<String>,
    
    /// Usage examples
    pub usage_examples: Vec<String>,
    
    /// Common pitfalls to avoid
    pub common_pitfalls: Vec<String>,
    
    /// Acceptance criteria
    pub acceptance_criteria: Vec<String>,
    
    /// Summary of improvements made
    pub improvements_summary: String,
}