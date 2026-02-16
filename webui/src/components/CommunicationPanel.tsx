import { useState, useRef, useEffect } from 'react';
import { HiPaperAirplane, HiHashtag, HiUsers, HiSignal, HiGlobeAlt, HiLockClosed, HiChevronLeft } from 'react-icons/hi2';

// GhostWire Data Structure
const STREAMS_DATA = {
  '1': [ // Local Mesh
    { id: '1', name: 'CORE_UPLINK', type: 'text', status: 'secure' },
    { id: '2', name: 'EMERGENCY_BAND', type: 'text', status: 'critical' },
  ],
  '2': [ // Global Grid
    { id: '3', name: 'INTEL_FEED', type: 'text', status: 'encrypted' },
    { id: '4', name: 'VOICE_OPS_A', type: 'voice', status: 'secure' },
    { id: '5', name: 'SAT_LINK_7', type: 'data', status: 'latency_high' },
  ],
  '3': [ // Dead Drop
    { id: '6', name: 'DARK_POOL_1', type: 'text', status: 'stealth' },
    { id: '7', name: 'DROP_BOX_ALPHA', type: 'file', status: 'offline' },
  ]
};

const ZONES = [
  { id: '1', name: 'Local Mesh', icon: <HiSignal />, color: 'text-cyan-400' },
  { id: '2', name: 'Global Grid', icon: <HiGlobeAlt />, color: 'text-emerald-400' },
  { id: '3', name: 'Dead Drop', icon: <HiLockClosed />, color: 'text-rose-400' },
];

interface CommunicationPanelProps {
  messages: any[];
  onSendMessage: (text: string, recipient: string) => void;
  onEmergency: () => void;
  user: any;
  peers: any[];
  activeRecipient: string;
  onRecipientSelect: (id: string) => void;
}

const CommunicationPanel = ({ messages, onSendMessage, onEmergency, user, peers, activeRecipient, onRecipientSelect }: CommunicationPanelProps) => {
  const [inputText, setInputText] = useState('');
  const [showNodeSidebar, setShowNodeSidebar] = useState(window.innerWidth >= 1024);
  const [showStreamSidebar, setShowStreamSidebar] = useState(window.innerWidth >= 1024);
  const [isMobile, setIsMobile] = useState(window.innerWidth < 1024);
  const [activeZone, setActiveZone] = useState('1');
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleResize = () => {
      const mobile = window.innerWidth < 1024;
      setIsMobile(mobile);
      if (mobile) {
        setShowNodeSidebar(false);
        setShowStreamSidebar(false);
      } else {
        setShowNodeSidebar(true);
        setShowStreamSidebar(true);
      }
    };

    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  }, []);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  const handleSend = () => {
    if (!inputText.trim()) return;
    onSendMessage(inputText, activeRecipient);
    setInputText('');
  };

  const formatTime = (dateInput: Date | string) => {
    try {
      const date = new Date(dateInput);
      return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
    } catch (e) {
      return "Now";
    }
  };

  const currentStreams = STREAMS_DATA[activeZone as keyof typeof STREAMS_DATA] || STREAMS_DATA['1'];

  return (
    <div className="flex h-full w-full bg-slate-900/50 backdrop-blur-sm text-gray-100 font-sans overflow-hidden relative">

      {/* 1. Zone/Frequency Selector (Far Left) - Drawer on Mobile */}
      <div className={`
        ${isMobile ? 'absolute left-0 top-0 bottom-0 z-50 bg-slate-950/90' : 'relative bg-slate-950/40'} 
        w-[72px] border-r border-white/5 flex flex-col items-center py-4 gap-4 shrink-0 backdrop-blur-md transition-transform duration-300
        ${isMobile && !showStreamSidebar ? '-translate-x-full' : 'translate-x-0'}
      `}>
        {ZONES.map(zone => (
          <div
            key={zone.id}
            onClick={() => setActiveZone(zone.id)}
            className="group relative flex items-center justify-center cursor-pointer"
          >
            {/* Active Indicator */}
            {activeZone === zone.id && (
              <div className="absolute -left-0 h-8 w-1 bg-cyan-500 rounded-r-full shadow-[0_0_10px_rgba(6,182,212,0.8)]" />
            )}

            <div className={`w-12 h-12 rounded-xl border border-white/10 flex items-center justify-center text-2xl transition-all duration-300
              ${activeZone === zone.id
                ? 'bg-slate-800/80 shadow-[0_0_15px_rgba(0,0,0,0.5)] border-cyan-500/30'
                : 'bg-transparent hover:bg-slate-800/50 hover:border-white/20'}`}>
              <span className={`${activeZone === zone.id ? zone.color : 'text-slate-400 group-hover:text-white'}`}>
                {zone.icon}
              </span>
            </div>

            {/* Tooltip */}
            {!isMobile && (
              <div className="absolute left-14 px-2 py-1 bg-black/90 border border-white/10 rounded text-xs opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none whitespace-nowrap z-50">
                {zone.name}
              </div>
            )}
          </div>
        ))}

        {/* Mobile toggle close */}
        {isMobile && (
          <div className="mt-auto mb-4">
            <button
              onClick={() => setShowStreamSidebar(false)}
              className="p-2 text-slate-500 hover:text-white"
            >
              <HiChevronLeft className="w-6 h-6" />
            </button>
          </div>
        )}
      </div>

      {/* 2. Streams Sidebar (Collapsible) - Drawer on Mobile */}
      <div className={`
          ${isMobile ? 'absolute left-[72px] top-0 bottom-0 z-40 shadow-2xl' : 'relative'}
          ${showStreamSidebar ? 'w-64 opacity-100' : 'w-0 opacity-0 overflow-hidden'} 
          bg-slate-900/90 border-r border-white/5 flex flex-col shrink-0 transition-all duration-300 backdrop-blur-xl
        `}>
        {/* Header */}
        <div className="h-16 border-b border-white/5 flex items-center px-6 font-bold tracking-wider text-sm text-cyan-500 shadow-sm bg-slate-900/20 whitespace-nowrap overflow-hidden">
          SECURE_MESH
        </div>

        {/* Stream List */}
        <div className="flex-1 overflow-y-auto p-3 space-y-1 scrollbar-hide">
          <div className="text-[10px] font-bold text-slate-500 uppercase tracking-widest mb-3 px-2 mt-2 truncate">
            {ZONES.find(z => z.id === activeZone)?.name || 'Streams'}
          </div>
          {currentStreams.map(stream => (
            <div
              key={stream.id}
              onClick={() => {
                onRecipientSelect(stream.name === 'CORE_UPLINK' ? 'broadcast' : stream.name);
                if (isMobile) setShowStreamSidebar(false);
              }}
              className={`flex items-center px-3 py-2 rounded-lg cursor-pointer group transition-all duration-200 border border-transparent
                ${(activeRecipient === 'broadcast' && stream.name === 'CORE_UPLINK') || activeRecipient === stream.name
                  ? 'bg-cyan-900/20 border-cyan-500/20 text-cyan-100 shadow-[0_0_10px_rgba(6,182,212,0.1)]'
                  : 'text-slate-400 hover:bg-white/5 hover:text-white'}`}
            >
              <HiHashtag className={`w-4 h-4 mr-2 shrink-0 ${(activeRecipient === 'broadcast' && stream.name === 'CORE_UPLINK') || activeRecipient === stream.name ? 'text-cyan-400' : 'text-slate-600'}`} />
              <div className="flex-1 flex flex-col overflow-hidden">
                <span className="font-medium text-sm truncate">{stream.name}</span>
                {((activeRecipient === 'broadcast' && stream.name === 'CORE_UPLINK') || activeRecipient === stream.name) && <span className="text-[9px] text-cyan-600 font-mono tracking-tight uppercase truncate">Encrypt: AES-256</span>}
              </div>
              {stream.status === 'critical' && <div className="w-1.5 h-1.5 rounded-full bg-red-500 animate-pulse shadow-[0_0_5px_rgba(239,68,68,0.8)] shrink-0" />}
            </div>
          ))}
        </div>

        {/* Connection Status */}
        <div className="p-4 bg-black/20 border-t border-white/5">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <div className="w-2 h-2 rounded-full bg-emerald-500 shadow-[0_0_8px_rgba(16,185,129,0.8)]" />
              <span className="text-xs text-emerald-500 font-mono truncate">UPLINK_STABLE</span>
            </div>
            <span className="text-[10px] text-slate-500 font-mono">24ms</span>
          </div>
        </div>
      </div>

      {/* 3. Terminal/Chat Area */}
      <div className="flex-1 flex flex-col bg-transparent min-w-0 relative h-full">
        {/* Background Grid Effect */}
        <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-5 pointer-events-none" />

        {/* Header */}
        <div className="h-16 border-b border-white/5 flex items-center justify-between px-4 md:px-6 bg-slate-900/20 backdrop-blur-sm z-10 shrink-0">
          <div className="flex items-center space-x-3 overflow-hidden">
            {/* Mobile: Toggle Streams */}
            <button
              onClick={() => setShowStreamSidebar(!showStreamSidebar)}
              className={`md:hidden p-2 -ml-2 rounded-lg text-slate-400 hover:text-white ${showStreamSidebar ? 'text-cyan-400' : ''}`}
            >
              <HiHashtag className="w-5 h-5" />
            </button>

            <HiHashtag className="hidden md:block w-5 h-5 text-cyan-500" />
            <div className="flex flex-col overflow-hidden">
              <span className="text-white font-bold text-sm tracking-wide truncate">{activeRecipient === 'broadcast' ? 'CORE_UPLINK' : `TARGET: ${activeRecipient}`}</span>
              <span className="text-[10px] text-slate-400 font-mono truncate">E2EE • <span className="text-emerald-400">VERIFIED</span></span>
            </div>
          </div>
          <div className="flex items-center space-x-2 md:space-x-4 shrink-0">
            <button
              onClick={() => setShowNodeSidebar(!showNodeSidebar)}
              className={`p-2 rounded-lg transition-colors ${showNodeSidebar ? 'text-cyan-400 bg-cyan-900/20' : 'text-slate-400 hover:text-white'}`}
              title="Toggle Nodes"
            >
              <HiUsers className="w-5 h-5" />
            </button>
          </div>
        </div>

        {/* Messages */}
        <div id="chat-interface" className="flex-1 overflow-y-auto p-4 md:p-6 space-y-6 custom-scrollbar relative z-0">
          {messages.length === 0 ? (
            <div className="flex flex-col items-center justify-center h-full text-slate-500">
              <div className="w-20 h-20 rounded-full border border-dashed border-slate-700 flex items-center justify-center mb-4">
                <HiSignal className="w-8 h-8 opacity-20" />
              </div>
              <p className="font-mono text-sm">No encrypted packets in stream.</p>
            </div>
          ) : (
            messages.map((msg, idx) => {
              const prevMsg = messages[idx - 1];
              const isMe = msg.sender === user.id;
              const isSequence = prevMsg && (prevMsg.sender === msg.sender);

              return (
                <div key={msg.id || idx} className={`flex ${isSequence ? 'mt-1' : 'mt-6'} ${isMe ? 'flex-row-reverse' : ''} group`}>

                  {!isSequence && (
                    <div className={`w-8 h-8 rounded shrink-0 flex items-center justify-center text-sm border font-mono shadow-lg
                        ${isMe ? 'bg-cyan-900/20 border-cyan-500/30 text-cyan-400 ml-2 md:ml-4' : 'bg-slate-800 border-white/10 text-slate-300 mr-2 md:mr-4'}`}>
                      {isMe ? 'ME' : 'ID'}
                    </div>
                  )}
                  {isSequence && <div className="w-10 md:w-12 ml-2 md:ml-4 mr-2 md:mr-4" />} {/* Spacer for sequence */}

                  <div className={`flex flex-col max-w-[75%] md:max-w-[70%] ${isMe ? 'items-end' : 'items-start'}`}>
                    {!isSequence && (
                      <div className="flex items-baseline mb-1 space-x-2">
                        <span className={`text-xs font-bold tracking-wide ${isMe ? 'text-cyan-400' : 'text-slate-200'}`}>
                          {isMe ? user.name : (msg.senderName || msg.sender)}
                        </span>
                        <span className="text-[9px] text-slate-500 font-mono">{formatTime(msg.timestamp)}</span>
                      </div>
                    )}

                    <div className={`relative px-3 py-2 md:px-4 md:py-2 rounded-sm text-sm leading-relaxed border shadow-sm break-words
                        ${isMe
                        ? 'bg-cyan-500/10 border-cyan-500/20 text-cyan-50 rounded-tr-none'
                        : 'bg-slate-800/50 border-white/5 text-slate-200 rounded-tl-none'}`}>
                      {/* Decorative corner accent */}
                      <div className={`absolute top-0 w-2 h-2 border-t border-l 
                            ${isMe ? 'right-0 border-cyan-500/40 rounded-tl' : 'left-0 border-white/20 rounded-br'}`} />

                      {msg.content}
                    </div>
                  </div>
                </div>
              );
            })
          )}
          <div ref={messagesEndRef} />
        </div>

        {/* Input Area */}
        <div className="p-3 md:p-4 shrink-0 bg-slate-900/40 backdrop-blur-md border-t border-white/5 z-20 pb-safe">
          <div className="relative flex items-center bg-black/30 border border-white/10 rounded-sm focus-within:border-cyan-500/50 focus-within:ring-1 focus-within:ring-cyan-500/20 transition-all">
            <input
              type="text"
              value={inputText}
              onChange={(e) => setInputText(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && handleSend()}
              placeholder={`Transmit...`}
              className="flex-1 bg-transparent border-none text-white px-3 py-3 md:px-4 text-sm focus:outline-none placeholder-slate-600 font-mono min-w-0"
            />

            <div className="flex items-center px-1 md:px-2 space-x-1 border-l border-white/5 bg-white/5 h-full">
              {/* SOS Button Inside */}
              <button
                onClick={onEmergency}
                className="p-2 text-red-500/70 hover:text-red-500 hover:bg-red-500/10 rounded transition-colors"
                title="EMERGENCY BROADCAST"
              >
                <div className="animate-pulse">⚠️</div>
              </button>

              <button className="p-2 text-slate-400 hover:text-cyan-400 transition-colors" onClick={handleSend}>
                <HiPaperAirplane className="w-4 h-4 -rotate-45" />
              </button>
            </div>
          </div>
          <div className="mt-2 flex justify-between text-[9px] text-slate-500 font-mono hidden md:flex">
            <span>ENCRYPTION: AES-256-GCM [ACTIVE]</span>
            <span>LATENCY: 12ms</span>
          </div>
        </div>
      </div>

      {/* 4. Connected Nodes (Right Sidebar) - Drawer on Mobile */}
      {/* Overlay backdrop for mobile */}
      {isMobile && showNodeSidebar && (
        <div className="absolute inset-0 bg-black/50 z-40 backdrop-blur-sm" onClick={() => setShowNodeSidebar(false)} />
      )}

      <div className={`
          ${isMobile ? 'absolute right-0 top-0 bottom-0 z-50 shadow-2xl h-full' : 'relative'}
          ${showNodeSidebar ? 'w-64 opacity-100 border-l' : 'w-0 opacity-0 overflow-hidden border-none'} 
          bg-slate-900/95 border-white/5 flex flex-col shrink-0 transition-all duration-300 backdrop-blur-xl
        `}>
        <div className="h-16 flex items-center px-4 font-bold text-xs text-slate-400 uppercase tracking-widest border-b border-white/5 bg-slate-900/20 whitespace-nowrap">
          <span className="truncate">Network Nodes</span>
          <span className="ml-auto bg-slate-800 px-1.5 py-0.5 rounded text-white font-mono shrink-0">{(peers?.length || 0) + 1}</span>
          {isMobile && (
            <button onClick={() => setShowNodeSidebar(false)} className="ml-2 text-slate-500">
              <HiChevronLeft className="w-5 h-5 rotate-180" />
            </button>
          )}
        </div>
        <div className="p-3 space-y-2 overflow-y-auto flex-1">
          {/* Me */}
          <div className="flex items-center p-2 rounded-lg bg-cyan-900/10 border border-cyan-500/20 shadow-sm relative overflow-hidden group">
            <div className="absolute inset-0 bg-gradient-to-r from-cyan-500/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity" />
            <div className="relative mr-3 w-8 h-8 rounded bg-cyan-900/30 flex items-center justify-center text-cyan-400 font-mono text-xs border border-cyan-500/30 shrink-0">
              ME
            </div>
            <div className="overflow-hidden">
              <div className="font-bold text-sm text-cyan-100 truncate">{user.name}</div>
              <div className="text-[10px] text-cyan-500/70 font-mono truncate">LOCAL_OPERATOR</div>
            </div>
            <div className="ml-auto w-1.5 h-1.5 rounded-full bg-cyan-400 shadow-[0_0_5px_rgba(34,211,238,0.8)] shrink-0" />
          </div>

          {/* Peers */}
          {peers?.map((peer: any) => (
            <div
              key={peer.node_id || peer.id}
              onClick={() => {
                onRecipientSelect(peer.node_id);
                if (isMobile) setShowNodeSidebar(false);
              }}
              className={`flex items-center p-2 rounded-lg border transition-all cursor-pointer
                ${activeRecipient === peer.node_id
                  ? 'bg-cyan-900/30 border-cyan-500/40 shadow-md'
                  : 'bg-slate-800/30 border-white/5 hover:border-white/10 hover:bg-slate-800/50'}`}
            >
              <div className="relative mr-3 w-8 h-8 rounded bg-slate-700/30 flex items-center justify-center text-slate-400 font-mono text-xs border border-white/10 shrink-0">
                ID
              </div>
              <div className="overflow-hidden flex-1 min-w-0">
                <div className="font-medium text-sm text-slate-300 truncate">{peer.username || peer.node_id?.substring(0, 8)}</div>
                <div className="text-[10px] text-slate-500 font-mono truncate">REMOTE_NODE</div>
              </div>
              <div className={`ml-auto w-1.5 h-1.5 rounded-full shrink-0 ${peer.status === 'online' ? 'bg-emerald-500 shadow-[0_0_5px_rgba(16,185,129,0.5)]' : 'bg-slate-600'}`} />
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default CommunicationPanel;