# Rask Technical Documentation

## Table of Contents
- [Architecture Overview](#architecture-overview)
- [Backend Documentation](#backend-documentation)
- [Frontend Documentation](#frontend-documentation)
- [API Reference](#api-reference)
- [Development Setup](#development-setup)
- [Deployment](#deployment)
- [Troubleshooting](#troubleshooting)

---

## Architecture Overview

### System Architecture
Rask implements a **hybrid CLI + Web architecture** with the following components:

```
┌─────────────────────────────────────────────────────────────┐
│                     Rask CLI (Rust)                        │
│  ┌─────────────────┬─────────────────┬─────────────────┐   │
│  │   Core Engine   │   AI Services   │  Web Server     │   │
│  │                 │                 │                 │   │
│  │ • State Mgmt    │ • Gemini API    │ • Axum Server   │   │
│  │ • Task CRUD     │ • Chat Context  │ • WebSocket     │   │
│  │ • Markdown      │ • Streaming     │ • File Watch    │   │
│  │   Parsing       │ • Suggestions   │ • Static Files  │   │
│  └─────────────────┴─────────────────┴─────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                              │
                              │ HTTP/WS + File System
                              │
┌─────────────────────────────────────────────────────────────┐
│                React Frontend (TypeScript)                 │
│  ┌─────────────────┬─────────────────┬─────────────────┐   │
│  │   Components    │   State Mgmt    │   API Client    │   │
│  │                 │                 │                 │   │
│  │ • Dashboard     │ • Zustand       │ • REST API      │   │
│  │ • Task Views    │ • React Query   │ • WebSocket     │   │
│  │ • AI Chat       │ • Local State   │ • Health Check  │   │
│  │ • Visualizations│ • Error Boundary│ • Error Handle  │   │
│  └─────────────────┴─────────────────┴─────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                              │
                              │ File System
                              │
┌─────────────────────────────────────────────────────────────┐
│                     Local Storage                          │
│                                                             │
│     .rask/                                                  │
│     ├── state.json          # Main project data            │
│     ├── config.toml         # Project configuration        │
│     ├── project-overview.md # Human-readable summary       │
│     ├── task-details.md     # Editable task metadata       │
│     └── ai/                 # AI conversation history      │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Data Flow
1. **CLI Operations** → Update `.rask/state.json` → File Watcher → WebSocket → Frontend Update
2. **Web Operations** → API Call → Update `.rask/state.json` → File Watcher → WebSocket → Frontend Update
3. **Real-time Sync** → Both CLI and Web interfaces stay synchronized via file watching

---

## Backend Documentation

### Technology Stack
- **Language**: Rust 1.70+
- **Web Framework**: Axum 0.7
- **Serialization**: Serde + serde_json
- **WebSocket**: tokio-tungstenite
- **File Watching**: notify 6.0
- **AI Integration**: Custom Gemini API client
- **Static Assets**: include_dir (embedded React build)

### Core Modules

#### `src/model.rs`
**Purpose**: Core data structures and business logic

**Key Types**:
```rust
pub struct Roadmap {
    pub title: String,
    pub tasks: Vec<Task>,
    pub source_file: Option<String>,
    pub metadata: ProjectMetadata,
    pub project_id: Option<String>,
}

pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: TaskStatus,       // Pending, Completed
    pub priority: Priority,       // Low, Medium, High, Critical
    pub phase: Phase,
    pub tags: Vec<String>,
    pub dependencies: Vec<u32>,
    pub created_at: String,
    pub completed_at: Option<String>,
    // ... additional fields
}

pub struct Phase {
    pub name: String,
    pub description: Option<String>,
    pub emoji: Option<String>,
}
```

#### `src/state.rs`
**Purpose**: Local file-based state management

**Key Functions**:
```rust
pub fn save_state(roadmap: &Roadmap) -> Result<(), Error>
pub fn load_state() -> Result<Roadmap, Error>
pub fn has_local_workspace() -> bool
```

**Storage Location**: `./.rask/state.json` (relative to current directory)

#### `src/web/` Module
**Purpose**: Web server implementation

**Structure**:
```
src/web/
├── mod.rs           # Module entry point
├── server.rs        # Axum server setup + graceful shutdown
├── websocket.rs     # WebSocket connection management
├── watcher.rs       # File system watching for real-time sync
├── handlers/        # Request handlers
└── routes/          # Route definitions
    ├── mod.rs       # Route organization + health endpoint
    ├── projects.rs  # Project management endpoints
    ├── tasks.rs     # Task CRUD endpoints
    ├── ai.rs        # AI integration endpoints
    └── static_files.rs # Embedded React app serving
```

### API Endpoints

#### Health & Status
```
GET /api/health
Response: {"status": "ok", "service": "rask-web", "version": "3.3.0"}
```

#### Project Management
```
GET /api/projects
Response: {
  "projects": [{
    "name": "current",
    "display_name": "Project Title", 
    "task_count": 9,
    "completed_tasks": 3,
    "phases": [...]
  }]
}

GET /api/projects/{name}
Response: {
  "title": "Project Title",
  "tasks": [...],
  "phases": [...],
  "metadata": {...}
}

GET /api/projects/{name}/tasks
Response: [
  {
    "id": 1,
    "description": "Task description",
    "status": "completed",  // Normalized for frontend
    "priority": "High",
    "phase": {...},
    "dependencies": [2, 3]
  }
]

GET /api/projects/{name}/dependencies  
Response: {
  "nodes": [...],
  "links": [...]
}

GET /api/projects/{name}/analytics
Response: {
  "total_tasks": 9,
  "completed_tasks": 3,
  "completion_rate": 33.3,
  "phase_breakdown": {...}
}
```

#### WebSocket
```
WS /ws/projects/{name}
Messages: {
  "type": "task_updated" | "project_modified" | "config_changed",
  "data": {...}
}
```

### Key Backend Features

#### File Watching & Real-time Sync
- **Implementation**: `notify` crate with debounced events
- **Watched Files**: `.rask/state.json`, `.rask/config.toml`
- **Event Handling**: File changes → WebSocket broadcast → Frontend update
- **Debouncing**: 500ms to prevent spam during rapid file changes

#### Status Normalization
Backend converts internal status format to frontend-compatible format:
```rust
// Internal: TaskStatus::Completed, TaskStatus::Pending
// API Response: "completed", "todo"
```

#### Error Handling
- **File Not Found**: 404 with descriptive message
- **Invalid JSON**: 500 with parsing error details
- **WebSocket Errors**: Automatic reconnection with exponential backoff

---

## Frontend Documentation

### Technology Stack
- **Framework**: React 18 + TypeScript 5
- **Build Tool**: Vite 5
- **Styling**: Tailwind CSS 3
- **State Management**: Zustand 4
- **Data Fetching**: TanStack React Query 5
- **Visualizations**: D3.js 7
- **WebSocket**: Native WebSocket API

### Project Structure
```
web-ui/
├── src/
│   ├── components/         # React components
│   │   ├── Dashboard/      # Project overview components
│   │   ├── Layout/         # App layout and navigation
│   │   ├── AI/             # AI chat interface
│   │   ├── Dependencies/   # Dependency graph visualization
│   │   └── UI/             # Reusable UI components
│   ├── pages/              # Top-level page components
│   ├── stores/             # Zustand state stores
│   ├── api/                # API client and WebSocket
│   ├── types/              # TypeScript type definitions
│   └── utils/              # Utility functions
├── public/                 # Static assets
└── dist/                   # Build output (embedded in Rust binary)
```

### State Management Architecture

#### App Store (`stores/appStore.ts`)
**Central state management using Zustand**:

```typescript
interface AppState {
  // Data state
  projects: ProjectInfo[];
  currentProject: Roadmap | null;
  tasks: Task[];
  isLoading: boolean;
  error: string | null;
  
  // UI state
  selectedProject?: string;
  activeTab: 'dashboard' | 'tasks' | 'dependencies' | 'analytics' | 'ai-chat';
  filters: TaskFilters;
  sidebarOpen: boolean;
  
  // Actions
  setCurrentProject: (project: Roadmap | null) => void;
  updateTask: (taskId: number, updates: Partial<Task>) => void;
  // ... other actions
}
```

#### Data Fetching with React Query
```typescript
// Automatic project loading
const { data: projectData } = useQuery({
  queryKey: ['project', selectedProject],
  queryFn: () => api.projects.getProject(selectedProject!),
  enabled: !!selectedProject,
  refetchInterval: 30000, // Background refresh
});

// Health check monitoring
const { data: healthData } = useQuery({
  queryKey: ['health'],
  queryFn: () => api.health.check(),
  refetchInterval: 30000,
  retry: 1,
});
```

### Key Frontend Components

#### Dashboard (`components/Dashboard/`)
- **ProjectOverview**: High-level project statistics
- **TasksOverview**: Task breakdown by priority and status
- **PhaseProgress**: Visual progress bars for each phase
- **RecentActivity**: Task change timeline

#### Dependency Graph (`components/Dependencies/DependencyGraph.tsx`)
- **Technology**: D3.js force-directed graph
- **Features**: Interactive nodes, zoom/pan, filtering, tooltips
- **Data**: Real task dependency relationships

#### AI Chat (`components/AI/ChatInterface.tsx`)
- **Features**: Streaming responses, message history, project context
- **Technology**: Server-Sent Events (SSE) for real-time streaming
- **State**: Persistent chat sessions with project awareness

### Type System

#### Core Types (`types/index.ts`)
```typescript
export interface Task {
  id: number;
  description: string;
  status: TaskStatus;          // 'todo' | 'in-progress' | 'completed' | 'blocked'
  priority: Priority;          // 'Low' | 'Medium' | 'High' | 'Critical'
  phase: Phase;
  tags: string[];
  dependencies: number[];
  created_at: string;
  completed_at?: string;
}

export interface Phase {
  name: string;
  emoji: string;
  description: string;
  custom: boolean;
}

export interface Roadmap {
  title: string;
  tasks: Task[];
  phases: Phase[];              // Added for frontend compatibility
  source_file?: string;
  metadata: ProjectMetadata;
}
```

### Error Handling

#### Error Boundary
```typescript
// Catches React component errors
<ErrorBoundary>
  <App />
</ErrorBoundary>
```

#### API Error Handling
```typescript
class ApiError extends Error {
  constructor(
    message: string,
    public status: number,
    public response?: any
  ) {
    super(message);
    this.name = 'ApiError';
  }
}
```

#### Health Check Integration
```typescript
// Shows connection status
{!healthData && (
  <div className="bg-yellow-50 border-b border-yellow-200">
    <p>Connecting to Rask server...</p>
  </div>
)}
```

---

## API Reference

### Authentication
**None required** - Local development server

### Base URL
`http://127.0.0.1:3000/api`

### Content Type
`application/json` for all requests

### Response Format
```typescript
// Success Response
{
  "data": any,           // Requested data
  "status": "success"    // Optional status field
}

// Error Response  
{
  "error": string,       // Error message
  "status": number       // HTTP status code
}
```

### Endpoint Details

#### GET /api/health
**Purpose**: Server health check  
**Parameters**: None  
**Response**:
```json
{
  "status": "ok",
  "service": "rask-web", 
  "version": "3.3.0"
}
```

#### GET /api/projects
**Purpose**: List available projects  
**Parameters**: None  
**Response**:
```json
{
  "projects": [
    {
      "name": "current",
      "display_name": "My Project",
      "task_count": 15,
      "completed_tasks": 7,
      "phases": [
        {
          "name": "MVP",
          "description": "Minimum viable product",
          "emoji": "🚀"
        }
      ]
    }
  ]
}
```

#### GET /api/projects/{name}
**Purpose**: Get complete project details  
**Parameters**: 
- `name` (path): Project identifier (usually "current")  
**Response**: Complete `Roadmap` object with tasks and phases

#### GET /api/projects/{name}/tasks
**Purpose**: Get project tasks with optional filtering  
**Parameters**:
- `name` (path): Project identifier
- `tag` (query): Filter by tag
- `priority` (query): Filter by priority (Low|Medium|High|Critical)
- `phase` (query): Filter by phase name
- `status` (query): Filter by status (todo|completed|in-progress|blocked)
- `search` (query): Text search in description/notes  
**Response**: Array of `Task` objects

#### GET /api/projects/{name}/dependencies
**Purpose**: Get dependency graph data for visualization  
**Parameters**: 
- `name` (path): Project identifier  
**Response**:
```json
{
  "nodes": [
    {
      "id": 1,
      "name": "Task description",
      "status": "completed",
      "priority": "High", 
      "phase": "MVP"
    }
  ],
  "links": [
    {
      "source": 1,
      "target": 2
    }
  ]
}
```

#### GET /api/projects/{name}/analytics
**Purpose**: Get project analytics and statistics  
**Parameters**: 
- `name` (path): Project identifier  
**Response**:
```json
{
  "total_tasks": 15,
  "completed_tasks": 7,
  "in_progress_tasks": 3,
  "pending_tasks": 5,
  "completion_rate": 46.7,
  "phase_breakdown": {
    "MVP": {
      "total": 8,
      "completed": 5,
      "in_progress": 2,
      "pending": 1
    }
  }
}
```

#### WS /ws/projects/{name}
**Purpose**: Real-time project updates  
**Protocol**: WebSocket  
**Messages**:
```json
{
  "type": "task_updated",
  "project": "current",
  "data": {
    "task": { /* Task object */ }
  },
  "timestamp": "2025-07-23T10:30:00Z"
}
```

---

## Development Setup

### Prerequisites
- **Rust**: 1.70+ with Cargo
- **Node.js**: 18+ with npm
- **Git**: For version control

### Backend Development
```bash
# Clone repository
git clone <repository-url>
cd rask

# Install Rust dependencies
cargo build

# Run backend only
cargo run --bin rask -- web --port 3000

# Run with file watching (development)
cargo watch -x "run --bin rask -- web --port 3000"
```

### Frontend Development
```bash
# Navigate to frontend directory  
cd web-ui

# Install dependencies
npm install

# Development server (with proxy to Rust backend)
npm run dev

# Type checking
npm run type-check

# Build for production
npm run build
```

### Full Stack Development
```bash
# Terminal 1: Start Rust backend
cargo run --bin rask -- web --port 3000

# Terminal 2: Start React dev server  
cd web-ui && npm run dev

# Access at: http://localhost:5173 (proxies to :3000)
```

### Environment Configuration

#### Backend Configuration
```toml
# .rask/config.toml
[web]
port = 3000
host = "127.0.0.1"
auto_open = false

[ai]
provider = "gemini"
api_key = "your-api-key"
```

#### Frontend Configuration
```json
// web-ui/vite.config.ts
export default defineConfig({
  server: {
    proxy: {
      '/api': 'http://127.0.0.1:3000',
      '/ws': {
        target: 'ws://127.0.0.1:3000',
        ws: true
      }
    }
  }
})
```

---

## Deployment

### Single Binary Deployment
Rask compiles to a single binary with embedded frontend assets:

```bash
# Build optimized binary
cargo build --release

# The binary includes:
# - Rust backend server
# - Embedded React frontend (from web-ui/dist/)
# - All static assets

# Deploy by copying single binary
cp target/release/rask /usr/local/bin/
```

### Frontend Asset Embedding
```rust
// src/web/routes/static_files.rs
use include_dir::{include_dir, Dir};

static ASSETS: Dir = include_dir!("$CARGO_MANIFEST_DIR/web-ui/dist");

// Assets are embedded at compile time
```

### Production Checklist
- [ ] Remove debug logging
- [ ] Set production AI API keys
- [ ] Configure CORS for production domains
- [ ] Set up reverse proxy (nginx/Apache)
- [ ] Configure HTTPS certificates
- [ ] Set up monitoring and logging

---

## Troubleshooting

### Common Issues

#### "Connecting to Rask server..."
**Symptoms**: Frontend shows persistent loading message  
**Causes**:
1. Backend server not running
2. Health endpoint not accessible
3. CORS issues
4. Port conflicts

**Solutions**:
```bash
# Check if server is running
curl http://127.0.0.1:3000/api/health

# Check server logs
cargo run --bin rask -- web --port 3000

# Check for port conflicts
lsof -i :3000
```

#### TypeScript Build Errors
**Symptoms**: `npm run build` fails with type errors  
**Causes**:
1. Outdated node_modules
2. TypeScript configuration issues
3. Missing type definitions

**Solutions**:
```bash
# Clear and reinstall dependencies
rm -rf node_modules package-lock.json
npm install

# Check TypeScript configuration
npm run type-check

# Update TypeScript settings in tsconfig.json
```

#### Backend Compilation Errors
**Symptoms**: `cargo build` fails  
**Causes**:
1. Missing dependencies
2. Rust version compatibility
3. Feature flag issues

**Solutions**:
```bash
# Update Rust toolchain
rustup update

# Clean build cache
cargo clean && cargo build

# Check Cargo.toml dependencies
```

#### WebSocket Connection Issues
**Symptoms**: Real-time updates not working  
**Causes**:
1. WebSocket endpoint not accessible
2. Browser blocking WebSocket
3. Proxy configuration issues

**Solutions**:
```bash
# Test WebSocket manually
wscat -c ws://127.0.0.1:3000/ws/projects/current

# Check browser console for WebSocket errors
# Verify proxy settings in vite.config.ts
```

### Debug Mode
Enable detailed logging for troubleshooting:

```bash
# Backend debug logging
RUST_LOG=debug cargo run --bin rask -- web

# Frontend debug logging  
# (Check browser console for [DEBUG] messages)
```

### Performance Issues
**Monitoring**:
- Backend: Use `cargo flamegraph` for profiling
- Frontend: Use React DevTools Profiler
- Network: Use browser Network tab

**Common Optimizations**:
- Reduce WebSocket message frequency
- Implement request debouncing
- Use React.memo for expensive components
- Optimize D3.js rendering with canvas

---

## Contributing

### Code Style
- **Rust**: Use `cargo fmt` and `cargo clippy`
- **TypeScript**: Use ESLint and Prettier
- **Documentation**: Update this file for major changes

### Testing
```bash
# Backend tests
cargo test

# Frontend tests  
cd web-ui && npm test

# Integration tests
# (To be implemented)
```

### Git Workflow
1. Create feature branch from `main`
2. Make changes with descriptive commits
3. Update documentation
4. Submit pull request

---

*Last Updated: July 23, 2025*  
*Version: 3.3.0*