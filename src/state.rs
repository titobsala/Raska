use crate::model::Roadmap;
use crate::project;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

pub fn save_state(roadmap: &Roadmap) -> Result<(), Error> {
    let state_file = project::get_current_state_file()?;
    let json_data = serde_json::to_string_pretty(roadmap)
        .map_err(|e| Error::new(ErrorKind::Other, e))?;
    fs::write(&state_file, json_data)
}

pub fn load_state() -> Result<Roadmap, Error> {
    let state_file = project::get_current_state_file()?;
    if !Path::new(&state_file).exists() {
        return Err(Error::new(ErrorKind::NotFound, "State file not found. Please run 'init' first or create a project."));
    }
    let json_data = fs::read_to_string(&state_file)?;
    let roadmap: Roadmap = serde_json::from_str(&json_data)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    Ok(roadmap)
}
