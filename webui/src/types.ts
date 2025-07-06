export interface User {
  id: string;
  name: string;
  avatar: string;
  status: string;
}

export interface Message {
  id: string;
  chatId: string;
  senderId: string;
  text: string;
  time: string;
  fromMe?: boolean;
}

export interface Chat {
  id: string;
  name: string;
  avatar: string;
  lastMessage?: string;
  unreadCount?: number;
  participants: string[];
}

export interface Contact {
  id: string;
  name: string;
  avatar: string;
  status: string;
}

export interface Group {
  id: string;
  name: string;
  avatar: string;
  members: string[];
  description?: string;
} 