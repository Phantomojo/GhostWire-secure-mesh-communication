# GhostWire Desktop GUI Implementation Plan

## Research Summary

### Technology Choices

**Framework: Tauri v2 (Stable)**
- Rust backend + WebView frontend
- Binary size: ~5-8MB (vs Electron ~150MB)
- Memory: ~30-50MB idle (vs Electron ~150MB)
- Native system access: keychain, file system, notifications
- Cross-platform: Linux, macOS, Windows
- Active development, stable v2 released

**Frontend: React + Tailwind CSS + shadcn/ui**
- React 18+ with TypeScript
- Tailwind CSS for utility-first styling
- shadcn/ui for accessible, customizable components
- Framer Motion for animations
- Zustand for state management (lightweight)

### Core Architecture

```
┌─────────────────────────────────────────────────┐
│                 Tauri Application                │
├─────────────────────────────────────────────────┤
│  Frontend (WebView)          │  Backend (Rust)   │
│  ┌─────────────────────┐     │  ┌──────────────┐ │
│  │ React + Tailwind    │     │  │ Core Engine  │ │
│  │ shadcn/ui           │◄────┼─►│ (Existing)   │ │
│  │ Framer Motion       │     │  │              │ │
│  └─────────────────────┘     │  └──────────────┘ │
│            │                 │         │         │
│            ▼                 │         ▼         │
│  ┌─────────────────────┐     │  ┌──────────────┐ │
│  │ Tauri Commands      │◄────┼─►│ IPC Layer    │ │
│  │ (TypeScript API)    │     │  │ (Commands)   │ │
│  └─────────────────────┘     │  └──────────────┘ │
└─────────────────────────────────────────────────┘
```

### Screen Breakdown (13 Screens)

#### 1. Lock Screen (Onboarding)
- **Flow**: App Start → Lock Screen → Unlock/Create
- **Components**: App logo, Unlock button, Create Account button, Password input
- **Animations**: Subtle fade-in, button hover effects

#### 2. Create Account Screen
- **Flow**: Lock Screen → Create Account → Dashboard
- **Components**: Username input, Password input, Confirm password, Create button
- **Validation**: Username length, password strength, password match
- **Backend**: `core.identity.create_new()`

#### 3. Unlock Screen
- **Flow**: Lock Screen → Unlock → Dashboard
- **Components**: Password input, Unlock button
- **Backend**: `core.identity.unlock(password)`

#### 4. Dashboard (Main Screen)
- **Flow**: Unlock → Dashboard (default view)
- **Components**:
  - Sidebar navigation
  - Mesh status indicator (online/offline, peers count)
  - Recent conversations list
  - Quick actions (New Chat, Add Contact)
  - System stats (uptime, messages sent/received)
- **Real-time updates**: Peer count, message status

#### 5. Chat Screen
- **Flow**: Dashboard → Chat → Message Thread
- **Components**:
  - Message list (infinite scroll)
  - Message input with send button
  - Typing indicators
  - Message status (sent, delivered, read)
  - Encryption indicator (E2E encrypted badge)
- **Backend**: `core.send_message()`, `core.send_direct_message()`
- **Animations**: Message appear, send button pulse

#### 6. Contacts Screen
- **Flow**: Dashboard → Contacts → Contact Details
- **Components**:
  - Contact list with avatars
  - Search/filter contacts
  - Add contact button
  - Contact details panel
- **Backend**: `core.get_peers()`, `core.connect_peer()`

#### 7. Add Contact Screen
- **Flow**: Contacts → Add Contact
- **Components**:
  - QR code scanner (camera)
  - Manual entry (Peer ID input)
  - Paste from clipboard button
  - Contact request preview
- **Backend**: `core.connect_peer()`

#### 8. Settings Screen
- **Flow**: Dashboard → Settings
- **Components**:
  - General settings (theme, notifications)
  - Network settings (bootstrap nodes, listen address)
  - Security settings (auto-lock timeout)
  - Advanced settings (debug mode)
- **Backend**: Config read/write

#### 9. Profile Screen
- **Flow**: Dashboard → Profile
- **Components**:
  - Avatar upload
  - Display name edit
  - Bio edit
  - QR code display (for sharing)
  - Copy Peer ID button
- **Backend**: `core.update_profile()`

#### 10. Network Status Screen
- **Flow**: Dashboard → Network Status
- **Components**:
  - Connected peers list
  - Peer health indicators
  - Mesh topology visualization (optional)
  - Reticulum status (if enabled)
  - Bootstrap node status
- **Backend**: `core.get_mesh_status()`, `core.get_reticulum_stats()`

#### 11. Security Center Screen
- **Flow**: Dashboard → Security Center
- **Components**:
  - Threat level indicator
  - Recent security events
  - Blacklisted peers
  - Encryption status
  - Panic button (if enabled)
- **Backend**: `core.get_security_stats()`

#### 12. Notification Settings Screen
- **Flow**: Settings → Notifications
- **Components**:
  - Desktop notifications toggle
  - Sound notifications toggle
  - Notification preview
- **Backend**: Config read/write

#### 13. About Screen
- **Flow**: Settings → About
- **Components**:
  - App version
  - Build information
  - License
  - Credits
- **Backend**: `core.constants::VERSION`

### Backend Integration Points

#### Tauri Commands (IPC Layer)

```rust
// Identity Management
#[tauri::command]
fn create_account(username: String, password: String) -> Result<(), String>

#[tauri::command]
fn unlock(password: String) -> Result<(), String>

#[tauri::command]
fn lock()

#[tauri::command]
fn get_identity() -> Result<IdentityInfo, String>

// Messaging
#[tauri::command]
async fn send_message(recipient: String, content: String) -> Result<(), String>

#[tauri::command]
async fn send_direct_message(recipient: String, content: String) -> Result<(), String>

#[tauri::command]
async fn get_messages(limit: usize) -> Result<Vec<Message>, String>

#[tauri::command]
async fn broadcast_message(content: String) -> Result<(), String>

// Contacts & Peers
#[tauri::command]
async fn get_peers() -> Result<Vec<Peer>, String>

#[tauri::command]
async fn connect_peer(peer_id: String) -> Result<(), String>

#[tauri::command]
async fn disconnect_peer(peer_id: String) -> Result<(), String>

#[tauri::command]
async fn ping_peer(peer_id: String) -> Result<(), String>

// Network
#[tauri::command]
async fn get_mesh_status() -> Result<MeshStatus, String>

#[tauri::command]
async fn get_reticulum_stats() -> Result<ReticulumStats, String>

#[tauri::command]
async fn get_bootstrap_peers() -> Result<Vec<BootstrapPeer>, String>

// Security
#[tauri::command]
async fn get_security_stats() -> Result<SecurityStats, String>

#[tauri::command]
async fn get_blacklisted_peers() -> Result<Vec<BlacklistedPeer>, String>

// Settings
#[tauri::command]
fn get_settings() -> Result<Settings, String>

#[tauri::command]
fn update_settings(settings: Settings) -> Result<(), String>

// System
#[tauri::command]
fn get_version() -> String

#[tauri::command]
fn get_system_info() -> Result<SystemInfo, String>
```

#### State Management (Tauri Events)

```rust
// Events emitted from backend
struct AppEvents;

impl AppEvents {
    // Identity events
    const IDENTITY_CREATED: &'static str = "identity-created";
    const IDENTITY_UNLOCKED: &'static str = "identity-unlocked";
    const IDENTITY_LOCKED: &'static str = "identity-locked";
    
    // Message events
    const MESSAGE_RECEIVED: &'static str = "message-received";
    const MESSAGE_SENT: &'static str = "message-sent";
    const MESSAGE_STATUS_CHANGED: &'static str = "message-status-changed";
    
    // Peer events
    const PEER_CONNECTED: &'static str = "peer-connected";
    const PEER_DISCONNECTED: &'static str = "peer-disconnected";
    const PEER_STATUS_CHANGED: &'static str = "peer-status-changed";
    
    // Network events
    const MESH_STATUS_CHANGED: &'static str = "mesh-status-changed";
    const RETICULUM_STATUS_CHANGED: &'static str = "reticulum-status-changed";
    
    // Security events
    const THREAT_DETECTED: &'static str = "threat-detected";
    const SECURITY_EVENT: &'static str = "security-event";
}
```

### File Structure

```
ghostwire-gui/
├── src-tauri/
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── commands/
│   │   │   ├── mod.rs
│   │   │   ├── identity.rs
│   │   │   ├── messaging.rs
│   │   │   ├── peers.rs
│   │   │   ├── network.rs
│   │   │   ├── security.rs
│   │   │   ├── settings.rs
│   │   │   └── system.rs
│   │   ├── state.rs
│   │   └── events.rs
│   └── icons/
│       ├── icon.png
│       └── icon.icns
├── src/
│   ├── App.tsx
│   ├── main.tsx
│   ├── index.css
│   ├── components/
│   │   ├── layout/
│   │   │   ├── Sidebar.tsx
│   │   │   ├── Header.tsx
│   │   │   └── Footer.tsx
│   │   ├── ui/
│   │   │   ├── Button.tsx
│   │   │   ├── Input.tsx
│   │   │   ├── Card.tsx
│   │   │   ├── Dialog.tsx
│   │   │   ├── Avatar.tsx
│   │   │   ├── Badge.tsx
│   │   │   ├── Tooltip.tsx
│   │   │   ├── Switch.tsx
│   │   │   ├── Tabs.tsx
│   │   │   └── ScrollArea.tsx
│   │   ├── chat/
│   │   │   ├── MessageList.tsx
│   │   │   ├── MessageItem.tsx
│   │   │   ├── MessageInput.tsx
│   │   │   └── TypingIndicator.tsx
│   │   ├── contacts/
│   │   │   ├── ContactList.tsx
│   │   │   ├── ContactItem.tsx
│   │   │   └── ContactDetails.tsx
│   │   └── shared/
│   │       ├── LoadingSpinner.tsx
│   │       ├── EmptyState.tsx
│   │       └── ErrorBoundary.tsx
│   ├── pages/
│   │   ├── LockScreen.tsx
│   │   ├── CreateAccount.tsx
│   │   ├── Unlock.tsx
│   │   ├── Dashboard.tsx
│   │   ├── Chat.tsx
│   │   ├── Contacts.tsx
│   │   ├── AddContact.tsx
│   │   ├── Settings.tsx
│   │   ├── Profile.tsx
│   │   ├── NetworkStatus.tsx
│   │   ├── SecurityCenter.tsx
│   │   ├── NotificationSettings.tsx
│   │   └── About.tsx
│   ├── hooks/
│   │   ├── useIdentity.ts
│   │   ├── useMessages.ts
│   │   ├── usePeers.ts
│   │   ├── useNetwork.ts
│   │   └── useSettings.ts
│   ├── stores/
│   │   ├── identityStore.ts
│   │   ├── messageStore.ts
│   │   ├── peerStore.ts
│   │   ├── networkStore.ts
│   │   └── settingsStore.ts
│   ├── types/
│   │   ├── identity.ts
│   │   ├── message.ts
│   │   ├── peer.ts
│   │   ├── network.ts
│   │   └── settings.ts
│   └── utils/
│       ├── api.ts
│       ├── formatters.ts
│       └── validators.ts
├── public/
│   └── index.html
├── package.json
├── tsconfig.json
├── tailwind.config.js
├── vite.config.ts
└── postcss.config.js
```

### Development Phases

#### Phase 1: Foundation (Week 1)
1. Set up Tauri project with React + TypeScript + Tailwind
2. Configure shadcn/ui components
3. Create Tauri commands for identity management
4. Implement Lock Screen, Create Account, Unlock screens
5. Set up state management (Zustand)
6. Create basic layout (Sidebar, Header)

#### Phase 2: Core Features (Week 2)
1. Implement Dashboard with mesh status
2. Add Chat screen with message list and input
3. Create Contacts screen with contact list
4. Implement Add Contact with QR scanner
5. Add real-time message updates via Tauri events

#### Phase 3: Advanced Features (Week 3)
1. Implement Settings screen
2. Add Profile screen with QR code display
3. Create Network Status screen
4. Implement Security Center
5. Add notification settings

#### Phase 4: Polish (Week 4)
1. Add animations (Framer Motion)
2. Implement dark/light theme
3. Add keyboard shortcuts
4. Create system tray integration
5. Test across platforms

### Key Implementation Details

#### Theme System
```typescript
// src/stores/themeStore.ts
import { create } from 'zustand';

type Theme = 'light' | 'dark' | 'system';

interface ThemeStore {
  theme: Theme;
  setTheme: (theme: Theme) => void;
}

export const useThemeStore = create<ThemeStore>((set) => ({
  theme: 'system',
  setTheme: (theme) => set({ theme }),
}));
```

#### Message Handling
```typescript
// src/hooks/useMessages.ts
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';

export function useMessages() {
  const [messages, setMessages] = useState<Message[]>([]);
  
  useEffect(() => {
    // Load initial messages
    invoke<Message[]>('get_messages', { limit: 50 }).then(setMessages);
    
    // Listen for new messages
    const unlisten = listen<Message>('message-received', (event) => {
      setMessages((prev) => [...prev, event.payload]);
    });
    
    return () => { unlisten.then((fn) => fn()); };
  }, []);
  
  const send = async (recipient: string, content: string) => {
    await invoke('send_message', { recipient, content });
  };
  
  return { messages, send };
}
```

#### Peer Management
```typescript
// src/hooks/usePeers.ts
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';

export function usePeers() {
  const [peers, setPeers] = useState<Peer[]>([]);
  
  useEffect(() => {
    invoke<Peer[]>('get_peers').then(setPeers);
    
    const unlisten1 = listen<Peer>('peer-connected', (event) => {
      setPeers((prev) => [...prev, event.payload]);
    });
    
    const unlisten2 = listen<string>('peer-disconnected', (event) => {
      setPeers((prev) => prev.filter((p) => p.id !== event.payload));
    });
    
    return () => {
      unlisten1.then((fn) => fn());
      unlisten2.then((fn) => fn());
    };
  }, []);
  
  return { peers };
}
```

### Performance Considerations

1. **Lazy Loading**: Load screens on demand, not all at once
2. **Virtual Scrolling**: For large message lists (react-window)
3. **Debounced Search**: For contact/message search
4. **Optimistic Updates**: Update UI immediately, sync with backend
5. **Image Caching**: Cache avatars and media locally
6. **Background Sync**: Sync messages in background, not blocking UI

### Security Considerations

1. **Password Handling**: Never store passwords in plaintext
2. **Secure Storage**: Use OS keychain for sensitive data
3. **Input Validation**: Validate all user inputs
4. **Rate Limiting**: Limit API calls to prevent abuse
5. **Encryption**: All data encrypted at rest and in transit
6. **Auto-lock**: Lock app after inactivity

### Testing Strategy

1. **Unit Tests**: Jest for React components
2. **Integration Tests**: Tauri testing utilities
3. **E2E Tests**: Playwright for full flow testing
4. **Performance Tests**: Load testing for message handling
5. **Security Tests**: Penetration testing

### Deployment

1. **Linux**: .deb, .AppImage, .rpm
2. **macOS**: .dmg, .app
3. **Windows**: .msi, .exe
4. **Auto-updater**: Built-in Tauri updater
5. **Code Signing**: For macOS and Windows

### Next Steps

1. Set up Tauri project structure
2. Implement identity management commands
3. Create Lock Screen and Onboarding flow
4. Build Dashboard with mesh status
5. Implement Chat screen
6. Add Contacts management
7. Create Settings screen
8. Polish with animations and themes
