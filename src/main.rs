// Module declarations
mod cli;
mod commands;
mod markdown_writer;
mod model;
mod parser;
mod state;
mod ui;

use cli::Commands;
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
        Commands::Add { description } => commands::add_task(description),
        Commands::Remove { id } => commands::remove_task(*id),
        Commands::Edit { id, description } => commands::edit_task(*id, description),
        Commands::Reset { id } => commands::reset_tasks(*id),
    }
}
