import React from 'react';

interface Chat {
  id: string;
  name: string;
  avatar: string;
  lastMessage: string;
  lastMessageTime: Date;
  unreadCount: number;
  isOnline: boolean;
  isGroup: boolean;
  members?: string[];
}

interface User {
  id: string;
  name: string;
  avatar: string;
  status: string;
  isOnline: boolean;
}

interface ChatSidebarProps {
  chats: Chat[];
  currentChat: Chat | null;
  currentUser: User;
  searchQuery: string;
  onSearchChange: (query: string) => void;
  onChatSelect: (chat: Chat) => void;
}

const ChatSidebar: React.FC<ChatSidebarProps> = ({
  chats,
  currentChat,
  currentUser,
  searchQuery,
  onSearchChange,
  onChatSelect
}) => {
  const formatTime = (date: Date) => {
    const now = new Date();
    const diff = now.getTime() - date.getTime();
    const minutes = Math.floor(diff / 60000);
    const hours = Math.floor(diff / 3600000);
    const days = Math.floor(diff / 86400000);

    if (minutes < 1) return 'now';
    if (minutes < 60) return `${minutes}m`;
    if (hours < 24) return `${hours}h`;
    if (days < 7) return `${days}d`;
    return date.toLocaleDateString();
  };

  return (
    <div className="chat-sidebar">
      {/* Header */}
      <div className="sidebar-header">
        <div className="user-profile">
          <div className="user-avatar">
            <span className="avatar-emoji">{currentUser.avatar}</span>
            {currentUser.isOnline && <div className="online-indicator"></div>}
          </div>
          <div className="user-info">
            <h3>{currentUser.name}</h3>
            <p>{currentUser.status}</p>
          </div>
        </div>
        <div className="header-actions">
          <button className="action-btn">📝</button>
          <button className="action-btn">⋮</button>
        </div>
      </div>

      {/* Search */}
      <div className="search-container">
        <div className="search-input-wrapper">
          <span className="search-icon">🔍</span>
          <input
            type="text"
            placeholder="Search or start new chat"
            value={searchQuery}
            onChange={(e) => onSearchChange(e.target.value)}
            className="search-input"
          />
        </div>
      </div>

      {/* Chat List */}
      <div className="chat-list">
        {chats.map((chat) => (
          <div
            key={chat.id}
            className={`chat-item ${currentChat?.id === chat.id ? 'active' : ''}`}
            onClick={() => onChatSelect(chat)}
          >
            <div className="chat-avatar">
              <span className="avatar-emoji">{chat.avatar}</span>
              {chat.isOnline && <div className="online-indicator"></div>}
            </div>
            <div className="chat-info">
              <div className="chat-header">
                <h4>{chat.name}</h4>
                <span className="chat-time">{formatTime(chat.lastMessageTime)}</span>
              </div>
              <div className="chat-preview">
                <p className="last-message">{chat.lastMessage}</p>
                {chat.unreadCount > 0 && (
                  <div className="unread-badge">{chat.unreadCount}</div>
                )}
              </div>
            </div>
          </div>
        ))}
      </div>

      {/* New Chat Button */}
      <div className="new-chat-button">
        <button className="new-chat-btn">
          <span>✏️</span>
          <span>New Chat</span>
        </button>
      </div>
    </div>
  );
};

export default ChatSidebar; 