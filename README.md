# Rask 🚀 - Advanced CLI Project Planner

[![Version](https://img.shields.io/badge/version-2.2.0-blue.svg)](https://github.com/tito-sala/rask)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

**Rask** is a powerful, feature-rich command-line project planner built with Rust. It transforms simple Markdown files into sophisticated project management systems with advanced task tracking, dependency management, multi-project support, and beautiful terminal UI.

## ✨ Key Features

### 🎯 **Core Task Management**
- Initialize projects from Markdown files
- Complete, add, edit, and remove tasks
- Maintain sync between state files and Markdown sources
- Beautiful, colored terminal output with progress tracking

### 🏷️ **Enhanced Task Metadata**
- **Tags**: Categorize tasks with custom tags (`#backend`, `#urgent`, etc.)
- **Priorities**: Four priority levels (Low, Medium, High, Critical) with visual indicators
- **Notes**: Detailed descriptions and context for each task
- **Dependencies**: Link tasks with sophisticated dependency management

### 🔍 **Advanced Filtering & Search**
- Filter by tags, priority, status, or any combination
- Full-text search across task descriptions and notes
- Flexible AND/OR filter operations
- List ready-to-start vs blocked tasks

### 🏢 **Multi-Project Workspace**
- Manage multiple projects simultaneously
- Project-specific state files and configurations
- Easy project switching and isolation
- Default project settings

### 🔗 **Dependency Management**
- Define task dependencies with automatic validation
- Circular dependency detection and prevention
- Visual dependency tree exploration
- Dependency chain analysis and impact assessment
- Block task completion until dependencies are satisfied

### 📊 **Visualization & Analysis**
- Comprehensive dependency trees with ASCII art
- Progress bars and completion statistics
- Ready vs blocked task analysis
- Project overview dashboards

## 🚀 Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/tito-sala/rask.git
cd rask

# Build and install
cargo install --path .
```

### Basic Usage

1. **Create a roadmap in Markdown:**
```markdown
# My Project Roadmap

- [ ] Set up development environment
- [ ] Design database schema
- [x] Create API endpoints
- [ ] Implement authentication
- [ ] Write tests
- [ ] Deploy to production
```

2. **Initialize your project:**
```bash
rask init roadmap.md
```

3. **View your project:**
```bash
rask show
```

4. **Complete tasks:**
```bash
rask complete 1
```

## 📚 Complete Command Reference

### Core Commands

#### `rask init <file.md>`
Initialize a new project from a Markdown file.
```bash
rask init project-roadmap.md
```

#### `rask show`
Display the current project status with beautiful formatting.
```bash
rask show  # Shows full project overview
```

#### `rask complete <task_id>`
Mark a task as completed (respects dependencies).
```bash
rask complete 5
```

#### `rask add <description> [OPTIONS]`
Add a new task with rich metadata.
```bash
# Basic task
rask add "Implement user authentication"

# Advanced task with metadata
rask add "Deploy to production" \
  --tag deployment,critical \
  --priority high \
  --depends-on 8,9,10 \
  --note "Requires database migration and SSL certificates"
```

**Options:**
- `--tag <TAGS>`: Comma-separated tags
- `--priority <LEVEL>`: low, medium, high, critical
- `--note <TEXT>`: Detailed description
- `--depends-on <IDS>`: Comma-separated task IDs

#### `rask edit <task_id> <new_description>`
Edit a task's description.
```bash
rask edit 3 "Updated task description"
```

#### `rask remove <task_id>`
Remove a task from the project.
```bash
rask remove 7
```

#### `rask reset [task_id]`
Reset task(s) to pending status.
```bash
rask reset 5    # Reset specific task
rask reset      # Reset all tasks
```

### Advanced Filtering & Search

#### `rask list [OPTIONS]`
List and filter tasks with advanced options.

```bash
# Filter by tags
rask list --tag backend,database

# Filter by priority
rask list --priority high,critical

# Filter by status
rask list --status pending

# Search by text
rask list --search "authentication"

# Combine filters (AND operation)
rask list --tag backend --priority high --status pending

# Show detailed view with metadata
rask list --detailed
```

**Filter Options:**
- `--tag <TAGS>`: Filter by comma-separated tags
- `--priority <LEVELS>`: Filter by priority levels
- `--status <STATUS>`: Filter by pending/completed
- `--search <QUERY>`: Full-text search
- `--detailed`: Show full task metadata
- `--and`: Use AND logic for multiple filters (default)
- `--or`: Use OR logic for multiple filters

### Project Management

#### `rask project create <name> [--description <desc>]`
Create a new project workspace.
```bash
rask project create web-app --description "Main web application project"
```

#### `rask project list`
List all available projects.
```bash
rask project list
```

#### `rask project switch <name>`
Switch to a different project.
```bash
rask project switch mobile-app
```

#### `rask project delete <name> [--force]`
Delete a project (requires confirmation unless --force).
```bash
rask project delete old-project --force
```

### Dependency Management

#### `rask dependencies`
Show dependency analysis overview.
```bash
rask dependencies  # Overview of all dependency statistics
```

#### `rask dependencies --validate`
Validate all dependencies for issues.
```bash
rask dependencies --validate
```

#### `rask dependencies --tree <task_id>`
Show dependency tree for a specific task.
```bash
rask dependencies --tree 15
```

#### `rask dependencies --ready`
Show tasks ready to be started.
```bash
rask dependencies --ready
```

#### `rask dependencies --blocked`
Show tasks blocked by incomplete dependencies.
```bash
rask dependencies --blocked
```

## 💡 Usage Examples

### Scenario 1: Web Development Project

```bash
# Initialize from roadmap
rask init web-project.md

# Add backend tasks with dependencies
rask add "Set up database" --tag backend,infrastructure --priority high
rask add "Create user model" --tag backend,database --depends-on 1
rask add "Implement authentication" --tag backend,security --depends-on 2

# Add frontend tasks
rask add "Create login component" --tag frontend,ui --depends-on 3
rask add "Design dashboard" --tag frontend,ui --priority medium

# Check what's ready to work on
rask dependencies --ready

# View specific dependency tree
rask dependencies --tree 4

# Work on ready tasks
rask complete 1  # Database setup
rask complete 2  # User model (now unblocked)
```

### Scenario 2: Multi-Project Workflow

```bash
# Create multiple projects
rask project create backend-api --description "REST API development"
rask project create mobile-app --description "React Native mobile app"
rask project create devops --description "Infrastructure and deployment"

# Switch between projects
rask project switch backend-api
rask add "Implement OAuth" --tag auth --priority critical

rask project switch mobile-app
rask add "Create login screen" --tag ui --depends-on backend-task-id

# List all projects
rask project list
```

### Scenario 3: Complex Dependency Analysis

```bash
# Add tasks with complex dependencies
rask add "Design system architecture" --tag architecture --priority critical
rask add "Set up CI/CD pipeline" --tag devops --depends-on 1
rask add "Implement core modules" --tag backend --depends-on 1
rask add "Write integration tests" --tag testing --depends-on 3
rask add "Deploy to staging" --tag deployment --depends-on 2,4

# Analyze the full dependency structure
rask dependencies
rask dependencies --validate
rask dependencies --tree 5

# See what's blocking progress
rask dependencies --blocked
```

## 🎨 Terminal UI Features

Rask provides a beautiful, intuitive terminal interface with:

- **Color-coded priorities**: Visual indicators for task importance
- **Progress bars**: Real-time completion tracking
- **Dependency trees**: ASCII art visualization of task relationships
- **Status indicators**: Clear visual task states
- **Project context**: Always know which project you're working on
- **Smart formatting**: Responsive layout that adapts to terminal width

## 🏗️ Architecture & Design

### Project Structure
```
~/.rask/
├── .rask_projects.json          # Global project configuration
├── .rask_current_project        # Current active project
├── .rask_state_project1.json    # Project-specific state files
└── .rask_state_project2.json
```

### Key Design Principles

1. **Markdown-first**: Original roadmaps remain in human-readable format
2. **State isolation**: Each project maintains independent state
3. **Dependency integrity**: Robust validation prevents impossible states
4. **Performance**: Efficient algorithms handle large project hierarchies
5. **Extensibility**: Modular design supports future enhancements

## 🛠️ Built With

- **[Rust](https://www.rust-lang.org/)** - Systems programming language
- **[clap](https://crates.io/crates/clap)** - Command-line argument parsing
- **[serde](https://crates.io/crates/serde)** - Serialization framework
- **[pulldown-cmark](https://crates.io/crates/pulldown-cmark)** - Markdown parsing
- **[colored](https://crates.io/crates/colored)** - Terminal color output
- **[chrono](https://crates.io/crates/chrono)** - Date and time handling

## 📈 Roadmap

- [x] Core task management
- [x] Enhanced filtering and search
- [x] Multi-project workspace system
- [x] Dependency management system
- [ ] Enhanced UI with color coding
- [ ] Bulk operations support
- [ ] Task templates
- [ ] Time estimation and tracking
- [ ] Progress analytics and reporting
- [ ] Export capabilities (JSON, CSV, HTML)
- [ ] Plugin system
- [ ] Web dashboard interface

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by modern project management tools
- Built with the Rust community's excellent ecosystem
- Designed for developers who live in the terminal

---

**Happy project planning!** 🎯
