import React from 'react';
import { cn } from '@/utils';
import type { Priority, TaskStatus } from '@/types';

interface BadgeProps {
  children: React.ReactNode;
  variant?: 'default' | 'primary' | 'secondary' | 'success' | 'warning' | 'danger';
  size?: 'sm' | 'md';
  className?: string;
}

const Badge: React.FC<BadgeProps> = ({ 
  children, 
  variant = 'default', 
  size = 'sm',
  className 
}) => {
  const baseClasses = 'inline-flex items-center font-medium rounded-full';
  
  const variantClasses = {
    default: 'bg-gray-100 text-gray-800',
    primary: 'bg-primary-100 text-primary-800',
    secondary: 'bg-gray-100 text-gray-600',
    success: 'bg-green-100 text-green-800',
    warning: 'bg-yellow-100 text-yellow-800',
    danger: 'bg-red-100 text-red-800',
  };
  
  const sizeClasses = {
    sm: 'px-2 py-0.5 text-xs',
    md: 'px-3 py-1 text-sm',
  };

  return (
    <span
      className={cn(
        baseClasses,
        variantClasses[variant],
        sizeClasses[size],
        className
      )}
    >
      {children}
    </span>
  );
};

// Specialized priority badge
interface PriorityBadgeProps {
  priority: Priority;
  className?: string;
}

export const PriorityBadge: React.FC<PriorityBadgeProps> = ({ priority, className }) => {
  const variants = {
    Low: 'default' as const,
    Medium: 'warning' as const,
    High: 'primary' as const,
    Critical: 'danger' as const,
  };

  const icons = {
    Low: '🔽',
    Medium: '🔶',
    High: '🔺',
    Critical: '🚨',
  };

  return (
    <Badge variant={variants[priority]} className={className}>
      <span className="mr-1">{icons[priority]}</span>
      {priority}
    </Badge>
  );
};

// Specialized status badge
interface StatusBadgeProps {
  status: TaskStatus;
  className?: string;
}

export const StatusBadge: React.FC<StatusBadgeProps> = ({ status, className }) => {
  const variants = {
    'todo': 'default' as const,
    'in-progress': 'primary' as const,
    'completed': 'success' as const,
    'blocked': 'danger' as const,
  };

  const icons = {
    'todo': '📝',
    'in-progress': '🔄',
    'completed': '✅',
    'blocked': '🚫',
  };

  const labels = {
    'todo': 'To Do',
    'in-progress': 'In Progress',
    'completed': 'Completed',
    'blocked': 'Blocked',
  };

  return (
    <Badge variant={variants[status]} className={className}>
      <span className="mr-1">{icons[status]}</span>
      {labels[status]}
    </Badge>
  );
};

export default Badge;