import React from 'react';
import { useAppStore } from '@/stores/appStore';
import { cn } from '@/utils';
import { 
  LayoutDashboard, 
  CheckSquare, 
  BarChart3, 
  Bot,
  Network,
  Settings,
  ChevronLeft,
  ChevronRight,
  Rocket
} from 'lucide-react';

const Sidebar: React.FC = () => {
  const { activeTab, setActiveTab, sidebarOpen, setSidebarOpen } = useAppStore();

  const navigationItems = [
    {
      id: 'dashboard' as const,
      name: 'Dashboard',
      icon: LayoutDashboard,
      description: 'Project overview'
    },
    {
      id: 'tasks' as const,
      name: 'Tasks',
      icon: CheckSquare,
      description: 'Task management'
    },
    {
      id: 'dependencies' as const,
      name: 'Dependencies',
      icon: Network,
      description: 'Task relationships'
    },
    {
      id: 'analytics' as const,
      name: 'Analytics',
      icon: BarChart3,
      description: 'Progress insights'
    },
    {
      id: 'ai-chat' as const,
      name: 'AI Assistant',
      icon: Bot,
      description: 'AI-powered help'
    },
  ];

  return (
    <div className={cn(
      'fixed inset-y-0 left-0 z-50 bg-white border-r border-gray-200 transition-all duration-300',
      sidebarOpen ? 'w-64' : 'w-16'
    )}>
      {/* Header */}
      <div className="flex items-center justify-between p-4 border-b border-gray-200">
        {sidebarOpen && (
          <div className="flex items-center space-x-2">
            <Rocket className="h-8 w-8 text-primary-600" />
            <h1 className="text-xl font-bold text-gray-900">Rask</h1>
          </div>
        )}
        
        <button
          onClick={() => setSidebarOpen(!sidebarOpen)}
          className="p-1 rounded-lg hover:bg-gray-100 transition-colors"
        >
          {sidebarOpen ? (
            <ChevronLeft className="h-5 w-5 text-gray-500" />
          ) : (
            <ChevronRight className="h-5 w-5 text-gray-500" />
          )}
        </button>
      </div>

      {/* Navigation */}
      <nav className="mt-6 px-3">
        <ul className="space-y-2">
          {navigationItems.map((item) => {
            const Icon = item.icon;
            const isActive = activeTab === item.id;
            
            return (
              <li key={item.id}>
                <button
                  onClick={() => setActiveTab(item.id)}
                  className={cn(
                    'w-full flex items-center px-3 py-2 text-sm font-medium rounded-lg transition-colors',
                    isActive
                      ? 'bg-primary-50 text-primary-700 border-primary-200'
                      : 'text-gray-600 hover:bg-gray-50 hover:text-gray-900'
                  )}
                  title={!sidebarOpen ? item.name : undefined}
                >
                  <Icon className={cn(
                    'flex-shrink-0',
                    sidebarOpen ? 'mr-3 h-5 w-5' : 'h-5 w-5'
                  )} />
                  
                  {sidebarOpen && (
                    <div className="flex-1 text-left">
                      <div className="text-sm font-medium">{item.name}</div>
                      <div className="text-xs text-gray-500">{item.description}</div>
                    </div>
                  )}
                </button>
              </li>
            );
          })}
        </ul>
      </nav>

      {/* Footer */}
      {sidebarOpen && (
        <div className="absolute bottom-0 left-0 right-0 p-4 border-t border-gray-200">
          <button
            className="w-full flex items-center px-3 py-2 text-sm text-gray-600 hover:bg-gray-50 rounded-lg transition-colors"
          >
            <Settings className="mr-3 h-4 w-4" />
            Settings
          </button>
        </div>
      )}
    </div>
  );
};

export default Sidebar;