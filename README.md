# Rask 🚀 - Advanced CLI Project Planner

[![Version](https://img.shields.io/badge/version-2.3.0-blue.svg)](https://github.com/tito-sala/rask)
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

### 📊 **Visualization & Analysis**
- Comprehensive dependency trees with ASCII art
- Progress bars and completion statistics
- Ready vs blocked task analysis
- Project overview dashboards
- **NEW**: Beautiful HTML exports with responsive design
- **NEW**: Phase-based progress tracking and recommendations

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

4. **Add tasks with metadata and phases:**
```bash
rask add "Implement OAuth" --tag backend,auth --priority high --phase beta --depends-on 2
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

6. **Complete tasks:**
```bash
rask complete 1
```

7. **Export progress with phase information:**
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
| `rask add <desc> [options]` | Add task with metadata and phase |
| `rask complete <id>` | Complete a task |
| `rask view <id>` | View detailed task information |
| `rask list [filters]` | List and filter tasks |
| `rask phase <operation>` | **NEW**: Manage roadmap phases |
| `rask dependencies [options]` | Analyze dependencies |
| `rask bulk <operation> <ids>` | Bulk operations on multiple tasks |
| `rask export <format> [options]` | Export to JSON/CSV/HTML with phases |
| `rask config <operation>` | Manage configuration |
| `rask project <operation>` | Multi-project management |

### Phase Commands

| Command | Description |
|---------|-------------|
| `rask phase overview` | Show comprehensive phase statistics |
| `rask phase list` | List all phases with descriptions |
| `rask phase show <phase>` | Display tasks in specific phase |
| `rask phase set <id> <phase>` | Set phase for individual task |
| `rask bulk set-phase <ids> <phase>` | Set phase for multiple tasks |

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
  ✓ ⬆️ # 2 Design database schema 🚀
  ✓ 🔥 # 3 Create API endpoints #backend 🧪
  □ ⬆️ # 4 Implement authentication #backend #security 🧪
      🔗 Depends on: 2, 3
  □ ▶️ # 5 Write tests #testing 🎯
      🔗 Depends on: 4
  □ 🔥 # 6 Deploy to production #deployment 🎯
      🔗 Depends on: 5
  ──────────────────────────────────────────────────
  
  📊 Phase Overview:
  🚀 MVP: 2/3 tasks (67% complete)
  🧪 Beta: 1/2 tasks (50% complete)  
  🎯 Release: 0/2 tasks (0% complete)
  
  🎯 Great progress! Focus on Beta phase next.
```

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

### ✅ Completed Features
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

### 🚧 In Progress
- [ ] Task templates and automation
- [ ] Time estimation and tracking
- [ ] Progress analytics and reporting

### 🔮 Future Plans
- [ ] Plugin system for extensibility
- [ ] Web dashboard interface
- [ ] Team collaboration features
- [ ] Integration with external tools (GitHub, Jira, etc.)
- [ ] Advanced reporting and analytics
- [ ] Mobile companion app

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
