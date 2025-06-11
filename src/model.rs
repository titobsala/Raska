use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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
    pub notes: Option<String>,
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
            notes: None,
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

        let all_tags: HashSet<String> = self.tasks.iter()
            .flat_map(|t| &t.tags)
            .cloned()
            .collect();

        RoadmapStatistics {
            total_tasks: total,
            completed_tasks: completed,
            pending_tasks: pending,
            tasks_by_priority: by_priority.into_iter().collect(),
            unique_tags: all_tags.len(),
            completion_percentage: if total > 0 { (completed * 100) / total } else { 0 },
        }
    }
}

#[derive(Debug)]
pub struct RoadmapStatistics {
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub pending_tasks: usize,
    pub tasks_by_priority: Vec<(Priority, usize)>,
    pub unique_tags: usize,
    pub completion_percentage: usize,
}
