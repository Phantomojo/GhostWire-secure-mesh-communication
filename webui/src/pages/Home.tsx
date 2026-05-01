
import { useState, useEffect } from 'react';
import HeroSection from '../components/HeroSection';

interface Message {
  id: number;
  content: string;
  sender: string;
  timestamp: string;
  isOwn: boolean;
}

const Home = () => {
  const [networkStatus, setNetworkStatus] = useState({
    connected: false,
    peers: 0,
    messages: 0,
    uptime: 0
  });
  const [messages, setMessages] = useState<Message[]>([]);
  const [newMessage, setNewMessage] = useState('');

  // Simulate network status updates
  useEffect(() => {
    const interval = setInterval(() => {
      setNetworkStatus(prev => ({
        ...prev,
        uptime: prev.uptime + 1,
        peers: Math.floor(Math.random() * 10),
        messages: prev.messages + Math.floor(Math.random() * 3)
      }));
    }, 1000);

    return () => clearInterval(interval);
  }, []);

  const sendMessage = () => {
    if (newMessage.trim()) {
      setMessages(prev => [...prev, {
        id: Date.now(),
        content: newMessage,
        sender: 'You',
        timestamp: new Date().toLocaleTimeString(),
        isOwn: true
      }]);
      setNewMessage('');
    }
  };

  return (
    <div className="relative min-h-screen bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900 overflow-hidden">
      <HeroSection />
      
      {/* Dashboard Section */}
      <div className="container mx-auto px-6 py-12">
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          
          {/* Network Status Card */}
          <div className="bg-white/10 backdrop-blur-lg rounded-xl p-6 border border-white/20">
            <h2 className="text-2xl font-bold text-white mb-4">🌐 Network Status</h2>
            <div className="space-y-3">
              <div className="flex justify-between">
                <span className="text-gray-300">Status:</span>
                <span className={`font-semibold ${networkStatus.connected ? 'text-green-400' : 'text-red-400'}`}>
                  {networkStatus.connected ? '🟢 Connected' : '🔴 Disconnected'}
                </span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-300">Peers:</span>
                <span className="text-white font-semibold">{networkStatus.peers}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-300">Messages:</span>
                <span className="text-white font-semibold">{networkStatus.messages}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-300">Uptime:</span>
                <span className="text-white font-semibold">{networkStatus.uptime}s</span>
              </div>
            </div>
            <button className="w-full mt-4 bg-indigo-600 hover:bg-indigo-700 text-white font-semibold py-2 px-4 rounded-lg transition">
              Connect to Network
            </button>
          </div>

          {/* Messaging Card */}
          <div className="bg-white/10 backdrop-blur-lg rounded-xl p-6 border border-white/20">
            <h2 className="text-2xl font-bold text-white mb-4">💬 Messaging</h2>
            <div className="h-64 overflow-y-auto mb-4 space-y-2">
              {messages.length === 0 ? (
                <p className="text-gray-400 text-center py-8">No messages yet. Start a conversation!</p>
              ) : (
                messages.map(msg => (
                  <div key={msg.id} className={`p-3 rounded-lg ${msg.isOwn ? 'bg-indigo-600 ml-8' : 'bg-gray-700 mr-8'}`}>
                    <div className="flex justify-between items-start mb-1">
                      <span className="font-semibold text-white">{msg.sender}</span>
                      <span className="text-xs text-gray-300">{msg.timestamp}</span>
                    </div>
                    <p className="text-white">{msg.content}</p>
                  </div>
                ))
              )}
            </div>
            <div className="flex gap-2">
              <input
                type="text"
                value={newMessage}
                onChange={(e) => setNewMessage(e.target.value)}
                onKeyPress={(e) => e.key === 'Enter' && sendMessage()}
                placeholder="Type your message..."
                className="flex-1 bg-gray-800 text-white px-3 py-2 rounded-lg border border-gray-600 focus:border-indigo-500 focus:outline-none"
              />
              <button
                onClick={sendMessage}
                className="bg-indigo-600 hover:bg-indigo-700 text-white px-4 py-2 rounded-lg transition"
              >
                Send
              </button>
            </div>
          </div>

          {/* Quick Actions Card */}
          <div className="bg-white/10 backdrop-blur-lg rounded-xl p-6 border border-white/20">
            <h2 className="text-2xl font-bold text-white mb-4">⚡ Quick Actions</h2>
            <div className="space-y-3">
              <button className="w-full bg-green-600 hover:bg-green-700 text-white font-semibold py-3 px-4 rounded-lg transition">
                🔍 Scan for Peers
              </button>
              <button className="w-full bg-blue-600 hover:bg-blue-700 text-white font-semibold py-3 px-4 rounded-lg transition">
                📡 Broadcast Message
              </button>
              <button className="w-full bg-purple-600 hover:bg-purple-700 text-white font-semibold py-3 px-4 rounded-lg transition">
                🔐 Generate Key Pair
              </button>
              <button className="w-full bg-orange-600 hover:bg-orange-700 text-white font-semibold py-3 px-4 rounded-lg transition">
                📊 View Statistics
              </button>
            </div>
          </div>
        </div>

        {/* System Info */}
        <div className="mt-8 bg-white/10 backdrop-blur-lg rounded-xl p-6 border border-white/20">
          <h2 className="text-2xl font-bold text-white mb-4">🔧 System Information</h2>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            <div className="text-center">
              <div className="text-3xl font-bold text-indigo-400">Rust</div>
              <div className="text-gray-300">Backend</div>
            </div>
            <div className="text-center">
              <div className="text-3xl font-bold text-blue-400">React</div>
              <div className="text-gray-300">Frontend</div>
            </div>
            <div className="text-center">
              <div className="text-3xl font-bold text-green-400">libp2p</div>
              <div className="text-gray-300">Networking</div>
            </div>
            <div className="text-center">
              <div className="text-3xl font-bold text-purple-400">AES-256</div>
              <div className="text-gray-300">Encryption</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Home; 