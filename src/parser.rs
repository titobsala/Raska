use crate::model::{Roadmap, Task, TaskStatus};
use pulldown_cmark::{Event, Parser as CmarkParser, Tag};
use std::io::{Error, ErrorKind};

fn extract_text(parser: &mut CmarkParser) -> String {
    let mut text = String::new();
    if let Some(Event::Text(t)) = parser.next() {
        text.push_str(&t);
    }
    text
}

pub fn parse_markdown_to_roadmap(markdown_input: &str) -> Result<Roadmap, Error> {
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
                tasks.push(Task {
                    id: task_id_counter,
                    description: task_text.trim().to_string(),
                    status: TaskStatus::Pending,
                });
            }
            _ => {}
        }
    }

    if roadmap_title.is_empty() {
        return Err(Error::new(ErrorKind::InvalidData, "Markdown is missing a project title (H1 heading)."));
    }

    Ok(Roadmap {
        title: roadmap_title,
        tasks,
    })
}
