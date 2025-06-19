//! Google Gemini AI provider implementation

use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::config::AiConfig;
use crate::model::{Task, Roadmap, Priority, Phase};
use super::{AiProvider, AiTaskAnalysis, AiTaskSuggestion, AiProjectInsights, AiRisk, AiMessageMetadata};

/// Google Gemini API client
pub struct GeminiProvider {
    client: Client,
    config: AiConfig,
    api_key: String,
}

/// Gemini API request structure
#[derive(Debug, Serialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
    #[serde(rename = "generationConfig")]
    generation_config: GeminiGenerationConfig,
}

#[derive(Debug, Serialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Debug, Serialize)]
struct GeminiPart {
    text: String,
}

#[derive(Debug, Serialize)]
struct GeminiGenerationConfig {
    temperature: f32,
    #[serde(rename = "maxOutputTokens")]
    max_output_tokens: u32,
}

/// Gemini API response structure
#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Vec<GeminiCandidate>,
    #[serde(rename = "usageMetadata")]
    usage_metadata: Option<GeminiUsageMetadata>,
}

#[derive(Debug, Deserialize)]
struct GeminiCandidate {
    content: GeminiResponseContent,
}

#[derive(Debug, Deserialize)]
struct GeminiResponseContent {
    parts: Vec<GeminiResponsePart>,
}

#[derive(Debug, Deserialize)]
struct GeminiResponsePart {
    text: String,
}

#[derive(Debug, Deserialize)]
struct GeminiUsageMetadata {
    #[serde(rename = "promptTokenCount")]
    prompt_token_count: Option<u32>,
    #[serde(rename = "candidatesTokenCount")]
    candidates_token_count: Option<u32>,
    #[serde(rename = "totalTokenCount")]
    total_token_count: Option<u32>,
}

impl GeminiProvider {
    /// Create a new Gemini provider
    pub fn new(config: &AiConfig) -> Result<Self> {
        let api_key = config.get_api_key()
            .ok_or_else(|| anyhow::anyhow!("Gemini API key not found. Set GEMINI_API_KEY environment variable or configure in settings."))?;

        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        let client = Client::builder()
            .timeout(Duration::from_secs(config.gemini.timeout))
            .default_headers(headers)
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            config: config.clone(),
            api_key,
        })
    }

    /// Make a request to the Gemini API
    async fn make_request(&self, prompt: &str) -> Result<(String, Option<AiMessageMetadata>)> {
        let request = GeminiRequest {
            contents: vec![GeminiContent {
                parts: vec![GeminiPart {
                    text: prompt.to_string(),
                }],
            }],
            generation_config: GeminiGenerationConfig {
                temperature: self.config.temperature,
                max_output_tokens: self.config.max_tokens,
            },
        };

        let url = format!(
            "{}/models/{}:generateContent?key={}",
            self.config.gemini.endpoint,
            self.config.default_model,
            self.api_key
        );

        let start_time = std::time::Instant::now();
        
        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Gemini API")?;

        let processing_time = start_time.elapsed().as_millis() as u64;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            anyhow::bail!("Gemini API error ({}): {}", status, error_text);
        }

        let gemini_response: GeminiResponse = response
            .json()
            .await
            .context("Failed to parse Gemini API response")?;

        let text = gemini_response
            .candidates
            .first()
            .and_then(|candidate| candidate.content.parts.first())
            .map(|part| part.text.clone())
            .ok_or_else(|| anyhow::anyhow!("No response content from Gemini API"))?;

        let metadata = gemini_response.usage_metadata.map(|usage| AiMessageMetadata {
            model: self.config.default_model.clone(),
            input_tokens: usage.prompt_token_count,
            output_tokens: usage.candidates_token_count,
            processing_time: Some(processing_time),
        });

        Ok((text, metadata))
    }

    /// Build context about the project for AI prompts
    fn build_project_context(&self, roadmap: &Roadmap) -> String {
        let total_tasks = roadmap.tasks.len();
        let completed_tasks = roadmap.tasks.iter().filter(|t| matches!(t.status, crate::model::TaskStatus::Completed)).count();
        let completion_rate = if total_tasks > 0 { (completed_tasks * 100) / total_tasks } else { 0 };

        let phases = roadmap.get_all_phases();
        let phase_summary = phases.iter()
            .map(|phase| {
                let phase_tasks = roadmap.filter_by_phase(phase);
                let phase_completed = phase_tasks.iter().filter(|t| matches!(t.status, crate::model::TaskStatus::Completed)).count();
                format!("- {}: {}/{} tasks completed", phase.name, phase_completed, phase_tasks.len())
            })
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "Project: {}\nDescription: {}\nTotal Progress: {}/{}  tasks ({} %)\nPhases:\n{}",
            roadmap.metadata.name,
            roadmap.metadata.description.as_deref().unwrap_or("No description"),
            completed_tasks,
            total_tasks,
            completion_rate,
            phase_summary
        )
    }

    /// Build task context for AI prompts
    fn build_task_context(&self, tasks: &[Task]) -> String {
        if tasks.is_empty() {
            return "No tasks available.".to_string();
        }

        let task_summaries: Vec<String> = tasks.iter()
            .take(20) // Limit to avoid overwhelming the AI
            .map(|task| {
                let status = match task.status {
                    crate::model::TaskStatus::Completed => "✓",
                    crate::model::TaskStatus::Pending => "○",
                };
                let priority = match task.priority {
                    Priority::Critical => "🔴",
                    Priority::High => "🟡",
                    Priority::Medium => "🔵",
                    Priority::Low => "🟢",
                };
                let tags = if task.tags.is_empty() {
                    String::new()
                } else {
                    format!(" #{}", task.tags.iter().cloned().collect::<Vec<_>>().join(" #"))
                };
                
                format!(
                    "{} {} [{}] {} - {}{}",
                    status,
                    priority,
                    task.phase.name,
                    task.id,
                    task.description,
                    tags
                )
            })
            .collect();

        format!("Current tasks:\n{}", task_summaries.join("\n"))
    }
}

#[async_trait]
impl AiProvider for GeminiProvider {
    async fn chat(&self, message: &str, context: Option<&str>) -> Result<String> {
        let prompt = if let Some(ctx) = context {
            format!(
                "You are an AI assistant helping with project management in Rask, a CLI task management tool.\n\nProject Context:\n{}\n\nUser Question: {}\n\nPlease provide a helpful, concise response focused on project management, task organization, and productivity.",
                ctx, message
            )
        } else {
            format!(
                "You are an AI assistant for Rask, a CLI project management tool. Please help the user with their question:\n\n{}",
                message
            )
        };

        let (response, _) = self.make_request(&prompt).await?;
        Ok(response)
    }

    async fn analyze_tasks(&self, tasks: &[Task]) -> Result<AiTaskAnalysis> {
        let task_context = self.build_task_context(tasks);
        
        let prompt = format!(
            "You are an expert project manager analyzing a list of tasks. Please provide a comprehensive analysis in the following JSON format:

{{
  \"health_score\": <number 0-100>,
  \"insights\": [\"insight1\", \"insight2\", ...],
  \"task_suggestions\": [
    {{
      \"description\": \"suggested task\",
      \"priority\": \"High|Medium|Low|Critical\",
      \"phase\": {{\"name\": \"phase_name\", \"description\": null, \"emoji\": null}},
      \"tags\": [\"tag1\", \"tag2\"],
      \"estimated_hours\": <number or null>,
      \"dependencies\": [\"dependency description\"],
      \"notes\": \"implementation notes\" or null,
      \"reasoning\": \"why this task is suggested\"
    }}
  ],
  \"workflow_recommendations\": [\"recommendation1\", \"recommendation2\"],
  \"potential_issues\": [\"issue1\", \"issue2\"]
}}

Task Context:
{}

Focus on identifying gaps, dependency issues, missing tests, documentation needs, and optimization opportunities. Provide actionable suggestions.",
            task_context
        );

        let (response, _) = self.make_request(&prompt).await?;
        
        // Try to parse as JSON, fallback to basic analysis if parsing fails
        match serde_json::from_str::<AiTaskAnalysis>(&response) {
            Ok(analysis) => Ok(analysis),
            Err(_) => {
                // Fallback: create a basic analysis from the text response
                Ok(AiTaskAnalysis {
                    health_score: 75, // Default score
                    insights: vec![response.clone()],
                    task_suggestions: vec![],
                    workflow_recommendations: vec!["Review task dependencies".to_string()],
                    potential_issues: vec!["Unable to parse detailed analysis".to_string()],
                })
            }
        }
    }

    async fn generate_task_breakdown(&self, description: &str) -> Result<Vec<AiTaskSuggestion>> {
        let prompt = format!(
            "Break down this high-level task into specific, actionable subtasks. Return as JSON array:

[
  {{
    \"description\": \"specific task description\",
    \"priority\": \"High|Medium|Low|Critical\",
    \"phase\": {{\"name\": \"MVP|Beta|Release|Future|Custom\", \"description\": null, \"emoji\": null}},
    \"tags\": [\"relevant\", \"tags\"],
    \"estimated_hours\": <number or null>,
    \"dependencies\": [\"dependency descriptions\"],
    \"notes\": \"implementation details\" or null,
    \"reasoning\": \"why this subtask is needed\"
  }}
]

High-level task: {}

Make tasks concrete, testable, and properly sequenced. Include testing and documentation tasks where appropriate.",
            description
        );

        let (response, _) = self.make_request(&prompt).await?;
        
        // Try to parse as JSON array
        match serde_json::from_str::<Vec<AiTaskSuggestion>>(&response) {
            Ok(suggestions) => Ok(suggestions),
            Err(_) => {
                // Fallback: create a single task suggestion
                Ok(vec![AiTaskSuggestion {
                    description: format!("Implement: {}", description),
                    priority: Priority::Medium,
                    phase: Phase::mvp(),
                    tags: vec!["ai-generated".to_string()],
                    estimated_hours: None,
                    dependencies: vec![],
                    notes: Some(response),
                    reasoning: "AI-generated task breakdown".to_string(),
                }])
            }
        }
    }

    async fn get_project_insights(&self, roadmap: &Roadmap) -> Result<AiProjectInsights> {
        let project_context = self.build_project_context(roadmap);
        let task_context = self.build_task_context(&roadmap.tasks);
        
        let prompt = format!(
            "Analyze this project and provide insights in JSON format:

{{
  \"completion_assessment\": \"overall project status\",
  \"critical_path\": [\"critical task 1\", \"critical task 2\"],
  \"resource_suggestions\": [\"suggestion 1\", \"suggestion 2\"],
  \"risks\": [
    {{
      \"description\": \"risk description\",
      \"severity\": \"Low|Medium|High|Critical\",
      \"mitigation\": [\"mitigation strategy\"],
      \"affected_areas\": [\"area 1\", \"area 2\"]
    }}
  ],
  \"next_actions\": [\"immediate action 1\", \"immediate action 2\"],
  \"performance_insights\": {{
    \"estimation_accuracy\": <number 0-1 or null>,
    \"efficient_areas\": [\"area 1\", \"area 2\"],
    \"improvement_areas\": [\"area 1\", \"area 2\"],
    \"productivity_trends\": \"trend description\"
  }}
}}

Project Context:
{}

Task Context:
{}

Provide strategic insights focusing on project health, bottlenecks, and optimization opportunities.",
            project_context, task_context
        );

        let (response, _) = self.make_request(&prompt).await?;
        
        // Try to parse as JSON, fallback to basic insights if parsing fails
        match serde_json::from_str::<AiProjectInsights>(&response) {
            Ok(insights) => Ok(insights),
            Err(_) => {
                // Fallback: create basic insights from the text response
                Ok(AiProjectInsights {
                    completion_assessment: "Analysis completed".to_string(),
                    critical_path: vec!["Review project dependencies".to_string()],
                    resource_suggestions: vec!["Consider task prioritization".to_string()],
                    risks: vec![AiRisk {
                        description: "Unable to parse detailed analysis".to_string(),
                        severity: "Low".to_string(),
                        mitigation: vec!["Review AI response format".to_string()],
                        affected_areas: vec!["Analysis".to_string()],
                    }],
                    next_actions: vec!["Continue project development".to_string()],
                    performance_insights: None,
                })
            }
        }
    }

    fn is_ready(&self) -> bool {
        !self.api_key.is_empty()
    }

    fn provider_name(&self) -> &str {
        "Google Gemini"
    }
}