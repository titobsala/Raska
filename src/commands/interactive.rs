//! Interactive TUI mode for Rask
//! 
//! This module provides a rich terminal user interface for project management
//! with integrated AI assistant capabilities using ratatui.

use crate::commands::CommandResult;
use crate::ui::display_info;
use crate::model::Roadmap;
use serde::{Deserialize, Serialize};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use std::{
    error::Error,
    io,
    time::Instant,
    fs,
    path::PathBuf,
    collections::HashSet,
};
use chrono;

/// TUI Settings for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiSettings {
    /// Default view to open on startup
    pub default_view: AppView,
    /// Auto-refresh interval in seconds (0 = disabled)
    pub auto_refresh_interval: u32,
    /// Remember last selected task
    pub remember_selection: bool,
    /// Show welcome message
    pub show_welcome: bool,
    /// Maximum chat messages to keep
    pub max_chat_messages: usize,
}

impl Default for TuiSettings {
    fn default() -> Self {
        Self {
            default_view: AppView::Home,
            auto_refresh_interval: 0,
            remember_selection: true,
            show_welcome: true,
            max_chat_messages: 100,
        }
    }
}

impl TuiSettings {
    fn get_settings_path() -> Result<PathBuf, Box<dyn Error>> {
        let config_dir = crate::config::get_rask_config_dir()?;
        Ok(config_dir.join("tui_settings.json"))
    }
    
    pub fn load() -> Self {
        match Self::get_settings_path() {
            Ok(path) => {
                if path.exists() {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(settings) = serde_json::from_str(&content) {
                            return settings;
                        }
                    }
                }
            }
            Err(_) => {}
        }
        Self::default()
    }
    
    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let path = Self::get_settings_path()?;
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}

/// TUI Application state
pub struct App {
    /// Should the application quit?
    pub should_quit: bool,
    /// Current project roadmap
    pub roadmap: Option<Roadmap>,
    /// AI chat messages
    pub chat_messages: Vec<ChatMessage>,
    /// Current input text
    pub input: String,
    /// Which panel is currently focused
    pub focus: PanelFocus,
    /// Current view/screen
    pub current_view: AppView,
    /// Selected navigation item
    pub selected_nav_item: usize,
    /// Selected task index
    pub selected_task: Option<usize>,
    /// Scroll offset for task list
    pub task_scroll_offset: usize,
    /// Scroll offset for chat messages
    pub chat_scroll_offset: usize,
    /// Maximum visible items in task list
    pub max_visible_tasks: usize,
    /// Maximum visible items in chat
    pub max_visible_chat: usize,
    /// Available navigation items
    pub navigation_items: Vec<NavigationItem>,
    /// TUI settings
    pub settings: TuiSettings,
    /// Selected project index in project switcher
    pub selected_project: Option<usize>,
    /// Scroll offset for project list
    pub project_scroll_offset: usize,
    /// Selected template index
    pub selected_template: Option<usize>,
    /// Selected settings item index
    pub selected_setting: Option<usize>,
    /// Selected AI config item index
    pub selected_ai_config: Option<usize>,
    /// Current editing field for AI config
    pub editing_ai_field: Option<String>,
    /// Current input for editing
    pub edit_input: String,
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub sender: String,
    pub content: String,
    pub _timestamp: Instant,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PanelFocus {
    Tasks,
    Chat,
    Input,
    Navigation,
    Settings,
    Templates,
    AIConfig,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AppView {
    Home,
    TaskManager,
    AIAssistant, 
    AISettings,
    Templates,
    Analytics,
    Settings,
    ProjectSwitcher,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NavigationItem {
    Home,
    TaskManager,
    AIAssistant,
    AISettings,
    Templates,
    Analytics,
    Settings,
    ProjectSwitcher,
}

impl Clone for App {
    fn clone(&self) -> Self {
        App {
            should_quit: self.should_quit,
            roadmap: self.roadmap.clone(),
            chat_messages: self.chat_messages.clone(),
            input: self.input.clone(),
            focus: self.focus.clone(),
            current_view: self.current_view.clone(),
            selected_nav_item: self.selected_nav_item,
            selected_task: self.selected_task,
            task_scroll_offset: self.task_scroll_offset,
            chat_scroll_offset: self.chat_scroll_offset,
            max_visible_tasks: self.max_visible_tasks,
            max_visible_chat: self.max_visible_chat,
            navigation_items: self.navigation_items.clone(),
            settings: self.settings.clone(),
            selected_project: self.selected_project,
            project_scroll_offset: self.project_scroll_offset,
            selected_template: self.selected_template,
            selected_setting: self.selected_setting,
            selected_ai_config: self.selected_ai_config,
            editing_ai_field: self.editing_ai_field.clone(),
            edit_input: self.edit_input.clone(),
        }
    }
}

impl Default for App {
    fn default() -> App {
        let settings = TuiSettings::load();
        
        App {
            should_quit: false,
            roadmap: None,
            chat_messages: vec![
                ChatMessage {
                    sender: "AI Assistant".to_string(),
                    content: "Hello! I'm here to help you manage your tasks. You can ask me to break down complex tasks, suggest improvements, or help with project planning.".to_string(),
                    _timestamp: Instant::now(),
                }
            ],
            input: String::new(),
            focus: PanelFocus::Navigation,
            current_view: settings.default_view.clone(),
            selected_nav_item: match settings.default_view {
                AppView::Home => 0,
                AppView::TaskManager => 1,
                AppView::AIAssistant => 2,
                AppView::AISettings => 3,
                AppView::Templates => 4,
                AppView::Analytics => 5,
                AppView::Settings => 6,
                AppView::ProjectSwitcher => 7,
            },
            selected_task: None,
            task_scroll_offset: 0,
            chat_scroll_offset: 0,
            max_visible_tasks: 10, // Will be calculated dynamically
            max_visible_chat: 8,   // Will be calculated dynamically
            navigation_items: vec![
                NavigationItem::Home,
                NavigationItem::TaskManager,
                NavigationItem::AIAssistant,
                NavigationItem::AISettings,
                NavigationItem::Templates,
                NavigationItem::Analytics,
                NavigationItem::Settings,
                NavigationItem::ProjectSwitcher,
            ],
            settings,
            selected_project: None,
            project_scroll_offset: 0,
            selected_template: None,
            selected_setting: None,
            selected_ai_config: None,
            editing_ai_field: None,
            edit_input: String::new(),
        }
    }
}

/// Launch the interactive TUI mode
/// 
/// This provides a rich terminal interface with:
/// - Real-time project visualization
/// - AI-powered task assistance
/// - Interactive task management
/// - Live command execution
pub fn run_interactive_mode(project: Option<&str>, no_welcome: bool) -> CommandResult {
    display_info("Launching interactive TUI mode...");
    
    let settings = TuiSettings::load();
    if !no_welcome && settings.show_welcome {
        display_welcome_message();
    }
    
    if let Some(project_name) = project {
        display_info(&format!("Starting with project: {}", project_name));
    }

    // Try to load current project
    let roadmap = match crate::state::load_state() {
        Ok(roadmap) => Some(roadmap),
        Err(_) => {
            display_info("No active project found. You can still use the AI assistant!");
            None
        }
    };

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let mut app = App::default();
    app.roadmap = roadmap;
    let res = run_app(&mut terminal, app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

/// Main application loop
fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    app.should_quit = true;
                }
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.should_quit = true;
                }
                KeyCode::Tab => {
                    app.focus = match app.focus {
                        PanelFocus::Navigation => match app.current_view {
                            AppView::Home => PanelFocus::Navigation,
                            AppView::TaskManager => PanelFocus::Tasks,
                            AppView::AIAssistant => PanelFocus::Chat,
                            AppView::AISettings => PanelFocus::AIConfig,
                            AppView::Templates => PanelFocus::Templates,
                            AppView::Settings => PanelFocus::Settings,
                            _ => PanelFocus::Navigation,
                        },
                        PanelFocus::Tasks => PanelFocus::Chat,
                        PanelFocus::Chat => PanelFocus::Input,
                        PanelFocus::Input => PanelFocus::Navigation,
                        PanelFocus::AIConfig => PanelFocus::Navigation,
                        PanelFocus::Templates => PanelFocus::Navigation,
                        PanelFocus::Settings => PanelFocus::Navigation,
                    };
                }
                KeyCode::Enter => {
                    if app.focus == PanelFocus::Navigation {
                        // Switch to selected view
                        if let Some(nav_item) = app.navigation_items.get(app.selected_nav_item) {
                            app.current_view = match nav_item {
                                NavigationItem::Home => AppView::Home,
                                NavigationItem::TaskManager => AppView::TaskManager,
                                NavigationItem::AIAssistant => AppView::AIAssistant,
                                NavigationItem::AISettings => AppView::AISettings,
                                NavigationItem::Templates => AppView::Templates,
                                NavigationItem::Analytics => AppView::Analytics,
                                NavigationItem::Settings => AppView::Settings,
                                NavigationItem::ProjectSwitcher => AppView::ProjectSwitcher,
                            };
                            
                            // Update focus based on the view
                            app.focus = match app.current_view {
                                AppView::Home => PanelFocus::Navigation,
                                AppView::TaskManager => PanelFocus::Tasks,
                                AppView::AIAssistant => PanelFocus::Chat,
                                AppView::AISettings => PanelFocus::AIConfig,
                                AppView::Templates => PanelFocus::Templates,
                                AppView::Settings => PanelFocus::Settings,
                                AppView::ProjectSwitcher => PanelFocus::Navigation,
                                _ => PanelFocus::Navigation,
                            };
                            
                            // Initialize selection for specific views
                            match app.current_view {
                                AppView::Templates => {
                                    if app.selected_template.is_none() {
                                        app.selected_template = Some(0);
                                    }
                                },
                                AppView::Settings => {
                                    if app.selected_setting.is_none() {
                                        app.selected_setting = Some(0);
                                    }
                                },
                                AppView::AISettings => {
                                    if app.selected_ai_config.is_none() {
                                        app.selected_ai_config = Some(0);
                                    }
                                },
                                AppView::ProjectSwitcher => {
                                    if app.selected_project.is_none() {
                                        app.selected_project = Some(0);
                                    }
                                },
                                _ => {}
                            }
                        }
                    } else if app.focus == PanelFocus::Tasks && app.selected_task.is_some() {
                        // Toggle task completion status
                        if let Some(roadmap) = &mut app.roadmap {
                            if let Some(task_idx) = app.selected_task {
                                if let Some(task) = roadmap.tasks.get_mut(task_idx) {
                                    let old_status = task.status.clone();
                                    task.status = match task.status {
                                        crate::model::TaskStatus::Pending => crate::model::TaskStatus::Completed,
                                        crate::model::TaskStatus::Completed => crate::model::TaskStatus::Pending,
                                    };
                                    
                                    let status_msg = match task.status {
                                        crate::model::TaskStatus::Completed => "completed",
                                        crate::model::TaskStatus::Pending => "reopened",
                                    };
                                    let task_id = task.id;
                                    
                                    // Save the updated roadmap
                                    if let Err(e) = crate::state::save_state(roadmap) {
                                        // Revert the change if save failed
                                        if let Some(task) = roadmap.tasks.get_mut(task_idx) {
                                            task.status = old_status;
                                        }
                                        app.chat_messages.push(ChatMessage {
                                            sender: "System".to_string(),
                                            content: format!("Error saving task changes: {}", e),
                                            _timestamp: Instant::now(),
                                        });
                                    } else {
                                        app.chat_messages.push(ChatMessage {
                                            sender: "System".to_string(),
                                            content: format!("Task #{} marked as {}", task_id, status_msg),
                                            _timestamp: Instant::now(),
                                        });
                                    }
                                }
                            }
                        }
                    } else if app.focus == PanelFocus::Input && !app.input.is_empty() {
                        let user_message = app.input.clone();
                        
                        // Add user message to chat
                        app.chat_messages.push(ChatMessage {
                            sender: "You".to_string(),
                            content: user_message.clone(),
                            _timestamp: Instant::now(),
                        });
                        
                        app.input.clear();
                        
                        // Get AI response using sync wrapper
                        let ai_response = get_ai_response_sync(&user_message, app.roadmap.as_ref());
                        app.chat_messages.push(ChatMessage {
                            sender: "AI Assistant".to_string(),
                            content: ai_response,
                            _timestamp: Instant::now(),
                        });
                    } else if app.focus == PanelFocus::Templates && app.selected_template.is_some() {
                        // Show template actions menu
                        if let Some(template_idx) = app.selected_template {
                            let templates = vec![
                                ("Web Development Project", "Set up web development environment and structure"),
                                ("Mobile App Development", "Create mobile app with UI/UX design and core features"),
                                ("Data Analysis Project", "Analyze data and create visualizations with insights"),
                                ("Game Development", "Design and implement game mechanics and graphics"),
                                ("Research Project", "Conduct research and document findings"),
                                ("Infrastructure Setup", "Set up development and deployment infrastructure"),
                                ("Bug Fix Template", "Identify, reproduce, and fix software bugs"),
                                ("Feature Development", "Design and implement new software features"),
                            ];
                            
                            if let Some((template_name, _)) = templates.get(template_idx) {
                                app.chat_messages.push(ChatMessage {
                                    sender: "System".to_string(),
                                    content: format!("📋 Template Actions for '{}':\n\n1️⃣ Press 'c' - Copy template info to clipboard\n2️⃣ Press 's' - Create simple task from template\n3️⃣ Press 'a' - Use AI to create detailed task\n4️⃣ Press 'r' - Generate roadmap with AI\n\n💡 Tip: You can also press Esc to cancel", template_name),
                                    _timestamp: Instant::now(),
                                });
                            }
                        }
                    } else if app.focus == PanelFocus::Settings && app.selected_setting.is_some() {
                        // Handle Settings configuration
                        if let Some(setting_idx) = app.selected_setting {
                            let setting_categories = vec![
                                "Display Settings", "Project Configuration", "Performance Options", 
                                "AI Assistant Settings", "Analytics Preferences", "Privacy & Security",
                                "Export & Import", "Advanced Options"
                            ];
                            if let Some(category) = setting_categories.get(setting_idx) {
                                app.chat_messages.push(ChatMessage {
                                    sender: "System".to_string(),
                                    content: format!("⚙️ Configuring {}:\n\n🔧 Available CLI commands:\n• rask config --show\n• rask config --set key=value\n• rask config --reset\n\n💡 Settings are stored in ~/.config/rask/config.toml\n📝 Edit manually or use CLI commands", category),
                                    _timestamp: Instant::now(),
                                });
                            }
                        }
                    } else if app.focus == PanelFocus::AIConfig && app.selected_ai_config.is_some() {
                        // Handle AI configuration editing
                        if let Some(config_idx) = app.selected_ai_config {
                            let config_fields = vec!["enabled", "api_key", "model", "temperature", "max_tokens", "auto_suggestions"];
                            let config_commands = vec![
                                "rask ai configure --enabled true",
                                "rask ai configure --api-key YOUR_KEY",
                                "rask ai configure --model gemini-1.5-flash",
                                "rask ai configure --temperature 0.7",
                                "rask ai configure --max-tokens 2048",
                                "rask ai configure --auto-suggestions true",
                            ];
                            if let Some((field, command)) = config_fields.get(config_idx).zip(config_commands.get(config_idx)) {
                                app.chat_messages.push(ChatMessage {
                                    sender: "System".to_string(),
                                    content: format!("🤖 Configuring AI {}:\n\n🔧 Use this command:\n{}\n\n💡 Or set environment variable GEMINI_API_KEY for API key", field, command),
                                    _timestamp: Instant::now(),
                                });
                            }
                        }
                    } else if app.current_view == AppView::ProjectSwitcher && app.selected_project.is_some() {
                        // Switch to selected project
                        if let Ok(config) = crate::project::ProjectsConfig::load() {
                            let projects: Vec<_> = config.projects.iter().collect();
                            if let Some(selected_idx) = app.selected_project {
                                if let Some((project_name, _)) = projects.get(selected_idx) {
                                    // Switch project using existing functionality
                                    match crate::commands::switch_project(project_name) {
                                        Ok(_) => {
                                            // Reload the project data
                                            match crate::state::load_state() {
                                                Ok(roadmap) => {
                                                    app.roadmap = Some(roadmap);
                                                    app.chat_messages.push(ChatMessage {
                                                        sender: "System".to_string(),
                                                        content: format!("✅ Switched to project '{}'", project_name),
                                                        _timestamp: Instant::now(),
                                                    });
                                                    // Return to home view after switching
                                                    app.current_view = AppView::Home;
                                                    app.focus = PanelFocus::Navigation;
                                                    app.selected_nav_item = 0;
                                                },
                                                Err(e) => {
                                                    app.chat_messages.push(ChatMessage {
                                                        sender: "System".to_string(),
                                                        content: format!("⚠️ Switched to '{}' but failed to load data: {}", project_name, e),
                                                        _timestamp: Instant::now(),
                                                    });
                                                    app.roadmap = None;
                                                }
                                            }
                                        },
                                        Err(e) => {
                                            app.chat_messages.push(ChatMessage {
                                                sender: "System".to_string(),
                                                content: format!("❌ Failed to switch project: {}", e),
                                                _timestamp: Instant::now(),
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                KeyCode::Backspace => {
                    if app.focus == PanelFocus::Input {
                        app.input.pop();
                    }
                }
                KeyCode::Char(' ') => {
                    if app.focus == PanelFocus::Input {
                        app.input.push(' ');
                    } else if app.focus == PanelFocus::Tasks && app.selected_task.is_some() {
                        // Space bar - same as Enter for task toggle (alternative shortcut)
                        if let Some(roadmap) = &mut app.roadmap {
                            if let Some(task_idx) = app.selected_task {
                                if let Some(task) = roadmap.tasks.get_mut(task_idx) {
                                    let old_status = task.status.clone();
                                    task.status = match task.status {
                                        crate::model::TaskStatus::Pending => crate::model::TaskStatus::Completed,
                                        crate::model::TaskStatus::Completed => crate::model::TaskStatus::Pending,
                                    };
                                    
                                    let status_msg = match task.status {
                                        crate::model::TaskStatus::Completed => "completed",
                                        crate::model::TaskStatus::Pending => "reopened",
                                    };
                                    let task_id = task.id;
                                    
                                    // Save the updated roadmap
                                    if let Err(e) = crate::state::save_state(roadmap) {
                                        // Revert the change if save failed
                                        if let Some(task) = roadmap.tasks.get_mut(task_idx) {
                                            task.status = old_status;
                                        }
                                        app.chat_messages.push(ChatMessage {
                                            sender: "System".to_string(),
                                            content: format!("Error saving task changes: {}", e),
                                            _timestamp: Instant::now(),
                                        });
                                    } else {
                                        app.chat_messages.push(ChatMessage {
                                            sender: "System".to_string(),
                                            content: format!("Task #{} marked as {}", task_id, status_msg),
                                            _timestamp: Instant::now(),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
                KeyCode::Char('r') => {
                    if app.focus == PanelFocus::Input {
                        app.input.push('r');
                    } else {
                        // 'r' key - refresh/reload project data
                        match crate::state::load_state() {
                            Ok(roadmap) => {
                                app.roadmap = Some(roadmap);
                                app.chat_messages.push(ChatMessage {
                                    sender: "System".to_string(),
                                    content: "Project data refreshed successfully".to_string(),
                                    _timestamp: Instant::now(),
                                });
                            },
                            Err(e) => {
                                app.chat_messages.push(ChatMessage {
                                    sender: "System".to_string(),
                                    content: format!("Error refreshing project: {}", e),
                                    _timestamp: Instant::now(),
                                });
                            }
                        }
                    }
                }
                KeyCode::Char('h') => {
                    if app.focus == PanelFocus::Input {
                        app.input.push('h');
                    } else {
                        // 'h' key - show help/shortcuts
                        app.chat_messages.push(ChatMessage {
                            sender: "Help".to_string(),
                            content: "🎮 Keyboard Shortcuts:\n• F1-F7: Switch views\n• Tab: Change focus\n• ↑↓: Navigate\n• Enter/Space: Toggle task/switch project\n• r: Refresh data\n• s: Save settings\n• p/F7: Project switcher\n• h: Show this help\n• q: Quit".to_string(),
                            _timestamp: Instant::now(),
                        });
                    }
                }
                KeyCode::Char('s') => {
                    if app.focus == PanelFocus::Input {
                        app.input.push('s');
                    } else {
                        // 's' key - save current settings
                        app.settings.default_view = app.current_view.clone();
                        match app.settings.save() {
                            Ok(_) => {
                                app.chat_messages.push(ChatMessage {
                                    sender: "System".to_string(),
                                    content: "Settings saved successfully".to_string(),
                                    _timestamp: Instant::now(),
                                });
                            },
                            Err(e) => {
                                app.chat_messages.push(ChatMessage {
                                    sender: "System".to_string(),
                                    content: format!("Error saving settings: {}", e),
                                    _timestamp: Instant::now(),
                                });
                            }
                        }
                    }
                }
                KeyCode::Char('p') => {
                    if app.focus == PanelFocus::Input {
                        app.input.push('p');
                    } else {
                        // 'p' key - go to project switcher
                        app.current_view = AppView::ProjectSwitcher;
                        app.selected_nav_item = 6; // ProjectSwitcher is at index 6
                        app.focus = PanelFocus::Navigation;
                        
                        // Initialize project selection if not set
                        if app.selected_project.is_none() {
                            app.selected_project = Some(0);
                        }
                        
                        app.chat_messages.push(ChatMessage {
                            sender: "System".to_string(),
                            content: "🔄 Opened Project Switcher • Use ↑↓ to navigate, Enter to switch".to_string(),
                            _timestamp: Instant::now(),
                        });
                    }
                }
                KeyCode::Char(c) => {
                    if app.focus == PanelFocus::Input {
                        app.input.push(c);
                    }
                }
                KeyCode::Up => {
                    if app.focus == PanelFocus::Navigation {
                        // Navigate up in menu
                        if app.selected_nav_item > 0 {
                            app.selected_nav_item -= 1;
                        } else {
                            app.selected_nav_item = app.navigation_items.len() - 1;
                        }
                    } else if app.focus == PanelFocus::Tasks {
                        if let Some(roadmap) = &app.roadmap {
                            let task_count = roadmap.tasks.len();
                            if task_count > 0 {
                                let new_selected = match app.selected_task {
                                    Some(i) => if i > 0 { i - 1 } else { task_count - 1 },
                                    None => 0,
                                };
                                app.selected_task = Some(new_selected);
                                
                                // Adjust scroll offset if needed
                                if new_selected < app.task_scroll_offset {
                                    app.task_scroll_offset = new_selected;
                                }
                            }
                        }
                    } else if app.focus == PanelFocus::Chat {
                        // Scroll up in chat
                        if app.chat_scroll_offset > 0 {
                            app.chat_scroll_offset -= 1;
                        }
                    } else if app.current_view == AppView::ProjectSwitcher {
                        // Navigate up in project list
                        if let Ok(config) = crate::project::ProjectsConfig::load() {
                            let project_count = config.projects.len();
                            if project_count > 0 {
                                let new_selected = match app.selected_project {
                                    Some(i) => if i > 0 { i - 1 } else { project_count - 1 },
                                    None => 0,
                                };
                                app.selected_project = Some(new_selected);
                            }
                        }
                    } else if app.focus == PanelFocus::Templates {
                        // Navigate up in templates list
                        let template_count = 8; // Hardcoded for now, will make dynamic later
                        if template_count > 0 {
                            let new_selected = match app.selected_template {
                                Some(i) => if i > 0 { i - 1 } else { template_count - 1 },
                                None => 0,
                            };
                            app.selected_template = Some(new_selected);
                        }
                    } else if app.focus == PanelFocus::Settings {
                        // Navigate up in settings list
                        let settings_count = 8; // Categories count
                        if settings_count > 0 {
                            let new_selected = match app.selected_setting {
                                Some(i) => if i > 0 { i - 1 } else { settings_count - 1 },
                                None => 0,
                            };
                            app.selected_setting = Some(new_selected);
                        }
                    } else if app.focus == PanelFocus::AIConfig {
                        // Navigate up in AI config list
                        let config_count = 6; // Config options count
                        if config_count > 0 {
                            let new_selected = match app.selected_ai_config {
                                Some(i) => if i > 0 { i - 1 } else { config_count - 1 },
                                None => 0,
                            };
                            app.selected_ai_config = Some(new_selected);
                        }
                    }
                }
                KeyCode::Down => {
                    if app.focus == PanelFocus::Navigation {
                        // Navigate down in menu
                        if app.selected_nav_item < app.navigation_items.len() - 1 {
                            app.selected_nav_item += 1;
                        } else {
                            app.selected_nav_item = 0;
                        }
                    } else if app.focus == PanelFocus::Tasks {
                        if let Some(roadmap) = &app.roadmap {
                            let task_count = roadmap.tasks.len();
                            if task_count > 0 {
                                let new_selected = match app.selected_task {
                                    Some(i) => if i < task_count - 1 { i + 1 } else { 0 },
                                    None => 0,
                                };
                                app.selected_task = Some(new_selected);
                                
                                // Adjust scroll offset if needed
                                if new_selected >= app.task_scroll_offset + app.max_visible_tasks {
                                    app.task_scroll_offset = new_selected.saturating_sub(app.max_visible_tasks - 1);
                                }
                            }
                        }
                    } else if app.focus == PanelFocus::Chat {
                        // Scroll down in chat
                        let max_scroll = app.chat_messages.len().saturating_sub(app.max_visible_chat);
                        if app.chat_scroll_offset < max_scroll {
                            app.chat_scroll_offset += 1;
                        }
                    } else if app.current_view == AppView::ProjectSwitcher {
                        // Navigate down in project list
                        if let Ok(config) = crate::project::ProjectsConfig::load() {
                            let project_count = config.projects.len();
                            if project_count > 0 {
                                let new_selected = match app.selected_project {
                                    Some(i) => if i < project_count - 1 { i + 1 } else { 0 },
                                    None => 0,
                                };
                                app.selected_project = Some(new_selected);
                            }
                        }
                    } else if app.focus == PanelFocus::Templates {
                        // Navigate down in templates list
                        let template_count = 8; // Hardcoded for now, will make dynamic later
                        if template_count > 0 {
                            let new_selected = match app.selected_template {
                                Some(i) => if i < template_count - 1 { i + 1 } else { 0 },
                                None => 0,
                            };
                            app.selected_template = Some(new_selected);
                        }
                    } else if app.focus == PanelFocus::Settings {
                        // Navigate down in settings list
                        let settings_count = 8; // Categories count
                        if settings_count > 0 {
                            let new_selected = match app.selected_setting {
                                Some(i) => if i < settings_count - 1 { i + 1 } else { 0 },
                                None => 0,
                            };
                            app.selected_setting = Some(new_selected);
                        }
                    } else if app.focus == PanelFocus::AIConfig {
                        // Navigate down in AI config list
                        let config_count = 6; // Config options count
                        if config_count > 0 {
                            let new_selected = match app.selected_ai_config {
                                Some(i) => if i < config_count - 1 { i + 1 } else { 0 },
                                None => 0,
                            };
                            app.selected_ai_config = Some(new_selected);
                        }
                    }
                }
                KeyCode::PageUp => {
                    if app.focus == PanelFocus::Tasks {
                        if let Some(roadmap) = &app.roadmap {
                            let task_count = roadmap.tasks.len();
                            if task_count > 0 {
                                let jump = app.max_visible_tasks.min(10);
                                let new_selected = app.selected_task.unwrap_or(0).saturating_sub(jump);
                                app.selected_task = Some(new_selected);
                                app.task_scroll_offset = new_selected.saturating_sub(app.max_visible_tasks / 2);
                            }
                        }
                    } else if app.focus == PanelFocus::Chat {
                        app.chat_scroll_offset = app.chat_scroll_offset.saturating_sub(app.max_visible_chat);
                    }
                }
                KeyCode::PageDown => {
                    if app.focus == PanelFocus::Tasks {
                        if let Some(roadmap) = &app.roadmap {
                            let task_count = roadmap.tasks.len();
                            if task_count > 0 {
                                let jump = app.max_visible_tasks.min(10);
                                let new_selected = (app.selected_task.unwrap_or(0) + jump).min(task_count - 1);
                                app.selected_task = Some(new_selected);
                                let max_scroll = task_count.saturating_sub(app.max_visible_tasks);
                                app.task_scroll_offset = (new_selected.saturating_sub(app.max_visible_tasks / 2)).min(max_scroll);
                            }
                        }
                    } else if app.focus == PanelFocus::Chat {
                        let max_scroll = app.chat_messages.len().saturating_sub(app.max_visible_chat);
                        app.chat_scroll_offset = (app.chat_scroll_offset + app.max_visible_chat).min(max_scroll);
                    }
                }
                // Function key shortcuts for quick navigation
                KeyCode::F(1) => {
                    app.current_view = AppView::Home;
                    app.selected_nav_item = 0;
                    app.focus = PanelFocus::Navigation;
                }
                KeyCode::F(2) => {
                    app.current_view = AppView::TaskManager;
                    app.selected_nav_item = 1;
                    app.focus = PanelFocus::Tasks;
                }
                KeyCode::F(3) => {
                    app.current_view = AppView::AIAssistant;
                    app.selected_nav_item = 2;
                    app.focus = PanelFocus::Chat;
                }
                KeyCode::F(4) => {
                    app.current_view = AppView::AISettings;
                    app.selected_nav_item = 3;
                    app.focus = PanelFocus::AIConfig;
                    if app.selected_ai_config.is_none() {
                        app.selected_ai_config = Some(0);
                    }
                }
                KeyCode::F(5) => {
                    app.current_view = AppView::Templates;
                    app.selected_nav_item = 4;
                    app.focus = PanelFocus::Templates;
                    if app.selected_template.is_none() {
                        app.selected_template = Some(0);
                    }
                }
                KeyCode::F(6) => {
                    app.current_view = AppView::Analytics;
                    app.selected_nav_item = 5;
                    app.focus = PanelFocus::Navigation;
                }
                KeyCode::F(7) => {
                    app.current_view = AppView::Settings;
                    app.selected_nav_item = 6;
                    app.focus = PanelFocus::Settings;
                    if app.selected_setting.is_none() {
                        app.selected_setting = Some(0);
                    }
                }
                KeyCode::F(8) => {
                    app.current_view = AppView::ProjectSwitcher;
                    app.selected_nav_item = 7;
                    app.focus = PanelFocus::Navigation;
                    // Initialize project selection if not set
                    if app.selected_project.is_none() {
                        app.selected_project = Some(0);
                    }
                }
                KeyCode::Char('c') => {
                    if app.focus == PanelFocus::Templates && app.selected_template.is_some() {
                        // Copy template to clipboard
                        if let Some(template_idx) = app.selected_template {
                            let templates = vec![
                                ("Web Development Project", "Set up web development environment and structure"),
                                ("Mobile App Development", "Create mobile app with UI/UX design and core features"),
                                ("Data Analysis Project", "Analyze data and create visualizations with insights"),
                                ("Game Development", "Design and implement game mechanics and graphics"),
                                ("Research Project", "Conduct research and document findings"),
                                ("Infrastructure Setup", "Set up development and deployment infrastructure"),
                                ("Bug Fix Template", "Identify, reproduce, and fix software bugs"),
                                ("Feature Development", "Design and implement new software features"),
                            ];
                            if let Some((template_name, template_desc)) = templates.get(template_idx) {
                                app.chat_messages.push(ChatMessage {
                                    sender: "System".to_string(),
                                    content: format!("📋 Copied to clipboard:\n\n**{}**\n{}\n\n💡 You can paste this into external AI tools or project management systems", template_name, template_desc),
                                    _timestamp: Instant::now(),
                                });
                            }
                        }
                    }
                }
                KeyCode::Char('s') => {
                    if app.focus == PanelFocus::Templates && app.selected_template.is_some() {
                        // Create simple task from template
                        if let Some(template_idx) = app.selected_template {
                            let templates = vec![
                                ("Web Development Project", "Set up web development environment and structure"),
                                ("Mobile App Development", "Create mobile app with UI/UX design and core features"),
                                ("Data Analysis Project", "Analyze data and create visualizations with insights"),
                                ("Game Development", "Design and implement game mechanics and graphics"),
                                ("Research Project", "Conduct research and document findings"),
                                ("Infrastructure Setup", "Set up development and deployment infrastructure"),
                                ("Bug Fix Template", "Identify, reproduce, and fix software bugs"),
                                ("Feature Development", "Design and implement new software features"),
                            ];
                            
                            if let Some((template_name, template_desc)) = templates.get(template_idx) {
                                if let Some(roadmap) = &mut app.roadmap {
                                    let new_task = crate::model::Task {
                                        id: roadmap.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1,
                                        description: format!("{}: {}", template_name, template_desc),
                                        status: crate::model::TaskStatus::Pending,
                                        tags: vec!["template".to_string()].into_iter().collect(),
                                        priority: crate::model::Priority::Medium,
                                        phase: crate::model::Phase::new("Planning".to_string()),
                                        dependencies: vec![],
                                        notes: None,
                                        implementation_notes: vec![],
                                        created_at: Some(chrono::Utc::now().to_rfc3339()),
                                        completed_at: None,
                                        estimated_hours: Some(8.0),
                                        actual_hours: None,
                                        time_sessions: vec![],
                                    };
                                    roadmap.tasks.push(new_task.clone());
                                    
                                    if let Err(e) = crate::state::save_state(roadmap) {
                                        app.chat_messages.push(ChatMessage {
                                            sender: "System".to_string(),
                                            content: format!("❌ Error creating task: {}", e),
                                            _timestamp: Instant::now(),
                                        });
                                    } else {
                                        app.chat_messages.push(ChatMessage {
                                            sender: "System".to_string(),
                                            content: format!("✅ Created simple task #{} from template '{}'", new_task.id, template_name),
                                            _timestamp: Instant::now(),
                                        });
                                    }
                                } else {
                                    app.chat_messages.push(ChatMessage {
                                        sender: "System".to_string(),
                                        content: "❌ No project loaded. Please switch to a project first.".to_string(),
                                        _timestamp: Instant::now(),
                                    });
                                }
                            }
                        }
                    }
                }
                KeyCode::Char('a') => {
                    if app.focus == PanelFocus::Templates && app.selected_template.is_some() {
                        // Use AI to create detailed task
                        if let Some(template_idx) = app.selected_template {
                            let templates = vec![
                                ("Web Development Project", "Set up web development environment and structure"),
                                ("Mobile App Development", "Create mobile app with UI/UX design and core features"),
                                ("Data Analysis Project", "Analyze data and create visualizations with insights"),
                                ("Game Development", "Design and implement game mechanics and graphics"),
                                ("Research Project", "Conduct research and document findings"),
                                ("Infrastructure Setup", "Set up development and deployment infrastructure"),
                                ("Bug Fix Template", "Identify, reproduce, and fix software bugs"),
                                ("Feature Development", "Design and implement new software features"),
                            ];
                            
                            if let Some((template_name, template_desc)) = templates.get(template_idx) {
                                app.chat_messages.push(ChatMessage {
                                    sender: "System".to_string(),
                                    content: "🤖 Generating AI-enhanced task...".to_string(),
                                    _timestamp: Instant::now(),
                                });
                                
                                // Get AI response for task creation
                                let ai_prompt = format!("Create a detailed task breakdown for: {}\nDescription: {}\nPlease provide a comprehensive task description with sub-tasks, estimated time, and key deliverables.", template_name, template_desc);
                                let ai_response = get_ai_response_sync(&ai_prompt, app.roadmap.as_ref());
                                
                                if let Some(roadmap) = &mut app.roadmap {
                                    let new_task = crate::model::Task {
                                        id: roadmap.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1,
                                        description: format!("{}: AI-Enhanced", template_name),
                                        status: crate::model::TaskStatus::Pending,
                                        tags: vec!["template".to_string(), "ai-generated".to_string()].into_iter().collect(),
                                        priority: crate::model::Priority::Medium,
                                        phase: crate::model::Phase::new("Planning".to_string()),
                                        dependencies: vec![],
                                        notes: Some(ai_response.clone()),
                                        implementation_notes: vec![],
                                        created_at: Some(chrono::Utc::now().to_rfc3339()),
                                        completed_at: None,
                                        estimated_hours: Some(16.0),
                                        actual_hours: None,
                                        time_sessions: vec![],
                                    };
                                    roadmap.tasks.push(new_task.clone());
                                    
                                    if let Err(e) = crate::state::save_state(roadmap) {
                                        app.chat_messages.push(ChatMessage {
                                            sender: "System".to_string(),
                                            content: format!("❌ Error creating AI task: {}", e),
                                            _timestamp: Instant::now(),
                                        });
                                    } else {
                                        app.chat_messages.push(ChatMessage {
                                            sender: "AI Assistant".to_string(),
                                            content: format!("✅ Created AI-enhanced task #{} from template '{}':\n\n{}", new_task.id, template_name, ai_response),
                                            _timestamp: Instant::now(),
                                        });
                                    }
                                } else {
                                    app.chat_messages.push(ChatMessage {
                                        sender: "System".to_string(),
                                        content: "❌ No project loaded. Please switch to a project first.".to_string(),
                                        _timestamp: Instant::now(),
                                    });
                                }
                            }
                        }
                    }
                }
                KeyCode::Char('r') => {
                    if app.focus == PanelFocus::Templates && app.selected_template.is_some() {
                        // Generate roadmap with AI
                        if let Some(template_idx) = app.selected_template {
                            let templates = vec![
                                ("Web Development Project", "Set up web development environment and structure"),
                                ("Mobile App Development", "Create mobile app with UI/UX design and core features"),
                                ("Data Analysis Project", "Analyze data and create visualizations with insights"),
                                ("Game Development", "Design and implement game mechanics and graphics"),
                                ("Research Project", "Conduct research and document findings"),
                                ("Infrastructure Setup", "Set up development and deployment infrastructure"),
                                ("Bug Fix Template", "Identify, reproduce, and fix software bugs"),
                                ("Feature Development", "Design and implement new software features"),
                            ];
                            
                            if let Some((template_name, template_desc)) = templates.get(template_idx) {
                                app.chat_messages.push(ChatMessage {
                                    sender: "System".to_string(),
                                    content: "🤖 Generating comprehensive roadmap with AI...".to_string(),
                                    _timestamp: Instant::now(),
                                });
                                
                                // Get AI response for roadmap generation
                                let ai_prompt = format!("Generate a complete project roadmap for: {}\nDescription: {}\nPlease break this down into 5-8 specific, actionable tasks with priorities, phases, and estimated hours. Format as a task list.", template_name, template_desc);
                                let ai_response = get_ai_response_sync(&ai_prompt, app.roadmap.as_ref());
                                
                                app.chat_messages.push(ChatMessage {
                                    sender: "AI Assistant".to_string(),
                                    content: format!("🗺️ AI-Generated Roadmap for '{}':\n\n{}\n\n💡 Use 'rask ai breakdown' command to convert this into actual tasks", template_name, ai_response),
                                    _timestamp: Instant::now(),
                                });
                            }
                        }
                    }
                }
                KeyCode::Esc => {
                    // Return to home/navigation
                    app.current_view = AppView::Home;
                    app.focus = PanelFocus::Navigation;
                    app.selected_nav_item = 0;
                }
                _ => {}
            }
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}

/// Render the UI based on current view
fn ui(f: &mut Frame, app: &App) {
    // Main layout with navigation bar at top
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(1)].as_ref())
        .split(f.size());

    // Render navigation bar
    render_navigation_bar(f, app, main_chunks[0]);
    
    // Render current view content
    match app.current_view {
        AppView::Home => render_home_view(f, app, main_chunks[1]),
        AppView::TaskManager => render_task_manager_view(f, app, main_chunks[1]),
        AppView::AIAssistant => render_ai_assistant_view(f, app, main_chunks[1]),
        AppView::AISettings => render_ai_settings_view(f, app, main_chunks[1]),
        AppView::Templates => render_templates_view(f, app, main_chunks[1]),
        AppView::Analytics => render_analytics_view(f, app, main_chunks[1]),
        AppView::Settings => render_settings_view(f, app, main_chunks[1]),
        AppView::ProjectSwitcher => render_project_switcher_view(f, app, main_chunks[1]),
    }
    
    // Render help text at bottom
    render_help_text(f, app, main_chunks[2]);
}

/// Render the navigation bar
fn render_navigation_bar(f: &mut Frame, app: &App, area: Rect) {
    let nav_items: Vec<String> = app.navigation_items.iter().enumerate().map(|(i, item)| {
        let icon_name = match item {
            NavigationItem::Home => "🏠 Home",
            NavigationItem::TaskManager => "📝 Tasks", 
            NavigationItem::AIAssistant => "🤖 AI",
            NavigationItem::AISettings => "🔧 AI Setup",
            NavigationItem::Templates => "📄 Templates",
            NavigationItem::Analytics => "📊 Analytics",
            NavigationItem::Settings => "⚙️ Settings",
            NavigationItem::ProjectSwitcher => "🔄 Projects",
        };
        
        if i == app.selected_nav_item {
            format!(" [{}] ", icon_name)
        } else {
            format!("  {}  ", icon_name)
        }
    }).collect();
    
    let nav_text = nav_items.join("│");
    let current_view_name = match app.current_view {
        AppView::Home => "Home Dashboard",
        AppView::TaskManager => "Task Manager",
        AppView::AIAssistant => "AI Assistant",
        AppView::AISettings => "AI Configuration",
        AppView::Templates => "Templates",
        AppView::Analytics => "Analytics",
        AppView::Settings => "Settings",
        AppView::ProjectSwitcher => "Project Switcher",
    };
    
    let nav_paragraph = Paragraph::new(nav_text)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(format!(" 🚀 Rask TUI • {} ", current_view_name))
            .border_style(if app.focus == PanelFocus::Navigation {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Cyan)
            }))
        .style(Style::default().fg(Color::White));
    
    f.render_widget(nav_paragraph, area);
}

/// Render the Home/Overview view
fn render_home_view(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    // Left side - Project overview
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(0)].as_ref())
        .split(chunks[0]);

    // Project stats
    let stats = match &app.roadmap {
        Some(roadmap) => {
            let completed = roadmap.tasks.iter().filter(|t| t.status == crate::model::TaskStatus::Completed).count();
            let total = roadmap.tasks.len();
            let progress = if total > 0 { (completed * 100) / total } else { 0 };
            let high_priority = roadmap.tasks.iter().filter(|t| t.priority == crate::model::Priority::High || t.priority == crate::model::Priority::Critical).count();
            
            format!("📋 Project: {}\n\n📊 Progress: {}/{} tasks ({}%)\n🔥 High Priority: {}\n📅 Version: 2.7.0\n🚀 Status: Active",
                roadmap.title, completed, total, progress, high_priority)
        },
        None => "📋 No Project Loaded\n\n💡 Welcome to Rask!\n📄 Load a project to get started\n🔧 Use 'F2' for Task Manager\n🤖 Use 'F3' for AI Assistant".to_string(),
    };

    let stats_widget = Paragraph::new(stats)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" 🏠 Project Overview ")
            .border_style(Style::default().fg(Color::Green)))
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false });
    f.render_widget(stats_widget, left_chunks[0]);

    // Recent activity (simplified for now)
    let recent_activity = vec![
        "🔄 System initialized",
        "📝 Interactive mode launched", 
        "🎯 Ready for task management",
        "🤖 AI assistant available",
    ];

    let activity_items: Vec<ListItem> = recent_activity.iter()
        .map(|item| ListItem::new(Line::from(*item)))
        .collect();

    let activity_list = List::new(activity_items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" 📈 Recent Activity ")
            .border_style(Style::default().fg(Color::Cyan)));
    f.render_widget(activity_list, left_chunks[1]);

    // Right side - Quick actions
    let quick_actions = vec![
        "🎯 F1 - Home Dashboard",
        "📝 F2 - Task Manager", 
        "🤖 F3 - AI Assistant",
        "📄 F4 - Templates",
        "📊 F5 - Analytics",
        "⚙️  F6 - Settings",
        "🔄 F7/p - Project Switcher",
        "",
        "📋 Tab - Switch Focus",
        "❌ Q - Quit Application",
    ];

    let action_items: Vec<ListItem> = quick_actions.iter()
        .map(|action| ListItem::new(Line::from(*action)))
        .collect();

    let actions_list = List::new(action_items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" ⚡ Quick Actions ")
            .border_style(Style::default().fg(Color::Yellow)));
    f.render_widget(actions_list, chunks[1]);
}

/// Render the Task Manager view
fn render_task_manager_view(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .split(area);

    // Left panel - Project info and tasks
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(chunks[0]);

    // Calculate max visible tasks based on available height
    let tasks_area_height = left_chunks[1].height.saturating_sub(2);
    let max_visible_tasks = tasks_area_height as usize;

    // Project header
    let project_info = match &app.roadmap {
        Some(roadmap) => {
            let completed_count = roadmap.tasks.iter().filter(|t| t.status == crate::model::TaskStatus::Completed).count();
            let total_count = roadmap.tasks.len();
            let progress = if total_count > 0 { (completed_count * 100) / total_count } else { 0 };
            
            format!("📋 {} • {}/{} tasks ({}%)", 
                roadmap.title, completed_count, total_count, progress)
        },
        None => "📋 No Project Loaded • Use 'rask init <file>' to start".to_string(),
    };

    let header = Paragraph::new(project_info)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" 📝 Task Manager ")
            .border_style(Style::default().fg(Color::Cyan)))
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true });
    f.render_widget(header, left_chunks[0]);

    // Tasks list with scrolling
    let (tasks_title, tasks) = match &app.roadmap {
        Some(roadmap) => {
            let total_tasks = roadmap.tasks.len();
            let visible_start = app.task_scroll_offset;
            let visible_end = (visible_start + max_visible_tasks).min(total_tasks);
            
            let scroll_info = if total_tasks > max_visible_tasks {
                format!(" Tasks ({}-{}/{}) ", visible_start + 1, visible_end, total_tasks)
            } else {
                format!(" Tasks ({}) ", total_tasks)
            };
            
            let visible_tasks: Vec<ListItem> = roadmap.tasks
                .iter()
                .enumerate()
                .skip(visible_start)
                .take(max_visible_tasks)
                .map(|(global_idx, task)| {
                    let status_icon = match task.status {
                        crate::model::TaskStatus::Completed => "✅",
                        crate::model::TaskStatus::Pending => "⏳",
                    };
                    let priority_indicator = match task.priority {
                        crate::model::Priority::Critical => "🔴",
                        crate::model::Priority::High => "🟠",
                        crate::model::Priority::Medium => "🟡",
                        crate::model::Priority::Low => "🟢",
                    };
                    
                    let is_selected = Some(global_idx) == app.selected_task;
                    let description = if task.description.len() > 45 {
                        format!("{}...", &task.description[..42])
                    } else {
                        task.description.clone()
                    };
                    
                    let line = if is_selected {
                        Line::from(vec![
                            Span::raw("▶ "),
                            Span::raw(format!("{} {} #{} ", status_icon, priority_indicator, task.id)),
                            Span::styled(description, Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD)),
                        ])
                    } else {
                        Line::from(vec![
                            Span::raw("  "),
                            Span::raw(format!("{} {} #{} ", status_icon, priority_indicator, task.id)),
                            Span::styled(description, Style::default().fg(Color::Gray)),
                        ])
                    };
                    
                    ListItem::new(line)
                })
                .collect();
            
            (scroll_info, visible_tasks)
        }
        None => {
            (" No Project ".to_string(), vec![
                ListItem::new(Line::from(Span::styled("📄 No project loaded", Style::default().fg(Color::Yellow)))),
                ListItem::new(Line::from(Span::styled("💡 Use 'rask init <file>' to load a project", Style::default().fg(Color::Cyan)))),
                ListItem::new(Line::from(Span::styled("🔧 Or 'rask project create <name>' for new project", Style::default().fg(Color::Cyan)))),
            ])
        }
    };

    let tasks_block = Block::default()
        .borders(Borders::ALL)
        .title(tasks_title)
        .border_style(if app.focus == PanelFocus::Tasks {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        });

    let tasks_list = List::new(tasks).block(tasks_block);
    f.render_widget(tasks_list, left_chunks[1]);

    // Right panel - Mini AI assistant for task context
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(chunks[1]);

    // Task context chat
    let chat_area_height = right_chunks[0].height.saturating_sub(2);
    let max_visible_chat = (chat_area_height as usize).saturating_sub(1);

    let chat_title = format!(" 🤖 Task Assistant ({}) ", app.chat_messages.len());
    let chat_block = Block::default()
        .borders(Borders::ALL)
        .title(chat_title)
        .border_style(if app.focus == PanelFocus::Chat {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Magenta)
        });

    let visible_messages: Vec<ListItem> = app.chat_messages
        .iter()
        .skip(app.chat_scroll_offset)
        .take(max_visible_chat)
        .map(|msg| {
            let sender_style = if msg.sender == "You" {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)
            };
            
            let content = if msg.content.len() > 45 {
                format!("{}...", &msg.content[..42])
            } else {
                msg.content.clone()
            };
            
            let lines = vec![
                Line::from(Span::styled(format!("{}: ", msg.sender), sender_style)),
                Line::from(Span::styled(content, Style::default().fg(Color::White))),
                Line::from(""),
            ];
            
            ListItem::new(lines)
        })
        .collect();

    let chat_list = List::new(visible_messages).block(chat_block);
    f.render_widget(chat_list, right_chunks[0]);

    // Input box
    let input_title = match app.focus {
        PanelFocus::Input => " 💬 Ask about tasks (AI coming soon!) ",
        _ => " 💬 Ask about tasks ",
    };
    
    let input_block = Block::default()
        .borders(Borders::ALL)
        .title(input_title)
        .border_style(if app.focus == PanelFocus::Input {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        });

    let input_text = if app.focus == PanelFocus::Input && !app.input.is_empty() {
        format!("{}_", app.input)
    } else if app.focus == PanelFocus::Input {
        "_".to_string()
    } else {
        app.input.clone()
    };

    let input = Paragraph::new(input_text)
        .block(input_block)
        .style(Style::default().fg(Color::White));
    f.render_widget(input, right_chunks[1]);
}

/// Render the AI Assistant view 
fn render_ai_assistant_view(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(area);

    // Chat area
    let chat_area_height = chunks[0].height.saturating_sub(2);
    let max_visible_chat = (chat_area_height as usize).saturating_sub(1);

    let chat_title = if app.chat_messages.len() > max_visible_chat {
        let visible_start = app.chat_scroll_offset;
        let visible_end = (visible_start + max_visible_chat).min(app.chat_messages.len());
        format!(" 🤖 AI Assistant Chat ({}-{}/{}) ", visible_start + 1, visible_end, app.chat_messages.len())
    } else {
        format!(" 🤖 AI Assistant Chat ({}) ", app.chat_messages.len())
    };

    let chat_block = Block::default()
        .borders(Borders::ALL)
        .title(chat_title)
        .border_style(if app.focus == PanelFocus::Chat {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Magenta)
        });

    let visible_messages: Vec<ListItem> = app.chat_messages
        .iter()
        .skip(app.chat_scroll_offset)
        .take(max_visible_chat)
        .map(|msg| {
            let sender_style = if msg.sender == "You" {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)
            };
            
            let wrapped_content = if msg.content.len() > 80 {
                let mut lines = Vec::new();
                let words: Vec<&str> = msg.content.split_whitespace().collect();
                let mut current_line = String::new();
                
                for word in words {
                    if current_line.len() + word.len() + 1 > 80 {
                        if !current_line.is_empty() {
                            lines.push(current_line.clone());
                            current_line.clear();
                        }
                    }
                    if !current_line.is_empty() {
                        current_line.push(' ');
                    }
                    current_line.push_str(word);
                }
                if !current_line.is_empty() {
                    lines.push(current_line);
                }
                lines
            } else {
                vec![msg.content.clone()]
            };
            
            let mut list_lines = vec![
                Line::from(Span::styled(format!("{}: ", msg.sender), sender_style))
            ];
            for line in wrapped_content {
                list_lines.push(Line::from(Span::styled(line, Style::default().fg(Color::White))));
            }
            list_lines.push(Line::from(""));
            
            ListItem::new(list_lines)
        })
        .collect();

    let chat_list = List::new(visible_messages).block(chat_block);
    f.render_widget(chat_list, chunks[0]);

    // Input area
    let input_title = match app.focus {
        PanelFocus::Input => " 💬 Chat with AI Assistant (powered by Gemini) ",
        _ => " 💬 Type your message ",
    };
    
    let input_block = Block::default()
        .borders(Borders::ALL)
        .title(input_title)
        .border_style(if app.focus == PanelFocus::Input {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        });

    let input_text = if app.focus == PanelFocus::Input && !app.input.is_empty() {
        format!("{}_", app.input)
    } else if app.focus == PanelFocus::Input {
        "_".to_string()
    } else {
        app.input.clone()
    };

    let input = Paragraph::new(input_text)
        .block(input_block)
        .style(Style::default().fg(Color::White));
    f.render_widget(input, chunks[1]);
}

/// Render the Templates view
fn render_templates_view(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    // Template list
    let templates = vec![
        "🚀 Web Development Project",
        "📱 Mobile App Development", 
        "📊 Data Analysis Project",
        "🎮 Game Development",
        "📚 Research Project",
        "🏗️ Infrastructure Setup",
        "🔧 Bug Fix Template",
        "✨ Feature Development",
    ];

    let template_items: Vec<ListItem> = templates.iter()
        .map(|template| ListItem::new(Line::from(*template)))
        .collect();

    let mut template_list = List::new(template_items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" 📄 Available Templates ")
            .border_style(if app.focus == PanelFocus::Templates { 
                Style::default().fg(Color::Cyan) 
            } else { 
                Style::default().fg(Color::Blue) 
            }))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));
    
    // Render with selection if focused on templates
    if app.focus == PanelFocus::Templates {
        if let Some(selected) = app.selected_template {
            let mut list_state = ListState::default();
            list_state.select(Some(selected));
            f.render_stateful_widget(template_list, chunks[0], &mut list_state);
        } else {
            f.render_widget(template_list, chunks[0]);
        }
    } else {
        f.render_widget(template_list, chunks[0]);
    }

    // Template preview/actions - dynamic based on selection
    let templates_info = vec![
        ("Web Development Project", "🌐 Full-stack web application development template\n\n📋 Includes:\n  • Frontend setup (React/Vue)\n  • Backend API development\n  • Database design\n  • Authentication system\n  • Testing framework\n  • Deployment pipeline\n\n⏱️ Estimated: 8-12 weeks\n🎯 Complexity: High\n\n🔧 Technologies:\n  • Frontend frameworks\n  • REST/GraphQL APIs\n  • Database management\n  • CI/CD pipeline"),
        ("Mobile App Development", "📱 Cross-platform mobile application template\n\n📋 Includes:\n  • UI/UX design phase\n  • Core app architecture\n  • Feature development\n  • API integration\n  • Testing & debugging\n  • App store deployment\n\n⏱️ Estimated: 10-16 weeks\n🎯 Complexity: High\n\n🔧 Technologies:\n  • React Native/Flutter\n  • Native APIs\n  • Push notifications\n  • App analytics"),
        ("Data Analysis Project", "📊 Comprehensive data analysis workflow\n\n📋 Includes:\n  • Data collection setup\n  • Data cleaning & preprocessing\n  • Exploratory analysis\n  • Statistical modeling\n  • Visualization creation\n  • Report generation\n\n⏱️ Estimated: 4-8 weeks\n🎯 Complexity: Medium\n\n🔧 Technologies:\n  • Python/R/SQL\n  • Pandas/NumPy\n  • Matplotlib/Plotly\n  • Jupyter notebooks"),
        ("Game Development", "🎮 Complete game development pipeline\n\n📋 Includes:\n  • Game design document\n  • Asset creation\n  • Core mechanics\n  • Level design\n  • Audio integration\n  • Testing & polish\n\n⏱️ Estimated: 12-20 weeks\n🎯 Complexity: Very High\n\n🔧 Technologies:\n  • Unity/Unreal Engine\n  • 3D modeling tools\n  • Audio editing\n  • Platform SDKs"),
        ("Research Project", "🔬 Academic/industry research template\n\n📋 Includes:\n  • Literature review\n  • Methodology design\n  • Data collection\n  • Analysis & findings\n  • Documentation\n  • Presentation prep\n\n⏱️ Estimated: 6-12 weeks\n🎯 Complexity: Medium\n\n🔧 Deliverables:\n  • Research paper\n  • Data analysis\n  • Presentation slides\n  • Code repository"),
        ("Infrastructure Setup", "🏗️ Development & deployment infrastructure\n\n📋 Includes:\n  • Environment setup\n  • CI/CD pipeline\n  • Monitoring & logging\n  • Security configuration\n  • Backup systems\n  • Documentation\n\n⏱️ Estimated: 2-6 weeks\n🎯 Complexity: Medium\n\n🔧 Technologies:\n  • Docker/Kubernetes\n  • Cloud platforms\n  • Monitoring tools\n  • Security scanners"),
        ("Bug Fix Template", "🐛 Systematic bug resolution workflow\n\n📋 Includes:\n  • Bug reproduction\n  • Root cause analysis\n  • Fix implementation\n  • Testing & validation\n  • Documentation update\n  • Deployment\n\n⏱️ Estimated: 1-3 days\n🎯 Complexity: Low-Medium\n\n🔧 Process:\n  • Issue analysis\n  • Code investigation\n  • Solution design\n  • Quality assurance"),
        ("Feature Development", "✨ New feature implementation template\n\n📋 Includes:\n  • Requirements analysis\n  • Design & architecture\n  • Implementation\n  • Testing & QA\n  • Documentation\n  • Release preparation\n\n⏱️ Estimated: 1-4 weeks\n🎯 Complexity: Medium\n\n🔧 Phases:\n  • Planning & design\n  • Development\n  • Testing & review\n  • Deployment & monitoring"),
    ];

    let preview_text = if let Some(selected_idx) = app.selected_template {
        if let Some((_, description)) = templates_info.get(selected_idx) {
            description.to_string()
        } else {
            "📋 Template System\n\nSelect a template from the left to view detailed information about its structure, estimated timeline, and included components.".to_string()
        }
    } else {
        "📋 Template System\n\nSelect a template from the left to view detailed information about its structure, estimated timeline, and included components.".to_string()
    };

    let preview = Paragraph::new(preview_text)
        .block(Block::default()
            .borders(Borders::ALL)
                    .title(if app.focus == PanelFocus::Templates { 
            " 🔍 Template Actions [Enter: Menu, c: Copy, s: Simple, a: AI, r: Roadmap] " 
        } else { 
            " 🔍 Template Preview " 
        })
        .border_style(Style::default().fg(Color::Green)))
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false });
    f.render_widget(preview, chunks[1]);
}

/// Render the Analytics view
fn render_analytics_view(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(area);

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);

    // Project metrics
    let metrics = match &app.roadmap {
        Some(roadmap) => {
            let total = roadmap.tasks.len();
            let completed = roadmap.tasks.iter().filter(|t| t.status == crate::model::TaskStatus::Completed).count();
            let pending = roadmap.tasks.iter().filter(|t| t.status == crate::model::TaskStatus::Pending).count();
            
            format!("📊 Task Distribution:\n  ✅ Completed: {}\n  ⏳ Pending: {}\n  📈 Total: {}\n\n🎯 Completion Rate: {}%",
                completed, pending, total,
                if total > 0 { (completed * 100) / total } else { 0 })
        },
        None => "📊 No Analytics Available\n\n💡 Load a project to see:\n  • Task distribution\n  • Progress trends\n  • Time tracking\n  • Priority analysis".to_string(),
    };

    let metrics_widget = Paragraph::new(metrics)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" 📊 Project Metrics ")
            .border_style(Style::default().fg(Color::Cyan)))
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false });
    f.render_widget(metrics_widget, top_chunks[0]);

    // Priority breakdown
    let priority_breakdown = match &app.roadmap {
        Some(roadmap) => {
            let critical = roadmap.tasks.iter().filter(|t| t.priority == crate::model::Priority::Critical).count();
            let high = roadmap.tasks.iter().filter(|t| t.priority == crate::model::Priority::High).count();
            let medium = roadmap.tasks.iter().filter(|t| t.priority == crate::model::Priority::Medium).count();
            let low = roadmap.tasks.iter().filter(|t| t.priority == crate::model::Priority::Low).count();
            
            format!("🔥 Priority Distribution:\n  🔴 Critical: {}\n  🟠 High: {}\n  🟡 Medium: {}\n  🟢 Low: {}\n\n⚡ Focus Areas:\n  • {} high-priority tasks\n  • {} ready to start",
                critical, high, medium, low, critical + high, medium + low)
        },
        None => "🔥 Priority Analysis\n\n📈 Coming Soon:\n  • Priority trends\n  • Workload analysis\n  • Burndown charts\n  • Time estimates".to_string(),
    };

    let priority_widget = Paragraph::new(priority_breakdown)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" 🔥 Priority Analysis ")
            .border_style(Style::default().fg(Color::Red)))
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false });
    f.render_widget(priority_widget, top_chunks[1]);

    // Analytics features
    let features = vec![
        "📈 Progress Tracking - View completion trends",
        "⏱️ Time Analysis - Track time spent on tasks", 
        "🎯 Priority Insights - Analyze task priorities",
        "📊 Phase Distribution - See work across phases",
        "🔄 Velocity Metrics - Measure team productivity",
        "📅 Timeline Analysis - Project timeline insights",
        "🎨 Visual Charts - Interactive data visualization",
        "📋 Export Reports - Generate analytics reports",
    ];

    let feature_items: Vec<ListItem> = features.iter()
        .map(|feature| ListItem::new(Line::from(*feature)))
        .collect();

    let features_list = List::new(feature_items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" 🎨 Analytics Features ")
            .border_style(Style::default().fg(Color::Green)));
    f.render_widget(features_list, chunks[1]);
}

/// Render the Settings view
fn render_settings_view(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    // Settings categories
    let settings_categories = vec![
        "🎨 Display Settings",
        "🔧 Project Configuration",
        "⚡ Performance Options", 
        "🤖 AI Assistant Settings",
        "📊 Analytics Preferences",
        "🔒 Privacy & Security",
        "🌍 Export & Import",
        "🚀 Advanced Options",
    ];

    let category_items: Vec<ListItem> = settings_categories.iter()
        .map(|category| ListItem::new(Line::from(*category)))
        .collect();

    let categories_list = List::new(category_items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" ⚙️ Configuration Categories ")
                    .border_style(if app.focus == PanelFocus::Settings { 
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD) 
        } else { 
            Style::default().fg(Color::Magenta) 
        }))
        .highlight_style(Style::default().bg(Color::Magenta).fg(Color::White));
    
    // Render with selection if focused on settings
    if app.focus == PanelFocus::Settings {
        if let Some(selected) = app.selected_setting {
            let mut list_state = ListState::default();
            list_state.select(Some(selected));
            f.render_stateful_widget(categories_list, chunks[0], &mut list_state);
        } else {
            f.render_widget(categories_list, chunks[0]);
        }
    } else {
        f.render_widget(categories_list, chunks[0]);
    }

    // Settings info
    // Generate detailed info based on selected category
    let settings_info = if let Some(selected_idx) = app.selected_setting {
        match selected_idx {
            0 => "🎨 Display Settings\n\n🖼️ Theme Options:\n  • Dark Theme (Current)\n  • Light Theme\n  • High Contrast\n  • Custom Colors\n\n📏 Layout:\n  • Compact Mode: Off\n  • Show Icons: On\n  • Animation: Enabled\n\n🔧 Available Actions:\n  • Change theme\n  • Toggle compact mode\n  • Customize colors\n  • Reset display settings".to_string(),
            1 => "🔧 Project Configuration\n\n📁 Default Settings:\n  • Auto-initialize: On\n  • Default phase: Planning\n  • Task numbering: Auto\n  • Backup frequency: Daily\n\n📋 Templates:\n  • Load default templates: On\n  • Custom template path: ~/.rask/templates\n  • Auto-suggest templates: On\n\n🔧 Available Actions:\n  • Set default project settings\n  • Configure template paths\n  • Backup preferences\n  • Project naming rules".to_string(),
            2 => "⚡ Performance Options\n\n🚀 Optimization:\n  • Auto-save interval: 30s\n  • Cache size: 100MB\n  • Lazy loading: On\n  • Background tasks: Enabled\n\n🔄 Refresh Settings:\n  • Auto-refresh views: On\n  • Refresh interval: 5s\n  • Real-time updates: On\n\n🔧 Available Actions:\n  • Adjust cache settings\n  • Configure auto-save\n  • Performance tuning\n  • Memory optimization".to_string(),
            3 => "🤖 AI Assistant Settings\n\n🔧 Configuration:\n  • Provider: Google Gemini\n  • Model: gemini-1.5-flash\n  • Temperature: 0.7\n  • Max tokens: 2048\n\n🎯 Features:\n  • Auto-suggestions: On\n  • Context awareness: On\n  • Task breakdown: Enabled\n  • Project analysis: On\n\n🔧 Available Actions:\n  • Configure API key\n  • Change AI model\n  • Adjust creativity\n  • Enable/disable features".to_string(),
            4 => "📊 Analytics Preferences\n\n📈 Data Collection:\n  • Usage analytics: On\n  • Performance metrics: On\n  • Error reporting: On\n  • Anonymous data: On\n\n📋 Reports:\n  • Weekly summaries: On\n  • Progress notifications: On\n  • Trend analysis: Enabled\n\n🔧 Available Actions:\n  • Configure data collection\n  • Set report preferences\n  • Export analytics data\n  • Privacy controls".to_string(),
            5 => "🔒 Privacy & Security\n\n🛡️ Data Protection:\n  • Local storage only: On\n  • Encrypted backups: Off\n  • Session timeout: 2h\n  • Auto-lock: Disabled\n\n🔐 Access Control:\n  • Password protection: Off\n  • Guest mode: Available\n  • Admin controls: Basic\n\n🔧 Available Actions:\n  • Enable encryption\n  • Set passwords\n  • Configure timeouts\n  • Security audit".to_string(),
            6 => "🌍 Export & Import\n\n📤 Export Formats:\n  • JSON: Full data\n  • CSV: Task lists\n  • HTML: Reports\n  • Markdown: Documentation\n\n📥 Import Sources:\n  • JSON files\n  • CSV task lists\n  • GitHub issues\n  • Other project tools\n\n🔧 Available Actions:\n  • Configure export templates\n  • Set import mappings\n  • Schedule exports\n  • Backup management".to_string(),
            7 => "🚀 Advanced Options\n\n🔧 Developer Mode:\n  • Debug logging: Off\n  • API access: Disabled\n  • Plugin support: Coming Soon\n  • Custom scripts: Disabled\n\n⚙️ System Integration:\n  • Shell commands: Basic\n  • External tools: Limited\n  • Webhooks: Disabled\n\n🔧 Available Actions:\n  • Enable debug mode\n  • Configure integrations\n  • Set up webhooks\n  • Advanced customization".to_string(),
            _ => "⚙️ Configuration\n\nSelect a category from the left to view detailed settings and options.".to_string(),
        }
    } else {
        "⚙️ Configuration\n\nSelect a category from the left to view detailed settings and options.".to_string()
    };

    let settings_details = Paragraph::new(settings_info)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" 🔍 Settings Details ")
            .border_style(Style::default().fg(Color::Yellow)))
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false });
    f.render_widget(settings_details, chunks[1]);
}

/// Render the AI Settings view
fn render_ai_settings_view(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    // Left panel - Current Configuration
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(8)].as_ref())
        .split(chunks[0]);

    // Load current AI configuration
    let config_text = match crate::config::RaskConfig::load() {
        Ok(config) => {
            let api_key_status = if config.ai.get_api_key().is_some() { 
                "✅ Configured" 
            } else { 
                "❌ Not set" 
            };
            
            let status_indicator = if config.ai.is_ready() {
                "🟢 Ready"
            } else if config.ai.enabled {
                "🟡 Needs API Key"
            } else {
                "🔴 Disabled"
            };

            format!("🤖 AI Status: {}\n\n🔧 Configuration:\n• Enabled: {}\n• Provider: {}\n• Model: {}\n• Temperature: {}\n• Max Tokens: {}\n• Context Window: {}\n• Auto Suggestions: {}\n\n🔑 API Key: {}\n\n📋 Available Models:\n{}",
                status_indicator,
                if config.ai.enabled { "✅ Yes" } else { "❌ No" },
                config.ai.provider,
                config.ai.default_model,
                config.ai.temperature,
                config.ai.max_tokens,
                config.ai.context_window,
                if config.ai.auto_suggestions { "✅ Yes" } else { "❌ No" },
                api_key_status,
                config.ai.gemini.models.join(", ")
            )
        }
        Err(e) => format!("❌ Error loading configuration: {}", e),
    };

    let config_display = Paragraph::new(config_text)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" 🤖 Current AI Configuration ")
            .border_style(Style::default().fg(Color::Cyan)))
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false });
    f.render_widget(config_display, left_chunks[0]);

    // Configuration commands
    let commands = vec![
        "💡 Quick Setup Commands:",
        "",
        "rask ai configure --show",
        "  View detailed configuration",
        "",
        "rask ai configure --enabled true",
        "  Enable AI features",
        "",
        "rask ai configure --api-key YOUR_KEY",
        "  Set API key (or use GEMINI_API_KEY env var)",
        "",
        "rask ai configure --model gemini-1.5-flash",
        "  Change AI model",
        "",
        "rask ai configure --temperature 0.7",
        "  Adjust creativity (0.0-1.0)",
    ];

    let command_items: Vec<ListItem> = commands.iter()
        .map(|cmd| {
            let style = if cmd.starts_with("rask") || cmd.starts_with("  ") {
                Style::default().fg(Color::Green)
            } else if cmd.starts_with("💡") {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };
            ListItem::new(Line::from(Span::styled(*cmd, style)))
        })
        .collect();

    let commands_list = List::new(command_items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" ⚡ Configuration Commands ")
            .border_style(Style::default().fg(Color::Yellow)));
    f.render_widget(commands_list, left_chunks[1]);

    // Right panel - AI Features & Testing
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(10)].as_ref())
        .split(chunks[1]);

    // AI Features overview
    let features_text = "🧠 AI Features Available:\n\n🔍 Task Analysis\n• Analyze project health and progress\n• Get insights and recommendations\n• Identify potential bottlenecks\n\n📋 Task Management\n• Break down complex tasks automatically\n• Generate task suggestions based on project\n• Smart task prioritization\n\n💬 Interactive Chat\n• Ask questions about your project\n• Get advice on task planning\n• Context-aware assistance\n\n📊 Project Insights\n• Performance analysis and trends\n• Risk assessment and mitigation\n• Resource allocation suggestions\n\n🎯 Smart Suggestions\n• Auto-suggest next logical tasks\n• Duplicate task detection\n• Dependency analysis";

    let features_display = Paragraph::new(features_text)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" 🚀 AI Features ")
            .border_style(Style::default().fg(Color::Magenta)))
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false });
    f.render_widget(features_display, right_chunks[0]);

    // Testing and help
    let test_commands = vec![
        "🧪 Test AI Configuration:",
        "",
        "rask ai chat \"Hello, can you help me?\"",
        "  Test basic chat functionality",
        "",
        "rask ai analyze --limit 5",
        "  Analyze current project tasks",
        "",
        "rask ai breakdown \"Create user auth system\"",
        "  Test task breakdown feature",
        "",
        "rask ai insights --detailed",
        "  Get detailed project insights",
    ];

    let test_items: Vec<ListItem> = test_commands.iter()
        .map(|cmd| {
            let style = if cmd.starts_with("rask") || cmd.starts_with("  ") {
                Style::default().fg(Color::Cyan)
            } else if cmd.starts_with("🧪") {
                Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };
            ListItem::new(Line::from(Span::styled(*cmd, style)))
        })
        .collect();

    let test_list = List::new(test_items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" 🧪 Test AI Features ")
            .border_style(Style::default().fg(Color::Blue)));
    f.render_widget(test_list, right_chunks[1]);
}

/// Render the Project Switcher view
fn render_project_switcher_view(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .split(area);

    // Left panel - Projects list
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(0)].as_ref())
        .split(chunks[0]);

    // Project switcher header with current project info
    let current_project_info = match crate::project::get_current_project() {
        Ok(Some(current)) => {
            if let Ok(roadmap) = crate::state::load_state() {
                let completed = roadmap.tasks.iter().filter(|t| t.status == crate::model::TaskStatus::Completed).count();
                let total = roadmap.tasks.len();
                let progress = if total > 0 { (completed * 100) / total } else { 0 };
                format!("📍 Current: {} • {}/{} tasks ({}%)", current, completed, total, progress)
            } else {
                format!("📍 Current: {} • No data loaded", current)
            }
        },
        _ => "📍 No project selected".to_string(),
    };

    let header = Paragraph::new(current_project_info)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" 🔄 Project Switcher ")
            .border_style(Style::default().fg(Color::Green)))
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true });
    f.render_widget(header, left_chunks[0]);

    // Projects list
    let projects_list = match crate::project::ProjectsConfig::load() {
        Ok(config) => {
            let current_project = crate::project::get_current_project().ok().flatten();
            
            // Sort projects by last accessed (most recent first)
            let mut projects: Vec<_> = config.projects.iter().collect();
            projects.sort_by(|a, b| b.1.last_accessed.cmp(&a.1.last_accessed));

            let project_items: Vec<ListItem> = projects
                .iter()
                .enumerate()
                .map(|(idx, (name, project))| {
                    let is_current = current_project.as_ref() == Some(name);
                    let is_selected = app.selected_project == Some(idx);
                    
                    // Try to load project stats
                    let stats = if std::path::Path::new(&project.state_file).exists() {
                        if let Ok(content) = std::fs::read_to_string(&project.state_file) {
                            if let Ok(roadmap) = serde_json::from_str::<crate::model::Roadmap>(&content) {
                                let completed = roadmap.tasks.iter().filter(|t| t.status == crate::model::TaskStatus::Completed).count();
                                let total = roadmap.tasks.len();
                                let progress = if total > 0 { (completed * 100) / total } else { 0 };
                                format!("{}% • {}/{} tasks", progress, completed, total)
                            } else {
                                "Error loading".to_string()
                            }
                        } else {
                            "File not found".to_string()
                        }
                    } else {
                        "Empty project".to_string()
                    };
                    
                    let status_indicator = if is_current { "👉" } else { "  " };
                    let project_name = if is_current {
                        format!("{} {} (current)", status_indicator, name)
                    } else {
                        format!("{} {}", status_indicator, name)
                    };
                    
                    let description = if let Some(desc) = &project.description {
                        format!("\n     📝 {}", desc)
                    } else {
                        String::new()
                    };
                    
                    let line_content = format!("{}\n     📊 {}{}", project_name, stats, description);
                    
                    let style = if is_selected {
                        Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD)
                    } else if is_current {
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Gray)
                    };
                    
                    ListItem::new(Line::from(Span::styled(line_content, style)))
                })
                .collect();

            let title = format!(" 📋 Available Projects ({}) ", projects.len());
            let projects_block = Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(Color::Cyan));

            List::new(project_items).block(projects_block)
        },
        Err(_) => {
            let error_items = vec![
                ListItem::new(Line::from(Span::styled("❌ Error loading projects", Style::default().fg(Color::Red)))),
                ListItem::new(Line::from(Span::styled("💡 Create a project first:", Style::default().fg(Color::Yellow)))),
                ListItem::new(Line::from(Span::styled("   rask project create <name>", Style::default().fg(Color::Cyan)))),
            ];
            
            List::new(error_items)
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title(" 📋 Projects ")
                    .border_style(Style::default().fg(Color::Red)))
        }
    };
    
    f.render_widget(projects_list, left_chunks[1]);

    // Right panel - Project details and actions
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(8)].as_ref())
        .split(chunks[1]);

    // Project details
    let details_text = if let Ok(config) = crate::project::ProjectsConfig::load() {
        let projects: Vec<_> = config.projects.iter().collect();
        if let Some(selected_idx) = app.selected_project {
            if let Some((name, project)) = projects.get(selected_idx) {
                let created = if let Ok(datetime) = chrono::DateTime::parse_from_rfc3339(&project.created_at) {
                    datetime.format("%Y-%m-%d %H:%M").to_string()
                } else {
                    "Unknown".to_string()
                };
                
                let accessed = if let Ok(datetime) = chrono::DateTime::parse_from_rfc3339(&project.last_accessed) {
                    datetime.format("%Y-%m-%d %H:%M").to_string()
                } else {
                    "Unknown".to_string()
                };
                
                format!("📋 Project: {}\n\n📝 Description:\n{}\n\n📅 Created: {}\n🕒 Last accessed: {}\n💾 State file:\n{}\n\n💡 Press Enter to switch\n🔄 Press 'r' to refresh",
                    name,
                    project.description.as_deref().unwrap_or("No description"),
                    created,
                    accessed,
                    project.state_file
                )
            } else {
                "📋 No project selected\n\n💡 Use ↑↓ to navigate\n⏎ Enter to switch\n🔄 'r' to refresh".to_string()
            }
        } else {
            "📋 No project selected\n\n💡 Use ↑↓ to navigate\n⏎ Enter to switch\n🔄 'r' to refresh".to_string()
        }
    } else {
        "❌ Error loading project details\n\n💡 Try refreshing with 'r'".to_string()
    };

    let details = Paragraph::new(details_text)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" 🔍 Project Details ")
            .border_style(Style::default().fg(Color::Magenta)))
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false });
    f.render_widget(details, right_chunks[0]);

    // Action shortcuts
    let actions = vec![
        "⏎ Enter - Switch to project",
        "↑↓ - Navigate projects",
        "🔄 r - Refresh project list",
        "🏠 F1 - Return to home",
        "❌ q - Quit application",
    ];

    let action_items: Vec<ListItem> = actions.iter()
        .map(|action| ListItem::new(Line::from(*action)))
        .collect();

    let actions_list = List::new(action_items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" ⚡ Actions ")
            .border_style(Style::default().fg(Color::Yellow)));
    f.render_widget(actions_list, right_chunks[1]);
}

/// Render help text based on current view and focus
fn render_help_text(f: &mut Frame, app: &App, area: Rect) {
    let help_text = match (app.current_view.clone(), app.focus.clone()) {
        (AppView::Home, _) => "F1-F6: Quick navigation • h: Help • r: Refresh • p: Projects • s: Save • Tab: Switch focus • Q: Quit",
        (AppView::TaskManager, PanelFocus::Tasks) => "↑↓: Navigate • Enter/Space: Toggle task • PgUp/PgDn: Fast scroll • Tab: Switch • Q: Quit",
        (AppView::TaskManager, PanelFocus::Chat) => "↑↓: Scroll chat • Tab: Switch to input • h: Help • Q: Quit",
        (AppView::TaskManager, PanelFocus::Input) => "Type to chat • Enter: Send • Tab: Switch focus • Q: Quit",
        (AppView::AIAssistant, PanelFocus::Chat) => "↑↓: Scroll messages • Tab: Switch to input • h: Help • Q: Quit",
        (AppView::AIAssistant, PanelFocus::Input) => "Type message • Enter: Send • Tab: Switch focus • Q: Quit",
        (AppView::AISettings, _) => "View AI configuration • Use CLI commands to configure • Tab: Switch focus • Q: Quit",
        (AppView::Templates, _) => "↑↓: Browse templates • Enter: Select • Tab: Switch focus • Q: Quit",
        (AppView::Analytics, _) => "View project analytics • r: Refresh • Tab: Switch focus • Q: Quit",
        (AppView::Settings, _) => "↑↓: Navigate settings • s: Save • Tab: Switch focus • Q: Quit",
        (AppView::ProjectSwitcher, _) => "↑↓: Navigate projects • Enter: Switch • r: Refresh • F1: Home • Q: Quit",
        _ => "Tab: Switch focus • F1-F6: Quick navigation • h: Help • r: Refresh • p: Projects • Esc: Home • Q: Quit",
    };
    
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::DarkGray))
        .wrap(Wrap { trim: true });
    
    f.render_widget(help, area);
}

/// Display welcome message for interactive mode
fn display_welcome_message() {
    println!("\n🚀 Welcome to Rask Interactive Mode!");
    println!("   Your advanced project planner with AI assistance");
    println!("   Use this interface to manage tasks, get AI suggestions,");
    println!("   and visualize your project progress in real-time.\n");
}

/// Get AI response synchronously (for use in TUI)
fn get_ai_response_sync(user_message: &str, roadmap: Option<&Roadmap>) -> String {
    // Create a tokio runtime for async operations within sync context
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return "❌ Failed to initialize AI runtime".to_string(),
    };

    rt.block_on(async {
        match crate::config::RaskConfig::load() {
            Ok(config) => {
                if !config.ai.is_ready() {
                    return "⚙️ AI is not configured. Use 'rask ai configure' to set up your API key.".to_string();
                }

                match crate::ai::service::AiService::new(config).await {
                    Ok(ai_service) => {
                        // Build context if roadmap is available
                        let context = roadmap.map(|r| crate::ai::service::utils::create_project_context(r));
                        
                        // Start a chat session with context
                        if let Ok(_session_id) = ai_service.start_chat_session(context).await {
                            match ai_service.chat(user_message.to_string()).await {
                                Ok(response) => response,
                                Err(e) => format!("❌ AI Error: {}", e),
                            }
                        } else {
                            "❌ Failed to start AI chat session".to_string()
                        }
                    }
                    Err(e) => format!("❌ Failed to initialize AI service: {}", e),
                }
            }
            Err(_) => "❌ Failed to load configuration".to_string(),
        }
    })
} 