use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    Completed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: usize,
    pub description: String,
    pub status: TaskStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Roadmap {
    pub title: String,
    pub tasks: Vec<Task>,
    pub source_file: Option<String>, // Path to the original markdown file
}
