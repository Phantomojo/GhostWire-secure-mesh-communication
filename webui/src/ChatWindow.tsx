import React, { useState, useEffect } from 'react';
import { useParams } from 'react-router-dom';
import { useAppState } from './state';
import { getMessages, sendMessage } from './services/api';
import MessageBubble from './MessageBubble';
import MessageInput from './MessageInput';

const ChatWindow: React.FC = () => {
  const { chatId } = useParams();
  const { chats, messages, setMessages } = useAppState();
  const chat = chats.find(c => c.id === chatId) || chats[0];
  const [loading, setLoading] = useState(false);
  const [sending, setSending] = useState(false);
  const [sendError, setSendError] = useState<string | null>(null);
  const chatMessages = messages.filter(m => m.chatId === chat.id);

  useEffect(() => {
    let mounted = true;
    async function fetchMsgs() {
      setLoading(true);
      try {
        const msgs = await getMessages(chat.id);
        if (mounted) setMessages(msgs);
      } finally {
        if (mounted) setLoading(false);
      }
    }
    if (chat && chat.id) fetchMsgs();
    return () => { mounted = false; };
  }, [chat.id, setMessages]);

  const [input, setInput] = useState('');

  async function handleSend(e: React.FormEvent) {
    e.preventDefault();
    if (!input.trim()) return;
    setSending(true);
    setSendError(null);
    try {
      const msg = await sendMessage(chat.id, input);
      setMessages([...messages, msg]);
      setInput('');
    } catch (err: any) {
      setSendError(err.message || 'Failed to send message');
    } finally {
      setSending(false);
    }
  }

  return (
    <div className="flex flex-col h-full">
      {/* Chat header */}
      <header className="flex items-center gap-4 px-6 py-4 border-b border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800">
        <img src={chat.avatar} alt={chat.name} className="w-10 h-10 rounded-full" />
        <div className="flex-1">
          <div className="font-semibold text-gray-800 dark:text-gray-100">{chat.name}</div>
          <div className="text-xs text-gray-500 dark:text-gray-400">Online</div>
        </div>
        {/* Actions */}
        <button className="text-cyan-500 hover:text-cyan-700">Call</button>
        <button className="text-cyan-500 hover:text-cyan-700">Video</button>
        <button className="text-cyan-500 hover:text-cyan-700">Info</button>
      </header>
      {/* Messages */}
      <div className="flex-1 overflow-y-auto px-6 py-4 space-y-4 bg-gray-50 dark:bg-gray-900">
        {loading ? (
          <div className="text-cyan-500 text-center">Loading messages...</div>
        ) : (
          chatMessages.map(msg => (
            <MessageBubble key={msg.id} fromMe={msg.fromMe ?? false} text={msg.text} time={msg.time} />
          ))
        )}
      </div>
      {/* Input bar */}
      <form onSubmit={handleSend} className="relative">
        <MessageInput input={input} setInput={setInput} />
        {sending && <div className="absolute right-8 top-1/2 -translate-y-1/2 text-cyan-500">Sending...</div>}
        {sendError && <div className="absolute left-8 top-1/2 -translate-y-1/2 text-red-500">{sendError}</div>}
      </form>
    </div>
  );
};

export default ChatWindow; 