import React from 'react';
import { useAppStore } from '@/stores/appStore';
import ProjectOverview from './ProjectOverview';
import TasksOverview from './TasksOverview';
import PhaseProgress from './PhaseProgress';
import RecentActivity from './RecentActivity';

const Dashboard: React.FC = () => {
  const { currentProject } = useAppStore();

  if (!currentProject) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-center">
          <h3 className="text-lg font-medium text-gray-900 mb-2">
            No Project Selected
          </h3>
          <p className="text-gray-500">
            Please select a project to view the dashboard
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-2xl font-bold text-gray-900">Dashboard</h1>
        <p className="text-gray-600">
          Get an overview of your project progress and AI insights
        </p>
      </div>

      {/* Main dashboard grid */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Left column - Main content */}
        <div className="lg:col-span-2 space-y-6">
          <ProjectOverview />
          <TasksOverview />
        </div>

        {/* Right column - Sidebar content */}
        <div className="space-y-6">
          <PhaseProgress />
          <RecentActivity />
        </div>
      </div>
    </div>
  );
};

export default Dashboard;