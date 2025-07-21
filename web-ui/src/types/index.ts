// Core types based on the Rust backend models

export interface Task {
  id: number;
  description: string;
  status: TaskStatus;
  priority: Priority;
  phase: Phase;
  tags: string[];
  notes?: string;
  dependencies: number[];
  created_at: string;
  completed_at?: string;
  estimated_hours?: number;
  actual_hours?: number;
  ai_info?: AiTaskInfo;
  implementation_notes: string[];
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
  phases: Phase[];
  source_file?: string;
  metadata: ProjectMetadata;
}

export interface ProjectMetadata {
  name: string;
  description?: string;
  created_at: string;
  last_modified: string;
  version: string;
}

export interface AiTaskInfo {
  is_ai_generated: boolean;
  operation: string;
  reasoning?: string;
  model?: string;
  generated_at: string;
}

export type TaskStatus = 'todo' | 'in-progress' | 'completed' | 'blocked';

export enum Priority {
  Low = 'Low',
  Medium = 'Medium', 
  High = 'High',
  Critical = 'Critical'
}

// API Response types
export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
  message?: string;
}

export interface ProjectListResponse {
  projects: ProjectInfo[];
}

export interface ProjectInfo {
  name: string;
  path: string;
  last_modified: string;
  task_count: number;
  completion_percentage: number;
}

export interface TaskFilters {
  tag?: string;
  priority?: Priority;
  phase?: string;
  status?: TaskStatus;
  search?: string;
}

// WebSocket message types
export interface WebSocketMessage {
  type: string;
  project?: string;
  data?: any;
  timestamp?: string;
}

export interface StateChangeMessage extends WebSocketMessage {
  type: 'task_updated' | 'project_modified' | 'config_changed';
  data: {
    task?: Task;
    roadmap?: Roadmap;
  };
}

// UI State types
export interface UIState {
  selectedProject?: string;
  activeTab: 'dashboard' | 'tasks' | 'dependencies' | 'analytics' | 'ai-chat';
  filters: TaskFilters;
  sidebarOpen: boolean;
}

// AI Chat types
export interface AiChatMessage {
  id: string;
  role: 'user' | 'assistant';
  content: string;
  timestamp: string;
}

export interface AiChatSession {
  id: string;
  messages: AiChatMessage[];
  project_context?: string;
}

// Analytics types
export interface ProjectAnalytics {
  total_tasks: number;
  completed_tasks: number;
  completion_percentage: number;
  tasks_by_priority: Record<Priority, number>;
  tasks_by_phase: Record<string, number>;
  estimated_hours: number;
  actual_hours: number;
  efficiency_score?: number;
}

// Chart data types
export interface ChartDataPoint {
  name: string;
  value: number;
  color?: string;
}

export interface TimeSeriesPoint {
  date: string;
  completed: number;
  total: number;
  percentage: number;
}