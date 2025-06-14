use serde::{Deserialize, Serialize};
use std::collections::{HashSet, HashMap};

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

    pub fn add_tag(&mut self, tag: String) {
        self.tags.insert(tag);
    }

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

    pub fn has_implementation_notes(&self) -> bool {
        !self.implementation_notes.is_empty()
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

    pub fn filter_by_tags(&self, tags: &[String]) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|task| tags.iter().any(|tag| task.has_tag(tag)))
            .collect()
    }

    pub fn filter_by_priority(&self, priority: &Priority) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|task| &task.priority == priority)
            .collect()
    }

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
        
        let task = self.find_task_by_id(task_id).unwrap();
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
    pub tasks_by_priority: Vec<(Priority, usize)>,
    pub tasks_by_phase: Vec<(Phase, usize)>,
    pub unique_tags: usize,
    pub completion_percentage: usize,
}
