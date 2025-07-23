import React from 'react';
import { useAppStore } from '@/stores/appStore';
import { Card } from '@/components/UI';
import { formatRelativeTime } from '@/utils';
import { CheckCircle, Plus, Edit, Trash } from 'lucide-react';

interface ActivityItem {
  id: string;
  type: 'completed' | 'created' | 'updated' | 'deleted';
  description: string;
  timestamp: string;
  taskId?: number;
}

const RecentActivity: React.FC = () => {
  const { currentProject } = useAppStore();
  
  if (!currentProject) return null;

  // Generate mock activity data based on tasks
  // In a real app, this would come from an activity log API
  const generateActivity = (): ActivityItem[] => {
    const activities: ActivityItem[] = [];
    
    // Add completed tasks
    currentProject.tasks
      .filter(task => task.status === 'completed' && task.completed_at)
      .sort((a, b) => new Date(b.completed_at!).getTime() - new Date(a.completed_at!).getTime())
      .slice(0, 3)
      .forEach(task => {
        activities.push({
          id: `completed-${task.id}`,
          type: 'completed',
          description: `Completed "${task.description}"`,
          timestamp: task.completed_at!,
          taskId: task.id,
        });
      });

    // Add recently created tasks
    currentProject.tasks
      .sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
      .slice(0, 2)
      .forEach(task => {
        activities.push({
          id: `created-${task.id}`,
          type: 'created',
          description: `Created "${task.description}"`,
          timestamp: task.created_at,
          taskId: task.id,
        });
      });

    return activities
      .sort((a, b) => new Date(b.timestamp).getTime() - new Date(a.timestamp).getTime())
      .slice(0, 5);
  };

  const activities = generateActivity();

  const getActivityIcon = (type: ActivityItem['type']) => {
    switch (type) {
      case 'completed':
        return <CheckCircle className="h-4 w-4 text-green-600" />;
      case 'created':
        return <Plus className="h-4 w-4 text-blue-600" />;
      case 'updated':
        return <Edit className="h-4 w-4 text-yellow-600" />;
      case 'deleted':
        return <Trash className="h-4 w-4 text-red-600" />;
      default:
        return <div className="h-4 w-4 bg-gray-400 rounded-full" />;
    }
  };

  const getActivityColor = (type: ActivityItem['type']) => {
    switch (type) {
      case 'completed':
        return 'border-green-200 bg-green-50';
      case 'created':
        return 'border-blue-200 bg-blue-50';
      case 'updated':
        return 'border-yellow-200 bg-yellow-50';
      case 'deleted':
        return 'border-red-200 bg-red-50';
      default:
        return 'border-gray-200 bg-gray-50';
    }
  };

  return (
    <Card>
      <Card.Header>
        <Card.Title>Recent Activity</Card.Title>
      </Card.Header>

      <Card.Content>
        <div className="space-y-3">
          {activities.length > 0 ? (
            activities.map((activity) => (
              <div
                key={activity.id}
                className={`flex items-start space-x-3 p-3 rounded-lg border ${getActivityColor(activity.type)}`}
              >
                <div className="flex-shrink-0 mt-0.5">
                  {getActivityIcon(activity.type)}
                </div>
                <div className="flex-1 min-w-0">
                  <p className="text-sm text-gray-900">{activity.description}</p>
                  <p className="text-xs text-gray-500 mt-1">
                    {formatRelativeTime(activity.timestamp)}
                  </p>
                </div>
              </div>
            ))
          ) : (
            <div className="text-center py-4">
              <p className="text-sm text-gray-500">No recent activity</p>
              <p className="text-xs text-gray-400 mt-1">
                Activity will appear here as you work on tasks
              </p>
            </div>
          )}
        </div>

        {/* Quick actions */}
        <div className="mt-6 pt-4 border-t border-gray-200">
          <h5 className="text-xs font-medium text-gray-500 uppercase tracking-wide mb-3">
            Quick Actions
          </h5>
          <div className="grid grid-cols-2 gap-2">
            <button className="inline-flex items-center justify-center px-3 py-2 text-xs bg-primary-50 text-primary-700 rounded-lg hover:bg-primary-100 transition-colors">
              <Plus className="h-3 w-3 mr-1" />
              Add Task
            </button>
            <button 
              onClick={() => useAppStore.getState().setActiveTab('ai-chat')}
              className="inline-flex items-center justify-center px-3 py-2 text-xs bg-purple-50 text-purple-700 rounded-lg hover:bg-purple-100 transition-colors"
            >
              🤖 Ask AI
            </button>
          </div>
        </div>
      </Card.Content>
    </Card>
  );
};

export default RecentActivity;