import React from 'react';
import { useAppState } from './state';

const GroupList: React.FC = () => {
  const { groups } = useAppState();
  return (
    <div className="space-y-2 p-4">
      {groups.map(group => (
        <div key={group.id} className="flex items-center gap-3 py-2">
          <div className="w-10 h-10 rounded-full bg-cyan-500 flex items-center justify-center text-white font-bold">
            {group.name.slice(0, 2).toUpperCase()}
          </div>
          <div>
            <div className="font-semibold text-gray-800 dark:text-gray-100">{group.name}</div>
            <div className="text-xs text-gray-500 dark:text-gray-400">{group.members.length} members</div>
          </div>
        </div>
      ))}
    </div>
  );
};

export default GroupList; 