import React, { useState } from 'react';
import { Network, Eye, Filter, Download } from 'lucide-react';
import Layout from '@/components/Layout/Layout';
import Card from '@/components/UI/Card';
import Button from '@/components/UI/Button';
import DependencyGraph from '@/components/Dependencies/DependencyGraph';
import { useAppStore } from '@/stores/appStore';
import { Task, Priority } from '@/types';

const DependenciesPage: React.FC = () => {
  const { currentProject } = useAppStore();
  const [selectedTask, setSelectedTask] = useState<Task | null>(null);
  const [showFilters, setShowFilters] = useState(false);
  const [filters, setFilters] = useState({
    priority: 'all' as Priority | 'all',
    status: 'all' as string,
    phase: 'all' as string,
  });

  const tasks = currentProject?.tasks || [];

  // Filter tasks based on current filters
  const filteredTasks = tasks.filter(task => {
    if (filters.priority !== 'all' && task.priority !== filters.priority) return false;
    if (filters.status !== 'all' && task.status !== filters.status) return false;
    if (filters.phase !== 'all' && task.phase.name !== filters.phase) return false;
    return true;
  });

  // Get unique values for filter options
  const uniqueStatuses = [...new Set(tasks.map(task => task.status))];
  const uniquePhases = [...new Set(tasks.map(task => task.phase.name))];
  const priorities = Object.values(Priority);

  const handleTaskClick = (task: Task) => {
    setSelectedTask(task);
  };

  const handleExportGraph = () => {
    // TODO: Implement graph export functionality
    console.log('Exporting dependency graph...');
  };

  const getDependencyStats = () => {
    const stats = {
      totalTasks: tasks.length,
      tasksWithDependencies: tasks.filter(t => t.dependencies.length > 0).length,
      blockedTasks: tasks.filter(t => 
        t.dependencies.some(depId => 
          tasks.find(dep => dep.id === depId)?.status !== 'completed'
        )
      ).length,
      isolatedTasks: tasks.filter(t => 
        t.dependencies.length === 0 && 
        !tasks.some(other => other.dependencies.includes(t.id))
      ).length,
    };

    return stats;
  };

  const stats = getDependencyStats();

  return (
    <Layout>
      <div className="space-y-6">
        {/* Header */}
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <div className="flex items-center justify-center w-10 h-10 bg-purple-100 rounded-lg">
              <Network className="h-6 w-6 text-purple-600" />
            </div>
            <div>
              <h1 className="text-2xl font-bold text-gray-900">Dependencies</h1>
              <p className="text-gray-600">
                Visualize and manage task relationships and dependencies
              </p>
            </div>
          </div>
          
          <div className="flex items-center gap-2">
            <Button
              variant="ghost"
              size="sm"
              onClick={() => setShowFilters(!showFilters)}
            >
              <Filter className="h-4 w-4 mr-2" />
              Filters
            </Button>
            <Button
              variant="ghost"
              size="sm"
              onClick={handleExportGraph}
            >
              <Download className="h-4 w-4 mr-2" />
              Export
            </Button>
          </div>
        </div>

        {/* Project Context */}
        {currentProject && (
          <Card>
            <Card.Content className="py-3">
              <div className="flex items-center gap-4 text-sm text-gray-600">
                <span className="font-medium">Project:</span>
                <span className="px-2 py-1 bg-purple-100 text-purple-800 rounded-md">
                  {currentProject.metadata.name}
                </span>
                <span className="text-gray-400">•</span>
                <span>{stats.totalTasks} total tasks</span>
                <span className="text-gray-400">•</span>
                <span>{stats.tasksWithDependencies} with dependencies</span>
                <span className="text-gray-400">•</span>
                <span className="text-red-600">{stats.blockedTasks} blocked</span>
              </div>
            </Card.Content>
          </Card>
        )}

        {/* Filters */}
        {showFilters && (
          <Card>
            <Card.Header>
              <Card.Title>Filters</Card.Title>
            </Card.Header>
            <Card.Content>
              <div className="grid grid-cols-3 gap-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    Priority
                  </label>
                  <select
                    value={filters.priority}
                    onChange={(e) => setFilters(prev => ({ 
                      ...prev, 
                      priority: e.target.value as Priority | 'all' 
                    }))}
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-purple-500"
                  >
                    <option value="all">All Priorities</option>
                    {priorities.map(priority => (
                      <option key={priority} value={priority}>
                        {priority}
                      </option>
                    ))}
                  </select>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    Status
                  </label>
                  <select
                    value={filters.status}
                    onChange={(e) => setFilters(prev => ({ 
                      ...prev, 
                      status: e.target.value 
                    }))}
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-purple-500"
                  >
                    <option value="all">All Statuses</option>
                    {uniqueStatuses.map(status => (
                      <option key={status} value={status}>
                        {status.replace('-', ' ').replace(/\b\w/g, l => l.toUpperCase())}
                      </option>
                    ))}
                  </select>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    Phase
                  </label>
                  <select
                    value={filters.phase}
                    onChange={(e) => setFilters(prev => ({ 
                      ...prev, 
                      phase: e.target.value 
                    }))}
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-purple-500"
                  >
                    <option value="all">All Phases</option>
                    {uniquePhases.map(phase => (
                      <option key={phase} value={phase}>
                        {phase}
                      </option>
                    ))}
                  </select>
                </div>
              </div>
            </Card.Content>
          </Card>
        )}

        {/* Main Content */}
        <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
          {/* Dependency Graph */}
          <div className="lg:col-span-3">
            <DependencyGraph
              tasks={filteredTasks}
              onTaskClick={handleTaskClick}
            />
          </div>

          {/* Side Panel */}
          <div className="space-y-4">
            {/* Stats Card */}
            <Card>
              <Card.Header>
                <Card.Title className="flex items-center gap-2">
                  <Eye className="h-4 w-4" />
                  Overview
                </Card.Title>
              </Card.Header>
              <Card.Content>
                <div className="space-y-3 text-sm">
                  <div className="flex justify-between">
                    <span className="text-gray-600">Total Tasks:</span>
                    <span className="font-medium">{stats.totalTasks}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600">With Dependencies:</span>
                    <span className="font-medium">{stats.tasksWithDependencies}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600">Blocked Tasks:</span>
                    <span className="font-medium text-red-600">{stats.blockedTasks}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600">Isolated Tasks:</span>
                    <span className="font-medium text-green-600">{stats.isolatedTasks}</span>
                  </div>
                </div>
              </Card.Content>
            </Card>

            {/* Selected Task Details */}
            {selectedTask && (
              <Card>
                <Card.Header>
                  <Card.Title>Task Details</Card.Title>
                </Card.Header>
                <Card.Content>
                  <div className="space-y-3">
                    <div>
                      <h4 className="font-medium text-gray-900 text-sm">
                        #{selectedTask.id}
                      </h4>
                      <p className="text-sm text-gray-600 mt-1">
                        {selectedTask.description}
                      </p>
                    </div>

                    <div className="grid grid-cols-2 gap-3 text-xs">
                      <div>
                        <span className="text-gray-500">Status:</span>
                        <p className="font-medium capitalize">
                          {selectedTask.status.replace('-', ' ')}
                        </p>
                      </div>
                      <div>
                        <span className="text-gray-500">Priority:</span>
                        <p className="font-medium">{selectedTask.priority}</p>
                      </div>
                      <div>
                        <span className="text-gray-500">Phase:</span>
                        <p className="font-medium">{selectedTask.phase.name}</p>
                      </div>
                      <div>
                        <span className="text-gray-500">Dependencies:</span>
                        <p className="font-medium">{selectedTask.dependencies.length}</p>
                      </div>
                    </div>

                    {selectedTask.dependencies.length > 0 && (
                      <div>
                        <span className="text-gray-500 text-xs">Depends on:</span>
                        <div className="mt-1 flex flex-wrap gap-1">
                          {selectedTask.dependencies.map(depId => (
                            <span
                              key={depId}
                              className="px-2 py-1 bg-gray-100 text-gray-700 rounded text-xs"
                            >
                              #{depId}
                            </span>
                          ))}
                        </div>
                      </div>
                    )}

                    {selectedTask.tags.length > 0 && (
                      <div>
                        <span className="text-gray-500 text-xs">Tags:</span>
                        <div className="mt-1 flex flex-wrap gap-1">
                          {selectedTask.tags.map(tag => (
                            <span
                              key={tag}
                              className="px-2 py-1 bg-blue-100 text-blue-700 rounded text-xs"
                            >
                              {tag}
                            </span>
                          ))}
                        </div>
                      </div>
                    )}
                  </div>
                </Card.Content>
              </Card>
            )}

            {/* Help */}
            <Card>
              <Card.Header>
                <Card.Title>How to use</Card.Title>
              </Card.Header>
              <Card.Content>
                <div className="text-xs text-gray-600 space-y-2">
                  <p>• <strong>Click</strong> nodes to view task details</p>
                  <p>• <strong>Drag</strong> nodes to reposition them</p>
                  <p>• <strong>Scroll</strong> to zoom in/out</p>
                  <p>• <strong>Arrows</strong> point from dependencies to tasks</p>
                  <p>• <strong>Colors</strong> indicate status and priority</p>
                </div>
              </Card.Content>
            </Card>
          </div>
        </div>
      </div>
    </Layout>
  );
};

export default DependenciesPage;