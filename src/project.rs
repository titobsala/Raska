use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

const PROJECTS_CONFIG_FILE: &str = ".rask_projects.json";
const CURRENT_PROJECT_FILE: &str = ".rask_current_project";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectConfig {
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub last_accessed: String,
    pub state_file: String, // Path to the project's state file
    pub source_file: Option<String>, // Original markdown file if any
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectsConfig {
    pub projects: HashMap<String, ProjectConfig>,
    pub default_project: Option<String>,
}

impl ProjectsConfig {
    pub fn load() -> Result<Self, Error> {
        if !Path::new(PROJECTS_CONFIG_FILE).exists() {
            return Ok(ProjectsConfig::default());
        }
        
        let json_data = fs::read_to_string(PROJECTS_CONFIG_FILE)?;
        let config: ProjectsConfig = serde_json::from_str(&json_data)
            .map_err(|e| Error::new(ErrorKind::InvalidData, format!("Failed to parse projects config: {}", e)))?;
        Ok(config)
    }
    
    pub fn save(&self) -> Result<(), Error> {
        let json_data = serde_json::to_string_pretty(self)
            .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to serialize projects config: {}", e)))?;
        fs::write(PROJECTS_CONFIG_FILE, json_data)
    }
    
    pub fn add_project(&mut self, name: String, description: Option<String>) -> Result<(), Error> {
        if self.projects.contains_key(&name) {
            return Err(Error::new(ErrorKind::AlreadyExists, format!("Project '{}' already exists", name)));
        }
        
        let now = chrono::Utc::now().to_rfc3339();
        let state_file = format!(".rask_state_{}.json", name);
        
        let project_config = ProjectConfig {
            name: name.clone(),
            description,
            created_at: now.clone(),
            last_accessed: now,
            state_file,
            source_file: None,
        };
        
        self.projects.insert(name.clone(), project_config);
        
        // Set as default if it's the first project
        if self.default_project.is_none() {
            self.default_project = Some(name);
        }
        
        self.save()
    }
    
    pub fn remove_project(&mut self, name: &str) -> Result<(), Error> {
        if !self.projects.contains_key(name) {
            return Err(Error::new(ErrorKind::NotFound, format!("Project '{}' not found", name)));
        }
        
        // Remove the project's state file
        if let Some(project) = self.projects.get(name) {
            let state_file_path = Path::new(&project.state_file);
            if state_file_path.exists() {
                fs::remove_file(state_file_path)?;
            }
        }
        
        self.projects.remove(name);
        
        // Update default project if needed
        if self.default_project.as_ref() == Some(&name.to_string()) {
            self.default_project = self.projects.keys().next().map(|k| k.clone());
        }
        
        self.save()
    }
    
    pub fn get_project(&self, name: &str) -> Option<&ProjectConfig> {
        self.projects.get(name)
    }
    
    pub fn update_last_accessed(&mut self, name: &str) -> Result<(), Error> {
        if let Some(project) = self.projects.get_mut(name) {
            project.last_accessed = chrono::Utc::now().to_rfc3339();
            self.save()?;
        }
        Ok(())
    }
}

pub fn get_current_project() -> Result<Option<String>, Error> {
    if !Path::new(CURRENT_PROJECT_FILE).exists() {
        return Ok(None);
    }
    
    let content = fs::read_to_string(CURRENT_PROJECT_FILE)?;
    Ok(Some(content.trim().to_string()))
}

pub fn set_current_project(project_name: &str) -> Result<(), Error> {
    fs::write(CURRENT_PROJECT_FILE, project_name)
}

pub fn get_current_state_file() -> Result<String, Error> {
    // Check if we have a current project set
    if let Some(current_project) = get_current_project()? {
        let projects_config = ProjectsConfig::load()?;
        if let Some(project) = projects_config.get_project(&current_project) {
            return Ok(project.state_file.clone());
        }
    }
    
    // Check if we have a projects config with a default
    let projects_config = ProjectsConfig::load()?;
    if let Some(default_project) = &projects_config.default_project {
        if let Some(project) = projects_config.get_project(default_project) {
            return Ok(project.state_file.clone());
        }
    }
    
    // Fall back to the original state file for backward compatibility
    Ok(".rask_state.json".to_string())
}

pub fn get_current_project_info() -> Result<Option<ProjectConfig>, Error> {
    if let Some(current_project) = get_current_project()? {
        let projects_config = ProjectsConfig::load()?;
        Ok(projects_config.get_project(&current_project).cloned())
    } else {
        let projects_config = ProjectsConfig::load()?;
        if let Some(default_project) = &projects_config.default_project {
            Ok(projects_config.get_project(default_project).cloned())
        } else {
            Ok(None)
        }
    }
} 