import React, { useState, useRef, useEffect } from 'react';
import { Send, Bot, User, Loader2 } from 'lucide-react';
import Card from '@/components/UI/Card';
import Button from '@/components/UI/Button';

interface Message {
  id: string;
  type: 'user' | 'ai';
  content: string;
  timestamp: Date;
}

interface ChatInterfaceProps {
  projectName?: string;
  className?: string;
}

const ChatInterface: React.FC<ChatInterfaceProps> = ({ 
  projectName, 
  className = '' 
}) => {
  const [messages, setMessages] = useState<Message[]>([]);
  const [input, setInput] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [isStreaming, setIsStreaming] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const abortControllerRef = useRef<AbortController | null>(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const addMessage = (type: 'user' | 'ai', content: string): string => {
    const id = `msg_${Date.now()}_${Math.random()}`;
    const message: Message = {
      id,
      type,
      content,
      timestamp: new Date(),
    };
    setMessages(prev => [...prev, message]);
    return id;
  };

  const updateMessage = (id: string, content: string) => {
    setMessages(prev => 
      prev.map(msg => 
        msg.id === id 
          ? { ...msg, content }
          : msg
      )
    );
  };

  const sendMessage = async (message: string) => {
    if (!message.trim() || isLoading) return;

    addMessage('user', message);
    setInput('');
    setIsLoading(true);
    setIsStreaming(true);

    // Create abort controller for this request
    abortControllerRef.current = new AbortController();

    try {
      const response = await fetch('/api/ai/chat/stream', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          message,
          project_context: projectName,
        }),
        signal: abortControllerRef.current.signal,
      });

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      const reader = response.body?.getReader();
      if (!reader) {
        throw new Error('No response body');
      }

      const decoder = new TextDecoder();
      let aiMessageId = '';
      let accumulatedContent = '';

      while (true) {
        const { done, value } = await reader.read();
        
        if (done) break;

        const chunk = decoder.decode(value, { stream: true });
        const lines = chunk.split('\n');

        for (const line of lines) {
          if (line.startsWith('data: ')) {
            const data = line.slice(6);
            
            if (data === '[DONE]') {
              setIsStreaming(false);
              break;
            }

            try {
              const parsed = JSON.parse(data);
              
              if (parsed.type === 'chunk' && parsed.content) {
                accumulatedContent += parsed.content;
                
                if (!aiMessageId) {
                  aiMessageId = addMessage('ai', parsed.content);
                } else {
                  updateMessage(aiMessageId, accumulatedContent);
                }
              } else if (parsed.type === 'done') {
                setIsStreaming(false);
                break;
              }
            } catch (e) {
              // Skip invalid JSON
              console.warn('Failed to parse SSE data:', e);
            }
          }
        }
      }
    } catch (error: any) {
      if (error.name === 'AbortError') {
        console.log('Request was aborted');
      } else {
        console.error('Chat error:', error);
        addMessage('ai', 'Sorry, I encountered an error. Please try again.');
      }
    } finally {
      setIsLoading(false);
      setIsStreaming(false);
      abortControllerRef.current = null;
    }
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    sendMessage(input);
  };

  const stopGeneration = () => {
    if (abortControllerRef.current) {
      abortControllerRef.current.abort();
      setIsStreaming(false);
      setIsLoading(false);
    }
  };

  const formatTimestamp = (date: Date) => {
    return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  };

  return (
    <Card className={`flex flex-col h-full ${className}`} padding={false}>
      <Card.Header className="border-b border-gray-200 p-4">
        <Card.Title className="flex items-center gap-2">
          <Bot className="h-5 w-5 text-primary-600" />
          AI Assistant
          {projectName && (
            <span className="text-sm font-normal text-gray-500">
              • {projectName}
            </span>
          )}
        </Card.Title>
      </Card.Header>

      {/* Messages */}
      <div className="flex-1 overflow-y-auto p-4 space-y-4">
        {messages.length === 0 && (
          <div className="text-center text-gray-500 py-8">
            <Bot className="h-12 w-12 text-gray-300 mx-auto mb-4" />
            <p className="text-lg font-medium">Start a conversation</p>
            <p className="text-sm">Ask me about your project, tasks, or anything else!</p>
          </div>
        )}

        {messages.map((message) => (
          <div
            key={message.id}
            className={`flex items-start gap-3 ${
              message.type === 'user' ? 'flex-row-reverse' : ''
            }`}
          >
            <div className={`flex-shrink-0 w-8 h-8 rounded-full flex items-center justify-center ${
              message.type === 'user' 
                ? 'bg-primary-600 text-white' 
                : 'bg-gray-100 text-gray-600'
            }`}>
              {message.type === 'user' ? (
                <User className="h-4 w-4" />
              ) : (
                <Bot className="h-4 w-4" />
              )}
            </div>

            <div className={`flex-1 max-w-[80%] ${
              message.type === 'user' ? 'text-right' : ''
            }`}>
              <div className={`inline-block px-4 py-2 rounded-lg ${
                message.type === 'user'
                  ? 'bg-primary-600 text-white'
                  : 'bg-gray-100 text-gray-900'
              }`}>
                <p className="whitespace-pre-wrap">{message.content}</p>
              </div>
              <div className={`text-xs text-gray-500 mt-1 ${
                message.type === 'user' ? 'text-right' : ''
              }`}>
                {formatTimestamp(message.timestamp)}
              </div>
            </div>
          </div>
        ))}

        {isStreaming && (
          <div className="flex items-center gap-2 text-gray-500">
            <Loader2 className="h-4 w-4 animate-spin" />
            <span className="text-sm">AI is typing...</span>
            <Button
              variant="ghost"
              size="sm"
              onClick={stopGeneration}
              className="text-red-600 hover:text-red-700"
            >
              Stop
            </Button>
          </div>
        )}

        <div ref={messagesEndRef} />
      </div>

      {/* Input */}
      <div className="border-t border-gray-200 p-4">
        <form onSubmit={handleSubmit} className="flex gap-2">
          <input
            type="text"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            placeholder={isLoading ? "AI is responding..." : "Type your message..."}
            disabled={isLoading}
            className="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent disabled:bg-gray-50"
          />
          <Button
            type="submit"
            disabled={!input.trim() || isLoading}
            className="px-4 py-2"
          >
            {isLoading ? (
              <Loader2 className="h-4 w-4 animate-spin" />
            ) : (
              <Send className="h-4 w-4" />
            )}
          </Button>
        </form>
      </div>
    </Card>
  );
};

export default ChatInterface;