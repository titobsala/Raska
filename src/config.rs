use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

/// The main configuration structure for Rask
/// This struct holds all user-configurable settings and preferences
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RaskConfig {
    /// UI and display preferences
    pub ui: UiConfig,
    
    /// Behavior and workflow settings
    pub behavior: BehaviorConfig,
    
    /// Export and integration settings
    pub export: ExportConfig,
    
    /// Power user features
    pub advanced: AdvancedConfig,
    
    /// Theme configuration (for future expansion)
    pub theme: ThemeConfig,
}

/// UI and display configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UiConfig {
    /// Color scheme: "light", "dark", or custom theme name
    pub color_scheme: Option<String>,
    
    /// Show completed tasks by default in list command
    pub show_completed: bool,
    
    /// Default sort order: "priority", "date", "id", "description"
    pub default_sort: String,
    
    /// Use compact view by default (less spacing and details)
    pub compact_view: bool,
    
    /// Show task IDs in output
    pub show_task_ids: bool,
    
    /// Maximum terminal width to use (0 = auto-detect)
    pub max_width: usize,
}

/// Behavior and workflow configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BehaviorConfig {
    /// Default project to switch to on startup
    pub default_project: Option<String>,
    
    /// Default priority for new tasks
    pub default_priority: String,
    
    /// Default tags to add to new tasks (comma-separated)
    pub default_tags: Vec<String>,
    
    /// Auto-archive completed tasks after X days (0 = never)
    pub auto_archive_days: u32,
    
    /// Show warnings about circular dependencies
    pub warn_on_circular: bool,
    
    /// Ask for confirmation on destructive actions
    pub confirm_destructive: bool,
    
    /// Automatically sync to markdown file after changes
    pub auto_sync_markdown: bool,
}

/// Export and integration configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExportConfig {
    /// Default export format: "json", "csv", "html", "markdown"
    pub default_format: String,
    
    /// Default directory for exports (relative to project or absolute)
    pub default_path: Option<String>,
    
    /// Include completed tasks in exports
    pub include_completed: bool,
    
    /// Include metadata in exports
    pub include_metadata: bool,
}

/// Advanced power user configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdvancedConfig {
    /// Custom command aliases (e.g., "c" -> "complete", "ls" -> "list")
    pub aliases: HashMap<String, String>,
    
    /// External editor command for editing notes/descriptions
    pub editor: Option<String>,
    
    /// Custom task templates (future feature)
    pub templates: HashMap<String, String>,
    
    /// Enable debug output
    pub debug: bool,
}

/// Theme configuration for future expansion
/// This allows for complete UI customization
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThemeConfig {
    /// Theme name
    pub name: String,
    
    /// Colors for different priority levels
    pub priority_colors: HashMap<String, String>,
    
    /// Colors for different task statuses
    pub status_colors: HashMap<String, String>,
    
    /// Icons/symbols to use for different elements
    pub symbols: SymbolConfig,
}

/// Symbol configuration for UI elements
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SymbolConfig {
    /// Symbol for pending tasks
    pub pending: String,
    
    /// Symbol for completed tasks
    pub completed: String,
    
    /// Symbol for blocked tasks
    pub blocked: String,
    
    /// Symbol for current project
    pub current_project: String,
    
    /// Symbol for dependencies
    pub dependency: String,
}

/// Default configuration values
impl Default for RaskConfig {
    fn default() -> Self {
        RaskConfig {
            ui: UiConfig::default(),
            behavior: BehaviorConfig::default(),
            export: ExportConfig::default(),
            advanced: AdvancedConfig::default(),
            theme: ThemeConfig::default(),
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        UiConfig {
            color_scheme: Some("auto".to_string()),
            show_completed: true,
            default_sort: "priority".to_string(),
            compact_view: false,
            show_task_ids: true,
            max_width: 0, // Auto-detect
        }
    }
}

impl Default for BehaviorConfig {
    fn default() -> Self {
        BehaviorConfig {
            default_project: None,
            default_priority: "medium".to_string(),
            default_tags: Vec::new(),
            auto_archive_days: 0, // Never auto-archive
            warn_on_circular: true,
            confirm_destructive: true,
            auto_sync_markdown: true,
        }
    }
}

impl Default for ExportConfig {
    fn default() -> Self {
        ExportConfig {
            default_format: "json".to_string(),
            default_path: None,
            include_completed: true,
            include_metadata: true,
        }
    }
}

impl Default for AdvancedConfig {
    fn default() -> Self {
        let mut aliases = HashMap::new();
        // Some sensible default aliases
        aliases.insert("ls".to_string(), "list".to_string());
        aliases.insert("c".to_string(), "complete".to_string());
        aliases.insert("a".to_string(), "add".to_string());
        aliases.insert("rm".to_string(), "remove".to_string());
        aliases.insert("s".to_string(), "show".to_string());
        
        AdvancedConfig {
            aliases,
            editor: std::env::var("EDITOR").ok(),
            templates: HashMap::new(),
            debug: false,
        }
    }
}

impl Default for ThemeConfig {
    fn default() -> Self {
        let mut priority_colors = HashMap::new();
        priority_colors.insert("critical".to_string(), "red".to_string());
        priority_colors.insert("high".to_string(), "yellow".to_string());
        priority_colors.insert("medium".to_string(), "blue".to_string());
        priority_colors.insert("low".to_string(), "green".to_string());
        
        let mut status_colors = HashMap::new();
        status_colors.insert("pending".to_string(), "white".to_string());
        status_colors.insert("completed".to_string(), "green".to_string());
        status_colors.insert("blocked".to_string(), "red".to_string());
        
        ThemeConfig {
            name: "default".to_string(),
            priority_colors,
            status_colors,
            symbols: SymbolConfig::default(),
        }
    }
}

impl Default for SymbolConfig {
    fn default() -> Self {
        SymbolConfig {
            pending: "⏳".to_string(),
            completed: "✅".to_string(),
            blocked: "🚫".to_string(),
            current_project: "👉".to_string(),
            dependency: "🔗".to_string(),
        }
    }
}

/// Get the path to the Rask configuration directory
/// On Linux: ~/.config/rask/
/// Creates the directory if it doesn't exist
pub fn get_rask_config_dir() -> Result<PathBuf, Error> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Could not determine config directory"))?
        .join("rask");
    
    // Create the directory if it doesn't exist
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }
    
    Ok(config_dir)
}

/// Get the path to the Rask data directory for state files
/// On Linux: ~/.local/share/rask/
/// Creates the directory if it doesn't exist
pub fn get_rask_data_dir() -> Result<PathBuf, Error> {
    let data_dir = dirs::data_dir()
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Could not determine data directory"))?
        .join("rask");
    
    // Create the directory if it doesn't exist
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)?;
    }
    
    Ok(data_dir)
}

/// Get the path to the local project .rask directory
/// Creates the directory if it doesn't exist
pub fn get_local_rask_dir() -> Result<PathBuf, Error> {
    let local_dir = PathBuf::from(".rask");
    
    // Create the directory if it doesn't exist
    if !local_dir.exists() {
        fs::create_dir_all(&local_dir)?;
    }
    
    Ok(local_dir)
}

impl RaskConfig {
    /// Load configuration with the following priority:
    /// 1. Local project config (.rask/config.toml)
    /// 2. User config (~/.config/rask/config.toml)
    /// 3. Default configuration
    pub fn load() -> Result<Self, Error> {
        let mut config = RaskConfig::default();
        
        // Try to load user config first (as base)
        if let Ok(user_config) = Self::load_user_config() {
            config = user_config;
        }
        
        // Then overlay with project config if it exists
        if let Ok(project_config) = Self::load_project_config() {
            config = Self::merge_configs(config, project_config);
        }
        
        Ok(config)
    }
    
    /// Load user configuration from ~/.config/rask/config.toml
    pub fn load_user_config() -> Result<Self, Error> {
        let config_dir = get_rask_config_dir()?;
        let config_path = config_dir.join("config.toml");
        
        if !config_path.exists() {
            return Err(Error::new(ErrorKind::NotFound, "User config not found"));
        }
        
        let config_str = fs::read_to_string(&config_path)?;
        let config: RaskConfig = toml::from_str(&config_str)
            .map_err(|e| Error::new(ErrorKind::InvalidData, format!("Failed to parse config: {}", e)))?;
        
        Ok(config)
    }
    
    /// Load project configuration from .rask/config.toml
    fn load_project_config() -> Result<Self, Error> {
        let project_config_path = PathBuf::from(".rask/config.toml");
        
        if !project_config_path.exists() {
            return Err(Error::new(ErrorKind::NotFound, "Project config not found"));
        }
        
        let config_str = fs::read_to_string(&project_config_path)?;
        let config: RaskConfig = toml::from_str(&config_str)
            .map_err(|e| Error::new(ErrorKind::InvalidData, format!("Failed to parse config: {}", e)))?;
        
        Ok(config)
    }
    
    /// Merge two configurations, with the second taking precedence
    fn merge_configs(base: RaskConfig, overlay: RaskConfig) -> RaskConfig {
        // For now, we'll do a simple overlay where overlay completely replaces sections
        // In the future, we could implement more sophisticated merging
        overlay
    }
    
    /// Save configuration to user config file
    pub fn save_user_config(&self) -> Result<(), Error> {
        let config_dir = get_rask_config_dir()?;
        let config_path = config_dir.join("config.toml");
        
        let config_str = toml::to_string_pretty(self)
            .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to serialize config: {}", e)))?;
        
        fs::write(&config_path, config_str)?;
        Ok(())
    }
    
    /// Save configuration to project config file
    pub fn save_project_config(&self) -> Result<(), Error> {
        let local_dir = get_local_rask_dir()?;
        let config_path = local_dir.join("config.toml");
        
        let config_str = toml::to_string_pretty(self)
            .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to serialize config: {}", e)))?;
        
        fs::write(&config_path, config_str)?;
        Ok(())
    }
    
    /// Initialize a new user configuration file with defaults
    pub fn init_user_config() -> Result<(), Error> {
        let config = RaskConfig::default();
        config.save_user_config()?;
        Ok(())
    }
    
    /// Initialize a new project configuration file with defaults
    pub fn init_project_config() -> Result<(), Error> {
        let config = RaskConfig::default();
        config.save_project_config()?;
        Ok(())
    }
    
    /// Get a configuration value by key (dot notation support)
    /// Example: "ui.color_scheme", "behavior.default_priority"
    pub fn get(&self, key: &str) -> Option<String> {
        let parts: Vec<&str> = key.split('.').collect();
        if parts.len() != 2 {
            return None;
        }
        
        match (parts[0], parts[1]) {
            ("ui", "color_scheme") => self.ui.color_scheme.clone(),
            ("ui", "show_completed") => Some(self.ui.show_completed.to_string()),
            ("ui", "default_sort") => Some(self.ui.default_sort.clone()),
            ("ui", "compact_view") => Some(self.ui.compact_view.to_string()),
            ("behavior", "default_project") => self.behavior.default_project.clone(),
            ("behavior", "default_priority") => Some(self.behavior.default_priority.clone()),
            ("behavior", "warn_on_circular") => Some(self.behavior.warn_on_circular.to_string()),
            ("behavior", "confirm_destructive") => Some(self.behavior.confirm_destructive.to_string()),
            ("export", "default_format") => Some(self.export.default_format.clone()),
            ("export", "default_path") => self.export.default_path.clone(),
            ("advanced", "editor") => self.advanced.editor.clone(),
            ("advanced", "debug") => Some(self.advanced.debug.to_string()),
            ("theme", "name") => Some(self.theme.name.clone()),
            _ => None,
        }
    }
    
    /// Set a configuration value by key
    pub fn set(&mut self, key: &str, value: &str) -> Result<(), Error> {
        let parts: Vec<&str> = key.split('.').collect();
        if parts.len() != 2 {
            return Err(Error::new(ErrorKind::InvalidInput, "Key must be in format 'section.key'"));
        }
        
        match (parts[0], parts[1]) {
            ("ui", "color_scheme") => self.ui.color_scheme = Some(value.to_string()),
            ("ui", "show_completed") => self.ui.show_completed = value.parse().map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid boolean value"))?,
            ("ui", "default_sort") => self.ui.default_sort = value.to_string(),
            ("ui", "compact_view") => self.ui.compact_view = value.parse().map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid boolean value"))?,
            ("behavior", "default_project") => self.behavior.default_project = if value.is_empty() { None } else { Some(value.to_string()) },
            ("behavior", "default_priority") => self.behavior.default_priority = value.to_string(),
            ("behavior", "warn_on_circular") => self.behavior.warn_on_circular = value.parse().map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid boolean value"))?,
            ("behavior", "confirm_destructive") => self.behavior.confirm_destructive = value.parse().map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid boolean value"))?,
            ("export", "default_format") => self.export.default_format = value.to_string(),
            ("export", "default_path") => self.export.default_path = if value.is_empty() { None } else { Some(value.to_string()) },
            ("advanced", "editor") => self.advanced.editor = if value.is_empty() { None } else { Some(value.to_string()) },
            ("advanced", "debug") => self.advanced.debug = value.parse().map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid boolean value"))?,
            ("theme", "name") => self.theme.name = value.to_string(),
            _ => return Err(Error::new(ErrorKind::InvalidInput, "Unknown configuration key")),
        }
        
        Ok(())
    }
} 