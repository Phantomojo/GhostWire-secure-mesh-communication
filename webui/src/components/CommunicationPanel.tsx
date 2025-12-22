import React, { useState } from 'react';
import { Badge } from './DesignSystem';
import {
  HiOutlineChatBubbleLeftRight,
  HiOutlineUserGroup,
  HiOutlineRadio,
  HiOutlineExclamationTriangle,
  HiOutlinePaperAirplane,
  HiOutlineLockClosed,
  HiOutlineSignal,
  HiOutlineArrowPath,
  HiOutlineMagnifyingGlass,
  HiOutlineChevronRight,
  HiOutlineEllipsisHorizontal
} from 'react-icons/hi2';

interface Message {
  id: string;
  content: string;
  sender: string;
  recipient: string;
  timestamp: Date;
  status: 'pending' | 'sent' | 'delivered' | 'failed';
  encrypted: boolean;
  isMe?: boolean;
}

interface CommunicationPanelProps {
  messages: Message[];
  onSendMessage: (content: string, recipient: string) => void;
  onBroadcast: () => void;
  onEmergency: () => void;
  onSync: () => void;
  onAnalyze: () => void;
  backendConnected: boolean;
}

const CommunicationPanel: React.FC<CommunicationPanelProps> = ({
  messages,
  onSendMessage,
  onBroadcast,
  onEmergency,
  onSync,
  onAnalyze,
  backendConnected
}) => {
  const [messageContent, setMessageContent] = useState('');
  const [activeThread, setActiveThread] = useState('all');

  const threadList = [
    { id: 'all', name: 'Mesh Global', subText: 'Public broadcast channel', icon: HiOutlineUserGroup, status: 'online' },
    { id: 'node-alpha', name: 'Node Alpha', subText: 'Secure direct link', icon: HiOutlineSignal, status: 'online' },
    { id: 'node-beta', name: 'Node Beta', subText: 'Latency: 42ms', icon: HiOutlineSignal, status: 'online' },
    { id: 'node-gamma', name: 'Node Gamma', subText: 'Encrypted tunnel', icon: HiOutlineSignal, status: 'offline' },
  ];

  const filteredMessages = messages
    .filter(m => activeThread === 'all' || m.sender === activeThread || m.recipient === activeThread)
    .map(m => ({ ...m, isMe: m.sender === 'LOCAL_NODE' }));

  const handleSend = () => {
    if (messageContent.trim()) {
      onSendMessage(messageContent, activeThread);
      setMessageContent('');
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  };

  return (
    <div className="h-[calc(100vh-12rem)] md:h-[calc(100vh-10rem)] flex flex-col md:flex-row gap-6 animate-fade-in">
      {/* Thread List - Instagram-style Sidebar */}
      <div className="w-full md:w-80 flex flex-col glass-panel rounded-3xl border border-white/5 overflow-hidden">
        <div className="p-6 border-b border-white/5">
          <h2 className="text-xl font-bold text-white flex items-center gap-2">
            GhostComms
            <Badge variant="indigo" className="text-[10px]">E2EE</Badge>
          </h2>
        </div>

        <div className="flex-1 overflow-y-auto custom-scrollbar p-2 space-y-1">
          {threadList.map(thread => (
            <button
              key={thread.id}
              onClick={() => setActiveThread(thread.id)}
              className={`w-full flex items-center gap-4 p-4 rounded-2xl transition-all ${activeThread === thread.id
                ? 'bg-indigo-600 text-white shadow-lg shadow-indigo-600/20'
                : 'text-slate-400 hover:bg-white/5'
                }`}
            >
              <div className={`w-12 h-12 rounded-full flex items-center justify-center relative ${activeThread === thread.id ? 'bg-white/20' : 'bg-slate-900 border border-white/5'
                }`}>
                <thread.icon className="text-xl" />
                {thread.status === 'online' && (
                  <div className="absolute bottom-0 right-0 w-3 h-3 bg-emerald-500 border-2 border-slate-950 rounded-full" />
                )}
              </div>
              <div className="flex-1 text-left">
                <div className={`text-sm font-bold ${activeThread === thread.id ? 'text-white' : 'text-slate-200'}`}>
                  {thread.name}
                </div>
                <div className={`text-[10px] truncate ${activeThread === thread.id ? 'text-indigo-200' : 'text-slate-500'}`}>
                  {thread.subText}
                </div>
              </div>
              <HiOutlineChevronRight className={`text-xs ${activeThread === thread.id ? 'text-white' : 'text-slate-700'}`} />
            </button>
          ))}
        </div>

        <div className="p-4 bg-slate-900/40 border-t border-white/5 flex gap-2">
          <button onClick={onBroadcast} className="flex-1 p-2 bg-slate-800 rounded-xl text-xs font-bold text-slate-400 hover:text-white transition-all flex items-center justify-center gap-2">
            <HiOutlineRadio className="text-sm" /> Broadcast
          </button>
          <button onClick={onSync} className="p-2 bg-slate-800 rounded-xl text-slate-400 hover:text-white transition-all">
            <HiOutlineArrowPath className="text-lg" />
          </button>
        </div>
      </div>

      {/* Chat Area */}
      <div className="flex-1 flex flex-col glass-panel rounded-3xl border border-white/5 overflow-hidden">
        {/* Chat Header */}
        <div className="p-4 md:p-6 border-b border-white/5 flex items-center justify-between bg-white/[0.02]">
          <div className="flex items-center gap-4">
            <div className="w-10 h-10 bg-indigo-600/20 rounded-full flex items-center justify-center text-indigo-400">
              {threadList.find(t => t.id === activeThread)?.icon({})}
            </div>
            <div>
              <h3 className="text-sm font-bold text-white uppercase tracking-wider">
                {threadList.find(t => t.id === activeThread)?.name}
              </h3>
              <span className="text-[10px] text-indigo-400 font-bold uppercase tracking-widest">Active Link Established</span>
            </div>
          </div>
          <div className="flex items-center gap-2">
            <button onClick={onAnalyze} className="p-2 text-slate-500 hover:text-white transition-all">
              <HiOutlineMagnifyingGlass className="text-xl" />
            </button>
            <button className="p-2 text-slate-500 hover:text-white transition-all">
              <HiOutlineEllipsisHorizontal className="text-xl" />
            </button>
          </div>
        </div>

        {/* Message View - Instagram Bubbles */}
        <div className="flex-1 overflow-y-auto custom-scrollbar p-6 space-y-4 bg-slate-950/20">
          {filteredMessages.length > 0 ? (
            filteredMessages.map((msg, idx) => {
              const nextMsg = filteredMessages[idx + 1];
              const isLastInGroup = !nextMsg || nextMsg.sender !== msg.sender;

              return (
                <div key={msg.id} className={`flex flex-col ${msg.isMe ? 'items-end' : 'items-start'}`}>
                  <div className={`max-w-[80%] md:max-w-[70%] group relative px-4 py-3 rounded-2xl text-sm transition-all ${msg.isMe
                    ? 'bg-indigo-600 text-white rounded-br-none shadow-lg shadow-indigo-600/10'
                    : 'bg-slate-800 text-slate-200 rounded-bl-none border border-white/5'
                    }`}>
                    {msg.content}
                    {msg.encrypted && (
                      <HiOutlineLockClosed className={`absolute bottom-1/2 -translate-y-1/2 text-[10px] opacity-0 group-hover:opacity-60 transition-opacity ${msg.isMe ? '-left-6 text-indigo-400' : '-right-6 text-slate-500'
                        }`} />
                    )}
                  </div>
                  {isLastInGroup && (
                    <span className="text-[9px] text-slate-600 font-bold uppercase mt-1 px-1 tracking-tighter">
                      {msg.timestamp.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
                      {msg.isMe && ` • ${msg.status}`}
                    </span>
                  )}
                </div>
              );
            })
          ) : (
            <div className="h-full flex flex-col items-center justify-center opacity-20">
              <HiOutlineChatBubbleLeftRight className="text-6xl mb-4" />
              <p className="text-xs font-bold uppercase tracking-[0.2em]">Secure communication buffer empty</p>
            </div>
          )}
        </div>

        {/* Modern Input Bar */}
        <div className="p-4 md:p-6 bg-slate-950/40 border-t border-white/5">
          <div className="relative flex items-center gap-3">
            <button
              onClick={onEmergency}
              className="w-10 h-10 flex-shrink-0 bg-rose-600/10 text-rose-500 rounded-full border border-rose-500/20 hover:bg-rose-600 hover:text-white transition-all flex items-center justify-center shadow-lg shadow-rose-600/5 group"
              title="SOS Priority"
            >
              <HiOutlineExclamationTriangle className="text-lg group-active:scale-90 transition-transform" />
            </button>

            <div className="flex-1 relative group">
              <input
                type="text"
                value={messageContent}
                onChange={(e) => setMessageContent(e.target.value)}
                onKeyPress={handleKeyPress}
                placeholder={`Message ${threadList.find(t => t.id === activeThread)?.name}...`}
                className="w-full bg-slate-900 border border-white/5 rounded-full px-6 py-3 text-sm text-white focus:outline-none focus:ring-2 focus:ring-indigo-500/30 transition-all placeholder:text-slate-600 pr-12"
              />
              <button
                onClick={handleSend}
                disabled={!messageContent.trim() || !backendConnected}
                className="absolute right-1.5 top-1.5 w-8 h-8 bg-indigo-600 text-white rounded-full flex items-center justify-center disabled:opacity-20 transition-all hover:scale-110 active:scale-90 shadow-lg shadow-indigo-600/30"
              >
                <HiOutlinePaperAirplane className="text-sm rotate-[-45deg] ml-0.5 mt-0.5" />
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default CommunicationPanel;