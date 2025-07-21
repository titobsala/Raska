//! Web server command implementation

use super::CommandResult;
use crate::ui::{display_error, display_info, display_success, display_warning};
use crate::web;
use std::process::{Command, Stdio};
use tokio::runtime::Runtime;

/// Handle the web command with various options
pub fn handle_web_command(
    port: u16, 
    host: &str, 
    daemon: bool, 
    stop: bool, 
    status: bool, 
    open: bool
) -> CommandResult {
    if status {
        return show_web_status();
    }

    if stop {
        return stop_web_daemon();
    }

    if daemon {
        return start_web_daemon(port, host);
    }

    // Default: start web server in foreground
    start_web_server(port, host, open)
}

/// Start the web server in foreground mode
fn start_web_server(port: u16, host: &str, open: bool) -> CommandResult {
    // Check if we're in a valid Rask project directory
    if !crate::state::has_local_workspace() {
        display_error("No .rask directory found in current directory.");
        display_info("Please run 'rask init <roadmap.md>' first to initialize a project.");
        return Ok(());
    }

    display_info("🌐 Preparing to start Rask web interface...");

    if open {
        let url = format!("http://{}:{}", host, port);
        display_info(&format!("🚀 Opening {} in your default browser...", url));
        
        // Try to open the browser (don't fail if it doesn't work)
        let _ = open_browser(&url);
    }

    // Create a tokio runtime for async operations
    let rt = Runtime::new().map_err(|e| format!("Failed to create async runtime: {}", e))?;

    rt.block_on(async {
        if let Err(e) = web::start_web_server(port, host).await {
            display_error(&format!("Failed to start web server: {}", e));
        }
    });

    Ok(())
}

/// Start the web server in daemon mode (background)
fn start_web_daemon(port: u16, host: &str) -> CommandResult {
    display_warning("⚠️  Daemon mode not yet implemented");
    display_info("For now, use 'rask web' to run in foreground mode");
    display_info("Press Ctrl+C to stop the server when running in foreground");
    
    // For now, fall back to foreground mode
    start_web_server(port, host, false)
}

/// Stop any running web daemon
fn stop_web_daemon() -> CommandResult {
    display_warning("⚠️  Daemon stop not yet implemented");
    display_info("If running in foreground mode, use Ctrl+C to stop the server");
    Ok(())
}

/// Show web server status
fn show_web_status() -> CommandResult {
    display_info("🌐 Web Server Status");
    display_warning("⚠️  Status checking not yet implemented");
    display_info("Web server management features are in development");
    Ok(())
}

/// Attempt to open URL in the default browser
fn open_browser(url: &str) -> Result<(), std::io::Error> {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/c", "start", url])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(url)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(url)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
    }

    Ok(())
}