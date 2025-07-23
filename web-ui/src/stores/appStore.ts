// Global application state management with Zustand

import { create } from 'zustand';
import { devtools } from 'zustand/middleware';
import type { 
  UIState, 
  Roadmap, 
  Task, 
  TaskFilters, 
  ProjectInfo,
  AiChatSession 
} from '@/types';

interface AppState extends UIState {
  // Data state
  projects: ProjectInfo[];
  currentProject: Roadmap | null;
  tasks: Task[];
  isLoading: boolean;
  error: string | null;
  
  // AI Chat state
  aiChatSession: AiChatSession | null;
  
  // WebSocket connection state
  isConnectedToWs: boolean;
  
  // Actions
  setProjects: (projects: ProjectInfo[]) => void;
  setCurrentProject: (project: Roadmap | null) => void;
  setTasks: (tasks: Task[]) => void;
  updateTask: (taskId: number, updates: Partial<Task>) => void;
  addTask: (task: Task) => void;
  removeTask: (taskId: number) => void;
  
  setSelectedProject: (projectName: string | undefined) => void;
  setActiveTab: (tab: UIState['activeTab']) => void;
  setFilters: (filters: Partial<TaskFilters>) => void;
  setSidebarOpen: (open: boolean) => void;
  
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  
  setAiChatSession: (session: AiChatSession | null) => void;
  addAiMessage: (message: { role: 'user' | 'assistant'; content: string }) => void;
  
  setWsConnected: (connected: boolean) => void;
  
  // Computed getters
  getFilteredTasks: () => Task[];
  getTaskById: (id: number) => Task | undefined;
  getTasksByPhase: () => Record<string, Task[]>;
  getCompletionStats: () => { completed: number; total: number; percentage: number };
}

export const useAppStore = create<AppState>()(
  devtools(
    (set, get) => ({
      // Initial state
      selectedProject: undefined,
      activeTab: 'dashboard',
      filters: {},
      sidebarOpen: true,
      
      projects: [],
      currentProject: null,
      tasks: [],
      isLoading: false,
      error: null,
      
      aiChatSession: null,
      isConnectedToWs: false,
      
      // Actions
      setProjects: (projects) => set({ projects }),
      
      setCurrentProject: (project) => set({ 
        currentProject: project,
        tasks: project?.tasks || [],
        error: null 
      }),
      
      setTasks: (tasks) => set({ tasks }),
      
      updateTask: (taskId, updates) => set((state) => ({
        tasks: state.tasks.map(task => 
          task.id === taskId ? { ...task, ...updates } : task
        ),
        currentProject: state.currentProject ? {
          ...state.currentProject,
          tasks: state.currentProject.tasks.map(task =>
            task.id === taskId ? { ...task, ...updates } : task
          )
        } : null
      })),
      
      addTask: (task) => set((state) => ({
        tasks: [...state.tasks, task],
        currentProject: state.currentProject ? {
          ...state.currentProject,
          tasks: [...state.currentProject.tasks, task]
        } : null
      })),
      
      removeTask: (taskId) => set((state) => ({
        tasks: state.tasks.filter(task => task.id !== taskId),
        currentProject: state.currentProject ? {
          ...state.currentProject,
          tasks: state.currentProject.tasks.filter(task => task.id !== taskId)
        } : null
      })),
      
      setSelectedProject: (projectName) => set({ selectedProject: projectName }),
      setActiveTab: (tab) => set({ activeTab: tab }),
      setFilters: (filters) => set((state) => ({ 
        filters: { ...state.filters, ...filters } 
      })),
      setSidebarOpen: (open) => set({ sidebarOpen: open }),
      
      setLoading: (loading) => set({ isLoading: loading }),
      setError: (error) => set({ error }),
      
      setAiChatSession: (session) => set({ aiChatSession: session }),
      addAiMessage: (message) => set((state) => {
        if (!state.aiChatSession) return state;
        
        const newMessage = {
          id: Date.now().toString(),
          ...message,
          timestamp: new Date().toISOString(),
        };
        
        return {
          aiChatSession: {
            ...state.aiChatSession,
            messages: [...state.aiChatSession.messages, newMessage]
          }
        };
      }),
      
      setWsConnected: (connected) => set({ isConnectedToWs: connected }),
      
      // Computed getters
      getFilteredTasks: () => {
        const { tasks, filters } = get();
        
        return tasks.filter(task => {
          if (filters.status && task.status !== filters.status) return false;
          if (filters.priority && task.priority !== filters.priority) return false;
          if (filters.phase && task.phase.name !== filters.phase) return false;
          if (filters.tag && !task.tags.some(tag => tag.includes(filters.tag!))) return false;
          if (filters.search) {
            const searchLower = filters.search.toLowerCase();
            const matchesDescription = task.description.toLowerCase().includes(searchLower);
            const matchesNotes = task.notes?.toLowerCase().includes(searchLower);
            const matchesTags = task.tags.some(tag => tag.toLowerCase().includes(searchLower));
            if (!matchesDescription && !matchesNotes && !matchesTags) return false;
          }
          
          return true;
        });
      },
      
      getTaskById: (id) => {
        const { tasks } = get();
        return tasks.find(task => task.id === id);
      },
      
      getTasksByPhase: () => {
        const { tasks } = get();
        const grouped: Record<string, Task[]> = {};
        
        tasks.forEach(task => {
          const phaseName = task.phase.name;
          if (!grouped[phaseName]) {
            grouped[phaseName] = [];
          }
          grouped[phaseName].push(task);
        });
        
        return grouped;
      },
      
      getCompletionStats: () => {
        const { tasks } = get();
        const completed = tasks.filter(task => task.status === 'completed').length;
        const total = tasks.length;
        const percentage = total > 0 ? Math.round((completed / total) * 100) : 0;
        
        return { completed, total, percentage };
      },
    }),
    {
      name: 'rask-app-store',
    }
  )
);