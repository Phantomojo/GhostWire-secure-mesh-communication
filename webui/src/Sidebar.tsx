import React from 'react';
import { NavLink } from 'react-router-dom';
import { useAppState } from './state';

const Sidebar: React.FC = () => {
  const { user } = useAppState();
  const navItems = [
    { name: 'Chats', to: '/chats/1' },
    { name: 'Contacts', to: '/contacts' },
    { name: 'Groups', to: '/groups' },
    { name: 'Settings', to: '/settings' },
    { name: 'Feed', to: '/feed' },
    { name: 'Help', to: '/help' },
    { name: 'Onboarding', to: '/onboarding' },
  ];
  return (
    <aside className="w-20 md:w-72 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 flex flex-col items-center md:items-stretch py-4">
      {/* User profile */}
      <div className="flex flex-col items-center mb-8">
        <img src={user?.avatar} alt={user?.name} className="w-12 h-12 rounded-full border-2 border-cyan-500" />
        <span className="hidden md:block mt-2 text-sm font-semibold text-gray-700 dark:text-gray-200">{user?.name}</span>
      </div>
      {/* Navigation */}
      <nav className="flex flex-col gap-2 mb-6">
        {navItems.map(item => (
          <NavLink
            key={item.name}
            to={item.to}
            className={({ isActive }: { isActive: boolean }) => `px-4 py-2 rounded-lg text-left ${isActive ? 'bg-cyan-100 dark:bg-cyan-900 font-bold' : 'hover:bg-cyan-50 dark:hover:bg-cyan-900'}`}
          >
            {item.name}
          </NavLink>
        ))}
      </nav>
      {/* Search bar */}
      <div className="px-4 mb-4">
        <input
          type="text"
          placeholder="Search..."
          className="w-full px-3 py-2 rounded-full border border-gray-300 dark:border-gray-600 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-100 focus:outline-none"
        />
      </div>
      {/* Chat list placeholder */}
      <div className="flex-1 overflow-y-auto px-2">
        <div className="text-gray-400 text-center mt-8">Chat list goes here</div>
      </div>
    </aside>
  );
};

export default Sidebar; 