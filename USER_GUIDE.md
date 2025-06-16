# Rask User Guide 📚

Welcome to the comprehensive Rask user guide! This document covers all features, commands, and usage patterns for the advanced CLI project planner.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Core Commands](#core-commands)
3. [Advanced Task Management](#advanced-task-management)
4. [Roadmap Phases System](#roadmap-phases-system)
5. [Timeline Visualization & Pagination](#timeline-visualization--pagination)
6. [Time Estimation and Tracking](#time-estimation-and-tracking)
7. [Filtering & Search](#filtering--search)
8. [Project Management](#project-management)
9. [Dependency Management](#dependency-management)
10. [Configuration System](#configuration-system)
11. [Bulk Operations](#bulk-operations)
12. [Export Capabilities](#export-capabilities)
13. [Usage Examples](#usage-examples)
14. [Terminal UI Features](#terminal-ui-features)
15. [Tips & Best Practices](#tips--best-practices)

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

**⚠️ Important Formatting Guidelines:**
- Use simple task descriptions without complex nested lists
- Avoid bullet points or numbered lists in Notes sections
- Use plain text descriptions or comma-separated items instead
- This prevents parsing issues and ensures clean task import

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

# Advanced task with metadata, phase, and time estimation
rask add "Deploy to production" \
  --tag deployment,critical \
  --priority high \
  --phase release \
  --depends-on 8,9,10 \
  --note "Requires database migration and SSL certificates" \
  --estimated-hours 3.5
```

**Options:**
- `--tag <TAGS>`: Comma-separated tags for categorization
- `--priority <LEVEL>`: low, medium, high, critical
- `--phase <PHASE>`: mvp, beta, release, future, backlog
- `--note <TEXT>`: Detailed description or context
- `--depends-on <IDS>`: Comma-separated task IDs this task depends on
- `--estimated-hours <HOURS>`: Time estimation in hours (e.g., 2.5)

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

## Timeline Visualization & Pagination

**NEW**: Rask includes a powerful timeline visualization system that displays your project phases horizontally with pagination support for managing large numbers of phases.

### Timeline Overview

The timeline view provides a horizontal layout showing all your project phases with their tasks, progress bars, and dependencies in a clean, organized format.

#### `rask timeline [OPTIONS]`
Display project timeline with horizontal phase layout.

```bash
# Basic timeline view (shows first 5 phases)
rask timeline

# Navigate between pages
rask timeline --page 2
rask timeline --page 1

# Customize page size
rask timeline --page-size 10    # Show all phases on one page
rask timeline --page-size 3     # Show only 3 phases per page

# Compact view for more information
rask timeline --compact

# Show only phases with tasks
rask timeline --active-only

# Combine options
rask timeline --page 2 --compact --active-only
```

### Timeline Features

#### **Pagination System**
- **Default**: 5 phases per page for optimal readability
- **Navigation**: Clear page indicators and navigation tips
- **Flexible**: Customizable page sizes from 1 to all phases
- **Smart**: No pagination info when all phases fit on one page

#### **Visual Layout**
- **Horizontal progression**: Phases flow left to right showing development timeline
- **Progress bars**: Individual progress bars for each phase
- **Task boxes**: Visual task representation with status and priority icons
- **Dependency flow**: Shows phase relationships and dependencies

#### **Interactive Navigation**
- **Page indicators**: "📄 Page 1 of 2 (showing 5 of 10 phases)"
- **Navigation tips**: Shows previous/next page commands
- **Error handling**: Clear messages for invalid page numbers
- **Size suggestions**: Recommends page-size adjustments

### Timeline Options

#### **`--page <PAGE>`**
Navigate to specific page number.

```bash
rask timeline --page 1    # First page (default)
rask timeline --page 2    # Second page
rask timeline --page 3    # Third page (if exists)
```

**Features:**
- Default page is 1
- Shows error for invalid page numbers
- Provides navigation guidance

#### **`--page-size <SIZE>`**
Set number of phases to show per page.

```bash
rask timeline --page-size 3     # Show 3 phases per page
rask timeline --page-size 5     # Default: 5 phases per page
rask timeline --page-size 10    # Show all phases (if ≤10)
```

**Benefits:**
- Customize display density
- Adapt to terminal width
- Show all phases when needed

#### **`--compact`**
Use compact view with less detail per task.

```bash
rask timeline --compact
rask timeline --compact --page 2
```

**Compact features:**
- Shorter task descriptions
- Task IDs only (no descriptions)
- More tasks visible per phase
- Better for overview

#### **`--active-only`**
Show only phases that contain tasks.

```bash
rask timeline --active-only
rask timeline --active-only --page-size 8
```

**Benefits:**
- Hide empty phases
- Focus on active development
- Cleaner display

#### **`--detailed`**
Show detailed task information in timeline.

```bash
rask timeline --detailed
rask timeline --detailed --page 1
```

### Phase-Grouped Display

#### `rask show --group-by-phase`
Display tasks organized by phases with individual progress bars.

```bash
# Group all tasks by phase
rask show --group-by-phase

# Group with detailed task information
rask show --group-by-phase --detailed

# Collapse completed phases
rask show --group-by-phase --collapse-completed

# Combine options
rask show --group-by-phase --detailed --collapse-completed
```

**Features:**
- **Phase sections**: Each phase displayed as a separate section
- **Progress bars**: Individual progress tracking per phase
- **Phase statistics**: Completion percentages and task counts
- **Collapsible**: Hide completed phases to reduce clutter
- **Dynamic phases**: Shows your actual custom phases, not hardcoded ones

#### `rask show --phase <PHASE>`
Filter display to show only tasks from a specific phase.

```bash
# Show only MVP tasks
rask show --phase mvp

# Show beta phase with details
rask show --phase beta --detailed

# Show custom phase
rask show --phase "Architecture"
```

**Benefits:**
- **Focused view**: See only relevant tasks
- **Phase-specific stats**: Progress for that phase only
- **Custom phases**: Works with your custom phase names
- **Detailed context**: Full task information for the phase

### Timeline Workflow Examples

#### **Daily Development Review**
```bash
# Start day with timeline overview
rask timeline

# Focus on current phase
rask show --phase mvp

# Check what's ready to work on
rask dependencies --ready

# Navigate to next phase for planning
rask timeline --page 2
```

#### **Sprint Planning with Timeline**
```bash
# Review overall project timeline
rask timeline --active-only

# Focus on current sprint phase
rask show --phase beta --detailed

# Plan next sprint
rask timeline --page 2 --compact

# Export sprint plan
rask export html --phase beta -o sprint_plan.html
```

#### **Large Project Management**
```bash
# Overview of all phases (paginated)
rask timeline

# Navigate through phases
rask timeline --page 1  # Foundation phases
rask timeline --page 2  # Development phases  
rask timeline --page 3  # Release phases

# Adjust view for team meeting
rask timeline --page-size 8 --compact

# Focus on active development
rask timeline --active-only --detailed
```

#### **Phase Transition Management**
```bash
# Review completed phases (collapsed)
rask show --group-by-phase --collapse-completed

# Check current phase progress
rask show --phase mvp

# Plan next phase
rask show --phase beta --detailed

# Timeline view for stakeholder presentation
rask timeline --page-size 10
```

### Timeline Navigation Tips

#### **Efficient Navigation**
- Use `--page` to jump to specific sections of your roadmap
- Use `--page-size` to adjust information density
- Combine `--compact` with larger page sizes for overviews
- Use `--active-only` to focus on phases with work

#### **Display Optimization**
- Default 5 phases per page works well for most terminals
- Use `--page-size 3` for narrow terminals or detailed review
- Use `--page-size 10+` for wide terminals or presentations
- Combine `--compact` with larger page sizes for maximum information

#### **Team Collaboration**
- Share specific page views: `rask timeline --page 2`
- Use consistent page sizes for team meetings
- Export timeline views for documentation
- Use phase-specific views for focused discussions

### Timeline Integration

#### **With Phase Management**
```bash
# Timeline shows your actual custom phases
rask timeline  # Shows Foundation, Architecture, Export, etc.

# Not hardcoded MVP, Beta, Release phases
# Dynamic detection from your roadmap data
```

#### **With Filtering**
```bash
# Timeline respects active-only filtering
rask timeline --active-only

# Phase-grouped view shows all your phases
rask show --group-by-phase

# Phase-specific filtering
rask show --phase "Export" --detailed
```

#### **With Export**
```bash
# Export timeline data
rask export html --include-completed -o timeline_report.html

# Export specific phases shown in timeline
rask export json --phase "Foundation,Architecture" --pretty
```

## Time Estimation and Tracking

**NEW**: Rask includes comprehensive time estimation and tracking capabilities to help you plan better and analyze your productivity.

### Time Estimation

#### Adding Tasks with Time Estimates
```bash
# Add task with time estimation
rask add "Implement user authentication" --estimated-hours 4.5

# Combine with other metadata
rask add "Design API endpoints" \
  --tag backend,api \
  --priority high \
  --phase mvp \
  --estimated-hours 6.0 \
  --note "RESTful API with JWT authentication"
```

#### Benefits of Time Estimation
- **Better planning**: Understand scope and effort required
- **Resource allocation**: Plan sprints and iterations effectively
- **Progress tracking**: Compare estimates vs actual time
- **Learning**: Improve estimation accuracy over time

### Time Tracking

#### `rask start <task_id> [--description <desc>]`
Start time tracking for a specific task.

```bash
# Start tracking a task
rask start 5

# Start with work session description
rask start 5 --description "Implementing OAuth integration with Google"

# Start with detailed context
rask start 12 --description "Bug fixing: resolving race condition in user registration"
```

**Features:**
- Only one task can be tracked at a time
- Automatic session timestamp recording
- Optional work session descriptions
- Validates task exists before starting

#### `rask stop`
Stop the currently active time tracking session.

```bash
# Stop current session
rask stop
```

**What happens:**
- Records session end time
- Calculates session duration
- Updates total time tracked for the task
- Shows session summary
- Clears active tracking state

#### `rask time [task_id] [OPTIONS]`
View time tracking information and analysis.

```bash
# View time info for specific task
rask time 5

# View time summary for all tasks
rask time --summary

# View detailed session history
rask time --detailed

# Combine options
rask time --summary --detailed
```

**Options:**
- No arguments: Shows time info for all tasks with time data
- `<task_id>`: Shows detailed time info for specific task
- `--summary`: Shows project-wide time tracking statistics
- `--detailed`: Shows individual time sessions with timestamps

### Time Analysis Features

#### Variance Analysis
Rask automatically calculates time variance between estimates and actual time:

```bash
# View task with time variance
rask view 5
```

**Shows:**
- **Estimated time**: Original time estimate
- **Actual time**: Total time tracked
- **Variance**: Difference between estimate and actual
- **Efficiency**: Percentage (actual/estimated * 100)
- **Status**: Over/under estimated indicators

#### Session Management
```bash
# View active session
rask time

# View session history for task
rask time 5 --detailed

# Check overall tracking summary
rask time --summary
```

### Time Tracking Workflow Examples

#### Daily Work Session
```bash
# Start your work day
rask start 3 --description "Morning: implementing user registration"

# Take a break (stop tracking)
rask stop

# Resume after lunch
rask start 3 --description "Afternoon: adding email verification"

# End of day
rask stop

# Review time spent
rask time 3
```

#### Sprint Time Tracking
```bash
# Plan sprint with estimates
rask add "Feature A" --estimated-hours 8.0 --priority high --phase mvp
rask add "Feature B" --estimated-hours 12.0 --priority medium --phase mvp
rask add "Bug fixes" --estimated-hours 4.0 --priority high --phase mvp

# Track work during sprint
rask start 1 --description "Initial implementation of Feature A"
# ... work on Feature A ...
rask stop

rask start 3 --description "Critical bug fix in user authentication"
# ... fix bugs ...
rask stop

# Sprint review
rask time --summary
rask time --detailed
```

#### Time Estimation Improvement
```bash
# Add tasks with initial estimates
rask add "Database migration" --estimated-hours 2.0
rask add "API integration" --estimated-hours 6.0
rask add "UI components" --estimated-hours 4.0

# Track actual time
rask start 1 --description "Setting up migration scripts"
# ... work ...
rask stop

# Analyze variance to improve future estimates
rask time 1  # Shows: estimated 2.0h, actual 3.5h, 175% of estimate
```

### Time Tracking Integration

#### With Phases
```bash
# Add time estimates by phase
rask add "Core API" --phase mvp --estimated-hours 8.0
rask add "Advanced features" --phase beta --estimated-hours 12.0
rask add "Performance optimization" --phase release --estimated-hours 6.0

# Track time per phase
rask phase show mvp  # Shows estimated vs tracked time
rask time --summary  # Shows time breakdown by phase
```

#### With Dependencies
```bash
# Add dependent tasks with time estimates
rask add "Database schema" --estimated-hours 3.0 --phase mvp
rask add "API endpoints" --estimated-hours 5.0 --depends-on 1 --phase mvp
rask add "Frontend integration" --estimated-hours 4.0 --depends-on 2 --phase beta

# Track critical path timing
rask start 1 --description "Designing user tables"
rask stop
rask start 2 --description "Building REST API"
rask stop

# Analyze dependency timing impact
rask dependencies --task-id 3
rask time 1,2,3
```

#### With Export
```bash
# Export time tracking data
rask export json --include-completed --pretty -o time_report.json
rask export html --include-completed -o time_dashboard.html
rask export csv -o time_analysis.csv
```

### Time Tracking Best Practices

#### Estimation Guidelines
- **Start with ranges**: Use pessimistic estimates initially
- **Break down large tasks**: Estimate smaller chunks more accurately
- **Include buffer time**: Add 20-30% for unknowns and interruptions
- **Learn from variance**: Analyze over/under estimates to improve

#### Tracking Workflow
- **Track consistently**: Start/stop tracking for all significant work
- **Use descriptive sessions**: Add context to understand time spent
- **Take breaks**: Stop tracking during meetings, breaks, and context switches
- **Review regularly**: Analyze time data weekly or per sprint

#### Integration Tips
- **Combine with phases**: Track time spent per development phase
- **Use with priorities**: Focus time tracking on high-priority tasks
- **Export for reporting**: Generate time reports for stakeholders
- **Learn and adapt**: Use variance data to improve future estimates

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

### Scenario 1: Web Development Project with Phases and Time Tracking

```bash
# Initialize from roadmap
rask init web-project.md

# Add MVP tasks with time estimates
rask add "Set up database" --tag backend,infrastructure --priority high --phase mvp --estimated-hours 4.0
rask add "Create user model" --tag backend,database --phase mvp --depends-on 1 --estimated-hours 3.0
rask add "Basic authentication" --tag backend,security --phase mvp --depends-on 2 --estimated-hours 6.0

# Add Beta features with estimates
rask add "Advanced user profiles" --tag backend,features --phase beta --depends-on 3 --estimated-hours 8.0
rask add "Social login integration" --tag backend,auth --phase beta --depends-on 3 --estimated-hours 5.0

# Add Release features with estimates
rask add "Performance optimization" --tag backend,performance --phase release --depends-on 4,5 --estimated-hours 12.0
rask add "Production deployment" --tag deployment,infrastructure --phase release --depends-on 6 --estimated-hours 3.0

# Check phase overview with time estimates
rask phase overview

# Work on MVP first with time tracking
rask phase show mvp
rask start 1 --description "Setting up PostgreSQL database"
# ... work on database setup ...
rask stop

rask complete 1  # Database setup
rask start 2 --description "Designing user schema and models"
# ... work on user model ...
rask stop
rask complete 2  # User model (now unblocked)

# Analyze time variance
rask time 1  # Compare estimated vs actual time
rask time 2

# Export MVP progress with time data
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

### Scenario 4: Sprint Planning and Time Tracking

```bash
# Initialize sprint with time estimates
rask add "User authentication API" --tag backend,auth --priority critical --phase mvp --estimated-hours 8.0
rask add "Login/logout endpoints" --tag backend,api --priority high --phase mvp --estimated-hours 4.0 --depends-on 1
rask add "Password reset flow" --tag backend,auth --priority medium --phase mvp --estimated-hours 6.0 --depends-on 1
rask add "Frontend login form" --tag frontend,ui --priority high --phase mvp --estimated-hours 5.0 --depends-on 2

# Sprint planning analysis
rask phase overview  # See total estimated time
rask time --summary  # Sprint capacity planning

# Daily time tracking workflow
rask start 1 --description "Setting up JWT authentication middleware"
# ... work for 2 hours ...
rask stop

rask start 1 --description "Adding password hashing and validation"
# ... work for 3 hours ...
rask stop

rask complete 1  # Authentication API done

# Track dependent task
rask start 2 --description "Implementing login endpoint with JWT tokens"
# ... work for 4 hours ...
rask stop
rask complete 2

# Sprint review with time analysis
rask time --summary --detailed
rask export html --include-completed -o sprint_report.html

# Analyze estimation accuracy for next sprint
rask time 1  # 8.0h estimated, 5.2h actual - better than expected
rask time 2  # 4.0h estimated, 4.1h actual - accurate estimate
```

### Scenario 5: Team Collaboration with Phases and Time Tracking

```bash
# Set up team configuration with time tracking defaults
rask config set behavior.default_phase mvp
rask config set behavior.confirm_destructive true
rask config set export.include_phases true

# Create detailed tasks with phase planning and time estimates
rask add "Payment system integration" \
  --tag backend,payments,critical \
  --priority critical \
  --phase beta \
  --estimated-hours 16.0 \
  --note "Integrate with Stripe API. Requires PCI compliance review."

rask add "Payment UI components" \
  --tag frontend,payments \
  --priority high \
  --phase beta \
  --estimated-hours 8.0 \
  --depends-on 1

# Phase-based team planning with time estimates
rask phase overview  # Get overall status with time breakdown
rask phase show mvp  # Current sprint focus with time estimates
rask phase show beta # Next sprint planning with capacity

# Track team work across features
rask start 1 --description "Team member A: Stripe API integration"
# ... distributed work tracking ...
rask stop

# Generate comprehensive team reports with time data
rask export html --phase mvp --include-completed -o current_sprint_time.html
rask export html --phase beta -o next_sprint_planning.html
rask export csv --include-completed -o team_time_analysis.csv

# Team retrospective with time insights
rask time --summary  # Overall team productivity
rask time --detailed # Individual task time analysis

# Bulk update after team meeting with time learnings
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
- **⏱️ Time Tracking**: Active time tracking indicator
- **🕐 Active Session**: Currently tracking time on task
- **📊 Time Data**: Shows estimated vs actual time
- **📄 Pagination**: Page indicators for timeline navigation
- **🔗 Dependencies**: Phase dependency flow visualization

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
- **Timeline view**: Horizontal phase progression with pagination
- **Phase-grouped displays**: Individual progress bars per phase

### Smart Formatting
- **Responsive layout**: Adapts to terminal width
- **Color coding**: Priority and status indicators
- **Project context**: Always shows current project
- **Phase context**: Clear phase organization
- **Hover effects**: Interactive elements where supported

### Information Density
- **Compact view**: Essential information only
- **Detailed view**: Full metadata display including time data
- **Filtered views**: Show only relevant tasks
- **Phase views**: Organized by development stage with time estimates
- **Search highlighting**: Emphasize search terms
- **Time summaries**: Estimated vs actual time at task and phase levels
- **Active tracking**: Real-time session duration display
- **Paginated timeline**: Manageable display of large phase collections
- **Navigation tips**: Clear guidance for timeline navigation

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

3. **Markdown formatting best practices:**
   ```bash
   # Good: Simple, clean formatting
   - [ ] Implement user authentication #backend #security (Priority: High)
     Notes: Use OAuth 2.0 with JWT tokens, implement rate limiting, add session management
   
   # Avoid: Complex nested lists in Notes
   - [ ] Implement user authentication #backend #security (Priority: High)
     Notes: 
     - Use OAuth 2.0 with JWT tokens
     - Implement rate limiting  
     - Add session management
   ```

4. **Template usage guidelines:**
   ```bash
   # Use templates for consistency
   rask template use "Feature Implementation" "User authentication system"
   rask template use "Bug Fix" "Login form validation error"
   
   # Create custom templates for recurring patterns
   rask template create "Code Review" "Review pull request for [PR_NAME]" \
     --tags "review,quality" --priority high --phase mvp
   ```

5. **Set up logical dependencies:**
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

1. **Start each day with timeline and phase analysis:**
   ```bash
   rask timeline                    # Overview of all phases
   rask timeline --active-only      # Focus on active phases
   rask phase overview              # Detailed phase statistics
   rask dependencies --ready        # Ready tasks
   rask show --phase mvp            # Current focus phase
   ```

2. **Use bulk operations for phase transitions:**
   ```bash
   # Move completed MVP features to beta
   rask bulk set-phase 15,16,17 beta
   
   # Promote beta features to release
   rask bulk set-phase 20,21,22 release
   ```

3. **Regular progress exports and timeline reviews:**
   ```bash
   # Weekly team report with phases
   rask export html --include-completed -o weekly_report.html
   
   # Phase-specific burndown data
   rask export csv --phase beta -o beta_progress.csv
   
   # Timeline view for stakeholder presentations
   rask timeline --page-size 10 --compact
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

### Time Tracking Workflows

1. **Daily time tracking routine:**
   ```bash
   # Start day with time tracking review
   rask time --summary
   
   # Begin focused work
   rask start 5 --description "Morning: core feature implementation"
   
   # Track context switches
   rask stop
   rask start 8 --description "Bug fix: critical authentication issue"
   
   # End day review
   rask stop
   rask time --detailed
   ```

2. **Sprint time management:**
   ```bash
   # Sprint planning with capacity
   rask time --summary  # Check team velocity
   rask phase overview  # Review estimated hours by phase
   
   # Daily standups with time data
   rask time --summary
   rask list --phase mvp --detailed  # Current sprint progress
   
   # Sprint retrospective
   rask export csv --include-completed -o sprint_time_analysis.csv
   ```

3. **Estimation improvement workflow:**
   ```bash
   # Regular estimation review
   rask time --summary --detailed  # Analyze variance patterns
   
   # Categorize estimation accuracy
   rask list --detailed | grep "estimated"  # Review all estimates
   
   # Update future estimates based on learnings
   # Use historical data to improve new task estimates
   ```

### Time Tracking Configuration

1. **Project-specific time settings:**
   ```bash
   # Set time tracking preferences per project
   rask config set behavior.auto_start_tracking true --project
   rask config set behavior.require_time_estimates true --project
   rask config set export.include_time_data true --project
   ```

2. **Team time tracking standards:**
   ```bash
   # Standardize time tracking across team
   rask config set behavior.default_time_estimate 4.0
   rask config set export.default_time_format detailed
   rask config set behavior.track_break_time false
   ```

### Timeline Navigation & Visualization

1. **Optimize timeline display for your workflow:**
   ```bash
   # Daily development review
   rask timeline --active-only              # Focus on active phases
   rask timeline --page 1 --compact         # Current work overview
   
   # Sprint planning
   rask timeline --page-size 8              # See more phases at once
   rask show --group-by-phase --detailed    # Detailed phase breakdown
   
   # Stakeholder presentations
   rask timeline --page-size 10 --compact   # Full project overview
   ```

2. **Use pagination effectively:**
   ```bash
   # Navigate large projects systematically
   rask timeline --page 1                   # Foundation & core phases
   rask timeline --page 2                   # Development phases
   rask timeline --page 3                   # Release & future phases
   
   # Adjust page size based on context
   rask timeline --page-size 3              # Detailed review
   rask timeline --page-size 8              # Team meetings
   rask timeline --page-size 15             # Executive overview
   ```

3. **Combine timeline with phase management:**
   ```bash
   # Phase transition workflow
   rask timeline --active-only              # See current state
   rask show --phase mvp --detailed         # Focus on current phase
   rask bulk set-phase 10,11,12 beta        # Move completed tasks
   rask timeline --page 2                   # Review next phases
   
   # Progress tracking
   rask show --group-by-phase --collapse-completed  # Hide completed work
   rask timeline --compact                  # Quick progress overview
   ```

4. **Timeline configuration for teams:**
   ```bash
   # Set consistent timeline defaults
   rask config set ui.default_page_size 5
   rask config set ui.timeline_compact_mode false
   rask config set ui.show_phase_dependencies true
   
   # Project-specific timeline settings
   rask config set ui.default_page_size 8 --project    # Larger projects
   rask config set ui.timeline_compact_mode true --project  # Dense info
   ```

---

This user guide covers all the features and capabilities of Rask, including the new phases system. For quick reference, see the main [README](README.md). For issues or feature requests, please visit the [GitHub repository](https://github.com/tito-sala/rask).

**Happy project planning with phases!** 🎯 