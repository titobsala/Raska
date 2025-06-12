use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use crate::config::{get_rask_data_dir, get_local_rask_dir};

/// File paths for project management
/// These are now centralized in the ~/.local/share/rask/ directory
fn get_projects_config_file() -> Result<PathBuf, Error> {
    let data_dir = get_rask_data_dir()?;
    Ok(data_dir.join("projects.json"))
}

fn get_current_project_file() -> Result<PathBuf, Error> {
    let data_dir = get_rask_data_dir()?;
    Ok(data_dir.join("current_project"))
}

/// Configuration for a single project
/// Contains metadata and file paths for project state management
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectConfig {
    /// Unique project name/identifier
    pub name: String,
    
    /// Optional human-readable description
    pub description: Option<String>,
    
    /// ISO 8601 timestamp of project creation
    pub created_at: String,
    
    /// ISO 8601 timestamp of last access (for recent project lists)
    pub last_accessed: String,
    
    /// Path to the project's JSON state file (in data directory)
    pub state_file: String,
    
    /// Path to the original markdown file (user's choice of location)
    pub source_file: Option<String>,
    
    /// Directory where this project was initialized (for context)
    pub work_directory: Option<String>,
}

/// Legacy project configuration structure for migration
/// This represents the old format without global_settings and work_directory
#[derive(Debug, Serialize, Deserialize)]
struct LegacyProjectsConfig {
    pub projects: HashMap<String, LegacyProjectConfig>,
    pub default_project: Option<String>,
}

/// Legacy project configuration for migration
#[derive(Debug, Serialize, Deserialize)]
struct LegacyProjectConfig {
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub last_accessed: String,
    pub state_file: String,
    pub source_file: Option<String>,
}

/// Global projects configuration
/// Manages all projects and default settings
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectsConfig {
    /// HashMap of project name -> project configuration
    pub projects: HashMap<String, ProjectConfig>,
    
    /// Default project to activate on startup
    pub default_project: Option<String>,
    
    /// Global settings that apply to all projects
    pub global_settings: GlobalProjectSettings,
}

/// Global settings that apply across all projects
#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalProjectSettings {
    /// Auto-switch to default project on startup
    pub auto_switch_default: bool,
    
    /// Keep track of recent projects (max count)
    pub recent_projects_count: usize,
    
    /// Auto-create local .rask directory when switching projects
    pub auto_create_local_dir: bool,
}

impl Default for GlobalProjectSettings {
    fn default() -> Self {
        GlobalProjectSettings {
            auto_switch_default: true,
            recent_projects_count: 10,
            auto_create_local_dir: true,
        }
    }
}

impl ProjectsConfig {
    /// Load the projects configuration from the centralized data directory
    /// Creates default configuration if none exists
    pub fn load() -> Result<Self, Error> {
        let config_file = get_projects_config_file()?;
        
        if !config_file.exists() {
            // Create default configuration and save it
            let default_config = ProjectsConfig::default();
            default_config.save()?;
            return Ok(default_config);
        }
        
        let json_data = fs::read_to_string(&config_file)?;
        
        // Try to parse as the new format first
        match serde_json::from_str::<ProjectsConfig>(&json_data) {
            Ok(config) => Ok(config),
            Err(_) => {
                // Try to parse as legacy format and migrate
                match serde_json::from_str::<LegacyProjectsConfig>(&json_data) {
                    Ok(legacy_config) => {
                        let migrated_config = legacy_config.migrate();
                        migrated_config.save()?;
                        Ok(migrated_config)
                    },
                    Err(e) => Err(Error::new(ErrorKind::InvalidData, format!("Failed to parse projects config: {}", e)))
                }
            }
        }
    }
    
    /// Save the projects configuration to the centralized data directory
    pub fn save(&self) -> Result<(), Error> {
        let config_file = get_projects_config_file()?;
        
        // Ensure the data directory exists
        if let Some(parent) = config_file.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let json_data = serde_json::to_string_pretty(self)
            .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to serialize projects config: {}", e)))?;
        
        fs::write(&config_file, json_data)
    }
    
    /// Add a new project to the configuration
    /// Creates the project state file in the centralized data directory
    pub fn add_project(&mut self, name: String, description: Option<String>) -> Result<(), Error> {
        if self.projects.contains_key(&name) {
            return Err(Error::new(ErrorKind::AlreadyExists, format!("Project '{}' already exists", name)));
        }
        
        let now = chrono::Utc::now().to_rfc3339();
        let data_dir = get_rask_data_dir()?;
        
        // State file is stored in the centralized data directory
        let state_file = data_dir.join(format!("project_{}.json", name))
            .to_string_lossy()
            .to_string();
        
        // Get current working directory for context
        let work_directory = std::env::current_dir()
            .ok()
            .and_then(|p| p.to_str().map(|s| s.to_string()));
        
        let project_config = ProjectConfig {
            name: name.clone(),
            description,
            created_at: now.clone(),
            last_accessed: now,
            state_file,
            source_file: None, // Will be set when initialized with a markdown file
            work_directory,
        };
        
        self.projects.insert(name.clone(), project_config);
        
        // Set as default if it's the first project
        if self.default_project.is_none() {
            self.default_project = Some(name);
        }
        
        self.save()
    }
    
    /// Remove a project and its associated files
    pub fn remove_project(&mut self, name: &str) -> Result<(), Error> {
        let project = self.projects.get(name)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, format!("Project '{}' not found", name)))?
            .clone();
        
        // Remove the project's state file
        let state_file_path = Path::new(&project.state_file);
        if state_file_path.exists() {
            fs::remove_file(state_file_path)?;
        }
        
        // Remove project from configuration
        self.projects.remove(name);
        
        // Update default project if needed
        if self.default_project.as_ref() == Some(&name.to_string()) {
            self.default_project = self.projects.keys().next().map(|k| k.clone());
        }
        
        self.save()
    }
    
    /// Get a project configuration by name
    pub fn get_project(&self, name: &str) -> Option<&ProjectConfig> {
        self.projects.get(name)
    }
    
    /// Get a mutable reference to a project configuration
    pub fn get_project_mut(&mut self, name: &str) -> Option<&mut ProjectConfig> {
        self.projects.get_mut(name)
    }
    
    /// Update the last accessed timestamp for a project
    /// Also manages the recent projects list
    pub fn update_last_accessed(&mut self, name: &str) -> Result<(), Error> {
        if let Some(project) = self.projects.get_mut(name) {
            project.last_accessed = chrono::Utc::now().to_rfc3339();
            self.save()?;
        }
        Ok(())
    }
    
    /// Get projects sorted by last accessed (most recent first)
    pub fn get_recent_projects(&self) -> Vec<(&String, &ProjectConfig)> {
        let mut projects: Vec<_> = self.projects.iter().collect();
        projects.sort_by(|a, b| b.1.last_accessed.cmp(&a.1.last_accessed));
        projects.truncate(self.global_settings.recent_projects_count);
        projects
    }
    
    /// Set the source file for a project (when initialized with markdown)
    pub fn set_project_source_file(&mut self, project_name: &str, source_file: &str) -> Result<(), Error> {
        if let Some(project) = self.projects.get_mut(project_name) {
            project.source_file = Some(source_file.to_string());
            self.save()?;
        }
        Ok(())
    }
}

impl LegacyProjectsConfig {
    /// Migrate legacy configuration to new format
    fn migrate(self) -> ProjectsConfig {
        let mut new_projects = HashMap::new();
        
        for (name, legacy_project) in self.projects {
            // Update state file path to new centralized location
            let new_state_file = if let Ok(data_dir) = get_rask_data_dir() {
                data_dir.join(format!("project_{}.json", name))
                    .to_string_lossy()
                    .to_string()
            } else {
                legacy_project.state_file // Fallback to original if data dir fails
            };
            
            let new_project = ProjectConfig {
                name: legacy_project.name,
                description: legacy_project.description,
                created_at: legacy_project.created_at,
                last_accessed: legacy_project.last_accessed,
                state_file: new_state_file,
                source_file: legacy_project.source_file,
                work_directory: None, // Legacy projects don't have this field
            };
            new_projects.insert(name, new_project);
        }
        
        ProjectsConfig {
            projects: new_projects,
            default_project: self.default_project,
            global_settings: GlobalProjectSettings::default(),
        }
    }
}

/// Get the currently active project name
/// Reads from the centralized current project file
pub fn get_current_project() -> Result<Option<String>, Error> {
    let current_file = get_current_project_file()?;
    
    if !current_file.exists() {
        return Ok(None);
    }
    
    let content = fs::read_to_string(&current_file)?;
    let project_name = content.trim().to_string();
    
    // Validate that the project still exists
    let projects_config = ProjectsConfig::load()?;
    if projects_config.projects.contains_key(&project_name) {
        Ok(Some(project_name))
    } else {
        // Clean up invalid current project
        let _ = fs::remove_file(&current_file);
        Ok(None)
    }
}

/// Set the currently active project
/// Updates the centralized current project file
pub fn set_current_project(project_name: &str) -> Result<(), Error> {
    let current_file = get_current_project_file()?;
    
    // Ensure the data directory exists
    if let Some(parent) = current_file.parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Validate that the project exists
    let projects_config = ProjectsConfig::load()?;
    if !projects_config.projects.contains_key(project_name) {
        return Err(Error::new(ErrorKind::NotFound, format!("Project '{}' does not exist", project_name)));
    }
    
    fs::write(&current_file, project_name)
}

/// Get the state file path for the currently active project
/// Falls back to legacy behavior if no project system is set up
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
    
    // Fall back to legacy behavior for backward compatibility
    // Check if old state file exists in current directory
    if Path::new(".rask_state.json").exists() {
        return Ok(".rask_state.json".to_string());
    }
    
    // Default to centralized location for new installations
    let data_dir = get_rask_data_dir()?;
    Ok(data_dir.join("default_project.json").to_string_lossy().to_string())
}

/// Get information about the currently active project
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

/// Initialize the local .rask directory for project-specific configurations
/// This creates a local .rask folder in the current directory for project overrides
pub fn init_local_rask_directory() -> Result<(), Error> {
    let local_dir = get_local_rask_dir()?;
    
    // Create a README to explain the directory structure
    let readme_content = r#"# Rask Project Directory

This directory contains project-specific configuration and state for Rask.

## Structure

- `config.toml` - Project-specific configuration overrides
- `cache/` - Local cache files (ignored by git)
- `exports/` - Default location for exported files

## Notes

- The `config.toml` file overrides user-level configuration
- State files are stored centrally in ~/.local/share/rask/
- The original markdown roadmap can be anywhere you choose
"#;
    
    let readme_path = local_dir.join("README.md");
    if !readme_path.exists() {
        fs::write(&readme_path, readme_content)?;
    }
    
    // Create cache directory
    let cache_dir = local_dir.join("cache");
    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir)?;
    }
    
    // Create exports directory
    let exports_dir = local_dir.join("exports");
    if !exports_dir.exists() {
        fs::create_dir_all(&exports_dir)?;
    }
    
    Ok(())
}

/// Migrate legacy project files to the new directory structure
/// This helps users transition from the old flat file structure
pub fn migrate_legacy_files() -> Result<(), Error> {
    let data_dir = get_rask_data_dir()?;
    
    // Migrate .rask_projects.json to new location
    let old_projects_file = Path::new(".rask_projects.json");
    let new_projects_file = data_dir.join("projects.json");
    
    if old_projects_file.exists() && !new_projects_file.exists() {
        fs::copy(old_projects_file, &new_projects_file)?;
        fs::remove_file(old_projects_file)?;
        println!("✅ Migrated projects configuration to {}", new_projects_file.display());
    }
    
    // Migrate .rask_current_project to new location
    let old_current_file = Path::new(".rask_current_project");
    let new_current_file = data_dir.join("current_project");
    
    if old_current_file.exists() && !new_current_file.exists() {
        fs::copy(old_current_file, &new_current_file)?;
        fs::remove_file(old_current_file)?;
        println!("✅ Migrated current project tracking to {}", new_current_file.display());
    }
    
    // Migrate state files to new location
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_str().unwrap_or("");
        
        if file_name_str.starts_with(".rask_state_") && file_name_str.ends_with(".json") {
            let project_name = file_name_str
                .strip_prefix(".rask_state_")
                .unwrap()
                .strip_suffix(".json")
                .unwrap();
            
            let new_state_file = data_dir.join(format!("project_{}.json", project_name));
            
            if !new_state_file.exists() {
                fs::copy(entry.path(), &new_state_file)?;
                fs::remove_file(entry.path())?;
                println!("✅ Migrated project '{}' state to {}", project_name, new_state_file.display());
            }
        }
    }
    
    Ok(())
} 