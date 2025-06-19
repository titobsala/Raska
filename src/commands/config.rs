//! Configuration management commands
//! 
//! This module handles all configuration-related operations including
//! showing, setting, getting, editing, initializing, and resetting configuration.

use crate::{config::RaskConfig, ui};
use super::{CommandResult, ConfigCommands};
use std::path::PathBuf;
use std::process::Command;

/// Handle configuration-related commands
pub fn handle_config_command(config_command: &ConfigCommands) -> CommandResult {
    match config_command {
        ConfigCommands::Show { section } => show_config(section.as_deref()),
        ConfigCommands::Set { key, value, project } => set_config(key, value, *project),
        ConfigCommands::Get { key } => get_config(key),
        ConfigCommands::Edit { project } => edit_config(*project),
        ConfigCommands::Init { project, user } => init_config(*project, *user),
        ConfigCommands::Reset { project, user, force } => reset_config(*project, *user, *force),
    }
}

/// Show current configuration or a specific section
fn show_config(section: Option<&str>) -> CommandResult {
    let config = RaskConfig::load()?;
    
    match section {
        Some("ui") => {
            ui::display_info("🎨 UI Configuration:");
            println!("  Color scheme: {:?}", config.ui.color_scheme);
            println!("  Show completed: {}", config.ui.show_completed);
            println!("  Default sort: {}", config.ui.default_sort);
            println!("  Compact view: {}", config.ui.compact_view);
            println!("  Show task IDs: {}", config.ui.show_task_ids);
            println!("  Max width: {} (0 = auto)", config.ui.max_width);
        },
        Some("behavior") => {
            ui::display_info("⚙️  Behavior Configuration:");
            println!("  Default project: {:?}", config.behavior.default_project);
            println!("  Default priority: {}", config.behavior.default_priority);
            println!("  Default tags: {:?}", config.behavior.default_tags);
            println!("  Auto archive days: {} (0 = never)", config.behavior.auto_archive_days);
            println!("  Warn on circular: {}", config.behavior.warn_on_circular);
            println!("  Confirm destructive: {}", config.behavior.confirm_destructive);
            println!("  Auto sync markdown: {}", config.behavior.auto_sync_markdown);
        },
        Some("export") => {
            ui::display_info("📤 Export Configuration:");
            println!("  Default format: {}", config.export.default_format);
            println!("  Default path: {:?}", config.export.default_path);
            println!("  Include completed: {}", config.export.include_completed);
            println!("  Include metadata: {}", config.export.include_metadata);
        },
        Some("advanced") => {
            ui::display_info("🔧 Advanced Configuration:");
            println!("  Aliases: {:?}", config.advanced.aliases);
            println!("  Editor: {:?}", config.advanced.editor);
            println!("  Templates: {:?}", config.advanced.templates);
            println!("  Debug: {}", config.advanced.debug);
        },
        Some("theme") => {
            ui::display_info("🎭 Theme Configuration:");
            println!("  Name: {}", config.theme.name);
            println!("  Priority colors: {:?}", config.theme.priority_colors);
            println!("  Status colors: {:?}", config.theme.status_colors);
            println!("  Symbols: {:?}", config.theme.symbols);
        },
        Some(unknown) => {
            return Err(format!("Unknown configuration section: {}. Available sections: ui, behavior, export, advanced, theme", unknown).into());
        },
        None => {
            // Show all configuration
            ui::display_info("📋 Complete Rask Configuration:");
            show_config(Some("ui"))?;
            println!();
            show_config(Some("behavior"))?;
            println!();
            show_config(Some("export"))?;
            println!();
            show_config(Some("advanced"))?;
            println!();
            show_config(Some("theme"))?;
            
            // Show config file locations
            println!();
            ui::display_info("📁 Configuration Files:");
            if let Ok(user_config_dir) = crate::config::get_rask_config_dir() {
                println!("  User config: {}", user_config_dir.join("config.toml").display());
            }
            println!("  Project config: .rask/config.toml");
        }
    }
    
    Ok(())
}

/// Set a configuration value
fn set_config(key: &str, value: &str, project_config: bool) -> CommandResult {
    let mut config = RaskConfig::load()?;
    
    // Set the configuration value
    config.set(key, value)?;
    
    // Save to the appropriate config file
    if project_config {
        config.save_project_config()?;
        ui::display_success(&format!("Set {} = {} in project configuration", key, value));
    } else {
        config.save_user_config()?;
        ui::display_success(&format!("Set {} = {} in user configuration", key, value));
    }
    
    Ok(())
}

/// Get a configuration value
fn get_config(key: &str) -> CommandResult {
    let config = RaskConfig::load()?;
    
    if let Some(value) = config.get(key) {
        println!("{}", value);
    } else {
        return Err(format!("Configuration key '{}' not found", key).into());
    }
    
    Ok(())
}

/// Edit configuration in the user's preferred editor
fn edit_config(project_config: bool) -> CommandResult {
    let config = RaskConfig::load()?;
    
    // Determine the editor to use
    let editor_env = std::env::var("EDITOR").ok();
    let editor = config.advanced.editor
        .as_ref()
        .or_else(|| editor_env.as_ref())
        .ok_or("No editor configured. Set EDITOR environment variable or use 'rask config set advanced.editor <editor>'")?;
    
    // Determine the config file path
    let config_path = if project_config {
        // Ensure local .rask directory exists
        std::fs::create_dir_all(".rask")?;
        PathBuf::from(".rask/config.toml")
    } else {
        let config_dir = crate::config::get_rask_config_dir()?;
        config_dir.join("config.toml")
    };
    
    // Create the config file if it doesn't exist
    if !config_path.exists() {
        if project_config {
            RaskConfig::init_project_config()?;
        } else {
            RaskConfig::init_user_config()?;
        }
    }
    
    // Launch the editor
    let status = Command::new(editor)
        .arg(&config_path)
        .status()?;
    
    if status.success() {
        ui::display_success(&format!("Configuration file {} edited successfully", config_path.display()));
    } else {
        return Err("Editor exited with error".into());
    }
    
    Ok(())
}

/// Initialize configuration files
fn init_config(project_config: bool, user_config: bool) -> CommandResult {
    if !project_config && !user_config {
        return Err("Specify --project or --user to initialize configuration".into());
    }
    
    if project_config {
        std::fs::create_dir_all(".rask")?;
        RaskConfig::init_project_config()?;
        ui::display_success("Initialized project configuration at .rask/config.toml");
    }
    
    if user_config {
        RaskConfig::init_user_config()?;
        let config_dir = crate::config::get_rask_config_dir()?;
        ui::display_success(&format!("Initialized user configuration at {}", config_dir.join("config.toml").display()));
    }
    
    Ok(())
}

/// Reset configuration to defaults
fn reset_config(project_config: bool, user_config: bool, force: bool) -> CommandResult {
    if !project_config && !user_config {
        return Err("Specify --project or --user to reset configuration".into());
    }
    
    if !force {
        ui::display_warning("This will reset configuration to defaults and cannot be undone.");
        ui::display_info("Use --force to confirm the reset operation");
        return Ok(());
    }
    
    if project_config {
        RaskConfig::init_project_config()?;
        ui::display_success("Reset project configuration to defaults");
    }
    
    if user_config {
        RaskConfig::init_user_config()?;
        ui::display_success("Reset user configuration to defaults");
    }
    
    Ok(())
} 