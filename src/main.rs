// Module declarations
mod cli;
mod commands;
mod markdown_writer;
mod model;
mod parser;
mod state;
mod ui;

use cli::{Commands, ProjectCommands};
use std::process;

fn main() {
    // Parse command line arguments
    let cli = cli::parse_args();
    
    // Execute the command and handle errors
    if let Err(e) = run_command(&cli.command) {
        ui::display_error(&e.to_string());
        process::exit(1);
    }
}

/// Route commands to their respective handlers
fn run_command(command: &Commands) -> commands::CommandResult {
    match command {
        Commands::Init { filepath } => commands::init_project(filepath),
        Commands::Show => commands::show_project(),
        Commands::Complete { id } => commands::complete_task(*id),
        Commands::Add { description, tag, priority, note, dependencies } => {
            commands::add_task_enhanced(description, tag, priority, note, dependencies)
        },
        Commands::Remove { id } => commands::remove_task(*id),
        Commands::Edit { id, description } => commands::edit_task(*id, description),
        Commands::Reset { id } => commands::reset_tasks(*id),
        Commands::List { tag, priority, status, search, detailed } => {
            commands::list_tasks(tag, priority, status, search, *detailed)
        },
        Commands::Project(project_command) => {
            match project_command {
                ProjectCommands::Create { name, description } => {
                    commands::create_project(name, description)
                },
                ProjectCommands::Switch { name } => {
                    commands::switch_project(name)
                },
                ProjectCommands::List => {
                    commands::list_projects()
                },
                ProjectCommands::Delete { name, force } => {
                    commands::delete_project(name, *force)
                },
            }
        }
    }
}
