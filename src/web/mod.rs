//! Web interface module for Rask
//! 
//! This module provides a web-based interface for Rask that complements the CLI.
//! It includes a REST API, WebSocket support for real-time updates, and serves
//! a React frontend for complex visualizations and AI interactions.

pub mod server;
pub mod routes;
pub mod websocket;
pub mod handlers;
pub mod watcher;

pub use server::*;

use anyhow::Result;

/// Start the web server with the given configuration
pub async fn start_web_server(port: u16, host: &str) -> Result<()> {
    server::run_server(port, host).await
}