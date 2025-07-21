import React from 'react';
import { useAppStore } from '@/stores/appStore';
import { Card } from '@/components/UI';
import { formatDate } from '@/utils';
import { 
  CheckCircle, 
  Clock, 
  AlertTriangle,
  TrendingUp
} from 'lucide-react';

const ProjectOverview: React.FC = () => {
  const { currentProject, getCompletionStats } = useAppStore();
  
  if (!currentProject) return null;

  const { completed, total, percentage } = getCompletionStats();
  const pendingTasks = total - completed;
  
  // Calculate some basic metrics
  const highPriorityTasks = currentProject.tasks.filter(task => 
    task.priority === 'High' || task.priority === 'Critical'
  ).length;
  
  const blockedTasks = currentProject.tasks.filter(task =>
    task.dependencies.length > 0 && 
    task.status === 'todo' &&
    task.dependencies.some(depId => {
      const depTask = currentProject.tasks.find(t => t.id === depId);
      return depTask?.status === 'todo';
    })
  ).length;

  const stats = [
    {
      name: 'Completed Tasks',
      value: completed,
      total: total,
      icon: CheckCircle,
      color: 'text-green-600',
      bgColor: 'bg-green-50',
    },
    {
      name: 'Pending Tasks',
      value: pendingTasks,
      icon: Clock,
      color: 'text-yellow-600',
      bgColor: 'bg-yellow-50',
    },
    {
      name: 'High Priority',
      value: highPriorityTasks,
      icon: TrendingUp,
      color: 'text-red-600',
      bgColor: 'bg-red-50',
    },
    {
      name: 'Blocked Tasks',
      value: blockedTasks,
      icon: AlertTriangle,
      color: 'text-orange-600',
      bgColor: 'bg-orange-50',
    },
  ];

  return (
    <Card>
      <Card.Header>
        <div className="flex items-center justify-between">
          <div>
            <Card.Title>{currentProject.title}</Card.Title>
            <p className="text-sm text-gray-500 mt-1">
              Last updated: {formatDate(currentProject.metadata.last_modified)}
            </p>
          </div>
          <div className="text-right">
            <div className="text-2xl font-bold text-gray-900">{percentage}%</div>
            <div className="text-sm text-gray-500">Complete</div>
          </div>
        </div>
      </Card.Header>

      <Card.Content>
        {/* Progress bar */}
        <div className="mb-6">
          <div className="flex items-center justify-between text-sm mb-2">
            <span className="text-gray-600">Overall Progress</span>
            <span className="text-gray-900 font-medium">{completed} of {total} tasks</span>
          </div>
          <div className="w-full bg-gray-200 rounded-full h-3">
            <div
              className="bg-primary-600 h-3 rounded-full transition-all duration-300"
              style={{ width: `${percentage}%` }}
            />
          </div>
        </div>

        {/* Stats grid */}
        <div className="grid grid-cols-2 lg:grid-cols-4 gap-4">
          {stats.map((stat) => {
            const Icon = stat.icon;
            return (
              <div
                key={stat.name}
                className={`${stat.bgColor} rounded-lg p-4 border border-gray-100`}
              >
                <div className="flex items-center">
                  <Icon className={`h-5 w-5 ${stat.color} mr-2`} />
                  <div className="text-2xl font-bold text-gray-900">
                    {stat.value}
                    {stat.total && (
                      <span className="text-sm text-gray-500 font-normal">
                        /{stat.total}
                      </span>
                    )}
                  </div>
                </div>
                <p className="text-sm text-gray-600 mt-1">{stat.name}</p>
              </div>
            );
          })}
        </div>

        {/* Project description */}
        {currentProject.metadata.description && (
          <div className="mt-6 p-4 bg-gray-50 rounded-lg">
            <h4 className="text-sm font-medium text-gray-900 mb-2">Description</h4>
            <p className="text-sm text-gray-600">{currentProject.metadata.description}</p>
          </div>
        )}
      </Card.Content>
    </Card>
  );
};

export default ProjectOverview;