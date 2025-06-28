//! AI service orchestration and management

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::config::RaskConfig;
use crate::model::{Task, Roadmap};
use super::{AiProvider, AiChatContext, AiTaskAnalysis, AiTaskSuggestion, AiProjectInsights, create_ai_provider};
use super::models::{AiTemplateGeneration, AiTemplateSuggestion, AiTemplateEnhancement};

/// High-level AI service that manages providers and conversations
pub struct AiService {
    provider: Arc<dyn AiProvider + Send + Sync>,
    config: RaskConfig,
    current_context: Arc<RwLock<Option<AiChatContext>>>,
}

impl AiService {
    /// Create a new AI service
    pub async fn new(config: RaskConfig) -> Result<Self> {
        if !config.ai.is_ready() {
            anyhow::bail!("AI is not properly configured. Please set up your API key and enable AI features.");
        }

        let provider = create_ai_provider(&config.ai)?;
        
        Ok(Self {
            provider: Arc::from(provider),
            config,
            current_context: Arc::new(RwLock::new(None)),
        })
    }

    /// Check if the AI service is ready to use
    pub fn is_ready(&self) -> bool {
        self.config.ai.is_ready() && self.provider.is_ready()
    }

    /// Get the current provider name
    pub fn provider_name(&self) -> &str {
        self.provider.provider_name()
    }

    /// Start a new chat session
    pub async fn start_chat_session(&self, project_context: Option<String>) -> Result<String> {
        let mut context = AiChatContext::new();
        
        if let Some(ctx) = project_context {
            context.set_project_context(ctx);
        }

        let session_id = context.session_id.clone();
        
        let mut current_context = self.current_context.write().await;
        *current_context = Some(context);
        
        Ok(session_id)
    }

    /// Send a chat message and get a response
    pub async fn chat(&self, message: String) -> Result<String> {
        let context_for_ai = {
            let current_context = self.current_context.read().await;
            current_context.as_ref().and_then(|ctx| ctx.project_context.clone())
        };

        // Get AI response
        let response = self.provider.chat(&message, context_for_ai.as_deref()).await?;

        // Update conversation history
        {
            let mut current_context = self.current_context.write().await;
            if let Some(ref mut ctx) = *current_context {
                ctx.add_user_message(message);
                ctx.add_ai_response(response.clone(), None);
            }
        }

        Ok(response)
    }

    /// Get the current chat context
    pub async fn get_chat_context(&self) -> Option<AiChatContext> {
        let current_context = self.current_context.read().await;
        current_context.clone()
    }

    /// Clear the current chat session
    pub async fn clear_chat_session(&self) {
        let mut current_context = self.current_context.write().await;
        *current_context = None;
    }

    /// Analyze tasks and get AI insights
    pub async fn analyze_tasks(&self, tasks: &[Task]) -> Result<AiTaskAnalysis> {
        self.provider.analyze_tasks(tasks).await
    }

    /// Generate task breakdown from a description
    pub async fn generate_task_breakdown(&self, description: &str) -> Result<Vec<AiTaskSuggestion>> {
        self.provider.generate_task_breakdown(description).await
    }

    /// Get project insights
    pub async fn get_project_insights(&self, roadmap: &Roadmap) -> Result<AiProjectInsights> {
        self.provider.get_project_insights(roadmap).await
    }

    /// Quick task suggestion based on current project state
    pub async fn suggest_next_tasks(&self, roadmap: &Roadmap, limit: usize) -> Result<Vec<AiTaskSuggestion>> {
        let analysis = self.analyze_tasks(&roadmap.tasks).await?;
        Ok(analysis.task_suggestions.into_iter().take(limit).collect())
    }

    /// Get AI-powered project status summary
    pub async fn get_project_summary(&self, roadmap: &Roadmap) -> Result<String> {
        let insights = self.get_project_insights(roadmap).await?;
        
        let summary = format!(
            "📊 Project Status: {}\n\n\
            🎯 Next Actions:\n{}\n\n\
            ⚠️  Potential Issues:\n{}\n\n\
            💡 Recommendations:\n{}",
            insights.completion_assessment,
            insights.next_actions.iter()
                .enumerate()
                .map(|(i, action)| format!("{}. {}", i + 1, action))
                .collect::<Vec<_>>()
                .join("\n"),
            insights.risks.iter()
                .map(|risk| format!("• {} ({})", risk.description, risk.severity))
                .collect::<Vec<_>>()
                .join("\n"),
            insights.resource_suggestions.iter()
                .enumerate()
                .map(|(i, suggestion)| format!("{}. {}", i + 1, suggestion))
                .collect::<Vec<_>>()
                .join("\n")
        );

        Ok(summary)
    }

    /// Generate template suggestions based on project context
    pub async fn generate_templates(&self, description: &str, count: usize, roadmap: Option<&Roadmap>) -> Result<Vec<AiTemplateGeneration>> {
        let context = roadmap.map(|r| utils::create_project_context(r));
        
        let prompt = format!(
            "Generate {} task template(s) based on this description: \"{}\"\n\n\
            {}Context: {}\n\n\
            For each template, provide:\n\
            1. A specific, actionable name\n\
            2. A clear description with placeholders where appropriate\n\
            3. Relevant tags (as array)\n\
            4. Appropriate priority (Critical, High, Medium, Low)\n\
            5. Suggested phase name\n\
            6. Category (Development, Testing, Documentation, DevOps, Design, Research, Meeting, Bug, Feature)\n\
            7. Implementation notes (as array)\n\
            8. Usage examples (as array)\n\
            9. Reasoning for this template\n\n\
            Respond with a JSON array of template objects with these exact fields:\n\
            [{{\n\
              \"name\": \"Template Name\",\n\
              \"description\": \"Template description with [PLACEHOLDER] syntax\",\n\
              \"tags\": [\"tag1\", \"tag2\"],\n\
              \"priority\": \"High\",\n\
              \"phase\": \"phase-name\",\n\
              \"category\": \"Development\",\n\
              \"implementation_notes\": [\"Step 1\", \"Step 2\"],\n\
              \"usage_examples\": [\"Example 1\", \"Example 2\"],\n\
              \"reasoning\": \"Why this template is useful\"\n\
            }}]",
            count,
            description,
            if context.is_some() { "Project " } else { "" },
            context.unwrap_or_else(|| "No project context available".to_string())
        );

        let response = self.provider.chat(&prompt, None).await?;
        
        // Parse JSON response
        let templates: Vec<AiTemplateGeneration> = serde_json::from_str(&response)
            .map_err(|e| anyhow::anyhow!("Failed to parse AI template response: {}", e))?;
        
        Ok(templates)
    }

    /// Suggest relevant templates for current project context
    pub async fn suggest_templates(&self, roadmap: &Roadmap, existing_templates: &[crate::model::TaskTemplate], limit: usize) -> Result<Vec<AiTemplateSuggestion>> {
        let project_context = utils::create_project_context(roadmap);
        let template_names: Vec<String> = existing_templates.iter().map(|t| t.name.clone()).collect();
        
        let prompt = format!(
            "Based on this project context: \"{}\"\n\n\
            Existing templates: {}\n\n\
            Suggest {} template ideas that would be most useful for this project.\n\
            Consider:\n\
            1. Current project phase and needs\n\
            2. Missing template types that could accelerate development\n\
            3. Common task patterns from the existing tasks\n\
            4. Industry best practices for this type of project\n\n\
            Respond with a JSON array:\n\
            [{{\n\
              \"name\": \"Suggested Template Name\",\n\
              \"description\": \"Why this template would be useful\",\n\
              \"category\": \"Development\",\n\
              \"priority\": \"High\",\n\
              \"reasoning\": \"Detailed explanation of why this template is recommended\",\n\
              \"usefulness_score\": 85\n\
            }}]",
            project_context,
            if template_names.is_empty() { "None".to_string() } else { template_names.join(", ") },
            limit
        );

        let response = self.provider.chat(&prompt, None).await?;
        
        let suggestions: Vec<AiTemplateSuggestion> = serde_json::from_str(&response)
            .map_err(|e| anyhow::anyhow!("Failed to parse AI template suggestions: {}", e))?;
        
        Ok(suggestions)
    }

    /// Enhance an existing template with AI improvements
    pub async fn enhance_template(&self, template: &crate::model::TaskTemplate, roadmap: Option<&Roadmap>) -> Result<AiTemplateEnhancement> {
        let context = roadmap.map(|r| utils::create_project_context(r));
        
        let prompt = format!(
            "Enhance this existing task template with better details and implementation guidance:\n\n\
            Current Template:\n\
            - Name: {}\n\
            - Description: {}\n\
            - Tags: {}\n\
            - Priority: {}\n\
            - Phase: {}\n\
            - Category: {}\n\
            - Notes: {}\n\
            - Implementation Notes: {}\n\n\
            {}Context: {}\n\n\
            Provide improvements for:\n\
            1. More specific and actionable description\n\
            2. Better implementation notes with step-by-step guidance\n\
            3. Additional relevant tags\n\
            4. Usage examples and scenarios\n\
            5. Common pitfalls to avoid\n\
            6. Acceptance criteria or completion indicators\n\n\
            Respond with JSON:\n\
            {{\n\
              \"enhanced_description\": \"Improved description\",\n\
              \"additional_tags\": [\"tag1\", \"tag2\"],\n\
              \"enhanced_implementation_notes\": [\"Step 1\", \"Step 2\"],\n\
              \"usage_examples\": [\"Example 1\", \"Example 2\"],\n\
              \"common_pitfalls\": [\"Pitfall 1\", \"Pitfall 2\"],\n\
              \"acceptance_criteria\": [\"Criteria 1\", \"Criteria 2\"],\n\
              \"improvements_summary\": \"Summary of what was improved\"\n\
            }}",
            template.name,
            template.description,
            template.tags.iter().cloned().collect::<Vec<_>>().join(", "),
            template.priority,
            template.phase.name,
            template.category.to_string(),
            template.notes.as_deref().unwrap_or("None"),
            template.implementation_notes.join(", "),
            if context.is_some() { "Project " } else { "" },
            context.unwrap_or_else(|| "No project context available".to_string())
        );

        let response = self.provider.chat(&prompt, None).await?;
        
        let enhancement: AiTemplateEnhancement = serde_json::from_str(&response)
            .map_err(|e| anyhow::anyhow!("Failed to parse AI template enhancement: {}", e))?;
        
        Ok(enhancement)
    }
    
    /// Generate or analyze a project roadmap with AI suggestions
    pub async fn generate_project_roadmap(&self, roadmap: &Roadmap, file: Option<&str>, focus: Option<&str>, generate_plan: bool) -> Result<String> {
        let project_context = utils::create_project_context(roadmap);
        
        // Read roadmap file if specified, otherwise use current project state
        let roadmap_content = if let Some(file_path) = file {
            match std::fs::read_to_string(file_path) {
                Ok(content) => content,
                Err(_) => {
                    return Err(anyhow::anyhow!("Unable to read roadmap file: {}", file_path));
                }
            }
        } else if let Some(source_file) = &roadmap.source_file {
            match std::fs::read_to_string(source_file) {
                Ok(content) => content,
                Err(_) => {
                    // Fallback to analyzing current project state
                    format!("# {}\n\nCurrent project with {} tasks across {} phases", 
                        roadmap.title, 
                        roadmap.tasks.len(),
                        roadmap.get_all_phases().len()
                    )
                }
            }
        } else {
            format!("# {}\n\nCurrent project with {} tasks across {} phases", 
                roadmap.title, 
                roadmap.tasks.len(),
                roadmap.get_all_phases().len()
            )
        };

        let focus_instruction = match focus {
            Some("structure") => "Focus on improving the overall project structure, organization, and hierarchy of tasks and phases.",
            Some("priorities") => "Focus on analyzing and optimizing task priorities, identifying critical path items and urgent tasks.",
            Some("phases") => "Focus on phase organization, phase transitions, and ensuring logical grouping of tasks.",
            Some("timeline") => "Focus on timeline analysis, scheduling, and identifying potential bottlenecks or scheduling conflicts.",
            Some("dependencies") => "Focus on task dependencies, identifying missing connections and potential circular dependencies.",
            _ => "Provide a comprehensive analysis covering structure, priorities, phases, timeline, and dependencies.",
        };

        let prompt = if generate_plan {
            format!(
                "Generate a new comprehensive project plan based on these requirements:\n\n{}\n\n\
                Current Project Context: {}\n\n\
                Create a well-structured roadmap with:\n\
                1. Clear phases with logical progression\n\
                2. Specific, actionable tasks\n\
                3. Appropriate priorities and dependencies\n\
                4. Realistic timeline estimates\n\
                5. Risk mitigation strategies\n\n\
                {} Format the response as a detailed markdown roadmap.",
                roadmap_content, project_context, focus_instruction
            )
        } else {
            format!(
                "Analyze this project roadmap and provide detailed suggestions for improvement:\n\n\
                ROADMAP CONTENT:\n{}\n\n\
                CURRENT PROJECT STATE:\n{}\n\n\
                ANALYSIS FOCUS:\n{}\n\n\
                Provide:\n\
                1. Overall assessment of the roadmap structure and quality\n\
                2. Specific suggestions for improvements\n\
                3. Identification of missing tasks or phases\n\
                4. Priority and timeline recommendations\n\
                5. Risk analysis and mitigation suggestions\n\
                6. Dependencies that should be added or modified\n\
                7. Phase organization improvements\n\n\
                Format your response with clear sections and actionable recommendations.",
                roadmap_content, project_context, focus_instruction
            )
        };

        let response = self.provider.chat(&prompt, None).await?;
        Ok(response)
    }
}

/// Utility functions for AI integration
pub mod utils {
    use crate::model::{Task, Priority};
    use super::AiTaskSuggestion;

    /// Convert AI task suggestion to a Task
    pub fn ai_suggestion_to_task(suggestion: AiTaskSuggestion, id: usize) -> Task {
        let mut task = Task::new(id, suggestion.description);
        task.priority = suggestion.priority;
        task.phase = suggestion.phase;
        task.tags = suggestion.tags.into_iter().collect();
        task.notes = suggestion.notes;
        
        if let Some(hours) = suggestion.estimated_hours {
            task.set_estimated_hours(hours);
        }

        // Add AI reasoning as an implementation note
        task.add_implementation_note(format!("AI Reasoning: {}", suggestion.reasoning));

        task
    }

    /// Format task suggestions for display
    pub fn format_task_suggestions(suggestions: &[AiTaskSuggestion]) -> String {
        if suggestions.is_empty() {
            return "No task suggestions available.".to_string();
        }

        let formatted = suggestions.iter()
            .enumerate()
            .map(|(i, suggestion)| {
                let priority_emoji = match suggestion.priority {
                    Priority::Critical => "🔴",
                    Priority::High => "🟡",
                    Priority::Medium => "🔵",
                    Priority::Low => "🟢",
                };
                
                let tags = if suggestion.tags.is_empty() {
                    String::new()
                } else {
                    format!(" #{}", suggestion.tags.join(" #"))
                };

                let time_estimate = suggestion.estimated_hours
                    .map(|h| format!(" [{}h]", h))
                    .unwrap_or_default();

                format!(
                    "{}. {} {} [{}] {}{}{}\n   💭 {}",
                    i + 1,
                    priority_emoji,
                    suggestion.description,
                    suggestion.phase.name,
                    tags,
                    time_estimate,
                    "\n   📝",
                    suggestion.reasoning
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n");

        format!("🤖 AI Task Suggestions:\n\n{}", formatted)
    }

    /// Create project context string from roadmap
    pub fn create_project_context(roadmap: &crate::model::Roadmap) -> String {
        format!(
            "Project: {} | Tasks: {}/{} completed | Phases: {}",
            roadmap.metadata.name,
            roadmap.tasks.iter().filter(|t| matches!(t.status, crate::model::TaskStatus::Completed)).count(),
            roadmap.tasks.len(),
            roadmap.get_all_phases().iter().map(|p| p.name.as_str()).collect::<Vec<_>>().join(", ")
        )
    }
}