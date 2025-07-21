// API client for communicating with the Rust backend

import type {
  ProjectListResponse,
  Roadmap,
  Task,
  TaskFilters,
  ProjectAnalytics,
} from '@/types';

const API_BASE = '/api';

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

async function fetchApi<T>(
  endpoint: string,
  options: RequestInit = {}
): Promise<T> {
  const url = `${API_BASE}${endpoint}`;
  
  const response = await fetch(url, {
    headers: {
      'Content-Type': 'application/json',
      ...options.headers,
    },
    ...options,
  });

  if (!response.ok) {
    const errorText = await response.text();
    throw new ApiError(
      `API Error: ${response.status} ${response.statusText}`,
      response.status,
      errorText
    );
  }

  return response.json();
}

// Project API
export const projectApi = {
  // List all available projects
  async listProjects(): Promise<ProjectListResponse> {
    return fetchApi<ProjectListResponse>('/projects');
  },

  // Get project details
  async getProject(name: string): Promise<Roadmap> {
    return fetchApi<Roadmap>(`/projects/${encodeURIComponent(name)}`);
  },

  // Get project tasks with optional filters
  async getTasks(name: string, filters?: TaskFilters): Promise<Task[]> {
    const params = new URLSearchParams();
    if (filters?.tag) params.append('tag', filters.tag);
    if (filters?.priority) params.append('priority', filters.priority);
    if (filters?.phase) params.append('phase', filters.phase);
    if (filters?.status) params.append('status', filters.status);
    if (filters?.search) params.append('search', filters.search);

    const queryString = params.toString();
    const endpoint = `/projects/${encodeURIComponent(name)}/tasks${queryString ? `?${queryString}` : ''}`;
    
    return fetchApi<Task[]>(endpoint);
  },

  // Get project analytics
  async getAnalytics(name: string): Promise<ProjectAnalytics> {
    return fetchApi<ProjectAnalytics>(`/projects/${encodeURIComponent(name)}/analytics`);
  },

  // Get project dependencies
  async getDependencies(name: string): Promise<any> {
    return fetchApi<any>(`/projects/${encodeURIComponent(name)}/dependencies`);
  },
};

// Task API
export const taskApi = {
  // Get task details
  async getTask(id: number): Promise<Task> {
    return fetchApi<Task>(`/tasks/${id}`);
  },

  // Update task
  async updateTask(id: number, updates: Partial<Task>): Promise<Task> {
    return fetchApi<Task>(`/tasks/${id}`, {
      method: 'PUT',
      body: JSON.stringify(updates),
    });
  },

  // Delete task
  async deleteTask(id: number): Promise<void> {
    return fetchApi<void>(`/tasks/${id}`, {
      method: 'DELETE',
    });
  },

  // Create new task
  async createTask(projectName: string, task: Omit<Task, 'id' | 'created_at'>): Promise<Task> {
    return fetchApi<Task>(`/projects/${encodeURIComponent(projectName)}/tasks`, {
      method: 'POST',
      body: JSON.stringify(task),
    });
  },
};

// AI API
export const aiApi = {
  // Send chat message
  async chat(message: string, projectContext?: string): Promise<string> {
    const response = await fetchApi<{ response: string }>('/ai/chat', {
      method: 'POST',
      body: JSON.stringify({ message, project_context: projectContext }),
    });
    return response.response;
  },

  // Send chat message with streaming (returns Response for SSE handling)
  async chatStream(message: string, projectContext?: string): Promise<Response> {
    const url = `${API_BASE}/ai/chat/stream`;
    
    return fetch(url, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ 
        message, 
        project_context: projectContext 
      }),
    });
  },

  // Analyze tasks
  async analyzeTasks(projectName: string): Promise<any> {
    return fetchApi<any>('/ai/analyze', {
      method: 'POST',
      body: JSON.stringify({ project_name: projectName }),
    });
  },

  // Get task breakdown
  async getTaskBreakdown(description: string): Promise<Task[]> {
    const response = await fetchApi<{ tasks: Task[] }>('/ai/breakdown', {
      method: 'POST',
      body: JSON.stringify({ description }),
    });
    return response.tasks;
  },

  // Get AI suggestions
  async getSuggestions(projectName: string, count?: number): Promise<Task[]> {
    const response = await fetchApi<{ suggestions: Task[] }>('/ai/suggest', {
      method: 'POST',
      body: JSON.stringify({ project_name: projectName, count }),
    });
    return response.suggestions;
  },

  // Get project insights
  async getInsights(projectName: string): Promise<any> {
    return fetchApi<any>(`/ai/insights?project=${encodeURIComponent(projectName)}`);
  },
};

// Health check
export const healthApi = {
  async check(): Promise<{ status: string; version: string }> {
    return fetchApi<{ status: string; version: string }>('/health');
  },
};

// Export all APIs
export const api = {
  projects: projectApi,
  tasks: taskApi,
  ai: aiApi,
  health: healthApi,
};

export default api;