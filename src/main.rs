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

use cli::{Commands, ProjectCommands, PhaseCommands, NotesCommands};
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
        Commands::Add { description, tag, priority, phase, note, dependencies } => {
            commands::add_task_enhanced(description, tag, priority, phase, note, dependencies)
        },
        Commands::Remove { id } => commands::remove_task(*id),
        Commands::Edit { id, description } => commands::edit_task(*id, description),
        Commands::Reset { id } => commands::reset_tasks(*id),
        Commands::List { tag, priority, phase, status, search, detailed } => {
            commands::list_tasks(tag, priority, phase, status, search, *detailed)
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
                ProjectCommands::Switcher => {
                    commands::project_switcher()
                },
                ProjectCommands::Delete { name, force } => {
                    commands::delete_project(name, *force)
                },
            }
        },
        Commands::Dependencies { task_id, validate, show_ready, show_blocked } => {
            commands::analyze_dependencies(task_id, *validate, *show_ready, *show_blocked)
        },
        Commands::Phase(phase_command) => {
            match phase_command {
                PhaseCommands::List => commands::list_phases(),
                PhaseCommands::Show { phase } => commands::show_phase_tasks(phase),
                PhaseCommands::Set { task_id, phase } => commands::set_task_phase(*task_id, phase),
                PhaseCommands::Overview => commands::show_phase_overview(),
                PhaseCommands::Create { name, description, emoji } => commands::create_custom_phase(name, description.as_deref(), emoji.as_deref()),
            }
        },
        Commands::Config(config_command) => {
            commands::handle_config_command(config_command)
        },
        Commands::View { id } => {
            commands::view_task(*id)
        },
        Commands::Bulk(bulk_command) => {
            commands::handle_bulk_command(bulk_command)
        },
        Commands::Notes(notes_command) => {
            handle_notes_command(notes_command)
        },
        Commands::Export { format, output, include_completed, tags, priority, phase, pretty } => {
            commands::export_roadmap(format, output.as_deref(), *include_completed, tags.as_deref(), priority.as_ref(), phase.as_ref(), *pretty)
        },
        Commands::Template(template_command) => {
            commands::handle_template_command(template_command.clone())
        },
    }
}

/// Handle notes command routing
fn handle_notes_command(notes_command: &NotesCommands) -> commands::CommandResult {
    match notes_command {
        NotesCommands::Add { task_id, note } => {
            commands::add_implementation_note(*task_id, note.clone())
        },
        NotesCommands::List { task_id } => {
            commands::list_implementation_notes(*task_id)
        },
        NotesCommands::Remove { task_id, index } => {
            commands::remove_implementation_note(*task_id, *index)
        },
        NotesCommands::Clear { task_id } => {
            commands::clear_implementation_notes(*task_id)
        },
        NotesCommands::Edit { task_id, index, note } => {
            commands::edit_implementation_note(*task_id, *index, note.clone())
        },
    }
}
