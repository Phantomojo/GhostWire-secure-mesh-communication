import { useState, useEffect, useRef } from 'react'
import './App.css'
import { api, WebSocketService } from './services/api'
import type { Message, Peer, Settings, DiscoveredPeer, NetworkInfo } from './services/api'

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
  const [discoveredPeers, setDiscoveredPeers] = useState<DiscoveredPeer[]>([])
  const [currentUsername, setCurrentUsername] = useState('GhostUser')
  const [isScanning, setIsScanning] = useState(false)
  const [networkInfo, setNetworkInfo] = useState<NetworkInfo | null>(null)
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
        
        // Update the "Connecting to backend..." message when connected
        if (status.status === 'Connected') {
          setMessages(prev => prev.map(msg => 
            msg.content === 'Connecting to backend...' 
              ? { ...msg, content: 'Connected to backend successfully!', timestamp: new Date().toLocaleTimeString() }
              : msg
          ))
        }
      },
      (error) => {
        setConnectionStatus('Error')
        addLog(`[ERROR] ${error}`)
        
        // Update the connection message on error
        setMessages(prev => prev.map(msg => 
          msg.content === 'Connecting to backend...' 
            ? { ...msg, content: `Connection failed: ${error}`, timestamp: new Date().toLocaleTimeString() }
            : msg
        ))
      }
    )

    wsServiceRef.current = wsService
    wsService.connect()

    // Load initial data
    loadPeers()
    loadSettings()
    loadUsername()
    loadNetworkInfo()
    testBackendConnection()

    return () => {
      wsService.disconnect()
    }
  }, [])

  const addLog = (log: string) => {
    const timestamp = new Date().toLocaleTimeString()
    setLogs(prev => [...prev.slice(-50), `[${timestamp}] ${log}`])
  }

  const testBackendConnection = async () => {
    try {
      const response = await fetch('http://localhost:3001/api/status')
      if (response.ok) {
        const data = await response.json()
        if (data.success) {
          addLog('[INFO] Backend API is reachable')
          setConnectionStatus('Connected')
          // Update the connection message
          setMessages(prev => prev.map(msg => 
            msg.content === 'Connecting to backend...' 
              ? { ...msg, content: 'Connected to backend successfully!', timestamp: new Date().toLocaleTimeString() }
              : msg
          ))
        } else {
          throw new Error(data.error || 'Unknown error')
        }
      } else {
        throw new Error(`HTTP ${response.status}`)
      }
    } catch (error) {
      addLog(`[ERROR] Backend connection test failed: ${error}`)
      setConnectionStatus('Error')
      setMessages(prev => prev.map(msg => 
        msg.content === 'Connecting to backend...' 
          ? { ...msg, content: `Connection failed: ${error}`, timestamp: new Date().toLocaleTimeString() }
          : msg
      ))
    }
  }

  const loadPeers = async () => {
    try {
      const peerList = await api.getPeers()
      setPeers(peerList)
      addLog(`[INFO] Loaded ${peerList.length} peers`)
    } catch (error) {
      addLog(`[ERROR] Failed to load peers: ${error}`)
      // Fallback to mock data if API fails
      setPeers([
        { id: 'peer1', name: 'Node-7A3F', status: 'online', lastSeen: '2 min ago' },
        { id: 'peer2', name: 'Node-B2E9', status: 'offline', lastSeen: '15 min ago' }
      ])
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

  const loadUsername = async () => {
    try {
      const username = await api.getUsername()
      setCurrentUsername(username)
      addLog(`[INFO] Username loaded: ${username}`)
    } catch (error) {
      addLog(`[ERROR] Failed to load username: ${error}`)
    }
  }

  const loadNetworkInfo = async () => {
    try {
      const info = await api.getNetworkInfo()
      setNetworkInfo(info)
      addLog(`[INFO] Network info loaded: ${info.local_ip}`)
    } catch (error) {
      addLog(`[ERROR] Failed to load network info: ${error}`)
    }
  }

  const updateUsername = async (newUsername: string) => {
    try {
      await api.setUsername(newUsername)
      setCurrentUsername(newUsername)
      addLog(`[INFO] Username updated to: ${newUsername}`)
    } catch (error) {
      addLog(`[ERROR] Failed to update username: ${error}`)
    }
  }

  const scanForPeers = async () => {
    setIsScanning(true)
    addLog('[INFO] Scanning network for GhostWire nodes...')
    
    try {
      const scanResult = await api.scanNetwork()
      setDiscoveredPeers(scanResult.discovered_peers)
      addLog(`[INFO] Found ${scanResult.discovered_peers.length} nodes on network`)
      
      // Auto-connect to discovered peers
      for (const peer of scanResult.discovered_peers) {
        try {
          await api.registerWithPeer(
            `${peer.ip}:${peer.port}`,
            peer.node_id,
            peer.username,
            peer.public_key
          )
          addLog(`[INFO] Connected to ${peer.username} (${peer.ip}:${peer.port})`)
        } catch (error) {
          addLog(`[WARN] Failed to connect to ${peer.username}: ${error}`)
        }
      }
      
      // Refresh peer list
      await loadPeers()
    } catch (error) {
      addLog(`[ERROR] Network scan failed: ${error}`)
    } finally {
      setIsScanning(false)
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
      // Add message locally even if API fails
      const message: Message = {
        id: Date.now().toString(),
        sender: 'You',
        content: newMessage,
        timestamp: new Date().toLocaleTimeString()
      }
      setMessages(prev => [...prev, message])
      setNewMessage('')
    }
  }

  const updateStealthMode = async (enabled: boolean) => {
    try {
      const updatedSettings = await api.updateSettings({ stealth_mode: enabled })
      setSettings(updatedSettings)
      addLog(`[INFO] Stealth mode ${enabled ? 'enabled' : 'disabled'}`)
    } catch (error) {
      addLog(`[ERROR] Failed to update stealth mode: ${error}`)
      // Update locally even if API fails
      setSettings({ ...settings, stealth_mode: enabled })
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
      {/* Username Section */}
      <div className="cyber-card">
        <h3 className="text-lg font-bold mb-4">👤 Your Identity</h3>
        <div className="flex space-x-2 items-center">
          <input
            type="text"
            value={currentUsername}
            onChange={(e) => setCurrentUsername(e.target.value)}
            placeholder="Enter your username..."
            className="cyber-input flex-1"
          />
          <button 
            onClick={() => updateUsername(currentUsername)}
            className="cyber-button"
          >
            SAVE
          </button>
        </div>
        <p className="text-sm text-cyber-gray mt-2">
          This username will be visible to other GhostWire users on the network.
        </p>
        
        {/* Network Info */}
        {networkInfo && (
          <div className="mt-3 p-2 bg-cyber-dark rounded border border-cyber-gray">
            <div className="text-sm">
              <span className="text-cyber-blue">🌐 Your IP:</span> 
              <span className="text-cyber-green ml-2">{networkInfo.local_ip}:3001</span>
            </div>
          </div>
        )}
      </div>

      {/* Network Scanner */}
      <div className="cyber-card">
        <div className="flex justify-between items-center mb-4">
          <h3 className="text-lg font-bold">🔍 Network Scanner</h3>
          <button 
            onClick={scanForPeers}
            disabled={isScanning}
            className={`cyber-button ${isScanning ? 'opacity-50' : ''}`}
          >
            {isScanning ? 'SCANNING...' : 'SCAN NETWORK'}
          </button>
        </div>
        
        {discoveredPeers.length > 0 && (
          <div className="space-y-2">
            <h4 className="font-bold text-cyber-blue">Discovered Nodes:</h4>
            {discoveredPeers.map((peer, index) => (
              <div key={index} className="bg-cyber-dark p-3 rounded border border-cyber-gray">
                <div className="flex justify-between items-center">
                  <div>
                    <div className="font-bold text-cyber-green">{peer.username}</div>
                    <div className="text-sm text-cyber-gray">
                      {peer.ip}:{peer.port} • {peer.last_seen}
                    </div>
                  </div>
                  <div className={`px-2 py-1 text-xs ${
                    peer.status === 'online' ? 'bg-cyber-green text-cyber-black' : 'bg-cyber-red text-white'
                  }`}>
                    {peer.status.toUpperCase()}
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
        
        <p className="text-sm text-cyber-gray mt-2">
          Automatically discovers other GhostWire nodes on your local network.
        </p>
      </div>

      {/* Connected Peers */}
      <div className="cyber-card">
        <div className="flex justify-between items-center mb-4">
          <h3 className="text-lg font-bold">📡 Connected Peers</h3>
          <button onClick={loadPeers} className="cyber-button text-sm">
            REFRESH
          </button>
        </div>
        
        <div className="space-y-2">
          {peers.length === 0 ? (
            <div className="text-center text-cyber-gray py-8">
              <div className="text-4xl mb-2">🌐</div>
              <div>No peers connected yet</div>
              <div className="text-sm mt-1">Use the network scanner above to find other GhostWire users</div>
            </div>
          ) : (
            peers.map((peer) => (
              <div key={peer.id} className="flex justify-between items-center p-3 bg-cyber-dark rounded border border-cyber-gray">
                <div>
                  <div className="font-bold">{peer.name}</div>
                  <div className="text-sm text-cyber-gray">Last seen: {peer.lastSeen}</div>
                </div>
                <div className={`px-2 py-1 text-xs ${
                  peer.status === 'online' ? 'bg-cyber-green text-cyber-black' : 'bg-cyber-red text-white'
                }`}>
                  {peer.status.toUpperCase()}
                </div>
              </div>
            ))
          )}
        </div>
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
        <p className="text-sm text-cyber-gray mt-2">
          Stealth mode obfuscates traffic patterns and timing to prevent surveillance.
        </p>
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