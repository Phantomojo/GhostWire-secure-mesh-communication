import React, { useState, useRef, useEffect } from 'react';

interface MessageInputProps {
  onSendMessage: (content: string) => void;
  onTypingChange: (isTyping: boolean) => void;
  placeholder: string;
}

const MessageInput: React.FC<MessageInputProps> = ({
  onSendMessage,
  onTypingChange,
  placeholder
}) => {
  const [message, setMessage] = useState('');
  const [isEmojiPickerOpen, setIsEmojiPickerOpen] = useState(false);
  const [isAttachmentMenuOpen, setIsAttachmentMenuOpen] = useState(false);
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const typingTimeoutRef = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);

  const emojis = ['😀', '😂', '😍', '🤔', '👍', '👎', '❤️', '🔥', '💯', '🎉', '🚀', '💻', '🔒', '🌐', '⚡', '🎯'];

  useEffect(() => {
    if (textareaRef.current) {
      textareaRef.current.style.height = 'auto';
      textareaRef.current.style.height = `${textareaRef.current.scrollHeight}px`;
    }
  }, [message]);

  const handleInputChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    const value = e.target.value;
    setMessage(value);
    
    // Handle typing indicator
    onTypingChange(true);
    
    if (typingTimeoutRef.current) {
      clearTimeout(typingTimeoutRef.current);
    }
    
    typingTimeoutRef.current = setTimeout(() => {
      onTypingChange(false);
    }, 1000);
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSendMessage();
    }
  };

  const handleSendMessage = () => {
    if (message.trim()) {
      onSendMessage(message.trim());
      setMessage('');
      onTypingChange(false);
      setIsEmojiPickerOpen(false);
      setIsAttachmentMenuOpen(false);
      
      if (textareaRef.current) {
        textareaRef.current.style.height = 'auto';
      }
    }
  };

  const addEmoji = (emoji: string) => {
    setMessage(prev => prev + emoji);
    setIsEmojiPickerOpen(false);
    textareaRef.current?.focus();
  };

  const handleFileUpload = (type: 'image' | 'file' | 'voice') => {
    // Simulate file upload
    const fileNames = {
      image: 'image.jpg',
      file: 'document.pdf',
      voice: 'voice_message.mp3'
    };
    
    onSendMessage(`📎 ${fileNames[type]}`);
    setIsAttachmentMenuOpen(false);
  };

  return (
    <div className="message-input-container">
      {/* Emoji Picker */}
      {isEmojiPickerOpen && (
        <div className="emoji-picker">
          <div className="emoji-grid">
            {emojis.map((emoji, index) => (
              <button
                key={index}
                className="emoji-btn"
                onClick={() => addEmoji(emoji)}
              >
                {emoji}
              </button>
            ))}
          </div>
        </div>
      )}

      {/* Attachment Menu */}
      {isAttachmentMenuOpen && (
        <div className="attachment-menu">
          <button
            className="attachment-option"
            onClick={() => handleFileUpload('image')}
          >
            <span>📷</span>
            <span>Photo</span>
          </button>
          <button
            className="attachment-option"
            onClick={() => handleFileUpload('file')}
          >
            <span>📎</span>
            <span>Document</span>
          </button>
          <button
            className="attachment-option"
            onClick={() => handleFileUpload('voice')}
          >
            <span>🎤</span>
            <span>Voice</span>
          </button>
        </div>
      )}

      {/* Input Area */}
      <div className="message-input-wrapper">
        <div className="input-actions">
          <button
            className="action-btn"
            onClick={() => setIsAttachmentMenuOpen(!isAttachmentMenuOpen)}
          >
            📎
          </button>
          <button
            className="action-btn"
            onClick={() => setIsEmojiPickerOpen(!isEmojiPickerOpen)}
          >
            😀
          </button>
        </div>

        <div className="input-field-wrapper">
          <textarea
            ref={textareaRef}
            value={message}
            onChange={handleInputChange}
            onKeyPress={handleKeyPress}
            placeholder={placeholder}
            className="message-input"
            rows={1}
            maxLength={1000}
          />
        </div>

        <button
          className={`send-btn ${message.trim() ? 'active' : ''}`}
          onClick={handleSendMessage}
          disabled={!message.trim()}
        >
          <span>📤</span>
        </button>
      </div>
    </div>
  );
};

export default MessageInput; 