# Rask 🚀 - Advanced CLI Project Planner

[![Version](https://img.shields.io/badge/version-2.2.0-blue.svg)](https://github.com/tito-sala/rask)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

**Rask** is a powerful, feature-rich command-line project planner built with Rust. It transforms simple Markdown files into sophisticated project management systems with advanced task tracking, dependency management, multi-project support, and beautiful terminal UI.

## ✨ Key Features

### 🎯 **Core Task Management**
- Initialize projects from Markdown files with automatic sync
- Complete, add, edit, and remove tasks with rich metadata
- Beautiful, colored terminal output with progress tracking
- **NEW**: Detailed task view with comprehensive information

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
- **NEW**: Comprehensive configuration system

### 🔗 **Dependency Management**
- Define task dependencies with automatic validation
- Circular dependency detection and prevention
- Visual dependency tree exploration with ASCII art
- Dependency chain analysis and impact assessment
- Block task completion until dependencies are satisfied

### ⚡ **Power User Features**
- **NEW**: Bulk operations (complete, tag, prioritize multiple tasks)
- **NEW**: Export capabilities (JSON, CSV, HTML) with filtering
- **NEW**: Configuration system with user and project-specific settings
- **NEW**: Enhanced dependency tree visualization

### 📊 **Visualization & Analysis**
- Comprehensive dependency trees with ASCII art
- Progress bars and completion statistics
- Ready vs blocked task analysis
- Project overview dashboards
- **NEW**: Beautiful HTML exports with responsive design

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

4. **Add tasks with metadata:**
```bash
rask add "Implement OAuth" --tag backend,auth --priority high --depends-on 2
```

5. **Complete tasks:**
```bash
rask complete 1
```

6. **Export progress:**
```bash
rask export html -o progress_report.html
```

## 📚 Documentation

For comprehensive documentation, examples, and advanced usage patterns, see the **[User Guide](USER_GUIDE.md)**.

### Quick Command Reference

| Command | Description |
|---------|-------------|
| `rask init <file.md>` | Initialize project from Markdown |
| `rask show` | Display project status |
| `rask add <desc> [options]` | Add task with metadata |
| `rask complete <id>` | Complete a task |
| `rask view <id>` | View detailed task information |
| `rask list [filters]` | List and filter tasks |
| `rask dependencies [options]` | Analyze dependencies |
| `rask bulk <operation> <ids>` | Bulk operations on multiple tasks |
| `rask export <format> [options]` | Export to JSON/CSV/HTML |
| `rask config <operation>` | Manage configuration |
| `rask project <operation>` | Multi-project management |

## 🎨 Terminal UI Preview

```
════════════════════════════════════════════════════════════
  My Web App Project
  📁 Project: web-app (Main development project)
════════════════════════════════════════════════════════════
  Progress: [████████████████░░░░░░░░░░░░] 67% (8/12)

  📋 Tasks:
  ──────────────────────────────────────────────────
  ✓ ▶️ # 1 Set up development environment
  ✓ ⬆️ # 2 Design database schema
  ✓ 🔥 # 3 Create API endpoints #backend
  □ ⬆️ # 4 Implement authentication #backend #security
      🔗 Depends on: 2, 3
  □ ▶️ # 5 Write tests #testing
      🔗 Depends on: 4
  □ 🔥 # 6 Deploy to production #deployment
      🔗 Depends on: 5
  ──────────────────────────────────────────────────
  🎯 Great progress! 4 more tasks to go.
```

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

- [x] Core task management with Markdown sync
- [x] Enhanced filtering and search capabilities
- [x] Multi-project workspace system
- [x] Sophisticated dependency management
- [x] Configuration system with user/project settings
- [x] Detailed task view and analysis
- [x] Bulk operations for productivity
- [x] Export capabilities (JSON, CSV, HTML)
- [x] Enhanced dependency tree visualization
- [ ] Task templates and automation
- [ ] Time estimation and tracking
- [ ] Progress analytics and reporting
- [ ] Plugin system for extensibility
- [ ] Web dashboard interface
- [ ] Team collaboration features

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
