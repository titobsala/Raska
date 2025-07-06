//! AI command implementations for intelligent task management

use tokio::runtime::Runtime;
use std::fs;

use crate::ai::service::{AiService, utils};
use crate::cli::AiCommands;
use crate::config::RaskConfig;
use crate::state::load_state;
use crate::ui::{display_info, display_success, display_error, display_warning};
use super::CommandResult;

/// Handle AI-related commands
pub fn handle_ai_command(ai_command: &AiCommands) -> CommandResult {
    // Create a tokio runtime for async operations
    let rt = Runtime::new().map_err(|e| format!("Failed to create async runtime: {}", e))?;
    
    rt.block_on(async {
        match ai_command {
            AiCommands::Chat { message, with_context } => {
                handle_ai_chat(message.as_deref(), *with_context).await
            }
            AiCommands::Analyze { limit, output, phase } => {
                handle_ai_analyze(*limit, output.as_deref(), phase.as_deref()).await
            }
            AiCommands::Breakdown { description, apply, phase } => {
                handle_ai_breakdown(description, *apply, phase.as_deref()).await
            }
            AiCommands::Insights { detailed, output } => {
                handle_ai_insights(*detailed, output.as_deref()).await
            }
            AiCommands::Configure { provider, api_key, model, enabled, temperature, show } => {
                handle_ai_configure(provider.as_deref(), api_key.as_deref(), model.as_deref(), 
                                  *enabled, *temperature, *show).await
            }
            AiCommands::Summary { with_recommendations, focus } => {
                handle_ai_summary(*with_recommendations, focus.as_deref()).await
            }
            AiCommands::Suggest { count, apply, priority, phase } => {
                handle_ai_suggest(*count, *apply, priority.as_deref(), phase.as_deref()).await
            }
            AiCommands::Roadmap { file, apply, focus, output, generate_plan } => {
                handle_ai_roadmap(file.as_deref(), *apply, focus.as_deref(), output.as_deref(), *generate_plan).await
            }
        }
    })
}

/// Handle AI chat command
async fn handle_ai_chat(initial_message: Option<&str>, with_context: bool) -> CommandResult {
    let config = RaskConfig::load().map_err(|e| format!("Failed to load configuration: {}", e))?;
    
    if !config.ai.is_ready() {
        display_error("AI is not configured. Please run 'rask ai configure' first to set up your API key.");
        return Ok(());
    }

    let ai_service = AiService::new(config).await
        .map_err(|e| format!("Failed to initialize AI service: {}", e))?;

    // Get project context if requested
    let project_context = if with_context {
        match load_state() {
            Ok(roadmap) => Some(utils::create_project_context(&roadmap)),
            Err(_) => {
                display_warning("No project found. Starting chat without project context.");
                None
            }
        }
    } else {
        None
    };

    // Start chat session
    let session_id = ai_service.start_chat_session(project_context).await
        .map_err(|e| format!("Failed to start chat session: {}", e))?;

    display_info(&format!("🤖 AI Chat Session Started ({})", &session_id[..8]));
    display_info("Type your message below. Type 'quit' or 'exit' to end the chat.");
    println!();

    // Send initial message if provided
    if let Some(msg) = initial_message {
        display_info(&format!("You: {}", msg));
        match ai_service.chat(msg.to_string()).await {
            Ok(response) => {
                println!("🤖 AI: {}", response);
                println!();
            }
            Err(e) => {
                display_error(&format!("AI Error: {}", e));
                return Ok(());
            }
        }
    }

    // Interactive chat loop
    loop {
        print!("You: ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
            break;
        }

        match ai_service.chat(input.to_string()).await {
            Ok(response) => {
                println!("🤖 AI: {}", response);
                println!();
            }
            Err(e) => {
                display_error(&format!("AI Error: {}", e));
                break;
            }
        }
    }

    ai_service.clear_chat_session().await;
    display_success("Chat session ended.");
    Ok(())
}

/// Handle AI analyze command
async fn handle_ai_analyze(limit: usize, output: Option<&str>, phase_filter: Option<&str>) -> CommandResult {
    let config = RaskConfig::load().map_err(|e| format!("Failed to load configuration: {}", e))?;
    
    if !config.ai.is_ready() {
        display_error("AI is not configured. Please run 'rask ai configure' first.");
        return Ok(());
    }

    let roadmap = load_state()?;
    let ai_service = AiService::new(config).await
        .map_err(|e| format!("Failed to initialize AI service: {}", e))?;

    // Filter tasks by phase if specified
    let tasks_to_analyze = if let Some(phase_name) = phase_filter {
        let phase = crate::model::Phase::from_string(phase_name);
        roadmap.filter_by_phase(&phase).into_iter().cloned().collect::<Vec<_>>()
    } else {
        roadmap.tasks.clone()
    };

    if tasks_to_analyze.is_empty() {
        display_warning("No tasks found to analyze.");
        return Ok(());
    }

    display_info(&format!("🔍 Analyzing {} tasks...", tasks_to_analyze.len()));

    match ai_service.analyze_tasks(&tasks_to_analyze).await {
        Ok(analysis) => {
            if let Some(output_path) = output {
                // Export to file
                let json_output = serde_json::to_string_pretty(&analysis)
                    .map_err(|e| format!("Failed to serialize analysis: {}", e))?;
                fs::write(output_path, json_output)
                    .map_err(|e| format!("Failed to write to file: {}", e))?;
                display_success(&format!("Analysis exported to {}", output_path));
            } else {
                // Display in terminal
                println!("📊 AI Task Analysis");
                println!("Health Score: {}/100", analysis.health_score);
                println!();

                if !analysis.insights.is_empty() {
                    println!("💡 Insights:");
                    for insight in &analysis.insights {
                        println!("  • {}", insight);
                    }
                    println!();
                }

                if !analysis.task_suggestions.is_empty() {
                    let suggestions_to_show = analysis.task_suggestions.iter().take(limit).cloned().collect::<Vec<_>>();
                    let formatted = utils::format_task_suggestions(&suggestions_to_show);
                    println!("{}", formatted);
                    println!();
                }

                if !analysis.workflow_recommendations.is_empty() {
                    println!("⚡ Workflow Recommendations:");
                    for rec in &analysis.workflow_recommendations {
                        println!("  • {}", rec);
                    }
                    println!();
                }

                if !analysis.potential_issues.is_empty() {
                    println!("⚠️  Potential Issues:");
                    for issue in &analysis.potential_issues {
                        println!("  • {}", issue);
                    }
                    println!();
                }
            }
        }
        Err(e) => {
            display_error(&format!("Failed to analyze tasks: {}", e));
        }
    }

    Ok(())
}

/// Handle AI breakdown command
async fn handle_ai_breakdown(description: &str, apply: bool, default_phase: Option<&str>) -> CommandResult {
    let config = RaskConfig::load().map_err(|e| format!("Failed to load configuration: {}", e))?;
    
    if !config.ai.is_ready() {
        display_error("AI is not configured. Please run 'rask ai configure' first.");
        return Ok(());
    }

    let model_name = config.ai.default_model.clone();
    let ai_service = AiService::new(config).await
        .map_err(|e| format!("Failed to initialize AI service: {}", e))?;

    display_info(&format!("🧠 Breaking down task: \"{}\"", description));

    match ai_service.generate_task_breakdown(description).await {
        Ok(suggestions) => {
            if suggestions.is_empty() {
                display_warning("No task breakdown suggestions generated.");
                return Ok(());
            }

            println!("📋 Generated Task Breakdown:");
            let formatted = utils::format_task_suggestions(&suggestions);
            println!("{}", formatted);

            if apply {
                let mut roadmap = load_state()?;
                let mut added_count = 0;

                for suggestion in suggestions {
                    let mut suggestion = suggestion;
                    
                    // Override phase if specified
                    if let Some(phase_name) = default_phase {
                        suggestion.phase = crate::model::Phase::from_string(phase_name);
                    }

                    let new_id = roadmap.get_next_task_id();
                    let mut task = utils::ai_suggestion_to_task(suggestion, new_id);
                    
                    // Update AI info with correct operation and model
                    task.mark_as_ai_generated(
                        "breakdown",
                        task.get_ai_reasoning().map(|s| s.clone()),
                        Some(model_name.clone())
                    );
                    
                    roadmap.add_task(task);
                    added_count += 1;
                }

                // Save the updated roadmap
                if let Err(e) = crate::state::save_state(&roadmap) {
                    display_error(&format!("Failed to save roadmap: {}", e));
                    return Ok(());
                }

                // Update markdown file if available
                if let Some(ref _source_file) = roadmap.source_file {
                    if let Err(e) = crate::markdown_writer::sync_to_source_file(&roadmap) {
                        display_warning(&format!("Failed to update markdown file: {}", e));
                    }
                }

                display_success(&format!("Applied {} tasks to the project!", added_count));
            } else {
                println!();
                display_info("Use --apply to add these tasks to your project");
            }
        }
        Err(e) => {
            display_error(&format!("Failed to generate task breakdown: {}", e));
        }
    }

    Ok(())
}

/// Handle AI insights command
async fn handle_ai_insights(detailed: bool, output: Option<&str>) -> CommandResult {
    let config = RaskConfig::load().map_err(|e| format!("Failed to load configuration: {}", e))?;
    
    if !config.ai.is_ready() {
        display_error("AI is not configured. Please run 'rask ai configure' first.");
        return Ok(());
    }

    let roadmap = load_state()?;
    let ai_service = AiService::new(config).await
        .map_err(|e| format!("Failed to initialize AI service: {}", e))?;

    display_info("🔮 Generating project insights...");

    match ai_service.get_project_insights(&roadmap).await {
        Ok(insights) => {
            if let Some(output_path) = output {
                let json_output = serde_json::to_string_pretty(&insights)
                    .map_err(|e| format!("Failed to serialize insights: {}", e))?;
                fs::write(output_path, json_output)
                    .map_err(|e| format!("Failed to write to file: {}", e))?;
                display_success(&format!("Insights exported to {}", output_path));
            } else {
                println!("🎯 Project Insights");
                println!("Status: {}", insights.completion_assessment);
                println!();

                if !insights.critical_path.is_empty() {
                    println!("🔥 Critical Path:");
                    for item in &insights.critical_path {
                        println!("  • {}", item);
                    }
                    println!();
                }

                if !insights.next_actions.is_empty() {
                    println!("⚡ Next Actions:");
                    for (i, action) in insights.next_actions.iter().enumerate() {
                        println!("  {}. {}", i + 1, action);
                    }
                    println!();
                }

                if !insights.risks.is_empty() {
                    println!("⚠️  Risks:");
                    for risk in &insights.risks {
                        println!("  • {} ({})", risk.description, risk.severity);
                        if detailed && !risk.mitigation.is_empty() {
                            println!("    Mitigation: {}", risk.mitigation.join(", "));
                        }
                    }
                    println!();
                }

                if !insights.resource_suggestions.is_empty() {
                    println!("💡 Resource Suggestions:");
                    for suggestion in &insights.resource_suggestions {
                        println!("  • {}", suggestion);
                    }
                    println!();
                }

                if detailed {
                    if let Some(ref performance) = insights.performance_insights {
                        println!("📈 Performance Insights:");
                        if let Some(accuracy) = performance.estimation_accuracy {
                            println!("  • Estimation Accuracy: {:.1}%", accuracy * 100.0);
                        }
                        if !performance.efficient_areas.is_empty() {
                            println!("  • Efficient Areas: {}", performance.efficient_areas.join(", "));
                        }
                        if !performance.improvement_areas.is_empty() {
                            println!("  • Areas for Improvement: {}", performance.improvement_areas.join(", "));
                        }
                        println!("  • Productivity Trends: {}", performance.productivity_trends);
                        println!();
                    }
                }
            }
        }
        Err(e) => {
            display_error(&format!("Failed to generate insights: {}", e));
        }
    }

    Ok(())
}

/// Handle AI configure command
async fn handle_ai_configure(
    provider: Option<&str>,
    api_key: Option<&str>,
    model: Option<&str>,
    enabled: Option<bool>,
    temperature: Option<f32>,
    show: bool,
) -> CommandResult {
    let mut config = RaskConfig::load().map_err(|e| format!("Failed to load configuration: {}", e))?;

    if show {
        println!("🤖 AI Configuration:");
        println!("  Enabled: {}", config.ai.enabled);
        println!("  Provider: {}", config.ai.provider);
        println!("  Default Model: {}", config.ai.default_model);
        println!("  Temperature: {}", config.ai.temperature);
        println!("  Max Tokens: {}", config.ai.max_tokens);
        println!("  Context Window: {}", config.ai.context_window);
        println!("  Auto Suggestions: {}", config.ai.auto_suggestions);
        println!();
        println!("  API Key Status: {}", 
            if config.ai.get_api_key().is_some() { "✅ Configured" } else { "❌ Not set" });
        println!("  Available Models: {}", config.ai.gemini.models.join(", "));
        println!();
        return Ok(());
    }

    let mut updated = false;

    if let Some(p) = provider {
        if p == "gemini" {
            config.ai.provider = p.to_string();
            updated = true;
            display_success(&format!("Set AI provider to: {}", p));
        } else {
            display_error(&format!("Unsupported provider: {}. Only 'gemini' is currently supported.", p));
            return Ok(());
        }
    }

    if let Some(key) = api_key {
        config.ai.gemini.api_key = Some(key.to_string());
        updated = true;
        display_success("API key configured (stored in config file)");
        display_info("For better security, consider using the GEMINI_API_KEY environment variable instead");
    }

    if let Some(m) = model {
        if config.ai.gemini.models.contains(&m.to_string()) {
            config.ai.default_model = m.to_string();
            updated = true;
            display_success(&format!("Set default model to: {}", m));
        } else {
            display_error(&format!("Unknown model: {}. Available models: {}", m, config.ai.gemini.models.join(", ")));
            return Ok(());
        }
    }

    if let Some(e) = enabled {
        config.ai.enabled = e;
        updated = true;
        display_success(&format!("AI features {}", if e { "enabled" } else { "disabled" }));
    }

    if let Some(t) = temperature {
        if (0.0..=1.0).contains(&t) {
            config.ai.temperature = t;
            updated = true;
            display_success(&format!("Set temperature to: {}", t));
        } else {
            display_error("Temperature must be between 0.0 and 1.0");
            return Ok(());
        }
    }

    if updated {
        config.save_user_config().map_err(|e| format!("Failed to save configuration: {}", e))?;
        
        if config.ai.is_ready() {
            display_success("✅ AI is now ready to use!");
        } else if config.ai.enabled && config.ai.get_api_key().is_none() {
            display_warning("⚠️  AI is enabled but no API key is configured. Set GEMINI_API_KEY environment variable or use --api-key");
        }
    } else {
        display_info("No configuration changes made. Use --show to view current settings.");
    }

    Ok(())
}

/// Handle AI summary command
async fn handle_ai_summary(with_recommendations: bool, _focus: Option<&str>) -> CommandResult {
    let config = RaskConfig::load().map_err(|e| format!("Failed to load configuration: {}", e))?;
    
    if !config.ai.is_ready() {
        display_error("AI is not configured. Please run 'rask ai configure' first.");
        return Ok(());
    }

    let roadmap = load_state()?;
    let ai_service = AiService::new(config).await
        .map_err(|e| format!("Failed to initialize AI service: {}", e))?;

    display_info("📊 Generating project summary...");

    match ai_service.get_project_summary(&roadmap).await {
        Ok(summary) => {
            println!("{}", summary);
            
            if with_recommendations {
                match ai_service.suggest_next_tasks(&roadmap, 3).await {
                    Ok(suggestions) => {
                        if !suggestions.is_empty() {
                            println!("\n🎯 Quick Recommendations:");
                            for (i, suggestion) in suggestions.iter().enumerate() {
                                println!("  {}. {} [{}]", i + 1, suggestion.description, suggestion.phase.name);
                            }
                        }
                    }
                    Err(e) => {
                        display_warning(&format!("Failed to get recommendations: {}", e));
                    }
                }
            }
        }
        Err(e) => {
            display_error(&format!("Failed to generate summary: {}", e));
        }
    }

    Ok(())
}

/// Handle AI suggest command
async fn handle_ai_suggest(count: usize, apply: bool, priority: Option<&str>, phase: Option<&str>) -> CommandResult {
    let config = RaskConfig::load().map_err(|e| format!("Failed to load configuration: {}", e))?;
    
    if !config.ai.is_ready() {
        display_error("AI is not configured. Please run 'rask ai configure' first.");
        return Ok(());
    }

    let model_name = config.ai.default_model.clone();
    let roadmap = load_state()?;
    let ai_service = AiService::new(config).await
        .map_err(|e| format!("Failed to initialize AI service: {}", e))?;

    display_info(&format!("🤖 Generating {} task suggestions...", count));

    match ai_service.suggest_next_tasks(&roadmap, count).await {
        Ok(mut suggestions) => {
            if suggestions.is_empty() {
                display_warning("No task suggestions generated.");
                return Ok(());
            }

            // Apply filters if specified
            if let Some(priority_str) = priority {
                let target_priority = match priority_str.to_lowercase().as_str() {
                    "critical" => crate::model::Priority::Critical,
                    "high" => crate::model::Priority::High,
                    "medium" => crate::model::Priority::Medium,
                    "low" => crate::model::Priority::Low,
                    _ => {
                        display_error("Invalid priority. Use: low, medium, high, critical");
                        return Ok(());
                    }
                };
                for suggestion in &mut suggestions {
                    suggestion.priority = target_priority.clone();
                }
            }

            if let Some(phase_name) = phase {
                let target_phase = crate::model::Phase::from_string(phase_name);
                for suggestion in &mut suggestions {
                    suggestion.phase = target_phase.clone();
                }
            }

            let formatted = utils::format_task_suggestions(&suggestions);
            println!("{}", formatted);

            if apply {
                let mut roadmap = load_state()?;
                let mut added_count = 0;

                for suggestion in suggestions {
                    let new_id = roadmap.get_next_task_id();
                    let mut task = utils::ai_suggestion_to_task(suggestion, new_id);
                    
                    // Update AI info with correct operation and model
                    task.mark_as_ai_generated(
                        "suggest",
                        task.get_ai_reasoning().map(|s| s.clone()),
                        Some(model_name.clone())
                    );
                    
                    roadmap.add_task(task);
                    added_count += 1;
                }

                if let Err(e) = crate::state::save_state(&roadmap) {
                    display_error(&format!("Failed to save roadmap: {}", e));
                    return Ok(());
                }

                if let Some(ref _source_file) = roadmap.source_file {
                    if let Err(e) = crate::markdown_writer::sync_to_source_file(&roadmap) {
                        display_warning(&format!("Failed to update markdown file: {}", e));
                    }
                }

                display_success(&format!("Applied {} suggested tasks to the project!", added_count));
            } else {
                println!();
                display_info("Use --apply to add these suggestions to your project");
            }
        }
        Err(e) => {
            display_error(&format!("Failed to generate suggestions: {}", e));
        }
    }

    Ok(())
}

/// Handle AI roadmap command
pub async fn handle_ai_roadmap(file: Option<&str>, apply: bool, focus: Option<&str>, output: Option<&str>, generate_plan: bool) -> CommandResult {
    let config = RaskConfig::load().map_err(|e| format!("Failed to load configuration: {}", e))?;
    
    if !config.ai.is_ready() {
        display_error("AI is not configured. Please run 'rask ai configure' first.");
        return Ok(());
    }

    let roadmap = load_state()?;
    let ai_service = AiService::new(config).await
        .map_err(|e| format!("Failed to initialize AI service: {}", e))?;

    display_info("🗓 Generating project roadmap...");

    match ai_service.generate_project_roadmap(&roadmap, file.as_deref(), focus.as_deref(), generate_plan).await {
        Ok(roadmap) => {
            if let Some(output_path) = output {
                let json_output = serde_json::to_string_pretty(&roadmap)
                    .map_err(|e| format!("Failed to serialize roadmap: {}", e))?;
                fs::write(output_path, json_output)
                    .map_err(|e| format!("Failed to write to file: {}", e))?;
                display_success(&format!("Project roadmap exported to {}", output_path));
            } else {
                println!("📋 Project Roadmap");
                println!("{}", roadmap);
            }
        }
        Err(e) => {
            display_error(&format!("Failed to generate project roadmap: {}", e));
        }
    }

    Ok(())
}