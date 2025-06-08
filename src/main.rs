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
            println!("\n{} {} Project initialized successfully!", "🎯".to_string(), "Success:".green().bold());
            println!("   📝 Project: {}", roadmap.title.bright_cyan());
            println!("   📊 Total tasks: {}", roadmap.tasks.len().to_string().bright_yellow());
            println!("   💾 State saved to: {}", ".rask_state.json".dimmed());
            println!("\n   💡 Use {} to view your tasks!", "rask show".bright_green());
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
                println!("\n{} {} Task #{} completed!", "✨".to_string(), "Success:".green().bold(), id);
                println!("   🎊 Well done! Keep up the great work!\n");
                print_roadmap(&roadmap);
            } else {
                return Err(format!("Task with ID {} not found.", id).into());
            }
        }
    }
    Ok(())
}

fn print_roadmap(roadmap: &Roadmap) {
    // Calculate progress statistics
    let total_tasks = roadmap.tasks.len();
    let completed_tasks = roadmap.tasks.iter().filter(|t| t.status == TaskStatus::Completed).count();
    let progress_percentage = if total_tasks > 0 { (completed_tasks * 100) / total_tasks } else { 0 };
    
    // Print header with project title
    println!("\n{}", "═".repeat(60).bright_blue());
    println!("  {}", roadmap.title.bold().bright_cyan());
    println!("{}", "═".repeat(60).bright_blue());
    
    // Print progress bar
    let bar_width = 40;
    let filled = (completed_tasks * bar_width) / total_tasks.max(1);
    let empty = bar_width - filled;
    
    print!("  Progress: [");
    print!("{}", "█".repeat(filled).green());
    print!("{}", "░".repeat(empty).dimmed());
    println!("] {}% ({}/{})", progress_percentage, completed_tasks, total_tasks);
    
    // Print tasks section header
    println!("\n  {} Tasks:", "📋".to_string().bold());
    println!("  {}", "─".repeat(50).dimmed());
    
    // Print tasks with better formatting
    for task in &roadmap.tasks {
        match task.status {
            TaskStatus::Pending => {
                println!("  {} {} {}", 
                    "□".bright_white(), 
                    format!("#{:2}", task.id).dimmed(), 
                    task.description.white()
                );
            }
            TaskStatus::Completed => {
                println!("  {} {} {}", 
                    "✓".bright_green().bold(), 
                    format!("#{:2}", task.id).strikethrough().dimmed(), 
                    task.description.strikethrough().dimmed()
                );
            }
        }
    }
    
    // Print footer
    println!("  {}", "─".repeat(50).dimmed());
    if completed_tasks == total_tasks && total_tasks > 0 {
        println!("  {} All tasks completed! Great job! {}", "🎉".to_string(), "🎉".to_string());
    } else if completed_tasks > 0 {
        println!("  {} Keep going! {} tasks remaining.", "💪".to_string(), total_tasks - completed_tasks);
    } else {
        println!("  {} Ready to start? Complete your first task!", "🚀".to_string());
    }
    println!();
}
