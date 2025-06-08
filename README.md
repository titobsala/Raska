# Rask - A CLI Project Planner in Rust

**Rask** is a command-line tool built with Rust to help you manage and track software projects. It's designed to be simple, fast, and keyboard-centric.

It takes a project plan written in a simple Markdown format, generates a task list, and allows you to update the project's state as you complete tasks.

## Core Concepts

- **Plan in Markdown:** Define your project epics and tasks in a simple, human-readable `.md` file.
- **Stateful CLI:** Rask initializes a state file (`.rask_state.json`) from your markdown plan. It then uses this state file to track progress, so your original plan remains untouched.
- **Keyboard-First:** All interactions are through the command line, designed for speed and efficiency.

## Current Status

This project is currently under development as a learning exercise for Rust.

## Getting Started (Future Steps)

1.  **Initialize a project:**
    ```bash
    rask init roadmap.md
    ```

2.  **View the current project status:**
    ```bash
    rask show
    ```

3.  **Complete a task:**
    ```bash
    rask complete <TASK_ID>
    ```

## Built With

- [Rust](https://www.rust-lang.org/)
- [clap](https://crates.io/crates/clap) - For command-line argument parsing.
- [serde](https://crates.io/crates/serde) - For data serialization/deserialization.
- [pulldown-cmark](https://crates.io/crates/pulldown-cmark) - For parsing Markdown.
- [colored](https://crates.io/crates/colored) - For beautiful terminal output.
