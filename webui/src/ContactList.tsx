import React from 'react';
import { useAppState } from './state';

const ContactList: React.FC = () => {
  const { contacts } = useAppState();
  return (
    <div className="space-y-2 p-4">
      {contacts.map(contact => (
        <div key={contact.id} className="flex items-center gap-3 py-2">
          <img src={contact.avatar} alt={contact.name} className="w-10 h-10 rounded-full" />
          <div>
            <div className="font-semibold text-gray-800 dark:text-gray-100">{contact.name}</div>
            <div className="text-xs text-gray-500 dark:text-gray-400">{contact.status}</div>
          </div>
        </div>
      ))}
    </div>
  );
};

export default ContactList; 