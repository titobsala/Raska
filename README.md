# Rask 🚀 - Advanced CLI Project Planner

[![Version](https://img.shields.io/badge/version-2.3.1-blue.svg)](https://github.com/tito-sala/rask)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

**Rask** is a powerful, feature-rich command-line project planner built with Rust. It transforms simple Markdown files into sophisticated project management systems with advanced task tracking, dependency management, multi-project support, **roadmap phases**, and beautiful terminal UI.

## ✨ Key Features

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

4. **Add tasks with metadata, phases, and time estimates:**
```bash
rask add "Implement OAuth" --tag backend,auth --priority high --phase beta --depends-on 2 --estimated-hours 4.5
```

5. **Manage phases:**
```bash
# View phase overview
rask phase overview

# Set task phases
rask phase set 5 release

# View tasks in specific phase
rask phase show mvp
```

6. **Track time on tasks:**
```bash
rask start 1 --description "Working on OAuth integration"
# ... work on the task ...
rask stop
```

7. **Complete tasks:**
```bash
rask complete 1
```

8. **View time tracking information:**
```bash
rask time 1          # View time for specific task
rask time --summary  # Overall time tracking summary
```

9. **Export progress with phase and time information:**
```bash
rask export html -o progress_report.html --include-completed
```

## 📚 Documentation

For comprehensive documentation, examples, and advanced usage patterns, see the **[User Guide](USER_GUIDE.md)**.

### Quick Command Reference

| Command | Description |
|---------|-------------|
| `rask init <file.md>` | Initialize project from Markdown |
| `rask show` | Display project status |
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

### Phase Commands

| Command | Description |
|---------|-------------|
| `rask phase overview` | Show comprehensive phase statistics |
| `rask phase list` | List all phases with descriptions |
| `rask phase show <phase>` | Display tasks in specific phase |
| `rask phase set <id> <phase>` | Set phase for individual task |
| `rask phase create <name> [options]` | **NEW**: Create custom phases |
| `rask bulk set-phase <ids> <phase>` | Set phase for multiple tasks |

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

## 🎨 Terminal UI Preview

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

## 🤖 AI-Assisted Roadmap Generation

Rask provides comprehensive templates and examples that work perfectly with AI assistants like ChatGPT, Claude, or any other AI tool. While Rask doesn't have built-in AI integration yet, it's designed to work seamlessly with AI-generated content, dramatically speeding up project planning and ensuring consistent task structure.

### 📝 Copy-Paste Template for AI Assistants

Use this template with any AI assistant to generate a complete roadmap that works perfectly with Rask:

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

### 🔄 Workflow Integration

1. **Generate**: Use AI assistants with our templates to create your initial roadmap
2. **Initialize**: `rask init ai-generated-roadmap.md`
3. **Enhance**: Use `rask template` commands to add standardized tasks
4. **Execute**: Track progress with Rask's powerful features
5. **Iterate**: Export progress and feed back to AI for roadmap updates

> **Future Enhancement**: Direct AI integration is planned for future releases, which will automate this workflow even further.

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
│   └── utils.rs       # Shared utilities
├── model.rs           # Data models and types
├── cli.rs             # Command-line interface
├── main.rs            # Application entry point
└── ...                # UI, parsing, and utilities
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

### ✅ Completed Features (v2.3.1)
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

### 🚧 Phase 3: Enhanced Export Capabilities 📤 (v2.4.0 - Current Focus)
- [ ] Time data integration in all export formats (JSON, CSV, HTML)
- [ ] Advanced filtering in exports (date ranges, time thresholds)
- [ ] Report templates (sprint reports, time tracking reports)
- [ ] Export scheduling and automation
- [ ] Interactive HTML reports with charts and visualizations
- [ ] Custom export configurations and profiles

### 🔮 Future Development Phases

**Phase 4: Plugin System Foundation 🔌 (v3.0.0)**
- Plugin architecture design and safe sandbox environment
- Hook system for extending functionality
- Custom command plugins and integration plugins

**Phase 5: Web Dashboard Interface 🌐 (v3.1.0)**
- Local web server for dashboard with REST API
- Real-time project visualization and interactive time tracking
- Team collaboration features and multi-user support

**Phase 6: External Tool Integration 🔗 (v3.2.0)**
- GitHub integration (sync with issues, PRs, commits)
- Git integration (link commits to tasks)
- Calendar integration (time blocking, meeting tracking)
- Slack/Discord notifications and custom slash commands

**Phase 7+: Advanced Features 🚀 (v4.0.0+)**
- AI-powered task estimation and scheduling
- Mobile companion app with offline sync
- Enterprise features and multi-tenant architecture

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
