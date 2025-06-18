# Rask 🚀 - Advanced CLI Project Planner

[![Version](https://img.shields.io/badge/version-2.8.0-blue.svg)](https://github.com/tito-sala/rask)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

**Rask** is a powerful, feature-rich command-line project planner built with Rust. It transforms simple Markdown files into sophisticated project management systems with advanced task tracking, dependency management, multi-project support, **roadmap phases**, **interactive TUI interface**, and beautiful terminal UI.

## ✨ Key Features

### 🖥️ **Interactive TUI Interface**
- **NEW**: Full-featured Terminal User Interface with real-time interaction
- **NEW**: Multi-view dashboard system (Home, Task Manager, AI Assistant, Templates, Analytics, Settings, Project Switcher)
- **NEW**: Project switcher for seamless navigation between projects
- **NEW**: Keyboard shortcuts (F1-F7) for instant view switching
- **NEW**: Real-time task completion toggling and progress updates
- **NEW**: Settings persistence and customizable interface
- **NEW**: Context-aware help system and navigation guidance

### 🎯 **Core Task Management**
- Initialize projects from Markdown files with automatic sync
- Complete, add, edit, and remove tasks with rich metadata
- Beautiful, colored terminal output with progress tracking
- **NEW**: Detailed task view with comprehensive information

### 🏗️ **Roadmap Phases System**
- **NEW**: Organize tasks by development lifecycle phases (MVP, Beta, Release, Future, Backlog)
- **NEW**: Create custom phases with personalized names, descriptions, and emojis
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
- **NEW**: Phase-specific task filtering and display

### 🏢 **Multi-Project Workspace**
- Manage multiple projects simultaneously
- Project-specific state files and configurations
- Easy project switching and isolation
- **NEW**: Comprehensive configuration system

### 🔗 **Dependency Management**
- Define task dependencies with automatic validation
- Circular dependency detection and prevention
- Visual dependency tree exploration with ASCII art
- Dependency chain analysis and impact assessment
- Block task completion until dependencies are satisfied

### ⚡ **Power User Features**
- **NEW**: Interactive TUI mode with comprehensive project management dashboard
- **NEW**: Bulk operations (complete, tag, prioritize, set phases for multiple tasks)
- **NEW**: Export capabilities (JSON, CSV, HTML) with filtering and phase information
- **NEW**: Configuration system with user and project-specific settings
- **NEW**: Enhanced dependency tree visualization
- **NEW**: Modular architecture for better maintainability and extensibility
- **NEW**: Time estimation and tracking with variance analysis
- **NEW**: Session-based time tracking with optional descriptions

### 📊 **Visualization & Analysis**
- Comprehensive dependency trees with ASCII art
- Progress bars and completion statistics
- Ready vs blocked task analysis
- Project overview dashboards
- **NEW**: Beautiful HTML exports with responsive design
- **NEW**: Phase-based progress tracking and recommendations
- **NEW**: Timeline view with horizontal phase progression and pagination
- **NEW**: Phase-grouped displays with individual progress bars

### 📋 **Task Templates System**
- **NEW**: Pre-configured task patterns for consistent task creation
- Built-in templates for common development scenarios (Bug Fix, Feature Implementation, etc.)
- Custom template creation with full metadata support
- Template categories (Development, Testing, Documentation, DevOps, Design, etc.)
- AI integration examples for automated roadmap generation
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

**NEW**: Launch the powerful interactive Terminal User Interface:

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
- **🤖 AI Assistant**: Chat interface for task planning and assistance
- **📄 Templates**: Browse and apply task templates
- **📊 Analytics**: Project progress visualization and insights
- **⚙️ Settings**: Customize TUI behavior and appearance
- **🔄 Project Switcher**: Seamlessly navigate between multiple projects

**Navigation:**
- **F1-F7**: Quick view switching
- **Tab**: Cycle through interface panels
- **↑↓**: Navigate lists and options
- **Enter/Space**: Toggle tasks or select options
- **p/F7**: Open project switcher
- **h**: Show help and keyboard shortcuts
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
| `rask interactive [options]` | **NEW**: Launch interactive TUI with full dashboard |
| `rask init <file.md>` | Initialize project from Markdown |
| `rask show [options]` | Display project status with phase grouping and filtering |
| `rask timeline [options]` | **NEW**: Show horizontal timeline with phase progression and pagination |
| `rask add <desc> [options]` | Add task with metadata, phase, and time estimate |
| `rask complete <id>` | Complete a task |
| `rask view <id>` | View detailed task information |
| `rask list [filters]` | List and filter tasks |
| `rask phase <operation>` | **NEW**: Manage roadmap phases |
| `rask dependencies [options]` | Analyze dependencies |
| `rask start <id> [options]` | **NEW**: Start time tracking for a task |
| `rask stop` | **NEW**: Stop current time tracking session |
| `rask time [id] [options]` | **NEW**: View time tracking information |
| `rask bulk <operation> <ids>` | Bulk operations on multiple tasks |
| `rask export <format> [options]` | Export to JSON/CSV/HTML with phases and time data |
| `rask config <operation>` | Manage configuration |
| `rask project <operation>` | Multi-project management |
| `rask template <operation>` | **NEW**: Manage task templates |

### Interactive TUI Commands

| Keyboard Shortcut | Description |
|-------------------|-------------|
| `F1` | Home Dashboard - Project overview and quick actions |
| `F2` | Task Manager - Interactive task list with completion |
| `F3` | AI Assistant - Chat interface for task planning |
| `F4` | Templates - Browse and apply task templates |
| `F5` | Analytics - Progress visualization and insights |
| `F6` | Settings - Customize TUI behavior |
| `F7` / `p` | Project Switcher - Navigate between projects |
| `Tab` | Cycle through interface panels |
| `↑↓` | Navigate lists and options |
| `Enter` / `Space` | Toggle tasks or select options |
| `h` | Show help and keyboard shortcuts |
| `r` | Refresh project data |
| `s` | Save current settings |
| `q` / `Ctrl+C` | Quit application |

### Phase Commands

| Command | Description |
|---------|-------------|
| `rask phase overview` | Show comprehensive phase statistics |
| `rask phase list` | List all phases with descriptions |
| `rask phase show <phase>` | Display tasks in specific phase |
| `rask phase set <id> <phase>` | Set phase for individual task |
| `rask phase create <name> [options]` | **NEW**: Create custom phases |
| `rask bulk set-phase <ids> <phase>` | Set phase for multiple tasks |

### Timeline & Visualization Commands

| Command | Description |
|---------|-------------|
| `rask timeline` | Show horizontal timeline with all phases (paginated) |
| `rask timeline --page <n>` | **NEW**: Navigate to specific page (default: 5 phases per page) |
| `rask timeline --page-size <n>` | **NEW**: Set number of phases per page |
| `rask timeline --compact` | Use compact view for more information |
| `rask timeline --active-only` | Show only phases containing tasks |
| `rask show --group-by-phase` | **NEW**: Group tasks by phase with progress bars |
| `rask show --phase <name>` | **NEW**: Filter display to specific phase |
| `rask show --collapse-completed` | **NEW**: Collapse completed phases |

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
| `rask template examples` | Show help and AI integration examples |
| `rask template generate <desc> [options]` | **NEW**: AI-powered template generation |
| `rask template suggest [options]` | **NEW**: AI template suggestions for project |
| `rask template enhance <name> [options]` | **NEW**: AI template enhancement |

### AI Commands

| Command | Description |
|---------|-------------|
| `rask ai chat [message] [options]` | **NEW**: Interactive AI chat with project context |
| `rask ai analyze [options]` | **NEW**: AI analysis of current tasks and project health |
| `rask ai breakdown <description> [options]` | **NEW**: Break down complex tasks using AI |
| `rask ai insights [options]` | **NEW**: Get AI-powered project insights and recommendations |
| `rask ai configure [options]` | **NEW**: Configure AI settings and API keys |
| `rask ai summary [options]` | **NEW**: Generate AI-powered project status summary |
| `rask ai suggest [options]` | **NEW**: Get AI suggestions for next tasks |

## 🎨 Interactive TUI Preview

**NEW**: Launch the interactive Terminal User Interface for a modern project management experience:

```
┌─ 🚀 Rask TUI • Task Manager ──────────────────────────────────────────────────┐
│  🏠 Home │📝 Tasks │🤖 AI │📄 Templates │📊 Analytics │⚙️ Settings │🔄 Projects │
└────────────────────────────────────────────────────────────────────────────────┘
┌─ 📋 My Web App Project • 8/12 tasks (67%) ──────────┐ ┌─ 🤖 Task Assistant (3) ─┐
│                                                      │ │                          │
│ ┌─ Tasks (1-8/12) ──────────────────────────────────┐ │ │ AI Assistant:           │
│ │ ▶ ✅ 🔥 #1 Set up development environment 🚀      │ │ │ Hello! I'm here to help │
│ │   ✅ ⬆️ #2 Design database schema 🚀 [2.8h]      │ │ │ you manage your tasks.  │
│ │   ✅ 🔥 #3 Create API endpoints #backend 🧪      │ │ │                          │
│ │   □ ⬆️ #4 Implement authentication 🧪 [4.5h]     │ │ │                          │
│ │       🔗 Depends on: 2, 3                        │ │ │                          │
│ │   ⏱️ ▶️ #5 Write tests #testing 🎯 [1.2h tracked] │ │ │                          │
│ │       🔗 Depends on: 4 | 🕐 Active session       │ │ │                          │
│ │   □ 🔥 #6 Deploy to production #deployment 🎯    │ │ │                          │
│ │       🔗 Depends on: 5                           │ │ └─────────────────────────┘
│ └──────────────────────────────────────────────────┘ │ ┌─ 💬 Ask about tasks ──┐
│                                                      │ │ _                       │
└──────────────────────────────────────────────────────┘ └─────────────────────────┘

F1-F7: Switch views • Tab: Change focus • ↑↓: Navigate • Enter/Space: Toggle • Q: Quit
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

## 🤖 AI-Powered Project Management

Rask includes comprehensive AI integration powered by Google Gemini, transforming how you plan, manage, and execute projects. The AI system provides intelligent assistance for task breakdown, project analysis, template generation, and strategic guidance.

### 🧠 Built-in AI Features

**AI Command Interface:**
- `rask ai chat` - Interactive AI conversations with project context
- `rask ai analyze` - AI-powered task and project health analysis  
- `rask ai breakdown <description>` - Break down complex tasks automatically
- `rask ai insights` - Get strategic project insights and recommendations
- `rask ai suggest` - AI suggestions for next logical tasks
- `rask ai summary` - Generate comprehensive project status reports

**AI Template System:**
- `rask template generate <description>` - Generate contextual templates with AI
- `rask template suggest` - AI suggestions for missing template types
- `rask template enhance <name>` - AI-powered template improvements

**Interactive TUI Integration:**
- F3: AI Assistant view with real-time chat interface
- Context-aware AI responses based on current project state
- Seamless integration between CLI and TUI AI features

### 📝 External AI Assistant Template

For additional AI assistance with external tools, use this template:

```markdown
Create a development roadmap for a [PROJECT_TYPE] with the following structure. 
Each task should include appropriate tags, priorities, phases, and implementation notes:

# Project: [PROJECT_NAME]

## MVP Phase
- [ ] Task description #tag1 #tag2 (Priority: High)
  Notes: Implementation details and considerations
  
- [ ] Another MVP task #backend #database (Priority: Medium)
  Notes: Database setup and configuration requirements

## Beta Phase  
- [ ] Beta feature #frontend #ui (Priority: High)
  Notes: User interface improvements and testing
  
- [ ] Integration testing #testing #qa (Priority: Medium)
  Notes: End-to-end testing scenarios and acceptance criteria

## Release Phase
- [ ] Production deployment #devops #deployment (Priority: Critical)
  Notes: Deployment checklist and rollback procedures
  
- [ ] Performance optimization #performance (Priority: High)
  Notes: Profiling and optimization targets

## Future Phase
- [ ] Advanced features #enhancement (Priority: Low)
  Notes: Future improvements and feature requests

## Custom Phases (Optional)
You can also request custom phases for specific project needs:

## Planning Phase
- [ ] Market research #research #planning (Priority: High)
  Notes: Competitive analysis and user research

## Design Phase  
- [ ] UI/UX mockups #design #frontend (Priority: High)
  Notes: User interface design and prototyping

IMPORTANT: When generating Notes sections, use plain text descriptions 
instead of bullet points or numbered lists to prevent parsing issues.
Use comma-separated items or flowing text instead.

Please format it as a markdown file that I can use with 'rask init roadmap.md'
```

### 🎯 Example AI Prompts

**For Web Applications:**
```
Create a development roadmap for a web application with user authentication, 
real-time chat, and file sharing capabilities. Include MVP, Beta, Release, 
and Future phases with appropriate priorities and implementation notes.
```

**For Mobile Apps:**
```
Generate a roadmap for a mobile fitness tracking app with social features. 
Include tasks for iOS/Android development, backend API, user onboarding, 
and analytics integration.
```

**For API Projects:**
```
Create a roadmap for a RESTful API with authentication, rate limiting, 
documentation, and monitoring. Focus on scalability and security.
```

**For Custom Phases:**
```
Create a development roadmap with custom phases for a [PROJECT_TYPE]. 
Include these phases: Planning, Design, Development, Testing, Deployment, Maintenance.
Each phase should have appropriate tasks with tags, priorities, and implementation notes.
```

### ✨ AI-Assisted Roadmap Benefits

- **Comprehensive Coverage**: AI assistants help ensure no critical tasks are missed
- **Proper Prioritization**: Smart priority assignment based on dependencies
- **Phase Organization**: Logical grouping of tasks by development lifecycle
- **Implementation Notes**: Detailed technical considerations for each task
- **Consistent Tagging**: Standardized tags for better organization
- **Time-Saving**: Generate complete roadmaps in seconds instead of hours
- **Template Integration**: Use Rask's task templates to standardize recurring tasks

### 🔄 AI-Powered Workflow

1. **AI Analysis**: `rask ai analyze` - Get intelligent project health assessment
2. **Smart Planning**: `rask ai breakdown "complex feature"` - Break down tasks automatically  
3. **Template Generation**: `rask template generate "API development"` - Create contextual templates
4. **Continuous Insights**: `rask ai insights` - Get strategic recommendations
5. **Interactive Chat**: `rask ai chat` or F3 in TUI - Real-time AI assistance
6. **Template Enhancement**: `rask template enhance` - Improve existing templates with AI

> **Advanced AI Features**: The AI system learns from your project patterns and provides increasingly personalized assistance.

## 🏗️ Architecture

Rask features a **modular architecture** designed for maintainability and extensibility:

```
src/
├── commands/           # Modular command system
│   ├── mod.rs         # Public API and routing
│   ├── core.rs        # Core task operations
│   ├── phases.rs      # Phase management system
│   ├── project.rs     # Multi-project support
│   ├── bulk.rs        # Bulk operations
│   ├── export.rs      # Export functionality
│   ├── config.rs      # Configuration management
│   ├── dependencies.rs # Dependency analysis
│   ├── analytics.rs   # Analytics and insights
│   ├── interactive.rs # Interactive TUI mode
│   ├── templates.rs   # Template management
│   ├── ai.rs         # AI-powered commands
│   ├── notes.rs      # Implementation notes
│   └── utils.rs      # Shared utilities
├── ai/                 # AI integration modules
│   ├── service.rs     # AI service layer
│   ├── models.rs      # AI data models
│   ├── gemini.rs      # Google Gemini provider
│   └── mod.rs         # AI module exports
├── cli/               # CLI argument parsing
│   ├── ai.rs         # AI command definitions
│   ├── template.rs   # Template command definitions
│   ├── bulk.rs       # Bulk operation definitions
│   ├── config.rs     # Config command definitions
│   ├── notes.rs      # Notes command definitions
│   ├── phase.rs      # Phase command definitions
│   ├── project.rs    # Project command definitions
│   └── types.rs      # Shared CLI types
├── ui/                # Terminal UI components
│   ├── analytics.rs  # Analytics visualization
│   ├── dependencies.rs # Dependency visualization
│   ├── messages.rs   # Message display utilities
│   ├── progress.rs   # Progress bars and indicators
│   ├── projects.rs   # Project display utilities
│   ├── roadmap.rs    # Roadmap visualization
│   ├── tasks.rs      # Task display utilities
│   └── helpers.rs    # UI helper functions
├── model.rs           # Data models and types
├── cli.rs             # Command-line interface
├── config.rs         # Configuration management
├── project.rs        # Project state management
├── state.rs          # Application state
├── parser.rs         # Markdown parsing
├── markdown_writer.rs # Markdown file sync
└── main.rs           # Application entry point
```

**Benefits:**
- **Single Responsibility**: Each module has a focused purpose
- **Better Maintainability**: Easier testing, debugging, and code reviews
- **Improved Developer Experience**: Better IDE support and faster compilation
- **Future-Proof**: Easy to extend with new features

## 🛠️ Built With

- **[Rust](https://www.rust-lang.org/)** - Systems programming language
- **[clap](https://crates.io/crates/clap)** - Command-line argument parsing
- **[serde](https://crates.io/crates/serde)** - Serialization framework
- **[pulldown-cmark](https://crates.io/crates/pulldown-cmark)** - Markdown parsing
- **[colored](https://crates.io/crates/colored)** - Terminal color output
- **[chrono](https://crates.io/crates/chrono)** - Date and time handling
- **[toml](https://crates.io/crates/toml)** - Configuration file parsing
- **[dirs](https://crates.io/crates/dirs)** - Platform-specific directories

## 📈 Roadmap

### ✅ Completed Features (v2.8.0)
- [x] Core task management with Markdown sync
- [x] Enhanced filtering and search capabilities
- [x] Multi-project workspace system
- [x] Sophisticated dependency management
- [x] Configuration system with user/project settings
- [x] Detailed task view and analysis
- [x] Bulk operations for productivity
- [x] Export capabilities (JSON, CSV, HTML)
- [x] Enhanced dependency tree visualization
- [x] **Modular architecture for better maintainability**
- [x] **Roadmap phases system (MVP, Beta, Release, Future, Backlog)**
- [x] **Custom phase creation with personalized names, descriptions, and emojis**
- [x] **Task templates system with built-in and custom templates**
- [x] **Time estimation and tracking with variance analysis**
- [x] **Session-based time tracking with start/stop functionality**
- [x] **Phase-based task organization with dynamic phase detection**
- [x] **Timeline view with horizontal phase progression**
- [x] **Timeline pagination for managing large numbers of phases**
- [x] **Phase-grouped displays with individual progress bars**
- [x] **Phase-specific filtering and visualization options**
- [x] **Interactive TUI with comprehensive multi-view dashboard**
- [x] **Project switcher for seamless navigation between projects**
- [x] **Real-time task completion toggling and progress updates**
- [x] **Settings persistence and customizable TUI interface**
- [x] **Context-aware help system and keyboard navigation**

### ✅ Phase 4: Advanced Phase Visualization 📊 (v2.6.2 - Completed)
- [x] **Phase-grouped task displays with individual progress bars**
- [x] **Timeline view showing horizontal phase progression**
- [x] **Dynamic phase detection from roadmap data (no hardcoded phases)**
- [x] **Timeline pagination for managing large numbers of phases**
- [x] **Phase-specific filtering options**
- [x] **Collapsible completed phases to reduce visual clutter**
- [x] **Enhanced UI layouts with multi-column sectioned displays**
- [x] **Comprehensive phase statistics and navigation**

### ✅ Phase 5: AI-Powered Product Development Environment 🤖 (v2.7.0 - v2.8.0 - Completed)
- [x] **Core AI Infrastructure with Google Gemini integration**
- [x] **Intelligent Task Analysis with AI-powered insights**
- [x] **CLI Command Interface with interactive AI chat**
- [x] **Advanced Template Integration** (generate, suggest, enhance)
- [x] **Comprehensive Interactive TUI System**
- [x] **Multi-view dashboard with navigation bar (Home, Tasks, AI, Templates, Analytics, Settings, Projects)**
- [x] **Real-time task completion toggling and progress updates**
- [x] **AI Assistant interface with chat functionality**
- [x] **Templates view with AI-powered generation and suggestions**
- [x] **Analytics view with progress visualization and insights**
- [x] **Settings persistence and customizable TUI interface**
- [x] **Professional TUI experience with keyboard navigation (F1-F7)**

### 🚧 Phase 6: Web Dashboard Interface 🌐 (v2.8.0 - Next Major Phase)
- [ ] **Dashboard Backend** - REST API for project data with WebSocket support
- [ ] **Interactive Frontend** - Real-time project visualization and charts
- [ ] **Web-based Time Tracking** - Browser-based time tracking interface
- [ ] **AI Integration Interface** - Web-based AI assistant chat
- [ ] **Team Collaboration Features** - Multi-user support and real-time updates

### 🔮 Future Development Phases

**Phase 7: Advanced AI Features 🧠 (v2.9.0)**
- Structured collaboration framework with living sources of truth
- Template-driven product workflows with cross-functional integration
- Historical data analysis and pattern recognition
- Smart project optimization and workflow automation

**Phase 8: Plugin System Foundation 🔌 (v3.0.0)**
- Core plugin framework with safe sandbox environment
- Hook system integration for pre/post command triggers
- Built-in plugin examples for custom commands and integrations

**Phase 9: External Tool Integration 🔗 (v3.1.0)**
- Git/GitHub integration with automatic task linking
- Calendar integration with time blocking and sync
- Communication platform integration (Slack/Discord)
- Project management tool sync (Jira, Trello, Linear)

**Phase 10+: Mobile & Enterprise 📱🏢 (v3.2.0+)**
- Mobile companion app with offline synchronization
- Enterprise features with multi-tenant architecture
- Advanced AI analytics and compliance features

> 📋 **Complete Development Plan**: See [roadmap.md](roadmap.md) for detailed phase breakdown, timelines, and technical specifications.

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

**Ready to supercharge your project planning?** 🎯

📖 **[Read the User Guide](USER_GUIDE.md)** for comprehensive documentation and examples.
