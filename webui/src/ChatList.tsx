import React from 'react';
import { NavLink, useParams } from 'react-router-dom';
import { useAppState } from './state';

const ChatList: React.FC = () => {
  const { chats } = useAppState();
  const { chatId } = useParams();
  return (
    <div className="space-y-2">
      {chats.map(chat => (
        <NavLink
          key={chat.id}
          to={`/chats/${chat.id}`}
          className={({ isActive }: { isActive: boolean }) => `flex items-center gap-3 px-4 py-3 cursor-pointer hover:bg-cyan-100 dark:hover:bg-cyan-900 rounded-lg ${isActive || chat.id === chatId ? 'bg-cyan-100 dark:bg-cyan-900' : ''}`}
        >
          <img src={chat.avatar} alt={chat.name} className="w-10 h-10 rounded-full" />
          <div>
            <div className="font-semibold text-gray-800 dark:text-gray-100">{chat.name}</div>
            <div className="text-xs text-gray-500 dark:text-gray-400 truncate w-32">{chat.lastMessage || ''}</div>
          </div>
        </NavLink>
      ))}
    </div>
  );
};

export default ChatList; 