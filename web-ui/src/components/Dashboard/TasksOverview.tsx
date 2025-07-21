import React from 'react';
import { useAppStore } from '@/stores/appStore';
import { Card, PriorityBadge, StatusBadge } from '@/components/UI';
import { truncateText, formatRelativeTime } from '@/utils';
import { ExternalLink, Plus } from 'lucide-react';

const TasksOverview: React.FC = () => {
  const { currentProject, setActiveTab } = useAppStore();
  
  if (!currentProject) return null;

  // Get recent tasks (last 5)
  const recentTasks = currentProject.tasks
    .sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
    .slice(0, 5);

  // Get high priority pending tasks
  const highPriorityTasks = currentProject.tasks
    .filter(task => 
      task.status === 'todo' && 
      (task.priority === 'High' || task.priority === 'Critical')
    )
    .slice(0, 3);

  return (
    <Card>
      <Card.Header>
        <div className="flex items-center justify-between">
          <Card.Title>Tasks Overview</Card.Title>
          <div className="flex space-x-2">
            <button 
              onClick={() => setActiveTab('tasks')}
              className="inline-flex items-center text-sm text-primary-600 hover:text-primary-700"
            >
              <ExternalLink className="h-4 w-4 mr-1" />
              View All
            </button>
          </div>
        </div>
      </Card.Header>

      <Card.Content>
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          {/* Recent Tasks */}
          <div>
            <h4 className="text-sm font-medium text-gray-900 mb-3">Recent Tasks</h4>
            <div className="space-y-3">
              {recentTasks.length > 0 ? (
                recentTasks.map((task) => (
                  <div
                    key={task.id}
                    className="flex items-center space-x-3 p-3 bg-gray-50 rounded-lg hover:bg-gray-100 transition-colors"
                  >
                    <div className="flex-shrink-0">
                      <StatusBadge status={task.status} />
                    </div>
                    <div className="flex-1 min-w-0">
                      <p className="text-sm font-medium text-gray-900">
                        {truncateText(task.description, 40)}
                      </p>
                      <p className="text-xs text-gray-500">
                        {formatRelativeTime(task.created_at)}
                      </p>
                    </div>
                    <div className="flex-shrink-0">
                      <PriorityBadge priority={task.priority} />
                    </div>
                  </div>
                ))
              ) : (
                <div className="text-center py-4">
                  <p className="text-sm text-gray-500">No tasks yet</p>
                  <button className="mt-2 inline-flex items-center text-sm text-primary-600 hover:text-primary-700">
                    <Plus className="h-4 w-4 mr-1" />
                    Add your first task
                  </button>
                </div>
              )}
            </div>
          </div>

          {/* High Priority Tasks */}
          <div>
            <h4 className="text-sm font-medium text-gray-900 mb-3">High Priority Tasks</h4>
            <div className="space-y-3">
              {highPriorityTasks.length > 0 ? (
                highPriorityTasks.map((task) => (
                  <div
                    key={task.id}
                    className="flex items-center space-x-3 p-3 bg-red-50 rounded-lg border border-red-100"
                  >
                    <div className="flex-shrink-0">
                      <PriorityBadge priority={task.priority} />
                    </div>
                    <div className="flex-1 min-w-0">
                      <p className="text-sm font-medium text-gray-900">
                        {truncateText(task.description, 40)}
                      </p>
                      <div className="flex items-center mt-1">
                        <span className="text-xs text-gray-500 mr-2">
                          {task.phase.emoji} {task.phase.name}
                        </span>
                        {task.tags.length > 0 && (
                          <div className="flex space-x-1">
                            {task.tags.slice(0, 2).map((tag) => (
                              <span
                                key={tag}
                                className="inline-block px-1.5 py-0.5 text-xs bg-gray-200 text-gray-700 rounded"
                              >
                                #{tag}
                              </span>
                            ))}
                            {task.tags.length > 2 && (
                              <span className="text-xs text-gray-500">
                                +{task.tags.length - 2}
                              </span>
                            )}
                          </div>
                        )}
                      </div>
                    </div>
                  </div>
                ))
              ) : (
                <div className="text-center py-4">
                  <p className="text-sm text-gray-500">No high priority tasks</p>
                  <p className="text-xs text-gray-400 mt-1">
                    Great! You're on top of urgent items.
                  </p>
                </div>
              )}
            </div>
          </div>
        </div>
      </Card.Content>
    </Card>
  );
};

export default TasksOverview;