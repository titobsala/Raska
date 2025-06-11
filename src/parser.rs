use crate::model::{Roadmap, Task, TaskStatus};
use pulldown_cmark::{Event, Parser as CmarkParser, Tag};
use std::io::{Error, ErrorKind};
use std::path::Path;

fn extract_text(parser: &mut CmarkParser) -> String {
    let mut text = String::new();
    
    // Continue parsing until we reach the end of the list item
    while let Some(event) = parser.next() {
        match event {
            Event::Text(t) => text.push_str(&t),
            Event::Code(t) => text.push_str(&t), // Handle inline code
            Event::End(_) => break,              // End of any tag
            _ => {}
        }
    }
    
    text
}

pub fn parse_markdown_to_roadmap(markdown_input: &str, source_file: Option<&Path>) -> Result<Roadmap, Error> {
    let mut parser = CmarkParser::new(markdown_input);
    let mut roadmap_title = String::new();
    let mut tasks: Vec<Task> = Vec::new();
    let mut task_id_counter = 0;

    while let Some(event) = parser.next() {
        match event {
            Event::Start(Tag::Heading { level: pulldown_cmark::HeadingLevel::H1, .. }) => {
                roadmap_title = extract_text(&mut parser);
            }
            Event::Start(Tag::Item) => {
                let task_text = extract_text(&mut parser);
                task_id_counter += 1;
                
                // Check if task is already completed (checkbox syntax)
                let (description, status) = parse_task_text(&task_text);
                
                let mut task = Task::new(task_id_counter, description);
                if status == TaskStatus::Completed {
                    task.mark_completed();
                }
                
                tasks.push(task);
            }
            _ => {}
        }
    }

    if roadmap_title.is_empty() {
        return Err(Error::new(ErrorKind::InvalidData, "Markdown is missing a project title (H1 heading)."));
    }

    let mut roadmap = Roadmap::new(roadmap_title);
    roadmap.tasks = tasks;
    if let Some(source) = source_file {
        roadmap = roadmap.with_source_file(source.to_string_lossy().to_string());
    }

    Ok(roadmap)
}

/// Parse task text to extract description and status
/// Supports both checkbox syntax and plain text
fn parse_task_text(text: &str) -> (String, TaskStatus) {
    let trimmed = text.trim();
    
    // Check for completed checkbox: [x] or [X]
    if trimmed.starts_with("[x]") || trimmed.starts_with("[X]") {
        let description = trimmed[3..].trim().to_string();
        return (description, TaskStatus::Completed);
    }
    
    // Check for unchecked checkbox: [ ]
    if trimmed.starts_with("[ ]") {
        let description = trimmed[3..].trim().to_string();
        return (description, TaskStatus::Pending);
    }
    
    // Default: plain text, assume pending
    (trimmed.to_string(), TaskStatus::Pending)
}
