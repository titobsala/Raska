Rask - Detailed MVP Build PlanThis document will guide you step-by-step through creating the first version of the Rask application. We will create the folder structure, write the code for each file, and run the program.Phase 1: Create the Project StructureFirst, we need to create the folders and empty files. This is the skeleton of our application.Open your terminal.Navigate to the directory where you want to create your project (e.g., ~/Documents/projects).Run the following commands to create the directory structure:# Create the main project folder
mkdir rask

# Move into the new folder
cd rask

# Create the `src` directory for our Rust source code
mkdir src

# Create the empty files we will edit
touch Cargo.toml
touch roadmap.md
touch src/main.rs
touch src/model.rs
touch src/parser.rs
touch src/state.rs
After running these commands, your folder structure inside your code editor (Cursor) should look exactly like this:rask/
├── Cargo.toml      <-- (empty for now)
├── roadmap.md      <-- (empty for now)
└── src/
    ├── main.rs     <-- (empty for now)
    ├── model.rs    <-- (empty for now)
    ├── parser.rs   <-- (empty for now)
    └── state.rs    <-- (empty for now)
Phase 2: Define Project Metadata and DependenciesNow, let's tell Rust what our project is and what external libraries (crates) it needs.Open the Cargo.toml file.Copy and paste the following code into it. This file defines our project's name and adds our necessary dependencies like clap for the CLI and serde for handling data.[package]
name = "rask"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4.5.4", features = ["derive"] }
serde = { version = "1.0.203", features = ["derive"] }
serde_json = "1.0.117"
pulldown-cmark = "0.10.0"
colored = "2.1.0"
Phase 3: Create the Input File (roadmap.md)This is the sample markdown file that our program will read.Open the roadmap.md file.Copy and paste the following content into it:# Project: Rask - A CLI Project Planner in Rust

This file outlines the tasks required to build the MVP for the Rask application.

- Initialize the Rust project and add dependencies.
- Define the core data structures (Task, Roadmap, Status).
- Implement the function to parse a markdown file.
- Extract the project title from the H1 heading.
- Extract tasks from markdown list items.
- Implement the `init` command using clap.
- Implement the `show` command.
- Implement the `complete` command.
- Add logic to save the project state to a `.json` file.
- Add logic to load the project state from the `.json` file.
- Use the `colored` crate to improve the `show` command's output.
- Write usage instructions in the `README.md` file.
Phase 4: Write the Rust CodeNow we will fill in our empty .rs files with the application's logic.Open src/model.rs. This file defines our application's data structures. Paste this code:use serde::{Deserialize, Serialize};

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
}
Open src/parser.rs. This file contains the logic for reading the markdown. Paste this code:use crate::model::{Roadmap, Task, TaskStatus};
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
Open src/state.rs. This handles saving and loading our .rask_state.json file. Paste this code:use crate::model::Roadmap;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

const STATE_FILE: &str = ".rask_state.json";

pub fn save_state(roadmap: &Roadmap) -> Result<(), Error> {
    let json_data = serde_json::to_string_pretty(roadmap)
        .map_err(|e| Error::new(ErrorKind::Other, e))?;
    fs::write(STATE_FILE, json_data)
}

pub fn load_state() -> Result<Roadmap, Error> {
    if !Path::new(STATE_FILE).exists() {
        return Err(Error::new(ErrorKind::NotFound, "State file not found. Please run 'init' first."));
    }
    let json_data = fs::read_to_string(STATE_FILE)?;
    let roadmap: Roadmap = serde_json::from_str(&json_data)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    Ok(roadmap)
}
Open src/main.rs. This is the final and most important file, as it runs the whole application. Paste this code:mod model;
mod parser;
mod state;

use clap::{Parser as ClapParser, Subcommand};
use colored::*;
use model::{Roadmap, TaskStatus};
use std::fs;
use std::path::PathBuf;
use std::process;

#[derive(ClapParser)]
#[command(version, about, long_about = "A CLI to manage project tasks from a Markdown file.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init { #[arg(value_name = "FILE")] filepath: PathBuf },
    Show,
    Complete { #[arg(value_name = "TASK_ID")] id: usize },
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run_command(&cli.command) {
        eprintln!("{} {}", "Error:".red().bold(), e);
        process::exit(1);
    }
}

fn run_command(command: &Commands) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Commands::Init { filepath } => {
            let markdown_content = fs::read_to_string(filepath)?;
            let roadmap = parser::parse_markdown_to_roadmap(&markdown_content)?;
            state::save_state(&roadmap)?;
            println!("{} Project '{}' initialized with {} tasks.", "Success:".green().bold(), roadmap.title, roadmap.tasks.len());
        }
        Commands::Show => {
            let roadmap = state::load_state()?;
            print_roadmap(&roadmap);
        }
        Commands::Complete { id } => {
            let mut roadmap = state::load_state()?;
            let task = roadmap.tasks.iter_mut().find(|t| t.id == *id);
            if let Some(task) = task {
                task.status = TaskStatus::Completed;
                state::save_state(&roadmap)?;
                println!("{} Task {} marked as complete.", "Success:".green().bold(), id);
                print_roadmap(&roadmap);
            } else {
                return Err(format!("Task with ID {} not found.", id).into());
            }
        }
    }
    Ok(())
}

fn print_roadmap(roadmap: &Roadmap) {
    println!("\n  {}\n", roadmap.title.bold().underline());
    for task in &roadmap.tasks {
        match task.status {
            TaskStatus::Pending => {
                println!("  [ ] {}: {}", task.id, task.description);
            }
            TaskStatus::Completed => {
                println!("  {} {}: {}", "[✔]".green(), task.id.to_string().strikethrough(), task.description.strikethrough().dimmed());
            }
        }
    }
    println!();
}
Phase 5: First Run and TestingYou have now written the entire application! Let's compile and run it.Go back to your terminal, making sure you are still in the rask directory.Run the init command. Cargo will automatically download the dependencies and compile your code the first time.cargo run -- init roadmap.md
You should see a success message.Run the show command to see your list of tasks.cargo run -- show
Complete a task!cargo run -- complete 1
You should see the task list printed again, but with the first task marked as complete.You have successfully built and tested the MVP! You can now use the rask_next_steps_plan to start adding new features.