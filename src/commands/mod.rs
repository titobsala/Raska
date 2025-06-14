//! Command module for Rask
//! 
//! This module provides a clean, modular interface for all Rask commands.
//! Each command category is organized into its own submodule for better maintainability.

pub mod core;
pub mod project;
pub mod bulk;
pub mod export;
pub mod config;
pub mod dependencies;
pub mod phases;
pub mod notes;
pub mod utils;

// Re-export all public command functions
pub use core::*;
pub use project::*;
pub use bulk::*;
pub use export::*;
pub use config::*;
pub use dependencies::*;
pub use phases::*;
pub use notes::*;

// Common types used across all command modules
pub type CommandResult = Result<(), Box<dyn std::error::Error>>;

// Re-export CLI types for convenience
pub use crate::cli::{CliPriority, ConfigCommands, BulkCommands, PhaseCommands, NotesCommands, ExportFormat}; 