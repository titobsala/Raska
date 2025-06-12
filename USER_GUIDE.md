# Rask User Guide 📚

Welcome to the comprehensive Rask user guide! This document covers all features, commands, and usage patterns for the advanced CLI project planner.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Core Commands](#core-commands)
3. [Advanced Task Management](#advanced-task-management)
4. [Filtering & Search](#filtering--search)
5. [Project Management](#project-management)
6. [Dependency Management](#dependency-management)
7. [Configuration System](#configuration-system)
8. [Bulk Operations](#bulk-operations)
9. [Export Capabilities](#export-capabilities)
10. [Usage Examples](#usage-examples)
11. [Terminal UI Features](#terminal-ui-features)
12. [Tips & Best Practices](#tips--best-practices)

## Getting Started

### Installation

```bash
# Clone the repository
git clone https://github.com/tito-sala/rask.git
cd rask

# Build and install
cargo install --path .
```

### Your First Project

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

## Core Commands

### `rask init <file.md>`
Initialize a new project from a Markdown file.

```bash
rask init project-roadmap.md
```

**What it does:**
- Parses your Markdown file
- Creates internal state tracking
- Maintains sync with original file
- Sets up project workspace

### `rask show`
Display the current project status with beautiful formatting.

```bash
rask show  # Shows full project overview
```

**Features:**
- Progress bar with completion percentage
- Color-coded task priorities
- Dependency indicators
- Project statistics
- Ready vs blocked task counts

### `rask complete <task_id>`
Mark a task as completed (respects dependencies).

```bash
rask complete 5
```

**Smart features:**
- Validates dependencies before completion
- Shows newly unblocked tasks
- Updates progress statistics
- Syncs changes to Markdown file

### `rask add <description> [OPTIONS]`
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
- `--tag <TAGS>`: Comma-separated tags for categorization
- `--priority <LEVEL>`: low, medium, high, critical
- `--note <TEXT>`: Detailed description or context
- `--depends-on <IDS>`: Comma-separated task IDs this task depends on

### `rask view <task_id>`
View detailed information about a specific task.

```bash
rask view 15
```

**Shows:**
- Complete task metadata
- All tags and priority
- Notes and creation date
- Dependencies and reverse dependencies
- Blocking/ready status

### `rask edit <task_id> <new_description>`
Edit a task's description.

```bash
rask edit 3 "Updated task description"
```

### `rask remove <task_id>`
Remove a task from the project.

```bash
rask remove 7
```

**Safety features:**
- Validates no other tasks depend on this one
- Updates dependency chains
- Renumbers remaining tasks

### `rask reset [task_id]`
Reset task(s) to pending status.

```bash
rask reset 5    # Reset specific task
rask reset      # Reset all tasks
```

## Advanced Task Management

### Task Priorities

Rask supports four priority levels with visual indicators:

- **🔥 Critical**: Urgent, blocking issues
- **⬆️ High**: Important tasks that should be done soon
- **▶️ Medium**: Standard priority (default)
- **⬇️ Low**: Nice-to-have features

```bash
# Set priority when adding
rask add "Fix security vulnerability" --priority critical

# View tasks by priority
rask list --priority critical,high
```

### Task Tags

Tags help categorize and organize your tasks:

```bash
# Add multiple tags
rask add "Implement OAuth" --tag backend,security,auth

# Filter by tags
rask list --tag backend
rask list --tag security,urgent  # Tasks with either tag
```

**Tag best practices:**
- Use consistent naming (e.g., `backend`, `frontend`, `testing`)
- Include context tags (`urgent`, `blocked`, `research`)
- Add feature tags (`auth`, `payments`, `dashboard`)

### Task Dependencies

Create sophisticated task relationships:

```bash
# Task that depends on others
rask add "Deploy to production" --depends-on 5,8,12

# View dependency tree
rask dependencies --task-id 15

# Check what's ready to work on
rask dependencies --ready
```

## Filtering & Search

### `rask list [OPTIONS]`
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

# Combine filters
rask list --tag backend --priority high --status pending

# Show detailed view with metadata
rask list --detailed
```

**Filter Options:**
- `--tag <TAGS>`: Filter by comma-separated tags
- `--priority <LEVELS>`: Filter by priority levels
- `--status <STATUS>`: Filter by pending/completed
- `--search <QUERY>`: Full-text search in descriptions and notes
- `--detailed`: Show full task metadata including notes and dependencies

### Advanced Search Examples

```bash
# Find all urgent backend tasks
rask list --tag backend,urgent --priority high,critical

# Search for authentication-related tasks
rask list --search "auth"

# Find all tasks with notes
rask list --detailed | grep -A5 "💭"

# Show only ready-to-start tasks
rask dependencies --ready
```

## Project Management

Rask supports multiple projects with isolated state and easy switching.

### `rask project create <name> [--description <desc>]`
Create a new project workspace.

```bash
rask project create web-app --description "Main web application project"
rask project create mobile-app --description "React Native mobile app"
```

### `rask project list`
List all available projects.

```bash
rask project list
```

**Output shows:**
- Project names and descriptions
- Current active project (marked with ★)
- Task counts and progress for each project

### `rask project switch <name>`
Switch to a different project.

```bash
rask project switch mobile-app
```

**What happens:**
- Saves current project state
- Loads target project state
- Updates current project indicator
- All subsequent commands operate on new project

### `rask project delete <name> [--force]`
Delete a project (requires confirmation unless --force).

```bash
rask project delete old-project --force
```

## Dependency Management

Rask provides sophisticated dependency management with validation and visualization.

### `rask dependencies`
Show dependency analysis overview.

```bash
rask dependencies
```

**Shows:**
- Total dependency relationships
- Circular dependency detection
- Ready vs blocked task counts
- Dependency health statistics

### `rask dependencies --validate`
Validate all dependencies for issues.

```bash
rask dependencies --validate
```

**Checks for:**
- Missing dependency targets
- Circular dependency loops
- Orphaned dependencies
- Impossible completion states

### `rask dependencies --task-id <id>`
Show dependency tree for a specific task.

```bash
rask dependencies --task-id 15
```

**Visualizes:**
- Complete dependency chain
- Task completion status
- Blocking relationships
- ASCII art tree structure

### `rask dependencies --ready`
Show tasks ready to be started.

```bash
rask dependencies --ready
```

### `rask dependencies --blocked`
Show tasks blocked by incomplete dependencies.

```bash
rask dependencies --blocked
```

## Configuration System

Rask includes a comprehensive configuration system for customizing behavior.

### `rask config show [section]`
Display current configuration.

```bash
rask config show           # Show all configuration
rask config show ui        # Show UI section only
rask config show behavior  # Show behavior settings
```

### `rask config set <key> <value> [--project]`
Set configuration values.

```bash
# User-wide settings
rask config set ui.color_scheme dark
rask config set behavior.default_priority high

# Project-specific settings
rask config set ui.show_completed false --project
```

### `rask config get <key>`
Get a specific configuration value.

```bash
rask config get ui.color_scheme
```

### `rask config edit [--project]`
Edit configuration in your default editor.

```bash
rask config edit           # Edit user config
rask config edit --project # Edit project config
```

### Configuration Sections

#### UI Settings
- `ui.color_scheme`: light, dark, auto
- `ui.show_completed`: true/false
- `ui.compact_view`: true/false
- `ui.show_task_ids`: true/false

#### Behavior Settings
- `behavior.default_priority`: low, medium, high, critical
- `behavior.default_project`: project name
- `behavior.auto_archive`: true/false
- `behavior.confirm_destructive`: true/false

#### Export Settings
- `export.default_format`: json, csv, html
- `export.include_completed`: true/false
- `export.pretty_json`: true/false

## Bulk Operations

Perform operations on multiple tasks efficiently.

### `rask bulk complete <ids>`
Complete multiple tasks at once.

```bash
rask bulk complete 1,2,3,4
```

**Features:**
- Validates dependencies for all tasks
- Shows which tasks were completed
- Reports any failures with reasons
- Updates progress statistics

### `rask bulk add-tags <ids> <tags>`
Add tags to multiple tasks.

```bash
rask bulk add-tags 5,6,7 urgent,backend
```

### `rask bulk remove-tags <ids> <tags>`
Remove tags from multiple tasks.

```bash
rask bulk remove-tags 5,6,7 old-tag,deprecated
```

### `rask bulk set-priority <ids> <priority>`
Set priority for multiple tasks.

```bash
rask bulk set-priority 8,9,10 high
```

### `rask bulk reset <ids>`
Reset multiple tasks to pending.

```bash
rask bulk reset 11,12,13
```

### `rask bulk remove <ids> [--force]`
Remove multiple tasks.

```bash
rask bulk remove 14,15,16 --force
```

## Export Capabilities

Export your roadmaps in various formats for sharing and reporting.

### `rask export <format> [OPTIONS]`
Export roadmap to different formats.

**Formats:**
- `json`: Structured data format
- `csv`: Spreadsheet-compatible format
- `html`: Beautiful web page

### JSON Export

```bash
# Basic JSON export
rask export json

# Pretty-printed JSON
rask export json --pretty

# Export to file
rask export json --pretty -o roadmap.json

# Include completed tasks
rask export json --include-completed --pretty
```

**JSON Structure:**
```json
{
  "roadmap": {
    "title": "Project Name",
    "exported_at": "2025-06-12T23:17:30Z",
    "progress": {
      "completed": 25,
      "total": 40,
      "percentage": 62.5
    }
  },
  "tasks": [...]
}
```

### CSV Export

```bash
# Export to CSV
rask export csv -o tasks.csv

# Filter by priority
rask export csv --priority high,critical -o urgent_tasks.csv
```

**CSV Columns:**
- ID, Description, Status, Priority
- Tags, Notes, Dependencies
- Created At, Completed At

### HTML Export

```bash
# Create beautiful HTML report
rask export html -o roadmap.html

# Filter by tags
rask export html --tags frontend,ui -o frontend_tasks.html
```

**HTML Features:**
- Responsive design
- Progress bars and statistics
- Color-coded priorities
- Interactive hover effects
- Professional styling

### Export Filtering

All export formats support filtering:

```bash
# Export only pending tasks
rask export json  # (default behavior)

# Include completed tasks
rask export json --include-completed

# Filter by tags
rask export html --tags backend,urgent

# Filter by priority
rask export csv --priority critical,high

# Combine filters
rask export json --tags testing --priority high --include-completed
```

## Usage Examples

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
rask dependencies --task-id 4

# Work on ready tasks
rask complete 1  # Database setup
rask complete 2  # User model (now unblocked)

# Export progress report
rask export html --include-completed -o progress_report.html
```

### Scenario 2: Multi-Project Workflow

```bash
# Create multiple projects
rask project create backend-api --description "REST API development"
rask project create mobile-app --description "React Native mobile app"
rask project create devops --description "Infrastructure and deployment"

# Configure each project
rask project switch backend-api
rask config set behavior.default_priority high --project
rask add "Implement OAuth" --tag auth --priority critical

rask project switch mobile-app
rask config set ui.color_scheme dark --project
rask add "Create login screen" --tag ui

# Bulk operations across projects
rask project switch devops
rask bulk add-tags 1,2,3,4 infrastructure

# Export reports for each project
rask project switch backend-api
rask export json --pretty -o backend_status.json

rask project switch mobile-app
rask export html -o mobile_progress.html
```

### Scenario 3: Complex Dependency Management

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
rask dependencies --task-id 5

# See what's blocking progress
rask dependencies --blocked

# Use bulk operations to manage priorities
rask bulk set-priority 1,2,3 critical

# Export dependency analysis
rask export json --include-completed --pretty -o dependency_analysis.json
```

### Scenario 4: Team Collaboration

```bash
# Set up shared configuration
rask config set behavior.confirm_destructive true
rask config set export.default_format html
rask config set export.include_completed true

# Create detailed task documentation
rask add "Implement payment system" \
  --tag backend,payments,critical \
  --priority critical \
  --note "Integrate with Stripe API. Requires PCI compliance review and security audit."

# Use detailed view for task planning
rask view 15

# Generate team reports
rask export html --priority critical,high -o critical_tasks.html
rask export csv --tags backend -o backend_tasks.csv

# Bulk update task priorities after sprint planning
rask bulk set-priority 10,11,12,13 high
```

## Terminal UI Features

Rask provides a beautiful, intuitive terminal interface with:

### Visual Indicators
- **🔥 Critical**: Red, urgent tasks
- **⬆️ High**: Orange, important tasks  
- **▶️ Medium**: Blue, standard tasks
- **⬇️ Low**: Gray, low priority tasks
- **✅ Completed**: Green checkmarks
- **🔒 Blocked**: Tasks waiting on dependencies

### Progress Tracking
- **Progress bars**: Visual completion percentage
- **Statistics**: Completed vs total task counts
- **Ready indicators**: Tasks available to start
- **Dependency trees**: ASCII art visualization

### Smart Formatting
- **Responsive layout**: Adapts to terminal width
- **Color coding**: Priority and status indicators
- **Project context**: Always shows current project
- **Hover effects**: Interactive elements where supported

### Information Density
- **Compact view**: Essential information only
- **Detailed view**: Full metadata display
- **Filtered views**: Show only relevant tasks
- **Search highlighting**: Emphasize search terms

## Tips & Best Practices

### Project Organization

1. **Use consistent tag naming:**
   ```bash
   # Good: consistent, descriptive
   --tag backend,api,auth
   --tag frontend,ui,dashboard
   --tag testing,integration
   
   # Avoid: inconsistent, vague
   --tag back-end,API,Authentication
   --tag front,user-interface
   ```

2. **Set up logical dependencies:**
   ```bash
   # Good: logical flow
   rask add "Design database schema" --priority high
   rask add "Implement user model" --depends-on 1
   rask add "Create authentication" --depends-on 2
   
   # Avoid: circular or illogical dependencies
   ```

3. **Use priorities effectively:**
   - **Critical**: Blockers, security issues, production bugs
   - **High**: Important features, sprint goals
   - **Medium**: Standard development tasks
   - **Low**: Nice-to-have features, technical debt

### Workflow Optimization

1. **Start each day with dependency analysis:**
   ```bash
   rask dependencies --ready
   rask dependencies --blocked
   ```

2. **Use bulk operations for sprint planning:**
   ```bash
   # Mark sprint tasks as high priority
   rask bulk set-priority 15,16,17,18,19 high
   
   # Add sprint tag to current iteration
   rask bulk add-tags 15,16,17,18,19 sprint-3
   ```

3. **Regular progress exports:**
   ```bash
   # Weekly team report
   rask export html --include-completed -o weekly_report.html
   
   # Sprint burndown data
   rask export csv --tags current-sprint -o sprint_data.csv
   ```

### Configuration Management

1. **Set up user defaults:**
   ```bash
   rask config set behavior.default_priority medium
   rask config set ui.color_scheme auto
   rask config set export.pretty_json true
   ```

2. **Project-specific settings:**
   ```bash
   # For critical projects
   rask config set behavior.confirm_destructive true --project
   
   # For large teams
   rask config set ui.show_task_ids true --project
   ```

### Maintenance

1. **Regular dependency validation:**
   ```bash
   rask dependencies --validate
   ```

2. **Clean up completed tasks:**
   ```bash
   # Archive old completed tasks
   rask list --status completed --detailed
   ```

3. **Export backups:**
   ```bash
   # Regular project backups
   rask export json --include-completed --pretty -o backup_$(date +%Y%m%d).json
   ```

---

This user guide covers all the features and capabilities of Rask. For quick reference, see the main [README](README.md). For issues or feature requests, please visit the [GitHub repository](https://github.com/tito-sala/rask).

**Happy project planning!** 🎯 