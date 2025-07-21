import React from 'react';
import { cn } from '@/utils';

interface CardProps {
  children: React.ReactNode;
  className?: string;
  padding?: boolean;
  hover?: boolean;
}

const Card: React.FC<CardProps> = ({ 
  children, 
  className, 
  padding = true, 
  hover = false 
}) => {
  return (
    <div
      className={cn(
        'bg-white rounded-xl shadow-sm border border-gray-200',
        padding && 'p-6',
        hover && 'hover:shadow-md transition-shadow duration-200',
        className
      )}
    >
      {children}
    </div>
  );
};

interface CardHeaderProps {
  children: React.ReactNode;
  className?: string;
}

const CardHeader: React.FC<CardHeaderProps> = ({ children, className }) => {
  return (
    <div className={cn('mb-4', className)}>
      {children}
    </div>
  );
};

interface CardTitleProps {
  children: React.ReactNode;
  className?: string;
}

const CardTitle: React.FC<CardTitleProps> = ({ children, className }) => {
  return (
    <h3 className={cn('text-lg font-semibold text-gray-900', className)}>
      {children}
    </h3>
  );
};

interface CardContentProps {
  children: React.ReactNode;
  className?: string;
}

const CardContent: React.FC<CardContentProps> = ({ children, className }) => {
  return (
    <div className={cn('text-gray-600', className)}>
      {children}
    </div>
  );
};

// Compound component with proper typing
interface CardComponent extends React.FC<CardProps> {
  Header: React.FC<CardHeaderProps>;
  Title: React.FC<CardTitleProps>;
  Content: React.FC<CardContentProps>;
}

const CardComponent = Card as CardComponent;
CardComponent.Header = CardHeader;
CardComponent.Title = CardTitle;
CardComponent.Content = CardContent;

export default CardComponent;