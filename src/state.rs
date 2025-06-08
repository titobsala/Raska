use crate::model::Roadmap;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

const STATE_FILE: &str = ".rask_state.json";

pub fn save_state(roadmap: &Roadmap) -> Result<(), Error> {
    let json_data = serde_json::to_string_pretty(roadmap)
        .map_err(|e| Error::new(ErrorKind::Other, e))?;
    fs::write(STATE_FILE, json_data)
}

pub fn load_state() -> Result<Roadmap, Error> {
    if !Path::new(STATE_FILE).exists() {
        return Err(Error::new(ErrorKind::NotFound, "State file not found. Please run 'init' first."));
    }
    let json_data = fs::read_to_string(STATE_FILE)?;
    let roadmap: Roadmap = serde_json::from_str(&json_data)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    Ok(roadmap)
}
