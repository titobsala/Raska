# Rask User Guide 📚

Welcome to the comprehensive Rask user guide! This document covers all features, commands, and usage patterns for the advanced CLI project planner.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Core Commands](#core-commands)
3. [Advanced Task Management](#advanced-task-management)
4. [Roadmap Phases System](#roadmap-phases-system)
5. [Filtering & Search](#filtering--search)
6. [Project Management](#project-management)
7. [Dependency Management](#dependency-management)
8. [Configuration System](#configuration-system)
9. [Bulk Operations](#bulk-operations)
10. [Export Capabilities](#export-capabilities)
11. [Usage Examples](#usage-examples)
12. [Terminal UI Features](#terminal-ui-features)
13. [Tips & Best Practices](#tips--best-practices)

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
- Assigns all tasks to MVP phase by default

### `rask show`
Display the current project status with beautiful formatting.

```bash
rask show  # Shows full project overview
```

**Features:**
- Progress bar with completion percentage
- Color-coded task priorities
- Phase indicators with emojis
- Dependency indicators
- Project statistics
- Ready vs blocked task counts
- Phase overview summary

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
- Maintains phase information

### `rask add <description> [OPTIONS]`
Add a new task with rich metadata.

```bash
# Basic task
rask add "Implement user authentication"

# Advanced task with metadata and phase
rask add "Deploy to production" \
  --tag deployment,critical \
  --priority high \
  --phase release \
  --depends-on 8,9,10 \
  --note "Requires database migration and SSL certificates"
```

**Options:**
- `--tag <TAGS>`: Comma-separated tags for categorization
- `--priority <LEVEL>`: low, medium, high, critical
- `--phase <PHASE>`: mvp, beta, release, future, backlog
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
- Phase assignment with emoji
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

## Roadmap Phases System

**NEW**: Rask includes a comprehensive phases system to organize tasks by development lifecycle stages.

### Available Phases

- **🚀 MVP** - Core features for minimum viable product
- **🧪 Beta** - Features for beta release and testing
- **🎯 Release** - Features for production release
- **🔮 Future** - Future enhancements and improvements
- **💡 Backlog** - Ideas and backlog items for consideration

### Phase Commands

#### `rask phase overview`
Show comprehensive phase statistics and recommendations.

```bash
rask phase overview
```

**Displays:**
- Task distribution across all phases
- Completion percentages for each phase
- Smart recommendations for focus areas
- Overall project progress
- Phase-specific insights

#### `rask phase list`
List all available phases with descriptions and task counts.

```bash
rask phase list
```

#### `rask phase show <phase>`
Display all tasks in a specific phase.

```bash
rask phase show mvp
rask phase show beta
rask phase show release
```

**Features:**
- Filtered view of phase-specific tasks
- Shows task details, priorities, and dependencies
- Displays completion status within the phase
- Beautiful formatting with phase emoji

#### `rask phase set <task_id> <phase>`
Set the phase for an individual task.

```bash
rask phase set 15 beta
rask phase set 23 future
```

**Validation:**
- Ensures phase exists
- Updates task metadata
- Maintains consistency across the project

### Phase Integration

Phases are integrated throughout Rask:

#### Adding Tasks with Phases
```bash
# Specify phase when creating tasks
rask add "Implement advanced analytics" --phase future --priority medium

# Default to MVP if no phase specified
rask add "Fix login bug" --priority critical
```

#### Filtering by Phase
```bash
# List tasks in specific phase
rask list --phase mvp

# Export phase-specific data
rask export json --phase beta --pretty

# Combine with other filters
rask list --phase release --priority high
```

#### Bulk Phase Operations
```bash
# Set multiple tasks to same phase
rask bulk set-phase 10,11,12,13 beta

# Move completed MVP tasks to release
rask bulk set-phase 1,2,3 release
```

### Phase Workflow Examples

#### Sprint Planning
```bash
# View current MVP progress
rask phase show mvp

# Move ready tasks to current sprint (beta)
rask bulk set-phase 15,16,17 beta

# Check what's ready for release
rask phase show release
```

#### Release Management
```bash
# Get release overview
rask phase overview

# Export release-ready tasks
rask export html --phase release -o release_tasks.html

# Move completed beta features to release
rask bulk set-phase 20,21,22 release
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

# Filter by phase
rask list --phase mvp,beta

# Search by text
rask list --search "authentication"

# Combine filters
rask list --tag backend --priority high --phase beta --status pending

# Show detailed view with metadata
rask list --detailed
```

**Filter Options:**
- `--tag <TAGS>`: Filter by comma-separated tags
- `--priority <LEVELS>`: Filter by priority levels
- `--phase <PHASES>`: Filter by development phases
- `--status <STATUS>`: Filter by pending/completed
- `--search <QUERY>`: Full-text search in descriptions and notes
- `--detailed`: Show full task metadata including notes and dependencies

### Advanced Search Examples

```bash
# Find all urgent backend tasks in MVP phase
rask list --tag backend,urgent --priority high,critical --phase mvp

# Search for authentication-related tasks
rask list --search "auth"

# Find all beta tasks with notes
rask list --phase beta --detailed | grep -A5 "💭"

# Show only ready-to-start tasks in current phase
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
- Phase distribution for each project

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
- Phase information
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
rask config set behavior.default_phase beta

# Project-specific settings
rask config set ui.show_completed false --project
```

### `rask config get <key>`
Get a specific configuration value.

```bash
rask config get ui.color_scheme
rask config get behavior.default_phase
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
- `ui.show_phase_emojis`: true/false

#### Behavior Settings
- `behavior.default_priority`: low, medium, high, critical
- `behavior.default_phase`: mvp, beta, release, future, backlog
- `behavior.default_project`: project name
- `behavior.auto_archive`: true/false
- `behavior.confirm_destructive`: true/false

#### Export Settings
- `export.default_format`: json, csv, html
- `export.include_completed`: true/false
- `export.include_phases`: true/false
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

### `rask bulk set-phase <ids> <phase>`
**NEW**: Set phase for multiple tasks.

```bash
rask bulk set-phase 11,12,13 beta
rask bulk set-phase 20,21,22 release
```

**Features:**
- Validates phase exists
- Updates all specified tasks
- Shows success/failure for each task
- Maintains project consistency

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

# Pretty-printed JSON with phases
rask export json --pretty

# Export to file
rask export json --pretty -o roadmap.json

# Include completed tasks
rask export json --include-completed --pretty

# Filter by phase
rask export json --phase mvp,beta --pretty
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
    },
    "phases": {
      "mvp": { "completed": 15, "total": 20, "percentage": 75.0 },
      "beta": { "completed": 5, "total": 10, "percentage": 50.0 },
      "release": { "completed": 3, "total": 8, "percentage": 37.5 },
      "future": { "completed": 2, "total": 2, "percentage": 100.0 },
      "backlog": { "completed": 0, "total": 0, "percentage": 0.0 }
    }
  },
  "tasks": [
    {
      "id": 1,
      "description": "Implement authentication",
      "completed": true,
      "priority": "High",
      "phase": "mvp",
      "tags": ["backend", "security"],
      "dependencies": [],
      "notes": "OAuth integration required",
      "created_at": "2025-01-15T10:30:00Z",
      "completed_at": "2025-01-16T14:20:00Z"
    }
  ]
}
```

### CSV Export

```bash
# Export to CSV
rask export csv -o tasks.csv

# Filter by priority and phase
rask export csv --priority high,critical --phase mvp,beta -o urgent_tasks.csv
```

**CSV Columns:**
- ID, Description, Status, Priority, Phase
- Tags, Notes, Dependencies
- Created At, Completed At

### HTML Export

```bash
# Create beautiful HTML report
rask export html -o roadmap.html

# Filter by phase
rask export html --phase release -o release_tasks.html

# Include completed tasks with phase information
rask export html --include-completed -o full_roadmap.html
```

**HTML Features:**
- Responsive design
- Progress bars and statistics
- Phase-based organization
- Color-coded priorities
- Interactive hover effects
- Professional styling
- Phase overview dashboard

### Export Filtering

All export formats support comprehensive filtering:

```bash
# Export only pending tasks (default)
rask export json

# Include completed tasks
rask export json --include-completed

# Filter by phases
rask export html --phase mvp,beta

# Filter by tags
rask export csv --tag backend,urgent

# Filter by priority
rask export json --priority critical,high

# Combine filters
rask export html --phase release --priority high --tag deployment --include-completed
```

## Usage Examples

### Scenario 1: Web Development Project with Phases

```bash
# Initialize from roadmap
rask init web-project.md

# Add MVP tasks
rask add "Set up database" --tag backend,infrastructure --priority high --phase mvp
rask add "Create user model" --tag backend,database --phase mvp --depends-on 1
rask add "Basic authentication" --tag backend,security --phase mvp --depends-on 2

# Add Beta features
rask add "Advanced user profiles" --tag backend,features --phase beta --depends-on 3
rask add "Social login integration" --tag backend,auth --phase beta --depends-on 3

# Add Release features
rask add "Performance optimization" --tag backend,performance --phase release --depends-on 4,5
rask add "Production deployment" --tag deployment,infrastructure --phase release --depends-on 6

# Check phase overview
rask phase overview

# Work on MVP first
rask phase show mvp
rask complete 1  # Database setup
rask complete 2  # User model (now unblocked)

# Export MVP progress
rask export html --phase mvp -o mvp_progress.html
```

### Scenario 2: Multi-Project with Phase Management

```bash
# Create multiple projects
rask project create backend-api --description "REST API development"
rask project create mobile-app --description "React Native mobile app"

# Configure backend project
rask project switch backend-api
rask config set behavior.default_phase mvp --project
rask add "Core API endpoints" --priority critical --phase mvp
rask add "Authentication system" --priority high --phase mvp
rask add "Advanced features" --priority medium --phase beta

# Configure mobile project
rask project switch mobile-app
rask config set behavior.default_phase beta --project
rask add "Login screen" --priority high --phase mvp
rask add "Dashboard UI" --priority medium --phase beta
rask add "Push notifications" --priority low --phase future

# Bulk phase operations
rask bulk set-phase 1,2 mvp
rask bulk set-phase 3,4 beta

# Export phase-specific reports
rask project switch backend-api
rask export json --phase mvp --pretty -o backend_mvp.json

rask project switch mobile-app
rask export html --phase beta -o mobile_beta.html
```

### Scenario 3: Complex Dependency and Phase Management

```bash
# Add tasks with complex dependencies across phases
rask add "System architecture" --tag architecture --priority critical --phase mvp
rask add "Database design" --tag database --phase mvp --depends-on 1
rask add "API framework setup" --tag backend --phase mvp --depends-on 1
rask add "Core endpoints" --tag backend --phase mvp --depends-on 2,3
rask add "Authentication middleware" --tag security --phase beta --depends-on 4
rask add "Advanced features" --tag features --phase beta --depends-on 5
rask add "Performance testing" --tag testing --phase release --depends-on 6
rask add "Production deployment" --tag deployment --phase release --depends-on 7

# Analyze dependencies across phases
rask dependencies
rask dependencies --task-id 8

# Check what's ready in each phase
rask phase show mvp
rask dependencies --ready

# Use bulk operations for sprint planning
rask bulk set-priority 1,2,3,4 critical
rask bulk add-tags 1,2,3,4 sprint-1

# Export comprehensive analysis
rask export json --include-completed --pretty -o full_analysis.json
```

### Scenario 4: Team Collaboration with Phases

```bash
# Set up team configuration
rask config set behavior.default_phase mvp
rask config set behavior.confirm_destructive true
rask config set export.include_phases true

# Create detailed tasks with phase planning
rask add "Payment system integration" \
  --tag backend,payments,critical \
  --priority critical \
  --phase beta \
  --note "Integrate with Stripe API. Requires PCI compliance review."

# Phase-based team planning
rask phase overview  # Get overall status
rask phase show mvp  # Current sprint focus
rask phase show beta # Next sprint planning

# Generate team reports by phase
rask export html --phase mvp -o current_sprint.html
rask export html --phase beta -o next_sprint.html
rask export csv --phase release -o release_planning.csv

# Bulk update after team meeting
rask bulk set-priority 10,11,12,13 high
rask bulk set-phase 14,15,16 beta
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

### Phase Indicators
- **🚀 MVP**: Core features for minimum viable product
- **🧪 Beta**: Features for beta release and testing
- **🎯 Release**: Features for production release
- **🔮 Future**: Future enhancements and improvements
- **💡 Backlog**: Ideas and backlog items

### Progress Tracking
- **Progress bars**: Visual completion percentage
- **Phase statistics**: Per-phase completion tracking
- **Statistics**: Completed vs total task counts
- **Ready indicators**: Tasks available to start
- **Dependency trees**: ASCII art visualization

### Smart Formatting
- **Responsive layout**: Adapts to terminal width
- **Color coding**: Priority and status indicators
- **Project context**: Always shows current project
- **Phase context**: Clear phase organization
- **Hover effects**: Interactive elements where supported

### Information Density
- **Compact view**: Essential information only
- **Detailed view**: Full metadata display
- **Filtered views**: Show only relevant tasks
- **Phase views**: Organized by development stage
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

2. **Plan phases strategically:**
   ```bash
   # Good: logical phase progression
   rask add "Core user system" --phase mvp --priority critical
   rask add "Advanced user features" --phase beta --priority high
   rask add "User analytics" --phase release --priority medium
   rask add "AI recommendations" --phase future --priority low
   
   # Use backlog for ideas
   rask add "Voice interface" --phase backlog --priority low
   ```

3. **Set up logical dependencies:**
   ```bash
   # Good: logical flow across phases
   rask add "Database schema" --phase mvp --priority high
   rask add "User authentication" --phase mvp --depends-on 1
   rask add "Social login" --phase beta --depends-on 2
   
   # Avoid: cross-phase dependencies that block progress
   ```

### Phase Management

1. **Start with MVP focus:**
   ```bash
   # Focus on core functionality first
   rask phase show mvp
   rask list --phase mvp --priority critical,high
   ```

2. **Use phase overview for planning:**
   ```bash
   # Regular phase reviews
   rask phase overview
   
   # Move completed MVP tasks to next phase
   rask bulk set-phase 1,2,3 beta
   ```

3. **Export phase-specific reports:**
   ```bash
   # Sprint planning
   rask export html --phase mvp -o current_sprint.html
   
   # Release planning
   rask export csv --phase release -o release_tasks.csv
   ```

### Workflow Optimization

1. **Start each day with phase and dependency analysis:**
   ```bash
   rask phase overview
   rask dependencies --ready
   rask phase show mvp  # or current focus phase
   ```

2. **Use bulk operations for phase transitions:**
   ```bash
   # Move completed MVP features to beta
   rask bulk set-phase 15,16,17 beta
   
   # Promote beta features to release
   rask bulk set-phase 20,21,22 release
   ```

3. **Regular progress exports:**
   ```bash
   # Weekly team report with phases
   rask export html --include-completed -o weekly_report.html
   
   # Phase-specific burndown data
   rask export csv --phase beta -o beta_progress.csv
   ```

### Configuration Management

1. **Set up phase defaults:**
   ```bash
   rask config set behavior.default_phase mvp
   rask config set behavior.default_priority medium
   rask config set export.include_phases true
   ```

2. **Project-specific phase settings:**
   ```bash
   # For experimental projects
   rask config set behavior.default_phase future --project
   
   # For production projects
   rask config set behavior.default_phase mvp --project
   rask config set behavior.confirm_destructive true --project
   ```

### Maintenance

1. **Regular phase validation:**
   ```bash
   rask dependencies --validate
   rask phase overview
   ```

2. **Clean up phase organization:**
   ```bash
   # Review and reorganize phases
   rask phase list
   rask bulk set-phase 25,26,27 backlog  # Move low-priority items
   ```

3. **Export comprehensive backups:**
   ```bash
   # Full project backup with phases
   rask export json --include-completed --pretty -o backup_$(date +%Y%m%d).json
   ```

### Advanced Phase Workflows

1. **Release management:**
   ```bash
   # Check release readiness
   rask phase show release
   rask dependencies --blocked
   
   # Prepare release report
   rask export html --phase release --include-completed -o release_report.html
   ```

2. **Sprint planning with phases:**
   ```bash
   # Plan next sprint from beta phase
   rask phase show beta
   rask bulk set-priority 30,31,32 high  # Sprint priorities
   rask bulk add-tags 30,31,32 sprint-5  # Sprint tracking
   ```

3. **Long-term roadmap planning:**
   ```bash
   # Review future and backlog
   rask phase show future
   rask phase show backlog
   
   # Promote promising ideas
   rask bulk set-phase 40,41 future
   ```

---

This user guide covers all the features and capabilities of Rask, including the new phases system. For quick reference, see the main [README](README.md). For issues or feature requests, please visit the [GitHub repository](https://github.com/tito-sala/rask).

**Happy project planning with phases!** 🎯 