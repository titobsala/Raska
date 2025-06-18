use crate::model::Roadmap;
use crate::project;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

/// Save state using the new workspace-aware dual-level system
/// Priority: Local .rask/ > Global projects > Legacy fallback
pub fn save_state(roadmap: &Roadmap) -> Result<(), Error> {
    let state_file = get_workspace_aware_state_file()?;
    let json_data = serde_json::to_string_pretty(roadmap)
        .map_err(|e| Error::new(ErrorKind::Other, e))?;
    
    // Ensure the directory exists
    if let Some(parent) = Path::new(&state_file).parent() {
        fs::create_dir_all(parent)?;
    }
    
    fs::write(&state_file, json_data)
}

/// Load state using the new workspace-aware dual-level system
/// Priority: Local .rask/ > Global projects > Legacy fallback
pub fn load_state() -> Result<Roadmap, Error> {
    let state_file = get_workspace_aware_state_file()?;
    if !Path::new(&state_file).exists() {
        return Err(Error::new(ErrorKind::NotFound, 
            "State file not found. Please run 'rask init roadmap.md' in this directory first."));
    }
    let json_data = fs::read_to_string(&state_file)?;
    let roadmap: Roadmap = serde_json::from_str(&json_data)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    Ok(roadmap)
}

/// Get the state file using workspace-aware logic
/// This implements your dual-level project management vision:
/// 1. Local workspace (.rask/) - for codebase-specific project state
/// 2. Global projects - for cross-project management via web dashboard
/// 3. Legacy fallback - for backward compatibility
fn get_workspace_aware_state_file() -> Result<String, Error> {
    // Priority 1: Local .rask directory (workspace-specific)
    let local_rask_dir = Path::new(".rask");
    if local_rask_dir.exists() && local_rask_dir.is_dir() {
        let local_state_file = local_rask_dir.join("state.json");
        return Ok(local_state_file.to_string_lossy().to_string());
    }

    // Priority 2: Global project system (for web dashboard and cross-project management)
    if let Ok(global_state) = project::get_current_state_file() {
        // Check if this is a centralized global project
        if global_state.contains("/.local/share/rask/") || global_state.contains("project_") {
            return Ok(global_state);
        }
    }

    // Priority 3: Legacy fallback for backward compatibility
    if Path::new(".rask_state.json").exists() {
        return Ok(".rask_state.json".to_string());
    }

    // Default: Use local .rask directory (create if needed)
    let local_state_file = Path::new(".rask").join("state.json");
    Ok(local_state_file.to_string_lossy().to_string())
}

/// Transfer existing global project state to local workspace
/// This migrates from the global project system to local .rask/ directory
pub fn transfer_global_to_local() -> Result<bool, Error> {
    // Check if we already have local state
    let local_rask_dir = Path::new(".rask");
    let local_state_file = local_rask_dir.join("state.json");
    
    if local_state_file.exists() {
        return Ok(false); // Already has local state
    }

    // Try to load from global project system
    if let Ok(global_state_file) = project::get_current_state_file() {
        if Path::new(&global_state_file).exists() && 
           (global_state_file.contains("/.local/share/rask/") || global_state_file.contains("project_")) {
            
            // Load the global state
            let json_data = fs::read_to_string(&global_state_file)?;
            let roadmap: Roadmap = serde_json::from_str(&json_data)
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

            // Create local .rask directory
            fs::create_dir_all(local_rask_dir)?;
            
            // Save to local state
            let json_data = serde_json::to_string_pretty(&roadmap)
                .map_err(|e| Error::new(ErrorKind::Other, e))?;
            fs::write(&local_state_file, json_data)?;

            // Note: Enhanced project files can be created later with 'rask sync --to-files'

            return Ok(true); // Successfully transferred
        }
    }

    Ok(false) // No global state to transfer
}

/// Check if current directory has a local .rask workspace
pub fn has_local_workspace() -> bool {
    let local_rask_dir = Path::new(".rask");
    local_rask_dir.exists() && local_rask_dir.is_dir()
}

/// Get workspace information for the current directory
pub fn get_workspace_info() -> Result<WorkspaceInfo, Error> {
    let current_dir = std::env::current_dir()?;
    let has_local = has_local_workspace();
    
    let workspace_type = if has_local {
        WorkspaceType::Local
    } else if project::get_current_project()?.is_some() {
        WorkspaceType::Global 
    } else {
        WorkspaceType::None
    };

    Ok(WorkspaceInfo {
        current_directory: current_dir,
        workspace_type,
        has_local_rask: has_local,
        has_global_project: project::get_current_project()?.is_some(),
    })
}

#[derive(Debug, Clone)]
pub struct WorkspaceInfo {
    pub current_directory: PathBuf,
    pub workspace_type: WorkspaceType,
    pub has_local_rask: bool,
    pub has_global_project: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum WorkspaceType {
    Local,   // Has .rask/ directory
    Global,  // Uses global project system
    None,    // No project configured
}
