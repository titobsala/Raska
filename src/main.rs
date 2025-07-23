// Module declarations
mod ai;
mod cli;
mod commands;
mod config;
mod markdown_writer;
mod model;
mod parser;
mod state;
mod ui;
mod web;

use cli::{Commands, PhaseCommands, NotesCommands};
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
        Commands::Show { group_by_phase, phase, detailed, collapse_completed } => {
            commands::show_project_enhanced(*group_by_phase, phase.as_deref(), *detailed, *collapse_completed)
        },
        Commands::Complete { id } => commands::complete_task(*id),
        Commands::Add { description, tag, priority, phase, note, dependencies, estimated_hours } => {
            commands::add_task_enhanced(description, tag, priority, phase, note, dependencies, estimated_hours)
        },
        Commands::Quick { text } => {
            commands::quick_add_task(text)
        },
        Commands::Remove { id } => commands::remove_task(*id),
        Commands::Edit { id, description } => commands::edit_task(*id, description),
        Commands::Reset { id } => commands::reset_tasks(*id),
        Commands::List { tag, priority, phase, status, search, detailed } => {
            commands::list_tasks(tag, priority, phase, status, search, *detailed)
        },
        Commands::Dependencies { task_id, validate, show_ready, show_blocked } => {
            commands::analyze_dependencies(task_id, *validate, *show_ready, *show_blocked)
        },
        Commands::Ready => commands::show_ready_tasks(),
        Commands::Urgent => commands::show_urgent_tasks(),
        Commands::Blocked => commands::show_blocked_tasks(),
        Commands::Find { query } => commands::find_tasks(query),
        Commands::Phase(phase_command) => {
            match phase_command {
                PhaseCommands::List => commands::list_phases(),
                PhaseCommands::Show { phase } => commands::show_phase_tasks(phase),
                PhaseCommands::Set { task_id, phase } => commands::set_task_phase(*task_id, phase),
                PhaseCommands::Overview => commands::show_phase_overview(),
                PhaseCommands::Create { name, description, emoji } => commands::create_custom_phase(name, description.as_deref(), emoji.as_deref()),
                PhaseCommands::Fork { new_phase, from_phase, task_ids, description, emoji, copy } => {
                    commands::fork_phase_or_tasks(new_phase, from_phase.as_deref(), task_ids.as_deref(), description.as_deref(), emoji.as_deref(), *copy)
                },
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
        Commands::Export { 
            format, output, include_completed, tags, priority, phase, pretty,
            created_after, created_before, min_estimated_hours, max_estimated_hours,
            min_actual_hours, max_actual_hours, with_time_data, active_sessions_only,
            over_estimated_only, under_estimated_only
        } => {
            commands::export_roadmap_enhanced(
                format, output.as_deref(), *include_completed, tags.as_deref(), 
                priority.as_ref(), phase.as_ref(), *pretty,
                created_after.as_deref(), created_before.as_deref(),
                *min_estimated_hours, *max_estimated_hours,
                *min_actual_hours, *max_actual_hours,
                *with_time_data, *active_sessions_only,
                *over_estimated_only, *under_estimated_only
            )
        },
        Commands::Template(template_command) => {
            commands::handle_template_command(template_command.clone())
        },
        Commands::Start { id, description } => {
            commands::start_time_tracking(*id, description.as_deref())
        },
        Commands::Stop => {
            commands::stop_time_tracking()
        },
        Commands::Time { task_id, summary, detailed } => {
            commands::show_time_tracking(task_id, *summary, *detailed)
        },
        Commands::Analytics { overview, time, phases, priorities, trends, export, all } => {
            commands::show_analytics(
                *overview || *all, 
                *time || *all, 
                *phases || *all, 
                *priorities || *all, 
                *trends || *all, 
                export.as_ref().map(|p| p.to_string_lossy().to_string())
            )
        },
        Commands::Timeline { detailed, active_only, compact, page, page_size } => {
            commands::show_timeline(*detailed, *active_only, *compact, *page, *page_size)
        },
        Commands::Ai(ai_command) => {
            commands::handle_ai_command(ai_command)
        },
        Commands::Interactive { project, no_welcome } => {
            commands::run_interactive_mode(project.as_deref(), *no_welcome)
        },
        Commands::Sync { from_roadmap, from_details, from_global, to_files, force, dry_run } => {
            commands::sync_project_files(*from_roadmap, *from_details, *from_global, *to_files, *force, *dry_run)
        },
        Commands::Web { port, host, daemon, stop, status, open } => {
            commands::handle_web_command(*port, host, *daemon, *stop, *status, *open)
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
