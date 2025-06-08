mod model;
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
