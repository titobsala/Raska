use serde::{Deserialize, Serialize};
use std::collections::{HashSet, HashMap};

/// Task template for creating reusable task patterns
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaskTemplate {
    pub name: String,
    pub description: String,
    pub tags: HashSet<String>,
    pub priority: Priority,
    pub phase: Phase,
    pub notes: Option<String>,
    pub implementation_notes: Vec<String>,
    pub created_at: String,
    pub category: TemplateCategory,
}

/// Categories for organizing templates
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TemplateCategory {
    Development,
    Testing,
    Documentation,
    DevOps,
    Design,
    Research,
    Meeting,
    Bug,
    Feature,
    Custom(String),
}

impl std::fmt::Display for TemplateCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateCategory::Development => write!(f, "Development"),
            TemplateCategory::Testing => write!(f, "Testing"),
            TemplateCategory::Documentation => write!(f, "Documentation"),
            TemplateCategory::DevOps => write!(f, "DevOps"),
            TemplateCategory::Design => write!(f, "Design"),
            TemplateCategory::Research => write!(f, "Research"),
            TemplateCategory::Meeting => write!(f, "Meeting"),
            TemplateCategory::Bug => write!(f, "Bug"),
            TemplateCategory::Feature => write!(f, "Feature"),
            TemplateCategory::Custom(name) => write!(f, "{}", name),
        }
    }
}

impl TaskTemplate {
    /// Create a new task template
    pub fn new(name: String, description: String) -> Self {
        TaskTemplate {
            name,
            description,
            tags: HashSet::new(),
            priority: Priority::Medium,
            phase: Phase::default(),
            notes: None,
            implementation_notes: Vec::new(),
            created_at: chrono::Utc::now().to_rfc3339(),
            category: TemplateCategory::Development,
        }
    }

    /// Create a task from this template
    pub fn create_task(&self, id: usize, custom_description: Option<String>) -> Task {
        Task {
            id,
            description: custom_description.unwrap_or_else(|| self.description.clone()),
            status: TaskStatus::Pending,
            tags: self.tags.clone(),
            priority: self.priority.clone(),
            phase: self.phase.clone(),
            notes: self.notes.clone(),
            implementation_notes: self.implementation_notes.clone(),
            dependencies: Vec::new(),
            created_at: Some(chrono::Utc::now().to_rfc3339()),
            completed_at: None,
            estimated_hours: None,
            actual_hours: None,
            time_sessions: Vec::new(),
            ai_info: AiTaskInfo::default(),
        }
    }

    /// Get predefined development templates
    pub fn predefined_templates() -> Vec<TaskTemplate> {
        vec![
            // Development Templates
            TaskTemplate {
                name: "Feature Implementation".to_string(),
                description: "Implement new feature: [FEATURE_NAME]".to_string(),
                tags: ["feature", "development"].iter().map(|s| s.to_string()).collect(),
                priority: Priority::Medium,
                phase: Phase::mvp(),
                notes: Some("Remember to write unit tests, update documentation, consider edge cases, and review security implications".to_string()),
                implementation_notes: vec![
                    "// TODO: Add implementation details".to_string(),
                    "// Consider performance implications".to_string(),
                    "// Add error handling".to_string(),
                ],
                created_at: chrono::Utc::now().to_rfc3339(),
                category: TemplateCategory::Feature,
            },
            TaskTemplate {
                name: "Bug Fix".to_string(),
                description: "Fix bug: [BUG_DESCRIPTION]".to_string(),
                tags: ["bug", "fix"].iter().map(|s| s.to_string()).collect(),
                priority: Priority::High,
                phase: Phase::mvp(),
                notes: Some("Steps to fix: 1) Reproduce the issue, 2) Identify root cause, 3) Implement fix, 4) Test thoroughly, 5) Update tests if needed".to_string()),
                implementation_notes: vec![
                    "// Reproduction steps:".to_string(),
                    "// Root cause analysis:".to_string(),
                    "// Fix implementation:".to_string(),
                ],
                created_at: chrono::Utc::now().to_rfc3339(),
                category: TemplateCategory::Bug,
            },
            // Testing Templates
            TaskTemplate {
                name: "Unit Tests".to_string(),
                description: "Write unit tests for [MODULE_NAME]".to_string(),
                tags: ["testing", "unit-tests"].iter().map(|s| s.to_string()).collect(),
                priority: Priority::Medium,
                phase: Phase::mvp(),
                notes: Some("Test coverage should include happy path scenarios, edge cases, error conditions, and boundary values".to_string()),
                implementation_notes: vec![
                    "// Test cases to implement:".to_string(),
                    "// Mock dependencies:".to_string(),
                    "// Assertions to verify:".to_string(),
                ],
                created_at: chrono::Utc::now().to_rfc3339(),
                category: TemplateCategory::Testing,
            },
            // Documentation Templates
            TaskTemplate {
                name: "API Documentation".to_string(),
                description: "Document API endpoints for [API_NAME]".to_string(),
                tags: ["documentation", "api"].iter().map(|s| s.to_string()).collect(),
                priority: Priority::Medium,
                phase: Phase::beta(),
                notes: Some("Documentation should include endpoint descriptions, request/response examples, error codes, and authentication requirements".to_string()),
                implementation_notes: vec![
                    "// Endpoints to document:".to_string(),
                    "// Example requests:".to_string(),
                    "// Example responses:".to_string(),
                ],
                created_at: chrono::Utc::now().to_rfc3339(),
                category: TemplateCategory::Documentation,
            },
            // DevOps Templates
            TaskTemplate {
                name: "CI/CD Pipeline".to_string(),
                description: "Set up CI/CD pipeline for [PROJECT_NAME]".to_string(),
                tags: ["devops", "ci-cd", "automation"].iter().map(|s| s.to_string()).collect(),
                priority: Priority::High,
                phase: Phase::mvp(),
                notes: Some("Pipeline should include automated testing, code quality checks, security scanning, and deployment automation".to_string()),
                implementation_notes: vec![
                    "// Pipeline stages:".to_string(),
                    "// Required tools:".to_string(),
                    "// Deployment targets:".to_string(),
                ],
                created_at: chrono::Utc::now().to_rfc3339(),
                category: TemplateCategory::DevOps,
            },
            // Research Templates
            TaskTemplate {
                name: "Technology Research".to_string(),
                description: "Research [TECHNOLOGY_NAME] for [USE_CASE]".to_string(),
                tags: ["research", "analysis"].iter().map(|s| s.to_string()).collect(),
                priority: Priority::Low,
                phase: Phase::future(),
                notes: Some("Research areas: technical capabilities, performance characteristics, integration requirements, cost implications, and learning curve".to_string()),
                implementation_notes: vec![
                    "// Key questions to answer:".to_string(),
                    "// Evaluation criteria:".to_string(),
                    "// Proof of concept plan:".to_string(),
                ],
                created_at: chrono::Utc::now().to_rfc3339(),
                category: TemplateCategory::Research,
            },
        ]
    }
}

/// Template storage and management
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplateCollection {
    pub templates: Vec<TaskTemplate>,
    pub created_at: String,
    pub last_modified: String,
}

impl Default for TemplateCollection {
    fn default() -> Self {
        TemplateCollection {
            templates: TaskTemplate::predefined_templates(),
            created_at: chrono::Utc::now().to_rfc3339(),
            last_modified: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl TemplateCollection {
    /// Create a new template collection
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a template to the collection
    pub fn add_template(&mut self, template: TaskTemplate) {
        self.templates.push(template);
        self.update_last_modified();
    }

    /// Remove a template by name
    pub fn remove_template(&mut self, name: &str) -> Option<TaskTemplate> {
        if let Some(pos) = self.templates.iter().position(|t| t.name == name) {
            self.update_last_modified();
            Some(self.templates.remove(pos))
        } else {
            None
        }
    }

    /// Find a template by name
    pub fn find_template(&self, name: &str) -> Option<&TaskTemplate> {
        self.templates.iter().find(|t| t.name == name)
    }

    /// Get templates by category
    #[allow(dead_code)]
    pub fn get_templates_by_category(&self, category: &TemplateCategory) -> Vec<&TaskTemplate> {
        self.templates.iter().filter(|t| &t.category == category).collect()
    }

    /// Get all template names
    #[allow(dead_code)]
    pub fn get_template_names(&self) -> Vec<&String> {
        self.templates.iter().map(|t| &t.name).collect()
    }

    /// Update last modified timestamp
    fn update_last_modified(&mut self) {
        self.last_modified = chrono::Utc::now().to_rfc3339();
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    Completed,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Medium
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Low => write!(f, "Low"),
            Priority::Medium => write!(f, "Medium"),
            Priority::High => write!(f, "High"),
            Priority::Critical => write!(f, "Critical"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Phase {
    pub name: String,
    pub description: Option<String>,
    pub emoji: Option<String>,
}

// Custom serialization/deserialization for backward compatibility
impl serde::Serialize for Phase {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Phase", 3)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("description", &self.description)?;
        state.serialize_field("emoji", &self.emoji)?;
        state.end()
    }
}

impl<'de> serde::Deserialize<'de> for Phase {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, Visitor};
        use serde_json::Value;

        struct PhaseVisitor;

        impl<'de> Visitor<'de> for PhaseVisitor {
            type Value = Phase;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a phase name string or phase object")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Phase::from_string(value))
            }

            fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
            where
                M: serde::de::MapAccess<'de>,
            {
                // Use serde_json to deserialize the map into a Value first
                let value = Value::deserialize(serde::de::value::MapAccessDeserializer::new(map))?;
                
                // Extract fields from the Value
                let name = value.get("name")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| de::Error::missing_field("name"))?
                    .to_string();
                
                let description = value.get("description")
                    .and_then(|v| if v.is_null() { None } else { v.as_str().map(|s| s.to_string()) });
                
                let emoji = value.get("emoji")
                    .and_then(|v| if v.is_null() { None } else { v.as_str().map(|s| s.to_string()) });
                
                Ok(Phase {
                    name,
                    description,
                    emoji,
                })
            }
        }

        deserializer.deserialize_any(PhaseVisitor)
    }
}

impl Phase {
    /// Create a new custom phase
    pub fn new(name: String) -> Self {
        Phase {
            name,
            description: None,
            emoji: None,
        }
    }
    
    /// Create a new custom phase with description and emoji
    pub fn with_details(name: String, description: Option<String>, emoji: Option<String>) -> Self {
        Phase {
            name,
            description,
            emoji,
        }
    }
    
    /// Create predefined MVP phase
    pub fn mvp() -> Self {
        Phase {
            name: "MVP".to_string(),
            description: Some("Core features for minimum viable product".to_string()),
            emoji: Some("🚀".to_string()),
        }
    }
    
    /// Create predefined Beta phase
    pub fn beta() -> Self {
        Phase {
            name: "Beta".to_string(),
            description: Some("Features for beta release and testing".to_string()),
            emoji: Some("🧪".to_string()),
        }
    }
    
    /// Create predefined Release phase
    pub fn release() -> Self {
        Phase {
            name: "Release".to_string(),
            description: Some("Features for production release".to_string()),
            emoji: Some("🎯".to_string()),
        }
    }
    
    /// Create predefined Future phase
    pub fn future() -> Self {
        Phase {
            name: "Future".to_string(),
            description: Some("Future enhancements and improvements".to_string()),
            emoji: Some("🔮".to_string()),
        }
    }
    
    /// Create predefined Backlog phase
    pub fn backlog() -> Self {
        Phase {
            name: "Backlog".to_string(),
            description: Some("Ideas and backlog items for consideration".to_string()),
            emoji: Some("💡".to_string()),
        }
    }
    
    /// Get all predefined phases
    pub fn predefined_phases() -> Vec<Phase> {
        vec![
            Phase::mvp(),
            Phase::beta(),
            Phase::release(),
            Phase::future(),
            Phase::backlog(),
        ]
    }
    
    /// Check if this is a predefined phase
    pub fn is_predefined(&self) -> bool {
        matches!(self.name.as_str(), "MVP" | "Beta" | "Release" | "Future" | "Backlog")
    }
    
    /// Get phase description (returns default if none set)
    pub fn description(&self) -> String {
        self.description.clone().unwrap_or_else(|| {
            if self.is_predefined() {
                match self.name.as_str() {
                    "MVP" => "Core features for minimum viable product".to_string(),
                    "Beta" => "Features for beta release and testing".to_string(),
                    "Release" => "Features for production release".to_string(),
                    "Future" => "Future enhancements and improvements".to_string(),
                    "Backlog" => "Ideas and backlog items for consideration".to_string(),
                    _ => "Custom phase".to_string(),
                }
            } else {
                "Custom phase".to_string()
            }
        })
    }
    
    /// Get phase emoji (returns default if none set)
    pub fn emoji(&self) -> String {
        self.emoji.clone().unwrap_or_else(|| {
            if self.is_predefined() {
                match self.name.as_str() {
                    "MVP" => "🚀".to_string(),
                    "Beta" => "🧪".to_string(),
                    "Release" => "🎯".to_string(),
                    "Future" => "🔮".to_string(),
                    "Backlog" => "💡".to_string(),
                    _ => "📋".to_string(),
                }
            } else {
                "📋".to_string()
            }
        })
    }
    
    /// Parse phase from string (case-insensitive)
    pub fn from_string(name: &str) -> Self {
        let normalized = name.trim().to_lowercase();
        match normalized.as_str() {
            "mvp" => Phase::mvp(),
            "beta" => Phase::beta(),
            "release" => Phase::release(),
            "future" => Phase::future(),
            "backlog" => Phase::backlog(),
            _ => Phase::new(name.trim().to_string()),
        }
    }
}

impl Default for Phase {
    fn default() -> Self {
        Phase::mvp()
    }
}

impl std::fmt::Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Represents a time tracking session for a task
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeSession {
    pub start_time: String, // ISO 8601 timestamp
    pub end_time: Option<String>, // ISO 8601 timestamp, None if session is active
    pub duration_minutes: Option<u32>, // Duration in minutes, calculated when session ends
    pub description: Option<String>, // Optional description of what was worked on
}

impl TimeSession {
    /// Create a new time session starting now
    pub fn start_now(description: Option<String>) -> Self {
        TimeSession {
            start_time: chrono::Utc::now().to_rfc3339(),
            end_time: None,
            duration_minutes: None,
            description,
        }
    }

    /// End the current session
    pub fn end_now(&mut self) {
        let now = chrono::Utc::now().to_rfc3339();
        self.end_time = Some(now.clone());
        
        // Calculate duration
        if let (Ok(start), Ok(end)) = (
            chrono::DateTime::parse_from_rfc3339(&self.start_time),
            chrono::DateTime::parse_from_rfc3339(&now)
        ) {
            let duration = end - start;
            self.duration_minutes = Some(duration.num_minutes() as u32);
        }
    }

    /// Check if session is currently active
    pub fn is_active(&self) -> bool {
        self.end_time.is_none()
    }

    /// Get duration in hours
    pub fn duration_hours(&self) -> Option<f64> {
        self.duration_minutes.map(|m| m as f64 / 60.0)
    }
}

/// Information about AI-generated content in tasks
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiTaskInfo {
    /// Whether this task was generated by AI
    pub ai_generated: bool,
    /// The AI operation that created this task (e.g., "breakdown", "suggest", "roadmap")
    pub ai_operation: Option<String>,
    /// AI reasoning or suggestions for this task
    pub ai_reasoning: Option<String>,
    /// Timestamp when AI content was added
    pub ai_timestamp: Option<String>,
    /// Model used for AI generation
    pub ai_model: Option<String>,
}

impl Default for AiTaskInfo {
    fn default() -> Self {
        AiTaskInfo {
            ai_generated: false,
            ai_operation: None,
            ai_reasoning: None,
            ai_timestamp: None,
            ai_model: None,
        }
    }
}

impl AiTaskInfo {
    pub fn new_ai_generated(operation: &str, reasoning: Option<String>, model: Option<String>) -> Self {
        AiTaskInfo {
            ai_generated: true,
            ai_operation: Some(operation.to_string()),
            ai_reasoning: reasoning,
            ai_timestamp: Some(chrono::Utc::now().to_rfc3339()),
            ai_model: model,
        }
    }
    
    pub fn add_ai_suggestion(&mut self, suggestion: String, operation: &str, model: Option<String>) {
        self.ai_reasoning = Some(suggestion);
        self.ai_operation = Some(operation.to_string());
        self.ai_timestamp = Some(chrono::Utc::now().to_rfc3339());
        self.ai_model = model;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: usize,
    pub description: String,  
    pub status: TaskStatus,
    #[serde(default)]
    pub tags: HashSet<String>,
    #[serde(default)]
    pub priority: Priority,
    #[serde(default)]
    pub phase: Phase,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub implementation_notes: Vec<String>, // Detailed implementation notes, code snippets, etc.
    #[serde(default)]
    pub dependencies: Vec<usize>, // Task IDs this task depends on
    #[serde(default)]
    pub created_at: Option<String>, // ISO 8601 timestamp
    #[serde(default)]
    pub completed_at: Option<String>, // ISO 8601 timestamp
    #[serde(default)]
    pub estimated_hours: Option<f64>, // Estimated time in hours
    #[serde(default)]
    pub actual_hours: Option<f64>, // Actual time spent in hours
    #[serde(default)]
    pub time_sessions: Vec<TimeSession>, // Individual time tracking sessions
    #[serde(default)]
    pub ai_info: AiTaskInfo, // AI-generated content and suggestions
}

impl Task {
    pub fn new(id: usize, description: String) -> Self {
        Task {
            id,
            description,
            status: TaskStatus::Pending,
            tags: HashSet::new(),
            priority: Priority::default(),
            phase: Phase::default(),
            notes: None,
            implementation_notes: Vec::new(),
            dependencies: Vec::new(),
            created_at: Some(chrono::Utc::now().to_rfc3339()),
            completed_at: None,
            estimated_hours: None,
            actual_hours: None,
            time_sessions: Vec::new(),
            ai_info: AiTaskInfo::default(),
        }
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags.into_iter().collect();
        self
    }

    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_notes(mut self, notes: String) -> Self {
        self.notes = Some(notes);
        self
    }

    pub fn with_dependencies(mut self, dependencies: Vec<usize>) -> Self {
        self.dependencies = dependencies;
        self
    }

    pub fn with_phase(mut self, phase: Phase) -> Self {
        self.phase = phase;
        self
    }

    pub fn mark_completed(&mut self) {
        self.status = TaskStatus::Completed;
        self.completed_at = Some(chrono::Utc::now().to_rfc3339());
    }

    pub fn mark_pending(&mut self) {
        self.status = TaskStatus::Pending;
        self.completed_at = None;
    }

    #[allow(dead_code)]
    pub fn add_tag(&mut self, tag: String) {
        self.tags.insert(tag);
    }

    #[allow(dead_code)]
    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.remove(tag);
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(tag)
    }

    pub fn can_be_started(&self, completed_tasks: &HashSet<usize>) -> bool {
        self.dependencies.iter().all(|dep_id| completed_tasks.contains(dep_id))
    }

    pub fn add_implementation_note(&mut self, note: String) {
        self.implementation_notes.push(note);
    }

    pub fn remove_implementation_note(&mut self, index: usize) -> Option<String> {
        if index < self.implementation_notes.len() {
            Some(self.implementation_notes.remove(index))
        } else {
            None
        }
    }

    pub fn clear_implementation_notes(&mut self) {
        self.implementation_notes.clear();
    }

    #[allow(dead_code)]
    pub fn has_implementation_notes(&self) -> bool {
        !self.implementation_notes.is_empty()
    }

    // Time tracking methods
    pub fn set_estimated_hours(&mut self, hours: f64) {
        self.estimated_hours = Some(hours);
    }

    pub fn start_time_session(&mut self, description: Option<String>) -> Result<(), String> {
        // Check if there's already an active session
        if self.has_active_time_session() {
            return Err("Task already has an active time session".to_string());
        }
        
        let session = TimeSession::start_now(description);
        self.time_sessions.push(session);
        Ok(())
    }

    pub fn end_current_time_session(&mut self) -> Result<f64, String> {
        // Find the active session index
        let active_index = self.time_sessions.iter().position(|s| s.is_active());
        
        if let Some(index) = active_index {
            // End the session
            self.time_sessions[index].end_now();
            
            // Update actual hours
            self.update_actual_hours();
            
            // Return session duration
            Ok(self.time_sessions[index].duration_hours().unwrap_or(0.0))
        } else {
            Err("No active time session found".to_string())
        }
    }

    pub fn has_active_time_session(&self) -> bool {
        self.time_sessions.iter().any(|s| s.is_active())
    }

    #[allow(dead_code)]
    pub fn get_active_time_session(&self) -> Option<&TimeSession> {
        self.time_sessions.iter().find(|s| s.is_active())
    }

    pub fn get_total_tracked_hours(&self) -> f64 {
        self.time_sessions
            .iter()
            .map(|s| s.duration_hours().unwrap_or(0.0))
            .sum()
    }

    fn update_actual_hours(&mut self) {
        self.actual_hours = Some(self.get_total_tracked_hours());
    }

    pub fn get_time_variance(&self) -> Option<f64> {
        match (self.estimated_hours, self.actual_hours) {
            (Some(estimated), Some(actual)) => Some(actual - estimated),
            _ => None,
        }
    }

    pub fn get_time_variance_percentage(&self) -> Option<f64> {
        match (self.estimated_hours, self.actual_hours) {
            (Some(estimated), Some(actual)) if estimated > 0.0 => {
                Some(((actual - estimated) / estimated) * 100.0)
            },
            _ => None,
        }
    }

    pub fn is_over_estimated(&self) -> bool {
        self.get_time_variance().map_or(false, |v| v > 0.0)
    }

    pub fn is_under_estimated(&self) -> bool {
        self.get_time_variance().map_or(false, |v| v < 0.0)
    }
    
    // AI-related methods
    pub fn mark_as_ai_generated(&mut self, operation: &str, reasoning: Option<String>, model: Option<String>) {
        self.ai_info = AiTaskInfo::new_ai_generated(operation, reasoning, model);
    }
    
    pub fn add_ai_suggestion(&mut self, suggestion: String, operation: &str, model: Option<String>) {
        self.ai_info.add_ai_suggestion(suggestion, operation, model);
    }
    
    pub fn is_ai_generated(&self) -> bool {
        self.ai_info.ai_generated
    }
    
    pub fn get_ai_operation(&self) -> Option<&String> {
        self.ai_info.ai_operation.as_ref()
    }
    
    pub fn get_ai_reasoning(&self) -> Option<&String> {
        self.ai_info.ai_reasoning.as_ref()
    }
    
    pub fn with_ai_info(mut self, operation: &str, reasoning: Option<String>, model: Option<String>) -> Self {
        self.mark_as_ai_generated(operation, reasoning, model);
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectMetadata {
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub last_modified: String,
    pub version: String,
}

impl Default for ProjectMetadata {
    fn default() -> Self {
        ProjectMetadata {
            name: "Untitled Project".to_string(),
            description: None,
            created_at: chrono::Utc::now().to_rfc3339(),
            last_modified: chrono::Utc::now().to_rfc3339(),
            version: "1.0.0".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Roadmap {
    pub title: String,
    pub tasks: Vec<Task>,
    #[serde(default)]
    pub source_file: Option<String>, // Path to the original markdown file
    #[serde(default)]
    pub metadata: ProjectMetadata,
    #[serde(default)]
    pub project_id: Option<String>, // Unique identifier for multi-project support
}

impl Roadmap {
    pub fn new(title: String) -> Self {
        let mut metadata = ProjectMetadata::default();
        metadata.name = title.clone();
        
        Roadmap {
            title,
            tasks: Vec::new(),
            source_file: None,
            metadata,
            project_id: None,
        }
    }

    pub fn with_source_file(mut self, source_file: String) -> Self {
        self.source_file = Some(source_file);
        self
    }

    #[allow(dead_code)]
    pub fn with_project_id(mut self, project_id: String) -> Self {
        self.project_id = Some(project_id);
        self
    }

    pub fn get_next_task_id(&self) -> usize {
        self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
    }

    pub fn find_task_by_id(&self, id: usize) -> Option<&Task> {
        self.tasks.iter().find(|t| t.id == id)
    }

    pub fn find_task_by_id_mut(&mut self, id: usize) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|t| t.id == id)
    }

    pub fn get_completed_task_ids(&self) -> HashSet<usize> {
        self.tasks
            .iter()
            .filter(|t| t.status == TaskStatus::Completed)
            .map(|t| t.id)
            .collect()
    }

    pub fn add_task(&mut self, mut task: Task) {
        task.id = self.get_next_task_id();
        self.tasks.push(task);
        self.update_last_modified();
    }

    pub fn remove_task(&mut self, id: usize) -> Option<Task> {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            let removed_task = self.tasks.remove(pos);
            // Renumber tasks to maintain sequential IDs
            self.renumber_tasks();
            self.update_last_modified();
            Some(removed_task)
        } else {
            None
        }
    }

    fn renumber_tasks(&mut self) {
        // First pass: collect ID mappings
        let mut id_mappings = Vec::new();
        for (index, task) in self.tasks.iter().enumerate() {
            let old_id = task.id;
            let new_id = index + 1;
            if old_id != new_id {
                id_mappings.push((old_id, new_id));
            }
        }
        
        // Second pass: update task IDs
        for (index, task) in self.tasks.iter_mut().enumerate() {
            task.id = index + 1;
        }
        
        // Third pass: update dependencies
        for task in &mut self.tasks {
            for (old_id, new_id) in &id_mappings {
                if let Some(pos) = task.dependencies.iter().position(|&dep| dep == *old_id) {
                    task.dependencies[pos] = *new_id;
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn filter_by_tags(&self, tags: &[String]) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|task| tags.iter().any(|tag| task.has_tag(tag)))
            .collect()
    }

    #[allow(dead_code)]
    pub fn filter_by_priority(&self, priority: &Priority) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|task| &task.priority == priority)
            .collect()
    }

    #[allow(dead_code)]
    pub fn filter_by_status(&self, status: &TaskStatus) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|task| &task.status == status)
            .collect()
    }

    pub fn filter_by_phase(&self, phase: &Phase) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|task| &task.phase == phase)
            .collect()
    }

    pub fn search_tasks(&self, query: &str) -> Vec<&Task> {
        let query_lower = query.to_lowercase();
        self.tasks
            .iter()
            .filter(|task| {
                task.description.to_lowercase().contains(&query_lower)
                    || task.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))
                    || task.notes.as_ref().map_or(false, |notes| notes.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    fn update_last_modified(&mut self) {
        self.metadata.last_modified = chrono::Utc::now().to_rfc3339();
    }

    pub fn get_statistics(&self) -> RoadmapStatistics {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.status == TaskStatus::Completed).count();
        let pending = total - completed;
        
        let by_priority = [
            (Priority::Critical, self.tasks.iter().filter(|t| t.priority == Priority::Critical).count()),
            (Priority::High, self.tasks.iter().filter(|t| t.priority == Priority::High).count()),
            (Priority::Medium, self.tasks.iter().filter(|t| t.priority == Priority::Medium).count()),
            (Priority::Low, self.tasks.iter().filter(|t| t.priority == Priority::Low).count()),
        ];

        // Group tasks by phase name dynamically
        let mut phase_counts: HashMap<String, usize> = HashMap::new();
        for task in &self.tasks {
            *phase_counts.entry(task.phase.name.clone()).or_insert(0) += 1;
        }
        
        // Convert to Vec<(Phase, usize)> for compatibility
        let by_phase: Vec<(Phase, usize)> = phase_counts.into_iter()
            .map(|(name, count)| (Phase::from_string(&name), count))
            .collect();

        let all_tags: HashSet<String> = self.tasks.iter()
            .flat_map(|t| &t.tags)
            .cloned()
            .collect();

        RoadmapStatistics {
            total_tasks: total,
            completed_tasks: completed,
            pending_tasks: pending,
            tasks_by_priority: by_priority.into_iter().collect(),
            tasks_by_phase: by_phase,
            unique_tags: all_tags.len(),
            completion_percentage: if total > 0 { (completed * 100) / total } else { 0 },
        }
    }

    /// Validate dependency relationships for a specific task
    pub fn validate_task_dependencies(&self, task_id: usize) -> Result<(), Vec<DependencyError>> {
        let mut errors = Vec::new();
        
        if let Some(task) = self.find_task_by_id(task_id) {
            // Check if all dependencies exist
            for &dep_id in &task.dependencies {
                if self.find_task_by_id(dep_id).is_none() {
                    errors.push(DependencyError::MissingDependency { task_id, missing_dep_id: dep_id });
                }
            }
            
            // Check for circular dependencies
            if let Err(cycle) = self.check_circular_dependencies_for_task(task_id) {
                errors.push(DependencyError::CircularDependency { cycle });
            }
        } else {
            errors.push(DependencyError::TaskNotFound { task_id });
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Validate all dependency relationships in the roadmap
    pub fn validate_all_dependencies(&self) -> Result<(), Vec<DependencyError>> {
        let mut all_errors = Vec::new();
        
        for task in &self.tasks {
            if let Err(mut errors) = self.validate_task_dependencies(task.id) {
                all_errors.append(&mut errors);
            }
        }
        
        if all_errors.is_empty() {
            Ok(())
        } else {
            Err(all_errors)
        }
    }

    /// Check for circular dependencies starting from a specific task
    fn check_circular_dependencies_for_task(&self, start_task_id: usize) -> Result<(), Vec<usize>> {
        let mut visited = HashSet::new();
        let mut path = Vec::new();
        
        if self.has_circular_dependency_recursive(start_task_id, &mut visited, &mut path) {
            Err(path)
        } else {
            Ok(())
        }
    }

    fn has_circular_dependency_recursive(
        &self, 
        task_id: usize, 
        visited: &mut HashSet<usize>, 
        path: &mut Vec<usize>
    ) -> bool {
        if path.contains(&task_id) {
            // Found a cycle - add the current task to complete the cycle
            path.push(task_id);
            return true;
        }
        
        if visited.contains(&task_id) {
            return false;
        }
        
        visited.insert(task_id);
        path.push(task_id);
        
        if let Some(task) = self.find_task_by_id(task_id) {
            for &dep_id in &task.dependencies {
                if self.has_circular_dependency_recursive(dep_id, visited, path) {
                    return true;
                }
            }
        }
        
        path.pop();
        false
    }

    /// Get the full dependency chain for a task (all tasks it depends on, recursively)
    pub fn get_dependency_chain(&self, task_id: usize) -> Vec<usize> {
        let mut chain = Vec::new();
        let mut visited = HashSet::new();
        self.collect_dependencies_recursive(task_id, &mut chain, &mut visited);
        chain
    }

    fn collect_dependencies_recursive(
        &self, 
        task_id: usize, 
        chain: &mut Vec<usize>, 
        visited: &mut HashSet<usize>
    ) {
        if visited.contains(&task_id) {
            return;
        }
        visited.insert(task_id);
        
        if let Some(task) = self.find_task_by_id(task_id) {
            for &dep_id in &task.dependencies {
                if !chain.contains(&dep_id) {
                    chain.push(dep_id);
                }
                self.collect_dependencies_recursive(dep_id, chain, visited);
            }
        }
    }

    /// Get all tasks that depend on a specific task (reverse dependencies)
    pub fn get_dependents(&self, task_id: usize) -> Vec<usize> {
        self.tasks
            .iter()
            .filter(|task| task.dependencies.contains(&task_id))
            .map(|task| task.id)
            .collect()
    }

    /// Get tasks that are ready to be started (all dependencies completed)
    pub fn get_ready_tasks(&self) -> Vec<&Task> {
        let completed_ids = self.get_completed_task_ids();
        self.tasks
            .iter()
            .filter(|task| task.status == TaskStatus::Pending && task.can_be_started(&completed_ids))
            .collect()
    }

    /// Get tasks that are blocked by incomplete dependencies
    pub fn get_blocked_tasks(&self) -> Vec<&Task> {
        let completed_ids = self.get_completed_task_ids();
        self.tasks
            .iter()
            .filter(|task| task.status == TaskStatus::Pending && !task.can_be_started(&completed_ids))
            .collect()
    }

    /// Get detailed dependency tree for visualization
    pub fn get_dependency_tree(&self, task_id: usize) -> Option<DependencyNode> {
        if let Some(_task) = self.find_task_by_id(task_id) {
            let mut visited = HashSet::new();
            Some(self.build_dependency_tree_recursive(task_id, &mut visited))
        } else {
            None
        }
    }

    fn build_dependency_tree_recursive(&self, task_id: usize, visited: &mut HashSet<usize>) -> DependencyNode {
        if visited.contains(&task_id) {
            // Circular reference detected
            return DependencyNode {
                task_id,
                description: "[Circular Reference]".to_string(),
                status: TaskStatus::Pending,
                dependencies: Vec::new(),
                is_circular: true,
            };
        }
        
        visited.insert(task_id);
        
        let task = match self.find_task_by_id(task_id) {
            Some(task) => task,
            None => {
                // Task not found - return a placeholder node
                return DependencyNode {
                    task_id,
                    description: "[Task Not Found]".to_string(),
                    status: TaskStatus::Pending,
                    dependencies: Vec::new(),
                    is_circular: false,
                };
            }
        };
        let dependencies = task.dependencies
            .iter()
            .map(|&dep_id| self.build_dependency_tree_recursive(dep_id, visited))
            .collect();
        
        visited.remove(&task_id);
        
        DependencyNode {
            task_id,
            description: task.description.clone(),
            status: task.status.clone(),
            dependencies,
            is_circular: false,
        }
    }

    /// Get all unique phases from the roadmap tasks
    pub fn get_all_phases(&self) -> Vec<Phase> {
        let mut phase_names: HashSet<String> = HashSet::new();
        let mut phases: Vec<Phase> = Vec::new();
        
        // Collect unique phases from tasks
        for task in &self.tasks {
            if phase_names.insert(task.phase.name.clone()) {
                phases.push(task.phase.clone());
            }
        }
        
        // Sort phases: predefined phases first in their natural order, then custom phases alphabetically
        phases.sort_by(|a, b| {
            let a_predefined = a.is_predefined();
            let b_predefined = b.is_predefined();
            
            match (a_predefined, b_predefined) {
                (true, true) => {
                    // Both predefined - use predefined order
                    let predefined_order = ["MVP", "Beta", "Release", "Future", "Backlog"];
                    let a_index = predefined_order.iter().position(|&x| x == a.name).unwrap_or(999);
                    let b_index = predefined_order.iter().position(|&x| x == b.name).unwrap_or(999);
                    a_index.cmp(&b_index)
                }
                (true, false) => std::cmp::Ordering::Less,  // Predefined comes first
                (false, true) => std::cmp::Ordering::Greater, // Custom comes after
                (false, false) => a.name.cmp(&b.name), // Both custom - alphabetical
            }
        });
        
        phases
    }

    /// Get phases that have tasks (non-empty phases)
    pub fn get_active_phases(&self) -> Vec<Phase> {
        self.get_all_phases().into_iter()
            .filter(|phase| self.tasks.iter().any(|task| task.phase.name == phase.name))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct DependencyNode {
    pub task_id: usize,
    pub description: String,
    pub status: TaskStatus,
    pub dependencies: Vec<DependencyNode>,
    pub is_circular: bool,
}

#[derive(Debug, Clone)]
pub enum DependencyError {
    TaskNotFound { task_id: usize },
    MissingDependency { task_id: usize, missing_dep_id: usize },
    CircularDependency { cycle: Vec<usize> },
}

impl std::fmt::Display for DependencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DependencyError::TaskNotFound { task_id } => {
                write!(f, "Task #{} not found", task_id)
            }
            DependencyError::MissingDependency { task_id, missing_dep_id } => {
                write!(f, "Task #{} depends on missing task #{}", task_id, missing_dep_id)
            }
            DependencyError::CircularDependency { cycle } => {
                let cycle_str = cycle.iter()
                    .map(|id| format!("#{}", id))
                    .collect::<Vec<_>>()
                    .join(" → ");
                write!(f, "Circular dependency detected: {}", cycle_str)
            }
        }
    }
}

impl std::error::Error for DependencyError {}

#[derive(Debug)]
pub struct RoadmapStatistics {
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub pending_tasks: usize,
    #[allow(dead_code)]
    pub tasks_by_priority: Vec<(Priority, usize)>,
    pub tasks_by_phase: Vec<(Phase, usize)>,
    #[allow(dead_code)]
    pub unique_tags: usize,
    pub completion_percentage: usize,
}
