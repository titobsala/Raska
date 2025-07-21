import React from 'react';
import { useAppStore } from '@/stores/appStore';
import { Button } from '@/components/UI';
import { 
  Search,
  Filter,
  Wifi,
  WifiOff,
  Loader2
} from 'lucide-react';

const Header: React.FC = () => {
  const { 
    currentProject, 
    isLoading, 
    isConnectedToWs,
    filters,
    setFilters
  } = useAppStore();

  return (
    <header className="bg-white border-b border-gray-200 px-6 py-4">
      <div className="flex items-center justify-between">
        {/* Left section - Project info */}
        <div className="flex items-center space-x-4">
          <div>
            <h2 className="text-xl font-semibold text-gray-900">
              {currentProject?.title || 'No Project Selected'}
            </h2>
            {currentProject && (
              <p className="text-sm text-gray-500">
                {currentProject.tasks.length} tasks • {currentProject.metadata.name}
              </p>
            )}
          </div>
          
          {isLoading && (
            <Loader2 className="h-4 w-4 animate-spin text-gray-400" />
          )}
        </div>

        {/* Center section - Search */}
        <div className="flex-1 max-w-lg mx-8">
          <div className="relative">
            <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-gray-400" />
            <input
              type="text"
              placeholder="Search tasks..."
              value={filters.search || ''}
              onChange={(e) => setFilters({ search: e.target.value })}
              className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent"
            />
          </div>
        </div>

        {/* Right section - Actions and status */}
        <div className="flex items-center space-x-3">
          {/* Filter button */}
          <Button variant="outline" size="sm">
            <Filter className="h-4 w-4 mr-2" />
            Filters
          </Button>

          {/* Connection status */}
          <div className="flex items-center space-x-2">
            {isConnectedToWs ? (
              <div className="flex items-center text-green-600">
                <Wifi className="h-4 w-4 mr-1" />
                <span className="text-xs">Live</span>
              </div>
            ) : (
              <div className="flex items-center text-gray-400">
                <WifiOff className="h-4 w-4 mr-1" />
                <span className="text-xs">Offline</span>
              </div>
            )}
          </div>

          {/* User avatar placeholder */}
          <div className="h-8 w-8 bg-primary-100 rounded-full flex items-center justify-center">
            <span className="text-sm font-medium text-primary-600">U</span>
          </div>
        </div>
      </div>

      {/* Active filters display */}
      {(filters.priority || filters.phase || filters.status || filters.tag) && (
        <div className="mt-3 flex items-center space-x-2">
          <span className="text-sm text-gray-500">Active filters:</span>
          
          {filters.priority && (
            <span className="inline-flex items-center px-2 py-1 text-xs bg-blue-100 text-blue-800 rounded-full">
              Priority: {filters.priority}
              <button
                onClick={() => setFilters({ priority: undefined })}
                className="ml-1 text-blue-600 hover:text-blue-800"
              >
                ×
              </button>
            </span>
          )}
          
          {filters.phase && (
            <span className="inline-flex items-center px-2 py-1 text-xs bg-green-100 text-green-800 rounded-full">
              Phase: {filters.phase}
              <button
                onClick={() => setFilters({ phase: undefined })}
                className="ml-1 text-green-600 hover:text-green-800"
              >
                ×
              </button>
            </span>
          )}
          
          {filters.status && (
            <span className="inline-flex items-center px-2 py-1 text-xs bg-yellow-100 text-yellow-800 rounded-full">
              Status: {filters.status}
              <button
                onClick={() => setFilters({ status: undefined })}
                className="ml-1 text-yellow-600 hover:text-yellow-800"
              >
                ×
              </button>
            </span>
          )}
          
          {filters.tag && (
            <span className="inline-flex items-center px-2 py-1 text-xs bg-purple-100 text-purple-800 rounded-full">
              Tag: {filters.tag}
              <button
                onClick={() => setFilters({ tag: undefined })}
                className="ml-1 text-purple-600 hover:text-purple-800"
              >
                ×
              </button>
            </span>
          )}
          
          <button
            onClick={() => setFilters({})}
            className="text-xs text-gray-500 hover:text-gray-700"
          >
            Clear all
          </button>
        </div>
      )}
    </header>
  );
};

export default Header;