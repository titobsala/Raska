# Rask User Guide 📚

Welcome to the comprehensive Rask user guide! This document covers all features, commands, and usage patterns for the advanced CLI project planner with interactive TUI interface.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Quick Task Creation & Smart Shortcuts](#quick-task-creation--smart-shortcuts)
3. [Interactive TUI Mode](#interactive-tui-mode)
4. [Core Commands](#core-commands)
5. [Advanced Task Management](#advanced-task-management)
6. [Roadmap Phases System](#roadmap-phases-system)
7. [Timeline Visualization & Pagination](#timeline-visualization--pagination)
8. [Time Estimation and Tracking](#time-estimation-and-tracking)
9. [Filtering & Search](#filtering--search)
10. [Project Management](#project-management)
11. [Dependency Management](#dependency-management)
12. [Configuration System](#configuration-system)
13. [Bulk Operations](#bulk-operations)
14. [Export Capabilities](#export-capabilities)
15. [Task Templates System](#task-templates-system)
16. [Usage Examples](#usage-examples)
17. [Terminal UI Features](#terminal-ui-features)
18. [Tips & Best Practices](#tips--best-practices)

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

## Quick Task Creation & Smart Shortcuts

**NEW in v3.2.0**: Rask now includes powerful productivity features for rapid task creation and instant filtering, designed to eliminate friction in your daily workflow.

### 🚀 Quick Task Creation with Natural Language Parsing

The `rask quick` command (alias: `rask q`) intelligently parses task descriptions to automatically extract metadata, dramatically speeding up task creation.

#### `rask quick <text>` / `rask q <text>`
Create tasks using natural language descriptions.

```bash
# Basic examples
rask quick "Fix login bug"
rask q "Add user dashboard"

# Advanced examples with automatic metadata extraction
rask quick "Fix login bug high priority backend"
# ✨ Extracts: priority=high, tags=[backend, bug]

rask quick "Add user dashboard 2 hours frontend feature"
# ✨ Extracts: tags=[frontend, feature], estimated_hours=2.0

rask quick "Deploy to production critical infrastructure"
# ✨ Extracts: priority=critical, tags=[infrastructure, deployment]

rask quick "Create API endpoint medium priority backend 4 hours"
# ✨ Extracts: priority=medium, tags=[backend, api], estimated_hours=4.0
```

### Natural Language Processing Features

#### **Priority Detection**
Automatically recognizes priority keywords:
- **Critical**: "critical", "urgent", "emergency", "blocking"  
- **High**: "high", "important", "priority"
- **Medium**: "medium", "normal" (default)
- **Low**: "low", "minor", "nice to have"

#### **Smart Tag Extraction**
Detects 20+ common development tags:
- **Backend**: backend, api, server, database, auth, security
- **Frontend**: frontend, ui, ux, css, html, javascript
- **Testing**: testing, test, qa, bug, fix
- **DevOps**: deployment, infrastructure, devops, docker, ci/cd
- **Documentation**: docs, documentation, readme, guide

#### **Time Estimation Parsing**
Extracts time estimates from natural language:
- "2 hours" → 2.0 hours
- "30 minutes" → 0.5 hours  
- "1.5h" → 1.5 hours
- "4h 30m" → 4.5 hours

#### **Phase Assignment**
Automatically assigns appropriate phases based on keywords:
- **MVP**: core, basic, essential, foundation
- **Beta**: feature, enhancement, improvement
- **Release**: deploy, production, launch, release
- **Future**: future, later, enhancement

### 🎯 Lightning-Fast Filter Shortcuts

Quick filter shortcuts provide instant access to the most common task filtering operations.

#### `rask ready` / `rask r`
**Show tasks ready to start** - Tasks with no blocking dependencies.

```bash
rask ready    # Show all ready tasks
rask r        # Shorthand alias

# Example output:
# 🎯 Ready Tasks (3 available to start)
# ▶️ #4 Implement authentication #backend
# 🔥 #7 Add error handling #backend  
# ⬆️ #12 Create user profile page #frontend
```

**Benefits:**
- Instant visibility into actionable work
- No need to remember complex filter syntax
- Perfect for daily standup preparation
- Eliminates context switching

#### `rask urgent` / `rask u`
**Show urgent tasks** - High and critical priority tasks only.

```bash
rask urgent   # Show urgent tasks
rask u        # Shorthand alias

# Example output:
# 🔥 Urgent Tasks (2 high-priority items)
# 🔥 #3 Fix security vulnerability #backend #security
# ⬆️ #8 Complete payment integration #backend #critical
```

**Use cases:**
- Daily priority review
- Crisis management
- Sprint planning focus
- Stakeholder updates

#### `rask blocked` / `rask b`
**Show blocked tasks** - Tasks waiting on incomplete dependencies.

```bash
rask blocked  # Show blocked tasks
rask b        # Shorthand alias

# Example output:
# 🚧 Blocked Tasks (4 waiting on dependencies)
# ▶️ #5 Frontend integration (blocked by: #2, #3)
# ⬆️ #9 Performance testing (blocked by: #7)
# 🔥 #11 Production deployment (blocked by: #8, #9, #10)
```

**Benefits:**
- Identify bottlenecks quickly
- Plan dependency resolution
- Unblock team members
- Optimize workflow

#### `rask find <query>` / `rask f <query>`
**Smart search** - Instant full-text search across tasks.

```bash
rask find "auth"          # Find authentication-related tasks
rask f "backend"          # Find backend tasks
rask find "user dashboard" # Find specific features
rask f "#urgent"          # Find tasks with urgent tag

# Example output:
# 🔍 Search Results for "auth" (3 matches)
# ✅ #2 User authentication system #backend #auth
# ⬆️ #4 OAuth integration #backend #auth  
# □ #15 Authentication testing #testing #auth
```

**Advanced search features:**
- Full-text search in descriptions and notes
- Tag-based searching with `#tagname`
- Case-insensitive matching
- Partial word matching
- Multiple keyword support

### Productivity Workflow Examples

#### **Morning Development Routine**
```bash
# Quick project overview
rask ready              # What can I work on?
rask urgent             # What's most important?
rask blocked            # What's holding us back?

# Add quick tasks from standup
rask q "Fix login timeout bug high priority backend"
rask q "Add loading spinner to dashboard frontend"
rask q "Update API documentation docs"
```

#### **Rapid Task Entry**
```bash
# Replace this verbose command:
rask add "Fix user authentication bug" --tag backend,bug --priority high --phase mvp

# With this natural language:
rask q "Fix user authentication bug high priority backend"
```

#### **Instant Status Checks**
```bash
# Team lead checking project status
rask urgent             # What needs immediate attention?
rask blocked            # What's blocking the team?
rask find "deployment"  # How's our release prep?
rask ready              # What work is available?
```

#### **Sprint Planning Acceleration**
```bash
# Quick task creation for sprint backlog
rask q "Implement user preferences API backend 6 hours"
rask q "Design settings page UI frontend 4 hours"  
rask q "Add preference validation testing 2 hours"
rask q "Deploy preferences feature production critical"

# Instant filtering for sprint planning
rask ready              # Available sprint work
rask urgent             # Must-have features
rask find "API"         # Backend-focused sprint items
```

### Integration with Existing Features

#### **Works with All Filtering**
```bash
# Combine quick shortcuts with traditional filtering
rask ready --phase mvp           # Ready MVP tasks
rask urgent --tag backend        # Urgent backend tasks
rask blocked --priority critical # Critical blocked tasks
rask find "API" --phase beta     # API tasks in beta phase
```

#### **Seamless TUI Integration**
```bash
# Use quick commands then launch TUI for detailed work
rask q "Fix dashboard loading bug high priority frontend"
rask urgent                      # Review urgent tasks  
rask interactive                 # Switch to TUI for detailed management
```

#### **Export and Reporting**
```bash
# Create filtered exports using quick shortcuts
rask urgent --export html -o urgent_tasks.html
rask blocked --export csv -o blocked_analysis.csv
rask ready --export json --pretty -o ready_tasks.json
```

### Best Practices for Quick Operations

#### **Effective Natural Language Patterns**
```bash
# Good patterns that work well:
rask q "Fix [issue] [priority] [area]"
rask q "Add [feature] [time] [area] [type]"
rask q "Create [component] [priority] [area]"
rask q "Deploy [feature] [priority] [environment]"

# Examples:
rask q "Fix login timeout critical backend"
rask q "Add user search 3 hours frontend feature"
rask q "Create payment API high priority backend"
rask q "Deploy auth system critical production"
```

#### **Quick Filter Workflow**
```bash
# Start each development session with:
rask ready              # What's actionable?
rask urgent             # What's most important?
rask blocked            # Any impediments?

# End sessions with:
rask find "in progress" # What did I work on?
rask ready              # What's ready for tomorrow?
```

#### **Team Collaboration**
```bash
# Share quick status with team:
rask urgent | head -5   # Top 5 urgent items
rask blocked            # Current blockers  
rask ready | wc -l      # Number of ready tasks

# Quick task assignment:
rask q "Review PR #123 high priority review @teammate"
rask q "Test feature X 2 hours testing @qa-team"
```

## Interactive TUI Mode

**NEW**: Rask includes a comprehensive Terminal User Interface (TUI) that provides a modern, interactive project management experience directly in your terminal. 

**🔧 Recent Improvements**: Navigation freezing issues in the project switcher have been resolved with caching optimizations, providing a much smoother user experience.

### Launching the TUI

```bash
# Basic launch
rask interactive

# Launch with specific project
rask interactive --project my-project

# Skip welcome message
rask interactive --no-welcome
```

### TUI Overview

The Interactive TUI features a multi-view dashboard system with:

- **Navigation Bar**: Quick access to all views with keyboard shortcuts
- **Context-Aware Help**: Bottom-bar help that changes based on current view
- **Real-Time Updates**: Instant feedback for all actions
- **Settings Persistence**: Remembers your preferences and view settings
- **Professional Layout**: Beautiful terminal interface with colors and formatting
- **Navigation Fixes**: Recent improvements have resolved freezing issues in project navigation

### TUI Views

#### 🏠 Home Dashboard (F1)
Your project command center with:

- **Project Overview**: Title, progress bars, task statistics
- **Recent Activity**: Live feed of actions and updates
- **Quick Actions**: Function key shortcuts and common commands
- **Phase Summary**: Overview of development phases with progress
- **Time Tracking**: Current session info and project time statistics

**Navigation:**
- View project statistics and overall progress
- Access quick action shortcuts
- Monitor recent activity and changes
- Get overview of all available functions

#### 📝 Task Manager (F2)
Interactive task management interface with:

- **Task List**: Scrollable list with all tasks, priorities, and status
- **Real-Time Completion**: Toggle task completion with Enter/Space
- **Dependency Indicators**: Visual representation of task dependencies
- **Phase Indicators**: Emoji-based phase identification
- **Mini AI Assistant**: Task-focused chat interface
- **Time Tracking Display**: Shows estimated vs actual time for tasks

**Key Features:**
- **Interactive Completion**: Toggle tasks complete/pending instantly
- **Scrolling Support**: Navigate large task lists with Up/Down arrows
- **Context Information**: See dependencies, priorities, and phases at a glance
- **Integrated Chat**: Ask questions about specific tasks
- **Progress Tracking**: Real-time progress updates

**Keyboard Shortcuts:**
- `↑↓`: Navigate tasks
- `Enter`/`Space`: Toggle task completion
- `Tab`: Switch between task list and chat
- `PgUp`/`PgDn`: Fast scroll through tasks

#### 📄 Templates (View available in TUI)
Browse and manage task templates:

- **Template Categories**: Organized by type (Development, Testing, etc.)
- **Template Preview**: See template details and structure
- **Quick Application**: Apply templates to create new tasks
- **Custom Templates**: Create and manage your own patterns

**Features:**
- Browse built-in templates for common scenarios
- Preview template structure before applying
- Create consistent task patterns
- Speed up project setup with proven templates

#### ⚙️ Settings (Available in TUI)
Customize TUI behavior and appearance:

- **Configuration Categories**: Organized settings sections
- **Real-Time Preview**: See changes instantly
- **Persistence**: Save settings for future sessions
- **Project-Specific**: Configure per-project preferences
- **Default Values**: Reset to defaults when needed

**Settings Categories:**
- **Display**: Color schemes, layout options, formatting
- **Behavior**: Default priorities, phases, confirmation settings
- **Performance**: Refresh rates, auto-save options
- **Export**: Default formats and options

#### 🔄 Project Switcher (Available in TUI)
Seamless multi-project navigation (navigation issues recently fixed):

- **Project List**: All available projects with statistics
- **Current Project**: Highlighted active project
- **Project Details**: Creation dates, descriptions, state files
- **Quick Switching**: Instant project switching with Enter
- **Project Statistics**: Task counts and completion progress for each project

**Features:**
- **Visual Project List**: See all projects with progress indicators
- **Current Project Highlighting**: Clear indication of active project
- **Project Metadata**: Creation dates, descriptions, file paths
- **Instant Switching**: Change projects without leaving TUI
- **Statistics Display**: Quick overview of each project's status
- **Navigation Fix**: Recent improvements resolved freezing issues

**Navigation:**
- `↑↓`: Navigate project list
- `Enter`: Switch to selected project
- `r`: Refresh project list


### Universal Keyboard Shortcuts

These shortcuts work across all views:

| Shortcut | Action |
|----------|--------|
| `F1` | Home Dashboard |
| `F2` | Task Manager |
| `F3` | AI Assistant |
| `F4` | Templates |
| `F5` | Analytics |
| `F6` | Settings |
| `F7` / `p` | Project Switcher |
| `Tab` | Cycle through panels in current view |
| `h` | Show help and shortcuts |
| `r` | Refresh project data |
| `s` | Save current settings |
| `Esc` | Return to Home view |
| `q` / `Ctrl+C` | Quit application |

### TUI Settings and Persistence

The TUI automatically saves your preferences:

- **Default View**: Which view to open on startup
- **Window Layout**: Panel sizes and arrangements
- **Display Options**: Color schemes and formatting
- **Behavior Settings**: Default actions and confirmations

Settings are stored in `~/.config/rask/tui_settings.json` and loaded automatically.

### TUI Workflow Examples

#### Daily Development Workflow
```bash
# Start your day with TUI
rask interactive

# Review project status (Home Dashboard - F1)
# Check tasks ready to work on (Task Manager - F2)
# Toggle task completion as you work (Enter/Space)
# Ask questions about complex tasks (AI Assistant - F3)
# Apply templates for new features (Templates - F4)
# Check progress at end of day (Analytics - F5)
```

#### Multi-Project Management
```bash
# Launch TUI
rask interactive

# Switch between projects (Project Switcher - F7/p)
# Compare progress across projects (Analytics - F5)
# Apply consistent templates across projects (Templates - F4)
# Configure project-specific settings (Settings - F6)
```

#### Sprint Planning Session
```bash
# Open TUI for collaborative planning
rask interactive

# Review current progress (Analytics - F5)
# Check task dependencies (Task Manager - F2)
# Plan next tasks with templates (Templates - F4)
# Discuss priorities with team (AI Assistant - F3)
# Configure sprint settings (Settings - F6)
```

### TUI vs CLI Usage

**Use TUI when:**
- Interactive task management and real-time updates
- Exploring project status and navigation
- Multi-project switching and comparison
- Learning Rask features with guided interface
- Collaborative planning and discussion
- Visual progress tracking and analytics

**Use CLI when:**
- Automation and scripting
- Bulk operations and batch processing
- Integration with other tools
- Quick single commands
- Export and reporting tasks
- CI/CD pipeline integration

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

## Task Templates System

Rask includes a comprehensive task templates system for consistent task creation and project setup.

### Template Commands

#### `rask template list [--category <cat>]`
List all available templates.

```bash
# List all templates
rask template list

# List templates by category
rask template list --category development
rask template list --category testing
```

#### `rask template show <name>`
Show detailed template information.

```bash
rask template show "Feature Implementation"
rask template show "Bug Fix"
```

#### `rask template use <name> [description]`
Create task from template.

```bash
# Use template with default description
rask template use "Bug Fix"

# Use template with custom description
rask template use "Feature Implementation" "User authentication system"
```

#### `rask template create <name> <desc> [options]`
Create custom template.

```bash
# Create basic template
rask template create "Code Review" "Review pull request for team member"

# Create template with metadata
rask template create "API Endpoint" "Implement new API endpoint" \
  --tags "backend,api" --priority high --phase mvp --estimated-hours 4.0
```

#### `rask template delete <name>`
Delete custom template.

```bash
rask template delete "Old Template"
```

#### `rask template examples`
Show help and integration examples.

```bash
rask template examples
```

### Built-in Templates

Rask comes with several built-in templates for common development scenarios:

| Template | Category | Description |
|----------|----------|-------------|
| Feature Implementation | Development | Complete feature development with testing |
| Bug Fix | Development | Systematic bug identification and resolution |
| Code Review | Development | Pull request review and feedback process |
| API Endpoint | Development | REST API endpoint implementation |
| Database Migration | Development | Database schema changes and migrations |
| Unit Testing | Testing | Comprehensive unit test coverage |
| Integration Testing | Testing | End-to-end integration testing |
| Performance Testing | Testing | Load and performance testing |
| Documentation | Documentation | Technical documentation writing |
| User Guide | Documentation | User-facing documentation |
| Deployment | DevOps | Production deployment process |
| Environment Setup | DevOps | Development environment configuration |
| Security Review | Security | Security assessment and improvements |
| UI Component | Design | User interface component development |

### Template Categories

Templates are organized into logical categories:

- **Development**: Core development tasks
- **Testing**: Quality assurance and testing
- **Documentation**: Documentation and guides
- **DevOps**: Deployment and infrastructure
- **Design**: UI/UX and design tasks
- **Security**: Security-related tasks

### Custom Template Creation

Create templates that match your team's workflow:

```bash
# Development workflow template
rask template create "Sprint Task" "Standard sprint development task" \
  --tags "development,sprint" \
  --priority medium \
  --phase mvp \
  --estimated-hours 8.0

# Bug tracking template
rask template create "Production Bug" "Critical production issue" \
  --tags "bug,production,urgent" \
  --priority critical \
  --phase release \
  --estimated-hours 2.0
```

### Template Best Practices

1. **Consistent Naming**: Use clear, descriptive template names
2. **Appropriate Tags**: Include relevant tags for filtering
3. **Realistic Estimates**: Base time estimates on historical data
4. **Phase Alignment**: Assign templates to appropriate phases
5. **Team Standards**: Create templates that match team conventions

### Template Integration Workflow

```bash
# Daily development with templates
rask template use "Feature Implementation" "User dashboard analytics"
rask template use "Unit Testing" "Test dashboard analytics feature"
rask template use "Code Review" "Review analytics implementation"

# Sprint planning with templates
rask template list --category development
rask template use "API Endpoint" "User preferences API"
rask template use "Database Migration" "Add user preferences table"
rask template use "Documentation" "API documentation for preferences"
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

### Scenario 6: Interactive TUI for Modern Project Management

```bash
# Launch interactive TUI for comprehensive project management
rask interactive

# Daily workflow using TUI interface:

# 1. Start with Home Dashboard (F1)
#    - Review project overview and statistics
#    - Check recent activity and progress bars
#    - See quick action shortcuts

# 2. Switch to Task Manager (F2)
#    - Navigate tasks with ↑↓ arrow keys
#    - Toggle task completion with Enter/Space
#    - View dependencies and phase indicators
#    - See time tracking information

# 3. Use Project Switcher (F7 or p key)
#    - Navigate between multiple projects
#    - See project statistics and progress
#    - Switch projects instantly with Enter
#    - Compare project states

# 4. Check Analytics (F5)
#    - View progress visualization
#    - Analyze priority distribution
#    - Check phase completion rates
#    - Review time estimation accuracy

# 5. Apply Templates (F4)
#    - Browse available task templates
#    - Apply templates for consistent task creation
#    - Use templates for recurring patterns

# 6. Chat with AI Assistant (F3)
#    - Ask questions about project planning
#    - Get help with task breakdown
#    - Request dependency guidance
#    - Chat about project strategy

# 7. Configure Settings (F6)
#    - Customize TUI appearance and behavior
#    - Set default views and preferences
#    - Configure project-specific settings
#    - Save settings for future sessions

# Throughout the session:
# - Use Tab to cycle through panels
# - Use h for context-sensitive help
# - Use r to refresh project data
# - Use s to save current settings
# - Use q to quit when done

# Example multi-project workflow in TUI:
# F7 → Select "backend-api" project → F2 → Complete some tasks
# F7 → Switch to "mobile-app" project → F5 → Check analytics
# F7 → Switch to "documentation" project → F4 → Apply templates
# F6 → Configure project-specific settings
# F1 → Return to home dashboard for overview
```

### Scenario 7: Rapid Development with Quick Task Creation & Smart Shortcuts (NEW v3.2.0)

```bash
# Morning standup - quick status check using smart shortcuts
rask ready              # What tasks are ready to work on?
rask urgent             # What needs immediate attention?
rask blocked            # What's holding up the team?

# Example output from ready tasks:
# 🎯 Ready Tasks (4 available to start)
# ▶️ #12 Implement user search API #backend
# 🔥 #15 Fix dashboard loading bug #frontend #critical
# ⬆️ #18 Add error handling to payments #backend
# ⬆️ #23 Create user profile tests #testing

# Quick task creation during standup using natural language
rask q "Fix authentication timeout bug critical backend"
# ✨ Parsed: priority=critical, tags=[backend,bug], phase=mvp

rask q "Add loading states to dashboard 2 hours frontend"
# ✨ Parsed: tags=[frontend,ui], estimated_hours=2.0, phase=mvp

rask q "Deploy hotfix to production urgent infrastructure"
# ✨ Parsed: priority=critical, tags=[infrastructure,deployment], phase=release

rask q "Create API documentation 3 hours docs"
# ✨ Parsed: tags=[docs,documentation], estimated_hours=3.0, phase=mvp

# Development workflow with instant filtering
rask urgent             # Focus on critical issues first
# 🔥 Urgent Tasks (3 high-priority items)
# 🔥 #24 Fix authentication timeout bug #backend #bug
# 🔥 #15 Fix dashboard loading bug #frontend #critical  
# 🔥 #26 Deploy hotfix to production #infrastructure #deployment

# Start working on most critical issue
rask start 24 --description "Investigating auth timeout in production"

# Quick progress check during the day
rask find "auth"        # Check all authentication-related work
# 🔍 Search Results for "auth" (4 matches)
# ⏱️ #24 Fix authentication timeout bug #backend #bug [Active: 1.2h]
# ✅ #8 User authentication system #backend #auth  
# □ #32 OAuth integration #backend #auth
# □ #45 Authentication testing #testing #auth

# Team collaboration using quick shortcuts
rask blocked            # Check what's blocking team progress
# 🚧 Blocked Tasks (2 waiting on dependencies)
# ▶️ #19 Frontend integration (blocked by: #15, #24)
# ⬆️ #31 Performance testing (blocked by: #26)

# Rapid task creation for sprint backlog
rask q "Implement user preferences API backend 8 hours"
rask q "Design settings page UI frontend 6 hours"
rask q "Add preference validation testing 3 hours"
rask q "Create user onboarding flow frontend 12 hours"
rask q "Add analytics tracking backend 4 hours"

# Sprint planning with smart filtering
rask ready --phase mvp           # Ready tasks for current sprint
rask urgent --tag backend        # Urgent backend work
rask find "API" --phase beta     # API development for next phase

# End of day workflow
rask stop                        # Stop time tracking
rask ready                       # Check tomorrow's ready tasks
rask urgent                      # Review remaining urgent items

# Export sprint progress using quick filters
rask urgent --export html -o urgent_sprint_items.html
rask ready --export json --pretty -o ready_tasks_tomorrow.json
rask blocked --export csv -o team_blockers.csv

# Team retrospective using natural language search
rask find "bug" --export csv -o bugs_this_sprint.csv
rask find "feature" --tag frontend -o frontend_features.json
rask time --summary              # Sprint time tracking analysis
```

### Scenario 8: Product Manager Workflow with Quick Operations

```bash
# Daily product management routine using smart shortcuts
rask urgent             # Critical items needing attention
rask blocked            # Bottlenecks affecting team velocity
rask find "user"        # All user-facing features
rask ready              # Available work capacity

# Stakeholder meeting preparation
rask find "release"     # Release-related tasks
rask urgent --phase release     # Critical release items
rask timeline --page-size 10    # Full project timeline

# Feature request processing with natural language
rask q "Add user notifications feature high priority frontend 8 hours"
rask q "Implement push notifications backend 12 hours"
rask q "Create notification settings UI 4 hours frontend"
rask q "Add notification analytics tracking 6 hours backend"

# Priority management across features
rask urgent             # Current high-priority work
rask bulk set-priority 48,49,50,51 medium  # Adjust notification feature priority

# Cross-team coordination
rask blocked            # Dependencies blocking teams
rask find "API" --tag backend   # Backend API development status
rask find "UI" --tag frontend   # Frontend UI component status

# Sprint planning with quick insights
rask ready --phase mvp           # MVP work available
rask urgent --phase beta         # Beta phase priorities
rask blocked --phase release     # Release blockers

# Generate reports for stakeholders
rask export html --phase release --include-completed -o release_status.html
rask urgent --export csv -o current_priorities.csv
rask timeline --compact > project_timeline.txt

# Feature impact analysis
rask find "notification" --export json --pretty -o notification_feature_analysis.json
rask dependencies --task-id 48  # Check notification feature dependencies
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

### Interactive TUI Features
- **Multi-view dashboard**: 7 integrated views with seamless navigation
- **Real-time updates**: Instant task completion toggling and progress updates
- **Project switcher**: Seamless navigation between multiple projects
- **Keyboard navigation**: F1-F7 shortcuts for instant view switching
- **Context-aware help**: Dynamic help system that changes based on current view
- **Settings persistence**: Remembers preferences and customizations
- **Professional interface**: Modern TUI experience with colors and formatting

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

### TUI Usage & Navigation

1. **Smooth navigation experience:**
   - Recent caching improvements have resolved project switcher freezing issues
   - Navigation between projects is now instant and responsive
   - Use `↑↓` arrows to navigate smoothly through project lists
   - Project switching no longer causes UI freezes

2. **Optimal TUI workflow:**
   ```bash
   # Start with interactive mode for best experience
   rask interactive
   
   # Use keyboard shortcuts for efficient navigation
   # Home: Overview and project status
   # Tasks: Interactive task management
   # Templates: Quick task creation
   # Settings: Customize your experience
   # Projects: Switch between projects seamlessly
   ```

3. **Performance considerations:**
   - The TUI now caches workspace and project information for better performance
   - Large project lists load quickly without blocking the interface
   - Multi-project switching is optimized for real-time use

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

1. **Start each day with Interactive TUI (Recommended):**
   ```bash
   rask interactive                 # Launch TUI dashboard
   # F1 - Review Home Dashboard with project overview
   # F2 - Check Task Manager for ready tasks
   # F5 - Review Analytics for progress insights
   # F7 - Switch projects if needed
   ```

2. **Or use traditional CLI for specific operations:**
   ```bash
   rask timeline                    # Overview of all phases
   rask timeline --active-only      # Focus on active phases
   rask phase overview              # Detailed phase statistics
   rask dependencies --ready        # Ready tasks
   rask show --phase mvp            # Current focus phase
   ```

2. **Use TUI for interactive project management:**
   ```bash
   # Interactive bulk operations via TUI
   rask interactive
   # F2 - Task Manager: Toggle multiple task completions
   # F7 - Project Switcher: Compare and switch projects
   # F5 - Analytics: Review progress across phases
   # F6 - Settings: Configure project preferences
   ```

3. **Use bulk CLI operations for automation:**
   ```bash
   # Move completed MVP features to beta
   rask bulk set-phase 15,16,17 beta
   
   # Promote beta features to release
   rask bulk set-phase 20,21,22 release
   ```

4. **Regular progress exports and timeline reviews:**
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

### Quick Task Creation & Smart Shortcuts Best Practices (NEW v3.2.0)

1. **Optimize natural language patterns:**
   ```bash
   # Effective patterns for quick task creation
   rask q "[Action] [Subject] [Priority] [Area] [Time]"
   
   # Examples that work well:
   rask q "Fix login timeout critical backend"       # bug fixes
   rask q "Add user search 4 hours frontend"        # features with time
   rask q "Deploy auth system urgent production"    # deployments
   rask q "Create API documentation 2 hours docs"   # documentation
   rask q "Test payment flow 3 hours testing"       # testing tasks
   ```

2. **Master the smart shortcuts workflow:**
   ```bash
   # Start each day with the "3R" routine:
   rask ready     # What's Ready to work on?
   rask urgent    # What's uRgent and needs attention?
   rask blocked   # What's bLocked and needs unblocking?
   
   # Use throughout the day for instant context:
   rask find "feature-name"  # Quick feature-specific search
   rask urgent --tag backend # Context-specific urgent items
   rask ready --phase mvp    # Phase-specific ready tasks
   ```

3. **Combine quick operations with existing workflows:**
   ```bash
   # Sprint planning enhancement:
   rask ready --phase mvp > sprint_ready.txt     # Export ready items
   rask urgent --export csv -o sprint_urgent.csv # Prioritized list
   rask blocked --phase mvp                      # Sprint blockers
   
   # Team standup preparation:
   rask find "my-feature" --detailed            # My work status
   rask blocked | grep "my-name"                # What I'm blocked on
   rask ready | head -3                         # Next 3 tasks to pick up
   ```

4. **Effective tag extraction optimization:**
   ```bash
   # Use consistent terminology that matches your extraction patterns:
   rask q "Fix API timeout bug high priority backend"  # Good: clear keywords
   rask q "Add search functionality to user panel frontend 4 hours"  # Good: specific area
   
   # Avoid ambiguous descriptions:
   rask q "Fix the thing that's broken somewhere"      # Poor: no extractable info
   rask q "Add stuff to the dashboard thing"          # Poor: vague terminology
   ```

5. **Quick operations for team coordination:**
   ```bash
   # Daily team sync shortcuts:
   rask urgent | wc -l              # Count of urgent items
   rask blocked --export csv        # Share blockers in team chat
   rask ready --tag backend | head -5  # Backend team's next work
   
   # Feature ownership and handoffs:
   rask find "auth" --detailed      # All auth-related work
   rask q "Handoff auth feature to QA team testing high priority"
   rask blocked --tag "feature-x"  # Feature-specific blockers
   ```

6. **Natural language consistency for teams:**
   ```bash
   # Establish team conventions for better parsing:
   # Priority: always use "critical", "high", "medium", "low"
   # Areas: standardize on "backend", "frontend", "testing", "docs", "infrastructure"
   # Time: use "X hours" or "X.5 hours" format consistently
   
   # Good team patterns:
   rask q "Fix user registration bug high priority backend"
   rask q "Add loading spinner medium priority frontend 1 hour"
   rask q "Create deployment script high priority infrastructure 3 hours"
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

4. **Time tracking configuration:**
   ```bash
   # Set time tracking preferences per project
   rask config set behavior.auto_start_tracking true --project
   rask config set behavior.require_time_estimates true --project
   rask config set export.include_time_data true --project
   ```

5. **Team time tracking standards:**
   ```bash
   # Standardize time tracking across team
   rask config set behavior.default_time_estimate 4.0
   rask config set export.default_time_format detailed
   rask config set behavior.track_break_time false
   ```

---

This user guide covers all the features and capabilities of Rask, including the comprehensive Interactive TUI system with project switcher navigation fixes, phases management, time tracking, and all CLI functionality. Recent improvements have resolved navigation freezing issues for a smoother user experience.

For quick reference, see the main [README](README.md). For issues or feature requests, please visit the [GitHub repository](https://github.com/tito-sala/rask).

**Happy project planning with the enhanced Interactive TUI!** 🎯 🖥️