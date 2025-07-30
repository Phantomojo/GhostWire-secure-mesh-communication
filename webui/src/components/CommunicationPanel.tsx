import React, { useState } from 'react';

interface Message {
  id: string;
  content: string;
  sender: string;
  recipient: string;
  timestamp: Date;
  status: 'pending' | 'sent' | 'delivered' | 'failed';
  encrypted: boolean;
}

interface CommunicationPanelProps {
  messages: Message[];
  onSendMessage: (content: string, recipient: string) => void;
  onBroadcast: () => void;
  onEmergency: () => void;
  onStatus: () => void;
  onScan: () => void;
  onSync: () => void;
  onAnalyze: () => void;
  backendConnected: boolean;
}

const CommunicationPanel: React.FC<CommunicationPanelProps> = ({
  messages,
  onSendMessage,
  onBroadcast,
  onEmergency,
  onStatus,
  onScan,
  onSync,
  onAnalyze,
  backendConnected
}) => {
  const [messageContent, setMessageContent] = useState('');
  const [recipient, setRecipient] = useState('all');
  const [selectedMessageType, setSelectedMessageType] = useState<'text' | 'broadcast' | 'emergency'>('text');

  const recipients = [
    { id: 'all', name: 'All Nodes', icon: '🌐' },
    { id: 'alpha', name: 'Node-Alpha', icon: '🟢' },
    { id: 'beta', name: 'Node-Beta', icon: '🟢' },
    { id: 'gamma', name: 'Node-Gamma', icon: '🟡' },
    { id: 'delta', name: 'Node-Delta', icon: '🔴' }
  ];

  const messageTypes = [
    { id: 'text', name: 'Text Message', icon: '💬' },
    { id: 'broadcast', name: 'Broadcast', icon: '📡' },
    { id: 'emergency', name: 'Emergency', icon: '🚨' }
  ];

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'pending': return '⏳';
      case 'sent': return '✓';
      case 'delivered': return '✓✓';
      case 'failed': return '❌';
      default: return '⏳';
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'pending': return '#ffaa00';
      case 'sent': return '#44ff44';
      case 'delivered': return '#0088ff';
      case 'failed': return '#ff4444';
      default: return '#888888';
    }
  };

  const formatTime = (date: Date) => {
    return date.toLocaleTimeString();
  };

  const handleSend = () => {
    if (messageContent.trim()) {
      const finalContent = selectedMessageType === 'emergency' 
        ? `🚨 EMERGENCY: ${messageContent}`
        : selectedMessageType === 'broadcast'
        ? `📡 BROADCAST: ${messageContent}`
        : messageContent;

      onSendMessage(finalContent, recipient);
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
    <div className="communication-panel">
      <div className="panel-header">
        <h2>📡 COMMUNICATION CENTER</h2>
        <div className="communication-stats">
          <div className="stat">
            <span className="stat-label">Total Messages:</span>
            <span className="stat-value">{messages.length}</span>
          </div>
          <div className="stat">
            <span className="stat-label">Encrypted:</span>
            <span className="stat-value">{messages.filter(m => m.encrypted).length}</span>
          </div>
          <div className="stat">
            <span className="stat-label">Failed:</span>
            <span className="stat-value">{messages.filter(m => m.status === 'failed').length}</span>
          </div>
        </div>
      </div>

      <div className="panel-content">
        {/* Message Composition */}
        <div className="message-composition">
          <div className="section-title">MESSAGE COMPOSITION</div>
          
          <div className="composition-controls">
            <div className="control-group">
              <div className="control-label">MESSAGE TYPE</div>
              <div className="type-selector">
                {messageTypes.map((type) => (
                  <button
                    key={type.id}
                    className={`type-btn ${selectedMessageType === type.id ? 'active' : ''}`}
                    onClick={() => setSelectedMessageType(type.id as any)}
                  >
                    <span className="type-icon">{type.icon}</span>
                    <span className="type-name">{type.name}</span>
                  </button>
                ))}
              </div>
            </div>

            <div className="control-group">
              <div className="control-label">RECIPIENT</div>
              <div className="recipient-selector">
                {recipients.map((rec) => (
                  <button
                    key={rec.id}
                    className={`recipient-btn ${recipient === rec.id ? 'active' : ''}`}
                    onClick={() => setRecipient(rec.id)}
                  >
                    <span className="recipient-icon">{rec.icon}</span>
                    <span className="recipient-name">{rec.name}</span>
                  </button>
                ))}
              </div>
            </div>
          </div>

          <div className="message-input-area">
            <textarea
              value={messageContent}
              onChange={(e) => setMessageContent(e.target.value)}
              onKeyPress={handleKeyPress}
              placeholder={`Type your ${selectedMessageType} message...`}
              className="message-textarea"
              rows={4}
            />
            <div className="input-actions">
              <div className="message-info">
                <span className="encryption-indicator">🔒 Encrypted</span>
                <span className="character-count">{messageContent.length}/1000</span>
              </div>
              <button 
                className="send-btn"
                onClick={handleSend}
                disabled={!messageContent.trim()}
              >
                <span className="send-icon">📤</span>
                <span className="send-text">SEND</span>
              </button>
            </div>
          </div>
        </div>

        {/* Message History */}
        <div className="message-history">
          <div className="section-title">MESSAGE HISTORY</div>
          <div className="messages-container">
            {messages.length === 0 ? (
              <div className="no-messages">
                <div className="no-messages-icon">📭</div>
                <div className="no-messages-text">No messages sent yet</div>
              </div>
            ) : (
              messages.map((message) => (
                <div key={message.id} className="message-item">
                  <div className="message-header">
                    <div className="message-sender">
                      <span className="sender-icon">👤</span>
                      <span className="sender-name">{message.sender}</span>
                    </div>
                    <div className="message-time">{formatTime(message.timestamp)}</div>
                  </div>
                  
                  <div className="message-content">
                    <div className="message-text">{message.content}</div>
                    <div className="message-meta">
                      <span className="recipient-info">To: {message.recipient}</span>
                      {message.encrypted && <span className="encryption-badge">🔒</span>}
                    </div>
                  </div>
                  
                  <div className="message-status">
                    <span 
                      className="status-indicator"
                      style={{ color: getStatusColor(message.status) }}
                    >
                      {getStatusIcon(message.status)} {message.status.toUpperCase()}
                    </span>
                  </div>
                </div>
              ))
            )}
          </div>
        </div>

        {/* Quick Actions */}
        <div className="communication-actions">
          <div className="section-title">QUICK ACTIONS</div>
          <div className="actions-grid">
            <button 
              className="comm-action-btn"
              onClick={onBroadcast}
              disabled={!backendConnected}
            >
              <div className="action-icon">📡</div>
              <div className="action-label">BROADCAST</div>
            </button>
            
            <button 
              className="comm-action-btn"
              onClick={onEmergency}
              disabled={!backendConnected}
            >
              <div className="action-icon">🚨</div>
              <div className="action-label">EMERGENCY</div>
            </button>
            
            <button 
              className="comm-action-btn"
              onClick={onStatus}
              disabled={!backendConnected}
            >
              <div className="action-icon">📋</div>
              <div className="action-label">STATUS</div>
            </button>
            
            <button 
              className="comm-action-btn"
              onClick={onScan}
              disabled={!backendConnected}
            >
              <div className="action-icon">🔍</div>
              <div className="action-label">SCAN</div>
            </button>
            
            <button 
              className="comm-action-btn"
              onClick={onSync}
              disabled={!backendConnected}
            >
              <div className="action-icon">🔄</div>
              <div className="action-label">SYNC</div>
            </button>
            
            <button 
              className="comm-action-btn"
              onClick={onAnalyze}
              disabled={!backendConnected}
            >
              <div className="action-icon">📊</div>
              <div className="action-label">ANALYZE</div>
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default CommunicationPanel; 