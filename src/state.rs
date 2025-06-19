use crate::model::Roadmap;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

/// Save state to local .rask/state.json only
pub fn save_state(roadmap: &Roadmap) -> Result<(), Error> {
    let state_file = get_local_state_file()?;
    let json_data = serde_json::to_string_pretty(roadmap)
        .map_err(|e| Error::new(ErrorKind::Other, e))?;
    
    // Ensure the .rask directory exists
    if let Some(parent) = Path::new(&state_file).parent() {
        fs::create_dir_all(parent)?;
    }
    
    fs::write(&state_file, json_data)
}

/// Load state from local .rask/state.json only
pub fn load_state() -> Result<Roadmap, Error> {
    let state_file = get_local_state_file()?;
    if !Path::new(&state_file).exists() {
        return Err(Error::new(ErrorKind::NotFound, 
            "No .rask directory found. Please run 'rask init <roadmap.md>' in this directory first."));
    }
    let json_data = fs::read_to_string(&state_file)?;
    let roadmap: Roadmap = serde_json::from_str(&json_data)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    Ok(roadmap)
}

/// Get the local .rask/state.json file path
/// This is the only state file location in the simplified local-only approach
fn get_local_state_file() -> Result<String, Error> {
    let local_rask_dir = Path::new(".rask");
    if !local_rask_dir.exists() {
        return Err(Error::new(ErrorKind::NotFound, 
            "No .rask directory found in current directory. Run 'rask init <roadmap.md>' first."));
    }
    
    let local_state_file = local_rask_dir.join("state.json");
    Ok(local_state_file.to_string_lossy().to_string())
}

/// Check if current directory has a local .rask workspace
pub fn has_local_workspace() -> bool {
    let local_rask_dir = Path::new(".rask");
    local_rask_dir.exists() && local_rask_dir.is_dir()
}
