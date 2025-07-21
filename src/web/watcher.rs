//! File watching implementation for real-time state synchronization

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc;
use tokio::sync::broadcast;
use anyhow::Result;

use crate::ui;

/// File watcher for monitoring .rask/state.json changes
pub struct StateWatcher {
    sender: broadcast::Sender<StateChangeEvent>,
}

/// Events that occur when state files change
#[derive(Debug, Clone)]
pub enum StateChangeEvent {
    TasksUpdated,
    ProjectModified,
    ConfigChanged,
}

impl StateWatcher {
    /// Create a new state watcher
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        Self { sender }
    }

    /// Start watching the .rask directory for changes
    pub async fn start_watching(&self, project_path: &str) -> Result<()> {
        let rask_path = Path::new(project_path).join(".rask");
        
        if !rask_path.exists() {
            return Err(anyhow::anyhow!("No .rask directory found at {}", project_path));
        }

        let sender = self.sender.clone();
        let rask_path_clone = rask_path.clone();

        tokio::spawn(async move {
            if let Err(e) = watch_directory(rask_path_clone, sender).await {
                ui::display_error(&format!("File watcher error: {}", e));
            }
        });

        ui::display_info(&format!("📁 Watching for changes in: {}", rask_path.display()));
        Ok(())
    }

    /// Subscribe to state change events
    pub fn subscribe(&self) -> broadcast::Receiver<StateChangeEvent> {
        self.sender.subscribe()
    }

    /// Get the broadcast sender (for testing or manual triggering)
    pub fn sender(&self) -> broadcast::Sender<StateChangeEvent> {
        self.sender.clone()
    }
}

/// Watch a directory for file changes
async fn watch_directory(
    path: impl AsRef<Path>,
    sender: broadcast::Sender<StateChangeEvent>,
) -> Result<()> {
    let (tx, rx) = mpsc::channel();

    let mut watcher = RecommendedWatcher::new(
        move |res| {
            if let Err(e) = tx.send(res) {
                eprintln!("Failed to send file watcher event: {}", e);
            }
        },
        Config::default(),
    )?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    // Process file system events
    tokio::task::spawn_blocking(move || {
        for event in rx {
            match event {
                Ok(event) => {
                    if let Err(e) = handle_file_event(event, &sender) {
                        eprintln!("Error handling file event: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("File watcher error: {}", e);
                }
            }
        }
    })
    .await?;

    Ok(())
}

/// Handle individual file system events
fn handle_file_event(
    event: notify::Event,
    sender: &broadcast::Sender<StateChangeEvent>,
) -> Result<()> {
    use notify::EventKind;

    // Only handle write events (file modifications)
    if !matches!(event.kind, EventKind::Modify(_)) {
        return Ok(());
    }

    for path in event.paths {
        if let Some(file_name) = path.file_name() {
            let change_event = match file_name.to_str() {
                Some("state.json") | Some(".rask_state.json") => {
                    StateChangeEvent::TasksUpdated
                }
                Some("config.toml") => StateChangeEvent::ConfigChanged,
                Some("task-details.md") | Some("project-overview.md") => {
                    StateChangeEvent::ProjectModified
                }
                _ => continue, // Ignore other files
            };

            // Broadcast the change event
            if let Err(e) = sender.send(change_event.clone()) {
                eprintln!("Failed to broadcast file change event: {}", e);
            } else {
                println!("📁 Detected change: {:?} in {}", change_event, path.display());
            }
        }
    }

    Ok(())
}

/// Helper function to create and start a state watcher for a project
pub async fn create_project_watcher(project_path: &str) -> Result<StateWatcher> {
    let watcher = StateWatcher::new();
    watcher.start_watching(project_path).await?;
    Ok(watcher)
}