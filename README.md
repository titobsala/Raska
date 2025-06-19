# Rask 🚀 - CLI Project Planner

[![Version](https://img.shields.io/badge/version-3.0.0-blue.svg)](https://github.com/tito-sala/rask)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

**Rask** is a command-line project planner built with Rust. It transforms Markdown files into sophisticated task management systems with advanced task tracking, dependency management, multi-project support, roadmap phases, interactive TUI interface, and comprehensive terminal UI. Recently fixed TUI navigation issues for smoother user experience.

## ✨ Key Features

### 🖥️ **Interactive TUI Interface**
- Full-featured Terminal User Interface with real-time interaction
- Multi-view dashboard system (Home, Tasks, Templates, Settings, Project Switcher)
- Project switcher for seamless navigation between projects
- Real-time task completion toggling and progress updates
- Recently fixed navigation freezing issues for smooth operation
- Context-aware help system and keyboard navigation

### 🎯 **Core Task Management**
- Initialize projects from Markdown files with automatic sync
- Complete, add, edit, and remove tasks with rich metadata
- Beautiful, colored terminal output with progress tracking
- Detailed task view with comprehensive information

### 🏗️ **Roadmap Phases System**
- Organize tasks by development lifecycle phases (MVP, Beta, Release, Future, Backlog)
- Create custom phases with personalized names and descriptions
- Phase-based filtering and visualization with emoji indicators
- Smart phase recommendations and progress tracking
- Comprehensive phase overview with statistics and insights
- Easy phase assignment and bulk phase operations

### 🏷️ **Enhanced Task Metadata**
- **Tags**: Categorize tasks with custom tags (`#backend`, `#urgent`, etc.)
- **Priorities**: Four priority levels (Low, Medium, High, Critical) with visual indicators
- **Notes**: Detailed descriptions and context for each task
- **Dependencies**: Link tasks with sophisticated dependency management
- **Phases**: Organize tasks by development lifecycle stages
- **Time Estimation**: Estimate completion time in hours for better planning
- **Time Tracking**: Track actual time spent on tasks with start/stop functionality

### 🔍 **Advanced Filtering & Search**
- Filter by tags, priority, status, phases, or any combination
- Full-text search across task descriptions and notes
- Flexible AND/OR filter operations
- List ready-to-start vs blocked tasks
- Phase-specific task filtering and display

### 🏢 **Multi-Project Workspace**
- Manage multiple projects simultaneously
- Project-specific state files and configurations
- Easy project switching and isolation
- Comprehensive configuration system

### 🔗 **Dependency Management**
- Define task dependencies with automatic validation
- Circular dependency detection and prevention
- Visual dependency tree exploration with ASCII art
- Dependency chain analysis and impact assessment
- Block task completion until dependencies are satisfied

### ⚡ **Power User Features**
- Interactive TUI mode with comprehensive project management dashboard
- Bulk operations (complete, tag, prioritize, set phases for multiple tasks)
- Export capabilities (JSON, CSV, HTML) with filtering and phase information
- Configuration system with user and project-specific settings
- Enhanced dependency tree visualization
- Modular architecture for better maintainability and extensibility
- Time estimation and tracking with variance analysis
- Session-based time tracking with optional descriptions

### 📊 **Visualization & Analysis**
- Comprehensive dependency trees with ASCII art
- Progress bars and completion statistics
- Ready vs blocked task analysis
- Project overview dashboards
- Beautiful HTML exports with responsive design
- Phase-based progress tracking and recommendations
- Timeline view with horizontal phase progression and pagination
- Phase-grouped displays with individual progress bars

### 📋 **Task Templates System**
- Pre-configured task patterns for consistent task creation
- Built-in templates for common development scenarios (Bug Fix, Feature Implementation, etc.)
- Custom template creation with full metadata support
- Template categories (Development, Testing, Documentation, DevOps, Design, etc.)
- Template export/import for team sharing
- Smart placeholders and implementation notes

## 🚀 Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/tito-sala/rask.git
cd rask

# Build and install
cargo install --path .
```

### Interactive TUI Mode

Launch the powerful interactive Terminal User Interface:

```bash
# Start interactive mode
rask interactive

# Start with specific project
rask interactive --project my-project

# Skip welcome message
rask interactive --no-welcome
```

**TUI Features:**
- **🏠 Home Dashboard**: Project overview with statistics and quick actions
- **📝 Task Manager**: Interactive task list with real-time completion toggling
- **📄 Templates**: Browse and apply task templates
- **⚙️ Settings**: Customize TUI behavior and appearance
- **🔄 Project Switcher**: Seamlessly navigate between multiple projects (navigation issues recently fixed)

**Navigation:**
- **↑↓**: Navigate lists and options
- **Enter/Space**: Toggle tasks or select options
- **Tab**: Cycle through interface panels
- **q**: Quit application

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

3. **Launch Interactive TUI (Recommended):**
```bash
rask interactive
```

4. **Or use traditional CLI:**
```bash
# View your project
rask show

# Add tasks with metadata, phases, and time estimates
rask add "Implement OAuth" --tag backend,auth --priority high --phase beta --depends-on 2 --estimated-hours 4.5

# Manage phases
rask phase overview

# Track time on tasks
rask start 1 --description "Working on OAuth integration"
# ... work on the task ...
rask stop

# Complete tasks
rask complete 1
```

5. **Export progress with phase and time information:**
```bash
rask export html -o progress_report.html --include-completed
```

## 📚 Documentation

For comprehensive documentation, examples, and advanced usage patterns, see the **[User Guide](USER_GUIDE.md)**.

### Quick Command Reference

| Command | Description |
|---------|-------------|
| `rask interactive [options]` | Launch interactive TUI with full dashboard |
| `rask init <file.md>` | Initialize project from Markdown |
| `rask show [options]` | Display project status with phase grouping and filtering |
| `rask timeline [options]` | Show horizontal timeline with phase progression and pagination |
| `rask add <desc> [options]` | Add task with metadata, phase, and time estimate |
| `rask complete <id>` | Complete a task |
| `rask view <id>` | View detailed task information |
| `rask list [filters]` | List and filter tasks |
| `rask phase <operation>` | Manage roadmap phases |
| `rask dependencies [options]` | Analyze dependencies |
| `rask start <id> [options]` | Start time tracking for a task |
| `rask stop` | Stop current time tracking session |
| `rask time [id] [options]` | View time tracking information |
| `rask bulk <operation> <ids>` | Bulk operations on multiple tasks |
| `rask export <format> [options]` | Export to JSON/CSV/HTML with phases and time data |
| `rask config <operation>` | Manage configuration |
| `rask project <operation>` | Multi-project management |
| `rask template <operation>` | Manage task templates |

### Interactive TUI Navigation

| Key | Action |
|-----|--------|
| `↑↓` | Navigate lists and options |
| `Enter` / `Space` | Toggle tasks or select options |
| `Tab` | Cycle through interface panels |
| `q` | Quit application |

### Phase Commands

| Command | Description |
|---------|-------------|
| `rask phase overview` | Show comprehensive phase statistics |
| `rask phase list` | List all phases with descriptions |
| `rask phase show <phase>` | Display tasks in specific phase |
| `rask phase set <id> <phase>` | Set phase for individual task |
| `rask phase create <name> [options]` | Create custom phases |
| `rask bulk set-phase <ids> <phase>` | Set phase for multiple tasks |

### Timeline & Visualization Commands

| Command | Description |
|---------|-------------|
| `rask timeline` | Show horizontal timeline with all phases (paginated) |
| `rask timeline --page <n>` | Navigate to specific page (default: 5 phases per page) |
| `rask timeline --page-size <n>` | Set number of phases per page |
| `rask timeline --compact` | Use compact view for more information |
| `rask timeline --active-only` | Show only phases containing tasks |
| `rask show --group-by-phase` | Group tasks by phase with progress bars |
| `rask show --phase <name>` | Filter display to specific phase |
| `rask show --collapse-completed` | Collapse completed phases |

### Time Tracking Commands

| Command | Description |
|---------|-------------|
| `rask start <id> [--description <desc>]` | Start time tracking for a specific task |
| `rask stop` | Stop the currently active time tracking session |
| `rask time [id]` | View time tracking info for a task (or all tasks) |
| `rask time --summary` | Show time tracking summary across all tasks |
| `rask time --detailed` | Show detailed time session history |
| `rask add --estimated-hours <hours>` | Add task with time estimation |

### Template Commands

| Command | Description |
|---------|-------------|
| `rask template list [--category <cat>]` | List all available templates |
| `rask template show <name>` | Show detailed template information |
| `rask template use <name> [description]` | Create task from template |
| `rask template create <name> <desc> [options]` | Create custom template |
| `rask template delete <name>` | Delete custom template |
| `rask template examples` | Show help and integration examples |

## 🎨 Interactive TUI Preview

Comprehensive Terminal User Interface with multiple views:

```
┌─ 🚀 Rask TUI • Task Manager ──────────────────────────────────────────────────┐
│  🏠 Home │📝 Tasks │📄 Templates │⚙️ Settings │🔄 Projects                    │
└────────────────────────────────────────────────────────────────────────────────┘
┌─ 📋 My Web App Project • 8/12 tasks (67%) ──────────┐ ┌─ 📊 Phase Progress ──┐
│                                                      │ │                      │
│ ┌─ Tasks (1-8/12) ──────────────────────────────────┐ │ │ 🚀 MVP: 67% (2/3)    │
│ │ ✅ ▶️ #1 Set up development environment 🚀      │ │ │ 🧪 Beta: 50% (1/2)   │
│ │ ✅ ⬆️ #2 Design database schema 🚀 [2.8h]      │ │ │ 🎯 Release: 0% (0/2) │
│ │ ✅ 🔥 #3 Create API endpoints #backend 🧪      │ │ │                      │
│ │ □ ⬆️ #4 Implement authentication 🧪 [4.5h]     │ │ │ 🎯 Ready: Task #4    │
│ │     🔗 Depends on: 2, 3                        │ │ │                      │
│ │ ⏱️ ▶️ #5 Write tests #testing 🎯 [1.2h tracked] │ │ │                      │
│ │     🔗 Depends on: 4 | 🕐 Active session       │ │ │                      │
│ │ □ 🔥 #6 Deploy to production #deployment 🎯    │ │ │                      │
│ │     🔗 Depends on: 5                           │ │ └──────────────────────┘
│ └──────────────────────────────────────────────────┘ │
│                                                      │
└──────────────────────────────────────────────────────┘

↑↓: Navigate • Enter/Space: Toggle • Tab: Switch panels • Q: Quit
```

### CLI Terminal Preview

```
════════════════════════════════════════════════════════════
  My Web App Project
  📁 Project: web-app (Main development project)
════════════════════════════════════════════════════════════
  Progress: [████████████████░░░░░░░░░░░░] 67% (8/12)

  📋 Tasks:
  ──────────────────────────────────────────────────
  ✓ ▶️ # 1 Set up development environment 🚀
  ✓ ⬆️ # 2 Design database schema 🚀 [2.5h estimated, 2.8h actual]
  ✓ 🔥 # 3 Create API endpoints #backend 🧪 [6.0h estimated, 5.2h actual]
  □ ⬆️ # 4 Implement authentication #backend #security 🧪 [4.5h estimated]
      🔗 Depends on: 2, 3
  ⏱️ ▶️ # 5 Write tests #testing 🎯 [3.0h estimated, 1.2h tracked]
      🔗 Depends on: 4 | 🕐 Active since 14:30
  □ 🔥 # 6 Deploy to production #deployment 🎯 [2.0h estimated]
      🔗 Depends on: 5
  ──────────────────────────────────────────────────
  
  📊 Phase Overview:
  🚀 MVP: 2/3 tasks (67% complete) | 8.5h estimated, 8.0h actual
  🧪 Beta: 1/2 tasks (50% complete) | 7.5h estimated, 6.4h tracked
  🎯 Release: 0/2 tasks (0% complete) | 2.0h estimated
  
  ⏱️ Time Tracking:
  📈 Total estimated: 18.0h | 🕐 Total tracked: 14.4h | ✅ Efficiency: 125%
  🔥 Currently tracking: Task #5 (1.2h active session)
  
  🎯 Great progress! Focus on Beta phase next.
```

## 🤖 Future: Web Dashboard

Based on the lessons learned from building a comprehensive TUI, our roadmap focuses on simplicity first. The next major milestone is a **web dashboard** that will provide:

### 🌐 Planned Web Interface
- **Clean Dashboard**: Simple, focused project overview
- **Task Management**: Web-based task completion and editing
- **Multi-Project View**: Easy switching between projects
- **Export & Sharing**: Better collaboration and reporting
- **Simple Visualizations**: Progress charts and dependency graphs

### 🎯 Development Philosophy
- **Simplicity First**: Learn from TUI complexity and keep it simple
- **Core Features Focus**: Prioritize essential functionality over advanced features
- **User-Centered**: Build what users actually need, not what's theoretically possible
- **Progressive Enhancement**: Start minimal, add features based on real usage

### 📝 AI Assistant Template

For AI assistance with external tools like ChatGPT or Claude, use this simple template:

```markdown
Create a simple development roadmap for a [PROJECT_TYPE] with basic task structure:

# Project: [PROJECT_NAME]

## MVP Phase
- [ ] Core task description #tag1 #tag2 (Priority: High)
- [ ] Another essential task #backend (Priority: Medium)

## Beta Phase  
- [ ] Enhancement feature #frontend (Priority: High)
- [ ] Testing task #testing (Priority: Medium)

## Release Phase
- [ ] Deployment task #deployment (Priority: Critical)
- [ ] Final optimization #performance (Priority: High)

## Future Phase
- [ ] Future improvements #enhancement (Priority: Low)

Keep it simple - focus on essential tasks only.
Format as markdown for 'rask init roadmap.md'
```

## 🛠️ Built With

- **[Rust](https://www.rust-lang.org/)** - Systems programming language
- **[clap](https://crates.io/crates/clap)** - Command-line argument parsing
- **[serde](https://crates.io/crates/serde)** - Serialization framework
- **[pulldown-cmark](https://crates.io/crates/pulldown-cmark)** - Markdown parsing
- **[colored](https://crates.io/crates/colored)** - Terminal color output
- **[ratatui](https://crates.io/crates/ratatui)** - Terminal UI framework

## 📈 Project Roadmap

### ✅ Current Status (v3.0.0)
- [x] Core CLI task management with Markdown sync
- [x] Interactive TUI with comprehensive dashboard
- [x] Multi-project workspace system
- [x] Advanced dependency management with visualization
- [x] Roadmap phases system with custom phase creation
- [x] Time estimation and tracking with variance analysis
- [x] Task filtering and search capabilities
- [x] Export capabilities (JSON, CSV, HTML) with phase information
- [x] Bulk operations for productivity
- [x] Task templates system
- [x] Timeline visualization with pagination
- [x] Configuration system
- [x] **Fixed TUI navigation freezing issues**

### 🎯 Next: Web Dashboard (v3.1.0)
- [ ] REST API backend for project data
- [ ] Web-based project overview dashboard
- [ ] Browser-based task management interface
- [ ] Real-time progress visualization
- [ ] Team collaboration features
- [ ] Web-based time tracking interface

### 🔮 Future Phases
- [ ] **Mobile Interface**: Companion app with offline sync
- [ ] **Advanced Analytics**: Project insights and reporting
- [ ] **External Integrations**: Git, GitHub, Jira, Slack connections
- [ ] **Enterprise Features**: Multi-tenant architecture

## 🎓 Lessons Learned

This project started with ambitious goals for a comprehensive TUI with AI integration, advanced analytics, and complex features. Through development, we learned valuable lessons:

### 🧠 Key Insights
- **Complexity vs. Usability**: More features don't always mean better user experience
- **Testing is Critical**: Complex features without proper testing lead to frustration
- **Simple is Better**: Users prefer focused tools that do core tasks well
- **Iterative Development**: Start small, validate with users, then expand

### 🔄 Course Correction
Moving forward, Rask focuses on:
- **Core CLI functionality** that works reliably
- **Simple TUI interface** for interactive task management
- **Web dashboard** as the next major milestone
- **User feedback** driving feature priorities

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by modern project management tools
- Built with the Rust community's excellent ecosystem
- Designed for developers who live in the terminal

---

**Ready to try simple, focused project planning?** 🎯

📖 **[Read the User Guide](USER_GUIDE.md)** for complete documentation and examples.
