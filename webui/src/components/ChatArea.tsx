import React, { useEffect, useRef } from 'react';

interface Message {
  id: string;
  content: string;
  sender: string;
  timestamp: Date;
  isOwn: boolean;
  status: 'sent' | 'delivered' | 'read';
  type: 'text' | 'image' | 'file' | 'voice';
}

interface ChatAreaProps {
  messages: Message[];
  isTyping: boolean;
}

const ChatArea: React.FC<ChatAreaProps> = ({ messages, isTyping }) => {
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const formatTime = (date: Date) => {
    return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'sent':
        return '✓';
      case 'delivered':
        return '✓✓';
      case 'read':
        return '✓✓';
      default:
        return '';
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'sent':
        return 'text-gray-400';
      case 'delivered':
        return 'text-gray-400';
      case 'read':
        return 'text-blue-500';
      default:
        return 'text-gray-400';
    }
  };

  const groupMessagesByDate = (messages: Message[]) => {
    const groups: { [key: string]: Message[] } = {};
    
    messages.forEach(message => {
      const date = message.timestamp.toDateString();
      if (!groups[date]) {
        groups[date] = [];
      }
      groups[date].push(message);
    });
    
    return groups;
  };

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    const today = new Date();
    const yesterday = new Date(today);
    yesterday.setDate(yesterday.getDate() - 1);

    if (date.toDateString() === today.toDateString()) {
      return 'Today';
    } else if (date.toDateString() === yesterday.toDateString()) {
      return 'Yesterday';
    } else {
      return date.toLocaleDateString();
    }
  };

  const messageGroups = groupMessagesByDate(messages);

  return (
    <div className="chat-area">
      <div className="messages-container">
        {Object.entries(messageGroups).map(([date, dateMessages]) => (
          <div key={date} className="message-group">
            <div className="date-separator">
              <span>{formatDate(date)}</span>
            </div>
            
            {dateMessages.map((message) => (
              <div
                key={message.id}
                className={`message-wrapper ${message.isOwn ? 'own' : 'other'}`}
              >
                <div className={`message ${message.isOwn ? 'own' : 'other'}`}>
                  {!message.isOwn && (
                    <div className="message-avatar">
                      <span className="avatar-emoji">
                        {message.sender === 'System' ? '🤖' : '👤'}
                      </span>
                    </div>
                  )}
                  
                  <div className="message-content">
                    {!message.isOwn && message.sender !== 'System' && (
                      <div className="message-sender">{message.sender}</div>
                    )}
                    
                    <div className={`message-bubble ${message.sender === 'System' ? 'system' : ''}`}>
                      {message.type === 'text' && (
                        <p className="message-text">{message.content}</p>
                      )}
                      {message.type === 'image' && (
                        <div className="message-image">
                          <img src={message.content} alt="Shared image" />
                        </div>
                      )}
                      {message.type === 'file' && (
                        <div className="message-file">
                          <span>📎</span>
                          <span>{message.content}</span>
                        </div>
                      )}
                      {message.type === 'voice' && (
                        <div className="message-voice">
                          <span>🎤</span>
                          <span>Voice message</span>
                        </div>
                      )}
                    </div>
                    
                    <div className="message-meta">
                      <span className="message-time">{formatTime(message.timestamp)}</span>
                      {message.isOwn && (
                        <span className={`message-status ${getStatusColor(message.status)}`}>
                          {getStatusIcon(message.status)}
                        </span>
                      )}
                    </div>
                  </div>
                </div>
              </div>
            ))}
          </div>
        ))}
        
        {isTyping && (
          <div className="message-wrapper other">
            <div className="message other">
              <div className="message-avatar">
                <span className="avatar-emoji">👤</span>
              </div>
              <div className="message-content">
                <div className="message-bubble typing">
                  <div className="typing-indicator">
                    <span></span>
                    <span></span>
                    <span></span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}
        
        <div ref={messagesEndRef} />
      </div>
    </div>
  );
};

export default ChatArea; 