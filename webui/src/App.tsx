import { useState } from 'react'
import './App.css'

type TabType = 'chat' | 'peers' | 'settings' | 'status'

function App() {
  const [activeTab, setActiveTab] = useState<TabType>('chat')
  const [messages] = useState([
    { id: '1', sender: 'System', content: 'GhostWire initialized. Secure communication ready.', timestamp: new Date().toLocaleTimeString() },
    { id: '2', sender: 'System', content: 'Connecting to backend...', timestamp: new Date().toLocaleTimeString() }
  ])
  const [newMessage, setNewMessage] = useState('')
  const [recipient, setRecipient] = useState('')
  const [peers] = useState([
    { id: 'peer1', name: 'Node-7A3F', status: 'online' as const, lastSeen: '2 min ago' },
    { id: 'peer2', name: 'Node-B2E9', status: 'offline' as const, lastSeen: '15 min ago' }
  ])
  const [settings, setSettings] = useState({ stealth_mode: false })

  const sendMessage = () => {
    if (!newMessage.trim() || !recipient.trim()) {
      alert('Please enter both recipient and message')
      return
    }
    alert(`Message sent to ${recipient}: ${newMessage}`)
    setNewMessage('')
  }

  const renderChat = () => (
    <div className="flex flex-col h-full">
      <div className="p-4 border-b border-cyber-gray">
        <div className="flex space-x-2 items-center">
          <span className="text-cyber-gray">To:</span>
          <input
            type="text"
            value={recipient}
            onChange={(e) => setRecipient(e.target.value)}
            placeholder="Enter peer ID..."
            className="cyber-input flex-1"
          />
        </div>
      </div>
      
      <div className="flex-1 overflow-y-auto p-4 space-y-2">
        {messages.map((msg) => (
          <div key={msg.id} className="cyber-card">
            <div className="flex justify-between items-start">
              <span className="text-cyber-blue font-bold">{msg.sender}</span>
              <span className="text-cyber-gray text-sm">{msg.timestamp}</span>
            </div>
            <p className="mt-1">{msg.content}</p>
          </div>
        ))}
      </div>
      
      <div className="p-4 border-t border-cyber-gray">
        <div className="flex space-x-2">
          <input
            type="text"
            value={newMessage}
            onChange={(e) => setNewMessage(e.target.value)}
            onKeyPress={(e) => e.key === 'Enter' && sendMessage()}
            placeholder="Type your message..."
            className="cyber-input flex-1"
          />
          <button onClick={sendMessage} className="cyber-button">
            SEND
          </button>
        </div>
      </div>
    </div>
  )

  const renderPeers = () => (
    <div className="p-4 space-y-4">
      <div className="flex justify-between items-center">
        <h2 className="text-xl font-bold text-cyber-green cyber-glow">Active Peers</h2>
        <button className="cyber-button text-sm">
          REFRESH
        </button>
      </div>
      
      <div className="space-y-2">
        {peers.map((peer) => (
          <div key={peer.id} className="cyber-card flex justify-between items-center">
            <div>
              <div className="font-bold">{peer.name}</div>
              <div className="text-sm text-cyber-gray">Last seen: {peer.lastSeen}</div>
            </div>
            <div className={`px-2 py-1 text-xs ${peer.status === 'online' ? 'bg-cyber-green text-cyber-black' : 'bg-cyber-red text-white'}`}>
              {peer.status.toUpperCase()}
            </div>
          </div>
        ))}
      </div>
    </div>
  )

  const renderSettings = () => (
    <div className="p-4 space-y-6">
      <h2 className="text-xl font-bold text-cyber-green cyber-glow">Settings</h2>
      
      <div className="cyber-card">
        <h3 className="text-lg font-bold mb-4">Stealth Mode</h3>
        <label className="flex items-center space-x-2">
          <input 
            type="checkbox" 
            checked={settings.stealth_mode}
            onChange={(e) => setSettings({ ...settings, stealth_mode: e.target.checked })}
            className="w-4 h-4 text-cyber-green bg-cyber-dark border-cyber-gray" 
          />
          <span>Enable stealth mode</span>
        </label>
      </div>
    </div>
  )

  const renderStatus = () => (
    <div className="p-4 space-y-4">
      <h2 className="text-xl font-bold text-cyber-green cyber-glow">System Status</h2>
      
      <div className="cyber-card">
        <h3 className="text-lg font-bold mb-4">Connection Status</h3>
        <div className="space-y-2">
          <div className="flex justify-between">
            <span>Backend API:</span>
            <span className="text-cyber-green">Connected</span>
          </div>
          <div className="flex justify-between">
            <span>WebSocket:</span>
            <span className="text-cyber-green">Connected</span>
          </div>
          <div className="flex justify-between">
            <span>Encryption:</span>
            <span className="text-cyber-green">Active</span>
          </div>
        </div>
      </div>
      
      <div className="cyber-card">
        <h3 className="text-lg font-bold mb-4">System Info</h3>
        <div className="space-y-2 text-sm">
          <div>GhostWire v1.0.0</div>
          <div>Node ID: GW-7A3F-B2E9</div>
          <div>Uptime: 2h 15m</div>
          <div>Messages sent: 47</div>
          <div>Messages received: 23</div>
        </div>
      </div>
    </div>
  )

  return (
    <div className="min-h-screen bg-cyber-black text-cyber-green">
      <div className="container mx-auto p-4">
        <header className="text-center mb-8">
          <h1 className="text-4xl font-bold cyber-glow mb-2">🌐 GhostWire</h1>
          <p className="text-cyber-gray">Secure Messaging Network</p>
        </header>

        <div className="bg-cyber-dark border border-cyber-gray min-h-[600px]">
          {/* Tab Navigation */}
          <div className="flex border-b border-cyber-gray">
            {(['chat', 'peers', 'settings', 'status'] as TabType[]).map((tab) => (
              <button
                key={tab}
                onClick={() => setActiveTab(tab)}
                className={`cyber-tab ${activeTab === tab ? 'active' : ''}`}
              >
                {tab.toUpperCase()}
              </button>
            ))}
          </div>

          {/* Tab Content */}
          <div className="h-[500px]">
            {activeTab === 'chat' && renderChat()}
            {activeTab === 'peers' && renderPeers()}
            {activeTab === 'settings' && renderSettings()}
            {activeTab === 'status' && renderStatus()}
          </div>
        </div>
      </div>
    </div>
  )
}

export default App 