import React, { useEffect } from 'react';
import { useQuery } from '@tanstack/react-query';
import { useAppStore } from '@/stores/appStore';
import { api } from '@/api/client';
import { Layout } from '@/components/Layout';
import { Dashboard } from '@/components/Dashboard';
import ErrorBoundary from '@/components/ErrorBoundary';
import LoadingSpinner from '@/components/LoadingSpinner';
import AIPage from '@/pages/AIPage';
import DependenciesPage from '@/pages/DependenciesPage';

const App: React.FC = () => {
  const { 
    activeTab, 
    selectedProject,
    currentProject,
    setCurrentProject,
    setLoading,
    setError,
    setSelectedProject
  } = useAppStore();

  // Health check query
  const { data: healthData } = useQuery({
    queryKey: ['health'],
    queryFn: () => api.health.check(),
    refetchInterval: 30000, // Check every 30 seconds
    retry: 1,
  });

  // Projects list query
  const { data: projectsData, isLoading: projectsLoading } = useQuery({
    queryKey: ['projects'],
    queryFn: () => api.projects.listProjects(),
    retry: 1,
  });

  // Current project query
  const { data: projectData, isLoading: projectLoading } = useQuery({
    queryKey: ['project', selectedProject],
    queryFn: () => api.projects.getProject(selectedProject!),
    enabled: !!selectedProject,
    retry: 1,
  });

  // Set loading state
  useEffect(() => {
    setLoading(projectsLoading || projectLoading);
  }, [projectsLoading, projectLoading, setLoading]);

  // Handle project data
  useEffect(() => {
    if (projectData) {
      setCurrentProject(projectData);
      setError(null);
    }
  }, [projectData, setCurrentProject, setError]);

  // Auto-select first available project if none selected
  useEffect(() => {
    if (projectsData && !selectedProject && projectsData.projects.length > 0) {
      // For now, try to detect current project based on the current directory
      // In a real implementation, this might come from the URL or stored preference
      setSelectedProject('current'); // Assuming 'current' represents the current directory project
    }
  }, [projectsData, selectedProject, setSelectedProject]);

  const renderContent = () => {
    switch (activeTab) {
      case 'dashboard':
        return <Dashboard />;
      case 'tasks':
        return (
          <div className="flex items-center justify-center h-64">
            <p className="text-gray-500">Task manager coming soon...</p>
          </div>
        );
      case 'dependencies':
        return <DependenciesPage />;
      case 'analytics':
        return (
          <div className="flex items-center justify-center h-64">
            <p className="text-gray-500">Analytics coming soon...</p>
          </div>
        );
      case 'ai-chat':
        return <AIPage />;
      default:
        return <Dashboard />;
    }
  };

  return (
    <ErrorBoundary>
      <div className="min-h-screen bg-gray-50">
        {/* Show connection status */}
        {!healthData && (
          <div className="bg-yellow-50 border-b border-yellow-200 px-4 py-2">
            <div className="flex items-center justify-center">
              <p className="text-sm text-yellow-800">
                Connecting to Rask server...
              </p>
            </div>
          </div>
        )}

        <Layout>
          {projectsLoading && !currentProject ? (
            <LoadingSpinner message="Loading projects..." />
          ) : (
            renderContent()
          )}
        </Layout>

        {/* Development info */}
        {process.env.NODE_ENV === 'development' && healthData && (
          <div className="fixed bottom-4 right-4 bg-gray-900 text-white p-2 rounded text-xs">
            Server: {healthData.status} | Version: {healthData.version}
          </div>
        )}
      </div>
    </ErrorBoundary>
  );
};

export default App;