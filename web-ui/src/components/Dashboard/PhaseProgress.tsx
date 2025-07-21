import React from 'react';
import { useAppStore } from '@/stores/appStore';
import { Card } from '@/components/UI';

const PhaseProgress: React.FC = () => {
  const { currentProject } = useAppStore();
  
  if (!currentProject) return null;

  // Group tasks by phase and calculate progress
  const phaseStats = currentProject.phases.map(phase => {
    const phaseTasks = currentProject.tasks.filter(task => task.phase.name === phase.name);
    const completedTasks = phaseTasks.filter(task => task.status === 'completed');
    const progress = phaseTasks.length > 0 ? (completedTasks.length / phaseTasks.length) * 100 : 0;
    
    return {
      ...phase,
      totalTasks: phaseTasks.length,
      completedTasks: completedTasks.length,
      progress: Math.round(progress),
    };
  }).filter(phase => phase.totalTasks > 0); // Only show phases with tasks

  return (
    <Card>
      <Card.Header>
        <Card.Title>Phase Progress</Card.Title>
      </Card.Header>

      <Card.Content>
        <div className="space-y-4">
          {phaseStats.length > 0 ? (
            phaseStats.map((phase) => (
              <div key={phase.name} className="space-y-2">
                <div className="flex items-center justify-between">
                  <div className="flex items-center space-x-2">
                    <span className="text-lg">{phase.emoji}</span>
                    <div>
                      <p className="text-sm font-medium text-gray-900">{phase.name}</p>
                      <p className="text-xs text-gray-500">{phase.description}</p>
                    </div>
                  </div>
                  <div className="text-right">
                    <p className="text-sm font-medium text-gray-900">{phase.progress}%</p>
                    <p className="text-xs text-gray-500">
                      {phase.completedTasks}/{phase.totalTasks}
                    </p>
                  </div>
                </div>
                
                <div className="w-full bg-gray-200 rounded-full h-2">
                  <div
                    className={`h-2 rounded-full transition-all duration-300 ${
                      phase.progress === 100
                        ? 'bg-green-500'
                        : phase.progress > 50
                        ? 'bg-blue-500'
                        : 'bg-yellow-500'
                    }`}
                    style={{ width: `${phase.progress}%` }}
                  />
                </div>
              </div>
            ))
          ) : (
            <div className="text-center py-4">
              <p className="text-sm text-gray-500">No phases with tasks</p>
            </div>
          )}
        </div>

        {/* Phase summary */}
        {phaseStats.length > 0 && (
          <div className="mt-6 pt-4 border-t border-gray-200">
            <div className="grid grid-cols-2 gap-4 text-center">
              <div>
                <p className="text-2xl font-bold text-gray-900">
                  {phaseStats.filter(p => p.progress === 100).length}
                </p>
                <p className="text-xs text-gray-500">Completed Phases</p>
              </div>
              <div>
                <p className="text-2xl font-bold text-gray-900">
                  {phaseStats.filter(p => p.progress > 0 && p.progress < 100).length}
                </p>
                <p className="text-xs text-gray-500">In Progress</p>
              </div>
            </div>
          </div>
        )}
      </Card.Content>
    </Card>
  );
};

export default PhaseProgress;