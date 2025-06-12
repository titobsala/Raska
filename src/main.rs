// Module declarations
mod cli;
mod commands;
mod config;
mod markdown_writer;
mod model;
mod parser;
mod project;
mod state;
mod ui;

use cli::{Commands, ProjectCommands};
use std::process;

fn main() {
    // Initialize or migrate configuration on first run
    if let Err(e) = initialize_rask() {
        ui::display_warning(&format!("Initialization warning: {}", e));
    }
    
    // Parse command line arguments
    let cli = cli::parse_args();
    
    // Execute the command and handle errors
    if let Err(e) = run_command(&cli.command) {
        ui::display_error(&e.to_string());
        process::exit(1);
    }
}

/// Initialize Rask configuration and directory structure
/// This handles first-time setup and migration from legacy versions
fn initialize_rask() -> Result<(), Box<dyn std::error::Error>> {
    // Create necessary directories
    config::get_rask_config_dir()?;
    config::get_rask_data_dir()?;
    
    // Migrate legacy files if they exist
    project::migrate_legacy_files()?;
    
    // Initialize user configuration if it doesn't exist
    if config::RaskConfig::load_user_config().is_err() {
        config::RaskConfig::init_user_config()?;
    }
    
    Ok(())
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
        },
        Commands::Dependencies { task_id, validate, show_ready, show_blocked } => {
            commands::analyze_dependencies(task_id, *validate, *show_ready, *show_blocked)
        },
        Commands::Config(config_command) => {
            commands::handle_config_command(config_command)
        },
    }
}
