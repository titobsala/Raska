// Utility functions

import { clsx, type ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';
import type { Priority, TaskStatus } from '@/types';

// Combine class names with Tailwind merge
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

// Priority utilities
export function getPriorityColor(priority: Priority): string {
  switch (priority) {
    case 'Low':
      return 'text-gray-500 bg-gray-100';
    case 'Medium':
      return 'text-yellow-700 bg-yellow-100';
    case 'High':
      return 'text-orange-700 bg-orange-100';
    case 'Critical':
      return 'text-red-700 bg-red-100';
    default:
      return 'text-gray-500 bg-gray-100';
  }
}

export function getPriorityIcon(priority: Priority): string {
  switch (priority) {
    case 'Low':
      return '🔽';
    case 'Medium':
      return '🔶';
    case 'High':
      return '🔺';
    case 'Critical':
      return '🚨';
    default:
      return '⚪';
  }
}

// Status utilities
export function getStatusColor(status: TaskStatus): string {
  switch (status) {
    case 'todo':
      return 'text-gray-700 bg-gray-100';
    case 'in-progress':
      return 'text-blue-700 bg-blue-100';
    case 'completed':
      return 'text-green-700 bg-green-100';
    case 'blocked':
      return 'text-red-700 bg-red-100';
    default:
      return 'text-gray-500 bg-gray-100';
  }
}

export function getStatusIcon(status: TaskStatus): string {
  switch (status) {
    case 'todo':
      return '📝';
    case 'in-progress':
      return '🔄';
    case 'completed':
      return '✅';
    case 'blocked':
      return '🚫';
    default:
      return '⚪';
  }
}

// Date utilities
export function formatDate(dateString: string): string {
  const date = new Date(dateString);
  return date.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
  });
}

export function formatDateTime(dateString: string): string {
  const date = new Date(dateString);
  return date.toLocaleString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  });
}

export function formatRelativeTime(dateString: string): string {
  const date = new Date(dateString);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffSeconds = Math.floor(diffMs / 1000);
  const diffMinutes = Math.floor(diffSeconds / 60);
  const diffHours = Math.floor(diffMinutes / 60);
  const diffDays = Math.floor(diffHours / 24);

  if (diffDays > 0) {
    return `${diffDays} day${diffDays !== 1 ? 's' : ''} ago`;
  } else if (diffHours > 0) {
    return `${diffHours} hour${diffHours !== 1 ? 's' : ''} ago`;
  } else if (diffMinutes > 0) {
    return `${diffMinutes} minute${diffMinutes !== 1 ? 's' : ''} ago`;
  } else {
    return 'Just now';
  }
}

// Time utilities
export function formatDuration(hours: number): string {
  if (hours < 1) {
    const minutes = Math.round(hours * 60);
    return `${minutes}m`;
  } else if (hours < 24) {
    const wholeHours = Math.floor(hours);
    const minutes = Math.round((hours - wholeHours) * 60);
    return minutes > 0 ? `${wholeHours}h ${minutes}m` : `${wholeHours}h`;
  } else {
    const days = Math.floor(hours / 24);
    const remainingHours = Math.floor(hours % 24);
    return remainingHours > 0 ? `${days}d ${remainingHours}h` : `${days}d`;
  }
}

// Text utilities
export function truncateText(text: string, maxLength: number): string {
  if (text.length <= maxLength) {
    return text;
  }
  return text.slice(0, maxLength).trim() + '...';
}

export function highlightText(text: string, query: string): string {
  if (!query) return text;
  
  const regex = new RegExp(`(${query})`, 'gi');
  return text.replace(regex, '<mark class="bg-yellow-200">$1</mark>');
}

// Array utilities
export function groupBy<T>(array: T[], keyFn: (item: T) => string): Record<string, T[]> {
  return array.reduce((groups, item) => {
    const key = keyFn(item);
    if (!groups[key]) {
      groups[key] = [];
    }
    groups[key].push(item);
    return groups;
  }, {} as Record<string, T[]>);
}

// Debounce utility
export function debounce<T extends (...args: any[]) => any>(
  func: T,
  delay: number
): (...args: Parameters<T>) => void {
  let timeoutId: ReturnType<typeof setTimeout>;
  
  return (...args: Parameters<T>) => {
    clearTimeout(timeoutId);
    timeoutId = setTimeout(() => func(...args), delay);
  };
}

// Local storage utilities
export function getFromStorage<T>(key: string, defaultValue: T): T {
  try {
    const item = localStorage.getItem(key);
    return item ? JSON.parse(item) : defaultValue;
  } catch {
    return defaultValue;
  }
}

export function setToStorage<T>(key: string, value: T): void {
  try {
    localStorage.setItem(key, JSON.stringify(value));
  } catch {
    // Silently fail if localStorage is not available
  }
}

// Validation utilities
export function isValidEmail(email: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(email);
}

export function isValidUrl(url: string): boolean {
  try {
    new URL(url);
    return true;
  } catch {
    return false;
  }
}

// Error handling
export function getErrorMessage(error: unknown): string {
  if (error instanceof Error) {
    return error.message;
  }
  if (typeof error === 'string') {
    return error;
  }
  return 'An unknown error occurred';
}