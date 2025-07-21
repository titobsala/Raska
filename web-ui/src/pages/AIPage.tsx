import React, { useState } from 'react';
import { Brain, MessageSquare, BarChart3, Network } from 'lucide-react';
import Layout from '@/components/Layout/Layout';
import Card from '@/components/UI/Card';
import Button from '@/components/UI/Button';
import ChatInterface from '@/components/AI/ChatInterface';
import { useAppStore } from '@/stores/appStore';

const AIPage: React.FC = () => {
  const { currentProject } = useAppStore();
  const [activeTab, setActiveTab] = useState<'chat' | 'analysis' | 'insights'>('chat');

  const tabs = [
    { id: 'chat' as const, label: 'Chat', icon: MessageSquare },
    { id: 'analysis' as const, label: 'Analysis', icon: BarChart3 },
    { id: 'insights' as const, label: 'Insights', icon: Network },
  ];

  const renderTabContent = () => {
    switch (activeTab) {
      case 'chat':
        return (
          <div className="h-[600px]">
            <ChatInterface projectName={currentProject?.metadata?.name} />
          </div>
        );
      
      case 'analysis':
        return (
          <Card>
            <Card.Header>
              <Card.Title>Project Analysis</Card.Title>
            </Card.Header>
            <Card.Content>
              <div className="text-center py-12">
                <BarChart3 className="h-16 w-16 text-gray-300 mx-auto mb-4" />
                <h3 className="text-lg font-medium text-gray-900 mb-2">
                  AI Analysis Coming Soon
                </h3>
                <p className="text-gray-600 mb-6">
                  Get deep insights into your project's health, progress, and optimization opportunities.
                </p>
                <Button disabled>
                  <BarChart3 className="h-4 w-4 mr-2" />
                  Analyze Project
                </Button>
              </div>
            </Card.Content>
          </Card>
        );
      
      case 'insights':
        return (
          <Card>
            <Card.Header>
              <Card.Title>Project Insights</Card.Title>
            </Card.Header>
            <Card.Content>
              <div className="text-center py-12">
                <Network className="h-16 w-16 text-gray-300 mx-auto mb-4" />
                <h3 className="text-lg font-medium text-gray-900 mb-2">
                  AI Insights Coming Soon
                </h3>
                <p className="text-gray-600 mb-6">
                  Discover hidden patterns, potential risks, and strategic recommendations for your project.
                </p>
                <Button disabled>
                  <Network className="h-4 w-4 mr-2" />
                  Generate Insights
                </Button>
              </div>
            </Card.Content>
          </Card>
        );
      
      default:
        return null;
    }
  };

  return (
    <Layout>
      <div className="space-y-6">
        {/* Header */}
        <div className="flex items-center gap-3">
          <div className="flex items-center justify-center w-10 h-10 bg-primary-100 rounded-lg">
            <Brain className="h-6 w-6 text-primary-600" />
          </div>
          <div>
            <h1 className="text-2xl font-bold text-gray-900">AI Assistant</h1>
            <p className="text-gray-600">
              Get intelligent help with your project planning and management
            </p>
          </div>
        </div>

        {/* Project Context */}
        {currentProject && (
          <Card>
            <Card.Content className="py-3">
              <div className="flex items-center gap-2 text-sm text-gray-600">
                <span className="font-medium">Current Project:</span>
                <span className="px-2 py-1 bg-primary-100 text-primary-800 rounded-md">
                  {currentProject.metadata.name}
                </span>
                <span className="text-gray-400">•</span>
                <span>
                  {currentProject.tasks?.length || 0} tasks
                </span>
              </div>
            </Card.Content>
          </Card>
        )}

        {/* Navigation Tabs */}
        <div className="border-b border-gray-200">
          <nav className="flex space-x-8">
            {tabs.map((tab) => {
              const Icon = tab.icon;
              const isActive = activeTab === tab.id;
              
              return (
                <button
                  key={tab.id}
                  onClick={() => setActiveTab(tab.id)}
                  className={`flex items-center gap-2 py-4 px-1 border-b-2 font-medium text-sm transition-colors ${
                    isActive
                      ? 'border-primary-500 text-primary-600'
                      : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                  }`}
                >
                  <Icon className="h-4 w-4" />
                  {tab.label}
                </button>
              );
            })}
          </nav>
        </div>

        {/* Tab Content */}
        <div className="pb-6">
          {renderTabContent()}
        </div>
      </div>
    </Layout>
  );
};

export default AIPage;