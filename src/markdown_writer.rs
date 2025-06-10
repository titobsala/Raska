use crate::model::{Roadmap, TaskStatus};
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

/// Write a roadmap back to a markdown file
pub fn write_roadmap_to_file(roadmap: &Roadmap, file_path: &Path) -> Result<(), Error> {
    let markdown_content = roadmap_to_markdown(roadmap);
    fs::write(file_path, markdown_content)
}

/// Convert a roadmap back to markdown format
fn roadmap_to_markdown(roadmap: &Roadmap) -> String {
    let mut content = String::new();
    
    // Add the title
    content.push_str(&format!("# {}\n\n", roadmap.title));
    
    // Add description (if we want to preserve it, we'd need to store it)
    content.push_str("This file outlines the tasks required to build the MVP for the Rask application.\n\n");
    
    // Add tasks
    for task in &roadmap.tasks {
        let checkbox = match task.status {
            TaskStatus::Pending => "[ ]",
            TaskStatus::Completed => "[x]",
        };
        content.push_str(&format!("- {} {}\n", checkbox, task.description));
    }
    
    content
}

/// Update the original markdown file with current task statuses
pub fn sync_to_source_file(roadmap: &Roadmap) -> Result<(), Error> {
    if let Some(source_file) = &roadmap.source_file {
        let path = Path::new(source_file);
        if path.exists() {
            write_roadmap_to_file(roadmap, path)?;
            println!("   📝 Synced changes to {}", source_file);
        } else {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("Source file not found: {}", source_file)
            ));
        }
    }
    Ok(())
} 