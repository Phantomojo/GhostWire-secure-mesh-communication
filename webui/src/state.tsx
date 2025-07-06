import React, { createContext, useContext, useState, useEffect } from 'react';
import type { User, Chat, Message, Contact, Group } from './types';
import { getUser, getChats, getMessages, getContacts, getGroups } from './services/api';
import { connectToMessageSocket } from './services/ws';

interface AppState {
  user: User | null;
  chats: Chat[];
  messages: Message[];
  contacts: Contact[];
  groups: Group[];
  loading: boolean;
  error: string | null;
  setUser: (user: User) => void;
  setChats: (chats: Chat[]) => void;
  setMessages: (messages: Message[]) => void;
  setContacts: (contacts: Contact[]) => void;
  setGroups: (groups: Group[]) => void;
}

const AppStateContext = createContext<AppState | undefined>(undefined);

export const AppStateProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);
  const [chats, setChats] = useState<Chat[]>([]);
  const [messages, setMessages] = useState<Message[]>([]);
  const [contacts, setContacts] = useState<Contact[]>([]);
  const [groups, setGroups] = useState<Group[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function fetchData() {
      setLoading(true);
      setError(null);
      try {
        const [user, chats, contacts, groups] = await Promise.all([
          getUser(),
          getChats(),
          getContacts(),
          getGroups(),
        ]);
        setUser(user);
        setChats(chats);
        setContacts(contacts);
        setGroups(groups);
        // Fetch messages for the first chat as default
        if (chats.length > 0) {
          const msgs = await getMessages(chats[0].id);
          setMessages(msgs);
        }
      } catch (e: any) {
        setError(e.message || 'Failed to load data');
      } finally {
        setLoading(false);
      }
    }
    fetchData();
  }, []);

  // WebSocket for real-time messages
  useEffect(() => {
    const disconnect = connectToMessageSocket((msg) => {
      // Only add if chat exists
      if (chats.some(c => c.id === msg.chatId)) {
        setMessages(prev => [...prev, msg]);
      }
    });
    return () => { disconnect(); };
  }, [chats]);

  return (
    <AppStateContext.Provider value={{ user, chats, messages, contacts, groups, loading, error, setUser, setChats, setMessages, setContacts, setGroups }}>
      {children}
    </AppStateContext.Provider>
  );
};

export function useAppState() {
  const ctx = useContext(AppStateContext);
  if (!ctx) throw new Error('useAppState must be used within AppStateProvider');
  return ctx;
} 