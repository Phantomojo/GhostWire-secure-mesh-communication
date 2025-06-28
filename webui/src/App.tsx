import { useState, useEffect, useRef } from 'react'
import './App.css'
import { api, WebSocketService, Message, Peer, Settings } from './services/api'

type TabType = 'chat' | 'peers' | 'settings' | 'status'

function App() {
  const [activeTab, setActiveTab] = useState<TabType>('chat')
  const [messages, setMessages] = useState<Message[]>([
    { id: '1', sender: 'System', content: 'GhostWire initialized. Secure communication ready.', timestamp: new Date().toLocaleTimeString() },
    { id: '2', sender: 'System', content: 'Connecting to backend...', timestamp: new Date().toLocaleTimeString() }
  ])
  const [newMessage, setNewMessage] = useState('')
  const [recipient, setRecipient] = useState('')
  const [peers, setPeers] = useState<Peer[]>([])
  const [settings, setSettings] = useState<Settings>({ stealth_mode: false })
  const [connectionStatus, setConnectionStatus] = useState('Connecting...')
  const [logs, setLogs] = useState<string[]>([])
  const wsServiceRef = useRef<WebSocketService | null>(null)

  // Initialize WebSocket connection
  useEffect(() => {
    const wsService = new WebSocketService(
      (message) => {
        setMessages(prev => [...prev, message])
        addLog(`[INFO] Received message from ${message.sender}`)
      },
      (updatedPeers) => {
        setPeers(updatedPeers)
        addLog(`[INFO] Peer list updated: ${updatedPeers.length} peers`)
      },
      (status) => {
        setConnectionStatus(status.status || 'Connected')
        addLog(`[INFO] Status update: ${status.status}`)
      },
      (error) => {
        setConnectionStatus('Error')
        addLog(`[ERROR] ${error}`)
      }
    )

    wsServiceRef.current = wsService
    wsService.connect()

    // Load initial data
    loadPeers()
    loadSettings()

    return () => {
      wsService.disconnect()
    }
  }, [])

  const addLog = (log: string) => {
    const timestamp = new Date().toLocaleTimeString()
    setLogs(prev => [...prev.slice(-50), `[${timestamp}] ${log}`])
  }

  const loadPeers = async () => {
    try {
      const peerList = await api.getPeers()
      setPeers(peerList)
      addLog(`[INFO] Loaded ${peerList.length} peers`)
    } catch (error) {
      addLog(`[ERROR] Failed to load peers: ${error}`)
    }
  }

  const loadSettings = async () => {
    try {
      const currentSettings = await api.getSettings()
      setSettings(currentSettings)
      addLog('[INFO] Settings loaded')
    } catch (error) {
      addLog(`[ERROR] Failed to load settings: ${error}`)
    }
  }

  const sendMessage = async () => {
    if (!newMessage.trim() || !recipient.trim()) {
      addLog('[WARN] Please enter both recipient and message')
      return
    }

    try {
      await api.sendMessage(recipient, newMessage)
      const message: Message = {
        id: Date.now().toString(),
        sender: 'You',
        content: newMessage,
        timestamp: new Date().toLocaleTimeString()
      }
      setMessages(prev => [...prev, message])
      setNewMessage('')
      addLog(`[INFO] Message sent to ${recipient}`)
    } catch (error) {
      addLog(`[ERROR] Failed to send message: ${error}`)
    }
  }

  const updateStealthMode = async (enabled: boolean) => {
    try {
      const updatedSettings = await api.updateSettings({ stealth_mode: enabled })
      setSettings(updatedSettings)
      addLog(`[INFO] Stealth mode ${enabled ? 'enabled' : 'disabled'}`)
    } catch (error) {
      addLog(`[ERROR] Failed to update stealth mode: ${error}`)
    }
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
        <button onClick={loadPeers} className="cyber-button text-sm">
          REFRESH
        </button>
      </div>
      
      <div className="space-y-2">
        {peers.length === 0 ? (
          <div className="cyber-card text-center text-cyber-gray">
            No peers discovered yet...
          </div>
        ) : (
          peers.map((peer) => (
            <div key={peer.id} className="cyber-card flex justify-between items-center">
              <div>
                <div className="font-bold">{peer.name}</div>
                <div className="text-sm text-cyber-gray">Last seen: {peer.lastSeen}</div>
              </div>
              <div className={`px-2 py-1 text-xs ${peer.status === 'online' ? 'bg-cyber-green text-cyber-black' : 'bg-cyber-red text-white'}`}>
                {peer.status.toUpperCase()}
              </div>
            </div>
          ))
        )}
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
            onChange={(e) => updateStealthMode(e.target.checked)}
            className="w-4 h-4 text-cyber-green bg-cyber-dark border-cyber-gray" 
          />
          <span>Enable stealth mode</span>
        </label>
      </div>

      <div className="cyber-card">
        <h3 className="text-lg font-bold mb-4">Connection Status</h3>
        <div className="space-y-2 text-sm">
          <div className="flex justify-between">
            <span>Status:</span>
            <span className={`${connectionStatus === 'Connected' ? 'text-cyber-green' : 'text-cyber-red'}`}>
              {connectionStatus.toUpperCase()}
            </span>
          </div>
          <div className="flex justify-between">
            <span>Backend:</span>
            <span className="text-cyber-green">RUNNING</span>
          </div>
        </div>
      </div>
    </div>
  )

  const renderStatus = () => (
    <div className="p-4 space-y-4">
      <h2 className="text-xl font-bold text-cyber-green cyber-glow">System Status</h2>
      
      <div className="cyber-card">
        <h3 className="text-lg font-bold mb-4">Network Status</h3>
        <div className="space-y-2 text-sm">
          <div className="flex justify-between">
            <span>Connection:</span>
            <span className={`${connectionStatus === 'Connected' ? 'text-cyber-green' : 'text-cyber-red'}`}>
              {connectionStatus.toUpperCase()}
            </span>
          </div>
          <div className="flex justify-between">
            <span>Peers Found:</span>
            <span className="text-cyber-blue">{peers.length}</span>
          </div>
          <div className="flex justify-between">
            <span>Messages Sent:</span>
            <span className="text-cyber-green">{messages.filter(m => m.sender === 'You').length}</span>
          </div>
        </div>
      </div>

      <div className="cyber-card">
        <h3 className="text-lg font-bold mb-4">Recent Logs</h3>
        <div className="space-y-1 text-xs font-mono max-h-64 overflow-y-auto">
          {logs.length === 0 ? (
            <div className="text-cyber-gray">No logs yet...</div>
          ) : (
            logs.map((log, index) => (
              <div key={index} className={
                log.includes('[ERROR]') ? 'text-cyber-red' :
                log.includes('[WARN]') ? 'text-cyber-yellow' :
                log.includes('[INFO]') ? 'text-cyber-green' :
                'text-cyber-gray'
              }>
                {log}
              </div>
            ))
          )}
        </div>
      </div>
    </div>
  )

  return (
    <div className="flex h-screen bg-cyber-black">
      {/* Sidebar */}
      <div className="cyber-sidebar w-64 flex flex-col">
        <div className="p-4 border-b border-cyber-gray">
          <h1 className="text-xl font-bold text-cyber-green cyber-glow">GHOSTWIRE</h1>
          <p className="text-xs text-cyber-gray mt-1">Secure Communication Network</p>
        </div>
        
        <nav className="flex-1 p-4 space-y-2">
          <button
            onClick={() => setActiveTab('chat')}
            className={`w-full text-left p-3 rounded-none transition-all ${
              activeTab === 'chat' 
                ? 'bg-cyber-green text-cyber-black' 
                : 'text-cyber-green hover:bg-cyber-gray'
            }`}
          >
            💬 Chat
          </button>
          <button
            onClick={() => setActiveTab('peers')}
            className={`w-full text-left p-3 rounded-none transition-all ${
              activeTab === 'peers' 
                ? 'bg-cyber-green text-cyber-black' 
                : 'text-cyber-green hover:bg-cyber-gray'
            }`}
          >
            🌐 Peers
          </button>
          <button
            onClick={() => setActiveTab('settings')}
            className={`w-full text-left p-3 rounded-none transition-all ${
              activeTab === 'settings' 
                ? 'bg-cyber-green text-cyber-black' 
                : 'text-cyber-green hover:bg-cyber-gray'
            }`}
          >
            ⚙️ Settings
          </button>
          <button
            onClick={() => setActiveTab('status')}
            className={`w-full text-left p-3 rounded-none transition-all ${
              activeTab === 'status' 
                ? 'bg-cyber-green text-cyber-black' 
                : 'text-cyber-green hover:bg-cyber-gray'
            }`}
          >
            📊 Status
          </button>
        </nav>
        
        <div className="p-4 border-t border-cyber-gray">
          <div className="text-xs text-cyber-gray">
            <div>Status: {connectionStatus.toUpperCase()}</div>
            <div>Version: 0.1.0</div>
          </div>
        </div>
      </div>

      {/* Main Content */}
      <div className="flex-1 flex flex-col">
        <div className="flex-1 overflow-hidden">
          {activeTab === 'chat' && renderChat()}
          {activeTab === 'peers' && renderPeers()}
          {activeTab === 'settings' && renderSettings()}
          {activeTab === 'status' && renderStatus()}
        </div>
      </div>
    </div>
  )
}

export default App 