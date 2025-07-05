//! Interactive TUI mode for Rask
//!
//! This module provides a rich terminal user interface for project management
//! with integrated AI assistant capabilities using ratatui.

use crate::commands::CommandResult;
use crate::ui::display_info;
use crate::model::{Roadmap, Task, TaskStatus, Priority, Phase};
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
    fs,
    path::PathBuf,
};
use chrono;

/// TUI Settings for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiSettings {
    /// Default view to open on startup
    pub default_view: AppView,
    /// Remember last selected task
    pub remember_selection: bool,
    /// Show welcome message
    pub show_welcome: bool,
}

impl Default for TuiSettings {
    fn default() -> Self {
        Self {
            default_view: AppView::Home,
            remember_selection: true,
            show_welcome: true,
        }
    }
}

impl TuiSettings {
    fn get_settings_path() -> Result<PathBuf, Box<dyn Error>> {
        let config_dir = crate::config::get_rask_config_dir()?;
        Ok(config_dir.join("tui_settings.json"))
    }
    
    pub fn load() -> Self {
        if let Ok(path) = Self::get_settings_path() {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(settings) = serde_json::from_str(&content) {
                    return settings;
                }
            }
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
    /// Which panel is currently focused
    pub focus: PanelFocus,
    /// Current view/screen
    pub current_view: AppView,
    /// Available navigation items
    pub navigation_items: Vec<NavigationItem>,
    /// Selected navigation item
    pub selected_nav_item: usize,
    /// Selected task index
    pub selected_task: Option<usize>,
    /// Scroll offset for task list
    pub task_scroll_offset: usize,
    /// Maximum visible items in task list
    pub max_visible_tasks: usize,
    /// TUI settings
    pub settings: TuiSettings,
    /// Selected template index
    pub selected_template: Option<usize>,
    /// Selected settings item index
    pub selected_setting: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PanelFocus {
    Navigation,
    Tasks,
    Templates,
    Settings,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AppView {
    Home,
    Tasks,
    Templates,
    Settings,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NavigationItem {
    Home,
    Tasks,
    Templates,
    Settings,
}

impl Default for App {
    fn default() -> App {
        let settings = TuiSettings::load();
        let navigation_items = vec![
            NavigationItem::Home,
            NavigationItem::Tasks,
            NavigationItem::Templates,
            NavigationItem::Settings,
        ];
        
        let initial_view = settings.default_view.clone();
        let selected_nav_item = navigation_items
            .iter()
            .position(|item| match (item, &initial_view) {
                (NavigationItem::Home, AppView::Home) => true,
                (NavigationItem::Tasks, AppView::Tasks) => true,
                (NavigationItem::Templates, AppView::Templates) => true,
                (NavigationItem::Settings, AppView::Settings) => true,
                _ => false,
            })
            .unwrap_or(0);

        App {
            should_quit: false,
            roadmap: None,
            focus: PanelFocus::Navigation,
            current_view: initial_view,
            selected_nav_item,
            selected_task: None,
            task_scroll_offset: 0,
            max_visible_tasks: 10, // Will be calculated dynamically
            navigation_items,
            settings,
            selected_template: None,
            selected_setting: None,
        }
    }
}

impl App {}

/// Launch the interactive TUI mode
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
            display_info("No local .rask directory found. Exit TUI and run 'rask init <roadmap.md>' first.");
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
        println!("An error occurred in the TUI: {:?}", err);
    }

    Ok(())
}

/// Main application loop
fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<(), Box<dyn Error>> {
    loop {
        // Clear terminal if needed for clean render
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.focus {
                PanelFocus::Navigation => handle_navigation_keys(key, &mut app),
                PanelFocus::Tasks => handle_tasks_keys(key, &mut app),
                PanelFocus::Templates => handle_templates_keys(key, &mut app),
                PanelFocus::Settings => handle_settings_keys(key, &mut app),
            }
        }

        if app.should_quit {
            app.settings.save()?;
            break;
        }
    }
    Ok(())
}

/// Handle key events when Navigation is focused
fn handle_navigation_keys(key: event::KeyEvent, app: &mut App) {
    match key.code {
        // Global quit
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => app.should_quit = true,

        // Navigation
        KeyCode::Down => {
            // Handle main navigation
            app.selected_nav_item = (app.selected_nav_item + 1) % app.navigation_items.len();
        }
        KeyCode::Up => {
            // Handle main navigation
            app.selected_nav_item = (app.selected_nav_item + app.navigation_items.len() - 1) % app.navigation_items.len();
        }
        KeyCode::Enter => {
            if let Some(nav_item) = app.navigation_items.get(app.selected_nav_item) {
                app.current_view = match nav_item {
                    NavigationItem::Home => AppView::Home,
                    NavigationItem::Tasks => AppView::Tasks,
                    NavigationItem::Templates => AppView::Templates,
                    NavigationItem::Settings => AppView::Settings,
                };
                
                // Initialize selections for specific views
                // Automatically switch focus to the main panel and initialize selections
                app.focus = match app.current_view {
                    AppView::Tasks => {
                        // Validate and fix task selection bounds
                        let task_count = app.roadmap.as_ref().map_or(0, |r| r.tasks.len());
                        if task_count > 0 {
                            if app.selected_task.is_none() {
                                app.selected_task = Some(0);
                            } else if let Some(selected) = app.selected_task {
                                if selected >= task_count {
                                    app.selected_task = Some(task_count - 1);
                                    app.task_scroll_offset = 0; // Reset scroll to avoid issues
                                }
                            }
                        } else {
                            app.selected_task = None;
                        }
                        PanelFocus::Tasks
                    },
                    AppView::Templates => {
                        if app.selected_template.is_none() {
                            app.selected_template = Some(0);
                        }
                        PanelFocus::Templates
                    },
                    AppView::Settings => {
                        if app.selected_setting.is_none() {
                            app.selected_setting = Some(0);
                        }
                        PanelFocus::Settings
                    },
                    _ => PanelFocus::Navigation,
                };
            }
        }
        KeyCode::Tab | KeyCode::Esc => {
            // Switch focus to the main panel of the current view or go back to navigation
            app.focus = match app.current_view {
                AppView::Tasks => PanelFocus::Tasks,
                AppView::Templates => PanelFocus::Templates,
                AppView::Settings => PanelFocus::Settings,
                _ => PanelFocus::Navigation,
            };
        }
        

        _ => {}
    }
}

/// Handle key events for the Tasks panel
fn handle_tasks_keys(key: event::KeyEvent, app: &mut App) {
    let task_count = app.roadmap.as_ref().map_or(0, |r| r.tasks.len());
    match key.code {
        KeyCode::Esc | KeyCode::Tab => app.focus = PanelFocus::Navigation,
        KeyCode::Down => {
            if task_count > 0 {
                let new_idx = app.selected_task.map_or(0, |i| (i + 1) % task_count);
                app.selected_task = Some(new_idx);
            } else {
                app.selected_task = None;
            }
        }
        KeyCode::Up => {
            if task_count > 0 {
                let new_idx = app.selected_task.map_or(task_count - 1, |i| (i + task_count - 1) % task_count);
                app.selected_task = Some(new_idx);
            } else {
                app.selected_task = None;
            }
        }
        KeyCode::Enter => { // Toggle task status
            if let (Some(roadmap), Some(idx)) = (&mut app.roadmap, app.selected_task) {
                if let Some(task) = roadmap.tasks.get_mut(idx) {
                    task.status = match task.status {
                        TaskStatus::Pending => TaskStatus::Completed,
                        TaskStatus::Completed => TaskStatus::Pending,
                    };
                    let _ = crate::state::save_state(roadmap);
                }
            }
        }
        _ => handle_global_keys(key, app),
    }
}

/// Handle key events for the Templates panel
fn handle_templates_keys(key: event::KeyEvent, app: &mut App) {
    let template_count = 8; // Hardcoded count of templates
    match key.code {
        KeyCode::Esc | KeyCode::Tab => app.focus = PanelFocus::Navigation,
        KeyCode::Down => {
            let new_idx = app.selected_template.map_or(0, |i| (i + 1) % template_count);
            app.selected_template = Some(new_idx);
        }
        KeyCode::Up => {
            let new_idx = app.selected_template.map_or(template_count - 1, |i| (i + template_count - 1) % template_count);
            app.selected_template = Some(new_idx);
        }
        KeyCode::Enter => { // Apply template by creating a new task
            if let (Some(roadmap), Some(template_idx)) = (&mut app.roadmap, app.selected_template) {
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
                if let Some((name, desc)) = templates.get(template_idx) {
                    let new_id = roadmap.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
                    let new_task = Task {
                        id: new_id,
                        description: format!("{}: {}", name, desc),
                        status: TaskStatus::Pending,
                        priority: Priority::Medium,
                        phase: Phase::new("Planning".to_string()),
                        created_at: Some(chrono::Utc::now().to_rfc3339()),
                        tags: std::collections::HashSet::new(),
                        dependencies: Vec::new(),
                        notes: None,
                        estimated_hours: None,
                        actual_hours: None,
                        time_sessions: Vec::new(),
                        implementation_notes: Vec::new(),
                        completed_at: None,
                        ai_info: crate::model::AiTaskInfo::default(),
                    };
                    roadmap.tasks.push(new_task);
                    let _ = crate::state::save_state(roadmap);
                    // Switch to tasks view to see the new task
                    app.current_view = AppView::Tasks;
                    app.focus = PanelFocus::Tasks;
                    app.selected_task = Some(roadmap.tasks.len() - 1);
                }
            }
        }
        _ => handle_global_keys(key, app),
    }
}

/// Handle key events for the Settings panel
fn handle_settings_keys(key: event::KeyEvent, app: &mut App) {
    let settings_count = 3; // Number of editable settings
    match key.code {
        KeyCode::Esc | KeyCode::Tab => app.focus = PanelFocus::Navigation,
        KeyCode::Down => {
            let new_idx = app.selected_setting.map_or(0, |i| (i + 1) % settings_count);
            app.selected_setting = Some(new_idx);
        }
        KeyCode::Up => {
            let new_idx = app.selected_setting.map_or(settings_count - 1, |i| (i + settings_count - 1) % settings_count);
            app.selected_setting = Some(new_idx);
        }
        KeyCode::Enter => { // Toggle boolean settings
            if let Some(idx) = app.selected_setting {
                match idx {
                    0 => { // Default View
                        let current_idx = match app.settings.default_view {
                            AppView::Home => 0, AppView::Tasks => 1, AppView::Templates => 2, AppView::Settings => 3,
                        };
                        let next_idx = (current_idx + 1) % 4;
                        app.settings.default_view = match next_idx {
                            0 => AppView::Home, 1 => AppView::Tasks, 2 => AppView::Templates, _ => AppView::Settings,
                        };
                    },
                    1 => app.settings.remember_selection = !app.settings.remember_selection,
                    2 => app.settings.show_welcome = !app.settings.show_welcome,
                    _ => {},
                }
            }
        }
        _ => handle_global_keys(key, app),
    }
}

/// Handle global keys that work in any non-navigation context
fn handle_global_keys(key: event::KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => app.should_quit = true,

        _ => {}
    }
}

/// Render the UI based on current state
fn ui(f: &mut Frame, app: &mut App) {
    // Main layout with navigation bar at top, content, and footer
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(1)].as_ref())
        .split(f.size());

    render_navigation_bar(f, app, main_chunks[0]);
    
    match app.current_view {
        AppView::Home => render_home_view(f, app, main_chunks[1]),
        AppView::Tasks => render_tasks_view(f, app, main_chunks[1]),
        AppView::Templates => render_templates_view(f, app, main_chunks[1]),
        AppView::Settings => render_settings_view(f, app, main_chunks[1]),
    }
    
    render_help_text(f, app, main_chunks[2]);
}

/// Render the top navigation bar
fn render_navigation_bar(f: &mut Frame, app: &mut App, area: Rect) {
    let nav_titles: Vec<String> = app.navigation_items.iter().map(|item| {
        match item {
            NavigationItem::Home => "Home".to_string(),
            NavigationItem::Tasks => "Tasks".to_string(),
            NavigationItem::Templates => "Templates".to_string(),
            NavigationItem::Settings => "Settings".to_string(),
        }
    }).collect();

    let nav_spans: Vec<Span> = nav_titles.iter().enumerate().map(|(i, title)| {
        if i == app.selected_nav_item && app.focus == PanelFocus::Navigation {
            Span::styled(format!(" {} ", title), Style::default().bg(Color::Blue).fg(Color::White))
        } else {
            Span::styled(format!(" {} ", title), Style::default().fg(Color::White))
        }
    }).collect();

    let mut nav_line_spans = Vec::new();
    for (i, span) in nav_spans.into_iter().enumerate() {
        if i > 0 {
            nav_line_spans.push(Span::raw(" | "));
        }
        nav_line_spans.push(span);
    }
    let nav_line = Line::from(nav_line_spans);
    
    let project_name = app.roadmap.as_ref()
        .map(|r| r.title.clone())
        .unwrap_or_else(|| "No Project Loaded".to_string());
    let view_name = format!("{:?}", app.current_view);

    let title = format!("🚀 Rask TUI • {} • {} ", view_name, project_name);
    
    let nav_paragraph = Paragraph::new(nav_line)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(if app.focus == PanelFocus::Navigation {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::Cyan)
            }));
    
    f.render_widget(nav_paragraph, area);
}

/// Render the Home/Overview view
fn render_home_view(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    // Left side - Project stats
    let stats_text = match &app.roadmap {
        Some(roadmap) => {
            let completed = roadmap.tasks.iter().filter(|t| t.status == TaskStatus::Completed).count();
            let total = roadmap.tasks.len();
            let progress = if total > 0 { (completed * 100) / total } else { 0 };
            vec![
                Line::from(vec![Span::styled("Project:", Style::default().add_modifier(Modifier::BOLD)), Span::raw(format!(" {}", roadmap.title))]),
                Line::from(""),
                Line::from(format!("📊 Progress: {}/{} tasks ({}%)", completed, total, progress)),
            ]
        },
        None => vec![Line::from("📋 No Project Loaded"), Line::from("💡 Navigate to Projects to switch or create a project.")],
    };
    let stats_widget = Paragraph::new(stats_text)
        .block(Block::default().borders(Borders::ALL).title(" 🏠 Project Overview ").border_style(Style::default().fg(Color::Green)))
        .wrap(Wrap { trim: false });
    f.render_widget(stats_widget, chunks[0]);

    // Right side - Quick actions
    let actions = vec![
        "↑↓ - Navigate Menu", "Enter - Select View", "Tab - Change Focus", 
        "", "Tasks: Manage your project tasks", "Templates: Create tasks from templates", 
        "Settings: Configure TUI preferences", "Projects: Switch between projects", "", "q - Quit",
    ];
    let action_items: Vec<ListItem> = actions.iter().map(|a| ListItem::new(Line::from(*a))).collect();
    let actions_list = List::new(action_items)
        .block(Block::default().borders(Borders::ALL).title(" ⚡ Quick Actions ").border_style(Style::default().fg(Color::Yellow)));
    f.render_widget(actions_list, chunks[1]);
}

/// Render the Task Manager view
fn render_tasks_view(f: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .title(" 📝 Task List ")
        .borders(Borders::ALL)
        .border_style(if app.focus == PanelFocus::Tasks { Style::default().fg(Color::Yellow) } else { Style::default() });
    
    let task_items: Vec<ListItem> = if let Some(roadmap) = &app.roadmap {
        if roadmap.tasks.is_empty() {
            vec![ListItem::new("No tasks in this project yet.")]
        } else {
            // Update max visible tasks based on area height
            app.max_visible_tasks = area.height.saturating_sub(2) as usize;

            // Scroll logic
            if let Some(selected) = app.selected_task {
                if selected < app.task_scroll_offset {
                    app.task_scroll_offset = selected;
                } else if selected >= app.task_scroll_offset + app.max_visible_tasks {
                    app.task_scroll_offset = selected - app.max_visible_tasks + 1;
                }
            }

            roadmap.tasks.iter().enumerate()
                .skip(app.task_scroll_offset)
                .take(app.max_visible_tasks)
                .map(|(i, task)| {
                let status_icon = if task.status == TaskStatus::Completed { "✅" } else { "⏳" };
                let content = format!("{} #{} {}", status_icon, task.id, task.description);
                // Fix: compare with the actual task index (i + scroll_offset) not just i
                let style = if app.selected_task == Some(i + app.task_scroll_offset) {
                    Style::default().bg(Color::Blue).fg(Color::White)
                } else {
                    Style::default()
                };
                ListItem::new(Line::from(Span::styled(content, style)))
            }).collect()
        }
    } else {
        vec![ListItem::new("No project loaded. Navigate to Projects to select one.")]
    };

    let list = List::new(task_items).block(block);
    let mut list_state = ListState::default();
    // Adjust selected index for scrolling offset
    let adjusted_selection = app.selected_task.map(|idx| {
        if idx >= app.task_scroll_offset {
            Some(idx - app.task_scroll_offset)
        } else {
            None
        }
    }).flatten();
    list_state.select(adjusted_selection);
    f.render_stateful_widget(list, area, &mut list_state);
}


/// Render the Templates view
fn render_templates_view(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(area);

    // Left panel: Template list
    let templates = vec![
        "Web Development Project", "Mobile App Development", "Data Analysis Project", "Game Development",
        "Research Project", "Infrastructure Setup", "Bug Fix Template", "Feature Development",
    ];
    let template_items: Vec<ListItem> = templates.iter().enumerate().map(|(i, name)| {
        let style = if app.selected_template == Some(i) {
            Style::default().bg(Color::Blue).fg(Color::White)
        } else {
            Style::default()
        };
        ListItem::new(Line::from(Span::styled(*name, style)))
    }).collect();
    
    let list_block = Block::default().borders(Borders::ALL).title(" 📄 Templates ").border_style(if app.focus == PanelFocus::Templates { Style::default().fg(Color::Yellow) } else { Style::default() });
    let list = List::new(template_items).block(list_block);
    let mut list_state = ListState::default();
    list_state.select(app.selected_template);
    f.render_stateful_widget(list, chunks[0], &mut list_state);

    // Right panel: Preview
    let descriptions = vec![
        "A full-stack web application template.", "A cross-platform mobile app template.", "A data analysis workflow template.", "A complete game development pipeline.",
        "An academic or industry research project.", "A DevOps infrastructure setup template.", "A systematic workflow for fixing bugs.", "A template for implementing new features.",
    ];
    let preview_text = app.selected_template.and_then(|i| descriptions.get(i)).unwrap_or(&"Select a template to see details.");
    let preview_widget = Paragraph::new(*preview_text)
        .block(Block::default().borders(Borders::ALL).title(" 🔍 Preview ").border_style(Style::default().fg(Color::Green)))
        .wrap(Wrap { trim: true });
    f.render_widget(preview_widget, chunks[1]);
}

/// Render the Settings view
fn render_settings_view(f: &mut Frame, app: &mut App, area: Rect) {
    let settings_items = vec![
        format!("Default View: {:?}", app.settings.default_view),
        format!("Remember Selection: {}", if app.settings.remember_selection { "Yes" } else { "No" }),
        format!("Show Welcome: {}", if app.settings.show_welcome { "Yes" } else { "No" }),
    ];

    let items: Vec<ListItem> = settings_items.iter().enumerate().map(|(i, item)| {
        let style = if app.selected_setting == Some(i) {
            Style::default().bg(Color::Blue).fg(Color::White)
        } else {
            Style::default()
        };
        ListItem::new(Line::from(Span::styled(item, style)))
    }).collect();

    let list_block = Block::default().borders(Borders::ALL).title(" ⚙️ Settings ").border_style(if app.focus == PanelFocus::Settings { Style::default().fg(Color::Yellow) } else { Style::default() });
    let list = List::new(items).block(list_block);
    let mut list_state = ListState::default();
    list_state.select(app.selected_setting);
    f.render_stateful_widget(list, area, &mut list_state);
}

/// Render the footer help text
fn render_help_text(f: &mut Frame, app: &App, area: Rect) {
    let help_text = match app.focus {
        PanelFocus::Navigation => "↑↓: Navigate menu | Enter: Select view | Tab: Focus content | q: Quit",
        PanelFocus::Tasks => "↑↓: Navigate tasks | Enter: Toggle status | Tab/Esc: Back to navigation | q: Quit",
        PanelFocus::Templates => "↑↓: Select template | Enter: Apply template | Tab/Esc: Back to navigation | q: Quit",
        PanelFocus::Settings => "↑↓: Select setting | Enter: Change value | Tab/Esc: Back to navigation | q: Quit",
    };
    let help = Paragraph::new(help_text).style(Style::default().fg(Color::DarkGray));
    f.render_widget(help, area);
}

/// Display a simple welcome message
fn display_welcome_message() {
    println!("\n🚀 Welcome to Rask Interactive Mode!");
    println!("   Manage your project tasks efficiently from the terminal.");
    println!("   Loading TUI...\n");
}