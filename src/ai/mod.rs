//! AI integration module for Rask
//! 
//! This module provides AI-powered features for task management, including:
//! - Task analysis and suggestions
//! - Natural language task creation
//! - Project insights and recommendations
//! - Conversational task planning

pub mod models;
pub mod gemini;
pub mod service;

pub use models::*;

use anyhow::Result;
use async_trait::async_trait;

/// Trait defining the AI service interface for different providers
#[async_trait]
pub trait AiProvider {
    /// Send a chat message and get a response
    async fn chat(&self, message: &str, context: Option<&str>) -> Result<String>;
    
    /// Analyze tasks and provide suggestions
    async fn analyze_tasks(&self, tasks: &[crate::model::Task]) -> Result<AiTaskAnalysis>;
    
    /// Generate task breakdown from a description
    async fn generate_task_breakdown(&self, description: &str) -> Result<Vec<AiTaskSuggestion>>;
    
    /// Get project insights and recommendations
    async fn get_project_insights(&self, roadmap: &crate::model::Roadmap) -> Result<AiProjectInsights>;
    
    /// Check if the provider is properly configured and ready
    fn is_ready(&self) -> bool;
    
    /// Get the provider name
    fn provider_name(&self) -> &str;
}

/// Factory function to create an AI provider based on configuration
pub fn create_ai_provider(config: &crate::config::AiConfig) -> Result<Box<dyn AiProvider + Send + Sync>> {
    match config.provider.as_str() {
        "gemini" => {
            let provider = gemini::GeminiProvider::new(config)?;
            Ok(Box::new(provider))
        }
        _ => anyhow::bail!("Unsupported AI provider: {}", config.provider),
    }
}